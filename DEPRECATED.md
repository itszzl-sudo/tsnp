# ⚠️ DEPRECATED

This project has been deprecated. Please use [cargo-tsn](https://github.com/itszzl-sudo/cargo-tsn) instead.

## Migration Guide

### Old Workflow (tsnp)
```bash
tsnp gen crypto      # Generate from GitHub
tsnp new crypto      # Create empty template
```

### New Workflow (cargo-tsn)
```bash
# 1. Write TypeScript declarations
declare function crypto_sha256(data: string): string;

# 2. Generate plugin scaffold
cargo tsn prepare

# 3. Copy to project
cp -r prepare/tsnp/crypto tsnp/

# 4. Implement C functions
# Edit tsnp/crypto/crypto_win.c
```

## Why Migrate?

1. **Better Integration**: `cargo-tsn` is the official ts-native project manager
2. **More Features**: `prepare` command generates complete plugin scaffolds with stub functions
3. **Simpler Workflow**: One tool instead of two
4. **Active Development**: `cargo-tsn` is actively maintained

## Timeline

- **2024-06**: `cargo-tsn` v0.2.0 released with `prepare` command
- **2024-06**: `tsnp` marked as deprecated
- **Future**: `tsnp` will be archived

## Need Help?

- [cargo-tsn Documentation](https://github.com/itszzl-sudo/cargo-tsn)
- [ts-native Documentation](https://github.com/itszzl-sudo/ts-native)
