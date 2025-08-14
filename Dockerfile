### マルチステージビルドを使用し、Rustのプログラムをビルド
FROM rust:1.89-slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

COPY . .
RUN cargo build --release

### 不要なソフトウェアを同梱する必要がないため　bookworm-slim を使用する
FROM debian:bookworm-slim
WORKDIR /app

### ユーザを作成しておく
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app

### 8080ポートを開放し、アプリケーションを起動
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]
