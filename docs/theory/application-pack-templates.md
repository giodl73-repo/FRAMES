# Application Pack Templates

Application packs are curated defaults for a context of use. They do not create
new theory; they choose which frame jobs, source families, authority checks, and
evidence boundaries should be emphasized for a particular audience.

Use a pack when a team needs repeatable frame selection for a class of
situations, such as product planning, operations review, leadership decisions,
learning design, or AI-agent suggestions.

## Pack Shape

Every application pack should define:

| Field | Meaning |
|---|---|
| Pack name | Stable name for the context of use. |
| Primary audience | Who will use or receive the frame. |
| Typical target situations | The situations this pack is meant to clarify. |
| Default frame jobs | The frame jobs to prefer first. |
| Preferred source families | Source-domain families that usually fit the context. |
| Authority checks | The power, ownership, or duty assumptions to verify. |
| Evidence defaults | The evidence boundary style expected for this pack. |
| Risk defaults | The normal risk band and escalation trigger. |
| Required alternates | When the pack must offer another frame or plain-language fallback. |
| Rejection rules | Frames this pack should not use. |

## Product Pack

| Field | Default |
|---|---|
| Primary audience | Product managers, designers, engineers, stakeholders. |
| Typical target situations | Roadmap choice, user tradeoff, launch readiness, scope control, feedback interpretation. |
| Default frame jobs | Priority, risk, coordination, momentum. |
| Preferred source families | Motion and navigation, signals and instruments, construction and repair, markets and budgets. |
| Authority checks | Who owns scope, launch, user safety, and tradeoff acceptance? |
| Evidence defaults | User evidence, technical readiness, business impact, known constraints. |
| Risk defaults | Medium when roadmap, budget, staffing, or public commitments are affected. |
| Required alternates | Offer a plain-language fallback when a frame could bias priority or blame. |
| Rejection rules | Do not frame users as obstacles, traffic, debt, or noise. |

Useful examples:

- red/yellow/green for readiness,
- load-bearing wall for architectural dependency,
- detour for preserving destination with changed route,
- fuel gauge for limited capacity.

## Operations Pack

| Field | Default |
|---|---|
| Primary audience | Operators, support teams, incident responders, service owners. |
| Typical target situations | Incident response, queue pressure, reliability risk, handoff, recovery, escalation. |
| Default frame jobs | Status, coordination, risk, momentum. |
| Preferred source families | Signals and instruments, queues and turns, health and care, construction and repair. |
| Authority checks | Who can stop work, escalate, declare incident state, or restore service? |
| Evidence defaults | Observed signal, threshold, owner, current mitigation, recovery condition. |
| Risk defaults | Medium by default; high when safety, availability, legal, or public impact is involved. |
| Required alternates | Pair vivid operational frames with direct severity language. |
| Rejection rules | Avoid emergency metaphors for routine pressure; avoid blame frames during response. |

Useful examples:

- dashboard warning light for attention,
- triage for sequencing response,
- shoulder pull-off for controlled pause,
- following distance for buffer.

## Leadership Pack

| Field | Default |
|---|---|
| Primary audience | Executives, founders, managers, program leaders. |
| Typical target situations | Strategy, accountability, resource allocation, decision rights, organizational risk. |
| Default frame jobs | Priority, risk, coordination, status. |
| Preferred source families | Markets and budgets, law and governance, stewardship, signals and instruments. |
| Authority checks | Who decides, who is accountable, who is affected, and who must be consulted? |
| Evidence defaults | Decision owner, tradeoff, constraint, accountable metric, revisit trigger. |
| Risk defaults | Medium by default; high when employment, legal, civic, identity, or public trust is affected. |
| Required alternates | Require at least one alternate frame for contested decisions. |
| Rejection rules | Do not use frames that naturalize layoffs, blame, coercion, or power imbalance. |

Useful examples:

- portfolio allocation for strategic tradeoffs,
- due process for legitimacy,
- guardrail for protected constraints,
- reserve for capacity protection.

## Learning Pack

| Field | Default |
|---|---|
| Primary audience | Learners, mentors, teachers, reviewers, apprentices. |
| Typical target situations | Skill growth, feedback, practice design, onboarding, mistake recovery. |
| Default frame jobs | Learning, momentum, risk, coordination. |
| Preferred source families | Learning and craft, cooking and preparation, sports and games, construction and repair. |
| Authority checks | Is the learner actually in a learning context, or being evaluated for performance? |
| Evidence defaults | Current capability, next practice step, feedback source, release condition. |
| Risk defaults | Low to medium; high when evaluation affects employment, status, or identity. |
| Required alternates | Offer a non-patronizing version for experienced practitioners. |
| Rejection rules | Do not infantilize, rank people as broken objects, or hide performance consequences. |

Useful examples:

- scaffolding for temporary support,
- practice loop for repeated improvement,
- mise en place for readiness before execution,
- timeout for reflective pause.

## AI-Agent Pack

| Field | Default |
|---|---|
| Primary audience | AI tools, assistants, evaluators, prompt builders, downstream software. |
| Typical target situations | Frame lookup, suggestion ranking, explanation generation, warning display. |
| Default frame jobs | Status, coordination, risk, priority, learning. |
| Preferred source families | Source families selected by metadata, not by story vividness. |
| Authority checks | Does the tool know who owns the decision and who bears risk? |
| Evidence defaults | Always return action cue, evidence boundary, misuse warning, and confidence limits. |
| Risk defaults | Medium when the output may influence human decision-making; high for regulated or sensitive domains. |
| Required alternates | Return at least two candidates when authority or audience is ambiguous. |
| Rejection rules | Do not generate persuasive certainty from a frame match. |

Useful examples:

- related-frame lookup as thesaurus,
- evidence boundary as required output,
- alternate candidates when authority model is unclear,
- plain-language fallback for high-risk contexts.

## Pack Selection Procedure

1. Name the actual user and target situation.
2. Choose the nearest pack.
3. Apply the pack authority checks.
4. Select frame candidates from the preferred source families.
5. Apply the source-domain taxonomy for risk and temporal fit.
6. Apply the fit rubric.
7. Attach evidence boundary, misuse warning, and required alternates.
8. If the risk band is high, lead with plain language and use the frame only as
   a secondary explanation.

## Pack Anti-Patterns

| Anti-Pattern | Why It Fails | Correction |
|---|---|---|
| Pack as audience stereotype | Assumes all leaders, learners, or operators think alike. | Treat pack defaults as starting points, then apply audience transfer. |
| Pack as persuasion preset | Chooses frames to make a decision feel obvious. | Require alternate frames for contested decisions. |
| Tool-only output | Gives an AI candidate without evidence boundary or misuse warning. | Return action, evidence, warning, and confidence limits together. |
| One-pack drift | Uses product or leadership defaults for every situation. | Re-select the pack whenever audience, stakes, or authority changes. |

## Future Catalog Fields

Application packs imply future metadata:

- recommended packs,
- excluded packs,
- required alternate count,
- risk escalation trigger,
- plain-language fallback,
- output obligations for tool callers.

These fields should wait until there is enough catalog evidence to avoid
locking in premature API structure.

Use [perspective-metadata.md](perspective-metadata.md) before converting pack
defaults into catalog or crate metadata.

Pack expansion order and coverage gates live in
[domain-pack-roadmap.md](domain-pack-roadmap.md).
Learning-pack progression guidance lives in
[learning-progression.md](learning-progression.md).
