//! SWUM (Software Word Usage Model) for identifier expansion.
//! Converts function/file names into readable sentences.

/// Expand a file stem or function name into a readable description.
pub fn expand_identifier(name: &str) -> String {
    let words = split_identifier(name);

    if words.is_empty() {
        return format!("Implements {name} functionality.");
    }

    let first = words.first().map_or("", String::as_str);
    let rest: String = words
        .iter()
        .skip(1)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");

    expand_verb_pattern(first, &rest)
}

/// Map verb to sentence pattern.
fn expand_verb_pattern(verb: &str, rest: &str) -> String {
    match verb {
        "get" | "fetch" | "load" | "read" | "retrieve" => format!("Gets the {rest}."),
        "set" | "update" | "write" | "save" | "store" => format!("Sets the {rest}."),
        "is" | "has" | "can" | "should" | "will" => format!("Checks if {rest}."),
        "create" | "new" | "build" | "make" | "init" => format!("Creates {rest}."),
        "delete" | "remove" | "drop" | "clear" => format!("Removes {rest}."),
        "parse" | "extract" | "decode" => format!("Parses {rest}."),
        "validate" | "check" | "verify" => format!("Validates {rest}."),
        "render" | "format" | "display" | "print" => format!("Formats {rest} for output."),
        "handle" | "process" | "run" | "exec" => format!("Processes {rest}."),
        "convert" | "transform" | "map" => format!("Converts {rest}."),
        "find" | "search" | "lookup" | "query" => format!("Finds {rest}."),
        "test" | "spec" => format!("Tests {rest}."),
        _ => default_expansion(verb, rest),
    }
}

/// Default expansion for unrecognized verbs.
fn default_expansion(verb: &str, rest: &str) -> String {
    if rest.is_empty() {
        format!("Implements {verb} functionality.")
    } else {
        format!("Implements {verb} {rest}.")
    }
}

/// Split identifier into words (handles `snake_case` and `camelCase`).
fn split_identifier(name: &str) -> Vec<String> {
    if name.contains('_') {
        return name
            .split('_')
            .filter(|s| !s.is_empty())
            .map(str::to_lowercase)
            .collect();
    }
    split_camel_case(name)
}

/// Split `camelCase` or `PascalCase` into words.
fn split_camel_case(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();

    for ch in name.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            words.push(current.to_lowercase());
            current = String::new();
        }
        current.push(ch);
    }

    if !current.is_empty() {
        words.push(current.to_lowercase());
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case() {
        assert_eq!(
            expand_identifier("get_user_profile"),
            "Gets the user profile."
        );
        assert_eq!(expand_identifier("parse_config"), "Parses config.");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(
            expand_identifier("getUserProfile"),
            "Gets the user profile."
        );
        assert_eq!(expand_identifier("validateInput"), "Validates input.");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(
            expand_identifier("parser"),
            "Implements parser functionality."
        );
    }
}
