use semmap::path_utils::{build_root_prefix, prefix_path, strip_prefix_for_lookup};

/// Current directory should produce empty prefix.
#[test]
fn test_build_root_prefix_current_dir() {
    assert_eq!(build_root_prefix(".".as_ref()), "");
    assert_eq!(build_root_prefix("./".as_ref()), "");
}

/// Relative path with ./ prefix should strip it.
#[test]
fn test_build_root_prefix_relative() {
    assert_eq!(build_root_prefix("./crates".as_ref()), "crates");
    assert_eq!(build_root_prefix("./src/lib".as_ref()), "src/lib");
}

/// Plain relative path should pass through.
#[test]
fn test_build_root_prefix_plain() {
    assert_eq!(build_root_prefix("crates".as_ref()), "crates");
    assert_eq!(build_root_prefix("src".as_ref()), "src");
}

/// Empty prefix should return path unchanged.
#[test]
fn test_prefix_path_empty() {
    assert_eq!(prefix_path("", "foo.rs"), "foo.rs");
    assert_eq!(prefix_path("", "src/lib.rs"), "src/lib.rs");
}

/// Non-empty prefix should be prepended with slash.
#[test]
fn test_prefix_path_with_prefix() {
    assert_eq!(prefix_path("crates", "foo.rs"), "crates/foo.rs");
    assert_eq!(prefix_path("src/lib", "mod.rs"), "src/lib/mod.rs");
}

/// Empty prefix should return path unchanged.
#[test]
fn test_strip_prefix_empty() {
    assert_eq!(strip_prefix_for_lookup("", "foo.rs"), "foo.rs");
}

/// Should strip prefix with trailing slash.
#[test]
fn test_strip_prefix_with_prefix() {
    assert_eq!(strip_prefix_for_lookup("crates", "crates/foo.rs"), "foo.rs");
    assert_eq!(strip_prefix_for_lookup("src", "src/lib.rs"), "lib.rs");
}

/// Path without prefix should be returned unchanged.
#[test]
fn test_strip_prefix_no_match() {
    assert_eq!(
        strip_prefix_for_lookup("crates", "other/foo.rs"),
        "other/foo.rs"
    );
}
