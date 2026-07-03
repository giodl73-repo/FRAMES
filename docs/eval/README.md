# Evaluation Fixtures

This directory contains machine-readable fixture sets for checking frame
selection, suppression, fallback, warning, and relation behavior.

The first package is `starter-fixtures.json`. It mirrors the starter fixture
backlog in `docs/theory/evaluation-set-design.md` and stays docs-level until
tooling proves the shape is stable.

`lifecycle-rejection-fixtures.json` adds the first explicit lifecycle
visibility and suppressed-candidate report fixtures. It links back to starter
fixture IDs but tests a narrower behavior: whether a tool keeps non-accepted,
docs-catalog, rejected, anti-pattern, and wrong-authority candidates out of
recommendations while preserving opt-in explanations.

`review-only-catalog-fixtures.json` adds the first machine-readable rows for
docs-catalog, anti-pattern, and held entries. It remains the review fixture
source for the first Rust `REVIEW_CATALOG` rows.

`relation-aware-ranking-fixtures.json` adds ordering, demotion, composition,
and hard-stop fixtures for relation-aware scoring. It does not change default
Rust search; it defines expected behavior for the opt-in relation report path.

## Fixture Package Rules

- Keep fixtures small, explicit, and traceable to theory or catalog sources.
- Treat `expected_primary: null` as a deliberate no-primary expectation.
- Use `must_not_return` for frames or patterns that should be suppressed by
  default.
- Use `expected_relation_behavior` only when relation behavior is part of the
  pass condition.
- Use `portability_profiles` to inspect audience, mobility, region, language,
  and authority risks before recommending a frame to unknown audiences.
- Use lifecycle/rejection fixtures to check `allowed_statuses`,
  `explain_suppressed`, `expected_suppressed`, and `display_rule` before adding
  Rust lifecycle filters or rejected-candidate report structs.
- Use review-only catalog fixtures to check row family, required fields,
  display rules, conversion rules, and promotion gates before loading
  `ReviewFrameEntry` rows into Rust.
- Use relation-aware ranking fixtures to check scoring order, hard-stop
  suppression, demotion, and expected ordering before implementing relation
  scoring.
- Do not treat these fixtures as empirical validation; they are specification
  checks for expected tool behavior.

## Validation

```powershell
Get-Content docs\eval\starter-fixtures.json -Raw | ConvertFrom-Json | Out-Null
Get-Content docs\eval\lifecycle-rejection-fixtures.json -Raw | ConvertFrom-Json | Out-Null
Get-Content docs\eval\review-only-catalog-fixtures.json -Raw | ConvertFrom-Json | Out-Null
Get-Content docs\eval\relation-aware-ranking-fixtures.json -Raw | ConvertFrom-Json | Out-Null
```
