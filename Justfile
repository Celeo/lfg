default: build

build: check
  @cargo build

check:
  @cargo check
  @cargo +nightly clippy

test:
  @cargo test
