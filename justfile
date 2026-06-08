build:
    cargo build -p server -p cli
    cd frontend && trunk build

watch:
    watchexec -r -e rs,toml,html -- sh -c "just build && cargo run -p server"

fmt:
    cargo fmt
    leptosfmt frontend/src
