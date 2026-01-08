[working-directory(".")]
test-all: test-shared-schemas
    cargo test --workspace --exclude proc-macro-impls --exclude test-server -- --nocapture

test-schemas:
    cargo test -p testing -- --nocapture

test-shared-schemas: gen-schemas
    cargo test -p test-reflection -- --nocapture
    cargo test --features reflection -p test-reflection -- --nocapture

gen-schemas:
    cargo run --bin test-schemas

[working-directory(".")]
expand-reflection: gen-schemas
    cargo expand --features reflection -p test-reflection > expanded.rs

test-renders:
    cargo test -p testing test_renders -- -q --nocapture

[working-directory("testing")]
build-protos:
    cargo run -p testing

build-server:
    cargo build -p test-server

test:
    cargo test --all-features -- -q --nocapture

update-changelog version:
    git cliff --tag {{ version }}
    git add "CHANGELOG.md"
    git commit -m "updated changelog"

release-test version: test
    cargo release {{ version }} -p protoschema

release-exec version: test (update-changelog version)
    cargo release {{ version }} -p protoschema --execute

build-docs:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open
