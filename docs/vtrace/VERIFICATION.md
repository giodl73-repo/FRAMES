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
| REQ-006 | unit test / example run | `cargo test`; `cargo run --example lookup` | Helpers construct queries, limit results, filter entries, and run in example form. | pass | EVID-006 / EVID-007 |
| REQ-007 | unit test / inspection | `cargo test`; inspect traffic docs and `STARTER_CATALOG` | New traffic frames are present in docs and index. | pass | EVID-008 / EVID-009 |
| REQ-008 | inspection / role review | inspect `docs/theory/*` | Theory defines frame jobs, fit tests, evidence boundaries, misuse patterns, and role review. | pass | EVID-010 / EVID-011 |
| REQ-009 | inspection | inspect `docs/theory/fit-rubric.md` and `docs/theory/theory-roadmap.md` | Rubric and roadmap are present and usable. | pass | EVID-012 / EVID-013 |
| REQ-010 | inspection | inspect `docs/theory/audience-transfer.md` | Audience transfer dimensions, bands, tests, alternates, and future fields are present. | pass | EVID-014 |
| REQ-011 | inspection | inspect `docs/theory/frame-lifecycle.md` | Lifecycle states, criteria, triggers, status fields, indexing rules, and deprecation rules are present. | pass | EVID-015 |
| REQ-012 | inspection | inspect `docs/theory/composition-and-conflict.md` | Composition rules, conflict types, sequencing, conflict resolution, and index implications are present. | pass | EVID-016 |
| REQ-013 | unit test / inspection | `cargo test`; inspect `FrameEntry` and starter catalog | Evidence boundary is required and populated for indexed entries. | pass | EVID-017 / EVID-018 |
| REQ-014 | inspection | inspect `docs/theory/research-grounding.md` | Research pillars, claim limits, design rules, backlog, and bibliography are present. | pass | EVID-019 |
| REQ-015 | inspection / role review | inspect `docs/theory/source-domain-taxonomy.md` and `.roles/parliament/research-grounding-reviewer.md` | Source-domain dimensions, families, authority models, temporal shapes, risk bands, and review lens are present. | pass | EVID-020 |
| REQ-016 | inspection | inspect `docs/theory/application-pack-templates.md` | Pack shape, pack defaults, authority checks, evidence defaults, alternates, rejection rules, and selection procedure are present. | pass | EVID-021 |
| REQ-017 | inspection | inspect `docs/theory/perspective-metadata.md` | Perspective fields, roles, fit test, conflicts, examples, and anti-patterns are present. | pass | EVID-022 |

## Commands

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
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
| EVID-006 | command output | `cargo test` | REQ-006, SPEC-005, CR-006 | pass |
| EVID-007 | command output | `cargo run --example lookup` | REQ-006, VAL-001, VAL-004 | pass |
| EVID-008 | command output | `cargo test` | REQ-007, SPEC-006 | pass |
| EVID-009 | inspection | `docs/frame-catalog.md`, `docs/examples/traffic-and-motion.md`, `src/lib.rs` | REQ-007, VAL-003 | pass |
| EVID-010 | inspection | `docs/theory/frame-theory.md` | REQ-008, SPEC-007, VAL-005 | pass |
| EVID-011 | role review | `docs/theory/role-review-2026-07-03.md` | REQ-008, VAL-005 | pass |
| EVID-012 | inspection | `docs/theory/fit-rubric.md` | REQ-009, SPEC-008, VAL-006 | pass |
| EVID-013 | inspection | `docs/theory/theory-roadmap.md` | REQ-009, VAL-006 | pass |
| EVID-014 | inspection | `docs/theory/audience-transfer.md` | REQ-010, SPEC-009, VAL-007 | pass |
| EVID-015 | inspection | `docs/theory/frame-lifecycle.md` | REQ-011, SPEC-010, VAL-008 | pass |
| EVID-016 | inspection | `docs/theory/composition-and-conflict.md` | REQ-012, SPEC-011, VAL-009 | pass |
| EVID-017 | command output | `cargo test` | REQ-013, SPEC-012, IF-015 | pass |
| EVID-018 | inspection | `src/lib.rs`, `examples/lookup.rs`, `docs/theory/evidence-boundary-schema.md` | REQ-013, VAL-010 | pass |
| EVID-019 | inspection | `docs/theory/research-grounding.md` | REQ-014, SPEC-013, IF-016, VAL-011 | pass |
| EVID-020 | inspection / role review | `docs/theory/source-domain-taxonomy.md`, `.roles/parliament/research-grounding-reviewer.md` | REQ-015, SPEC-014, IF-017, VAL-012 | pass |
| EVID-021 | inspection | `docs/theory/application-pack-templates.md` | REQ-016, SPEC-015, IF-018, VAL-013 | pass |
| EVID-022 | inspection | `docs/theory/perspective-metadata.md` | REQ-017, SPEC-016, IF-019, VAL-014 | pass |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Starter search is lexical, not semantic. | Some useful analogies may not rank without tag hints. | Accept for first deterministic index; revisit with explicit work package. |
| Starter catalog is small. | Search coverage is limited. | Accept for foundation; expand through frame-pack pulses. |
