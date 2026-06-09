//! Typed M5 storage, retention, export, and offboarding matrix for new durable artifacts.
//!
//! This module publishes the canonical matrix that governs the storage posture,
//! retention policy, export path, and offboarding behavior for every new durable
//! artifact class introduced in Milestone 5. Each [`M5ArtifactRetentionRow`] binds
//! one artifact class to:
//!
//! - a [`ArtifactRetentionPosture`] composed of required posture indicators
//!   ([`RetentionPostureIndicator`]), each with its kind, state, and artifact ref,
//! - the matrix state earned ([`ArtifactRetentionState`]), the active gap reasons
//!   ([`ArtifactRetentionGapReason`]), and the effective label after narrowing
//!   ([`M5ArtifactRetentionRow::published_label`]),
//! - a proof packet with freshness SLO ([`ProofPacket`]),
//! - owner sign-off ([`OwnerSignoff`]),
//! - stop rules that gate publication when posture indicators are missing, stale, or
//!   unsigned.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between an artifact class that may publish as Stable and one that must
//! narrow below it. The [`ArtifactRetentionStopRule`] set names the closed conditions
//! that gate publication, and [`M5StorageRetentionMatrix::publication`] records the
//! proceed/hold verdict.
//!
//! The matrix is checked in at
//! `artifacts/release/m5/publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts.json`
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

/// Supported matrix schema version.
pub const PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_RECORD_KIND: &str =
    "publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts";

/// Repo-relative path to the checked-in matrix.
pub const PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_PATH: &str =
    "artifacts/release/m5/publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts.json";

/// Embedded checked-in matrix JSON.
pub const PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_JSON: &str =
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5/publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts.json"
    ));

/// M5 durable artifact class a row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableArtifactKind {
    /// User-owned local state (settings, profiles, local history).
    UserOwnedLocal,
    /// Workspace-owned state with optional managed mirror.
    WorkspaceOwnedManaged,
    /// AI context, embeddings, and generated memory artifacts.
    AiMemory,
    /// Sync-backed state and device registry.
    SyncState,
    /// Exportable session artifacts.
    SessionExport,
    /// Regenerable caches and indexes.
    DerivedCache,
}

impl M5DurableArtifactKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UserOwnedLocal,
        Self::WorkspaceOwnedManaged,
        Self::AiMemory,
        Self::SyncState,
        Self::SessionExport,
        Self::DerivedCache,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserOwnedLocal => "user_owned_local",
            Self::WorkspaceOwnedManaged => "workspace_owned_managed",
            Self::AiMemory => "ai_memory",
            Self::SyncState => "sync_state",
            Self::SessionExport => "session_export",
            Self::DerivedCache => "derived_cache",
        }
    }
}

/// Kind of posture indicator in an artifact retention posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionPostureIndicatorKind {
    /// Storage posture is defined (location, encryption, tenancy).
    StorageDefined,
    /// Retention policy is defined (trigger, duration, floor, ceiling).
    RetentionDefined,
    /// Export path is defined (format, manifest, self-serve vs admin).
    ExportDefined,
    /// Offboarding path is tested (delete path, completion evidence, legal hold).
    OffboardingTested,
}

impl RetentionPostureIndicatorKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StorageDefined,
        Self::RetentionDefined,
        Self::ExportDefined,
        Self::OffboardingTested,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StorageDefined => "storage_defined",
            Self::RetentionDefined => "retention_defined",
            Self::ExportDefined => "export_defined",
            Self::OffboardingTested => "offboarding_tested",
        }
    }
}

/// State of a retention posture indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionPostureIndicatorState {
    /// Indicator is planned but not yet drafted.
    Planned,
    /// Indicator is drafted but not yet reviewed.
    Drafted,
    /// Indicator has been reviewed.
    Reviewed,
    /// Indicator is published and current.
    Published,
    /// Indicator is published but has gone stale.
    Stale,
    /// Indicator is missing.
    Missing,
}

impl RetentionPostureIndicatorState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Planned,
        Self::Drafted,
        Self::Reviewed,
        Self::Published,
        Self::Stale,
        Self::Missing,
    ];

    /// Stable token recorded in the matrix.
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

    /// Whether the state counts as complete for a posture indicator.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Reviewed | Self::Published)
    }
}

/// Matrix state a row earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactRetentionState {
    /// All posture indicators are complete and the owner is signed off.
    Complete,
    /// One or more required posture indicators are missing or incomplete.
    Incomplete,
    /// A posture indicator has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers
    /// a recorded gap.
    OnWaiver,
    /// Blocked by a missing owner or incomplete owner sign-off.
    OwnerBlocked,
}

impl ArtifactRetentionState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Complete,
        Self::Incomplete,
        Self::Stale,
        Self::OnWaiver,
        Self::OwnerBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::OwnerBlocked => "owner_blocked",
        }
    }

    /// Whether the state lets a row carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Complete | Self::OnWaiver)
    }

    /// Whether the state forces the row below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an artifact retention row narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactRetentionGapReason {
    /// Retention posture is incomplete.
    RetentionPostureIncomplete,
    /// Retention posture is stale.
    RetentionPostureStale,
    /// Proof packet is missing.
    ProofPacketMissing,
    /// Proof packet is stale.
    ProofPacketStale,
    /// Owner sign-off is missing.
    OwnerSignoffMissing,
    /// A required waiver has expired.
    WaiverExpired,
}

impl ArtifactRetentionGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RetentionPostureIncomplete,
        Self::RetentionPostureStale,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetentionPostureIncomplete => "retention_posture_incomplete",
            Self::RetentionPostureStale => "retention_posture_stale",
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
pub enum ArtifactRetentionAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Staff the retention posture work.
    StaffRetentionPolicy,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl ArtifactRetentionAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::StaffRetentionPolicy,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::StaffRetentionPolicy => "staff_retention_policy",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One indicator in an artifact retention posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetentionPostureIndicator {
    /// Stable indicator id.
    pub indicator_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of posture indicator.
    pub indicator_kind: RetentionPostureIndicatorKind,
    /// Ref to the artifact that fulfills this indicator.
    pub artifact_ref: String,
    /// Current state of the indicator.
    pub indicator_state: RetentionPostureIndicatorState,
    /// The owner responsible for this indicator.
    pub owner_ref: String,
    /// Reviewable reason this indicator carries this state.
    pub rationale: String,
}

/// The retention posture for a durable artifact class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRetentionPosture {
    /// Stable posture id.
    pub posture_id: String,
    /// Posture version.
    pub posture_version: u32,
    /// Posture indicators.
    pub indicators: Vec<RetentionPostureIndicator>,
}

impl ArtifactRetentionPosture {
    /// True when every required [`RetentionPostureIndicatorKind`] is present and complete.
    pub fn is_complete(&self) -> bool {
        let present: BTreeSet<RetentionPostureIndicatorKind> =
            self.indicators.iter().map(|e| e.indicator_kind).collect();
        if present.len() != RetentionPostureIndicatorKind::ALL.len() {
            return false;
        }
        self.indicators.iter().all(|e| e.indicator_state.is_complete())
    }

    /// True when every required indicator kind is present.
    pub fn has_all_required_indicators(&self) -> bool {
        let present: BTreeSet<RetentionPostureIndicatorKind> =
            self.indicators.iter().map(|e| e.indicator_kind).collect();
        present.len() == RetentionPostureIndicatorKind::ALL.len()
    }

    /// Returns indicators whose state is stale.
    pub fn stale_indicators(&self) -> Vec<&RetentionPostureIndicator> {
        self.indicators
            .iter()
            .filter(|e| e.indicator_state == RetentionPostureIndicatorState::Stale)
            .collect()
    }

    /// Returns indicators whose state is missing.
    pub fn missing_indicators(&self) -> Vec<&RetentionPostureIndicator> {
        self.indicators
            .iter()
            .filter(|e| e.indicator_state == RetentionPostureIndicatorState::Missing)
            .collect()
    }
}

/// One artifact retention stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRetentionStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: ArtifactRetentionGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: ArtifactRetentionAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 durable artifact retention row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactRetentionRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The artifact kind this row governs.
    pub artifact_kind: M5DurableArtifactKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the row.
    pub surface_summary: String,
    /// Whether the row is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The retention posture for this artifact class.
    pub retention_posture: ArtifactRetentionPosture,
    /// Matrix state earned for the row.
    pub row_state: ArtifactRetentionState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<ArtifactRetentionGapReason>,
    /// The lifecycle label the row effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5ArtifactRetentionRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the row carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.row_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: ArtifactRetentionGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ArtifactRetentionSummary {
    /// Total number of artifact rows.
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
    /// User-owned local rows.
    pub user_owned_local_entries: usize,
    /// Workspace-owned managed rows.
    pub workspace_owned_managed_entries: usize,
    /// AI memory rows.
    pub ai_memory_entries: usize,
    /// Sync state rows.
    pub sync_state_entries: usize,
    /// Session export rows.
    pub session_export_entries: usize,
    /// Derived cache rows.
    pub derived_cache_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Posture indicators whose state is `published`.
    pub indicators_published: usize,
    /// Posture indicators whose state is `stale`.
    pub indicators_stale: usize,
    /// Posture indicators whose state is `missing`.
    pub indicators_missing: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactRetentionExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The artifact kind this row governs.
    pub artifact_kind: M5DurableArtifactKind,
    /// The surface id this row speaks about.
    pub surface_ref: String,
    /// Whether the row is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this row backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Matrix state earned.
    pub row_state: ArtifactRetentionState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<ArtifactRetentionGapReason>,
    /// Owner ref.
    pub owner_ref: String,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactRetentionExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5ArtifactRetentionExportRow>,
}

/// The typed M5 storage, retention, export, and offboarding matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5StorageRetentionMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// Lifecycle status of this matrix artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this matrix ingests.
    pub claim_manifest_ref: String,
    /// Ref to the M5 feature-train matrix this matrix builds on.
    pub feature_train_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed artifact-kind vocabulary.
    pub artifact_kinds: Vec<M5DurableArtifactKind>,
    /// Closed posture-indicator-kind vocabulary.
    pub indicator_kinds: Vec<RetentionPostureIndicatorKind>,
    /// Closed posture-indicator-state vocabulary.
    pub indicator_states: Vec<RetentionPostureIndicatorState>,
    /// Closed row-state vocabulary.
    pub row_states: Vec<ArtifactRetentionState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<ArtifactRetentionGapReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<ArtifactRetentionAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this matrix must cover.
    pub release_blocking_artifact_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<ArtifactRetentionStopRule>,
    /// Artifact rows.
    pub rows: Vec<M5ArtifactRetentionRow>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5ArtifactRetentionSummary,
}

impl M5StorageRetentionMatrix {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5ArtifactRetentionRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5ArtifactRetentionRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5ArtifactRetentionRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5ArtifactRetentionRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one artifact kind.
    pub fn rows_for_kind(&self, kind: M5DurableArtifactKind) -> Vec<&M5ArtifactRetentionRow> {
        self.rows
            .iter()
            .filter(|row| row.artifact_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the matrix covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &ArtifactRetentionStopRule) -> bool {
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

    /// Lane-row ids that trigger a blocking, firing rule, sorted and unique.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<ArtifactRetentionGapReason> = self
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
    pub fn computed_summary(&self) -> M5ArtifactRetentionSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5DurableArtifactKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&M5ArtifactRetentionRow> = self.release_blocking_rows();
        let indicators = |state: RetentionPostureIndicatorState| {
            self.rows
                .iter()
                .flat_map(|row| &row.retention_posture.indicators)
                .filter(|e| e.indicator_state == state)
                .count()
        };
        M5ArtifactRetentionSummary {
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
                .filter(|row| row.row_state == ArtifactRetentionState::OnWaiver)
                .count(),
            entries_owner_blocked: self
                .rows
                .iter()
                .filter(|row| row.row_state == ArtifactRetentionState::OwnerBlocked)
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
            user_owned_local_entries: kind(M5DurableArtifactKind::UserOwnedLocal),
            workspace_owned_managed_entries: kind(M5DurableArtifactKind::WorkspaceOwnedManaged),
            ai_memory_entries: kind(M5DurableArtifactKind::AiMemory),
            sync_state_entries: kind(M5DurableArtifactKind::SyncState),
            session_export_entries: kind(M5DurableArtifactKind::SessionExport),
            derived_cache_entries: kind(M5DurableArtifactKind::DerivedCache),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            indicators_published: indicators(RetentionPostureIndicatorState::Published),
            indicators_stale: indicators(RetentionPostureIndicatorState::Stale),
            indicators_missing: indicators(RetentionPostureIndicatorState::Missing),
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

    /// Produces an export/Help-About-safe projection of the matrix that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5ArtifactRetentionExportProjection {
        M5ArtifactRetentionExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5ArtifactRetentionExportRow {
                    entry_id: row.entry_id.clone(),
                    artifact_kind: row.artifact_kind,
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    row_state: row.row_state,
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    owner_ref: row.owner_signoff.owner_ref.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<M5ArtifactRetentionViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5ArtifactRetentionViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5ArtifactRetentionViolation::EmptyMatrix);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ArtifactRetentionViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ArtifactRetentionViolation>) {
        if self.schema_version
            != PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_SCHEMA_VERSION
        {
            violations.push(M5ArtifactRetentionViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind
            != PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_RECORD_KIND
        {
            violations.push(M5ArtifactRetentionViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("feature_train_matrix_ref", &self.feature_train_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ArtifactRetentionViolation::EmptyField {
                    entry_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.artifact_kinds != M5DurableArtifactKind::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "artifact_kinds",
            });
        }
        if self.indicator_kinds != RetentionPostureIndicatorKind::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "indicator_kinds",
            });
        }
        if self.indicator_states != RetentionPostureIndicatorState::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "indicator_states",
            });
        }
        if self.row_states != ArtifactRetentionState::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "row_states",
            });
        }
        if self.gap_reasons != ArtifactRetentionGapReason::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.stop_rule_actions != ArtifactRetentionAction::ALL.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5ArtifactRetentionViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5ArtifactRetentionViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5ArtifactRetentionViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5ArtifactRetentionViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5ArtifactRetentionViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ArtifactRetentionViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5ArtifactRetentionViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in ArtifactRetentionGapReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5ArtifactRetentionViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5ArtifactRetentionRow,
        violations: &mut Vec<M5ArtifactRetentionViolation>,
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
                violations.push(M5ArtifactRetentionViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no row may carry a label wider than the public claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5ArtifactRetentionViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be a positive number of days and the warn
        // window may not exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5ArtifactRetentionViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5ArtifactRetentionViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must have owner sign-off.
        if row.holds_label() && !row.owner_signoff.signed_off {
            violations.push(M5ArtifactRetentionViolation::HeldWithoutSignoff {
                entry_id: row.entry_id.clone(),
            });
        }

        // A row that holds its label must not have active gap reasons.
        if row.holds_label() && !row.active_gap_reasons.is_empty() {
            violations.push(M5ArtifactRetentionViolation::HeldWithActiveGap {
                entry_id: row.entry_id.clone(),
                reasons: row.active_gap_reasons.clone(),
            });
        }

        // A held row must ride a packet within its freshness SLO.
        let slo_state = row.proof_packet.slo_state;
        if row.holds_label() && !slo_state.is_within_slo() {
            violations.push(M5ArtifactRetentionViolation::HeldOnStalePacket {
                entry_id: row.entry_id.clone(),
                slo_state,
            });
        }

        // A row whose state forces narrowing must actually narrow.
        if row.row_state.forces_narrowing() && row.published_label.rank() >= row.claim_label.rank()
        {
            violations.push(M5ArtifactRetentionViolation::PublishedLabelNotNarrowed {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
                state: row.row_state,
            });
        }

        // Retention posture must contain all required indicator kinds.
        if !row.retention_posture.has_all_required_indicators() {
            violations.push(M5ArtifactRetentionViolation::IncompleteRetentionPosture {
                entry_id: row.entry_id.clone(),
            });
        }

        // A held row must have a complete retention posture.
        if row.holds_label() && !row.retention_posture.is_complete() {
            violations.push(M5ArtifactRetentionViolation::IncompleteRetentionPosture {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5ArtifactRetentionViolation>) {
        let covered_refs: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.surface_ref.clone())
            .collect();
        for surface_ref in &self.release_blocking_artifact_refs {
            if !covered_refs.contains(surface_ref) {
                violations.push(
                    M5ArtifactRetentionViolation::ReleaseBlockingSurfaceUncovered {
                        surface_ref: surface_ref.clone(),
                    },
                );
            }
        }
        let rb_set: BTreeSet<String> =
            self.release_blocking_artifact_refs.iter().cloned().collect();
        for row in &self.rows {
            if row.release_blocking && !rb_set.contains(&row.surface_ref) {
                violations.push(
                    M5ArtifactRetentionViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5ArtifactRetentionViolation>) {
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5ArtifactRetentionViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        let blocking_rules = self.computed_blocking_rule_ids();
        if self.publication.blocking_rule_ids != blocking_rules {
            violations.push(
                M5ArtifactRetentionViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        let blocking_entries = self.computed_blocking_entry_ids();
        if self.publication.blocking_claim_ids != blocking_entries {
            violations.push(
                M5ArtifactRetentionViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// Validation error for the M5 storage/retention/export/offboarding matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ArtifactRetentionViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the matrix.
        actual: u32,
    },
    /// Record kind does not match the expected kind.
    UnsupportedRecordKind {
        /// Record kind found in the matrix.
        actual: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A closed vocabulary field does not match the canonical set.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// No stop rules are defined.
    NoStopRules,
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule watches no labels.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no covering stop rule.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: ArtifactRetentionGapReason,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// The matrix contains no rows.
    EmptyMatrix,
    /// The published label is wider than the canonical claim label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
    /// A row that holds its label lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A row that holds its label has active gap reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
        /// Active gap reasons.
        reasons: Vec<ArtifactRetentionGapReason>,
    },
    /// A held row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A row whose state forces narrowing did not narrow.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
        /// Row state.
        state: ArtifactRetentionState,
    },
    /// A release-blocking surface has no covering row.
    ReleaseBlockingSurfaceUncovered {
        /// Surface ref.
        surface_ref: String,
    },
    /// A release-blocking row is not declared in release_blocking_artifact_refs.
    ReleaseBlockingRowNotDeclared {
        /// Row id.
        entry_id: String,
    },
    /// The declared publication decision disagrees with the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// A publication blocking set field disagrees with firing stop rules.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// Summary counts disagree with row state.
    SummaryMismatch,
    /// Retention posture is missing required indicators.
    IncompleteRetentionPosture {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for M5ArtifactRetentionViolation {
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
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
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
            Self::HeldOnStalePacket { entry_id, slo_state } => {
                write!(
                    f,
                    "row {entry_id} holds stable on stale packet {slo_state:?}"
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
                    "release-blocking row {entry_id} is not declared in release_blocking_artifact_refs"
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
            Self::IncompleteRetentionPosture { entry_id } => {
                write!(f, "row {entry_id} retention posture is incomplete")
            }
        }
    }
}

impl Error for M5ArtifactRetentionViolation {}

/// Loads the embedded M5 storage/retention/export/offboarding matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`M5StorageRetentionMatrix`].
pub fn current_m5_storage_retention_matrix() -> Result<M5StorageRetentionMatrix, serde_json::Error> {
    serde_json::from_str(
        PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_JSON,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> M5StorageRetentionMatrix {
        current_m5_storage_retention_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let m = matrix();
        assert_eq!(
            m.schema_version,
            PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_SCHEMA_VERSION
        );
        assert_eq!(
            m.record_kind,
            PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_RECORD_KIND
        );
        assert_eq!(m.validate(), Vec::new());
        assert!(!m.rows.is_empty());
    }

    #[test]
    fn covers_every_artifact_kind() {
        let m = matrix();
        for kind in M5DurableArtifactKind::ALL {
            assert!(
                !m.rows_for_kind(kind).is_empty(),
                "artifact kind {} must have at least one row",
                kind.as_str()
            );
        }
    }

    #[test]
    fn covers_every_declared_release_blocking_surface() {
        let m = matrix();
        assert!(!m.release_blocking_artifact_refs.is_empty());
        let covered: Vec<&str> = m
            .release_blocking_rows()
            .iter()
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared in &m.release_blocking_artifact_refs {
            assert!(
                covered.contains(&declared.as_str()),
                "{declared} has no covering release-blocking row"
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let m = matrix();
        assert_eq!(m.summary, m.computed_summary());
        assert_eq!(
            m.summary.entries_holding_stable + m.summary.entries_narrowed,
            m.rows.len()
        );
    }

    #[test]
    fn publication_decision_matches_computed() {
        let m = matrix();
        assert_eq!(m.publication.decision, m.computed_publication_decision());
        assert_eq!(
            m.publication.blocking_rule_ids,
            m.computed_blocking_rule_ids()
        );
        assert_eq!(
            m.publication.blocking_claim_ids,
            m.computed_blocking_entry_ids()
        );
    }

    #[test]
    fn every_gap_reason_has_a_stop_rule() {
        let m = matrix();
        let covered: BTreeSet<ArtifactRetentionGapReason> = m
            .stop_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in ArtifactRetentionGapReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_a_held_row_with_active_gap() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.active_gap_reasons
            .push(ArtifactRetentionGapReason::ProofPacketMissing);
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5ArtifactRetentionViolation::HeldWithActiveGap { .. })));
    }

    #[test]
    fn validate_flags_a_narrowing_row_that_does_not_narrow() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.row_state = ArtifactRetentionState::Incomplete;
        row.active_gap_reasons
            .push(ArtifactRetentionGapReason::ProofPacketMissing);
        row.published_label = StableClaimLevel::Stable;
        m.summary = m.computed_summary();
        m.publication.decision = m.computed_publication_decision();
        m.publication.blocking_rule_ids = m.computed_blocking_rule_ids();
        m.publication.blocking_claim_ids = m.computed_blocking_entry_ids();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5ArtifactRetentionViolation::PublishedLabelNotNarrowed { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_row_on_stale_packet() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.proof_packet.slo_state = FreshnessSloState::Breached;
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5ArtifactRetentionViolation::HeldOnStalePacket { .. })));
    }

    #[test]
    fn validate_flags_an_inconsistent_publication_decision() {
        let mut m = matrix();
        m.publication.decision = PromotionDecision::Proceed;
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5ArtifactRetentionViolation::PublicationDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn validate_flags_a_held_claim_without_signoff() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.owner_signoff.signed_off = false;
        row.owner_signoff.signed_at = None;
        m.summary = m.computed_summary();
        assert!(m
            .validate()
            .iter()
            .any(|v| matches!(v, M5ArtifactRetentionViolation::HeldWithoutSignoff { .. })));
    }

    #[test]
    fn validate_flags_an_incomplete_retention_posture() {
        let mut m = matrix();
        let row = m
            .rows
            .iter_mut()
            .find(|row| row.publishes_stable())
            .expect("a held row exists");
        row.retention_posture.indicators.clear();
        m.summary = m.computed_summary();
        assert!(m.validate().iter().any(|v| matches!(
            v,
            M5ArtifactRetentionViolation::IncompleteRetentionPosture { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let m = matrix();
        let projection = m.support_export_projection();
        assert_eq!(projection.rows.len(), m.rows.len());
        for (row, proj) in m.rows.iter().zip(&projection.rows) {
            assert_eq!(row.entry_id, proj.entry_id);
            assert_eq!(row.publishes_stable(), proj.publishes_stable);
        }
    }
}
