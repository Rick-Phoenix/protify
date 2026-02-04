use crate::*;

pub fn fallback_derive_impls(target_ident: &Ident, kind: ItemKind) -> TokenStream2 {
  let mut output = quote! {
    impl Default for #target_ident {
      fn default() -> Self {
        unimplemented!()
      }
    }

    impl Clone for #target_ident {
      fn clone(&self) -> Self {
        unimplemented!()
      }
    }

    impl std::fmt::Debug for #target_ident {
      fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
      }
    }

    impl PartialEq for #target_ident {
      fn eq(&self, _: &Self) -> bool {
        unimplemented!()
      }
    }
  };

  output.extend(match kind {
      ItemKind::Oneof => {
        quote! {
          impl #target_ident {
            pub fn encode(&self, buf: &mut impl ::protify::prost::bytes::BufMut) {
              unimplemented!()
            }

            pub fn merge(
              _: &mut ::core::option::Option<Self>,
              _: u32,
              _: ::protify::prost::encoding::wire_type::WireType,
              _: &mut impl ::protify::prost::bytes::Buf,
              _: ::protify::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::protify::prost::DecodeError>
            {
              unimplemented!()
            }

            pub fn encoded_len(&self) -> usize {
              unimplemented!()
            }
          }
        }
      }
      ItemKind::Message => {
        quote! {
          impl ::protify::prost::Message for #target_ident {
            fn encoded_len(&self) -> usize {
              unimplemented!()
            }

            fn encode_raw(&self, _: &mut impl ::protify::prost::bytes::buf::BufMut) { unimplemented!() }

            fn merge_field(&mut self, _: u32, _: prost::encoding::WireType, _: &mut impl ::protify::prost::bytes::buf::Buf, _: prost::encoding::DecodeContext) -> std::result::Result<(), prost::DecodeError> { unimplemented!() }

            fn clear(&mut self) {}
          }
        }
      }
    }
    );

  if cfg!(feature = "cel") {
    output.extend(match kind {
        ItemKind::Oneof => quote! {
          impl ::protify::CelOneof for #target_ident {
            #[doc(hidden)]
            fn try_into_cel(self) -> Result<(String, ::protify::cel::Value), ::protify::proto_types::cel::CelConversionError> {
              unimplemented!()
            }
          }

          impl TryFrom<#target_ident> for ::protify::cel::Value {
            type Error = ::protify::proto_types::cel::CelConversionError;

            #[inline]
            fn try_from(value: #target_ident) -> Result<Self, Self::Error> {
              unimplemented!()
            }
          }
        },
        ItemKind::Message => quote! {
          impl ::protify::CelValue for #target_ident {}

          impl TryFrom<#target_ident> for ::protify::cel::Value {
            type Error = ::protify::proto_types::cel::CelConversionError;

            fn try_from(value: #target_ident) -> Result<Self, Self::Error> {
              unimplemented!()
            }
          }
        },
      });
  }

  output
}
