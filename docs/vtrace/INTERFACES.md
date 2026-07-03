# Interfaces

## Scope

Repo or feature: `frames-core`

## Rust API

| ID | Surface | Purpose | Stability |
|---|---|---|---|
| IF-001 | `FrameEntry` | Structured frame metadata. | target |
| IF-002 | `FrameKind` | Status, coordination, momentum, and risk categories. | target |
| IF-003 | `FrameQuery` | Situation text, optional kind, and tags. | target |
| IF-004 | `FrameCandidate` | Ranked search result and match notes. | target |
| IF-005 | `FrameIndex::search` | Deterministic candidate lookup. | target |
| IF-006 | `FrameIndex::related_to` | Related-frame lookup by stable ID. | target |

## Non-Interfaces

- VTRACE work package IDs are not public API.
- `.roles` review lenses are not library API.
- Natural-language generation prompts are out of scope for this slice.

