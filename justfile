build:
    cargo build -p server -p cli
    cd frontend && trunk build

watch:
    watchexec -r -e rs,toml,html -- cargo run -p server & cd frontend && trunk serve -p 3001

fmt:
    cargo fmt
    leptosfmt frontend/src
