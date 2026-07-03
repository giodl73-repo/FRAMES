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
| REQ-012 | NEED-001 | CON-003 | Define composition and conflict guidance. | SPEC-011 | DES-005 | n/a | WP-008 | `docs/theory/composition-and-conflict.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md` | VER-010 inspection | VAL-009 | EVID-016 | validated |
| REQ-013 | NEED-001 | CON-001 / CON-003 | Preserve evidence boundary in indexed entries. | SPEC-012 | DES-004 / DES-005 | CR-003 / CR-005 | WP-009 | `src/lib.rs`, `examples/lookup.rs`, `docs/theory/evidence-boundary-schema.md` | VER-011 unit test / inspection | VAL-010 | EVID-017 / EVID-018 | validated |
| REQ-014 | NEED-001 | CON-003 | Define research grounding for cognitive-science claims. | SPEC-013 | DES-005 | n/a | WP-010 | `docs/theory/research-grounding.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md` | VER-012 inspection | VAL-011 | EVID-019 | validated |
| REQ-015 | NEED-001 | CON-003 | Define source-domain taxonomy. | SPEC-014 | DES-005 | n/a | WP-011 | `docs/theory/source-domain-taxonomy.md`, `.roles/parliament/research-grounding-reviewer.md`, `docs/theory/theory-roadmap.md` | VER-013 inspection / role review | VAL-012 | EVID-020 | validated |
| REQ-016 | NEED-001 | CON-001 / CON-003 | Define application-pack templates. | SPEC-015 | DES-005 | n/a | WP-012 | `docs/theory/application-pack-templates.md`, `docs/theory/frame-theory.md`, `docs/theory/source-domain-taxonomy.md`, `docs/theory/theory-roadmap.md` | VER-014 inspection | VAL-013 | EVID-021 | validated |
| REQ-017 | NEED-001 | CON-003 | Define perspective metadata guidance. | SPEC-016 | DES-005 | n/a | WP-013 | `docs/theory/perspective-metadata.md`, `docs/theory/frame-theory.md`, `docs/theory/source-domain-taxonomy.md`, `docs/theory/application-pack-templates.md` | VER-015 inspection | VAL-014 | EVID-022 | validated |
| REQ-018 | NEED-001 | CON-003 | Define story-job taxonomy guidance. | SPEC-017 | DES-005 | n/a | WP-014 | `docs/theory/story-job-taxonomy.md`, `docs/theory/fit-rubric.md`, `docs/theory/frame-theory.md`, `docs/theory/external-frame-practitioners.md` | VER-016 inspection | VAL-015 | EVID-023 | validated |
| REQ-019 | NEED-001 | CON-003 | Define relational transfer fields. | SPEC-018 | DES-005 | n/a | WP-015 | `docs/theory/relational-transfer-fields.md`, `docs/theory/fit-rubric.md`, `docs/theory/frame-theory.md`, `docs/theory/source-domain-taxonomy.md` | VER-017 inspection | VAL-016 | EVID-024 | validated |
| REQ-020 | NEED-001 | CON-003 | Define claim-strength labels. | SPEC-019 | DES-005 | n/a | WP-016 | `docs/theory/claim-strength-labels.md`, `docs/theory/research-grounding.md`, `docs/theory/evidence-boundary-schema.md`, `docs/theory/fit-rubric.md` | VER-018 inspection | VAL-017 | EVID-025 | validated |
| REQ-021 | NEED-001 | CON-003 | Define role-reviewed domain examples. | SPEC-020 | DES-005 | n/a | WP-017 | `docs/theory/role-reviewed-domain-examples.md`, `.roles/ROLE.md`, `.roles/parliament/*`, `docs/theory/theory-roadmap.md` | VER-019 inspection / role review | VAL-018 | EVID-026 | validated |
| REQ-022 | NEED-001 | CON-003 | Define structured RESONANCE MANAGE imports. | SPEC-021 | DES-005 | n/a | WP-018 | `docs/theory/resonance-manage-frame-imports.md`, `docs/theory/resonance-manage-import-map.md`, `docs/theory/claim-strength-labels.md`, `docs/theory/relational-transfer-fields.md` | VER-020 inspection | VAL-019 | EVID-027 | validated |
| REQ-023 | NEED-001 | CON-003 | Define structured CAREER Gravity imports. | SPEC-022 | DES-005 | n/a | WP-019 | `docs/theory/career-gravity-frame-imports.md`, `docs/theory/career-gravity-import-map.md`, `docs/theory/claim-strength-labels.md`, `docs/theory/relational-transfer-fields.md` | VER-021 inspection | VAL-020 | EVID-028 | validated |
| REQ-024 | NEED-001 | CON-003 | Define theme-swimlane extraction. | SPEC-023 | DES-005 | n/a | WP-020 | `docs/theory/theme-swimlane-extraction.md`, `docs/theory/claim-strength-labels.md`, `docs/theory/role-reviewed-domain-examples.md`, `docs/theory/frame-theory.md` | VER-022 inspection | VAL-021 | EVID-029 | validated |
| REQ-025 | NEED-001 | CON-003 | Define empirical validation plan. | SPEC-024 | DES-005 | n/a | WP-021 | `docs/theory/empirical-validation-plan.md`, `docs/theory/claim-strength-labels.md`, `docs/theory/research-grounding.md`, `docs/theory/frame-theory.md` | VER-023 inspection | VAL-022 | EVID-030 | validated |
| REQ-026 | NEED-001 | CON-001 / CON-003 | Define catalog metadata migration plan. | SPEC-025 | DES-005 | n/a | WP-022 | `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md`, `src/lib.rs` | VER-024 inspection | VAL-023 | EVID-031 | validated |
| REQ-027 | NEED-001 | CON-001 / CON-003 | Define AI response contract. | SPEC-026 | DES-005 | n/a | WP-023 | `docs/theory/ai-response-contract.md`, `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/frame-theory.md`, `docs/theory/application-pack-templates.md` | VER-025 inspection | VAL-024 | EVID-032 | validated |

## ID Legend

- NEED-001: mission need in `MISSION.md`.
- CON-001..CON-003: operating scenarios in `CONOPS.md`.
- SPEC-001..SPEC-026: specification baseline.
- DES-001..DES-005: design elements.
- CR-001..CR-006: code-rigor constraints.
- WP-001: frame index crate work package.
- WP-002: frame index ergonomics work package.
- WP-003: traffic frame-pack expansion work package.
- WP-004: frame theory baseline work package.
- WP-005: fit rubric and theory roadmap work package.
- WP-006: audience transfer guide work package.
- WP-007: frame lifecycle model work package.
- WP-008: composition and conflict theory work package.
- WP-009: evidence-boundary schema work package.
- WP-010: research-grounding theory work package.
- WP-011: source-domain taxonomy work package.
- WP-012: application-pack templates work package.
- WP-013: perspective metadata work package.
- WP-014: story-job taxonomy work package.
- WP-015: relational transfer fields work package.
- WP-016: claim-strength labels work package.
- WP-017: role-reviewed domain examples work package.
- WP-018: structured RESONANCE MANAGE imports work package.
- WP-019: structured CAREER Gravity imports work package.
- WP-020: theme-swimlane extraction work package.
- WP-021: empirical validation plan work package.
- WP-022: catalog metadata migration plan work package.
- WP-023: AI response contract work package.
- EVID-* evidence: `EVIDENCE.md` and `VERIFICATION.md`.
