# Story Job Taxonomy

A frame has two different jobs:

- The frame job says what the analogy clarifies: status, coordination,
  momentum, risk, priority, or learning.
- The story job says what the narrative is doing for the audience: building
  trust, teaching, repairing empathy, creating vision, surfacing objections, or
  helping a decision.

Do not collapse these. A four-way stop is usually a coordination frame, but the
story job may be teaching a meeting norm, repairing cross-team conflict, or
warning that unclear turn order creates risk.

## Story Jobs

| Story Job | Use When | Action Cue | Evidence Boundary | Misuse Warning |
|---|---|---|---|---|
| Trust / identity | The audience needs to know who is speaking and why they are credible. | State the speaker's role, experience, and stake. | Trust does not prove the recommendation. | Can become authority theater if motive is hidden. |
| Motive / legitimacy | People may suspect a hidden agenda or unfair demand. | Name the real motive and who benefits or pays. | Claimed motive must match incentives and behavior. | Can launder self-interest as shared interest. |
| Vision / destination | The group needs a picture of a better future state. | Contrast current state with a reachable future state. | Vision still needs path, cost, owner, and constraints. | Can become aspiration without operating detail. |
| Values in action | Abstract values need concrete behavior. | Show the value under pressure. | One example does not define all values conflicts. | Can turn one anecdote into policy. |
| Teaching / simulation | People need a safe rehearsal before acting. | Let the audience practice the decision in miniature. | Simulation needs stated limits and transfer map. | Can oversimplify the real stakes. |
| Objection / resistance | The audience is already thinking of a counterargument. | Name the objection before answering it. | Objection handling needs evidence, not just empathy. | Can straw-man or preempt dissent. |
| Decision / tradeoff | Options and consequences need to become visible. | Compare choices, thresholds, and next moves. | Decision still needs data, owner, and reversibility check. | Can hide power by making the choice feel obvious. |
| Warning / cautionary | A familiar failure pattern is about to repeat. | Pause, inspect, buffer, or escalate. | Warning needs observable signals and trigger thresholds. | Can create fear without a testable risk. |
| Repair / empathy | Conflict is stuck because each side has a partial story. | Retell the scene from another role's view. | Empathy does not settle facts or accountability. | Can pressure forgiveness before repair. |
| Change / springboard | A small concrete example can make a possible change believable. | Show one credible before/after move. | Springboard stories need scope and follow-through. | Can overpromise broad change from one case. |

## Selection Procedure

1. Name the target situation in one direct sentence.
2. Name the frame job.
3. Name the story job.
4. Identify the audience role and perspective risk.
5. Pick a source scene that the audience can picture.
6. Write the action cue, evidence boundary, and misuse warning.
7. Check whether an alternate story job would be more honest.

If the story job is not explicit, the audience will infer one. That inference
often carries the most risk.

## Examples

| Frame | Frame Job | Story Job | Notes |
|---|---|---|---|
| Red / yellow / green | Status | Decision / tradeoff or warning | Status color should trigger a threshold conversation, not replace it. |
| Four-way stop | Coordination | Teaching / simulation or repair / empathy | Useful when peers need turn order; weak when one actor owns the decision. |
| Phantom dissonance | Risk / coordination | Repair / empathy | Helps teams consider that conflict may come from incomplete perspective. |
| Veto rule | Risk / priority | Decision / tradeoff | Clarifies what concern blocks action and who owns the block. |
| Springboard story | Momentum / learning | Change / springboard | Makes a future practice imaginable through a small concrete case. |

## Hard Stops

Reject or rewrite the story layer when:

- The story job is persuasion but is presented as neutral teaching.
- The story asks for trust without naming motive, stake, or evidence.
- The teaching story lacks a transfer boundary.
- The warning story has no observable signal.
- The empathy story erases accountability or repair obligations.
- The vision story has no path, owner, cost, or constraint.

## Fit-Rubric Overlay

Use this as an overlay after the base fit score in
[fit-rubric.md](fit-rubric.md). It does not change the 14-point base score yet.

| Overlay Check | Pass | Revise | Hold |
|---|---|---|---|
| Story-job clarity | The story job is named and matches the use case. | The job is implied but vague. | The story appears to be doing a different job than claimed. |
| Audience-role alignment | The story assigns the audience a fair role. | The role is mostly fair but needs qualification. | The story manipulates role, duty, or authority. |
| Vividness / evidence balance | The story is memorable and keeps evidence visible. | The story is vivid but needs stronger evidence boundary. | The story feels like proof. |
| Alternate readiness | A safer alternate is known for contested use. | Alternate exists but is not documented. | No alternate is available for a high-stakes use. |

Any `Hold` result should block catalog acceptance for medium- or high-stakes
contexts until the story layer is revised.

## Design Consequences

- Future `FrameEntry` metadata should distinguish `kind` from `story_job`.
- AI-agent suggestions should explain why a story job was selected.
- Leadership and conflict-resolution packs should require alternate story jobs
  for contested situations.
- Practitioner-derived story techniques should be imported as bounded story
  jobs, not as general persuasion rules.
