# Roadmap

## Planned Features

- Support for the protobuf crate
    The protobuf crate by google is currently in beta. Once its api becomes stable, I will look into how I can make this library work with it.

## Unplanned Features

- Support for validation for `Vec<u8>`. 
    Supporting both that and `Bytes` would increase the complexity of the macro handling, and it would also cause the BytesValidator to be implemented for multiple types with the same targets, which would make is less ergonomic to use without having to rely on the fully qualified trait syntax. 

- Async Validators
    I have played around with this idea and I think that the extra complexity is just not worth it.
    More generally speaking, I think that async validation should be handled by a general-purpose validation library such as this one, because async validation is, by definition, very custom business logic (a db query or an API call) that should be handled on a per-case basis and should not mix with regular validation that relies on static values. I may change my mind about this in the future but at the moment I think that the amount of extra code and complexity that would be added to the crate is not worth it.

- Refs In Proxies
    At the moment, the proxies are simply meant to be a utility interface for the prost-compatible types. Doing the mapping in such a way cuts a huge portion of complexity and allows me to implement a lot of methods and traits without needing to worry if the target struct/enum has lifetimes or not. Since prost messages always own their values anyway, the value of using a proxy with references is in my opinion not that great.

    However, we know that the protobuf crate by google will rely almost exclusively on references, so once it becomes stable and I try to find a way to adapt this library to work with that, supporting lifetimes and references might become a more worthy investment.

- Support for pseudo-oneofs in `(buf.validate.message).oneof` style
