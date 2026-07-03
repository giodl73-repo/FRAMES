//! A small frame index for finding useful analogies.
//!
//! FRAMES treats analogies as structured entries: a familiar source situation,
//! target situations where it helps, action cues, and warnings for misuse.

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
    pub failure_mode: &'static str,
    pub related: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameQuery<'a> {
    pub situation: &'a str,
    pub desired_kind: Option<FrameKind>,
    pub tags: &'a [&'a str],
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
        failure_mode: "Hides why status changed unless paired with evidence.",
        related: &["dashboard-warning-light", "fuel-gauge"],
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
        failure_mode: "Can understate the need for explicit approval gates.",
        related: &["four-way-stop", "crosswalk-yield"],
    },
    FrameEntry {
        id: "detour",
        name: "Detour",
        kind: FrameKind::Momentum,
        everyday_source: "Blocked route",
        target_situations: &["changed plan", "blocked path with stable destination"],
        tags: &["momentum", "planning", "route", "adaptation"],
        action_cue: "Preserve the destination, change the route, and mark the cost.",
        failure_mode: "Can hide whether the destination is still valid.",
        related: &["downshift", "rest-stop"],
    },
    FrameEntry {
        id: "downshift",
        name: "Downshift",
        kind: FrameKind::Momentum,
        everyday_source: "Driving uphill or slowing safely",
        target_situations: &["reducing scope under load", "regaining control"],
        tags: &["momentum", "scope", "control", "load"],
        action_cue: "Trade top speed for control and torque.",
        failure_mode: "Can sound like failure unless framed as control.",
        related: &["detour", "rest-stop"],
    },
    FrameEntry {
        id: "rest-stop",
        name: "Rest stop",
        kind: FrameKind::Momentum,
        everyday_source: "Planned pause on a long trip",
        target_situations: &["planned recovery", "fatigue management"],
        tags: &["momentum", "recovery", "fatigue", "pace"],
        action_cue: "Pause deliberately before fatigue causes mistakes.",
        failure_mode: "Can be mistaken for loss of commitment.",
        related: &["walking-pace", "downshift"],
    },
    FrameEntry {
        id: "walking-pace",
        name: "Walking pace",
        kind: FrameKind::Momentum,
        everyday_source: "Sustainable movement",
        target_situations: &["team execution speed", "sustainable progress"],
        tags: &["momentum", "pace", "sustainability", "progress"],
        action_cue: "Choose a pace that can continue.",
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
        failure_mode: "Can blame individuals for system visibility gaps.",
        related: &["dashboard-warning-light", "load-bearing-wall"],
    },
    FrameEntry {
        id: "dashboard-warning-light",
        name: "Dashboard warning light",
        kind: FrameKind::Risk,
        everyday_source: "Vehicle diagnostics",
        target_situations: &["emerging risk", "early warning signal"],
        tags: &["risk", "status", "warning", "diagnostic"],
        action_cue: "Inspect soon before failure becomes expensive.",
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
        failure_mode: "Can imply a single resource when many constraints matter.",
        related: &["red-yellow-green", "rest-stop"],
    },
    FrameEntry {
        id: "load-bearing-wall",
        name: "Load-bearing wall",
        kind: FrameKind::Risk,
        everyday_source: "Building structure",
        target_situations: &["critical dependency", "structural constraint"],
        tags: &["risk", "dependency", "structure", "constraint"],
        action_cue: "Inspect before removing or changing.",
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
        let results = index.search(&FrameQuery {
            situation: "two teams need turn order around constrained attention",
            desired_kind: Some(FrameKind::Coordination),
            tags: &["priority"],
        });

        assert_eq!(results[0].entry.id, "four-way-stop");
        assert!(results[0].score > 0);
        assert!(results[0].match_notes.kind_match);
    }

    #[test]
    fn related_frames_resolve_from_ids() {
        let index = FrameIndex::new();
        let related = index.related_to("red-yellow-green");
        let ids: Vec<_> = related.iter().map(|entry| entry.id).collect();

        assert_eq!(ids, vec!["dashboard-warning-light", "fuel-gauge"]);
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
