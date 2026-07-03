# Audience Transfer

Audience transfer is the question: will this frame survive contact with the
people who need to use it?

Use [cultural-portability.md](cultural-portability.md) when the audience is
unknown, mixed, global, non-driver, accessibility-sensitive, or outside the
source scene's home assumptions.

A frame can be strong in one audience and weak in another. FRAMES should record
that openly instead of pretending an everyday scene is universal.

## Transfer Dimensions

| Dimension | Question | Risk |
|---|---|---|
| Role | What work does the audience do? | The frame clarifies the wrong decision. |
| Expertise | How much domain knowledge does the audience have? | The frame assumes hidden knowledge. |
| Region | Which local rules or customs are assumed? | Traffic, school, work, or service norms may differ. |
| Culture | Which social scripts are assumed? | Turn-taking, authority, conflict, and politeness can transfer poorly. |
| Mobility | Does the source assume driving, walking, or physical ability? | The source excludes or confuses part of the audience. |
| Stakes | How serious is the target situation? | A casual frame can trivialize high-impact decisions. |
| Power | Who can act on the cue? | The frame can pressure people without authority. |

## Audience Bands

Use these bands when drafting or reviewing a frame.

| Band | Use When | Required Treatment |
|---|---|---|
| Broad | Most intended readers likely know the source scene. | Keep the frame concise and still name limits. |
| Role-specific | The frame works for a job family or domain. | Name the role and avoid general-public claims. |
| Local | The frame depends on local rules, norms, or infrastructure. | Mark the locality and provide an alternate. |
| Expert | The frame requires specialist knowledge. | Use only in expert-facing docs or explain the source first. |
| Fragile | The frame may exclude, confuse, or harm some readers. | Prefer an alternate unless the protected value is explicit. |

## Transfer Test

Before accepting a frame, answer:

1. Who is the intended audience?
2. What source-scene knowledge must they already have?
3. What part of the frame is local, cultural, professional, or ability-bound?
4. What alternate frame works if the source does not transfer?
5. Does the action cue require authority the audience may not have?
6. Could the frame shame, patronize, or flatten the people involved?
7. Is the frame still useful if the source scene is explained in one sentence?

## Alternates

Each fragile or local frame should have an alternate. The alternate does not need
to be perfect; it gives the author a safer fallback.

| If This Frame Fails | Try |
|---|---|
| Four-way stop | Turn-taking queue, agenda order, explicit facilitator. |
| Crosswalk yield | Protected review gate, safety buffer, service window. |
| Speed limit | Operating envelope, capacity cap, review budget. |
| Merge lane | Handoff checklist, integration window, onboarding path. |
| Walking pace | Sustainable cadence, batch size, burn rate. |

## Example: Four-Way Stop

Audience: US driving-familiar product and engineering teams.

Transfer strengths:

- quick scene recognition,
- clear turn-taking,
- visible pause before movement,
- easy action cue.

Transfer risks:

- not all audiences know four-way-stop rules,
- equal-right-of-way assumption fails when one owner has authority,
- emergency work should not wait for polite turn-taking.

Alternate frames:

- facilitator assigns agenda order,
- service queue with explicit priority,
- incident command for urgent work.

## Example: Crosswalk Yield

Audience: teams discussing protected values such as safety, accessibility,
compliance, onboarding clarity, or user harm.

Transfer strengths:

- the faster actor absorbs delay,
- vulnerable crossing gets priority,
- action cue is early slowing, not late reaction.

Transfer risks:

- calling a team or person "the pedestrian" can sound patronizing,
- traffic laws and pedestrian expectations differ,
- the frame can hide the protected value if the analogy becomes the point.

Safer wording:

"This is a protected-crossing situation: throughput waits until the review
capacity is safe."

## Recording Audience Fit

Future catalog entries should add audience fields:

| Field | Meaning |
|---|---|
| Intended audience | Who the frame is primarily for. |
| Transfer band | Broad, role-specific, local, expert, or fragile. |
| Assumed source knowledge | What the audience must know already. |
| Alternate frame | Fallback when the source does not transfer. |
| Authority check | Who can act on the action cue. |

## Design Consequence for `frames-core`

Do not add audience metadata casually. The next schema work should add audience
fields only when the docs have enough examples to avoid premature structure.

Near-term:

- keep audience risks in docs,
- use tags sparingly for obvious audience constraints,
- prefer related frames that can act as alternates,
- do not rank a local or fragile frame above a broad frame without an explicit
  query tag.
