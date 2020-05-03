docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/nf_simulator -w /usr/src/nf_simulator rust-lkl:latest cargo $@
