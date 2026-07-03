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
