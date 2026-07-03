# Mission

## Scope

Repo or feature: `frames-core`

VTRACE adoption scope: control the first Rust frame index for AI/tool lookup of
structured analogy frames.

## Mission Need

FRAMES needs a reusable index that lets AI tools search, compare, and cite
candidate frames without treating metaphor selection as free-form generation.
The index should act like a thesaurus for frames: find related options, show the
action cue, and preserve warnings about where an analogy misleads.

## Users

| User | Need | Success Signal |
|---|---|---|
| AI tool builder | Query frame candidates for a target situation. | Search returns ranked structured entries. |
| Methodology maintainer | Keep frame catalog and implementation aligned. | Starter catalog entries have docs and code counterparts. |
| Business or product lead | Inspect why a frame was suggested. | Candidate includes action cue and failure mode. |

## Operating Context

The library is local-first, dependency-free, and intended for downstream agents
or tools that need deterministic frame lookup before any natural-language
response is drafted.

## Constraints

- Keep the first crate dependency-free.
- Keep entries structured and inspectable.
- Preserve failure modes and misuse warnings.
- Do not build an AI generation engine.
- Do not treat frames as proof or evidence.

## Non-Goals

- No vector database or embedding search in this slice.
- No LLM prompting layer.
- No external service.
- No claim that the starter catalog is complete.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| `frames-core` exposes a searchable index. | Rust unit tests. | EVID-001 |
| Search returns ranked candidates with match notes. | Rust unit tests. | EVID-001 |
| Related-frame lookup works from stable IDs. | Rust unit tests. | EVID-001 |
| Docs and VTRACE package pass repository checks. | `git diff --check`; VTRACE validation. | EVID-002, EVID-003 |

