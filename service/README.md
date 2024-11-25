# Bing Images Rust API

## Description

使用 `Rust` 编写的 `Bing` 图片 API，用于获取 `Bing` 每日图片，并保存到 `MongoDB` 数据库中，提供 `RESTful` 接口

## Environment

在与根目录同级的目录下创建 `config.toml` 文件

`config.toml`

```toml
[mongo]
host = "localhost"
port = 27017
username = "bing"
password = "pwd"
database = "bing"
collection = "images"
```

## Run

```bash
cargo run
```

## Build

安装 `cross` 工具

```bash
cargo install cross
```

安装 `Docker` 或 `Docker Desktop`

```bash
# Windows
choco install docker-desktop
# MacOS
brew install docker
# Linux
sudo apt-get install docker.io
```

使用 `cross` 构建 Windows 版本

```bash
cross build --target x86_64-pc-windows-gnu --release
```

构建其他平台版本

```bash
# Linux
cross build --target x86_64-unknown-linux-gnu --release
# MacOS
cross build --target x86_64-apple-darwin --release
```

如果需要构建其他平台版本，可以查看 `cross` 支持的平台