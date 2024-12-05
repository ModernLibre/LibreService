# 使用官方 Rust 镜像作为基础镜像
FROM rust:1.83.0 AS builder

# 设置工作目录
WORKDIR /usr/src/app

# 复制项目的 Cargo.toml 和 Cargo.lock 文件
COPY Cargo.toml Cargo.lock ./

# 创建空的 src 目录以便缓存依赖项层
RUN mkdir src

# 构建依赖项以缓存依赖项层
RUN cargo build --release || true

# 复制项目的源代码
COPY . .

# 重新构建项目
RUN cargo build --release

# 运行可执行文件
CMD ["cargo", "run", "--release"]