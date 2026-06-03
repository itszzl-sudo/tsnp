//! # tsnp (TypeScript Native Plugin Tool)
//!
//! Auto-generate ts-native plugin configuration from Rust crates.
//!
//! **Repository:** https://github.com/itszzl-sudo/tsnp
//!
//! ## Overview
//!
//! `tsnp` analyzes Rust crates for `#[no_mangle] extern "C"` FFI functions
//! and generates TypeScript type definitions and configuration files for
//! the ts-native runtime.
//!
//! ## Commands
//!
//! - `cargo tsnp gen <crate>` - Generate from crates.io crate
//! - `cargo tsnp new <name>` - Create empty template
//!
//! ## Output
//!
//! Generates `tsnp/<name>/` directory containing:
//! - `ts-native.toml` - Plugin configuration
//! - `index.d.ts` - TypeScript type definitions
//! - `README.md` - Usage documentation
//!
//! ## Example
//!
//! ```bash
//! cargo tsnp gen regex
//! ```
//!
//! Produces TypeScript definitions:
//! ```typescript
//! declare module "tsnp-regex" {
//!     export function regex_compile(pattern: string): number;
//!     export function regex_match(handle: number, text: string): boolean;
//! }
//! ```
//!
//! ## Links
//!
//! - [ts-native Runtime](https://github.com/itszzl-sudo/ts-native)
//! - [tsnp on GitHub](https://github.com/itszzl-sudo/tsnp)

pub mod crates_api;
pub mod github_api;
pub mod parser;
pub mod generator;
