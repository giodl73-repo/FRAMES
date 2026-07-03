# Evidence Ledger

## Scope

Repo or feature: `frames-core`

## Evidence Records

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |
|---|---|---|---|---|---|
| EVID-001 | command | `cargo test` | Rust unit tests pass. | 3 passed, 0 failed. | pass |
| EVID-002 | command | `git diff --check` | No whitespace errors. | pass. | pass |
| EVID-003 | command | `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .` | VTRACE package validates. | pass. | pass |
| EVID-004 | inspection | `src/lib.rs`, `docs/frame-catalog.md` | Indexed entries retain action cues and failure modes. | pass. | pass |
| EVID-005 | inspection | `Cargo.toml`, `Cargo.lock` | No third-party dependencies in first crate. | pass. | pass |
| EVID-006 | command | `cargo test` | Helper tests pass. | pass. | pass |
| EVID-007 | command | `cargo run --example lookup` | Lookup example runs and prints candidates with warnings. | pass. | pass |
| EVID-008 | command | `cargo test` | Traffic-pack alignment tests pass. | pass. | pass |
| EVID-009 | inspection | `docs/frame-catalog.md`, `docs/examples/traffic-and-motion.md`, `src/lib.rs` | Traffic frame docs and index entries are aligned. | pass. | pass |
| EVID-010 | inspection | `docs/theory/frame-theory.md` | Frame theory covers core parts, jobs, fit tests, evidence boundaries, and misuse patterns. | pass. | pass |
| EVID-011 | role review | `docs/theory/role-review-2026-07-03.md` | Role panel review completed. | pass. | pass |
| EVID-012 | inspection | `docs/theory/fit-rubric.md` | Scored accept/revise/hold/reject rubric exists. | pass. | pass |
| EVID-013 | inspection | `docs/theory/theory-roadmap.md` | Remaining theory work is prioritized. | pass. | pass |
| EVID-014 | inspection | `docs/theory/audience-transfer.md` | Audience transfer guide exists. | pass. | pass |
| EVID-015 | inspection | `docs/theory/frame-lifecycle.md` | Frame lifecycle model exists. | pass. | pass |

## Evidence Rules

- Evidence IDs are stable and referenced from `TRACE.md`.
- Command evidence records the exact command and result.
- Review evidence points to `REVIEW.md`.
- Deferred evidence includes a revisit trigger.
