FROM rust:slim-buster AS build

WORKDIR /opt
RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
COPY --from=build /opt/target/x86_64-unknown-linux-musl/release/smoke-test /smoke-test
EXPOSE 1234
CMD ["/smoke-test"]
