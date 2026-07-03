# Accepted Catalog Review Process

This process defines how a draft, promoted heuristic, or local import can become
an accepted catalog entry. It is the gate between "interesting frame" and "safe
enough for default AI/tool suggestion."

Acceptance is not empirical validation. It means the frame is structurally
sound, bounded, reviewed, and useful enough to appear in the accepted catalog
with explicit claim strength and warnings.

## Review Inputs

A candidate cannot enter accepted-catalog review until it has:

| Input | Required Evidence |
|---|---|
| Frame shape | Source scene, target situation, useful mapping, action cue, failure mode, better-fit-than or alternate. |
| Relational transfer | Source relation, target relation, authority relation, protected value, exclusions, and transfer strength. |
| Evidence boundary | What must still be checked outside the frame. |
| Claim strength | Current label and why stronger labels are not claimed. |
| Audience transfer | Intended audience, familiarity assumptions, and alternates. |
| Application pack | Product, operations, leadership, learning, AI-agent, or explicitly excluded. |
| Fit score | Current fit-rubric score with hard-stop check. |
| Role review | Business, novice/journeyman, evidence, misuse, and catalog findings. |
| Examples | At least two target examples and one near-miss or hard-stop example. |

## Acceptance Board

The acceptance board is a set of review lenses, not a standing human committee.
Each lens must record pass, pass with gate, revise, hold, or reject.

| Lens | Acceptance Question |
|---|---|
| Frame Fit Reviewer | Does the frame improve action selection compared with plain language? |
| Evidence Boundary Reviewer | Is the evidence obligation visible and testable? |
| Misuse Risk Reviewer | Does the frame avoid dehumanization, coercion, blame, and hidden cuts? |
| Audience Transfer Reviewer | Will the intended audience understand the source and authority assumptions? |
| Business Leader | Does the frame clarify decisions, tradeoffs, ownership, or customer promise? |
| Novice Reader | Can a new reader understand the frame without repo history? |
| Journeyman Practitioner | Can a practitioner use it in a normal meeting or planning workflow? |
| Catalog Structure Reviewer | Are metadata, lifecycle status, related frames, and index rules consistent? |

## Decision Bands

| Outcome | Meaning | Catalog / Index Rule |
|---|---|---|
| Accepted | Fit score 12+, no hard stops, role gates pass, metadata populated. | May enter accepted catalog and default search. |
| Accepted with caveat | Fit score 12+, one bounded caveat with explicit display warning. | May enter accepted catalog; default search only when caveat can display. |
| Revise | Useful, but field shape, role review, examples, or evidence boundary is incomplete. | Stay draft; not default search. |
| Hold | Potentially useful but unresolved safety, authority, audience, or evidence risk. | Stay held; not default search. |
| Reject | Misleading, unsafe, decorative, or lower value than plain language. | Keep only as anti-pattern if useful. |

## Hard Stops

Reject or hold immediately when:

- transfer strength is dangerous;
- the frame turns people into obstacles, resources, defects, or terrain;
- authority assumptions are wrong or hidden;
- high-stakes use lacks plain-language fallback;
- the action cue is weaker than a direct instruction;
- no evidence boundary exists;
- every example fits because the frame is too vague;
- the frame increases confidence without improving evidence;
- role reviewers cannot name where the analogy stops.

## Acceptance Procedure

1. Confirm lifecycle state is draft or promoted draft heuristic.
2. Fill the review-input table.
3. Score with [fit-rubric.md](fit-rubric.md).
4. Apply relational-transfer review.
5. Apply audience-transfer review.
6. Run role-lens review.
7. Add at least two positive examples and one near-miss or hard-stop example.
8. Assign decision band.
9. Populate accepted catalog metadata if accepted.
10. Decide index behavior: default search, filtered search only, or docs only.
11. Record VTRACE requirement/spec/interface updates for API or catalog changes.

## Metadata Required For Acceptance

Accepted entries must provide:

| Field | Rule |
|---|---|
| `id` | Stable before index use. |
| `status` | `accepted` or `accepted_with_caveat` in docs; current Rust API uses `accepted`. |
| `claim_strength` | Usually `heuristic`; `empirically_validated` only with validation report. |
| `risk_band` | Low, medium, or high with display warning. |
| `application_packs` | At least one; exclusions are named when relevant. |
| `source_family` | Controlled taxonomy term. |
| `authority_model` | Peer, steward, operator, protected party, owner, command, reviewer, or mixed. |
| `transfer_strength` | Structural or partial; illustrative frames are not accepted as primary frames. |
| `evidence_boundary` | Non-empty and distinct from failure mode. |
| `misuse_warning` | Non-empty and human-safety aware. |
| `related` | At least one alternate or safer fallback when medium/high risk. |

## Accepted-Catalog Review Template

```text
Candidate:
Current lifecycle state:
Source:
Target:
Frame job:
Application pack:
Claim strength:
Risk band:
Transfer strength:
Fit score:
Hard stops checked:

Positive examples:
Near-miss or hard-stop example:

Role findings:
- Frame Fit:
- Evidence Boundary:
- Misuse Risk:
- Audience Transfer:
- Business Leader:
- Novice Reader:
- Journeyman Practitioner:
- Catalog Structure:

Decision:
Catalog metadata:
Index behavior:
Next review:
```

## Promotion From Local Imports

Promoted local imports from
[local-import-promotion-review.md](local-import-promotion-review.md) are not
accepted by default. They must pass this process after promotion.

Recommended first candidates remain:

1. Veto Rule.
2. Board Session.
3. Momentum State.
4. Portability Index.
5. Contact Structure.
6. Theme Swimlanes.

Theme Swimlanes additionally needs its dedicated role gates and pilot evidence
before accepted catalog use.

## Design Consequences

- Default Rust search should continue to include accepted starter entries only.
- Draft and held imports need lifecycle filtering before they can appear in
  tool-facing search.
- Frame anti-pattern taxonomy is defined in
  [frame-antipattern-taxonomy.md](frame-antipattern-taxonomy.md) and should be
  applied before rejected examples are expanded.
- Evaluation sets should follow
  [evaluation-set-design.md](evaluation-set-design.md) and include accepted,
  revise, hold, reject, near-miss, hard-stop, anti-pattern, and relation-behavior
  examples.
- Accepted-catalog review is the next gate before adding local imports to the
  catalog or index.
