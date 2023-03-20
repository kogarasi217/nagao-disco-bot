FROM rust:1.68-bullseye

RUN apt-get update && \
  apt-get -y install git && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  rustup update &&\
  rustup component add rls rust-analysis rust-src rustfmt clippy && \
  cargo install cargo-edit cargo-watch

# ready to shuttle
RUN cargo install cargo-shuttle

# Build cache
# https://qiita.com/tatsuya6502/items/76b28a6786a1ddc9d479
RUN cargo install sccache
RUN export RUSTC_WRAPPER=$(which sccache)