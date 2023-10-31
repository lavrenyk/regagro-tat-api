FROM rust:1.73.0 as buider

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/regagro-tat-api"]