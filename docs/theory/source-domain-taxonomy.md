# Source-Domain Taxonomy

Source domains are the familiar scenes FRAMES borrows from. A source domain is
not just a topic like "driving" or "cooking"; it is a bundle of embodied
experience, social rules, authority assumptions, emotional load, and expected
actions.

This taxonomy keeps frame selection from becoming a loose metaphor list.

## Domain Dimensions

Every source domain should be described across these dimensions before it is
used broadly.

| Dimension | Question | Why It Matters |
|---|---|---|
| Embodied schema | What physical experience does the source depend on? | Determines whether the scene is easy to simulate. |
| Social script | What shared rule or norm organizes the scene? | Determines whether people infer the same next action. |
| Authority model | Who has right-of-way, duty, control, or decision rights? | Prevents importing the wrong power relationship. |
| Temporal shape | Is the scene instant, sequential, cyclical, or cumulative? | Helps match status, momentum, lifecycle, and recovery situations. |
| Risk model | What can go wrong, and who bears the cost? | Separates coordination risk from safety, quality, or reputational risk. |
| Evidence style | What evidence would people normally inspect? | Helps define the evidence boundary. |
| Emotional load | What feelings or moral judgments does the source carry? | Flags coercive, blame-heavy, or high-stakes frames. |
| Audience portability | Who is likely to know the source scene? | Guides novice, regional, cultural, and expertise checks. |

## Starter Source Families

| Family | Typical Sources | Good For | Main Risk |
|---|---|---|---|
| Motion and navigation | walking, driving, wayfinding, detours, merging | momentum, route choice, sequencing, pace | Treating people as traffic or obstacles. |
| Signals and instruments | lights, gauges, dashboards, alarms, checklists | status, threshold, attention, escalation | Confusing signal clarity with proof. |
| Queues and turns | lines, tickets, appointments, four-way stops | fairness, turn order, contention, coordination | Hiding authority or urgency differences. |
| Construction and repair | foundations, scaffolds, load, inspection, maintenance | dependency, quality, support, readiness | Treating social systems as static objects. |
| Cooking and preparation | heat, timing, batches, ingredients, mise en place | readiness, sequencing, constraint, iteration | Overstating control in uncertain situations. |
| Health and care | symptoms, diagnosis, triage, recovery, prevention | risk, intervention, monitoring, resilience | Medicalizing normal conflict or performance. |
| Sports and games | field position, fouls, timeouts, advantage, practice | strategy, rules, practice, recovery | Encouraging zero-sum or win-at-all-costs framing. |
| Learning and craft | apprenticeship, drills, critique, scaffolding | skill growth, feedback, progression | Patronizing experienced audiences. |
| Markets and budgets | tradeoffs, portfolios, debt, options, reserves | allocation, priority, capacity, opportunity cost | Reducing human value to financial terms. |
| Law and governance | precedent, jurisdiction, due process, escalation | authority, legitimacy, procedure, accountability | Over-formalizing normal collaboration. |

## Authority Models

Authority is one of the easiest places for a frame to mislead.

| Model | Source Pattern | Use When | Avoid When |
|---|---|---|---|
| Peer coordination | four-way stop, queue, passing on a trail | Parties have similar rights and need visible sequencing. | One actor owns the decision or bears protected duty. |
| Protected party | crosswalk, guardrail, safety stop | One side must absorb delay to protect another value. | The protected party label would be patronizing or inaccurate. |
| Operator responsibility | dashboard, speed limit, checklist | A role-holder must inspect, slow, or intervene. | The operator lacks actual control. |
| Command authority | incident command, captain, referee | Fast coordination requires one accountable decider. | The situation needs consent, deliberation, or distributed ownership. |
| Stewardship | maintenance, budgeting, reserves | Long-lived assets or relationships need care. | Immediate action is more important than preservation. |
| Learning scaffold | coach, mentor, apprenticeship | Capability is developing through supported practice. | The audience is already expert or the issue is not skill. |

## Temporal Shapes

| Shape | Typical Frame | Fits | Misfit |
|---|---|---|---|
| Threshold | red/yellow/green, warning light | Status changes at known bounds. | Slow drift without clear thresholds. |
| Sequence | merge lane, recipe steps, queue | Order and timing matter. | Open-ended exploration. |
| Cycle | inspection, maintenance, practice loop | Repeated check-and-adjust work. | One-time irreversible choices. |
| Recovery | rest stop, triage, repair | Fatigue, damage, or interruption. | Normal planned pause with no loss. |
| Accumulation | technical debt, fuel gauge, reserves | Capacity or risk builds over time. | Binary go/no-go conditions. |
| Branching path | detour, fork, route planning | Alternate paths preserve destination. | Destination itself is disputed. |

## Risk Bands

| Risk Band | Use Rule | Extra Requirement |
|---|---|---|
| Low | Everyday, low-stakes coordination. | Normal evidence boundary and misuse warning. |
| Medium | Affects priority, staffing, budget, or public commitment. | Add alternate frame and explicit authority check. |
| High | Affects safety, employment, legal, medical, civic, or identity-sensitive decisions. | Prefer plain-language statement first; use frame only as secondary explanation. |
| Rejected | Dehumanizes, coerces, hides blame, or imports discriminatory assumptions. | Do not catalog except as an anti-pattern. |

## Domain Selection Procedure

1. Name the target situation plainly.
2. Choose the frame job: status, coordination, momentum, risk, priority, or
   learning.
3. Identify the needed authority model.
4. Identify the needed temporal shape.
5. Choose candidate source families with matching relational structure.
6. Check audience portability and emotional load.
7. Prefer the lowest-risk source domain that preserves the needed relation.
8. Record the evidence boundary and misuse warning.

## Catalog Implications

Future catalog entries should eventually expose:

- source family,
- embodied schema,
- authority model,
- temporal shape,
- risk band,
- implied perspective,
- audience portability notes,
- alternate source families.

The first Rust crate does not need these fields yet. Until the API changes,
store the most important domain signals in tags, docs, and review notes.
Use [application-pack-templates.md](application-pack-templates.md) to choose
context-specific defaults after selecting source-domain structure.
Use [perspective-metadata.md](perspective-metadata.md) to check whether the
source domain assigns the listener the correct role and duty.

## Anti-Patterns

| Anti-Pattern | Why It Fails | Safer Move |
|---|---|---|
| Source-first selection | Starts with a clever scene rather than the target relation. | Start from job, authority, and temporal shape. |
| One-domain dominance | Uses traffic, sports, or markets for everything. | Keep at least two source families available for common jobs. |
| Authority smuggling | Uses a peer frame when one actor has decision rights. | Name the actual authority model. |
| Risk inflation | Uses emergency, medical, or combat frames for ordinary coordination. | Use lower-stakes motion, queue, or craft frames. |
| Emotional laundering | Makes a contested decision feel natural by choosing a familiar scene. | State the decision directly before applying a frame. |
