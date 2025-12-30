mod cel_tests;
mod maps_tests;
mod oneof_tags_tests;
mod rendering_tests;
mod repeated_tests;

use std::collections::HashMap;

use bytes::Bytes;
use prelude::{test_utils::*, *};
use proc_macro_impls::*;
use similar_asserts::assert_eq as assert_eq_pretty;

proto_file!("testing", package = "testing");
