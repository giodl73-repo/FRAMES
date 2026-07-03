# Rust Lifecycle Filter API Design

This design translates lifecycle filtering and rejected-candidate reporting into
the `frames-core` API shape. The first implementation is additive: default
search still returns accepted starter entries, while `search_with_lifecycle`
returns a report with suggestions, fallbacks, and opt-in suppressed-candidate
explanations.

## Design Rule

Default Rust search must remain accepted-starter only.

The first lifecycle API should make broader visibility explicit instead of
silently expanding `FrameIndex::search`.

## Current Boundary

`frames-core` currently has:

- `FrameStatus` values for accepted entries and suppressed/report-only cases;
- accepted starter entries only;
- `FrameQuery` filters for authority model, risk band, application pack, kind,
  and tags;
- `FrameCandidate` for suggested accepted entries;
- `LifecycleFilter`, `VisibilityMode`, `ResultClass`, `DisplayRule`,
  `SuppressedCandidate`, and `FrameSearchReport`;
- fixture-backed suppressed reports for selected rejected/docs-catalog cases.

This boundary is correct. The API adds expression power before catalog breadth.
Review-only rows still do not live in `STARTER_CATALOG`.

## Future Types

### Lifecycle Status

Keep `Accepted` as the only default-search status. Add more statuses only when
review-only rows can be loaded without becoming recommendations.

```rust
pub enum FrameStatus {
    Accepted,
    AcceptedWithCaveat,
    Candidate,
    Draft,
    Held,
    Deprecated,
    Rejected,
    AntiPattern,
}
```

Status names should serialize to the docs terms used by
`lifecycle-rejection-fixtures.json`.

### Visibility Mode

```rust
pub enum VisibilityMode {
    DefaultSearch,
    CatalogReview,
    AntiPatternReview,
    DocsCatalogPreview,
    ExplanationMode,
}
```

`DefaultSearch` should be the default and should allow only accepted entries.
Other modes should require explicit construction.

### Lifecycle Filter

```rust
pub struct LifecycleFilter<'a> {
    pub mode: VisibilityMode,
    pub allowed_statuses: &'a [FrameStatus],
    pub include_docs_catalog: bool,
    pub include_draft: bool,
    pub include_held: bool,
    pub include_rejected: bool,
    pub explain_suppressed: bool,
}
```

The safest constructor is:

```rust
impl<'a> LifecycleFilter<'a> {
    pub const fn default_search() -> Self;
    pub const fn explanation_mode() -> Self;
}
```

Avoid a constructor that accepts every status by default.

### Result Class

Search output should distinguish recommendation from explanation.

```rust
pub enum ResultClass {
    Suggested,
    Alternate,
    Fallback,
    Suppressed,
    ReviewOnly,
}
```

Only `Suggested`, `Alternate`, and `Fallback` are recommendation classes.

### Suppressed Candidate Report

```rust
pub struct SuppressedCandidate<'a> {
    pub candidate_id: &'a str,
    pub candidate_name: &'a str,
    pub status: FrameStatus,
    pub matched_reason: &'a str,
    pub rejection_class: &'a str,
    pub violated_boundary: &'a str,
    pub safer_frame: Option<&'a str>,
    pub plain_language_fallback: &'a str,
    pub source_docs: &'a [&'a str],
    pub display_rule: DisplayRule,
}
```

Use strings for `rejection_class` until anti-pattern classes stabilize enough
for an enum.

### Display Rule

```rust
pub enum DisplayRule {
    SuppressByDefault,
    ExplainWhenRequested,
    ReviewOnly,
    DocsOnly,
}
```

Display rules are safety semantics, not presentation hints.

## Search API Shape

Keep existing APIs:

```rust
pub fn search(&self, query: &FrameQuery<'_>) -> Vec<FrameCandidate<'static>>;
pub fn search_top(&self, query: &FrameQuery<'_>, limit: usize) -> Vec<FrameCandidate<'static>>;
```

The additive lifecycle API is:

```rust
pub fn search_with_lifecycle(
    &self,
    query: &FrameQuery<'_>,
    lifecycle: &LifecycleFilter<'_>,
) -> FrameSearchReport<'static>;
```

Report shape:

```rust
pub struct FrameSearchReport<'a> {
    pub suggestions: Vec<FrameCandidate<'a>>,
    pub fallbacks: Vec<&'a str>,
    pub suppressed: Vec<SuppressedCandidate<'a>>,
}
```

The existing `search` remains the stable default-search helper. Tests prove
`search_with_lifecycle(query, LifecycleFilter::default_search())` preserves its
suggestion behavior.

## Data Loading Boundary

Do not mix review-only rows into `STARTER_CATALOG`.

Prefer separate static slices or loaders:

```rust
pub const STARTER_CATALOG: &[FrameEntry] = &[/* accepted */];
pub const REVIEW_CATALOG: &[FrameEntry] = &[/* docs-catalog, draft, held */];
pub const REJECTED_CATALOG: &[RejectedFrameEntry] = &[/* anti-patterns */];
```

The first implementation may use fixture-derived static rejected candidates
instead of full `FrameEntry` rows. That avoids pretending anti-patterns have the
same shape as accepted suggestions.

The review-only row model is defined in
[review-only-catalog-data-model.md](review-only-catalog-data-model.md).

## Migration Steps

1. Add tests that parse `docs/eval/lifecycle-rejection-fixtures.json`.
2. Add `DisplayRule`, `ResultClass`, and report structs without changing
   `search`. Complete.
3. Add `LifecycleFilter::default_search()` and tests proving default output is
   unchanged. Complete.
4. Add `search_with_lifecycle` for accepted entries only. Complete.
5. Add suppressed-candidate reports from fixture-backed rejected examples.
   Complete for first lifecycle fixtures.
6. Add docs-catalog or review-only rows only after display rules prevent
   recommendation. Start from
   [review-only-catalog-data-model.md](review-only-catalog-data-model.md).
7. Consider expanding `FrameStatus` in public API only when review rows are
   actually represented. Complete for report statuses; review rows remain out
   of `STARTER_CATALOG`.

## Fixture Coverage

Implementation tests should cover:

| Fixture | API Expectation |
|---|---|
| `LIFE-DEFAULT-001` | Default search suppresses docs-catalog Veto Rule. |
| `LIFE-EXPLAIN-001` | Explanation mode reports unsupported Veto Rule as suppressed, not suggested. |
| `LIFE-ANTI-001` | People-as-obstacles reports `Suppressed` plus plain-language fallback. |
| `LIFE-REL-001` | Rejected near-miss does not appear as alternate. |
| `LIFE-TRAFFIC-001` | Wrong-authority accepted frame can be explained as suppressed. |
| `LIFE-STORY-001` | Empathy-story anti-pattern falls back to repair/accountability language. |

## Compatibility Rules

- Do not rename existing `FrameIndex::search`, `FrameQuery`, or
  `FrameCandidate`.
- Do not add required fields to `FrameEntry` until accepted starter rows and
  review-only rows can both populate them clearly.
- Do not make `include_rejected` imply recommendation.
- Do not expose suppressed candidates unless `explain_suppressed` or a review
  mode is explicit.
- Do not let `accepted_with_caveat` enter default search unless caveat display
  is guaranteed.

## Design Consequences

- Rust lifecycle work is additive and preserves existing examples.
- The first code change proves default-search equivalence and adds
  fixture-backed suppressed reports.
- Rejected-candidate reporting is a separate output channel from suggestions.
- Lifecycle filters should be tested from fixtures before broader catalog rows
  are loaded.
