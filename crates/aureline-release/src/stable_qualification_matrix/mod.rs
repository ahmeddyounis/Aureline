//! Typed stable qualification matrix for the desktop, remote/helper, ecosystem,
//! state/schema, provider, and accessibility lanes — with explicit mixed-version
//! sections for every claimed cross-binary or cross-service boundary.
//!
//! Where the [`stable_claim_matrix`](crate::stable_claim_matrix) decides which
//! *subjects* may publish as Stable, this matrix finalizes the per-lane
//! *qualification rows* that ground those claims, and adds the piece a flat
//! claim row cannot carry: for every boundary that spans two binaries or two
//! services, a [`MixedVersionSection`] that publishes the negotiated fields, the
//! supported skew window, the upgrade order, the rollback order, and the
//! unsupported-state behavior. A boundary that cannot publish that data is, by
//! construction, *coordinated-upgrade-only* and may not inherit a Stable
//! mixed-version claim.
//!
//! Each [`QualificationRow`] binds one lane subject to:
//!
//! - the lane it speaks for ([`QualificationRowScope`]) and the stable level it
//!   is put forward as ([`QualificationRow::claimed_level`]),
//! - a qualification row carrying proof refs, a freshness window, an optional
//!   waiver, and owner sign-off (reused from the stable claim matrix:
//!   [`QualificationEvidence`], [`QualificationWaiver`], [`OwnerSignoff`]),
//! - the qualification state earned ([`QualificationState`]), the active
//!   downgrade reasons ([`DowngradeReason`]), and the level it *effectively*
//!   holds after narrowing ([`QualificationRow::effective_level`]), and
//! - for a cross-binary or cross-service lane, the
//!   [`MixedVersionSection`] that publishes its mixed-version posture.
//!
//! The matrix reuses the launch cutline, level vocabulary, and qualification
//! state vocabulary of the stable claim matrix rather than re-minting lifecycle
//! labels. It is checked in at `artifacts/release/stable_qualification_matrix.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Date
//! arithmetic (waiver expiry and evidence staleness against an `as_of` date)
//! lives in the CI gate; this model enforces the structural and logical
//! invariants that hold regardless of the clock — narrowing consistency, the
//! no-widening rule, the mixed-version completeness/coordinated-upgrade-only
//! rule, owner sign-off on held claims, downgrade-rule wiring, and the promotion
//! verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationEvidence, QualificationState,
    QualificationWaiver, StableClaimLevel,
};

/// Supported matrix schema version.
pub const STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const STABLE_QUALIFICATION_MATRIX_RECORD_KIND: &str = "stable_qualification_matrix";

/// Repo-relative path to the checked-in matrix.
pub const STABLE_QUALIFICATION_MATRIX_PATH: &str =
    "artifacts/release/stable_qualification_matrix.json";

/// Embedded checked-in matrix JSON.
pub const STABLE_QUALIFICATION_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable_qualification_matrix.json"
));

/// The qualification lane a row speaks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationRowScope {
    /// Desktop client and the local helper/sidecar processes it launches.
    Desktop,
    /// Desktop/CLI clients and the remote agent/helper or managed control plane.
    RemoteHelper,
    /// Extension host and the extension ABI surface.
    Ecosystem,
    /// Saved-artifact and schema readers and writers.
    StateSchema,
    /// Provider adapters.
    Provider,
    /// Accessibility behavior across the touched surfaces.
    Accessibility,
}

impl QualificationRowScope {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Desktop,
        Self::RemoteHelper,
        Self::Ecosystem,
        Self::StateSchema,
        Self::Provider,
        Self::Accessibility,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::RemoteHelper => "remote_helper",
            Self::Ecosystem => "ecosystem",
            Self::StateSchema => "state_schema",
            Self::Provider => "provider",
            Self::Accessibility => "accessibility",
        }
    }

    /// Whether this lane represents a cross-binary or cross-service boundary and
    /// therefore must publish a [`MixedVersionSection`]. Every lane but
    /// accessibility crosses a binary or service boundary.
    pub const fn requires_mixed_version(self) -> bool {
        !matches!(self, Self::Accessibility)
    }
}

/// The cross-binary or cross-service boundary a mixed-version section describes.
///
/// The closed set mirrors the boundary families the spec enumerates: launcher
/// and local sidecars, desktop/CLI and the remote agent/helper, desktop/CLI/
/// browser and the managed control plane, the extension host and ABI,
/// saved-artifact/schema readers and writers, and provider adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryFamily {
    /// Launcher and the local sidecar/helper processes.
    LauncherAndLocalSidecars,
    /// Desktop/CLI client and the remote agent/helper.
    DesktopCliAndRemoteAgent,
    /// Desktop/CLI/browser client and the managed control plane.
    DesktopCliBrowserAndManagedControlPlane,
    /// Extension host and the extension ABI.
    ExtensionHostAndAbi,
    /// Saved-artifact and schema readers and writers.
    SavedArtifactAndSchemaReadersWriters,
    /// Provider adapters.
    ProviderAdapters,
}

impl BoundaryFamily {
    /// Every boundary family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LauncherAndLocalSidecars,
        Self::DesktopCliAndRemoteAgent,
        Self::DesktopCliBrowserAndManagedControlPlane,
        Self::ExtensionHostAndAbi,
        Self::SavedArtifactAndSchemaReadersWriters,
        Self::ProviderAdapters,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LauncherAndLocalSidecars => "launcher_and_local_sidecars",
            Self::DesktopCliAndRemoteAgent => "desktop_cli_and_remote_agent",
            Self::DesktopCliBrowserAndManagedControlPlane => {
                "desktop_cli_browser_and_managed_control_plane"
            }
            Self::ExtensionHostAndAbi => "extension_host_and_abi",
            Self::SavedArtifactAndSchemaReadersWriters => {
                "saved_artifact_and_schema_readers_writers"
            }
            Self::ProviderAdapters => "provider_adapters",
        }
    }
}

/// The mixed-version posture a boundary is put forward as or effectively holds.
///
/// `rolling_skew_supported` and `bounded_skew_supported` are the two Stable
/// mixed-version claims, strongest first; `coordinated_upgrade_only` is the
/// floor a boundary drops to when it cannot back a mixed-version claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MixedVersionPosture {
    /// Mixed versions across a broad rolling skew window are supported.
    RollingSkewSupported,
    /// Mixed versions across a bounded skew window are supported; outside it the
    /// boundary degrades or fails closed.
    BoundedSkewSupported,
    /// No mixed-version support is claimed; the binaries/services must upgrade
    /// together as one coordinated set.
    CoordinatedUpgradeOnly,
}

impl MixedVersionPosture {
    /// Every posture, strongest to weakest.
    pub const ALL: [Self; 3] = [
        Self::RollingSkewSupported,
        Self::BoundedSkewSupported,
        Self::CoordinatedUpgradeOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollingSkewSupported => "rolling_skew_supported",
            Self::BoundedSkewSupported => "bounded_skew_supported",
            Self::CoordinatedUpgradeOnly => "coordinated_upgrade_only",
        }
    }

    /// Strength rank; a stronger mixed-version claim ranks higher.
    pub const fn rank(self) -> u8 {
        match self {
            Self::RollingSkewSupported => 2,
            Self::BoundedSkewSupported => 1,
            Self::CoordinatedUpgradeOnly => 0,
        }
    }

    /// True when this posture is a Stable mixed-version claim (rolling or
    /// bounded). `coordinated_upgrade_only` is not a Stable mixed-version claim.
    pub const fn is_stable_mixed_version_claim(self) -> bool {
        self.rank() >= Self::BoundedSkewSupported.rank()
    }
}

/// Behavior when a boundary is operated outside its supported skew window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutOfWindowPosture {
    /// The boundary refuses to operate.
    FailClosed,
    /// The boundary operates read-only.
    ReadOnly,
    /// The boundary operates with reduced function.
    Degraded,
    /// The boundary is explicitly unsupported in this state.
    ExplicitlyUnsupported,
}

impl OutOfWindowPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FailClosed,
        Self::ReadOnly,
        Self::Degraded,
        Self::ExplicitlyUnsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::ReadOnly => "read_only",
            Self::Degraded => "degraded",
            Self::ExplicitlyUnsupported => "explicitly_unsupported",
        }
    }
}

/// Closed reason a qualification row narrows or a downgrade rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// Required qualification proof is absent.
    QualificationEvidenceMissing,
    /// Qualification proof exists but is no longer current.
    QualificationEvidenceStale,
    /// A waiver the claim relied on has expired.
    WaiverExpired,
    /// The evidence freshness window has been exceeded.
    FreshnessWindowExceeded,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The backing stable claim narrowed below the stable cutline.
    BackingClaimNarrowed,
    /// The mixed-version section does not publish complete negotiation data, so
    /// the boundary is coordinated-upgrade-only.
    MixedVersionDataIncomplete,
    /// The boundary is being operated outside its supported skew window.
    MixedVersionSkewUnsupported,
}

impl DowngradeReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::QualificationEvidenceMissing,
        Self::QualificationEvidenceStale,
        Self::WaiverExpired,
        Self::FreshnessWindowExceeded,
        Self::OwnerSignoffMissing,
        Self::BackingClaimNarrowed,
        Self::MixedVersionDataIncomplete,
        Self::MixedVersionSkewUnsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualificationEvidenceMissing => "qualification_evidence_missing",
            Self::QualificationEvidenceStale => "qualification_evidence_stale",
            Self::WaiverExpired => "waiver_expired",
            Self::FreshnessWindowExceeded => "freshness_window_exceeded",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::BackingClaimNarrowed => "backing_claim_narrowed",
            Self::MixedVersionDataIncomplete => "mixed_version_data_incomplete",
            Self::MixedVersionSkewUnsupported => "mixed_version_skew_unsupported",
        }
    }

    /// True when this is a mixed-version-specific reason.
    pub const fn is_mixed_version_reason(self) -> bool {
        matches!(
            self,
            Self::MixedVersionDataIncomplete | Self::MixedVersionSkewUnsupported
        )
    }
}

/// Default action a downgrade rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationAction {
    /// Hold stable promotion until the condition clears.
    HoldPromotion,
    /// Narrow the public claim below the cutline.
    NarrowClaim,
    /// Refresh the qualification evidence packet.
    RefreshEvidencePacket,
    /// Require a coordinated upgrade and withdraw the mixed-version claim.
    RequireCoordinatedUpgrade,
    /// Staff a correction lane.
    StaffCorrectionLane,
}

impl QualificationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HoldPromotion,
        Self::NarrowClaim,
        Self::RefreshEvidencePacket,
        Self::RequireCoordinatedUpgrade,
        Self::StaffCorrectionLane,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowClaim => "narrow_claim",
            Self::RefreshEvidencePacket => "refresh_evidence_packet",
            Self::RequireCoordinatedUpgrade => "require_coordinated_upgrade",
            Self::StaffCorrectionLane => "staff_correction_lane",
        }
    }
}

/// The supported version-skew window for a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkewWindow {
    /// Stable window class token (e.g. `bounded_one_minor_skew`).
    pub window_class: String,
    /// Reviewable description of the window.
    pub summary: String,
    /// Ref into the version-skew register that defines this window.
    pub skew_register_ref: String,
}

impl SkewWindow {
    /// True when every field carries content.
    pub fn is_complete(&self) -> bool {
        !self.window_class.trim().is_empty()
            && !self.summary.trim().is_empty()
            && !self.skew_register_ref.trim().is_empty()
    }
}

/// A declared upgrade or rollback order across a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderRecord {
    /// The ordered steps. Empty only when the order has not been declared.
    #[serde(default)]
    pub declared_order: Vec<String>,
    /// Reviewable note on the order.
    pub notes: String,
}

impl OrderRecord {
    /// True when the order is declared with a non-empty note.
    pub fn is_declared(&self) -> bool {
        !self.declared_order.is_empty() && !self.notes.trim().is_empty()
    }
}

/// The behavior published for the unsupported (out-of-window) skew state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnsupportedStateBehavior {
    /// Stable state-class token for the unsupported state.
    pub state_class: String,
    /// Posture taken outside the supported window.
    pub out_of_window_posture: OutOfWindowPosture,
    /// Reviewable contract rule governing the unsupported state.
    pub contract_rule: String,
}

impl UnsupportedStateBehavior {
    /// True when the behavior carries a state class and a contract rule.
    pub fn is_complete(&self) -> bool {
        !self.state_class.trim().is_empty() && !self.contract_rule.trim().is_empty()
    }
}

/// The mixed-version section for one cross-binary or cross-service boundary.
///
/// The boundary publishes a Stable mixed-version claim only when it publishes
/// *complete* negotiation data: negotiated fields, a supported skew window, an
/// upgrade order, a rollback order, and the unsupported-state behavior. A
/// section that is missing any of these is coordinated-upgrade-only and the
/// boundary may not inherit a Stable mixed-version claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MixedVersionSection {
    /// The boundary family this section describes.
    pub boundary_family: BoundaryFamily,
    /// Human-readable boundary label.
    pub boundary_label: String,
    /// The mixed-version posture the boundary is put forward as.
    pub claimed_posture: MixedVersionPosture,
    /// The mixed-version posture the boundary effectively holds after narrowing.
    pub effective_posture: MixedVersionPosture,
    /// The negotiated fields exchanged across the boundary. Empty only when the
    /// negotiation surface has not been declared.
    #[serde(default)]
    pub negotiated_fields: Vec<String>,
    /// The supported skew window, or null when none has been declared.
    #[serde(default)]
    pub supported_skew_window: Option<SkewWindow>,
    /// The declared upgrade order.
    pub upgrade_order: OrderRecord,
    /// The declared rollback order.
    pub rollback_order: OrderRecord,
    /// The unsupported-state behavior, or null when none has been declared.
    #[serde(default)]
    pub unsupported_state_behavior: Option<UnsupportedStateBehavior>,
    /// Proof refs backing the mixed-version posture.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewable reason the boundary carries this posture.
    pub rationale: String,
}

impl MixedVersionSection {
    /// True when the section publishes every required mixed-version field: the
    /// negotiated fields, a complete supported skew window, a declared upgrade
    /// order, a declared rollback order, and the unsupported-state behavior.
    pub fn publishes_complete_negotiation_data(&self) -> bool {
        !self.negotiated_fields.is_empty()
            && self
                .supported_skew_window
                .as_ref()
                .is_some_and(SkewWindow::is_complete)
            && self.upgrade_order.is_declared()
            && self.rollback_order.is_declared()
            && self
                .unsupported_state_behavior
                .as_ref()
                .is_some_and(UnsupportedStateBehavior::is_complete)
    }

    /// True when the boundary effectively holds a Stable mixed-version claim.
    pub fn holds_stable_mixed_version_claim(&self) -> bool {
        self.effective_posture.is_stable_mixed_version_claim()
    }
}

/// One downgrade rule: a closed condition that narrows a claim and may gate
/// stable promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The downgrade reason whose presence on a claimed row fires this rule.
    pub trigger_reason: DowngradeReason,
    /// Claimed levels this rule watches.
    pub applies_to_levels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: QualificationAction,
    /// Whether firing this rule blocks stable promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One stable qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Human-readable title.
    pub title: String,
    /// The lane this row speaks for.
    pub row_scope: QualificationRowScope,
    /// Subject family the row speaks for.
    pub subject_family: String,
    /// The stable-claim id this qualification row grounds, when present.
    #[serde(default)]
    pub backing_claim_ref: Option<String>,
    /// The stable level the row is put forward as. Always at or above cutline.
    pub claimed_level: StableClaimLevel,
    /// Qualification state earned for the claimed level.
    pub qualification_state: QualificationState,
    /// Qualification proof and freshness window.
    pub evidence: QualificationEvidence,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active downgrade reasons narrowing the row.
    #[serde(default)]
    pub active_downgrade_reasons: Vec<DowngradeReason>,
    /// The level the row effectively holds after narrowing.
    pub effective_level: StableClaimLevel,
    /// The mixed-version section, required for every cross-binary lane and absent
    /// for the accessibility lane.
    #[serde(default)]
    pub mixed_version: Option<MixedVersionSection>,
    /// Publication destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl QualificationRow {
    /// True when the row's effective level is at or above the cutline.
    pub fn holds_stable(&self) -> bool {
        self.effective_level.is_at_or_above_cutline()
    }

    /// True when a downgrade reason is active on the row.
    pub fn has_active_reason(&self, reason: DowngradeReason) -> bool {
        self.active_downgrade_reasons.contains(&reason)
    }

    /// True when this lane must carry a mixed-version section.
    pub fn requires_mixed_version(&self) -> bool {
        self.row_scope.requires_mixed_version()
    }
}

/// The recorded promotion verdict for the stable train.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionDecisionRecord {
    /// The gate this verdict governs.
    pub promotion_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Rule ids that block promotion, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_row_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableQualificationMatrixSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Rows whose effective level is at or above the cutline.
    pub rows_holding_stable: usize,
    /// Rows narrowed below the cutline.
    pub rows_narrowed_below_cutline: usize,
    /// Rows holding a claim via an active waiver.
    pub rows_on_active_waiver: usize,
    /// Cross-binary rows carrying a mixed-version section.
    pub cross_binary_rows: usize,
    /// Rows whose mixed-version section holds a Stable mixed-version claim.
    pub rows_with_stable_mixed_version_claim: usize,
    /// Rows whose mixed-version section is coordinated-upgrade-only.
    pub coordinated_upgrade_only_rows: usize,
    /// Total active downgrade reasons across all rows.
    pub total_active_downgrade_reasons: usize,
    /// Number of downgrade rules currently firing.
    pub downgrade_rules_firing: usize,
}

/// The typed stable qualification matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableQualificationMatrix {
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
    /// Ref to the stable claim matrix this matrix grounds.
    pub claim_matrix_ref: String,
    /// Closed lane vocabulary.
    pub row_scopes: Vec<QualificationRowScope>,
    /// Closed level vocabulary.
    pub claim_levels: Vec<StableClaimLevel>,
    /// Closed qualification-state vocabulary.
    pub qualification_states: Vec<QualificationState>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed downgrade-action vocabulary.
    pub downgrade_actions: Vec<QualificationAction>,
    /// Closed mixed-version-posture vocabulary.
    pub mixed_version_postures: Vec<MixedVersionPosture>,
    /// Closed boundary-family vocabulary.
    pub boundary_families: Vec<BoundaryFamily>,
    /// Closed out-of-window-posture vocabulary.
    pub out_of_window_postures: Vec<OutOfWindowPosture>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Downgrade rules.
    pub downgrade_rules: Vec<DowngradeRule>,
    /// Stable qualification rows.
    pub rows: Vec<QualificationRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: StableQualificationMatrixSummary,
}

impl StableQualificationMatrix {
    /// Returns the row registered for `row_id`.
    pub fn row(&self, row_id: &str) -> Option<&QualificationRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Returns the rows whose effective level is at or above the cutline.
    pub fn rows_holding_stable(&self) -> Vec<&QualificationRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&QualificationRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// Returns the cross-binary rows carrying a mixed-version section.
    pub fn cross_binary_rows(&self) -> Vec<&QualificationRow> {
        self.rows
            .iter()
            .filter(|row| row.mixed_version.is_some())
            .collect()
    }

    /// True when `rule` fires: a claimed row in its watch set carries its
    /// trigger reason.
    pub fn downgrade_rule_fires(&self, rule: &DowngradeRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_levels.contains(&row.claimed_level)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and downgrade rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .downgrade_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.downgrade_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .downgrade_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.downgrade_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing downgrade rule, sorted and unique.
    pub fn computed_blocking_row_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<DowngradeReason> = self
            .downgrade_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.downgrade_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claimed_level.is_at_or_above_cutline()
                && row
                    .active_downgrade_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.row_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and downgrade rules.
    pub fn computed_summary(&self) -> StableQualificationMatrixSummary {
        StableQualificationMatrixSummary {
            total_rows: self.rows.len(),
            rows_holding_stable: self.rows.iter().filter(|row| row.holds_stable()).count(),
            rows_narrowed_below_cutline: self.rows.iter().filter(|row| !row.holds_stable()).count(),
            rows_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.qualification_state == QualificationState::ProvisionalOnWaiver)
                .count(),
            cross_binary_rows: self
                .rows
                .iter()
                .filter(|row| row.mixed_version.is_some())
                .count(),
            rows_with_stable_mixed_version_claim: self
                .rows
                .iter()
                .filter(|row| {
                    row.mixed_version
                        .as_ref()
                        .is_some_and(MixedVersionSection::holds_stable_mixed_version_claim)
                })
                .count(),
            coordinated_upgrade_only_rows: self
                .rows
                .iter()
                .filter(|row| {
                    row.mixed_version.as_ref().is_some_and(|mv| {
                        mv.effective_posture == MixedVersionPosture::CoordinatedUpgradeOnly
                    })
                })
                .count(),
            total_active_downgrade_reasons: self
                .rows
                .iter()
                .map(|row| row.active_downgrade_reasons.len())
                .sum(),
            downgrade_rules_firing: self
                .downgrade_rules
                .iter()
                .filter(|rule| self.downgrade_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the matrix that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> QualificationExportProjection {
        QualificationExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| QualificationExportRow {
                    row_id: row.row_id.clone(),
                    row_scope: row.row_scope,
                    subject_family: row.subject_family.clone(),
                    claimed_level: row.claimed_level,
                    effective_level: row.effective_level,
                    holds_stable: row.holds_stable(),
                    qualification_state: row.qualification_state,
                    active_downgrade_reasons: row.active_downgrade_reasons.clone(),
                    boundary_family: row.mixed_version.as_ref().map(|mv| mv.boundary_family),
                    mixed_version_effective_posture: row
                        .mixed_version
                        .as_ref()
                        .map(|mv| mv.effective_posture),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<StableQualificationMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        let mut covered_boundaries = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.row_id.clone()) {
                violations.push(StableQualificationMatrixViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if let Some(mixed) = &row.mixed_version {
                covered_boundaries.insert(mixed.boundary_family);
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(StableQualificationMatrixViolation::EmptyMatrix);
        }

        // Every boundary family the spec enumerates must be covered by at least
        // one cross-binary row, so no claimed boundary slips out of the matrix.
        for family in BoundaryFamily::ALL {
            if !covered_boundaries.contains(&family) {
                violations
                    .push(StableQualificationMatrixViolation::BoundaryFamilyUncovered { family });
            }
        }

        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(StableQualificationMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<StableQualificationMatrixViolation>) {
        if self.schema_version != STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION {
            violations.push(
                StableQualificationMatrixViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != STABLE_QUALIFICATION_MATRIX_RECORD_KIND {
            violations.push(StableQualificationMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_matrix_ref", &self.claim_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableQualificationMatrixViolation::EmptyField {
                    row_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.row_scopes != QualificationRowScope::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "row_scopes",
                },
            );
        }
        if self.claim_levels != StableClaimLevel::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "claim_levels",
                },
            );
        }
        if self.qualification_states != QualificationState::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "qualification_states",
                },
            );
        }
        if self.downgrade_reasons != DowngradeReason::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "downgrade_reasons",
                },
            );
        }
        if self.downgrade_actions != QualificationAction::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "downgrade_actions",
                },
            );
        }
        if self.mixed_version_postures != MixedVersionPosture::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "mixed_version_postures",
                },
            );
        }
        if self.boundary_families != BoundaryFamily::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "boundary_families",
                },
            );
        }
        if self.out_of_window_postures != OutOfWindowPosture::ALL.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "out_of_window_postures",
                },
            );
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.cutline_level",
                },
            );
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.above_cutline_levels",
                },
            );
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(
                StableQualificationMatrixViolation::ClosedVocabularyMismatch {
                    field: "launch_cutline.below_cutline_levels",
                },
            );
        }
        if cutline.description.trim().is_empty() {
            violations.push(StableQualificationMatrixViolation::EmptyField {
                row_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<StableQualificationMatrixViolation>) {
        if self.downgrade_rules.is_empty() {
            violations.push(StableQualificationMatrixViolation::NoDowngradeRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.downgrade_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(StableQualificationMatrixViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(StableQualificationMatrixViolation::EmptyField {
                        row_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_levels.is_empty() {
                violations.push(StableQualificationMatrixViolation::RuleWithoutLevels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        // Every downgrade reason must have a rule, so a narrowing reason cannot
        // fire without a corresponding promotion gate.
        for reason in DowngradeReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    StableQualificationMatrixViolation::DowngradeReasonWithoutRule { reason },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<StableQualificationMatrixViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("title", &row.title),
            ("subject_family", &row.subject_family),
            ("rationale", &row.rationale),
            ("evidence.proof_index_ref", &row.evidence.proof_index_ref),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(StableQualificationMatrixViolation::EmptyField {
                    row_id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // A qualification row must assert a level at or above the cutline.
        if !row.claimed_level.is_at_or_above_cutline() {
            violations.push(
                StableQualificationMatrixViolation::ClaimedLevelBelowCutline {
                    row_id: row.row_id.clone(),
                    claimed: row.claimed_level,
                },
            );
        }

        // No widening: the effective level may not be stronger than claimed.
        if row.effective_level.rank() > row.claimed_level.rank() {
            violations.push(
                StableQualificationMatrixViolation::EffectiveWiderThanClaimed {
                    row_id: row.row_id.clone(),
                    claimed: row.claimed_level,
                    effective: row.effective_level,
                },
            );
        }

        if row.evidence.freshness_window_days == 0 {
            violations.push(StableQualificationMatrixViolation::EmptyField {
                row_id: row.row_id.clone(),
                field_name: "evidence.freshness_window_days",
            });
        }

        // A state that forces narrowing must drop the row below the cutline and
        // name at least one active reason.
        if row.qualification_state.forces_narrowing() {
            if row.holds_stable() {
                violations.push(
                    StableQualificationMatrixViolation::EffectiveLevelNotNarrowed {
                        row_id: row.row_id.clone(),
                        state: row.qualification_state,
                        effective: row.effective_level,
                    },
                );
            }
            if row.active_downgrade_reasons.is_empty() {
                violations.push(StableQualificationMatrixViolation::NarrowingWithoutReason {
                    row_id: row.row_id.clone(),
                    state: row.qualification_state,
                });
            }
        }

        // A row holding a stable claim must have current, proof-backed,
        // owner-signed qualification with no active reason.
        if row.holds_stable() {
            if row.qualification_state.forces_narrowing() {
                violations.push(
                    StableQualificationMatrixViolation::StableClaimWithNarrowingState {
                        row_id: row.row_id.clone(),
                        state: row.qualification_state,
                    },
                );
            }
            if !row.active_downgrade_reasons.is_empty() {
                violations.push(
                    StableQualificationMatrixViolation::StableClaimWithActiveDowngrade {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.evidence.evidence_refs.is_empty() {
                violations.push(
                    StableQualificationMatrixViolation::StableClaimWithoutEvidence {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(
                    StableQualificationMatrixViolation::StableClaimWithoutSignoff {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        }

        self.validate_state_reason_coherence(row, violations);
        self.validate_mixed_version(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<StableQualificationMatrixViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<StableQualificationMatrixViolation>,
                               expected: DowngradeReason| {
            violations.push(StableQualificationMatrixViolation::StateReasonIncoherent {
                row_id: row.row_id.clone(),
                state: row.qualification_state,
                expected_reason: expected,
            });
        };

        match row.qualification_state {
            QualificationState::NotQualified => {
                // A row is not_qualified either because its surface proof is
                // missing or because, as a cross-binary boundary, it could not
                // back the mixed-version claim it was put forward as.
                if !(row.has_active_reason(DowngradeReason::QualificationEvidenceMissing)
                    || row.has_active_reason(DowngradeReason::MixedVersionDataIncomplete))
                {
                    push_incoherent(violations, DowngradeReason::QualificationEvidenceMissing);
                }
            }
            QualificationState::EvidenceStale => {
                if !(row.has_active_reason(DowngradeReason::QualificationEvidenceStale)
                    || row.has_active_reason(DowngradeReason::FreshnessWindowExceeded))
                {
                    push_incoherent(violations, DowngradeReason::QualificationEvidenceStale);
                }
            }
            QualificationState::WaiverExpired => {
                if !row.has_active_reason(DowngradeReason::WaiverExpired) {
                    push_incoherent(violations, DowngradeReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(
                        StableQualificationMatrixViolation::WaiverStateWithoutWaiver {
                            row_id: row.row_id.clone(),
                            state: row.qualification_state,
                        },
                    );
                }
            }
            QualificationState::ProvisionalOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|waiver| {
                        waiver.waiver_ref.trim().is_empty() || waiver.expires_at.trim().is_empty()
                    })
                    .unwrap_or(true)
                {
                    violations.push(
                        StableQualificationMatrixViolation::WaiverStateWithoutWaiver {
                            row_id: row.row_id.clone(),
                            state: row.qualification_state,
                        },
                    );
                }
            }
            QualificationState::Qualified => {}
        }
    }

    fn validate_mixed_version(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<StableQualificationMatrixViolation>,
    ) {
        match (&row.mixed_version, row.requires_mixed_version()) {
            (None, true) => {
                violations.push(
                    StableQualificationMatrixViolation::MissingMixedVersionSection {
                        row_id: row.row_id.clone(),
                        scope: row.row_scope,
                    },
                );
                return;
            }
            (Some(_), false) => {
                violations.push(
                    StableQualificationMatrixViolation::UnexpectedMixedVersionSection {
                        row_id: row.row_id.clone(),
                        scope: row.row_scope,
                    },
                );
                return;
            }
            (None, false) => return,
            (Some(_), true) => {}
        }

        let mixed = row
            .mixed_version
            .as_ref()
            .expect("mixed-version section present");

        for (field, value) in [
            ("mixed_version.boundary_label", &mixed.boundary_label),
            ("mixed_version.rationale", &mixed.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(StableQualificationMatrixViolation::EmptyField {
                    row_id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // No widening: the effective mixed-version posture may not be stronger
        // than the claimed one.
        if mixed.effective_posture.rank() > mixed.claimed_posture.rank() {
            violations.push(
                StableQualificationMatrixViolation::MixedVersionPostureWidened {
                    row_id: row.row_id.clone(),
                    claimed: mixed.claimed_posture,
                    effective: mixed.effective_posture,
                },
            );
        }

        let complete = mixed.publishes_complete_negotiation_data();

        // The v15 core: a boundary that does not publish complete negotiation
        // data is coordinated-upgrade-only and must carry the
        // mixed_version_data_incomplete reason.
        if !complete {
            if mixed.effective_posture != MixedVersionPosture::CoordinatedUpgradeOnly {
                violations.push(
                    StableQualificationMatrixViolation::IncompleteMixedVersionNotCoordinatedOnly {
                        row_id: row.row_id.clone(),
                        effective: mixed.effective_posture,
                    },
                );
            }
            if !row.has_active_reason(DowngradeReason::MixedVersionDataIncomplete) {
                violations.push(
                    StableQualificationMatrixViolation::IncompleteMixedVersionWithoutReason {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        }

        // A Stable mixed-version claim (rolling/bounded effective posture)
        // requires complete data and a row that itself holds stable. A narrowed
        // row may not inherit a Stable mixed-version claim.
        if mixed.holds_stable_mixed_version_claim() {
            if !complete {
                violations.push(
                    StableQualificationMatrixViolation::StableMixedVersionWithoutCompleteData {
                        row_id: row.row_id.clone(),
                        effective: mixed.effective_posture,
                    },
                );
            }
            if !row.holds_stable() {
                violations.push(
                    StableQualificationMatrixViolation::StableMixedVersionOnNarrowedRow {
                        row_id: row.row_id.clone(),
                        effective: mixed.effective_posture,
                        row_level: row.effective_level,
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<StableQualificationMatrixViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(StableQualificationMatrixViolation::EmptyField {
                row_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(StableQualificationMatrixViolation::EmptyField {
                row_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                StableQualificationMatrixViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                StableQualificationMatrixViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_row_ids != self.computed_blocking_row_ids() {
            violations.push(
                StableQualificationMatrixViolation::PromotionBlockingSetMismatch {
                    field: "blocking_row_ids",
                },
            );
        }
    }
}

/// A redaction-safe export row projected from the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationExportRow {
    /// Stable row id.
    pub row_id: String,
    /// The lane the row speaks for.
    pub row_scope: QualificationRowScope,
    /// Subject family.
    pub subject_family: String,
    /// Level the row is put forward as.
    pub claimed_level: StableClaimLevel,
    /// Level the row effectively holds.
    pub effective_level: StableClaimLevel,
    /// Whether the row holds a stable claim.
    pub holds_stable: bool,
    /// Qualification state.
    pub qualification_state: QualificationState,
    /// Active downgrade reasons.
    pub active_downgrade_reasons: Vec<DowngradeReason>,
    /// Boundary family, when this is a cross-binary row.
    pub boundary_family: Option<BoundaryFamily>,
    /// Effective mixed-version posture, when this is a cross-binary row.
    pub mixed_version_effective_posture: Option<MixedVersionPosture>,
}

/// A redaction-safe export projection of the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationExportProjection {
    /// Matrix id this projection was produced from.
    pub matrix_id: String,
    /// Matrix as-of date.
    pub as_of: String,
    /// Promotion verdict.
    pub promotion_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<QualificationExportRow>,
}

/// A validation violation for the stable qualification matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableQualificationMatrixViolation {
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
    /// The matrix has no downgrade rules.
    NoDowngradeRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        row_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A downgrade rule names no levels to watch.
    RuleWithoutLevels {
        /// Rule id.
        rule_id: String,
    },
    /// A downgrade reason has no rule watching for it.
    DowngradeReasonWithoutRule {
        /// Uncovered reason.
        reason: DowngradeReason,
    },
    /// A boundary family the spec enumerates is covered by no cross-binary row.
    BoundaryFamilyUncovered {
        /// Uncovered boundary family.
        family: BoundaryFamily,
    },
    /// A row asserts a level below the cutline.
    ClaimedLevelBelowCutline {
        /// Row id.
        row_id: String,
        /// Claimed level.
        claimed: StableClaimLevel,
    },
    /// An effective level is stronger than the claimed level.
    EffectiveWiderThanClaimed {
        /// Row id.
        row_id: String,
        /// Claimed level.
        claimed: StableClaimLevel,
        /// Effective level.
        effective: StableClaimLevel,
    },
    /// A narrowing state did not drop the row below the cutline.
    EffectiveLevelNotNarrowed {
        /// Row id.
        row_id: String,
        /// Qualification state.
        state: QualificationState,
        /// Effective level.
        effective: StableClaimLevel,
    },
    /// A narrowing state carries no active downgrade reason.
    NarrowingWithoutReason {
        /// Row id.
        row_id: String,
        /// Qualification state.
        state: QualificationState,
    },
    /// A row holds a stable claim while its state forces narrowing.
    StableClaimWithNarrowingState {
        /// Row id.
        row_id: String,
        /// Qualification state.
        state: QualificationState,
    },
    /// A row holds a stable claim while a downgrade reason is active.
    StableClaimWithActiveDowngrade {
        /// Row id.
        row_id: String,
    },
    /// A row holds a stable claim with no qualification evidence.
    StableClaimWithoutEvidence {
        /// Row id.
        row_id: String,
    },
    /// A row holds a stable claim without owner sign-off.
    StableClaimWithoutSignoff {
        /// Row id.
        row_id: String,
    },
    /// A qualification state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        row_id: String,
        /// Qualification state.
        state: QualificationState,
        /// Reason the state requires.
        expected_reason: DowngradeReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        row_id: String,
        /// Qualification state.
        state: QualificationState,
    },
    /// A cross-binary lane carries no mixed-version section.
    MissingMixedVersionSection {
        /// Row id.
        row_id: String,
        /// The lane.
        scope: QualificationRowScope,
    },
    /// A non-boundary lane carries a mixed-version section.
    UnexpectedMixedVersionSection {
        /// Row id.
        row_id: String,
        /// The lane.
        scope: QualificationRowScope,
    },
    /// An effective mixed-version posture is stronger than the claimed one.
    MixedVersionPostureWidened {
        /// Row id.
        row_id: String,
        /// Claimed posture.
        claimed: MixedVersionPosture,
        /// Effective posture.
        effective: MixedVersionPosture,
    },
    /// A mixed-version section lacking complete data is not coordinated-upgrade-only.
    IncompleteMixedVersionNotCoordinatedOnly {
        /// Row id.
        row_id: String,
        /// Effective posture.
        effective: MixedVersionPosture,
    },
    /// A mixed-version section lacking complete data names no incompleteness reason.
    IncompleteMixedVersionWithoutReason {
        /// Row id.
        row_id: String,
    },
    /// A Stable mixed-version claim rides incomplete negotiation data.
    StableMixedVersionWithoutCompleteData {
        /// Row id.
        row_id: String,
        /// Effective posture.
        effective: MixedVersionPosture,
    },
    /// A narrowed row inherits a Stable mixed-version claim.
    StableMixedVersionOnNarrowedRow {
        /// Row id.
        row_id: String,
        /// Effective posture.
        effective: MixedVersionPosture,
        /// The row's effective level.
        row_level: StableClaimLevel,
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
}

impl fmt::Display for StableQualificationMatrixViolation {
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
            Self::NoDowngradeRules => write!(f, "matrix has no downgrade rules"),
            Self::EmptyField { row_id, field_name } => {
                write!(f, "{row_id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate qualification row id {row_id}"),
            Self::DuplicateRuleId { rule_id } => write!(f, "duplicate downgrade rule id {rule_id}"),
            Self::RuleWithoutLevels { rule_id } => {
                write!(f, "downgrade rule {rule_id} watches no levels")
            }
            Self::DowngradeReasonWithoutRule { reason } => write!(
                f,
                "downgrade reason {} has no rule watching for it",
                reason.as_str()
            ),
            Self::BoundaryFamilyUncovered { family } => write!(
                f,
                "boundary family {} is covered by no cross-binary row",
                family.as_str()
            ),
            Self::ClaimedLevelBelowCutline { row_id, claimed } => write!(
                f,
                "row {row_id} asserts level {} which is below the stable cutline",
                claimed.as_str()
            ),
            Self::EffectiveWiderThanClaimed {
                row_id,
                claimed,
                effective,
            } => write!(
                f,
                "row {row_id} effective level {} is wider than claimed level {}",
                effective.as_str(),
                claimed.as_str()
            ),
            Self::EffectiveLevelNotNarrowed {
                row_id,
                state,
                effective,
            } => write!(
                f,
                "row {row_id} state {} must narrow below the cutline but holds {}",
                state.as_str(),
                effective.as_str()
            ),
            Self::NarrowingWithoutReason { row_id, state } => write!(
                f,
                "row {row_id} state {} narrows without naming an active downgrade reason",
                state.as_str()
            ),
            Self::StableClaimWithNarrowingState { row_id, state } => write!(
                f,
                "row {row_id} holds stable while its state {} forces narrowing",
                state.as_str()
            ),
            Self::StableClaimWithActiveDowngrade { row_id } => write!(
                f,
                "row {row_id} holds stable while a downgrade reason is active"
            ),
            Self::StableClaimWithoutEvidence { row_id } => {
                write!(f, "row {row_id} holds stable with no qualification evidence")
            }
            Self::StableClaimWithoutSignoff { row_id } => {
                write!(f, "row {row_id} holds stable without owner sign-off")
            }
            Self::StateReasonIncoherent {
                row_id,
                state,
                expected_reason,
            } => write!(
                f,
                "row {row_id} state {} requires active reason {}",
                state.as_str(),
                expected_reason.as_str()
            ),
            Self::WaiverStateWithoutWaiver { row_id, state } => {
                write!(f, "row {row_id} state {} names no waiver", state.as_str())
            }
            Self::MissingMixedVersionSection { row_id, scope } => write!(
                f,
                "row {row_id} lane {} is a cross-binary boundary but carries no mixed-version section",
                scope.as_str()
            ),
            Self::UnexpectedMixedVersionSection { row_id, scope } => write!(
                f,
                "row {row_id} lane {} is not a cross-binary boundary but carries a mixed-version section",
                scope.as_str()
            ),
            Self::MixedVersionPostureWidened {
                row_id,
                claimed,
                effective,
            } => write!(
                f,
                "row {row_id} effective mixed-version posture {} is wider than claimed {}",
                effective.as_str(),
                claimed.as_str()
            ),
            Self::IncompleteMixedVersionNotCoordinatedOnly { row_id, effective } => write!(
                f,
                "row {row_id} mixed-version section lacks complete data but holds {} instead of coordinated_upgrade_only",
                effective.as_str()
            ),
            Self::IncompleteMixedVersionWithoutReason { row_id } => write!(
                f,
                "row {row_id} mixed-version section lacks complete data without the mixed_version_data_incomplete reason"
            ),
            Self::StableMixedVersionWithoutCompleteData { row_id, effective } => write!(
                f,
                "row {row_id} claims the Stable mixed-version posture {} on incomplete negotiation data",
                effective.as_str()
            ),
            Self::StableMixedVersionOnNarrowedRow {
                row_id,
                effective,
                row_level,
            } => write!(
                f,
                "row {row_id} narrowed to {} may not inherit the Stable mixed-version posture {}",
                row_level.as_str(),
                effective.as_str()
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion decision {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with the firing downgrade rules")
            }
            Self::SummaryMismatch => write!(f, "matrix summary counts disagree with the rows"),
        }
    }
}

impl Error for StableQualificationMatrixViolation {}

/// Loads the embedded stable qualification matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`StableQualificationMatrix`] — including when a row carries a level,
/// qualification state, downgrade reason, mixed-version posture, boundary
/// family, or out-of-window posture outside the closed vocabularies.
pub fn current_stable_qualification_matrix() -> Result<StableQualificationMatrix, serde_json::Error>
{
    serde_json::from_str(STABLE_QUALIFICATION_MATRIX_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> StableQualificationMatrix {
        current_stable_qualification_matrix().expect("matrix parses")
    }

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let matrix = matrix();
        assert_eq!(
            matrix.schema_version,
            STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION
        );
        assert_eq!(matrix.record_kind, STABLE_QUALIFICATION_MATRIX_RECORD_KIND);
        assert_eq!(matrix.validate(), Vec::new());
        assert!(!matrix.rows.is_empty());
    }

    #[test]
    fn every_boundary_family_is_covered() {
        let matrix = matrix();
        let covered: BTreeSet<BoundaryFamily> = matrix
            .rows
            .iter()
            .filter_map(|row| row.mixed_version.as_ref().map(|mv| mv.boundary_family))
            .collect();
        for family in BoundaryFamily::ALL {
            assert!(covered.contains(&family), "{}", family.as_str());
        }
    }

    #[test]
    fn accessibility_row_carries_no_mixed_version_section() {
        let matrix = matrix();
        let accessibility = matrix
            .rows
            .iter()
            .find(|row| row.row_scope == QualificationRowScope::Accessibility)
            .expect("an accessibility row exists");
        assert!(accessibility.mixed_version.is_none());
        assert!(!accessibility.requires_mixed_version());
    }

    #[test]
    fn matrix_exercises_holding_and_narrowed_rows() {
        let matrix = matrix();
        assert!(!matrix.rows_holding_stable().is_empty());
        assert!(!matrix.rows_narrowed().is_empty());
    }

    #[test]
    fn coordinated_upgrade_only_row_exists_with_incomplete_reason() {
        let matrix = matrix();
        let row = matrix
            .rows
            .iter()
            .find(|row| {
                row.mixed_version.as_ref().is_some_and(|mv| {
                    !mv.publishes_complete_negotiation_data()
                        && mv.effective_posture == MixedVersionPosture::CoordinatedUpgradeOnly
                })
            })
            .expect("an incomplete mixed-version row exists");
        assert!(row.has_active_reason(DowngradeReason::MixedVersionDataIncomplete));
    }

    #[test]
    fn summary_counts_match_rows() {
        let matrix = matrix();
        assert_eq!(matrix.summary, matrix.computed_summary());
        assert_eq!(
            matrix.summary.rows_holding_stable + matrix.summary.rows_narrowed_below_cutline,
            matrix.rows.len()
        );
    }

    #[test]
    fn promotion_holds_when_a_blocking_rule_fires() {
        let matrix = matrix();
        assert_eq!(matrix.promotion.decision, PromotionDecision::Hold);
        assert_eq!(
            matrix.promotion.decision,
            matrix.computed_promotion_decision()
        );
        assert!(!matrix.promotion.blocking_rule_ids.is_empty());
        assert!(!matrix.promotion.blocking_row_ids.is_empty());
    }

    #[test]
    fn every_downgrade_reason_has_a_rule() {
        let matrix = matrix();
        let covered: BTreeSet<DowngradeReason> = matrix
            .downgrade_rules
            .iter()
            .map(|rule| rule.trigger_reason)
            .collect();
        for reason in DowngradeReason::ALL {
            assert!(covered.contains(&reason), "{}", reason.as_str());
        }
    }

    #[test]
    fn validate_flags_incomplete_mixed_version_not_narrowed() {
        let mut matrix = matrix();
        // Force a complete-looking posture onto an incomplete section.
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| {
                row.mixed_version
                    .as_ref()
                    .is_some_and(|mv| !mv.publishes_complete_negotiation_data())
            })
            .expect("an incomplete mixed-version row exists");
        if let Some(mv) = row.mixed_version.as_mut() {
            mv.effective_posture = MixedVersionPosture::BoundedSkewSupported;
        }
        assert!(matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::IncompleteMixedVersionNotCoordinatedOnly { .. }
        )));
    }

    #[test]
    fn validate_flags_missing_mixed_version_on_cross_binary_row() {
        let mut matrix = matrix();
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| row.requires_mixed_version())
            .expect("a cross-binary row exists");
        row.mixed_version = None;
        assert!(matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::MissingMixedVersionSection { .. }
        )));
    }

    #[test]
    fn validate_flags_stable_mixed_version_on_narrowed_row() {
        let mut matrix = matrix();
        // Put a Stable mixed-version posture on a narrowed row with complete data.
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| {
                !row.holds_stable()
                    && row
                        .mixed_version
                        .as_ref()
                        .is_some_and(MixedVersionSection::publishes_complete_negotiation_data)
            })
            .expect("a narrowed row with a complete mixed-version section exists");
        if let Some(mv) = row.mixed_version.as_mut() {
            mv.claimed_posture = MixedVersionPosture::BoundedSkewSupported;
            mv.effective_posture = MixedVersionPosture::BoundedSkewSupported;
        }
        assert!(matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::StableMixedVersionOnNarrowedRow { .. }
        )));
    }

    #[test]
    fn validate_flags_inconsistent_promotion_decision() {
        let mut matrix = matrix();
        matrix.promotion.decision = PromotionDecision::Proceed;
        assert!(matrix.validate().iter().any(|violation| matches!(
            violation,
            StableQualificationMatrixViolation::PromotionDecisionInconsistent { .. }
        )));
    }

    #[test]
    fn export_projection_mirrors_rows() {
        let matrix = matrix();
        let projection = matrix.support_export_projection();
        assert_eq!(projection.rows.len(), matrix.rows.len());
        assert_eq!(projection.promotion_decision, matrix.promotion.decision);
        for (row, projected) in matrix.rows.iter().zip(&projection.rows) {
            assert_eq!(row.row_id, projected.row_id);
            assert_eq!(row.holds_stable(), projected.holds_stable);
            assert_eq!(
                row.mixed_version.as_ref().map(|mv| mv.boundary_family),
                projected.boundary_family
            );
        }
    }
}
