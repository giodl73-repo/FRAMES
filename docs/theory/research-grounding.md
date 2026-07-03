# Research Grounding

FRAMES should use cognitive science as a constraint, not as decoration. The
project can learn from research on metaphor, analogy, framing, semantic frames,
mental models, and decision effects, but it should not overclaim that any single
frame will reliably change behavior in every audience.

## Research Pillars

| Pillar | Core Idea | FRAMES Consequence |
|---|---|---|
| Conceptual metaphor | People often reason about abstract domains through more concrete source domains. | Prefer familiar embodied scenes for abstract target situations, but document the boundary. |
| Structure mapping | Strong analogies preserve relational structure, not just surface similarity. | Score transfer maps by relations, roles, causality, and constraints before vividness. |
| Frame semantics | Words evoke larger situation structures with roles, expectations, and perspective. | Treat a frame as a situation model, not a label. Record actors, duties, and perspective. |
| Mental models | People reason by building simplified internal models of how a situation works. | Good frames should make state, next action, and failure conditions easier to simulate. |
| Framing effects | Different presentations of equivalent facts can shift choice and attention. | Require ethical warnings, evidence boundaries, and alternate frames for high-stakes use. |
| Conceptual blending | People can combine multiple source structures into a useful new model. | Compose frames only when the combined frame keeps authority, evidence, and action clear. |

## What We Can Claim

FRAMES can safely claim:

- Analogies and metaphors can guide attention, interpretation, and possible
  action.
- Strong frames depend on audience familiarity with the source scene.
- Strong frames transfer relational structure, not merely matching words.
- Frames can mislead by importing wrong authority, causality, emotional tone, or
  moral judgment.
- Frame suggestions should include the evidence still needed outside the
  analogy.

FRAMES should not claim without direct validation:

- A frame will reliably persuade a particular audience.
- A frame is culturally universal.
- A vivid frame is more accurate than a plain description.
- A frame can replace metrics, expertise, or stakeholder review.
- A research finding about one domain transfers unchanged to business, product,
  clinical, legal, or public-policy contexts.

Use [claim-strength-labels.md](claim-strength-labels.md) before publishing any
claim about a frame's effect, evidence status, practitioner precedent, or
research support.

## Design Rules From Research

### Prefer Relational Fit

The source scene should match the target's relationships:

- who has right-of-way,
- what state changes,
- what constraint matters,
- what action is available,
- what signal changes behavior,
- what failure would look like.

Surface matches are weaker. A "traffic jam" may feel like a slow project, but
the useful transfer depends on whether the actual issue is capacity, sequencing,
blocked exits, poor signaling, or demand surge.

### Name Perspective

A frame changes what role the listener occupies. A four-way stop gives each
party partial agency. A crosswalk puts duty on the driver. A dashboard warning
light puts duty on the operator. These are not interchangeable.

Every accepted frame should make the implied perspective visible:

- operator,
- pedestrian,
- owner,
- responder,
- learner,
- reviewer,
- protected party.

### Separate Frame From Evidence

Research on framing effects makes evidence boundaries more important, not less.
If presentation can shift attention, then every frame must say what remains to
be checked.

Use this rule:

> A frame is acceptable only when it increases demand for the right evidence.

### Treat Persuasion As Risk

FRAMES is for shared sense-making and goal-driving, not covert influence. If a
frame is likely to push people toward one policy, budget, staffing, or blame
interpretation before evidence is reviewed, mark it as high risk and offer at
least one alternate frame.

### Test Audience Familiarity

A frame that is obvious to one group can fail for another. Driving frames may
work poorly for audiences without driving experience, with different traffic
norms, or with different associations around police, safety, and authority.

Catalog review should ask:

- Does the audience know this source scene?
- Do they share the same norms for it?
- Does the scene carry unwanted emotional or political load?
- Is there a lower-risk source scene with the same relational structure?

## Theory Backlog From Research

| Need | Reason |
|---|---|
| Source-domain taxonomy | Cognitive source domains should be grouped by embodied experience, social script, authority model, and risk. |
| Perspective metadata | Frame entries need to expose the listener role implied by the source scene. |
| Relational transfer fields | The index should eventually store mapped relations, not only tags. |
| Claim-strength labels | Public docs should distinguish theory-informed guidance from empirically validated effects. |
| Alternates for sensitive domains | High-stakes frames need non-persuasive alternates and plain-language fallback. |

The first taxonomy artifact is [source-domain-taxonomy.md](source-domain-taxonomy.md).
Claim-strength labeling is defined in
[claim-strength-labels.md](claim-strength-labels.md).

## Starter Bibliography

- George Lakoff and Mark Johnson, *Metaphors We Live By*, 1980.
- Dedre Gentner, "Structure-Mapping: A Theoretical Framework for Analogy,"
  *Cognitive Science*, 1983:
  <https://groups.psych.northwestern.edu/gentner/papers/Gentner83.pdf>
- Charles J. Fillmore, "Frame Semantics," 1982.
- Marvin Minsky, "A Framework for Representing Knowledge," 1974:
  <http://web.media.mit.edu/~minsky/papers/Frames/frames.html>
- Amos Tversky and Daniel Kahneman, "The Framing of Decisions and the Psychology
  of Choice," *Science*, 1981.
- Paul H. Thibodeau and Lera Boroditsky, "Metaphors We Think With: The Role of
  Metaphor in Reasoning," *PLOS ONE*, 2011:
  <https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0016782>
- Brian F. Bowdle and Dedre Gentner, "The Career of Metaphor," *Psychological
  Review*, 2005:
  <https://groups.psych.northwestern.edu/gentner/papers/BowdleGentner2005.pdf>
- Gilles Fauconnier and Mark Turner, *The Way We Think: Conceptual Blending and
  the Mind's Hidden Complexities*, 2002.
