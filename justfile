build: generate_bindings
    cargo build

build_frontend:
    cargo build --package frontend

generate_bindings: build_frontend
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

test: build
    cargo test --workspace --exclude frontend
