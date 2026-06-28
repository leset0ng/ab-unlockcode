# AstroBox NG Plugin Template (Rust)

一个用于 [AstroBox NG](https://github.com/AstralSightStudios/AstroBox-NG) 的 Rust 插件模板项目。

## 环境准备

### 1. 安装 Rust

👉 https://www.rust-lang.org/learn/get-started

### 2. 安装 Python 3

构建脚本使用 Python 编写，需要安装 Python 3。

👉 https://www.python.org/downloads/

### 3. 安装 wasm32-wasip2 编译目标

```bash
rustup target add wasm32-wasip2
```

## 构建

```bash
# Debug 构建到 dist 文件夹
python scripts/build_dist.py

# Release 构建到 dist 文件夹
python scripts/build_dist.py --release

# Release 构建并打包为 .abp 插件包
python scripts/build_dist.py --release --package
```

构建产物会输出到 `dist/` 目录，包含编译后的 wasm 文件、`manifest.json` 和图标。

使用 `--package` 时会额外生成一个 `.abp` 文件，可直接通过 AstroBox 安装。
