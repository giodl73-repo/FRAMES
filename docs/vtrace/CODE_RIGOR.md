# Code Rigor

## Scope

Repo or feature: `frames-core`

## Constraints

| ID | Constraint | Verification |
|---|---|---|
| CR-001 | Keep the first crate dependency-free. | Inspect `Cargo.toml`; run `cargo test`. |
| CR-002 | Keep matching deterministic. | Unit tests cover ranking and empty results. |
| CR-003 | Preserve warnings with candidates. | Inspect `FrameEntry` fields and starter catalog. |
| CR-004 | Keep public API names frame-index oriented, not borrowing-oriented. | Inspect `src/lib.rs`. |
| CR-005 | Format and test Rust code before commit. | `cargo fmt --check`; `cargo test`. |

