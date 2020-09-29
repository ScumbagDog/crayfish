FROM rust:latest
WORKDIR /usr/crayfish
COPY . .
RUN cargo build --release
ENTRYPOINT ./target/release/crayfish
