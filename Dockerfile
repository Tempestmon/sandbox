FROM rust:1.76 as build

RUN USER=root cargo new --bin web_server
WORKDIR /web_server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo install diesel_cli
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/web_server*
RUN cargo build --release

FROM rust:1.76-slim-buster

COPY --from=build /web_server/target/release/web_server .

CMD ["./web_server"]