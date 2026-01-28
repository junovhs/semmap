use crate::types::DepKind;
use regex::Regex;

pub fn extract_imports(content: &str) -> Vec<(String, DepKind)> {
    let mut deps = Vec::new();
    
    extract_relative_imports(content, &mut deps);
    extract_simple_imports(content, &mut deps);

    deps
}

fn extract_relative_imports(content: &str, deps: &mut Vec<(String, DepKind)>) {
    let Some(re) = Regex::new(r"from\s+\.(\w+)\s+import").ok() else {
        return;
    };
    
    for cap in re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            deps.push((format!("{}.py", m.as_str()), DepKind::Import));
        }
    }
}

fn extract_simple_imports(content: &str, deps: &mut Vec<(String, DepKind)>) {
    let Some(re) = Regex::new(r"import\s+(\w+)").ok() else {
        return;
    };

    for cap in re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            let module = m.as_str();
            if !is_stdlib(module) {
                deps.push((format!("{module}.py"), DepKind::Import));
            }
        }
    }
}

fn is_stdlib(module: &str) -> bool {
    let stdlib = ["os", "sys", "re", "json", "typing", "collections", "pathlib"];
    stdlib.contains(&module)
}