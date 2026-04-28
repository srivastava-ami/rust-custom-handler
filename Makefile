# Variables
BINARY_NAME=handler
RUST_APP_DIR=rustapp
# Detect OS for local builds
ifeq ($(OS),Windows_NT)
    EXECUTABLE=$(BINARY_NAME).exe
else
    EXECUTABLE=$(BINARY_NAME)
endif

DOCKER_IMAGE=rust-azure-handler:latest

.PHONY: all help build-local build-linux docker-build clean

all: help

help:
	@echo "Ami Lab - Rust Azure Handler Build System"
	@echo "Usage: make [target]"
	@echo "  build-local   - Build for current host OS (Assuming Mac OS) - Requires Rust installed locally"
	@echo "  build-linux   - Build for Linux (musl) - Requires local target added"
	@echo "  docker-build  - Build via Docker (Platform Agnostic)"
	@echo "  clean         - Wipe build artifacts"

build-local:
	cd $(RUST_APP_DIR) && cargo build --release
	cp $(RUST_APP_DIR)/target/release/$(EXECUTABLE) ./$(BINARY_NAME)

# Get the host architecture
LOCAL_ARCH := $(shell uname -m)
ifeq ($(LOCAL_ARCH),x86_64)
    PLATFORM := linux/amd64
else
    PLATFORM := linux/arm64
endif

docker-build:
	docker build --platform $(PLATFORM) -t $(DOCKER_IMAGE) .
	@echo "Successfully built OCI image: $(DOCKER_IMAGE)"

# docker-run:
# 	docker run -d -p 8080:8080 --name ami-rust-app $(DOCKER_IMAGE)


clean:
	cd $(RUST_APP_DIR) && cargo clean
	rm -f $(BINARY_NAME)