# Work Packages

## Scope

Repo or feature: `frames-core`

Product boundary rule: VTRACE closeout is not product scope. Do not build
product subcommands such as `work-package`, `prove`, `readiness`, or `evidence`
unless product requirements explicitly define them as user-facing behavior.

## Work Package Table

| ID | Objective | Product Requirement | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | VTRACE-Only Closeout | Status |
|---|---|---|---|---|---|---|---|---|---|
| WP-001 | Add dependency-free Rust frame index. | AI/tool users can search structured analogy frames like a thesaurus. | REQ-001 / REQ-002 / REQ-003 / REQ-004 / REQ-005 / SPEC-001 / SPEC-002 / SPEC-003 / SPEC-004 | `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `README.md`, `PRODUCT_PLAN.md`, `context/waves/*`, `docs/vtrace/*` | Requirements and public API target accepted. | Rust tests pass, docs checks pass, trace/evidence/review are recorded. | L0: `cargo test`; `git diff --check` / L1: VTRACE validate / L2: role review for public API readiness | evidence / trace / review / status rows | complete |
| WP-002 | Add ergonomic lookup helpers and example. | AI/tool users can construct common frame-index queries and inspect results quickly. | REQ-006 / SPEC-005 / IF-007 / IF-008 / DES-005 / CR-006 | `src/lib.rs`, `examples/lookup.rs`, `README.md`, `context/waves/*`, `docs/vtrace/*` | WP-001 complete and helper API target accepted. | Helper tests pass, example runs, docs and VTRACE checks pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review if publishing API | evidence / trace / review / status rows | complete |

## Work Package Details

### WP-001: Add dependency-free Rust frame index

Objective: provide the first Rust library surface for deterministic frame index
lookup.

Parent IDs: REQ-001, REQ-002, REQ-003, REQ-004, REQ-005, SPEC-001, SPEC-002,
SPEC-003, SPEC-004, IF-001, IF-002, DES-001, DES-002, DES-003, DES-004,
CR-001, CR-002, CR-003, CR-004, CR-005.

Affected files/modules:

- `Cargo.toml`
- `Cargo.lock`
- `src/lib.rs`
- `README.md`
- `PRODUCT_PLAN.md`
- `context/waves/2026-07-03-foundation/*`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
git diff --check
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | `cargo test`; `git diff --check` | pass |
| L1 | yes | VTRACE validator over repo package | pass |
| L2 | yes | Role review through `.roles` lenses before public release. | pending public release |

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Need / CONOPS | NEED-001, CON-001, CON-002, CON-003 | closed | Index lookup, related alternatives, and warning inspection. |
| Requirements | REQ-001..REQ-005 | closed | All assigned to WP-001. |
| Architecture / Interface | ARCH-001..ARCH-003, IF-001..IF-006 | closed | Dependency-free Rust crate. |
| Design / Code Rigor | DES-001..DES-004, CR-001..CR-005 | closed | Deterministic starter index. |
| Implementation | `src/lib.rs`, `Cargo.toml` | closed | First frame index crate. |
| Verification | EVID-001, EVID-002, EVID-003 | closed | Tests and docs checks pass; VTRACE validator pending if unavailable. |
| Validation | VAL-001, VAL-002, VAL-003 | partial | API scenarios covered by tests; public release review remains pending. |
| Trace | `TRACE.md` rows include WP-001 | closed | No orphan accepted requirements. |
| Gate | `REVIEW.md` decision `pass_with_risk` | closed | Accepted risk: starter catalog is intentionally small. |

### WP-002: Add ergonomic lookup helpers and example

Objective: make the frame index easier to use from AI tooling and downstream
Rust code.

Parent IDs: REQ-006, SPEC-005, IF-007, IF-008, DES-005, CR-006.

Affected files/modules:

- `src/lib.rs`
- `examples/lookup.rs`
- `README.md`
- `context/waves/2026-07-03-foundation/*`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` | pass |
| L1 | yes | VTRACE validator over repo package | pass |
| L2 | no | Public API role review deferred until publish step. | n/a |

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-006 | closed | Helper requirement assigned to WP-002. |
| Interface / Design | IF-007, IF-008, DES-005 | closed | Builder and lookup helpers added. |
| Code Rigor | CR-006 | closed | Deterministic helper behavior tested. |
| Implementation | `src/lib.rs`, `examples/lookup.rs` | closed | Helpers and example added. |
| Verification | EVID-006, EVID-007 | closed | Tests and example run pass. |
| Validation | VAL-001, VAL-002, VAL-004 | closed | Query and related lookup flows easier to execute. |
