# Relational Transfer Fields

Relational transfer is the controlled mapping between a familiar source scene
and a target situation. It answers: what relationship is being borrowed, what
does it authorize someone to do, and what must not transfer?

FRAMES should prefer relational transfer over surface similarity. "This launch
is like merging onto a highway" is weak until it says which relation transfers:
matching pace, signaling intent, finding an opening, and joining predictably.

## Field Set

Use these fields when drafting, reviewing, or indexing a frame.

| Field | Question | Failure if Missing |
|---|---|---|
| Source relation | What relation in the source scene carries the meaning? | The frame borrows vibes or imagery instead of structure. |
| Target relation | What relation in the real situation needs clarification? | The frame cannot guide action. |
| Actor roles | Who acts, waits, yields, protects, decides, or reviews? | The analogy hides agency or assigns the wrong duty. |
| Authority relation | Are actors peers, operators, owners, stewards, protected parties, or command roles? | The frame imports the wrong power model. |
| Constraint relation | What limit shapes behavior: time, capacity, safety, attention, budget, trust, or dependency? | The action cue optimizes the wrong variable. |
| Signal relation | What visible signal changes behavior? | The audience cannot tell when the frame applies. |
| Threshold relation | What boundary changes the action? | Status and risk frames become subjective labels. |
| Sequence relation | What order, handoff, or turn structure matters? | Coordination frames become slogans. |
| Feedback relation | What observation updates the frame? | The frame cannot learn from reality. |
| Protected value | What must not be sacrificed while using the frame? | Efficiency or persuasion can override the real goal. |
| Transfer exclusions | What source details must not carry over? | Irrelevant or harmful assumptions leak into the target. |
| Distortion risk | What false inference is the audience likely to make? | Misuse appears only after the frame spreads. |

## Transfer Strength

Not every mapping has the same quality. Record the strength before catalog
acceptance.

| Strength | Meaning | Catalog Rule |
|---|---|---|
| Structural | Multiple relations match: actors, authority, constraint, signal, sequence, and stop condition. | Eligible for accepted use if other checks pass. |
| Partial | One or two relations match, but important authority, evidence, or audience assumptions differ. | Use with explicit caveat or alternate. |
| Illustrative | The scene helps people picture the issue but does not guide action directly. | Keep as story/example, not primary frame. |
| Decorative | The metaphor is memorable but does not preserve useful relations. | Reject or rewrite as plain language. |
| Dangerous | The transfer imports dehumanization, false authority, coercion, or unsupported certainty. | Reject as anti-pattern. |

## Relational Transfer Procedure

1. Name the target relation before naming the source scene.
2. List the actor roles in both source and target.
3. Check authority relation before action cue.
4. Identify the main constraint and protected value.
5. Name the signal, threshold, sequence, or feedback relation that makes the
   frame actionable.
6. Write transfer exclusions in direct language.
7. Assign transfer strength.
8. Apply the fit rubric and story-job overlay.

If you cannot name the target relation, do not use a frame yet. Say the plain
sentence first.

## Examples

### Four-Way Stop

| Field | Transfer |
|---|---|
| Source relation | Peers arrive at shared contention point and negotiate turn order through visible stopping and signaling. |
| Target relation | Peer teams need fair sequencing around constrained attention or shared dependency. |
| Actor roles | Each party stops, reads order, signals intent, and proceeds visibly. |
| Authority relation | Peer coordination. |
| Constraint relation | Shared attention, dependency capacity, or sequencing conflict. |
| Signal relation | Visible pause, explicit claim of next move, acknowledgement. |
| Threshold relation | Use only when parties have roughly equal right to proceed. |
| Sequence relation | First arrival, explicit yield, then visible movement. |
| Feedback relation | If another party does not acknowledge, pause and renegotiate. |
| Protected value | Fairness and collision avoidance. |
| Transfer exclusions | Cars are not people; do not import obstacle language or equal authority when one owner has decision rights. |
| Distortion risk | May hide urgency, asymmetry, or protected-party duties. |
| Strength | Structural for peer coordination; partial or dangerous for asymmetric authority. |

### Red / Yellow / Green

| Field | Transfer |
|---|---|
| Source relation | A simple status signal changes allowed behavior at known thresholds. |
| Target relation | A project, risk, or operational condition needs a shared readiness label. |
| Actor roles | Reviewer labels status; owner explains evidence and next action. |
| Authority relation | Operator responsibility or stewardship. |
| Constraint relation | Risk tolerance, schedule, capacity, quality, or confidence. |
| Signal relation | Color label plus threshold evidence. |
| Threshold relation | Green within bounds, yellow near or beyond warning threshold, red outside acceptable bounds. |
| Sequence relation | Report, inspect, decide, act. |
| Feedback relation | Status updates when evidence or threshold changes. |
| Protected value | Decision clarity without false certainty. |
| Transfer exclusions | Green does not mean risk-free; red does not assign blame by itself. |
| Distortion risk | Color can become theater if thresholds are vague. |
| Strength | Structural only when thresholds and owners are explicit. |

### Bag Of Chips Conflict Story

| Field | Transfer |
|---|---|
| Source relation | A person interprets another actor's behavior as theft until new information reverses the perspective. |
| Target relation | Cross-team conflict may be driven by incomplete information or mistaken ownership assumptions. |
| Actor roles | Each party may be both harmed observer and accidental contributor. |
| Authority relation | Peer repair with possible accountability follow-up. |
| Constraint relation | Limited information under social stress. |
| Signal relation | Surprise evidence that changes the interpretation. |
| Threshold relation | Use when facts are incomplete, not when harm is already established. |
| Sequence relation | Pause judgment, inspect ownership/facts, retell from both perspectives, then repair. |
| Feedback relation | New facts update the conflict story. |
| Protected value | Empathy without erasing accountability. |
| Transfer exclusions | Do not imply every conflict is a misunderstanding. |
| Distortion risk | Can pressure people to forgive before facts and repair are handled. |
| Strength | Illustrative to partial; use as story support, not proof. |

## Fit-Rubric Integration

Relational transfer sharpens the `Transfer clarity` score in
[fit-rubric.md](fit-rubric.md):

- Score `2` only when source relation, target relation, authority relation,
  action relation, and transfer exclusions are explicit.
- Score `1` when the main relation is clear but authority, threshold, or
  exclusions need revision.
- Score `0` when the frame relies on surface similarity, mood, wordplay, or
  imagery without a usable relation.

Any dangerous transfer strength is a hard stop.

## Design Consequences

- Future catalog metadata should include transfer strength and exclusions before
  adding richer narrative fields.
- AI-agent lookup should rank frames by relation fit before source-domain
  familiarity.
- Transfer-aware search design lives in
  [transfer-aware-search-design.md](transfer-aware-search-design.md).
- `related` frames should prefer adjacent relational structure, not just shared
  source family.
- High-stakes application packs should require explicit authority relation,
  protected value, and transfer exclusions.
