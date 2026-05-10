# syntax=docker/dockerfile:1
# 本地智能体全栈构建镜像
# 阶段1：Rust runtime-core
FROM rust:1.86-slim-bookworm AS rust-builder
WORKDIR /build
COPY crates/runtime-core/Cargo.toml crates/runtime-core/Cargo.lock ./
COPY crates/runtime-core/src ./src
RUN cargo build --release

# 阶段2：Go gateway
FROM golang:1.25-bookworm AS go-builder
WORKDIR /build
COPY gateway/go.mod gateway/go.sum ./
RUN go mod download
COPY gateway/ ./
RUN CGO_ENABLED=1 go build -o /bin/gateway ./cmd/server

# 阶段3：Node frontend
FROM node:24-slim AS frontend-builder
WORKDIR /build
RUN npm install -g pnpm
COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY frontend/ ./
RUN pnpm run build

# 阶段4：运行镜像
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates sqlite3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=go-builder /bin/gateway /app/gateway
COPY --from=frontend-builder /build/dist /app/frontend/dist
EXPOSE 8080
ENTRYPOINT ["/app/gateway"]
