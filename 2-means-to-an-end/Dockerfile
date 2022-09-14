FROM rust:slim-buster AS build

WORKDIR /opt
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=build /opt/target/x86_64-unknown-linux-musl/release/means-to-an-end means-to-an-end/
CMD ["/means-to-an-end"]
