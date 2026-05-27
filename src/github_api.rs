use syn::{ItemFn, Signature, Type, FnArg, PatType, ReturnType};

pub struct FFIFunction {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: String,
    pub raw_signature: String,
}

pub fn find_ffi_functions_local(path: &str) -> Result<Vec<FFIFunction>, String> {
    use std::path::Path;
    
    let base_path = Path::new(path);
    
    if !base_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    let src_path = base_path.join("src");
    let scan_path = if src_path.exists() { &src_path } else { base_path };
    
    println!("Scanning directory: {}", scan_path.display());
    
    let mut rust_files = Vec::new();
    scan_rust_files(scan_path, &mut rust_files);
    
    println!("Found {} Rust source files", rust_files.len());
    
    let mut functions = Vec::new();
    
    for (filename, content) in rust_files {
        let funcs = parse_ffi_functions(&content);
        if !funcs.is_empty() {
            println!("  {}: {} FFI functions", filename, funcs.len());
        }
        functions.extend(funcs);
    }
    
    Ok(functions)
}

fn scan_rust_files(dir: &std::path::Path, files: &mut Vec<(String, String)>) {
    use std::fs;
    use std::io::Read;
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_rust_files(&path, files);
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Ok(mut file) = fs::File::open(&path) {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        let filename = path.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "unknown".to_string());
                        files.push((filename, content));
                    }
                }
            }
        }
    }
}

pub fn find_ffi_functions_tarball(repo_url: &str) -> Result<Vec<FFIFunction>, String> {
    let (owner, repo) = parse_github_url(repo_url)?;
    
    println!("Downloading tarball from GitHub: {}/{}", owner, repo);
    
    let tarball_url = format!(
        "https://api.github.com/repos/{}/{}/tarball",
        owner, repo
    );
    
    let response = reqwest::blocking::Client::new()
        .get(&tarball_url)
        .header("User-Agent", "tsnp/0.1.0")
        .send()
        .map_err(|e| format!("GitHub download error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub download failed: {}", response.status()));
    }
    
    let tarball_data = response.bytes()
        .map_err(|e| format!("Failed to read tarball: {}", e))?;
    
    println!("Parsing tarball ({} bytes)...", tarball_data.len());
    
    let rust_files = extract_rust_files(&tarball_data)?;
    
    println!("Found {} Rust source files", rust_files.len());
    
    let mut functions = Vec::new();
    
    for (filename, content) in rust_files {
        let funcs = parse_ffi_functions(&content);
        if !funcs.is_empty() {
            println!("  {}: {} FFI functions", filename, funcs.len());
        }
        functions.extend(funcs);
    }
    
    Ok(functions)
}

pub fn find_ffi_functions_search(repo_url: &str, token: &str) -> Result<Vec<FFIFunction>, String> {
    let (owner, repo) = parse_github_url(repo_url)?;
    
    println!("Searching GitHub code with Search API: {}/{}", owner, repo);
    
    let query = format!("repo:{}/{} extension:rs \"#[no_mangle]\"", owner, repo);
    let search_url = format!(
        "https://api.github.com/search/code?q={}",
        urlencoding::encode(&query)
    );
    
    let response = reqwest::blocking::Client::new()
        .get(&search_url)
        .header("User-Agent", "tsnp/0.1.0")
        .header("Authorization", format!("token {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .map_err(|e| format!("GitHub Search API error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub Search API failed: {}", response.status()));
    }
    
    #[derive(serde::Deserialize)]
    struct SearchResult {
        items: Vec<SearchItem>,
    }
    
    #[derive(serde::Deserialize)]
    struct SearchItem {
        name: String,
        #[allow(dead_code)]
        path: String,
        url: String,
    }
    
    let search_result: SearchResult = response
        .json()
        .map_err(|e| format!("JSON parse error: {}", e))?;
    
    println!("Found {} files with #[no_mangle]", search_result.items.len());
    
    let mut functions = Vec::new();
    
    for item in search_result.items {
        let file_response = reqwest::blocking::Client::new()
            .get(&item.url)
            .header("User-Agent", "tsnp/0.1.0")
            .header("Authorization", format!("token {}", token))
            .header("Accept", "application/vnd.github.v3.raw")
            .send()
            .ok();
        
        if let Some(resp) = file_response {
            if let Ok(content) = resp.text() {
                let funcs = parse_ffi_functions(&content);
                if !funcs.is_empty() {
                    println!("  {}: {} FFI functions", item.name, funcs.len());
                }
                functions.extend(funcs);
            }
        }
    }
    
    Ok(functions)
}

fn parse_github_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim_end_matches('/');
    
    if url.contains("github.com") {
        let parts: Vec<&str> = url.split('/').collect();
        let len = parts.len();
        if len >= 2 {
            return Ok((parts[len-2].to_string(), parts[len-1].to_string()));
        }
    }
    
    Err("Invalid GitHub URL".to_string())
}

fn extract_rust_files(tarball: &[u8]) -> Result<Vec<(String, String)>, String> {
    use flate2::read::GzDecoder;
    use tar::Archive;
    use std::io::Read;
    
    let gz = GzDecoder::new(tarball);
    let mut archive = Archive::new(gz);
    
    let mut rust_files = Vec::new();
    
    for entry in archive.entries().map_err(|e| format!("Tar error: {}", e))? {
        let mut entry = entry.map_err(|e| format!("Entry error: {}", e))?;
        
        let path = entry.path().map_err(|e| format!("Path error: {}", e))?;
        let path_str = path.to_string_lossy().to_string();
        
        if path_str.ends_with(".rs") && path_str.contains("/src/") {
            let mut content = String::new();
            entry.read_to_string(&mut content).ok();
            
            let filename = path_str.rsplit('/').next().unwrap_or("unknown").to_string();
            
            rust_files.push((filename, content));
        }
    }
    
    Ok(rust_files)
}

fn parse_ffi_functions(content: &str) -> Vec<FFIFunction> {
    let mut functions = Vec::new();
    
    let parsed = match syn::parse_file(content) {
        Ok(p) => p,
        Err(_) => return functions,
    };
    
    for item in parsed.items {
        if let syn::Item::Fn(ItemFn { 
            attrs, 
            sig: Signature { ident, inputs, output, .. }, 
            ..
        }) = item {
            let has_no_mangle = attrs.iter().any(|attr| {
                attr.path.is_ident("no_mangle")
            });
            
            if has_no_mangle {
                let params: Vec<String> = inputs.iter()
                    .filter_map(|arg| {
                        if let FnArg::Typed(PatType { ty, .. }) = arg {
                            Some(type_to_string(ty))
                        } else {
                            None
                        }
                    })
                    .collect();
                
                let return_type = match output {
                    ReturnType::Default => "void".to_string(),
                    ReturnType::Type(_, ty) => type_to_string(&ty),
                };
                
                let raw_sig = format!(
                    "fn({}) -> {}",
                    params.join(", "),
                    return_type
                );
                
                functions.push(FFIFunction {
                    name: ident.to_string(),
                    params,
                    return_type,
                    raw_signature: raw_sig,
                });
            }
        }
    }
    
    functions
}

fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            let seg = p.path.segments.last().unwrap();
            seg.ident.to_string()
        }
        Type::Ptr(p) => {
            let inner = type_to_string(&p.elem);
            if p.mutability.is_some() {
                format!("*mut {}", inner)
            } else {
                format!("*const {}", inner)
            }
        }
        Type::Reference(r) => {
            let inner = type_to_string(&r.elem);
            if r.mutability.is_some() {
                format!("&mut {}", inner)
            } else {
                format!("&{}", inner)
            }
        }
        _ => "unknown".to_string(),
    }
}
