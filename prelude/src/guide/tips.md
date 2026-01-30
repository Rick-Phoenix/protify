# Tips

- If your message has a reserved rust keyword as a field name, your cel expression should reflect that. So if a field is named `type` in protobuf, the output struct will have a field named `r#type`, and your cel expression should refer to it using `this['r#type']`, NOT `this.type`.
