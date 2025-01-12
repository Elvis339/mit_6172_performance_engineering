FROM rust:1.83-alpine

COPY . .

RUN cargo test

# Test on x86_64 arch
# docker buildx build --platform linux/amd64 -t atomics . && docker run --platform linux/amd64 -it atomics