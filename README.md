# FRAMES

FRAMES is a methodology repo for turning familiar human situations into useful
decision frames.

The repo studies everyday metaphors, analogies, and shared scripts people
already understand, then turns them into reusable ways to explain status,
coordination, risk, momentum, and goal tradeoffs. Red/yellow/green progress
tracking is the seed example: it works because most people already understand
stop, caution, and go without needing a new vocabulary.

## Thesis

Good frames reduce explanation cost. A useful frame makes a situation easier to
see, discuss, and act on without pretending the analogy is the actual system.

FRAMES starts with ordinary human activities:

- Driving: signals, intersections, lanes, speed, yielding, detours, merging.
- Walking: pace, stride, footing, crosswalks, fatigue, crowds, wayfinding.
- Waiting: queues, turns, holds, appointments, service windows.
- Cooking: prep, timing, heat, seasoning, batches, mise en place.
- Building: foundations, scaffolds, load, fit, inspection, punch lists.
- Sports and games: field position, timeouts, fouls, advantage, reset.

## Rust Frame Index

FRAMES also includes `frames-core`, a small Rust library for AI tools and other
software that need a structured frame index. It provides a starter catalog,
query types, ranked candidates, related-frame lookup, action cues, and failure
mode warnings. Accepted starter entries also expose compact metadata for status,
claim strength, authority model, risk band, application packs, and first
transfer-aware query filters. Lifecycle report APIs add explicit visibility
filters, fallbacks, and suppressed-candidate explanations while keeping default
search accepted-starter only. Review-only catalog helpers and review modes
expose docs-catalog, held, and anti-pattern rows separately from accepted
search. Private relation metadata tables now stage relation-aware ranking
without changing default search.

```powershell
cargo test
```

Example lookup:

```rust
use frames_core::{ApplicationPack, AuthorityModel, FrameIndex, FrameKind, FrameQuery};

let index = FrameIndex::new();
let query = FrameQuery::new("two teams need turn order around constrained attention")
    .with_kind(FrameKind::Coordination)
    .with_authority_model(AuthorityModel::Peer)
    .with_application_pack(ApplicationPack::Product)
    .with_tags(&["priority"]);

let candidates = index.search_top(&query, 3);
```

Lifecycle-aware lookup:

```rust
use frames_core::{ApplicationPack, AuthorityModel, FrameIndex, FrameQuery, LifecycleFilter, RiskBand};

let index = FrameIndex::new();
let query = FrameQuery::new("another team is a roadblock")
    .with_authority_model(AuthorityModel::Owner)
    .with_risk_band(RiskBand::Medium)
    .with_application_pack(ApplicationPack::Operations);
let report = index.search_with_lifecycle(&query, &LifecycleFilter::explanation_mode());
```

Review-only catalog lookup:

```rust
use frames_core::{FrameIndex, FrameQuery, LifecycleFilter};

let index = FrameIndex::new();
let report = index.search_with_lifecycle(&FrameQuery::new(""), &LifecycleFilter::catalog_review());
let review_rows = report.review_only;
```

## First Catalog

The initial catalog lives in [docs/frame-catalog.md](docs/frame-catalog.md).
The first worked examples live in
[docs/examples/traffic-and-motion.md](docs/examples/traffic-and-motion.md).
The operating theory lives in [docs/theory/frame-theory.md](docs/theory/frame-theory.md).
The current theory gap audit lives in [docs/theory/theory-gap-audit.md](docs/theory/theory-gap-audit.md).
The first scoring rubric lives in [docs/theory/fit-rubric.md](docs/theory/fit-rubric.md).
Audience transfer guidance lives in [docs/theory/audience-transfer.md](docs/theory/audience-transfer.md).
Cultural portability guidance lives in [docs/theory/cultural-portability.md](docs/theory/cultural-portability.md).
The first applied portability review lives in [docs/theory/cultural-portability-application-fixtures.md](docs/theory/cultural-portability-application-fixtures.md).
Frame lifecycle guidance lives in [docs/theory/frame-lifecycle.md](docs/theory/frame-lifecycle.md).
Lifecycle filtering and rejection reporting guidance lives in [docs/theory/lifecycle-filtering-and-rejection-reporting.md](docs/theory/lifecycle-filtering-and-rejection-reporting.md).
Controlled ontology guidance lives in [docs/theory/frame-ontology.md](docs/theory/frame-ontology.md).
Accepted-catalog review process lives in [docs/theory/accepted-catalog-review-process.md](docs/theory/accepted-catalog-review-process.md).
The first applied accepted-catalog review lives in [docs/theory/accepted-catalog-review-veto-rule.md](docs/theory/accepted-catalog-review-veto-rule.md).
Frame anti-pattern taxonomy lives in [docs/theory/frame-antipattern-taxonomy.md](docs/theory/frame-antipattern-taxonomy.md).
The first applied anti-pattern review lives in [docs/theory/frame-antipattern-application-veto-rule.md](docs/theory/frame-antipattern-application-veto-rule.md).
Composition and conflict guidance lives in [docs/theory/composition-and-conflict.md](docs/theory/composition-and-conflict.md).
Evidence boundary schema lives in [docs/theory/evidence-boundary-schema.md](docs/theory/evidence-boundary-schema.md).
Research grounding lives in [docs/theory/research-grounding.md](docs/theory/research-grounding.md).
Claim-strength labels live in [docs/theory/claim-strength-labels.md](docs/theory/claim-strength-labels.md).
Source-domain taxonomy lives in [docs/theory/source-domain-taxonomy.md](docs/theory/source-domain-taxonomy.md).
Relational transfer fields live in [docs/theory/relational-transfer-fields.md](docs/theory/relational-transfer-fields.md).
Transfer-aware search design lives in [docs/theory/transfer-aware-search-design.md](docs/theory/transfer-aware-search-design.md).
Related-frame relation taxonomy lives in [docs/theory/related-frame-taxonomy.md](docs/theory/related-frame-taxonomy.md).
The first applied related-frame map lives in [docs/theory/related-frame-application-starter.md](docs/theory/related-frame-application-starter.md).
Evaluation-set design lives in [docs/theory/evaluation-set-design.md](docs/theory/evaluation-set-design.md).
The first machine-readable evaluation package lives in [docs/eval/starter-fixtures.json](docs/eval/starter-fixtures.json).
Lifecycle and rejection-reporting fixtures live in [docs/eval/lifecycle-rejection-fixtures.json](docs/eval/lifecycle-rejection-fixtures.json).
Review-only catalog fixtures live in [docs/eval/review-only-catalog-fixtures.json](docs/eval/review-only-catalog-fixtures.json).
Relation-aware ranking fixtures live in [docs/eval/relation-aware-ranking-fixtures.json](docs/eval/relation-aware-ranking-fixtures.json).
Role-reviewed domain examples live in [docs/theory/role-reviewed-domain-examples.md](docs/theory/role-reviewed-domain-examples.md).
Application pack templates live in [docs/theory/application-pack-templates.md](docs/theory/application-pack-templates.md).
Domain pack roadmap lives in [docs/theory/domain-pack-roadmap.md](docs/theory/domain-pack-roadmap.md).
Plain-language fallback theory lives in [docs/theory/plain-language-fallbacks.md](docs/theory/plain-language-fallbacks.md).
Learning progression lives in [docs/theory/learning-progression.md](docs/theory/learning-progression.md).
Perspective metadata guidance lives in [docs/theory/perspective-metadata.md](docs/theory/perspective-metadata.md).
Story-job taxonomy lives in [docs/theory/story-job-taxonomy.md](docs/theory/story-job-taxonomy.md).
The local RESONANCE MANAGE import map lives in [docs/theory/resonance-manage-import-map.md](docs/theory/resonance-manage-import-map.md).
Structured RESONANCE MANAGE frame imports live in [docs/theory/resonance-manage-frame-imports.md](docs/theory/resonance-manage-frame-imports.md).
The local CAREER Gravity import map lives in [docs/theory/career-gravity-import-map.md](docs/theory/career-gravity-import-map.md).
Structured CAREER Gravity frame imports live in [docs/theory/career-gravity-frame-imports.md](docs/theory/career-gravity-frame-imports.md).
Theme swimlane extraction lives in [docs/theory/theme-swimlane-extraction.md](docs/theory/theme-swimlane-extraction.md).
Theme swimlane role review lives in [docs/theory/theme-swimlane-role-review.md](docs/theory/theme-swimlane-role-review.md).
Theme swimlane leadership worksheet lives in [docs/theory/theme-swimlane-leadership-worksheet.md](docs/theory/theme-swimlane-leadership-worksheet.md).
Theme swimlane leadership pilot ledger lives in [docs/validation/theme-swimlane-leadership-pilot-ledger.md](docs/validation/theme-swimlane-leadership-pilot-ledger.md).
Empirical validation planning lives in [docs/theory/empirical-validation-plan.md](docs/theory/empirical-validation-plan.md).
The first empirical validation trial lives in [docs/theory/empirical-validation-trial-001-theme-swimlanes.md](docs/theory/empirical-validation-trial-001-theme-swimlanes.md).
Its execution packet and empty results ledger live in [docs/validation](docs/validation).
Catalog metadata migration planning lives in [docs/theory/catalog-metadata-migration-plan.md](docs/theory/catalog-metadata-migration-plan.md).
AI response contract guidance lives in [docs/theory/ai-response-contract.md](docs/theory/ai-response-contract.md).
Rust lifecycle filter API design lives in [docs/theory/rust-lifecycle-filter-api-design.md](docs/theory/rust-lifecycle-filter-api-design.md).
Review-only catalog data model lives in [docs/theory/review-only-catalog-data-model.md](docs/theory/review-only-catalog-data-model.md).
Rust relation-aware ranking design lives in [docs/theory/rust-relation-aware-ranking-design.md](docs/theory/rust-relation-aware-ranking-design.md).
External frame-practitioner benchmarks live in [docs/theory/external-frame-practitioners.md](docs/theory/external-frame-practitioners.md).
Frame acquisition method lives in [docs/theory/frame-acquisition-method.md](docs/theory/frame-acquisition-method.md).
Project review roles live in [.roles/ROLE.md](.roles/ROLE.md).

## Frame Shape

Each frame should name:

| Field | Meaning |
|---|---|
| Everyday source | The familiar situation people already understand. |
| Target situation | The work, goal, risk, or coordination problem being explained. |
| Useful mapping | Which parts of the source transfer well. |
| Action cue | What the frame helps people do next. |
| Failure mode | Where the analogy breaks or misleads. |
| Better fit than | The frames it should replace for this use case. |

## Non-goals

- FRAMES is not a persuasion dark-pattern library.
- FRAMES is not a generic quote or metaphor collection.
- FRAMES does not claim every analogy is culturally universal.
- FRAMES does not replace domain evidence, metrics, or judgment.

## Validation

Current validation:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
cargo run --example ai_response_contract
git diff --check
```
