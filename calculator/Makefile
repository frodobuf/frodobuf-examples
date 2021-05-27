
WASMCLOUD=$(HOME)/projects/git/wasmcloud/target/debug/wasmcloud

.PHONY: all build clean run

all: release

release:
	cd actor && make release
	cd calc-provider && make build par push

clean:
	cd calc-interface/rust && cargo clean
	cd actor && make clean
	cd calc-provider && make clean
	wash drain all

run:
	export CALC_ACTOR=$(shell cd actor && make actor_id 2>&1 | tail -1)
	RUST_LOG=debug $(WASMCLOUD) \
			 --allowed-insecure localhost:5000 \
			 -m manifest.yaml


