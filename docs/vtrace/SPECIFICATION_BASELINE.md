# Specification Baseline

## Scope

Repo or feature: `frames-core`

## Specification Table

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 / REQ-004 | API / data model | target | `FrameEntry` exposes stable frame metadata, action cue, failure mode, and related IDs. | unit test / inspection | VAL-001 | FRAMES | low | accepted |
| SPEC-002 | REQ-002 | API / behavior | target | `FrameIndex::search` returns deterministic ranked candidates for matching queries and empty results for unmatched queries. | unit test | VAL-001 | FRAMES | medium | accepted |
| SPEC-003 | REQ-003 | API / behavior | target | `FrameIndex::related_to` resolves related entries from stable IDs and returns empty results for unknown IDs. | unit test | VAL-002 | FRAMES | low | accepted |
| SPEC-004 | REQ-005 | package boundary | target | `frames-core` has no third-party runtime dependencies. | manifest inspection / cargo test | VAL-001 | FRAMES | low | accepted |

## Contract Table

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 / SPEC-003 | Rust library API | Public type and method names should change only with a spec update. | Renaming or removing public API. | EVID-001 |
| IF-002 | SPEC-001 | Starter catalog IDs | IDs are stable once published. | Renaming or deleting an ID. | EVID-001 / EVID-004 |

