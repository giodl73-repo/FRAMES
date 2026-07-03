# Theory Gap Audit

This audit asks what FRAMES still lacks after the baseline theory, role reviews,
local imports, empirical protocol, metadata plan, and transfer-aware search
design.

The short answer: FRAMES now has enough theory to keep expanding carefully, but
not enough to scale catalog growth, automate selection, or make strong public
claims. The next gaps are about control: vocabulary, anti-patterns, evaluation
sets, governance, and acceptance decisions.

## Current Strengths

| Strength | Why It Matters |
|---|---|
| Core frame theory | Defines frame parts, jobs, fit tests, evidence boundaries, and misuse risks. |
| Transfer theory | Pushes selection toward relational structure instead of surface similarity. |
| Role review | Lets business, novice, evidence, misuse, and catalog lenses disagree visibly. |
| Claim strength | Prevents local observations and heuristics from sounding validated. |
| Application packs | Separates product, operations, leadership, learning, and AI-agent use. |
| Anti-pattern taxonomy | Gives bad frames reusable rejection classes instead of one-off warnings. |
| Related-frame taxonomy | Separates alternates, fallbacks, conflicts, sequences, and rejected near-misses. |
| Evaluation-set design | Defines fixture types, expected behavior, pass bands, and review procedure. |
| Cultural portability | Adds region, language, mobility, accessibility, and authority checks for everyday sources. |
| Empirical protocol | Gives a path to test narrow claims instead of relying on author confidence. |
| Metadata migration | Keeps Rust API changes behind docs-level proof. |

## Blocking Gaps

These gaps can cause wrong recommendations or premature catalog acceptance.

| Gap | Risk | First Artifact |
|---|---|---|
| Controlled frame ontology | Tags and relation names drift as the catalog grows. | `frame-ontology.md` |
| Anti-pattern taxonomy application | Bad-frame classes now need to be applied to accepted-catalog reviews, evaluation sets, and rejected-candidate reporting. | `frame-antipattern-taxonomy.md` |
| Acceptance review board | Promoted draft heuristics need the accepted-catalog gate applied before default search. | `accepted-catalog-review-process.md` |
| Evaluation-set application | Fixtures now need to be populated and used as gates before semantic search or draft-frame inclusion. | `evaluation-set-design.md` |
| Related-frame relation application | Typed related-frame relations need to be applied to catalog rows, evaluation sets, and future Rust metadata. | `related-frame-taxonomy.md` |

## Growth Gaps

These gaps limit expansion into broader users, domains, and public use.

| Gap | Risk | First Artifact |
|---|---|---|
| Cultural portability application | Portability bands need to be applied to catalog rows, evaluation fixtures, and future query filters. | `cultural-portability.md` |
| Domain pack strategy | New packs could grow opportunistically instead of through coverage goals. | `domain-pack-roadmap.md` |
| Frame acquisition method | The repo lacks a repeatable way to discover and import new frame candidates. | `frame-acquisition-method.md` |
| Plain-language fallback theory | High-stakes or low-transfer situations need non-metaphor alternatives. | `plain-language-fallbacks.md` |
| Teaching progression | Novice, journeyman, and expert use needs a learning path, not just docs. | `learning-progression.md` |

## Implementation Gaps

These gaps block reliable tool behavior.

| Gap | Risk | First Artifact |
|---|---|---|
| AI response examples in Rust | The contract exists in docs but not in runnable example shape. | Rust example using `FrameIndex` output fields. |
| First Rust metadata migration | Accepted docs metadata is not yet tool-readable. | Add compact metadata fields after API review. |
| Transfer-aware query API | Search design exists but query filters are not implemented. | Nonbreaking query filter design. |
| Rejected-candidate reporting | Tools cannot yet explain why a tempting frame was rejected. | Rejection result shape. |
| Lifecycle filtering | Draft, held, deprecated, and accepted states cannot be filtered in code. | Status metadata and filters. |

## Deep Theory Gaps

These are not immediate blockers, but they matter if FRAMES becomes a serious
methodology rather than a useful catalog.

| Gap | Question |
|---|---|
| Frame grammar | What are the reusable building blocks of a frame: actor, constraint, signal, threshold, protected value, feedback, and stop condition? |
| Frame transformation | How does a frame change when it moves from driving to walking, from peer teams to hierarchy, or from status to risk? |
| Misuse recovery | What should a facilitator do after a frame has already harmed a discussion? |
| Narrative dose | How much story is enough to help memory without overwhelming evidence? |
| Frame decay | When do once-useful frames become stale, coercive, cliche, or theater? |
| Multi-frame reasoning | When should a system offer a sequence of frames rather than one best frame? |
| Power and legitimacy | Who is allowed to impose a frame on whom, and when must the audience choose the frame? |

## Recommended Sequence

1. Add AI response contract examples in Rust.
2. Add first Rust metadata migration for accepted starter entries.
3. Apply the accepted-catalog review process to first promoted candidates.
4. Apply frame anti-pattern taxonomy to first promoted candidates and evaluation fixtures.
5. Apply related-frame relation taxonomy to catalog examples and evaluation fixtures.
6. Populate first evaluation fixtures for traffic, anti-pattern, relation, theme-swimlane, and audience-transfer cases.
7. Apply cultural portability bands to traffic, status, story, and theme-swimlane fixtures.
8. Define frame acquisition method.

This sequence keeps tool safety ahead of catalog growth.

## What Not To Do Yet

- Do not add many more draft frames before anti-pattern and acceptance gates are
  applied.
- Do not add semantic search before evaluation sets and rejection examples
  exist.
- Do not claim empirical validation before EVT-001 or another comparison trial
  is run.
- Do not encode every theory field in Rust before accepted catalog rows prove
  the values are stable.
- Do not promote local imports into default search until lifecycle filtering
  exists.

## Decision

FRAMES is no longer missing foundation theory. It is missing scale theory: the
rules that keep a growing frame index coherent, reviewable, portable, and safe
when AI tools start selecting frames automatically.
