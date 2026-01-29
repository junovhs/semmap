//! Stereotype classification for architectural role detection.

use std::path::Path;

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
    let lower = path.to_lowercase();

    if let Some(s) = classify_by_filename(&lower, path) {
        return s;
    }
    if let Some(s) = classify_by_imports(content) {
        return s;
    }
    if let Some(s) = classify_by_name_pattern(&lower, content) {
        return s;
    }
    if is_mostly_structs(content) {
        return Stereotype::Entity;
    }

    Stereotype::Unknown
}

fn classify_by_filename(lower: &str, path: &str) -> Option<Stereotype> {
    if is_config_file(lower, path) {
        return Some(Stereotype::Config);
    }
    if lower.contains("test") || lower.contains("spec") {
        return Some(Stereotype::Test);
    }
    if lower.ends_with("main.rs") || lower.ends_with("lib.rs") {
        return Some(Stereotype::Entrypoint);
    }
    if lower.contains("error") {
        return Some(Stereotype::Error);
    }
    None
}

fn classify_by_imports(content: &str) -> Option<Stereotype> {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("use clap") || t.starts_with("use structopt") {
            return Some(Stereotype::Cli);
        }
        if t.starts_with("use axum") || t.starts_with("use actix") {
            return Some(Stereotype::Handler);
        }
        if t.starts_with("use diesel") || t.starts_with("use sqlx") {
            return Some(Stereotype::Repository);
        }
    }
    None
}

fn classify_by_name_pattern(lower: &str, content: &str) -> Option<Stereotype> {
    let has_regex = content.lines().any(|l| l.trim().starts_with("use regex"));
    if lower.contains("parse") || has_regex {
        return Some(Stereotype::Parser);
    }
    if lower.contains("format") || lower.contains("render") {
        return Some(Stereotype::Formatter);
    }
    if lower.contains("util") || lower.contains("helper") {
        return Some(Stereotype::Utility);
    }
    if lower.contains("types") || lower.contains("model") {
        return Some(Stereotype::Entity);
    }
    if lower.contains("service") || lower.contains("command") {
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

fn is_config_file(lower: &str, path: &str) -> bool {
    let p = Path::new(path);
    let is_config_ext = p
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_lowercase().as_str(), "toml" | "yaml" | "yml" | "json"));

    is_config_ext || lower.contains("config") || lower.contains("cargo")
}

fn is_mostly_structs(content: &str) -> bool {
    let structs = content.matches("pub struct ").count();
    let fns = content.matches("pub fn ").count();
    structs > 2 && structs > fns
}
