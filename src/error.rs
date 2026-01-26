use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SemmapError {
    Io(io::Error),
    Parse(ParseError),
    Validation(Vec<ValidationIssue>),
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub line: Option<usize>,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl fmt::Display for SemmapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Parse(e) => write!(f, "Parse error at line {}: {}", e.line, e.message),
            Self::Validation(issues) => {
                writeln!(f, "Validation failed with {} issues:", issues.len())?;
                for issue in issues {
                    writeln!(f, "  {:?}: {}", issue.severity, issue.message)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SemmapError {}

impl From<io::Error> for SemmapError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl ValidationIssue {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            line: None,
            path: None,
            message: message.into(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            line: None,
            path: None,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    #[must_use]
    pub fn for_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}
