use frames_core::{
    ApplicationPack, AuthorityModel, FrameIndex, FrameKind, FrameQuery, ProtectedValue,
    RelationCandidate, RelationQuery, RiskBand, TargetRelation, TransferExclusion,
};

fn main() {
    let index = FrameIndex::new();
    let query = RelationQuery::new(
        FrameQuery::new("one team must slow down to protect vulnerable customers during rollout")
            .with_kind(FrameKind::Coordination)
            .with_authority_model(AuthorityModel::ProtectedParty)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Product),
    )
    .with_target_relation(TargetRelation::ProtectedPartyDuty)
    .with_protected_value(ProtectedValue::CustomerSafety)
    .with_excluded_transfers(&[TransferExclusion::StrongActorDutyShiftedToProtectedParty]);

    let report = index.search_with_relations(&query);

    println!("relation_report:");
    for candidate in report.suggestions.iter().take(3) {
        print_candidate(candidate);
    }

    if !report.suppressed.is_empty() {
        println!("suppressed:");
        for candidate in report.suppressed {
            println!("  - frame_id: {}", candidate.candidate_id);
            println!("    reason: {}", candidate.matched_reason);
            println!("    fallback: {}", candidate.plain_language_fallback);
        }
    }

    if !report.fallbacks.is_empty() {
        println!("fallbacks:");
        for fallback in report.fallbacks {
            println!("  - {fallback}");
        }
    }
}

fn print_candidate(candidate: &RelationCandidate<'_>) {
    let entry = candidate.candidate.entry;

    println!("  - frame_id: {}", entry.id);
    println!("    name: {}", entry.name);
    println!("    relation_score: {}", candidate.relation_score);
    println!("    rank_band: {}", candidate.rank_band.as_str());
    println!("    decision: {}", candidate.decision.as_str());
    println!("    authority_fit: {}", candidate.authority_fit.as_str());
    println!("    matched_relations: {:?}", candidate.matched_relations);
    println!("    evidence_boundary: {}", entry.evidence_boundary);
    println!("    warning: {}", entry.failure_mode);
    if !candidate.warnings.is_empty() {
        println!("    relation_warnings: {:?}", candidate.warnings);
    }
}
