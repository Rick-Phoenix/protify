# Correctness

This crate tries to enforce correctness as at compile/testing time, so that badly set up validators do not make their way into runtime.

The [`Validator`](crate::Validator) trait holds a method called [`check_consistency`](crate::Validator::check_consistency) which checks if the inputs inside of it make sense or were caused by an error. All provided validators implement this method and perform a variety of checks on their inputs, and all custom validators can optionally implement it too. For example, the [`StringValidator`](crate::StringValidator) will:

- Check if `max_len` is less than `min_len`
- Check if `prefix` or `suffix` match the `not_contains` rule
- Check if an item is both in the lists of allowed and forbidden values
- ...and many others

Whereas the [`EnumValidator`](crate::EnumValidator) will perform checks like ensuring that all the values in the `in` rule are actually defined in the target enum.

All validators as a whole will:

- Check the validity of CEL expressions used in them
- Check if a custom error message is defined but never used
- Check if the `const` rule is used among with other rules (which would be ignored)

There is a particular emphasis on checking the validity of CEL expressions because they are written as plain text and are very easy to get wrong, and they **will panic** at initialization if they fail to compile.

Unless specified otherwise with the `skip_checks` attribute, the macros will generate a test that calls this method for every validator assigned to a message/oneof, and panics on failure.
