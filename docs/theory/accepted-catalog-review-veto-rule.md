# Accepted-Catalog Review: Veto Rule

This review applies `accepted-catalog-review-process.md` to the promoted draft
heuristic `Veto Rule`. It does not add the frame to the accepted starter catalog
or Rust index.

## Review Summary

| Field | Value |
|---|---|
| Candidate | Veto Rule |
| Current lifecycle state | promoted draft heuristic |
| Source | `career-gravity-frame-imports.md`, `career-gravity-import-map.md` |
| Frame job | risk / priority |
| Ontology relation | required_gate |
| Authority term | reviewer |
| Application pack | product, operations, leadership, AI-agent |
| Claim strength | heuristic |
| Risk band | medium |
| Transfer strength | structural when a dimension is truly required; partial otherwise |
| Decision | Revise |
| Index behavior | Docs only; do not include in default Rust search |

## Frame Shape

| Field | Review Entry |
|---|---|
| Source scene | A gate with multiple locks where one locked point blocks passage. |
| Target situation | A decision, readiness model, launch, migration, or review with several required dimensions where one critical blocker dominates averages. |
| Useful mapping | Strong dimensions do not compensate for a truly required locked dimension. |
| Action cue | Find the binding lock before optimizing high scores. |
| Evidence boundary | Verify the dimension is truly required, not merely preferred, politically salient, or movable. |
| Misuse warning | Can overstate a blocker when the system actually allows tradeoffs or compensating strengths. |
| Better fit than | Average-score dashboards when one threshold is non-negotiable. |

## Examples

Positive examples:

| Situation | Expected Use |
|---|---|
| Product launch has great adoption signals, but privacy review is not passed. | Treat privacy approval as a required lock before launch. |
| Migration plan is fast and cheap, but rollback is impossible. | Treat rollback as a binding launch condition unless risk is explicitly accepted by the owner. |

Near-miss / hard-stop example:

| Situation | Expected Decision |
|---|---|
| A stakeholder dislikes the design but no requirement, policy, safety, or customer outcome is blocked. | Do not call it a veto. Name preference, tradeoff, or decision ownership directly. |

## Fit Score

| Dimension | Score | Rationale |
|---|---:|---|
| Recognition | 1 | Multi-lock gate is understandable but may need setup. |
| Target specificity | 2 | Required-dimension decisions are concrete enough for action. |
| Transfer clarity | 2 | Required lock maps cleanly to binding decision condition. |
| Action cue | 2 | Find the binding lock before optimizing other dimensions. |
| Evidence boundary | 2 | Required-vs-preferred check is explicit. |
| Human safety | 1 | Can be misused to launder bias, moving goalposts, or hidden authority. |
| Stop condition | 1 | Needs clearer stop when the blocker becomes negotiable, expired, or owner-accepted. |

Total: 11. Decision band: revise.

## Role Findings

| Lens | Finding |
|---|---|
| Frame Fit Reviewer | Pass with gate: useful for required-dimension decisions, but relation must be limited to true hard requirements. |
| Evidence Boundary Reviewer | Pass: required-vs-preferred evidence boundary is strong and testable. |
| Misuse Risk Reviewer | Revise: add bias, moving-goalpost, and false-veto safeguards before catalog acceptance. |
| Audience Transfer Reviewer | Pass with setup: gate/lock source is broadly understandable but not universal. |
| Business Leader | Pass: clarifies why averages can hide launch, policy, safety, or customer-risk blockers. |
| Novice Reader | Revise: needs a short definition of "binding lock" and examples before use. |
| Journeyman Practitioner | Pass with gate: useful in planning and review meetings if owner and requirement are named. |
| Catalog Structure Reviewer | Revise: add evaluation fixtures and accepted-catalog metadata before default search. |

## Decision

Decision: revise.

Required revision before accepted catalog:

1. Add an evaluation fixture where `Veto Rule` should be recommended.
2. Add a near-miss fixture where a preference is falsely called a veto.
3. Add a stop condition: the frame ends when the requirement is satisfied,
   waived by the accountable owner, expires, or is shown to be preferred rather
   than required.
4. Add a plain-language fallback for high-stakes or contested authority use.

Catalog metadata if revised successfully:

| Field | Candidate Value |
|---|---|
| `id` | `veto-rule` |
| `status` | accepted with caveat |
| `claim_strength` | heuristic |
| `risk_band` | medium |
| `application_packs` | product, operations, leadership, AI-agent |
| `source_family` | gates and locks |
| `authority_term` | reviewer |
| `relation_term` | required_gate |
| `transfer_strength` | structural for true required dimensions |

Next review: after fixtures and stop condition are added.
