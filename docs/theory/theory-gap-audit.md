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
| Frame ontology | Stabilizes tags, jobs, relation names, authority terms, and risk vocabulary before catalog growth. |
| Transfer theory | Pushes selection toward relational structure instead of surface similarity. |
| Role review | Lets business, novice, evidence, misuse, and catalog lenses disagree visibly. |
| Claim strength | Prevents local observations and heuristics from sounding validated. |
| Application packs | Separates product, operations, leadership, learning, and AI-agent use. |
| Anti-pattern taxonomy | Gives bad frames reusable rejection classes instead of one-off warnings. |
| Related-frame taxonomy | Separates alternates, fallbacks, conflicts, sequences, and rejected near-misses. |
| Evaluation-set design | Defines fixture types, expected behavior, pass bands, and review procedure. |
| Cultural portability | Adds region, language, mobility, accessibility, and authority checks for everyday sources. |
| Leadership worksheet | Gives Theme Swimlanes a pilot artifact without accepted-catalog promotion. |
| Empirical protocol | Gives a path to test narrow claims instead of relying on author confidence. |
| Metadata migration | Keeps Rust API changes behind docs-level proof. |

## Blocking Gaps

These gaps can cause wrong recommendations or premature catalog acceptance.

| Gap | Risk | First Artifact |
|---|---|---|
| Acceptance review board operation | Promoted draft heuristics need named board decisions and recorded dissent before default search. | `accepted-catalog-review-process.md` |

## Growth Gaps

These gaps limit expansion into broader users, domains, and public use.

| Gap | Risk | First Artifact |
|---|---|---|
| Public learning path | FRAMES needs a reader-facing path before broad public onboarding. | Start from `learning-progression.md` and package it later. |

## Implementation Gaps

These gaps block reliable tool behavior.

| Gap | Risk | First Artifact |
|---|---|---|
| Relation-aware ranking | Authority, risk, and application-pack filters exist, but target-relation and protected-value ranking are not implemented. | Relation metadata plus evaluation-backed ranking design. |
| Rejected-candidate reporting | First review-only rows back suppressed reports and explicit review modes, but broader rejected-candidate scoring is not implemented. | Add evaluation-backed review scoring before expanding rows. |
| Lifecycle filtering | `search_with_lifecycle` separates suggestions, fallbacks, suppressed reports, and review-only rows, but relation-aware review ordering is not implemented. | Add relation-aware ranking design and fixtures before scoring review rows. |

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

1. Collect EVT-001 participant responses and score the locked packet without upgrading claim strength prematurely.
2. Pilot the theme-swimlane leadership worksheet and record changed decisions.
3. Define relation-aware ranking fixtures before scoring accepted or review-only rows beyond metadata filters.

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
- Do not promote local imports into default search until review-only catalog
  loading and display rules prove they cannot become recommendations by
  accident.

## Decision

FRAMES is no longer missing foundation theory. It is missing scale theory: the
rules that keep a growing frame index coherent, reviewable, portable, and safe
when AI tools start selecting frames automatically.
