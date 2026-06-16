//! Typed M5 qualification-row, support-window, skew-window, and deprecation-packet matrix.
//!
//! This module freezes the canonical compatibility-governance control surface for
//! the M5 stable-facing families and boundaries. Where the depth-claim manifest
//! speaks for the *depth claim* each feature family publishes, this matrix speaks
//! for the *qualification row* every stable-facing family must hold before it may
//! claim support, parity, or certification: a named row across qualification
//! dimensions, a declared skew window with explicit unsupported-skew behavior, a
//! support window, a deprecation packet, and the claim-publication linkage that
//! binds it to a public claim. Each [`QualificationRow`] binds one M5 family to:
//!
//! - the stable claim it backs ([`QualificationRow::claim_ref`],
//!   [`QualificationRow::claim_label`]),
//! - a qualification row ([`QualificationRow::qualification_row`]) of one
//!   [`QualificationCell`] per [`QualificationDimension`] (platform, deployment
//!   profile, archetype/workflow bundle, toolchain envelope, client scope), so
//!   each dimension is an explicit, inspectable truth,
//! - a [`SkewWindow`] naming the supported skew class, version floor/ceiling,
//!   negotiated fields, and the [`SkewUnsupportedBehavior`] (fail-closed, reconnect
//!   required, reinstall required, coordinated-upgrade-only, block-boundary) a peer
//!   outside the window triggers,
//! - a [`SupportWindowSpec`] (support class, supported-since, end-of-support) and a
//!   [`DeprecationPacket`] (status, successor, removal date, migration ref),
//! - the overall row state earned ([`RowState`]), the active narrowing reasons
//!   ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`QualificationRow::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, an owner sign-off, and an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a family that may publish a Stable qualification claim and one that must
//! narrow below it. The [`QualificationStopRule`] set names the closed conditions
//! that gate M5 promotion — one per [`NarrowingReason`] — and
//! [`QualificationSkewMatrix::promotion`] records the proceed/hold verdict.
//!
//! The matrix is checked in at the path named by
//! [`FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_PATH`] and embedded here, so this
//! typed consumer and the CI gate agree on every row without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material.

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
pub const FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_RECORD_KIND: &str =
    "freeze_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix";

/// Repo-relative path to the checked-in matrix.
pub const FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_PATH: &str =
    "artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json";

/// Embedded checked-in matrix JSON.
pub const FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json"
));

/// M5 stable-facing family or boundary a qualification row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyKind {
    /// Notebook and data-rich runtime surfaces.
    Notebook,
    /// Helper/agent/provider boundary.
    AiProvider,
    /// Remote/helper boundary.
    RemoteHelper,
    /// Browser/mobile companion boundary.
    Companion,
    /// Extension/sideload boundary.
    Ecosystem,
    /// Managed sync/relay/registry service boundary.
    ManagedService,
    /// Toolchain/runtime boundary.
    ToolchainRuntime,
}

impl FamilyKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Notebook,
        Self::AiProvider,
        Self::RemoteHelper,
        Self::Companion,
        Self::Ecosystem,
        Self::ManagedService,
        Self::ToolchainRuntime,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::AiProvider => "ai_provider",
            Self::RemoteHelper => "remote_helper",
            Self::Companion => "companion",
            Self::Ecosystem => "ecosystem",
            Self::ManagedService => "managed_service",
            Self::ToolchainRuntime => "toolchain_runtime",
        }
    }
}

/// One column of the per-family qualification row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDimension {
    /// Operating-system and architecture platform coverage.
    Platform,
    /// Deployment profile (local-OSS, self-hosted, managed, air-gapped).
    DeploymentProfile,
    /// Archetype / workflow-bundle coverage.
    ArchetypeBundle,
    /// Toolchain version envelope coverage.
    ToolchainEnvelope,
    /// Client-scope coverage.
    ClientScope,
}

impl QualificationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Platform,
        Self::DeploymentProfile,
        Self::ArchetypeBundle,
        Self::ToolchainEnvelope,
        Self::ClientScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Platform => "platform",
            Self::DeploymentProfile => "deployment_profile",
            Self::ArchetypeBundle => "archetype_bundle",
            Self::ToolchainEnvelope => "toolchain_envelope",
            Self::ClientScope => "client_scope",
        }
    }

    /// The narrowing reason a non-holding cell in this dimension must name, given
    /// the cell's [`QualificationState`]. Holding states return `None`.
    pub const fn reason_for_state(self, state: QualificationState) -> Option<NarrowingReason> {
        match state {
            QualificationState::RetestPending => Some(NarrowingReason::RetestPending),
            QualificationState::Stale => Some(NarrowingReason::QualificationStale),
            QualificationState::Missing => Some(NarrowingReason::QualificationIncomplete),
            QualificationState::Qualified
            | QualificationState::Limited
            | QualificationState::Waived => None,
        }
    }
}

/// The state of one qualification-row cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationState {
    /// The dimension is fully qualified.
    Qualified,
    /// Qualified, but only under a recorded compatibility caveat.
    Limited,
    /// Qualified before, but a change requires a retest.
    RetestPending,
    /// Qualification existed but its evidence has gone stale.
    Stale,
    /// Held provisionally under an active, unexpired waiver.
    Waived,
    /// The dimension has no qualification evidence at all.
    Missing,
}

impl QualificationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Qualified,
        Self::Limited,
        Self::RetestPending,
        Self::Stale,
        Self::Waived,
        Self::Missing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::Stale => "stale",
            Self::Waived => "waived",
            Self::Missing => "missing",
        }
    }

    /// Whether a cell in this state lets the family hold its qualification claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Qualified | Self::Limited | Self::Waived)
    }
}

/// Overall qualification state a row earned for its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowState {
    /// Every dimension is qualified and current.
    Qualified,
    /// Holds the claimed label with a recorded compatibility caveat.
    Limited,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// A dimension or boundary requires a retest before it may re-qualify.
    RetestPending,
    /// A dimension's qualification evidence has gone stale.
    Stale,
    /// A peer is outside the supported skew window.
    UnsupportedSkew,
    /// The family is deprecated or scheduled for removal.
    Deprecated,
    /// One or more required dimensions are incomplete or missing.
    Incomplete,
}

impl RowState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Qualified,
        Self::Limited,
        Self::OnWaiver,
        Self::RetestPending,
        Self::Stale,
        Self::UnsupportedSkew,
        Self::Deprecated,
        Self::Incomplete,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Limited => "limited",
            Self::OnWaiver => "on_waiver",
            Self::RetestPending => "retest_pending",
            Self::Stale => "stale",
            Self::UnsupportedSkew => "unsupported_skew",
            Self::Deprecated => "deprecated",
            Self::Incomplete => "incomplete",
        }
    }

    /// Whether the state lets a family carry the claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Qualified | Self::Limited | Self::OnWaiver)
    }

    /// Whether the state forces the family below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Supported skew class for a cross-binary or cross-service boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkewWindowClass {
    /// Peers must run the identical version; no skew is supported.
    LockstepOnly,
    /// A bounded version skew is supported in both directions.
    BoundedSkew,
    /// Newer peers interoperate with older ones.
    BackwardCompatible,
    /// Older peers interoperate with newer ones.
    ForwardCompatible,
    /// The current skew is outside any supported window.
    UnsupportedSkew,
}

impl SkewWindowClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LockstepOnly,
        Self::BoundedSkew,
        Self::BackwardCompatible,
        Self::ForwardCompatible,
        Self::UnsupportedSkew,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LockstepOnly => "lockstep_only",
            Self::BoundedSkew => "bounded_skew",
            Self::BackwardCompatible => "backward_compatible",
            Self::ForwardCompatible => "forward_compatible",
            Self::UnsupportedSkew => "unsupported_skew",
        }
    }

    /// Whether a boundary in this class is inside a supported skew window.
    pub const fn is_supported(self) -> bool {
        !matches!(self, Self::UnsupportedSkew)
    }
}

/// Behavior a boundary applies when a peer is outside the supported skew window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkewUnsupportedBehavior {
    /// The boundary fails closed rather than operate on unsupported skew.
    FailClosed,
    /// The client must reconnect after upgrading to a supported version.
    ReconnectRequired,
    /// The client must reinstall to reach a supported version.
    ReinstallRequired,
    /// Both ends must upgrade together; no independent upgrade is supported.
    CoordinatedUpgradeOnly,
    /// The boundary is blocked entirely until skew is resolved.
    BlockBoundary,
}

impl SkewUnsupportedBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FailClosed,
        Self::ReconnectRequired,
        Self::ReinstallRequired,
        Self::CoordinatedUpgradeOnly,
        Self::BlockBoundary,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ReconnectRequired => "reconnect_required",
            Self::ReinstallRequired => "reinstall_required",
            Self::CoordinatedUpgradeOnly => "coordinated_upgrade_only",
            Self::BlockBoundary => "block_boundary",
        }
    }
}

/// Support class a family's support window commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Full support: fixes, security, and compatibility.
    FullSupport,
    /// Maintenance only: critical fixes and security.
    MaintenanceOnly,
    /// Security fixes only.
    SecurityOnly,
    /// Limited support, narrowed by a recorded caveat.
    Limited,
    /// The support window has ended.
    EndOfLife,
}

impl SupportClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullSupport,
        Self::MaintenanceOnly,
        Self::SecurityOnly,
        Self::Limited,
        Self::EndOfLife,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSupport => "full_support",
            Self::MaintenanceOnly => "maintenance_only",
            Self::SecurityOnly => "security_only",
            Self::Limited => "limited",
            Self::EndOfLife => "end_of_life",
        }
    }

    /// Whether the support window is still open (not end-of-life).
    pub const fn is_open(self) -> bool {
        !matches!(self, Self::EndOfLife)
    }
}

/// Deprecation status of a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprecationStatus {
    /// Not deprecated.
    Active,
    /// Deprecated, no removal scheduled yet.
    Deprecated,
    /// Deprecated with a named successor available.
    SuccessorAvailable,
    /// Deprecated with a scheduled removal.
    RemovalScheduled,
    /// Removed.
    Removed,
}

impl DeprecationStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Active,
        Self::Deprecated,
        Self::SuccessorAvailable,
        Self::RemovalScheduled,
        Self::Removed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Deprecated => "deprecated",
            Self::SuccessorAvailable => "successor_available",
            Self::RemovalScheduled => "removal_scheduled",
            Self::Removed => "removed",
        }
    }

    /// Whether the status carries a deprecation (anything but active).
    pub const fn is_deprecated(self) -> bool {
        !matches!(self, Self::Active)
    }

    /// Whether the status forces the family below the cutline (removal staged).
    pub const fn forces_narrowing(self) -> bool {
        matches!(self, Self::RemovalScheduled | Self::Removed)
    }
}

/// Closed reason an M5 qualification claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A qualification dimension is incomplete or missing.
    QualificationIncomplete,
    /// A qualification dimension's evidence has gone stale.
    QualificationStale,
    /// A dimension or boundary requires a retest.
    RetestPending,
    /// A peer is outside the supported skew window.
    SkewWindowExceeded,
    /// The family is deprecated with a scheduled removal.
    DeprecationScheduled,
    /// The support window has ended.
    SupportWindowEnded,
    /// A waiver the family relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The backing claim publication is missing.
    ClaimPublicationMissing,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::QualificationIncomplete,
        Self::QualificationStale,
        Self::RetestPending,
        Self::SkewWindowExceeded,
        Self::DeprecationScheduled,
        Self::SupportWindowEnded,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::ClaimPublicationMissing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationIncomplete => "qualification_incomplete",
            Self::QualificationStale => "qualification_stale",
            Self::RetestPending => "retest_pending",
            Self::SkewWindowExceeded => "skew_window_exceeded",
            Self::DeprecationScheduled => "deprecation_scheduled",
            Self::SupportWindowEnded => "support_window_ended",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::ClaimPublicationMissing => "claim_publication_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the qualification claim below the cutline.
    NarrowLabel,
    /// Complete the qualification row.
    CompleteQualification,
    /// Refresh the qualification evidence packet.
    RefreshEvidence,
    /// Retest the boundary.
    RetestBoundary,
    /// Widen or document the supported skew window.
    WidenOrDocumentSkew,
    /// Publish the successor and migration packet.
    PublishSuccessorMigration,
    /// Renew the support window commitment.
    RenewSupportWindow,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Republish the backing claim.
    RepublishClaim,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::CompleteQualification,
        Self::RefreshEvidence,
        Self::RetestBoundary,
        Self::WidenOrDocumentSkew,
        Self::PublishSuccessorMigration,
        Self::RenewSupportWindow,
        Self::RequestOwnerSignoff,
        Self::RepublishClaim,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::CompleteQualification => "complete_qualification",
            Self::RefreshEvidence => "refresh_evidence",
            Self::RetestBoundary => "retest_boundary",
            Self::WidenOrDocumentSkew => "widen_or_document_skew",
            Self::PublishSuccessorMigration => "publish_successor_migration",
            Self::RenewSupportWindow => "renew_support_window",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RepublishClaim => "republish_claim",
        }
    }
}

/// One cell of the per-family qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationCell {
    /// The qualification dimension this cell speaks for.
    pub dimension: QualificationDimension,
    /// The qualification state earned for the dimension.
    pub state: QualificationState,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// The declared skew window for a family's boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkewWindow {
    /// The supported skew class.
    pub skew_window_class: SkewWindowClass,
    /// Inclusive version floor of the supported window.
    pub min_supported_version: String,
    /// Inclusive version ceiling of the supported window.
    pub max_supported_version: String,
    /// Negotiated wire/state fields the boundary exchanges.
    #[serde(default)]
    pub negotiated_fields: Vec<String>,
    /// Behavior applied when a peer is outside the window.
    pub unsupported_behavior: SkewUnsupportedBehavior,
    /// Ref to the reviewer-facing skew-window record.
    pub skew_window_ref: String,
}

/// The declared support window for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportWindowSpec {
    /// The support class committed to.
    pub support_class: SupportClass,
    /// UTC date support started.
    pub supported_since: String,
    /// UTC date support ends, or null when open-ended.
    #[serde(default)]
    pub end_of_support: Option<String>,
    /// Ref to the reviewer-facing support-window record.
    pub support_window_ref: String,
}

/// The deprecation packet for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationPacket {
    /// The deprecation status.
    pub status: DeprecationStatus,
    /// Ref to the successor family, when one exists.
    #[serde(default)]
    pub successor_ref: Option<String>,
    /// UTC date after which the family is removed, when scheduled.
    #[serde(default)]
    pub removal_after: Option<String>,
    /// Ref to the migration packet, when one exists.
    #[serde(default)]
    pub migration_ref: Option<String>,
    /// Ref to the reviewer-facing deprecation packet.
    pub deprecation_packet_ref: String,
}

/// One M5 qualification-claim stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched row fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One M5 qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The family this row governs.
    pub family_kind: FamilyKind,
    /// The family ref this row speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall qualification state earned for the row.
    pub row_state: RowState,
    /// The qualification row: one cell per [`QualificationDimension`].
    pub qualification_row: Vec<QualificationCell>,
    /// The declared skew window.
    pub skew_window: SkewWindow,
    /// The declared support window.
    pub support_window: SupportWindowSpec,
    /// The deprecation packet.
    pub deprecation_packet: DeprecationPacket,
    /// Recorded compatibility caveats. Non-empty when the row is limited.
    #[serde(default)]
    pub compatibility_caveats: Vec<String>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the row below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl QualificationRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the family carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.row_state.holds_label()
    }

    /// True when a narrowing reason is active on the row.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// True when any cell is in a `limited` state.
    pub fn has_limited_cell(&self) -> bool {
        self.qualification_row
            .iter()
            .any(|cell| cell.state == QualificationState::Limited)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: QualificationDimension) -> Option<&QualificationCell> {
        self.qualification_row
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationSkewMatrixSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Distinct families covered.
    pub total_families: usize,
    /// Rows publishing a label at or above the cutline.
    pub rows_qualified: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed: usize,
    /// Rows holding their label via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Rows holding their label with a recorded caveat.
    pub rows_limited: usize,
    /// Rows narrowed because a retest is pending.
    pub rows_retest_pending: usize,
    /// Rows narrowed because evidence is stale.
    pub rows_stale: usize,
    /// Rows narrowed because a peer is outside the skew window.
    pub rows_unsupported_skew: usize,
    /// Rows narrowed because the family is deprecated.
    pub rows_deprecated: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing a label at or above the cutline.
    pub release_blocking_qualified: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook rows.
    pub notebook_rows: usize,
    /// AI/provider rows.
    pub ai_provider_rows: usize,
    /// Remote/helper rows.
    pub remote_helper_rows: usize,
    /// Companion rows.
    pub companion_rows: usize,
    /// Ecosystem rows.
    pub ecosystem_rows: usize,
    /// Managed-service rows.
    pub managed_service_rows: usize,
    /// Toolchain/runtime rows.
    pub toolchain_runtime_rows: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all rows.
    pub total_active_narrowing_reasons: usize,
    /// Total qualification cells across all rows.
    pub total_qualification_cells: usize,
    /// Cells in the `qualified` state.
    pub cells_qualified: usize,
    /// Cells in the `limited` state.
    pub cells_limited: usize,
    /// Cells in the `retest_pending` state.
    pub cells_retest_pending: usize,
    /// Cells in the `stale` state.
    pub cells_stale: usize,
    /// Cells in the `waived` state.
    pub cells_waived: usize,
    /// Cells in the `missing` state.
    pub cells_missing: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationSkewExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The family this row governs.
    pub family_kind: FamilyKind,
    /// The family ref this row speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim entry id this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the row publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall row state earned.
    pub row_state: RowState,
    /// The supported skew class.
    pub skew_window_class: SkewWindowClass,
    /// The support class committed to.
    pub support_class: SupportClass,
    /// The deprecation status.
    pub deprecation_status: DeprecationStatus,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationSkewExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<QualificationSkewExportRow>,
}

/// The typed M5 qualification-row, support-window, skew-window, and
/// deprecation-packet matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationSkewMatrix {
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
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<FamilyKind>,
    /// Closed qualification-dimension vocabulary.
    pub qualification_dimensions: Vec<QualificationDimension>,
    /// Closed qualification-state vocabulary.
    pub qualification_states: Vec<QualificationState>,
    /// Closed row-state vocabulary.
    pub row_states: Vec<RowState>,
    /// Closed skew-window-class vocabulary.
    pub skew_window_classes: Vec<SkewWindowClass>,
    /// Closed unsupported-skew-behavior vocabulary.
    pub skew_unsupported_behaviors: Vec<SkewUnsupportedBehavior>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed deprecation-status vocabulary.
    pub deprecation_statuses: Vec<DeprecationStatus>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this matrix must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<QualificationStopRule>,
    /// Qualification rows.
    pub rows: Vec<QualificationRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: QualificationSkewMatrixSummary,
}

impl QualificationSkewMatrix {
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&QualificationRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&QualificationRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&QualificationRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&QualificationRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one family kind.
    pub fn rows_for_kind(&self, kind: FamilyKind) -> Vec<&QualificationRow> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
            .collect()
    }

    /// Distinct families (by family ref) the matrix covers.
    pub fn families(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.family_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &QualificationStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and stop rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *promotion* blocker, it merely
    /// inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> QualificationSkewMatrixSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: FamilyKind| self.rows_for_kind(kind).len();
        let row_state = |state: RowState| {
            self.rows
                .iter()
                .filter(|row| row.row_state == state)
                .count()
        };
        let cell_state = |state: QualificationState| {
            self.rows
                .iter()
                .flat_map(|row| row.qualification_row.iter())
                .filter(|cell| cell.state == state)
                .count()
        };
        let release_blocking: Vec<&QualificationRow> = self.release_blocking_rows();
        QualificationSkewMatrixSummary {
            total_rows: self.rows.len(),
            total_families: self.families().len(),
            rows_qualified: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            rows_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            rows_on_active_waiver: row_state(RowState::OnWaiver),
            rows_limited: row_state(RowState::Limited),
            rows_retest_pending: row_state(RowState::RetestPending),
            rows_stale: row_state(RowState::Stale),
            rows_unsupported_skew: row_state(RowState::UnsupportedSkew),
            rows_deprecated: row_state(RowState::Deprecated),
            release_blocking_total: release_blocking.len(),
            release_blocking_qualified: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_rows: kind(FamilyKind::Notebook),
            ai_provider_rows: kind(FamilyKind::AiProvider),
            remote_helper_rows: kind(FamilyKind::RemoteHelper),
            companion_rows: kind(FamilyKind::Companion),
            ecosystem_rows: kind(FamilyKind::Ecosystem),
            managed_service_rows: kind(FamilyKind::ManagedService),
            toolchain_runtime_rows: kind(FamilyKind::ToolchainRuntime),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_qualification_cells: self
                .rows
                .iter()
                .map(|row| row.qualification_row.len())
                .sum(),
            cells_qualified: cell_state(QualificationState::Qualified),
            cells_limited: cell_state(QualificationState::Limited),
            cells_retest_pending: cell_state(QualificationState::RetestPending),
            cells_stale: cell_state(QualificationState::Stale),
            cells_waived: cell_state(QualificationState::Waived),
            cells_missing: cell_state(QualificationState::Missing),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> QualificationSkewExportProjection {
        QualificationSkewExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| QualificationSkewExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    family_ref: row.family_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    row_state: row.row_state,
                    skew_window_class: row.skew_window.skew_window_class,
                    support_class: row.support_window.support_class,
                    deprecation_status: row.deprecation_packet.status,
                    slo_state: row.proof_packet.slo_state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<QualificationSkewMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(QualificationSkewMatrixViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(QualificationSkewMatrixViolation::EmptyMatrix);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(QualificationSkewMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<QualificationSkewMatrixViolation>) {
        if self.schema_version != FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_SCHEMA_VERSION {
            violations.push(QualificationSkewMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_RECORD_KIND {
            violations.push(QualificationSkewMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(QualificationSkewMatrixViolation::EmptyField {
                    entry_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != FamilyKind::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.qualification_dimensions != QualificationDimension::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "qualification_dimensions",
            });
        }
        if self.qualification_states != QualificationState::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "qualification_states",
            });
        }
        if self.row_states != RowState::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "row_states",
            });
        }
        if self.skew_window_classes != SkewWindowClass::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "skew_window_classes",
            });
        }
        if self.skew_unsupported_behaviors != SkewUnsupportedBehavior::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "skew_unsupported_behaviors",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.deprecation_statuses != DeprecationStatus::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "deprecation_statuses",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(QualificationSkewMatrixViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(QualificationSkewMatrixViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<QualificationSkewMatrixViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(QualificationSkewMatrixViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(QualificationSkewMatrixViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(QualificationSkewMatrixViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(QualificationSkewMatrixViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    QualificationSkewMatrixViolation::NarrowingReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("family_ref", &row.family_ref),
            ("family_summary", &row.family_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "skew_window.min_supported_version",
                &row.skew_window.min_supported_version,
            ),
            (
                "skew_window.max_supported_version",
                &row.skew_window.max_supported_version,
            ),
            (
                "skew_window.skew_window_ref",
                &row.skew_window.skew_window_ref,
            ),
            (
                "support_window.supported_since",
                &row.support_window.supported_since,
            ),
            (
                "support_window.support_window_ref",
                &row.support_window.support_window_ref,
            ),
            (
                "deprecation_packet.deprecation_packet_ref",
                &row.deprecation_packet.deprecation_packet_ref,
            ),
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
                violations.push(QualificationSkewMatrixViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_qualification_row(row, violations);

        // The ceiling: no family may carry a label wider than the claim's label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(QualificationSkewMatrixViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(QualificationSkewMatrixViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(QualificationSkewMatrixViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_skew_window(row, violations);
        self.validate_support_window(row, violations);
        self.validate_deprecation_packet(row, violations);
        self.validate_caveats(row, violations);

        // A claim whose canonical label is below the cutline forces the family to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(QualificationSkewMatrixViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(QualificationSkewMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.row_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed family carries exactly the claim's canonical label, carries
            // no active reason, rides a captured within-SLO packet, and is
            // owner-signed.
            if row.published_label != row.claim_label {
                violations.push(QualificationSkewMatrixViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(QualificationSkewMatrixViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(QualificationSkewMatrixViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(QualificationSkewMatrixViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(QualificationSkewMatrixViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(
                    QualificationSkewMatrixViolation::PublishedLabelNotNarrowed {
                        entry_id: row.entry_id.clone(),
                        state: row.row_state,
                        published: row.published_label,
                    },
                );
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(QualificationSkewMatrixViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.row_state,
                });
            }
            // A narrowing family whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::QualificationStale)
            {
                violations.push(
                    QualificationSkewMatrixViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::QualificationIncomplete)
            {
                violations.push(
                    QualificationSkewMatrixViolation::MissingPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_qualification_row(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        let mut seen: BTreeSet<QualificationDimension> = BTreeSet::new();
        for cell in &row.qualification_row {
            if !seen.insert(cell.dimension) {
                violations.push(QualificationSkewMatrixViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other state must.
            if cell.state != QualificationState::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(QualificationSkewMatrixViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.state == QualificationState::Waived && row.waiver.is_none() {
                violations.push(QualificationSkewMatrixViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-holding cell must name its narrowing reason.
            if !cell.state.holds() {
                if let Some(reason) = cell.dimension.reason_for_state(cell.state) {
                    if !row.has_active_reason(reason) {
                        violations.push(QualificationSkewMatrixViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The qualification row must carry exactly one cell per dimension.
        for dimension in QualificationDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(
                    QualificationSkewMatrixViolation::QualificationRowIncompleteCoverage {
                        entry_id: row.entry_id.clone(),
                        dimension,
                    },
                );
            }
        }
    }

    fn validate_skew_window(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        // A boundary outside its supported skew window must narrow and name the
        // skew reason.
        if !row.skew_window.skew_window_class.is_supported() {
            if row.holds_label() {
                violations.push(QualificationSkewMatrixViolation::UnsupportedSkewHeld {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.has_active_reason(NarrowingReason::SkewWindowExceeded) {
                violations.push(
                    QualificationSkewMatrixViolation::SkewExceededWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_support_window(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        // A family whose support window has ended must narrow and name the reason.
        if !row.support_window.support_class.is_open() {
            if row.holds_label() {
                violations.push(QualificationSkewMatrixViolation::SupportEndedHeld {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.has_active_reason(NarrowingReason::SupportWindowEnded) {
                violations.push(
                    QualificationSkewMatrixViolation::SupportEndedWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_deprecation_packet(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        let dep = &row.deprecation_packet;
        // A successor-available, removal-scheduled, or removed status must name a
        // successor.
        if matches!(
            dep.status,
            DeprecationStatus::SuccessorAvailable
                | DeprecationStatus::RemovalScheduled
                | DeprecationStatus::Removed
        ) && dep
            .successor_ref
            .as_ref()
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            violations.push(
                QualificationSkewMatrixViolation::DeprecationWithoutSuccessor {
                    entry_id: row.entry_id.clone(),
                    status: dep.status,
                },
            );
        }
        // A removal-scheduled status must name the removal date.
        if dep.status == DeprecationStatus::RemovalScheduled
            && dep
                .removal_after
                .as_ref()
                .map(|s| s.trim().is_empty())
                .unwrap_or(true)
        {
            violations.push(
                QualificationSkewMatrixViolation::RemovalScheduledWithoutDate {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
        // A staged removal must narrow the family and name the deprecation reason.
        if dep.status.forces_narrowing() {
            if row.holds_label() {
                violations.push(QualificationSkewMatrixViolation::DeprecatedHeld {
                    entry_id: row.entry_id.clone(),
                    status: dep.status,
                });
            }
            if !row.has_active_reason(NarrowingReason::DeprecationScheduled) {
                violations.push(QualificationSkewMatrixViolation::DeprecationWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_caveats(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        // A row that holds with a limited dimension or a limited row state must
        // record at least one compatibility caveat.
        let limited = row.row_state == RowState::Limited || row.has_limited_cell();
        if limited
            && row
                .compatibility_caveats
                .iter()
                .all(|c| c.trim().is_empty())
        {
            violations.push(QualificationSkewMatrixViolation::LimitedWithoutCaveat {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<QualificationSkewMatrixViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<QualificationSkewMatrixViolation>,
                               expected: NarrowingReason| {
            violations.push(QualificationSkewMatrixViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.row_state,
                expected_reason: expected,
            });
        };

        match row.row_state {
            RowState::RetestPending => {
                if !row.has_active_reason(NarrowingReason::RetestPending) {
                    push_incoherent(violations, NarrowingReason::RetestPending);
                }
            }
            RowState::Stale => {
                if !row.has_active_reason(NarrowingReason::QualificationStale) {
                    push_incoherent(violations, NarrowingReason::QualificationStale);
                }
            }
            RowState::UnsupportedSkew => {
                if !row.has_active_reason(NarrowingReason::SkewWindowExceeded) {
                    push_incoherent(violations, NarrowingReason::SkewWindowExceeded);
                }
            }
            RowState::Deprecated => {
                if !row.has_active_reason(NarrowingReason::DeprecationScheduled) {
                    push_incoherent(violations, NarrowingReason::DeprecationScheduled);
                }
            }
            RowState::Incomplete => {
                if !row.has_active_reason(NarrowingReason::QualificationIncomplete)
                    && !row.has_active_reason(NarrowingReason::ClaimPublicationMissing)
                    && !row.has_active_reason(NarrowingReason::OwnerSignoffMissing)
                {
                    push_incoherent(violations, NarrowingReason::QualificationIncomplete);
                }
            }
            RowState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(QualificationSkewMatrixViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.row_state,
                    });
                }
            }
            RowState::Limited | RowState::Qualified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<QualificationSkewMatrixViolation>) {
        let covered: BTreeSet<String> =
            self.rows.iter().map(|row| row.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(
                    QualificationSkewMatrixViolation::ReleaseBlockingFamilyUncovered {
                        family_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_family_refs.contains(&row.family_ref)
            {
                violations.push(
                    QualificationSkewMatrixViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<QualificationSkewMatrixViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(QualificationSkewMatrixViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(QualificationSkewMatrixViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                QualificationSkewMatrixViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                QualificationSkewMatrixViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                QualificationSkewMatrixViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 qualification/skew matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualificationSkewMatrixViolation {
    /// The matrix carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the matrix.
        actual: u32,
    },
    /// The matrix carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the matrix.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// The matrix has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no stop rule watching for it.
    NarrowingReasonWithoutStopRule {
        /// Uncovered reason.
        reason: NarrowingReason,
    },
    /// A qualification row has two cells for one dimension.
    DuplicateDimension {
        /// Row id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: QualificationDimension,
    },
    /// A qualification row is missing a dimension cell.
    QualificationRowIncompleteCoverage {
        /// Row id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: QualificationDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Row id.
        entry_id: String,
        /// Dimension.
        dimension: QualificationDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Dimension.
        dimension: QualificationDimension,
    },
    /// A non-holding cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Row id.
        entry_id: String,
        /// Dimension.
        dimension: QualificationDimension,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A row holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RowState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RowState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held row carries active narrowing reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held row has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held row rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a breached proof packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing row with a missing proof packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A boundary outside its skew window holds its label.
    UnsupportedSkewHeld {
        /// Row id.
        entry_id: String,
    },
    /// A boundary outside its skew window does not name the skew reason.
    SkewExceededWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A family whose support window ended holds its label.
    SupportEndedHeld {
        /// Row id.
        entry_id: String,
    },
    /// A family whose support window ended does not name the reason.
    SupportEndedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A deprecated status that requires a successor names none.
    DeprecationWithoutSuccessor {
        /// Row id.
        entry_id: String,
        /// Deprecation status.
        status: DeprecationStatus,
    },
    /// A removal-scheduled status names no removal date.
    RemovalScheduledWithoutDate {
        /// Row id.
        entry_id: String,
    },
    /// A staged removal holds its label.
    DeprecatedHeld {
        /// Row id.
        entry_id: String,
        /// Deprecation status.
        status: DeprecationStatus,
    },
    /// A staged removal does not name the deprecation reason.
    DeprecationWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A limited row records no compatibility caveat.
    LimitedWithoutCaveat {
        /// Row id.
        entry_id: String,
    },
    /// A row state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RowState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Row state.
        state: RowState,
    },
    /// A release-blocking family ref has no covering row.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking row is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Row id.
        entry_id: String,
    },
    /// The declared promotion decision disagrees with the computed one.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared promotion blocking set disagrees with the computed one.
    PromotionBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for QualificationSkewMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported matrix record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "matrix {field} is not the canonical value")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::NoStopRules => write!(f, "matrix has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicateDimension {
                entry_id,
                dimension,
            } => write!(
                f,
                "row {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::QualificationRowIncompleteCoverage {
                entry_id,
                dimension,
            } => write!(
                f,
                "row {entry_id} qualification row is missing dimension {}",
                dimension.as_str()
            ),
            Self::CellEvidenceMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "row {entry_id} dimension {} has no evidence ref",
                dimension.as_str()
            ),
            Self::WaivedCellWithoutWaiver {
                entry_id,
                dimension,
            } => write!(
                f,
                "row {entry_id} dimension {} is waived without a waiver",
                dimension.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                dimension,
                reason,
            } => write!(
                f,
                "row {entry_id} dimension {} requires active reason {}",
                dimension.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "row {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "row {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "row {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "row {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "row {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "row {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "row {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "row {entry_id} holds stable without owner signoff")
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} breached packet without qualification_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} missing packet without qualification_incomplete reason"
                )
            }
            Self::UnsupportedSkewHeld { entry_id } => {
                write!(f, "row {entry_id} holds label on unsupported skew")
            }
            Self::SkewExceededWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} unsupported skew without skew_window_exceeded reason"
                )
            }
            Self::SupportEndedHeld { entry_id } => {
                write!(f, "row {entry_id} holds label on ended support window")
            }
            Self::SupportEndedWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} ended support window without support_window_ended reason"
                )
            }
            Self::DeprecationWithoutSuccessor { entry_id, status } => write!(
                f,
                "row {entry_id} deprecation status {} names no successor",
                status.as_str()
            ),
            Self::RemovalScheduledWithoutDate { entry_id } => {
                write!(f, "row {entry_id} removal scheduled without a removal date")
            }
            Self::DeprecatedHeld { entry_id, status } => write!(
                f,
                "row {entry_id} holds label on deprecation status {}",
                status.as_str()
            ),
            Self::DeprecationWithoutReason { entry_id } => {
                write!(
                    f,
                    "row {entry_id} staged removal without deprecation_scheduled reason"
                )
            }
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(
                    f,
                    "row {entry_id} is limited without a compatibility caveat"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "row {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingFamilyUncovered { family_ref } => {
                write!(
                    f,
                    "release-blocking family {family_ref} has no covering row"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking row {entry_id} is not declared in release_blocking_family_refs"
                )
            }
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "promotion {declared:?} disagrees with computed {computed:?}"
                )
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "row {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for QualificationSkewMatrixViolation {}

/// Loads the embedded M5 qualification/skew matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`QualificationSkewMatrix`].
pub fn current_m5_qualification_and_skew_matrix(
) -> Result<QualificationSkewMatrix, serde_json::Error> {
    serde_json::from_str(FREEZE_M5_QUALIFICATION_AND_SKEW_MATRIX_JSON)
}

#[cfg(test)]
mod tests;
