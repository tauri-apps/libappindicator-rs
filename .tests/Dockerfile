FROM ubuntu:18.04 as build

# Dependencies
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    build-essential ca-certificates curl libappindicator3-dev

# Drop privileges or Rust will have trouble installing
RUN useradd -m --uid 1000 --user-group rust
USER rust
WORKDIR /home/rust/src

# Install toolchain
ENV RUST_TOOLCHAIN=nightly
ENV RUST_BACKTRACE=1
ENV CARGO_INCREMENTAL=0
ENV CARGO_PROFILE_DEV_DEBUG=0

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain $RUST_TOOLCHAIN -y
ENV PATH=/home/rust/.cargo/bin:$PATH
RUN rustup component add rust-src --toolchain $RUST_TOOLCHAIN

# Build our app.
COPY --chown=rust:rust . /home/rust/src
ARG build_cmd="cargo build --release --example=hello"
RUN sh -c "${build_cmd}"


FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y libcanberra-gtk3-module binutils

RUN mkdir -p /app

ARG with_lib=""
RUN apt-get install -y ${with_lib}

ENV TRAY_ICON_DIR /app/icons
COPY examples/rust-logo.png /app/icons/rust-logo.png
COPY --from=build /home/rust/src/target/release/examples/hello /app/bin/hello

# RUN ls -la /usr/lib/x86_64-linux-gnu/ | grep indicator;\
#     ldconfig --print-cache | grep indic;\
#     objdump -p /usr/lib/x86_64-linux-gnu/*indicator* | grep SONAME;\
#     true

CMD ["/app/bin/hello"]
