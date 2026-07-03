use frames_core::{FrameIndex, FrameKind, FrameQuery};

fn main() {
    let index = FrameIndex::new();
    let query = FrameQuery::new("two teams need turn order around constrained attention")
        .with_kind(FrameKind::Coordination)
        .with_tags(&["priority"]);

    for candidate in index.search_top(&query, 3) {
        println!(
            "{} [{}]: {}",
            candidate.entry.name, candidate.score, candidate.entry.action_cue
        );
        println!("evidence: {}", candidate.entry.evidence_boundary);
        println!("warning: {}", candidate.entry.failure_mode);
    }
}
