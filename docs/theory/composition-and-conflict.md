# Composition and Conflict

Some situations need more than one frame. Composition is useful when frames
clarify different jobs in the same situation. Conflict appears when frames imply
different actions, authority models, or evidence boundaries.

## Composition Rule

Compose frames only when each frame owns a different job.

Good composition:

| Frame A | Frame B | Why It Works |
|---|---|---|
| Red / yellow / green | Dashboard warning light | Status says severity; warning light says inspect cause. |
| Detour | Fuel gauge | Route changes; resource runway must still be checked. |
| Four-way stop | Following distance | Turn order plus buffer between dependent work. |
| Speed limit | Downshift | Maximum safe pace plus control under load. |

Bad composition:

| Frame A | Frame B | Problem |
|---|---|---|
| Four-way stop | Incident command | Equal turn-taking conflicts with directed authority. |
| Detour | New destination | Path change conflicts with goal change. |
| Green light | Dashboard warning light | Proceed signal conflicts with inspect-soon signal unless scope differs. |
| Crosswalk yield | Merge lane | Protected priority conflicts with equal integration unless protected value is named. |

## Frame Roles in a Composition

A composition should assign roles:

| Role | Meaning |
|---|---|
| Primary frame | The frame that gives the main action cue. |
| Secondary frame | The frame that adds a constraint, warning, or evidence check. |
| Boundary frame | The frame that says when to stop using the primary frame. |

Example:

- Primary: detour.
- Secondary: fuel gauge.
- Boundary: new destination.

Meaning: change route if the destination is still valid and resources support
the new path; stop calling it a detour if the destination changes.

## Conflict Types

| Conflict | Description | Resolution |
|---|---|---|
| Action conflict | Frames imply different next moves. | Choose the frame tied to the required decision. |
| Authority conflict | Frames imply different decision rights. | Use the frame matching the actual owner or escalation model. |
| Evidence conflict | Frames require different proof. | Name both evidence needs or choose the stricter one. |
| Audience conflict | One frame transfers, another excludes. | Use the frame with broader or intended audience fit. |
| Safety conflict | One frame risks dehumanizing or coercing. | Reject the unsafe frame. |
| Time conflict | One frame fits now, another fits later. | Sequence frames rather than blending them. |

## Sequencing Instead of Blending

When frames conflict over time, sequence them:

1. **Blind spot**: identify missing visibility before changing direction.
2. **Four-way stop**: establish turn order once parties are visible.
3. **Merge lane**: integrate work after priority and timing are clear.

Do not present the sequence as one metaphor. Name the transition.

## Composition Test

Before composing frames, answer:

1. What job does each frame do?
2. Which frame gives the primary action cue?
3. What does the secondary frame add?
4. Do the frames imply the same authority model?
5. Do the frames require compatible evidence?
6. Does one frame create a hard stop for the other?
7. Would one direct sentence be clearer than composition?

If the answer to 7 is yes, skip composition.

## Conflict Resolution Order

Resolve conflicts in this order:

1. Human safety.
2. Actual authority model.
3. Evidence requirement.
4. Target situation specificity.
5. Audience transfer.
6. Action cue clarity.
7. Story memorability.

Story never wins over safety, authority, or evidence.

## Index Implications

`frames-core` currently exposes related frames, not composed frames. That is the
right default. Related frames are candidates for comparison; composition should
remain a caller or higher-level pack decision until FRAMES has enough reviewed
examples. Typed related-frame relations are defined in
[related-frame-taxonomy.md](related-frame-taxonomy.md).

Future index metadata may add:

- compatible-with,
- conflicts-with,
- primary/secondary role hints,
- boundary frame hints,
- sequence-before and sequence-after.

Do not add those fields until composition and related-frame examples exist
across at least three frame jobs.
