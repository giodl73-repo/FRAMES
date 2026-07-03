# Theme Swimlane Extraction

Theme swimlanes turn a broad program promise into a small set of contribution
lanes that teams can use to align work. This artifact structures the local
pattern behind names such as `Run One`, `Run Lean`, `Run Fast`, `Run Safe`, and
three-lane customer promises such as more control, more visibility, and less
effort.

This is a local observation, not a universal program-management rule. It starts
as `locally_observed` and can become `heuristic` only after role review shows
that the lanes clarify real tradeoffs, owners, measures, and customer outcomes.

## Pattern Shape

| Field | Meaning |
|---|---|
| Unifying promise | The customer or operating promise that makes the program coherent. |
| Swimlane | A small named contribution path teams can attach work to. |
| Lane owner | The role accountable for keeping the lane meaningful. |
| Customer outcome | What the customer, operator, or stakeholder should experience differently. |
| Measure | The evidence that the lane is improving the promise. |
| Contribution rule | What work belongs in the lane. |
| Exclusion rule | What work does not belong, even if the label sounds attractive. |
| Tradeoff | What the lane may sacrifice or constrain. |
| Failure mode | How the theme can become misleading. |

## Candidate Frame: Theme Swimlanes

| Field | Draft Entry |
|---|---|
| Source scene | Swimlanes in a pool or track keep parallel movement visible without forcing every participant into the same stroke or pace. |
| Target situation | A program needs many teams to contribute different work to one coherent customer promise. |
| Frame job | Priority / coordination. |
| Story job | Vision / destination and decision / tradeoff. |
| Claim strength | `locally_observed`. |
| Relational transfer | A shared promise creates visible contribution lanes; work is easier to coordinate when each lane has a customer outcome and measure. |
| Action cue | Name the promise, define three to five lanes, assign owners, attach measures, and reject work that does not fit a lane or the promise. |
| Evidence boundary | Check customer outcome, lane owner, measure, tradeoff, dependency, and excluded work. |
| Failure mode | Can become slogan grouping if lanes do not change sequencing, funding, measures, or customer outcomes. |
| Transfer exclusions | A lane label does not prove priority, eliminate dependencies, or replace product strategy. |
| Application pack | Leadership / product / operations. |
| Draft status | Promoted draft heuristic; needs fit score before catalog acceptance. |

Promotion review lives in
[local-import-promotion-review.md](local-import-promotion-review.md).

## Local Variants

### Run Family

| Theme | Promise Role | Good Contribution Examples | Evidence Boundary | Main Risk |
|---|---|---|---|---|
| `Run One` | Unification and coherence. | Shared identity, common operating model, standard entry points, fewer fragmented customer paths. | Check whether the shared model improves customer or operator experience. | Can flatten legitimate local differences. |
| `Run Lean` | Efficiency and waste reduction. | Fewer handoffs, simpler process, lower duplicate effort, reduced cost-to-serve. | Check effort, cost, cycle time, and quality together. | Can become cost cutting without value preservation. |
| `Run Fast` | Speed and flow. | Shorter cycle time, faster decisions, smaller batches, reduced wait states. | Check whether speed improves outcome without raising unmanaged risk. | Can reward haste over learning or safety. |
| `Run Safe` | Reliability, safety, and trust. | Guardrails, recovery paths, auditability, reduced operational risk. | Check risk indicators, incident paths, and protected values. | Can become bureaucracy if safety is not tied to real risk. |

### Three Customer-Promise Lanes

| Lane | Customer Promise | Good Contribution Examples | Evidence Boundary | Main Risk |
|---|---|---|---|---|
| More control | Customers can choose, govern, override, or recover with confidence. | Permissions, policy controls, undo/recovery, configuration, clear ownership. | Check whether control is usable and not merely more settings. | Can increase complexity and decision burden. |
| More visibility | Customers can see status, cause, progress, risk, or next action. | Dashboards, explanations, traceability, alerts, audit trails. | Check whether visibility leads to a useful decision or action. | Can become passive reporting or noise. |
| Less effort | Customers accomplish the same outcome with fewer steps, lower cognitive load, or fewer handoffs. | Automation, defaults, guided flows, fewer duplicate entries, simpler escalation. | Check that effort is removed from the customer, not displaced invisibly to another party. | Can hide work, reduce agency, or remove needed review. |

## Extraction Procedure

1. Name the unifying promise in one customer-facing sentence.
2. Choose three to five lanes that express different contribution modes.
3. For each lane, name the customer outcome.
4. Assign a lane owner or accountable review role.
5. Define at least one measure and one tradeoff.
6. Write the contribution rule and exclusion rule.
7. Map current team work to lanes.
8. Reject, defer, or reframe work that cannot explain its lane contribution.
9. Review whether lane names pressure agreement or hide cuts.

## Fit Checks

| Check | Pass | Revise | Reject |
|---|---|---|---|
| Promise clarity | One sentence names the customer or operating promise. | Promise is internal or vague. | Promise is only a slogan. |
| Lane independence | Lanes are distinct contribution modes. | Some overlap but teams can still classify work. | Every work item fits every lane. |
| Outcome evidence | Each lane has a customer or operator measure. | Measures are indirect or incomplete. | No measure changes because of the lane. |
| Tradeoff visibility | Each lane names what it may sacrifice. | Tradeoffs are implied. | Tradeoffs are hidden or denied. |
| Exclusion power | The lanes help say no. | Lanes mostly organize existing work. | Lanes accept everything. |
| Human safety | The lanes clarify work without blaming teams. | Some pressure risk needs wording changes. | Lanes are used to shame, cut, or coerce. |

## Promotion Criteria

Move from `locally_observed` to `heuristic` only when:

- the promise, lanes, owners, measures, tradeoffs, and exclusions are explicit;
- at least two different program examples can be mapped without changing the
  field shape;
- Business Leader confirms the lanes clarify investment or sequencing;
- Evidence Boundary Reviewer confirms slogans do not replace outcome evidence;
- Misuse Risk Reviewer confirms the lanes do not hide cuts, blame, or coercion.

## Design Consequences

- Theme swimlanes should become a leadership/application-pack template before
  becoming a general catalog entry.
- Future metadata should distinguish `theme`, `promise`, `lane`, `measure`, and
  `exclusion`.
- AI-agent use should return lane fit plus a reason, not just assign a work item
  to the nearest-sounding theme.
