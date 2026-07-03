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
| VAL-005 | Methodology maintainer | Choose whether a new frame belongs in the catalog. | Apply the theory fit test and role review. | Selection procedure identifies action cue, evidence boundary, and misuse risk. | EVID-010 / EVID-011 | pass |
| VAL-006 | Methodology maintainer | Decide whether a candidate frame is accepted, revised, held, or rejected. | Score the frame with `fit-rubric.md`. | Rubric produces a decision band and hard-stop checks. | EVID-012 / EVID-013 | pass |
| VAL-007 | Methodology maintainer | Check whether a frame transfers to the intended audience. | Apply `audience-transfer.md`. | Guide identifies assumptions, alternates, authority checks, and transfer bands. | EVID-014 | pass |
| VAL-008 | Methodology maintainer | Decide whether a frame should be indexed, held, deprecated, or rejected. | Apply `frame-lifecycle.md`. | Lifecycle state and indexing rule are clear. | EVID-015 | pass |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Downstream AI agent integration | No consumer has adopted `frames-core` yet. | API may need changes after real agent use. | First external or portfolio consumer. |
| Larger frame-pack coverage | Starter catalog is intentionally small. | Search may miss useful frames. | Traffic and walking frame-pack pulses. |
