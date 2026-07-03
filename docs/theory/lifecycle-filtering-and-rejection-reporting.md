# Lifecycle Filtering And Rejection Reporting

Lifecycle filtering controls which frames a tool is allowed to suggest.
Rejected-candidate reporting controls how a tool explains tempting matches that
were suppressed. These two behaviors must be designed together because a
larger frame index will contain accepted frames, docs-catalog candidates,
drafts, held ideas, deprecated frames, rejected near-misses, anti-patterns, and
plain-language fallbacks.

The default rule is conservative: `frames-core` search continues to return only
accepted starter frames until lifecycle filters, rejection fixtures, and
suppression explanations exist.

## Problem

FRAMES is becoming more than a metaphor list. It now has:

- accepted starter entries in Rust,
- reviewed docs-catalog candidates such as Veto Rule,
- draft local imports,
- anti-pattern examples,
- rejected near-misses,
- relation fixtures,
- plain-language fallbacks.

Without lifecycle filtering, a tool can accidentally recommend a docs-only,
held, rejected, or anti-pattern frame. Without rejected-candidate reporting, a
tool cannot explain why a vivid or lexical match was not recommended.

## Visibility Modes

| Mode | Purpose | Visible Statuses | Default Caller |
|---|---|---|---|
| Default search | Suggest safe-enough frames for use. | Accepted starter entries only. | AI/tool users. |
| Catalog review | Inspect candidates before promotion. | Candidate, draft, held, accepted, accepted with caveat, deprecated, rejected. | Catalog reviewers. |
| Anti-pattern review | Teach or test bad matches. | Rejected, anti-pattern, held, safer fallback. | Misuse reviewers and evaluator builders. |
| Docs-catalog preview | Show reviewed but not default-search entries. | Accepted with caveat and docs-catalog candidates. | Maintainers and expert users. |
| Explanation mode | Explain why suppressed candidates were not returned. | Suppressed rejected/held/draft/deprecated candidates by opt-in only. | AI-agent builders. |

Default search must not silently include anything outside accepted starter
entries. A caller may request broader visibility only by naming the mode or
explicit filters.

## Query And Filter Fields

A future tool-facing query should separate normal search from review expansion:

| Field | Meaning | Default |
|---|---|---|
| `allowed_statuses` | Exact lifecycle states allowed in candidate results. | `accepted`. |
| `include_docs_catalog` | Allows reviewed docs-catalog candidates with caveat display. | `false`. |
| `include_draft` | Allows candidate or draft rows for review, not recommendation. | `false`. |
| `include_held` | Allows held rows when reviewer needs unresolved examples. | `false`. |
| `include_rejected` | Allows rejected or anti-pattern rows for review or evaluation. | `false`. |
| `explain_suppressed` | Returns rejected/held/draft matches as explanations, not suggestions. | `false`. |
| `application_pack` | Applies pack defaults and exclusions. | Caller supplied. |
| `authority_model` | Filters wrong authority transfers before lexical scoring. | Caller supplied. |
| `risk_band` | Filters or warns based on use risk. | Caller supplied. |
| `protected_value` | Raises fallback and suppression requirements. | Optional until metadata stabilizes. |

`include_rejected` and `explain_suppressed` are different. Including rejected
frames makes them visible for review. Explaining suppressed frames keeps them
out of recommendations while showing why they were rejected.

## Result Classes

| Class | Can Be Recommended? | Use |
|---|---|---|
| `suggested` | Yes. | Accepted frame that passes filters. |
| `alternate` | Yes, with rationale. | Accepted alternate or safer fallback. |
| `fallback` | Yes. | Plain-language fallback when metaphor is unsafe or unclear. |
| `suppressed` | No. | Tempting but disallowed draft, held, deprecated, rejected, or anti-pattern candidate. |
| `review_only` | No. | Candidate visible only in catalog or anti-pattern review mode. |

Search ranking should not blend these classes. A suppressed candidate can have
a high lexical match and still be excluded from suggestion output.

## Rejected-Candidate Report Shape

When `explain_suppressed` is enabled, each suppressed candidate should expose:

| Field | Meaning |
|---|---|
| `candidate_id` | Stable ID when available; otherwise source document and heading. |
| `candidate_name` | Human-readable name. |
| `status` | `draft`, `held`, `deprecated`, `rejected`, `anti_pattern`, or `accepted_with_caveat`. |
| `matched_reason` | Why the candidate looked tempting: lexical match, source scene, relation, tag, or story appeal. |
| `rejection_class` | Anti-pattern or lifecycle reason, such as false authority transfer or people as obstacles. |
| `violated_boundary` | Authority, evidence, safety, audience, portability, protected value, or stop condition boundary. |
| `safer_frame` | Accepted alternate when one exists. |
| `plain_language_fallback` | Direct sentence when a frame would add risk. |
| `source_docs` | Theory, review, or fixture documents supporting the rejection. |
| `display_rule` | `suppress_by_default`, `explain_when_requested`, `review_only`, or `docs_only`. |

The report should say why the candidate was attractive before saying why it was
rejected. This helps AI agents avoid repeating the same bad match.

## Default Behavior

Default search:

- searches only accepted starter entries;
- returns suggested frames, accepted alternates, and accepted fallbacks only;
- does not return docs-catalog candidates;
- does not return draft, held, deprecated, rejected, or anti-pattern rows;
- does not expose suppressed candidates unless `explain_suppressed` is set.

Catalog review mode:

- can inspect draft, held, docs-catalog, deprecated, and rejected entries;
- must display status and caveat labels;
- must not present review-only rows as recommended use.

AI-agent mode:

- may request suppressed-candidate explanations;
- must keep suppressed candidates out of the recommendation list;
- must include safer frame or plain-language fallback when suppression is due
  to safety, authority, evidence, or protected-value risk.

## Fixture Gate

Lifecycle filtering and rejected-candidate reporting should not move into Rust
until machine-readable fixtures cover at least:

| Fixture | Expected Behavior |
|---|---|
| `EVAL-VETO-003` | False veto is suppressed or explained with authority/evidence boundary. |
| `EVAL-ANTI-001` | People-as-obstacles frame is rejected and replaced with direct dependency language. |
| `EVAL-REL-004` | Rejected near-miss relation is not returned as an alternate. |
| `EVAL-TRAFFIC-002` | Protected-party yield beats peer turn-taking when authority is asymmetric. |
| `EVAL-STORY-002` | Bag-of-chips empathy story falls back to repair/accountability after facts establish harm. |

Each fixture should include query fields, allowed statuses, expected
suggestions, expected suppressed candidates, safer fallback, and source docs.

The first machine-readable package for this gate lives in
[../eval/lifecycle-rejection-fixtures.json](../eval/lifecycle-rejection-fixtures.json).
It links suppression behavior back to the starter evaluation package without
changing default Rust search.

The Rust API shape and first additive implementation are described in
[rust-lifecycle-filter-api-design.md](rust-lifecycle-filter-api-design.md).

## Rust Migration Gate

Do not expand the Rust lifecycle enum or default search behavior until:

1. docs-level lifecycle status values are stable;
2. rejected-candidate report fields have fixture coverage;
3. default accepted-starter behavior is preserved by tests;
4. review-only rows can be loaded without becoming recommendations;
5. AI response contract examples can display suppression without confusing it
   with suggestion.

The first Rust implementation should add filters and result classes before
adding broader catalog rows. The accepted starter catalog can remain unchanged
while the filtering machinery proves its boundaries.

## Design Consequences

- Do not promote Veto Rule or other docs-catalog candidates to default search
  until lifecycle filtering and rejected-candidate reporting are implemented.
- Do not add rejected anti-patterns to default related-frame lookup.
- Do not use lexical match as evidence that a rejected frame should be shown.
- Prefer plain-language fallback when a suppressed candidate failed authority,
  evidence, safety, or protected-value checks.
- Treat rejected-candidate reporting as a safety feature, not as a way to offer
  more colorful metaphors.

## Role Checks

| Role | Check |
|---|---|
| Catalog Structure Reviewer | Are statuses and display rules machine-readable before tool use? |
| Misuse Risk Reviewer | Are unsafe tempting matches suppressed by default? |
| Evidence Boundary Reviewer | Does every rejection name the evidence boundary it protects? |
| Business Leader | Does the output help a decision-maker choose a safer action, not just understand why a phrase was blocked? |
| AI Tool Builder | Can the model distinguish suggestions from suppressed explanations? |
| Novice Reader | Is rejected-candidate language clear without requiring taxonomy expertise? |
