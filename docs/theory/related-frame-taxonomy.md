# Related-Frame Relation Taxonomy

This taxonomy defines what it means for one frame to be related to another.
`frames-core` currently exposes `related` as an undifferentiated list of frame
IDs. That is useful for a starter thesaurus, but it is not enough for safe AI
selection because "related" can mean alternate, safer fallback, sequence,
conflict, or rejected near-miss.

Typed related-frame relations remain docs-level until enough reviewed examples
exist to justify a Rust API migration.

## Relation Rule

A related-frame link should answer one of these questions:

- Is this another acceptable way to frame the same target?
- Is this safer in a higher-risk or lower-transfer situation?
- Should this frame come before or after the current one?
- Does this frame conflict with the current one?
- Is this a tempting bad match the tool should reject or explain?

If the link cannot answer one of those questions, it should not be modeled as a
related-frame relation.

## Relation Types

| Relation | Direction | Meaning | Use When | Default Tool Behavior |
|---|---|---|---|---|
| `alternate` | symmetric | Either frame may fit the same target job. | Both frames are acceptable and differ mainly by source scene or audience familiarity. | Offer as an alternate. |
| `safer_fallback` | directional | Target frame is safer, plainer, lower-risk, or more evidence-preserving. | The source frame has audience, authority, evidence, or protected-value risk. | Prefer fallback in high-risk or uncertain contexts. |
| `narrower_variant` | directional | Target frame applies to a narrower version of the source frame's target. | A broad frame needs a more specific operational cue. | Offer when query contains the narrower condition. |
| `broader_variant` | directional | Target frame covers a broader class of situations. | A narrow frame may be too specific or unfamiliar. | Offer for novice or low-context audiences. |
| `sequence_before` | directional | Source frame should normally be applied before target frame. | The first frame exposes information, priority, or risk needed before the second. | Show as a step, not as a substitute. |
| `sequence_after` | directional | Source frame should normally be applied after target frame. | The current frame becomes useful only after a prior condition is handled. | Show as a later step. |
| `boundary_frame` | directional | Target frame marks where the source frame stops applying. | A frame needs a stop condition or guardrail. | Display as a warning or limit. |
| `compatible_with` | symmetric | Frames can be composed because they clarify different jobs. | Action cues, authority, evidence, and safety obligations do not conflict. | Offer as optional composition. |
| `conflicts_with` | symmetric | Frames imply incompatible action, authority, evidence, audience, timing, or safety. | Choosing one frame changes or blocks the other. | Do not blend; require selection or sequencing. |
| `replaces` | directional | Source frame should replace a deprecated or weaker frame. | Lifecycle review found a better accepted frame. | Prefer replacement; retain prior only for legacy references. |
| `rejected_near_miss` | directional | Target frame is tempting but should not be recommended. | Lexical similarity, vividness, or source familiarity can trick search. | Exclude from default suggestions; explain only when requested. |
| `plain_language_fallback` | directional | Target is not a metaphor but a direct sentence or instruction. | High stakes, low transfer, or misuse risk makes analogy worse than plain language. | Prefer when safety or clarity gates fail. |

## Direction Rules

Directional relations should read from the current frame to the linked target:

- `safer_fallback`: if this is risky, use that.
- `narrower_variant`: for this specific case, use that.
- `broader_variant`: for a broader or novice-facing case, use that.
- `sequence_before`: use this before that.
- `sequence_after`: use this after that.
- `boundary_frame`: that marks the limit of this.
- `replaces`: this replaces that.
- `rejected_near_miss`: that may look similar but should be rejected.
- `plain_language_fallback`: use that direct sentence when framing fails.

Symmetric relations should be stored once conceptually, even if a future API
materializes them from both frames.

## Selection Implications

| Situation | Relation To Prefer |
|---|---|
| Same target, different audience familiarity | `alternate` or `broader_variant` |
| High-stakes or authority-sensitive use | `safer_fallback`, `plain_language_fallback`, or `boundary_frame` |
| Multi-step reasoning | `sequence_before` and `sequence_after` |
| Conflicting action cues | `conflicts_with` |
| Deprecated or weaker accepted frame | `replaces` |
| Search returns a tempting bad analogy | `rejected_near_miss` |

## Examples

| Source Frame | Relation | Target Frame Or Fallback | Rationale |
|---|---|---|---|
| Four-way stop | `alternate` | Queue turn-taking | Both clarify peer sequencing, but queue may be more familiar for non-drivers. |
| Four-way stop | `conflicts_with` | Incident command | Peer turn-taking conflicts with directed authority. |
| Crosswalk yield | `boundary_frame` | Merge lane | Yield protects a vulnerable party; merge assumes roughly compatible parties. |
| Blind spot | `sequence_before` | Lane change | Identify missing visibility before changing direction. |
| Run Fast | `safer_fallback` | Run Fast / Run Safe | Speed needs an explicit protected-value guard. |
| Team as roadblock | `rejected_near_miss` | Dependency blocked by unresolved ownership | The surface match is tempting, but people-as-obstacles should be rejected. |
| Bag-of-chips story | `plain_language_fallback` | "We do not know intent yet; check facts before assigning blame." | The story is useful only before facts establish harm or duty. |

## Applied Reviews

- [related-frame-application-starter.md](related-frame-application-starter.md)
  applies relation types to starter catalog links and relation fixture backlog
  rows.

## Review Procedure

1. Name the source frame and target frame or fallback.
2. Identify whether the relation is acceptable, safer, sequential, conflicting,
   replacement, or rejected.
3. Check action cue, authority model, evidence boundary, audience transfer, and
   human-safety implications.
4. Decide whether the relation should appear in default search, warning output,
   accepted-catalog metadata, evaluation fixtures, or docs only.
5. Record direction and rationale.

## Catalog Field Shape

Future catalog rows should be able to express:

```text
related:
  - frame_id:
    relation:
    direction:
    rationale:
    display_rule:
```

`display_rule` should be one of:

- show_as_alternate;
- show_as_warning;
- show_as_sequence;
- show_as_fallback;
- suppress_by_default;
- docs_only.

## AI And Tool Implications

- Default related lookup should continue returning accepted related frames until
  typed relation filters exist.
- AI response contracts should distinguish alternates from safer fallbacks and
  rejected near-misses.
- Transfer-aware search should use `conflicts_with`, `boundary_frame`,
  `rejected_near_miss`, and `plain_language_fallback` before ranking surface
  similarity.
- Evaluation sets should include relation-specific expectations so a tool can be
  tested for suggesting, suppressing, sequencing, or explaining related frames.

## Design Consequences

- `frames-core` should not expose a typed relation enum until accepted catalog
  rows have enough reviewed examples.
- The next metadata migration should keep current `related` IDs compatible while
  adding docs-level relation labels.
- Evaluation-set design should include cases for `alternate`,
  `safer_fallback`, `conflicts_with`, `sequence_before`, `rejected_near_miss`,
  and `plain_language_fallback`.
- Accepted-catalog review should require at least one safer fallback or boundary
  relation for medium- and high-risk frames.
