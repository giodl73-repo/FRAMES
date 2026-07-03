# Plain-Language Fallbacks

Plain-language fallbacks are direct, non-metaphor statements used when a frame
would add confusion, risk, persuasion pressure, or false authority.

FRAMES should treat fallback language as first-class methodology, not as a
failure to find a better analogy.

## Fallback Rule

Use plain language first when:

- stakes are high or harm is possible;
- authority, ownership, or decision rights are unclear;
- evidence is missing or disputed;
- audience transfer is weak, unknown, or exclusionary;
- the frame would make a person or team into an obstacle;
- the frame would make a preference sound like a requirement;
- the frame is more memorable than the action, evidence, or accountability.

Use a frame second only if it helps memory or coordination after the direct
statement is clear.

## Fallback Shape

A good fallback names:

| Field | Question |
|---|---|
| Situation | What is happening without metaphor? |
| Decision or action | What needs to be decided, done, stopped, or checked? |
| Owner or authority | Who owns the next decision or evidence? |
| Evidence | What proof, threshold, or signal is still needed? |
| Boundary | What should not be inferred? |
| Revisit trigger | When should the decision be reviewed, waived, or changed? |

## Fallback Types

| Type | Use When | Template |
|---|---|---|
| Decision-rights fallback | Authority or veto language is unclear. | "Name the decision owner, tradeoff, and evidence before treating this as blocking." |
| Evidence fallback | Status or confidence is asserted without proof. | "Name the threshold and evidence before accepting the status." |
| Dependency fallback | People are framed as obstacles. | "The dependency is blocked by unresolved ownership or a decision path." |
| Protected-value fallback | Speed, efficiency, or control may harm safety, fairness, trust, or accountability. | "Move only inside explicit protected-value gates." |
| Audience-transfer fallback | The source scene may not be familiar. | "Use direct turn-order, owner, or priority language before the analogy." |
| Repair fallback | A story might erase harm after facts are known. | "Facts are now known; name harm, repair, and ownership." |
| Scope fallback | A frame is too broad or decorative. | "State the specific action, owner, and exclusion rule." |

## Examples

| Risky Frame Use | Fallback |
|---|---|
| "This is vetoed" without requirement, owner, or evidence. | "This is an unresolved preference or decision-rights issue." |
| "Green means done" without threshold. | "The status cannot be accepted until the threshold and evidence are named." |
| "That team is a roadblock." | "The dependency is blocked by unresolved ownership or a decision path." |
| "Run Fast" for risky customer migration. | "Move quickly only inside explicit customer-safety gates." |
| Bag-of-chips story after facts establish harm. | "Facts are now known; name harm, repair, and ownership." |
| Four-way stop for incident command. | "The incident owner should state the decision path and next action." |

## Selection Procedure

1. Identify whether the frame is helping action or adding risk.
2. If risk is high, unknown, contested, or authority-sensitive, write the
   fallback first.
3. Check that the fallback names owner, evidence, and boundary.
4. Decide whether a frame can appear as a secondary memory aid.
5. If the frame remains unsafe, record it as rejected or anti-pattern only.
6. Add fallback expectations to evaluation fixtures.

## Tool Behavior

AI and search outputs should:

- include `plain_language_fallback` for medium-risk frames;
- lead with fallback for high-risk, unknown-audience, authority-sensitive, or
  unsafe cases;
- suppress the frame when fallback is safer than analogy;
- explain whether the fallback is replacing the frame or accompanying it;
- never use fallback wording to smuggle in unsupported certainty.

## Evaluation Implications

Evaluation fixtures should fail when:

- a high-risk frame appears without fallback;
- a rejected frame appears as primary instead of fallback-only;
- fallback text omits owner, evidence, or boundary;
- fallback language uses metaphor again instead of direct language;
- fallback language blames people or hides decision rights.

## Design Consequences

- Fallbacks should remain stable enough to become reusable outputs for AI-agent
  contracts.
- Related-frame taxonomy should keep `plain_language_fallback` distinct from
  `safer_fallback`; the target is a direct sentence, not another frame.
- Catalog review should reject medium/high-risk entries that lack fallback.
- Rust search should not rank fallbacks as frames until output contracts support
  non-frame results.
