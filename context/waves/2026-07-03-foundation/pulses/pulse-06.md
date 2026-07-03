# Pulse 06: Frame index ergonomics

## Goal

Make `frames-core` easier for AI tools and downstream Rust callers to use
without learning the full struct shape first.

## Changes

- Add `FrameQuery::new`, `with_kind`, and `with_tags`.
- Add `FrameIndex::search_top`.
- Add `FrameIndex::by_kind` and `with_tag`.
- Add runnable `examples/lookup.rs`.
- Update README usage and validation commands.
- Add tests for helper behavior.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run --example lookup`
- `git diff --check`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
