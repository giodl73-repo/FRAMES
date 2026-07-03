# Learning Progression

This progression defines how people should learn to use FRAMES across novice,
journeyman, expert, and AI-tool contexts.

The goal is not to make everyone learn the whole theory stack. The goal is to
give each audience the minimum useful model, the right safeguards, and a clear
path to deeper review when risk increases.

## Progression Rule

Teach frame use in layers:

1. Use a clear frame only when it improves action.
2. Name where the frame stops working.
3. Add evidence, owner, authority, and fallback.
4. Compare alternates and rejected near-misses.
5. Apply lifecycle, evaluation, portability, and role review before expansion.

Do not teach advanced taxonomy before a user can explain the action cue and
failure mode in plain language.

## Learner Levels

| Level | Learner Need | Teach First | Do Not Require Yet |
|---|---|---|---|
| Novice | Understand why a frame helps and where it fails. | Source, target, action cue, failure mode, fallback. | Full ontology, VTRACE IDs, lifecycle states. |
| Journeyman | Use frames in meetings and decisions. | Fit checks, evidence boundary, owner, authority, alternates. | Rust metadata, empirical claim rules, full relation taxonomy. |
| Expert | Review, adapt, reject, or compose frames. | Anti-patterns, portability, relation types, role gates, lifecycle. | Code implementation unless changing tool behavior. |
| AI-tool builder | Produce bounded suggestions safely. | Response contract, fixtures, filters, warnings, fallbacks. | Draft-frame inclusion before lifecycle filtering. |

## Novice Path

Novice users should learn:

1. A frame is a comparison, not proof.
2. A useful frame changes what someone checks or does.
3. Every frame needs a failure mode.
4. Plain language is better when stakes, authority, or audience are unclear.
5. The safest first examples are accepted starter frames with visible warnings.

Starter exercises:

| Exercise | Expected Skill |
|---|---|
| Explain red/yellow/green with thresholds. | Avoid status-as-proof. |
| Compare four-way stop with incident owner language. | See authority boundaries. |
| Rewrite "team as roadblock" as dependency language. | Avoid people-as-obstacles. |
| Choose plain language over a weak analogy. | Preserve accountability. |

## Journeyman Path

Journeyman practitioners should learn:

1. Select a frame for a live meeting or planning decision.
2. Name evidence still needed outside the frame.
3. Offer at least one alternate or fallback.
4. Identify when authority or risk makes a frame unsafe.
5. Record changed decisions, not just memorable phrasing.

Meeting checklist:

```text
Target situation:
Audience:
Frame candidate:
Action cue:
Evidence boundary:
Owner / authority:
Failure mode:
Alternate or fallback:
Decision changed:
```

## Expert Path

Expert reviewers should learn:

1. Apply fit rubric and anti-pattern taxonomy.
2. Check cultural portability, perspective, and source-domain risk.
3. Type related-frame links as alternate, fallback, conflict, sequence, boundary,
   or rejected near-miss.
4. Decide lifecycle state and catalog boundary.
5. Require fixtures before tool behavior depends on a frame.

Expert review output:

| Field | Required Decision |
|---|---|
| Fit score | Accept, revise, hold, or reject. |
| Misuse class | Named anti-pattern or none. |
| Portability band | Broad, bounded, limited, unknown, or unsafe. |
| Lifecycle state | Note, draft, reviewed, docs-catalog, accepted, held, rejected. |
| Fixture need | Positive, near-miss, hard-stop, anti-pattern, relation, fallback. |

## AI-Tool Builder Path

AI-tool builders should learn:

1. Search score is retrieval evidence, not fit score.
2. Accepted starter entries are safer than draft imports.
3. Authority, risk, and application-pack filters should gate before ranking.
4. Outputs need action cue, evidence boundary, misuse warning, alternates, and
   plain-language fallback.
5. Rejected and held candidates should not appear in default suggestions until
   lifecycle filtering and rejected-candidate reporting exist.

## Teaching Anti-Patterns

| Anti-Pattern | Why It Fails | Correction |
|---|---|---|
| Theory dump | Starts with all terms before users can act. | Teach source, target, action, failure first. |
| Cleverness reward | Praises memorable frames without evidence or boundary. | Score action and safety, not vividness. |
| Infantilizing learning | Treats capable practitioners as children or broken objects. | Use craft, review, practice, or performance language. |
| Tool magic | Lets AI output frames without showing warnings or fallbacks. | Require response contract and fixtures. |
| Expert gatekeeping | Makes every user learn VTRACE or Rust details. | Keep advanced review behind risk and catalog expansion. |

## Release Criteria

A learner can move forward when they can:

| From | To | Release Check |
|---|---|---|
| Novice | Journeyman | Explain action cue, evidence boundary, failure mode, and fallback for three frames. |
| Journeyman | Expert reviewer | Reject or revise a frame using fit, authority, portability, and anti-pattern checks. |
| Expert reviewer | Catalog contributor | Add fixture expectations and lifecycle state before promotion. |
| AI-tool builder | Tool behavior contributor | Pass fixture checks and preserve response-contract boundaries. |

## Design Consequences

- Docs should expose short learner paths before deep theory documents.
- The learning pack should avoid childlike metaphors for professional audiences.
- Evaluation fixtures should include audience-transfer and learning-progression
  cases.
- Future tooling can present "novice", "meeting", "review", and "tool" views
  without changing the underlying catalog data.
