FROM rust:1.49

WORKDIR /usr/src/rust-wordpress
COPY . .

RUN cargo install --path .

CMD ["rust-wordpress"]