# Validation

## Scope

Repo or feature: `frames-core`

## Validation Scenarios

| Scenario ID | User / Actor | Need | Workflow | Success Criteria | Evidence Pointer | Result |
|---|---|---|---|---|---|---|
| VAL-001 | AI tool builder | Search frame candidates for a target situation. | Build `FrameQuery`, call `FrameIndex::search`, inspect candidates. | Candidate ranking is deterministic and includes action cues and warnings. | EVID-001 | pass |
| VAL-002 | Methodology maintainer | Explore adjacent frame alternatives. | Call `FrameIndex::related_to` for a stable frame ID. | Related entries resolve from IDs. | EVID-001 | pass |
| VAL-003 | Business or product lead | Review whether a suggested frame is safe to use. | Inspect candidate `action_cue` and `failure_mode`. | Warning remains visible with the frame entry. | EVID-004 | pass |
| VAL-004 | AI tool builder | Try the index from a runnable example. | Run `cargo run --example lookup`. | Example prints ranked candidates and warnings. | EVID-007 | pass |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Downstream AI agent integration | No consumer has adopted `frames-core` yet. | API may need changes after real agent use. | First external or portfolio consumer. |
| Larger frame-pack coverage | Starter catalog is intentionally small. | Search may miss useful frames. | Traffic and walking frame-pack pulses. |
