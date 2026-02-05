# Interacting With Databases

An important benefit that comes from having a "rust-first" approach when defining our models is that they can easily be used for operations such as db queries, without needing to create separate structs to map to the generated protos, or injecting the attributes as plain text with the `prost-build` helper, which can be unergonomic and brittle.

And with proxies, the interactions with a database becomes even easier, because we can have the proto-facing struct with a certain shape, while the proxy can represent the state of a message after its data has been mapped, for example, to an item queried from the database.

You can take a look at the [test-server](https://github.com/Rick-Phoenix/protify/tree/main/test-server) crate in the repo for an example of database interaction in a `tonic` handler.

```rust
use diesel::prelude::*;
use protify::proto_types::Timestamp;
use protify::*;

proto_package!(DB_TEST, name = "db_test", no_cel_test);
define_proto_file!(DB_TEST_FILE, name = "db_test.proto", package = DB_TEST);

mod schema {
	diesel::table! {
	  users {
		id -> Integer,
		name -> Text,
		created_at -> Timestamp
	  }
	}
}

// If we want to use the message as is for the db model
#[proto_message]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
	#[diesel(skip_insertion)]
	pub id: i32,
	pub name: String,
	#[diesel(skip_insertion)]
	// We need this to keep `Option` for this field
	// which is necessary for protobuf
	#[diesel(select_expression = schema::users::columns::created_at.nullable())]
	#[proto(timestamp)]
	pub created_at: Option<Timestamp>,
}

// If we want to use the proxy as the db model, for example
// to avoid having `created_at` as `Option`
#[proto_message(proxied)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProxiedUser {
	#[diesel(skip_insertion)]
	pub id: i32,
	pub name: String,
	#[diesel(skip_insertion)]
	#[proto(timestamp, from_proto = |v| v.unwrap_or_default())]
	pub created_at: Timestamp,
}

fn main() {
	use schema::users::dsl::*;

	let conn = &mut SqliteConnection::establish(":memory:").unwrap();

	let table_query = r"
    CREATE TABLE users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      name TEXT NOT NULL,
      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
      );
    ";

	diesel::sql_query(table_query)
		.execute(conn)
		.expect("Failed to create the table");

	let insert_user = User {
		id: 0,
		name: "Gandalf".to_string(),
		created_at: None,
	};

	diesel::insert_into(users)
		.values(&insert_user)
		.execute(conn)
		.expect("Failed to insert user");

	let queried_user = users
		.filter(id.eq(1))
		.select(User::as_select())
		.get_result(conn)
		.expect("Failed to query user");

	assert_eq!(queried_user.id, 1);
	assert_eq!(queried_user.name, "Gandalf");
	// The timestamp will be populated by the database upon insertion
	assert_ne!(queried_user.created_at.unwrap(), Timestamp::default());

	let proxied_user = ProxiedUser {
		id: 0,
		name: "Aragorn".to_string(),
		created_at: Default::default(),
	};

	diesel::insert_into(users)
		.values(&proxied_user)
		.execute(conn)
		.expect("Failed to insert user");

	let queried_proxied_user = users
		.filter(id.eq(2))
		.select(ProxiedUser::as_select())
		.get_result(conn)
		.expect("Failed to query user");

	assert_eq!(queried_proxied_user.id, 2);
	assert_eq!(queried_proxied_user.name, "Aragorn");

	// Now we have the message, with the `created_at` field populated
	let msg = queried_proxied_user.into_message();

	assert_ne!(msg.created_at.unwrap(), Timestamp::default());
}
```
