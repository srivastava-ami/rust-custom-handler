# Stage 1: Build
FROM --platform=$BUILDPLATFORM rust:1.90-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    musl-tools \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY rustapp/ ./rustapp/

RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then TARGET="x86_64-unknown-linux-musl"; \
    elif [ "$ARCH" = "aarch64" ]; then TARGET="aarch64-unknown-linux-musl"; fi && \
    rustup target add $TARGET && \
    cd rustapp && \
    cargo build --release --target $TARGET && \
    cp target/$TARGET/release/handler /build/handler

# Create a non-root user in the builder stage to copy into scratch
RUN echo "nobody:x:65534:65534:nobody:/_:/usr/sbin/nologin" > /etc/passwd.minimal

# Stage 2: The Production Nano Image
FROM scratch

# 1. Security: Copy the minimal user definition
COPY --from=builder /etc/passwd.minimal /etc/passwd
USER nobody

# 2. Networking: Copy SSL certificates for outbound HTTPS calls
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# 3. Application: The compressed static binary
WORKDIR /app
COPY --from=builder /build/handler .

# 4. Configuration: The "Spec"
COPY host.json .
COPY httpTrigger/ ./httpTrigger/

ENV FUNCTIONS_CUSTOMHANDLER_PORT=8080
EXPOSE 8080

ENTRYPOINT ["./handler"]