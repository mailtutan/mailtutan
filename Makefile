
all: build-web build-backend

run:
	cargo run -p mailtutan

build-backend:
	cargo build --release
	strip target/release/mailtutan

build-web:
	(cd mailtutan-web && cargo build && trunk build --release)

trunk:
	(cd mailtutan-web && trunk serve)

publish:
	cargo publish -p mailtutan-web
	cargo publish -p mailtutan-lib
	cargo publish -p mailtutan

VERSION := $(shell toml get Cargo.toml workspace.package.version --raw)

docker-build:
	docker build . -t mailtutan/mailtutan:$(VERSION)

docker-push:
	docker tag mailtutan/mailtutan:$(VERSION) mailtutan/mailtutan:latest
	docker push mailtutan/mailtutan:$(VERSION)
	docker push mailtutan/mailtutan:latest
