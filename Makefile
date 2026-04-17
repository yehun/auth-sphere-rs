all:
	@echo "run"
	@echo "build"
	@echo "build-release"
	@echo "clean"

run:
	cargo run -p auth-sphere-api

build:
	cargo build -p auth-sphere-api

build-web:
	cd web && npm run build

build-release:
	cargo build --release -p auth-sphere-api

clean:
	cross clean