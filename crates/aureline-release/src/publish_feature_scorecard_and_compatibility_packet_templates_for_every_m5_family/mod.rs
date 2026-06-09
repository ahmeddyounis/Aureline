//! Typed M5 feature scorecard and compatibility-packet template register.
//!
//! This module publishes the canonical scorecard and compatibility-packet
//! templates for every M5 feature family. Each [`M5FamilyTemplateRow`] binds
//! one M5 family to:
//!
//! - a [`ScorecardTemplate`] that defines the required scorecard sections
//!   ([`ScorecardSectionKind`]) and their publication state,
//! - a [`CompatibilityPacketTemplate`] that defines the required
//!   compatibility-packet sections ([`CompatibilityPacketSectionKind`]) and
//!   their publication state,
//! - the register state earned ([`TemplateRegisterState`]), the active gap
//!   reasons ([`TemplateGapReason`]), and the effective label after narrowing
//!   ([`M5FamilyTemplateRow::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a family whose templates may publish as Stable and one that
//! must narrow below it. The [`TemplateStopRule`] set names the closed
//! conditions that gate template publication, and
//! [`M5TemplateRegister::publication`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no raw artifacts, raw logs, signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_RECORD_KIND: &str =
    "publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family";

/// Repo-relative path to the checked-in register.
pub const PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_PATH: &str =
    "artifacts/release/m5/publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family.json";

/// Embedded checked-in register JSON.
pub const PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family.json"
    ));

/// M5 feature family a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FamilyKind {
    /// Notebook and data-rich promoted surfaces.
    Notebook,
    /// Data-heavy surfaces (result grids, variable explorers).
    DataRich,
    /// AI-adjacent surfaces and language intelligence.
    AiAdjacent,
    /// Core framework and platform foundations.
    Framework,
    /// Review and diff surfaces.
    Review,
    /// Browser/mobile companion surfaces.
    Companion,
    /// Managed-depth and infrastructure surfaces.
    ManagedDepth,
}

impl M5FamilyKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Notebook,
        Self::DataRich,
        Self::AiAdjacent,
        Self::Framework,
        Self::Review,
        Self::Companion,
        Self::ManagedDepth,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataRich => "data_rich",
            Self::AiAdjacent => "ai_adjacent",
            Self::Framework => "framework",
            Self::Review => "review",
            Self::Companion => "companion",
            Self::ManagedDepth => "managed_depth",
        }
    }
}

/// Section kind in a scorecard template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScorecardSectionKind {
    /// Proof packet section.
    ProofPacket,
    /// Compatibility report section.
    CompatibilityReport,
    /// Admin/policy story section.
    AdminPolicy,
    /// Rollback or downgrade path section.
    RollbackPath,
    /// Owner sign-off section.
    OwnerSignoff,
}

impl ScorecardSectionKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProofPacket,
        Self::CompatibilityReport,
        Self::AdminPolicy,
        Self::RollbackPath,
        Self::OwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofPacket => "proof_packet",
            Self::CompatibilityReport => "compatibility_report",
            Self::AdminPolicy => "admin_policy",
            Self::RollbackPath => "rollback_path",
            Self::OwnerSignoff => "owner_signoff",
        }
    }
}

/// Section kind in a compatibility-packet template.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityPacketSectionKind {
    /// Schema surface compatibility section.
    SchemaSurface,
    /// API surface compatibility section.
    ApiSurface,
    /// CLI surface compatibility section.
    CliSurface,
    /// Platform matrix section.
    PlatformMatrix,
    /// Downgrade behavior section.
    DowngradeBehavior,
    /// Mixed-version posture section.
    MixedVersionPosture,
    /// Deprecation window section.
    DeprecationWindow,
}

impl CompatibilityPacketSectionKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SchemaSurface,
        Self::ApiSurface,
        Self::CliSurface,
        Self::PlatformMatrix,
        Self::DowngradeBehavior,
        Self::MixedVersionPosture,
        Self::DeprecationWindow,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaSurface => "schema_surface",
            Self::ApiSurface => "api_surface",
            Self::CliSurface => "cli_surface",
            Self::PlatformMatrix => "platform_matrix",
            Self::DowngradeBehavior => "downgrade_behavior",
            Self::MixedVersionPosture => "mixed_version_posture",
            Self::DeprecationWindow => "deprecation_window",
        }
    }
}

/// State of an individual template section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateSectionState {
    /// Section is planned but not yet drafted.
    Planned,
    /// Section is drafted but not yet reviewed.
    Drafted,
    /// Section has been reviewed.
    Reviewed,
    /// Section is published and current.
    Published,
    /// Section is published but has gone stale.
    Stale,
    /// Section is missing.
    Missing,
}

impl TemplateSectionState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Planned,
        Self::Drafted,
        Self::Reviewed,
        Self::Published,
        Self::Stale,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Drafted => "drafted",
            Self::Reviewed => "reviewed",
            Self::Published => "published",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// Whether the state counts as complete for a template section.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Reviewed | Self::Published)
    }
}

/// Register state a feature-family template binding earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateRegisterState {
    /// All template sections are complete and the owner is signed off.
    Complete,
    /// One or more required template sections are missing or incomplete.
    Incomplete,
    /// A template section has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    OnWaiver,
    /// Blocked by a missing owner or incomplete owner sign-off.
    OwnerBlocked,
}

impl TemplateRegisterState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Complete,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::OwnerBlocked,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::OwnerBlocked => "owner_blocked",
        }
    }

    /// Whether the state lets a family carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Complete | Self::OnWaiver)
    }

    /// Whether the state forces the family below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an M5 template row narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateGapReason {
    /// Scorecard template is incomplete.
    ScorecardTemplateIncomplete,
    /// Scorecard template is stale.
    ScorecardTemplateStale,
    /// Compatibility-packet template is incomplete.
    CompatibilityPacketTemplateIncomplete,
    /// Compatibility-packet template is stale.
    CompatibilityPacketTemplateStale,
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Owner sign-off is missing.
    OwnerSignoffMissing,
    /// A required waiver has expired.
    WaiverExpired,
}

impl TemplateGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ScorecardTemplateIncomplete,
        Self::ScorecardTemplateStale,
        Self::CompatibilityPacketTemplateIncomplete,
        Self::CompatibilityPacketTemplateStale,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScorecardTemplateIncomplete => "scorecard_template_incomplete",
            Self::ScorecardTemplateStale => "scorecard_template_stale",
            Self::CompatibilityPacketTemplateIncomplete => {
                "compatibility_packet_template_incomplete"
            }
            Self::CompatibilityPacketTemplateStale => "compatibility_packet_template_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Refresh the scorecard template.
    RefreshScorecardTemplate,
    /// Refresh the compatibility-packet template.
    RefreshCompatibilityPacketTemplate,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl TemplateAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::RefreshScorecardTemplate,
        Self::RefreshCompatibilityPacketTemplate,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::RefreshScorecardTemplate => "refresh_scorecard_template",
            Self::RefreshCompatibilityPacketTemplate => "refresh_compatibility_packet_template",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}
/// One section in a scorecard template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScorecardTemplateSection {
    /// Stable section id.
    pub section_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of scorecard section.
    pub section_kind: ScorecardSectionKind,
    /// Whether this section is required for the template to be complete.
    pub required: bool,
    /// Current state of the section.
    pub item_state: TemplateSectionState,
    /// Reviewable reason this section carries this state.
    pub rationale: String,
}

/// The scorecard template for an M5 family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScorecardTemplate {
    /// Stable template id.
    pub template_id: String,
    /// Template version.
    pub template_version: u32,
    /// Template sections.
    pub sections: Vec<ScorecardTemplateSection>,
}

impl ScorecardTemplate {
    /// True when every required section is present and complete.
    pub fn is_complete(&self) -> bool {
        self.sections
            .iter()
            .all(|s| !s.required || s.item_state.is_complete())
    }

    /// True when every required section is present.
    pub fn has_all_required_sections(&self) -> bool {
        let present: BTreeSet<ScorecardSectionKind> = self
            .sections
            .iter()
            .filter(|s| s.required)
            .map(|s| s.section_kind)
            .collect();
        present.len() == ScorecardSectionKind::ALL.len()
    }

    /// Returns sections whose state is stale.
    pub fn stale_sections(&self) -> Vec<&ScorecardTemplateSection> {
        self.sections
            .iter()
            .filter(|s| s.item_state == TemplateSectionState::Stale)
            .collect()
    }

    /// Returns sections whose state is missing.
    pub fn missing_sections(&self) -> Vec<&ScorecardTemplateSection> {
        self.sections
            .iter()
            .filter(|s| s.item_state == TemplateSectionState::Missing)
            .collect()
    }

    /// Returns required sections whose state is not complete.
    pub fn incomplete_required_sections(&self) -> Vec<&ScorecardTemplateSection> {
        self.sections
            .iter()
            .filter(|s| s.required && !s.item_state.is_complete())
            .collect()
    }
}

/// One section in a compatibility-packet template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityPacketTemplateSection {
    /// Stable section id.
    pub section_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of compatibility-packet section.
    pub section_kind: CompatibilityPacketSectionKind,
    /// Whether this section is required for the template to be complete.
    pub required: bool,
    /// Current state of the section.
    pub item_state: TemplateSectionState,
    /// Reviewable reason this section carries this state.
    pub rationale: String,
}

/// The compatibility-packet template for an M5 family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityPacketTemplate {
    /// Stable template id.
    pub template_id: String,
    /// Template version.
    pub template_version: u32,
    /// Template sections.
    pub sections: Vec<CompatibilityPacketTemplateSection>,
}

impl CompatibilityPacketTemplate {
    /// True when every required section is present and complete.
    pub fn is_complete(&self) -> bool {
        self.sections
            .iter()
            .all(|s| !s.required || s.item_state.is_complete())
    }

    /// True when every required section kind is present.
    pub fn has_all_required_sections(&self) -> bool {
        let present: BTreeSet<CompatibilityPacketSectionKind> = self
            .sections
            .iter()
            .filter(|s| s.required)
            .map(|s| s.section_kind)
            .collect();
        present.len() == CompatibilityPacketSectionKind::ALL.len()
    }

    /// Returns sections whose state is stale.
    pub fn stale_sections(&self) -> Vec<&CompatibilityPacketTemplateSection> {
        self.sections
            .iter()
            .filter(|s| s.item_state == TemplateSectionState::Stale)
            .collect()
    }

    /// Returns sections whose state is missing.
    pub fn missing_sections(&self) -> Vec<&CompatibilityPacketTemplateSection> {
        self.sections
            .iter()
            .filter(|s| s.item_state == TemplateSectionState::Missing)
            .collect()
    }

    /// Returns required sections whose state is not complete.
    pub fn incomplete_required_sections(&self) -> Vec<&CompatibilityPacketTemplateSection> {
        self.sections
            .iter()
            .filter(|s| s.required && !s.item_state.is_complete())
            .collect()
    }
}

/// One template stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemplateStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: TemplateGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: TemplateAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 family template row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5FamilyTemplateRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The family kind this row governs.
    pub family_kind: M5FamilyKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the row.
    pub surface_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The scorecard template for this family.
    pub scorecard_template: ScorecardTemplate,
    /// The compatibility-packet template for this family.
    pub compatibility_packet_template: CompatibilityPacketTemplate,
    /// Register state earned for the row.
    pub template_state: TemplateRegisterState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<TemplateGapReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5FamilyTemplateRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the family carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.template_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: TemplateGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when the scorecard template has all required sections complete.
    pub fn scorecard_complete(&self) -> bool {
        self.scorecard_template.is_complete() && self.scorecard_template.has_all_required_sections()
    }

    /// True when the compatibility-packet template has all required sections complete.
    pub fn compatibility_packet_complete(&self) -> bool {
        self.compatibility_packet_template.is_complete()
            && self
                .compatibility_packet_template
                .has_all_required_sections()
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5TemplateRegisterSummary {
    /// Total number of family rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows publishing a label at or above the cutline.
    pub entries_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Rows blocked by a missing owner sign-off.
    pub entries_owner_blocked: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_holding: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook rows.
    pub notebook_entries: usize,
    /// Data-rich rows.
    pub data_rich_entries: usize,
    /// AI-adjacent rows.
    pub ai_adjacent_entries: usize,
    /// Framework rows.
    pub framework_entries: usize,
    /// Review rows.
    pub review_entries: usize,
    /// Companion rows.
    pub companion_entries: usize,
    /// Managed-depth rows.
    pub managed_depth_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Scorecard-template sections whose state is `published`.
    pub scorecard_sections_published: usize,
    /// Scorecard-template sections whose state is `stale`.
    pub scorecard_sections_stale: usize,
    /// Scorecard-template sections whose state is `missing`.
    pub scorecard_sections_missing: usize,
    /// Compatibility-packet-template sections whose state is `published`.
    pub compat_sections_published: usize,
    /// Compatibility-packet-template sections whose state is `stale`.
    pub compat_sections_stale: usize,
    /// Compatibility-packet-template sections whose state is `missing`.
    pub compat_sections_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateRegisterExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The family kind this row governs.
    pub family_kind: M5FamilyKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Register state earned.
    pub template_state: TemplateRegisterState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<TemplateGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TemplateRegisterExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5TemplateRegisterExportRow>,
}

/// The typed M5 feature scorecard and compatibility-packet template register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5TemplateRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests.
    pub claim_manifest_ref: String,
    /// Ref to the M5 feature-train matrix this register builds on.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<M5FamilyKind>,
    /// Closed scorecard-section-kind vocabulary.
    pub scorecard_section_kinds: Vec<ScorecardSectionKind>,
    /// Closed compatibility-packet-section-kind vocabulary.
    pub compatibility_packet_section_kinds: Vec<CompatibilityPacketSectionKind>,
    /// Closed template-section-state vocabulary.
    pub template_section_states: Vec<TemplateSectionState>,
    /// Closed template-register-state vocabulary.
    pub template_states: Vec<TemplateRegisterState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<TemplateGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<TemplateAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<TemplateStopRule>,
    /// Family rows.
    pub rows: Vec<M5FamilyTemplateRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5TemplateRegisterSummary,
}

impl M5TemplateRegister {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5FamilyTemplateRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5FamilyTemplateRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5FamilyTemplateRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5FamilyTemplateRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one family kind.
    pub fn rows_for_kind(&self, kind: M5FamilyKind) -> Vec<&M5FamilyTemplateRow> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &TemplateStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Family-row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<TemplateGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> M5TemplateRegisterSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5FamilyKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5FamilyTemplateRow> = self.release_blocking_rows();
        let scorecard_sections = |state: TemplateSectionState| {
            self.rows
                .iter()
                .flat_map(|row| &row.scorecard_template.sections)
                .filter(|s| s.item_state == state)
                .count()
        };
        let compat_sections = |state: TemplateSectionState| {
            self.rows
                .iter()
                .flat_map(|row| &row.compatibility_packet_template.sections)
                .filter(|s| s.item_state == state)
                .count()
        };
        M5TemplateRegisterSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_holding_stable: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.template_state == TemplateRegisterState::OnWaiver)
                .count(),
            entries_owner_blocked: self
                .rows
                .iter()
                .filter(|row| row.template_state == TemplateRegisterState::OwnerBlocked)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_holding: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_entries: kind(M5FamilyKind::Notebook),
            data_rich_entries: kind(M5FamilyKind::DataRich),
            ai_adjacent_entries: kind(M5FamilyKind::AiAdjacent),
            framework_entries: kind(M5FamilyKind::Framework),
            review_entries: kind(M5FamilyKind::Review),
            companion_entries: kind(M5FamilyKind::Companion),
            managed_depth_entries: kind(M5FamilyKind::ManagedDepth),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            scorecard_sections_published: scorecard_sections(TemplateSectionState::Published),
            scorecard_sections_stale: scorecard_sections(TemplateSectionState::Stale),
            scorecard_sections_missing: scorecard_sections(TemplateSectionState::Missing),
            compat_sections_published: compat_sections(TemplateSectionState::Published),
            compat_sections_stale: compat_sections(TemplateSectionState::Stale),
            compat_sections_missing: compat_sections(TemplateSectionState::Missing),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5TemplateRegisterExportProjection {
        M5TemplateRegisterExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5TemplateRegisterExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    template_state: row.template_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    owner_ref: row.owner_signoff.owner_ref.clone(),
                })
                .collect(),
        }
    }
    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5TemplateRegisterViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5TemplateRegisterViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5TemplateRegisterViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5TemplateRegisterViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5TemplateRegisterViolation>) {
        if self.schema_version != PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_SCHEMA_VERSION
        {
            violations.push(M5TemplateRegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_RECORD_KIND
        {
            violations.push(M5TemplateRegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("feature_train_matrix_ref", &self.feature_train_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5TemplateRegisterViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != M5FamilyKind::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.scorecard_section_kinds != ScorecardSectionKind::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "scorecard_section_kinds",
            });
        }
        if self.compatibility_packet_section_kinds != CompatibilityPacketSectionKind::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "compatibility_packet_section_kinds",
            });
        }
        if self.template_section_states != TemplateSectionState::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "template_section_states",
            });
        }
        if self.template_states != TemplateRegisterState::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "template_states",
            });
        }
        if self.gap_reasons != TemplateGapReason::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != TemplateAction::ALL.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5TemplateRegisterViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5TemplateRegisterViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5TemplateRegisterViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5TemplateRegisterViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5TemplateRegisterViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5TemplateRegisterViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5TemplateRegisterViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in TemplateGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5TemplateRegisterViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5FamilyTemplateRow,
        violations: &mut Vec<M5TemplateRegisterViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5TemplateRegisterViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5TemplateRegisterViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5TemplateRegisterViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5TemplateRegisterViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must have owner sign-off.
        if row.holds_label() && !row.owner_signoff.signed_off {
            violations.push(M5TemplateRegisterViolation::HeldWithoutSignoff {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must not have active gap reasons.
        if row.holds_label() && !row.active_gap_reasons.is_empty() {
            violations.push(M5TemplateRegisterViolation::HeldWithActiveGap {
                entry_id: row.entry_id.clone(),
                reasons: row.active_gap_reasons.clone(),
            });
        }

        // A row whose state forces narrowing must actually narrow.
        if row.template_state.forces_narrowing()
            && row.published_label.rank() >= row.claim_label.rank()
        {
            violations.push(M5TemplateRegisterViolation::PublishedLabelNotNarrowed {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
                state: row.template_state,
            });
        }

        // Scorecard template must contain all required section kinds.
        if !row.scorecard_template.has_all_required_sections() {
            violations.push(M5TemplateRegisterViolation::IncompleteScorecardTemplate {
                entry_id: row.entry_id.clone(),
            });
        }

        // Compatibility-packet template must contain all required section kinds.
        if !row
            .compatibility_packet_template
            .has_all_required_sections()
        {
            violations.push(
                M5TemplateRegisterViolation::IncompleteCompatibilityPacketTemplate {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5TemplateRegisterViolation>) {
        let covered_refs: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for surface_ref in &self.release_blocking_family_refs {
            if !covered_refs.contains(surface_ref) {
                violations.push(
                    M5TemplateRegisterViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: surface_ref.clone(),
                    },
                );
            }
        }
        let rb_set: BTreeSet<String> = self.release_blocking_family_refs.iter().cloned().collect();
        for row in &self.rows {
            if row.release_blocking && !rb_set.contains(&row.surface_ref) {
                violations.push(M5TemplateRegisterViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5TemplateRegisterViolation>) {
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5TemplateRegisterViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        let blocking_rules = self.computed_blocking_rule_ids();
        if self.publication.blocking_rule_ids != blocking_rules {
            violations.push(
                M5TemplateRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        let blocking_entries = self.computed_blocking_entry_ids();
        if self.publication.blocking_claim_ids != blocking_entries {
            violations.push(
                M5TemplateRegisterViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// Validation error for the M5 template register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TemplateRegisterViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion { actual: u32 },
    /// Record kind does not match the expected kind.
    UnsupportedRecordKind { actual: String },
    /// A required field is empty.
    EmptyField {
        entry_id: String,
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch { field: &'static str },
    /// No stop rules are defined.
    NoStopRules,
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId { rule_id: String },
    /// A stop rule watches no labels.
    StopRuleWithoutLabels { rule_id: String },
    /// A gap reason has no covering stop rule.
    GapReasonWithoutStopRule { reason: TemplateGapReason },
    /// A row id appears more than once.
    DuplicateEntryId { entry_id: String },
    /// The register contains no rows.
    EmptyRegister,
    /// The published label is wider than the canonical claim label.
    PublishedWiderThanClaim {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
    },
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent { entry_id: String },
    /// A row that holds its label lacks owner sign-off.
    HeldWithoutSignoff { entry_id: String },
    /// A row that holds its label has active gap reasons.
    HeldWithActiveGap {
        entry_id: String,
        reasons: Vec<TemplateGapReason>,
    },
    /// A row whose state forces narrowing did not narrow.
    PublishedLabelNotNarrowed {
        entry_id: String,
        claim: StableClaimLevel,
        published: StableClaimLevel,
        state: TemplateRegisterState,
    },
    /// A release-blocking surface has no covering row.
    ReleaseBlockingSurfaceUncovered { surface_ref: String },
    /// A release-blocking row is not declared in release_blocking_family_refs.
    ReleaseBlockingRowNotDeclared { entry_id: String },
    /// The declared publication decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        declared: PromotionDecision,
        computed: PromotionDecision,
    },
    /// A publication blocking set field disagrees with firing stop rules.
    PublicationBlockingSetMismatch { field: &'static str },
    /// Summary counts disagree with row state.
    SummaryMismatch,
    /// Scorecard template is missing required sections.
    IncompleteScorecardTemplate { entry_id: String },
    /// Compatibility-packet template is missing required sections.
    IncompleteCompatibilityPacketTemplate { entry_id: String },
}

impl fmt::Display for M5TemplateRegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind {actual}")
            }
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id}: empty field {field_name}"),
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::NoStopRules => write!(f, "no stop rules defined"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop-rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => {
                write!(f, "gap reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "{entry_id}: published label {published:?} is wider than claim {claim:?}"
            ),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(
                    f,
                    "row {entry_id} holds its label but lacks owner sign-off"
                )
            }
            Self::HeldWithActiveGap { entry_id, reasons } => {
                write!(
                    f,
                    "row {entry_id} holds its label but has active gaps: {reasons:?}"
                )
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                claim,
                published,
                state,
            } => write!(
                f,
                "row {entry_id} state {state:?} forces narrowing but claim {claim:?} / published {published:?} does not narrow"
            ),
            Self::ReleaseBlockingSurfaceUncovered { surface_ref } => {
                write!(
                    f,
                    "release-blocking surface {surface_ref} has no covering row"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking row {entry_id} is not declared in release_blocking_family_refs"
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication {declared:?} disagrees with computed {computed:?}"
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::IncompleteScorecardTemplate { entry_id } => {
                write!(f, "row {entry_id} scorecard template is incomplete")
            }
            Self::IncompleteCompatibilityPacketTemplate { entry_id } => {
                write!(f, "row {entry_id} compatibility-packet template is incomplete")
            }
        }
    }
}

impl Error for M5TemplateRegisterViolation {}

/// Loads the embedded M5 template register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5TemplateRegister`].
pub fn current_m5_template_register() -> Result<M5TemplateRegister, serde_json::Error> {
    serde_json::from_str(
        PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_JSON,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    fn register() -> M5TemplateRegister {
        current_m5_template_register().expect("register parses")
    }

    #[test]
    fn embedded_register_parses_and_validates() {
        let r = register();
        assert_eq!(
            r.schema_version,
            PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_SCHEMA_VERSION
        );
        assert_eq!(
            r.record_kind,
            PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_RECORD_KIND
        );
        assert_eq!(r.validate(), Vec::new());
        assert!(!r.rows.is_empty());
    }

    #[test]
    fn covers_every_family_kind() {
        let r = register();
        for kind in M5FamilyKind::ALL {
            assert!(
                !r.rows_for_kind(kind).is_empty(),
                "family kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let r = register();
        assert!(!r.release_blocking_family_refs.is_empty());
        let covered: Vec<&str> = r
            .release_blocking_rows()
            .iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &r.release_blocking_family_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let r = register();
        assert_eq!(r.summary, r.computed_summary());
        assert_eq!(
            r.summary.entries_holding_stable + r.summary.entries_narrowed,
            r.rows.len()
        );
    }

    #[test]
    fn publication_decision_matches_computed() {
        let r = register();
        assert_eq!(r.publication.decision, r.computed_publication_decision());
        assert_eq!(
            r.publication.blocking_rule_ids,
            r.computed_blocking_rule_ids()
        );
        assert_eq!(
            r.publication.blocking_claim_ids,
            r.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let r = register();
        let covered: BTreeSet<TemplateGapReason> = r
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in TemplateGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_gap() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.active_gap_reasons
            .push(TemplateGapReason::ProofPacketMissing);
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5TemplateRegisterViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.template_state = TemplateRegisterState::Incomplete;
        row.active_gap_reasons
            .push(TemplateGapReason::ProofPacketMissing);
        row.published_label = StableClaimLevel::Stable;
        r.summary = r.computed_summary();
        r.publication.decision = r.computed_publication_decision();
        r.publication.blocking_rule_ids = r.computed_blocking_rule_ids();
        r.publication.blocking_claim_ids = r.computed_blocking_entry_ids();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5TemplateRegisterViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut r = register();
        r.publication.decision = PromotionDecision::Proceed;
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5TemplateRegisterViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_claim_without_signoff() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        r.summary = r.computed_summary();
        assert!(r
            .validate()
            .iter()
            .any(|v| matches!(v, M5TemplateRegisterViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn validate_flags_an_incomplete_scorecard_template() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.scorecard_template.sections.clear();
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5TemplateRegisterViolation::IncompleteScorecardTemplate { .. }
        )));
    }

    #[test]
    fn validate_flags_an_incomplete_compatibility_packet_template() {
        let mut r = register();
        let row = r
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.compatibility_packet_template.sections.clear();
        r.summary = r.computed_summary();
        assert!(r.validate().iter().any(|v| matches!(
            v,
            M5TemplateRegisterViolation::IncompleteCompatibilityPacketTemplate { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let r = register();
        let projection = r.support_export_projection();
        assert_eq!(projection.rows.len(), r.rows.len());
        for (row, proj) in r.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
