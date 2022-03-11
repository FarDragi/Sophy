FROM golang:1.17-alpine3.15 as builder

WORKDIR /app
COPY ./grpc .

RUN go build

FROM alpine:3.15

WORKDIR /app
COPY --from=builder /app/grpc grpc

RUN chmod +x grpc

ENTRYPOINT [ "./grpc" ]
