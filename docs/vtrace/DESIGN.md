# Design

## Scope

Repo or feature: `frames-core`

## Design Elements

| ID | Design Element | Parent Specs | Description |
|---|---|---|---|
| DES-001 | Static starter catalog | SPEC-001 / SPEC-004 | A dependency-free catalog of `FrameEntry` values. |
| DES-002 | Deterministic scoring | SPEC-002 | Score kind matches, target-situation overlap, tag matches, and name hits. |
| DES-003 | Related lookup | SPEC-003 | Resolve related IDs through the same index. |
| DES-004 | Warning preservation | SPEC-001 | Every candidate carries `failure_mode` with the entry. |
| DES-005 | Ergonomic lookup helpers | SPEC-005 | Provide query builders, top-N search, kind filters, and tag filters without adding dependencies. |

## Design Notes

This is intentionally not semantic search. The first version favors deterministic
matching and inspectability so downstream AI tools can explain why a frame was
found before using it.
