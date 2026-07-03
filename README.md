# FRAMES

FRAMES is a methodology repo for turning familiar human situations into useful
decision frames.

The repo studies everyday metaphors, analogies, and shared scripts people
already understand, then turns them into reusable ways to explain status,
coordination, risk, momentum, and goal tradeoffs. Red/yellow/green progress
tracking is the seed example: it works because most people already understand
stop, caution, and go without needing a new vocabulary.

## Thesis

Good frames reduce explanation cost. A useful frame makes a situation easier to
see, discuss, and act on without pretending the analogy is the actual system.

FRAMES starts with ordinary human activities:

- Driving: signals, intersections, lanes, speed, yielding, detours, merging.
- Walking: pace, stride, footing, crosswalks, fatigue, crowds, wayfinding.
- Waiting: queues, turns, holds, appointments, service windows.
- Cooking: prep, timing, heat, seasoning, batches, mise en place.
- Building: foundations, scaffolds, load, fit, inspection, punch lists.
- Sports and games: field position, timeouts, fouls, advantage, reset.

## Rust Frame Index

FRAMES also includes `frames-core`, a small Rust library for AI tools and other
software that need a structured frame index. It provides a starter catalog,
query types, ranked candidates, related-frame lookup, action cues, and failure
mode warnings.

```powershell
cargo test
```

Example lookup:

```rust
use frames_core::{FrameIndex, FrameKind, FrameQuery};

let index = FrameIndex::new();
let query = FrameQuery::new("two teams need turn order around constrained attention")
    .with_kind(FrameKind::Coordination)
    .with_tags(&["priority"]);

let candidates = index.search_top(&query, 3);
```

## First Catalog

The initial catalog lives in [docs/frame-catalog.md](docs/frame-catalog.md).
The first worked examples live in
[docs/examples/traffic-and-motion.md](docs/examples/traffic-and-motion.md).
The operating theory lives in [docs/theory/frame-theory.md](docs/theory/frame-theory.md).
The first scoring rubric lives in [docs/theory/fit-rubric.md](docs/theory/fit-rubric.md).
Audience transfer guidance lives in [docs/theory/audience-transfer.md](docs/theory/audience-transfer.md).
Frame lifecycle guidance lives in [docs/theory/frame-lifecycle.md](docs/theory/frame-lifecycle.md).
Composition and conflict guidance lives in [docs/theory/composition-and-conflict.md](docs/theory/composition-and-conflict.md).
Evidence boundary schema lives in [docs/theory/evidence-boundary-schema.md](docs/theory/evidence-boundary-schema.md).
Research grounding lives in [docs/theory/research-grounding.md](docs/theory/research-grounding.md).
Claim-strength labels live in [docs/theory/claim-strength-labels.md](docs/theory/claim-strength-labels.md).
Source-domain taxonomy lives in [docs/theory/source-domain-taxonomy.md](docs/theory/source-domain-taxonomy.md).
Relational transfer fields live in [docs/theory/relational-transfer-fields.md](docs/theory/relational-transfer-fields.md).
Role-reviewed domain examples live in [docs/theory/role-reviewed-domain-examples.md](docs/theory/role-reviewed-domain-examples.md).
Application pack templates live in [docs/theory/application-pack-templates.md](docs/theory/application-pack-templates.md).
Perspective metadata guidance lives in [docs/theory/perspective-metadata.md](docs/theory/perspective-metadata.md).
Story-job taxonomy lives in [docs/theory/story-job-taxonomy.md](docs/theory/story-job-taxonomy.md).
The local RESONANCE MANAGE import map lives in [docs/theory/resonance-manage-import-map.md](docs/theory/resonance-manage-import-map.md).
Structured RESONANCE MANAGE frame imports live in [docs/theory/resonance-manage-frame-imports.md](docs/theory/resonance-manage-frame-imports.md).
The local CAREER Gravity import map lives in [docs/theory/career-gravity-import-map.md](docs/theory/career-gravity-import-map.md).
Structured CAREER Gravity frame imports live in [docs/theory/career-gravity-frame-imports.md](docs/theory/career-gravity-frame-imports.md).
Theme swimlane extraction lives in [docs/theory/theme-swimlane-extraction.md](docs/theory/theme-swimlane-extraction.md).
External frame-practitioner benchmarks live in [docs/theory/external-frame-practitioners.md](docs/theory/external-frame-practitioners.md).
Project review roles live in [.roles/ROLE.md](.roles/ROLE.md).

## Frame Shape

Each frame should name:

| Field | Meaning |
|---|---|
| Everyday source | The familiar situation people already understand. |
| Target situation | The work, goal, risk, or coordination problem being explained. |
| Useful mapping | Which parts of the source transfer well. |
| Action cue | What the frame helps people do next. |
| Failure mode | Where the analogy breaks or misleads. |
| Better fit than | The frames it should replace for this use case. |

## Non-goals

- FRAMES is not a persuasion dark-pattern library.
- FRAMES is not a generic quote or metaphor collection.
- FRAMES does not claim every analogy is culturally universal.
- FRAMES does not replace domain evidence, metrics, or judgment.

## Validation

Current validation:

```powershell
cargo fmt --check
cargo test
cargo run --example lookup
git diff --check
```
