//! # Guide
//!
//! This is the guide for `protify`.
//!
//!
//! ## Setup
//! - [Setup with full schema features](crate::guide::package_setup)
//! - [Setup with pre-built protos (validation only)](crate::guide::reflection_usage)
//! - [Managing files](crate::guide::managing_files)
//!	## Features
//!	- [Validators](crate::guide::validators)
//!	- [Proxies](crate::guide::proxies)
//!	- [Reusing Oneofs](crate::guide::reusing_oneofs)
//!	- [Correctness](crate::guide::correctness)
//!	- [CEL validation](crate::guide::cel_validation)
//!	## Usage Examples
//!	- [Database Mapping](crate::guide::database_mapping)
//!	- [Usage With Tonic](crate::guide::usage_with_tonic)
//!	## Things To Know
//!	- [Tips](crate::guide::tips)
//!	- [Code elimination](crate::guide::code_elimination)
//!	- [Limitations](crate::guide::limitations)
//!	- [Roadmap](crate::guide::roadmap)

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
