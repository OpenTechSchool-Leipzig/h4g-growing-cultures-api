help:
    just --list

deploy:
    scp swagger.yaml growcult-uberspace:html/swagger.yaml
    rsync -r docs/ growcult-uberspace:html/api/docs/
    rsync -r examples/ growcult-uberspace:html/api/examples/

build:
    cargo build

fmt:
    cargo fmt --all

doc:
    cargo doc --open

test:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt --all -- --check

run-server:
    cargo run

watch-server:
    cargo watch -x run
