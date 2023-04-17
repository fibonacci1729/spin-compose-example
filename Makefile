.PHONY: example
example: compose-component
	spin up

.PHONY: compose-component
compose-component: build-middleware build-service
	wasm-tools compose -c config.yml -o composed.wasm middleware/target/wasm32-unknown-unknown/release/middleware.wasm

.PHONY: build-middleware
build-middleware:
	cd middleware && cargo component build --release --target wasm32-unknown-unknown

.PHONY: build-service
build-service:
	cd service && cargo component build --release --target wasm32-unknown-unknown
