# FRAMES Product Plan

## Product Thesis

Teams lose time when they lack shared language for fuzzy situations. FRAMES
creates a practical catalog of analogy patterns that help people reason about
goals, progress, coordination, risk, and intervention timing.

The product wedge is a small, inspectable method library and Rust frame index:
pick a target situation, search related frames like a thesaurus, inspect the
mapping, and get a usable communication frame with explicit limits.

## Audience

- Product and engineering leads explaining project state.
- Operators coordinating decisions under uncertainty.
- Designers shaping dashboards, status models, and onboarding language.
- AI methodology work that needs low-friction human-readable steering frames.

## Core Questions

1. Which everyday situations are most widely understood?
2. Which parts of those situations map cleanly to goal-directed work?
3. Which action cues do frames create?
4. Where do frames overfit, trivialize, or mislead?
5. How can frames be compared without turning them into slogans?

## Initial Waves

| Wave | Outcome |
|---|---|
| Foundation catalog | Establish frame schema, traffic/motion examples, and validation rules. |
| Frame index library | Add a dependency-free Rust crate with structured entries, search, related-frame lookup, and warnings. |
| Situation taxonomy | Expand everyday source domains and classify action cues. |
| Fit and misuse rubric | Score whether a frame clarifies, distorts, or hides the real issue. |
| Application packs | Produce goal-tracking, risk-review, and coordination frame packs. |

## First Pulses

| Pulse | Title | Outcome |
|------:|---|---|
| 01 | Workspace foundation | Create docs, catalog, skills, and validation contract. |
| 02 | Traffic frame pack | Deepen four-way stop, yielding, merge, signal, and detour frames. |
| 03 | Walking frame pack | Add pace, footing, stride, crowding, wayfinding, and fatigue frames. |
| 04 | Fit rubric | Add frame selection and misuse checks. |
| 05 | Frame index crate | Add `frames-core` for AI/tool lookup of related frames and warnings. |
| 06 | Theory baseline | Define frame jobs, fit tests, evidence boundaries, and misuse patterns. |

## Dependency Placement

FRAMES belongs in AI Methodology. The foundation crate should remain
dependency-free. It may later consume SIGNALS language for decision intelligence
or PANEL-style review, but it should not depend on either for the frame index
foundation.

## Non-goals

- No AI generation engine in the foundation wave; the Rust crate is an index and
  search surface.
- No claim that traffic metaphors fit every culture or audience.
- No replacement of quantitative status metrics.
- No manipulative framing or sales-script optimization.
