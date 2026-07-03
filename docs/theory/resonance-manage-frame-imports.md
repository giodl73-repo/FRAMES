# RESONANCE MANAGE Frame Imports

This artifact converts the high-value candidates from
[resonance-manage-import-map.md](resonance-manage-import-map.md) into
FRAMES-native draft entries. These are not accepted catalog entries yet. They
are structured imports with local-source provenance, claim-strength labels,
relational transfer, evidence boundaries, and promotion criteria.

Source claim boundary: all entries start as `locally_observed`. They can move
to `heuristic` only after fit scoring, role review, and stable evidence
boundaries.

## Import Status

| Candidate | Source Pointer | Import Status | Claim Strength | Next Gate |
|---|---|---|---|---|
| Phantom Dissonance | `01-chips.md` | imported draft | `locally_observed` | Conflict-repair workflow review. |
| Board Session | `02-nobody-likes-to-be-told.md` | imported draft | `locally_observed` | Coordination fit score. |
| Advocacy Baseline | `03-your-people.md` | imported draft | `locally_observed` | Feedback/learning role review. |
| Off-Axis Contact | `05-the-friction.md` | imported draft | `locally_observed` | Misuse and authority review. |
| Contact Structure | `09-the-org-chart.md` | imported draft | `locally_observed` | Leadership/operations pack review. |

## Phantom Dissonance

| Field | Draft Entry |
|---|---|
| Source scene | Someone interprets a shared-food situation through the wrong ownership model. |
| Target situation | A team conflict where each side may be reacting to a false model of ownership, scarcity, or threat. |
| Frame job | Risk / coordination. |
| Story job | Repair / empathy. |
| Claim strength | `locally_observed`. |
| Relational transfer | Initial interpretation changes when missing evidence changes the ownership model. |
| Action cue | Pause blame, inspect the ownership/fact model, retell the situation from both perspectives, then decide repair. |
| Evidence boundary | Check facts, ownership, harm, and accountability before treating the conflict as misunderstanding. |
| Failure mode | Can pressure forgiveness or minimize real harm if used before facts and repair obligations are clear. |
| Transfer exclusions | Not every conflict is a misunderstanding; do not erase power, harm, or repeated behavior. |
| Application pack | Leadership / operations / learning. |
| Draft status | Hold as story-supported frame until conflict-repair workflow is explicit. |

Promotion criteria:

- Add a concrete repair workflow.
- Add authority and harm checks.
- Fit rubric score reaches accepted band for low/medium-stakes conflict use.
- Misuse Risk Reviewer accepts that accountability is preserved.

## Board Session

| Field | Draft Entry |
|---|---|
| Source scene | Multiple options are placed on a shared board so people can compare them before choosing. |
| Target situation | A leader, product team, or partner group needs visible alternatives before deciding. |
| Frame job | Coordination / learning. |
| Story job | Teaching / simulation or decision / tradeoff. |
| Claim strength | `locally_observed`. |
| Relational transfer | Hidden models become visible options; comparison improves when alternatives share one surface. |
| Action cue | Put the candidate pitches, models, or plans side by side before asking for commitment. |
| Evidence boundary | Check whether the board includes the real options, decision owner, criteria, and dissent path. |
| Failure mode | Can become performative inclusion if the decision is already fixed. |
| Transfer exclusions | A shared board does not imply equal authority or consensus. |
| Application pack | Product / leadership / AI-agent. |
| Draft status | Revise toward accepted coordination frame. |

Promotion criteria:

- Add selection criteria and decision-owner fields.
- Add alternate for situations where the decision is already constrained.
- Business Leader confirms the frame clarifies a real choice.

## Advocacy Baseline

| Field | Draft Entry |
|---|---|
| Source scene | A person can receive hard feedback better when they know the speaker is for them. |
| Target situation | Coaching, performance, or development feedback where correct advice may fail without trust. |
| Frame job | Learning / risk. |
| Story job | Trust / identity and teaching / simulation. |
| Claim strength | `locally_observed`. |
| Relational transfer | Feedback receptivity depends on whether the listener believes the speaker is invested in their success. |
| Action cue | Establish advocacy and shared goal before giving corrective feedback. |
| Evidence boundary | Check whether advocacy is real, feedback is specific, and the listener has agency. |
| Failure mode | Can be manipulative if advocacy is performed only to make criticism easier to deliver. |
| Transfer exclusions | Support does not remove the need for standards, consequences, or clear expectations. |
| Application pack | Learning / leadership. |
| Draft status | Revise toward accepted learning frame. |

Promotion criteria:

- Add examples for novice, journeyman, and leader audiences.
- Add feedback specificity and agency checks.
- Evidence Boundary Reviewer confirms the frame does not soften needed standards.

## Off-Axis Contact

| Field | Draft Entry |
|---|---|
| Source scene | Two parties build contact on a lower-conflict axis before returning to the high-conflict axis. |
| Target situation | Inherited teams, tense partnerships, or transition contexts where direct confrontation would harden resistance. |
| Frame job | Coordination / momentum. |
| Story job | Trust / identity and repair / empathy. |
| Claim strength | `locally_observed`. |
| Relational transfer | Trust can grow through a low-threat shared surface before the contested surface is addressed. |
| Action cue | Find a real low-conflict collaboration point, build reliable contact there, then return to the harder issue. |
| Evidence boundary | Check whether the low-conflict axis is genuine, relevant, and not avoidance. |
| Failure mode | Can become conflict avoidance or side-channel manipulation. |
| Transfer exclusions | Do not use low-conflict contact to bypass rightful owners, suppress dissent, or delay necessary decisions. |
| Application pack | Leadership / operations. |
| Draft status | Hold for authority and misuse review. |

Promotion criteria:

- Add owner and consent checks.
- Add stop condition for returning to the main conflict.
- Misuse Risk Reviewer confirms it does not hide accountability.

## Contact Structure

| Field | Draft Entry |
|---|---|
| Source scene | A network map shows who actually sees, hears, and influences work, not only formal reporting lines. |
| Target situation | A manager or operator needs to understand real information flow across a team or organization. |
| Frame job | Risk / coordination. |
| Story job | Decision / tradeoff or warning. |
| Claim strength | `locally_observed`. |
| Relational transfer | Formal structure and actual contact structure differ; risk hides where the real signal path differs from the chart. |
| Action cue | Map who actually exchanges information, decisions, trust, and escalation signals. |
| Evidence boundary | Check observed contact, decision paths, escalation routes, and missing voices. |
| Failure mode | Can become surveillance or informal power mapping if used without purpose and boundaries. |
| Transfer exclusions | Informal contact does not override formal accountability or privacy obligations. |
| Application pack | Leadership / operations / AI-agent. |
| Draft status | Revise toward leadership/operations example. |

Promotion criteria:

- Add privacy and consent boundaries.
- Add visible output template for contact maps.
- Business Leader confirms it clarifies risk, owner, escalation, or information flow.

## Cross-Import Lessons

- RESONANCE MANAGE provides many strong management patterns, but FRAMES should
  import structure rather than prose.
- Most imports start at `locally_observed`, even when they feel practically
  useful.
- Conflict and leadership frames need stronger misuse warnings than traffic or
  status examples.
- The next crate metadata candidates are `claim_strength`, `transfer_strength`,
  `application_pack`, and `draft_status`, not longer prose fields.
- `Phantom Dissonance`, `Advocacy Baseline`, and `Off-Axis Contact` should not
  become default AI suggestions until authority, harm, and accountability
  boundaries are explicit.
