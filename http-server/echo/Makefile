COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
REVISION = 2

.PHONY: all build check clean doc test update

all: build

build:
	@$(CARGO) build
	wash claims sign $(DEBUG)/echo.wasm -c wasmcloud:httpserver --name "echo" --ver $(VERSION) --rev $(REVISION)

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

test: build
	@$(CARGO) test

update:
	@$(CARGO) update

release:
	@$(CARGO) build --release
	wash claims sign $(RELEASE)/echo.wasm -c wasmcloud:httpserver --name "echo" --ver $(VERSION) --rev $(REVISION)

actor_id: release
	wash claims inspect $(RELEASE)/echo_s.wasm -o json | jq .module | tr -d "\"" 

