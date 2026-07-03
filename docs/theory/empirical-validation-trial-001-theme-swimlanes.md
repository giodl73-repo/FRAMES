# Empirical Validation Trial 001: Theme Swimlanes

This artifact defines the first FRAMES empirical validation trial. It is a
trial protocol, not a completed study. No claim is upgraded until responses are
collected, scored, and reviewed against the reporting shape below.

## Trial Summary

| Field | Value |
|---|---|
| Trial ID | EVT-001 |
| Candidate | Theme Swimlanes |
| Source docs | `theme-swimlane-extraction.md`, `theme-swimlane-role-review.md` |
| Study level | V2 comparison task, with V1 comprehension checks embedded. |
| Audience | Product, program, operations, or engineering leaders who participate in planning. |
| Context | Multi-team planning for one customer or operating promise. |
| Task | Assign work to lanes, reject work that does not fit, and explain evidence boundaries. |
| Comparison | Plain priority-list explanation vs. theme-swimlane explanation. |
| Current result | Not run. |
| Claim strength after protocol | Still `heuristic`; no empirical upgrade. |

## Hypothesis

Theme swimlanes improve program-planning review only if they help participants:

- assign work to distinct contribution lanes;
- reject attractive but off-promise work;
- name owner, measure, tradeoff, and exclusion evidence;
- avoid using lane labels as slogans.

The trial should not test whether participants like the theme names. Preference
is not enough for a claim-strength upgrade.

## Scenario Material

Participants receive this planning scenario:

```text
A customer platform program promises to help enterprise customers operate with
more control, more visibility, and less effort. Six teams propose work for the
next planning cycle:

1. Add admin policy controls for risky configuration changes.
2. Add a dashboard showing request status, blocked steps, and owner.
3. Automate duplicate data entry between two internal systems.
4. Add a new executive report that no customer sees.
5. Add ten advanced settings requested by one internal power user.
6. Replace a manual approval with an automated path that has no rollback.
```

Condition A receives the scenario plus a plain instruction:

```text
Choose which proposed work should be prioritized for the customer platform
program. Explain why and name what evidence you would check.
```

Condition B receives the scenario plus the theme-swimlane instruction:

```text
Use the three lanes more control, more visibility, and less effort. For each
proposed work item, choose a lane or reject it. Explain the customer outcome,
the evidence to check, and any tradeoff or exclusion.
```

## Response Form

Each participant answers:

```text
Participant role:
Condition:

For each work item:
Decision: prioritize / defer / reject / needs revision
Lane, if any:
Customer outcome:
Measure or evidence:
Tradeoff:
Reason:

Which item is most likely to misuse the frame?
What would you say no to?
Confidence: 1-5
```

## Scoring Rubric

Score each response before viewing the condition label when possible.

| Dimension | 0 | 1 | 2 |
|---|---|---|---|
| Lane assignment | No clear lane logic. | Some correct lane choices, with overlap or weak reasons. | Work is assigned or rejected using distinct lane logic. |
| Exclusion power | Accepts nearly all work. | Rejects some weak work but misses a major off-promise item. | Rejects or revises work that lacks customer outcome, evidence, or safety. |
| Evidence boundary | Gives opinions or preferences only. | Names some measures but misses tradeoffs or owner evidence. | Names customer outcome, measure, tradeoff, and uncertainty. |
| Misuse detection | Does not see frame misuse. | Names a generic risk. | Identifies slogan use, hidden cuts, displaced effort, unusable control, or unsafe automation. |
| Decision quality | Priorities are ungrounded. | Priorities are partly grounded. | Priorities are defensible under the stated promise and constraints. |
| Confidence calibration | High confidence with weak evidence. | Confidence roughly tracks partial evidence. | Confidence is bounded by explicit evidence gaps. |

Maximum score: 12.

## Analysis Plan

Compare Condition A and Condition B on:

- total score;
- exclusion-power score;
- evidence-boundary score;
- false acceptance of off-promise work;
- false confidence in weakly evidenced work;
- observed misuse language.

Condition B supports a narrow empirical claim only if it improves exclusion
power and evidence-boundary scores without increasing false confidence or
coercive/slogan language.

## Report Template

```text
Frame: Theme Swimlanes
Audience:
Context:
Task:
Comparison:
Sample size:
Measure:
Result:
Observed failures:
Adverse effects:
Claim strength after test:
Boundary:
Next review:
```

## Hard Stops

Do not upgrade the claim if:

- Condition B improves preference but not scored decisions;
- lane labels cause participants to accept all proposed work;
- participants over-prioritize unsafe automation because it sounds like less
  effort;
- participants use the frame to blame teams for being outside a lane;
- the scoring rubric is changed after responses are collected;
- the reported claim is broader than the tested planning scenario and audience.

## Expected Use After Running

If the trial passes, theme swimlanes may gain a narrow validation note such as:

```text
In EVT-001, for planning-oriented product/program participants, the theme
swimlane condition improved lane exclusion and evidence-boundary recall on a
synthetic customer-platform planning task compared with a plain priority-list
instruction.
```

If the trial fails, keep the pattern as a draft heuristic and update the role
review with the observed failure mode.
