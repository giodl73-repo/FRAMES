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
| WP-003 | Expand traffic frame pack. | Accepted traffic frame docs and indexed entries stay aligned. | REQ-007 / SPEC-006 / IF-009 | `docs/frame-catalog.md`, `docs/examples/traffic-and-motion.md`, `src/lib.rs`, `context/waves/*`, `docs/vtrace/*` | WP-001 complete and traffic-pack candidates accepted. | Traffic docs include added frames, Rust index tests pass, VTRACE validates. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review if publishing catalog | evidence / trace / review / status rows | complete |
| WP-004 | Add role-reviewed frame theory baseline. | Frame expansion uses explicit theory for fit, action, evidence, and misuse. | REQ-008 / SPEC-007 / IF-010 | `docs/theory/*`, `README.md`, `PRODUCT_PLAN.md`, `context/waves/*`, `docs/vtrace/*` | Role panel exists and traffic/frame-index foundation is complete. | Theory docs and role review exist, docs checks pass, VTRACE validates. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: research review before public cognitive-science claims | evidence / trace / review / status rows | complete |
| WP-005 | Add fit rubric and theory roadmap. | Frame candidates can be scored before catalog/index acceptance, and future theory work is prioritized. | REQ-009 / SPEC-008 / IF-011 | `docs/theory/*`, `README.md`, `context/waves/*`, `docs/vtrace/*` | WP-004 complete. | Fit rubric and roadmap exist, pulse 04 closed, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before publishing rubric as stable | evidence / trace / review / status rows | complete |
| WP-006 | Add audience transfer guide. | Frame selection accounts for role, expertise, region, culture, mobility, stakes, and power differences. | REQ-010 / SPEC-009 / IF-012 | `docs/theory/audience-transfer.md`, `docs/theory/theory-roadmap.md`, `README.md`, `docs/vtrace/*` | WP-005 complete. | Audience transfer guide exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before adding audience metadata to crate | evidence / trace / review / status rows | complete |
| WP-007 | Add frame lifecycle model. | Frame catalog entries have controlled status transitions and indexing rules. | REQ-011 / SPEC-010 / IF-013 | `docs/theory/frame-lifecycle.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md`, `README.md`, `docs/vtrace/*` | WP-006 complete. | Lifecycle guide exists, roadmap fixed, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding lifecycle in crate | evidence / trace / review / status rows | complete |
| WP-008 | Add composition and conflict theory. | Related frames can be composed or rejected based on action, authority, evidence, audience, and safety rules. | REQ-012 / SPEC-011 / IF-014 | `docs/theory/composition-and-conflict.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md`, `README.md`, `docs/vtrace/*` | WP-007 complete. | Composition guide exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding composition metadata in crate | evidence / trace / review / status rows | complete |
| WP-009 | Add evidence-boundary schema to frame index. | AI/tool callers receive the evidence obligation with each indexed frame. | REQ-013 / SPEC-012 / IF-015 | `src/lib.rs`, `examples/lookup.rs`, `docs/theory/evidence-boundary-schema.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-008 complete. | `FrameEntry` has required evidence boundary, starter catalog populated, tests and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before publishing API | evidence / trace / review / status rows | complete |
| WP-010 | Add research-grounding theory. | Public FRAMES claims about cognition, metaphor, analogy, and persuasion are bounded by research-grounding rules. | REQ-014 / SPEC-013 / IF-016 | `docs/theory/research-grounding.md`, `docs/theory/frame-theory.md`, `docs/theory/theory-roadmap.md`, `README.md`, `docs/vtrace/*` | WP-009 complete and first research sources identified. | Research-grounding guide exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: literature review before public research claims | evidence / trace / review / status rows | complete |
| WP-011 | Add source-domain taxonomy. | Frame source domains are classified by embodied schema, social script, authority model, temporal shape, risk band, and portability. | REQ-015 / SPEC-014 / IF-017 | `docs/theory/source-domain-taxonomy.md`, `.roles/*`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-010 complete. | Source-domain taxonomy exists, research-grounding role lens exists, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding taxonomy in crate metadata | evidence / trace / review / status rows | complete |
| WP-012 | Add application-pack templates. | Product, operations, leadership, learning, and AI-agent contexts have explicit frame-selection defaults and rejection rules. | REQ-016 / SPEC-015 / IF-018 | `docs/theory/application-pack-templates.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-011 complete. | Pack templates exist, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding packs in crate metadata | evidence / trace / review / status rows | complete |
| WP-013 | Add perspective metadata theory. | Frame selection exposes the listener role, agency, duty, authority, and perspective risk implied by a source scene. | REQ-017 / SPEC-016 / IF-019 | `docs/theory/perspective-metadata.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-012 complete. | Perspective metadata guide exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding perspective in crate metadata | evidence / trace / review / status rows | complete |
| WP-014 | Add story-job taxonomy. | Narrative frame use distinguishes analogy function from audience story purpose. | REQ-018 / SPEC-017 / IF-020 | `docs/theory/story-job-taxonomy.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-013 complete and external practitioner benchmark exists. | Story-job taxonomy exists, fit-rubric overlay exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding story jobs in crate metadata | evidence / trace / review / status rows | complete |
| WP-015 | Add relational transfer fields. | Frame selection maps source-target relations, exclusions, and transfer strength before catalog acceptance. | REQ-019 / SPEC-018 / IF-021 | `docs/theory/relational-transfer-fields.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-014 complete. | Relational transfer guide exists, fit-rubric integration exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding transfer fields in crate metadata | evidence / trace / review / status rows | complete |
| WP-016 | Add claim-strength labels. | Frame claims declare whether they are illustrative, heuristic, theory-informed, observed, reviewed, validated, or anti-patterns. | REQ-020 / SPEC-019 / IF-022 | `docs/theory/claim-strength-labels.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-015 complete. | Claim-strength guide exists, research-grounding integration exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before encoding claim strength in crate metadata | evidence / trace / review / status rows | complete |
| WP-017 | Add role-reviewed domain examples. | Concrete domains are reviewed through role lenses before broader catalog expansion. | REQ-021 / SPEC-020 / IF-023 | `docs/theory/role-reviewed-domain-examples.md`, `.roles/*`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-016 complete. | Role-reviewed examples exist, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before catalog expansion | evidence / trace / review / status rows | complete |
| WP-018 | Add structured RESONANCE MANAGE imports. | High-value local management patterns are converted into FRAMES-native draft entries. | REQ-022 / SPEC-021 / IF-024 | `docs/theory/resonance-manage-frame-imports.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-017 complete and import map exists. | Structured imports exist, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before catalog acceptance | evidence / trace / review / status rows | complete |
| WP-019 | Add structured CAREER Gravity imports. | High-value local career/gravity patterns are converted into FRAMES-native draft entries. | REQ-023 / SPEC-022 / IF-025 | `docs/theory/career-gravity-frame-imports.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-018 complete and import map exists. | Structured imports exist, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before catalog acceptance | evidence / trace / review / status rows | complete |
| WP-020 | Add theme-swimlane extraction. | Local program theme patterns are converted into promise, lane, owner, measure, tradeoff, and exclusion fields. | REQ-024 / SPEC-023 / IF-026 | `docs/theory/theme-swimlane-extraction.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-019 complete and local theme pattern identified. | Theme-swimlane extraction exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: business-leader and evidence-boundary review before heuristic promotion | evidence / trace / review / status rows | complete |
| WP-021 | Add empirical validation plan. | Frame claim upgrades are supported by narrow audience, context, task, comparison, measure, result, and boundary records. | REQ-025 / SPEC-024 / IF-027 | `docs/theory/empirical-validation-plan.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-020 complete and claim-strength labels exist. | Empirical validation plan exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: research review before any public validated claim | evidence / trace / review / status rows | complete |
| WP-022 | Add catalog metadata migration plan. | Theory fields have a staged path into catalog docs and `frames-core` without premature API churn. | REQ-026 / SPEC-025 / IF-028 | `docs/theory/catalog-metadata-migration-plan.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*`, `src/lib.rs` | WP-021 complete and current index shape inspected. | Migration plan exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: API review before Rust metadata fields are added | evidence / trace / review / status rows | complete |
| WP-023 | Add AI response contract. | AI/tool frame suggestions return bounded recommendations with action, evidence, warning, score semantics, claim strength, and alternates. | REQ-027 / SPEC-026 / IF-029 | `docs/theory/ai-response-contract.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-022 complete and application-pack defaults exist. | Response contract exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: API review before implementing contract structs | evidence / trace / review / status rows | complete |
| WP-024 | Add metadata-backed accepted starter catalog. | Accepted starter frames expose docs-level metadata before Rust API migration. | REQ-028 / SPEC-027 / IF-030 | `docs/frame-catalog.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-023 complete and migration plan exists. | Accepted starter metadata table exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: catalog structure review before Rust field migration | evidence / trace / review / status rows | complete |
| WP-025 | Add role-reviewed local import promotion. | Local imports have explicit promote/hold outcomes before catalog acceptance. | REQ-029 / SPEC-028 / IF-031 | `docs/theory/local-import-promotion-review.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-024 complete and local structured imports exist. | Promotion review exists, import statuses updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: fit scoring before accepted catalog promotion | evidence / trace / review / status rows | complete |
| WP-026 | Add theme-swimlane role review. | Theme swimlanes have explicit role gates before leadership-pack or catalog acceptance. | REQ-030 / SPEC-029 / IF-032 | `docs/theory/theme-swimlane-role-review.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-025 complete and theme swimlanes promoted only as draft heuristic. | Role review exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: pilot review before accepted catalog promotion | evidence / trace / review / status rows | complete |
| WP-027 | Add first empirical validation trial protocol. | Claim upgrades have a concrete comparison protocol before empirical claims are made. | REQ-031 / SPEC-030 / IF-033 | `docs/theory/empirical-validation-trial-001-theme-swimlanes.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-026 complete and validation plan exists. | Trial protocol exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: execute trial before claim-strength upgrade | evidence / trace / review / status rows | complete |
| WP-028 | Add transfer-aware search design. | Future search ranks structural fit before surface wording or vivid source scenes. | REQ-032 / SPEC-031 / IF-034 | `docs/theory/transfer-aware-search-design.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-027 complete and relational transfer fields exist. | Search design exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: API review before Rust ranking changes | evidence / trace / review / status rows | complete |
| WP-029 | Add theory gap audit. | Remaining theory gaps are visible before catalog growth or AI selection scale. | REQ-033 / SPEC-032 / IF-035 | `docs/theory/theory-gap-audit.md`, `docs/theory/theory-roadmap.md`, `README.md`, `docs/vtrace/*` | WP-028 complete and current theory set inspected. | Gap audit exists, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `git diff --check` / L1: VTRACE validate / L2: role review before major roadmap pivot | evidence / trace / review / status rows | complete |
| WP-030 | Add AI response contract Rust example. | Tool builders can see bounded frame suggestions using current `FrameIndex` output. | REQ-034 / SPEC-033 / IF-036 | `examples/ai_response_contract.rs`, `docs/theory/ai-response-contract.md`, `README.md`, `docs/theory/theory-roadmap.md`, `docs/vtrace/*` | WP-029 complete and AI response contract exists. | Example runs, docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: API review before contract structs | evidence / trace / review / status rows | complete |
| WP-031 | Add accepted starter Rust metadata migration. | Accepted starter frames expose basic display-safety metadata and filters. | REQ-035 / SPEC-034 / IF-037 | `src/lib.rs`, `examples/ai_response_contract.rs`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-030 complete and accepted starter metadata table exists. | Metadata fields and filters exist, examples run, docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: API review before transfer-aware ranking | evidence / trace / review / status rows | complete |
| WP-032 | Add accepted-catalog review process. | Draft heuristics have a repeatable gate before accepted catalog or default search. | REQ-036 / SPEC-035 / IF-038 | `docs/theory/accepted-catalog-review-process.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-031 complete and lifecycle/promotion docs exist. | Process exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply process to first promoted candidate | evidence / trace / review / status rows | complete |
| WP-033 | Add frame anti-pattern taxonomy. | Bad frames have reusable rejection classes before rejected examples or evaluation sets expand. | REQ-037 / SPEC-036 / IF-039 | `docs/theory/frame-antipattern-taxonomy.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-032 complete and hard-stop/review docs exist. | Taxonomy exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply taxonomy to evaluation fixtures and rejected-candidate reporting | evidence / trace / review / status rows | complete |
| WP-034 | Add related-frame relation taxonomy. | Related links have stable meanings before typed metadata or relation-aware evaluation sets. | REQ-038 / SPEC-037 / IF-040 | `docs/theory/related-frame-taxonomy.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-033 complete and composition/search docs exist. | Taxonomy exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply relation taxonomy to catalog rows and evaluation fixtures | evidence / trace / review / status rows | complete |
| WP-035 | Add evaluation-set design. | Frame selection behavior has fixture rules before semantic search or draft-frame inclusion. | REQ-039 / SPEC-038 / IF-041 | `docs/theory/evaluation-set-design.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-034 complete and anti-pattern/relation taxonomies exist. | Evaluation design exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: populate first machine-readable fixtures | evidence / trace / review / status rows | complete |
| WP-036 | Add cultural portability guidance. | Everyday source frames have portability checks before broad catalog or AI use. | REQ-040 / SPEC-039 / IF-042 | `docs/theory/cultural-portability.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-035 complete and audience-transfer docs exist. | Portability guide exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply portability bands to catalog rows and evaluation fixtures | evidence / trace / review / status rows | complete |
| WP-037 | Add theme-swimlane leadership worksheet. | Theme Swimlanes have a leadership-pack pilot artifact before catalog acceptance. | REQ-041 / SPEC-040 / IF-043 | `docs/theory/theme-swimlane-leadership-worksheet.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-036 complete and theme-swimlane role review exists. | Worksheet exists, linked docs updated, roadmap updated, docs checks and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: run a real pilot and record changed decisions | evidence / trace / review / status rows | complete |
| WP-038 | Add transfer-aware search filters. | AI/tool users can filter accepted starter search by authority, risk, and application context. | REQ-042 / SPEC-041 / IF-044 | `src/lib.rs`, `examples/*`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-037 complete and accepted starter metadata exists. | Authority, risk, and application-pack filters exist, examples run, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: run evaluation fixtures before relation-aware ranking | evidence / trace / review / status rows | complete |
| WP-039 | Add EVT-001 execution packet. | Theme Swimlanes empirical validation can collect responses against locked prompts, scoring, and report shape. | REQ-043 / SPEC-042 / IF-045 | `docs/validation/*`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-038 complete and EVT-001 protocol exists. | Runbook, response sheet, empty results ledger, docs, and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: collect and score participant responses | evidence / trace / review / status rows | complete |
| WP-040 | Add frame ontology. | Catalog growth uses controlled jobs, relations, authority, risk, and tag families. | REQ-044 / SPEC-043 / IF-046 | `docs/theory/frame-ontology.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-039 complete and gap audit identifies ontology drift risk. | Ontology exists, roadmap/gap audit updated, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply ontology terms to catalog rows and fixtures | evidence / trace / review / status rows | complete |
| WP-041 | Apply ontology to catalog and fixtures. | Accepted starter rows and evaluation fixtures use controlled ontology terms. | REQ-045 / SPEC-044 / IF-047 | `docs/frame-catalog.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/frame-ontology.md`, `docs/theory/*`, `docs/vtrace/*` | WP-040 complete and ontology terms defined. | Catalog and fixture backlog use controlled terms, roadmap/gap audit updated, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply accepted-catalog review to promoted candidates | evidence / trace / review / status rows | complete |
| WP-042 | Apply accepted-catalog review to Veto Rule. | The first promoted local candidate has a recorded review decision before catalog/index promotion. | REQ-046 / SPEC-045 / IF-048 | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-041 complete and Veto Rule is promoted draft heuristic. | Veto Rule review records revise decision, revision items, and no default search promotion; docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: add fixtures and stop condition before catalog acceptance | evidence / trace / review / status rows | complete |
| WP-043 | Close Veto Rule revision items. | Veto Rule has fixtures, stop condition, fallback language, and a docs-only accepted-with-caveat boundary. | REQ-047 / SPEC-046 / IF-049 | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/evaluation-set-design.md`, `docs/theory/*`, `docs/vtrace/*` | WP-042 complete and revision items identified. | Revision items closed, roadmap updated, docs and VTRACE pass, default Rust search unchanged. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: add docs catalog row when catalog scope expands | evidence / trace / review / status rows | complete |
| WP-044 | Add Veto Rule docs-catalog row. | Reviewed docs-catalog candidates are visible without changing starter catalog or default Rust search. | REQ-048 / SPEC-047 / IF-050 | `docs/frame-catalog.md`, `docs/theory/*`, `docs/vtrace/*` | WP-043 complete and Veto Rule accepted-with-caveat boundary exists. | Veto Rule row added under reviewed docs-catalog candidates, roadmap updated, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: add lifecycle filtering before default search expansion | evidence / trace / review / status rows | complete |
| WP-045 | Apply anti-pattern taxonomy to Veto Rule. | Veto Rule misuse risks are classified and covered by evaluation fixture backlog. | REQ-049 / SPEC-048 / IF-051 | `docs/theory/frame-antipattern-application-veto-rule.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-044 complete and Veto Rule docs-catalog row exists. | Anti-pattern review exists, fixture backlog updated, roadmap updated, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: apply related-frame taxonomy to catalog examples | evidence / trace / review / status rows | complete |
| WP-046 | Apply related-frame taxonomy to starter links. | Starter related IDs have docs-level relation types and fixture backlog coverage. | REQ-050 / SPEC-049 / IF-052 | `docs/theory/related-frame-application-starter.md`, `docs/theory/related-frame-taxonomy.md`, `docs/theory/evaluation-set-design.md`, `docs/frame-catalog.md`, `README.md`, `docs/vtrace/*` | WP-045 complete and related-frame taxonomy exists. | Applied relation map exists, fixture backlog updated, roadmap updated, Rust API unchanged, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: populate first machine-readable evaluation fixtures | evidence / trace / review / status rows | complete |
| WP-047 | Publish starter evaluation fixture package. | The starter fixture backlog is available as a parseable docs-level package. | REQ-051 / SPEC-050 / IF-053 | `docs/eval/*`, `docs/theory/evaluation-set-design.md`, `README.md`, `docs/vtrace/*` | WP-046 complete and fixture backlog categories are stable. | JSON fixture package parses, covers starter categories, roadmap updated, docs and VTRACE pass. | L0: JSON parse; `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: add fixture runner only after package shape stabilizes | evidence / trace / review / status rows | complete |
| WP-048 | Apply portability bands to starter fixtures. | Starter fixtures carry portability bands and safer fallbacks. | REQ-052 / SPEC-051 / IF-054 | `docs/eval/starter-fixtures.json`, `docs/theory/cultural-portability-application-fixtures.md`, `docs/theory/*`, `README.md`, `docs/vtrace/*` | WP-047 complete and cultural portability taxonomy exists. | Portability profiles cover all starter fixtures, applied review exists, roadmap updated, docs and VTRACE pass. | L0: JSON parse; `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: theme-swimlane worksheet pilot or frame acquisition method | evidence / trace / review / status rows | complete |
| WP-049 | Add theme-swimlane leadership pilot ledger. | Real Theme Swimlane worksheet pilots have a ledger before evidence claims. | REQ-053 / SPEC-052 / IF-055 | `docs/validation/theme-swimlane-leadership-pilot-ledger.md`, `docs/theory/theme-swimlane-*`, `README.md`, `docs/vtrace/*` | WP-048 complete and worksheet exists. | Pilot ledger exists, dry-run is marked not evidence, linked docs updated, docs and VTRACE pass. | L0: `cargo fmt --check`; `cargo test`; `cargo run --example lookup`; `cargo run --example ai_response_contract`; `git diff --check` / L1: VTRACE validate / L2: run a real pilot and record changed decisions | evidence / trace / review / status rows | complete |

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

### WP-003: Expand traffic frame pack

Objective: add traffic-pack frames to docs and the starter index.

Parent IDs: REQ-007, SPEC-006, IF-009.

Affected files/modules:

- `docs/frame-catalog.md`
- `docs/examples/traffic-and-motion.md`
- `src/lib.rs`
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

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-007 | closed | Traffic docs and starter index alignment required. |
| Specification / Interface | SPEC-006, IF-009 | closed | Stable IDs added for new traffic frames. |
| Implementation | `docs/frame-catalog.md`, `docs/examples/traffic-and-motion.md`, `src/lib.rs` | closed | Speed limit, shoulder/pull-off, and following distance added. |
| Verification | EVID-008, EVID-009 | closed | Unit tests and inspection cover alignment. |
| Validation | VAL-003, VAL-004 | closed | Warnings visible in docs and index. |

### WP-004: Add role-reviewed frame theory baseline

Objective: define the theory needed before broader catalog expansion.

Parent IDs: REQ-008, SPEC-007, IF-010.

Affected files/modules:

- `docs/theory/frame-theory.md`
- `docs/theory/role-review-2026-07-03.md`
- `README.md`
- `PRODUCT_PLAN.md`
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

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-008 | closed | Theory required before broader catalog expansion. |
| Specification / Interface | SPEC-007, IF-010 | closed | Theory terms defined and linked to frame-index vocabulary. |
| Implementation | `docs/theory/*` | closed | Theory and role review added. |
| Verification | EVID-010, EVID-011 | closed | Inspection and role review cover theory readiness. |
| Validation | VAL-005 | closed | Theory can guide future frame-pack selection. |

### WP-005: Add fit rubric and theory roadmap

Objective: make frame readiness comparable and identify remaining theory work.

Parent IDs: REQ-009, SPEC-008, IF-011.

Affected files/modules:

- `docs/theory/fit-rubric.md`
- `docs/theory/theory-roadmap.md`
- `docs/theory/frame-theory.md`
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

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-009 | closed | Scored rubric and roadmap required. |
| Specification / Interface | SPEC-008, IF-011 | closed | Rubric dimensions and decision bands defined. |
| Implementation | `docs/theory/fit-rubric.md`, `docs/theory/theory-roadmap.md` | closed | Theory roadmap and scoring rubric added. |
| Verification | EVID-012, EVID-013 | closed | Inspection covers rubric and roadmap. |
| Validation | VAL-006 | closed | Rubric supports accept/revise/hold/reject decisions. |

### WP-006: Add audience transfer guide

Objective: define how frames transfer or fail across audiences.

Parent IDs: REQ-010, SPEC-009, IF-012.

Affected files/modules:

- `docs/theory/audience-transfer.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-010 | closed | Audience transfer guidance required. |
| Specification / Interface | SPEC-009, IF-012 | closed | Transfer dimensions and bands defined. |
| Implementation | `docs/theory/audience-transfer.md` | closed | Guide added with tests, examples, alternates, and future fields. |
| Verification | EVID-014 | closed | Inspection covers audience transfer guide. |
| Validation | VAL-007 | closed | Guide supports safer frame selection across audiences. |

### WP-007: Add frame lifecycle model

Objective: define controlled status transitions and indexing rules for frames.

Parent IDs: REQ-011, SPEC-010, IF-013.

Affected files/modules:

- `docs/theory/frame-lifecycle.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-011 | closed | Lifecycle guidance required. |
| Specification / Interface | SPEC-010, IF-013 | closed | Lifecycle states and indexing rules defined. |
| Implementation | `docs/theory/frame-lifecycle.md` | closed | Lifecycle model added and linked. |
| Verification | EVID-015 | closed | Inspection covers lifecycle guide. |
| Validation | VAL-008 | closed | Lifecycle supports catalog/index status decisions. |

### WP-008: Add composition and conflict theory

Objective: define when multiple frames clarify a situation and when they
conflict.

Parent IDs: REQ-012, SPEC-011, IF-014.

Affected files/modules:

- `docs/theory/composition-and-conflict.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-012 | closed | Composition/conflict guidance required. |
| Specification / Interface | SPEC-011, IF-014 | closed | Composition roles and conflict types defined. |
| Implementation | `docs/theory/composition-and-conflict.md` | closed | Composition and conflict guide added and linked. |
| Verification | EVID-016 | closed | Inspection covers composition guide. |
| Validation | VAL-009 | closed | Guide supports multi-frame selection decisions. |

### WP-009: Add evidence-boundary schema to frame index

Objective: make evidence obligations first-class in indexed frame entries.

Parent IDs: REQ-013, SPEC-012, IF-015.

Affected files/modules:

- `src/lib.rs`
- `examples/lookup.rs`
- `docs/theory/evidence-boundary-schema.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-013 | closed | Indexed frames carry evidence boundaries. |
| Specification / Interface | SPEC-012, IF-015 | closed | `FrameEntry::evidence_boundary` added as required field. |
| Implementation | `src/lib.rs`, `examples/lookup.rs`, `docs/theory/evidence-boundary-schema.md` | closed | Catalog entries populated and example displays evidence boundary. |
| Verification | EVID-017, EVID-018 | closed | Unit test and inspection cover evidence-boundary schema. |
| Validation | VAL-010 | closed | Search result can expose action, evidence, and warning together. |

### WP-010: Add research-grounding theory

Objective: ground FRAMES theory in cognitive-science literature while keeping
claim boundaries explicit.

Parent IDs: REQ-014, SPEC-013, IF-016.

Affected files/modules:

- `docs/theory/research-grounding.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-014 | closed | Research grounding guidance required. |
| Specification / Interface | SPEC-013, IF-016 | closed | Claim boundaries and design implications defined. |
| Implementation | `docs/theory/research-grounding.md` | closed | Research guide added and linked. |
| Verification | EVID-019 | closed | Inspection covers research guide. |
| Validation | VAL-011 | closed | Guide supports safer public cognitive-science claims. |

### WP-011: Add source-domain taxonomy

Objective: classify everyday source domains so frame selection is based on
relational structure, authority, time, risk, and audience portability.

Parent IDs: REQ-015, SPEC-014, IF-017.

Affected files/modules:

- `docs/theory/source-domain-taxonomy.md`
- `.roles/ROLE.md`
- `.roles/parliament/research-grounding-reviewer.md`
- `docs/theory/frame-theory.md`
- `docs/theory/research-grounding.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-015 | closed | Source-domain taxonomy guidance required. |
| Specification / Interface | SPEC-014, IF-017 | closed | Taxonomy terms and change triggers defined. |
| Implementation | `docs/theory/source-domain-taxonomy.md`, `.roles/parliament/research-grounding-reviewer.md` | closed | Taxonomy and role lens added. |
| Verification | EVID-020 | closed | Inspection covers taxonomy and role lens. |
| Validation | VAL-012 | closed | Guide supports source-domain selection before frame-pack expansion. |

### WP-012: Add application-pack templates

Objective: define context-specific frame-selection defaults for product,
operations, leadership, learning, and AI-agent use.

Parent IDs: REQ-016, SPEC-015, IF-018.

Affected files/modules:

- `docs/theory/application-pack-templates.md`
- `docs/theory/frame-theory.md`
- `docs/theory/source-domain-taxonomy.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-016 | closed | Application-pack template guidance required. |
| Specification / Interface | SPEC-015, IF-018 | closed | Pack terms and change triggers defined. |
| Implementation | `docs/theory/application-pack-templates.md` | closed | Product, operations, leadership, learning, and AI-agent packs added. |
| Verification | EVID-021 | closed | Inspection covers application-pack templates. |
| Validation | VAL-013 | closed | Guide supports context-specific frame defaults and rejection rules. |

### WP-013: Add perspective metadata theory

Objective: define how frames assign listener roles, agency, duty, authority, and
perspective risk.

Parent IDs: REQ-017, SPEC-016, IF-019.

Affected files/modules:

- `docs/theory/perspective-metadata.md`
- `docs/theory/frame-theory.md`
- `docs/theory/source-domain-taxonomy.md`
- `docs/theory/application-pack-templates.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-017 | closed | Perspective metadata guidance required. |
| Specification / Interface | SPEC-016, IF-019 | closed | Perspective terms and change triggers defined. |
| Implementation | `docs/theory/perspective-metadata.md` | closed | Perspective fields, roles, conflicts, examples, and anti-patterns added. |
| Verification | EVID-022 | closed | Inspection covers perspective metadata guide. |
| Validation | VAL-014 | closed | Guide supports role, duty, and authority checks for frame selection. |

### WP-014: Add story-job taxonomy

Objective: define how the narrative layer of a frame affects the audience.

Parent IDs: REQ-018, SPEC-017, IF-020.

Affected files/modules:

- `docs/theory/story-job-taxonomy.md`
- `docs/theory/fit-rubric.md`
- `docs/theory/frame-theory.md`
- `docs/theory/external-frame-practitioners.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-018 | closed | Story-job taxonomy guidance required. |
| Specification / Interface | SPEC-017, IF-020 | closed | Story-job categories and overlay gates defined. |
| Implementation | `docs/theory/story-job-taxonomy.md`, `docs/theory/fit-rubric.md` | closed | Taxonomy, examples, hard stops, and rubric overlay added. |
| Verification | EVID-023 | closed | Inspection covers story-job taxonomy and overlay. |
| Validation | VAL-015 | closed | Guide supports narrative-purpose review for frame selection. |

### WP-015: Add relational transfer fields

Objective: define how source-target relations transfer and where they must stop.

Parent IDs: REQ-019, SPEC-018, IF-021.

Affected files/modules:

- `docs/theory/relational-transfer-fields.md`
- `docs/theory/fit-rubric.md`
- `docs/theory/frame-theory.md`
- `docs/theory/source-domain-taxonomy.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-019 | closed | Relational transfer guidance required. |
| Specification / Interface | SPEC-018, IF-021 | closed | Transfer fields and strength labels defined. |
| Implementation | `docs/theory/relational-transfer-fields.md`, `docs/theory/fit-rubric.md` | closed | Fields, procedure, examples, and scoring integration added. |
| Verification | EVID-024 | closed | Inspection covers relational transfer guide and rubric integration. |
| Validation | VAL-016 | closed | Guide supports structure-first frame review. |

### WP-016: Add claim-strength labels

Objective: define how strongly FRAMES may present frame claims and guidance.

Parent IDs: REQ-020, SPEC-019, IF-022.

Affected files/modules:

- `docs/theory/claim-strength-labels.md`
- `docs/theory/research-grounding.md`
- `docs/theory/evidence-boundary-schema.md`
- `docs/theory/frame-theory.md`
- `docs/theory/fit-rubric.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-020 | closed | Claim-strength labels required. |
| Specification / Interface | SPEC-019, IF-022 | closed | Labels, promotion rules, and claim shape defined. |
| Implementation | `docs/theory/claim-strength-labels.md`, `docs/theory/research-grounding.md` | closed | Claim labels and research-grounding integration added. |
| Verification | EVID-025 | closed | Inspection covers claim-strength guide and integration. |
| Validation | VAL-017 | closed | Guide supports public and AI-facing claim review. |

### WP-017: Add role-reviewed domain examples

Objective: show how concrete frame patterns are accepted, revised, held, or
rejected through role lenses.

Parent IDs: REQ-021, SPEC-020, IF-023.

Affected files/modules:

- `docs/theory/role-reviewed-domain-examples.md`
- `.roles/ROLE.md`
- `.roles/parliament/*`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-021 | closed | Role-reviewed domain examples required. |
| Specification / Interface | SPEC-020, IF-023 | closed | Review shape and outcome labels defined. |
| Implementation | `docs/theory/role-reviewed-domain-examples.md` | closed | Domain examples and role pressure added. |
| Verification | EVID-026 | closed | Inspection covers examples and role alignment. |
| Validation | VAL-018 | closed | Examples demonstrate concrete theory decisions. |

### WP-018: Add structured RESONANCE MANAGE imports

Objective: convert high-value RESONANCE MANAGE patterns into FRAMES-native draft
entries without overclaiming their evidence status.

Parent IDs: REQ-022, SPEC-021, IF-024.

Affected files/modules:

- `docs/theory/resonance-manage-frame-imports.md`
- `docs/theory/resonance-manage-import-map.md`
- `docs/theory/claim-strength-labels.md`
- `docs/theory/relational-transfer-fields.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-022 | closed | Structured RESONANCE MANAGE imports required. |
| Specification / Interface | SPEC-021, IF-024 | closed | Import draft shape and promotion criteria defined. |
| Implementation | `docs/theory/resonance-manage-frame-imports.md` | closed | Five high-value local patterns imported as draft entries. |
| Verification | EVID-027 | closed | Inspection covers structured imports. |
| Validation | VAL-019 | closed | Imports preserve provenance, boundaries, and draft status. |

### WP-019: Add structured CAREER Gravity imports

Objective: convert high-value CAREER Gravity patterns into FRAMES-native draft
entries without overclaiming their evidence status.

Parent IDs: REQ-023, SPEC-022, IF-025.

Affected files/modules:

- `docs/theory/career-gravity-frame-imports.md`
- `docs/theory/career-gravity-import-map.md`
- `docs/theory/claim-strength-labels.md`
- `docs/theory/relational-transfer-fields.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-023 | closed | Structured CAREER Gravity imports required. |
| Specification / Interface | SPEC-022, IF-025 | closed | Import draft shape and promotion criteria defined. |
| Implementation | `docs/theory/career-gravity-frame-imports.md` | closed | Six high-value local patterns imported as draft entries. |
| Verification | EVID-028 | closed | Inspection covers structured imports. |
| Validation | VAL-020 | closed | Imports preserve provenance, boundaries, and draft status. |

### WP-020: Add theme-swimlane extraction

Objective: convert local program theme patterns into structured contribution
lanes without letting slogans replace customer outcome evidence.

Parent IDs: REQ-024, SPEC-023, IF-026.

Affected files/modules:

- `docs/theory/theme-swimlane-extraction.md`
- `docs/theory/claim-strength-labels.md`
- `docs/theory/role-reviewed-domain-examples.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-024 | closed | Theme-swimlane extraction required. |
| Specification / Interface | SPEC-023, IF-026 | closed | Promise, lane, owner, measure, tradeoff, and exclusion fields defined. |
| Implementation | `docs/theory/theme-swimlane-extraction.md` | closed | Pattern shape, local variants, extraction procedure, and fit checks added. |
| Verification | EVID-029 | closed | Inspection covers extraction guide. |
| Validation | VAL-021 | closed | Guide supports business-leader theme extraction without overclaiming. |

### WP-021: Add empirical validation plan

Objective: define how FRAMES can test frame effects without making broad
persuasion or universality claims.

Parent IDs: REQ-025, SPEC-024, IF-027.

Affected files/modules:

- `docs/theory/empirical-validation-plan.md`
- `docs/theory/claim-strength-labels.md`
- `docs/theory/research-grounding.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-025 | closed | Empirical validation guidance required. |
| Specification / Interface | SPEC-024, IF-027 | closed | Study levels, measures, report fields, and hard stops defined. |
| Implementation | `docs/theory/empirical-validation-plan.md` | closed | Validation boundary, protocol, templates, and backlog added. |
| Verification | EVID-030 | closed | Inspection covers validation plan. |
| Validation | VAL-022 | closed | Guide supports bounded claim-strength upgrades. |

### WP-022: Add catalog metadata migration plan

Objective: define how FRAMES promotes theory fields into catalog rows and Rust
metadata without premature API churn.

Parent IDs: REQ-026, SPEC-025, IF-028.

Affected files/modules:

- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `src/lib.rs`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-026 | closed | Catalog metadata migration guidance required. |
| Specification / Interface | SPEC-025, IF-028 | closed | Metadata families, stages, promotion rules, and compatibility gates defined. |
| Implementation | `docs/theory/catalog-metadata-migration-plan.md` | closed | Migration plan and first field backlog added. |
| Verification | EVID-031 | closed | Inspection covers migration plan and current Rust shape. |
| Validation | VAL-023 | closed | Guide supports safe catalog/API metadata decisions. |

### WP-023: Add AI response contract

Objective: define the safe output shape for AI/tool frame suggestions.

Parent IDs: REQ-027, SPEC-026, IF-029.

Affected files/modules:

- `docs/theory/ai-response-contract.md`
- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/frame-theory.md`
- `docs/theory/application-pack-templates.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-027 | closed | AI response contract required. |
| Specification / Interface | SPEC-026, IF-029 | closed | Required fields, scoring semantics, gates, and language rules defined. |
| Implementation | `docs/theory/ai-response-contract.md` | closed | Contract, JSON shape, examples, and implementation implications added. |
| Verification | EVID-032 | closed | Inspection covers response contract. |
| Validation | VAL-024 | closed | Guide supports bounded AI/tool frame suggestions. |

### WP-024: Add metadata-backed accepted starter catalog

Objective: apply the first stable metadata fields to accepted starter catalog
entries without promoting local draft imports.

Parent IDs: REQ-028, SPEC-027, IF-030.

Affected files/modules:

- `docs/frame-catalog.md`
- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/theory-roadmap.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-028 | closed | Metadata-backed accepted starter catalog required. |
| Specification / Interface | SPEC-027, IF-030 | closed | Accepted metadata columns and accepted-only rule defined. |
| Implementation | `docs/frame-catalog.md` | closed | Accepted starter metadata table added. |
| Verification | EVID-033 | closed | Inspection covers metadata table and migration-plan update. |
| Validation | VAL-025 | closed | Catalog supports tool-facing metadata review before Rust migration. |

### WP-025: Add role-reviewed local import promotion

Objective: classify local imports as promoted draft heuristics or held patterns
before any accepted catalog promotion.

Parent IDs: REQ-029, SPEC-028, IF-031.

Affected files/modules:

- `docs/theory/local-import-promotion-review.md`
- `docs/theory/resonance-manage-frame-imports.md`
- `docs/theory/career-gravity-frame-imports.md`
- `docs/theory/theme-swimlane-extraction.md`
- `docs/theory/theory-roadmap.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-029 | closed | Local import promotion decisions required. |
| Specification / Interface | SPEC-028, IF-031 | closed | Promote/hold outcomes and criteria defined. |
| Implementation | `docs/theory/local-import-promotion-review.md` | closed | Six promoted draft heuristics and six held imports recorded. |
| Verification | EVID-034 | closed | Inspection covers promotion review and updated import statuses. |
| Validation | VAL-026 | closed | Guide prevents local drafts from silently becoming accepted catalog entries. |

### WP-026: Add theme-swimlane role review

Objective: review theme swimlanes through FRAMES role lenses before leadership
pack or catalog acceptance.

Parent IDs: REQ-030, SPEC-029, IF-032.

Affected files/modules:

- `docs/theory/theme-swimlane-role-review.md`
- `docs/theory/theme-swimlane-extraction.md`
- `docs/theory/local-import-promotion-review.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-030 | closed | Dedicated theme-swimlane role review required. |
| Specification / Interface | SPEC-029, IF-032 | closed | Review gates and output-template fields defined. |
| Implementation | `docs/theory/theme-swimlane-role-review.md` | closed | Acceptance decision, role findings, fit score, gates, hard stops, and template added. |
| Verification | EVID-035 | closed | Inspection covers theme-swimlane role review. |
| Validation | VAL-027 | closed | Guide supports pilot-use decisions before catalog acceptance. |

### WP-027: Add first empirical validation trial protocol

Objective: define a concrete comparison protocol for the first FRAMES empirical
validation trial without upgrading claim strength before data exists.

Parent IDs: REQ-031, SPEC-030, IF-033.

Affected files/modules:

- `docs/theory/empirical-validation-trial-001-theme-swimlanes.md`
- `docs/theory/empirical-validation-plan.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-031 | closed | First empirical validation trial protocol required. |
| Specification / Interface | SPEC-030, IF-033 | closed | Protocol fields and scoring stability gate defined. |
| Implementation | `docs/theory/empirical-validation-trial-001-theme-swimlanes.md` | closed | Scenario, comparison, response form, scoring, analysis, hard stops, and report template added. |
| Verification | EVID-036 | closed | Inspection covers first empirical trial protocol. |
| Validation | VAL-028 | closed | Protocol supports future data collection without overclaiming. |

### WP-028: Add transfer-aware search design

Objective: define how future search should rank structural transfer fit before
surface wording, vividness, or source-scene familiarity.

Parent IDs: REQ-032, SPEC-031, IF-034.

Affected files/modules:

- `docs/theory/transfer-aware-search-design.md`
- `docs/theory/relational-transfer-fields.md`
- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-032 | closed | Transfer-aware search design required. |
| Specification / Interface | SPEC-031, IF-034 | closed | Scoring order, hard stops, and metadata gates defined. |
| Implementation | `docs/theory/transfer-aware-search-design.md` | closed | Query inputs, entry metadata, scoring, hard stops, output notes, and migration path added. |
| Verification | EVID-037 | closed | Inspection covers transfer-aware search design. |
| Validation | VAL-029 | closed | Design supports future Rust ranking without premature API churn. |

### WP-029: Add theory gap audit

Objective: identify what theory FRAMES still lacks after the foundation work,
before scaling catalog growth or AI selection behavior.

Parent IDs: REQ-033, SPEC-032, IF-035.

Affected files/modules:

- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-033 | closed | Current theory gap audit required. |
| Specification / Interface | SPEC-032, IF-035 | closed | Gap categories and roadmap priority stability defined. |
| Implementation | `docs/theory/theory-gap-audit.md` | closed | Strengths, blocking gaps, growth gaps, implementation gaps, deep questions, sequence, and non-goals added. |
| Verification | EVID-038 | closed | Inspection covers theory gap audit. |
| Validation | VAL-030 | closed | Audit supports roadmap planning before catalog or AI-selection scale. |

### WP-030: Add AI response contract Rust example

Objective: show how current `FrameIndex` results can be returned in a bounded
AI response contract shape without adding metadata fields prematurely.

Parent IDs: REQ-034, SPEC-033, IF-036.

Affected files/modules:

- `examples/ai_response_contract.rs`
- `docs/theory/ai-response-contract.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-034 | closed | Runnable AI response contract example required. |
| Specification / Interface | SPEC-033, IF-036 | closed | Output fields and retrieval-vs-fit caveat defined. |
| Implementation | `examples/ai_response_contract.rs` | closed | Example prints bounded recommendation, fallback, hard stops, alternates, and notes. |
| Verification | EVID-039 | closed | Example run and inspection cover contract shape. |
| Validation | VAL-031 | closed | Tool builders can inspect contract-shaped output using current API. |

### WP-031: Add accepted starter Rust metadata migration

Objective: expose compact metadata for accepted starter frames in `frames-core`
without adding draft frames or transfer-aware ranking.

Parent IDs: REQ-035, SPEC-034, IF-037.

Affected files/modules:

- `src/lib.rs`
- `examples/ai_response_contract.rs`
- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/ai-response-contract.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-035 | closed | Accepted starter Rust metadata required. |
| Specification / Interface | SPEC-034, IF-037 | closed | Metadata fields and helper filters defined. |
| Implementation | `src/lib.rs`, `examples/ai_response_contract.rs` | closed | Status, claim strength, risk band, application packs, and filters added. |
| Verification | EVID-040 | closed | Tests and example cover metadata presence and display. |
| Validation | VAL-032 | closed | Tool builders can filter and display accepted starter metadata. |

### WP-032: Add accepted-catalog review process

Objective: define the repeatable gate for moving draft heuristics into accepted
catalog entries or default Rust search.

Parent IDs: REQ-036, SPEC-035, IF-038.

Affected files/modules:

- `docs/theory/accepted-catalog-review-process.md`
- `docs/theory/frame-lifecycle.md`
- `docs/theory/local-import-promotion-review.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-036 | closed | Accepted-catalog review process required. |
| Specification / Interface | SPEC-035, IF-038 | closed | Acceptance gates, hard stops, and metadata requirements defined. |
| Implementation | `docs/theory/accepted-catalog-review-process.md` | closed | Inputs, role lenses, decision bands, procedure, template, and promotion rule added. |
| Verification | EVID-041 | closed | Inspection covers accepted-catalog review process. |
| Validation | VAL-033 | closed | Process supports draft-to-accepted catalog decisions. |

### WP-033: Add frame anti-pattern taxonomy

Objective: define reusable failure classes for bad frames so rejection,
evaluation fixtures, and future rejected-candidate reporting are consistent.

Parent IDs: REQ-037, SPEC-036, IF-039.

Affected files/modules:

- `docs/theory/frame-antipattern-taxonomy.md`
- `docs/theory/accepted-catalog-review-process.md`
- `docs/theory/frame-theory.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-037 | closed | Frame anti-pattern taxonomy required. |
| Specification / Interface | SPEC-036, IF-039 | closed | Detection rule, anti-pattern classes, dispositions, and fixture fields defined. |
| Implementation | `docs/theory/frame-antipattern-taxonomy.md` | closed | Taxonomy, review procedure, examples, and design consequences added. |
| Verification | EVID-042 | closed | Inspection covers anti-pattern taxonomy. |
| Validation | VAL-034 | closed | Taxonomy supports revise/hold/reject decisions and safer fallbacks. |

### WP-034: Add related-frame relation taxonomy

Objective: define stable relation types for related frames so alternates,
fallbacks, conflicts, sequences, and rejected near-misses can be reviewed before
typed Rust metadata exists.

Parent IDs: REQ-038, SPEC-037, IF-040.

Affected files/modules:

- `docs/theory/related-frame-taxonomy.md`
- `docs/theory/composition-and-conflict.md`
- `docs/theory/transfer-aware-search-design.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-038 | closed | Related-frame relation taxonomy required. |
| Specification / Interface | SPEC-037, IF-040 | closed | Relation types, direction rules, display rules, and tool implications defined. |
| Implementation | `docs/theory/related-frame-taxonomy.md` | closed | Taxonomy, examples, catalog field shape, and design consequences added. |
| Verification | EVID-043 | closed | Inspection covers related-frame taxonomy. |
| Validation | VAL-035 | closed | Taxonomy supports alternate, fallback, sequence, conflict, and suppression decisions. |

### WP-035: Add evaluation-set design

Objective: define fixture structure and review rules for testing selection,
suppression, warnings, fallbacks, and related-frame behavior before semantic
search or broad catalog expansion.

Parent IDs: REQ-039, SPEC-038, IF-041.

Affected files/modules:

- `docs/theory/evaluation-set-design.md`
- `docs/theory/accepted-catalog-review-process.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-039 | closed | Evaluation-set design required. |
| Specification / Interface | SPEC-038, IF-041 | closed | Fixture types, fields, pass bands, and expected decisions defined. |
| Implementation | `docs/theory/evaluation-set-design.md` | closed | Evaluation jobs, starter backlog, scoring, review procedure, and non-goals added. |
| Verification | EVID-044 | closed | Inspection covers evaluation-set design. |
| Validation | VAL-036 | closed | Design supports future selection, suppression, warning, fallback, and relation-behavior fixtures. |

### WP-036: Add cultural portability guidance

Objective: define how everyday source frames travel across region, language,
mobility, accessibility, authority, professional culture, and institutional
trust differences.

Parent IDs: REQ-040, SPEC-039, IF-042.

Affected files/modules:

- `docs/theory/cultural-portability.md`
- `docs/theory/audience-transfer.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-040 | closed | Cultural portability guidance required. |
| Specification / Interface | SPEC-039, IF-042 | closed | Portability bands, dimensions, fields, and fallback obligations defined. |
| Implementation | `docs/theory/cultural-portability.md` | closed | Rule, dimensions, bands, source guidance, examples, and implications added. |
| Verification | EVID-045 | closed | Inspection covers cultural portability guide. |
| Validation | VAL-037 | closed | Guide supports broad, bounded, limited, unknown, and unsafe portability decisions. |

### WP-037: Add theme-swimlane leadership worksheet

Objective: provide a leadership-pack worksheet for Theme Swimlanes pilot use
without promoting the frame into accepted catalog or default search.

Parent IDs: REQ-041, SPEC-040, IF-043.

Affected files/modules:

- `docs/theory/theme-swimlane-leadership-worksheet.md`
- `docs/theory/theme-swimlane-extraction.md`
- `docs/theory/theme-swimlane-role-review.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-041 | closed | Theme-swimlane leadership worksheet required. |
| Specification / Interface | SPEC-040, IF-043 | closed | Promise, lane, mapping, decision, risk, role-gate, and closeout fields defined. |
| Implementation | `docs/theory/theme-swimlane-leadership-worksheet.md` | closed | Worksheet, worked example, risk review, and pilot closeout added. |
| Verification | EVID-046 | closed | Inspection covers worksheet. |
| Validation | VAL-038 | closed | Worksheet supports leadership pilot use without accepted-catalog promotion. |

### WP-038: Add transfer-aware search filters

Objective: add the first strict transfer-aware Rust filters without changing
the existing deterministic lexical scoring.

Parent IDs: REQ-042, SPEC-041, IF-044.

Affected files/modules:

- `src/lib.rs`
- `examples/lookup.rs`
- `examples/ai_response_contract.rs`
- `docs/theory/transfer-aware-search-design.md`
- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/ai-response-contract.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-042 | closed | Strict transfer-aware filters required. |
| Specification / Interface | SPEC-041, IF-044 | closed | Authority model and query filter APIs defined. |
| Implementation | `src/lib.rs`, `examples/*` | closed | Filters gate authority, risk, and application pack before lexical scoring. |
| Verification | EVID-047 | closed | Tests and examples cover filter behavior and display. |
| Validation | VAL-039 | closed | Tool builders can avoid obvious authority, risk, and context mismatches. |

### WP-039: Add EVT-001 execution packet

Objective: prepare EVT-001 for participant collection with locked prompts,
condition assignment, scoring, and report shape.

Parent IDs: REQ-043, SPEC-042, IF-045.

Affected files/modules:

- `docs/validation/evt-001-theme-swimlanes-runbook.md`
- `docs/validation/evt-001-response-and-scoring-sheet.md`
- `docs/validation/evt-001-theme-swimlanes-results.md`
- `docs/theory/empirical-validation-trial-001-theme-swimlanes.md`
- `docs/theory/empirical-validation-plan.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-043 | closed | Locked EVT-001 execution packet required before collection. |
| Specification / Interface | SPEC-042, IF-045 | closed | Prompts, assignment, scoring, and report shape are locked. |
| Implementation | `docs/validation/*` | closed | Runbook, response sheet, and empty results ledger added. |
| Verification | EVID-048 | closed | Inspection covers execution packet and no-response status. |
| Validation | VAL-040 | closed | Maintainers can collect responses without post-hoc scoring changes. |

### WP-040: Add frame ontology

Objective: define controlled vocabulary for frame jobs, relations, authority,
risk, and tag families before further catalog growth.

Parent IDs: REQ-044, SPEC-043, IF-046.

Affected files/modules:

- `docs/theory/frame-ontology.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-044 | closed | Controlled frame ontology required before catalog growth. |
| Specification / Interface | SPEC-043, IF-046 | closed | Controlled terms and admission rules defined. |
| Implementation | `docs/theory/frame-ontology.md` | closed | Jobs, relations, authority, risk, tag rules, and checklist added. |
| Verification | EVID-049 | closed | Inspection covers ontology artifact. |
| Validation | VAL-041 | closed | Catalog reviewers can prevent term drift. |

### WP-041: Apply ontology to catalog and fixtures

Objective: make the controlled ontology visible in accepted starter metadata and
evaluation fixture planning.

Parent IDs: REQ-045, SPEC-044, IF-047.

Affected files/modules:

- `docs/frame-catalog.md`
- `docs/theory/evaluation-set-design.md`
- `docs/theory/frame-ontology.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-045 | closed | Ontology terms required in catalog and fixture planning. |
| Specification / Interface | SPEC-044, IF-047 | closed | Catalog columns and fixture fields use controlled terms. |
| Implementation | `docs/frame-catalog.md`, `docs/theory/evaluation-set-design.md` | closed | Authority, relation, and job terms applied. |
| Verification | EVID-050 | closed | Inspection covers catalog and fixture ontology usage. |
| Validation | VAL-042 | closed | Reviewers can use ontology terms in catalog and fixture review. |

### WP-042: Apply accepted-catalog review to Veto Rule

Objective: record the first accepted-catalog review decision for a promoted
local heuristic without promoting it into default search.

Parent IDs: REQ-046, SPEC-045, IF-048.

Affected files/modules:

- `docs/theory/accepted-catalog-review-veto-rule.md`
- `docs/theory/accepted-catalog-review-process.md`
- `docs/theory/local-import-promotion-review.md`
- `docs/theory/frame-ontology.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-046 | closed | First promoted local candidate review required. |
| Specification / Interface | SPEC-045, IF-048 | closed | Review decision is distinct from catalog/index promotion. |
| Implementation | `docs/theory/accepted-catalog-review-veto-rule.md` | closed | Fit score, role findings, revise decision, and revision items recorded. |
| Verification | EVID-051 | closed | Inspection covers Veto Rule review record. |
| Validation | VAL-043 | closed | Maintainers can continue from revision items before acceptance. |

### WP-043: Close Veto Rule revision items

Objective: close the revision items from the Veto Rule accepted-catalog review
without adding it to default Rust search.

Parent IDs: REQ-047, SPEC-046, IF-049.

Affected files/modules:

- `docs/theory/accepted-catalog-review-veto-rule.md`
- `docs/theory/evaluation-set-design.md`
- `docs/theory/accepted-catalog-review-process.md`
- `docs/theory/local-import-promotion-review.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-047 | closed | Veto Rule revision items required closure before promotion. |
| Specification / Interface | SPEC-046, IF-049 | closed | Caveat and no-default-search boundary defined. |
| Implementation | `docs/theory/accepted-catalog-review-veto-rule.md`, `docs/theory/evaluation-set-design.md` | closed | Fixtures, stop conditions, fallback, and caveat status added. |
| Verification | EVID-052 | closed | Inspection covers revision closure. |
| Validation | VAL-044 | closed | Maintainers can add a docs catalog row later without changing default search. |

### WP-044: Add Veto Rule docs-catalog row

Objective: add Veto Rule to the docs catalog as a reviewed candidate while
keeping it out of accepted starter metadata and default Rust search.

Parent IDs: REQ-048, SPEC-047, IF-050.

Affected files/modules:

- `docs/frame-catalog.md`
- `docs/theory/accepted-catalog-review-veto-rule.md`
- `docs/theory/catalog-metadata-migration-plan.md`
- `docs/theory/accepted-catalog-review-process.md`
- `docs/theory/local-import-promotion-review.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-048 | closed | Reviewed docs-catalog row required separation from starter/default search. |
| Specification / Interface | SPEC-047, IF-050 | closed | Reviewed candidates section and no-default-search boundary defined. |
| Implementation | `docs/frame-catalog.md` | closed | Veto Rule row added under reviewed docs-catalog candidates. |
| Verification | EVID-053 | closed | Inspection covers catalog separation. |
| Validation | VAL-045 | closed | Reviewers can find Veto Rule without changing Rust index behavior. |

### WP-045: Apply anti-pattern taxonomy to Veto Rule

Objective: classify Veto Rule misuse risks and add a rejected-use fixture before
broader catalog or tool behavior depends on it.

Parent IDs: REQ-049, SPEC-048, IF-051.

Affected files/modules:

- `docs/theory/frame-antipattern-application-veto-rule.md`
- `docs/theory/frame-antipattern-taxonomy.md`
- `docs/theory/evaluation-set-design.md`
- `docs/theory/accepted-catalog-review-veto-rule.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-049 | closed | Anti-pattern application required for first docs-catalog candidate. |
| Specification / Interface | SPEC-048, IF-051 | closed | Misuse classes, rejected scenarios, and fixture link defined. |
| Implementation | `docs/theory/frame-antipattern-application-veto-rule.md` | closed | Veto Rule anti-pattern review and fixture linkage added. |
| Verification | EVID-054 | closed | Inspection covers applied anti-pattern review. |
| Validation | VAL-046 | closed | Misuse reviewers can suppress unsupported Veto Rule use. |

### WP-046: Apply related-frame taxonomy to starter links

Objective: classify representative starter related-frame IDs and add relation
behavior fixture expectations before typed relation metadata is added to Rust.

Parent IDs: REQ-050, SPEC-049, IF-052.

Affected files/modules:

- `docs/theory/related-frame-application-starter.md`
- `docs/theory/related-frame-taxonomy.md`
- `docs/theory/evaluation-set-design.md`
- `docs/frame-catalog.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-050 | closed | Related taxonomy application required before typed Rust relation behavior. |
| Specification / Interface | SPEC-049, IF-052 | closed | Relation type, display rule, rationale, fixture implication, and API boundary defined. |
| Implementation | `docs/theory/related-frame-application-starter.md` | closed | Starter relation map and rejected near-misses added. |
| Verification | EVID-055 | closed | Inspection covers applied relation map and fixtures. |
| Validation | VAL-047 | closed | Maintainers can interpret related IDs without changing `FrameIndex::related_to`. |

### WP-047: Publish starter evaluation fixture package

Objective: convert the starter fixture backlog into a parseable docs-level JSON
package before adding semantic search, typed relation behavior, rejected
candidate reporting, or broad draft-frame inclusion.

Parent IDs: REQ-051, SPEC-050, IF-053.

Affected files/modules:

- `docs/eval/README.md`
- `docs/eval/starter-fixtures.json`
- `docs/theory/evaluation-set-design.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
Get-Content docs\eval\starter-fixtures.json -Raw | ConvertFrom-Json | Out-Null
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-051 | closed | Machine-readable starter fixture package required before tool behavior gates. |
| Specification / Interface | SPEC-050, IF-053 | closed | Fixture fields, parseability, source docs, and docs-level boundary defined. |
| Implementation | `docs/eval/starter-fixtures.json` | closed | Traffic, risk, veto, anti-pattern, story, relation, theme, and audience fixtures added. |
| Verification | EVID-056 | closed | JSON parse and inspection cover fixture package. |
| Validation | VAL-048 | closed | AI tool builders can use fixtures as a future behavior gate. |

### WP-048: Apply portability bands to starter fixtures

Objective: add cultural portability bands and safer fallbacks to the starter
fixture package before any portability-aware Rust API, search filter, or ranking
behavior is added.

Parent IDs: REQ-052, SPEC-051, IF-054.

Affected files/modules:

- `docs/eval/starter-fixtures.json`
- `docs/eval/README.md`
- `docs/theory/cultural-portability-application-fixtures.md`
- `docs/theory/cultural-portability.md`
- `docs/theory/evaluation-set-design.md`
- `docs/theory/theory-gap-audit.md`
- `docs/theory/theory-roadmap.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
Get-Content docs\eval\starter-fixtures.json -Raw | ConvertFrom-Json | Out-Null
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-052 | closed | Portability application required before portability-aware tool behavior. |
| Specification / Interface | SPEC-051, IF-054 | closed | Fixture portability profile fields and no-Rust-API boundary defined. |
| Implementation | `docs/eval/starter-fixtures.json` | closed | Portability profiles added for all starter fixtures. |
| Verification | EVID-057 | closed | JSON parse and inspection cover portability profiles. |
| Validation | VAL-049 | closed | Audience-transfer reviewers can identify bounded, limited, unknown, and unsafe cases. |

### WP-049: Add theme-swimlane leadership pilot ledger

Objective: create the ledger that will record real Theme Swimlane worksheet
pilots and changed decisions without treating dry runs as evidence.

Parent IDs: REQ-053, SPEC-052, IF-055.

Affected files/modules:

- `docs/validation/theme-swimlane-leadership-pilot-ledger.md`
- `docs/theory/theme-swimlane-leadership-worksheet.md`
- `docs/theory/theme-swimlane-role-review.md`
- `docs/theory/theme-swimlane-extraction.md`
- `README.md`
- `docs/vtrace/*`

Verification commands:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
cargo run --manifest-path ..\..\standards-protocols\vtrace\Cargo.toml -- validate .
```

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Requirements | REQ-053 | closed | Pilot ledger required before counting worksheet pilots as evidence. |
| Specification / Interface | SPEC-052, IF-055 | closed | Record template, dry-run boundary, closeout rule, and promotion guard defined. |
| Implementation | `docs/validation/theme-swimlane-leadership-pilot-ledger.md` | closed | Empty real-pilot ledger and local dry-run example added. |
| Verification | EVID-058 | closed | Inspection covers ledger and linked theme-swimlane docs. |
| Validation | VAL-050 | closed | Leaders can prepare a real pilot without overstating evidence. |
