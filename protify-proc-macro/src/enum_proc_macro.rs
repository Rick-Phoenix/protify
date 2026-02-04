use crate::*;

struct EnumVariantCtx {
	name: String,
	options: TokensOr<TokenStream2>,
	tag: i32,
	ident: Ident,
	deprecated: bool,
	span: Span,
}

#[derive(Default)]
struct EnumData {
	variants_data: Vec<EnumVariantCtx>,
	enum_attrs: EnumAttrs,
}

fn extract_enum_data(item: &mut ItemEnum) -> syn::Result<EnumData> {
	let ItemEnum {
		ident: enum_ident,
		variants,
		attrs,
		..
	} = item;

	let enum_attrs = process_derive_enum_attrs(enum_ident, attrs)?;

	let mut variants_data: Vec<EnumVariantCtx> = Vec::new();
	let mut manually_set_tags: Vec<ParsedNum> = Vec::new();

	for variant in variants.iter() {
		if let Some((_, expr)) = &variant.discriminant {
			let num = expr.as_int::<i32>()?;

			manually_set_tags.push(ParsedNum {
				num,
				span: variant.ident.span(),
			});
		}
	}

	let unavailable_ranges =
		build_unavailable_ranges(&enum_attrs.reserved_numbers, &mut manually_set_tags)?;

	let mut tag_allocator = TagAllocator::new(&unavailable_ranges);

	for (i, variant) in variants.iter_mut().enumerate() {
		let variant_ident = &variant.ident;

		if !variant.fields.is_empty() {
			bail!(variant_ident, "Protobuf enums can only have unit variants");
		}

		let EnumVariantAttrs {
			options,
			name,
			deprecated,
		} = process_derive_enum_variants_attrs(&enum_attrs.name, variant_ident, &variant.attrs)?;

		if enum_attrs.reserved_names.contains(&name) {
			bail!(variant_ident, "Name `{name}` is reserved");
		}

		let tag = if let Some((_, expr)) = &variant.discriminant {
			let tag = expr.as_int::<i32>()?;

			if i == 0 && tag != 0 {
				bail!(
					expr,
					"The first variant of a protobuf enum must have have a tag of 0"
				);
			}

			tag
		} else {
			let next_tag = if i == 0 {
				0
			} else {
				tag_allocator.next_tag(variant.ident.span())?.num
			};

			let tag_expr: Expr = parse_quote!(#next_tag);
			variant.discriminant = Some((token::Eq::default(), tag_expr));

			next_tag
		};

		variants_data.push(EnumVariantCtx {
			name,
			options,
			tag,
			ident: variant_ident.clone(),
			deprecated,
			span: variant_ident.span(),
		});
	}

	Ok(EnumData {
		variants_data,
		enum_attrs,
	})
}

pub fn enum_proc_macro(mut item: ItemEnum) -> TokenStream2 {
	let mut error: Option<TokenStream2> = None;

	let EnumData {
		variants_data,
		enum_attrs:
			EnumAttrs {
				reserved_names,
				reserved_numbers,
				options: enum_options,
				parent_message,
				name: proto_name,
				deprecated,
				file,
				module_path,
				..
			},
	} = extract_enum_data(&mut item).unwrap_or_else(|e| {
		error = Some(e.into_compile_error());
		EnumData::default()
	});

	let enum_ident = &item.ident;

	let proto_name_method = if let Some(parent) = &parent_message {
		quote_spanned! {parent.span()=>
		  static __FULL_NAME: ::protify::Lazy<String> = ::protify::Lazy::new(|| {
			::protify::format!("{}.{}", <#parent as ::protify::ProtoMessage>::proto_name(), #proto_name)
		  });

		  &*__FULL_NAME
		}
	} else {
		quote! { #proto_name }
	};

	let parent_message_registry = if let Some(parent) = &parent_message {
		quote_spanned! {parent.span()=> Some(|| <#parent as ::protify::ProtoMessage>::proto_name()) }
	} else {
		quote! { None }
	};

	let rust_ident_str = enum_ident.to_string();

	let variants_tokens = if error.is_some() {
		quote! { unimplemented!() }
	} else {
		let tokens = variants_data.iter().map(|var| {
    let EnumVariantCtx {
      name,
      options,
      tag,
      deprecated,
      span,
      ..
    } = var;

    quote_spanned! {*span=>
      ::protify::EnumVariant { name: #name.into(), options: ::protify::collect_options(#options, #deprecated), tag: #tag, }
    }
  });

		quote! { #(#tokens),* }
	};

	let from_str_impl = if error.is_some() {
		quote! { unimplemented!() }
	} else {
		let tokens = variants_data.iter().map(|var| {
			let EnumVariantCtx {
				name, ident, span, ..
			} = var;

			quote_spanned! {*span=>
			  #name => Some(Self::#ident)
			}
		});

		quote! {
		  match name {
			#(#tokens,)*
			_ => None
		  }
		}
	};

	let as_str_impl = if error.is_some() {
		quote! { unimplemented!() }
	} else {
		let tokens = variants_data.iter().map(|var| {
			let EnumVariantCtx {
				name, ident, span, ..
			} = var;

			quote_spanned! {*span=>
			  Self::#ident => #name
			}
		});

		quote! {
		  match self {
			#(#tokens),*
		  }
		}
	};

	let try_from_impl = if error.is_some() {
		quote! { unimplemented!() }
	} else {
		let tokens = variants_data.iter().map(|var| {
			let EnumVariantCtx {
				tag, ident, span, ..
			} = var;

			quote_spanned! {*span=>
			  #tag => Ok(#enum_ident::#ident)
			}
		});

		quote! {
		  match value {
			#(#tokens,)*
			_ => Err(::protify::prost::UnknownEnumValue(value))
		  }
		}
	};

	let first_variant_ident = &variants_data.first().as_ref().unwrap().ident;

	let file_name = if let Some(ident) = &file {
		quote! { <#ident as ::protify::FileSchema>::NAME }
	} else {
		quote! { __PROTO_FILE.name }
	};

	let package = if let Some(ident) = &file {
		quote! { <#ident as ::protify::FileSchema>::PACKAGE }
	} else {
		quote! { __PROTO_FILE.package }
	};

	let module_path = module_path.as_ref().map_or_else(
		|| {
			if let Some(ident) = &file {
				quote! { <#ident as ::protify::FileSchema>::EXTERN_PATH }
			} else {
				quote! { __PROTO_FILE.extern_path }
			}
		},
		|path_override| path_override.to_token_stream(),
	);

	quote! {
	  #[repr(i32)]
	  #[derive(::protify::macros::Enum, Hash, PartialEq, Eq, Debug, Clone, Copy)]
	  #item

	  ::protify::register_proto_data! {
		::protify::RegistryEnum {
		  parent_message: #parent_message_registry,
		  package: #package,
		  enum_: || <#enum_ident as ::protify::ProtoEnumSchema>::proto_schema()
		}
	  }

	  impl TryFrom<i32> for #enum_ident {
		type Error = ::protify::prost::UnknownEnumValue;

		#[inline]
		fn try_from(value: i32) -> Result<Self, Self::Error> {
		  #try_from_impl
		}
	  }

	  impl Default for #enum_ident {
		#[inline]
		fn default() -> Self {
		  #enum_ident::#first_variant_ident
		}
	  }

	  impl From<#enum_ident> for i32 {
		#[inline]
		fn from(value: #enum_ident) -> i32 {
		  value as i32
		}
	  }

	  impl ::protify::ProtoValidation for #enum_ident {
		#[doc(hidden)]
		type Target = i32;
		#[doc(hidden)]
		type Stored = i32;
		type Validator = ::protify::EnumValidator<#enum_ident>;
		type ValidatorBuilder = ::protify::EnumValidatorBuilder<#enum_ident>;

		#[doc(hidden)]
		type UniqueStore<'a>
		  = ::protify::CopyHybridStore<i32>
		where
		  Self: 'a;

		#[doc(hidden)]
		const HAS_DEFAULT_VALIDATOR: bool = false;
	  }

	  impl ::protify::AsProtoType for #enum_ident {
		fn proto_type() -> ::protify::ProtoType {
		  ::protify::ProtoType::Enum(
			<Self as ::protify::ProtoEnumSchema>::proto_path()
		  )
		}
	  }

	  impl ::protify::ProtoEnum for #enum_ident {
		fn proto_name() -> &'static str {
		  #proto_name_method
		}
	  }

	  impl ::protify::ProtoEnumSchema for #enum_ident {
		fn proto_path() -> ::protify::ProtoPath {
		  ::protify::ProtoPath {
			name: <Self as ::protify::ProtoEnum>::proto_name().into(),
			file: #file_name.into(),
			package: #package.into(),
		  }
		}

		#[inline]
		fn as_proto_name(&self) -> &'static str {
		  #as_str_impl
		}

		#[inline]
		fn from_proto_name(name: &str) -> Option<Self> {
		  #from_str_impl
		}

		fn proto_schema() -> ::protify::EnumSchema {
		  ::protify::EnumSchema {
			short_name: #proto_name.into(),
			name: <Self as ::protify::ProtoEnum>::proto_name().into(),
			file: #file_name.into(),
			package: #package.into(),
			variants: ::protify::vec! [ #variants_tokens ],
			reserved_names: ::protify::vec![ #(#reserved_names.into()),* ],
			reserved_numbers: #reserved_numbers,
			options: ::protify::collect_options(#enum_options, #deprecated),
			rust_path:  ::protify::format!("::{}::{}", #module_path, #rust_ident_str).into()
		  }
		}
	  }

	  #error
	}
}
