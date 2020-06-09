FROM alpine as buildbase

WORKDIR /bench

RUN apk update
RUN apk add cargo
RUN apk add rust

COPY Cargo.* ./
RUN cargo fetch

COPY . .
RUN cargo build --release --all-features

FROM alpine as runtime

RUN apk update
RUN apk add libgcc

WORKDIR /bench

COPY --from=buildbase /bench/target/release/benchtool-rust .

ENTRYPOINT ["./benchtool-rust"]