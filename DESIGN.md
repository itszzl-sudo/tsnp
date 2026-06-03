# ts-native-plugin-tool 设计文档

## 概述

自动生成 ts-native 插件配置的工具。

## 命令

```bash
cargo tsnp gen <crate-name>     # 从 GitHub 下载并生成
cargo tsnp new <plugin-name>    # 生成空模板（手动配置）
```

## 流程

### `cargo tsnp gen <crate-name>`

```
1. 查询 crates.io API
   GET https://crates.io/api/v1/crates/<name>
   → 获取 repository URL

2. GitHub 搜索 API 查找 FFI 函数
   GET https://api.github.com/search/code?q="#[no_mangle]"+repo:<owner>/<repo>
   → 获取文件列表

3. 下载文件内容
   GET https://raw.githubusercontent.com/<owner>/<repo>/<branch>/<file>
   → 解析函数签名

4. 推断类型
   - 支持类型：生成配置
   - 不支持类型：注释掉

5. 生成文件到 tsnp/<name>/
   - ts-native.toml
   - index.d.ts
   - README.md
```

### `cargo tsnp new <plugin-name>`

```
生成空模板到 tsnp/<plugin-name>/
  - ts-native.toml
  - index.d.ts
  - README.md
```

## 类型推断

### 支持的类型

| Rust 类型 | TS 类型 | 说明 |
|-----------|---------|------|
| i8/u8/i16/u16/i32/u32/i64/u64 | number | 整数 |
| usize/isize | number | 平台相关整数 |
| f32/f64 | number | 浮点 |
| *const T/*mut T | number | 指针 |
| &T/&mut T | number | 引用 |
| *const c_char | string | C 字符串 |
| () | void | 无返回 |

### 不支持的类型

- Option<T>/Result<T, E>
- Vec<T>/String/&str
- 结构体按值传递
- 回调函数
- 泛型

处理方式：在配置中注释掉，用户手动配置。

## 输出目录

```
tsnp/
└─ <name>/
   ├─ ts-native.toml
   ├─ index.d.ts
   └─ README.md
```

### 目录处理

- tsnp/ 不存在：直接创建
- tsnp/ 存在且为空：使用
- tsnp/ 存在且非空：提示用户清空或取消
- 同名目录已存在：覆盖

## 版本

不指定版本，版本跟随项目 Cargo.toml 中的依赖版本。

## 命名

- 插件目录名：`tsnp/<crate-name>`
- ts-native.toml 中 name：`tsnp-<crate-name>`
- 不支持改名

## 链接

```toml
[link]
lib = "<original-crate-name>"
```

直接链接原 crate，不创建新 crate，不重新导出。

## 发布

不发布到 crates.io。

## GitHub API 限流

- 未认证：60 次/小时
- 已认证：5000 次/小时

处理：限流时提示用户配置 GitHub token。

## 文件模板

### ts-native.toml

```toml
[package]
name = "tsnp-<name>"
version = "0.1.0"

[functions]
# Example:
# "document.createElement" = { impl_name = "js_dom_create_element" }
# "element.textContent" = { impl_name = "js_dom_set_text" }

[link]
lib = "<name>"
```

### index.d.ts

```typescript
declare module "tsnp-<name>" {
    // Example:
    // export function createElement(tag: string): number;
    // export function textContent(el: number, text: string): void;
}
```

### README.md

```markdown
# tsnp-<name>

ts-native plugin for [<name>](https://crates.io/crates/<name>)

## Usage

1. Add dependency to your project's Cargo.toml:
   ```toml
   [dependencies]
   <name> = "0.1.0"
   ```

2. Configure ts-native.toml with function mappings

3. Import in TypeScript:
   ```typescript
   import { createElement } from "tsnp-<name>";
   ```

## Manual Configuration

Edit `ts-native.toml` to add functions:

```toml
[functions]
"function.name" = { impl_name = "rust_function_name" }
```

See [ts-native docs](https://github.com/...) for details.
```

## 命令行输出

### gen 命令

```
$ cargo tsnp gen dom

Generated: tsnp/dom/
  - ts-native.toml
  - index.d.ts
  - README.md

See README.md for configuration guide.
```

### new 命令

```
$ cargo tsnp new my-plugin

Generated: tsnp/my-plugin/
  - ts-native.toml
  - index.d.ts
  - README.md

See README.md for configuration guide.
```

### 目录非空

```
$ cargo tsnp gen dom

Warning: tsnp/ is not empty
[1] Clear and continue
[2] Cancel

Choice:
```

### 无 FFI 函数

```
$ cargo tsnp gen dom

No #[no_mangle] extern "C" functions found.
Use 'cargo tsnp new dom' for manual configuration.
```

## 实现模块

1. **crates.io API** - 获取 crate 元数据
2. **GitHub API** - 搜索代码、下载文件
3. **Rust 解析器** - 提取函数签名（基于 syn）
4. **类型推断器** - 映射 Rust → TS 类型
5. **模板生成器** - 生成 toml/d.ts/md
6. **CLI** - 命令行界面

## 依赖

- reqwest - HTTP 客户端
- syn - Rust 源码解析
- serde/serde_json - JSON 解析
- clap - 命令行参数
