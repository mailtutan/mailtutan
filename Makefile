
all: build-web

run:
	cargo run -p mailtutan

build-web:
	(cd mailtutan-web && trunk build)
