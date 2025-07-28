run:
  cargo run

fmt:
  cargo +nightly fmt

test name:
  cargo test {{name}} -- --nocapture

example *args:
  cargo lrun --example {{args}}

