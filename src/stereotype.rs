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

    if let Some(s) = classify_by_filename(&lower_path) {
        return s;
    }
    if let Some(s) = classify_by_imports(content) {
        return s;
    }
    if let Some(s) = classify_by_name_pattern(&lower_path, content) {
        return s;
    }
    if is_mostly_structs(content) {
        return Stereotype::Entity;
    }

    Stereotype::Unknown
}

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

fn classify_by_imports(content: &str) -> Option<Stereotype> {
    // Parse actual use statements - check line starts with "use crate::"
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("use clap") || t.starts_with("use structopt") || t.starts_with("use argh")
        {
            return Some(Stereotype::Cli);
        }
        if t.starts_with("use axum") || t.starts_with("use actix") || t.starts_with("use rocket") {
            return Some(Stereotype::Handler);
        }
        if t.starts_with("use diesel") || t.starts_with("use sqlx") || t.starts_with("use rusqlite")
        {
            return Some(Stereotype::Repository);
        }
    }
    None
}

fn classify_by_name_pattern(path: &str, content: &str) -> Option<Stereotype> {
    let has_regex = content.lines().any(|l| l.trim().starts_with("use regex"));
    if path.contains("parse") || has_regex {
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
    path.ends_with(".toml")
        || path.ends_with(".yaml")
        || path.ends_with(".yml")
        || path.ends_with(".json")
        || path.contains("config")
        || path.contains("cargo")
}

fn is_test_file(path: &str) -> bool {
    path.contains("test") || path.contains("spec")
}

fn is_entrypoint(path: &str) -> bool {
    path.ends_with("main.rs") || path.ends_with("lib.rs") || path.ends_with("mod.rs")
}

fn is_mostly_structs(content: &str) -> bool {
    let structs = content.matches("pub struct ").count();
    let fns = content.matches("pub fn ").count();
    structs > 2 && structs > fns
}
