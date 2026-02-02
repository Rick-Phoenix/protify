use crate::*;

pub fn field_schema_tokens(data: &FieldData) -> TokenStream2 {
  let FieldData {
    tag,
    validators,
    options,
    proto_name,
    proto_field,
    deprecated,
    span,
    ..
  } = data;

  let validator_schema_tokens = validators
    .iter()
    // For default validators (messages only) we skip the schema generation
    .filter(|v| !v.kind.is_default())
    .map(|e| {
      let validator_target_type = proto_field.validator_target_type(*span);

      quote_spanned! {*span=>
        ::protify::Validator::<#validator_target_type>::schema(&#e)
      }
    });

  if let ProtoField::Oneof(OneofInfo { path, .. }) = proto_field {
    quote_spanned! {*span=>
      ::protify::MessageEntry::Oneof(
        <#path as ::protify::ProtoOneof>::proto_schema()
          .with_name(#proto_name)
          .with_options(#options)
          .with_validators(::protify::filter_validators([ #(#validator_schema_tokens),* ]))
      )
    }
  } else {
    let field_type_tokens = proto_field.proto_field_target_type(*span);

    quote_spanned! {*span=>
      ::protify::Field {
        name: #proto_name.into(),
        tag: #tag,
        options: ::protify::collect_options(#options, #deprecated),
        type_: #field_type_tokens,
        validators: ::protify::collect_validators([ #(#validator_schema_tokens),* ]),
      }
    }
  }
}

impl MessageCtx<'_> {
  pub fn generate_schema_impls(&self) -> TokenStream2 {
    let MessageAttrs {
      reserved_names,
      reserved_numbers,
      options: message_options,
      name: proto_name,
      parent_message,
      deprecated,
      validators,
      file,
      module_path,
      ..
    } = &self.message_attrs;

    let entries_tokens = if self.fields_data.is_empty() {
      quote! { unimplemented!() }
    } else {
      let tokens = self
        .fields_data
        .iter()
        .filter_map(|d| d.as_normal())
        .map(|data| {
          let field = field_schema_tokens(data);

          if data.proto_field.is_oneof() {
            field
          } else {
            quote_spanned! {data.span=>
              ::protify::MessageEntry::Field(
                #field
              )
            }
          }
        });

      quote! { #(#tokens),* }
    };

    let proto_struct = self.proto_struct_ident();

    let name_method = if let Some(parent) = parent_message {
      quote_spanned! {parent.span()=>
        static __NAME: ::protify::Lazy<String> = ::protify::Lazy::new(|| {
          format!("{}.{}", <#parent as ::protify::ProtoMessage>::proto_name(), #proto_name)
        });

        &*__NAME
      }
    } else {
      quote! { #proto_name }
    };

    let registry_parent_message = if let Some(parent) = parent_message {
      quote_spanned! {parent.span()=> Some(|| <#parent as ::protify::ProtoMessage>::proto_name()) }
    } else {
      quote! { None }
    };

    let rust_ident_str = proto_struct.to_string();

    let proxy_struct_impl = self
      .shadow_struct_ident
      .map(|shadow_struct_ident| {
        let orig_struct_ident = &self.orig_struct_ident;

        quote_spanned! {orig_struct_ident.span()=>
          impl ::protify::AsProtoType for #orig_struct_ident {
            fn proto_type() -> ::protify::ProtoType {
              <#shadow_struct_ident as ::protify::AsProtoType>::proto_type()
            }
          }
        }
      });

    let file_name = if let Some(ident) = file {
      quote! { <#ident as ::protify::FileSchema>::NAME }
    } else {
      quote! { __PROTO_FILE.name }
    };

    let package = if let Some(ident) = file {
      quote! { <#ident as ::protify::FileSchema>::PACKAGE }
    } else {
      quote! { __PROTO_FILE.package }
    };

    let module_path = module_path.as_ref().map_or_else(
      || {
        if let Some(ident) = file {
          quote! { <#ident as ::protify::FileSchema>::EXTERN_PATH }
        } else {
          quote! { __PROTO_FILE.extern_path }
        }
      },
      |path_override| path_override.to_token_stream(),
    );

    quote! {
      ::protify::register_proto_data! {
        ::protify::RegistryMessage {
          package: #package,
          parent_message: #registry_parent_message,
          message: || <#proto_struct as ::protify::ProtoMessage>::proto_schema()
        }
      }

      impl ::protify::AsProtoType for #proto_struct {
        fn proto_type() -> ::protify::ProtoType {
          ::protify::ProtoType::Message(
            <Self as ::protify::MessagePath>::proto_path()
          )
        }
      }

      impl ::protify::prost::Name for #proto_struct {
        #[doc(hidden)]
        const PACKAGE: &str = <Self as ::protify::ProtoMessage>::PACKAGE;
        #[doc(hidden)]
        const NAME: &str = <Self as ::protify::ProtoMessage>::SHORT_NAME;

        #[doc(hidden)]
        fn full_name() -> ::protify::String {
          <Self as ::protify::ProtoMessage>::full_name().into()
        }

        #[doc(hidden)]
        fn type_url() -> ::protify::String {
          <Self as ::protify::ProtoMessage>::type_url().into()
        }
      }

      impl ::protify::MessagePath for #proto_struct {
        fn proto_path() -> ::protify::ProtoPath {
          ::protify::ProtoPath {
            name: <Self as ::protify::ProtoMessage>::proto_name().into(),
            file: #file_name.into(),
            package: #package.into(),
          }
        }
      }

      impl ::protify::ProtoMessage for #proto_struct {
        const PACKAGE: &str = #package;
        const SHORT_NAME: &str = #proto_name;

        fn type_url() -> &'static str {
          static URL: ::protify::Lazy<String> = ::protify::Lazy::new(|| {
            format!("/{}.{}", <#proto_struct as ::protify::ProtoMessage>::PACKAGE, <#proto_struct as ::protify::ProtoMessage>::proto_name())
          });

          &*URL
        }

        fn full_name() -> &'static str {
          static NAME: ::protify::Lazy<String> = ::protify::Lazy::new(|| {
            format!("{}.{}", <#proto_struct as ::protify::ProtoMessage>::PACKAGE, <#proto_struct as ::protify::ProtoMessage>::proto_name())
          });

          &*NAME
        }

        fn proto_name() -> &'static str {
          #name_method
        }

        fn proto_schema() -> ::protify::MessageSchema {
          ::protify::MessageSchema {
            short_name: #proto_name.into(),
            name: <Self as ::protify::ProtoMessage>::proto_name().into(),
            file: #file_name.into(),
            package: #package.into(),
            reserved_names: vec![ #(#reserved_names.into()),* ],
            reserved_numbers: #reserved_numbers,
            options: ::protify::collect_options(#message_options, #deprecated),
            messages: vec![],
            enums: vec![],
            entries: vec![ #entries_tokens ],
            validators: ::protify::collect_validators([ #(::protify::Validator::<#proto_struct>::schema(&#validators)),* ]),
            rust_path:  format!("::{}::{}", #module_path, #rust_ident_str).into()
          }
        }
      }

      #proxy_struct_impl
    }
  }
}
