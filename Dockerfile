FROM rust:1.79.0 as builder
WORKDIR /usr/src/grpc-lb-tester
COPY . .
RUN apt update && apt install -y protobuf-compiler
RUN cargo install --path . --bin grpc-lb-tester-client

FROM debian:bookworm-slim

RUN apt update && apt install -y openssl ca-certificates

COPY --from=builder /usr/local/cargo/bin/grpc-lb-tester-client /usr/local/bin/grpc-lb-tester-client
CMD ["grpc-lb-tester-client"]