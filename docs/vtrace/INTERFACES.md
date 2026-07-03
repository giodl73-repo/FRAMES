# Interfaces

## Scope

Repo or feature: `frames-core`

## Rust API

| ID | Surface | Purpose | Stability |
|---|---|---|---|
| IF-001 | `FrameEntry` | Structured frame metadata. | target |
| IF-002 | `FrameKind` | Status, coordination, momentum, and risk categories. | target |
| IF-003 | `FrameQuery` | Situation text, optional kind, authority model, risk band, application pack, and tags. | target |
| IF-004 | `FrameCandidate` | Ranked search result and match notes. | target |
| IF-005 | `FrameIndex::search` | Deterministic candidate lookup. | target |
| IF-006 | `FrameIndex::related_to` | Related-frame lookup by stable ID. | target |
| IF-007 | `FrameQuery::new`, `with_kind`, `with_authority_model`, `with_risk_band`, `with_application_pack`, `with_tags` | Ergonomic query construction. | target |
| IF-008 | `FrameIndex::search_top`, `by_kind`, `with_tag`, metadata helper filters | Common lookup and filtering helpers. | target |
| IF-009 | Traffic frame IDs | Stable IDs for accepted traffic-pack frames. | target |
| IF-010 | `FrameStatus`, `ClaimStrength`, `RiskBand`, `AuthorityModel`, `ApplicationPack` | Compact display and filtering metadata. | target |
| IF-011 | `LifecycleFilter`, `VisibilityMode`, `ResultClass`, `DisplayRule` | Lifecycle visibility and output classification. | target |
| IF-012 | `FrameIndex::search_with_lifecycle`, `FrameSearchReport`, `SuppressedCandidate` | Lifecycle-aware report lookup with separate suggestions, fallbacks, and suppressed candidates. | target |
| IF-013 | `ReviewFamily`, `ReviewFrameEntry`, `REVIEW_CATALOG`, `FrameIndex::review_entries`, `review_entry`, `review_by_family` | Review-only catalog rows and lookup helpers separate from accepted starter search. | target |
| IF-014 | `ReviewCandidate`, `FrameSearchReport::review_only`, `LifecycleFilter::catalog_review`, `anti_pattern_review`, `docs_catalog_preview` | Explicit review-mode output for review-only rows. | target |

## Non-Interfaces

- VTRACE work package IDs are not public API.
- `.roles` review lenses are not library API.
- Natural-language generation prompts are out of scope for this slice.
