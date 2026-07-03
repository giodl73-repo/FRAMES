# Requirements

## Scope

Repo or feature: `frames-core`

## Requirement Table

| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | The library shall expose structured frame entries with stable IDs, frame kind, source, target situations, tags, action cue, failure mode, and related-frame IDs. | NEED-001 / CON-001 | AI/tool users need inspectable entries, not generated prose only. | must | FRAMES | unit test / inspection | accepted |
| REQ-002 | The library shall provide deterministic search over situation text, desired frame kind, and tags. | NEED-001 / CON-001 | Frame lookup should be repeatable and local. | must | FRAMES | unit test | accepted |
| REQ-003 | The library shall provide related-frame lookup from a stable frame ID. | NEED-001 / CON-002 | Users need alternatives like a thesaurus. | must | FRAMES | unit test | accepted |
| REQ-004 | The starter catalog shall preserve misuse or failure-mode warnings for every indexed frame. | NEED-001 / CON-003 | Frame suggestions must not hide analogy limits. | must | FRAMES | inspection / unit test | accepted |
| REQ-005 | The first crate shall remain dependency-free. | NEED-001 | Keeps the index easy to audit and embed. | should | FRAMES | inspection / cargo test | accepted |
| REQ-006 | The library shall provide ergonomic query and filtering helpers for common AI/tool lookup flows. | NEED-001 / CON-001 / CON-002 | Callers should not need to construct every query and filter manually. | should | FRAMES | unit test / example run | accepted |
| REQ-007 | Traffic frame docs and starter index entries shall stay aligned for accepted traffic-pack frames. | NEED-001 / CON-001 / CON-003 | AI/tool lookup should reflect the documented frame pack. | must | FRAMES | unit test / inspection | accepted |
| REQ-008 | The repo shall define role-reviewed theory for frame jobs, fit tests, evidence boundaries, and misuse patterns before broad catalog expansion. | NEED-001 / CON-003 | Catalog growth needs selection theory, not just examples. | must | FRAMES | inspection / role review | accepted |
| REQ-009 | The repo shall define a scored fit rubric and theory roadmap for prioritizing future theory work. | NEED-001 / CON-003 | Frames need comparable readiness decisions and a clear theory backlog. | must | FRAMES | inspection | accepted |
| REQ-010 | The repo shall define audience transfer guidance for role, expertise, region, culture, mobility, stakes, and power differences. | NEED-001 / CON-003 | Frames can fail when source familiarity or authority assumptions do not transfer. | must | FRAMES | inspection | accepted |
| REQ-011 | The repo shall define lifecycle states and indexing rules for candidate, draft, accepted, revised, held, deprecated, and rejected frames. | NEED-001 / CON-003 | The catalog needs controlled status transitions before broader growth. | must | FRAMES | inspection | accepted |
| REQ-012 | The repo shall define composition and conflict guidance for using multiple frames in one situation. | NEED-001 / CON-003 | Related frames can clarify or contradict each other; selection needs rules. | must | FRAMES | inspection | accepted |
| REQ-013 | Indexed frame entries shall preserve an evidence boundary distinct from failure mode. | NEED-001 / CON-001 / CON-003 | AI/tool callers need to know what must still be checked after a frame is suggested. | must | FRAMES | unit test / inspection | accepted |
| REQ-014 | The repo shall define research-grounding guidance for cognitive-science claims and frame-design implications. | NEED-001 / CON-003 | FRAMES needs defensible theory boundaries before making public claims about metaphor, analogy, persuasion, or cognition. | must | FRAMES | inspection | accepted |
| REQ-015 | The repo shall define a source-domain taxonomy for everyday frame sources, authority models, temporal shapes, and risk bands. | NEED-001 / CON-003 | Frame selection needs controlled source-domain structure before broad catalog expansion. | must | FRAMES | inspection / role review | accepted |
| REQ-016 | The repo shall define application-pack templates for product, operations, leadership, learning, and AI-agent frame use. | NEED-001 / CON-001 / CON-003 | Different contexts need different frame defaults, authority checks, evidence obligations, and rejection rules. | must | FRAMES | inspection | accepted |
| REQ-017 | The repo shall define perspective metadata guidance for listener role, agency, duty, authority, and perspective risk. | NEED-001 / CON-003 | Frames can mislead by assigning the wrong role or duty even when the analogy is familiar. | must | FRAMES | inspection | accepted |
| REQ-018 | The repo shall define story-job taxonomy guidance for narrative purpose, audience effect, evidence boundary, and misuse risk. | NEED-001 / CON-003 | Frame selection needs to distinguish what an analogy clarifies from what the story is doing to the audience. | must | FRAMES | inspection | accepted |
| REQ-019 | The repo shall define relational transfer fields for source-target relation mapping, transfer exclusions, and transfer strength. | NEED-001 / CON-003 | Frame selection needs to prefer relational structure over surface similarity. | must | FRAMES | inspection | accepted |
| REQ-020 | The repo shall define claim-strength labels for frame claims, practitioner observations, local observations, role review, and empirical validation. | NEED-001 / CON-003 | Public and AI-facing frame guidance needs to avoid overstating evidence. | must | FRAMES | inspection | accepted |
| REQ-021 | The repo shall define role-reviewed domain examples before broader catalog expansion. | NEED-001 / CON-003 | Theory needs concrete reviewed examples that show how roles accept, revise, hold, or reject frame patterns. | must | FRAMES | inspection / role review | accepted |
| REQ-022 | The repo shall define structured RESONANCE MANAGE frame imports with provenance, claim strength, transfer boundaries, evidence boundaries, and draft status. | NEED-001 / CON-003 | Local management patterns need controlled import before becoming catalog or index entries. | must | FRAMES | inspection | accepted |
| REQ-023 | The repo shall define structured CAREER Gravity frame imports with provenance, claim strength, transfer boundaries, evidence boundaries, and draft status. | NEED-001 / CON-003 | Local career and gravity patterns need controlled import before becoming general-purpose catalog or index entries. | must | FRAMES | inspection | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to mission need or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is required.
