use crate::*;

#[derive(Default, Clone, Debug)]
pub(crate) struct ReservedNumbers(pub Vec<Range<i32>>);

pub const PROTOBUF_MAX_TAG: i32 = 536_870_911;

impl ReservedNumbers {
  pub fn build_unavailable_ranges(self, manual_tags: Vec<i32>) -> Self {
    if manual_tags.is_empty() {
      return self;
    }

    let mut ranges = self.0;

    for tag in manual_tags {
      ranges.push(tag..(tag + 1));
    }

    ranges.sort_by_key(|r| r.start);

    // Coalesce
    let mut merged: Vec<Range<i32>> = Vec::new();
    let mut current = ranges[0].clone();

    for next in ranges.into_iter().skip(1) {
      if next.start < current.end {
        panic!("Using a taken tag");
      } else if next.start == current.end {
        // Extend current to the max end
        current.end = std::cmp::max(current.end, next.end);
      } else {
        // Gap found, push current and start new
        merged.push(current);
        current = next;
      }
    }
    merged.push(current);

    Self(merged)
  }
}

impl ToTokens for ReservedNumbers {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let mut agg_tokens = TokenStream2::new();

    for range in &self.0 {
      let start = range.start;
      let end = range.end;

      agg_tokens.extend(quote! {
        #start..#end,
      });
    }

    tokens.extend(agg_tokens);
  }
}

impl Parse for ReservedNumbers {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut ranges: Vec<Range<i32>> = Vec::new();

    let items = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

    for item in items {
      if let Expr::Range(range_expr) = &item {
        let start = if let Some(start_expr) = &range_expr.start {
          extract_i32(start_expr)?
        } else {
          0
        };

        let end = if let Some(end_expr) = &range_expr.end {
          extract_i32(end_expr)?
        } else {
          PROTOBUF_MAX_TAG + 1
        };

        let final_end = if let RangeLimits::HalfOpen(_) = &range_expr.limits {
          end
        } else {
          end + 1
        };

        ranges.push(start..final_end);
      } else if let Expr::Lit(lit) = &item && let Lit::Int(lit_int) = &lit.lit {
        let num = lit_int.base10_parse::<i32>()?;

        ranges.push(num..num + 1);
      } else {
        return Err(spanned_error!(
          item,
          "Expected a range (e.g. `1..5`, `10..=15`)"
        ));
      }
    }

    ranges.sort_by_key(|range| range.start);

    Ok(Self(ranges))
  }
}
