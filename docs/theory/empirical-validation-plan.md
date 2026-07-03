# Empirical Validation Plan

Empirical validation is the process for testing whether a frame helps a defined
audience perform a defined task better than a plain explanation or alternate
frame. It is not a license to claim universal persuasion or universal
understanding.

FRAMES should validate narrow effects:

- comprehension,
- decision quality,
- evidence recall,
- misuse detection,
- action selection,
- confidence calibration,
- audience portability.

## Validation Boundary

A frame can be `empirically_validated` only for the tested audience, context,
task, comparison, and measured outcome. It remains weaker everywhere else.

Example:

```text
Validated claim: In a product-lead workshop, the four-way stop frame improved
turn-order action selection compared with an unframed paragraph.

Not validated: Four-way stop improves coordination for all teams.
```

## Study Levels

| Level | Use When | Method | Claim Upgrade |
|---|---|---|---|
| V0 inspection | A frame is being drafted. | Fit rubric, role review, evidence-boundary review. | No empirical upgrade. |
| V1 comprehension check | Need to know if the audience understands the frame. | Ask readers to restate source, target, action cue, and boundary. | May support `heuristic`. |
| V2 comparison task | Need to compare frame vs. plain explanation or alternate frame. | Randomized or counterbalanced task with scored outputs. | May support narrow `empirically_validated`. |
| V3 field trial | Need to see if the frame works in real workflow use. | Observe meeting, planning, support, or decision workflow outcomes. | May support local operational validation. |
| V4 longitudinal review | Need to know whether the frame keeps working over time. | Repeated use, misuse tracking, retirement checks. | May support lifecycle retention. |

## Measurement Targets

| Target | Question | Example Measure |
|---|---|---|
| Recognition | Does the audience understand the source scene? | Correct source-scene restatement. |
| Transfer clarity | Can the audience name what transfers and what does not? | Source-target mapping score. |
| Action selection | Does the frame improve next-step choice? | Scored action choice against rubric. |
| Evidence recall | Does the audience remember what still must be checked? | Boundary recall after delay. |
| Misuse detection | Can the audience detect where the frame fails? | False-use rejection score. |
| Decision quality | Does the frame improve a decision under constraint? | Role-scored decision output. |
| Confidence calibration | Does the frame reduce overconfidence? | Confidence vs. correctness gap. |
| Portability | Does the frame work for another audience? | Same task across roles or regions. |

## Minimal Validation Protocol

Use this protocol before upgrading a frame to `empirically_validated`.

1. Name the frame ID or candidate name.
2. Name the audience and context.
3. Name the task the frame should improve.
4. Define a comparison: plain explanation, alternate frame, or no frame.
5. Define the scoring rubric before collecting responses.
6. Collect enough responses to inspect variance and failure patterns.
7. Record exclusions, confusion, and adverse effects.
8. Publish only the narrow validated claim.
9. Keep the claim-strength label weaker for untested audiences.

## Test Templates

### Comprehension Test

Prompt:

```text
Read the frame. In your own words:
1. What is the source scene?
2. What is the target situation?
3. What action should someone take?
4. What still needs evidence?
5. When would this frame be misleading?
```

Pass rule: most intended readers correctly restate source, target, action, and
evidence boundary without importing a hard-stop misuse.

### Decision Comparison

Prompt:

```text
You are given a short scenario. One group receives a plain explanation. Another
group receives the same explanation plus the frame. Choose the next action and
explain what evidence you would check.
```

Score:

- action selection,
- evidence boundary,
- authority awareness,
- misuse avoidance,
- confidence calibration.

### Misuse Probe

Prompt:

```text
Here are three situations. In which one should this frame not be used, and why?
```

Pass rule: readers reject cases with wrong authority, missing evidence,
dehumanizing transfer, or high-stakes overclaim.

## Candidate Validation Backlog

| Candidate | First Test | Reason |
|---|---|---|
| Red / yellow / green | Evidence recall and threshold clarity. | Common but easy to overclaim. |
| Four-way stop | Action selection for peer coordination. | Strong structural frame with authority failure mode. |
| Theme swimlanes | Lane assignment and exclusion power. | Local pattern needs proof that lanes clarify contribution. |
| Veto Rule | Blocker identification vs. averaging. | Broadly reusable risk/priority candidate. |
| Mentor vs. Sponsor | Evidence-boundary recognition. | Tests whether support and advocacy stay distinct. |
| Phantom Dissonance | Misuse probe for conflict repair. | High empathy value but high accountability risk. |

## Reporting Shape

The first concrete protocol is
[empirical-validation-trial-001-theme-swimlanes.md](empirical-validation-trial-001-theme-swimlanes.md).
It is not yet executed and does not upgrade claim strength.

Every validation report should use this shape:

```text
Frame:
Audience:
Context:
Task:
Comparison:
Measure:
Result:
Observed failures:
Claim strength after test:
Boundary:
Next review:
```

## Hard Stops

Do not call a frame empirically validated when:

- there is no comparison condition;
- the task is vague or scored after the fact;
- only author confidence or participant preference was measured;
- adverse effects were not inspected;
- the claim is broader than the tested audience or context;
- high-stakes decisions were influenced without plain-language fallback.

## Design Consequences

- `empirically_validated` should remain rare and narrow.
- AI-agent outputs should cite validation scope, not just validation status.
- Future catalog metadata should separate `claim_strength` from
  `validation_scope`.
- Lifecycle review should downgrade frames when validated effects fail to
  transfer to a new audience or context.
