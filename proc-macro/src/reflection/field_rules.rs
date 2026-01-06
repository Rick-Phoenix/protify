use ::proto_types::protovalidate::*;

use super::*;

pub fn get_field_validator(ctx: &RulesCtx, proto_type: &ProtoType) -> syn::Result<BuilderTokens> {
  let validator = match proto_type {
    ProtoType::String => get_string_validator(ctx),
    ProtoType::Bool => get_bool_validator(ctx),
    ProtoType::Bytes => get_bytes_validator(ctx),
    ProtoType::Enum(path) => get_enum_validator(ctx, path),
    ProtoType::Message(message_info) => get_message_field_validator(ctx, &message_info.path),
    ProtoType::Float => get_numeric_validator::<FloatRules>(ctx),
    ProtoType::Double => get_numeric_validator::<DoubleRules>(ctx),
    ProtoType::Int32 => get_numeric_validator::<Int32Rules>(ctx),
    ProtoType::Int64 => get_numeric_validator::<Int64Rules>(ctx),
    ProtoType::Uint32 => get_numeric_validator::<UInt32Rules>(ctx),
    ProtoType::Uint64 => get_numeric_validator::<UInt64Rules>(ctx),
    ProtoType::Sint32 => get_numeric_validator::<SInt32Rules>(ctx),
    ProtoType::Sint64 => get_numeric_validator::<SInt64Rules>(ctx),
    ProtoType::Fixed32 => get_numeric_validator::<Fixed32Rules>(ctx),
    ProtoType::Fixed64 => get_numeric_validator::<Fixed64Rules>(ctx),
    ProtoType::Sfixed32 => get_numeric_validator::<SFixed32Rules>(ctx),
    ProtoType::Sfixed64 => get_numeric_validator::<SFixed64Rules>(ctx),
    ProtoType::Duration => get_duration_validator(ctx),
    ProtoType::Timestamp => get_timestamp_validator(ctx),
    ProtoType::Any => get_any_validator(ctx),
    ProtoType::FieldMask => get_field_mask_validator(ctx),
  };

  Ok(validator)
}
