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
| VAL-009 | Methodology maintainer | Decide whether two frames should be composed, sequenced, or kept separate. | Apply `composition-and-conflict.md`. | Primary/secondary/boundary roles and conflict resolution are clear. | EVID-016 | pass |
| VAL-010 | AI tool builder | Display frame suggestion with action, evidence boundary, and warning. | Query `FrameIndex` and inspect `FrameEntry`. | Each returned frame carries `action_cue`, `evidence_boundary`, and `failure_mode`. | EVID-017 / EVID-018 | pass |
| VAL-011 | Methodology maintainer | Decide whether a cognitive-science claim is safe to publish. | Apply `research-grounding.md`. | The claim is either supported with a citation and bounded scope or downgraded to an internal hypothesis. | EVID-019 | pass |
| VAL-012 | Methodology maintainer | Choose a source domain before adding a frame pack. | Apply `source-domain-taxonomy.md` and research-grounding review. | Source family, authority model, temporal shape, risk band, and audience portability are clear. | EVID-020 | pass |
| VAL-013 | Methodology maintainer or AI tool builder | Choose context-specific defaults for frame suggestions. | Apply `application-pack-templates.md`. | Pack choice identifies audience, target situations, default jobs, source families, authority checks, evidence obligations, risk defaults, alternates, and rejection rules. | EVID-021 | pass |
| VAL-014 | Methodology maintainer | Check whether a frame assigns the right listener role and duty. | Apply `perspective-metadata.md`. | Implied role, counterparty role, agency, duty, authority, vulnerability, and perspective risk are visible. | EVID-022 | pass |
| VAL-015 | Methodology maintainer or communicator | Check what narrative purpose a frame is serving. | Apply `story-job-taxonomy.md` and the fit-rubric overlay. | Story job, audience role, vividness/evidence balance, hard stops, and alternates are visible. | EVID-023 | pass |
| VAL-016 | Methodology maintainer or AI tool builder | Check whether a frame transfers by structure rather than surface similarity. | Apply `relational-transfer-fields.md` before scoring transfer clarity. | Source relation, target relation, actor roles, authority, constraints, signals, thresholds, exclusions, distortion risk, and transfer strength are visible. | EVID-024 | pass |
| VAL-017 | Methodology maintainer or public writer | Check whether a frame claim overstates its evidence. | Apply `claim-strength-labels.md` before publishing or agent output. | Label, applies-when scope, evidence, boundary, and risk are visible. | EVID-025 | pass |
| VAL-018 | Methodology maintainer | Test whether the theory produces usable domain decisions. | Apply role-reviewed examples before catalog expansion. | Concrete examples produce accept, revise, hold, or reject outcomes with role pressure visible. | EVID-026 | pass |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Downstream AI agent integration | No consumer has adopted `frames-core` yet. | API may need changes after real agent use. | First external or portfolio consumer. |
| Larger frame-pack coverage | Starter catalog is intentionally small. | Search may miss useful frames. | Traffic and walking frame-pack pulses. |
