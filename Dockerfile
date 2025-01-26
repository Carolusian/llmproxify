FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM rust:slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/llmproxify .

CMD ["./llmproxify"]