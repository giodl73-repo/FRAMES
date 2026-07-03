# Frame Acquisition Method

This method defines how FRAMES discovers, captures, screens, and promotes new
frame candidates without turning the catalog into a metaphor grab bag.

The goal is not to collect clever analogies. The goal is to find source scenes
that transfer useful structure to a target situation while preserving evidence,
authority, portability, and human-safety boundaries.

## Acquisition Rule

A candidate can enter the repo only when it names:

- the target situation it helps;
- the source scene people may recognize;
- the structural relation being transferred;
- the action cue it changes;
- the evidence boundary it must not replace;
- the misuse risk that would make it harmful;
- the intended audience and application pack.

If a candidate cannot name those fields, keep it in notes, not catalog or eval
fixtures.

## Source Channels

| Channel | Use For | Required Capture |
|---|---|---|
| Local practice | Patterns used in real work, such as program themes or conflict stories. | Context, observed effect, changed decision, and limitation. |
| Role interviews | Examples from business leaders, novices, journeymen, storytellers, reviewers, and operators. | Role, situation, frame, why it helped, and where it failed. |
| Literature and research | Cognitive science, metaphor theory, pedagogy, decision science, and communication research. | Citation, claim-strength limit, and transfer implication. |
| Existing catalog adjacency | Alternates, safer fallbacks, boundaries, and rejected near-misses around accepted frames. | Relation type, display rule, and expected tool behavior. |
| External practitioners | Public writers, facilitators, and narrative designers. | Technique extracted without copying proprietary text or overstating evidence. |
| Anti-pattern review | Misleading or harmful frames worth recording as rejected examples. | Anti-pattern class, trigger, safer output, and suppression rule. |

## Candidate Intake Template

```text
Candidate ID:
Name:
Source channel:
Observed or proposed by:
Date captured:

Source scene:
Target situation:
Application pack:
Audience:
Frame job:
Relation term:
Authority term:

Action cue:
Evidence boundary:
Misuse risk:
Portability risk:
Plain-language fallback:

Example use:
Example non-use:
Related frames:
Candidate status:
Next gate:
```

## Screening Gates

| Gate | Pass Question | Failing Outcome |
|---|---|---|
| Fit | Does the source relation match the target structure, not just words? | Hold as note. |
| Action | Would a user act, decide, sequence, or inspect differently? | Reject as decorative. |
| Evidence | Does the frame name what must still be proven outside the analogy? | Hold until boundary exists. |
| Authority | Does the frame match actual decision rights and duties? | Reject or add fallback. |
| Human safety | Does it avoid blaming, dehumanizing, coercing, or trivializing harm? | Reject or anti-pattern only. |
| Portability | Is the audience likely to understand the source, or is a fallback supplied? | Mark bounded, limited, unknown, or unsafe. |
| Lifecycle | Is the next state draft, reviewed candidate, accepted with caveat, accepted, held, rejected, or deprecated? | Do not index. |

## Promotion Path

1. Capture candidate using the intake template.
2. Assign ontology terms from `frame-ontology.md`.
3. Apply fit scoring from `fit-rubric.md`.
4. Apply audience and cultural portability review.
5. Apply anti-pattern taxonomy.
6. Add relation map entries for alternates, boundaries, fallbacks, conflicts,
   and rejected near-misses.
7. Add at least one positive and one near-miss or anti-pattern evaluation
   fixture before tool behavior depends on the candidate.
8. Run accepted-catalog review when promotion is requested.
9. Keep Rust index changes behind accepted status, lifecycle filtering, and
   evaluation-backed behavior.

## Candidate States

| State | Meaning | Allowed Use |
|---|---|---|
| note | Captured idea without enough structure. | Research backlog only. |
| draft candidate | Intake fields are mostly complete. | Theory discussion and examples. |
| role-reviewed draft | Role lenses reviewed the candidate. | Controlled worksheet or application-pack pilot. |
| docs-catalog candidate | Accepted with caveat for docs visibility. | Docs catalog only, not default Rust search. |
| accepted starter | Passed catalog review and starter constraints. | Default deterministic search. |
| held | Useful but missing evidence, safety, portability, or lifecycle support. | Revisit only when gate is resolved. |
| rejected | Fails fit, safety, evidence, or authority. | Anti-pattern or rejected near-miss only. |

## Scoring Notes

Acquisition scoring should penalize:

- surface word overlap without structural transfer;
- memorable stories that erase evidence or harm;
- frames with no stop condition;
- source scenes that assume a narrow culture, role, language, mobility, or
  authority model;
- candidates that cannot produce a near-miss fixture.

Acquisition scoring should reward:

- clear action cue;
- explicit evidence boundary;
- role-reviewed usefulness;
- safer fallback;
- strong relation to existing catalog frames;
- clear rejection cases.

## Initial Backlog Buckets

| Bucket | Examples | First Gate |
|---|---|---|
| Traffic and movement | Roundabout, zipper merge, shoulder pull-off, walking pace. | Portability and authority check. |
| Signals and instruments | Thermostat, smoke alarm, checklist, compass. | Evidence-boundary check. |
| Story and perspective | Bag-of-chips, multi-perspective same-person story. | Empathy-erasure and repair check. |
| Program themes | Run One, Run Lean, Run Fast, Run Safe, customer promise lanes. | Changed-decision pilot. |
| Conflict and dependency | Load-bearing wall, blind spot, blocked dependency. | People-as-obstacles check. |
| Plain-language fallbacks | Direct owner, threshold, requirement, and tradeoff language. | High-stakes clarity check. |

## Design Consequences

- New candidates should not enter accepted starter catalog without intake,
  role review, anti-pattern review, portability review, fixture coverage, and
  accepted-catalog decision.
- Future tooling should support candidate intake separately from search.
- Rejected candidates are valuable if they improve suppression, warnings, or
  safer fallbacks.
- Acquisition remains docs-level until candidate-state and lifecycle filtering
  are implemented in Rust.
