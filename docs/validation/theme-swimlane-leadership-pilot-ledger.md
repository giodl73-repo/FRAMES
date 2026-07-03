# Theme Swimlane Leadership Pilot Ledger

This ledger records leadership-pack pilot uses of the Theme Swimlane worksheet.
It is not an empirical validation trial. It captures whether the worksheet
changed real funding, sequencing, staffing, scope, dependency, or rejection
decisions.

Theme Swimlanes remain a promoted draft heuristic until at least one real pilot
is completed, reviewed, and accepted-catalog gates are revisited.

## Pilot Rules

Use this ledger only when:

- the worksheet is applied to a real program promise;
- the program has three to five candidate lanes;
- decision makers can name at least one decision that changed or explicitly did
  not change;
- misuse risks are reviewed before lifecycle promotion.

Do not count brainstorming, retrospective interpretation, or a dry-run example
as pilot evidence.

## Pilot Record Template

```text
Pilot ID:
Date:
Program:
Worksheet owner:
Decision forum:
Participants / roles:

Customer or operating promise:

Lanes:
- lane:
  owner:
  measure:
  contribution rule:
  exclusion rule:
  tradeoff:

Changed decisions:
- decision:
  prior default:
  worksheet effect:
  evidence used:
  owner:
  revisit trigger:

Work funded:

Work sequenced later:

Work reframed:

Work rejected:

Evidence still missing:

Misuse or confusion observed:

Role gate result:
- Business Leader:
- Evidence Boundary Reviewer:
- Misuse Risk Reviewer:
- Audience Transfer Reviewer:
- Catalog Structure Reviewer:

Recommended lifecycle change:
```

## Pilot Ledger

| Pilot ID | Date | Program | Promise | Changed Decision Count | Role Gate Result | Lifecycle Recommendation | Evidence Status |
|---|---|---|---|---:|---|---|---|
| PILOT-THEME-001 | not run | TBD real program | TBD | 0 | not reviewed | no change | no pilot evidence |

## Local Dry-Run Example

This dry run uses the local pattern from `theme-swimlane-extraction.md`. It is a
format check, not pilot evidence.

| Field | Dry-Run Value |
|---|---|
| Program | Platform operating model wave |
| Promise | Customers and operators can use one coherent platform with lower operating friction. |
| Candidate lanes | `Run One`, `Run Lean`, `Run Fast`, `Run Safe` |
| Example changed decision | Split a speed-oriented migration into `Run Fast` only after `Run Safe` gates are named. |
| Evidence still missing | Real decision forum, owners, measures, and post-decision outcome check. |
| Lifecycle recommendation | No change; still draft heuristic. |

Dry-run lane sketch:

| Lane | Outcome | Measure | Exclusion |
|---|---|---|---|
| `Run One` | Customers and operators experience one coherent entry path. | Fewer duplicate entry points and clearer ownership. | Local variation with proven customer value is not flattened. |
| `Run Lean` | Customers and operators spend less effort on duplicate handoffs. | Handoff count, cycle time, and support contacts decrease together. | Cost cutting that degrades quality or transfers work invisibly. |
| `Run Fast` | Teams reduce wait states and decision latency. | Cycle time and batch size improve without unmanaged risk. | Speed work without safety or learning gates. |
| `Run Safe` | Customers can trust recovery, auditability, and operational guardrails. | Incident paths, recovery checks, and audit evidence improve. | Safety theater without a named protected value. |

Dry-run changed-decision sketch:

| Decision | Prior Default | Worksheet Effect | Why It Is Not Evidence |
|---|---|---|---|
| Risky migration under `Run Fast` | Treat speed as the primary lane. | Reframe as `Run Fast` only when paired with `Run Safe` gates. | No real decision forum or owner accepted the change. |
| Duplicate admin workflow under `Run Lean` | Fund because it reduces effort for one team. | Hold until evidence shows effort is removed for customers, not displaced to another team. | No real work item or measure was reviewed. |

## Closeout Rule

A completed pilot must record at least one changed decision or explicitly record
that the worksheet did not change decisions. If no decision changed, the
recommended lifecycle change should be `hold`, not promotion.
