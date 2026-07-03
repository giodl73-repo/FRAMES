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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameSearchReport<'a> {
    pub suggestions: Vec<FrameCandidate<'a>>,
    pub fallbacks: Vec<&'a str>,
    pub suppressed: Vec<SuppressedCandidate<'a>>,
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

        FrameSearchReport {
            suggestions,
            fallbacks,
            suppressed,
        }
    }
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

fn suppressed_for_query(query: &FrameQuery<'_>) -> Vec<SuppressedCandidate<'static>> {
    SUPPRESSED_FIXTURES
        .iter()
        .filter(|fixture| fixture.matches(query))
        .map(|fixture| fixture.report)
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct SuppressedFixture {
    terms: &'static [&'static str],
    authority_model: Option<AuthorityModel>,
    application_pack: Option<ApplicationPack>,
    report: SuppressedCandidate<'static>,
}

impl SuppressedFixture {
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

const SUPPRESSED_FIXTURES: &[SuppressedFixture] = &[
    SuppressedFixture {
        terms: &[
            "privacy approval",
            "required approval",
            "approval is missing",
        ],
        authority_model: Some(AuthorityModel::Reviewer),
        application_pack: Some(ApplicationPack::Product),
        report: SuppressedCandidate {
            candidate_id: "veto-rule",
            candidate_name: "Veto Rule",
            status: FrameStatus::AcceptedWithCaveat,
            matched_reason: "required approval gate matches the query",
            rejection_class: "docs_catalog_not_default_search",
            violated_boundary: "lifecycle status",
            safer_frame: None,
            plain_language_fallback:
                "The launch is blocked until the required approval is cleared or waived.",
            source_docs: &[
                "docs/theory/accepted-catalog-review-veto-rule.md",
                "docs/frame-catalog.md",
            ],
            display_rule: DisplayRule::SuppressByDefault,
        },
    },
    SuppressedFixture {
        terms: &["vetoed", "veto"],
        authority_model: None,
        application_pack: Some(ApplicationPack::Leadership),
        report: SuppressedCandidate {
            candidate_id: "veto-rule",
            candidate_name: "Veto Rule",
            status: FrameStatus::AcceptedWithCaveat,
            matched_reason: "veto wording and required-gate relation match the query",
            rejection_class: "false_authority_transfer",
            violated_boundary: "authority and evidence",
            safer_frame: None,
            plain_language_fallback: "This is an unresolved preference or decision-rights issue.",
            source_docs: &["docs/theory/frame-antipattern-application-veto-rule.md"],
            display_rule: DisplayRule::ExplainWhenRequested,
        },
    },
    SuppressedFixture {
        terms: &["roadblock"],
        authority_model: Some(AuthorityModel::Owner),
        application_pack: Some(ApplicationPack::Operations),
        report: SuppressedCandidate {
            candidate_id: "team-as-roadblock",
            candidate_name: "Team as roadblock",
            status: FrameStatus::AntiPattern,
            matched_reason: "roadblock wording directly matches the query",
            rejection_class: "people_as_obstacles",
            violated_boundary: "human safety and ownership",
            safer_frame: Some("load-bearing-wall"),
            plain_language_fallback:
                "The dependency is blocked by unresolved ownership or a decision path.",
            source_docs: &[
                "docs/theory/frame-antipattern-taxonomy.md",
                "docs/theory/related-frame-application-starter.md",
            ],
            display_rule: DisplayRule::ExplainWhenRequested,
        },
    },
    SuppressedFixture {
        terms: &["veto rule", "requirement authority"],
        authority_model: None,
        application_pack: Some(ApplicationPack::AiAgent),
        report: SuppressedCandidate {
            candidate_id: "veto-rule",
            candidate_name: "Veto Rule",
            status: FrameStatus::AcceptedWithCaveat,
            matched_reason: "required-gate relation and veto wording match the query",
            rejection_class: "rejected_near_miss",
            violated_boundary: "authority and evidence",
            safer_frame: None,
            plain_language_fallback:
                "Name the decision owner, tradeoff, and evidence before treating this as blocking.",
            source_docs: &[
                "docs/theory/related-frame-application-starter.md",
                "docs/theory/frame-antipattern-application-veto-rule.md",
            ],
            display_rule: DisplayRule::ExplainWhenRequested,
        },
    },
    SuppressedFixture {
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
    },
    SuppressedFixture {
        terms: &["bag-of-chips", "facts establish harm"],
        authority_model: Some(AuthorityModel::Peer),
        application_pack: Some(ApplicationPack::Leadership),
        report: SuppressedCandidate {
            candidate_id: "bag-of-chips-as-excuse",
            candidate_name: "Bag-of-chips story as excuse",
            status: FrameStatus::AntiPattern,
            matched_reason: "story source and conflict-empathy job match the query",
            rejection_class: "empathy_eraser",
            violated_boundary: "repair and accountability",
            safer_frame: None,
            plain_language_fallback: "Facts are now known; name harm, repair, and ownership.",
            source_docs: &["docs/theory/frame-antipattern-taxonomy.md"],
            display_rule: DisplayRule::ExplainWhenRequested,
        },
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
        related: &["rest-stop", "downshift"],
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

        assert_eq!(index.by_status(FrameStatus::Accepted).len(), 15);
        assert_eq!(index.by_claim_strength(ClaimStrength::Heuristic).len(), 15);
        assert_eq!(index.by_risk_band(RiskBand::Low).len(), 4);
        assert_eq!(index.by_authority_model(AuthorityModel::Peer).len(), 2);
        assert_eq!(
            index.with_application_pack(ApplicationPack::AiAgent).len(),
            7
        );
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
