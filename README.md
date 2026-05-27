# tsnp

ts-native 插件开发工具，生成 TypeScript 类型定义和配置文件。

## 安装

```bash
cargo install tsnp
```

## 能做什么

- 分析 Rust crate 的 FFI 函数
- 生成 ts-native 需要的配置文件
- 生成 TypeScript 类型定义

## 怎么做

### 从 Rust crate 生成

```bash
tsnp gen regex

# 选择源码来源：
# [1] 本地路径
# [2] 从 GitHub 下载
# [3] Search API（需要 token）
```

### 手动创建空模板

```bash
tsnp new my-plugin
```

### 输出

```
tsnp/<name>/
├── ts-native.toml    # 函数映射
├── index.d.ts        # TypeScript 类型
└── README.md         # 使用说明
```

### ts-native.toml 示例

```toml
[package]
name = "tsnp-math"
version = "0.1.0"

[functions]
"add" = { args = ["number", "number"], ret = "number", impl_name = "add" }

[link]
lib = "math"
```

## 工作流程

```
Rust FFI 函数
    ↓
tsnp gen 生成配置
    ↓
tsnp/<name>/ts-native.toml
    ↓
ts-native 编译时自动加载
```

## 类型映射

| Rust | TypeScript |
|------|------------|
| i32, u32, i64, u64, f32, f64 | number |
| *const c_char | string |
| () | void |

## 许可证

MIT
