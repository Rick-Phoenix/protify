use std::collections::HashMap;

use protify::*;

proto_package!(TESTING_PKG, name = "testing", no_cel_test);

define_proto_file!(TESTING, name = "testing.proto", package = TESTING_PKG);

#[allow(clippy::use_self)]
#[proto_message]
#[proto(skip_checks(all))]
struct MsgWithNoValidator {
  id: i32,
  #[proto(oneof(tags(1, 2, 3)))]
  oneof: Option<OneofWithNoValidator>,
  #[proto(message)]
  recursive: Option<Box<MsgWithNoValidator>>,
  #[proto(repeated(message))]
  vec: Vec<MsgWithNoValidator>,
  #[proto(map(int32, message))]
  map: HashMap<i32, MsgWithNoValidator>,
  #[proto(message)]
  second_degree_recursion: Option<SecondDegreeRecursion>,
}

#[proto_message]
#[proto(skip_checks(all))]
struct SecondDegreeRecursion {
  #[proto(message)]
  recursive: Option<Box<MsgWithNoValidator>>,
  #[proto(oneof(tags(1, 2, 3)))]
  third_degree_recursion: Option<OneofWithNoValidator>,
}

#[proto_oneof]
#[proto(skip_checks(all))]
enum OneofWithNoValidator {
  #[proto(tag = 1)]
  A(i32),
  #[proto(tag = 2)]
  B(i32),
  #[proto(tag = 3, message)]
  Recursive(Box<MsgWithNoValidator>),
}

// Run `just check-asm-output` to view the asm output of this function.
//
// If the validator is being correctly detected as empty, the assembly output should be more or less like this:
//
// .section .text.trigger_validation,"ax",@progbits
// .globl  trigger_validation
// .p2align        4
// .type   trigger_validation,@function
// trigger_validation:
// .cfi_startproc
// ret
#[unsafe(no_mangle)]
#[inline(never)]
fn trigger_validation(msg: &MsgWithNoValidator) {
  let _ = msg.validate();
}

// Dummy fn that should crash the test if there is a regression
// about code elimination
unsafe extern "C" {
  fn __this_code_should_have_been_eliminated();
}

#[test]
fn code_elimination_test() {
  if MsgWithNoValidator::HAS_DEFAULT_VALIDATOR
    || OneofWithNoValidator::HAS_DEFAULT_VALIDATOR
    || SecondDegreeRecursion::HAS_DEFAULT_VALIDATOR
  {
    unsafe { __this_code_should_have_been_eliminated() }
  }
}
