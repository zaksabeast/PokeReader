FROM devkitpro/devkitarm

RUN apt-get update && \
  # Install build-essential 
  apt-get install -y --no-install-recommends build-essential && \
  # Cleanup apt
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  # Install rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
  . $HOME/.cargo/env && \
  # Install toolchain
  rustup toolchain install nightly-x86_64-unknown-linux-gnu && \
  rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
