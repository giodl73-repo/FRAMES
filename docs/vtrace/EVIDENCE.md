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

## Evidence Rules

- Evidence IDs are stable and referenced from `TRACE.md`.
- Command evidence records the exact command and result.
- Review evidence points to `REVIEW.md`.
- Deferred evidence includes a revisit trigger.
