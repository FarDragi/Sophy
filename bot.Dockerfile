FROM rust:1.59.0-alpine3.15 as builder

WORKDIR /app
COPY ./bot .
COPY ./proto /proto

RUN cargo build --release

FROM alpine:3.15

WORKDIR /app
COPY --from=builder /app/target/release/sophy_bot sophy_bot

RUN chmod +x sophy_bot

ENTRYPOINT [ "./sophy_bot" ]
