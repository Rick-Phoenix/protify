# Reusing Files

If you want to define items in different rust files that are descendants of the same module, and place them into the same proto file, you can use one of two macros to bring the file into scope.

- The [`use_proto_file`](crate::use_proto_file) macro, which brings the file into scope and applies the extern path of its own `module_path!()` output
- The [`inherit_proto_file`](crate::inherit_proto_file) macro, which does the same but keeps the import path of the parent module (for re-exported items).

```rust
mod root {
    use prelude::*;

    proto_package!(MY_PKG, name = "my_pkg");
    define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);

    pub mod submod {
        use super::*;
        
        // The file is now in scope, and 
        // will be picked up automatically 
        // by all items defined in this module
        use_proto_file!(MY_FILE);

        // This message will have the extern path
        // of the `module_path!()` output in here, 
        // so `::cratename::submod`
        #[proto_message]
        pub struct Msg {
           pub id: i32
        }
    }

    pub use re_exported::Msg;

    mod re_exported {
        use super::*;

        // The file is now in scope, and 
        // will be picked up automatically by 
        // all items defined in this module
        inherit_proto_file!(MY_FILE);

        // This message will have the extern path
        // of the parent module
        #[proto_message]
        pub struct Msg {
            pub id: i32
        }
    }
}
```

# ⚠️ Caveats with glob imports

Under the hood, the file macros generate a constant named `__PROTO_FILE` that is picked up by the macro output for each item. This constant is private to the module and hidden so that it cannot be brought into scope accidentally, but using glob imports from children modules like `use super::*` will bring it into scope. 

It's not recommended to rely on this method to bring the file into scope and to use the [`use_proto_file`](crate::use_proto_file) or [`inherit_proto_file`](crate::inherit_proto_file) macros for more clarity.

In a case where you need to use a global import from the parent module but you want the items to be in a separate proto file, then you must make sure to define a new file with the [`define_proto_file`](crate::define_proto_file) macro, or the items will be picked up by the wrong file.
