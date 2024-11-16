# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Install necessary packages
RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config;

# Install sqlx-cli
RUN cargo install sqlx-cli;

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock .env ./
COPY migrations ./migrations
COPY templates ./templates
COPY pdfs ./pdfs
COPY .cargo ./.cargo

# Copy the source code
COPY src ./src

# Prepare the sqlx database
RUN cargo sqlx database setup;
RUN cargo sqlx prepare;

# Build the Rust project
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target=x86_64-unknown-linux-gnu;
# --target=x86_64-unknown-linux-musl

# Use a more recent version of Debian as the base image for the final stage
FROM debian:bookworm-slim

# Install necessary packages including Bun and Chromium
RUN apt-get update && apt-get install -y pkg-config wget curl libc6 unzip python3 perl libgbm1 libgl1 libglx-mesa0 libgtk-3-0 libnss3 libsecret-1-0 libxss1 pulseaudio shared-mime-info libxshmfence1;
RUN wget https://freeshell.de/phd/chromium/jammy/pool/chromium_116.0.5845.179~linuxmint1+victoria/chromium_116.0.5845.179~linuxmint1+victoria_amd64.deb \
    && dpkg -i chromium_116.0.5845.179~linuxmint1+victoria_amd64.deb \
    && apt --fix-broken install \
    && dpkg -i chromium_116.0.5845.179~linuxmint1+victoria_amd64.deb;

# Install Bun
RUN wget -qO- https://bun.sh/install | bash;

# Set the working directory
WORKDIR /usr/src/app

RUN mv /usr/bin/chromium /usr/bin/chromium-browser-116;

# Create a wrapper script for Chromium to add the --no-sandbox flag
RUN echo '#!/bin/bash\nexec /usr/bin/chromium-browser-116 --no-sandbox "$@"' > /usr/local/bin/chromium-wrapper \
    && chmod +x /usr/local/bin/chromium-wrapper;

# Ensure that any calls to `chrome` or `chromium` use the `chromium-wrapper`
RUN ln -sf /usr/local/bin/chromium-wrapper /usr/bin/chrome && ln -sf /usr/local/bin/chromium-wrapper /usr/bin/chromium;

# Set the CHROME_PATH environment variable to use the wrapper script
ENV CHROME_PATH=/usr/local/bin/chromium-wrapper;

# Copy the entire release folder from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release /usr/local/bin/release

# Create a symlink to the casual-cli binary
RUN ln -s /usr/local/bin/release/casual-cli /usr/local/bin/casual-cli;

# Create a symlink to Bun
RUN ln -s /root/.bun/bin/bun /usr/local/bin/bun;
COPY .env casual.sqlite /usr/local/bin/release/

# Copy the Bun project files
RUN ln -s /usr/local/bin/release/casual.sqlite /usr/src/app/casual.sqlite;
COPY index.html index.ts bun.js package.json bun.lockb tsconfig.json .env ./
COPY public ./public
COPY templates /root/.ccli/templates

# Upgrade Bun
RUN bun upgrade;

# Install Bun dependencies
RUN bun install

# Expose the port that the Bun server will run on
EXPOSE 3000

# Run the Bun server
CMD ["bun", "run", "start"]
