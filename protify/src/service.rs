use crate::*;

/// A trait that enables the generation of a protobuf service schema.
///
/// Implemented by the enums annotated with the [`proto_service`] macro.
pub trait ProtoService {
	fn proto_schema() -> Service;
}

/// A struct representing a protobuf service.
#[derive(Debug, PartialEq, Builder)]
#[cfg_attr(feature = "std", derive(Template))]
#[cfg_attr(feature = "std", template(path = "service.proto.j2"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Service {
	pub name: FixedStr,
	pub file: FixedStr,
	pub options: Vec<ProtoOption>,
	pub handlers: Vec<ServiceHandler>,
	pub package: FixedStr,
}

impl Service {
	/// Renders the schema representation.
	#[cfg(feature = "std")]
	pub fn render_schema(&self) -> Result<String, askama::Error> {
		self.render()
	}
}

/// A struct that represents a protobuf service handler.
#[derive(Debug, PartialEq, Builder)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct ServiceHandler {
	pub name: FixedStr,
	pub options: Vec<ProtoOption>,
	pub request: ProtoPath,
	pub response: ProtoPath,
}
