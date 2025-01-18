FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:slim-bookworm

WORKDIR /app
COPY --from=builder /app/target/release/imphnen-chat-backend ./imphnen-chat-backend

CMD ["./imphnen-chat-backend"]
