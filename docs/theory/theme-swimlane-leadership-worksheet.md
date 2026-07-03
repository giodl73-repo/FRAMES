# Theme Swimlane Leadership Worksheet

This worksheet turns the Theme Swimlanes draft heuristic into a practical
leadership-pack artifact. It is for program planning, portfolio alignment, and
cross-team prioritization where leaders need a small set of contribution lanes
tied to one customer or operating promise.

Using this worksheet does not make Theme Swimlanes an accepted catalog entry.
It remains a promoted draft heuristic until accepted-catalog review, evaluation
fixtures, and pilot evidence support broader use.

## When To Use

Use the worksheet when:

- a program has one customer or operating promise that many teams must support;
- teams need to map work to three to five contribution lanes;
- leaders need to make funding, sequencing, staffing, or scope decisions;
- the frame must help say no, not merely group existing work.

Do not use it when the themes are slogans, every work item fits every lane, or
the lanes are being used to hide cuts, blame, or forced alignment.

## Step 1: Promise

```text
Program:
Audience:
Decision date:

Customer or operating promise:

Who experiences the promise:

What changes for them:

Evidence that the promise improved:

Plain-language fallback if lanes fail:
```

Pass rule: the promise must name an external customer, operator, stakeholder, or
protected value. Internal reorganization language is not enough.

## Step 2: Lane Table

Use three to five lanes. Fewer usually under-specifies the promise; more usually
turns the worksheet into taxonomy management.

| Lane | Customer Outcome | Owner | Measure | Contribution Rule | Exclusion Rule | Tradeoff | Review Date |
|---|---|---|---|---|---|---|---|
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |

Each lane must have:

- an accountable owner or review role;
- one customer or operator outcome;
- one measure;
- one exclusion rule;
- one tradeoff.

## Step 3: Work Mapping

| Work Item | Owning Team | Proposed Lane | Why It Belongs | Evidence Needed | Decision |
|---|---|---|---|---|---|
|  |  |  |  |  | fund / sequence / defer / reframe / reject |
|  |  |  |  |  | fund / sequence / defer / reframe / reject |
|  |  |  |  |  | fund / sequence / defer / reframe / reject |

Decision rule: if work cannot explain its lane contribution, either reframe the
work, defer it, or reject it from the program promise. Do not create a catch-all
lane.

## Step 4: Decision Log

| Decision | Lane Evidence | Tradeoff Accepted | Owner | Revisit Trigger |
|---|---|---|---|---|
|  |  |  |  |  |

The worksheet is successful only if it changes at least one real decision:
funding, sequencing, staffing, scope, dependency resolution, or explicit
rejection of non-fitting work.

## Step 5: Risk Review

| Risk | Check | Disposition |
|---|---|---|
| Slogan compression | Are the lane names more memorable than their measures? | revise / hold / reject |
| Protected-value inversion | Does speed, efficiency, or control override safety, fairness, trust, or accountability? | revise / hold / reject |
| Hidden cuts | Is a lane being used as cover for budget or staffing cuts? | hold / reject |
| Blame laundering | Does a lane make one team carry a system problem? | revise / reject |
| Catch-all lane | Does every work item fit the same lane? | revise / reject |
| Evidence replacement | Are lane labels replacing outcome evidence? | revise / reject |
| Coercive alignment | Does disagreement become disloyalty to the theme? | reject |

Use [frame-antipattern-taxonomy.md](frame-antipattern-taxonomy.md) for reusable
failure classes.

## Step 6: Role Gate

| Role | Pass Question | Finding |
|---|---|---|
| Business Leader | Did the lanes improve a funding, sequencing, staffing, or scope decision? | pass / revise / hold / reject |
| Evidence Boundary Reviewer | Are measures and evidence obligations visible? | pass / revise / hold / reject |
| Misuse Risk Reviewer | Are cuts, blame, coercion, and protected-value inversion blocked? | pass / revise / hold / reject |
| Audience Transfer Reviewer | Will all participating teams understand the promise and lane rules? | pass / revise / hold / reject |
| Catalog Structure Reviewer | Are lifecycle status, claim strength, and index behavior clear? | pass / revise / hold / reject |

All five role gates must pass before pilot use. Pilot use is not accepted-catalog
acceptance.

## Worked Example: Three Customer Lanes

Promise: customers can operate the platform with more control, more visibility,
and less effort.

| Lane | Customer Outcome | Measure | Tradeoff |
|---|---|---|---|
| More control | Customers can choose, govern, override, or recover with confidence. | Successful self-service policy changes, recovery use, ownership clarity. | More control can increase complexity. |
| More visibility | Customers can see status, risk, ownership, and next action. | Fewer status escalations, clearer ownership, better signal freshness. | More visibility can create noise or surveillance concerns. |
| Less effort | Customers complete core work with fewer handoffs and lower cognitive load. | Task completion time, handoff count, support contact reduction. | Less effort can hide needed expert review. |

Decision test: a proposed work item must state which lane it improves and which
measure would move. If it improves no lane, it is outside this promise.

## Pilot Closeout

```text
Pilot program:
Date:
Worksheet owner:

Changed decisions:

Work funded:

Work sequenced later:

Work reframed:

Work rejected:

Evidence still missing:

Misuse or confusion observed:

Recommended lifecycle change:
```

## Design Consequences

- Theme Swimlanes should remain excluded from default Rust search until
  lifecycle filters, leadership-pack filters, and accepted-catalog review exist.
- Evaluation fixtures should test whether a lane worksheet changes decisions
  and blocks slogan compression.
- AI response contracts should display draft status and pilot-only boundaries
  for Theme Swimlanes.
- Accepted-catalog review should require at least one completed worksheet pilot
  before catalog acceptance.
