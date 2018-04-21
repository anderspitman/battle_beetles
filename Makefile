PROTO_DIR := protos
RUST_DIR := src
CLIENT_DIR := ui
JS_DIR := $(CLIENT_DIR)/src
RUST_GEN_DIR := $(RUST_DIR)/gen
JS_GEN_DIR := $(JS_DIR)/gen

PROTO_SRC := $(PROTO_DIR)/messages.proto
RUST_PROTO_GEN := $(RUST_GEN_DIR)/messages.rs
JS_PROTO_GEN := $(JS_GEN_DIR)/messages_pb.js
JS_SRC := $(wildcard $(JS_DIR)/*.js) $(JS_PROTO_GEN) $(CLIENT_DIR)/dist/index.html
RUST_SRC := $(wildcard $(RUST_DIR)/*.rs) $(wildcard $(RUST_DIR)/simulation/*.rs) $(RUST_PROTO_GEN)
BUNDLE_JS := $(CLIENT_DIR)/dist/bundle.js

run: rust
	cargo run

.PHONY: only_rust
only_rust:
	cargo build

rust: $(RUST_SRC) $(BUNDLE_JS)
	cargo build

proto: $(RUST_PROTO_GEN) $(JS_PROTO_GEN)

$(RUST_PROTO_GEN): $(PROTO_SRC)
	protoc --rust_out $(RUST_GEN_DIR) $(PROTO_SRC)

$(JS_PROTO_GEN): $(PROTO_SRC)
	protoc --proto_path=$(PROTO_DIR) --js_out=import_style=commonjs,binary:$(JS_GEN_DIR) $(PROTO_SRC)

$(BUNDLE_JS): $(JS_SRC)
	npm run --prefix ui dev
