# Related-Frame Application: Starter Catalog

This artifact applies `related-frame-taxonomy.md` to starter catalog links while
keeping Rust `FrameEntry::related` as an untyped compatibility field.

The goal is to make current related IDs explainable before adding typed related
metadata or relation-aware ranking.

## Applied Relation Map

| Source Frame | Current Related Target | Relation | Display Rule | Rationale |
|---|---|---|---|---|
| `four-way-stop` | `merge-lane` | `alternate` | show_as_alternate | Both help peer coordination, but four-way stop emphasizes turn order while merge lane emphasizes timing into active flow. |
| `four-way-stop` | `crosswalk-yield` | `boundary_frame` | show_as_warning | Crosswalk yield marks where peer turn-taking stops because protected-party duty overrides equal turns. |
| `merge-lane` | `following-distance` | `sequence_before` | show_as_sequence | Buffer and reaction time should be checked before joining an active system. |
| `crosswalk-yield` | `merge-lane` | `conflicts_with` | show_as_warning | Merge lane assumes compatible parties; crosswalk yield assigns delay duty to the stronger actor. |
| `blind-spot` | `load-bearing-wall` | `sequence_after` | show_as_sequence | A missing dependency or stakeholder should be identified before deciding whether a dependency is structural. |
| `dashboard-warning-light` | `red-yellow-green` | `safer_fallback` | show_as_fallback | A warning light can preserve diagnostic uncertainty better than a simple status color. |
| `speed-limit` | `following-distance` | `compatible_with` | show_as_alternate | Pace constraint and buffer spacing can compose when safety and coupling are both named. |
| `shoulder-pull-off` | `merge-lane` | `sequence_after` | show_as_sequence | Re-entry into flow comes after stabilization. |

## Rejected Near-Misses

| Tempting Source | Rejected Target Or Fallback | Relation | Display Rule | Rationale |
|---|---|---|---|---|
| Four-way stop for incident response | Incident command language | `conflicts_with` | suppress_by_default | Incident response has owner authority; peer turn-taking would obscure command rights. |
| Team as roadblock | Dependency blocked by unresolved ownership | `rejected_near_miss` | suppress_by_default | The surface metaphor is vivid but turns people into obstacles. |
| Unsupported Veto Rule | Preference or decision-rights language | `plain_language_fallback` | show_as_fallback | Without requirement, owner, and evidence, the frame should fall back to direct accountability language. |

## Fixture Additions

These relation applications justify adding:

- `EVAL-REL-002` for `four-way-stop` to `crosswalk-yield` boundary behavior.
- `EVAL-REL-003` for `merge-lane` to `following-distance` sequence behavior.
- `EVAL-REL-004` for unsupported Veto Rule plain-language fallback behavior.

## Index Implication

Do not add typed relation enums to `frames-core` until at least the starter
relation fixtures are machine-readable and relation behavior can be tested.
Until then, `FrameIndex::related_to` remains a thesaurus-style adjacency lookup.
