# Evidence Boundary Schema

Evidence boundary is the field that keeps a frame from pretending to be proof.
It answers: after this frame is used, what still has to be checked?

Use [claim-strength-labels.md](claim-strength-labels.md) when the question is
not "what still needs checking?" but "how strongly are we allowed to state this
claim?"

## Schema Rule

Every accepted indexed frame should carry an evidence boundary.

| Field | Meaning |
|---|---|
| `evidence_boundary` | The concrete evidence, threshold, owner, or judgment that remains outside the analogy. |

The field should be short enough to display with a search result and concrete
enough to guide follow-up.

## Good Boundary Shape

Good evidence boundaries name one or more of:

- threshold,
- owner,
- signal,
- resource,
- constraint,
- protected value,
- restart condition,
- failure consequence,
- authority model.

Examples:

| Frame | Evidence Boundary |
|---|---|
| Red / yellow / green | Check which threshold changed and what would move the status. |
| Four-way stop | Check whether parties are peers or one owner has authority. |
| Detour | Verify the destination is still valid and the new route cost is acceptable. |
| Blind spot | Identify the missing dependency, stakeholder, or signal. |

## Weak Boundary Patterns

| Weak Boundary | Problem | Better |
|---|---|---|
| "Check the data." | Too generic. | Name the threshold, source, or owner. |
| "Make sure it is safe." | No observable evidence. | Name the protected value and failure condition. |
| "Validate assumptions." | Too abstract. | Name the assumption the frame depends on. |
| "Follow up later." | No action path. | Name the next inspection or restart condition. |

## Distinction From Failure Mode

Evidence boundary and failure mode are different.

| Field | Purpose |
|---|---|
| Evidence boundary | What must still be checked outside the analogy. |
| Failure mode | How the analogy can mislead, distort, or harm. |

Example:

- Frame: speed limit.
- Evidence boundary: check which constraint sets the maximum safe pace.
- Failure mode: can be misused as a universal cap instead of a context rule.

## Design Consequence for `frames-core`

`frames-core::FrameEntry` includes `evidence_boundary` as a required static
field. Search results should display it near `action_cue` and `failure_mode` so
AI/tool callers do not use a frame without its evidence obligation.
