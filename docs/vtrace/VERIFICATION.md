# Verification

## Scope

Repo or feature: `frames-core`

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Result | Status | Evidence |
|---|---|---|---|---|---|
| REQ-001 | unit test / inspection | `cargo test`; inspect `FrameEntry` | Structured entries carry required fields. | pass | EVID-001 / EVID-004 |
| REQ-002 | unit test | `cargo test` | Search ranks matching candidates and returns empty unmatched results. | pass | EVID-001 |
| REQ-003 | unit test | `cargo test` | Related-frame IDs resolve to entries. | pass | EVID-001 |
| REQ-004 | inspection | inspect `STARTER_CATALOG` and docs catalog | Failure modes are present. | pass | EVID-004 |
| REQ-005 | inspection / command | inspect `Cargo.toml`; `cargo test` | No third-party dependencies are required. | pass | EVID-001 / EVID-005 |

## Commands

```powershell
cargo fmt --check
cargo test
git diff --check
```

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| EVID-001 | command output | `cargo test` | REQ-001, REQ-002, REQ-003, REQ-005, VAL-001, VAL-002 | pass |
| EVID-002 | command output | `git diff --check` | Docs and package formatting | pass |
| EVID-003 | command output | `cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .` | VTRACE package structure | pass |
| EVID-004 | inspection | `src/lib.rs`, `docs/frame-catalog.md` | REQ-001, REQ-004, VAL-003 | pass |
| EVID-005 | inspection | `Cargo.toml`, `Cargo.lock` | REQ-005 | pass |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Starter search is lexical, not semantic. | Some useful analogies may not rank without tag hints. | Accept for first deterministic index; revisit with explicit work package. |
| Starter catalog is small. | Search coverage is limited. | Accept for foundation; expand through frame-pack pulses. |
