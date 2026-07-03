# Review-Only Catalog Data Model

The review-only catalog data model defines how non-default rows can be shaped
before they enter `frames-core`. It covers docs-catalog candidates, draft
frames, held frames, deprecated frames, rejected frames, and anti-patterns.

The rule is simple: review-only rows may be loaded for inspection and
suppressed-candidate explanation, but they must not become recommendations
without an accepted-catalog decision and an explicit display rule.

## Why A Separate Model

Accepted starter entries and review-only entries do different jobs.

Accepted starter entries are suggestions. They need action cues, evidence
boundaries, misuse warnings, and enough metadata for safe default lookup.

Review-only entries are governance objects. They need status, decision reason,
rejection class, caveat, fallback, source evidence, and display rule. Some do
not deserve a full `FrameEntry` shape because they should never be suggested.

Do not force anti-patterns and rejected rows into `STARTER_CATALOG` just to make
them searchable.

## Row Families

| Family | Purpose | Default Recommendation? | Example |
|---|---|---|---|
| Docs-catalog candidate | Reviewed for docs use but not default search. | No. | `veto-rule`. |
| Draft frame | Has enough shape to review. | No. | Local import before acceptance. |
| Held frame | Has unresolved safety, authority, evidence, or transfer risk. | No. | Theme without owners/measures. |
| Deprecated frame | Formerly useful but replaced or retired. | No. | Older phrase replaced by safer fallback. |
| Rejected frame | Misleading, unsafe, decorative, or lower value than plain language. | No. | Unsupported veto. |
| Anti-pattern | Failed frame useful for teaching or suppression. | No. | Team as roadblock. |

## Minimal Row Shape

```text
id:
name:
status:
review_family:
claim_strength:
risk_band:
application_packs:
authority_model:
source_family:
relation_term:
target_situations:
tags:
matched_terms:
action_cue:
evidence_boundary:
misuse_warning:
review_decision:
decision_reason:
rejection_class:
violated_boundary:
plain_language_fallback:
safer_frame:
source_docs:
display_rule:
review_status:
review_date:
revisit_trigger:
```

Not every row needs every field. Anti-pattern and rejected rows may omit
`action_cue` if the correct action is the fallback. Accepted-with-caveat rows
must include enough fields to explain why they are not default suggestions.

## Required Fields By Status

| Status | Required Fields |
|---|---|
| `accepted_with_caveat` | id, name, status, caveat or decision reason, evidence boundary, misuse warning, fallback, display rule, source docs. |
| `candidate` | id, name, source/target shape, source docs, open question, display rule. |
| `draft` | id, name, source, target situations, action cue, evidence boundary, misuse warning, review status, display rule. |
| `held` | id, name, violated boundary or unresolved question, fallback, revisit trigger, display rule. |
| `deprecated` | id, name, replacement or fallback, retirement reason, display rule. |
| `rejected` | id, name, rejection class, violated boundary, fallback, source docs, display rule. |
| `anti_pattern` | id, name, rejection class, matched terms, violated boundary, fallback, source docs, display rule. |

## Display Rules

| Display Rule | Meaning | Allowed Use |
|---|---|---|
| `suppress_by_default` | Hidden from recommendations and ordinary search. | Default for review-only rows. |
| `explain_when_requested` | May appear in suppressed-candidate reports only when caller opts in. | Rejected near-misses, anti-patterns, wrong-authority matches. |
| `review_only` | Visible in catalog review mode, not tool suggestions. | Draft, held, deprecated, and candidate rows. |
| `docs_only` | Visible in docs but not loaded into tool behavior yet. | Early candidates or examples without stable metadata. |

Display rules are not UI labels. They are behavior constraints.

## Rust Loading Boundary

Keep accepted and review-only data separate:

```rust
pub const STARTER_CATALOG: &[FrameEntry] = &[/* accepted suggestions */];
pub const REVIEW_CATALOG: &[ReviewFrameEntry] = &[/* non-default rows */];
```

Suggested review row type:

```rust
pub struct ReviewFrameEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub status: FrameStatus,
    pub review_family: ReviewFamily,
    pub risk_band: Option<RiskBand>,
    pub authority_model: Option<AuthorityModel>,
    pub application_packs: &'static [ApplicationPack],
    pub target_situations: &'static [&'static str],
    pub matched_terms: &'static [&'static str],
    pub evidence_boundary: &'static str,
    pub misuse_warning: &'static str,
    pub review_decision: &'static str,
    pub rejection_class: &'static str,
    pub violated_boundary: &'static str,
    pub plain_language_fallback: &'static str,
    pub safer_frame: Option<&'static str>,
    pub source_docs: &'static [&'static str],
    pub display_rule: DisplayRule,
}
```

`ReviewFrameEntry` should not implement or imply `FrameCandidate`. It can be
converted into `SuppressedCandidate` or a review-mode row only after lifecycle
filters allow it.

## Review Families

```rust
pub enum ReviewFamily {
    DocsCatalogCandidate,
    Candidate,
    Draft,
    Held,
    Deprecated,
    Rejected,
    AntiPattern,
}
```

`ReviewFamily` answers why the row is outside default search. `FrameStatus`
answers the lifecycle state.

## Conversion Rules

| From | To | Rule |
|---|---|---|
| `FrameEntry` | `FrameCandidate` | Allowed in default search when status is accepted and filters pass. |
| `ReviewFrameEntry` | `SuppressedCandidate` | Allowed when matched and `explain_suppressed` or review mode is explicit. |
| `ReviewFrameEntry` | Review-mode output | Allowed in catalog, anti-pattern, or docs-catalog preview mode. |
| `ReviewFrameEntry` | `FrameEntry` | Only after accepted-catalog review promotes it and metadata is complete. |
| `ReviewFrameEntry` | Default suggestion | Never directly. Promote first. |

## Matching Rules

Review-only matching should be simpler and stricter than accepted search:

- match only explicit `matched_terms`, stable IDs, or tagged relation terms;
- require authority/application filters when the fixture expects them;
- never use source vividness alone;
- never rank a review-only row above an accepted suggestion;
- return fallback when review-only row is the only strong match.

## First Rows To Model

| ID | Family | Status | Display Rule | Source |
|---|---|---|---|---|
| `veto-rule` | docs-catalog candidate | accepted_with_caveat | suppress_by_default / explain_when_requested | `accepted-catalog-review-veto-rule.md` |
| `team-as-roadblock` | anti-pattern | anti_pattern | explain_when_requested | `frame-antipattern-taxonomy.md` |
| `bag-of-chips-as-excuse` | anti-pattern | anti_pattern | explain_when_requested | `frame-antipattern-taxonomy.md` |
| `theme-swimlanes` | draft or held depending on evidence | held until owner/measure/pilot evidence exists | review_only | `theme-swimlane-*` docs |

## Migration Steps

1. Define `ReviewFamily` and `ReviewFrameEntry` in docs.
2. Add a machine-readable review-only catalog fixture. Complete in
   [../eval/review-only-catalog-fixtures.json](../eval/review-only-catalog-fixtures.json).
3. Add Rust static review rows for the first small set. Complete in
   `src/lib.rs`.
4. Convert review rows to `SuppressedCandidate` in `search_with_lifecycle`.
   Complete for the first suppression examples, with wrong-authority accepted
   frames remaining separate.
5. Add catalog review mode output only after suppressed behavior is tested.
6. Keep `STARTER_CATALOG` accepted-only.

## Acceptance Gate

A review-only row can become a default-search candidate only when:

- accepted-catalog review records accepted or accepted-with-caveat decision;
- caveat display is guaranteed if accepted with caveat;
- evidence boundary and misuse warning are non-empty;
- relation and anti-pattern fixtures pass;
- VTRACE records the promotion work package;
- default-search tests prove no rejected or held rows leak into suggestions.

## Design Consequences

- Suppressed reports for review-only rows are backed by `REVIEW_CATALOG`; accepted
  wrong-authority reports remain separate from review rows.
- Review-only data gives tools a memory of rejected near-misses without making
  them suggestions.
- Default search remains accepted-starter only until a row is explicitly
  promoted.
- The next implementation step should add catalog review mode output for
  `REVIEW_CATALOG` rows without changing default search.
