# Anti-Pattern Application: Veto Rule

This artifact applies `frame-antipattern-taxonomy.md` to the reviewed
docs-catalog candidate `Veto Rule`.

`Veto Rule` is useful only when a dimension is truly required. Its main risk is
that a preference, bias, or moving goalpost gets disguised as a non-negotiable
gate.

## Applied Anti-Pattern Checks

| Anti-Pattern | Applicability | Required Safeguard |
|---|---|---|
| False authority transfer | High. A stakeholder may claim veto power without owning the requirement. | Name the accountable owner and source of authority. |
| Evidence replacement | High. The word "veto" can replace proof that a requirement exists. | Name the requirement and evidence that it is required. |
| Coercive alignment | Medium. Calling something a veto can shut down dissent. | Allow challenge unless the requirement source is explicit. |
| Overconfidence amplifier | Medium. A crisp veto label can make the decision feel more certain than the evidence supports. | Pair the frame with clearance or waiver condition. |
| Category error | Medium. A weak metric, preference, or tradeoff can be mistaken for a hard gate. | Check required vs. preferred before applying the frame. |
| No stop condition | Controlled. The accepted-catalog review now defines stop conditions. | Stop when satisfied, waived, expired, or shown not required. |

## Rejected Use

| Scenario | Decision | Safer Output |
|---|---|---|
| A senior stakeholder dislikes an option and says it is a veto, but no policy, customer outcome, safety condition, or accountable owner is named. | Reject Veto Rule. | "This is an unresolved preference or decision-rights issue. Name the owner, tradeoff, and evidence before treating it as blocking." |
| A team keeps adding new required checks after each prior objection is cleared. | Hold or reject Veto Rule. | "The requirement boundary is moving. Freeze the acceptance criteria or escalate ownership." |
| A weak metric is below target but the owner explicitly accepts the risk. | Do not call it a veto. | "This is accepted risk with a monitor, not a blocking requirement." |

## Evaluation Fixture

`EVAL-VETO-003` should test the anti-pattern case where "veto" language is
tempting but unsupported by authority or evidence.

Expected decision:

- suppress `veto-rule` as primary;
- name false-authority or evidence-replacement risk;
- return plain-language decision-rights fallback.

## Index Implication

Do not add `Veto Rule` to default Rust search until rejected-candidate reporting
and lifecycle filtering can explain why false-veto cases are suppressed.

