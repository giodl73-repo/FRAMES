# Architecture

## Scope

Repo or feature: `frames-core`

## Architecture Elements

| ID | Element | Responsibility | Boundary |
|---|---|---|---|
| ARCH-001 | `frames-core` crate | Provide structured frame index types and deterministic lookup. | No external service, LLM, or runtime dependency. |
| ARCH-002 | Starter catalog | Seed the index from current FRAMES docs. | Not a complete ontology or universal culture model. |
| ARCH-003 | VTRACE package | Control requirements, specs, evidence, and review. | Not part of the public API. |

## Dependency Posture

The first implementation has no third-party dependencies. Future richer search
or serialization support must be introduced through a new work package and
should keep the core index usable without AI-provider dependencies.

