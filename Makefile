.PHONY: build install clean run dev

build:
	cargo build --release
	mkdir -p bin
	cp target/release/waybar-lyrics-rs bin/

install: build
	cp bin/waybar-lyrics-rs /usr/local/bin/

clean:
	cargo clean
	rm -rf bin/

run:
	cargo run

dev:
	cargo run -- --verbose
