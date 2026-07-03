# CONOPS

## Scope

Repo or feature: `frames-core`

## Operating Scenarios

| ID | Actor | Scenario | Expected Outcome |
|---|---|---|---|
| CON-001 | AI tool | Search for a frame that fits a target situation and optional tags. | Tool receives ranked candidates with action cues and failure modes. |
| CON-002 | Methodology maintainer | Inspect related frames for a candidate. | Maintainer sees nearby alternatives instead of a single forced metaphor. |
| CON-003 | Product or business lead | Review a suggested frame before using it in status or planning. | Leader can see the decision cue and where the frame breaks. |

## Normal Workflow

1. Caller constructs a `FrameQuery`.
2. `FrameIndex::search` returns ranked `FrameCandidate` values.
3. Caller inspects `entry.action_cue` and `entry.failure_mode`.
4. Caller may use `FrameIndex::related_to` to compare alternatives.
5. Human or AI response uses the frame only after checking fit and warnings.

## Off-Nominal Workflow

| Situation | Expected Behavior |
|---|---|
| Query has no match | Return an empty candidate list. |
| Frame ID is unknown | Return `None` or an empty related list. |
| Candidate has a risky failure mode | Surface the warning with the candidate. |

