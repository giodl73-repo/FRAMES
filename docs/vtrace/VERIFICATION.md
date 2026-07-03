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
| REQ-018 | inspection | inspect `docs/theory/story-job-taxonomy.md` and `docs/theory/fit-rubric.md` | Story-job categories, selection procedure, examples, hard stops, and rubric overlay are present. | pass | EVID-023 |
| REQ-019 | inspection | inspect `docs/theory/relational-transfer-fields.md` and `docs/theory/fit-rubric.md` | Transfer fields, strength labels, examples, exclusions, and rubric integration are present. | pass | EVID-024 |
| REQ-020 | inspection | inspect `docs/theory/claim-strength-labels.md` and `docs/theory/research-grounding.md` | Label set, defaults, promotion rules, required claim shape, hard stops, and integration points are present. | pass | EVID-025 |
| REQ-021 | inspection / role review | inspect `docs/theory/role-reviewed-domain-examples.md` and `.roles/` | Review shape, domain examples, role pressure, review outcomes, and method lessons are present. | pass | EVID-026 |
| REQ-022 | inspection | inspect `docs/theory/resonance-manage-frame-imports.md` and `docs/theory/resonance-manage-import-map.md` | Import status, draft entries, evidence boundaries, transfer exclusions, failure modes, and promotion criteria are present. | pass | EVID-027 |
| REQ-023 | inspection | inspect `docs/theory/career-gravity-frame-imports.md` and `docs/theory/career-gravity-import-map.md` | Import status, draft entries, evidence boundaries, transfer exclusions, failure modes, and promotion criteria are present. | pass | EVID-028 |
| REQ-024 | inspection | inspect `docs/theory/theme-swimlane-extraction.md` | Pattern shape, local variants, extraction procedure, fit checks, promotion criteria, and design consequences are present. | pass | EVID-029 |
| REQ-025 | inspection | inspect `docs/theory/empirical-validation-plan.md` | Validation boundary, study levels, measures, protocol, templates, backlog, reporting shape, hard stops, and design consequences are present. | pass | EVID-030 |
| REQ-026 | inspection | inspect `docs/theory/catalog-metadata-migration-plan.md` and `src/lib.rs` | Current indexed shape, metadata families, migration stages, promotion rules, initial backlog, starter target, and compatibility rules are present. | pass | EVID-031 |
| REQ-027 | inspection | inspect `docs/theory/ai-response-contract.md` | Required inputs, output fields, JSON shape, selection rules, scoring interpretation, gating rules, language rules, and examples are present. | pass | EVID-032 |
| REQ-028 | inspection | inspect `docs/frame-catalog.md` and `docs/theory/catalog-metadata-migration-plan.md` | Accepted starter metadata table includes stable metadata columns and excludes local draft imports. | pass | EVID-033 |
| REQ-029 | inspection / role review | inspect `docs/theory/local-import-promotion-review.md` and local import docs | Promotion outcomes, promoted heuristics, held imports, promotion rules, and next catalog candidates are present. | pass | EVID-034 |
| REQ-030 | inspection / role review | inspect `docs/theory/theme-swimlane-role-review.md` and linked theme-swimlane docs | Acceptance decision, role findings, fit score, evidence gates, hard stops, output template, and next actions are present. | pass | EVID-035 |
| REQ-031 | inspection | inspect `docs/theory/empirical-validation-trial-001-theme-swimlanes.md` and validation plan link | Trial summary, hypothesis, scenario material, comparison conditions, response form, scoring rubric, analysis plan, report template, hard stops, and claim boundary are present. | pass | EVID-036 |
| REQ-032 | inspection | inspect `docs/theory/transfer-aware-search-design.md` and linked transfer/migration docs | Current search limits, query inputs, entry metadata, scoring order, hard stops, output notes, migration path, open questions, and design consequences are present. | pass | EVID-037 |
| REQ-033 | inspection | inspect `docs/theory/theory-gap-audit.md` and roadmap link | Current strengths, blocking gaps, growth gaps, implementation gaps, deep theory gaps, recommended sequence, non-goals, and decision statement are present. | pass | EVID-038 |
| REQ-034 | example run / inspection | `cargo run --example ai_response_contract`; inspect `examples/ai_response_contract.rs` | Example prints bounded AI response fields and keeps retrieval score distinct from omitted fit score. | pass | EVID-039 |
| REQ-035 | unit test / example run / inspection | `cargo test`; `cargo run --example ai_response_contract`; inspect `FrameEntry` | Accepted starter entries expose status, claim strength, risk band, application packs, and metadata filters. | pass | EVID-040 |
| REQ-036 | inspection | inspect `docs/theory/accepted-catalog-review-process.md` and linked lifecycle/promotion docs | Review inputs, board lenses, decision bands, hard stops, procedure, metadata requirements, template, promotion rule, and design consequences are present. | pass | EVID-041 |
| REQ-037 | inspection | inspect `docs/theory/frame-antipattern-taxonomy.md` and linked review docs | Detection rule, failure classes, review procedure, dispositions, evaluation-set implications, AI/tool implications, examples, and design consequences are present. | pass | EVID-042 |
| REQ-038 | inspection | inspect `docs/theory/related-frame-taxonomy.md` and linked composition/search docs | Relation rule, relation types, direction rules, selection implications, examples, review procedure, catalog field shape, AI/tool implications, and design consequences are present. | pass | EVID-043 |
| REQ-039 | inspection | inspect `docs/theory/evaluation-set-design.md` and linked review docs | Evaluation jobs, fixture types, fixture shape, required fields, starter backlog, scoring dimensions, pass bands, review procedure, non-goals, AI/tool implications, and design consequences are present. | pass | EVID-044 |
| REQ-040 | inspection | inspect `docs/theory/cultural-portability.md` and linked audience-transfer docs | Portability rule, dimensions, bands, source-specific guidance, decision procedure, examples, catalog fields, evaluation implications, AI/tool implications, and design consequences are present. | pass | EVID-045 |
| REQ-041 | inspection | inspect `docs/theory/theme-swimlane-leadership-worksheet.md` and linked theme-swimlane docs | Use criteria, promise capture, lane table, work mapping, decision log, risk review, role gate, worked example, pilot closeout, and design consequences are present. | pass | EVID-046 |
| REQ-042 | unit test / example run / inspection | `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; inspect `src/lib.rs` | Search supports strict authority, risk, and application-pack filters before lexical scoring. | pass | EVID-047 |
| REQ-043 | inspection | inspect `docs/validation/evt-001-theme-swimlanes-runbook.md`, `docs/validation/evt-001-response-and-scoring-sheet.md`, and `docs/validation/evt-001-theme-swimlanes-results.md` | EVT-001 has a locked execution packet, response/scoring sheet, and empty results ledger before collection. | pass | EVID-048 |
| REQ-044 | inspection | inspect `docs/theory/frame-ontology.md` | Ontology rule, entity types, controlled terms, tag rules, term admission, review checklist, and design consequences are present. | pass | EVID-049 |
| REQ-045 | inspection | inspect `docs/frame-catalog.md`, `docs/theory/evaluation-set-design.md`, and `docs/theory/frame-ontology.md` | Accepted starter rows and fixture backlog use controlled ontology terms. | pass | EVID-050 |
| REQ-046 | inspection | inspect `docs/theory/accepted-catalog-review-veto-rule.md` and linked review/promotion docs | Veto Rule accepted-catalog review records revise decision, role findings, fit score, revision items, metadata candidate, and non-index behavior. | pass | EVID-051 |
| REQ-047 | inspection | inspect `docs/theory/accepted-catalog-review-veto-rule.md` and `docs/theory/evaluation-set-design.md` | Veto Rule revision items are closed with fixtures, stop conditions, fallback, and docs-only caveat boundary. | pass | EVID-052 |
| REQ-048 | inspection | inspect `docs/frame-catalog.md` and `docs/theory/catalog-metadata-migration-plan.md` | Reviewed docs-catalog candidates are separated from accepted starter metadata and default Rust search. | pass | EVID-053 |
| REQ-049 | inspection | inspect `docs/theory/frame-antipattern-application-veto-rule.md`, `docs/theory/frame-antipattern-taxonomy.md`, and `docs/theory/evaluation-set-design.md` | Veto Rule anti-pattern classes and rejected-use fixture are present. | pass | EVID-054 |
| REQ-050 | inspection | inspect `docs/theory/related-frame-application-starter.md`, `docs/theory/related-frame-taxonomy.md`, `docs/theory/evaluation-set-design.md`, and `docs/frame-catalog.md` | Starter related-frame links are typed at docs level and relation behavior fixtures are present. | pass | EVID-055 |
| REQ-051 | inspection / JSON parse | parse `docs/eval/starter-fixtures.json` and inspect fixture fields | First fixture package covers starter categories and required expected-behavior fields. | pass | EVID-056 |
| REQ-052 | inspection / JSON parse | parse `docs/eval/starter-fixtures.json` and inspect portability profiles plus `docs/theory/cultural-portability-application-fixtures.md` | Starter fixtures have portability band profiles and safer fallbacks. | pass | EVID-057 |
| REQ-053 | inspection | inspect `docs/validation/theme-swimlane-leadership-pilot-ledger.md` and linked theme-swimlane docs | Pilot ledger defines record shape, dry-run boundary, and no-evidence status. | pass | EVID-058 |
| REQ-054 | inspection | inspect `docs/theory/frame-acquisition-method.md` | Acquisition rule, source channels, intake template, screening gates, promotion path, candidate states, and design consequences are present. | pass | EVID-059 |
| REQ-055 | inspection | inspect `docs/theory/domain-pack-roadmap.md` and `docs/theory/application-pack-templates.md` | Domain pack growth rule, expansion order, coverage matrix, hold conditions, and drift warnings are present. | pass | EVID-060 |
| REQ-056 | inspection | inspect `docs/theory/plain-language-fallbacks.md` and linked response/evaluation docs | Fallback rule, shape, types, examples, selection procedure, tool behavior, evaluation implications, and design consequences are present. | pass | EVID-061 |
| REQ-057 | inspection | inspect `docs/theory/learning-progression.md` | Learner levels, paths, teaching anti-patterns, release criteria, and design consequences are present. | pass | EVID-062 |
| REQ-058 | inspection | inspect `docs/theory/lifecycle-filtering-and-rejection-reporting.md` | Visibility modes, status filters, result classes, rejected-candidate report shape, default behavior, fixture gates, Rust migration gates, and design consequences are present. | pass | EVID-063 |

## Commands

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
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
| EVID-023 | inspection | `docs/theory/story-job-taxonomy.md`, `docs/theory/fit-rubric.md` | REQ-018, SPEC-017, IF-020, VAL-015 | pass |
| EVID-024 | inspection | `docs/theory/relational-transfer-fields.md`, `docs/theory/fit-rubric.md` | REQ-019, SPEC-018, IF-021, VAL-016 | pass |
| EVID-025 | inspection | `docs/theory/claim-strength-labels.md`, `docs/theory/research-grounding.md` | REQ-020, SPEC-019, IF-022, VAL-017 | pass |
| EVID-026 | inspection / role review | `docs/theory/role-reviewed-domain-examples.md`, `.roles/ROLE.md` | REQ-021, SPEC-020, IF-023, VAL-018 | pass |
| EVID-027 | inspection | `docs/theory/resonance-manage-frame-imports.md`, `docs/theory/resonance-manage-import-map.md` | REQ-022, SPEC-021, IF-024, VAL-019 | pass |
| EVID-028 | inspection | `docs/theory/career-gravity-frame-imports.md`, `docs/theory/career-gravity-import-map.md` | REQ-023, SPEC-022, IF-025, VAL-020 | pass |
| EVID-029 | inspection | `docs/theory/theme-swimlane-extraction.md` | REQ-024, SPEC-023, IF-026, VAL-021 | pass |
| EVID-030 | inspection | `docs/theory/empirical-validation-plan.md` | REQ-025, SPEC-024, IF-027, VAL-022 | pass |
| EVID-031 | inspection | `docs/theory/catalog-metadata-migration-plan.md`, `src/lib.rs` | REQ-026, SPEC-025, IF-028, VAL-023 | pass |
| EVID-032 | inspection | `docs/theory/ai-response-contract.md` | REQ-027, SPEC-026, IF-029, VAL-024 | pass |
| EVID-033 | inspection | `docs/frame-catalog.md`, `docs/theory/catalog-metadata-migration-plan.md` | REQ-028, SPEC-027, IF-030, VAL-025 | pass |
| EVID-034 | inspection / role review | `docs/theory/local-import-promotion-review.md` | REQ-029, SPEC-028, IF-031, VAL-026 | pass |
| EVID-035 | inspection / role review | `docs/theory/theme-swimlane-role-review.md` | REQ-030, SPEC-029, IF-032, VAL-027 | pass |
| EVID-036 | inspection | `docs/theory/empirical-validation-trial-001-theme-swimlanes.md` | REQ-031, SPEC-030, IF-033, VAL-028 | pass |
| EVID-037 | inspection | `docs/theory/transfer-aware-search-design.md` | REQ-032, SPEC-031, IF-034, VAL-029 | pass |
| EVID-038 | inspection | `docs/theory/theory-gap-audit.md` | REQ-033, SPEC-032, IF-035, VAL-030 | pass |
| EVID-039 | command output / inspection | `cargo run --example ai_response_contract`, `examples/ai_response_contract.rs` | REQ-034, SPEC-033, IF-036, VAL-031 | pass |
| EVID-040 | command output / inspection | `cargo test`, `cargo run --example ai_response_contract`, `src/lib.rs` | REQ-035, SPEC-034, IF-037, VAL-032 | pass |
| EVID-041 | inspection | `docs/theory/accepted-catalog-review-process.md` | REQ-036, SPEC-035, IF-038, VAL-033 | pass |
| EVID-042 | inspection | `docs/theory/frame-antipattern-taxonomy.md` | REQ-037, SPEC-036, IF-039, VAL-034 | pass |
| EVID-043 | inspection | `docs/theory/related-frame-taxonomy.md` | REQ-038, SPEC-037, IF-040, VAL-035 | pass |
| EVID-044 | inspection | `docs/theory/evaluation-set-design.md` | REQ-039, SPEC-038, IF-041, VAL-036 | pass |
| EVID-045 | inspection | `docs/theory/cultural-portability.md` | REQ-040, SPEC-039, IF-042, VAL-037 | pass |
| EVID-046 | inspection | `docs/theory/theme-swimlane-leadership-worksheet.md` | REQ-041, SPEC-040, IF-043, VAL-038 | pass |
| EVID-047 | command output / inspection | `cargo test`, `cargo run --example lookup`, `cargo run --example ai_response_contract`, `src/lib.rs` | REQ-042, SPEC-041, IF-044, VAL-039 | pass |
| EVID-048 | inspection | `docs/validation/evt-001-theme-swimlanes-runbook.md`, `docs/validation/evt-001-response-and-scoring-sheet.md`, `docs/validation/evt-001-theme-swimlanes-results.md` | REQ-043, SPEC-042, IF-045, VAL-040 | pass |
| EVID-049 | inspection | `docs/theory/frame-ontology.md` | REQ-044, SPEC-043, IF-046, VAL-041 | pass |
| EVID-050 | inspection | `docs/frame-catalog.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/frame-ontology.md` | REQ-045, SPEC-044, IF-047, VAL-042 | pass |
| EVID-051 | inspection | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/accepted-catalog-review-process.md`, `docs/theory/local-import-promotion-review.md` | REQ-046, SPEC-045, IF-048, VAL-043 | pass |
| EVID-052 | inspection | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/theory-roadmap.md` | REQ-047, SPEC-046, IF-049, VAL-044 | pass |
| EVID-053 | inspection | `docs/frame-catalog.md`, `docs/theory/catalog-metadata-migration-plan.md` | REQ-048, SPEC-047, IF-050, VAL-045 | pass |
| EVID-054 | inspection | `docs/theory/frame-antipattern-application-veto-rule.md`, `docs/theory/frame-antipattern-taxonomy.md`, `docs/theory/evaluation-set-design.md` | REQ-049, SPEC-048, IF-051, VAL-046 | pass |
| EVID-055 | inspection | `docs/theory/related-frame-application-starter.md`, `docs/theory/related-frame-taxonomy.md`, `docs/theory/evaluation-set-design.md`, `docs/frame-catalog.md` | REQ-050, SPEC-049, IF-052, VAL-047 | pass |
| EVID-056 | inspection / JSON parse | `docs/eval/starter-fixtures.json`, `docs/eval/README.md`, `docs/theory/evaluation-set-design.md` | REQ-051, SPEC-050, IF-053, VAL-048 | pass |
| EVID-057 | inspection / JSON parse | `docs/eval/starter-fixtures.json`, `docs/theory/cultural-portability-application-fixtures.md`, `docs/theory/cultural-portability.md` | REQ-052, SPEC-051, IF-054, VAL-049 | pass |
| EVID-058 | inspection | `docs/validation/theme-swimlane-leadership-pilot-ledger.md`, `docs/theory/theme-swimlane-leadership-worksheet.md`, `docs/theory/theme-swimlane-role-review.md`, `docs/theory/theme-swimlane-extraction.md` | REQ-053, SPEC-052, IF-055, VAL-050 | pass |
| EVID-059 | inspection | `docs/theory/frame-acquisition-method.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | REQ-054, SPEC-053, IF-056, VAL-051 | pass |
| EVID-060 | inspection | `docs/theory/domain-pack-roadmap.md`, `docs/theory/application-pack-templates.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | REQ-055, SPEC-054, IF-057, VAL-052 | pass |
| EVID-061 | inspection | `docs/theory/plain-language-fallbacks.md`, `docs/theory/ai-response-contract.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/frame-acquisition-method.md` | REQ-056, SPEC-055, IF-058, VAL-053 | pass |
| EVID-062 | inspection | `docs/theory/learning-progression.md`, `docs/theory/application-pack-templates.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | REQ-057, SPEC-056, IF-059, VAL-054 | pass |
| EVID-063 | inspection | `docs/theory/lifecycle-filtering-and-rejection-reporting.md`, `docs/theory/frame-lifecycle.md`, `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md` | REQ-058, SPEC-057, IF-060, VAL-055 | pass |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Starter search is lexical, not semantic. | Some useful analogies may not rank without tag hints. | Accept for first deterministic index; revisit with explicit work package. |
| Starter catalog is small. | Search coverage is limited. | Accept for foundation; expand through frame-pack pulses. |
