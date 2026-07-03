# Traffic and Motion Examples

Traffic and motion frames work because people already understand right-of-way,
speed, attention, friction, delay, and safety buffers.

## Red / Yellow / Green

Use for simple state communication:

| Color | Meaning | Expected action |
|---|---|---|
| Green | Proceeding within expected range | Continue and monitor. |
| Yellow | Attention needed before the next checkpoint | Investigate, reduce uncertainty, or prepare a choice. |
| Red | Blocked, unsafe, or outside agreed bounds | Stop, escalate, or replan. |

Good use: project readiness, operational health, release gates.

Weak use: complex strategy, morale, root-cause analysis. The color says what
state the system is in; it does not explain why.

## Four-Way Stop

Use when several parties arrive at a shared constraint and nobody can move well
unless the order is clear.

Useful mapping:

| Road behavior | Work mapping |
|---|---|
| Come to a complete stop | Pause long enough to observe the other parties. |
| Read arrival order | Identify who is waiting and who has priority. |
| Signal intent | Make the next move visible before acting. |
| Proceed predictably | Do the agreed slice without surprising others. |

Action cue: slow down the meeting or workflow just enough to establish turn
order, then move.

Failure mode: do not use this when there is a clear owner, emergency override,
or legal/safety rule. In those cases, "incident command" or "crosswalk yield"
may fit better.

## Slow Down for a Pedestrian

Use when throughput pressure could harm someone with less protection, less
visibility, or less ability to absorb mistakes.

Useful mapping:

| Road behavior | Work mapping |
|---|---|
| Pedestrian has priority in the crossing | The vulnerable party's safety or clarity takes priority. |
| Driver slows before the conflict point | The faster or more powerful actor changes behavior early. |
| Driver does not wave vaguely from speed | Intent must be unambiguous. |
| Driver waits until the crossing is clear | Resume only after the risk is actually past. |

Action cue: the stronger actor absorbs the delay.

Failure mode: avoid labeling a team or person as "the pedestrian" in a way that
sounds weak. Name the protected value instead: user safety, compliance review,
support capacity, accessibility, or onboarding clarity.

## Merge Lane

Use when new work must join a moving system.

Action cue: match speed before insertion. Signal the change, find a gap, and
avoid forcing the active system to brake hard.

Good use: onboarding a new service, joining a roadmap already in flight,
bringing a new reviewer into an active decision.

Failure mode: a merge frame can hide authority. Some merges require approval,
not just good timing.

## Detour

Use when the destination remains valid but the planned path is blocked.

Action cue: preserve the goal, change the route, and mark the cost of the new
path.

Failure mode: if the destination is no longer valid, "detour" is too weak. Use a
"new destination" or "trip canceled" frame instead.

## Downshift

Use when maintaining speed would reduce control.

Action cue: reduce speed or scope to regain torque, traction, and steering.

Good use: scope reduction under technical debt, incident response under load,
moving from broad exploration to one smaller buildable slice.

Failure mode: downshifting is not stopping. If the work is blocked, use red
light, road closed, or pit stop instead.

## Speed Limit

Use when a team needs an explicit upper bound on pace because the environment,
quality bar, or review capacity cannot absorb unlimited speed.

Useful mapping:

| Road behavior | Work mapping |
|---|---|
| Posted limit reflects road conditions | Pace should reflect constraints, not ambition alone. |
| Going faster raises stopping distance | Higher speed reduces correction time. |
| Limits can change by zone | Different work modes can have different caps. |

Action cue: set the maximum safe pace before speed creates hidden risk.

Failure mode: speed limit is not a target speed. It can be misused to punish
teams for going slower when the road conditions are bad.

## Shoulder / Pull-Off

Use when work needs to leave the main flow temporarily without being abandoned.

Useful mapping:

| Road behavior | Work mapping |
|---|---|
| Move out of the active lane | Stop blocking the main workflow. |
| Stabilize before re-entering | Resolve the immediate issue before resuming. |
| Re-enter with signal and speed match | Make the return visible and coordinated. |

Action cue: pause outside the main flow, stabilize, and plan re-entry.

Failure mode: pulling off without a re-entry plan can become indefinite parking.

## Following Distance

Use when dependent work items are moving close enough that a change in one lane
can cause a collision in another.

Useful mapping:

| Road behavior | Work mapping |
|---|---|
| Keep enough gap to brake | Preserve reaction time for downstream teams. |
| Increase distance in bad weather | Add buffer under uncertainty or fragility. |
| Tailgating raises stress and risk | Over-tight coupling makes ordinary changes dangerous. |

Action cue: add buffer where coupling or uncertainty would otherwise amplify
small mistakes.

Failure mode: following distance should name the specific risk. Otherwise it can
sound like generic slack.
