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
| REQ-028 | NEED-001 | CON-001 / CON-003 | Add metadata-backed accepted starter catalog. | SPEC-027 | DES-005 | n/a | WP-024 | `docs/frame-catalog.md`, `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/theory-roadmap.md` | VER-026 inspection | VAL-025 | EVID-033 | validated |
| REQ-029 | NEED-001 | CON-003 | Define role-reviewed local import promotion. | SPEC-028 | DES-005 | n/a | WP-025 | `docs/theory/local-import-promotion-review.md`, `docs/theory/resonance-manage-frame-imports.md`, `docs/theory/career-gravity-frame-imports.md`, `docs/theory/theme-swimlane-extraction.md` | VER-027 inspection / role review | VAL-026 | EVID-034 | validated |
| REQ-030 | NEED-001 | CON-003 | Define dedicated theme-swimlane role review. | SPEC-029 | DES-005 | n/a | WP-026 | `docs/theory/theme-swimlane-role-review.md`, `docs/theory/theme-swimlane-extraction.md`, `docs/theory/local-import-promotion-review.md`, `docs/theory/theory-roadmap.md` | VER-028 inspection / role review | VAL-027 | EVID-035 | validated |
| REQ-031 | NEED-001 | CON-003 | Define first empirical validation trial protocol. | SPEC-030 | DES-005 | n/a | WP-027 | `docs/theory/empirical-validation-trial-001-theme-swimlanes.md`, `docs/theory/empirical-validation-plan.md`, `docs/theory/theory-roadmap.md` | VER-029 inspection | VAL-028 | EVID-036 | validated |
| REQ-032 | NEED-001 | CON-001 / CON-003 | Define transfer-aware search design. | SPEC-031 | DES-005 | n/a | WP-028 | `docs/theory/transfer-aware-search-design.md`, `docs/theory/relational-transfer-fields.md`, `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/theory-roadmap.md` | VER-030 inspection | VAL-029 | EVID-037 | validated |
| REQ-033 | NEED-001 | CON-003 | Define current theory gap audit. | SPEC-032 | DES-005 | n/a | WP-029 | `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-031 inspection | VAL-030 | EVID-038 | validated |
| REQ-034 | NEED-001 | CON-001 / CON-003 | Add runnable AI response contract example. | SPEC-033 | DES-005 | CR-006 | WP-030 | `examples/ai_response_contract.rs`, `docs/theory/ai-response-contract.md`, `README.md`, `docs/theory/theory-roadmap.md` | VER-032 example run / inspection | VAL-031 | EVID-039 | validated |
| REQ-035 | NEED-001 | CON-001 / CON-003 | Add Rust metadata for accepted starter entries. | SPEC-034 | DES-005 | CR-006 | WP-031 | `src/lib.rs`, `examples/ai_response_contract.rs`, `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/ai-response-contract.md`, `README.md` | VER-033 unit test / example run / inspection | VAL-032 | EVID-040 | validated |
| REQ-036 | NEED-001 | CON-003 | Define accepted-catalog review process. | SPEC-035 | DES-005 | n/a | WP-032 | `docs/theory/accepted-catalog-review-process.md`, `docs/theory/frame-lifecycle.md`, `docs/theory/local-import-promotion-review.md`, `docs/theory/theory-roadmap.md` | VER-034 inspection | VAL-033 | EVID-041 | validated |
| REQ-037 | NEED-001 | CON-003 | Define frame anti-pattern taxonomy. | SPEC-036 | DES-005 | n/a | WP-033 | `docs/theory/frame-antipattern-taxonomy.md`, `docs/theory/accepted-catalog-review-process.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-035 inspection | VAL-034 | EVID-042 | validated |
| REQ-038 | NEED-001 | CON-001 / CON-003 | Define related-frame relation taxonomy. | SPEC-037 | DES-005 | n/a | WP-034 | `docs/theory/related-frame-taxonomy.md`, `docs/theory/composition-and-conflict.md`, `docs/theory/transfer-aware-search-design.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-036 inspection | VAL-035 | EVID-043 | validated |
| REQ-039 | NEED-001 | CON-001 / CON-003 | Define evaluation-set design. | SPEC-038 | DES-005 | n/a | WP-035 | `docs/theory/evaluation-set-design.md`, `docs/theory/accepted-catalog-review-process.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-037 inspection | VAL-036 | EVID-044 | validated |
| REQ-040 | NEED-001 | CON-003 | Define cultural portability guidance. | SPEC-039 | DES-005 | n/a | WP-036 | `docs/theory/cultural-portability.md`, `docs/theory/audience-transfer.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-038 inspection | VAL-037 | EVID-045 | validated |
| REQ-041 | NEED-001 | CON-003 | Define theme-swimlane leadership worksheet. | SPEC-040 | DES-005 | n/a | WP-037 | `docs/theory/theme-swimlane-leadership-worksheet.md`, `docs/theory/theme-swimlane-extraction.md`, `docs/theory/theme-swimlane-role-review.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-039 inspection | VAL-038 | EVID-046 | validated |
| REQ-042 | NEED-001 | CON-001 / CON-003 | Add strict transfer-aware search filters. | SPEC-041 | DES-005 | CR-006 | WP-038 | `src/lib.rs`, `examples/lookup.rs`, `examples/ai_response_contract.rs`, `docs/theory/transfer-aware-search-design.md`, `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/ai-response-contract.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-040 unit test / example run / inspection | VAL-039 | EVID-047 | validated |
| REQ-043 | NEED-001 | CON-003 | Add locked EVT-001 execution packet. | SPEC-042 | DES-005 | n/a | WP-039 | `docs/validation/evt-001-theme-swimlanes-runbook.md`, `docs/validation/evt-001-response-and-scoring-sheet.md`, `docs/validation/evt-001-theme-swimlanes-results.md`, `docs/theory/empirical-validation-trial-001-theme-swimlanes.md`, `docs/theory/empirical-validation-plan.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-041 inspection | VAL-040 | EVID-048 | validated |
| REQ-044 | NEED-001 | CON-001 / CON-003 | Define controlled frame ontology. | SPEC-043 | DES-005 | n/a | WP-040 | `docs/theory/frame-ontology.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-042 inspection | VAL-041 | EVID-049 | validated |
| REQ-045 | NEED-001 | CON-001 / CON-003 | Apply ontology terms to catalog and fixtures. | SPEC-044 | DES-005 | n/a | WP-041 | `docs/frame-catalog.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/frame-ontology.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | VER-043 inspection | VAL-042 | EVID-050 | validated |
| REQ-046 | NEED-001 | CON-003 | Apply accepted-catalog review to Veto Rule. | SPEC-045 | DES-005 | n/a | WP-042 | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/accepted-catalog-review-process.md`, `docs/theory/local-import-promotion-review.md`, `docs/theory/frame-ontology.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-044 inspection | VAL-043 | EVID-051 | validated |
| REQ-047 | NEED-001 | CON-003 | Close Veto Rule review revision items. | SPEC-046 | DES-005 | n/a | WP-043 | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/accepted-catalog-review-process.md`, `docs/theory/local-import-promotion-review.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | VER-045 inspection | VAL-044 | EVID-052 | validated |
| REQ-048 | NEED-001 | CON-001 / CON-003 | Add Veto Rule reviewed docs-catalog row. | SPEC-047 | DES-005 | n/a | WP-044 | `docs/frame-catalog.md`, `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/accepted-catalog-review-process.md`, `docs/theory/local-import-promotion-review.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | VER-046 inspection | VAL-045 | EVID-053 | validated |
| REQ-049 | NEED-001 | CON-003 | Apply anti-pattern taxonomy to Veto Rule. | SPEC-048 | DES-005 | n/a | WP-045 | `docs/theory/frame-antipattern-application-veto-rule.md`, `docs/theory/frame-antipattern-taxonomy.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-047 inspection | VAL-046 | EVID-054 | validated |
| REQ-050 | NEED-001 | CON-001 / CON-002 / CON-003 | Apply related-frame taxonomy to starter catalog links. | SPEC-049 | DES-003 / DES-005 | n/a | WP-046 | `docs/theory/related-frame-application-starter.md`, `docs/theory/related-frame-taxonomy.md`, `docs/theory/evaluation-set-design.md`, `docs/frame-catalog.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-048 inspection | VAL-047 | EVID-055 | validated |
| REQ-051 | NEED-001 | CON-001 / CON-002 / CON-003 | Publish first machine-readable evaluation fixture package. | SPEC-050 | DES-005 | n/a | WP-047 | `docs/eval/starter-fixtures.json`, `docs/eval/README.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-049 inspection / JSON parse | VAL-048 | EVID-056 | validated |
| REQ-052 | NEED-001 | CON-001 / CON-003 | Apply cultural portability bands to starter fixtures. | SPEC-051 | DES-005 | n/a | WP-048 | `docs/eval/starter-fixtures.json`, `docs/eval/README.md`, `docs/theory/cultural-portability-application-fixtures.md`, `docs/theory/cultural-portability.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-050 inspection / JSON parse | VAL-049 | EVID-057 | validated |
| REQ-053 | NEED-001 | CON-003 | Add theme-swimlane leadership pilot ledger. | SPEC-052 | DES-005 | n/a | WP-049 | `docs/validation/theme-swimlane-leadership-pilot-ledger.md`, `docs/theory/theme-swimlane-leadership-worksheet.md`, `docs/theory/theme-swimlane-role-review.md`, `docs/theory/theme-swimlane-extraction.md`, `README.md` | VER-051 inspection | VAL-050 | EVID-058 | validated |
| REQ-054 | NEED-001 | CON-001 / CON-003 | Define frame acquisition method. | SPEC-053 | DES-005 | n/a | WP-050 | `docs/theory/frame-acquisition-method.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-052 inspection | VAL-051 | EVID-059 | validated |
| REQ-055 | NEED-001 | CON-001 / CON-003 | Define domain pack roadmap. | SPEC-054 | DES-005 | n/a | WP-051 | `docs/theory/domain-pack-roadmap.md`, `docs/theory/application-pack-templates.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md` | VER-053 inspection | VAL-052 | EVID-060 | validated |

## ID Legend

- NEED-001: mission need in `MISSION.md`.
- CON-001..CON-003: operating scenarios in `CONOPS.md`.
- SPEC-001..SPEC-054: specification baseline.
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
- WP-024: metadata-backed accepted starter catalog work package.
- WP-025: role-reviewed local import promotion work package.
- WP-026: theme-swimlane role review work package.
- WP-027: empirical validation trial protocol work package.
- WP-028: transfer-aware search design work package.
- WP-029: theory gap audit work package.
- WP-030: AI response contract example work package.
- WP-031: accepted starter Rust metadata migration work package.
- WP-032: accepted-catalog review process work package.
- WP-033: frame anti-pattern taxonomy work package.
- WP-034: related-frame relation taxonomy work package.
- WP-035: evaluation-set design work package.
- WP-036: cultural portability guidance work package.
- WP-037: theme-swimlane leadership worksheet work package.
- WP-038: transfer-aware search filters work package.
- WP-039: EVT-001 execution packet work package.
- WP-040: frame ontology work package.
- WP-041: ontology application work package.
- WP-042: Veto Rule accepted-catalog review work package.
- WP-043: Veto Rule revision closure work package.
- WP-044: Veto Rule docs-catalog row work package.
- WP-045: Veto Rule anti-pattern application work package.
- WP-046: starter related-frame application work package.
- WP-047: starter evaluation fixture package work package.
- WP-048: starter fixture portability application work package.
- WP-049: theme-swimlane leadership pilot ledger work package.
- WP-050: frame acquisition method work package.
- WP-051: domain pack roadmap work package.
- EVID-* evidence: `EVIDENCE.md` and `VERIFICATION.md`.
