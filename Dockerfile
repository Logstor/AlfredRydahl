FROM rust:1.57 as builder

RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/rust_docker_web*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app
