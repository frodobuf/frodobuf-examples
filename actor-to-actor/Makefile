
.PHONY: release clean run


release:
	cd pinger && make release
	cd ponger && make release

clean:
	cd ping-interface/rust && cargo clean
	cd pinger && cargo clean
	cd ponger && make clean

run:
	export PINGER_ACTOR=$(shell cd pinger && make actor_id 2>&1 | tail -1)
	export PONGER_ACTOR=$(shell cd ponger && make actor_id 2>&1 | tail -1)
	RUST_LOG=trace wasmcloud -m manifest.yaml
