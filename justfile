build:
    cargo build

generate_bindings: build
    wasm-bindgen \
      --target web \
      --out-dir server/pkg \
      ./target/wasm32-unknown-unknown/debug/frontend.wasm

watch:
    watchexec -r \
    -i "./target/wasm32-unknown-unknown/debug/frontend.wasm" \
    -i "server/pkg/*" \
    -- "just generate_bindings; cargo run --bin itlkd-server"

create_db:
    mkdir -p db
    diesel migration run

serve_backend:
  cargo run --bin server

serve_frontend: generate_bindings
  bun run --cwd frontend dev
