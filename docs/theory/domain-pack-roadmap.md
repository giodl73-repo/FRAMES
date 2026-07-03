# Domain Pack Roadmap

This roadmap controls how FRAMES expands from generic starter frames into
domain-specific application packs. It builds on `application-pack-templates.md`
and keeps pack growth behind evidence, evaluation fixtures, and lifecycle gates.

Application packs should not become themed collections of clever analogies.
They are curated defaults for a specific use context.

## Pack Growth Rule

A domain pack can expand only when it has:

- a named audience and recurring target situation;
- at least three reviewed candidate frames or accepted starter frames;
- one plain-language fallback pattern;
- one positive fixture, one near-miss fixture, and one anti-pattern or hard-stop
  fixture;
- a role reviewer who can reject pack drift;
- a statement of which frames are excluded from the pack.

If those are missing, keep the pack as a template, not a catalog expansion.

## Expansion Order

| Priority | Pack | Why First | First Coverage Goal | Hold Condition |
|---|---|---|---|---|
| 1 | AI-agent | Tool callers need safe defaults before search/ranking expands. | Accepted starter frames with warnings, alternates, fallbacks, and fixture gates. | Hold if rejected-candidate reporting and lifecycle filtering are missing for draft frames. |
| 2 | Product | Product planning already appears in starter frames and local imports. | Readiness, dependency, scope, launch gate, and customer tradeoff frames. | Hold if user evidence or owner authority is missing. |
| 3 | Operations | Traffic, signal, buffer, and recovery frames map well to service review. | Incident, queue, handoff, recovery, threshold, and escalation frames. | Hold if frames obscure incident authority or severity language. |
| 4 | Leadership | Theme swimlanes, Veto Rule, and decision-rights frames influence real decisions. | Decision rights, tradeoffs, funding, sequencing, accountability, and promise lanes. | Hold if a frame can hide cuts, coercion, or unsupported authority. |
| 5 | Learning | Learning use needs careful audience and status boundaries. | Practice, feedback, scaffolding, recovery, and progression frames. | Hold if the frame infantilizes, hides evaluation, or affects employment/status. |

## Pack Coverage Matrix

| Pack | Required Frame Jobs | Required Safety Checks | Required Fixtures |
|---|---|---|---|
| AI-agent | status, coordination, risk, priority | evidence boundary, misuse warning, lifecycle status, confidence limit | selection, suppression, relation behavior, fallback |
| Product | priority, risk, coordination, momentum | owner, user evidence, launch gate, tradeoff acceptance | positive, near-miss, evidence boundary |
| Operations | status, coordination, risk, momentum | severity, owner authority, recovery condition, protected value | hard stop, relation conflict, fallback |
| Leadership | priority, risk, coordination, status | decision rights, accountable owner, affected parties, power risk | near-miss, anti-pattern, changed-decision pilot |
| Learning | learning, momentum, risk, coordination | learner agency, evaluation boundary, non-patronizing language | audience transfer, positive, fallback |

## Candidate Sources By Pack

| Pack | Strong Sources | Weak Or Risky Sources |
|---|---|---|
| AI-agent | Accepted metadata, evaluation fixtures, relation maps, fallback patterns. | Vivid stories without lifecycle or rejection metadata. |
| Product | Roadmaps, launch reviews, customer support patterns, architecture decisions. | Market-war metaphors, user-as-obstacle language, unsupported urgency frames. |
| Operations | Incident review, queues, reliability signals, runbooks, recovery plans. | Emergency metaphors for routine pressure, blame frames during response. |
| Leadership | Decision forums, portfolio reviews, theme-swimlane pilots, accountability reviews. | Sports victory metaphors, coercive alignment, hidden cuts. |
| Learning | Mentoring, practice design, feedback loops, onboarding, review habits. | Childlike metaphors for professionals, performance issues disguised as learning. |

## Promotion Gates

1. Apply `frame-acquisition-method.md` to every new candidate.
2. Assign application pack and excluded packs.
3. Add pack-specific evidence boundary and authority check.
4. Add or update evaluation fixtures for pack behavior.
5. Apply portability and anti-pattern reviews.
6. Decide lifecycle state before catalog or Rust changes.
7. Keep draft/local pack candidates out of default search until lifecycle
   filtering exists.

## Pack Drift Warnings

| Drift | Symptom | Correction |
|---|---|---|
| One-pack dominance | Product or leadership defaults are used for every problem. | Re-select pack from audience and decision context. |
| Domain costume | A generic frame gets renamed with domain jargon but adds no fit. | Return to source relation and action cue. |
| Safety dilution | Pack expansion removes warnings to make suggestions smoother. | Restore evidence boundary, misuse warning, and fallback. |
| Tool overreach | AI-agent pack starts recommending draft frames as accepted. | Require lifecycle filtering and fixture-backed behavior. |
| Leadership theater | Pack frames make strategy sound aligned without changed decisions. | Require changed-decision or explicit hold evidence. |

## Roadmap

| Phase | Work | Exit Criteria |
|---|---|---|
| P0 | Keep current application-pack templates and accepted starter metadata. | Current Rust filters remain accepted-starter only. |
| P1 | Add pack-specific fixture subsets from `starter-fixtures.json`. | Each pack has selection and suppression examples. |
| P2 | Add docs-level excluded-pack notes for reviewed candidates. | Catalog reviewers can explain pack fit and non-fit. |
| P3 | Add lifecycle-aware pack views. | Draft, held, docs-catalog, and accepted entries are separable. |
| P4 | Consider Rust pack metadata beyond accepted starters. | Fixture runner and lifecycle filters pass. |

## Design Consequences

- Domain packs grow by coverage goals, not by volume.
- Leadership and learning packs require stronger human-safety review than
  product and operations packs.
- AI-agent pack is a safety/output contract first, not a creativity mode.
- Domain-pack expansion should wait for rejected-candidate reporting before
  draft frames appear in tool suggestions.
