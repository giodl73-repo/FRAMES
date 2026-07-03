# Evaluation Fixtures

This directory contains machine-readable fixture sets for checking frame
selection, suppression, fallback, warning, and relation behavior.

The first package is `starter-fixtures.json`. It mirrors the starter fixture
backlog in `docs/theory/evaluation-set-design.md` and stays docs-level until
tooling proves the shape is stable.

## Fixture Package Rules

- Keep fixtures small, explicit, and traceable to theory or catalog sources.
- Treat `expected_primary: null` as a deliberate no-primary expectation.
- Use `must_not_return` for frames or patterns that should be suppressed by
  default.
- Use `expected_relation_behavior` only when relation behavior is part of the
  pass condition.
- Do not treat these fixtures as empirical validation; they are specification
  checks for expected tool behavior.

## Validation

```powershell
Get-Content docs\eval\starter-fixtures.json -Raw | ConvertFrom-Json | Out-Null
```
