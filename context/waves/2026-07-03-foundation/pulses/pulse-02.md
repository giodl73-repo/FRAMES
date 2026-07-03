# Pulse 02: Traffic frame pack

## Goal

Turn traffic and driving examples into a fuller reusable frame pack.

## Candidate Frames

- Red / yellow / green.
- Four-way stop.
- Crosswalk yield.
- Merge lane.
- Detour.
- Blind spot.
- Dashboard warning light.
- Downshift.
- Speed limit.
- Shoulder / pull-off.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run --example lookup`
- `git diff --check`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
