FROM rust:1.83-slim as builder

WORKDIR /usr/src/mit
COPY . .

RUN cargo clean
RUN cargo build --workspace --release
RUN cargo build --workspace

FROM debian:bookworm-slim

WORKDIR /usr/local/bin

RUN apt-get update && apt-get upgrade && apt-get -y install valgrind

# Copy debug binaries
COPY --from=builder /usr/src/mit/target/debug/intro_0 ./intro_0-debug
COPY --from=builder /usr/src/mit/target/debug/matrix_mul_1 ./matrix_mul_1-debug

# Copy release binaries
COPY --from=builder /usr/src/mit/target/release/intro_0 ./intro_0-release
COPY --from=builder /usr/src/mit/target/release/matrix_mul_1 ./matrix_mul_1-release

ENV PATH="/usr/local/bin:${PATH}"