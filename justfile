help:
    just --list

deploy:
    scp swagger.yaml growcult-uberspace:html/swagger.yaml
    scp -r docs growcult-uberspace:html/api/docs
