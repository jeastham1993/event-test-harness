FROM --platform=${BUILDPLATFORM} rust:1-alpine3.19 as builder

ARG TARGETPLATFORM
ARG RUST_TARGET

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk update && apk add pkgconfig openssl openssl-dev libc-dev musl-dev bash alpine-sdk

COPY ./scripts ./scripts

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc   
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc

COPY ./src .

# The leading '.' passes the environment variable back up to Docker build
RUN . scripts/target.sh && rustup target add $RUST_TARGET
RUN . scripts/target.sh && echo RUST_TARGET: $RUST_TARGET && cargo fetch --target $RUST_TARGET
RUN . scripts/target.sh && cargo build --release --target $RUST_TARGET --package rust_event_harness  && cp target/$RUST_TARGET/release/rust_event_harness target/rust_event_harness

RUN strip target/rust_event_harness

FROM alpine:3.19
RUN apk update \
    && apk add openssl ca-certificates libgcc

EXPOSE 8080

COPY --from=builder /target/rust_event_harness .

ENTRYPOINT ["/rust_event_harness"]