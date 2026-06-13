.PHONY: build run docker-build docker-run lint

# Local binary name
BINARY_NAME=vyzorix
DOCKER_IMAGE=vyzorix/cli:latest

# Builds the Rust binary locally (Optimized release)
build:
	@echo "=> Compiling release binary..."
	cargo build --release

# Runs the CLI locally in dev mode
run:
	@echo "=> Running CLI..."
	cargo run -- --dev $(CMD)

# Builds the isolated non-root Docker container
docker-build:
	@echo "=> Building Docker Image (Alpine Non-Root)..."
	docker build -t $(DOCKER_IMAGE) .

# Runs the container securely without root, mounting the current dir
docker-run:
	@echo "=> Running container securely..."
	docker run --rm -it \
		--user $$(id -u):$$(id -g) \
		-v $$(pwd):/workspace \
		$(DOCKER_IMAGE) $(CMD)

# Runs standard format & lint checks
lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
