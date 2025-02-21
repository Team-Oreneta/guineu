FROM debian:latest

# Install dependencies
RUN apt update && apt install -y \
    build-essential \
    nasm \
    xorriso \
    clang \
    grub-common \
    mtools \
    gcc-multilib \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && echo 'source "$HOME/.cargo/env"' >> $HOME/.bashrc

# Set Rust to nightly
RUN /root/.cargo/bin/rustup toolchain install nightly \
    && /root/.cargo/bin/rustup override set nightly

RUN /root/.cargo/bin/rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# Set working directory
WORKDIR /guineu

# Copy source code
COPY . .

# Build the project
RUN /bin/bash -c "source $HOME/.bashrc && make"

CMD ["/bin/bash"]
