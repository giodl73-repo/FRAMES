# Claim Strength Labels

Claim strength labels say how much confidence FRAMES is allowed to attach to a
frame, pattern, recommendation, or public statement. They prevent vivid stories,
practitioner popularity, or cognitive-science references from sounding stronger
than the evidence allows.

A frame can be useful without being validated. The label tells the reader what
kind of usefulness is being claimed.

## Label Set

| Label | Meaning | Allowed Language | Required Evidence | Do Not Say |
|---|---|---|---|---|
| `illustrative` | Helps people picture or discuss a situation. | "This can illustrate..." | Clear source scene, target situation, and exclusions. | "This shows that..." |
| `heuristic` | Offers a practical rule of thumb for selection or action. | "Use this to check..." | Fit rubric, evidence boundary, misuse warning, and at least one example. | "This will work..." |
| `theory_informed` | Aligns with cited research or established theory, but is not directly validated for the current context. | "This is consistent with..." | Citation, bounded interpretation, and transfer limits. | "Research proves this frame..." |
| `practitioner_observed` | Pattern appears in successful practitioner, leadership, product, or teaching usage. | "Practitioners use this pattern to..." | Source tradition, bounded lesson, and risk note. | "This is validated because it is popular..." |
| `locally_observed` | Pattern appears in internal notes, books, meetings, or portfolio experience. | "In local sources, this appears as..." | Source pointer, extraction note, and review status. | "Teams generally..." |
| `role_reviewed` | A repo role review accepted the frame or theory for a stated use. | "Role review accepted this for..." | Reviewer role, decision, risks, and scope. | "Approved for all contexts..." |
| `empirically_validated` | Direct evidence supports the effect in a defined audience/context. | "In this tested context..." | Study or validation protocol, sample/context, result, and limitation. | "Universal..." |
| `anti_pattern` | Known or likely misuse that should be avoided. | "Reject when..." | Failure mechanism, safer replacement, and trigger. | "Never use any related frame..." |

## Default Labels

Use conservative defaults:

| Artifact | Default Label |
|---|---|
| New frame candidate | `illustrative` |
| Accepted catalog entry without direct validation | `heuristic` |
| Cognitive-science design rule with citation | `theory_informed` |
| Public writer or consultant pattern | `practitioner_observed` |
| Extracted local story or book pattern | `locally_observed` |
| Role panel acceptance | `role_reviewed` |
| Measured audience outcome | `empirically_validated` |
| Rejected misuse pattern | `anti_pattern` |

The default can be upgraded only when the evidence changes. A memorable story
does not upgrade itself.

## Promotion Rules

| From | To | Promotion Requirement |
|---|---|---|
| `illustrative` | `heuristic` | Fit rubric passes, evidence boundary is explicit, and misuse warning is usable. |
| `heuristic` | `role_reviewed` | Named role review accepts the use case and records risks. |
| `heuristic` | `theory_informed` | Research grounding cites a relevant theory and bounds the transfer. |
| `practitioner_observed` | `heuristic` | FRAMES translates the practitioner pattern into action cue, evidence boundary, and misuse warning. |
| `locally_observed` | `heuristic` | Local source extraction is reviewed and generalized without overclaiming. |
| `role_reviewed` | `empirically_validated` | A validation protocol observes the intended effect in a defined audience/context. |

Labels can also be downgraded. If a frame spreads beyond its reviewed context,
drop it to the weakest defensible label until the new use is reviewed.

## Required Claim Shape

Public-facing claims should use this shape:

```text
Claim: <plain statement>
Strength: <label>
Applies when: <audience/context>
Evidence: <source, review, or validation>
Boundary: <what this does not prove>
Risk: <main misuse>
```

Example:

```text
Claim: Four-way stop can help peer teams reason about turn order.
Strength: heuristic
Applies when: teams have roughly equal authority and a shared contention point.
Evidence: fit rubric and relational transfer review.
Boundary: does not apply when one party owns the decision or bears protected duty.
Risk: can hide urgency or authority asymmetry.
```

## Local Example: Theme Swimlanes

Pattern: a large program creates a unifying wave such as `Run One`, then
extends the same force pattern into adjacent promises such as `Run Lean`,
`Run Fast`, and `Run Safe`. Another variant chooses three high-level customer
promise lanes such as more control, more visibility, and less effort.

Claim shape:

```text
Claim: Theme swimlanes can help teams contribute different work to one shared
customer promise.
Strength: locally_observed
Applies when: multiple teams need a small set of memorable contribution lanes.
Evidence: local program practice and portfolio reflection.
Boundary: does not prove that three lanes is always the right number or that
theme naming fixes portfolio priority conflicts.
Risk: slogans can hide tradeoffs if lane owners, measures, and customer outcomes
are not explicit.
```

Promotion path: this pattern can move from `locally_observed` to `heuristic`
only after FRAMES defines the relational transfer: shared promise, contribution
lane, owner, measure, constraint, and exclusion. It can move to `role_reviewed`
after business-leader, catalog-structure, and evidence-boundary review.

## Hard Stops

Reject or rewrite a claim when:

- It says or implies that a frame proves a conclusion.
- It upgrades practitioner success into empirical validation.
- It cites cognitive science without naming the transfer boundary.
- It presents a local story as a general human pattern.
- It uses "validated" without a defined audience, context, method, and result.
- It hides that a frame is persuasive, contested, or high stakes.

## Integration Points

Use claim-strength labels with:

- [research-grounding.md](research-grounding.md) for public cognitive-science
  statements.
- [evidence-boundary-schema.md](evidence-boundary-schema.md) for frame-specific
  proof boundaries.
- [fit-rubric.md](fit-rubric.md) when deciding whether a frame may move from
  illustrative to heuristic.
- [relational-transfer-fields.md](relational-transfer-fields.md) when deciding
  whether a claim is structure-based or only surface-similar.
- [external-frame-practitioners.md](external-frame-practitioners.md) when
  importing public writer patterns.
- Local import maps when extracting patterns from RESONANCE, CAREER, or other
  portfolio sources.

## Design Consequences

- Future catalog entries should include `claim_strength` before FRAMES presents
  recommendations as stable guidance.
- AI-agent responses should avoid stronger language than the label permits.
- Search results should expose weak labels for high-stakes contexts.
- Public docs should reserve `empirically_validated` for direct validation, not
  for citations about adjacent theory.
