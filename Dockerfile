# 使用官方Rust镜像作为构建环境
FROM rust:1.88-slim AS builder

# 设置工作目录
WORKDIR /app

# 复制Cargo.toml
COPY Cargo.toml ./

# 复制crates目录
COPY crates ./crates

# 构建项目
RUN cargo build --release

# 使用更小的运行时镜像
FROM debian:bookworm-slim

# 安装必要的运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -r -s /bin/false app

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/permission-api /app/permission-api

# 复制配置文件（如果有）
COPY --chown=app:app sql/ ./sql/

# 切换到非root用户
USER app

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info

# 运行应用
CMD ["./permission-api"] 