//! One controlled version-match/freshness vocabulary plus stale-example and
//! broken-link findings reused across the claimed M5 docs lanes.
//!
//! Earlier rows each minted their own version-match and freshness chips. This
//! lane freezes a single [`DocsVersionFreshnessState`] vocabulary — `exact`,
//! `nearby`, `project_specific`, `mirrored`, `cached`, `stale`, `policy_blocked`,
//! and `browser_handoff_required` — that docs result rows, symbol-linked
//! reference cards, docs pages, AI citation chips, onboarding/glossary surfaces,
//! and support exports all project verbatim. The eight states stay *distinct*:
//! `browser_handoff_required`, `cached`, `mirrored`, and `project_specific` must
//! never collapse into one generic info badge, because the distinction is part
//! of the product truth. Each state carries its own confidence treatment
//! ([`DocsVersionFreshnessConfidence`]) so cached or nearby-version documentation
//! never renders with the same confidence as exact current documentation.
//!
//! On top of the state vocabulary the lane adds actionable
//! [`DocsVersionFreshnessFinding`] review items — stale-example and broken-link
//! findings that compare a doc's code blocks, commands, API references, config
//! paths, or links against the current graph/pack metadata. Every finding keeps
//! a stable identity and preserves its suppress / compare / open-current-source
//! actions across every consumer surface and the support export.
//!
//! [`DocsVersionFreshnessPacket::materialize`] computes the validation findings
//! and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so a card that shares exact confidence with a
//! cached/nearby state, a version mismatch that hides the active or viewed
//! version, or a finding that drops its actions automatically narrows or blocks
//! before it reaches a consumer surface. The packet is an inspectable,
//! serde-serializable truth packet: it carries no raw document bodies, raw URLs,
//! raw provider payloads, or credentials — only metadata, state truth, findings,
//! and contract references.
//!
//! The boundary schema is
//! [`schemas/docs/add-version-freshness-vocabulary-and-stale-example-broken-link-findings.schema.json`](../../../../schemas/docs/add-version-freshness-vocabulary-and-stale-example-broken-link-findings.schema.json).
//! The contract doc is
//! [`docs/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings.md`](../../../../docs/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/`](../../../../fixtures/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsVersionFreshnessPacket`].
pub const DOCS_VERSION_FRESHNESS_RECORD_KIND: &str = "docs_version_freshness_findings_packet";

/// Stable record-kind tag carried by [`DocsVersionFreshnessSupportExport`].
pub const DOCS_VERSION_FRESHNESS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_version_freshness_findings_support_export";

/// Schema version for docs version-freshness records.
pub const DOCS_VERSION_FRESHNESS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_VERSION_FRESHNESS_SCHEMA_REF: &str =
    "schemas/docs/add-version-freshness-vocabulary-and-stale-example-broken-link-findings.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_VERSION_FRESHNESS_DOC_REF: &str =
    "docs/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings.md";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_VERSION_FRESHNESS_ARTIFACT_REF: &str =
    "artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_VERSION_FRESHNESS_SUMMARY_REF: &str =
    "artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_VERSION_FRESHNESS_FIXTURE_DIR: &str =
    "fixtures/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings";

/// The controlled version-match / freshness vocabulary.
///
/// One stable state per claimed docs answer. The states are deliberately kept
/// distinct: `browser_handoff_required`, `cached`, `mirrored`, and
/// `project_specific` must never collapse into one generic info badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessState {
    /// Source exactly matches the active code/package version; current
    /// authoritative guidance.
    Exact,
    /// A near-version match (compatible drift); correctness could change, so it
    /// must not read as exact-current.
    Nearby,
    /// Workspace/project documentation — not vendor docs; current to the project
    /// but scoped to it.
    ProjectSpecific,
    /// Served from a pinned, signed mirror of upstream docs.
    Mirrored,
    /// Served from a local cache and not verified live; freshness is unverified.
    Cached,
    /// Known stale; must not claim current authority.
    Stale,
    /// Blocked by policy; the answer is not rendered inline and a reason is named.
    PolicyBlocked,
    /// The answer requires handing off to a browser/provider console; it is not
    /// answered inline and a reason is named.
    BrowserHandoffRequired,
}

impl DocsVersionFreshnessState {
    /// Every state in the controlled vocabulary, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Exact,
        Self::Nearby,
        Self::ProjectSpecific,
        Self::Mirrored,
        Self::Cached,
        Self::Stale,
        Self::PolicyBlocked,
        Self::BrowserHandoffRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Nearby => "nearby",
            Self::ProjectSpecific => "project_specific",
            Self::Mirrored => "mirrored",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::BrowserHandoffRequired => "browser_handoff_required",
        }
    }

    /// Confidence treatment a card in this state must declare. Only [`Self::Exact`]
    /// maps to the current-exact treatment, so cached or nearby-version
    /// documentation never shares exact's confidence.
    pub const fn confidence_class(self) -> DocsVersionFreshnessConfidence {
        match self {
            Self::Exact => DocsVersionFreshnessConfidence::CurrentExact,
            Self::Nearby => DocsVersionFreshnessConfidence::QualifiedNearby,
            Self::ProjectSpecific => DocsVersionFreshnessConfidence::ProjectScoped,
            Self::Mirrored => DocsVersionFreshnessConfidence::MirroredVerified,
            Self::Cached => DocsVersionFreshnessConfidence::CachedUnverified,
            Self::Stale => DocsVersionFreshnessConfidence::NotCurrent,
            Self::PolicyBlocked | Self::BrowserHandoffRequired => {
                DocsVersionFreshnessConfidence::InlineUnavailable
            }
        }
    }

    /// Whether a card in this state must disclose both the active code/package
    /// version and the viewed docs version (the version-mismatch states).
    pub const fn requires_version_disclosure(self) -> bool {
        matches!(
            self,
            Self::Nearby | Self::Mirrored | Self::Cached | Self::Stale
        )
    }

    /// Whether a card in this state must name a reason (the not-rendered-inline
    /// states).
    pub const fn requires_state_reason(self) -> bool {
        matches!(self, Self::PolicyBlocked | Self::BrowserHandoffRequired)
    }

    /// Whether the answer is rendered inline rather than deferred to a browser
    /// handoff or blocked by policy.
    pub const fn answered_inline(self) -> bool {
        !self.requires_state_reason()
    }
}

/// Confidence treatment derived from a [`DocsVersionFreshnessState`].
///
/// Kept distinct from the state badge so a surface can both render the exact
/// state token *and* a confidence tier; only [`Self::CurrentExact`] is the
/// full-confidence current treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessConfidence {
    /// Exact, current, authoritative.
    CurrentExact,
    /// Near-version match; usable but qualified, version visible.
    QualifiedNearby,
    /// Project-scoped truth; authoritative for the workspace, not vendor docs.
    ProjectScoped,
    /// Pinned signed mirror; verified but not the live source.
    MirroredVerified,
    /// Local cache; usable but explicitly unverified.
    CachedUnverified,
    /// Known not current.
    NotCurrent,
    /// Not answered inline (policy-blocked or browser-handoff-required).
    InlineUnavailable,
}

impl DocsVersionFreshnessConfidence {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentExact => "current_exact",
            Self::QualifiedNearby => "qualified_nearby",
            Self::ProjectScoped => "project_scoped",
            Self::MirroredVerified => "mirrored_verified",
            Self::CachedUnverified => "cached_unverified",
            Self::NotCurrent => "not_current",
            Self::InlineUnavailable => "inline_unavailable",
        }
    }

    /// Whether this treatment is the full-confidence current-exact one.
    pub const fn is_current_exact(self) -> bool {
        matches!(self, Self::CurrentExact)
    }
}

/// Both versions disclosed when the viewed docs version differs from the active
/// code/package version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessDisclosure {
    /// The active code/package version reference.
    pub active_version_ref: String,
    /// The viewed docs version reference.
    pub viewed_version_ref: String,
    /// Whether the difference changes API or workflow truth (acceptance #2 makes
    /// the disclosure mandatory whenever this is true).
    pub difference_changes_api_or_workflow: bool,
    /// Optional human-readable note on the difference (no raw bodies).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub difference_summary: Option<String>,
}

impl DocsVersionFreshnessDisclosure {
    /// Whether both version references are present.
    pub fn is_complete(&self) -> bool {
        !self.active_version_ref.trim().is_empty() && !self.viewed_version_ref.trim().is_empty()
    }
}

/// What a finding (or the subject of a comparison) concerns inside a doc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessSubjectKind {
    /// A fenced code block / example.
    CodeBlock,
    /// A shell or tool command.
    Command,
    /// An API symbol reference.
    ApiReference,
    /// A configuration path / key.
    ConfigPath,
    /// A hyperlink / anchor.
    Link,
}

impl DocsVersionFreshnessSubjectKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeBlock => "code_block",
            Self::Command => "command",
            Self::ApiReference => "api_reference",
            Self::ConfigPath => "config_path",
            Self::Link => "link",
        }
    }

    /// Whether this subject is a hyperlink/anchor.
    pub const fn is_link(self) -> bool {
        matches!(self, Self::Link)
    }
}

/// Class of a stale-example or broken-link finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessFindingClass {
    /// An example is stale for the active build/pack.
    StaleExample,
    /// A referenced link/anchor is broken.
    BrokenLink,
    /// A nearer-version example exists for the active build.
    NearbyVersionExample,
    /// A referenced API no longer exists in the current graph.
    RemovedApiReference,
    /// A referenced config path moved or changed in the current pack metadata.
    ChangedConfigPath,
    /// A documented command's syntax changed.
    CommandSyntaxChanged,
}

impl DocsVersionFreshnessFindingClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleExample => "stale_example",
            Self::BrokenLink => "broken_link",
            Self::NearbyVersionExample => "nearby_version_example",
            Self::RemovedApiReference => "removed_api_reference",
            Self::ChangedConfigPath => "changed_config_path",
            Self::CommandSyntaxChanged => "command_syntax_changed",
        }
    }

    /// Whether this finding class describes a broken link.
    pub const fn is_broken_link(self) -> bool {
        matches!(self, Self::BrokenLink)
    }

    /// Whether `subject` is a consistent subject for this finding class. A
    /// broken-link finding must be about a [`DocsVersionFreshnessSubjectKind::Link`];
    /// every other class must be about a non-link subject.
    pub const fn allows_subject(self, subject: DocsVersionFreshnessSubjectKind) -> bool {
        if self.is_broken_link() {
            subject.is_link()
        } else {
            !subject.is_link()
        }
    }
}

/// Severity of a stale-example / broken-link finding, feeding promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessFindingSeverity {
    /// Advisory only; the packet stays stable.
    Advisory,
    /// Narrows the packet below stable but it stays valid.
    Narrowing,
    /// Blocks the stable claim.
    Blocking,
}

impl DocsVersionFreshnessFindingSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Narrowing => "narrowing",
            Self::Blocking => "blocking",
        }
    }
}

/// Suppression state of a finding's actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessSuppressionState {
    /// The finding is active and shown.
    Active,
    /// A reviewer suppressed the finding; a reason must be named.
    SuppressedByReviewer,
    /// Policy suppressed the finding; a reason must be named.
    SuppressedByPolicy,
}

impl DocsVersionFreshnessSuppressionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::SuppressedByReviewer => "suppressed_by_reviewer",
            Self::SuppressedByPolicy => "suppressed_by_policy",
        }
    }

    /// Whether the finding is suppressed (no longer surfaced as active).
    pub const fn is_suppressed(self) -> bool {
        !matches!(self, Self::Active)
    }

    /// Whether this state must name a suppression reason.
    pub const fn requires_reason(self) -> bool {
        self.is_suppressed()
    }
}

/// The suppress / compare / open-current-source actions a finding preserves on
/// every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessFindingActions {
    /// Suppression state of the finding.
    pub suppression_state: DocsVersionFreshnessSuppressionState,
    /// Disclosed reason when the finding is suppressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_reason: Option<String>,
    /// Ref to a compare/diff view of the documented vs current value.
    pub compare_ref: String,
    /// Ref to open the current authoritative source for the subject.
    pub open_current_source_ref: String,
}

impl DocsVersionFreshnessFindingActions {
    /// Whether the compare and open-current-source actions are both preserved.
    pub fn preserves_actions(&self) -> bool {
        !self.compare_ref.trim().is_empty() && !self.open_current_source_ref.trim().is_empty()
    }

    /// Whether a suppressed finding names its reason.
    pub fn suppression_disclosed(&self) -> bool {
        if !self.suppression_state.requires_reason() {
            return true;
        }
        self.suppression_reason
            .as_deref()
            .map(|reason| !reason.trim().is_empty())
            .unwrap_or(false)
    }
}

/// An actionable stale-example or broken-link review item with stable identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessFinding {
    /// Stable finding id (carried verbatim across surfaces and exports).
    pub finding_id: String,
    /// Finding class.
    pub finding_class: DocsVersionFreshnessFindingClass,
    /// The doc subject the finding concerns.
    pub subject_kind: DocsVersionFreshnessSubjectKind,
    /// Finding severity, feeding promotion.
    pub severity: DocsVersionFreshnessFindingSeverity,
    /// The card this finding annotates.
    pub card_id_ref: String,
    /// Docs-node ref containing the subject (no raw body).
    pub doc_node_ref: String,
    /// Ref to the value observed in the doc (no raw body).
    pub observed_ref: String,
    /// Ref to the current graph/pack metadata the subject was compared against.
    pub compared_against_ref: String,
    /// Ref to the current correct value, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_ref: Option<String>,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// Suppress / compare / open-current-source actions.
    pub actions: DocsVersionFreshnessFindingActions,
}

impl DocsVersionFreshnessFinding {
    /// Whether every required identity/comparison field is present.
    pub fn is_well_formed(&self) -> bool {
        !self.finding_id.trim().is_empty()
            && !self.card_id_ref.trim().is_empty()
            && !self.doc_node_ref.trim().is_empty()
            && !self.observed_ref.trim().is_empty()
            && !self.compared_against_ref.trim().is_empty()
            && !self.summary.trim().is_empty()
    }

    /// Whether the subject is consistent with the finding class.
    pub fn subject_consistent(&self) -> bool {
        self.finding_class.allows_subject(self.subject_kind)
    }

    /// Severity after applying suppression: a suppressed finding no longer
    /// narrows or blocks promotion.
    pub fn effective_severity(&self) -> DocsVersionFreshnessFindingSeverity {
        if self.actions.suppression_state.is_suppressed() {
            DocsVersionFreshnessFindingSeverity::Advisory
        } else {
            self.severity
        }
    }
}

/// One claimed docs answer/card carrying its resolved version/freshness state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessCard {
    /// Stable card id.
    pub card_id: String,
    /// Docs-node ref the card resolves to (no raw body).
    pub doc_node_ref: String,
    /// Human-readable label.
    pub display_label: String,
    /// Resolved version/freshness state — the badge.
    pub state: DocsVersionFreshnessState,
    /// Confidence treatment; must equal `state.confidence_class()`.
    pub confidence: DocsVersionFreshnessConfidence,
    /// Both versions, present whenever the state requires version disclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_disclosure: Option<DocsVersionFreshnessDisclosure>,
    /// Reason a not-rendered-inline state defers; present for `policy_blocked`
    /// and `browser_handoff_required`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    /// Ref to open the current authoritative source (or browser handoff target).
    pub open_current_source_ref: String,
    /// True when raw URLs, raw bodies, secrets, and provider payloads are
    /// excluded from this card.
    pub raw_boundary_material_excluded: bool,
}

impl DocsVersionFreshnessCard {
    /// Whether the card's identity fields are present.
    pub fn is_well_formed(&self) -> bool {
        !self.card_id.trim().is_empty()
            && !self.doc_node_ref.trim().is_empty()
            && !self.display_label.trim().is_empty()
            && !self.open_current_source_ref.trim().is_empty()
    }

    /// Whether the declared confidence matches the state's confidence class.
    pub fn confidence_consistent(&self) -> bool {
        self.confidence == self.state.confidence_class()
    }

    /// Whether the version disclosure is present and complete when the state
    /// requires it.
    pub fn version_disclosed_when_required(&self) -> bool {
        if !self.state.requires_version_disclosure() {
            return true;
        }
        self.version_disclosure
            .as_ref()
            .map(DocsVersionFreshnessDisclosure::is_complete)
            .unwrap_or(false)
    }

    /// Whether a not-rendered-inline state names its reason.
    pub fn state_reason_when_required(&self) -> bool {
        if !self.state.requires_state_reason() {
            return true;
        }
        self.state_reason
            .as_deref()
            .map(|reason| !reason.trim().is_empty())
            .unwrap_or(false)
    }
}

/// A claimed M5 consumer surface that reads the same version/freshness truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessConsumerSurface {
    /// Docs/search result row.
    ResultRow,
    /// Symbol-linked reference card.
    SymbolReferenceCard,
    /// Rendered docs page / hover-peek.
    DocsPage,
    /// AI citation chip.
    AiCitationChip,
    /// Onboarding / glossary surface.
    OnboardingGlossary,
    /// Support export bundle.
    SupportExport,
}

impl DocsVersionFreshnessConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::ResultRow,
        Self::SymbolReferenceCard,
        Self::DocsPage,
        Self::AiCitationChip,
        Self::OnboardingGlossary,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultRow => "result_row",
            Self::SymbolReferenceCard => "symbol_reference_card",
            Self::DocsPage => "docs_page",
            Self::AiCitationChip => "ai_citation_chip",
            Self::OnboardingGlossary => "onboarding_glossary",
            Self::SupportExport => "support_export",
        }
    }
}

/// How a consumer surface projects the version/freshness packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessConsumerProjection {
    /// Surface that consumes the packet.
    pub surface: DocsVersionFreshnessConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface renders the exact state badge.
    pub preserves_state_badge: bool,
    /// Whether the surface keeps the eight states distinct rather than collapsing
    /// `cached` / `mirrored` / `project_specific` / `browser_handoff_required`
    /// into one generic info badge.
    pub preserves_state_distinctions: bool,
    /// Whether the surface keeps the per-state confidence treatment so cached or
    /// nearby never renders as exact-current.
    pub preserves_confidence_treatment: bool,
    /// Whether the surface shows the active + viewed version when required.
    pub preserves_version_disclosure: bool,
    /// Whether the surface shows the stale-example / broken-link findings.
    pub preserves_findings: bool,
    /// Whether the surface keeps the suppress / compare / open-current-source
    /// finding actions.
    pub preserves_finding_actions: bool,
    /// Whether raw private material is excluded from the projection.
    pub raw_private_material_excluded: bool,
}

impl DocsVersionFreshnessConsumerProjection {
    /// Whether the supporting flags (everything except the state-distinction
    /// guardrail and the packet-id match) are preserved.
    fn preserves_supporting_flags(&self) -> bool {
        !self.projection_ref.trim().is_empty()
            && self.preserves_state_badge
            && self.preserves_confidence_treatment
            && self.preserves_version_disclosure
            && self.preserves_findings
            && self.preserves_finding_actions
            && self.raw_private_material_excluded
    }
}

/// Promotion state derived from the packet's findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessPromotionState {
    /// Packet certifies the stable claim.
    Stable,
    /// Packet narrowed below stable but stays valid.
    NarrowedBelowStable,
    /// Packet blocks stable publication.
    BlocksStable,
}

impl DocsVersionFreshnessPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessValidationSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`DocsVersionFreshnessPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsVersionFreshnessValidationKind {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Packet declared no cards.
    MissingCards,
    /// A card is incomplete.
    CardIncomplete,
    /// A card duplicates an existing card id.
    DuplicateCardId,
    /// A card's confidence collapses into a treatment its state does not allow
    /// (e.g. a cached/nearby card claiming exact-current confidence).
    CardConfidenceCollapsed,
    /// A version-mismatch card hides the active or viewed version.
    VersionDisclosureMissing,
    /// A not-rendered-inline state drops its reason.
    StateReasonMissing,
    /// The cards do not exercise every state in the controlled vocabulary.
    VocabularyCoverageMissing,
    /// A finding is incomplete.
    FindingIncomplete,
    /// A finding's subject disagrees with its class.
    FindingSubjectClassMismatch,
    /// A finding duplicates an existing finding id.
    DuplicateFindingId,
    /// A finding references a card absent from the packet.
    FindingOrphan,
    /// A finding drops its compare or open-current-source action.
    FindingActionsMissing,
    /// A suppressed finding drops its reason.
    FindingSuppressionReasonMissing,
    /// A required consumer surface has no projection.
    RequiredSurfaceMissing,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A consumer projection collapses the distinct state badges.
    StateDistinctionCollapsed,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// Raw boundary material is present in the export.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DocsVersionFreshnessValidationKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingCards => "missing_cards",
            Self::CardIncomplete => "card_incomplete",
            Self::DuplicateCardId => "duplicate_card_id",
            Self::CardConfidenceCollapsed => "card_confidence_collapsed",
            Self::VersionDisclosureMissing => "version_disclosure_missing",
            Self::StateReasonMissing => "state_reason_missing",
            Self::VocabularyCoverageMissing => "vocabulary_coverage_missing",
            Self::FindingIncomplete => "finding_incomplete",
            Self::FindingSubjectClassMismatch => "finding_subject_class_mismatch",
            Self::DuplicateFindingId => "duplicate_finding_id",
            Self::FindingOrphan => "finding_orphan",
            Self::FindingActionsMissing => "finding_actions_missing",
            Self::FindingSuppressionReasonMissing => "finding_suppression_reason_missing",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::StateDistinctionCollapsed => "state_distinction_collapsed",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the version-freshness validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DocsVersionFreshnessValidationKind,
    /// Finding severity.
    pub severity: DocsVersionFreshnessValidationSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DocsVersionFreshnessValidationFinding {
    fn blocker(
        finding_kind: DocsVersionFreshnessValidationKind,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: DocsVersionFreshnessValidationSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`DocsVersionFreshnessPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Claimed docs answer cards.
    pub cards: Vec<DocsVersionFreshnessCard>,
    /// Stale-example / broken-link findings.
    pub findings: Vec<DocsVersionFreshnessFinding>,
    /// Per-surface projections.
    pub consumer_projections: Vec<DocsVersionFreshnessConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

/// Export-safe docs version/freshness packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessPacket {
    /// Record kind; must equal [`DOCS_VERSION_FRESHNESS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_VERSION_FRESHNESS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Claimed docs answer cards.
    pub cards: Vec<DocsVersionFreshnessCard>,
    /// Stale-example / broken-link findings.
    pub findings: Vec<DocsVersionFreshnessFinding>,
    /// Per-surface projections.
    pub consumer_projections: Vec<DocsVersionFreshnessConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Derived promotion state.
    pub promotion_state: DocsVersionFreshnessPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<DocsVersionFreshnessValidationFinding>,
}

impl DocsVersionFreshnessPacket {
    /// Materializes the packet and records its derived findings and promotion
    /// state.
    pub fn materialize(input: DocsVersionFreshnessPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_VERSION_FRESHNESS_RECORD_KIND.to_owned(),
            schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generated_at: input.generated_at,
            cards: input.cards,
            findings: input.findings,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            promotion_state: DocsVersionFreshnessPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for(&findings, &packet.findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet's invariants, including the stored promotion
    /// state.
    pub fn validate(&self) -> Vec<DocsVersionFreshnessValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker validation findings exist.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DocsVersionFreshnessValidationSeverity::Blocker)
    }

    /// Returns true when the packet certifies the clean stable claim.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == DocsVersionFreshnessPromotionState::Stable
            && self.validate().is_empty()
    }

    /// Returns true when at least one projection preserves this packet for
    /// `surface`.
    pub fn has_projection_for(&self, surface: DocsVersionFreshnessConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.surface == surface
                && projection.packet_id_ref == self.packet_id
                && projection.preserves_state_distinctions
                && projection.preserves_supporting_flags()
        })
    }

    /// Returns the unique state tokens carried across the cards.
    pub fn state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for card in &self.cards {
            set.insert(card.state);
        }
        set.into_iter()
            .map(DocsVersionFreshnessState::as_str)
            .collect()
    }

    /// Wraps the packet in an export-safe support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DocsVersionFreshnessSupportExport {
        DocsVersionFreshnessSupportExport {
            record_kind: DOCS_VERSION_FRESHNESS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_VERSION_FRESHNESS_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs version freshness packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs Version/Freshness State And Stale-Example Findings\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} validation findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Cards: {} / Findings: {} / Surfaces: {}\n",
            self.cards.len(),
            self.findings.len(),
            self.consumer_projections.len()
        ));
        out.push_str("\n## Cards\n\n");
        for card in &self.cards {
            out.push_str(&format!(
                "- **{}** (`{}`): state `{}` / confidence `{}`\n",
                card.display_label,
                card.card_id,
                card.state.as_str(),
                card.confidence.as_str(),
            ));
            if let Some(disclosure) = &card.version_disclosure {
                out.push_str(&format!(
                    "   - active `{}` vs viewed `{}`\n",
                    disclosure.active_version_ref, disclosure.viewed_version_ref
                ));
            }
        }
        if !self.findings.is_empty() {
            out.push_str("\n## Findings\n\n");
            for finding in &self.findings {
                out.push_str(&format!(
                    "- `{}` [{}/{}/{}] on `{}`: {}\n",
                    finding.finding_id,
                    finding.finding_class.as_str(),
                    finding.subject_kind.as_str(),
                    finding.severity.as_str(),
                    finding.card_id_ref,
                    finding.summary,
                ));
            }
        }
        out
    }

    fn derived_findings(
        &self,
        check_promotion: bool,
    ) -> Vec<DocsVersionFreshnessValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != DOCS_VERSION_FRESHNESS_RECORD_KIND {
            findings.push(DocsVersionFreshnessValidationFinding::blocker(
                DocsVersionFreshnessValidationKind::WrongRecordKind,
                "record kind does not match the docs version-freshness contract",
            ));
        }
        if self.schema_version != DOCS_VERSION_FRESHNESS_SCHEMA_VERSION {
            findings.push(DocsVersionFreshnessValidationFinding::blocker(
                DocsVersionFreshnessValidationKind::WrongSchemaVersion,
                "schema version does not match the docs version-freshness contract",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            findings.push(DocsVersionFreshnessValidationFinding::blocker(
                DocsVersionFreshnessValidationKind::MissingPacketIdentity,
                "packet identity is incomplete",
            ));
        }

        self.validate_source_contracts(&mut findings);
        self.validate_cards(&mut findings);
        self.validate_vocabulary_coverage(&mut findings);
        self.validate_findings(&mut findings);
        self.validate_projections(&mut findings);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("docs version freshness packet serializes"),
        ) {
            findings.push(DocsVersionFreshnessValidationFinding::blocker(
                DocsVersionFreshnessValidationKind::RawBoundaryMaterialPresent,
                "export contains forbidden raw boundary material",
            ));
        }

        if check_promotion {
            let derived = promotion_state_for(&findings, &self.findings);
            if self.promotion_state != derived {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::PromotionStateMismatch,
                    "stored promotion state disagrees with derived findings",
                ));
            }
        }

        findings
    }

    fn validate_source_contracts(&self, findings: &mut Vec<DocsVersionFreshnessValidationFinding>) {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(DOCS_VERSION_FRESHNESS_SCHEMA_REF)
            || !refs.contains(DOCS_VERSION_FRESHNESS_DOC_REF)
        {
            findings.push(DocsVersionFreshnessValidationFinding::blocker(
                DocsVersionFreshnessValidationKind::MissingSourceContracts,
                "source contract refs omit the schema or contract doc",
            ));
        }
    }

    fn validate_cards(&self, findings: &mut Vec<DocsVersionFreshnessValidationFinding>) {
        if self.cards.is_empty() {
            findings.push(DocsVersionFreshnessValidationFinding::blocker(
                DocsVersionFreshnessValidationKind::MissingCards,
                "packet must declare at least one card",
            ));
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for card in &self.cards {
            if !card.is_well_formed() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::CardIncomplete,
                    format!("card {} drops a required identity field", card.card_id),
                ));
            }
            if !card.card_id.trim().is_empty() && !seen.insert(card.card_id.as_str()) {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::DuplicateCardId,
                    format!("duplicate card id {}", card.card_id),
                ));
            }
            if !card.confidence_consistent() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::CardConfidenceCollapsed,
                    format!(
                        "card {} state {} declares confidence {} but must declare {}",
                        card.card_id,
                        card.state.as_str(),
                        card.confidence.as_str(),
                        card.state.confidence_class().as_str()
                    ),
                ));
            }
            if !card.version_disclosed_when_required() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::VersionDisclosureMissing,
                    format!(
                        "card {} state {} must disclose both the active and viewed version",
                        card.card_id,
                        card.state.as_str()
                    ),
                ));
            }
            if !card.state_reason_when_required() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::StateReasonMissing,
                    format!(
                        "card {} state {} must name a reason",
                        card.card_id,
                        card.state.as_str()
                    ),
                ));
            }
            if !card.raw_boundary_material_excluded {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::RawBoundaryMaterialPresent,
                    format!("card {} retains raw boundary material", card.card_id),
                ));
            }
        }
    }

    fn validate_vocabulary_coverage(
        &self,
        findings: &mut Vec<DocsVersionFreshnessValidationFinding>,
    ) {
        let present: BTreeSet<DocsVersionFreshnessState> =
            self.cards.iter().map(|card| card.state).collect();
        for state in DocsVersionFreshnessState::ALL {
            if !present.contains(&state) {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::VocabularyCoverageMissing,
                    format!(
                        "no card exercises the {} state of the controlled vocabulary",
                        state.as_str()
                    ),
                ));
                break;
            }
        }
    }

    fn validate_findings(&self, findings: &mut Vec<DocsVersionFreshnessValidationFinding>) {
        let card_ids: BTreeSet<&str> = self
            .cards
            .iter()
            .map(|card| card.card_id.as_str())
            .collect();

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for finding in &self.findings {
            if !finding.is_well_formed() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::FindingIncomplete,
                    format!("finding {} drops a required field", finding.finding_id),
                ));
            }
            if !finding.subject_consistent() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::FindingSubjectClassMismatch,
                    format!(
                        "finding {} class {} disagrees with subject {}",
                        finding.finding_id,
                        finding.finding_class.as_str(),
                        finding.subject_kind.as_str()
                    ),
                ));
            }
            if !finding.finding_id.trim().is_empty() && !seen.insert(finding.finding_id.as_str()) {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::DuplicateFindingId,
                    format!("duplicate finding id {}", finding.finding_id),
                ));
            }
            if !finding.card_id_ref.trim().is_empty()
                && !card_ids.contains(finding.card_id_ref.as_str())
            {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::FindingOrphan,
                    format!(
                        "finding {} references unknown card {}",
                        finding.finding_id, finding.card_id_ref
                    ),
                ));
            }
            if !finding.actions.preserves_actions() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::FindingActionsMissing,
                    format!(
                        "finding {} drops its compare or open-current-source action",
                        finding.finding_id
                    ),
                ));
            }
            if !finding.actions.suppression_disclosed() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::FindingSuppressionReasonMissing,
                    format!(
                        "finding {} is suppressed without a disclosed reason",
                        finding.finding_id
                    ),
                ));
            }
        }
    }

    fn validate_projections(&self, findings: &mut Vec<DocsVersionFreshnessValidationFinding>) {
        let present: BTreeSet<DocsVersionFreshnessConsumerSurface> = self
            .consumer_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for required in DocsVersionFreshnessConsumerSurface::REQUIRED {
            if !present.contains(&required) {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::RequiredSurfaceMissing,
                    format!(
                        "no projection reuses the packet on the {} surface",
                        required.as_str()
                    ),
                ));
                break;
            }
        }

        for projection in &self.consumer_projections {
            if projection.packet_id_ref != self.packet_id {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::ConsumerProjectionPacketIdMismatch,
                    format!(
                        "surface {} references packet {}",
                        projection.surface.as_str(),
                        projection.packet_id_ref
                    ),
                ));
            }
            if !projection.preserves_state_distinctions {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::StateDistinctionCollapsed,
                    format!(
                        "surface {} collapses the distinct state badges into one generic badge",
                        projection.surface.as_str()
                    ),
                ));
            }
            if !projection.preserves_supporting_flags() {
                findings.push(DocsVersionFreshnessValidationFinding::blocker(
                    DocsVersionFreshnessValidationKind::ConsumerProjectionDrift,
                    format!(
                        "surface {} drops a required preservation flag",
                        projection.surface.as_str()
                    ),
                ));
            }
        }
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsVersionFreshnessSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Exported packet id.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Exact packet preserved by the export.
    pub export_packet: DocsVersionFreshnessPacket,
}

impl DocsVersionFreshnessSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_VERSION_FRESHNESS_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_VERSION_FRESHNESS_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in version-freshness export.
#[derive(Debug)]
pub enum DocsVersionFreshnessArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export's packet failed validation.
    Validation(Vec<DocsVersionFreshnessValidationFinding>),
    /// Support export wrapper is not export-safe.
    NotExportSafe,
}

impl fmt::Display for DocsVersionFreshnessArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs version freshness export parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs version freshness export failed validation: {tokens}"
                )
            }
            Self::NotExportSafe => {
                write!(
                    formatter,
                    "docs version freshness export wrapper is not export-safe"
                )
            }
        }
    }
}

impl Error for DocsVersionFreshnessArtifactError {}

/// Returns the seeded stable version-freshness packet input.
pub fn seeded_stable_docs_version_freshness_input() -> DocsVersionFreshnessPacketInput {
    seed::seeded_input()
}

/// Materializes the checked-in stable version-freshness packet.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_docs_version_freshness_packet(
) -> Result<DocsVersionFreshnessPacket, DocsVersionFreshnessArtifactError> {
    let packet =
        DocsVersionFreshnessPacket::materialize(seeded_stable_docs_version_freshness_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DocsVersionFreshnessArtifactError::Validation(findings))
    }
}

/// Reads and validates the checked-in stable support export.
///
/// # Errors
///
/// Returns an error when the checked artifact fails to parse, is not
/// export-safe, or its packet fails validation.
pub fn current_stable_docs_version_freshness_export(
) -> Result<DocsVersionFreshnessSupportExport, DocsVersionFreshnessArtifactError> {
    let export: DocsVersionFreshnessSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/support_export.json"
    )))
    .map_err(DocsVersionFreshnessArtifactError::SupportExport)?;
    let findings = export.export_packet.validate();
    if !findings.is_empty() {
        return Err(DocsVersionFreshnessArtifactError::Validation(findings));
    }
    if !export.is_export_safe() {
        return Err(DocsVersionFreshnessArtifactError::NotExportSafe);
    }
    Ok(export)
}

fn promotion_state_for(
    validation: &[DocsVersionFreshnessValidationFinding],
    findings: &[DocsVersionFreshnessFinding],
) -> DocsVersionFreshnessPromotionState {
    let any_blocker = validation
        .iter()
        .any(|finding| finding.severity == DocsVersionFreshnessValidationSeverity::Blocker)
        || findings.iter().any(|finding| {
            finding.effective_severity() == DocsVersionFreshnessFindingSeverity::Blocking
        });
    if any_blocker {
        return DocsVersionFreshnessPromotionState::BlocksStable;
    }

    let any_warning = validation
        .iter()
        .any(|finding| finding.severity == DocsVersionFreshnessValidationSeverity::Warning)
        || findings.iter().any(|finding| {
            finding.effective_severity() == DocsVersionFreshnessFindingSeverity::Narrowing
        });
    if any_warning {
        DocsVersionFreshnessPromotionState::NarrowedBelowStable
    } else {
        DocsVersionFreshnessPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_query:")
                || lower.contains("raw_body:")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

mod seed {
    use super::*;

    const PACKET_ID: &str = "packet:docs_version_freshness_findings:001";

    pub(super) fn seeded_input() -> DocsVersionFreshnessPacketInput {
        DocsVersionFreshnessPacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label:
                "workflow:docs_version_freshness_state_and_stale_example_broken_link_findings:stable"
                    .to_owned(),
            generated_at: "2026-06-26T00:00:00Z".to_owned(),
            cards: cards(),
            findings: findings(),
            consumer_projections: projections(),
            source_contract_refs: vec![
                DOCS_VERSION_FRESHNESS_SCHEMA_REF.to_owned(),
                DOCS_VERSION_FRESHNESS_DOC_REF.to_owned(),
                DOCS_VERSION_FRESHNESS_ARTIFACT_REF.to_owned(),
                DOCS_VERSION_FRESHNESS_SUMMARY_REF.to_owned(),
            ],
            redaction_class_token: "metadata_safe_default".to_owned(),
        }
    }

    fn disclosure(
        active: &str,
        viewed: &str,
        changes: bool,
        summary: &str,
    ) -> DocsVersionFreshnessDisclosure {
        DocsVersionFreshnessDisclosure {
            active_version_ref: active.to_owned(),
            viewed_version_ref: viewed.to_owned(),
            difference_changes_api_or_workflow: changes,
            difference_summary: Some(summary.to_owned()),
        }
    }

    fn card(
        card_id: &str,
        doc_node_ref: &str,
        display_label: &str,
        state: DocsVersionFreshnessState,
        version_disclosure: Option<DocsVersionFreshnessDisclosure>,
        state_reason: Option<&str>,
        open_current_source_ref: &str,
    ) -> DocsVersionFreshnessCard {
        DocsVersionFreshnessCard {
            card_id: card_id.to_owned(),
            doc_node_ref: doc_node_ref.to_owned(),
            display_label: display_label.to_owned(),
            state,
            confidence: state.confidence_class(),
            version_disclosure,
            state_reason: state_reason.map(str::to_owned),
            open_current_source_ref: open_current_source_ref.to_owned(),
            raw_boundary_material_excluded: true,
        }
    }

    fn cards() -> Vec<DocsVersionFreshnessCard> {
        vec![
            card(
                "card:exact:async-runtime-api",
                "docnode:mirror:tokio/runtime#spawn",
                "tokio::runtime spawn (exact)",
                DocsVersionFreshnessState::Exact,
                None,
                None,
                "open-current-source:docnode:mirror:tokio/runtime#spawn",
            ),
            card(
                "card:nearby:http-client-guide",
                "docnode:mirror:reqwest/client-guide",
                "reqwest client guide (nearby)",
                DocsVersionFreshnessState::Nearby,
                Some(disclosure(
                    "package:reqwest@0.12.5",
                    "docs:reqwest@0.12.2",
                    true,
                    "the builder signature changed between the viewed and active minor",
                )),
                None,
                "open-current-source:docnode:mirror:reqwest/client-guide@active",
            ),
            card(
                "card:project_specific:workspace-architecture",
                "docnode:project-docs:architecture/overview",
                "Workspace architecture overview (project)",
                DocsVersionFreshnessState::ProjectSpecific,
                None,
                None,
                "open-current-source:repo:docs/architecture/overview.md",
            ),
            card(
                "card:mirrored:serde-derive",
                "docnode:mirror:serde/derive",
                "serde derive guide (mirrored)",
                DocsVersionFreshnessState::Mirrored,
                Some(disclosure(
                    "package:serde@1.0.203",
                    "mirror:serde@1.0.203",
                    false,
                    "pinned signed mirror at the active version",
                )),
                None,
                "open-current-source:mirror:serde/derive",
            ),
            card(
                "card:cached:cli-reference",
                "docnode:cache:cargo/cli-reference",
                "cargo CLI reference (cached)",
                DocsVersionFreshnessState::Cached,
                Some(disclosure(
                    "toolchain:cargo@1.84.0",
                    "docs-cache:cargo@1.81.0",
                    true,
                    "cached copy predates the active toolchain; refresh unverified",
                )),
                None,
                "open-current-source:docnode:cache:cargo/cli-reference@active",
            ),
            card(
                "card:stale:migration-guide",
                "docnode:mirror:axum/migration-0.6-to-0.7",
                "axum migration guide (stale)",
                DocsVersionFreshnessState::Stale,
                Some(disclosure(
                    "package:axum@0.8.1",
                    "docs:axum@0.7.0",
                    true,
                    "guide predates the active major and is known stale",
                )),
                None,
                "open-current-source:docnode:mirror:axum/migration@active",
            ),
            card(
                "card:policy_blocked:enterprise-runbook",
                "docnode:support-runbook:enterprise/rotation",
                "Enterprise rotation runbook (policy-blocked)",
                DocsVersionFreshnessState::PolicyBlocked,
                None,
                Some("organization policy restricts this runbook to the security workspace"),
                "open-current-source:policy:enterprise/rotation",
            ),
            card(
                "card:browser_handoff:hosted-changelog",
                "docnode:live-external:vendor/changelog",
                "Vendor changelog (browser handoff)",
                DocsVersionFreshnessState::BrowserHandoffRequired,
                None,
                Some("source is not mirrored locally; opens in an isolated browser session"),
                "open-current-source:browser-handoff:vendor/changelog",
            ),
        ]
    }

    fn actions(
        suppression_state: DocsVersionFreshnessSuppressionState,
        suppression_reason: Option<&str>,
        compare_ref: &str,
        open_current_source_ref: &str,
    ) -> DocsVersionFreshnessFindingActions {
        DocsVersionFreshnessFindingActions {
            suppression_state,
            suppression_reason: suppression_reason.map(str::to_owned),
            compare_ref: compare_ref.to_owned(),
            open_current_source_ref: open_current_source_ref.to_owned(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn finding(
        finding_id: &str,
        finding_class: DocsVersionFreshnessFindingClass,
        subject_kind: DocsVersionFreshnessSubjectKind,
        card_id_ref: &str,
        doc_node_ref: &str,
        observed_ref: &str,
        compared_against_ref: &str,
        current_ref: Option<&str>,
        summary: &str,
        actions: DocsVersionFreshnessFindingActions,
    ) -> DocsVersionFreshnessFinding {
        DocsVersionFreshnessFinding {
            finding_id: finding_id.to_owned(),
            finding_class,
            subject_kind,
            severity: DocsVersionFreshnessFindingSeverity::Advisory,
            card_id_ref: card_id_ref.to_owned(),
            doc_node_ref: doc_node_ref.to_owned(),
            observed_ref: observed_ref.to_owned(),
            compared_against_ref: compared_against_ref.to_owned(),
            current_ref: current_ref.map(str::to_owned),
            summary: summary.to_owned(),
            actions,
        }
    }

    fn findings() -> Vec<DocsVersionFreshnessFinding> {
        vec![
            finding(
                "finding:stale-example:axum-router",
                DocsVersionFreshnessFindingClass::StaleExample,
                DocsVersionFreshnessSubjectKind::CodeBlock,
                "card:stale:migration-guide",
                "docnode:mirror:axum/migration-0.6-to-0.7#router",
                "example:axum-router@0.6",
                "graph:symbol:axum::Router@0.8.1",
                Some("example:axum-router@0.8"),
                "the router example predates the active major and no longer compiles",
                actions(
                    DocsVersionFreshnessSuppressionState::Active,
                    None,
                    "compare:example:axum-router@0.6-vs-0.8",
                    "open-current-source:graph:symbol:axum::Router@active",
                ),
            ),
            finding(
                "finding:broken-link:reqwest-anchor",
                DocsVersionFreshnessFindingClass::BrokenLink,
                DocsVersionFreshnessSubjectKind::Link,
                "card:nearby:http-client-guide",
                "docnode:mirror:reqwest/client-guide#proxies",
                "link:reqwest/client-guide#proxy-config",
                "graph:anchor-index:reqwest/client-guide@0.12.5",
                None,
                "the anchor moved between minors and the in-doc link no longer resolves",
                actions(
                    DocsVersionFreshnessSuppressionState::Active,
                    None,
                    "compare:anchor:reqwest/client-guide#proxies",
                    "open-current-source:docnode:mirror:reqwest/client-guide@active#proxies",
                ),
            ),
            finding(
                "finding:nearby-version:reqwest-builder",
                DocsVersionFreshnessFindingClass::NearbyVersionExample,
                DocsVersionFreshnessSubjectKind::Command,
                "card:nearby:http-client-guide",
                "docnode:mirror:reqwest/client-guide#builder",
                "example:reqwest-builder@0.12.2",
                "graph:symbol:reqwest::ClientBuilder@0.12.5",
                Some("example:reqwest-builder@0.12.5"),
                "a nearer-version example exists for the active minor",
                actions(
                    DocsVersionFreshnessSuppressionState::Active,
                    None,
                    "compare:example:reqwest-builder@0.12.2-vs-0.12.5",
                    "open-current-source:graph:symbol:reqwest::ClientBuilder@active",
                ),
            ),
            finding(
                "finding:removed-api:cargo-flag",
                DocsVersionFreshnessFindingClass::RemovedApiReference,
                DocsVersionFreshnessSubjectKind::ApiReference,
                "card:cached:cli-reference",
                "docnode:cache:cargo/cli-reference#z-flags",
                "api:cargo::--z-unstable-options@1.81.0",
                "graph:symbol:cargo-cli@1.84.0",
                None,
                "the referenced unstable flag was removed in the active toolchain",
                actions(
                    DocsVersionFreshnessSuppressionState::Active,
                    None,
                    "compare:api:cargo-cli@1.81.0-vs-1.84.0",
                    "open-current-source:graph:symbol:cargo-cli@active",
                ),
            ),
            finding(
                "finding:changed-config:serde-rename",
                DocsVersionFreshnessFindingClass::ChangedConfigPath,
                DocsVersionFreshnessSubjectKind::ConfigPath,
                "card:mirrored:serde-derive",
                "docnode:mirror:serde/derive#container-attrs",
                "config:serde(rename_all=lowercase)",
                "graph:pack-metadata:serde-attrs@1.0.203",
                Some("config:serde(rename_all=snake_case)"),
                "the documented attribute value differs from the current pack metadata",
                actions(
                    DocsVersionFreshnessSuppressionState::Active,
                    None,
                    "compare:config:serde-rename-all",
                    "open-current-source:graph:pack-metadata:serde-attrs@active",
                ),
            ),
        ]
    }

    fn projection(
        surface: DocsVersionFreshnessConsumerSurface,
    ) -> DocsVersionFreshnessConsumerProjection {
        DocsVersionFreshnessConsumerProjection {
            surface,
            projection_ref: format!("projection:{}:{}", PACKET_ID, surface.as_str()),
            packet_id_ref: PACKET_ID.to_owned(),
            preserves_state_badge: true,
            preserves_state_distinctions: true,
            preserves_confidence_treatment: true,
            preserves_version_disclosure: true,
            preserves_findings: true,
            preserves_finding_actions: true,
            raw_private_material_excluded: true,
        }
    }

    fn projections() -> Vec<DocsVersionFreshnessConsumerProjection> {
        DocsVersionFreshnessConsumerSurface::REQUIRED
            .into_iter()
            .map(projection)
            .collect()
    }
}
