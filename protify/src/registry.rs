use hashbrown::DefaultHashBuilder;

use crate::*;

#[cfg(feature = "inventory")]
pub(crate) mod inventory_collection {
	use super::*;

	type Map<K, V> = ordermap::OrderMap<K, V, DefaultHashBuilder>;

	#[track_caller]
	fn process_msg(
		msg_name: &FixedStr,
		messages: &mut Map<FixedStr, MessageSchema>,
		enums: &mut Map<FixedStr, EnumSchema>,
		parent_messages_map: &mut Map<FixedStr, NestedItems>,
	) -> MessageSchema {
		let mut msg = messages
			.swap_remove(msg_name)
			.unwrap_or_else(|| panic!("Could not find message {msg_name}"));

		let Some(children) = parent_messages_map.swap_remove(msg_name) else {
			return msg;
		};

		for child in children.messages {
			let child_data = process_msg(&child, messages, enums, parent_messages_map);

			msg.messages.push(child_data);
		}

		for enum_ in children.enums {
			let enum_data = enums
				.swap_remove(&enum_)
				.unwrap_or_else(|| panic!("Could not find enum {enum_}"));

			msg.enums.push(enum_data);
		}

		msg
	}

	#[derive(Debug, Default)]
	struct NestedItems {
		pub enums: Vec<FixedStr>,
		pub messages: Vec<FixedStr>,
	}

	#[must_use]
	#[track_caller]
	pub(crate) fn collect_package(package: &'static str) -> Package {
		let mut messages: Map<FixedStr, MessageSchema> = Map::default();
		let mut enums: Map<FixedStr, EnumSchema> = Map::default();
		let mut parent_messages_map: Map<FixedStr, NestedItems> = Map::default();
		let mut root_messages: Vec<FixedStr> = Vec::new();
		let mut files: Map<FixedStr, ProtoFile> = Map::default();

		for file_entry in inventory::iter::<RegistryFile>().filter(|f| f.package == package) {
			let file: ProtoFile = (file_entry.file)();

			files.insert(file.name.clone(), file);
		}

		for msg_entry in inventory::iter::<RegistryMessage>().filter(|rm| rm.package == package) {
			let msg = (msg_entry.message)();

			if let Some(parent_getter) = msg_entry.parent_message {
				let parent = parent_getter();

				parent_messages_map
					.entry(parent.into())
					.or_default()
					.messages
					.push(msg.name.clone());
			} else {
				root_messages.push(msg.name.clone());
			}

			messages.insert(msg.name.clone(), msg);
		}

		for enum_entry in inventory::iter::<RegistryEnum>().filter(|rm| rm.package == package) {
			let enum_ = (enum_entry.enum_)();

			if let Some(parent_getter) = enum_entry.parent_message {
				let parent = parent_getter();

				parent_messages_map
					.entry(parent.into())
					.or_default()
					.enums
					.push(enum_.name.clone());

				enums.insert(enum_.name.clone(), enum_);
			} else {
				files
					.get_mut(&enum_.file)
					.unwrap_or_else(|| panic!("Could not find the data for file {}", enum_.file))
					.enums
					.push(enum_);
			}
		}

		for root_message_name in root_messages {
			let msg = process_msg(
				&root_message_name,
				&mut messages,
				&mut enums,
				&mut parent_messages_map,
			);

			files
				.get_mut(&msg.file)
				.unwrap_or_else(|| panic!("Could not find the data for file {}", msg.file))
				.with_messages(vec![msg]);
		}

		for service_entry in inventory::iter::<RegistryService>().filter(|rs| rs.package == package)
		{
			let service = (service_entry.service)();

			files
				.get_mut(&service.file)
				.unwrap_or_else(|| panic!("Could not find the data for file {}", service.file))
				.with_services(vec![service]);
		}

		let files: Vec<ProtoFile> = files
			.into_values()
			.map(|mut file| {
				file.sort_items();

				file
			})
			.collect();

		Package {
			name: package.into(),
			files,
		}
	}
}

#[doc(hidden)]
pub struct RegistryMessage {
	pub package: &'static str,
	pub parent_message: Option<fn() -> &'static str>,
	pub message: fn() -> MessageSchema,
}

#[doc(hidden)]
pub struct RegistryEnum {
	pub package: &'static str,
	pub parent_message: Option<fn() -> &'static str>,
	pub enum_: fn() -> EnumSchema,
}

#[doc(hidden)]
pub struct RegistryService {
	pub package: &'static str,
	pub service: fn() -> Service,
}

#[doc(hidden)]
pub struct RegistryFile {
	pub package: &'static str,
	pub file: fn() -> ProtoFile,
}

#[cfg(feature = "inventory")]
inventory::collect!(RegistryMessage);
#[cfg(feature = "inventory")]
inventory::collect!(RegistryEnum);
#[cfg(feature = "inventory")]
inventory::collect!(RegistryService);
#[cfg(feature = "inventory")]
inventory::collect!(RegistryFile);
