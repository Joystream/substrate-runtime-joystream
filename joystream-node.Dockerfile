FROM liuchong/rustup:1.47.0 AS rustup
RUN rustup component add rustfmt clippy
RUN rustup install nightly-2020-10-06 --force
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-06
RUN apt-get update && \
  apt-get install -y curl git gcc xz-utils sudo pkg-config unzip clang libc6-dev-i386

FROM rustup AS builder
LABEL description="Compiles all workspace artifacts"
WORKDIR /joystream
COPY . /joystream

# Build all cargo crates
# Ensure our tests and linter pass before actual build
ENV WASM_BUILD_TOOLCHAIN=nightly-2020-10-06
ARG TEST_NODE
RUN echo "TEST_NODE=$TEST_NODE"
RUN test -n "$TEST_NODE" && sed -i 's/MILLISECS_PER_BLOCK: Moment = 6000/MILLISECS_PER_BLOCK: Moment = 1000/' ./runtime/src/constants.rs
RUN test -n "$TEST_NODE" && sed -i 's/SLOT_DURATION: Moment = 6000/SLOT_DURATION: Moment = 1000/' ./runtime/src/constants.rs
RUN export ALL_PROPOSALS_PARAMETERS_JSON="$(test -n "$TEST_NODE" && cat ./tests/integration-tests/proposal-parameters.json)" &&\
    BUILD_DUMMY_WASM_BINARY=1 cargo clippy --release --all -- -D warnings && \
    cargo test --release --all && \
    cargo build --release

FROM debian:stretch
LABEL description="Joystream node"
WORKDIR /joystream
COPY --from=builder /joystream/target/release/joystream-node /joystream/node
COPY --from=builder /joystream/target/release/wbuild/joystream-node-runtime/joystream_node_runtime.compact.wasm /joystream/runtime.compact.wasm
COPY --from=builder /joystream/target/release/chain-spec-builder /joystream/chain-spec-builder

# confirm it works
RUN /joystream/node --version

# https://manpages.debian.org/stretch/coreutils/b2sum.1.en.html
# RUN apt-get install coreutils
# print the blake2 256 hash of the wasm blob
RUN b2sum -l 256 /joystream/runtime.compact.wasm
# print the blake2 512 hash of the wasm blob
RUN b2sum -l 512 /joystream/runtime.compact.wasm

EXPOSE 30333 9933 9944

# Use these volumes to persits chain state and keystore, eg.:
# --base-path /data
# optionally separate keystore (otherwise it will be stored in the base path)
# --keystore-path /keystore
# if base-path isn't specified, chain state is stored inside container in ~/.local/share/joystream-node/
# which is not ideal
VOLUME ["/data", "/keystore"]

ENTRYPOINT ["/joystream/node"]
