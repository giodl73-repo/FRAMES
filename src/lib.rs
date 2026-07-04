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
pub enum FrameStatus {
    Accepted,
    AcceptedWithCaveat,
    Candidate,
    Draft,
    Held,
    Deprecated,
    Rejected,
    AntiPattern,
}

impl FrameStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::AcceptedWithCaveat => "accepted_with_caveat",
            Self::Candidate => "candidate",
            Self::Draft => "draft",
            Self::Held => "held",
            Self::Deprecated => "deprecated",
            Self::Rejected => "rejected",
            Self::AntiPattern => "anti_pattern",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimStrength {
    Heuristic,
}

impl ClaimStrength {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Heuristic => "heuristic",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskBand {
    Low,
    Medium,
}

impl RiskBand {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityModel {
    Peer,
    Steward,
    Operator,
    ProtectedParty,
    Owner,
    Reviewer,
}

impl AuthorityModel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Peer => "peer",
            Self::Steward => "steward",
            Self::Operator => "operator",
            Self::ProtectedParty => "protected party",
            Self::Owner => "owner",
            Self::Reviewer => "reviewer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationPack {
    Product,
    Operations,
    Leadership,
    Learning,
    AiAgent,
}

impl ApplicationPack {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Product => "product",
            Self::Operations => "operations",
            Self::Leadership => "leadership",
            Self::Learning => "learning",
            Self::AiAgent => "AI-agent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityMode {
    DefaultSearch,
    CatalogReview,
    AntiPatternReview,
    DocsCatalogPreview,
    ExplanationMode,
}

impl VisibilityMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultSearch => "default_search",
            Self::CatalogReview => "catalog_review",
            Self::AntiPatternReview => "anti_pattern_review",
            Self::DocsCatalogPreview => "docs_catalog_preview",
            Self::ExplanationMode => "explanation_mode",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleFilter<'a> {
    pub mode: VisibilityMode,
    pub allowed_statuses: &'a [FrameStatus],
    pub include_docs_catalog: bool,
    pub include_draft: bool,
    pub include_held: bool,
    pub include_rejected: bool,
    pub explain_suppressed: bool,
}

const ACCEPTED_ONLY: &[FrameStatus] = &[FrameStatus::Accepted];
const REVIEW_STATUSES: &[FrameStatus] = &[
    FrameStatus::AcceptedWithCaveat,
    FrameStatus::Candidate,
    FrameStatus::Draft,
    FrameStatus::Held,
    FrameStatus::Deprecated,
    FrameStatus::Rejected,
    FrameStatus::AntiPattern,
];
const ANTI_PATTERN_ONLY: &[FrameStatus] = &[FrameStatus::AntiPattern];
const DOCS_CATALOG_PREVIEW_STATUSES: &[FrameStatus] = &[FrameStatus::AcceptedWithCaveat];

impl<'a> LifecycleFilter<'a> {
    pub const fn default_search() -> Self {
        Self {
            mode: VisibilityMode::DefaultSearch,
            allowed_statuses: ACCEPTED_ONLY,
            include_docs_catalog: false,
            include_draft: false,
            include_held: false,
            include_rejected: false,
            explain_suppressed: false,
        }
    }

    pub const fn explanation_mode() -> Self {
        Self {
            mode: VisibilityMode::ExplanationMode,
            allowed_statuses: ACCEPTED_ONLY,
            include_docs_catalog: false,
            include_draft: false,
            include_held: false,
            include_rejected: false,
            explain_suppressed: true,
        }
    }

    pub const fn catalog_review() -> Self {
        Self {
            mode: VisibilityMode::CatalogReview,
            allowed_statuses: REVIEW_STATUSES,
            include_docs_catalog: true,
            include_draft: true,
            include_held: true,
            include_rejected: true,
            explain_suppressed: true,
        }
    }

    pub const fn anti_pattern_review() -> Self {
        Self {
            mode: VisibilityMode::AntiPatternReview,
            allowed_statuses: ANTI_PATTERN_ONLY,
            include_docs_catalog: false,
            include_draft: false,
            include_held: false,
            include_rejected: true,
            explain_suppressed: true,
        }
    }

    pub const fn docs_catalog_preview() -> Self {
        Self {
            mode: VisibilityMode::DocsCatalogPreview,
            allowed_statuses: DOCS_CATALOG_PREVIEW_STATUSES,
            include_docs_catalog: true,
            include_draft: false,
            include_held: false,
            include_rejected: false,
            explain_suppressed: true,
        }
    }
}

impl Default for LifecycleFilter<'_> {
    fn default() -> Self {
        Self::default_search()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultClass {
    Suggested,
    Alternate,
    Fallback,
    Suppressed,
    ReviewOnly,
}

impl ResultClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Suggested => "suggested",
            Self::Alternate => "alternate",
            Self::Fallback => "fallback",
            Self::Suppressed => "suppressed",
            Self::ReviewOnly => "review_only",
        }
    }

    pub const fn is_recommendation(self) -> bool {
        matches!(self, Self::Suggested | Self::Alternate | Self::Fallback)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayRule {
    SuppressByDefault,
    ExplainWhenRequested,
    ReviewOnly,
    DocsOnly,
}

impl DisplayRule {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuppressByDefault => "suppress_by_default",
            Self::ExplainWhenRequested => "explain_when_requested",
            Self::ReviewOnly => "review_only",
            Self::DocsOnly => "docs_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewFamily {
    DocsCatalogCandidate,
    Candidate,
    Draft,
    Held,
    Deprecated,
    Rejected,
    AntiPattern,
}

impl ReviewFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsCatalogCandidate => "docs_catalog_candidate",
            Self::Candidate => "candidate",
            Self::Draft => "draft",
            Self::Held => "held",
            Self::Deprecated => "deprecated",
            Self::Rejected => "rejected",
            Self::AntiPattern => "anti_pattern",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: FrameKind,
    pub status: FrameStatus,
    pub claim_strength: ClaimStrength,
    pub risk_band: RiskBand,
    pub authority_model: AuthorityModel,
    pub application_packs: &'static [ApplicationPack],
    pub everyday_source: &'static str,
    pub target_situations: &'static [&'static str],
    pub tags: &'static [&'static str],
    pub action_cue: &'static str,
    pub evidence_boundary: &'static str,
    pub failure_mode: &'static str,
    pub related: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewFrameEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub status: FrameStatus,
    pub review_family: ReviewFamily,
    pub claim_strength: &'static str,
    pub risk_band: Option<RiskBand>,
    pub authority_model: Option<AuthorityModel>,
    pub application_packs: &'static [ApplicationPack],
    pub source_family: &'static str,
    pub relation_term: &'static str,
    pub target_situations: &'static [&'static str],
    pub tags: &'static [&'static str],
    pub matched_terms: &'static [&'static str],
    pub action_cue: &'static str,
    pub evidence_boundary: &'static str,
    pub misuse_warning: &'static str,
    pub review_decision: &'static str,
    pub decision_reason: &'static str,
    pub rejection_class: &'static str,
    pub violated_boundary: &'static str,
    pub plain_language_fallback: &'static str,
    pub safer_frame: Option<&'static str>,
    pub source_docs: &'static [&'static str],
    pub display_rule: DisplayRule,
    pub review_status: &'static str,
    pub review_date: &'static str,
    pub revisit_trigger: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetRelation {
    PeerTurnTaking,
    ProtectedPartyDuty,
    DirectedAuthority,
    FlowJoining,
    RequiredGate,
    ThresholdSignal,
    PerspectiveRepair,
    AttentionLimit,
    DependencyIntegrity,
    PaceAdjustment,
    StabilizationReentry,
    RecoveryPause,
    RouteAdjustment,
    ReserveTracking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintRelation {
    Coupling,
    EvidenceMissing,
    AuthorityMissing,
    FactsKnown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedValue {
    CustomerSafety,
    IncidentControl,
    SystemStability,
    DecisionQuality,
    DecisionLegitimacy,
    RepairAccountability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferStrength {
    Structural,
    Partial,
    Dangerous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferExclusion {
    PeerTurnsUnderOwnerAuthority,
    StrongActorDutyShiftedToProtectedParty,
    SpeedWithoutSafetyGate,
    StatusWithoutEvidence,
    UnsupportedAuthorityGate,
    StoryAfterFactsKnown,
    UnknownTreatedAsStructural,
    PauseWithoutRestartCondition,
    StopWithoutDestination,
    ScarceResourceMissing,
    LoadMissing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RelationMetadata {
    frame_id: &'static str,
    target_relations: &'static [TargetRelation],
    constraint_relations: &'static [ConstraintRelation],
    protected_values: &'static [ProtectedValue],
    transfer_strength: TransferStrength,
    exclusions: &'static [TransferExclusion],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankBand {
    Strong,
    UsableWithWarning,
    Demoted,
    Suppressed,
}

impl RankBand {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strong => "strong",
            Self::UsableWithWarning => "usable_with_warning",
            Self::Demoted => "demoted",
            Self::Suppressed => "suppressed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityFit {
    Matches,
    Compatible,
    Mismatch,
    NotAssessed,
}

impl AuthorityFit {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Matches => "matches",
            Self::Compatible => "compatible",
            Self::Mismatch => "mismatch",
            Self::NotAssessed => "not_assessed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationDecision {
    RecommendBoundaryFrame,
    RecommendSequence,
    RecommendWithEvidenceWarning,
    ExplainRejection,
    FallbackOnly,
}

impl RelationDecision {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecommendBoundaryFrame => "recommend-boundary-frame",
            Self::RecommendSequence => "recommend-sequence",
            Self::RecommendWithEvidenceWarning => "recommend-with-evidence-warning",
            Self::ExplainRejection => "explain-rejection",
            Self::FallbackOnly => "fallback-only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationQuery<'a> {
    pub base: FrameQuery<'a>,
    pub target_relation: Option<TargetRelation>,
    pub constraint: Option<ConstraintRelation>,
    pub protected_value: Option<ProtectedValue>,
    pub excluded_transfers: &'a [TransferExclusion],
}

impl<'a> RelationQuery<'a> {
    pub const fn new(base: FrameQuery<'a>) -> Self {
        Self {
            base,
            target_relation: None,
            constraint: None,
            protected_value: None,
            excluded_transfers: &[],
        }
    }

    pub const fn with_target_relation(mut self, relation: TargetRelation) -> Self {
        self.target_relation = Some(relation);
        self
    }

    pub const fn with_constraint(mut self, constraint: ConstraintRelation) -> Self {
        self.constraint = Some(constraint);
        self
    }

    pub const fn with_protected_value(mut self, value: ProtectedValue) -> Self {
        self.protected_value = Some(value);
        self
    }

    pub const fn with_excluded_transfers(mut self, exclusions: &'a [TransferExclusion]) -> Self {
        self.excluded_transfers = exclusions;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameQuery<'a> {
    pub situation: &'a str,
    pub desired_kind: Option<FrameKind>,
    pub authority_model: Option<AuthorityModel>,
    pub risk_band: Option<RiskBand>,
    pub application_pack: Option<ApplicationPack>,
    pub tags: &'a [&'a str],
}

impl<'a> FrameQuery<'a> {
    pub const fn new(situation: &'a str) -> Self {
        Self {
            situation,
            desired_kind: None,
            authority_model: None,
            risk_band: None,
            application_pack: None,
            tags: &[],
        }
    }

    pub const fn with_kind(mut self, kind: FrameKind) -> Self {
        self.desired_kind = Some(kind);
        self
    }

    pub const fn with_authority_model(mut self, authority_model: AuthorityModel) -> Self {
        self.authority_model = Some(authority_model);
        self
    }

    pub const fn with_risk_band(mut self, risk_band: RiskBand) -> Self {
        self.risk_band = Some(risk_band);
        self
    }

    pub const fn with_application_pack(mut self, application_pack: ApplicationPack) -> Self {
        self.application_pack = Some(application_pack);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuppressedCandidate<'a> {
    pub candidate_id: &'a str,
    pub candidate_name: &'a str,
    pub status: FrameStatus,
    pub matched_reason: &'a str,
    pub rejection_class: &'a str,
    pub violated_boundary: &'a str,
    pub safer_frame: Option<&'a str>,
    pub plain_language_fallback: &'a str,
    pub source_docs: &'a [&'a str],
    pub display_rule: DisplayRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewCandidate<'a> {
    pub entry: &'a ReviewFrameEntry,
    pub result_class: ResultClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameSearchReport<'a> {
    pub suggestions: Vec<FrameCandidate<'a>>,
    pub fallbacks: Vec<&'a str>,
    pub suppressed: Vec<SuppressedCandidate<'a>>,
    pub review_only: Vec<ReviewCandidate<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RelationSearchReport<'a> {
    pub suggestions: Vec<RelationCandidate<'a>>,
    pub fallbacks: Vec<&'a str>,
    pub suppressed: Vec<SuppressedCandidate<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationCandidate<'a> {
    pub candidate: FrameCandidate<'a>,
    pub relation_score: i16,
    pub rank_band: RankBand,
    pub matched_relations: Vec<&'a str>,
    pub authority_fit: AuthorityFit,
    pub decision: RelationDecision,
    pub warnings: Vec<&'a str>,
    pub plain_language_fallback: Option<&'a str>,
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

    pub const fn review_entries(&self) -> &'static [ReviewFrameEntry] {
        REVIEW_CATALOG
    }

    pub fn get(&self, id: &str) -> Option<&'static FrameEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    pub fn review_entry(&self, id: &str) -> Option<&'static ReviewFrameEntry> {
        review_entry_by_id(id)
    }

    pub fn review_by_family(&self, family: ReviewFamily) -> Vec<&'static ReviewFrameEntry> {
        REVIEW_CATALOG
            .iter()
            .filter(|entry| entry.review_family == family)
            .collect()
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

    pub fn by_status(&self, status: FrameStatus) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.status == status)
            .collect()
    }

    pub fn by_claim_strength(&self, claim_strength: ClaimStrength) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.claim_strength == claim_strength)
            .collect()
    }

    pub fn by_risk_band(&self, risk_band: RiskBand) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.risk_band == risk_band)
            .collect()
    }

    pub fn by_authority_model(&self, authority_model: AuthorityModel) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.authority_model == authority_model)
            .collect()
    }

    pub fn with_application_pack(&self, pack: ApplicationPack) -> Vec<&'static FrameEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.application_packs.contains(&pack))
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

    pub fn search_with_lifecycle(
        &self,
        query: &FrameQuery<'_>,
        lifecycle: &LifecycleFilter<'_>,
    ) -> FrameSearchReport<'static> {
        let suggestions = if lifecycle.allowed_statuses.contains(&FrameStatus::Accepted) {
            self.search(query)
        } else {
            Vec::new()
        };

        let matched_suppressed = suppressed_for_query(query);
        let fallbacks = unique_fallbacks(&matched_suppressed);
        let suppressed = if lifecycle.explain_suppressed || lifecycle.include_rejected {
            matched_suppressed
        } else {
            Vec::new()
        };
        let review_only = review_entries_for_lifecycle(query, lifecycle);

        FrameSearchReport {
            suggestions,
            fallbacks,
            suppressed,
            review_only,
        }
    }

    pub fn search_with_relations(
        &self,
        query: &RelationQuery<'_>,
    ) -> RelationSearchReport<'static> {
        let suppressed = relation_suppressed_for_query(query);
        let hard_stop_ids: Vec<_> = suppressed
            .iter()
            .map(|candidate| candidate.candidate_id)
            .collect();
        let loose_base = FrameQuery {
            authority_model: None,
            ..query.base.clone()
        };

        let mut suggestions: Vec<_> = self
            .entries
            .iter()
            .filter_map(|entry| relation_candidate_for_entry(entry, &loose_base, query))
            .into_iter()
            .filter(|candidate| !hard_stop_ids.contains(&candidate.entry.id))
            .filter_map(|candidate| relation_candidate(candidate, query))
            .collect();

        suggestions.sort_by(|left, right| {
            right
                .relation_score
                .cmp(&left.relation_score)
                .then_with(|| right.candidate.score.cmp(&left.candidate.score))
                .then_with(|| left.candidate.entry.name.cmp(right.candidate.entry.name))
        });

        let fallbacks = if suggestions.is_empty() {
            relation_fallbacks(query, &suppressed)
        } else {
            Vec::new()
        };

        RelationSearchReport {
            suggestions,
            fallbacks,
            suppressed,
        }
    }
}

fn relation_candidate_for_entry(
    entry: &'static FrameEntry,
    loose_base: &FrameQuery<'_>,
    query: &RelationQuery<'_>,
) -> Option<FrameCandidate<'static>> {
    let metadata = relation_metadata_by_id(entry.id)?;
    if !relation_metadata_relevant(metadata, query) {
        return None;
    }

    Some(score_entry(entry, loose_base).unwrap_or(FrameCandidate {
        entry,
        score: 0,
        match_notes: MatchNotes::default(),
    }))
}

fn relation_metadata_relevant(metadata: &RelationMetadata, query: &RelationQuery<'_>) -> bool {
    if let Some(target_relation) = query.target_relation {
        return metadata.target_relations.contains(&target_relation)
            || query
                .excluded_transfers
                .iter()
                .any(|exclusion| metadata.exclusions.contains(exclusion))
            || relation_composes_with_target(metadata, target_relation, query);
    }

    query
        .constraint
        .is_some_and(|constraint| metadata.constraint_relations.contains(&constraint))
        || query
            .protected_value
            .is_some_and(|value| metadata.protected_values.contains(&value))
        || query
            .excluded_transfers
            .iter()
            .any(|exclusion| metadata.exclusions.contains(exclusion))
}

fn relation_composes_with_target(
    metadata: &RelationMetadata,
    target_relation: TargetRelation,
    query: &RelationQuery<'_>,
) -> bool {
    target_relation == TargetRelation::PaceAdjustment
        && query.constraint == Some(ConstraintRelation::Coupling)
        && query.protected_value == Some(ProtectedValue::SystemStability)
        && metadata
            .target_relations
            .contains(&TargetRelation::FlowJoining)
        && metadata
            .constraint_relations
            .contains(&ConstraintRelation::Coupling)
        || target_relation == TargetRelation::StabilizationReentry
            && metadata.frame_id == "merge-lane"
        || target_relation == TargetRelation::RecoveryPause
            && metadata.frame_id == "shoulder-pull-off"
        || target_relation == TargetRelation::RouteAdjustment
            && metadata.frame_id == "shoulder-pull-off"
        || target_relation == TargetRelation::ReserveTracking
            && metadata.frame_id == "red-yellow-green"
}

fn score_entry(
    entry: &'static FrameEntry,
    query: &FrameQuery<'_>,
) -> Option<FrameCandidate<'static>> {
    if query
        .authority_model
        .is_some_and(|model| entry.authority_model != model)
    {
        return None;
    }

    if query
        .risk_band
        .is_some_and(|risk_band| entry.risk_band != risk_band)
    {
        return None;
    }

    if query
        .application_pack
        .is_some_and(|pack| !entry.application_packs.contains(&pack))
    {
        return None;
    }

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

fn relation_candidate(
    candidate: FrameCandidate<'static>,
    query: &RelationQuery<'_>,
) -> Option<RelationCandidate<'static>> {
    let metadata = relation_metadata_by_id(candidate.entry.id)?;
    let mut relation_score = candidate.score as i16;
    let mut matched_relations = Vec::new();
    let mut warnings = Vec::new();

    if let Some(relation) = query.target_relation {
        if metadata.target_relations.contains(&relation) {
            relation_score += 40;
            matched_relations.push(target_relation_name(relation));
        }
    }

    if let Some(constraint) = query.constraint {
        if metadata.constraint_relations.contains(&constraint) {
            relation_score += 24;
            matched_relations.push(constraint_relation_name(constraint));
        }
    }

    if let Some(value) = query.protected_value {
        if metadata.protected_values.contains(&value) {
            relation_score += 20;
            matched_relations.push(protected_value_name(value));
        } else if value == ProtectedValue::CustomerSafety {
            relation_score -= 30;
            push_unique_warning(&mut warnings, "protected-value warning");
        }
    }

    for exclusion in query.excluded_transfers {
        if metadata.exclusions.contains(exclusion) {
            relation_score -= transfer_exclusion_penalty(*exclusion);
            push_unique_warning(&mut warnings, transfer_exclusion_warning(*exclusion));
        }
    }

    relation_score += match metadata.transfer_strength {
        TransferStrength::Structural => 12,
        TransferStrength::Partial => 4,
        TransferStrength::Dangerous => -80,
    };

    let authority_fit = relation_authority_fit(candidate.entry.authority_model, query);
    match authority_fit {
        AuthorityFit::Matches => relation_score += 10,
        AuthorityFit::Compatible => relation_score += 4,
        AuthorityFit::Mismatch => {
            relation_score -= 40;
            push_unique_warning(&mut warnings, "authority warning");
        }
        AuthorityFit::NotAssessed => {}
    }

    add_relation_warnings(&mut warnings, metadata, query);

    let rank_band = relation_rank_band(relation_score, &warnings);
    let decision = relation_decision(metadata, query, rank_band);

    Some(RelationCandidate {
        candidate,
        relation_score,
        rank_band,
        matched_relations,
        authority_fit,
        decision,
        warnings,
        plain_language_fallback: relation_static_fallback(query),
    })
}

fn relation_suppressed_for_query(query: &RelationQuery<'_>) -> Vec<SuppressedCandidate<'static>> {
    let mut suppressed = suppressed_for_query(&query.base);

    if query.base.authority_model == Some(AuthorityModel::Owner)
        && query.target_relation == Some(TargetRelation::DirectedAuthority)
    {
        suppress_accepted_relation_candidate(
            &mut suppressed,
            "four-way-stop",
            "Four-way stop",
            "peer turn-taking conflicts with owner-directed action",
            "false_authority_transfer",
            "authority",
            "The incident owner should state the decision path and next action.",
        );
        suppress_accepted_relation_candidate(
            &mut suppressed,
            "merge-lane",
            "Merge lane",
            "flow-joining wording conflicts with owner-directed action",
            "false_authority_transfer",
            "authority",
            "The incident owner should state the decision path and next action.",
        );
    }

    suppressed
}

fn suppress_accepted_relation_candidate(
    suppressed: &mut Vec<SuppressedCandidate<'static>>,
    candidate_id: &'static str,
    candidate_name: &'static str,
    matched_reason: &'static str,
    rejection_class: &'static str,
    violated_boundary: &'static str,
    plain_language_fallback: &'static str,
) {
    if suppressed
        .iter()
        .any(|candidate| candidate.candidate_id == candidate_id)
    {
        return;
    }

    suppressed.push(SuppressedCandidate {
        candidate_id,
        candidate_name,
        status: FrameStatus::Accepted,
        matched_reason,
        rejection_class,
        violated_boundary,
        safer_frame: None,
        plain_language_fallback,
        source_docs: &[
            "docs/theory/related-frame-taxonomy.md",
            "docs/theory/transfer-aware-search-design.md",
        ],
        display_rule: DisplayRule::ExplainWhenRequested,
    });
}

fn relation_fallbacks(
    query: &RelationQuery<'_>,
    suppressed: &[SuppressedCandidate<'static>],
) -> Vec<&'static str> {
    if let Some(fallback) = relation_static_fallback(query) {
        return vec![fallback];
    }

    unique_fallbacks(suppressed)
}

fn relation_static_fallback(query: &RelationQuery<'_>) -> Option<&'static str> {
    match (
        query.target_relation,
        query.constraint,
        query.protected_value,
    ) {
        (Some(TargetRelation::DirectedAuthority), _, Some(ProtectedValue::IncidentControl)) => {
            Some("The incident owner should state the decision path and next action.")
        }
        (
            Some(TargetRelation::RequiredGate),
            Some(ConstraintRelation::AuthorityMissing),
            Some(ProtectedValue::DecisionLegitimacy),
        ) => Some("This is an unresolved preference or decision-rights issue."),
        (
            Some(TargetRelation::PerspectiveRepair),
            Some(ConstraintRelation::FactsKnown),
            Some(ProtectedValue::RepairAccountability),
        ) => Some("Facts are now known; name harm, repair, and ownership."),
        _ => None,
    }
}

fn relation_authority_fit(
    candidate_authority: AuthorityModel,
    query: &RelationQuery<'_>,
) -> AuthorityFit {
    let Some(authority) = query.base.authority_model else {
        return AuthorityFit::NotAssessed;
    };

    if candidate_authority == authority {
        return AuthorityFit::Matches;
    }

    if query.target_relation == Some(TargetRelation::ProtectedPartyDuty)
        && candidate_authority == AuthorityModel::Peer
    {
        return AuthorityFit::Mismatch;
    }

    if query.target_relation == Some(TargetRelation::DirectedAuthority)
        && candidate_authority == AuthorityModel::Peer
    {
        return AuthorityFit::Mismatch;
    }

    AuthorityFit::Compatible
}

fn relation_rank_band(score: i16, warnings: &[&str]) -> RankBand {
    if warnings.iter().any(|warning| {
        warning.contains("authority")
            || warning.contains("protected-value")
            || warning.contains("buffer")
            || warning.contains("coupling")
            || warning.contains("structural evidence")
            || warning.contains("restart condition")
            || warning.contains("destination")
            || warning.contains("scarce resource")
            || warning.contains("load")
    }) {
        RankBand::Demoted
    } else if score < 0 {
        RankBand::Suppressed
    } else if score >= 80 {
        RankBand::Strong
    } else {
        RankBand::UsableWithWarning
    }
}

fn relation_decision(
    metadata: &RelationMetadata,
    query: &RelationQuery<'_>,
    rank_band: RankBand,
) -> RelationDecision {
    if rank_band == RankBand::Suppressed
        || metadata.transfer_strength == TransferStrength::Dangerous
    {
        if query.target_relation == Some(TargetRelation::RequiredGate) {
            RelationDecision::ExplainRejection
        } else {
            RelationDecision::FallbackOnly
        }
    } else if query.constraint == Some(ConstraintRelation::Coupling) {
        RelationDecision::RecommendSequence
    } else if query.constraint == Some(ConstraintRelation::EvidenceMissing) {
        RelationDecision::RecommendWithEvidenceWarning
    } else if query.target_relation == Some(TargetRelation::StabilizationReentry) {
        RelationDecision::RecommendSequence
    } else {
        RelationDecision::RecommendBoundaryFrame
    }
}

fn add_relation_warnings(
    warnings: &mut Vec<&'static str>,
    metadata: &RelationMetadata,
    query: &RelationQuery<'_>,
) {
    if query.target_relation == Some(TargetRelation::ProtectedPartyDuty)
        && metadata
            .target_relations
            .contains(&TargetRelation::PeerTurnTaking)
    {
        push_unique_warning(warnings, "authority warning");
    }

    if query.constraint == Some(ConstraintRelation::Coupling)
        && metadata.transfer_strength == TransferStrength::Partial
    {
        push_unique_warning(warnings, "coupling risk");
    }

    if query.constraint == Some(ConstraintRelation::EvidenceMissing) {
        push_unique_warning(warnings, "evidence boundary");
        if query.target_relation == Some(TargetRelation::AttentionLimit) {
            push_unique_warning(warnings, "missing signal");
        } else {
            push_unique_warning(warnings, "threshold required");
        }
    }
}

fn push_unique_warning(warnings: &mut Vec<&'static str>, warning: &'static str) {
    if !warnings.contains(&warning) {
        warnings.push(warning);
    }
}

fn target_relation_name(relation: TargetRelation) -> &'static str {
    match relation {
        TargetRelation::PeerTurnTaking => "peer_turn_taking",
        TargetRelation::ProtectedPartyDuty => "protected_party_duty",
        TargetRelation::DirectedAuthority => "directed_authority",
        TargetRelation::FlowJoining => "flow_joining",
        TargetRelation::RequiredGate => "required_gate",
        TargetRelation::ThresholdSignal => "threshold_signal",
        TargetRelation::PerspectiveRepair => "perspective_repair",
        TargetRelation::AttentionLimit => "attention_limit",
        TargetRelation::DependencyIntegrity => "dependency_integrity",
        TargetRelation::PaceAdjustment => "pace_adjustment",
        TargetRelation::StabilizationReentry => "stabilization_reentry",
        TargetRelation::RecoveryPause => "recovery_pause",
        TargetRelation::RouteAdjustment => "route_adjustment",
        TargetRelation::ReserveTracking => "reserve_tracking",
    }
}

fn constraint_relation_name(relation: ConstraintRelation) -> &'static str {
    match relation {
        ConstraintRelation::Coupling => "coupling",
        ConstraintRelation::EvidenceMissing => "evidence_missing",
        ConstraintRelation::AuthorityMissing => "authority_missing",
        ConstraintRelation::FactsKnown => "facts_known",
    }
}

fn protected_value_name(value: ProtectedValue) -> &'static str {
    match value {
        ProtectedValue::CustomerSafety => "customer safety",
        ProtectedValue::IncidentControl => "incident control",
        ProtectedValue::SystemStability => "system stability",
        ProtectedValue::DecisionQuality => "decision quality",
        ProtectedValue::DecisionLegitimacy => "decision legitimacy",
        ProtectedValue::RepairAccountability => "repair accountability",
    }
}

fn transfer_exclusion_warning(exclusion: TransferExclusion) -> &'static str {
    match exclusion {
        TransferExclusion::PeerTurnsUnderOwnerAuthority => "relation conflict",
        TransferExclusion::StrongActorDutyShiftedToProtectedParty => "protected-value warning",
        TransferExclusion::SpeedWithoutSafetyGate => "buffer evidence required",
        TransferExclusion::StatusWithoutEvidence => "evidence boundary",
        TransferExclusion::UnsupportedAuthorityGate => "false authority transfer",
        TransferExclusion::StoryAfterFactsKnown => "repair required",
        TransferExclusion::UnknownTreatedAsStructural => "structural evidence required",
        TransferExclusion::PauseWithoutRestartCondition => "restart condition required",
        TransferExclusion::StopWithoutDestination => "destination required",
        TransferExclusion::ScarceResourceMissing => "scarce resource required",
        TransferExclusion::LoadMissing => "load required",
    }
}

fn transfer_exclusion_penalty(exclusion: TransferExclusion) -> i16 {
    match exclusion {
        TransferExclusion::StatusWithoutEvidence => 0,
        TransferExclusion::UnknownTreatedAsStructural => 60,
        TransferExclusion::PauseWithoutRestartCondition => 60,
        TransferExclusion::StopWithoutDestination => 60,
        TransferExclusion::ScarceResourceMissing => 60,
        TransferExclusion::LoadMissing => 60,
        TransferExclusion::PeerTurnsUnderOwnerAuthority
        | TransferExclusion::StrongActorDutyShiftedToProtectedParty
        | TransferExclusion::SpeedWithoutSafetyGate => 60,
        TransferExclusion::UnsupportedAuthorityGate | TransferExclusion::StoryAfterFactsKnown => {
            100
        }
    }
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

fn unique_fallbacks(suppressed: &[SuppressedCandidate<'static>]) -> Vec<&'static str> {
    let mut fallbacks = Vec::new();
    for candidate in suppressed {
        if !fallbacks.contains(&candidate.plain_language_fallback) {
            fallbacks.push(candidate.plain_language_fallback);
        }
    }
    fallbacks
}

fn review_entries_for_lifecycle(
    query: &FrameQuery<'_>,
    lifecycle: &LifecycleFilter<'_>,
) -> Vec<ReviewCandidate<'static>> {
    if matches!(
        lifecycle.mode,
        VisibilityMode::DefaultSearch | VisibilityMode::ExplanationMode
    ) {
        return Vec::new();
    }

    REVIEW_CATALOG
        .iter()
        .filter(|entry| review_entry_allowed(entry, lifecycle))
        .filter(|entry| review_entry_matches_query(entry, query))
        .map(|entry| ReviewCandidate {
            entry,
            result_class: ResultClass::ReviewOnly,
        })
        .collect()
}

fn review_entry_allowed(entry: &ReviewFrameEntry, lifecycle: &LifecycleFilter<'_>) -> bool {
    match lifecycle.mode {
        VisibilityMode::CatalogReview => lifecycle.allowed_statuses.contains(&entry.status),
        VisibilityMode::AntiPatternReview => entry.review_family == ReviewFamily::AntiPattern,
        VisibilityMode::DocsCatalogPreview => {
            entry.review_family == ReviewFamily::DocsCatalogCandidate
                && lifecycle.allowed_statuses.contains(&entry.status)
        }
        VisibilityMode::DefaultSearch | VisibilityMode::ExplanationMode => false,
    }
}

fn review_entry_matches_query(entry: &ReviewFrameEntry, query: &FrameQuery<'_>) -> bool {
    if query
        .authority_model
        .is_some_and(|model| entry.authority_model != Some(model))
    {
        return false;
    }

    if query
        .risk_band
        .is_some_and(|risk_band| entry.risk_band != Some(risk_band))
    {
        return false;
    }

    if query
        .application_pack
        .is_some_and(|pack| !entry.application_packs.contains(&pack))
    {
        return false;
    }

    if query
        .tags
        .iter()
        .any(|tag| !contains_ignore_ascii_case(entry.tags, tag))
    {
        return false;
    }

    let situation = query.situation.trim();
    if situation.is_empty() {
        return true;
    }

    contains_word(situation, entry.id)
        || contains_word(situation, entry.name)
        || entry
            .matched_terms
            .iter()
            .any(|term| contains_word(situation, term))
        || entry
            .target_situations
            .iter()
            .any(|target| text_overlaps(situation, target))
}

fn suppressed_for_query(query: &FrameQuery<'_>) -> Vec<SuppressedCandidate<'static>> {
    let mut suppressed: Vec<_> = REVIEW_SUPPRESSION_RULES
        .iter()
        .filter_map(|rule| rule.to_suppressed(query))
        .collect();

    suppressed.extend(
        ACCEPTED_SUPPRESSION_RULES
            .iter()
            .filter(|rule| rule.matches(query))
            .map(|rule| rule.report),
    );

    suppressed
}

#[derive(Debug, Clone, Copy)]
struct ReviewSuppressionRule {
    review_id: &'static str,
    terms: &'static [&'static str],
    authority_model: Option<AuthorityModel>,
    application_pack: Option<ApplicationPack>,
    matched_reason: &'static str,
    rejection_class: Option<&'static str>,
    violated_boundary: Option<&'static str>,
    plain_language_fallback: Option<&'static str>,
    source_docs: Option<&'static [&'static str]>,
    display_rule: Option<DisplayRule>,
}

impl ReviewSuppressionRule {
    fn to_suppressed(self, query: &FrameQuery<'_>) -> Option<SuppressedCandidate<'static>> {
        if !self.matches(query) {
            return None;
        }

        let entry = review_entry_by_id(self.review_id)?;
        Some(SuppressedCandidate {
            candidate_id: entry.id,
            candidate_name: entry.name,
            status: entry.status,
            matched_reason: self.matched_reason,
            rejection_class: self.rejection_class.unwrap_or(entry.rejection_class),
            violated_boundary: self.violated_boundary.unwrap_or(entry.violated_boundary),
            safer_frame: entry.safer_frame,
            plain_language_fallback: self
                .plain_language_fallback
                .unwrap_or(entry.plain_language_fallback),
            source_docs: self.source_docs.unwrap_or(entry.source_docs),
            display_rule: self.display_rule.unwrap_or(entry.display_rule),
        })
    }

    fn matches(self, query: &FrameQuery<'_>) -> bool {
        if self
            .authority_model
            .is_some_and(|model| query.authority_model != Some(model))
        {
            return false;
        }

        if self
            .application_pack
            .is_some_and(|pack| query.application_pack != Some(pack))
        {
            return false;
        }

        self.terms
            .iter()
            .any(|term| contains_word(query.situation, term))
    }
}

#[derive(Debug, Clone, Copy)]
struct AcceptedSuppressionRule {
    terms: &'static [&'static str],
    authority_model: Option<AuthorityModel>,
    application_pack: Option<ApplicationPack>,
    report: SuppressedCandidate<'static>,
}

impl AcceptedSuppressionRule {
    fn matches(self, query: &FrameQuery<'_>) -> bool {
        if self
            .authority_model
            .is_some_and(|model| query.authority_model != Some(model))
        {
            return false;
        }

        if self
            .application_pack
            .is_some_and(|pack| query.application_pack != Some(pack))
        {
            return false;
        }

        self.terms
            .iter()
            .any(|term| contains_word(query.situation, term))
    }
}

fn review_entry_by_id(id: &str) -> Option<&'static ReviewFrameEntry> {
    REVIEW_CATALOG.iter().find(|entry| entry.id == id)
}

#[allow(dead_code)]
fn relation_metadata_by_id(id: &str) -> Option<&'static RelationMetadata> {
    RELATION_METADATA
        .iter()
        .find(|metadata| metadata.frame_id == id)
}

#[allow(dead_code)]
const RELATION_METADATA: &[RelationMetadata] = &[
    RelationMetadata {
        frame_id: "four-way-stop",
        target_relations: &[TargetRelation::PeerTurnTaking],
        constraint_relations: &[],
        protected_values: &[],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[
            TransferExclusion::PeerTurnsUnderOwnerAuthority,
            TransferExclusion::StrongActorDutyShiftedToProtectedParty,
        ],
    },
    RelationMetadata {
        frame_id: "crosswalk-yield",
        target_relations: &[TargetRelation::ProtectedPartyDuty],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::CustomerSafety],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "merge-lane",
        target_relations: &[TargetRelation::FlowJoining],
        constraint_relations: &[],
        protected_values: &[],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[
            TransferExclusion::PeerTurnsUnderOwnerAuthority,
            TransferExclusion::SpeedWithoutSafetyGate,
        ],
    },
    RelationMetadata {
        frame_id: "following-distance",
        target_relations: &[TargetRelation::FlowJoining],
        constraint_relations: &[ConstraintRelation::Coupling],
        protected_values: &[ProtectedValue::SystemStability],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "fuel-gauge",
        target_relations: &[TargetRelation::ReserveTracking],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "red-yellow-green",
        target_relations: &[TargetRelation::ThresholdSignal],
        constraint_relations: &[ConstraintRelation::EvidenceMissing],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[
            TransferExclusion::StatusWithoutEvidence,
            TransferExclusion::ScarceResourceMissing,
        ],
    },
    RelationMetadata {
        frame_id: "dashboard-warning-light",
        target_relations: &[TargetRelation::ThresholdSignal],
        constraint_relations: &[ConstraintRelation::EvidenceMissing],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "speed-limit",
        target_relations: &[TargetRelation::PaceAdjustment],
        constraint_relations: &[ConstraintRelation::Coupling],
        protected_values: &[ProtectedValue::SystemStability],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[TransferExclusion::LoadMissing],
    },
    RelationMetadata {
        frame_id: "walking-pace",
        target_relations: &[TargetRelation::PaceAdjustment],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[TransferExclusion::LoadMissing],
    },
    RelationMetadata {
        frame_id: "stride-length",
        target_relations: &[TargetRelation::PaceAdjustment],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[TransferExclusion::LoadMissing],
    },
    RelationMetadata {
        frame_id: "crowded-sidewalk",
        target_relations: &[TargetRelation::FlowJoining],
        constraint_relations: &[ConstraintRelation::Coupling],
        protected_values: &[ProtectedValue::SystemStability],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[TransferExclusion::SpeedWithoutSafetyGate],
    },
    RelationMetadata {
        frame_id: "trail-marker",
        target_relations: &[TargetRelation::RouteAdjustment],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[TransferExclusion::StopWithoutDestination],
    },
    RelationMetadata {
        frame_id: "stumble-and-recover",
        target_relations: &[TargetRelation::RecoveryPause],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Partial,
        exclusions: &[TransferExclusion::PauseWithoutRestartCondition],
    },
    RelationMetadata {
        frame_id: "downshift",
        target_relations: &[TargetRelation::PaceAdjustment],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::SystemStability],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "shoulder-pull-off",
        target_relations: &[TargetRelation::StabilizationReentry],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::SystemStability],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[
            TransferExclusion::PauseWithoutRestartCondition,
            TransferExclusion::StopWithoutDestination,
        ],
    },
    RelationMetadata {
        frame_id: "detour",
        target_relations: &[TargetRelation::RouteAdjustment],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "rest-stop",
        target_relations: &[TargetRelation::RecoveryPause],
        constraint_relations: &[],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "blind-spot",
        target_relations: &[TargetRelation::AttentionLimit],
        constraint_relations: &[ConstraintRelation::EvidenceMissing],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "footing",
        target_relations: &[TargetRelation::DependencyIntegrity],
        constraint_relations: &[ConstraintRelation::EvidenceMissing],
        protected_values: &[ProtectedValue::DecisionQuality],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[],
    },
    RelationMetadata {
        frame_id: "load-bearing-wall",
        target_relations: &[TargetRelation::DependencyIntegrity],
        constraint_relations: &[ConstraintRelation::EvidenceMissing],
        protected_values: &[ProtectedValue::SystemStability],
        transfer_strength: TransferStrength::Structural,
        exclusions: &[TransferExclusion::UnknownTreatedAsStructural],
    },
    RelationMetadata {
        frame_id: "veto-rule",
        target_relations: &[TargetRelation::RequiredGate],
        constraint_relations: &[ConstraintRelation::AuthorityMissing],
        protected_values: &[ProtectedValue::DecisionLegitimacy],
        transfer_strength: TransferStrength::Dangerous,
        exclusions: &[TransferExclusion::UnsupportedAuthorityGate],
    },
    RelationMetadata {
        frame_id: "bag-of-chips-as-excuse",
        target_relations: &[TargetRelation::PerspectiveRepair],
        constraint_relations: &[ConstraintRelation::FactsKnown],
        protected_values: &[ProtectedValue::RepairAccountability],
        transfer_strength: TransferStrength::Dangerous,
        exclusions: &[TransferExclusion::StoryAfterFactsKnown],
    },
];

const REVIEW_SUPPRESSION_RULES: &[ReviewSuppressionRule] = &[
    ReviewSuppressionRule {
        review_id: "veto-rule",
        terms: &[
            "privacy approval",
            "required approval",
            "approval is missing",
        ],
        authority_model: Some(AuthorityModel::Reviewer),
        application_pack: Some(ApplicationPack::Product),
        matched_reason: "required approval gate matches the query",
        rejection_class: Some("docs_catalog_not_default_search"),
        violated_boundary: Some("lifecycle status"),
        plain_language_fallback: None,
        source_docs: Some(&[
            "docs/theory/accepted-catalog-review-veto-rule.md",
            "docs/frame-catalog.md",
        ]),
        display_rule: Some(DisplayRule::SuppressByDefault),
    },
    ReviewSuppressionRule {
        review_id: "veto-rule",
        terms: &["vetoed", "veto"],
        authority_model: None,
        application_pack: Some(ApplicationPack::Leadership),
        matched_reason: "veto wording and required-gate relation match the query",
        rejection_class: None,
        violated_boundary: Some("authority and evidence"),
        plain_language_fallback: Some("This is an unresolved preference or decision-rights issue."),
        source_docs: Some(&["docs/theory/frame-antipattern-application-veto-rule.md"]),
        display_rule: Some(DisplayRule::ExplainWhenRequested),
    },
    ReviewSuppressionRule {
        review_id: "team-as-roadblock",
        terms: &["roadblock"],
        authority_model: Some(AuthorityModel::Owner),
        application_pack: Some(ApplicationPack::Operations),
        matched_reason: "roadblock wording directly matches the query",
        rejection_class: None,
        violated_boundary: None,
        plain_language_fallback: None,
        source_docs: None,
        display_rule: None,
    },
    ReviewSuppressionRule {
        review_id: "veto-rule",
        terms: &["veto rule", "requirement authority"],
        authority_model: None,
        application_pack: Some(ApplicationPack::AiAgent),
        matched_reason: "required-gate relation and veto wording match the query",
        rejection_class: Some("rejected_near_miss"),
        violated_boundary: Some("authority and evidence"),
        plain_language_fallback: Some(
            "Name the decision owner, tradeoff, and evidence before treating this as blocking.",
        ),
        source_docs: Some(&[
            "docs/theory/related-frame-application-starter.md",
            "docs/theory/frame-antipattern-application-veto-rule.md",
        ]),
        display_rule: Some(DisplayRule::ExplainWhenRequested),
    },
    ReviewSuppressionRule {
        review_id: "bag-of-chips-as-excuse",
        terms: &["bag-of-chips", "facts establish harm"],
        authority_model: Some(AuthorityModel::Peer),
        application_pack: Some(ApplicationPack::Leadership),
        matched_reason: "story source and conflict-empathy job match the query",
        rejection_class: None,
        violated_boundary: None,
        plain_language_fallback: None,
        source_docs: None,
        display_rule: None,
    },
];

const ACCEPTED_SUPPRESSION_RULES: &[AcceptedSuppressionRule] = &[AcceptedSuppressionRule {
    terms: &["incident response", "direct the next action"],
    authority_model: Some(AuthorityModel::Owner),
    application_pack: Some(ApplicationPack::Operations),
    report: SuppressedCandidate {
        candidate_id: "four-way-stop",
        candidate_name: "Four-way stop",
        status: FrameStatus::Accepted,
        matched_reason: "turn-order wording matches the query",
        rejection_class: "false_authority_transfer",
        violated_boundary: "authority",
        safer_frame: None,
        plain_language_fallback:
            "The incident owner should state the decision path and next action.",
        source_docs: &[
            "docs/theory/related-frame-taxonomy.md",
            "docs/theory/transfer-aware-search-design.md",
        ],
        display_rule: DisplayRule::ExplainWhenRequested,
    },
}];

pub const REVIEW_CATALOG: &[ReviewFrameEntry] = &[
    ReviewFrameEntry {
        id: "veto-rule",
        name: "Veto Rule",
        status: FrameStatus::AcceptedWithCaveat,
        review_family: ReviewFamily::DocsCatalogCandidate,
        claim_strength: "heuristic",
        risk_band: Some(RiskBand::Medium),
        authority_model: Some(AuthorityModel::Reviewer),
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
            ApplicationPack::AiAgent,
        ],
        source_family: "gates and locks",
        relation_term: "required_gate",
        target_situations: &[
            "a required approval or non-negotiable requirement blocks launch",
            "an apparently strong option misses a mandatory gate",
        ],
        tags: &["risk", "gate", "approval", "requirement"],
        matched_terms: &[
            "veto",
            "required approval",
            "approval is missing",
            "requirement authority",
        ],
        action_cue: "Name the requirement, owner, evidence, and clearance or waiver condition.",
        evidence_boundary:
            "Name the requirement, accountable owner, evidence, and clearance or waiver condition.",
        misuse_warning:
            "Can turn preferences into false hard stops when authority and evidence are missing.",
        review_decision: "accepted_with_caveat_for_docs_catalog",
        decision_reason:
            "Useful for true required gates, but unsafe for default search until caveat display is guaranteed.",
        rejection_class: "false_authority_transfer",
        violated_boundary: "authority and evidence when unsupported",
        plain_language_fallback:
            "The launch is blocked until the required approval is cleared or waived.",
        safer_frame: None,
        source_docs: &[
            "docs/theory/accepted-catalog-review-veto-rule.md",
            "docs/theory/frame-antipattern-application-veto-rule.md",
            "docs/frame-catalog.md",
        ],
        display_rule: DisplayRule::SuppressByDefault,
        review_status: "accepted-fixture",
        review_date: "2026-07-03",
        revisit_trigger:
            "Caveat display and lifecycle filters are implemented for docs-catalog preview.",
    },
    ReviewFrameEntry {
        id: "team-as-roadblock",
        name: "Team as roadblock",
        status: FrameStatus::AntiPattern,
        review_family: ReviewFamily::AntiPattern,
        claim_strength: "anti_pattern",
        risk_band: Some(RiskBand::Medium),
        authority_model: Some(AuthorityModel::Owner),
        application_packs: &[
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
            ApplicationPack::AiAgent,
        ],
        source_family: "motion and navigation",
        relation_term: "dependency_integrity",
        target_situations: &[
            "a team is described as blocking another team",
            "dependency language turns people into obstacles",
        ],
        tags: &["anti-pattern", "dependency", "ownership", "blame"],
        matched_terms: &["roadblock", "blocking team", "team is blocking"],
        action_cue: "",
        evidence_boundary: "Name the blocked dependency, owner, and decision path.",
        misuse_warning: "Turns people into obstacles and hides ownership or system design.",
        review_decision: "reject_as_anti_pattern",
        decision_reason: "People-as-obstacles framing dehumanizes teams and launders blame.",
        rejection_class: "people_as_obstacles",
        violated_boundary: "human safety and ownership",
        plain_language_fallback:
            "The dependency is blocked by unresolved ownership or a decision path.",
        safer_frame: Some("load-bearing-wall"),
        source_docs: &[
            "docs/theory/frame-antipattern-taxonomy.md",
            "docs/theory/related-frame-application-starter.md",
        ],
        display_rule: DisplayRule::ExplainWhenRequested,
        review_status: "accepted-fixture",
        review_date: "2026-07-03",
        revisit_trigger: "Only reopen if the target is a non-human structural dependency.",
    },
    ReviewFrameEntry {
        id: "bag-of-chips-as-excuse",
        name: "Bag-of-chips story as excuse",
        status: FrameStatus::AntiPattern,
        review_family: ReviewFamily::AntiPattern,
        claim_strength: "anti_pattern",
        risk_band: Some(RiskBand::Medium),
        authority_model: Some(AuthorityModel::Peer),
        application_packs: &[
            ApplicationPack::Leadership,
            ApplicationPack::Learning,
            ApplicationPack::AiAgent,
        ],
        source_family: "social scripts",
        relation_term: "perspective_repair",
        target_situations: &[
            "an empathy story is used after facts establish harm",
            "a perspective-taking story is used to avoid repair",
        ],
        tags: &["anti-pattern", "story", "empathy", "repair"],
        matched_terms: &["bag-of-chips", "chips story", "facts establish harm"],
        action_cue: "",
        evidence_boundary: "Use uncertainty stories only while intent and facts are unresolved.",
        misuse_warning: "Can erase established harm and delay repair or ownership.",
        review_decision: "reject_as_anti_pattern_after_facts_known",
        decision_reason:
            "Perspective stories are useful before facts are known; after harm is established they can become empathy erasers.",
        rejection_class: "empathy_eraser",
        violated_boundary: "repair and accountability",
        plain_language_fallback: "Facts are now known; name harm, repair, and ownership.",
        safer_frame: None,
        source_docs: &["docs/theory/frame-antipattern-taxonomy.md"],
        display_rule: DisplayRule::ExplainWhenRequested,
        review_status: "accepted-fixture",
        review_date: "2026-07-03",
        revisit_trigger: "Use only when facts and intent are still unresolved.",
    },
    ReviewFrameEntry {
        id: "theme-swimlanes",
        name: "Theme Swimlanes",
        status: FrameStatus::Held,
        review_family: ReviewFamily::Held,
        claim_strength: "locally_observed",
        risk_band: Some(RiskBand::Medium),
        authority_model: Some(AuthorityModel::Steward),
        application_packs: &[ApplicationPack::Leadership, ApplicationPack::Operations],
        source_family: "work organization",
        relation_term: "route_adjustment",
        target_situations: &[
            "a program leader wants three contribution lanes under one customer promise",
            "teams need to map work into a few themes without losing ownership",
        ],
        tags: &["theme", "swimlane", "promise", "ownership", "measure"],
        matched_terms: &[
            "swimlane",
            "three themes",
            "run one",
            "run lean",
            "run fast",
            "run safe",
        ],
        action_cue:
            "Name one customer promise, two to four contribution lanes, each lane owner, each measure, and explicit exclusions.",
        evidence_boundary: "Check whether each theme has an owner, measure, and customer promise.",
        misuse_warning:
            "Can become slogan compression when ownership, measures, or tradeoffs are missing.",
        review_decision: "hold_for_pilot_evidence",
        decision_reason:
            "Locally useful pattern needs real pilot records and role gates before accepted catalog or default search.",
        rejection_class: "slogan_compression",
        violated_boundary: "ownership and measurement",
        plain_language_fallback:
            "These are slogans until ownership and measures make them decision lanes.",
        safer_frame: None,
        source_docs: &[
            "docs/theory/theme-swimlane-extraction.md",
            "docs/theory/theme-swimlane-role-review.md",
            "docs/theory/theme-swimlane-leadership-worksheet.md",
            "docs/validation/theme-swimlane-leadership-pilot-ledger.md",
        ],
        display_rule: DisplayRule::ReviewOnly,
        review_status: "role-reviewed",
        review_date: "2026-07-03",
        revisit_trigger:
            "A real pilot records changed or explicitly unchanged decisions with role-gate closeout.",
    },
];

pub const STARTER_CATALOG: &[FrameEntry] = &[
    FrameEntry {
        id: "red-yellow-green",
        name: "Red / yellow / green",
        kind: FrameKind::Status,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
            ApplicationPack::AiAgent,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Peer,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
            ApplicationPack::AiAgent,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::ProtectedParty,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Peer,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::AiAgent,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Low,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Learning,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Low,
        authority_model: AuthorityModel::Steward,
        application_packs: &[
            ApplicationPack::Operations,
            ApplicationPack::Learning,
            ApplicationPack::Leadership,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Low,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Learning,
            ApplicationPack::Operations,
        ],
        everyday_source: "Sustainable movement",
        target_situations: &["team execution speed", "sustainable progress"],
        tags: &["momentum", "pace", "sustainability", "progress"],
        action_cue: "Choose a pace that can continue.",
        evidence_boundary: "Check deadline pressure and whether the pace meets required outcomes.",
        failure_mode: "Can excuse low urgency if no deadline exists.",
        related: &["stride-length", "rest-stop", "downshift"],
    },
    FrameEntry {
        id: "stride-length",
        name: "Stride length",
        kind: FrameKind::Momentum,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Learning,
            ApplicationPack::Operations,
        ],
        everyday_source: "Step size during walking",
        target_situations: &[
            "batch size and adjustment frequency",
            "step size under uncertain correction cost",
        ],
        tags: &["momentum", "pace", "batch-size", "feedback"],
        action_cue: "Match step size to feedback speed and correction cost.",
        evidence_boundary: "Check correction cost, feedback latency, and the current load.",
        failure_mode: "Long strides can hide rework risk; short strides can hide avoidance.",
        related: &["walking-pace", "downshift", "speed-limit"],
    },
    FrameEntry {
        id: "crowded-sidewalk",
        name: "Crowded sidewalk",
        kind: FrameKind::Coordination,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Peer,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
        ],
        everyday_source: "Shared narrow walkway",
        target_situations: &[
            "parallel teams with competing movement in one channel",
            "high-traffic coordination with speed mismatch",
        ],
        tags: &["coordination", "flow", "shared-channel", "signal"],
        action_cue: "Signal intent early and move long debates out of the flow lane.",
        evidence_boundary: "Check channel capacity, role ownership, and queue pressure.",
        failure_mode: "Can normalize congestion instead of triggering capacity decisions.",
        related: &["merge-lane", "four-way-stop", "following-distance"],
    },
    FrameEntry {
        id: "trail-marker",
        name: "Trail marker",
        kind: FrameKind::Coordination,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Steward,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Learning,
        ],
        everyday_source: "Route markers on uncertain paths",
        target_situations: &[
            "shared orientation through changing plans",
            "decision forks where teams may diverge",
        ],
        tags: &["coordination", "wayfinding", "orientation", "decision-fork"],
        action_cue: "Place clear direction markers at decision forks and refresh stale guidance.",
        evidence_boundary: "Check marker freshness, owner, and the active destination.",
        failure_mode: "Stale markers create confident drift.",
        related: &["detour", "walking-pace", "shoulder-pull-off"],
    },
    FrameEntry {
        id: "stumble-and-recover",
        name: "Stumble and recover",
        kind: FrameKind::Momentum,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[ApplicationPack::Product, ApplicationPack::Operations],
        everyday_source: "Losing and regaining balance while walking",
        target_situations: &[
            "minor failure recovery during active delivery",
            "stabilize after a small incident before resuming speed",
        ],
        tags: &["momentum", "recovery", "stability", "incident"],
        action_cue: "Recover control first, then adapt before resuming speed.",
        evidence_boundary: "Check impact, repair completion, and restart safety conditions.",
        failure_mode: "Treating every stumble as a full stop can cause chronic caution.",
        related: &["rest-stop", "shoulder-pull-off", "downshift"],
    },
    FrameEntry {
        id: "blind-spot",
        name: "Blind spot",
        kind: FrameKind::Risk,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Reviewer,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
            ApplicationPack::AiAgent,
        ],
        everyday_source: "Driving awareness gap",
        target_situations: &["unknown dependency", "missing stakeholder visibility"],
        tags: &["risk", "visibility", "dependency", "stakeholder"],
        action_cue: "Check before changing lanes.",
        evidence_boundary: "Identify the missing dependency, stakeholder, or signal.",
        failure_mode: "Can blame individuals for system visibility gaps.",
        related: &["footing", "dashboard-warning-light", "load-bearing-wall"],
    },
    FrameEntry {
        id: "footing",
        name: "Footing",
        kind: FrameKind::Risk,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Reviewer,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Learning,
        ],
        everyday_source: "Stable vs unstable walking surface",
        target_situations: &[
            "execution on uncertain assumptions",
            "fragile surface before scaling a change",
        ],
        tags: &["risk", "assumptions", "stability", "traction"],
        action_cue: "Stabilize assumptions before optimizing pace.",
        evidence_boundary: "Check assumption validity, traction signals, and fallback controls.",
        failure_mode: "Can overstate risk and block reasonable movement.",
        related: &["blind-spot", "load-bearing-wall", "stumble-and-recover"],
    },
    FrameEntry {
        id: "dashboard-warning-light",
        name: "Dashboard warning light",
        kind: FrameKind::Risk,
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Operations,
            ApplicationPack::Product,
            ApplicationPack::AiAgent,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Low,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
            ApplicationPack::AiAgent,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Steward,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[ApplicationPack::Operations, ApplicationPack::Product],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Operator,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::AiAgent,
        ],
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
        status: FrameStatus::Accepted,
        claim_strength: ClaimStrength::Heuristic,
        risk_band: RiskBand::Medium,
        authority_model: AuthorityModel::Owner,
        application_packs: &[
            ApplicationPack::Product,
            ApplicationPack::Operations,
            ApplicationPack::Leadership,
        ],
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
    use std::collections::HashSet;
    use std::path::Path;

    fn repo_has_file(relative_path: &str) -> bool {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(relative_path.replace('/', "\\"))
            .exists()
    }

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
    fn metadata_helpers_filter_entries() {
        let index = FrameIndex::new();

        assert_eq!(index.by_status(FrameStatus::Accepted).len(), 20);
        assert_eq!(index.by_claim_strength(ClaimStrength::Heuristic).len(), 20);
        assert_eq!(index.by_risk_band(RiskBand::Low).len(), 4);
        assert_eq!(index.by_authority_model(AuthorityModel::Peer).len(), 3);
        assert_eq!(
            index.with_application_pack(ApplicationPack::AiAgent).len(),
            7
        );
    }

    #[test]
    fn review_catalog_exposes_review_only_rows() {
        let index = FrameIndex::new();

        assert_eq!(index.review_entries().len(), 4);
        let veto_rule = index.review_entry("veto-rule").unwrap();
        assert_eq!(veto_rule.status, FrameStatus::AcceptedWithCaveat);
        assert_eq!(veto_rule.review_family, ReviewFamily::DocsCatalogCandidate);
        assert_eq!(veto_rule.display_rule, DisplayRule::SuppressByDefault);
        assert!(veto_rule.matched_terms.contains(&"required approval"));

        let anti_patterns = index.review_by_family(ReviewFamily::AntiPattern);
        assert_eq!(anti_patterns.len(), 2);
        assert!(anti_patterns
            .iter()
            .any(|entry| entry.id == "team-as-roadblock"));
    }

    #[test]
    fn review_rows_do_not_enter_starter_catalog_or_default_search() {
        let index = FrameIndex::new();
        let query = FrameQuery::new("privacy approval is missing")
            .with_authority_model(AuthorityModel::Reviewer)
            .with_application_pack(ApplicationPack::Product);

        assert!(index.get("veto-rule").is_none());
        assert!(index.review_entry("veto-rule").is_some());
        assert!(index
            .search(&query)
            .iter()
            .all(|candidate| candidate.entry.id != "veto-rule"));
    }

    #[test]
    fn catalog_review_mode_lists_review_rows_only() {
        let index = FrameIndex::new();
        let report =
            index.search_with_lifecycle(&FrameQuery::new(""), &LifecycleFilter::catalog_review());

        assert!(report.suggestions.is_empty());
        assert!(report.suppressed.is_empty());
        assert_eq!(report.review_only.len(), 4);
        assert!(report
            .review_only
            .iter()
            .all(|candidate| candidate.result_class == ResultClass::ReviewOnly));
        assert!(report
            .review_only
            .iter()
            .any(|candidate| candidate.entry.id == "theme-swimlanes"));
    }

    #[test]
    fn review_modes_filter_by_family_and_query_metadata() {
        let index = FrameIndex::new();
        let anti_pattern_report = index.search_with_lifecycle(
            &FrameQuery::new("").with_application_pack(ApplicationPack::Leadership),
            &LifecycleFilter::anti_pattern_review(),
        );

        assert_eq!(anti_pattern_report.review_only.len(), 2);
        assert!(anti_pattern_report
            .review_only
            .iter()
            .all(|candidate| candidate.entry.review_family == ReviewFamily::AntiPattern));

        let docs_report = index.search_with_lifecycle(
            &FrameQuery::new("required approval is missing")
                .with_authority_model(AuthorityModel::Reviewer)
                .with_application_pack(ApplicationPack::Product),
            &LifecycleFilter::docs_catalog_preview(),
        );

        assert_eq!(docs_report.review_only.len(), 1);
        assert_eq!(docs_report.review_only[0].entry.id, "veto-rule");
        assert!(docs_report.suggestions.is_empty());
    }

    #[test]
    fn relation_metadata_covers_ranking_fixture_ids() {
        let expected_ids = [
            "crosswalk-yield",
            "four-way-stop",
            "merge-lane",
            "following-distance",
            "red-yellow-green",
            "dashboard-warning-light",
            "blind-spot",
            "load-bearing-wall",
            "speed-limit",
            "stride-length",
            "crowded-sidewalk",
            "trail-marker",
            "stumble-and-recover",
            "shoulder-pull-off",
            "rest-stop",
            "detour",
            "fuel-gauge",
            "downshift",
            "footing",
            "veto-rule",
            "bag-of-chips-as-excuse",
        ];

        for id in expected_ids {
            assert!(relation_metadata_by_id(id).is_some(), "{id}");
        }
    }

    #[test]
    fn relation_metadata_covers_all_accepted_entries() {
        let index = FrameIndex::new();

        for entry in index.entries() {
            assert!(
                relation_metadata_by_id(entry.id).is_some(),
                "missing relation metadata for accepted entry: {}",
                entry.id
            );
        }
    }

    #[test]
    fn related_frame_links_are_resolvable_and_not_self_referential() {
        let index = FrameIndex::new();

        for entry in index.entries() {
            assert!(
                !entry.related.is_empty(),
                "accepted entry should expose at least one related frame: {}",
                entry.id
            );

            for related_id in entry.related {
                assert_ne!(
                    *related_id, entry.id,
                    "entry references itself as related: {}",
                    entry.id
                );
                assert!(
                    index.get(related_id).is_some(),
                    "entry has unresolved related frame id: {} -> {}",
                    entry.id,
                    related_id
                );
            }
        }
    }

    #[test]
    fn related_frame_links_are_unique_per_entry() {
        let index = FrameIndex::new();

        for entry in index.entries() {
            let mut related_ids = HashSet::new();
            for related_id in entry.related {
                assert!(
                    related_ids.insert(*related_id),
                    "entry contains duplicate related frame id: {} -> {}",
                    entry.id,
                    related_id
                );
            }
        }
    }

    #[test]
    fn starter_and_relation_metadata_ids_are_unique() {
        let index = FrameIndex::new();

        let mut starter_ids = HashSet::new();
        for entry in index.entries() {
            assert!(
                starter_ids.insert(entry.id),
                "duplicate starter catalog id: {}",
                entry.id
            );
        }

        let mut relation_ids = HashSet::new();
        for metadata in RELATION_METADATA {
            assert!(
                relation_ids.insert(metadata.frame_id),
                "duplicate relation metadata id: {}",
                metadata.frame_id
            );
        }

        let mut review_ids = HashSet::new();
        for entry in index.review_entries() {
            assert!(
                review_ids.insert(entry.id),
                "duplicate review catalog id: {}",
                entry.id
            );
        }
    }

    #[test]
    fn relation_metadata_ids_resolve_to_catalog_entries() {
        let index = FrameIndex::new();

        for metadata in RELATION_METADATA {
            let in_starter = index.get(metadata.frame_id).is_some();
            let in_review = index.review_entry(metadata.frame_id).is_some();
            assert!(
                in_starter || in_review,
                "relation metadata id does not resolve to starter or review catalog: {}",
                metadata.frame_id
            );
        }
    }

    #[test]
    fn relation_metadata_rows_declare_target_relations() {
        for metadata in RELATION_METADATA {
            assert!(
                !metadata.target_relations.is_empty(),
                "relation metadata row has no target relations: {}",
                metadata.frame_id
            );
        }
    }

    #[test]
    fn suppression_rules_reference_existing_candidates() {
        let index = FrameIndex::new();

        for rule in REVIEW_SUPPRESSION_RULES {
            assert!(
                index.review_entry(rule.review_id).is_some(),
                "review suppression rule references unknown review id: {}",
                rule.review_id
            );
        }

        for rule in ACCEPTED_SUPPRESSION_RULES {
            assert!(
                index.get(rule.report.candidate_id).is_some(),
                "accepted suppression rule references unknown starter id: {}",
                rule.report.candidate_id
            );
        }
    }

    #[test]
    fn review_catalog_rows_have_required_operator_fields() {
        let index = FrameIndex::new();

        for entry in index.review_entries() {
            assert!(
                !entry.evidence_boundary.trim().is_empty(),
                "review row missing evidence boundary: {}",
                entry.id
            );
            assert!(
                !entry.misuse_warning.trim().is_empty(),
                "review row missing misuse warning: {}",
                entry.id
            );
            assert!(
                !entry.decision_reason.trim().is_empty(),
                "review row missing decision reason: {}",
                entry.id
            );
            assert!(
                !entry.source_docs.is_empty(),
                "review row missing source docs: {}",
                entry.id
            );

            if entry.status == FrameStatus::AntiPattern {
                assert!(
                    !entry.plain_language_fallback.trim().is_empty(),
                    "anti-pattern row missing plain-language fallback: {}",
                    entry.id
                );
            }
        }
    }

    #[test]
    fn suppression_rules_have_required_fields() {
        for rule in REVIEW_SUPPRESSION_RULES {
            assert!(
                !rule.terms.is_empty(),
                "review suppression rule has no trigger terms: {}",
                rule.review_id
            );
            assert!(
                !rule.matched_reason.trim().is_empty(),
                "review suppression rule missing matched reason: {}",
                rule.review_id
            );
        }

        for rule in ACCEPTED_SUPPRESSION_RULES {
            assert!(
                !rule.terms.is_empty(),
                "accepted suppression rule has no trigger terms: {}",
                rule.report.candidate_id
            );
            assert!(
                !rule.report.matched_reason.trim().is_empty(),
                "accepted suppression report missing matched reason: {}",
                rule.report.candidate_id
            );
        }
    }

    #[test]
    fn accepted_suppression_reports_match_starter_catalog_names() {
        let index = FrameIndex::new();

        for rule in ACCEPTED_SUPPRESSION_RULES {
            let entry = index
                .get(rule.report.candidate_id)
                .expect("suppressed starter candidate should exist");
            assert_eq!(
                rule.report.candidate_name, entry.name,
                "accepted suppression report candidate_name mismatch for {}",
                rule.report.candidate_id
            );
        }
    }

    #[test]
    fn accepted_suppression_reports_have_complete_explanation_fields() {
        for rule in ACCEPTED_SUPPRESSION_RULES {
            let report = rule.report;
            assert!(
                !report.rejection_class.trim().is_empty(),
                "accepted suppression report missing rejection_class: {}",
                report.candidate_id
            );
            assert!(
                !report.violated_boundary.trim().is_empty(),
                "accepted suppression report missing violated_boundary: {}",
                report.candidate_id
            );
            assert!(
                !report.plain_language_fallback.trim().is_empty(),
                "accepted suppression report missing plain_language_fallback: {}",
                report.candidate_id
            );
            assert!(
                !report.source_docs.is_empty(),
                "accepted suppression report missing source_docs: {}",
                report.candidate_id
            );
        }
    }

    #[test]
    fn review_suppression_overrides_are_non_empty_when_present() {
        for rule in REVIEW_SUPPRESSION_RULES {
            if let Some(rejection_class) = rule.rejection_class {
                assert!(
                    !rejection_class.trim().is_empty(),
                    "review suppression override has empty rejection_class: {}",
                    rule.review_id
                );
            }
            if let Some(violated_boundary) = rule.violated_boundary {
                assert!(
                    !violated_boundary.trim().is_empty(),
                    "review suppression override has empty violated_boundary: {}",
                    rule.review_id
                );
            }
            if let Some(plain_language_fallback) = rule.plain_language_fallback {
                assert!(
                    !plain_language_fallback.trim().is_empty(),
                    "review suppression override has empty fallback: {}",
                    rule.review_id
                );
            }
            if let Some(source_docs) = rule.source_docs {
                assert!(
                    !source_docs.is_empty(),
                    "review suppression override has empty source_docs: {}",
                    rule.review_id
                );
            }
        }
    }

    #[test]
    fn review_catalog_source_docs_exist() {
        let index = FrameIndex::new();

        for entry in index.review_entries() {
            for doc in entry.source_docs {
                assert!(
                    repo_has_file(doc),
                    "review row references missing source_doc path: {} -> {}",
                    entry.id,
                    doc
                );
            }
        }
    }

    #[test]
    fn suppression_source_docs_exist_when_provided() {
        for rule in REVIEW_SUPPRESSION_RULES {
            if let Some(source_docs) = rule.source_docs {
                for doc in source_docs {
                    assert!(
                        repo_has_file(doc),
                        "review suppression rule references missing source_doc path: {} -> {}",
                        rule.review_id,
                        doc
                    );
                }
            }
        }

        for rule in ACCEPTED_SUPPRESSION_RULES {
            for doc in rule.report.source_docs {
                assert!(
                    repo_has_file(doc),
                    "accepted suppression report references missing source_doc path: {} -> {}",
                    rule.report.candidate_id,
                    doc
                );
            }
        }
    }

    #[test]
    fn source_doc_lists_are_unique_per_entry() {
        let index = FrameIndex::new();

        for entry in index.review_entries() {
            let mut docs = HashSet::new();
            for doc in entry.source_docs {
                assert!(
                    docs.insert(*doc),
                    "review row has duplicate source_doc reference: {} -> {}",
                    entry.id,
                    doc
                );
            }
        }

        for rule in REVIEW_SUPPRESSION_RULES {
            if let Some(source_docs) = rule.source_docs {
                let mut docs = HashSet::new();
                for doc in source_docs {
                    assert!(
                        docs.insert(*doc),
                        "review suppression rule has duplicate source_doc reference: {} -> {}",
                        rule.review_id,
                        doc
                    );
                }
            }
        }

        for rule in ACCEPTED_SUPPRESSION_RULES {
            let mut docs = HashSet::new();
            for doc in rule.report.source_docs {
                assert!(
                    docs.insert(*doc),
                    "accepted suppression report has duplicate source_doc reference: {} -> {}",
                    rule.report.candidate_id,
                    doc
                );
            }
        }
    }

    #[test]
    fn starter_catalog_rows_have_required_core_fields() {
        let index = FrameIndex::new();

        for entry in index.entries() {
            assert!(
                !entry.target_situations.is_empty(),
                "starter entry missing target_situations: {}",
                entry.id
            );
            assert!(
                !entry.tags.is_empty(),
                "starter entry missing tags: {}",
                entry.id
            );
            assert!(
                !entry.action_cue.trim().is_empty(),
                "starter entry missing action_cue: {}",
                entry.id
            );
            assert!(
                !entry.everyday_source.trim().is_empty(),
                "starter entry missing everyday_source: {}",
                entry.id
            );
            assert!(
                !entry.failure_mode.trim().is_empty(),
                "starter entry missing failure_mode: {}",
                entry.id
            );
        }
    }

    #[test]
    fn review_catalog_rows_have_required_core_fields() {
        let index = FrameIndex::new();

        for entry in index.review_entries() {
            assert!(
                !entry.target_situations.is_empty(),
                "review entry missing target_situations: {}",
                entry.id
            );
            assert!(
                !entry.tags.is_empty(),
                "review entry missing tags: {}",
                entry.id
            );
            assert!(
                !entry.source_family.trim().is_empty(),
                "review entry missing source_family: {}",
                entry.id
            );
            assert!(
                !entry.relation_term.trim().is_empty(),
                "review entry missing relation_term: {}",
                entry.id
            );
            assert!(
                !entry.review_decision.trim().is_empty(),
                "review entry missing review_decision: {}",
                entry.id
            );
        }
    }

    #[test]
    fn review_catalog_rows_have_non_empty_matched_terms() {
        let index = FrameIndex::new();

        for entry in index.review_entries() {
            assert!(
                !entry.matched_terms.is_empty(),
                "review entry missing matched_terms: {}",
                entry.id
            );
            for term in entry.matched_terms {
                assert!(
                    !term.trim().is_empty(),
                    "review entry has blank matched_term: {}",
                    entry.id
                );
            }
        }
    }

    #[test]
    fn suppression_rule_terms_are_unique_and_non_empty() {
        for rule in REVIEW_SUPPRESSION_RULES {
            let mut terms = HashSet::new();
            for term in rule.terms {
                assert!(
                    !term.trim().is_empty(),
                    "review suppression rule has blank term: {}",
                    rule.review_id
                );
                assert!(
                    terms.insert(*term),
                    "review suppression rule has duplicate term: {} -> {}",
                    rule.review_id,
                    term
                );
            }
        }

        for rule in ACCEPTED_SUPPRESSION_RULES {
            let mut terms = HashSet::new();
            for term in rule.terms {
                assert!(
                    !term.trim().is_empty(),
                    "accepted suppression rule has blank term: {}",
                    rule.report.candidate_id
                );
                assert!(
                    terms.insert(*term),
                    "accepted suppression rule has duplicate term: {} -> {}",
                    rule.report.candidate_id,
                    term
                );
            }
        }
    }

    fn docs_accepted_starter_ids() -> HashSet<&'static str> {
        const FRAME_CATALOG_DOC: &str = include_str!("../docs/frame-catalog.md");
        let mut ids = HashSet::new();
        let mut in_accepted_section = false;

        for line in FRAME_CATALOG_DOC.lines() {
            if line.starts_with("## Accepted Starter Metadata") {
                in_accepted_section = true;
                continue;
            }

            if in_accepted_section && line.starts_with("## Reviewed Docs-Catalog Candidates") {
                break;
            }

            if !in_accepted_section {
                continue;
            }

            if line.starts_with("| `") {
                let mut parts = line.split('`');
                let _ = parts.next();
                if let Some(id) = parts.next() {
                    ids.insert(id);
                }
            }
        }

        ids
    }

    #[test]
    fn docs_accepted_starter_ids_match_starter_catalog() {
        let index = FrameIndex::new();
        let starter_ids: HashSet<&'static str> = index.entries().iter().map(|entry| entry.id).collect();
        let doc_ids = docs_accepted_starter_ids();

        assert_eq!(
            starter_ids, doc_ids,
            "accepted starter IDs differ between src/lib.rs STARTER_CATALOG and docs/frame-catalog.md"
        );
    }

    fn extract_fixture_ids_for_key(doc: &str, key: &str) -> HashSet<String> {
        let mut ids = HashSet::new();
        let key_pattern = format!("\"{key}\"");
        let mut cursor = 0usize;

        while let Some(key_pos) = doc[cursor..].find(&key_pattern) {
            let key_abs = cursor + key_pos;
            let after_key = &doc[key_abs + key_pattern.len()..];
            let Some(open_rel) = after_key.find('[') else {
                cursor = key_abs + key_pattern.len();
                continue;
            };
            let array_start = key_abs + key_pattern.len() + open_rel + 1;
            let after_open = &doc[array_start..];
            let Some(close_rel) = after_open.find(']') else {
                break;
            };
            let array_body = &doc[array_start..array_start + close_rel];

            let mut in_quote = false;
            let mut token_start = 0usize;
            for (idx, ch) in array_body.char_indices() {
                if ch == '"' {
                    if in_quote {
                        let token = &array_body[token_start..idx];
                        if !token.is_empty() {
                            ids.insert(token.to_string());
                        }
                        in_quote = false;
                    } else {
                        in_quote = true;
                        token_start = idx + 1;
                    }
                }
            }

            cursor = array_start + close_rel + 1;
        }

        ids
    }

    #[test]
    fn relation_fixtures_reference_existing_catalog_ids() {
        const RELATION_FIXTURES: &str =
            include_str!("../docs/eval/relation-aware-ranking-fixtures.json");
        const SYNTHETIC_SUPPRESSED_IDS: &[&str] = &["green-means-done"];
        let index = FrameIndex::new();
        let mut ids = HashSet::new();
        for key in ["expected_order", "must_demote"] {
            ids.extend(extract_fixture_ids_for_key(RELATION_FIXTURES, key));
        }
        for id in ids {
            let in_starter = index.get(&id).is_some();
            let in_review = index.review_entry(&id).is_some();
            assert!(
                in_starter || in_review,
                "relation fixture references unknown catalog id: {}",
                id
            );
        }

        for id in extract_fixture_ids_for_key(RELATION_FIXTURES, "must_suppress") {
            let in_starter = index.get(&id).is_some();
            let in_review = index.review_entry(&id).is_some();
            let is_synthetic = SYNTHETIC_SUPPRESSED_IDS.contains(&id.as_str());
            assert!(
                in_starter || in_review || is_synthetic,
                "relation fixture suppresses unknown id without synthetic allowlist entry: {}",
                id
            );
        }
    }

    #[test]
    fn accepted_relation_metadata_is_not_dangerous() {
        let index = FrameIndex::new();

        for entry in index.entries() {
            let metadata =
                relation_metadata_by_id(entry.id).expect("accepted entry must have relation metadata");
            assert_ne!(
                metadata.transfer_strength,
                TransferStrength::Dangerous,
                "accepted starter entry has dangerous transfer strength: {}",
                entry.id
            );
        }
    }

    #[test]
    fn fixture_ranked_starter_ids_have_relation_metadata() {
        const RELATION_FIXTURES: &str =
            include_str!("../docs/eval/relation-aware-ranking-fixtures.json");
        let index = FrameIndex::new();
        let mut ranked_ids = HashSet::new();
        ranked_ids.extend(extract_fixture_ids_for_key(RELATION_FIXTURES, "expected_order"));
        ranked_ids.extend(extract_fixture_ids_for_key(RELATION_FIXTURES, "must_demote"));

        for id in ranked_ids {
            if index.get(&id).is_some() {
                assert!(
                    relation_metadata_by_id(&id).is_some(),
                    "fixture-ranked starter id is missing relation metadata: {}",
                    id
                );
            }
        }
    }

    #[test]
    fn relation_metadata_maps_first_ranking_fixtures() {
        let crosswalk = relation_metadata_by_id("crosswalk-yield").unwrap();
        let four_way = relation_metadata_by_id("four-way-stop").unwrap();
        assert!(crosswalk
            .target_relations
            .contains(&TargetRelation::ProtectedPartyDuty));
        assert!(crosswalk
            .protected_values
            .contains(&ProtectedValue::CustomerSafety));
        assert!(four_way
            .target_relations
            .contains(&TargetRelation::PeerTurnTaking));
        assert!(four_way
            .exclusions
            .contains(&TransferExclusion::StrongActorDutyShiftedToProtectedParty));

        let following_distance = relation_metadata_by_id("following-distance").unwrap();
        let merge_lane = relation_metadata_by_id("merge-lane").unwrap();
        assert_eq!(
            following_distance.transfer_strength,
            TransferStrength::Structural
        );
        assert_eq!(merge_lane.transfer_strength, TransferStrength::Partial);
        assert!(following_distance
            .constraint_relations
            .contains(&ConstraintRelation::Coupling));

        let veto_rule = relation_metadata_by_id("veto-rule").unwrap();
        let chips = relation_metadata_by_id("bag-of-chips-as-excuse").unwrap();
        assert_eq!(veto_rule.transfer_strength, TransferStrength::Dangerous);
        assert!(veto_rule
            .exclusions
            .contains(&TransferExclusion::UnsupportedAuthorityGate));
        assert_eq!(chips.transfer_strength, TransferStrength::Dangerous);
        assert!(chips
            .constraint_relations
            .contains(&ConstraintRelation::FactsKnown));

        let blind_spot = relation_metadata_by_id("blind-spot").unwrap();
        let footing = relation_metadata_by_id("footing").unwrap();
        let load_bearing_wall = relation_metadata_by_id("load-bearing-wall").unwrap();
        let speed_limit = relation_metadata_by_id("speed-limit").unwrap();
        let stride_length = relation_metadata_by_id("stride-length").unwrap();
        let crowded_sidewalk = relation_metadata_by_id("crowded-sidewalk").unwrap();
        let trail_marker = relation_metadata_by_id("trail-marker").unwrap();
        let stumble_and_recover = relation_metadata_by_id("stumble-and-recover").unwrap();
        let shoulder_pull_off = relation_metadata_by_id("shoulder-pull-off").unwrap();
        let rest_stop = relation_metadata_by_id("rest-stop").unwrap();
        let detour = relation_metadata_by_id("detour").unwrap();
        let fuel_gauge = relation_metadata_by_id("fuel-gauge").unwrap();
        let downshift = relation_metadata_by_id("downshift").unwrap();
        assert!(blind_spot
            .target_relations
            .contains(&TargetRelation::AttentionLimit));
        assert!(footing
            .target_relations
            .contains(&TargetRelation::DependencyIntegrity));
        assert!(load_bearing_wall
            .exclusions
            .contains(&TransferExclusion::UnknownTreatedAsStructural));
        assert!(speed_limit
            .target_relations
            .contains(&TargetRelation::PaceAdjustment));
        assert!(stride_length
            .target_relations
            .contains(&TargetRelation::PaceAdjustment));
        assert!(crowded_sidewalk
            .target_relations
            .contains(&TargetRelation::FlowJoining));
        assert!(trail_marker
            .target_relations
            .contains(&TargetRelation::RouteAdjustment));
        assert!(stumble_and_recover
            .target_relations
            .contains(&TargetRelation::RecoveryPause));
        assert!(shoulder_pull_off
            .target_relations
            .contains(&TargetRelation::StabilizationReentry));
        assert!(rest_stop
            .target_relations
            .contains(&TargetRelation::RecoveryPause));
        assert!(detour
            .target_relations
            .contains(&TargetRelation::RouteAdjustment));
        assert!(fuel_gauge
            .target_relations
            .contains(&TargetRelation::ReserveTracking));
        assert!(downshift
            .target_relations
            .contains(&TargetRelation::PaceAdjustment));
        assert!(speed_limit
            .exclusions
            .contains(&TransferExclusion::LoadMissing));
    }

    #[test]
    fn relation_report_prioritizes_stride_length_for_pace_adjustment() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("stride length should match feedback speed and correction cost")
                .with_kind(FrameKind::Momentum)
                .with_authority_model(AuthorityModel::Operator)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Learning),
        )
        .with_target_relation(TargetRelation::PaceAdjustment)
        .with_protected_value(ProtectedValue::DecisionQuality);

        let report = index.search_with_relations(&query);
        assert_eq!(report.suggestions[0].candidate.entry.id, "stride-length");
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendBoundaryFrame
        );
    }

    #[test]
    fn relation_report_checks_footing_before_structural_claim() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "footing feels unstable because assumptions are uncertain before changing dependencies",
            )
            .with_kind(FrameKind::Risk)
            .with_authority_model(AuthorityModel::Reviewer)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::DependencyIntegrity)
        .with_constraint(ConstraintRelation::EvidenceMissing)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["footing", "load-bearing-wall"]);
        assert!(report.suggestions[0]
            .warnings
            .contains(&"evidence boundary"));
    }

    #[test]
    fn relation_metadata_does_not_change_default_search() {
        let index = FrameIndex::new();
        let query = FrameQuery::new("two teams need turn order around constrained attention")
            .with_kind(FrameKind::Coordination)
            .with_authority_model(AuthorityModel::Peer)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Product)
            .with_tags(&["priority"]);

        let results = index.search(&query);

        assert_eq!(results[0].entry.id, "four-way-stop");
        assert!(relation_metadata_by_id(results[0].entry.id).is_some());
    }

    #[test]
    fn relation_report_prefers_protected_party_duty_over_peer_turns() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "one team must slow down to protect vulnerable customers during rollout",
            )
            .with_kind(FrameKind::Coordination)
            .with_authority_model(AuthorityModel::ProtectedParty)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::ProtectedPartyDuty)
        .with_protected_value(ProtectedValue::CustomerSafety)
        .with_excluded_transfers(&[TransferExclusion::StrongActorDutyShiftedToProtectedParty]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["crosswalk-yield", "four-way-stop"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendBoundaryFrame
        );
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1]
            .warnings
            .contains(&"protected-value warning"));
    }

    #[test]
    fn relation_report_hard_stops_peer_frames_under_owner_authority() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "an incident owner must direct the next action but the words say teams need turns",
            )
            .with_kind(FrameKind::Coordination)
            .with_authority_model(AuthorityModel::Owner)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Operations),
        )
        .with_target_relation(TargetRelation::DirectedAuthority)
        .with_protected_value(ProtectedValue::IncidentControl);

        let report = index.search_with_relations(&query);

        assert!(report.suggestions.is_empty());
        assert_eq!(
            report.fallbacks,
            vec!["The incident owner should state the decision path and next action."]
        );
        assert!(report
            .suppressed
            .iter()
            .any(|candidate| candidate.candidate_id == "four-way-stop"));
        assert!(report
            .suppressed
            .iter()
            .any(|candidate| candidate.candidate_id == "merge-lane"));
    }

    #[test]
    fn relation_report_ranks_buffer_before_merge_under_coupling() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "merge work into an active system with tight coupling and little reaction time",
            )
            .with_kind(FrameKind::Risk)
            .with_authority_model(AuthorityModel::Operator)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Operations),
        )
        .with_target_relation(TargetRelation::FlowJoining)
        .with_constraint(ConstraintRelation::Coupling)
        .with_protected_value(ProtectedValue::SystemStability)
        .with_excluded_transfers(&[TransferExclusion::SpeedWithoutSafetyGate]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["following-distance", "crowded-sidewalk"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendSequence
        );
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
    }

    #[test]
    fn relation_report_preserves_status_frame_with_evidence_warning() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("the report says green but no threshold or evidence is named")
                .with_kind(FrameKind::Status)
                .with_authority_model(AuthorityModel::Operator)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::ThresholdSignal)
        .with_constraint(ConstraintRelation::EvidenceMissing)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::StatusWithoutEvidence]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["red-yellow-green", "dashboard-warning-light"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendWithEvidenceWarning
        );
        assert!(report.suggestions[0]
            .warnings
            .contains(&"evidence boundary"));
    }

    #[test]
    fn relation_report_suppresses_unsupported_veto_near_miss() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("a senior stakeholder says the option is vetoed without evidence")
                .with_kind(FrameKind::Risk)
                .with_authority_model(AuthorityModel::Reviewer)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Leadership),
        )
        .with_target_relation(TargetRelation::RequiredGate)
        .with_constraint(ConstraintRelation::AuthorityMissing)
        .with_protected_value(ProtectedValue::DecisionLegitimacy)
        .with_excluded_transfers(&[TransferExclusion::UnsupportedAuthorityGate]);

        let report = index.search_with_relations(&query);

        assert!(report.suggestions.is_empty());
        assert_eq!(
            report.fallbacks,
            vec!["This is an unresolved preference or decision-rights issue."]
        );
        assert!(report
            .suppressed
            .iter()
            .any(|candidate| candidate.candidate_id == "veto-rule"));
    }

    #[test]
    fn relation_report_suppresses_story_after_facts_known() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("a bag-of-chips story is suggested after facts establish harm")
                .with_authority_model(AuthorityModel::Peer)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Leadership),
        )
        .with_target_relation(TargetRelation::PerspectiveRepair)
        .with_constraint(ConstraintRelation::FactsKnown)
        .with_protected_value(ProtectedValue::RepairAccountability)
        .with_excluded_transfers(&[TransferExclusion::StoryAfterFactsKnown]);

        let report = index.search_with_relations(&query);

        assert!(report.suggestions.is_empty());
        assert_eq!(
            report.fallbacks,
            vec!["Facts are now known; name harm, repair, and ownership."]
        );
        assert!(report
            .suppressed
            .iter()
            .any(|candidate| candidate.candidate_id == "bag-of-chips-as-excuse"));
    }

    #[test]
    fn relation_report_checks_visibility_before_structural_dependency() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("unknown stakeholder or dependency before changing the system")
                .with_kind(FrameKind::Risk)
                .with_authority_model(AuthorityModel::Reviewer)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::AttentionLimit)
        .with_constraint(ConstraintRelation::EvidenceMissing)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::UnknownTreatedAsStructural]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["blind-spot", "load-bearing-wall"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendWithEvidenceWarning
        );
        assert!(report.suggestions[0].warnings.contains(&"missing signal"));
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1]
            .warnings
            .contains(&"structural evidence required"));
    }

    #[test]
    fn relation_report_composes_pace_constraint_with_buffer_spacing() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("maximum safe pace with reaction buffer between coupled work")
                .with_kind(FrameKind::Status)
                .with_authority_model(AuthorityModel::Steward)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Operations),
        )
        .with_target_relation(TargetRelation::PaceAdjustment)
        .with_constraint(ConstraintRelation::Coupling)
        .with_protected_value(ProtectedValue::SystemStability);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["speed-limit", "following-distance"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendSequence
        );
        assert!(report.suggestions[0].warnings.is_empty());
        assert!(report.suggestions[1].warnings.is_empty());
    }

    #[test]
    fn relation_report_sequences_stabilization_before_reentry() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "pause outside the main flow, stabilize, and then re-enter active work",
            )
            .with_kind(FrameKind::Momentum)
            .with_authority_model(AuthorityModel::Operator)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Operations),
        )
        .with_target_relation(TargetRelation::StabilizationReentry);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["shoulder-pull-off", "merge-lane"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendSequence
        );
        assert!(report.suggestions[0].warnings.is_empty());
        assert!(report.suggestions[1].warnings.is_empty());
    }

    #[test]
    fn relation_report_prefers_recovery_pause_with_restart_condition() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "deliberate recovery pause before fatigue causes mistakes with a restart condition",
            )
            .with_kind(FrameKind::Momentum)
            .with_authority_model(AuthorityModel::Steward)
            .with_risk_band(RiskBand::Low)
            .with_application_pack(ApplicationPack::Learning),
        )
        .with_target_relation(TargetRelation::RecoveryPause)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::PauseWithoutRestartCondition]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["rest-stop", "stumble-and-recover"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendBoundaryFrame
        );
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1]
            .warnings
            .contains(&"restart condition required"));
    }

    #[test]
    fn relation_report_prefers_route_adjustment_with_stable_destination() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("blocked route needs a different path while preserving destination")
                .with_kind(FrameKind::Momentum)
                .with_authority_model(AuthorityModel::Operator)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::RouteAdjustment)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::StopWithoutDestination]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["detour", "trail-marker"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendBoundaryFrame
        );
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1]
            .warnings
            .contains(&"destination required"));
    }

    #[test]
    fn relation_report_prefers_reserve_tracking_with_named_resource() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("remaining runway and capacity before committing to more work")
                .with_kind(FrameKind::Status)
                .with_authority_model(AuthorityModel::Operator)
                .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::ReserveTracking)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::ScarceResourceMissing]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["fuel-gauge", "red-yellow-green"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendBoundaryFrame
        );
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1]
            .warnings
            .contains(&"scarce resource required"));
    }

    #[test]
    fn relation_report_prefers_downshift_with_named_load() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new("reduce scope under load to regain control and stability")
                .with_kind(FrameKind::Momentum)
                .with_authority_model(AuthorityModel::Operator)
                .with_risk_band(RiskBand::Low)
                .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::PaceAdjustment)
        .with_protected_value(ProtectedValue::SystemStability)
        .with_excluded_transfers(&[TransferExclusion::LoadMissing]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["downshift", "speed-limit"]);
        assert_eq!(
            report.suggestions[0].decision,
            RelationDecision::RecommendBoundaryFrame
        );
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1].warnings.contains(&"load required"));
    }

    #[test]
    fn relation_report_demotes_stride_length_when_load_is_unnamed() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "stride length looks right but load and control boundary are not named",
            )
            .with_kind(FrameKind::Momentum)
            .with_authority_model(AuthorityModel::Operator)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Learning),
        )
        .with_target_relation(TargetRelation::PaceAdjustment)
        .with_constraint(ConstraintRelation::EvidenceMissing)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::LoadMissing]);

        let report = index.search_with_relations(&query);
        let stride = report
            .suggestions
            .iter()
            .find(|candidate| candidate.candidate.entry.id == "stride-length")
            .expect("stride-length should be present");
        assert_eq!(stride.rank_band, RankBand::Demoted);
        assert!(stride.warnings.contains(&"load required"));
        assert!(stride.warnings.contains(&"evidence boundary"));
    }

    #[test]
    fn relation_report_demotes_structural_dependency_when_unknowns_remain() {
        let index = FrameIndex::new();
        let query = RelationQuery::new(
            FrameQuery::new(
                "shaky footing and unknowns mean dependency integrity should avoid premature structural certainty",
            )
            .with_kind(FrameKind::Risk)
            .with_authority_model(AuthorityModel::Reviewer)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Product),
        )
        .with_target_relation(TargetRelation::DependencyIntegrity)
        .with_constraint(ConstraintRelation::EvidenceMissing)
        .with_protected_value(ProtectedValue::DecisionQuality)
        .with_excluded_transfers(&[TransferExclusion::UnknownTreatedAsStructural]);

        let report = index.search_with_relations(&query);
        let ids: Vec<_> = report
            .suggestions
            .iter()
            .take(2)
            .map(|candidate| candidate.candidate.entry.id)
            .collect();

        assert_eq!(ids, vec!["footing", "load-bearing-wall"]);
        assert_eq!(report.suggestions[1].rank_band, RankBand::Demoted);
        assert!(report.suggestions[1]
            .warnings
            .contains(&"structural evidence required"));
    }

    #[test]
    fn lifecycle_default_search_matches_existing_search() {
        let index = FrameIndex::new();
        let query = FrameQuery::new("two teams need turn order around constrained attention")
            .with_kind(FrameKind::Coordination)
            .with_authority_model(AuthorityModel::Peer)
            .with_risk_band(RiskBand::Medium)
            .with_application_pack(ApplicationPack::Product)
            .with_tags(&["priority"]);

        let normal = index.search(&query);
        let report = index.search_with_lifecycle(&query, &LifecycleFilter::default_search());

        assert_eq!(report.suggestions, normal);
        assert!(report.suppressed.is_empty());
        assert!(report.review_only.is_empty());
    }

    #[test]
    fn lifecycle_default_search_hides_suppressed_candidates() {
        let index = FrameIndex::new();
        let query =
            FrameQuery::new("Launch scores are strong overall, but privacy approval is missing.")
                .with_authority_model(AuthorityModel::Reviewer)
                .with_application_pack(ApplicationPack::Product)
                .with_risk_band(RiskBand::Medium);

        let report = index.search_with_lifecycle(&query, &LifecycleFilter::default_search());

        assert!(report
            .suggestions
            .iter()
            .all(|candidate| candidate.entry.id != "veto-rule"));
        assert!(report.suppressed.is_empty());
        assert_eq!(
            report.fallbacks,
            vec!["The launch is blocked until the required approval is cleared or waived."]
        );
    }

    #[test]
    fn lifecycle_explanation_mode_reports_suppressed_candidates() {
        let index = FrameIndex::new();
        let query = FrameQuery::new("A query says another team is a roadblock.")
            .with_authority_model(AuthorityModel::Owner)
            .with_application_pack(ApplicationPack::Operations)
            .with_risk_band(RiskBand::Medium);

        let report = index.search_with_lifecycle(&query, &LifecycleFilter::explanation_mode());

        assert!(report.suggestions.is_empty());
        assert_eq!(report.suppressed.len(), 1);
        let suppressed = report.suppressed[0];
        assert_eq!(suppressed.candidate_id, "team-as-roadblock");
        assert_eq!(suppressed.status, FrameStatus::AntiPattern);
        assert_eq!(suppressed.rejection_class, "people_as_obstacles");
        assert_eq!(suppressed.display_rule, DisplayRule::ExplainWhenRequested);
        assert_eq!(suppressed.safer_frame, Some("load-bearing-wall"));
        assert_eq!(
            suppressed.plain_language_fallback,
            index
                .review_entry("team-as-roadblock")
                .unwrap()
                .plain_language_fallback
        );
    }

    #[test]
    fn result_classes_mark_only_recommendation_outputs() {
        assert!(ResultClass::Suggested.is_recommendation());
        assert!(ResultClass::Alternate.is_recommendation());
        assert!(ResultClass::Fallback.is_recommendation());
        assert!(!ResultClass::Suppressed.is_recommendation());
        assert!(!ResultClass::ReviewOnly.is_recommendation());
    }

    #[test]
    fn search_filters_by_transfer_metadata() {
        let index = FrameIndex::new();
        let peer_results = index.search(
            &FrameQuery::new("two teams need turn order around constrained attention")
                .with_kind(FrameKind::Coordination)
                .with_authority_model(AuthorityModel::Peer)
                .with_risk_band(RiskBand::Medium)
                .with_application_pack(ApplicationPack::Product)
                .with_tags(&["priority"]),
        );

        assert_eq!(peer_results[0].entry.id, "four-way-stop");
        assert!(peer_results
            .iter()
            .all(|candidate| candidate.entry.authority_model == AuthorityModel::Peer));

        let protected_results = index.search(
            &FrameQuery::new("stronger actor should slow down for vulnerable users")
                .with_kind(FrameKind::Coordination)
                .with_authority_model(AuthorityModel::ProtectedParty),
        );

        assert_eq!(protected_results[0].entry.id, "crosswalk-yield");
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
            assert!(!entry.application_packs.is_empty(), "{}", entry.id);
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
            authority_model: None,
            risk_band: None,
            application_pack: None,
            tags: &[],
        });

        assert!(results.is_empty());
    }
}
