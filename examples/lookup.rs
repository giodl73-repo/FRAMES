use frames_core::{ApplicationPack, AuthorityModel, FrameIndex, FrameKind, FrameQuery, RiskBand};

fn main() {
    let index = FrameIndex::new();
    let query = FrameQuery::new("two teams need turn order around constrained attention")
        .with_kind(FrameKind::Coordination)
        .with_authority_model(AuthorityModel::Peer)
        .with_risk_band(RiskBand::Medium)
        .with_application_pack(ApplicationPack::Product)
        .with_tags(&["priority"]);

    for candidate in index.search_top(&query, 3) {
        println!(
            "{} [{}]: {}",
            candidate.entry.name, candidate.score, candidate.entry.action_cue
        );
        println!("evidence: {}", candidate.entry.evidence_boundary);
        println!("authority: {}", candidate.entry.authority_model.as_str());
        println!("warning: {}", candidate.entry.failure_mode);
    }
}
