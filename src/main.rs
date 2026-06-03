use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use std::fs;

mod crates_api;
mod github_api;
mod parser;
mod generator;

#[derive(Parser)]
#[command(name = "cargo-tsnp")]
#[command(about = "Generate ts-native plugin configuration")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate from GitHub")]
    Gen {
        #[arg(help = "Crate name")]
        name: String,
    },
    #[command(about = "Create empty template")]
    New {
        #[arg(help = "Plugin name")]
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Gen { name } => gen_plugin(&name),
        Commands::New { name } => new_plugin(&name),
    }
}

fn gen_plugin(name: &str) -> Result<()> {
    // 验证crate名称
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        anyhow::bail!("Invalid crate name: {}", name);
    }
    
    println!("Fetching crate '{}' from crates.io...", name);
    
    let crate_info = crates_api::fetch_crate(name)
        .context("Failed to fetch crate info from crates.io")?;
    
    let repo_url = crate_info.crate_.repository
        .ok_or_else(|| anyhow::anyhow!("Crate has no repository URL"))?;
    
    println!("\nChoose source for FFI function analysis:");
    println!("[1] Local source code (specify local path)");
    println!("[2] Download from GitHub (no token required, slower)");
    println!("[3] Download from GitHub with token (faster via Search API)");
    print!("\nChoice: ");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .context("Failed to read input")?;
    
    let functions = match input.trim() {
        "1" => {
            print!("Enter local path to crate source: ");
            let mut path = String::new();
            std::io::stdin().read_line(&mut path)
                .context("Failed to read input")?;
            let path = path.trim();
            
            println!("Scanning local source at '{}'...", path);
            github_api::find_ffi_functions_local(path)
                .context("Failed to scan local source")
        }
        "2" => {
            println!("Downloading from GitHub (tarball)...");
            github_api::find_ffi_functions_tarball(&repo_url)
                .context("Failed to download from GitHub")
        }
        "3" => {
            print!("Enter GitHub token (or press Enter to use GITHUB_TOKEN env): ");
            let mut token = String::new();
            std::io::stdin().read_line(&mut token)
                .context("Failed to read input")?;
            let token = token.trim();
            
            let token = if token.is_empty() {
                std::env::var("GITHUB_TOKEN")
                    .context("GITHUB_TOKEN environment variable not set")?
            } else {
                token.to_string()
            };
            
            println!("Downloading from GitHub (Search API)...");
            github_api::find_ffi_functions_search(&repo_url, &token)
                .context("Failed to search GitHub")
        }
        _ => {
            println!("Cancelled.");
            return Ok(());
        }
    }?;
    
    if functions.is_empty() {
        println!("No #[no_mangle] extern \"C\" functions found.");
        println!("Use 'cargo tsnp new {}' for manual configuration.", name);
        return Ok(());
    }
    
    let (supported, unsupported) = parser::categorize_functions(functions);
    
    println!("Found {} supported, {} unsupported functions", 
        supported.len(), unsupported.len());
    
    prepare_tsnp_dir()?;
    
    let output_dir = format!("tsnp/{}", name);
    fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;
    
    generator::generate(&output_dir, name, &supported, &unsupported)?;
    
    println!("\nGenerated: {}/", output_dir);
    println!("  - ts-native.toml");
    println!("  - index.d.ts");
    println!("  - README.md");
    println!("\nSee README.md for configuration guide.");
    
    Ok(())
}

fn new_plugin(name: &str) -> Result<()> {
    // 验证插件名称
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        anyhow::bail!("Invalid plugin name: {}", name);
    }
    
    println!("Creating plugin: {}", name);
    println!("Current directory: {:?}", std::env::current_dir());
    
    prepare_tsnp_dir()?;
    
    let output_dir = format!("tsnp/{}", name);
    println!("Output directory: {}", output_dir);
    
    fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;
    
    generator::generate_empty(&output_dir, name)?;
    
    println!("\nGenerated: {}/", output_dir);
    println!("  - ts-native.toml");
    println!("  - index.d.ts");
    println!("  - README.md");
    println!("\nSee README.md for configuration guide.");
    
    Ok(())
}

fn prepare_tsnp_dir() -> Result<()> {
    let tsnp_dir = "tsnp";
    
    if !std::path::Path::new(tsnp_dir).exists() {
        fs::create_dir_all(tsnp_dir)
            .context("Failed to create tsnp directory")?;
        return Ok(());
    }
    
    let is_empty = fs::read_dir(tsnp_dir)
        .map(|mut d| d.next().is_none())
        .unwrap_or(false);
    
    if !is_empty {
        println!("Warning: tsnp/ is not empty");
        println!("[1] Clear and continue");
        println!("[2] Cancel");
        print!("\nChoice: ");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .context("Failed to read input")?;
        
        match input.trim() {
            "1" => {
                fs::remove_dir_all(tsnp_dir)
                    .context("Failed to clear tsnp directory")?;
                fs::create_dir_all(tsnp_dir)
                    .context("Failed to create tsnp directory")?;
            }
            _ => {
                println!("Cancelled.");
                std::process::exit(0);
            }
        }
    }
    
    Ok(())
}
