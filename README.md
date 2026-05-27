# tsnp

Plugin generator for the tsn toolchain.

## What is it

A tool that analyzes Rust crates and generates FFI plugin configurations for tsn (TypeScript native compiler).

## What can it do

Generate configuration files that tell tsn which Rust FFI functions can be called from TypeScript.

## How to use

```bash
tsnp gen <name>
```

## Limitations

Requires Rust source code to analyze FFI functions.

## Solution

Provides three source options:
- [1] Local path - existing source code
- [2] GitHub tarball - automatic download
- [3] Search API - download only FFI files (requires token)

## Commands

### Automatic generation

```bash
tsnp gen regex

# Select source:
# [1] Local path
# [2] GitHub tarball
# [3] Search API

# Output: tsnp/regex/
```

### Manual creation

Don't want to analyze source code? Write configuration yourself:

```bash
tsnp new my-plugin
# Output: tsnp/my-plugin/
```

### Output structure

```
tsnp/<name>/
├── ts-native.toml    # Function mapping configuration
├── index.d.ts        # TypeScript type definitions
└── README.md         # Usage documentation
```

### ts-native.toml format

```toml
[package]
name = "tsnp-math"
version = "0.1.0"

[functions]
"add" = { args = ["number", "number"], ret = "number", impl_name = "add" }

[link]
lib = "math"
```

## Type mapping

| Rust | TypeScript |
|------|------------|
| i32, u32, i64, u64, f32, f64 | number |
| *const c_char | string |
| () | void |

## Complete workflow

```
1. Write FFI function in Rust
   #[no_mangle]
   pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

2. Generate configuration with tsnp
   tsnp gen my-plugin

3. Compile with tsn
   tsn main.ts
   (automatically scans tsnp/ directory)

4. Run
   ./a.exe
```

## Installation

```bash
cargo install tsnp
```

## Related tools

- **tsn** - TypeScript native compiler: `cargo install tsn`
- **cargo-tsn** - Project manager: `cargo install cargo-tsn`

## License

MIT
