# Task: Deprecate `semmap update`

## Why
`generate` is now smart enough (doc extraction, SWUM, stereotypes) that manual SEMMAP editing shouldn't be needed. If descriptions are wrong, fix the source code's `//!` comments.

One command. No confusion. SEMMAP becomes a pure derived artifact.

## Files to Modify

1. **src/main.rs** - Remove `Update` from CLI enum
2. **src/commands.rs** - Delete `update()` function
3. **src/commands/update_helpers.rs** - Delete entire file
4. **src/lib.rs** - Remove `update_helpers` module if exposed
5. **tests/update_tests.rs** - Delete
6. **tests/update_more_tests.rs** - Delete

## Verification

```bash
cargo build  # Should compile without update
cargo test   # Remaining tests pass
semmap --help  # No update command listed
```

## Done When
- `semmap update` doesn't exist
- All tests pass
- `slopchop check` passes
