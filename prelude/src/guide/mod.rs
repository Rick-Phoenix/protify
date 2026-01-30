//! # Summary

macro_rules! doc_mod {
  ($name:ident) => {
    #[doc = include_str!(concat!(stringify!($name), ".md"))]
    pub mod $name {}
  };
}

doc_mod!(reusing_oneofs);
doc_mod!(server_usage);
doc_mod!(reflection_usage);
doc_mod!(validators);
doc_mod!(correctness);
doc_mod!(proxied);
doc_mod!(package_setup);
doc_mod!(cel_validation);
doc_mod!(roadmap);
doc_mod!(limitations);
doc_mod!(tips);
doc_mod!(reusing_files);
