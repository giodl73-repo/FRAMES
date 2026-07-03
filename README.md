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

## First Catalog

The initial catalog lives in [docs/frame-catalog.md](docs/frame-catalog.md).
The first worked examples live in
[docs/examples/traffic-and-motion.md](docs/examples/traffic-and-motion.md).
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
cargo test
git diff --check
```
