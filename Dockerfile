FROM rust:1.58 as build

WORKDIR /app
COPY . .

RUN cargo build --release

FROM rust:1.58-alpine as  production

WORKDIR /app
COPY --from=build /app/target/release .

RUN chmod +x ./app/sophy_bot

ENTRYPOINT [ "./app/sophy_bot" ]
