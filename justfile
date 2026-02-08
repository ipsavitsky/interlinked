all: build

build_frontend:
    cargo build --package frontend --target wasm32-unknown-unknown

generate_bindings: build_frontend
    wasm-bindgen \
      --target web \
      --no-typescript \
      --out-dir server/pkg \
      ./target/wasm32-unknown-unknown/debug/frontend.wasm

build: generate_bindings
    cargo build -p server -p cli

watch:
    watchexec -r \
    -i "./target/wasm32-unknown-unknown/debug/frontend.wasm" \
    -i "server/pkg/*" \
    -i "objects/*" \
    -i "interlinked.db*" \
    -i "*.db*" \
    -- "just generate_bindings; cargo run --bin itlkd-server"

create_db:
    mkdir -p db
    diesel migration run

test: build
    cargo test --workspace --exclude frontend
