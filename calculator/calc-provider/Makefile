CAPABILITY_ID := "wasmcloud:calculator"
NAME          := "calcprovider"
#NAME          := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name')
VERSION       := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
LIBNAME       := $(shell echo $(NAME) | tr '-' '_')
VENDOR        := wasmcloud
REVISION      ?= 5
DESTINATION   ?= calc_provider.par.gz

# NOTE: if REVISION changes, update the revision in the manifest.yaml file

.PHONY: all build par push clean

build:
	cargo build --release

par:
	wash par create \
		--arch x86_64-linux \
		--binary target/release/libcalc_provider.so \
		--capid $(CAPABILITY_ID) \
		--name $(NAME) \
		--vendor $(VENDOR) \
		--version $(VERSION) \
		--revision $(REVISION) \
		--destination $(DESTINATION) \
		--compress

push:
	wash reg push --insecure localhost:5000/$(NAME):$(REVISION) $(DESTINATION)

clean:
	cargo clean && rm -f $(DESTINATION)
