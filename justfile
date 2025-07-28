fmt:
  cargo +nightly fmt

test name:
  cargo test {{name}} -- --nocapture

example *args:
  cd yin-yang && cargo lrun --example {{args}}
