# Wave: Foundation Catalog

## Goal

Establish FRAMES as a docs-first methodology repo with a reusable frame schema,
an initial everyday-situation catalog, and traffic/motion examples for goal
tracking and coordination.

## Thesis

Frames are useful when they lower coordination cost and produce better action.
The foundation wave records each analogy as a mapping with explicit limits so
the repo does not become a loose metaphor list.

## Pulse Table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Workspace foundation | complete | Created repo skeleton, docs, skills, and validation contract. |
| 02 | Traffic frame pack | complete | Expanded driving examples and indexed traffic frames for status, coordination, momentum, and risk. |
| 03 | Walking frame pack | pending | Add pace, footing, crowding, fatigue, and wayfinding frames. |
| 04 | Fit rubric | complete | Added scored rubric for accept, revise, hold, and reject decisions. |
| 05 | Frame index crate | complete | Added dependency-free Rust frame index with search and related-frame lookup. |
| 06 | Frame index ergonomics | complete | Added query builder helpers, filters, top-N lookup, and runnable example. |
| 07 | Theory baseline | complete | Added role-reviewed frame theory for jobs, fit, evidence boundaries, and misuse. |

## Success Criteria

- README explains the repo purpose and first catalog.
- Product plan names audience, waves, and non-goals.
- Wave/pulse scaffolding exists.
- Skills exist for future wave, pulse, and research execution.
- `frames-core` exposes a structured frame index for AI/tool lookup.
- Frame index usage is visible through a runnable example.
- Theory defines frame jobs, fit tests, evidence boundaries, and misuse checks.
- Fit rubric scores frame readiness before catalog/index acceptance.
- Traffic and motion examples cover red/yellow/green, four-way stops, yielding,
  merging, detours, and downshifting.
- `cargo test` passes.
- `cargo run --example lookup` passes.
- `git diff --check` passes.
