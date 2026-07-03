# Frame Theory

FRAMES treats a frame as a controlled analogy for action. A good frame does not
decorate a situation; it changes what people can notice, decide, or coordinate.

## Core Model

Every useful frame has six parts:

| Part | Question | Failure if Missing |
|---|---|---|
| Source scene | What familiar situation does the audience already understand? | The frame needs too much explanation. |
| Target situation | What real situation is being clarified? | The frame becomes generic advice. |
| Transfer map | Which source features actually carry over? | The analogy imports irrelevant details. |
| Action cue | What should someone do next? | The frame is memorable but inert. |
| Evidence boundary | What still has to be checked? | The frame replaces proof or metrics. |
| Misuse warning | How could this frame distort or harm? | The frame hides accountability or dehumanizes people. |

The transfer map is the center. Without it, a metaphor is only a phrase.

## Frame Jobs

Frames should be grouped by the job they do, not only by their source domain.

| Job | What It Clarifies | Common Action |
|---|---|---|
| Status | Health, readiness, confidence, or capacity. | Proceed, pause, inspect, escalate. |
| Coordination | Turn order, right-of-way, handoff, ownership. | Signal, yield, sequence, confirm. |
| Momentum | Pace, fatigue, delay, recovery, route change. | Continue, downshift, pause, reroute. |
| Risk | Visibility, coupling, fragility, dependency. | Inspect, buffer, isolate, protect. |
| Priority | Which concern wins under constraint. | Fund, cut, sequence, protect. |
| Learning | How expertise changes with practice. | Explain, demonstrate, scaffold, release. |

One source scene can support multiple jobs. A traffic signal can communicate
status, but a speed limit communicates a constrained pace. Treating both as
"traffic metaphors" is less useful than distinguishing their jobs.

## Fit Test

A frame is fit for use when it passes these checks:

1. **Recognition**: the audience can picture the source scene quickly.
2. **Specificity**: the target situation is concrete enough to act on.
3. **Transfer**: the mapped features are explicit and limited.
4. **Decision value**: the frame changes a decision, sequence, or allocation.
5. **Evidence boundary**: the frame says what data or judgment remains outside
   the analogy.
6. **Human safety**: the frame does not cast people as obstacles, hazards, or
   weak objects.
7. **Stop condition**: the entry says when to stop using the frame.

If a frame fails recognition, use a simpler source. If it fails decision value,
say the direct sentence instead. If it fails human safety, rewrite or reject it.
Use [fit-rubric.md](fit-rubric.md) when a frame needs a scored accept, revise,
hold, or reject decision.
Use [frame-lifecycle.md](frame-lifecycle.md) to decide whether a frame belongs
in the index, remains in draft, or should be deprecated.
Use [composition-and-conflict.md](composition-and-conflict.md) when more than
one frame seems useful for the same situation.
Use [evidence-boundary-schema.md](evidence-boundary-schema.md) when adding or
reviewing evidence-boundary fields in docs or `frames-core`.
Use [research-grounding.md](research-grounding.md) when making public claims
about metaphor, analogy, cognition, persuasion, or human decision behavior.
Use [source-domain-taxonomy.md](source-domain-taxonomy.md) when choosing source
families, authority models, temporal shapes, and risk bands.
Use [application-pack-templates.md](application-pack-templates.md) when tailoring
frame defaults for product, operations, leadership, learning, or AI-agent use.
Use [perspective-metadata.md](perspective-metadata.md) when checking what role,
duty, agency, or authority a frame assigns to the listener.

## Audience Levels

FRAMES should support several reader levels without making separate catalogs.

| Reader | Needs | Theory Implication |
|---|---|---|
| Novice | Plain target, plain action, concrete warning. | Avoid internal method terms in first-use examples. |
| Journeyman | A frame that works in a real meeting. | State who acts, pauses, or decides. |
| Expert storyteller | Memorable scene with one main point. | Keep the story vivid but not theatrical. |
| Business leader | Tradeoff, accountability, resource choice. | Separate health, progress, priority, and confidence. |
| AI/tool caller | Stable IDs, tags, related frames, warnings. | Keep the Rust index structured and deterministic. |

The same frame can serve all five only when the short form and structured form
agree.

## Evidence Boundary

Frames explain. They do not prove.

| Frame Claim | Evidence Still Needed |
|---|---|
| "This is yellow." | Which threshold changed, what risk increased, and what would make it green or red. |
| "We need a detour." | Whether the destination is still valid and what the new route costs. |
| "This needs a speed limit." | Which constraint makes higher pace unsafe. |
| "This is a blind spot." | Which dependency, stakeholder, or signal is missing. |

A frame should increase the demand for the right evidence, not reduce it.

## Misuse Patterns

| Misuse | Example | Correction |
|---|---|---|
| People as obstacles | "That team is the roadblock." | Name the blocked decision, dependency, or capacity limit. |
| Status as certainty | "Green means safe." | Green means within agreed bounds, not risk-free. |
| Vividness as proof | "It feels like a boiling pot." | Pair the frame with observed pressure signals. |
| Wrong authority model | Four-way stop when one owner has decision rights. | Use an owner, incident-command, or escalation frame. |
| No stop condition | Detour continues forever. | Name when the original plan is restored or replaced. |

## Selection Procedure

Use this order when choosing a frame:

1. Name the target situation in one direct sentence.
2. Name the job: status, coordination, momentum, risk, priority, or learning.
3. Choose two or three candidate frames from the same job.
4. Compare action cues.
5. Compare failure modes.
6. Pick the frame with the clearest action and least harmful distortion.
7. Attach the evidence boundary.

## Design Consequence for `frames-core`

The Rust index should stay close to this theory:

- `FrameKind` should describe the frame job.
- Tags should expose target constraints, not decorative source details.
- Related frames should show alternatives with adjacent jobs or action cues.
- Every indexed entry should preserve `failure_mode`.
- Future scoring should reward action-cue and evidence-boundary alignment before
  source-domain similarity.
- Public claims should carry research-grounding and claim-strength boundaries
  before they are treated as stable method guidance.
- Future catalog metadata should expose source family, authority model,
  temporal shape, risk band, and implied perspective.
- Application packs should select defaults without bypassing fit, audience,
  evidence, or misuse review.
- Perspective metadata should be reviewed before any frame is accepted for a
  high-stakes, asymmetric, or authority-sensitive situation.
