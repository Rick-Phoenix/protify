# Tips

- CEL expressions should always target the name of the fields in rust, not those of the message. For example, if a field is named `type` in protobuf, the output struct will have a field named `r#type`, and your cel expression should refer to it using `this['r#type']`, NOT `this.type`.

- If you use a [`BTreeMap`](std::collections::BTreeMap) in one of your fields, you need to add `prost` as a dependency and cannot rely on the re-export from this crate, due to a bug in `prost`'s macro output for BTreeMaps.
