gen-readme:
    cargo run -p gen-readme

check-asm-output:
    cargo asm --release -p testing --test code_elimination trigger_validation

[working-directory(".")]
test-all: test-shared-schemas test-schemas test-no-std test-proc-macro test-code-elimination
    cargo test -p prelude -- --nocapture

test-code-elimination:
    cargo test -p testing --release code_elimination

test-proc-macro:
    cargo test -p proc-macro-impls -- --nocapture

test-no-std:
    cargo test --features reflection -p test-no-std -- --nocapture
    cargo test -p test-no-std -- --nocapture

test-schemas:
    cargo test -p testing -- --nocapture
    cargo test -p test-schemas -- --nocapture

test-shared-schemas: gen-schemas
    cargo test -p test-reflection -- --nocapture
    cargo test --features reflection -p test-reflection -- --nocapture

gen-schemas:
    cargo run --bin test-schemas

[working-directory(".")]
expand-reflection: gen-schemas
    cargo expand --features reflection -p test-reflection > expanded.rs

test-renders:
    cargo test -p testing rendering_test -- -q --nocapture

update-changelog version:
    git cliff --tag {{ version }}
    git add "CHANGELOG.md"
    git commit -m "updated changelog"

build-docs: gen-readme
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps -p prelude --all-features --open
