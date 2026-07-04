# Frame Catalog

This catalog records candidate frames as structured analogies, not as slogans.
Each entry should make the mapping useful and the limits visible.

## Accepted Starter Metadata

These rows are the first metadata-backed catalog migration. They cover accepted
starter entries only and do not promote local draft imports from RESONANCE,
CAREER, or theme swimlanes.

| ID | Frame | Status | Claim strength | Risk band | Application packs | Source family | Authority term | Relation term | Transfer strength | Review note |
|---|---|---|---|---|---|---|---|---|---|---|
| `red-yellow-green` | Red / yellow / green | accepted | heuristic | medium | product, operations, leadership, AI-agent | signals and instruments | operator | threshold_signal | structural when thresholds are explicit | Pair color with threshold and owner. |
| `fuel-gauge` | Fuel gauge | accepted | heuristic | low | product, operations, leadership, AI-agent | signals and instruments | operator | reserve_tracking | partial | Name the scarce resource and other constraints. |
| `dashboard-warning-light` | Dashboard warning light | accepted | heuristic | medium | operations, product, AI-agent | signals and instruments | operator | threshold_signal | structural when diagnostic signal is known | Avoid warning fatigue. |
| `speed-limit` | Speed limit | accepted | heuristic | medium | product, operations, leadership | motion and navigation | steward | pace_adjustment | structural when constraint sets pace | Do not turn local constraint into universal cap. |
| `four-way-stop` | Four-way stop | accepted | heuristic | medium | product, operations, leadership, AI-agent | queues and turns | peer | peer_turn_taking | structural for peer coordination | Reject when authority is asymmetric. |
| `merge-lane` | Merge lane | accepted | heuristic | medium | product, operations, AI-agent | motion and navigation | peer | flow_joining | structural when timing and approval are clear | Check whether explicit approval is required. |
| `crosswalk-yield` | Crosswalk yield | accepted | heuristic | medium | product, operations, leadership | queues and turns | protected_party | protected_party_duty | partial | Name protected value without labeling people as weak. |
| `crowded-sidewalk` | Crowded sidewalk | accepted | heuristic | medium | product, operations, leadership | motion and navigation | peer | flow_joining | partial | Preserve flow and move deep debates out of high-traffic lanes. |
| `trail-marker` | Trail marker | accepted | heuristic | medium | product, operations, learning | motion and navigation | steward | route_adjustment | structural when route and marker ownership are explicit | Refresh stale markers at decision forks. |
| `walking-pace` | Walking pace | accepted | heuristic | low | product, learning, operations | motion and navigation | operator | pace_adjustment | partial | Check urgency and required outcome. |
| `stride-length` | Stride length | accepted | heuristic | medium | product, learning, operations | motion and navigation | operator | pace_adjustment | structural when correction cost is explicit | Match step size to feedback speed and correction cost. |
| `downshift` | Downshift | accepted | heuristic | low | product, operations, learning | motion and navigation | operator | pace_adjustment | structural when load is named | Frame as control, not failure. |
| `detour` | Detour | accepted | heuristic | medium | product, operations, leadership | motion and navigation | operator | route_adjustment | structural when destination is stable | Verify destination before changing route. |
| `rest-stop` | Rest stop | accepted | heuristic | low | operations, learning, leadership | motion and navigation | steward | recovery_pause | structural when restart is explicit | Require restart condition. |
| `stumble-and-recover` | Stumble and recover | accepted | heuristic | medium | product, operations | motion and navigation | operator | recovery_pause | structural when restart and impact checks are explicit | Recover control, then adapt before resuming speed. |
| `shoulder-pull-off` | Shoulder / pull-off | accepted | heuristic | medium | operations, product | motion and navigation | operator | stabilization_reentry | structural when stabilization and reentry are clear | Do not normalize stopping without reentry. |
| `blind-spot` | Blind spot | accepted | heuristic | medium | product, operations, leadership, AI-agent | motion and navigation | reviewer | attention_limit | structural when missing signal is named | Avoid blaming individuals for system visibility gaps. |
| `footing` | Footing | accepted | heuristic | medium | product, operations, learning | motion and navigation | reviewer | attention_limit | structural when assumptions and traction checks are explicit | Stabilize assumptions before optimizing pace. |
| `following-distance` | Following distance | accepted | heuristic | medium | product, operations, AI-agent | motion and navigation | operator | buffer_spacing | structural when coupling and reaction time are known | Buffer must be tied to named risk. |
| `load-bearing-wall` | Load-bearing wall | accepted | heuristic | medium | product, operations, leadership | construction and repair | owner | dependency_integrity | structural when dependency is truly structural | Inspect before treating change as impossible. |

Metadata meanings:

- `status` follows [theory/frame-lifecycle.md](theory/frame-lifecycle.md).
- `claim_strength` follows
  [theory/claim-strength-labels.md](theory/claim-strength-labels.md).
- `risk_band`, `source_family`, `authority_term`, and `relation_term` follow
  [theory/frame-ontology.md](theory/frame-ontology.md).
- `authority_term` should stay aligned with Rust `AuthorityModel` for accepted
  starter entries.
- Source-domain details also follow
  [theory/source-domain-taxonomy.md](theory/source-domain-taxonomy.md).
- `transfer_strength` follows
  [theory/relational-transfer-fields.md](theory/relational-transfer-fields.md).
- Application packs follow
  [theory/application-pack-templates.md](theory/application-pack-templates.md).

Draft local imports stay in their import artifacts until they are fit-scored,
role-reviewed, and promoted.

## Reviewed Docs-Catalog Candidates

These rows are reviewed candidates that passed accepted-catalog review for docs
use but are not part of the accepted starter index or default Rust search.

| ID | Frame | Status | Claim strength | Risk band | Application packs | Source family | Authority term | Relation term | Transfer strength | Review note |
|---|---|---|---|---|---|---|---|---|---|---|
| `veto-rule` | Veto Rule | accepted with caveat | heuristic | medium | product, operations, leadership, AI-agent | gates and locks | reviewer | required_gate | structural for true required dimensions | Use only when a requirement, owner, evidence, and clearance or waiver condition are explicit. |

Review record:

- [theory/accepted-catalog-review-veto-rule.md](theory/accepted-catalog-review-veto-rule.md)
  records the caveat, stop condition, fallback, and no-default-search boundary.

Review-only row modeling:

- [theory/review-only-catalog-data-model.md](theory/review-only-catalog-data-model.md)
  defines how docs-catalog, draft, held, deprecated, rejected, and anti-pattern
  rows can be loaded for review or suppressed-candidate explanation without
  becoming default recommendations.

## Typed Related-Frame Application

Current Rust entries expose untyped `related` IDs. The first docs-level typed
relation map lives in
[theory/related-frame-application-starter.md](theory/related-frame-application-starter.md).
It classifies starter links such as `four-way-stop` to `merge-lane` as
`alternate`, `four-way-stop` to `crosswalk-yield` as `boundary_frame`, and
`merge-lane` to `following-distance` as `sequence_before`.

## Status Frames

| Frame | Everyday source | Target situation | Action cue | Failure mode |
|---|---|---|---|---|
| Red / yellow / green | Traffic signal | Progress or readiness | Stop, watch, or proceed | Hides why status changed unless paired with evidence. |
| Fuel gauge | Driving range | Resource burn or runway | Refill, reduce consumption, or plan a stop | Can imply a single resource when many constraints matter. |
| Dashboard warning light | Vehicle diagnostics | Emerging risk | Inspect soon before failure becomes expensive | Overused warnings can become background noise. |
| Speed limit | Posted road limit | Execution pace under constraints | Set an upper bound before speed becomes unsafe | Can be misused as a universal cap instead of a context rule. |
| Weather forecast | Near-term conditions | Uncertain external pressure | Prepare for likely conditions without demanding certainty | Can make controllable problems sound inevitable. |

## Coordination Frames

| Frame | Everyday source | Target situation | Action cue | Failure mode |
|---|---|---|---|---|
| Four-way stop | Drivers negotiating turn order | Peer teams sharing constrained attention | Arrive, pause, read priority, proceed visibly | Breaks when authority or safety rules are not equal. |
| Merge lane | Joining traffic | Integrating work into an active system | Match speed, signal intent, find a gap | Can understate the need for explicit approval gates. |
| Crosswalk yield | Pedestrian priority | Protecting vulnerable users or downstream teams | Slow down and give space before optimizing throughput | Can be patronizing if the "pedestrian" label maps to people. |
| Crowded sidewalk | Shared narrow walkway | Parallel teams with competing movement in one channel | Signal intent early and move long debates out of the flow lane | Can normalize congestion instead of triggering capacity decisions. |
| Trail marker | Route markers on uncertain paths | Shared orientation through changing plans | Place clear direction markers at decision forks and refresh stale guidance | Stale markers create confident drift. |
| Relay handoff | Passing a baton | Ownership transfer | Align speed, pass cleanly, confirm control | Over-focus on the handoff can hide preparation quality. |

## Momentum Frames

| Frame | Everyday source | Target situation | Action cue | Failure mode |
|---|---|---|---|---|
| Walking pace | Sustainable movement | Team execution speed | Choose a pace that can continue | Can excuse low urgency if no deadline exists. |
| Stride length | Step size during walking | Batch size and adjustment frequency | Match step size to feedback speed and correction cost | Long strides can hide rework risk; short strides can hide avoidance. |
| Downshift | Driving uphill or slowing safely | Reducing scope under load | Trade top speed for control and torque | Can sound like failure unless framed as control. |
| Detour | Blocked route | Changed plan | Preserve destination while changing path | Can hide whether the destination is still valid. |
| Rest stop | Long trip | Planned recovery | Pause deliberately before fatigue causes mistakes | Can be mistaken for loss of commitment. |
| Shoulder / pull-off | Safe roadside stop | Temporary pause outside the main flow | Leave the lane, stabilize, and re-enter deliberately | Can normalize stopping without a re-entry plan. |

## Risk Frames

| Frame | Everyday source | Target situation | Action cue | Failure mode |
|---|---|---|---|---|
| Blind spot | Driving awareness gap | Unknown dependency or stakeholder | Check before changing lanes | Can blame individuals for system visibility gaps. |
| Footing | Stable vs unstable walking surface | Execution on uncertain assumptions | Stabilize the surface before optimizing pace | Can overstate risk and block reasonable movement. |
| Following distance | Safe gap between vehicles | Buffer between dependent work items | Create enough space to react without collision | Can be read as slack for its own sake if risk is not named. |
| Stumble and recover | Losing then regaining balance | Minor failure recovery | Recover control first, then adapt before resuming speed | Treating every stumble as a full stop can cause chronic caution. |
| Slippery floor | Walking hazard | Fragile process or environment | Slow down, add support, avoid sudden moves | Can overstate danger when ordinary caution is enough. |
| Boiling pot | Cooking heat | Escalating pressure | Reduce heat or vent before spillover | Can imply emotion rather than structural pressure. |
| Load-bearing wall | Building structure | Critical dependency | Inspect before removing or changing | Can make change feel impossible without analysis. |

## Selection Rules

1. Prefer frames with clear action cues.
2. Prefer frames that reveal timing, priority, or tradeoff.
3. Avoid frames that turn people into obstacles.
4. Pair status frames with evidence.
5. Name where the analogy stops working.
6. Prefer accepted metadata rows for AI/tool suggestions.
7. Treat local draft imports as `locally_observed` until promoted.
