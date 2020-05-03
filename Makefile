build-builder:
	docker build . -t rust-lkl

bindgen:
	bindgen --use-core ../../lkl/linux/tools/lkl/include/lkl_host.h -o ./src/lkl_host.rs