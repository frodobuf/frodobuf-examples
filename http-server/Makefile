
.PHONY: release clean run


release:
	#cd rust && cargo build --release
	cd echo && make release

clean:
	cd rust && cargo clean
	cd echo && make clean

run:
	export ECHO_ACTOR=$(shell cd echo && make actor_id 2>&1 | tail -1)
	RUST_LOG=trace wasmcloud -m manifest.yaml
