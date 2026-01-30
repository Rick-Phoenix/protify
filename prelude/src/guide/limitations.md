# Limitations

- Usage of protobuf `bytes` fields implies usage of the [`Bytes`](::bytes::Bytes) type in Rust, rather than `Vec<u8>`. Supporting both would increase the complexity of the macros and create more trait indirection for the bytes validator. Since [`Bytes`](::bytes::Bytes) is a tiny, widely used and very convenient dependency to have, I've decided to target that directly.

- Usage of `Box` in a oneof or message is automatically implied as a sign of recursion and handled accordingly in the macros' logic. Using `Box` for non-recursive fields or variants is not supported.

- The schema features are not heavily optimized for runtime usage, and they are intended to be used merely for generating files, not as a substitute for a full-fledged reflection library. I can recommend the excellent [`prost-reflect`](https://crates.io/crates/prost-reflect) for that purpose.

- Proxy structs/enums do not support generics or lifetimes.

Refer to the [roadmap](crate::guide::roadmap) to learn more about planned and unplanned features.
