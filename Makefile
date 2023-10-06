# This Makefile is for a Rust project using Cargo.

# Project name and binary.
PROJECT_NAME = computor
BIN_NAME = target/debug/$(PROJECT_NAME)

# Build and run commands.
CARGO = cargo
RUSTFLAGS = -O

# Default target.
.PHONY: all
all: build

# Build the project using Cargo.
.PHONY: build
build:
	$(CARGO) build

# Build the project using Cargo.
.PHONY: release
release:
	$(CARGO) build --release

# Clean the build artifacts.
.PHONY: clean
clean:
	$(CARGO) clean

# Remove build artifacts and generated files.
.PHONY: fclean
fclean: clean
	rm -f Cargo.lock

# Run the compiled binary with arguments.
.PHONY: run
run: build
	$(BIN_NAME) $(ARGS)

# Help target to display available make targets.
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all       : Build the project (default)"
	@echo "  build     : Build the project using Cargo"
	@echo "  release   : Build the project using Cargo in release mode"
	@echo "  clean     : Clean build artifacts"
	@echo "  run ARGS  : Run the compiled binary with arguments"
	@echo "  help      : Show this help message"

# Prevent make from treating the targets as files.
.PHONY: all build clean run help
