# Rust Relation-Aware Ranking Design

This design describes how `frames-core` should implement relation-aware ranking
after the fixture package exists. It is an implementation plan, not a scoring
change.

Default `FrameIndex::search` should keep its current deterministic behavior
until accepted starter entries have enough compact relation metadata to score
against [../eval/relation-aware-ranking-fixtures.json](../eval/relation-aware-ranking-fixtures.json).

## Design Rule

Relation-aware ranking must make structural fit visible before it changes
default recommendations.

The first implementation should be additive:

- keep `FrameIndex::search` unchanged;
- add relation metadata types and query inputs only after starter rows can be
  populated;
- add a separate relation-aware report method before replacing default ranking;
- prove fixture behavior before any default-search promotion.

## Fixture Contract

The implementation must be tested against these fixture behaviors:

| Fixture | Required Behavior |
|---|---|
| `REL-RANK-001` | Prefer `crosswalk-yield` over peer turn-taking when protected-party duty is present. |
| `REL-RANK-002` | Suppress peer coordination frames when owner authority directs the action. |
| `REL-RANK-003` | Rank buffer/sequence behavior before merge behavior under tight coupling. |
| `REL-RANK-004` | Preserve status frames only with threshold/evidence warning. |
| `REL-RANK-005` | Suppress unsupported `veto-rule` as a rejected near-miss. |
| `REL-RANK-006` | Prefer repair/accountability fallback after facts establish harm. |

The fixture package defines expected order, demotion, suppression, relation
behavior, warnings, and fallback text. Tests should compare those fields before
they compare numeric scores.

## First API Shape

Keep existing query APIs stable and add a relation-specific query wrapper:

```rust
pub struct RelationQuery<'a> {
    pub base: FrameQuery<'a>,
    pub target_relation: Option<TargetRelation>,
    pub constraint: Option<ConstraintRelation>,
    pub protected_value: Option<ProtectedValue>,
    pub excluded_transfers: &'a [TransferExclusion],
}
```

Start with small enums only where fixture values are stable:

```rust
pub enum TargetRelation {
    PeerTurnTaking,
    ProtectedPartyDuty,
    DirectedAuthority,
    FlowJoining,
    RequiredGate,
    ThresholdSignal,
    PerspectiveRepair,
}

pub enum ConstraintRelation {
    Coupling,
    EvidenceMissing,
    AuthorityMissing,
    FactsKnown,
}

pub enum ProtectedValue {
    CustomerSafety,
    IncidentControl,
    SystemStability,
    DecisionQuality,
    DecisionLegitimacy,
    RepairAccountability,
}
```

Do not make these enums public until their naming matches the ontology and
fixtures. If the names still churn, keep them private or docs-only.

## Entry Metadata Shape

Do not add relation fields to `FrameEntry` until all accepted starter rows can
populate them cleanly. The first implementation can use a parallel metadata
table keyed by frame ID:

```rust
pub struct RelationMetadata {
    pub frame_id: &'static str,
    pub target_relations: &'static [TargetRelation],
    pub constraint_relations: &'static [ConstraintRelation],
    pub protected_values: &'static [ProtectedValue],
    pub transfer_strength: TransferStrength,
    pub exclusions: &'static [TransferExclusion],
}
```

A parallel table avoids breaking `FrameEntry` construction while the values are
still proving themselves. Promote fields onto `FrameEntry` only after the table
is complete and useful.

## Report Shape

Relation-aware output should separate retrieval from fit:

```rust
pub struct RelationCandidate<'a> {
    pub candidate: FrameCandidate<'a>,
    pub relation_score: i16,
    pub rank_band: RankBand,
    pub matched_relations: &'a [&'a str],
    pub authority_fit: AuthorityFit,
    pub decision: RelationDecision,
    pub warnings: &'a [&'a str],
    pub plain_language_fallback: Option<&'a str>,
}
```

`relation_score` is not empirical proof. It is a deterministic implementation
score used to order candidates after hard stops and metadata gates.

## Scoring Procedure

Apply scoring in this order:

1. Hard-stop rejection: remove or suppress frames that violate authority,
   dangerous transfer, evidence, protected value, or explicit exclusions.
2. Frame kind gate: keep the user's requested job as the first positive match.
3. Target relation: reward matching relation metadata more than word overlap.
4. Authority compatibility: reward exact fit; suppress serious mismatches.
5. Constraint compatibility: reward coupling, evidence, authority, or fact-state
   matches.
6. Protected value: reward frames that preserve the named value.
7. Transfer strength: structural beats partial; partial beats illustrative.
8. Application pack fit: reward context-appropriate frames.
9. Lexical scoring: use current text/tag/name scoring as secondary evidence.

Hard stops should produce suppressed reports or fallbacks, not low-scoring
recommendations.

## Compatibility Plan

1. Add private relation metadata and parser-free static tables.
   Complete for the first fixture IDs.
2. Add tests that map the fixture package into hand-written Rust scenarios.
   Complete for the first relation metadata expectations.
3. Add `search_with_relations` or `rank_with_relations` as a separate method.
   Complete for the first fixture-backed report path.
4. Keep `FrameIndex::search` unchanged.
5. Consider default-search promotion only after relation-aware output passes
   fixtures and examples show the boundary clearly.

## Non-Goals

- Do not add semantic search.
- Do not add runtime JSON loading.
- Do not rank review-only rows as default recommendations.
- Do not replace lifecycle suppression with a numeric penalty.
- Do not expose every fixture string as a public enum.

## Acceptance Gate

Before implementation can replace or influence default ranking:

- relation metadata exists for every accepted starter row;
- the relation fixture package passes;
- suppressed and fallback behavior matches lifecycle rules;
- docs explain retrieval score versus fit/ranking score;
- VTRACE records the API and behavior change.

## Design Consequences

- Relation-aware ranking should start as a separate report path, not a mutation
  of default search.
- A parallel relation metadata table is the least disruptive first Rust step.
- Fixture behavior should drive ranking decisions before numeric score tuning.
- Private relation metadata tables now cover the first ranking fixture IDs
  without changing `FrameIndex::search`.
- `search_with_relations` now provides a separate relation-aware report path
  while preserving default search.
- A runnable relation-aware example now shows the report shape.
- Relation fixture coverage now includes visibility-before-dependency and
  pace-plus-buffer composition cases.
- Relation fixture coverage now also includes stabilization-before-reentry
  sequencing.
- Relation fixture coverage now includes recovery-pause restart boundary
  behavior.
- Relation fixture coverage now includes route-adjustment destination boundary
  behavior.
- The next implementation step should continue deepening fixture coverage
  before considering any default-search promotion.
