use maplit::btreemap;
use protify::proto_types::{
	Any, Duration, FieldMask, Timestamp, protovalidate::violations_data::*,
};

use super::*;

macro_rules! impl_custom_messages {
  ($typ:ident => $($rules:ident),*) => {
    paste! {
      btreemap! {
        $(
          [< $typ Violation >]::$rules => "they're taking the hobbits to Isengard!"
        ),*
      }
    }
  };
}

fn string_validator() -> StringValidator {
	StringValidator::builder()
		.min_len(4)
		.min_bytes(4)
		.contains("abcde")
		.not_contains("abc")
		.email()
		.in_(["abcde"])
		.not_in(["abc"])
		.with_error_messages(
			impl_custom_messages!(String => MinLen, MinBytes, Contains, NotContains, Email, In, NotIn),
		)
		.build()
}

fn bytes_validator() -> BytesValidator {
	BytesValidator::builder()
		.min_len(4)
		.contains(b"abcde")
		.ip()
		.in_([b"abcde"])
		.not_in([b"abc"])
		.with_error_messages(impl_custom_messages!(Bytes => MinLen, Contains, Ip, In, NotIn))
		.build()
}

fn any_validator() -> AnyValidator {
	AnyValidator::builder()
		.in_(["abcde"])
		.not_in(["abc"])
		.with_error_messages(impl_custom_messages!(Any => In, NotIn))
		.build()
}

fn enum_validator() -> EnumValidator<SimpleEnum> {
	EnumValidator::<SimpleEnum>::builder()
		.in_([2])
		.not_in([10])
		.defined_only()
		.with_error_messages(impl_custom_messages!(Enum => DefinedOnly, In, NotIn))
		.build()
}

fn duration_validator() -> DurationValidator {
	DurationValidator::builder()
		.in_([Duration::new(10, 0)])
		.not_in([Duration::default()])
		.gt(Duration::default())
		.with_error_messages(impl_custom_messages!(Duration => Gt, In, NotIn))
		.build()
}

fn timestamp_validator() -> TimestampValidator {
	TimestampValidator::builder()
		.gt(Timestamp::default())
		.within(Duration::default())
		.with_error_messages(impl_custom_messages!(Timestamp => Gt, Within))
		.build()
}

fn field_mask_validator() -> FieldMaskValidator {
	FieldMaskValidator::builder()
		.in_(["abcde"])
		.not_in(["abc"])
		.with_error_messages(impl_custom_messages!(FieldMask => In, NotIn))
		.build()
}

fn int_validator() -> IntValidator<i32> {
	IntValidator::<i32>::builder()
		.in_([10])
		.not_in([0])
		.gt(0)
		.with_error_messages(impl_custom_messages!(Int32 => Gt, In, NotIn))
		.build()
}

fn float_validator() -> FloatValidator<f32> {
	FloatValidator::<f32>::builder()
		.in_([10.0])
		.not_in([0.0])
		.gt(0.0)
		.with_error_messages(impl_custom_messages!(Float => Gt, In, NotIn))
		.build()
}

fn repeated_validator() -> RepeatedValidator<i32> {
	RepeatedValidator::<i32>::builder()
		.max_items(1)
		.unique()
		.with_error_messages(impl_custom_messages!(Repeated => MaxItems, Unique))
		.build()
}

fn map_validator() -> MapValidator<i32, i32> {
	MapValidator::<i32, i32>::builder()
		.min_pairs(1)
		.with_error_messages(impl_custom_messages!(Map => MinPairs))
		.build()
}

#[proto_message]
#[proto(skip_checks(all))]
struct CustomErrorMessages {
	#[proto(validate = string_validator())]
	string: String,
	#[proto(bytes, validate = bytes_validator())]
	bytes: Bytes,
	#[proto(any, validate = any_validator())]
	any: Option<Any>,
	#[proto(enum_(SimpleEnum), validate = enum_validator())]
	enum_: i32,
	#[proto(duration, validate = duration_validator())]
	duration: Option<Duration>,
	#[proto(timestamp, validate = timestamp_validator())]
	timestamp: Option<Timestamp>,
	#[proto(field_mask, validate = field_mask_validator())]
	field_mask: Option<FieldMask>,
	#[proto(validate = int_validator())]
	int: i32,
	#[proto(validate = float_validator())]
	float: f32,
	#[proto(validate = repeated_validator())]
	repeated: Vec<i32>,
	#[proto(map(int32, int32), validate = map_validator())]
	map: HashMap<i32, i32>,
	#[proto(oneof(tags(1, 2)), validate = |v| v.required().with_error_message("they're taking the hobbits to Isengard!"))]
	required_oneof: Option<SimpleOneof>,
	#[proto(message, validate = |v| v.required().required_error_message("they're taking the hobbits to Isengard!"))]
	required_message: Option<DirectMsg>,
}

#[test]
fn custom_error_messages() {
	let msg = CustomErrorMessages {
		string: "abc".to_string(),
		bytes: Bytes::from_static(b"abc"),
		any: Some(Any {
			type_url: "abc".to_string(),
			value: vec![],
		}),
		enum_: 10,
		duration: Some(Default::default()),
		timestamp: Some(Default::default()),
		field_mask: Some(FieldMask {
			paths: vec!["abc".to_string()],
		}),
		int: Default::default(),
		float: Default::default(),
		repeated: vec![0, 0],
		map: Default::default(),
		required_oneof: None,
		required_message: None,
	};

	let violations = msg.validate_all().unwrap_err().into_violations();

	// 7 for strings
	// + 5 for bytes
	// + 2 for any
	// + 3 for enum
	// + 3 for duration
	// + 2 for timestamp
	// + 2 for fieldmask
	// + 3 for ints
	// + 3 for float
	// + 2 for repeated
	// + 1 for map
	// + 1 for required oneof
	// + 1 for required message
	// = 35
	assert_eq_pretty!(violations.len(), 35);

	for v in violations {
		assert_eq_pretty!(v.message(), "they're taking the hobbits to Isengard!");
	}
}
