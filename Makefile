.PHONY: build install clean run-waybar run-lyrics dev-waybar dev-lyrics

# Build all binaries
build:
	cargo build --release
	mkdir -p bin
	cp target/release/waybar-bard bin/
	cp target/release/bard bin/

# Install all binaries
install: build
	cp bin/waybar-bard /usr/local/bin/
	cp bin/bard /usr/local/bin/

# Install just waybar integration
install-waybar: build
	cp bin/waybar-bard /usr/local/bin/

# Install just lyrics terminal app
install-lyrics: build
	cp bin/bard /usr/local/bin/

# Clean everything
clean:
	cargo clean
	rm -rf bin/

# Run specific applications
run-waybar:
	cargo run -p waybar-bard

run-lyrics:
	cargo run -p bard

# Development mode with verbose output
dev-waybar:
	cargo run -p waybar-bard -- --verbose

dev-lyrics:
	cargo run -p bard -- --verbose
