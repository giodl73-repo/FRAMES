# Trace Matrix

| Requirement ID | Parent Need | CONOPS | Requirement | Specification Item | Design Element | Code Rigor Constraint | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 | CON-001 / CON-003 | Expose structured frame entries. | SPEC-001 | DES-001 / DES-004 | CR-003 / CR-004 | WP-001 | `src/lib.rs` | VER-001 unit test / inspection | VAL-001 / VAL-003 | EVID-001 / EVID-004 | verified |
| REQ-002 | NEED-001 | CON-001 | Provide deterministic search. | SPEC-002 | DES-002 | CR-002 / CR-005 | WP-001 | `src/lib.rs` | VER-001 unit test | VAL-001 | EVID-001 | validated |
| REQ-003 | NEED-001 | CON-002 | Provide related-frame lookup. | SPEC-003 | DES-003 | CR-002 / CR-005 | WP-001 | `src/lib.rs` | VER-001 unit test | VAL-002 | EVID-001 | validated |
| REQ-004 | NEED-001 | CON-003 | Preserve failure-mode warnings. | SPEC-001 | DES-004 | CR-003 | WP-001 | `src/lib.rs`, `docs/frame-catalog.md` | VER-002 inspection | VAL-003 | EVID-004 | verified |
| REQ-005 | NEED-001 | CON-001 | Keep first crate dependency-free. | SPEC-004 | DES-001 | CR-001 | WP-001 | `Cargo.toml`, `Cargo.lock` | VER-003 inspection / cargo test | VAL-001 | EVID-001 / EVID-005 | verified |
| REQ-006 | NEED-001 | CON-001 / CON-002 | Provide ergonomic query and filtering helpers. | SPEC-005 | DES-005 | CR-006 | WP-002 | `src/lib.rs`, `examples/lookup.rs`, `README.md` | VER-004 unit test / example run | VAL-001 / VAL-002 / VAL-004 | EVID-006 / EVID-007 | validated |
| REQ-007 | NEED-001 | CON-001 / CON-003 | Keep traffic docs and starter index aligned. | SPEC-006 | DES-001 / DES-004 | CR-003 / CR-005 | WP-003 | `docs/frame-catalog.md`, `docs/examples/traffic-and-motion.md`, `src/lib.rs` | VER-005 unit test / inspection | VAL-003 / VAL-004 | EVID-008 / EVID-009 | validated |
| REQ-008 | NEED-001 | CON-003 | Define role-reviewed frame theory before broad expansion. | SPEC-007 | DES-005 | n/a | WP-004 | `docs/theory/frame-theory.md`, `docs/theory/role-review-2026-07-03.md` | VER-006 inspection / role review | VAL-005 | EVID-010 / EVID-011 | validated |
| REQ-009 | NEED-001 | CON-003 | Define scored fit rubric and theory roadmap. | SPEC-008 | DES-005 | n/a | WP-005 | `docs/theory/fit-rubric.md`, `docs/theory/theory-roadmap.md` | VER-007 inspection | VAL-006 | EVID-012 / EVID-013 | validated |
| REQ-010 | NEED-001 | CON-003 | Define audience transfer guidance. | SPEC-009 | DES-005 | n/a | WP-006 | `docs/theory/audience-transfer.md`, `docs/theory/theory-roadmap.md` | VER-008 inspection | VAL-007 | EVID-014 | validated |
| REQ-011 | NEED-001 | CON-003 | Define frame lifecycle states and indexing rules. | SPEC-010 | DES-005 | n/a | WP-007 | `docs/theory/frame-lifecycle.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md` | VER-009 inspection | VAL-008 | EVID-015 | validated |

## ID Legend

- NEED-001: mission need in `MISSION.md`.
- CON-001..CON-003: operating scenarios in `CONOPS.md`.
- SPEC-001..SPEC-010: specification baseline.
- DES-001..DES-005: design elements.
- CR-001..CR-006: code-rigor constraints.
- WP-001: frame index crate work package.
- WP-002: frame index ergonomics work package.
- WP-003: traffic frame-pack expansion work package.
- WP-004: frame theory baseline work package.
- WP-005: fit rubric and theory roadmap work package.
- WP-006: audience transfer guide work package.
- WP-007: frame lifecycle model work package.
- EVID-* evidence: `EVIDENCE.md` and `VERIFICATION.md`.
