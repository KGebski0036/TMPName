
FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY /src ./src

RUN cargo fetch

RUN cargo build --release

COPY /templates /etc/server/templates 

FROM debian:bookworm-slim

ENV REPO_DIR="/var/server/repo"
ENV TEMPLATE_DIR="/etc/server/templates"

COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/server

WORKDIR /usr/local/bin

VOLUME ["/var/server/repo"]


EXPOSE 8080

CMD ["server"]
