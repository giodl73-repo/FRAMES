# Frame Ontology

The frame ontology defines controlled vocabulary for catalog growth. It keeps
tags, jobs, relations, source domains, risk labels, and output obligations from
drifting as more frames are imported.

This is not a full semantic model. It is the first naming discipline for a
frame index that humans and AI tools can inspect.

## Ontology Rule

Use a controlled term when a value affects selection, filtering, safety,
review, or display. Use prose only for explanation, examples, and source notes.

A new term should be added only when:

- at least two candidate frames need it;
- it changes selection or rejection behavior;
- it is distinct from existing terms;
- role reviewers can apply it without extra author context;
- a safer fallback or hard-stop implication is clear when relevant.

## Core Entity Types

| Entity | Meaning | Examples |
|---|---|---|
| Frame | A reusable source-target comparison with action, boundary, and warning. | Four-way stop, Crosswalk yield |
| Source scene | The familiar situation borrowed for structure. | Drivers at a four-way stop |
| Target situation | The work, decision, conflict, or learning situation being clarified. | Peer teams needing turn order |
| Frame job | What the frame is meant to help the audience do. | coordination, risk, status |
| Transfer relation | The structural mapping that justifies the frame. | peer turn-taking, protected-party duty |
| Evidence boundary | What must be checked outside the frame. | whether one party owns the decision |
| Misuse warning | How the frame can distort or harm. | hides authority asymmetry |
| Fallback | Plain-language or safer alternate response. | state the decision right directly |

## Controlled Frame Jobs

These are the high-level jobs the index should use before inventing new tags.

| Job | Use When | Do Not Use For |
|---|---|---|
| `status` | Showing condition, readiness, threshold, or alert state. | Explaining who should act next. |
| `coordination` | Ordering action among actors with shared constraints. | Command decisions with one accountable owner. |
| `momentum` | Adjusting pace, flow, recovery, or sequencing. | Justifying speed without evidence. |
| `risk` | Surfacing hazard, exposure, uncertainty, or irreversible cost. | Creating fear without a decision path. |
| `priority` | Choosing what wins under constraint. | Ranking everything as equally important. |
| `capacity` | Matching work to available attention, energy, staffing, or load. | Treating people as interchangeable resources. |
| `learning` | Helping a novice understand, practice, or transfer a concept. | Hiding evaluation or performance judgment. |
| `trust` | Repairing credibility, expectations, or legitimacy. | Avoiding accountability for harm. |

Rust currently exposes the first four jobs as `FrameKind`. Additional jobs stay
docs-level until catalog rows prove stable.

## Controlled Relation Terms

Use relation terms to describe why a frame fits structurally.

| Relation | Meaning | Example Frame | Main Failure |
|---|---|---|---|
| `peer_turn_taking` | Actors have roughly equal standing and need visible order. | Four-way stop | One actor actually owns the decision. |
| `protected_party_duty` | One actor should absorb delay or cost to protect another. | Crosswalk yield | Treats protection as optional courtesy. |
| `threshold_signal` | A visible state indicates when action should change. | Red / yellow / green | Status label replaces evidence. |
| `flow_joining` | Work must enter an active stream without disrupting it. | Merge lane | Approval gates are hidden. |
| `pace_adjustment` | The right move is to slow, recover, or downshift. | Downshift | Slowness becomes avoidance. |
| `dependency_integrity` | A structural support should not be changed casually. | Load-bearing wall | All change is treated as dangerous. |
| `attention_limit` | Actors need to respect finite attention or visibility. | Blind spot | Responsibility is shifted away from the operator. |
| `reserve_tracking` | Remaining capacity should guide timing and commitment. | Fuel gauge | Numeric display implies false precision. |

## Authority Vocabulary

Authority terms should match `source-domain-taxonomy.md` and `frames-core`
metadata.

| Term | Meaning | Selection Implication |
|---|---|---|
| `peer` | Actors have comparable decision standing. | Peer frames may apply. |
| `operator` | Actor controls a system or action path. | Action cue can assign operational responsibility. |
| `steward` | Actor protects shared conditions or long-term health. | Frame should preserve system or community value. |
| `protected_party` | Actor bears vulnerability or deserves priority protection. | Stronger actor may need to yield. |
| `owner` | Actor has decision rights and accountability. | Peer turn-taking frames may fail. |
| `reviewer` | Actor checks blind spots, evidence, or readiness. | Frame should emphasize inspection and boundary. |
| `command` | One accountable actor directs urgent coordination. | Requires explicit command context; not in starter Rust enum yet. |
| `mixed` | Multiple authority models are present. | Return alternates or ask for clarification. |

## Risk Vocabulary

| Term | Meaning | Output Obligation |
|---|---|---|
| `low` | Reversible or learning-oriented use. | Display warning; frame can lead. |
| `medium` | Affects priority, budget, staffing, public commitment, or cross-team action. | Include evidence boundary and alternate. |
| `high` | Affects safety, employment, legal, medical, civic, identity, or irreversible decisions. | Plain language first; frame only as secondary explanation. |
| `unsafe` | The frame imports coercion, dehumanization, false authority, or unsupported certainty. | Reject and provide fallback. |
| `unknown` | Stakes or authority are unclear. | Ask for clarification or return multiple authority assumptions. |

Rust currently exposes low, medium, and high as `RiskBand`. `unsafe` and
`unknown` are output states, not accepted starter metadata.

## Tag Rules

Tags remain useful for flexible search, but they should not become a private
ontology. Prefer these tag families:

| Family | Pattern | Examples |
|---|---|---|
| Job tag | high-level frame job when not first-class. | `priority`, `capacity`, `trust` |
| Relation tag | structural transfer relation. | `peer_turn_taking`, `threshold_signal` |
| Domain tag | source domain or everyday scene. | `traffic`, `walking`, `maintenance` |
| Safety tag | risk or failure concern. | `authority`, `evidence`, `misuse` |
| Pack tag | application context. | `product`, `operations`, `leadership` |

Do not use tags for:

- private jokes or local shorthand;
- source-scene charm with no selection effect;
- duplicate names that differ only by tense or plural;
- claims of validation not backed by a validation report.

## New Term Admission

When adding a controlled term, record:

| Field | Question |
|---|---|
| Term | What exact lowercase identifier should be used? |
| Definition | What does it mean in one sentence? |
| Contrasts | Which nearby terms does it not mean? |
| Selection effect | How does it change lookup, review, warning, or fallback behavior? |
| Example | Which frame uses it first? |
| Misuse risk | What goes wrong if it is applied too broadly? |
| Promotion path | Docs-only, catalog metadata, or Rust API candidate? |

## Starter Ontology Decisions

| Decision | Rationale |
|---|---|
| Keep `FrameKind` small in Rust. | The current index should remain stable while docs-level jobs mature. |
| Treat relation terms as docs-level first. | Relation-aware ranking needs evaluation fixtures before API fields. |
| Keep authority model controlled. | Wrong authority transfer is a high-frequency failure mode. |
| Keep high/unknown/unsafe output obligations explicit. | Risk labels should change behavior, not merely decorate results. |
| Let tags bridge future metadata. | Tags are acceptable only when they follow family rules and do not overclaim. |

## Review Checklist

Before accepting or importing a frame, ask:

1. What is its primary frame job?
2. What relation term explains the transfer?
3. What authority model does it assume?
4. What risk term controls output obligations?
5. Which tags are only search aids, and which should become metadata later?
6. Which fallback applies if the relation, authority, or risk does not fit?

## Design Consequences

- Catalog rows should prefer controlled terms over ad hoc tags.
- Evaluation fixtures should use the same job, relation, authority, and risk
  terms as review docs.
- Rust API additions should promote only terms that are stable in catalog rows
  and change tool behavior.
- Rejected-candidate reporting should name ontology mismatches, not just low
  score.

