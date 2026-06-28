# 解锁码计算

一个用于 [AstroBox NG](https://github.com/AstralSightStudios/AstroBox-NG) 的插件，根据设备的 MAC 地址和序列号（SN）计算小米设备解锁码。

## 算法

```text
unlock_code = SHA256(upper(MAC without separators) + upper(SN) + "XIAOMI")
code = 前 10 个字节分别对 0xA 取模后拼接成的 10 位数字
```

MAC 输入会自动去除 `:`、`：`（中文冒号）、`-`、空格、`.` 等分隔符，并转换为大写。

## 环境准备

```bash
rustup target add wasm32-wasip2
```

## 构建

```bash
# Debug 构建到 dist 文件夹
python3 scripts/build_dist.py

# Release 构建到 dist 文件夹
python3 scripts/build_dist.py --release

# Release 构建并打包为 .abp 插件包
python3 scripts/build_dist.py --release --package
```

构建产物位于 `dist/`，其中 `AB Unlock Code.abp` 可直接通过 AstroBox 安装。

## 依赖

- `astrobox-ng-wit` 依赖已升级为 `0.3.1`，对应 `psys-world-v3` / `api_level: 3`。
