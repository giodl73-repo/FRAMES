# Catalog Metadata Migration Plan

This plan defines how FRAMES should migrate from prose-heavy theory documents
to structured catalog metadata and eventually richer `frames-core` fields.

The goal is not to put every theory term into the Rust API immediately. The goal
is to promote fields only after they have survived draft imports, role review,
fit scoring, and real lookup needs.

## Current Indexed Shape

`frames-core::FrameEntry` currently exposes:

| Field | Status | Notes |
|---|---|---|
| `id` | stable | Required for lookup and related frames. |
| `name` | stable | Human-readable label. |
| `kind` | stable | Current frame job enum: status, coordination, momentum, risk. |
| `everyday_source` | stable | Short source scene. |
| `target_situations` | stable | Searchable target text. |
| `tags` | stable | Flexible bridge for metadata not yet first-class. |
| `action_cue` | stable | Required next-action guidance. |
| `evidence_boundary` | stable | Required proof boundary. |
| `failure_mode` | stable | Required misuse warning. |
| `related` | stable | Adjacent alternatives. |

This is enough for first lookup. It is not enough for safe expansion into local
imports, leadership packs, or validated claims.

## Metadata Families

| Family | Fields | Source Theory | Migration Priority |
|---|---|---|---|
| Lifecycle | status, fit score, review date, review lens, revisit trigger | `frame-lifecycle.md` | P0 docs, P2 API |
| Claim | claim strength, validation scope, source provenance | `claim-strength-labels.md`, `empirical-validation-plan.md` | P0 docs, P2 API |
| Source domain | source family, authority model, temporal shape, risk band, audience portability | `source-domain-taxonomy.md` | P1 docs, P2 API |
| Relational transfer | source relation, target relation, authority relation, protected value, exclusions, transfer strength | `relational-transfer-fields.md` | P1 docs, P3 API |
| Perspective | implied role, counterparty role, agency, duty, authority, vulnerability, risk | `perspective-metadata.md` | P1 docs, P3 API |
| Story | story job, vividness/evidence balance, alternate story job | `story-job-taxonomy.md` | P1 docs, P3 API |
| Application | application pack, output obligations, rejection rules, alternates | `application-pack-templates.md` | P1 docs, P2 API |
| Validation | audience, context, task, comparison, measure, result, boundary | `empirical-validation-plan.md` | P1 docs, P4 API |

## Migration Stages

### Stage 0: Docs-Only Stabilization

Use markdown tables and draft import artifacts to test field names.

Required before leaving this stage:

- at least three local imports use the same field names;
- role review can apply the fields without extra explanation;
- VTRACE records the theory artifact;
- no Rust API changes are needed for lookup.

Current status: complete for the first accepted starter metadata table in
`docs/frame-catalog.md`; still active for local draft imports and future API
metadata.

### Stage 1: Catalog Table Normalization

Normalize docs catalog rows before changing the crate.

Add these fields to accepted catalog docs first:

- lifecycle status,
- claim strength,
- source family,
- authority model,
- risk band,
- transfer strength,
- application pack,
- review note.

Do not add long prose fields to the main catalog table. Keep detailed transfer
maps in theory/import docs and link to them.

### Stage 2: Nonbreaking Rust Metadata

Add compact enum/string metadata only after docs rows prove stable.

Candidate additions:

```rust
pub enum FrameStatus { Accepted, Revised }
pub enum ClaimStrength { Heuristic, RoleReviewed, EmpiricallyValidated }
pub enum RiskBand { Low, Medium, High }
pub enum ApplicationPack { Product, Operations, Leadership, Learning, AiAgent }
```

Nonbreaking approach:

- add fields only when every `STARTER_CATALOG` entry can be populated;
- prefer small enums for stable, low-cardinality concepts;
- keep open-ended concepts in tags or docs until stable;
- add helper filters before changing search scoring.

### Stage 3: Transfer-Aware Lookup

Add transfer metadata only after at least one catalog pack uses it consistently.

Candidate additions:

- transfer strength,
- authority model,
- protected value,
- transfer exclusions.

Search scoring can then prefer:

1. frame kind match,
2. target situation match,
3. authority/risk compatibility,
4. transfer strength,
5. tags.

Do not rank by vividness, story appeal, or source-family familiarity alone.

### Stage 4: Validation Scope

Add validation metadata only after a real validation report exists.

Candidate additions:

- validation audience,
- validation task,
- validation comparison,
- validation result summary,
- validation boundary.

Until then, `empirically_validated` should remain absent from the starter index.

## Field Promotion Rules

Promote a field from docs to API only when:

- it has a stable definition in theory docs;
- at least five entries can populate it without awkward placeholders;
- it changes lookup, filtering, display safety, or acceptance decisions;
- role review agrees the field prevents a real failure mode;
- VTRACE requirement/spec/interface rows are updated.

Do not promote a field when:

- it is only useful for one local import;
- it duplicates a tag without improving safety or selection;
- the values are still open-ended;
- it would make accepted entries look more validated than they are.

## Initial Migration Backlog

| Step | Change | Rationale | Gate |
|---|---|---|---|
| M1 | Add docs catalog columns for status and claim strength. | Prevent accepted examples from looking equally validated. | Catalog Structure review. |
| M2 | Add docs catalog columns for source family, authority model, and risk band. | Make source-domain risk visible. | Source-domain review. |
| M3 | Add docs catalog column for transfer strength. | Separate structural frames from illustrative stories. | Fit rubric review. |
| M4 | Add docs catalog column for application pack. | Support product, operations, leadership, learning, and AI-agent defaults. | Application-pack review. |
| M5 | Add `ClaimStrength` and `RiskBand` to Rust only for accepted starter entries. | Improve display safety for tool callers. | All starter entries populated. |
| M6 | Add transfer-aware search filters. | Let AI/tool callers avoid authority or risk mismatches. | Transfer metadata stable. |

## Starter Catalog Migration Target

The first metadata-backed catalog migration should touch only accepted starter
entries and should not import local draft frames yet.

This first migration is implemented as the "Accepted Starter Metadata" table in
`docs/frame-catalog.md`.

Suggested first fields:

| Field | Why First |
|---|---|
| `status` | Starter index should imply accepted use only. |
| `claim_strength` | Prevents heuristic entries from sounding validated. |
| `risk_band` | Helps AI/tool callers avoid high-stakes overuse. |
| `application_packs` | Supports context-specific suggestions. |

Deferred fields:

- full relational transfer maps,
- story-job overlays,
- empirical validation scope,
- local source provenance.

These are too detailed or too unstable for the first API migration.

## Compatibility Rules

- Public Rust field additions are breaking for literal struct construction by
  downstream users; batch them intentionally.
- Prefer adding constructors or builder helpers before many required fields.
- Avoid optional metadata fields unless absence has a clear meaning.
- Keep `STARTER_CATALOG` accepted-only until lifecycle metadata exists.
- Do not encode held, draft, deprecated, or rejected frames in default search
  until tool behavior can filter them safely.

## Design Consequences

- Docs remain the proving ground for metadata fields.
- VTRACE must track every API metadata promotion as a product requirement, not
  only as theory work.
- First API migration should optimize safety and display clarity before ranking
  sophistication.
- Local imports should stay draft docs until promoted by fit score and role
  review.
- The AI response contract should drive which fields are promoted first, because
  tool callers need display safety before richer ranking.
