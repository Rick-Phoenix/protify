use super::*;

proto_package!(EXTERN_PATH_TEST, name = "extern_path_test", no_cel_test);

define_proto_file!(
	FILE,
	name = "file.proto",
	package = EXTERN_PATH_TEST,
	extern_path = "testing"
);

#[proto_message]
#[proto(skip_checks(all))]
pub struct NormalMsg {
	pub id: i32,
}

#[proto_message]
#[proto(skip_checks(all))]
#[proto(parent_message = NormalMsg)]
pub struct NormalNestedMsg {
	pub id: i32,
}

#[proto_enum]
pub enum NormalEnum {
	Unspecified,
	A,
	B,
}

#[proto_enum]
#[proto(parent_message = NormalMsg)]
pub enum NormalNestedEnum {
	Unspecified,
	A,
	B,
}

pub(crate) mod re_exported {
	use super::*;

	// Inheriting the file and keeping the same module path
	// (for re-exported items)
	inherit_proto_file!(FILE);

	#[proto_message]
	#[proto(skip_checks(all))]
	pub struct ReExportedMsg {
		pub id: i32,
	}

	#[proto_enum]
	pub enum ReExportedEnum {
		Unspecified,
		A,
		B,
	}
}

pub mod other_file {
	use super::*;

	define_proto_file!(
		OTHER_FILE,
		name = "other_file.proto",
		package = EXTERN_PATH_TEST
	);
}

#[allow(unused)]
pub use overrides::MsgWithFileAndPathOverride;

pub mod overrides {
	use super::other_file::OTHER_FILE;
	use super::*;

	#[proto_message]
	#[proto(module_path = "testing")]
	#[proto(file = OTHER_FILE)]
	#[proto(skip_checks(all))]
	pub struct MsgWithFileAndPathOverride {
		pub id: i32,
	}

	#[proto_message]
	#[proto(file = OTHER_FILE)]
	#[proto(skip_checks(all))]
	pub struct MsgWithFileOverride {
		pub id: i32,
	}

	#[proto_enum]
	#[proto(module_path = "testing")]
	#[proto(file = OTHER_FILE)]
	pub enum EnumWithFileAndPathOverride {
		Unspecified,
		A,
		B,
	}

	#[proto_enum]
	#[proto(file = OTHER_FILE)]
	pub enum EnumWithFileOverride {
		Unspecified,
		A,
		B,
	}
}

pub mod submod {
	use super::*;

	// Inheriting the file handle, but overriding extern path
	use_proto_file!(FILE);

	#[proto_message]
	#[proto(skip_checks(all))]
	pub struct SubmodMsg {
		pub id: i32,
	}

	#[proto_message]
	#[proto(skip_checks(all))]
	#[proto(parent_message = SubmodMsg)]
	pub struct SubmodNestedMsg {
		pub id: i32,
	}

	#[proto_enum]
	pub enum SubmodEnum {
		Unspecified,
		A,
		B,
	}

	#[proto_enum]
	#[proto(parent_message = SubmodMsg)]
	pub enum SubmodNestedEnum {
		Unspecified,
		A,
		B,
	}
}

#[test]
fn test_file_assignment() {
	use re_exported::*;
	use submod::*;
	macro_rules! check_file {
    ($($name:ident),*) => {
      $(
        assert_eq_pretty!($name::proto_schema().file, "file.proto");
      )*
    };
  }

	check_file!(
		NormalMsg,
		NormalNestedMsg,
		NormalEnum,
		NormalNestedMsg,
		ReExportedMsg,
		ReExportedEnum,
		SubmodMsg,
		SubmodNestedMsg,
		SubmodEnum,
		SubmodNestedEnum
	);

	macro_rules! check_other_file {
    ($($name:ident),*) => {
      $(
        assert_eq_pretty!(overrides::$name::proto_schema().file, "other_file.proto");
      )*
    };
  }

	check_other_file!(
		MsgWithFileAndPathOverride,
		MsgWithFileOverride,
		EnumWithFileAndPathOverride,
		EnumWithFileOverride
	);
}

#[test]
fn test_extern_path() {
	let pkg = EXTERN_PATH_TEST::get_package();
	let mut paths = pkg.extern_paths();

	let expected = [
		(
			".extern_path_test.SubmodMsg",
			concat!("::", module_path!(), "::submod::SubmodMsg"),
		),
		(
			".extern_path_test.SubmodMsg.SubmodNestedMsg",
			concat!("::", module_path!(), "::submod::SubmodNestedMsg"),
		),
		(
			".extern_path_test.SubmodEnum",
			concat!("::", module_path!(), "::submod::SubmodEnum"),
		),
		(
			".extern_path_test.SubmodMsg.SubmodNestedEnum",
			concat!("::", module_path!(), "::submod::SubmodNestedEnum"),
		),
		(
			".extern_path_test.ReExportedMsg",
			"::testing::ReExportedMsg",
		),
		(".extern_path_test.NormalMsg", "::testing::NormalMsg"),
		(
			".extern_path_test.NormalMsg.NormalNestedMsg",
			"::testing::NormalNestedMsg",
		),
		(
			".extern_path_test.ReExportedEnum",
			"::testing::ReExportedEnum",
		),
		(".extern_path_test.NormalEnum", "::testing::NormalEnum"),
		(
			".extern_path_test.NormalMsg.NormalNestedEnum",
			"::testing::NormalNestedEnum",
		),
		(
			".extern_path_test.MsgWithFileAndPathOverride",
			"::testing::MsgWithFileAndPathOverride",
		),
		(
			".extern_path_test.MsgWithFileOverride",
			concat!("::", module_path!(), "::other_file::MsgWithFileOverride"),
		),
		(
			".extern_path_test.EnumWithFileAndPathOverride",
			"::testing::EnumWithFileAndPathOverride",
		),
		(
			".extern_path_test.EnumWithFileOverride",
			concat!("::", module_path!(), "::other_file::EnumWithFileOverride"),
		),
	];

	for (exp_name, exp_path) in expected {
		let idx = paths
			.iter()
			.position(|(name, path)| {
				if exp_name == name {
					assert_eq_pretty!(exp_path, path);
				}

				exp_name == name && exp_path == path
			})
			.unwrap_or_else(|| panic!("Could not find {exp_name} in the extern paths"));

		paths.remove(idx);
	}

	if !paths.is_empty() {
		panic!("Unexpected extern paths: {paths:#?}");
	}
}
