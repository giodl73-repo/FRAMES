# Theme Swimlane Role Review

This review applies the FRAMES role lenses to the `Theme Swimlanes` pattern.
The goal is to decide whether it is ready for leadership and program use, not
whether it belongs in the accepted starter catalog.

This is an AI-simulated repo role review. It upgrades confidence only inside
the stated boundary: theme swimlanes are a draft heuristic for planning review,
not an empirically validated management method.

## Review Target

| Field | Value |
|---|---|
| Pattern | Theme Swimlanes |
| Source | Local program patterns: `Run One`, `Run Lean`, `Run Fast`, `Run Safe`, and three-lane promises such as more control, more visibility, and less effort. |
| Current status | Promoted draft heuristic. |
| Claim strength | `heuristic` inside local leadership/program review. |
| Catalog status | Draft, not accepted catalog. |
| Application pack | Leadership / product / operations. |
| Frame job | Priority / coordination. |
| Story job | Vision / destination and decision / tradeoff. |
| Risk band | Medium, because the frame can influence funding, staffing, sequencing, and customer commitments. |

## Acceptance Decision

Decision: revise before acceptance.

Theme swimlanes are ready for pilot use when a team needs one program promise
and three to five distinct contribution lanes. They are not ready for accepted
catalog or default Rust search until the output template has been used on live
program plans and reviewed against outcome evidence.

Use the frame when:

- one customer or operating promise needs many teams to contribute different
  work;
- each lane has an owner, outcome, measure, tradeoff, and exclusion rule;
- the lanes change what gets funded, sequenced, deferred, or rejected;
- teams can place real work into lanes without forcing every item into every
  lane.

Do not use the frame when:

- the lane names are slogans without customer outcomes;
- all work can fit all lanes;
- the frame hides cuts, blame, or coercive alignment;
- no lane owner can explain the measure or tradeoff;
- the situation involves employment, legal, regulatory, or safety decisions
  without a direct policy and evidence statement.

## Role Findings

| Role | Decision | Finding |
|---|---|---|
| Business Leader | Conditional pass | The pattern is valuable when it clarifies investment, sequencing, ownership, and customer promise. It fails as an optics exercise. |
| Frame Fit Reviewer | Conditional pass | The swimlane source transfers when lanes create visible parallel contribution paths. It is weak when it merely groups existing work under attractive headings. |
| Evidence Boundary Reviewer | Revise | Every lane needs observable outcome evidence. "Less effort" must prove effort is removed, not displaced. "More visibility" must lead to a useful decision. "More control" must be usable, not just more settings. |
| Misuse Risk Reviewer | Revise | The pattern can pressure agreement, justify cuts, or blame teams for being "off lane." Uses that affect people need a plain-language decision rationale outside the frame. |
| Novice Reader | Revise | Lane names must be literal enough to understand without repo history. Internal slogans need definitions and examples. |
| Journeyman Practitioner | Conditional pass | The pattern is meeting-usable if a team can fill the lane worksheet in one planning session and classify live work in under 30 minutes. |
| Expert Storyteller | Conditional pass | Names may be memorable, but the story must stay anchored to one promise and a small number of lanes. Clever theme names are secondary. |
| Catalog Structure Reviewer | Hold from accepted catalog | Keep as a leadership/program draft heuristic. It may become an application-pack template before it becomes a general catalog entry. |

## Fit Score

| Dimension | Score | Reason |
|---|---:|---|
| Recognition | 2 | Swimlanes and tracks are familiar in many business and program settings. |
| Target specificity | 2 | The target is narrow: multi-team work aligned to one promise. |
| Transfer clarity | 1 | The relation is useful but still needs concrete lane examples to avoid slogan grouping. |
| Action cue | 2 | The extraction procedure gives clear next actions. |
| Evidence boundary | 1 | Outcome measures exist as a requirement, but the pattern has not yet shown live evidence. |
| Human safety | 1 | The frame can hide pressure, blame, or cuts if misused. |
| Stop condition | 1 | The current draft needs a stronger review date, end state, and retirement rule. |
| Total | 10 / 14 | Revise before acceptance. |

Decision band: revise. The pattern is useful enough for controlled pilot use,
but not yet strong enough for accepted catalog status.

## Evidence Gate

A theme-swimlane use passes review only when it supplies:

| Gate | Required Question |
|---|---|
| Promise | What customer or operating promise is the whole program making? |
| Lane | What distinct contribution mode does this lane represent? |
| Owner | Who is accountable for keeping the lane meaningful? |
| Customer outcome | What changes for the customer, operator, or stakeholder? |
| Measure | What evidence would show the lane improved the promise? |
| Contribution rule | What work belongs in this lane? |
| Exclusion rule | What attractive work does not belong here? |
| Tradeoff | What can this lane sacrifice, constrain, or complicate? |
| Example in | What current work clearly fits? |
| Example out | What current work should be deferred, reframed, or rejected? |
| Changed decision | What funding, sequencing, scope, or staffing decision changed because of this lane? |
| Review date | When does the lane get checked, revised, merged, or retired? |

## Hard Stops

Reject or hold the frame when:

- every work item fits every lane;
- a lane is a euphemism for cuts, blame, or forced agreement;
- the theme name does not express a customer or operating outcome;
- the team cannot name a lane owner and measure;
- a high-stakes decision uses the frame without direct policy, evidence, and
  accountability language;
- the lane labels are more memorable than the promise they serve.

## Output Template

```text
Promise:
Lane:
Owner:
Customer outcome:
Measure:
Contribution rule:
Exclusion rule:
Tradeoff:
Example in:
Example out:
Decision changed:
Review date:
```

## Next Actions

- Use [theme-swimlane-leadership-worksheet.md](theme-swimlane-leadership-worksheet.md)
  for leadership-pack pilot use.
- Run one pilot on a real program plan and record the changed decisions in
  [../validation/theme-swimlane-leadership-pilot-ledger.md](../validation/theme-swimlane-leadership-pilot-ledger.md).
- Add a short empirical validation trial comparing lane framing against a
  plain priority list.
- Keep theme swimlanes out of accepted starter catalog and default Rust search
  until metadata fields and leadership-pack filters exist.
