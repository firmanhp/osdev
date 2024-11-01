# Directory for storing generated documentation
OUTPUT_DIR = docs

# Ensure dependencies are available
check-dependencies:
	@command -v cargo > /dev/null 2>&1 || { echo >&2 "Cargo is required but not installed. Please install Rust and Cargo from https://rustup.rs/"; exit 1; }
	@command -v rustdoc > /dev/null 2>&1 || { echo >&2 "Rustdoc is required but not found. Please install Rust and Cargo from https://rustup.rs/"; exit 1; }
	@command -v python3 > /dev/null 2>&1 || { echo >&2 "Python3 is required but not installed. Please install it to use the serve feature."; exit 1; }

# Default target to generate documentation
all: check-dependencies $(OUTPUT_DIR)/index.html

# Generate documentation and place it in OUTPUT_DIR
$(OUTPUT_DIR)/index.html: 
	cargo doc --no-deps --document-private-items
	mkdir -p $(OUTPUT_DIR)
	cp -r target/doc/* $(OUTPUT_DIR)

# Serve documentation locally
serve: all
	@echo "Serving documentation at http://localhost:8000..."
	cd $(OUTPUT_DIR) && python3 -m http.server 8000

# Clean target to remove generated documentation
clean:
	rm -rf $(OUTPUT_DIR) target/doc

# PHONY targets (not actual files)
.PHONY: all clean serve check-dependencies
