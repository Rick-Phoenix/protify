//! # Guide
//!
//! 1. [Creating A Package](crate::guide::package_setup)

macro_rules! doc_mod {
  ($name:ident) => {
    #[doc = include_str!(concat!(stringify!($name), ".md"))]
    pub mod $name {}
  };
}

doc_mod!(code_elimination);
doc_mod!(reusing_oneofs);
doc_mod!(usage_with_tonic);
doc_mod!(reflection_usage);
doc_mod!(validators);
doc_mod!(correctness);
doc_mod!(proxies);
doc_mod!(package_setup);
doc_mod!(cel_validation);
doc_mod!(roadmap);
doc_mod!(limitations);
doc_mod!(tips);
doc_mod!(managing_files);
doc_mod!(database_mapping);
