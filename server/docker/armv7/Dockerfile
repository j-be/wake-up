# syntax=docker/dockerfile:1

# Using multistage build:
# 	https://docs.docker.com/develop/develop-images/multistage-build/
# 	https://whitfin.io/speeding-up-rust-docker-builds/

########################## BUILD IMAGE  ##########################
FROM ghcr.io/j-be/rust-diesel-postgres:armv7 as build

# Copies over *only* your manifests and build files
COPY ./Cargo.* ./
COPY ./rust-toolchain ./rust-toolchain

RUN rustup target add armv7-unknown-linux-gnueabihf

# Copies the complete project
# To avoid copying unneeded files, use .dockerignore
COPY . .

# Builds again, this time it'll just be
# your actual source files being built
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf
