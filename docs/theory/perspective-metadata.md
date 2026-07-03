# Perspective Metadata

Perspective metadata names the role a frame asks the listener to inhabit. It is
one of the strongest guards against misleading analogies because many frames do
their work by assigning duty, agency, protection, or decision rights.

A frame does not only say "this situation is like that situation." It also says
"you are like this participant in that situation."

## Why Perspective Matters

Two frames can describe the same target situation but imply different duties:

| Frame | Implied Perspective | Implied Duty |
|---|---|---|
| Four-way stop | Peer participant | Signal, yield by rule, proceed visibly. |
| Crosswalk | Stronger actor / protected party | Slow down to protect the vulnerable side. |
| Dashboard warning light | Operator | Inspect the signal and decide whether to continue. |
| Incident command | Accountable commander | Coordinate quickly under authority. |
| Apprenticeship | Learner or mentor | Practice, demonstrate, scaffold, release. |

If the perspective is wrong, the frame may still sound familiar while assigning
the wrong responsibility.

A useful literary pattern is the same person seen as pedestrian, driver, and
bicyclist: each role carries a different local grievance, safety model, and
right-of-way expectation, even when the underlying actor is identical. FRAMES
should treat that as a warning. Perspective does not merely reveal opinion; it
changes which constraints, risks, and duties are salient.

Use this pattern for review:

| Perspective | Likely Complaint | Hidden Duty |
|---|---|---|
| Pedestrian | Drivers and cyclists move too fast and threaten safety. | Cross predictably and notice traffic constraints. |
| Driver | Pedestrians and cyclists are unpredictable obstacles. | Control speed and protect more exposed road users. |
| Bicyclist | Drivers are dangerous and pedestrians interrupt flow. | Signal, yield where required, and manage mixed vulnerability. |

The lesson is not that all parties are hypocrites. The lesson is that a frame
can make one role's burden obvious while making another role's burden invisible.

## Perspective Fields

Future catalog entries should eventually expose these fields.

| Field | Question | Example Values |
|---|---|---|
| Implied role | Who does the listener become in the source scene? | operator, peer, owner, protected party, learner, steward |
| Counterparty role | Who else is implied by the scene? | pedestrian, reviewer, responder, dependent team, user |
| Agency level | How much action can the implied role take? | observe, signal, pause, decide, command, protect |
| Duty type | What obligation is imported? | yield, inspect, coordinate, preserve, escalate, teach |
| Authority assumption | Who has decision rights? | peer, owner, command, steward, legal/procedural |
| Vulnerability assumption | Who or what is protected? | user, learner, public trust, safety, system capacity |
| Perspective risk | What goes wrong if this role assignment is false? | blame shift, paternalism, false peerhood, hidden power |

## Perspective Roles

| Role | Good For | Common Frames | Main Risk |
|---|---|---|---|
| Operator | Status, monitoring, intervention. | dashboard, checklist, speed limit. | Assumes control the listener may not have. |
| Peer participant | Coordination among roughly equal parties. | four-way stop, queue, merge lane. | Hides hierarchy, urgency, or ownership. |
| Protected party | Safety, care, fairness, user protection. | crosswalk, guardrail, safety stop. | Can patronize or mislabel people as weak. |
| Owner | Accountability, scope, decision rights. | property line, budget owner, release captain. | Can erase consultation or shared governance. |
| Steward | Long-term care and preservation. | maintenance, reserves, garden, trust. | Can delay necessary change. |
| Learner | Skill growth and feedback. | apprenticeship, practice loop, scaffolding. | Can infantilize experienced people. |
| Responder | Urgent triage and recovery. | incident response, first aid, evacuation. | Can inflate routine work into emergency. |
| Reviewer | Inspection, quality, legitimacy. | code review, audit, building inspection. | Can over-formalize exploratory work. |

## Perspective Fit Test

Ask these questions before accepting a frame:

1. Who is the listener in the source scene?
2. Does the listener actually have that agency in the target situation?
3. Who is protected, delayed, inspected, taught, or commanded?
4. Does the frame shift blame away from the accountable actor?
5. Does the frame make a power imbalance visible or hide it?
6. Would another perspective produce a safer action cue?
7. What plain-language sentence names the duty without the frame?

If these answers are unclear, the frame should be revised or held.

## Perspective Conflicts

Some frame pairs conflict because they assign incompatible listener roles.

| Conflict | Example | Resolution |
|---|---|---|
| Peer vs. owner | Four-way stop vs. release owner. | Use peer frame only for coordination; use owner frame for decision rights. |
| Operator vs. passenger | Dashboard warning when the listener cannot act. | Name the real operator or use escalation frame. |
| Protector vs. partner | Crosswalk when the other party is not vulnerable. | Use priority or negotiation frame instead. |
| Learner vs. performer | Apprenticeship during formal evaluation. | Separate coaching from assessment. |
| Responder vs. planner | Incident frame for normal planning tradeoff. | Use roadmap, budget, or sequencing frame. |

## Metadata Before API

The first Rust index should not add perspective fields until there is enough
catalog evidence. Before that, perspective should be recorded in docs, tags,
role reviews, and evidence boundaries.

Near-term convention:

- add perspective words to tags when useful,
- mention perspective in frame-pack docs,
- reject frames with false peerhood, false control, or hidden protection duties,
- include perspective in role-review notes for high-risk frames.

## Examples

| Frame | Target Situation | Perspective Check |
|---|---|---|
| Red/yellow/green | Progress tracking. | Who can change the status, and who only observes it? |
| Four-way stop | Shared attention or sequencing. | Are the parties actually peers? |
| Crosswalk yield | Protecting user safety or overloaded staff. | Who has duty to slow down, and is the protected label respectful? |
| Detour | Plan change. | Who chooses the route, and is the destination still agreed? |
| Load-bearing wall | Dependency or architecture. | Who has authority to alter it, and who bears failure cost? |

## Conflict-Empathy Pattern: Phantom Dissonance

The "someone is eating my chips" story is useful for team conflict: a person
believes someone else is taking their food, reacts from that assumption, and
later discovers their own food was still in their bag. They were eating the
other person's food.

As a frame, this works because it moves the listener through three roles:

| Role | What Feels True | What Must Be Checked |
|---|---|---|
| Wronged party | "They are taking what is mine." | What evidence proves ownership or intent? |
| Silent counterparty | "They may be confused, generous, avoidant, or equally baffled." | What has the other party actually observed? |
| Reframed actor | "I may be contributing to the conflict I am judging." | What assumption was I carrying without verification? |

Use this frame for cross-team tension when the goal is empathy and fact-finding,
not blame. It is especially useful when one group thinks another group is
careless, obstructive, selfish, or disrespectful.

Structural name: **Phantom Dissonance**.

Use it when tension is real but may be running on a false model of scarcity,
ownership, threat, or intent. The correction is not to reduce contact or demand
friendliness. The correction is to put the actual resource structure where both
parties can inspect it.

Action cue:

1. Pause the accusation.
2. Check the shared facts.
3. Ask who actually owns or supplies the contested resource.
4. Ask what the other team sees from their side.
5. Separate impact from intent.
6. Re-enter the conflict with a repair move.

Evidence boundary: the story does not prove nobody acted badly. It only proves
that the first explanation may be wrong.

Misuse warning: do not use this frame to minimize real harm, excuse repeated
behavior, or pressure an injured party to absorb responsibility.

Failure condition: if both parties have stated their models, the evidence is
visible, and the conflict remains, the dissonance is not phantom. Treat the
disagreement as real.

Related pattern: **Deferred D**. This is not a false model but an unexamined
one. People know the ownership, authority, or resource question is unresolved,
but delay naming it because naming it creates commitment. Use the same visible
board move: put both answers in the room and ask what would make the boundary
fair enough to proceed.

Internal source note: adapted from the MANAGE "The Chips" chapter in the local
RESONANCE materials.

## Anti-Patterns

| Anti-Pattern | Why It Fails | Correction |
|---|---|---|
| False peerhood | Treats unequal actors as equal participants. | Name decision rights and duties explicitly. |
| False control | Makes the listener an operator without actual control. | Use escalation or dependency frame. |
| Hidden protector | Assigns one side a duty to absorb cost without saying so. | Name the protected value and tradeoff. |
| Infantilizing learner frame | Recasts a capable adult as a novice. | Use craft, review, or performance frame. |
| Blame transfer | Makes a system issue look like an individual failure. | Name the system condition and accountable owner. |
