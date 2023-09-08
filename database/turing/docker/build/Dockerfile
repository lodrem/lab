FROM golang:1.21-bookworm AS builder

WORKDIR /workspace
COPY . .

RUN CGO_ENABLED=0 go build -o bin/turing ./...

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates
COPY --from=builder /workspace/bin/turing /usr/local/bin/turing

ENTRYPOINT ["/usr/local/bin/turing"]