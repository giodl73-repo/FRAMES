# Pulse 07: Theory baseline

## Goal

Develop a role-reviewed theory baseline before expanding more frame packs.

## Changes

- Add `docs/theory/frame-theory.md`.
- Define source scene, target situation, transfer map, action cue, evidence
  boundary, and misuse warning.
- Define frame jobs: status, coordination, momentum, risk, priority, and
  learning.
- Add fit tests, audience levels, evidence boundary guidance, misuse patterns,
  and selection procedure.
- Add role review notes using the FRAMES `.roles` panel.

## Validation

- `cargo fmt --check`
- `cargo test`
- `cargo run --example lookup`
- `git diff --check`
- `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .`

## Status

Complete.
