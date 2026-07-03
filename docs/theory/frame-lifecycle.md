# Frame Lifecycle

The frame lifecycle controls how an analogy moves through the catalog. It keeps
FRAMES from treating every memorable comparison as ready for AI/tool use.

## Lifecycle States

| State | Meaning | Can Be Indexed? |
|---|---|---|
| Candidate | A proposed frame with enough shape to review. | No |
| Draft | A frame with source, target, mapping, action cue, and warning. | No |
| Accepted | A frame that passes fit, audience, evidence, and misuse checks. | Yes |
| Revised | An accepted frame that has changed materially and needs re-checking. | Conditional |
| Held | A frame with potential but unresolved fit, transfer, or evidence issues. | No |
| Deprecated | A previously accepted frame that should be replaced. | No for new use |
| Rejected | A frame that is misleading, unsafe, or too weak. | No |

## Entry Criteria

| To Enter | Required Evidence |
|---|---|
| Candidate | Name, source scene, and rough target situation. |
| Draft | Core six-part frame shape is filled in. |
| Accepted | Fit score 12+, no hard stops, audience transfer checked. |
| Revised | Specific change reason and affected fields. |
| Held | Clear unresolved question and revisit trigger. |
| Deprecated | Replacement or retirement rationale. |
| Rejected | Hard stop or low score with rationale. |

Detailed acceptance gates live in
[accepted-catalog-review-process.md](accepted-catalog-review-process.md).

## Exit Criteria

| From State | Exit Path |
|---|---|
| Candidate | Draft, held, or rejected. |
| Draft | Accepted, revised, held, or rejected. |
| Accepted | Revised, deprecated, or retained after review. |
| Revised | Accepted or held after re-scoring changed dimensions. |
| Held | Draft, accepted, or rejected after the revisit trigger. |
| Deprecated | Retired once downstream references are updated. |
| Rejected | Reopened only with a new target situation or source mapping. |

## Change Triggers

Re-review a frame when:

- the target situation changes,
- the intended audience changes,
- the source scene proves unfamiliar or exclusionary,
- the action cue creates the wrong behavior,
- a misuse pattern appears in real use,
- evidence requirements are missing,
- the frame is added to `frames-core`,
- related frames create a better alternate.

## Catalog Status Fields

Future catalog rows should add:

| Field | Meaning |
|---|---|
| Status | candidate, draft, accepted, revised, held, deprecated, rejected. |
| Fit score | Latest rubric score. |
| Review date | When the current status was assigned. |
| Review lens | Role or reviewer that assigned status. |
| Replacement | Better frame when deprecated or rejected. |
| Revisit trigger | Condition that reopens held or accepted frames. |

## Indexing Rule

Only accepted frames should be indexed by default. Revised frames may stay in
the index only when the changed fields do not affect action cue, evidence
boundary, human safety, or audience transfer.

Held, deprecated, and rejected frames can remain in docs as examples, but tools
should not suggest them unless the caller explicitly asks for anti-patterns or
rejected comparisons.

## Deprecation Rule

Deprecation is not failure. It is how FRAMES preserves learning without forcing
old language into new situations.

Deprecate rather than delete when:

- a frame has existing references,
- a better alternate exists,
- the source scene became audience-limited,
- the action cue is too weak for current use,
- the frame is useful as a cautionary example.

## Role Checks

| Role | Lifecycle Question |
|---|---|
| Frame Fit Reviewer | Does the current status match the latest fit score? |
| Audience Transfer Reviewer | Has the intended audience changed? |
| Evidence Boundary Reviewer | Does acceptance require evidence not recorded yet? |
| Misuse Risk Reviewer | Should a misuse report move the frame to held or deprecated? |
| Catalog Structure Reviewer | Are lifecycle fields consistent across docs and index? |
| Business Leader | Does the status help a decision-maker know whether to use the frame now? |

## Design Consequence for `frames-core`

`frames-core` should eventually expose lifecycle status. Until then, the starter
index should include only accepted frames. Anti-patterns such as "team as
roadblock" belong in theory docs, not default search results.
