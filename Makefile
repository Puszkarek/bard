.PHONY: build install clean run-waybar run-lyrics dev-waybar dev-lyrics

# Build all binaries
build-all:
	cargo build --release
	mkdir -p bin
	cp target/release/waybar-lyrics-rs bin/
	cp target/release/lyrics-rs bin/

# Install all binaries
install-all: build
	cp bin/waybar-lyrics-rs /usr/local/bin/
	cp bin/lyrics-rs /usr/local/bin/

# Install just waybar integration
install-waybar: build
	cp bin/waybar-lyrics-rs /usr/local/bin/

# Install just lyrics terminal app
install-lyrics: build
	cp bin/lyrics-rs /usr/local/bin/

# Clean everything
clean:
	cargo clean
	rm -rf bin/

# Run specific applications
run-waybar:
	cargo run -p waybar-lyrics-rs

run-lyrics:
	cargo run -p lyrics-rs

# Development mode with verbose output
dev-waybar:
	cargo run -p waybar-lyrics-rs -- --verbose

dev-lyrics:
	cargo run -p lyrics-rs -- --verbose
