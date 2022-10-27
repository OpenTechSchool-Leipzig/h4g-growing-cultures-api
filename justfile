help:
    just --list

deploy:
    scp swagger.yaml growcult-uberspace:html/swagger.yaml
    rsync -r docs/ growcult-uberspace:html/api/docs/
    rsync -r examples/ growcult-uberspace:html/api/examples/

fmt:
    cargo fmt --all

test:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt --all -- --check

run-server:
    cargo run

watch-server:
    cargo watch -x run
