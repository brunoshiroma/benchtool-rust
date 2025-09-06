FROM rust:alpine as buildbase

WORKDIR /bench

COPY Cargo.* ./
RUN cargo fetch

COPY . .
RUN cargo build --release --all-features
FROM alpine as runtime

WORKDIR /bench

COPY --from=buildbase /bench/target/release/benchtool-rust .

ENTRYPOINT ["./benchtool-rust"]