FROM golang:1.17-alpine3.15 as builder

WORKDIR /app
COPY ./server .

RUN go build

FROM alpine:3.15

WORKDIR /app
COPY --from=builder /app/server server

RUN chmod +x server

ENTRYPOINT [ "./server" ]
