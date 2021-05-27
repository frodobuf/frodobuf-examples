COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
REVISION = 2

CALC_CAPABILITY_ID = wasmcloud:calculator

.PHONY: all build check clean doc test update

all: build

build:
	@$(CARGO) build
	wash claims sign $(DEBUG)/calc_actor.wasm --name "calc_actor" \
		--cap $(CALC_CAPABILITY_ID) \
		--cap "wasmcloud:httpserver" \
		--ver $(VERSION) --rev $(REVISION)

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
	wash claims sign $(RELEASE)/calc_actor.wasm --name "calc_actor" \
		--cap $(CALC_CAPABILITY_ID) \
		--cap "wasmcloud:httpserver" \
		--ver $(VERSION) --rev $(REVISION) 

actor_id: release
	wash claims inspect $(RELEASE)/calc_actor_s.wasm -o json | jq .module | tr -d "\""


