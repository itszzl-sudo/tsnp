# tsnp

## 是什么

ts-native 插件开发工具。

## 能做什么

生成配置文件，告诉 ts-native 可以调用哪些 Rust FFI 函数。

## 怎么做

```bash
tsnp gen <name>
```

## 有什么缺陷

需要 Rust 源码才能分析 FFI 函数。

## 怎么弥补

提供三种源码来源：
- [1] 本地路径 - 已有源码
- [2] GitHub tarball - 自动下载
- [3] Search API - 只下载 FFI 文件（需要 token）

## 用什么工具

tsnp 自己就是工具。

## 这个工具怎么用

### 自动生成

```bash
tsnp gen regex

# 选择源码：
# [1] 本地路径
# [2] GitHub tarball
# [3] Search API

# 输出：tsnp/regex/
```

### 手动创建

不想分析源码？自己写配置：

```bash
tsnp new my-plugin
# 输出：tsnp/my-plugin/
```

### 输出什么

```
tsnp/<name>/
├── ts-native.toml    # 函数映射配置
├── index.d.ts        # TypeScript 类型定义
└── README.md         # 使用说明
```

### ts-native.toml 内容

```toml
[package]
name = "tsnp-math"
version = "0.1.0"

[functions]
"add" = { args = ["number", "number"], ret = "number", impl_name = "add" }

[link]
lib = "math"
```

## 类型映射

| Rust | TypeScript |
|------|------------|
| i32, u32, i64, u64, f32, f64 | number |
| *const c_char | string |
| () | void |

## 完整流程

```
1. Rust 写 FFI 函数
   #[no_mangle]
   pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

2. tsnp 生成配置
   tsnp gen my-plugin

3. ts-native 编译
   ts-native main.ts
   （自动扫描 tsnp/ 目录）

4. 运行
   ./a.exe
```

## 许可证

MIT
