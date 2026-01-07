build:
    cargo build

generate_bindings: build
    wasm-bindgen \
      --target web \
      --out-dir frontend/pkg \
      ./target/wasm32-unknown-unknown/debug/frontend.wasm

create_db:
    mkdir -p db
    diesel migration run

serve_backend:
  cargo run --bin server

serve_frontend: generate_bindings
  bun run --cwd frontend dev
