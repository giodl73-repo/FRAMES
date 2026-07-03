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
| EVID-008 | command | `cargo test` | Traffic-pack alignment tests pass. | pass. | pass |
| EVID-009 | inspection | `docs/frame-catalog.md`, `docs/examples/traffic-and-motion.md`, `src/lib.rs` | Traffic frame docs and index entries are aligned. | pass. | pass |
| EVID-010 | inspection | `docs/theory/frame-theory.md` | Frame theory covers core parts, jobs, fit tests, evidence boundaries, and misuse patterns. | pass. | pass |
| EVID-011 | role review | `docs/theory/role-review-2026-07-03.md` | Role panel review completed. | pass. | pass |
| EVID-012 | inspection | `docs/theory/fit-rubric.md` | Scored accept/revise/hold/reject rubric exists. | pass. | pass |
| EVID-013 | inspection | `docs/theory/theory-roadmap.md` | Remaining theory work is prioritized. | pass. | pass |
| EVID-014 | inspection | `docs/theory/audience-transfer.md` | Audience transfer guide exists. | pass. | pass |
| EVID-015 | inspection | `docs/theory/frame-lifecycle.md` | Frame lifecycle model exists. | pass. | pass |
| EVID-016 | inspection | `docs/theory/composition-and-conflict.md` | Composition and conflict guide exists. | pass. | pass |
| EVID-017 | command | `cargo test` | Evidence-boundary unit test passes. | pass. | pass |
| EVID-018 | inspection | `src/lib.rs`, `examples/lookup.rs`, `docs/theory/evidence-boundary-schema.md` | Evidence-boundary field, example display, and schema doc exist. | pass. | pass |
| EVID-019 | inspection | `docs/theory/research-grounding.md` | Research grounding guide exists with cognitive-science pillars, claim limits, design rules, and bibliography. | pass. | pass |
| EVID-020 | inspection / role review | `docs/theory/source-domain-taxonomy.md`, `.roles/parliament/research-grounding-reviewer.md` | Source-domain taxonomy and research-grounding role lens exist. | pass. | pass |
| EVID-021 | inspection | `docs/theory/application-pack-templates.md` | Application-pack templates exist for product, operations, leadership, learning, and AI-agent contexts. | pass. | pass |
| EVID-022 | inspection | `docs/theory/perspective-metadata.md` | Perspective metadata guide exists with fields, roles, fit test, conflicts, examples, and anti-patterns. | pass. | pass |
| EVID-023 | inspection | `docs/theory/story-job-taxonomy.md`, `docs/theory/fit-rubric.md` | Story-job taxonomy and fit-rubric overlay exist. | pass. | pass |
| EVID-024 | inspection | `docs/theory/relational-transfer-fields.md`, `docs/theory/fit-rubric.md` | Relational transfer fields and fit-rubric integration exist. | pass. | pass |
| EVID-025 | inspection | `docs/theory/claim-strength-labels.md`, `docs/theory/research-grounding.md` | Claim-strength labels and research-grounding integration exist. | pass. | pass |
| EVID-026 | inspection / role review | `docs/theory/role-reviewed-domain-examples.md`, `.roles/ROLE.md` | Role-reviewed domain examples exist for traffic coordination, status signals, conflict repair, and theme swimlanes. | pass. | pass |
| EVID-027 | inspection | `docs/theory/resonance-manage-frame-imports.md`, `docs/theory/resonance-manage-import-map.md` | Structured RESONANCE MANAGE imports exist for high-value local patterns. | pass. | pass |
| EVID-028 | inspection | `docs/theory/career-gravity-frame-imports.md`, `docs/theory/career-gravity-import-map.md` | Structured CAREER Gravity imports exist for high-value local patterns. | pass. | pass |
| EVID-029 | inspection | `docs/theory/theme-swimlane-extraction.md` | Theme-swimlane extraction exists for local program theme patterns. | pass. | pass |
| EVID-030 | inspection | `docs/theory/empirical-validation-plan.md` | Empirical validation plan exists with study levels, measures, templates, and hard stops. | pass. | pass |
| EVID-031 | inspection | `docs/theory/catalog-metadata-migration-plan.md`, `src/lib.rs` | Catalog metadata migration plan exists and reflects the current indexed shape. | pass. | pass |
| EVID-032 | inspection | `docs/theory/ai-response-contract.md` | AI response contract exists with required fields, scoring distinction, gates, language rules, and examples. | pass. | pass |
| EVID-033 | inspection | `docs/frame-catalog.md`, `docs/theory/catalog-metadata-migration-plan.md` | Accepted starter metadata table exists and excludes local draft imports. | pass. | pass |
| EVID-034 | inspection / role review | `docs/theory/local-import-promotion-review.md` | Local import promotion review exists with promoted draft heuristics and held imports. | pass. | pass |
| EVID-035 | inspection / role review | `docs/theory/theme-swimlane-role-review.md` | Theme-swimlane role review exists with acceptance decision, role findings, fit score, evidence gates, hard stops, and output template. | pass. | pass |
| EVID-036 | inspection | `docs/theory/empirical-validation-trial-001-theme-swimlanes.md` | First empirical validation trial protocol exists with scenario, comparison conditions, response form, scoring rubric, analysis plan, report template, hard stops, and no claim upgrade. | pass. | pass |
| EVID-037 | inspection | `docs/theory/transfer-aware-search-design.md` | Transfer-aware search design exists with query inputs, metadata gates, scoring order, hard stops, output notes, migration path, and no immediate Rust API change. | pass. | pass |
| EVID-038 | inspection | `docs/theory/theory-gap-audit.md` | Theory gap audit exists with current strengths, blocking gaps, growth gaps, implementation gaps, deep theory gaps, recommended sequence, non-goals, and decision statement. | pass. | pass |
| EVID-039 | command / inspection | `cargo run --example ai_response_contract`; `examples/ai_response_contract.rs` | AI response contract example runs and displays retrieval score, fit-score caveat, action cue, evidence boundary, misuse warning, fallback, hard stops, alternates, and notes. | pass. | pass |
| EVID-040 | command / inspection | `cargo test`; `cargo run --example ai_response_contract`; `src/lib.rs`; `docs/theory/catalog-metadata-migration-plan.md` | Accepted starter entries expose Rust status, claim strength, risk band, application packs, and metadata filters; AI response example reads metadata from `FrameEntry`. | pass. | pass |

## Evidence Rules

- Evidence IDs are stable and referenced from `TRACE.md`.
- Command evidence records the exact command and result.
- Review evidence points to `REVIEW.md`.
- Deferred evidence includes a revisit trigger.
