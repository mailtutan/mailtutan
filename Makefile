
all: build-web build-backend

run:
	cargo run -p mailtutan

build-backend:
	cargo build --release
	strip target/release/mailtutan

build-web:
	(cd mailtutan-web && cargo build && trunk build)

trunk:
	(cd mailtutan-web && trunk serve --proxy-backend="http://localhost:1080/api/")
