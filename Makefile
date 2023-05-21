
all: build-web build

run:
	cargo run -p mailtutan

build:
	cargo build --release
	strip target/release/mailtutan

build-web:
	(cd mailtutan-web && trunk build)
