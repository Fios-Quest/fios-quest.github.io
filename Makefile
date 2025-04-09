.PHONY: %
default: hot

install:
	cargo install dioxus-cli
	cargo install static-web-server

build:
	dx bundle --platform web --ssg

serve: build
	static-web-server --port 8787 --root ./target/dx/fios-quest-github-io/release/web/public

hot:
	dx serve --addr 127.0.0.1

clean:
	cargo clean
