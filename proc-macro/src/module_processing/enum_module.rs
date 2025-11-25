use crate::*;

pub fn process_enum_from_module(
  enum_data: &mut EnumData,
  parent_message: Option<String>,
  package_attribute: &Attribute,
) -> Result<(), Error> {
  let EnumData {
    name: proto_name,
    reserved_numbers,
    variants,
    used_tags,
    tokens,
  } = enum_data;

  let reserved_numbers = std::mem::take(reserved_numbers);

  let taken_tags = reserved_numbers.build_unavailable_ranges(used_tags);

  let mut tag_allocator = TagAllocator::new(&taken_tags);

  let repr_attr: Attribute = parse_quote!(#[repr(i32)]);
  tokens.attrs.push(repr_attr);

  for variant in variants {
    if variant.tag.is_none() {
      let tag = tag_allocator.next_tag();

      variant.tag = Some(tag);

      let variant_attr: Attribute = parse_quote!(#[proto(tag = #tag)]);

      variant.inject_attr(variant_attr);
    }

    let tag = variant.tag.unwrap();
    let tag_expr: Expr = parse_quote!(#tag);
    variant.tokens.discriminant = Some((token::Eq::default(), tag_expr));
  }

  if let Some(parent_message) = parent_message {
    let full_name = format!("{parent_message}.{proto_name}");

    let full_name_attr: Attribute = parse_quote!(#[proto(full_name = #full_name)]);
    enum_data.inject_attr(full_name_attr);
  }

  enum_data.inject_attr(package_attribute.clone());

  Ok(())
}
