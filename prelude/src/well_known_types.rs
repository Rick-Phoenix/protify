use proto_types::{Any, Code, Duration, Empty, FieldMask, Status, Timestamp};

use crate::*;

impl_known_type!(target = Status, package = "google.rpc");
impl_known_type!(target = Code, package = "google.rpc", type_ = Enum);

impl_known_type!(target = Empty, package = "google.protobuf");
type Unit = ();
impl_known_type!(target = Unit, package = "google.protobuf", name = "Empty");

impl_known_type!(
  target = Duration,
  impl_validator = false,
  package = "google.protobuf"
);
impl_known_type!(
  target = Timestamp,
  impl_validator = false,
  package = "google.protobuf"
);
impl_known_type!(
  target = FieldMask,
  impl_validator = false,
  package = "google.protobuf"
);
impl_known_type!(
  target = Any,
  impl_validator = false,
  package = "google.protobuf"
);

#[doc(hidden)]
#[derive(Clone, Copy, Default)]
pub struct NoOpValidator<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized + Send + Sync + ToOwned> Validator<T> for NoOpValidator<T> {
  type Target = T;

  #[inline(always)]
  fn validate_core<V>(&self, _ctx: &mut ValidationCtx, _val: Option<&V>) -> ValidationResult
  where
    V: Borrow<Self::Target> + ?Sized,
  {
    Ok(IsValid::Yes)
  }
}

#[doc(hidden)]
pub struct NoOpValidatorBuilder<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Default for NoOpValidatorBuilder<T> {
  fn default() -> Self {
    Self(Default::default())
  }
}

impl<T> ValidatorBuilderFor<T> for NoOpValidatorBuilder<T>
where
  T: ?Sized + Send + Sync + ToOwned,
{
  type Validator = NoOpValidator<T>;
  fn build_validator(self) -> Self::Validator {
    NoOpValidator(PhantomData)
  }
}

#[cfg(feature = "common-types")]
mod google_dot_type {
  use super::*;
  use proto_types::*;

  macro_rules! impl_types {
    ($($name:ident),*) => {
      $(
        impl_known_type!(target = $name, package = "google.type");
      )*
    };
  }

  impl_types!(
    Date,
    Interval,
    Money,
    Color,
    Fraction,
    Decimal,
    PostalAddress,
    PhoneNumber,
    Quaternion,
    LocalizedText,
    Expr
  );

  impl_known_type!(
    target = DateTime,
    package = "google.type",
    file = "google/type/datetime.proto"
  );
  impl_known_type!(
    target = TimeZone,
    package = "google.type",
    file = "google/type/datetime.proto"
  );
  impl_known_type!(
    target = LatLng,
    package = "google.type",
    file = "google/type/latlng.proto"
  );
  impl_known_type!(
    target = TimeOfDay,
    package = "google.type",
    file = "google/type/timeofday.proto"
  );
  impl_known_type!(
    target = CalendarPeriod,
    package = "google.type",
    type_ = Enum
  );
  impl_known_type!(target = Month, package = "google.type", type_ = Enum);
  impl_known_type!(
    target = DayOfWeek,
    package = "google.type",
    type_ = Enum,
    file = "google/type/dayofweek.proto"
  );
}

#[cfg(feature = "common-types")]
mod rpc_types {
  use super::*;
  use proto_types::{
    bad_request::FieldViolation, help::Link,
    precondition_failure::Violation as PreconditionViolation,
    quota_failure::Violation as QuotaFailureViolation, *,
  };

  macro_rules! impl_types {
    ($($name:ident),*) => {
      $(
        impl_known_type!(target = $name, package = "google.rpc", file = "google/rpc/error_details.proto");
      )*
    };
  }

  impl_types!(
    ErrorInfo,
    DebugInfo,
    RetryInfo,
    QuotaFailure,
    PreconditionFailure,
    BadRequest,
    RequestInfo,
    ResourceInfo,
    Help,
    LocalizedMessage
  );

  impl_known_type!(
    target = HttpRequest,
    package = "google.rpc",
    file = "google/rpc/http.proto"
  );
  impl_known_type!(
    target = HttpResponse,
    package = "google.rpc",
    file = "google/rpc/http.proto"
  );
  impl_known_type!(
    target = HttpHeader,
    package = "google.rpc",
    file = "google/rpc/http.proto",
    store = hybrid
  );

  impl_known_type!(
    target = QuotaFailureViolation,
    name = "QuotaFailure.Violation",
    package = "google.rpc",
    file = "google/rpc/error_details.proto"
  );

  impl_known_type!(
    target = PreconditionViolation,
    name = "PreconditionFailure.Violation",
    package = "google.rpc",
    file = "google/rpc/error_details.proto"
  );

  impl_known_type!(
    target = FieldViolation,
    name = "BadRequest.FieldViolation",
    package = "google.rpc",
    file = "google/rpc/error_details.proto"
  );

  impl_known_type!(
    target = Link,
    name = "Help.Link",
    package = "google.rpc",
    file = "google/rpc/error_details.proto"
  );
}
