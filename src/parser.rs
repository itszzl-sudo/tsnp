use crate::github_api::FFIFunction;

pub struct SupportedFunction {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: String,
}

pub struct UnsupportedFunction {
    pub name: String,
    pub raw_signature: String,
    pub reason: String,
}

pub fn categorize_functions(functions: Vec<FFIFunction>) -> (Vec<SupportedFunction>, Vec<UnsupportedFunction>) {
    let mut supported = Vec::new();
    let mut unsupported = Vec::new();
    
    for func in functions {
        if is_supported(&func.params, &func.return_type) {
            supported.push(SupportedFunction {
                name: func.name,
                params: func.params,
                return_type: func.return_type,
            });
        } else {
            unsupported.push(UnsupportedFunction {
                name: func.name,
                raw_signature: func.raw_signature,
                reason: "Unsupported type".to_string(),
            });
        }
    }
    
    (supported, unsupported)
}

fn is_supported(params: &[String], return_type: &str) -> bool {
    params.iter().all(|p| is_supported_type(p)) && is_supported_type(return_type)
}

fn is_supported_type(ty: &str) -> bool {
    let ty = ty.trim();
    
    if ty == "void" || ty == "()" {
        return true;
    }
    
    let numeric_types = [
        "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64",
        "isize", "usize", "f32", "f64",
        "c_char", "c_uchar", "c_short", "c_ushort", "c_int", "c_uint",
        "c_long", "c_ulong", "c_longlong", "c_ulonglong", "c_float", "c_double",
    ];
    
    if numeric_types.contains(&ty) {
        return true;
    }
    
    if ty.starts_with("*const") || ty.starts_with("*mut") || ty.starts_with("&") {
        return true;
    }
    
    false
}

pub fn rust_type_to_ts(ty: &str) -> String {
    let ty = ty.trim();
    
    if ty == "void" || ty == "()" {
        return "void".to_string();
    }
    
    if ty.starts_with("*const c_char") {
        return "string".to_string();
    }
    
    "number".to_string()
}
