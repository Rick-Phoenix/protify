use std::cmp::Ordering;

use crate::*;

pub struct TagAllocator<'a> {
  pub unavailable: &'a [Range<i32>],
  pub reserved_to_max: bool,
  pub next_tag: i32,
}

pub const PROTOBUF_MAX_TAG: i32 = 536_870_911;

impl<'a> TagAllocator<'a> {
  pub fn new(unavailable: &'a [Range<i32>]) -> Self {
    let reserved_to_max = unavailable
      .last()
      .is_some_and(|last| last.end > PROTOBUF_MAX_TAG + 1);

    Self {
      unavailable,
      next_tag: 1,
      reserved_to_max,
    }
  }

  pub fn tag_is_unavailable(&self, number: &i32) -> bool {
    let result = self.unavailable.binary_search_by(|range| {
      if range.contains(number) {
        Ordering::Equal
      } else if *number < range.start {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    });

    result.is_ok()
  }

  #[track_caller]
  pub fn next_tag(&mut self) -> i32 {
    loop {
      let idx = self.unavailable.partition_point(|r| r.end <= self.next_tag);

      if let Some(range) = self.unavailable.get(idx)
        && range.contains(&self.next_tag) {
          self.next_tag = range.end;
          continue;
        }

      if self.reserved_to_max {
        panic!("Protobuf tag limit exceeded! No available tags left.");
      }

      let tag = self.next_tag;
      self.next_tag += 1;
      return tag;
    }
  }

  #[track_caller]
  pub fn get_or_next(&mut self, manual_tag: Option<i32>) -> i32 {
    if let Some(tag) = manual_tag {
      return tag;
    }
    self.next_tag()
  }
}
