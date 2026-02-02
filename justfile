gen-readme:
    cargo run -p gen-readme

check-asm-output:
    cargo asm --release -p protify --test code_elimination trigger_validation

[working-directory(".")]
test-all: test-shared-schemas test-schemas test-no-std test-proc-macro test-code-elimination
    cargo test -p protify -- --nocapture

test-code-elimination:
    cargo test -p protify --release code_elimination

test-proc-macro:
    cargo test -p protify-proc-macro -- --nocapture

test-no-std:
    cargo test --features reflection -p test-no-std -- --nocapture
    cargo test -p test-no-std -- --nocapture

test-schemas:
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
    cargo test -p test-schemas rendering_test -- -q --nocapture

release version exec="": test-all gen-readme
    ../pre_release.sh {{ version }} {{ exec }}
    cargo release {{ version }} {{ exec }}

build-docs: gen-readme
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps -p protify --all-features --open
