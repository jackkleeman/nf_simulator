FROM rust:latest

RUN apt-get update
RUN apt-get install -y libfuse-dev libarchive-dev xfsprogs flex bison bc

# create a new empty shell project
RUN USER=root cargo new --bin nf_simulator
WORKDIR /nf_simulator

# copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-lkl ./rust-lkl
COPY ./app/Cargo.toml ./app/Cargo.toml
COPY ./app/Cargo.lock ./app/Cargo.lock
COPY ./dummy.rs ./app/src/main.rs

# this build step will cache your dependencies
RUN cargo build --release
RUN rm ./app/src/main.rs

# copy your source tree
COPY ./app/src ./app/src
COPY ./app/build.rs ./app/build.rs

# build for release
# RUN rm ./target/release/deps/nf_simulator*
RUN cargo build --release

# set the startup command to run your binary
CMD ["./target/release/nf_simulator"]