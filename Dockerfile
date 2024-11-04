FROM rust:1.82

WORKDIR /axum-api
COPY . .

EXPOSE 8080/tcp


RUN cargo install --path .

RUN cargo build

WORKDIR /axum-api/target/debug/

CMD ["axum-api"]


