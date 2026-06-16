//! Typed M5 public-interface diff-report register.
//!
//! Where the M5 qualification/skew matrix freezes the *static* qualification row,
//! support window, and deprecation packet each stable-facing family holds, and the
//! stable version-windows freeze the per-surface version floor/ceiling, this
//! register speaks for the *change*: every stable-facing M5 contract that M5
//! touched gets one [`ContractDiffReport`] binding the contract to:
//!
//! - the [`ContractKind`] it is — a wire/state [`ContractKind::Schema`], a
//!   [`ContractKind::CliHeadlessOutput`], an [`ContractKind::ExportedPacket`], an
//!   [`ContractKind::SdkRuntimeContract`], or a [`ContractKind::CompatibilityBridge`]
//!   — and the [`ChangeClass`] of the change (additive, behavioral, or breaking),
//! - the public-interface [`InterfaceDiff`] — the added, removed, and changed
//!   surface elements plus the reader-side and writer-side compatibility review,
//!   so a producer-side schema update is never treated as sufficient without a
//!   reader/writer review,
//! - the [`CompatibilityWindow`] the contract lives in — its version floor,
//!   current, and ceiling, its [`CompatibilityPosture`], and whether the support
//!   window is still open,
//! - the [`SupportClassCaveat`] it publishes — the [`SupportClass`] and the
//!   caveats that narrow the marketed claim,
//! - the successor/[`DeprecationPacket`] that governs how a deprecated contract
//!   leaves the window — its [`DeprecationStatus`], owner, successor (replacement
//!   path), alias map, removal checkpoint and horizon, migration, and rollback
//!   implications,
//! - the stable claim it backs ([`ContractDiffReport::claim_ref`],
//!   [`ContractDiffReport::claim_label`]), the overall [`ReportState`] earned, the
//!   active [`NarrowingReason`] set, and the effective label after narrowing
//!   ([`ContractDiffReport::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, an owner sign-off, and an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a contract that may publish a Stable support claim and one that must
//! narrow below it. The [`DiffReportStopRule`] set names the closed conditions that
//! gate M5 promotion — one per [`NarrowingReason`] — and the register records the
//! proceed/hold verdict.
//!
//! The register is checked in at the path named by
//! [`M5_PUBLIC_INTERFACE_DIFF_REPORTS_PATH`] and embedded here, so this typed
//! consumer and the CI gate agree on every report without a cargo build in CI.
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

/// Supported register schema version.
pub const M5_PUBLIC_INTERFACE_DIFF_REPORTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_PUBLIC_INTERFACE_DIFF_REPORTS_RECORD_KIND: &str =
    "implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts";

/// Repo-relative path to the checked-in register.
pub const M5_PUBLIC_INTERFACE_DIFF_REPORTS_PATH: &str =
    "artifacts/release/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.json";

/// Embedded checked-in register JSON.
pub const M5_PUBLIC_INTERFACE_DIFF_REPORTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.json"
));

/// The stable-facing M5 contract kind a diff report speaks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractKind {
    /// A wire or persisted-state schema.
    Schema,
    /// A CLI or headless command output contract.
    CliHeadlessOutput,
    /// An exported truth/support packet contract.
    ExportedPacket,
    /// An SDK or runtime contract exposed to clients.
    SdkRuntimeContract,
    /// A mixed-version compatibility bridge.
    CompatibilityBridge,
}

impl ContractKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Schema,
        Self::CliHeadlessOutput,
        Self::ExportedPacket,
        Self::SdkRuntimeContract,
        Self::CompatibilityBridge,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::CliHeadlessOutput => "cli_headless_output",
            Self::ExportedPacket => "exported_packet",
            Self::SdkRuntimeContract => "sdk_runtime_contract",
            Self::CompatibilityBridge => "compatibility_bridge",
        }
    }
}

/// The classification of the change a diff report describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeClass {
    /// Backward-compatible additions only.
    Additive,
    /// Changed behavior over a compatible surface.
    Behavioral,
    /// A removed or incompatible surface element.
    Breaking,
}

impl ChangeClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 3] = [Self::Additive, Self::Behavioral, Self::Breaking];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Additive => "additive",
            Self::Behavioral => "behavioral",
            Self::Breaking => "breaking",
        }
    }

    /// Whether a change in this class must be governed by a deprecation packet.
    pub const fn requires_deprecation_packet(self) -> bool {
        matches!(self, Self::Breaking)
    }
}

/// Compatibility posture of the changed contract's version window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityPosture {
    /// Compatible in both directions.
    FullyCompatible,
    /// Newer readers interoperate with older producers.
    BackwardCompatible,
    /// Older readers interoperate with newer producers.
    ForwardCompatible,
    /// The change breaks the wire/behavioral contract.
    Breaking,
}

impl CompatibilityPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyCompatible,
        Self::BackwardCompatible,
        Self::ForwardCompatible,
        Self::Breaking,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyCompatible => "fully_compatible",
            Self::BackwardCompatible => "backward_compatible",
            Self::ForwardCompatible => "forward_compatible",
            Self::Breaking => "breaking",
        }
    }
}

/// Whether the contract's compatibility/support window is open or closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowSupportState {
    /// The contract is inside its supported compatibility window.
    WithinWindow,
    /// The supported window for the changed contract has ended.
    SupportEnded,
}

impl WindowSupportState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 2] = [Self::WithinWindow, Self::SupportEnded];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinWindow => "within_window",
            Self::SupportEnded => "support_ended",
        }
    }

    /// Whether the support window is still open.
    pub const fn is_open(self) -> bool {
        matches!(self, Self::WithinWindow)
    }
}

/// Reader-side or writer-side compatibility review posture for the diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPosture {
    /// The reviewer confirmed the change is compatible on this side.
    Compatible,
    /// The reviewer confirmed the change is breaking on this side.
    Breaking,
    /// No reader/writer compatibility review has been done on this side.
    Unreviewed,
}

impl ReviewPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [Self::Compatible, Self::Breaking, Self::Unreviewed];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Breaking => "breaking",
            Self::Unreviewed => "unreviewed",
        }
    }

    /// Whether this side has been reviewed at all.
    pub const fn is_reviewed(self) -> bool {
        !matches!(self, Self::Unreviewed)
    }

    /// Whether the review found a breaking change on this side.
    pub const fn is_breaking(self) -> bool {
        matches!(self, Self::Breaking)
    }
}

/// The support class a changed contract publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Fully supported with no caveats.
    FullySupported,
    /// Supported, but with recorded caveats a consumer must heed.
    SupportedWithCaveats,
    /// Limited support; the contract is narrowed.
    Limited,
    /// Unsupported; the contract is below any support claim.
    Unsupported,
}

impl SupportClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullySupported,
        Self::SupportedWithCaveats,
        Self::Limited,
        Self::Unsupported,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::SupportedWithCaveats => "supported_with_caveats",
            Self::Limited => "limited",
            Self::Unsupported => "unsupported",
        }
    }

    /// Whether this class must record at least one caveat.
    pub const fn requires_caveat(self) -> bool {
        matches!(self, Self::SupportedWithCaveats)
    }
}

/// Lifecycle status of a deprecated contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprecationStatus {
    /// Marked deprecated; still served.
    Deprecated,
    /// Superseded by a successor; still served during the window.
    Superseded,
    /// A removal is scheduled at a named checkpoint.
    RemovalScheduled,
    /// The contract has been removed.
    Removed,
}

impl DeprecationStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Deprecated,
        Self::Superseded,
        Self::RemovalScheduled,
        Self::Removed,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deprecated => "deprecated",
            Self::Superseded => "superseded",
            Self::RemovalScheduled => "removal_scheduled",
            Self::Removed => "removed",
        }
    }
}

/// Overall state a diff report earned for its claimed support label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportState {
    /// The diff report is current and the contract holds its claim cleanly.
    Published,
    /// Holds the claim with a recorded compatibility caveat.
    Limited,
    /// Holds the claim provisionally under an active, unexpired waiver.
    OnWaiver,
    /// A breaking change carries no deprecation packet.
    BreakingUnpacketed,
    /// A deprecation packet is present but incomplete.
    DeprecationIncomplete,
    /// The reader/writer compatibility review is not complete.
    CompatReviewPending,
    /// A deprecation packet's removal checkpoint is overdue.
    RemovalOverdue,
    /// The contract's compatibility/support window has ended.
    SupportWindowEnded,
    /// The report's proof packet went stale.
    EvidenceStale,
    /// Report evidence, owner sign-off, or claim publication is incomplete.
    Incomplete,
}

impl ReportState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Published,
        Self::Limited,
        Self::OnWaiver,
        Self::BreakingUnpacketed,
        Self::DeprecationIncomplete,
        Self::CompatReviewPending,
        Self::RemovalOverdue,
        Self::SupportWindowEnded,
        Self::EvidenceStale,
        Self::Incomplete,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Limited => "limited",
            Self::OnWaiver => "on_waiver",
            Self::BreakingUnpacketed => "breaking_unpacketed",
            Self::DeprecationIncomplete => "deprecation_incomplete",
            Self::CompatReviewPending => "compat_review_pending",
            Self::RemovalOverdue => "removal_overdue",
            Self::SupportWindowEnded => "support_window_ended",
            Self::EvidenceStale => "evidence_stale",
            Self::Incomplete => "incomplete",
        }
    }

    /// Whether the state lets the report carry its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published | Self::Limited | Self::OnWaiver)
    }
}

/// Closed reason a report's support claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A breaking change carries no deprecation packet.
    BreakingChangeUnpacketed,
    /// The reader/writer compatibility review is missing.
    ReaderWriterReviewMissing,
    /// The deprecation packet is incomplete.
    DeprecationPacketIncomplete,
    /// A deprecation packet's removal checkpoint is overdue.
    RemovalOverdue,
    /// The contract's compatibility/support window has ended.
    SupportWindowEnded,
    /// The report's proof packet has gone stale.
    EvidenceStale,
    /// The report has no captured proof packet.
    EvidenceMissing,
    /// A waiver the report relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The backing claim publication is missing.
    ClaimPublicationMissing,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::BreakingChangeUnpacketed,
        Self::ReaderWriterReviewMissing,
        Self::DeprecationPacketIncomplete,
        Self::RemovalOverdue,
        Self::SupportWindowEnded,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::ClaimPublicationMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BreakingChangeUnpacketed => "breaking_change_unpacketed",
            Self::ReaderWriterReviewMissing => "reader_writer_review_missing",
            Self::DeprecationPacketIncomplete => "deprecation_packet_incomplete",
            Self::RemovalOverdue => "removal_overdue",
            Self::SupportWindowEnded => "support_window_ended",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
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
    /// Publish the missing deprecation packet for the breaking change.
    PublishDeprecationPacket,
    /// Complete the reader/writer compatibility review.
    CompleteCompatReview,
    /// Complete the incomplete deprecation packet.
    CompleteDeprecationPacket,
    /// Execute the overdue removal or extend the checkpoint.
    ExecuteOrExtendRemoval,
    /// Extend or formally close the compatibility window.
    ExtendOrCloseWindow,
    /// Refresh the diff-report evidence packet.
    RefreshEvidence,
    /// Capture the diff-report evidence packet.
    CaptureEvidence,
    /// Narrow the support claim below the cutline.
    NarrowLabel,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Republish the backing claim.
    RepublishClaim,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::PublishDeprecationPacket,
        Self::CompleteCompatReview,
        Self::CompleteDeprecationPacket,
        Self::ExecuteOrExtendRemoval,
        Self::ExtendOrCloseWindow,
        Self::RefreshEvidence,
        Self::CaptureEvidence,
        Self::NarrowLabel,
        Self::RequestOwnerSignoff,
        Self::RepublishClaim,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishDeprecationPacket => "publish_deprecation_packet",
            Self::CompleteCompatReview => "complete_compat_review",
            Self::CompleteDeprecationPacket => "complete_deprecation_packet",
            Self::ExecuteOrExtendRemoval => "execute_or_extend_removal",
            Self::ExtendOrCloseWindow => "extend_or_close_window",
            Self::RefreshEvidence => "refresh_evidence",
            Self::CaptureEvidence => "capture_evidence",
            Self::NarrowLabel => "narrow_label",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RepublishClaim => "republish_claim",
        }
    }
}

/// The public-interface diff a report carries for a changed contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceDiff {
    /// Surface elements added by the change.
    #[serde(default)]
    pub added: Vec<String>,
    /// Surface elements removed by the change.
    #[serde(default)]
    pub removed: Vec<String>,
    /// Surface elements whose behavior or shape changed.
    #[serde(default)]
    pub changed: Vec<String>,
    /// The reader-side compatibility review posture.
    pub reader_posture: ReviewPosture,
    /// The writer-side compatibility review posture.
    pub writer_posture: ReviewPosture,
    /// Ref to the reviewer-facing public-interface diff.
    pub diff_ref: String,
}

impl InterfaceDiff {
    /// True when both the reader and writer sides have been reviewed.
    pub fn is_reviewed(&self) -> bool {
        self.reader_posture.is_reviewed() && self.writer_posture.is_reviewed()
    }

    /// True when either side's review found a breaking change.
    pub fn found_breaking(&self) -> bool {
        self.reader_posture.is_breaking() || self.writer_posture.is_breaking()
    }

    /// True when the diff shows a removed or changed surface element.
    pub fn has_incompatible_surface(&self) -> bool {
        !self.removed.is_empty() || !self.changed.is_empty()
    }
}

/// The compatibility window a changed contract lives in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityWindow {
    /// The compatibility posture.
    pub posture: CompatibilityPosture,
    /// Inclusive version floor of the supported window.
    pub min_supported_version: String,
    /// The current version of the contract.
    pub current_version: String,
    /// Inclusive version ceiling of the supported window.
    pub max_supported_version: String,
    /// Whether the support window is open or has ended.
    pub support_state: WindowSupportState,
    /// Ref to the reviewer-facing compatibility-window record.
    pub window_ref: String,
}

/// The support-class caveat a changed contract publishes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportClassCaveat {
    /// The support class.
    pub support_class: SupportClass,
    /// Recorded caveats. Non-empty when the class is supported-with-caveats.
    #[serde(default)]
    pub caveats: Vec<String>,
}

/// One old→new contract-name alias entry in a deprecation packet's alias map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractAlias {
    /// The old contract surface name.
    pub from: String,
    /// The replacement contract surface name.
    pub to: String,
}

/// The successor/deprecation packet that governs how a contract leaves the window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeprecationPacket {
    /// Lifecycle status of the deprecated contract.
    pub status: DeprecationStatus,
    /// Owning team or role responsible for the deprecation.
    pub owner_ref: String,
    /// The successor (replacement path) contract ref, when named.
    #[serde(default)]
    pub successor_ref: Option<String>,
    /// Old→new contract-name aliases.
    #[serde(default)]
    pub alias_map: Vec<ContractAlias>,
    /// The named removal checkpoint, when set.
    #[serde(default)]
    pub removal_checkpoint: Option<String>,
    /// The removal horizon (UTC date), when set.
    #[serde(default)]
    pub removal_date: Option<String>,
    /// Whether the removal checkpoint has passed.
    pub removal_overdue: bool,
    /// Ref to the migration guide, when set.
    #[serde(default)]
    pub migration_ref: Option<String>,
    /// Reviewable rollback implications, when stated.
    #[serde(default)]
    pub rollback_implications: Option<String>,
    /// Ref to the reviewer-facing deprecation packet.
    pub packet_ref: String,
}

impl DeprecationPacket {
    /// True when the packet names an owner, successor, removal checkpoint and
    /// horizon, migration guide, and rollback implications — the closed set the
    /// guardrail requires before a contract may be deprecated.
    pub fn is_complete(&self) -> bool {
        let present =
            |value: &Option<String>| value.as_deref().is_some_and(|s| !s.trim().is_empty());
        !self.owner_ref.trim().is_empty()
            && present(&self.successor_ref)
            && present(&self.removal_checkpoint)
            && present(&self.removal_date)
            && present(&self.migration_ref)
            && present(&self.rollback_implications)
    }
}

/// One diff-report stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffReportStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched report fires this rule.
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

/// One M5 public-interface diff report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractDiffReport {
    /// Stable report id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The kind of stable-facing contract this report speaks for.
    pub contract_kind: ContractKind,
    /// The contract ref this report speaks about.
    pub contract_ref: String,
    /// Reviewable one-line statement of the contract.
    pub contract_summary: String,
    /// Whether the contract is part of the release-blocking set.
    pub release_blocking: bool,
    /// The classification of the change.
    pub change_class: ChangeClass,
    /// The stable-claim entry id whose claim this contract backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall report state earned for the row.
    pub report_state: ReportState,
    /// The public-interface diff.
    pub interface_diff: InterfaceDiff,
    /// The compatibility window the contract lives in.
    pub compatibility_window: CompatibilityWindow,
    /// The support-class caveat the contract publishes.
    pub support_caveat: SupportClassCaveat,
    /// The successor/deprecation packet, when the contract is being deprecated.
    #[serde(default)]
    pub deprecation_packet: Option<DeprecationPacket>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the report below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the contract effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this report's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the report carries this posture.
    pub rationale: String,
}

/// The deprecation gap a report carries, derived from its change and packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeprecationGap {
    /// No deprecation gap.
    None,
    /// A breaking change carries no deprecation packet.
    Unpacketed,
    /// A deprecation packet is present but incomplete.
    Incomplete,
    /// A deprecation packet's removal checkpoint is overdue.
    Overdue,
}

impl ContractDiffReport {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the report's state lets the contract carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.report_state.holds_label()
    }

    /// True when a narrowing reason is active on the report.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// True when the contract is being deprecated (carries a deprecation packet).
    pub fn is_deprecated(&self) -> bool {
        self.deprecation_packet.is_some()
    }

    /// The deprecation gap implied by the change class and the packet.
    fn deprecation_gap(&self) -> DeprecationGap {
        match &self.deprecation_packet {
            None => {
                if self.change_class.requires_deprecation_packet() {
                    DeprecationGap::Unpacketed
                } else {
                    DeprecationGap::None
                }
            }
            Some(packet) => {
                if !packet.is_complete() {
                    DeprecationGap::Incomplete
                } else if packet.removal_overdue {
                    DeprecationGap::Overdue
                } else {
                    DeprecationGap::None
                }
            }
        }
    }

    /// True when the reader/writer compatibility review is missing on either side.
    fn review_missing(&self) -> bool {
        !self.interface_diff.is_reviewed()
    }

    /// True when the compatibility/support window has ended.
    fn window_ended(&self) -> bool {
        !self.compatibility_window.support_state.is_open()
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractDiffReportRegisterSummary {
    /// Total number of reports.
    pub total_reports: usize,
    /// Distinct contracts covered.
    pub total_contracts: usize,
    /// Reports publishing a label at or above the cutline.
    pub reports_publishing_stable: usize,
    /// Reports narrowed below the cutline.
    pub reports_narrowed: usize,
    /// Reports in a label-holding state.
    pub reports_holding: usize,
    /// Reports in the published state.
    pub reports_published: usize,
    /// Reports holding their label with a recorded caveat.
    pub reports_limited: usize,
    /// Reports holding their label via an active waiver.
    pub reports_on_waiver: usize,
    /// Reports narrowed because a breaking change is unpacketed.
    pub reports_breaking_unpacketed: usize,
    /// Reports narrowed because the deprecation packet is incomplete.
    pub reports_deprecation_incomplete: usize,
    /// Reports narrowed because the reader/writer review is pending.
    pub reports_compat_review_pending: usize,
    /// Reports narrowed because a removal checkpoint is overdue.
    pub reports_removal_overdue: usize,
    /// Reports narrowed because the compatibility window ended.
    pub reports_support_window_ended: usize,
    /// Reports narrowed because evidence is stale.
    pub reports_evidence_stale: usize,
    /// Reports narrowed because evidence or sign-off is incomplete.
    pub reports_incomplete: usize,
    /// Total release-blocking reports.
    pub release_blocking_total: usize,
    /// Release-blocking reports publishing a label at or above the cutline.
    pub release_blocking_publishing_stable: usize,
    /// Release-blocking reports narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Schema-contract reports.
    pub schema_reports: usize,
    /// CLI/headless-output reports.
    pub cli_headless_output_reports: usize,
    /// Exported-packet reports.
    pub exported_packet_reports: usize,
    /// SDK/runtime-contract reports.
    pub sdk_runtime_contract_reports: usize,
    /// Compatibility-bridge reports.
    pub compatibility_bridge_reports: usize,
    /// Additive-change reports.
    pub additive_changes: usize,
    /// Behavioral-change reports.
    pub behavioral_changes: usize,
    /// Breaking-change reports.
    pub breaking_changes: usize,
    /// Reports carrying a deprecation packet.
    pub reports_with_deprecation_packet: usize,
    /// Reports whose deprecation packet is complete.
    pub complete_deprecation_packets: usize,
    /// Reports publishing the fully-supported class.
    pub support_fully_supported: usize,
    /// Reports publishing the supported-with-caveats class.
    pub support_supported_with_caveats: usize,
    /// Reports publishing the limited class.
    pub support_limited: usize,
    /// Reports publishing the unsupported class.
    pub support_unsupported: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total added surface elements across all reports.
    pub total_added_elements: usize,
    /// Total removed surface elements across all reports.
    pub total_removed_elements: usize,
    /// Total changed surface elements across all reports.
    pub total_changed_elements: usize,
    /// Total active narrowing reasons across all reports.
    pub total_active_narrowing_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractDiffReportExportRow {
    /// Stable report id.
    pub entry_id: String,
    /// The kind of contract this report speaks for.
    pub contract_kind: ContractKind,
    /// The contract ref this report speaks about.
    pub contract_ref: String,
    /// The classification of the change.
    pub change_class: ChangeClass,
    /// Whether the contract is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim entry id this contract backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the report publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall report state earned.
    pub report_state: ReportState,
    /// The compatibility posture.
    pub compatibility_posture: CompatibilityPosture,
    /// Whether the support window is open.
    pub support_state: WindowSupportState,
    /// The support class published.
    pub support_class: SupportClass,
    /// The deprecation status, when the contract is being deprecated.
    pub deprecation_status: Option<DeprecationStatus>,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, release-center, service-health, support,
/// upgrade-notes, and export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractDiffReportExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<ContractDiffReportExportRow>,
}

/// The typed M5 public-interface diff-report register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractDiffReportRegister {
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
    /// Ref to the stable version-windows freeze this register reuses.
    pub version_windows_ref: String,
    /// Ref to the qualification/skew matrix whose vocabulary this register reuses.
    pub qualification_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed contract-kind vocabulary.
    pub contract_kinds: Vec<ContractKind>,
    /// Closed change-class vocabulary.
    pub change_classes: Vec<ChangeClass>,
    /// Closed compatibility-posture vocabulary.
    pub compatibility_postures: Vec<CompatibilityPosture>,
    /// Closed window-support-state vocabulary.
    pub window_support_states: Vec<WindowSupportState>,
    /// Closed review-posture vocabulary.
    pub review_postures: Vec<ReviewPosture>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed deprecation-status vocabulary.
    pub deprecation_statuses: Vec<DeprecationStatus>,
    /// Closed report-state vocabulary.
    pub report_states: Vec<ReportState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking contract refs this register must cover.
    pub release_blocking_contract_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<DiffReportStopRule>,
    /// Diff reports.
    pub reports: Vec<ContractDiffReport>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: ContractDiffReportRegisterSummary,
}

impl ContractDiffReportRegister {
    /// Returns the report registered for `entry_id`.
    pub fn report(&self, entry_id: &str) -> Option<&ContractDiffReport> {
        self.reports.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the reports publishing a label at or above the cutline.
    pub fn reports_publishing_stable(&self) -> Vec<&ContractDiffReport> {
        self.reports
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the reports narrowed below the cutline.
    pub fn reports_narrowed(&self) -> Vec<&ContractDiffReport> {
        self.reports
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking reports.
    pub fn release_blocking_reports(&self) -> Vec<&ContractDiffReport> {
        self.reports
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the reports for one contract kind.
    pub fn reports_for_kind(&self, kind: ContractKind) -> Vec<&ContractDiffReport> {
        self.reports
            .iter()
            .filter(|row| row.contract_kind == kind)
            .collect()
    }

    /// Returns the reports for one change class.
    pub fn reports_for_change_class(&self, class: ChangeClass) -> Vec<&ContractDiffReport> {
        self.reports
            .iter()
            .filter(|row| row.change_class == class)
            .collect()
    }

    /// Distinct contracts (by contract ref) the register covers.
    pub fn contracts(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.reports {
            set.insert(row.contract_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched report carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &DiffReportStopRule) -> bool {
        self.reports.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the reports and stop rules.
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

    /// Report ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only reports whose claim is at or above the cutline count: a report whose
    /// claim is already canonically narrowed is not a *promotion* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.reports {
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

    /// Recomputes the summary block from the reports and stop rules.
    pub fn computed_summary(&self) -> ContractDiffReportRegisterSummary {
        let packets = |state: FreshnessSloState| {
            self.reports
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: ContractKind| self.reports_for_kind(kind).len();
        let change = |class: ChangeClass| self.reports_for_change_class(class).len();
        let support = |class: SupportClass| {
            self.reports
                .iter()
                .filter(|row| row.support_caveat.support_class == class)
                .count()
        };
        let state = |state: ReportState| {
            self.reports
                .iter()
                .filter(|row| row.report_state == state)
                .count()
        };
        let with_packet: Vec<&ContractDiffReport> = self
            .reports
            .iter()
            .filter(|row| row.is_deprecated())
            .collect();
        let release_blocking: Vec<&ContractDiffReport> = self.release_blocking_reports();
        ContractDiffReportRegisterSummary {
            total_reports: self.reports.len(),
            total_contracts: self.contracts().len(),
            reports_publishing_stable: self.reports_publishing_stable().len(),
            reports_narrowed: self.reports_narrowed().len(),
            reports_holding: self.reports.iter().filter(|row| row.holds_label()).count(),
            reports_published: state(ReportState::Published),
            reports_limited: state(ReportState::Limited),
            reports_on_waiver: state(ReportState::OnWaiver),
            reports_breaking_unpacketed: state(ReportState::BreakingUnpacketed),
            reports_deprecation_incomplete: state(ReportState::DeprecationIncomplete),
            reports_compat_review_pending: state(ReportState::CompatReviewPending),
            reports_removal_overdue: state(ReportState::RemovalOverdue),
            reports_support_window_ended: state(ReportState::SupportWindowEnded),
            reports_evidence_stale: state(ReportState::EvidenceStale),
            reports_incomplete: state(ReportState::Incomplete),
            release_blocking_total: release_blocking.len(),
            release_blocking_publishing_stable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            schema_reports: kind(ContractKind::Schema),
            cli_headless_output_reports: kind(ContractKind::CliHeadlessOutput),
            exported_packet_reports: kind(ContractKind::ExportedPacket),
            sdk_runtime_contract_reports: kind(ContractKind::SdkRuntimeContract),
            compatibility_bridge_reports: kind(ContractKind::CompatibilityBridge),
            additive_changes: change(ChangeClass::Additive),
            behavioral_changes: change(ChangeClass::Behavioral),
            breaking_changes: change(ChangeClass::Breaking),
            reports_with_deprecation_packet: with_packet.len(),
            complete_deprecation_packets: with_packet
                .iter()
                .filter(|row| {
                    row.deprecation_packet
                        .as_ref()
                        .is_some_and(DeprecationPacket::is_complete)
                })
                .count(),
            support_fully_supported: support(SupportClass::FullySupported),
            support_supported_with_caveats: support(SupportClass::SupportedWithCaveats),
            support_limited: support(SupportClass::Limited),
            support_unsupported: support(SupportClass::Unsupported),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_added_elements: self
                .reports
                .iter()
                .map(|row| row.interface_diff.added.len())
                .sum(),
            total_removed_elements: self
                .reports
                .iter()
                .map(|row| row.interface_diff.removed.len())
                .sum(),
            total_changed_elements: self
                .reports
                .iter()
                .map(|row| row.interface_diff.changed.len())
                .sum(),
            total_active_narrowing_reasons: self
                .reports
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> ContractDiffReportExportProjection {
        ContractDiffReportExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .reports
                .iter()
                .map(|row| ContractDiffReportExportRow {
                    entry_id: row.entry_id.clone(),
                    contract_kind: row.contract_kind,
                    contract_ref: row.contract_ref.clone(),
                    change_class: row.change_class,
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    report_state: row.report_state,
                    compatibility_posture: row.compatibility_window.posture,
                    support_state: row.compatibility_window.support_state,
                    support_class: row.support_caveat.support_class,
                    deprecation_status: row.deprecation_packet.as_ref().map(|p| p.status),
                    slo_state: row.proof_packet.slo_state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<ContractDiffReportViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.reports {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(ContractDiffReportViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_report(row, &mut violations);
        }
        if self.reports.is_empty() {
            violations.push(ContractDiffReportViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ContractDiffReportViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ContractDiffReportViolation>) {
        if self.schema_version != M5_PUBLIC_INTERFACE_DIFF_REPORTS_SCHEMA_VERSION {
            violations.push(ContractDiffReportViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PUBLIC_INTERFACE_DIFF_REPORTS_RECORD_KIND {
            violations.push(ContractDiffReportViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("version_windows_ref", &self.version_windows_ref),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(ContractDiffReportViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.contract_kinds != ContractKind::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "contract_kinds",
            });
        }
        if self.change_classes != ChangeClass::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "change_classes",
            });
        }
        if self.compatibility_postures != CompatibilityPosture::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "compatibility_postures",
            });
        }
        if self.window_support_states != WindowSupportState::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "window_support_states",
            });
        }
        if self.review_postures != ReviewPosture::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "review_postures",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.deprecation_statuses != DeprecationStatus::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "deprecation_statuses",
            });
        }
        if self.report_states != ReportState::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "report_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(ContractDiffReportViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(ContractDiffReportViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<ContractDiffReportViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(ContractDiffReportViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(ContractDiffReportViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(ContractDiffReportViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(ContractDiffReportViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(ContractDiffReportViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_report(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("contract_ref", &row.contract_ref),
            ("contract_summary", &row.contract_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("interface_diff.diff_ref", &row.interface_diff.diff_ref),
            (
                "compatibility_window.min_supported_version",
                &row.compatibility_window.min_supported_version,
            ),
            (
                "compatibility_window.current_version",
                &row.compatibility_window.current_version,
            ),
            (
                "compatibility_window.max_supported_version",
                &row.compatibility_window.max_supported_version,
            ),
            (
                "compatibility_window.window_ref",
                &row.compatibility_window.window_ref,
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
                violations.push(ContractDiffReportViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The ceiling: no contract may publish a label wider than the claim's label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(ContractDiffReportViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(ContractDiffReportViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(ContractDiffReportViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_diff_class(row, violations);
        self.validate_deprecation(row, violations);
        self.validate_review(row, violations);
        self.validate_window(row, violations);
        self.validate_support(row, violations);

        // A claim whose canonical label is below the cutline forces the contract to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(ContractDiffReportViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(ContractDiffReportViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.report_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed contract carries the claim's canonical label, carries no
            // active reason, rides a captured within-SLO packet, and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(ContractDiffReportViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(ContractDiffReportViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(ContractDiffReportViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(ContractDiffReportViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(ContractDiffReportViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(ContractDiffReportViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.report_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(ContractDiffReportViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.report_state,
                });
            }
            // A narrowing contract whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::EvidenceStale)
            {
                violations.push(ContractDiffReportViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::EvidenceMissing)
            {
                violations.push(ContractDiffReportViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_coherence(row, violations);
    }

    fn validate_diff_class(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        // A breaking change must show the breaking surface in its diff.
        if row.change_class == ChangeClass::Breaking
            && !row.interface_diff.has_incompatible_surface()
        {
            violations.push(ContractDiffReportViolation::BreakingDiffEmpty {
                entry_id: row.entry_id.clone(),
            });
        }
        // An additive change may not remove surface.
        if row.change_class == ChangeClass::Additive && !row.interface_diff.removed.is_empty() {
            violations.push(ContractDiffReportViolation::AdditiveWithRemoval {
                entry_id: row.entry_id.clone(),
            });
        }
        // A reader/writer review that found a breaking change must be classified
        // breaking.
        if row.interface_diff.found_breaking() && row.change_class != ChangeClass::Breaking {
            violations.push(ContractDiffReportViolation::ReviewBreakingClassMismatch {
                entry_id: row.entry_id.clone(),
                change_class: row.change_class,
            });
        }
    }

    fn validate_deprecation(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        match row.deprecation_gap() {
            DeprecationGap::None => {}
            DeprecationGap::Unpacketed => {
                if row.holds_label() {
                    violations.push(ContractDiffReportViolation::BreakingHeldWithoutPacket {
                        entry_id: row.entry_id.clone(),
                    });
                }
                if !row.has_active_reason(NarrowingReason::BreakingChangeUnpacketed) {
                    violations.push(ContractDiffReportViolation::DeprecationReasonMissing {
                        entry_id: row.entry_id.clone(),
                        expected: NarrowingReason::BreakingChangeUnpacketed,
                    });
                }
            }
            DeprecationGap::Incomplete => {
                if row.holds_label() {
                    violations.push(ContractDiffReportViolation::IncompletePacketHeld {
                        entry_id: row.entry_id.clone(),
                    });
                }
                if !row.has_active_reason(NarrowingReason::DeprecationPacketIncomplete) {
                    violations.push(ContractDiffReportViolation::DeprecationReasonMissing {
                        entry_id: row.entry_id.clone(),
                        expected: NarrowingReason::DeprecationPacketIncomplete,
                    });
                }
            }
            DeprecationGap::Overdue => {
                if row.holds_label() {
                    violations.push(ContractDiffReportViolation::RemovalOverdueHeld {
                        entry_id: row.entry_id.clone(),
                    });
                }
                if !row.has_active_reason(NarrowingReason::RemovalOverdue) {
                    violations.push(ContractDiffReportViolation::DeprecationReasonMissing {
                        entry_id: row.entry_id.clone(),
                        expected: NarrowingReason::RemovalOverdue,
                    });
                }
            }
        }
    }

    fn validate_review(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        // A producer-side change whose reader/writer review is missing must narrow
        // and name the review reason.
        if row.review_missing() {
            if row.holds_label() {
                violations.push(ContractDiffReportViolation::ReviewPendingHeld {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.has_active_reason(NarrowingReason::ReaderWriterReviewMissing) {
                violations.push(ContractDiffReportViolation::ReviewReasonMissing {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_window(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        // A breaking compatibility posture must accompany a breaking change class.
        if row.compatibility_window.posture == CompatibilityPosture::Breaking
            && row.change_class != ChangeClass::Breaking
        {
            violations.push(ContractDiffReportViolation::PostureClassIncoherent {
                entry_id: row.entry_id.clone(),
                change_class: row.change_class,
            });
        }
        // A closed compatibility/support window must narrow and name the reason.
        if row.window_ended() {
            if row.holds_label() {
                violations.push(ContractDiffReportViolation::WindowEndedHeld {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.has_active_reason(NarrowingReason::SupportWindowEnded) {
                violations.push(ContractDiffReportViolation::WindowEndedReasonMissing {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_support(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        // A supported-with-caveats class must record at least one caveat.
        if row.support_caveat.support_class.requires_caveat()
            && row
                .support_caveat
                .caveats
                .iter()
                .all(|c| c.trim().is_empty())
        {
            violations.push(ContractDiffReportViolation::SupportCaveatMissing {
                entry_id: row.entry_id.clone(),
            });
        }
        // A limited report state must record at least one caveat.
        if row.report_state == ReportState::Limited
            && row
                .support_caveat
                .caveats
                .iter()
                .all(|c| c.trim().is_empty())
        {
            violations.push(ContractDiffReportViolation::LimitedWithoutCaveat {
                entry_id: row.entry_id.clone(),
            });
        }
        // A held contract may not publish the unsupported support class.
        if row.holds_label() && row.support_caveat.support_class == SupportClass::Unsupported {
            violations.push(ContractDiffReportViolation::HeldWhileUnsupported {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_state_coherence(
        &self,
        row: &ContractDiffReport,
        violations: &mut Vec<ContractDiffReportViolation>,
    ) {
        let push_reason = |violations: &mut Vec<ContractDiffReportViolation>,
                           expected: NarrowingReason| {
            violations.push(ContractDiffReportViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.report_state,
                expected_reason: expected,
            });
        };

        match row.report_state {
            ReportState::Published | ReportState::Limited => {}
            ReportState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(ContractDiffReportViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.report_state,
                    });
                }
            }
            ReportState::BreakingUnpacketed => {
                if !row.has_active_reason(NarrowingReason::BreakingChangeUnpacketed) {
                    push_reason(violations, NarrowingReason::BreakingChangeUnpacketed);
                }
            }
            ReportState::DeprecationIncomplete => {
                if !row.has_active_reason(NarrowingReason::DeprecationPacketIncomplete) {
                    push_reason(violations, NarrowingReason::DeprecationPacketIncomplete);
                }
            }
            ReportState::CompatReviewPending => {
                if !row.has_active_reason(NarrowingReason::ReaderWriterReviewMissing) {
                    push_reason(violations, NarrowingReason::ReaderWriterReviewMissing);
                }
            }
            ReportState::RemovalOverdue => {
                if !row.has_active_reason(NarrowingReason::RemovalOverdue) {
                    push_reason(violations, NarrowingReason::RemovalOverdue);
                }
            }
            ReportState::SupportWindowEnded => {
                if !row.has_active_reason(NarrowingReason::SupportWindowEnded) {
                    push_reason(violations, NarrowingReason::SupportWindowEnded);
                }
            }
            ReportState::EvidenceStale => {
                if !row.has_active_reason(NarrowingReason::EvidenceStale) {
                    push_reason(violations, NarrowingReason::EvidenceStale);
                }
            }
            ReportState::Incomplete => {
                if !row.has_active_reason(NarrowingReason::EvidenceMissing)
                    && !row.has_active_reason(NarrowingReason::OwnerSignoffMissing)
                    && !row.has_active_reason(NarrowingReason::ClaimPublicationMissing)
                {
                    push_reason(violations, NarrowingReason::EvidenceMissing);
                }
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<ContractDiffReportViolation>) {
        let covered: BTreeSet<String> = self
            .reports
            .iter()
            .map(|row| row.contract_ref.clone())
            .collect();
        for declared in &self.release_blocking_contract_refs {
            if !covered.contains(declared) {
                violations.push(
                    ContractDiffReportViolation::ReleaseBlockingContractUncovered {
                        contract_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.reports {
            if row.release_blocking
                && !self
                    .release_blocking_contract_refs
                    .contains(&row.contract_ref)
            {
                violations.push(ContractDiffReportViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<ContractDiffReportViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(ContractDiffReportViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(ContractDiffReportViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(ContractDiffReportViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(ContractDiffReportViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(ContractDiffReportViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 public-interface diff-report register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractDiffReportViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no reports.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A report id appears more than once.
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
    /// A breaking change shows no removed or changed surface in its diff.
    BreakingDiffEmpty {
        /// Row id.
        entry_id: String,
    },
    /// An additive change removes surface.
    AdditiveWithRemoval {
        /// Row id.
        entry_id: String,
    },
    /// A reader/writer review found a breaking change but the row is not breaking.
    ReviewBreakingClassMismatch {
        /// Row id.
        entry_id: String,
        /// Declared change class.
        change_class: ChangeClass,
    },
    /// A breaking change holds its label without a deprecation packet.
    BreakingHeldWithoutPacket {
        /// Row id.
        entry_id: String,
    },
    /// An incomplete deprecation packet holds its label.
    IncompletePacketHeld {
        /// Row id.
        entry_id: String,
    },
    /// An overdue removal holds its label.
    RemovalOverdueHeld {
        /// Row id.
        entry_id: String,
    },
    /// A deprecation gap does not name its narrowing reason.
    DeprecationReasonMissing {
        /// Row id.
        entry_id: String,
        /// The reason the gap requires.
        expected: NarrowingReason,
    },
    /// A missing reader/writer review holds its label.
    ReviewPendingHeld {
        /// Row id.
        entry_id: String,
    },
    /// A missing reader/writer review does not name the review reason.
    ReviewReasonMissing {
        /// Row id.
        entry_id: String,
    },
    /// A breaking compatibility posture is paired with a non-breaking change class.
    PostureClassIncoherent {
        /// Row id.
        entry_id: String,
        /// Declared change class.
        change_class: ChangeClass,
    },
    /// A closed compatibility/support window holds its label.
    WindowEndedHeld {
        /// Row id.
        entry_id: String,
    },
    /// A closed compatibility/support window does not name the window reason.
    WindowEndedReasonMissing {
        /// Row id.
        entry_id: String,
    },
    /// A supported-with-caveats class records no caveat.
    SupportCaveatMissing {
        /// Row id.
        entry_id: String,
    },
    /// A limited report records no caveat.
    LimitedWithoutCaveat {
        /// Row id.
        entry_id: String,
    },
    /// A held report publishes the unsupported support class.
    HeldWhileUnsupported {
        /// Row id.
        entry_id: String,
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
    /// A report holds a label while the claim is below the cutline.
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
        /// Report state.
        state: ReportState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Report state.
        state: ReportState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held report carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held report carries active narrowing reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held report has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held report rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held report lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing report with a breached proof packet does not name the stale
    /// reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing report with a missing proof packet does not name the missing
    /// reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A report state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Report state.
        state: ReportState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Report state.
        state: ReportState,
    },
    /// A release-blocking contract ref has no covering report.
    ReleaseBlockingContractUncovered {
        /// Contract ref.
        contract_ref: String,
    },
    /// A release-blocking report is not declared in the release-blocking list.
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
    /// The summary counts disagree with the reports.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for ContractDiffReportViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no reports"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
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
            Self::BreakingDiffEmpty { entry_id } => {
                write!(
                    f,
                    "report {entry_id} is breaking but shows no removed or changed surface"
                )
            }
            Self::AdditiveWithRemoval { entry_id } => {
                write!(f, "report {entry_id} is additive but removes surface")
            }
            Self::ReviewBreakingClassMismatch {
                entry_id,
                change_class,
            } => write!(
                f,
                "report {entry_id} review found a breaking change but class is {}",
                change_class.as_str()
            ),
            Self::BreakingHeldWithoutPacket { entry_id } => {
                write!(f, "report {entry_id} holds label on a breaking change without a deprecation packet")
            }
            Self::IncompletePacketHeld { entry_id } => {
                write!(
                    f,
                    "report {entry_id} holds label on an incomplete deprecation packet"
                )
            }
            Self::RemovalOverdueHeld { entry_id } => {
                write!(f, "report {entry_id} holds label on an overdue removal")
            }
            Self::DeprecationReasonMissing { entry_id, expected } => write!(
                f,
                "report {entry_id} deprecation gap requires active reason {}",
                expected.as_str()
            ),
            Self::ReviewPendingHeld { entry_id } => {
                write!(
                    f,
                    "report {entry_id} holds label with a missing reader/writer review"
                )
            }
            Self::ReviewReasonMissing { entry_id } => {
                write!(
                    f,
                    "report {entry_id} missing review without reader_writer_review_missing reason"
                )
            }
            Self::PostureClassIncoherent {
                entry_id,
                change_class,
            } => write!(
                f,
                "report {entry_id} has a breaking compatibility posture but class is {}",
                change_class.as_str()
            ),
            Self::WindowEndedHeld { entry_id } => {
                write!(
                    f,
                    "report {entry_id} holds label on a closed compatibility window"
                )
            }
            Self::WindowEndedReasonMissing { entry_id } => {
                write!(
                    f,
                    "report {entry_id} closed window without support_window_ended reason"
                )
            }
            Self::SupportCaveatMissing { entry_id } => {
                write!(
                    f,
                    "report {entry_id} is supported-with-caveats without a caveat"
                )
            }
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(f, "report {entry_id} is limited without a caveat")
            }
            Self::HeldWhileUnsupported { entry_id } => {
                write!(
                    f,
                    "report {entry_id} holds label while publishing the unsupported class"
                )
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "report {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "report {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "report {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "report {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "report {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "report {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "report {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "report {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "report {entry_id} holds stable without owner signoff")
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "report {entry_id} breached packet without evidence_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "report {entry_id} missing packet without evidence_missing reason"
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "report {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "report {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingContractUncovered { contract_ref } => {
                write!(
                    f,
                    "release-blocking contract {contract_ref} has no covering report"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking report {entry_id} is not declared in release_blocking_contract_refs"
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
            Self::SummaryMismatch => write!(f, "summary counts disagree with reports"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "report {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for ContractDiffReportViolation {}

/// Loads the embedded M5 public-interface diff-report register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`ContractDiffReportRegister`].
pub fn current_m5_public_interface_diff_reports(
) -> Result<ContractDiffReportRegister, serde_json::Error> {
    serde_json::from_str(M5_PUBLIC_INTERFACE_DIFF_REPORTS_JSON)
}

#[cfg(test)]
mod tests;
