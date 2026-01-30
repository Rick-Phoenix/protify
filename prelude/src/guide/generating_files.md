# Generating Files

When the `inventory` feature is enabled, generating files is done in a single step. You just take the package handle created by the [`proto_package`](crate::proto_package) macro, and call the [`render_files`](crate::Package::render_files) method with the desided root directory of the package, and that's it.

# No_std usage

The inventory feature relies on the `inventory` crate being available, which is not the case in a no_std crate, and may also be undesirable for users with extreme binary size concerns, as the registry collection applied with inventory may increase the binary size slightly. 

In cases such as these, we need a different method for collecting the schema items, which can use one of the following workarounds.

1. You can create a utility crate in your workspace which builds your no_std crate, but enables the `inventory` feature. In this case, the inventory functionalities will work normally, all the items will be picked up automatically like in a std-compatible crate and you will be able to generate the files with a single method call as described above.
This is in theory the simplest way, but there is a big catch. Since rust-analyzer compiles only one version per crate in a workspace (as of today), if you apply this workaround, your IDE will show all sorts of errors, because it will apply the checks to the version of the crate that is using the inventory feature, even if the actual crate is not using the feature. You can check this out by cloning the repo and looking inside the test-no-std crate which is where I tried applying exactly this. If you can find a workaround for this issue, then this should be the way to go.

2. The alternative is to use the [`file_schema`](crate::file_schema) and [`package_schema`](crate::package_schema) macros, which allow you to manually define the elements of a package.

The [`file_schema`](crate::file_schema) macro accepts all the inputs of the [`define_proto_file`](crate::define_proto_file) macro, plus the list of messages, enums and services, which are just bracketed lists of paths for each element.
Nested messages and enums are defined by using `ParentMessage = { enums = [ NestedEnum ], messages = [ NestedMsg ] }` instead of just the message's name, as shown below.

The [`package_schema`](crate::package_schema) macro simply accepts the name of the package as the first argument, and a bracketed list of idents for the files that it contains.

# Example:

```rust
use prelude::*;

// This would be the no_std crate where you define your models...
mod imagine_this_is_the_models_crate {
  use super::*;
  
  // The package and file handles are still needed here,
  // but they do not collect the items automatically
  // when the inventory feature is disabled, so we must
  // create the schemas manually below...
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


