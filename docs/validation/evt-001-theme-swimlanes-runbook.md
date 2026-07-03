# EVT-001 Theme Swimlanes Runbook

This runbook executes the protocol in
`docs/theory/empirical-validation-trial-001-theme-swimlanes.md` without changing
the trial after responses are collected.

## Execution Status

| Field | Value |
|---|---|
| Trial ID | EVT-001 |
| Protocol status | Locked for first pilot run |
| Response status | No participant responses collected |
| Scoring status | Rubric locked before collection |
| Claim strength | Still `heuristic`; no empirical upgrade |

## Participant Screen

Use participants who normally take part in product, program, operations, or
engineering planning. Do not include people who helped write the protocol or
the scoring rubric.

Record only role-level information:

- product leader,
- program leader,
- operations leader,
- engineering leader,
- other planning participant.

Do not record names in the public report.

## Assignment

Assign each response to one condition before scoring:

| Condition | Prompt |
|---|---|
| A | Plain priority-list explanation. |
| B | Theme-swimlane explanation with more control, more visibility, and less effort. |

For the first pilot, alternate conditions by response order:

1. A
2. B
3. B
4. A
5. A
6. B
7. B
8. A

If fewer than eight responses are collected, report the smaller sample plainly.
If more are collected, repeat the same assignment block.

## Collection Procedure

1. Give each participant only their assigned prompt from the protocol.
2. Ask them to complete the response form without discussion.
3. Save each answer as `EVT-001-PXX-condition-A|B`.
4. Remove names or identifying details before scoring.
5. Score each response against the locked rubric.
6. Record observed failures and adverse effects before calculating averages.
7. Update `evt-001-theme-swimlanes-results.md`.

## Scoring Lock

Score each response on the six dimensions from the protocol:

| Dimension | Range |
|---|---|
| Lane assignment | 0-2 |
| Exclusion power | 0-2 |
| Evidence boundary | 0-2 |
| Misuse detection | 0-2 |
| Decision quality | 0-2 |
| Confidence calibration | 0-2 |

Maximum score: 12.

Do not add, remove, or redefine dimensions after collection starts. If the
rubric fails, record that as a trial limitation instead of repairing it
midstream.

## Pass Rule

Condition B supports only a narrow validation note if it improves exclusion
power and evidence-boundary scores without increasing false confidence,
slogan use, coercive language, or unsafe automation acceptance.

No result from this run validates Theme Swimlanes universally.

