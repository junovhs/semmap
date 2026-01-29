//! Stereotype classification for architectural role detection.
//! Infers a file's role from its name, imports, and code patterns.

/// Architectural stereotypes for code classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stereotype {
    Config,
    Entrypoint,
    Entity,
    Service,
    Repository,
    Handler,
    Utility,
    Parser,
    Formatter,
    Error,
    Cli,
    Test,
    Unknown,
}

/// Classify a file into an architectural stereotype.
pub fn classify(path: &str, content: &str) -> Stereotype {
    let lower_path = path.to_lowercase();

    // Check filename-based stereotypes first (fast path)
    if let Some(s) = classify_by_filename(&lower_path) {
        return s;
    }

    // Check import-based stereotypes
    if let Some(s) = classify_by_imports(content) {
        return s;
    }

    // Check name-based patterns
    if let Some(s) = classify_by_name_pattern(&lower_path, content) {
        return s;
    }

    // Check content-based patterns
    if is_mostly_structs(content) {
        return Stereotype::Entity;
    }

    Stereotype::Unknown
}

/// Classify based on filename patterns.
fn classify_by_filename(path: &str) -> Option<Stereotype> {
    if is_config_file(path) {
        return Some(Stereotype::Config);
    }
    if is_test_file(path) {
        return Some(Stereotype::Test);
    }
    if is_entrypoint(path) {
        return Some(Stereotype::Entrypoint);
    }
    if path.contains("error") || path.contains("err.") {
        return Some(Stereotype::Error);
    }
    None
}

/// Classify based on import statements.
fn classify_by_imports(content: &str) -> Option<Stereotype> {
    if has_cli_imports(content) {
        return Some(Stereotype::Cli);
    }
    if has_http_imports(content) {
        return Some(Stereotype::Handler);
    }
    if has_db_imports(content) {
        return Some(Stereotype::Repository);
    }
    None
}

/// Classify based on name patterns.
fn classify_by_name_pattern(path: &str, content: &str) -> Option<Stereotype> {
    if path.contains("parse") || has_regex_imports(content) {
        return Some(Stereotype::Parser);
    }
    if path.contains("format") || path.contains("render") {
        return Some(Stereotype::Formatter);
    }
    if path.contains("util") || path.contains("helper") {
        return Some(Stereotype::Utility);
    }
    if path.contains("types") || path.contains("model") {
        return Some(Stereotype::Entity);
    }
    if path.contains("service") || path.contains("command") {
        return Some(Stereotype::Service);
    }
    None
}

/// Get the WHY description for a stereotype.
pub fn stereotype_to_why(s: Stereotype) -> &'static str {
    match s {
        Stereotype::Config => "Centralizes project configuration.",
        Stereotype::Entrypoint => "Provides application entry point.",
        Stereotype::Entity => "Defines domain data structures.",
        Stereotype::Service => "Orchestrates business logic.",
        Stereotype::Repository => "Handles data persistence.",
        Stereotype::Handler => "Handles HTTP/API requests.",
        Stereotype::Utility => "Provides reusable helper functions.",
        Stereotype::Parser => "Parses input into structured data.",
        Stereotype::Formatter => "Formats data for output.",
        Stereotype::Error => "Defines error types and handling.",
        Stereotype::Cli => "Defines command-line interface.",
        Stereotype::Test => "Verifies correctness.",
        Stereotype::Unknown => "Supports application functionality.",
    }
}

fn is_config_file(path: &str) -> bool {
    let config_exts = [".toml", ".yaml", ".yml", ".json"];
    let config_names = ["config", "settings", "cargo", "package", "tsconfig"];

    config_exts.iter().any(|e| path.ends_with(e)) || config_names.iter().any(|n| path.contains(n))
}

fn is_test_file(path: &str) -> bool {
    path.contains("test") || path.contains("spec") || path.contains("_test.")
}

fn is_entrypoint(path: &str) -> bool {
    path.ends_with("main.rs") || path.ends_with("lib.rs") || path.ends_with("mod.rs")
}

fn has_cli_imports(content: &str) -> bool {
    content.contains("use clap")
        || content.contains("use structopt")
        || content.contains("use argh")
}

fn has_http_imports(content: &str) -> bool {
    content.contains("use axum")
        || content.contains("use actix")
        || content.contains("use rocket")
        || content.contains("use warp")
}

fn has_db_imports(content: &str) -> bool {
    content.contains("use diesel")
        || content.contains("use sqlx")
        || content.contains("use rusqlite")
}

fn has_regex_imports(content: &str) -> bool {
    content.contains("use regex")
}

fn is_mostly_structs(content: &str) -> bool {
    let struct_count = content.matches("pub struct ").count();
    let fn_count = content.matches("pub fn ").count();
    struct_count > 2 && struct_count > fn_count
}
