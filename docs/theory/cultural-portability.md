# Cultural Portability

Cultural portability asks whether a frame can travel across regions, languages,
mobility assumptions, organizational norms, role expectations, and power
distance without losing its useful relation or importing a harmful one.

This guide extends [audience-transfer.md](audience-transfer.md). Audience
transfer asks whether a known audience understands a frame. Cultural portability
asks what must change when the audience or context is not known, mixed, or
outside the source scene's home culture.

## Portability Rule

Do not treat an everyday source as universal. A frame is portable only when:

- the audience is likely to recognize the source scene;
- the source rules are similar enough to the target relation;
- the authority and duty assumptions transfer;
- the source does not exclude part of the audience through mobility, ability,
  region, class, language, profession, or lived experience;
- a plain-language fallback or safer alternate is available.

If those conditions are unknown, mark portability as limited or unknown and use
a lower-risk alternate.

## Portability Dimensions

| Dimension | Check | Common Failure | Safer Move |
|---|---|---|---|
| Regional norms | Are the source rules similar in the audience's region? | Four-way stop, queue, school, healthcare, or service norms differ. | Use a more generic coordination frame or name the rule directly. |
| Mobility and ability | Does the source assume driving, walking, cycling, sight, hearing, or physical ease? | Audience cannot access or may be harmed by the assumed embodied script. | Offer queue, checklist, handoff, or plain-language alternatives. |
| Language and idiom | Does the phrase translate without hidden idiom or pun? | The metaphor is memorable only in one language. | Use literal relation language before metaphor. |
| Authority distance | Does the source assume peer turn-taking, command, stewardship, or protected-party duty? | A peer frame is used in a hierarchy, or a command frame is used among peers. | Match actual decision rights. |
| Social script | Does politeness, conflict, waiting, interruption, or deference work similarly? | A "speak up" or "take the lane" cue is unsafe or rude in context. | Name the expected behavior and risk directly. |
| Institutional trust | Does the source rely on trust in police, road rules, courts, schools, or management? | The source carries fear, surveillance, punishment, or unfairness. | Use lower-authority sources or plain language. |
| Professional culture | Does the source assume engineering, operations, product, finance, clinical, or legal habits? | Jargon makes the frame feel obvious to insiders and opaque to others. | Translate to a cross-role source. |
| Stakes and harm | Does the source trivialize serious harm, identity, safety, money, or career risk? | A playful frame reduces accountability. | Use direct accountability and evidence language. |

## Portability Bands

| Band | Meaning | Default Use |
|---|---|---|
| Broad | Source scene is likely familiar and low-risk across the intended audience. | May use with normal evidence boundary. |
| Bounded | Source works for a named audience, region, role, or domain. | Use only with audience note and alternate. |
| Limited | Source is familiar to some but likely excludes or misleads others. | Prefer alternate or plain language. |
| Unknown | Audience familiarity or authority assumptions are not known. | Use plain language first; frame second if useful. |
| Unsafe | Source imports harmful, coercive, dehumanizing, or exclusionary assumptions. | Reject or keep as anti-pattern only. |

## Source-Specific Guidance

| Source Family | Portability Risk | Safer Alternate |
|---|---|---|
| Driving and traffic | Assumes driving familiarity, local road law, trust in enforcement, and shared right-of-way norms. | Queue, appointment, handoff, checklist, or explicit priority rule. |
| Walking and wayfinding | Assumes mobility, physical access, and shared spatial experience. | Route plan, sequence, checklist, map, or dependency graph. |
| Sports and games | Assumes game knowledge and can imply winners, losers, fouls, and play-acting. | Work queue, scenario, constraint, or decision table. |
| Cooking | Assumes food practices, equipment, timing norms, and comfort with domestic metaphors. | Preparation, staging, batch, or readiness language. |
| Military and command | Imports hierarchy, urgency, obedience, and threat. | Incident role, escalation path, owner decision, or stewardship. |
| Markets and investment | Imports competition, winners, risk appetite, and financialized value. | Resource allocation, option value, or tradeoff table. |

## Decision Procedure

1. Name the intended audience and any unknown audience segments.
2. Identify the source scene's home assumptions: region, language, authority,
   mobility, role, and institutional trust.
3. Compare those assumptions to the target situation.
4. Assign a portability band.
5. Add a safer alternate or plain-language fallback for bounded, limited,
   unknown, or unsafe bands.
6. Record the portability note in catalog or evaluation fixtures.

## Examples

| Candidate | Portability Issue | Decision |
|---|---|---|
| Four-way stop for global team turn-taking | Road rules and driving familiarity vary. | Bounded; offer queue turn-taking or explicit turn order. |
| Crosswalk yield for protected customer safety | Driving source may work, but duty and vulnerability must be named. | Bounded; keep only when protected-party duty is visible. |
| Take the lane for stakeholder escalation | Cycling rule and confrontation script may not transfer. | Limited; use "make ownership explicit" instead. |
| Green/yellow/red status | Color meanings and accessibility vary; color alone excludes some readers. | Bounded; pair color with label, threshold, and text. |
| Bag-of-chips conflict story | Story may encourage empathy but can erase harm after facts are known. | Limited; use only when facts are incomplete. |

## Applied Reviews

- [cultural-portability-application-fixtures.md](cultural-portability-application-fixtures.md)
  applies portability bands to the starter evaluation fixture package.

## Catalog Fields

Future catalog rows should record:

```text
portability_band:
portability_note:
known_audience_fit:
known_audience_risks:
regional_assumptions:
language_or_idiom_risk:
mobility_or_accessibility_risk:
safer_portability_alternate:
plain_language_fallback:
```

These fields should stay docs-level until accepted catalog rows and evaluation
fixtures prove the values are stable.

## Evaluation-Set Implications

Evaluation fixtures should include:

- the same source frame used for a familiar audience and an unfamiliar audience;
- driving-frame cases where queue, handoff, or plain language should win;
- color/status cases that require text labels and thresholds;
- authority-distance cases where peer and command frames diverge;
- accessibility cases where embodied source assumptions should trigger a
  fallback.

## AI And Tool Implications

- Default AI suggestions should not assume US driving familiarity.
- High-risk or unknown-audience outputs should include a plain-language fallback.
- Search should be allowed to prefer a lower-scoring but more portable alternate
  when audience information is missing.
- Future query filters should accept portability band, region, language risk,
  and mobility/accessibility risk.

## Design Consequences

- Accepted-catalog review should reject or hold frames with unknown portability
  and no fallback.
- Evaluation-set design should include portability fixtures before semantic
  search expansion.
- Related-frame taxonomy should mark portable alternates and plain-language
  fallbacks for source scenes with limited portability.
- Rust metadata migration should not expose portability fields until docs-level
  catalog rows have consistent values.
