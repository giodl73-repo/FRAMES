# Trace Matrix

| Requirement ID | Parent Need | CONOPS | Requirement | Specification Item | Design Element | Code Rigor Constraint | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 | CON-001 / CON-003 | Expose structured frame entries. | SPEC-001 | DES-001 / DES-004 | CR-003 / CR-004 | WP-001 | `src/lib.rs` | VER-001 unit test / inspection | VAL-001 / VAL-003 | EVID-001 / EVID-004 | verified |
| REQ-002 | NEED-001 | CON-001 | Provide deterministic search. | SPEC-002 | DES-002 | CR-002 / CR-005 | WP-001 | `src/lib.rs` | VER-001 unit test | VAL-001 | EVID-001 | validated |
| REQ-003 | NEED-001 | CON-002 | Provide related-frame lookup. | SPEC-003 | DES-003 | CR-002 / CR-005 | WP-001 | `src/lib.rs` | VER-001 unit test | VAL-002 | EVID-001 | validated |
| REQ-004 | NEED-001 | CON-003 | Preserve failure-mode warnings. | SPEC-001 | DES-004 | CR-003 | WP-001 | `src/lib.rs`, `docs/frame-catalog.md` | VER-002 inspection | VAL-003 | EVID-004 | verified |
| REQ-005 | NEED-001 | CON-001 | Keep first crate dependency-free. | SPEC-004 | DES-001 | CR-001 | WP-001 | `Cargo.toml`, `Cargo.lock` | VER-003 inspection / cargo test | VAL-001 | EVID-001 / EVID-005 | verified |

## ID Legend

- NEED-001: mission need in `MISSION.md`.
- CON-001..CON-003: operating scenarios in `CONOPS.md`.
- SPEC-001..SPEC-004: specification baseline.
- DES-001..DES-004: design elements.
- CR-001..CR-005: code-rigor constraints.
- WP-001: frame index crate work package.
- EVID-* evidence: `EVIDENCE.md` and `VERIFICATION.md`.

