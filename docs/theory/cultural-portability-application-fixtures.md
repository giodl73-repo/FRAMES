# Cultural Portability Application: Starter Fixtures

This artifact applies `cultural-portability.md` to the starter evaluation
fixture package in `docs/eval/starter-fixtures.json`.

The first pass keeps portability as fixture-level metadata, not Rust metadata.
That lets reviewers check audience, mobility, region, language, and authority
risks before search or ranking uses portability automatically.

## Band Decisions

| Fixture Area | Default Band | Main Risk | Required Move |
|---|---|---|---|
| Traffic coordination | Bounded | Driving rules and right-of-way norms vary by region and audience. | Offer queue, explicit turn order, or direct authority language. |
| Status colors | Bounded | Color semantics and accessibility vary; color alone hides thresholds. | Pair color with text label, threshold, owner, and evidence. |
| Veto Rule | Bounded | "Veto" may imply formal authority where none exists. | Require owner, source of authority, evidence, and fallback to decision-rights language. |
| People-as-obstacles anti-pattern | Unsafe | Dehumanizes teams and launders blame. | Reject and use dependency or ownership language. |
| Theme swimlanes | Bounded | Swimlane and slogan metaphors can be opaque or theatrical. | Name customer promise, lane owners, measures, and plain contribution lanes. |
| Bag-of-chips story | Limited | Empathy story can erase harm after facts are known. | Use only while facts are incomplete; otherwise name repair and ownership. |
| Relation behavior | Bounded | Relation types can hide authority or safety differences if treated as alternates. | Preserve relation type and display rule. |
| Audience transfer | Unknown | Audience familiarity is explicitly unknown. | Prefer plain language or non-driving alternate first. |

## Fixture Package Implication

`starter-fixtures.json` now includes `portability_profiles` keyed by fixture ID.
Each profile records:

- `band`;
- `risk`;
- `safer_alternate`;
- `plain_language_fallback`;
- `review_note`.

These fields remain docs-level until portability filters and evaluation checks
are implemented.
