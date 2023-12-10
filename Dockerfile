FROM rust:1.74.1 as builder

RUN USER=root cargo new --bin cloudflare_images_dev

WORKDIR /cloudflare_images_dev

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/cloudflare_images_dev*
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update && apt upgrade --yes

COPY --from=builder /cloudflare_images_dev/target/release/cloudflare_images_dev .
RUN mkdir .files

EXPOSE 3030

CMD ["./cloudflare_images_dev"]
