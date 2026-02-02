# Maximizing Code Elimination

If you were to use `cargo expand` on the [`proto_message`](crate::proto_message) or [`proto_oneof`](crate::proto_oneof), you'll realize that the [`ProtoValidation`](crate::ProtoValidation), [`ValidatedMessage`](crate::ValidatedMessage) and [`ValidatedOneof`](crate::ValidatedOneof) traits use associated constants to check if oneofs and messages have validators in them or not.

This is done with the intent of maximizing code elimination, so that the `.validate()` method will be completely eliminated by the compiler if a oneof/message has no validators, and any call to it will also be eliminated.

This is very important because it allows us to implement these traits freely without hurting performance or binary size, and it also means that we can make blanket calls to .validate() for all incoming messages, because if some of those messages do not have validators, that call will be a no-op and it will be eliminated by the compiler entirely.

The crate has a dedicated test for this purpose, which checks the assembly output of a function that calls an empty validator.
