# Frame Anti-Pattern Taxonomy

This taxonomy defines reusable failure classes for frames that should be
revised, held, rejected, or kept only as cautionary examples. It is not a list
of every weak metaphor. It names the common ways a familiar source scene can
make a target situation less accurate, less humane, or less actionable.

Anti-pattern examples do not belong in default search results unless the caller
explicitly asks for rejected comparisons or misuse examples.

## Detection Rule

A frame is an anti-pattern when it does one or more of the following:

- causes the wrong action;
- hides or weakens evidence obligations;
- imports the wrong authority model;
- turns people into obstacles, defects, resources, or terrain;
- inflates confidence without improving understanding;
- makes plain language less clear.

When a direct sentence is safer and clearer than the frame, use the direct
sentence.

## Taxonomy

| Anti-Pattern | Failure | Detection Question | Default Disposition | Example |
|---|---|---|---|---|
| People as obstacles | Treats a person or team as traffic, friction, terrain, waste, or a defect. | Who becomes an object instead of an accountable actor or stakeholder? | Reject | "That team is the roadblock." |
| False authority transfer | Imports command, ownership, or turn-taking assumptions that do not match the target situation. | Does the source scene give someone authority they do not have? | Hold or reject | Treating a peer dependency like a traffic cop command. |
| Evidence replacement | Lets the frame label stand in for measurement, threshold, or factual review. | What proof would still be required if the metaphor were removed? | Revise or reject | "Green means fine" without a threshold. |
| Slogan compression | Makes a phrase memorable while hiding action, tradeoff, owner, or measure. | Can a team act differently from this without extra explanation? | Revise or hold | "Run Fast" without speed, safety, or quality gates. |
| Blame laundering | Converts a system, process, or visibility failure into individual fault. | Does the frame make one actor carry a shared-system problem? | Reject or revise | "They missed the blind spot" when the process hid the signal. |
| Protected-value inversion | Lets efficiency, speed, or convenience override safety, fairness, trust, dignity, or customer harm. | Which protected value is being traded away without review? | Reject | Using "move fast" to justify an unsafe migration. |
| Overconfidence amplifier | A vivid story makes the audience more certain than the evidence allows. | Does the frame increase confidence more than it increases evidence recall? | Hold or reject | A memorable incident treated as proof of a general rule. |
| Decorative metaphor | Adds imagery without improving selection, action, or evidence. | What decision changes because of this frame? | Reject or rewrite | Calling a backlog a "garden" without any pruning rule. |
| Category error | Uses the wrong source structure for the target relation, stakes, or reversibility. | Is this a queue, negotiation, triage, race, care duty, or command problem? | Hold or reject | Treating irreversible harm as a game move. |
| Coercive alignment | Pressures agreement or compliance instead of clarifying the decision. | Does dissent become disloyalty inside the frame? | Reject | "Get on the bus" for unresolved ethical or customer-risk concerns. |
| Empathy eraser | Uses a perspective story to erase established harm, duty, or accountability. | Are we asking for curiosity where repair or ownership is already due? | Hold or revise | A bag-of-chips story used after facts already show harm. |
| Universality claim | Assumes an everyday source is familiar, safe, or culturally universal for all audiences. | Who might not share this source script or might read it differently? | Revise or hold | A driving frame for an audience without driving familiarity. |
| No stop condition | Cannot say when the analogy no longer applies. | Where does the frame stop being useful? | Revise | A "marathon" frame used for every prolonged effort. |
| Rejected-candidate trap | A tool returns a tempting frame because lexical match beats fit, authority, or safety. | Why did this bad frame look searchable? | Reject and demote | Returning "roadblock" because a query mentions a blocked team. |

## Review Procedure

1. Name the target action the frame is supposed to improve.
2. Apply the hard stops in
   [accepted-catalog-review-process.md](accepted-catalog-review-process.md).
3. Identify any anti-pattern class from this taxonomy.
4. Decide whether to rewrite, revise, hold, reject, or keep only as an
   anti-pattern example.
5. Add a plain-language fallback.
6. Record the evidence boundary and at least one safer alternate when the frame
   is held or rejected.

## Dispositions

| Disposition | Use When |
|---|---|
| Rewrite as plain language | The frame adds risk or confusion without improving action. |
| Revise frame | The source is useful, but the action cue, evidence boundary, or stop condition is incomplete. |
| Hold | The frame may work in a narrower audience or target, but authority, safety, or evidence risk is unresolved. |
| Reject | The frame is misleading, unsafe, coercive, dehumanizing, or lower value than plain language. |
| Use as anti-pattern example only | The failed frame is useful for teaching rejection or evaluation behavior. |

## Evaluation-Set Implications

Evaluation sets should include positive, near-miss, hard-stop, and anti-pattern
fixtures. Each anti-pattern fixture should record:

| Field | Meaning |
|---|---|
| Scenario | Target situation the tool or reviewer receives. |
| Tempting bad frame | The frame that may match lexically or narratively. |
| Anti-pattern class | One or more classes from this taxonomy. |
| Safer frame or fallback | The accepted alternate or direct sentence. |
| Expected decision | Revise, hold, reject, or anti-pattern-only. |
| Evidence boundary | What still has to be checked outside the frame. |

## AI And Tool Implications

- Default search should not return rejected anti-patterns.
- Rejected-candidate reporting should explain why a tempting frame was not
  recommended.
- High-risk, unknown-authority, or protected-value cases should include a
  plain-language fallback even when a frame is suggested.
- Anti-pattern metadata should remain docs-level until lifecycle filters and
  rejected-candidate reporting exist in `frames-core`.

## Examples

| Candidate | Anti-Pattern | Decision | Safer Output |
|---|---|---|---|
| "That team is a roadblock." | People as obstacles; blame laundering. | Reject. | "The dependency is blocked by an unresolved ownership or decision path." |
| "Green means done." | Evidence replacement. | Revise. | "Green means the agreed threshold has been met; list the threshold." |
| "Run Fast" for a risky customer migration. | Protected-value inversion unless paired with safety gates. | Hold or revise. | Use "Run Fast / Run Safe" with explicit customer-risk checks. |
| Unsupported Veto Rule. | False authority transfer; evidence replacement. | Reject or hold. | Name the requirement, accountable owner, and evidence before treating it as blocking. |
| Bag-of-chips conflict story after facts are known. | Empathy eraser. | Revise. | Use the story only when facts are incomplete; once harm is established, name repair and ownership. |

## Applied Reviews

- [frame-antipattern-application-veto-rule.md](frame-antipattern-application-veto-rule.md)
  applies the taxonomy to Veto Rule and adds the `EVAL-VETO-003` anti-pattern
  fixture.

## Design Consequences

- Accepted-catalog review must apply this taxonomy before accepting or rejecting
  promoted draft heuristics.
- Evaluation-set design should include anti-pattern fixtures from this taxonomy.
- Related-frame relation taxonomy should include safer fallback, conflict, and
  rejected near-miss relation types.
- Future AI response contracts should be able to expose rejected-candidate
  warnings without making anti-patterns default recommendations.
