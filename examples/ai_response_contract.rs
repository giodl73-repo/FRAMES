use frames_core::{FrameCandidate, FrameIndex, FrameKind, FrameQuery};

fn main() {
    let target_situation = "two peer teams need turn order around constrained attention";
    let query = FrameQuery::new(target_situation)
        .with_kind(FrameKind::Coordination)
        .with_tags(&["priority"]);

    let index = FrameIndex::new();
    let candidates = index.search_top(&query, 3);
    let Some(recommended) = candidates.first() else {
        println!("plain_language_fallback: No matching frame found. State the decision directly.");
        return;
    };

    print_contract_response(target_situation, recommended, &candidates[1..]);
}

fn print_contract_response(
    target_situation: &str,
    recommended: &FrameCandidate<'_>,
    alternates: &[FrameCandidate<'_>],
) {
    let entry = recommended.entry;

    println!("target_situation: {target_situation}");
    println!("recommended:");
    println!("  frame_id: {}", entry.id);
    println!("  name: {}", entry.name);
    println!("  status: {}", entry.status.as_str());
    println!("  claim_strength: {}", entry.claim_strength.as_str());
    println!("  retrieval_score: {}", recommended.score);
    println!("  fit_score: omitted; human rubric review required");
    println!("  fit_reason: {}", fit_reason(recommended));
    println!("  action_cue: {}", entry.action_cue);
    println!("  evidence_boundary: {}", entry.evidence_boundary);
    println!("  misuse_warning: {}", entry.failure_mode);
    println!("  authority_check: verify that peer coordination applies");
    println!("  risk_band: {}", entry.risk_band.as_str());
    println!(
        "  plain_language_fallback: The teams need an explicit turn order before work proceeds."
    );
    println!("  do_not_use_when:");
    println!("    - one party owns the decision");
    println!("    - urgency or protected-party duty overrides peer sequencing");
    println!("    - the frame would replace evidence or accountability");

    println!("alternates:");
    for alternate in alternates {
        println!("  - frame_id: {}", alternate.entry.id);
        println!("    name: {}", alternate.entry.name);
        println!("    use_when: {}", alternate.entry.action_cue);
    }

    println!("notes:");
    println!("  - Search score is retrieval evidence, not human fit score.");
    println!("  - Frame suggestions explain; they do not prove.");
}

fn fit_reason(candidate: &FrameCandidate<'_>) -> &'static str {
    if candidate.match_notes.kind_match && candidate.match_notes.tag_hits > 0 {
        "Kind and tag matched the requested coordination/priority job."
    } else if candidate.match_notes.kind_match {
        "Kind matched the requested frame job."
    } else if candidate.match_notes.situation_hits > 0 {
        "Target situation text overlapped an indexed use case."
    } else {
        "Candidate was retrieved, but fit needs human review."
    }
}
