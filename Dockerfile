FROM rust:1-slim as buildbase

WORKDIR /bench

COPY Cargo.* ./

RUN cargo fetch

FROM buildbase

COPY . .

RUN cargo install --path .

ENTRYPOINT ["benchtool-rust"]