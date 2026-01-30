# Creating A Package

Start by creating a package using the [`proto_package`](crate::proto_package) macro.

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
```

The ident that is used as the first argument will be the ident for this [`PackageHandle`](crate::PackageHandle) instance. There should be only one of such an instance for each package.

ℹ️ **NOTE**: By default, this macro also generates a test that ensures that there aren't CEL rules with the same ID within the same message scope. If you want to disable this, you can provide `no_cel_test` as one of the arguments. 

This handle will serve two purposes. One is to serve as a reference for each file definition, and the other one, which is only fullfilled when the `inventory` feature is enabled, is to be able to generate the fully built [`Package`](crate::Package) struct, which can then be used to generate the `.proto` files, access the `extern_path` of each item (to use when setting up tonic), and so on.

# Creating Files

For each item definition (oneof, message, enum), a proto file reference must be brought in scope so that the item can be assigned to it. A new file can be defined with the [`define_proto_file`](crate::define_proto_file) macro.

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
```

Once a file is brought into scope, we can define the items that will be assigned to it.

For more information on how to reuse the same file for different children of the same module, refer to the [`reusing_files`](crate::guide::reusing_files) section.

```rust
use prelude::*;

proto_package!(MY_PKG, name = "my_pkg");
define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG, extensions = [ MyExt ]);

#[proto_extension(target = MessageOptions)]
pub struct MyExt {
  #[proto(tag = 5000)]
  cool_opt: String
}

#[proto_service]
enum MyService {
    Service1 {
        request: MyMsg,
        response: MyMsg
    }
}

#[proto_message]
pub struct MyMsg {
    pub id: i32
}

#[proto_oneof]
pub enum MyOneof {
    #[proto(tag = 1)]
    A(i32),
    #[proto(tag = 2)]
    B(u32),
}

#[proto_enum]
pub enum MyEnum {
    Unspecified,
    A,
    B
}
```

To learn more about each element, visit the reference for messages, extensions, oneofs

# Collecting A Package

When the `inventory` feature is enabled, all the elements in a package are collected automatically. To get the fully built package, all you need to do is to call the [`get_package`](crate::PackageHandle::get_package) method, and that's it.

This will give you access to the [`render_files`](crate::Package::render_files) method, which is what you need to generate the `.proto` files associated with that package, or the [`extern_paths`](crate::Package::extern_paths) method, which is what you can use to map messages to their Rust path when using `tonic`.

However, since this relies on the `inventory` crate which is not available in a `no_std` environment, if we are in such a scenario we need to use a manual workaround to collect the full package.

## No_std usage

The inventory feature relies on the `inventory` crate being available, which is not the case in a no_std crate, and may also be undesirable for users with extreme binary size concerns, as the registry collection applied with inventory may increase the binary size slightly. 

In cases such as these, we need a different method for collecting the schema items, which can use one of the following workarounds.

1. You can create a utility crate in your workspace which builds your no_std crate, but adds this crate as a dependency and enables the `inventory` feature. In this case, the inventory functionalities will work normally, all the items will be picked up automatically like in a std-compatible crate and you will be able to generate the files with a single method call as described above.
This is in theory the simplest way, but there is a big catch. <br/> Since rust-analyzer compiles only one version per crate in a workspace (as of today), if you apply this workaround, your IDE will show all sorts of errors, because it will apply the checks to the version of the crate that is using the inventory feature, even if the actual crate is not using the feature. You can check this out by cloning the repo and looking inside the test-no-std crate which is where I tried applying exactly this. If you can find a workaround for this issue, then this is the easiest way to go.

2. You can use the [`file_schema`](crate::file_schema) and [`package_schema`](crate::package_schema) macros, which allow you to manually define the elements of a package.

The [`file_schema`](crate::file_schema) macro accepts all the inputs of the [`define_proto_file`](crate::define_proto_file) macro, plus the list of messages, enums and services, which are just bracketed lists of paths for each element.
Nested messages and enums are defined by using `ParentMessage = { enums = [ NestedEnum ], messages = [ NestedMsg ] }` instead of just the message's name, as shown below.

The [`package_schema`](crate::package_schema) macro simply accepts the name of the package as the first argument, and a bracketed list of idents for the files that it contains.

### Example of manual definition

```rust
use prelude::*;

// This would be the no_std crate where you define your models...
mod imagine_this_is_the_models_crate {
  use super::*;
  
  // The package and file handles are still needed here,
  // but they do not collect the items automatically
  // when the inventory feature is disabled, so we must
  // create the schemas manually like we do below...
  proto_package!(MY_PKG, name = "my_pkg");
  define_proto_file!(MY_FILE, name = "my_file.proto", package = MY_PKG);
  
  #[proto_message]
  pub struct Msg1 {
    pub id: i32
  }
  #[proto_message]
  #[proto(parent_message = Msg1)]
  pub struct Nested {
    pub id: i32
  }
  #[proto_message]
  pub struct Msg2 {
    pub id: i32
  }
  #[proto_enum]
  pub enum Enum1 {
    Unspecified, A, B
  }
  #[proto_enum]
  #[proto(parent_message = Msg1)]
  pub enum NestedEnum {
    Unspecified, A, B
  }
  #[proto_service]
  pub enum MyService {
    GetMsg {
      request: Msg1,
      response: Msg2
    }
  }
}

// From an external utility crate, or the build.rs file of the consuming crate:
fn main() {
  use imagine_this_is_the_models_crate::*;
  let manual_file = file_schema!(
    name = "test.proto",
    messages = [
      Msg2,
      Msg1 = { messages = [ Nested ], enums = [ NestedEnum ] }
    ],
    services = [ MyService ],
    enums = [ Enum1 ],
    // Imports, options, etc...
  );
  let manual_pkg = package_schema!("my_pkg", files = [ manual_file ]);
  // Now we can use the package handle to create the files,
  // access the `extern_path`s and so on...
}
```
