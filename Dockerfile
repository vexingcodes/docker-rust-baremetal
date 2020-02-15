FROM rustlang/rust:nightly
RUN cargo install cargo-xbuild bootimage \
 && rustup component add rust-src \
 && rustup component add llvm-tools-preview
RUN apt-get update \
 && apt-get install --yes --no-install-recommends \
      qemu-kvm \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . ./
