PROTO_PATH := protos
PROTO_SRC := ${PROTO_PATH}/messages.proto

rust: bundle_js rust_proto js_proto
	cargo build

run: bundle_js rust_proto js_proto
	cargo run

rust_proto: ${PROTO_SRC}
	protoc --rust_out src/gen ${PROTO_SRC}

js_proto: ${PROTO_SRC}
	protoc --proto_path=${PROTO_PATH} --js_out=import_style=commonjs,binary:ui/src/gen ${PROTO_SRC}

.PHONY: build_js
bundle_js:
	npm run --prefix ui build
