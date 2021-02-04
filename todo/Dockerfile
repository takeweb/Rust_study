FROM rust:1.49 AS builder

# /todoでビルド
WORKDIR /todo

# Cargo.tomlのみを先にイメージに追加する。
COPY Cargo.toml Cargo.toml

# ビルドするために何もしないソースコードを入れておく。
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 空コードと依存クレートをビルド
RUN cargo build --release

# アプリケーションコードをイメージにコピー
COPY ./src ./src
COPY ./templates ./templates

# 空コードのビルド成果物の内、アプリケーションのもののみを削除する。
RUN rm -f target/release/deps/todo*

# 改めてアプリケーションをビルドする。
RUN cargo build --release

# 新しくリリース用イメージを用意する。
FROM debian:10.4

# builderイメージからtodoのみをコピーして/usr/local/binに配置する。
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo

# コンテナ起動時にWebアプリを実行
CMD ["todo"]