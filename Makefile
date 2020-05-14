build-builder:
	docker build ./builder -t nf-builder

build:
	docker build . -t rust-lkl

shell:
	docker run -it --cap-add NET_ADMIN -v $(CURDIR):/nf_simulator:delegated -w /nf_simulator nf-builder:latest

bindgen:
	bindgen --use-core ../../lkl/linux/tools/lkl/include/lkl_host.h -o ./src/lkl_host.rs