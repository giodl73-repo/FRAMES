# Transfer-Aware Search Design

Transfer-aware search ranks frames by structural fit before surface wording. It
asks whether the source relation, target relation, authority model, constraint,
protected value, and exclusions fit the situation before rewarding a familiar or
vivid metaphor.

This is a design artifact, not a Rust API change. The current `frames-core`
search remains deterministic lexical lookup over kind, target situations, tags,
and name. Transfer-aware search should be added only after enough accepted
catalog entries have stable transfer metadata.

## Current Search Shape

`FrameIndex::search` currently scores:

| Signal | Current Source | Strength | Limitation |
|---|---|---|---|
| Frame kind | `FrameQuery::desired_kind` | Strong deterministic filter/rank signal. | Does not distinguish authority or risk fit. |
| Target text overlap | `FrameEntry::target_situations` | Useful for simple lookup. | Can match words without matching relations. |
| Tags | `FrameEntry::tags` and query tags | Flexible bridge metadata. | Tags can become vague or overloaded. |
| Name hit | `FrameEntry::name` | Helps direct lookup. | Rewards remembered names even when transfer is wrong. |

This is acceptable for the starter index. It is not enough for larger catalog
use, local draft imports, leadership packs, or high-stakes suggestions.

## Search Goal

Transfer-aware search should prefer a less flashy frame with the right
structure over a vivid frame with the wrong authority or evidence boundary.

Example:

```text
Query: one team must slow down to protect a vulnerable customer group.
Prefer: Crosswalk yield.
Avoid: Four-way stop, because the parties are not peers.
```

## Query Inputs

Add these inputs only after docs metadata is stable:

| Input | Meaning | Why It Matters |
|---|---|---|
| Target relation | The relation the user needs clarified: peer coordination, protected-party duty, threshold status, structural dependency, etc. | Prevents source-scene word matching from driving selection. |
| Authority relation | Peer, owner, steward, operator, protected party, command, reviewer, or mixed. | Blocks frames that import the wrong power model. |
| Constraint | Time, capacity, safety, budget, attention, dependency, trust, or uncertainty. | Helps distinguish pace, risk, and coordination frames. |
| Protected value | What must not be sacrificed. | Keeps optimization frames from overriding safety, fairness, trust, or accountability. |
| Risk tolerance | Low, medium, or high. | Raises evidence and fallback requirements. |
| Application pack | Product, operations, leadership, learning, or AI-agent. | Applies context-specific defaults and rejection rules. |
| Excluded transfers | Relations the caller explicitly does not want. | Lets users reject blame, command, competition, or people-as-obstacle patterns. |

## Entry Metadata

Transfer-aware ranking needs compact metadata derived from
[relational-transfer-fields.md](relational-transfer-fields.md):

| Entry Field | Source Theory | API Timing |
|---|---|---|
| `transfer_strength` | Structural, partial, illustrative, decorative, dangerous. | Docs now, API after accepted entries are populated. |
| `authority_model` | Peer, owner, steward, operator, protected party, command, reviewer, mixed. | Candidate for early API metadata. |
| `target_relation` | The relation clarified by the frame. | Keep docs-only until value set stabilizes. |
| `constraint_relation` | The main constraint the frame handles. | Keep docs-only or tag-backed initially. |
| `protected_value` | The value the frame must preserve. | Docs first; display field before ranking field. |
| `transfer_exclusions` | What must not carry over. | Display field before scoring field. |
| `risk_band` | Low, medium, high. | Candidate for early API metadata. |
| `application_packs` | Contexts where the frame is usually useful. | Candidate for early API metadata. |

Do not encode full prose transfer maps into the first Rust search update.
Prefer compact enums and short display fields.

## Scoring Order

When metadata exists, scoring should move toward this order:

| Order | Signal | Effect |
|---:|---|---|
| 1 | Hard-stop rejection | Remove dangerous transfers, disallowed authority models, or high-risk frames without fallback. |
| 2 | Frame kind | Preserve the user's requested job: status, coordination, momentum, or risk. |
| 3 | Target relation | Reward relation match more than word overlap. |
| 4 | Authority compatibility | Reward peer/owner/protected-party fit; penalize wrong authority imports. |
| 5 | Constraint compatibility | Reward matching time, capacity, safety, budget, attention, dependency, trust, or uncertainty constraints. |
| 6 | Protected value compatibility | Reward frames that preserve the user's named value. |
| 7 | Transfer strength | Prefer structural over partial, partial over illustrative. |
| 8 | Application pack fit | Reward context-appropriate defaults. |
| 9 | Text and tag match | Use lexical signals as secondary evidence. |
| 10 | Name hit | Use only as a small direct-lookup boost. |

Search should return both a retrieval score and a fit rationale. Retrieval score
answers "why this matched"; fit rationale answers "why this is safe enough to
consider."

## Hard Stops

Transfer-aware search must reject or demote when:

- query authority is asymmetric but the frame requires peer authority;
- a frame assigns duty to the protected party rather than the stronger actor;
- a speed, efficiency, or momentum frame would override a protected value;
- the transfer strength is `dangerous`;
- high-risk use lacks an evidence boundary and plain-language fallback;
- the only match is source-scene familiarity or story appeal.

## Output Notes

Every transfer-aware candidate should expose:

```text
Frame:
Retrieval score:
Fit rationale:
Matched relations:
Authority fit:
Evidence boundary:
Transfer exclusions:
Misuse warning:
Safer alternate, if any:
```

This shape aligns with [ai-response-contract.md](ai-response-contract.md): AI
callers should show action, evidence, warning, claim strength, and alternates
instead of returning a bare metaphor name.

## Migration Path

1. Keep current Rust search unchanged.
2. Normalize docs-level `authority_model`, `risk_band`, `transfer_strength`, and
   `application_packs` for accepted starter frames.
3. Add Rust display metadata for accepted frames before ranking changes.
4. Add query filters for authority, risk, and application pack.
5. Add relation-aware ranking only after accepted entries have enough stable
   relation metadata.
6. Keep draft, held, and local-import frames out of default search until
   lifecycle filtering exists.

## Open Design Questions

| Question | Default Answer |
|---|---|
| Should relation metadata be enums or strings? | Start with enums only for stable low-cardinality fields; keep target relation strings in docs. |
| Should transfer strength affect search before authority filters exist? | No. Authority mismatch is a more serious failure than weak transfer strength. |
| Should vivid source familiarity boost ranking? | Only after hard stops, authority fit, relation fit, and evidence boundary checks. |
| Should search include draft heuristics? | Not in default search. Add opt-in lifecycle filters later. |
| Should AI callers receive rejected candidates? | Only when explaining safer alternates or hard-stop warnings. |

## Design Consequences

- The next Rust metadata migration should prioritize `risk_band`,
  `application_packs`, and `authority_model` before full relation scoring.
- Accepted catalog rows need stable transfer metadata before the crate can rank
  by transfer fit.
- Related-frame lookup should eventually use
  [related-frame-taxonomy.md](related-frame-taxonomy.md), not just shared source
  family or manually curated IDs.
- Transfer-aware search should make wrong-frame rejection as visible as
  right-frame recommendation.
