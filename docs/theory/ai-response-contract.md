# AI Response Contract

The AI response contract defines what an assistant, tool, or downstream system
must return when suggesting frames. It connects selection, indexing, scoring,
claim strength, evidence boundaries, lifecycle status, and misuse warnings into
one safe output shape.

This contract is docs-first. `frames-core` should implement it only after the
metadata migration plan promotes the needed fields.

## Contract Goal

An AI frame suggestion should not be a clever metaphor. It should be a bounded
candidate with action, evidence, warning, fit rationale, and alternates.

Minimum promise:

```text
Return a frame only with its action cue, evidence boundary, misuse warning,
claim strength, and reason it fits the target situation.
```

## Required Inputs

| Input | Required? | Meaning |
|---|---|---|
| `target_situation` | yes | Plain description of the situation to clarify. |
| `intended_audience` | recommended | Who will use or hear the frame. |
| `desired_job` | recommended | Status, coordination, momentum, risk, priority, or learning. |
| `application_pack` | optional | Product, operations, leadership, learning, or AI-agent. |
| `risk_context` | recommended | Low, medium, high, or unknown. |
| `known_authority` | optional | Peer, owner, steward, operator, protected party, command, or unknown. |
| `must_avoid` | optional | Source domains, emotional tones, or analogies to exclude. |

If `risk_context` or `known_authority` is unknown, the response must include an
alternate frame or a plain-language fallback.

## Required Output Fields

| Field | Meaning |
|---|---|
| `frame_id` | Stable ID when indexed; draft name when not indexed. |
| `name` | Human-readable frame name. |
| `status` | accepted, draft, held, deprecated, rejected, or unknown. |
| `claim_strength` | illustrative, heuristic, theory_informed, practitioner_observed, locally_observed, role_reviewed, empirically_validated, or anti_pattern. |
| `fit_score` | Human rubric score when available; otherwise omitted with reason. |
| `fit_reason` | Why the frame matches the target relation. |
| `action_cue` | What the user should do next. |
| `evidence_boundary` | What still must be checked outside the frame. |
| `misuse_warning` | How the frame can distort or harm. |
| `authority_check` | Authority assumption that must be verified. |
| `risk_band` | Low, medium, high, rejected, or unknown. |
| `plain_language_fallback` | Direct non-metaphor statement, required for high or unknown risk. |
| `alternates` | Adjacent frames or safer fallback choices. |
| `do_not_use_when` | Hard-stop conditions. |

## Suggested JSON Shape

```json
{
  "target_situation": "...",
  "recommended": {
    "frame_id": "four-way-stop",
    "name": "Four-way stop",
    "status": "accepted",
    "claim_strength": "heuristic",
    "fit_score": 12,
    "fit_reason": "Peer actors need visible turn order around a shared contention point.",
    "action_cue": "Pause, read priority, signal intent, then proceed visibly.",
    "evidence_boundary": "Check whether parties are peers or one owner has authority.",
    "misuse_warning": "Breaks when authority, urgency, or safety rules are not equal.",
    "authority_check": "Use only for peer coordination.",
    "authority_model": "peer",
    "risk_band": "medium",
    "plain_language_fallback": "The teams need an explicit turn order before work proceeds.",
    "do_not_use_when": [
      "one party owns the decision",
      "there is protected-party duty",
      "urgency overrides peer sequencing"
    ]
  },
  "alternates": [
    {
      "frame_id": "crosswalk-yield",
      "name": "Crosswalk yield",
      "use_when": "one party has a duty to absorb delay to protect another value"
    },
    {
      "frame_id": "merge-lane",
      "name": "Merge lane",
      "use_when": "work must join an active flow predictably"
    }
  ],
  "notes": [
    "Frame suggestions explain; they do not prove."
  ]
}
```

## Selection Rules

1. Start with the plain target situation.
2. Identify frame job and application pack.
3. Reject frames with mismatched authority.
4. Prefer accepted frames over drafts.
5. Prefer `heuristic` or stronger claim strength over `locally_observed` unless
   the caller explicitly asks for local/draft patterns.
6. Prefer structural transfer over illustrative transfer.
7. Include evidence boundary and misuse warning in every response.
8. Include alternates for medium, high, unknown, or contested contexts.
9. Include plain-language fallback for high or unknown risk.

Fallback construction follows
[plain-language-fallbacks.md](plain-language-fallbacks.md).
10. Never present a frame as proof.

## Scoring Interpretation

Rust search score and human fit score are different.

| Score Type | Meaning | May Be Used For |
|---|---|---|
| Search score | Lexical/tag/kind match from the index. | Candidate retrieval. |
| Fit score | Human rubric score from `fit-rubric.md`. | Accept/revise/hold/reject decision. |
| Transfer strength | Structural, partial, illustrative, decorative, dangerous. | Safety and ranking gate. |
| Claim strength | Evidence confidence label. | Language strength and display warning. |

AI responses must not treat a high search score as a high fit score.

## Gating Rules

| Condition | Required Behavior |
|---|---|
| High risk | Lead with plain language; use frame as secondary explanation only. |
| Unknown authority | Return at least two candidates with different authority assumptions. |
| Draft or local-observed frame | Label clearly and do not recommend as settled guidance. |
| Held frame | Explain unresolved question; do not present as recommendation. |
| Deprecated frame | Prefer replacement; use only for history or comparison. |
| Rejected / anti-pattern | Use only when caller asks for anti-patterns or misuse examples. |
| No evidence boundary | Do not return the frame. |
| Dangerous transfer | Reject and provide safer plain-language fallback. |

## Language Rules

Allowed:

- "This frame can help inspect..."
- "Use this as a heuristic when..."
- "This is locally observed and still needs review..."
- "The evidence boundary is..."

Disallowed:

- "This proves..."
- "This guarantees..."
- "This will persuade..."
- "Research validates this frame..." unless validation scope is named.
- "The team is the roadblock."

## Response Examples

### Accepted Frame

```text
Use four-way stop as a coordination heuristic if the teams are peers. It suggests:
pause, read order, signal intent, and proceed visibly. Check whether one team
actually owns the decision; if so, use an owner/escalation frame instead.
```

### Draft Local Frame

```text
Theme swimlanes is locally observed, not validated. It may help if the program
has one customer promise and teams need contribution lanes. Define the lane
owner, customer outcome, measure, tradeoff, and exclusion before using it.
```

### High-Risk Fallback

```text
Plain language first: this decision affects employment and needs direct policy,
evidence, and review. A frame can be used later as a secondary explanation, but
not as the decision basis.
```

## Implementation Implications

- `frames-core` exposes the first metadata needed by this contract: status,
  claim strength, authority model, risk band, and application packs.
- Search APIs should distinguish retrieval score from fit score.
- Tool callers should be able to request accepted-only results.
- Draft/local imports should not appear in default search until lifecycle and
  claim-strength filters exist.
- Suppressed candidates should use a separate report channel from
  recommendations, following
  [rust-lifecycle-filter-api-design.md](rust-lifecycle-filter-api-design.md)
  through `FrameIndex::search_with_lifecycle`.
- Examples should test that every suggested frame displays action cue, evidence
  boundary, and misuse warning together.

The first runnable example is `examples/ai_response_contract.rs`. It wraps the
current `FrameIndex` output into the contract shape and reads status, claim
strength, authority model, risk band, and application pack metadata from
`FrameEntry`. The example uses the first transfer-aware query filters. Fields
not yet represented in the API stay explicit instead of being inferred.
