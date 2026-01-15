#[cfg(feature = "reflection")]
mod proto {
  include!(concat!(env!("OUT_DIR"), "/no_std_models.rs"));
}

#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod test {
  use maplit::btreemap;
  use prelude::ValidatedMessage;

  #[cfg(feature = "reflection")]
  use crate::proto::{test_msg::*, *};

  #[cfg(not(feature = "reflection"))]
  use no_std_models::*;

  #[test]
  fn name() {
    let mut msg = TestMsg {
      map: btreemap! { 1 => 1, 2 => 2 },
      test_oneof: Some(TestOneof::A(1)),
      enum_field: TestEnum::A.into(),
    };
    let baseline = msg.clone();

    assert!(msg.validate().is_ok());

    msg.map.clear();

    assert!(msg.validate().is_err());
    msg = baseline.clone();

    msg.enum_field = 100;
    assert!(msg.validate().is_err());
  }
}
