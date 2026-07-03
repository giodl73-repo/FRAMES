# Pulse 05: Frame index crate

## Goal

Add a small Rust library that lets AI tools and other software search FRAMES as
a structured index, similar to a thesaurus for analogy frames.

## Changes

- Add `frames-core` crate metadata.
- Add `FrameEntry`, `FrameIndex`, `FrameQuery`, and `FrameCandidate` types.
- Add starter catalog entries from the current docs.
- Add ranked search over situation text, frame kind, and tags.
- Add related-frame lookup.
- Preserve action cues and failure mode warnings in indexed entries.
- Add tests for ranking, related lookup, and empty queries.

## Validation

- `cargo test`
- `git diff --check`

## Status

Complete.

