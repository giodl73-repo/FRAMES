# Theory Roadmap

This roadmap names the theory FRAMES still needs before it becomes a reliable
frame index rather than a growing metaphor list.

## Current Baseline

`frame-theory.md` defines the foundation:

- core frame parts,
- frame jobs,
- fit tests,
- audience levels,
- evidence boundaries,
- research-grounding boundaries,
- misuse patterns,
- selection procedure,
- design consequences for `frames-core`.

The current theory gap audit lives in
[theory-gap-audit.md](theory-gap-audit.md). It now separates blocking gaps,
growth gaps, implementation gaps, and deeper theory questions.

## Needed Theory

| Area | Why It Matters | First Artifact |
|---|---|---|
| Fit scoring | Catalog entries need comparable readiness, not just prose review. | `fit-rubric.md` |
| Theory gap audit | The project needs a current account of what is still missing after the baseline theory is in place. | `theory-gap-audit.md` |
| Audience transfer | Familiarity differs by role, region, culture, expertise, and mobility. | `audience-transfer.md` |
| Cultural portability | Everyday sources need explicit region, language, mobility, accessibility, and authority checks. | `cultural-portability.md` |
| Frame lifecycle | Frames should move from candidate to accepted, revised, deprecated, or rejected. | `frame-lifecycle.md` |
| Frame ontology | Tags, jobs, relations, authority terms, and risk terms need controlled vocabulary before catalog growth. | `frame-ontology.md` |
| Accepted-catalog review | Draft heuristics need a repeatable gate before accepted catalog or default search. | `accepted-catalog-review-process.md` |
| Frame anti-pattern taxonomy | Bad frames need reusable rejection classes before rejected examples and evaluation sets expand. | `frame-antipattern-taxonomy.md` |
| Composition | Some situations need two frames; some frames conflict. | `composition-and-conflict.md` |
| Evidence schema | `frames-core` stores the evidence boundary that keeps frames from replacing proof. | `evidence-boundary-schema.md` |
| Claim-strength labels | Public claims need to distinguish illustration, heuristic, theory support, local observation, role review, and validation. | `claim-strength-labels.md` |
| Source-domain taxonomy | Everyday domains need controlled names and risks. | `source-domain-taxonomy.md` |
| Relational transfer fields | Frames need explicit source-target relation maps, exclusions, and transfer strength. | `relational-transfer-fields.md` |
| Transfer-aware search design | Search needs to rank structural fit above surface wording before catalog growth. | `transfer-aware-search-design.md` |
| Related-frame relation taxonomy | Related links need typed meanings such as alternate, safer fallback, conflict, sequence, and rejected near-miss. | `related-frame-taxonomy.md` |
| Evaluation-set design | Search and AI outputs need curated fixtures before semantic search or broad catalog growth. | `evaluation-set-design.md` |
| Role-reviewed domain examples | Theory needs concrete role-reviewed examples before broader catalog expansion. | `role-reviewed-domain-examples.md` |
| Application packs | Product, operations, leadership, learning, and AI-agent packs need different defaults. | `application-pack-templates.md` |
| Domain pack strategy | Application packs need expansion order, coverage goals, and hold conditions. | `domain-pack-roadmap.md` |
| Plain-language fallback theory | High-stakes, low-transfer, or unsafe frame uses need direct non-metaphor alternatives. | `plain-language-fallbacks.md` |
| Learning progression | Novice, journeyman, expert, and AI-tool users need staged learning paths. | `learning-progression.md` |
| Perspective metadata | Frames assign listener roles, agency, duty, and authority assumptions. | `perspective-metadata.md` |
| Story-job taxonomy | Frames need to declare what their narrative layer is doing for the audience. | `story-job-taxonomy.md` |
| Internal source import | Local RESONANCE MANAGE patterns need structured import before catalog expansion. | `resonance-manage-import-map.md` |
| RESONANCE structured imports | High-value MANAGE candidates need FRAMES-native draft entries before catalog acceptance. | `resonance-manage-frame-imports.md` |
| Career source import | Local CAREER Gravity patterns need structured import before catalog expansion. | `career-gravity-import-map.md` |
| CAREER structured imports | High-value Gravity candidates need FRAMES-native draft entries before catalog acceptance. | `career-gravity-frame-imports.md` |
| External practitioners | Successful public frame/story writers should be benchmarked for reusable design lessons. | `external-frame-practitioners.md` |
| Research grounding | Public claims about cognition, metaphor, or pedagogy need citations and claim-strength limits. | `research-grounding.md` |
| Theme swimlanes | Local program patterns such as `Run One`, `Run Lean`, `Run Fast`, `Run Safe`, and three-lane promises need structured extraction. | `theme-swimlane-extraction.md` |
| Theme-swimlane role review | Promoted theme swimlanes need role gates before leadership-pack or catalog acceptance. | `theme-swimlane-role-review.md` |
| Theme-swimlane leadership worksheet | Leadership-pack use needs a practical worksheet before pilot or catalog acceptance. | `theme-swimlane-leadership-worksheet.md` |
| Empirical validation | Claim upgrades need task-specific tests with comparison conditions and narrow scope. | `empirical-validation-plan.md` |
| First empirical validation trial | The validation plan needs a runnable comparison protocol before any empirical claim upgrade. | `empirical-validation-trial-001-theme-swimlanes.md` |
| Catalog metadata migration | Theory fields need a staged path into catalog rows and `frames-core` without premature API churn. | `catalog-metadata-migration-plan.md` |
| AI response contract | Tool callers need a safe output shape for suggestions, alternates, scores, warnings, and evidence boundaries. | `ai-response-contract.md` |
| Metadata-backed catalog | Accepted starter frames need docs-level status, claim strength, risk band, application pack, authority, and transfer metadata. | `../frame-catalog.md` |
| Local import promotion | Local imports need role-reviewed promotion decisions before catalog acceptance. | `local-import-promotion-review.md` |
| Frame acquisition method | New frame candidates need a repeatable intake, screening, and promotion path. | `frame-acquisition-method.md` |
| Lifecycle filtering and rejection reporting | Tools need explicit visibility modes, status filters, suppressed-candidate explanations, and fixtures before non-accepted frames enter tool behavior. | `lifecycle-filtering-and-rejection-reporting.md`, `../eval/lifecycle-rejection-fixtures.json` |
| Rust lifecycle filter API | The crate needs additive lifecycle report behavior before review-only rows can be loaded safely. | `rust-lifecycle-filter-api-design.md` |
| Review-only catalog data model | Docs-catalog, draft, held, deprecated, rejected, and anti-pattern rows need a separate data model before they enter the crate. | `review-only-catalog-data-model.md` |
| Review-only catalog fixtures | The review-only data model needs machine-readable rows before Rust loads them. | `../eval/review-only-catalog-fixtures.json` |
| Rust review-only catalog rows | The crate needs separate review rows before catalog review modes can list non-default entries safely. | `../src/lib.rs` |
| Catalog review mode output | Review-only rows need explicit listing behavior before broader non-default catalog use. | Start from `rust-lifecycle-filter-api-design.md` and `review-only-catalog-data-model.md` |
| Relation-aware ranking fixtures | Accepted and review-only search need structural ranking fixtures before scoring moves past metadata filters. | Extend `../eval/starter-fixtures.json` and review-only fixtures |
| Rust relation-aware ranking design | The crate needs an implementation plan for relation scoring before metadata fields or scoring changes are added. | Start from `transfer-aware-search-design.md` and `../eval/relation-aware-ranking-fixtures.json` |
| Private relation metadata tables | The crate needs fixture-mapped metadata before relation-aware output can be implemented safely. | Start from `rust-relation-aware-ranking-design.md` |
| Relation-aware report path | The crate needs relation-aware examples and broader fixture coverage before relation scoring can affect default search. | Start from `search_with_relations` and ranking fixtures |

## Priority Order

1. Collect EVT-001 participant responses and score the locked packet.
2. Pilot the theme-swimlane leadership worksheet and record changed decisions.
3. Expand relation-aware report examples and fixture coverage without changing default search.

## Role Guidance

| Role | Theory Pressure |
|---|---|
| Frame Fit Reviewer | Make every theory artifact improve selection quality. |
| Novice Reader | Keep terms learnable without repo history. |
| Journeyman Practitioner | Make artifacts usable in a normal meeting. |
| Business Leader | Preserve accountability, tradeoff, and decision value. |
| Evidence Boundary Reviewer | Keep frames from replacing proof. |
| Research Grounding Reviewer | Separate literature-informed guidance from validated claims. |
| Misuse Risk Reviewer | Reject people-as-obstacles and coercive framing. |
| Catalog Structure Reviewer | Keep theory reflected in catalog shape and `frames-core`. |
