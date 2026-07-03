//! A small frame index for finding useful analogies.
//!
//! FRAMES treats analogies as structured entries: a familiar source situation,
//! target situations where it helps, action cues, evidence boundaries, and
//! warnings for misuse.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    Status,
    Coordination,
    Momentum,
    Risk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: FrameKind,
    pub everyday_source: &'static str,
    pub target_situations: &'static [&'static str],
    pub tags: &'static [&'static str],
    pub action_cue: &'static str,
    pub evidence_boundary: &'static str,
    pub failure_mode: &'static str,
    pub related: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameQuery<'a> {
    pub situation: &'a str,
    pub desired_kind: Option<FrameKind>,
    pub tags: &'a [&'a str],
}

impl<'a> FrameQuery<'a> {
    pub const fn new(situation: &'a str) -> Self {
        Self {
            situation,
            desired_kind: None,
            tags: &[],
        }
    }

    pub const fn with_kind(mut self, kind: FrameKind) -> Self {
        self.desired_kind = Some(kind);
        self
    }

    pub const fn with_tags(mut self, tags: &'a [&'a str]) -> Self {
        self.tags = tags;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameCandidate<'a> {
    pub entry: &'a FrameEntry,
    pub score: u16,
    pub match_notes: MatchNotes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MatchNotes {
    pub kind_match: bool,
    pub situation_hits: u8,
    pub tag_hits: u8,
    pub name_hit: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameIndex {
    entries: &'static [FrameEntry],
}

impl Default for FrameIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameIndex {
    pub const fn new() -> Self {
        Self {
            entries: STARTER_CATALOG,
        }
    }

    pub const fn entries(&self) -> &'static [FrameEntry] {
        self.entries
    }

    pub fn get(&self, id: &str) -> Option<&'static FrameEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    pub fn by_kind(&self, kind: FrameKind) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.kind == kind)
            .collect()
    }

    pub fn with_tag(&self, tag: &str) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| contains_ignore_ascii_case(entry.tags, tag))
            .collect()
    }

    pub fn related_to(&self, id: &str) -> Vec<&'static FrameEntry> {
        let Some(entry) = self.get(id) else {
            return Vec::new();
        };

        entry
            .related
            .iter()
            .filter_map(|related_id| self.get(related_id))
            .collect()
    }

    pub fn search(&self, query: &FrameQuery<'_>) -> Vec<FrameCandidate<'static>> {
        let mut candidates: Vec<_> = self
            .entries
            .iter()
            .filter_map(|entry| score_entry(entry, query))
            .collect();

        candidates.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.entry.name.cmp(right.entry.name))
        });
        candidates
    }

    pub fn search_top(&self, query: &FrameQuery<'_>, limit: usize) -> Vec<FrameCandidate<'static>> {
        self.search(query).into_iter().take(limit).collect()
    }
}

fn score_entry(
    entry: &'static FrameEntry,
    query: &FrameQuery<'_>,
) -> Option<FrameCandidate<'static>> {
    let mut score = 0;
    let mut notes = MatchNotes::default();

    if query.desired_kind == Some(entry.kind) {
        score += 20;
        notes.kind_match = true;
    }

    for target in entry.target_situations {
        if text_overlaps(query.situation, target) {
            score += 12;
            notes.situation_hits = notes.situation_hits.saturating_add(1);
        }
    }

    for tag in query.tags {
        if contains_ignore_ascii_case(entry.tags, tag) {
            score += 8;
            notes.tag_hits = notes.tag_hits.saturating_add(1);
        }
    }

    if contains_word(query.situation, entry.name) {
        score += 6;
        notes.name_hit = true;
    }

    if score == 0 {
        return None;
    }

    Some(FrameCandidate {
        entry,
        score,
        match_notes: notes,
    })
}

fn contains_ignore_ascii_case(haystack: &[&str], needle: &str) -> bool {
    haystack
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(needle))
}

fn contains_word(text: &str, phrase: &str) -> bool {
    let text = text.to_ascii_lowercase();
    let phrase = phrase.to_ascii_lowercase();
    text.contains(&phrase)
}

fn text_overlaps(left: &str, right: &str) -> bool {
    let right = right.to_ascii_lowercase();
    left.split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| word.len() >= 4)
        .map(str::to_ascii_lowercase)
        .any(|word| right.contains(&word))
}

pub const STARTER_CATALOG: &[FrameEntry] = &[
    FrameEntry {
        id: "red-yellow-green",
        name: "Red / yellow / green",
        kind: FrameKind::Status,
        everyday_source: "Traffic signal",
        target_situations: &["progress readiness", "release health", "operational status"],
        tags: &["status", "readiness", "progress", "gate"],
        action_cue: "Stop, watch, or proceed.",
        evidence_boundary: "Check which threshold changed and what would move the status.",
        failure_mode: "Hides why status changed unless paired with evidence.",
        related: &["dashboard-warning-light", "fuel-gauge", "speed-limit"],
    },
    FrameEntry {
        id: "four-way-stop",
        name: "Four-way stop",
        kind: FrameKind::Coordination,
        everyday_source: "Drivers negotiating turn order",
        target_situations: &[
            "peer teams sharing constrained attention",
            "coordination turn order",
        ],
        tags: &["coordination", "priority", "turn-order", "handoff"],
        action_cue: "Pause, read priority, signal intent, then proceed visibly.",
        evidence_boundary: "Check whether parties are peers or one owner has authority.",
        failure_mode: "Breaks when authority, urgency, or safety rules are not equal.",
        related: &["merge-lane", "crosswalk-yield"],
    },
    FrameEntry {
        id: "crosswalk-yield",
        name: "Crosswalk yield",
        kind: FrameKind::Coordination,
        everyday_source: "Pedestrian priority at a crossing",
        target_situations: &["protecting vulnerable users", "downstream review capacity"],
        tags: &["coordination", "safety", "vulnerability", "priority"],
        action_cue: "The stronger actor absorbs delay before optimizing throughput.",
        evidence_boundary: "Name the protected value and who has the duty to slow down.",
        failure_mode: "Can be patronizing if a person or team is labeled as weak.",
        related: &["four-way-stop", "merge-lane"],
    },
    FrameEntry {
        id: "merge-lane",
        name: "Merge lane",
        kind: FrameKind::Coordination,
        everyday_source: "Joining moving traffic",
        target_situations: &[
            "integrating work into an active system",
            "joining roadmap in flight",
        ],
        tags: &["coordination", "integration", "timing", "handoff"],
        action_cue: "Match speed, signal intent, find a gap, and join predictably.",
        evidence_boundary: "Check whether timing is enough or explicit approval is required.",
        failure_mode: "Can understate the need for explicit approval gates.",
        related: &["four-way-stop", "crosswalk-yield", "following-distance"],
    },
    FrameEntry {
        id: "detour",
        name: "Detour",
        kind: FrameKind::Momentum,
        everyday_source: "Blocked route",
        target_situations: &["changed plan", "blocked path with stable destination"],
        tags: &["momentum", "planning", "route", "adaptation"],
        action_cue: "Preserve the destination, change the route, and mark the cost.",
        evidence_boundary:
            "Verify the destination is still valid and the new route cost is acceptable.",
        failure_mode: "Can hide whether the destination is still valid.",
        related: &["downshift", "rest-stop", "shoulder-pull-off"],
    },
    FrameEntry {
        id: "downshift",
        name: "Downshift",
        kind: FrameKind::Momentum,
        everyday_source: "Driving uphill or slowing safely",
        target_situations: &["reducing scope under load", "regaining control"],
        tags: &["momentum", "scope", "control", "load"],
        action_cue: "Trade top speed for control and torque.",
        evidence_boundary: "Check what load or constraint makes current speed unsafe.",
        failure_mode: "Can sound like failure unless framed as control.",
        related: &["detour", "rest-stop", "speed-limit"],
    },
    FrameEntry {
        id: "rest-stop",
        name: "Rest stop",
        kind: FrameKind::Momentum,
        everyday_source: "Planned pause on a long trip",
        target_situations: &["planned recovery", "fatigue management"],
        tags: &["momentum", "recovery", "fatigue", "pace"],
        action_cue: "Pause deliberately before fatigue causes mistakes.",
        evidence_boundary: "Check fatigue, error rate, and the planned restart condition.",
        failure_mode: "Can be mistaken for loss of commitment.",
        related: &["walking-pace", "downshift", "shoulder-pull-off"],
    },
    FrameEntry {
        id: "walking-pace",
        name: "Walking pace",
        kind: FrameKind::Momentum,
        everyday_source: "Sustainable movement",
        target_situations: &["team execution speed", "sustainable progress"],
        tags: &["momentum", "pace", "sustainability", "progress"],
        action_cue: "Choose a pace that can continue.",
        evidence_boundary: "Check deadline pressure and whether the pace meets required outcomes.",
        failure_mode: "Can excuse low urgency if no deadline exists.",
        related: &["rest-stop", "downshift"],
    },
    FrameEntry {
        id: "blind-spot",
        name: "Blind spot",
        kind: FrameKind::Risk,
        everyday_source: "Driving awareness gap",
        target_situations: &["unknown dependency", "missing stakeholder visibility"],
        tags: &["risk", "visibility", "dependency", "stakeholder"],
        action_cue: "Check before changing lanes.",
        evidence_boundary: "Identify the missing dependency, stakeholder, or signal.",
        failure_mode: "Can blame individuals for system visibility gaps.",
        related: &[
            "dashboard-warning-light",
            "load-bearing-wall",
            "following-distance",
        ],
    },
    FrameEntry {
        id: "dashboard-warning-light",
        name: "Dashboard warning light",
        kind: FrameKind::Risk,
        everyday_source: "Vehicle diagnostics",
        target_situations: &["emerging risk", "early warning signal"],
        tags: &["risk", "status", "warning", "diagnostic"],
        action_cue: "Inspect soon before failure becomes expensive.",
        evidence_boundary: "Check the diagnostic signal, severity, owner, and next inspection.",
        failure_mode: "Overused warnings can become background noise.",
        related: &["red-yellow-green", "blind-spot"],
    },
    FrameEntry {
        id: "fuel-gauge",
        name: "Fuel gauge",
        kind: FrameKind::Status,
        everyday_source: "Driving range",
        target_situations: &["resource burn", "runway", "capacity remaining"],
        tags: &["status", "resource", "capacity", "runway"],
        action_cue: "Refill, reduce consumption, or plan a stop.",
        evidence_boundary: "Check which resource is scarce and whether other constraints matter.",
        failure_mode: "Can imply a single resource when many constraints matter.",
        related: &["red-yellow-green", "rest-stop"],
    },
    FrameEntry {
        id: "speed-limit",
        name: "Speed limit",
        kind: FrameKind::Status,
        everyday_source: "Posted road limit",
        target_situations: &["execution pace under constraints", "maximum safe pace"],
        tags: &["status", "pace", "constraint", "safety"],
        action_cue: "Set an upper bound before speed becomes unsafe.",
        evidence_boundary: "Check which constraint sets the maximum safe pace.",
        failure_mode: "Can be misused as a universal cap instead of a context rule.",
        related: &["downshift", "red-yellow-green", "following-distance"],
    },
    FrameEntry {
        id: "shoulder-pull-off",
        name: "Shoulder / pull-off",
        kind: FrameKind::Momentum,
        everyday_source: "Safe roadside stop",
        target_situations: &["temporary pause outside the main flow", "stabilization"],
        tags: &["momentum", "pause", "stabilize", "reentry"],
        action_cue: "Leave the lane, stabilize, and re-enter deliberately.",
        evidence_boundary: "Check the stabilization task and re-entry condition.",
        failure_mode: "Can normalize stopping without a re-entry plan.",
        related: &["detour", "rest-stop", "merge-lane"],
    },
    FrameEntry {
        id: "following-distance",
        name: "Following distance",
        kind: FrameKind::Risk,
        everyday_source: "Safe gap between vehicles",
        target_situations: &["buffer between dependent work items", "reaction time"],
        tags: &["risk", "buffer", "dependency", "coupling"],
        action_cue: "Create enough space to react without collision.",
        evidence_boundary: "Check coupling, reaction time, and the cost of extra buffer.",
        failure_mode: "Can be read as slack for its own sake if risk is not named.",
        related: &["blind-spot", "merge-lane", "speed-limit"],
    },
    FrameEntry {
        id: "load-bearing-wall",
        name: "Load-bearing wall",
        kind: FrameKind::Risk,
        everyday_source: "Building structure",
        target_situations: &["critical dependency", "structural constraint"],
        tags: &["risk", "dependency", "structure", "constraint"],
        action_cue: "Inspect before removing or changing.",
        evidence_boundary: "Check which dependency is structural and what fails if it changes.",
        failure_mode: "Can make change feel impossible without analysis.",
        related: &["blind-spot", "dashboard-warning-light"],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_ranks_kind_and_tag_matches() {
        let index = FrameIndex::new();
        let results = index.search(
            &FrameQuery::new("two teams need turn order around constrained attention")
                .with_kind(FrameKind::Coordination)
                .with_tags(&["priority"]),
        );

        assert_eq!(results[0].entry.id, "four-way-stop");
        assert!(results[0].score > 0);
        assert!(results[0].match_notes.kind_match);
    }

    #[test]
    fn query_helpers_limit_and_filter_results() {
        let index = FrameIndex::new();
        let results = index.search_top(
            &FrameQuery::new("release status").with_kind(FrameKind::Status),
            1,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(index.by_kind(FrameKind::Status).len(), 3);
        assert_eq!(index.with_tag("priority").len(), 2);
    }

    #[test]
    fn traffic_pack_entries_are_indexed() {
        let index = FrameIndex::new();

        assert!(index.get("speed-limit").is_some());
        assert!(index.get("shoulder-pull-off").is_some());
        assert!(index.get("following-distance").is_some());
        assert_eq!(index.by_kind(FrameKind::Status).len(), 3);
    }

    #[test]
    fn indexed_entries_preserve_evidence_boundaries() {
        let index = FrameIndex::new();

        for entry in index.entries() {
            assert!(!entry.evidence_boundary.is_empty(), "{}", entry.id);
        }
    }

    #[test]
    fn related_frames_resolve_from_ids() {
        let index = FrameIndex::new();
        let related = index.related_to("red-yellow-green");
        let ids: Vec<_> = related.iter().map(|entry| entry.id).collect();

        assert_eq!(
            ids,
            vec!["dashboard-warning-light", "fuel-gauge", "speed-limit"]
        );
    }

    #[test]
    fn unknown_query_returns_empty_results() {
        let index = FrameIndex::new();
        let results = index.search(&FrameQuery {
            situation: "unmatched",
            desired_kind: None,
            tags: &[],
        });

        assert!(results.is_empty());
    }
}
