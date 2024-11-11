# Bing Images Rust API

## Environment

`/config/default.toml`

```toml
[database]
uri = "mongodb://localhost:27017"
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

安装 `openssl` 开发包

```bash
# Windows
choco install openssl
# MacOS
brew install openssl
# Linux
sudo apt-get install libssl-dev
```

使用 `cross` 构建 Windows 版本

```bash
cross build --target x86_64-pc-windows-gnu --release
```

构建其他平台版本

```bash
cross build --target x86_64-unknown-linux-gnu --release
cross build --target x86_64-apple-darwin --release
```
