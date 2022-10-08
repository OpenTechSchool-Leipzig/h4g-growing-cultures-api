help:
    just --list

deploy:
    scp swagger.yaml growcult-uberspace:html/swagger.yaml
    rsync -r docs/ growcult-uberspace:html/api/docs/
    rsync -r examples/ growcult-uberspace:html/api/examples/
