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
