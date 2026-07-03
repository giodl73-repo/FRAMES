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
| Decision | Accepted with caveat for docs catalog; not starter catalog or default Rust search |
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
| AI agent output passes style and speed checks, but fails a required safety policy. | Treat policy failure as a required gate; do not average it away with strong non-safety scores. |

Near-miss / hard-stop example:

| Situation | Expected Decision |
|---|---|
| A stakeholder dislikes the design but no requirement, policy, safety, or customer outcome is blocked. | Do not call it a veto. Name preference, tradeoff, or decision ownership directly. |
| One metric is weak but the accountable owner explicitly accepts the tradeoff and no hard requirement is violated. | Do not call it a veto; record accepted risk and monitor the metric. |

## Stop Condition And Fallback

Stop using Veto Rule when:

- the required dimension is satisfied;
- the accountable owner explicitly waives the requirement;
- the requirement expires or no longer applies;
- evidence shows the dimension was preferred, not required;
- the frame is being used to hide bias, moving goalposts, or unowned authority.

Plain-language fallback for high-stakes or contested authority use:

```text
This decision has a possible blocking requirement. Before calling it a veto,
name the requirement, the accountable owner, the evidence that it is required,
and the condition that would clear or waive it.
```

## Fit Score

| Dimension | Score | Rationale |
|---|---:|---|
| Recognition | 1 | Multi-lock gate is understandable but may need setup. |
| Target specificity | 2 | Required-dimension decisions are concrete enough for action. |
| Transfer clarity | 2 | Required lock maps cleanly to binding decision condition. |
| Action cue | 2 | Find the binding lock before optimizing other dimensions. |
| Evidence boundary | 2 | Required-vs-preferred check is explicit. |
| Human safety | 1 | Can be misused to launder bias, moving goalposts, or hidden authority. |
| Stop condition | 2 | Stop, waiver, expiry, and preferred-vs-required boundaries are explicit. |

Total: 12. Decision band: accepted with caveat.

## Role Findings

| Lens | Finding |
|---|---|
| Frame Fit Reviewer | Pass with gate: useful for required-dimension decisions, but relation must be limited to true hard requirements. |
| Evidence Boundary Reviewer | Pass: required-vs-preferred evidence boundary is strong and testable. |
| Misuse Risk Reviewer | Pass with caveat: bias, moving-goalpost, and false-veto safeguards must display with the frame. |
| Audience Transfer Reviewer | Pass with setup: gate/lock source is broadly understandable but not universal. |
| Business Leader | Pass: clarifies why averages can hide launch, policy, safety, or customer-risk blockers. |
| Novice Reader | Pass with setup: define binding lock as a required condition that blocks progress until cleared or waived. |
| Journeyman Practitioner | Pass with gate: useful in planning and review meetings if owner and requirement are named. |
| Catalog Structure Reviewer | Pass with caveat: docs catalog candidate is ready after fixtures; Rust default search waits for lifecycle filtering. |

## Decision

Decision: accepted with caveat for docs catalog. Do not add to accepted starter
catalog or default Rust search until lifecycle filtering and draft/local import
indexing rules exist.

Revision items closed:

1. Added evaluation fixture where `Veto Rule` should be recommended:
   `EVAL-VETO-001`.
2. Added near-miss fixture where a preference is falsely called a veto:
   `EVAL-VETO-002`.
3. Added stop condition for satisfied, waived, expired, or non-required
   dimensions.
4. Added plain-language fallback for high-stakes or contested authority use.

Catalog metadata for future docs row:

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

Next review: before adding to accepted starter catalog or Rust index.

Anti-pattern application:

- [frame-antipattern-application-veto-rule.md](frame-antipattern-application-veto-rule.md)
  records false-authority, evidence-replacement, coercive-alignment, and
  category-error checks for Veto Rule.
