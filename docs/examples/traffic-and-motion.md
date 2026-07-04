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

## Walking Pace

Use when a team needs sustained movement instead of short bursts.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Pick a pace you can maintain | Choose a cadence that can survive the full delivery window. |
| Keep checking your breathing and terrain | Re-evaluate against deadlines, complexity, and team capacity. |
| Speed up only where footing is stable | Increase pace where uncertainty is low and recovery is easy. |

Action cue: choose the fastest sustainable cadence, then make pace changes
explicit.

Failure mode: "walking pace" can become an excuse for drift if the required
outcome and deadline are not named.

## Footing

Use when progress depends on whether the current surface can support confident
steps.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Test footing before committing weight | Validate assumptions before scaling a change. |
| Use handrails on unstable surfaces | Add controls, review, or pairing for fragile work. |
| Shorten steps when traction is uncertain | Reduce change size under uncertainty. |

Action cue: stabilize the surface before optimizing speed.

Failure mode: overusing "bad footing" can overstate risk and block reasonable
movement.

## Stride Length

Use when the question is not whether to move, but how big each step should be.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Longer stride covers distance faster | Larger increments can increase throughput. |
| Longer stride reduces correction frequency | Bigger batches reduce opportunities to adjust course. |
| Shorter stride increases control | Smaller slices improve feedback and correction quality. |

Action cue: set step size to match feedback speed and correction cost.

Failure mode: defaulting to short strides can hide fear of commitment; defaulting
to long strides can hide avoidable rework risk.

## Crowded Sidewalk

Use when many actors with different speeds share a narrow decision space.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Keep flow without forcing collisions | Coordinate changes so teams can keep moving. |
| Signal lane changes early | Announce ownership or priority shifts before execution. |
| Pull aside for long conversations | Move deep debates out of high-traffic channels. |

Action cue: preserve flow by making intent visible and minimizing surprise.

Failure mode: this frame can normalize congestion as "just how it is" instead of
triggering capacity or scope decisions.

## Trail Marker

Use when people must navigate uncertain terrain without re-solving direction at
every step.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Mark the route at key forks | Publish decisions where teams are likely to diverge. |
| Confirm marker visibility from the path | Keep guidance where the work actually happens. |
| Refresh faded markers | Update docs and runbooks when context changes. |

Action cue: place clear directional markers at decision points, not only in
planning artifacts.

Failure mode: stale markers create false confidence and can be worse than no
marker.

## Rest Stop

Use when continued motion without recovery will degrade quality or safety.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Pause before exhaustion becomes injury | Schedule recovery before error rates spike. |
| Refill water and check route | Use the pause to restore capacity and confirm plan. |
| Restart with a clear next leg | Resume with a named objective and handoff condition. |

Action cue: take bounded recovery pauses tied to explicit restart criteria.

Failure mode: unbounded rest stops become parked work with no return plan.

## Stumble and Recover

Use when a minor failure happens during motion and the priority is safe recovery,
not blame.

Useful mapping:

| Walking behavior | Work mapping |
|---|---|
| Regain balance before accelerating | Stabilize system behavior before restoring full throughput. |
| Check for injury after a stumble | Assess impact before declaring normal operations. |
| Adjust path to avoid repeating the trip point | Capture the cause and modify process safeguards. |

Action cue: recover control first, then learn and adapt before resuming speed.

Failure mode: treating every stumble as a full stop can over-rotate from normal
error handling to chronic caution.
