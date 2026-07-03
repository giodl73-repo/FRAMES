# RESONANCE MANAGE Import Map

This note maps reusable frame candidates from the local RESONANCE MANAGE guide
into FRAMES terms. It is an import map, not a copy source. FRAMES should borrow
structure, action cues, evidence boundaries, and failure conditions rather than
chapter prose.

Local source corpus:

- `C:\src\RESONANCE\guides\books\manage\index.md`
- `C:\src\RESONANCE\guides\books\manage\01-chips.md`
- `C:\src\RESONANCE\guides\books\manage\02-nobody-likes-to-be-told.md`
- `C:\src\RESONANCE\guides\books\manage\03-your-people.md`
- `C:\src\RESONANCE\guides\books\manage\04-the-team.md`
- `C:\src\RESONANCE\guides\books\manage\05-the-friction.md`
- `C:\src\RESONANCE\guides\books\manage\06-hard-conversations.md`
- `C:\src\RESONANCE\guides\books\manage\07-adult-in-the-room.md`
- `C:\src\RESONANCE\guides\books\manage\08-the-train.md`
- `C:\src\RESONANCE\guides\books\manage\09-the-org-chart.md`
- `C:\src\RESONANCE\guides\books\manage\10-reading-the-room.md`

## Candidate Frames

| Candidate | Source | Frame Job | Source Family | Core Use |
|---|---|---|---|---|
| Phantom Dissonance | `01-chips.md` | Coordination / risk | Food sharing / ownership | Real tension caused by a false model of ownership, scarcity, or threat. |
| Deferred D | `01-chips.md` | Risk / coordination | Unopened bag / hidden boundary | A known unresolved boundary is avoided because naming it creates commitment. |
| Heating the Pot | `01-chips.md` | Risk / escalation | Cooking / heat | Continued evaluation extends another team's commitment and raises escalation cost. |
| Board Session | `02-nobody-likes-to-be-told.md` | Coordination / learning | Shared board / visible options | Put multiple pitches or models on the same surface before deciding. |
| Consent License | `02-nobody-likes-to-be-told.md` | Coordination | Invitation / consent | A change lands differently when the affected people invited the next step. |
| Attributed Contribution | `02-nobody-likes-to-be-told.md` | Learning / trust | Credit loop | Acknowledge the work that shaped a decision so the loop closes. |
| Stored R Question | `03-your-people.md` | Learning / momentum | Career trajectory | Ask where the person stopped believing the path pointed somewhere. |
| Advocacy Baseline | `03-your-people.md` | Learning / risk | Ally signal | Feedback becomes receivable only after the person knows you are for them. |
| High-R Exit | `03-your-people.md` | Learning / momentum | Transfer / route change | Help someone move to better-fit work when the current role is not the path. |
| Hiring Delta | `04-the-team.md` | Priority / risk | Portfolio fit | Ask whether a candidate improves this team, not whether they are excellent in general. |
| Genuine vs. Instrumental R | `04-the-team.md` | Risk / trust | Relationship quality | Distinguish real interest from interest performed to obtain an outcome. |
| Hostile Inheritance | `05-the-friction.md` | Risk / coordination | Inherited territory | A new owner represents prior organizational force before they represent themselves. |
| Scared Inheritance | `05-the-friction.md` | Risk / coordination | Uncertain handoff | The inherited team is anxious rather than hostile; surface concerns before acting. |
| Off-Axis Contact | `05-the-friction.md` | Coordination / momentum | Side path | Build trust on a low-conflict axis before increasing contact on the high-conflict axis. |
| Onion Protocol | `05-the-friction.md` | Learning / coordination | Layered approach | Approach a deep model from pitch to use case to spec to implementation. |
| Recoverable Loss | `06-hard-conversations.md` | Priority / risk | Forked path | When both options hurt, choose by which loss can be recovered from. |
| Adult in the Room | `07-adult-in-the-room.md` | Priority / coordination | Vacuum filling | In a destination vacuum, someone must name a direction others can coordinate around. |
| The Train | `08-the-train.md` | Priority / momentum | Train / capacity | Make resource allocation visible so tradeoffs become accountable. |
| Contact Structure | `09-the-org-chart.md` | Risk / coordination | Network map | The org chart shows reporting; the contact structure shows what people actually see. |
| Soft Signals | `10-reading-the-room.md` | Risk / status | Room reading | Treat ease, silence, and softened questioning as signals that need calibration. |

## High-Value Imports

These should become first-class FRAMES entries before the broader MANAGE set:

1. **Phantom Dissonance**: already represented in
   [perspective-metadata.md](perspective-metadata.md).
2. **Board Session**: strong candidate for a coordination frame because it turns
   hidden models into visible options.
3. **Advocacy Baseline**: strong candidate for a learning/feedback frame because
   it explains why correct feedback can fail.
4. **Off-Axis Contact**: strong candidate for merger, inheritance, and hostile
   transition situations.
5. **Contact Structure**: strong candidate for leadership and operations packs
   because it separates reporting lines from information flow.

The first structured draft imports are in
[resonance-manage-frame-imports.md](resonance-manage-frame-imports.md).

## Import Rules

- Preserve the pattern name only when it is structurally useful.
- Rewrite examples into FRAMES-native language.
- Add an evidence boundary for every imported pattern.
- Add a failure condition for every imported pattern.
- Prefer docs-only import before adding crate entries.
- Do not treat RESONANCE terminology as public research evidence.

## Candidate Entry Shape

Each imported frame should eventually answer:

| Field | Required Question |
|---|---|
| Source scene | What familiar situation makes the pattern legible? |
| Target situation | What management or team situation is clarified? |
| Transfer map | Which relationships transfer? |
| Perspective | Who is the listener in the source scene? |
| Action cue | What should the user do next? |
| Evidence boundary | What still has to be checked? |
| Failure mode | When does this pattern mislead? |
| Application pack | Product, operations, leadership, learning, AI-agent, or mixed? |

## Next Import Candidates

| Candidate | Needed Artifact |
|---|---|
| Board Session | Coordination frame entry and example. |
| Advocacy Baseline | Learning/feedback frame entry and misuse warning. |
| Off-Axis Contact | Source-domain and perspective review. |
| Contact Structure | Leadership/operations example. |
| The Train | Priority/resource allocation example. |
