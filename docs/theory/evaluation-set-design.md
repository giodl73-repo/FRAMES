# Evaluation-Set Design

This design defines how FRAMES should build evaluation sets before adding
semantic search, typed related-frame metadata, or broad catalog expansion. An
evaluation set is not empirical validation with human subjects. It is a curated
fixture set for checking whether frame selection, rejection, fallback, and
explanation behavior matches the theory.

Evaluation fixtures should be small, reviewed, and traceable. A large noisy set
is worse than a small set with clear expected behavior.

## Evaluation Jobs

An evaluation set should test five jobs:

| Job | Question |
|---|---|
| Select | Does the system suggest an acceptable frame for the target situation? |
| Suppress | Does the system avoid a tempting but unsafe or misleading frame? |
| Explain | Does the output name evidence boundaries, misuse warnings, and claim strength? |
| Relate | Does the system distinguish alternates, safer fallbacks, conflicts, sequences, and rejected near-misses? |
| Fallback | Does the system choose plain language when a frame would make the situation worse? |

## Fixture Types

| Fixture Type | Purpose | Expected Output |
|---|---|---|
| Positive | Known-good target situation for an accepted frame. | Recommend expected frame or accepted alternate with action cue and warning. |
| Near-miss | Situation resembles a frame but misses authority, audience, timing, or protected-value fit. | Do not recommend as primary; explain boundary or offer safer alternate. |
| Hard stop | Situation violates a hard-stop rule. | Reject or hold; return fallback or warning. |
| Anti-pattern | Tempting bad frame matches surface language or story appeal. | Suppress by default; name anti-pattern class when explanation is requested. |
| Relation behavior | Related frames require alternate, fallback, conflict, sequence, or rejected-near-miss behavior. | Display, suppress, sequence, or warn according to relation type. |
| Evidence boundary | Frame is plausible but key evidence is missing. | Recommend only with explicit evidence boundary or fallback. |
| Audience transfer | Source scene may not transfer to intended audience. | Prefer broader variant, alternate, or plain language. |

## Fixture Shape

```text
id:
title:
fixture_type:
target_situation:
audience:
application_pack:
risk_band:
frame_job:
relation_term:
authority_term:
expected_primary:
acceptable_alternates:
must_not_return:
expected_relation_behavior:
expected_decision:
expected_warnings:
evidence_boundary:
plain_language_fallback:
source_docs:
review_status:
```

## Required Fields

| Field | Meaning |
|---|---|
| `id` | Stable fixture ID such as `EVAL-TRAFFIC-001`. |
| `fixture_type` | Positive, near-miss, hard-stop, anti-pattern, relation behavior, evidence boundary, or audience transfer. |
| `target_situation` | The input scenario given to a tool or reviewer. |
| `audience` | Intended reader or user context. |
| `frame_job` | Controlled job from `frame-ontology.md`. |
| `relation_term` | Controlled relation term from `frame-ontology.md`. |
| `authority_term` | Controlled authority term from `frame-ontology.md`. |
| `expected_primary` | Frame ID expected as first choice, or `none` when no frame should be primary. |
| `acceptable_alternates` | Accepted frame IDs that can pass when justified. |
| `must_not_return` | Frame IDs, draft frames, anti-patterns, or source patterns that should be suppressed. |
| `expected_relation_behavior` | Relation behavior such as alternate, safer fallback, conflict, sequence, or rejected near-miss. |
| `expected_decision` | Recommend, recommend-with-warning, fallback-only, reject, hold, or explain-rejection. |
| `expected_warnings` | Evidence boundary, misuse warning, authority warning, audience warning, or protected-value warning. |
| `plain_language_fallback` | Direct sentence expected when analogy should not lead. |
| `source_docs` | Theory docs, catalog rows, or review docs that justify the expectation. |
| `review_status` | Draft, role-reviewed, accepted-fixture, deprecated, or rejected-fixture. |

## Starter Fixture Backlog

The first machine-readable fixture package lives in
[../eval/starter-fixtures.json](../eval/starter-fixtures.json). The table below
is the readable backlog view of that package.

| ID | Fixture Type | Ontology Terms | Target Situation | Expected Behavior |
|---|---|---|---|---|
| EVAL-TRAFFIC-001 | Positive | coordination / peer_turn_taking / peer | Peer teams need turn order around constrained attention. | Recommend `four-way-stop`; show authority boundary. |
| EVAL-TRAFFIC-002 | Hard stop | coordination / peer_turn_taking / owner | One party owns an incident response decision. | Suppress `four-way-stop`; prefer command or plain language. |
| EVAL-TRAFFIC-003 | Relation behavior | coordination / peer_turn_taking / mixed | Peer turn-taking and incident command both match words in the query. | Mark conflict; do not blend. |
| EVAL-RISK-001 | Evidence boundary | status / threshold_signal / operator | Status is called green but no threshold is named. | Require threshold evidence; do not accept green as proof. |
| EVAL-VETO-001 | Positive | risk / required_gate / reviewer | Launch scores are strong overall, but privacy approval is missing. | Recommend `veto-rule` as docs-catalog candidate; require owner and clearance condition. |
| EVAL-VETO-002 | Near-miss | priority / required_gate / owner | A stakeholder dislikes a design but no requirement, policy, safety, or customer outcome is blocked. | Do not call it a veto; use preference, tradeoff, or decision-owner language. |
| EVAL-VETO-003 | Anti-pattern | risk / required_gate / mixed | A senior stakeholder says an option is vetoed but names no requirement, evidence, or accountable owner. | Suppress `veto-rule`; warn false authority transfer and evidence replacement. |
| EVAL-ANTI-001 | Anti-pattern | coordination / dependency_integrity / owner | A query says another team is a roadblock. | Suppress people-as-obstacles; return dependency/ownership fallback. |
| EVAL-THEME-001 | Positive | priority / route_adjustment / steward | Program leader wants three contribution lanes under one customer promise. | Recommend theme swimlanes only as draft/pilot unless accepted-catalog review passes. |
| EVAL-THEME-002 | Near-miss | priority / route_adjustment / mixed | Three slogans exist but no owner, measure, or customer promise exists. | Hold or fallback; warn about slogan compression. |
| EVAL-STORY-001 | Anti-pattern | trust / perspective_repair / peer | Bag-of-chips story is used after facts establish harm. | Warn empathy eraser; require repair and ownership language. |
| EVAL-REL-001 | Relation behavior | risk / pace_adjustment / steward | Run Fast is suggested for a risky customer migration. | Prefer `safer_fallback` to Run Fast / Run Safe or plain language. |
| EVAL-REL-002 | Relation behavior | coordination / peer_turn_taking / protected_party | Four-way stop is suggested where one party has protected-party duty. | Treat `crosswalk-yield` as `boundary_frame`; do not present it as a peer alternate. |
| EVAL-REL-003 | Relation behavior | coordination / flow_joining / operator | A team wants to merge work into an active system with tight coupling. | Sequence `following-distance` before `merge-lane`; require buffer evidence. |
| EVAL-REL-004 | Relation behavior | risk / required_gate / mixed | Veto Rule is suggested but requirement authority and evidence are missing. | Use `plain_language_fallback`; suppress unsupported `veto-rule`. |
| EVAL-AUD-001 | Audience transfer | learning / route_adjustment / unknown | Driving frame for non-driver or non-US audience. | Prefer queue/walking/plain-language alternate. |

## Scoring Dimensions

Evaluation results should score behavior, not just frame ID match:

| Dimension | Pass Criteria |
|---|---|
| Primary selection | Expected primary frame appears first, or accepted alternate is justified. |
| Suppression | Must-not-return frames are absent from default output. |
| Warning quality | Required warnings are visible and specific. |
| Evidence boundary | Output names what must still be checked outside the frame. |
| Relation behavior | Alternates, fallbacks, conflicts, sequences, and rejected near-misses are handled differently. |
| Fallback behavior | Plain language appears when frame risk is high or transfer is weak. |
| Claim boundary | Output does not imply empirical validation unless evidence exists. |

## Pass Bands

| Band | Meaning |
|---|---|
| Pass | All must-pass checks succeed; minor wording differences are acceptable. |
| Pass with caveat | Expected behavior succeeds, but warning or rationale is incomplete. |
| Fail | Wrong primary, missing hard-stop suppression, missing fallback, or overclaimed evidence. |
| Invalid fixture | Fixture expectation is ambiguous, stale, or not justified by source docs. |

## Review Procedure

1. Draft fixture from an accepted catalog entry, reviewed theory example, local
   promoted heuristic, or known anti-pattern.
2. Link source docs that justify the expected behavior.
3. Assign fixture type and expected decision.
4. Add must-not-return frames and required warnings.
5. Review through Frame Fit, Evidence Boundary, Misuse Risk, Audience Transfer,
   Business Leader, and Catalog Structure lenses.
6. Mark fixture `accepted-fixture` only when expectations are unambiguous.

## What Not To Evaluate Yet

- Do not benchmark semantic search quality before fixture expectations exist.
- Do not compare LLM creativity or prose style as a primary score.
- Do not treat draft local imports as accepted expected-primary frames.
- Do not use empirical validation trial results as automatic fixture truth.
- Do not encode every fixture field in Rust before the docs-level shape proves
  stable.

## AI And Tool Implications

- Future tool tests should check selected frame IDs, suppressed frames, warning
  fields, evidence boundary, fallback text, and relation behavior separately.
- Search score should not be treated as fit score in evaluation reports.
- Rejected-candidate reporting should be evaluated only in opt-in modes until
  default search has lifecycle and rejection filters.
- Evaluation fixtures should become the gate before semantic search, typed
  related-frame metadata, or draft-frame inclusion.

## Design Consequences

- `frames-core` can keep deterministic lexical search while evaluation fixtures
  are defined.
- A future `docs/eval/` package should hold machine-readable fixture files after
  this docs-level design stabilizes.
- Accepted-catalog review should contribute positive, near-miss, and hard-stop
  fixtures for each accepted frame.
- Anti-pattern and related-frame taxonomy examples should be promoted into
  fixtures before broader catalog growth.
