//! Typed M5 mixed-version boundary skew-inspector register.
//!
//! Where the M5 qualification/skew matrix freezes the *static* qualification row,
//! support window, and deprecation packet each stable-facing family holds, this
//! register speaks for the *runtime* skew inspectors bound to the M5
//! boundary-crossing flows: helper/agent attach, extension/runtime load,
//! workspace-state import/restore, and provider snapshot/open. Each
//! [`BoundaryInspector`] binds one boundary-crossing flow to:
//!
//! - the boundary it guards ([`BoundaryInspector::boundary_kind`],
//!   [`BoundaryInspector::boundary_ref`]) and the mutating-or-privileged
//!   [`GatedAction`] it gates ([`BoundaryInspector::gated_action`],
//!   [`BoundaryInspector::action_risk`]),
//! - the helper/agent/host/schema/provider [`DowngradeSubject`] it speaks for,
//! - the [`SkewWindow`] it inspects — the local and peer versions, the declared
//!   supported [`SkewWindowClass`], the version floor/ceiling, and the negotiated
//!   fields,
//! - the [`InspectorVerdict`] it reports before the gated action runs
//!   (inside-window, or one of the fail-closed states: unsupported skew, reconnect
//!   required, reinstall required, migration needed, retest pending) and the
//!   resulting [`GatePosture`] (allow or fail-closed),
//! - the structured [`UpgradeOrderGuide`] — which side upgrades first and in what
//!   order — that tells a user or support how to bring an out-of-window boundary
//!   back inside its window,
//! - the stable claim it backs ([`BoundaryInspector::claim_ref`],
//!   [`BoundaryInspector::claim_label`]), the overall [`InspectorState`] earned,
//!   the active [`NarrowingReason`] set, and the effective label after narrowing
//!   ([`BoundaryInspector::published_label`]),
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, an owner sign-off, and an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between an inspector that may publish a Stable support claim and one that must
//! narrow below it. The [`InspectorStopRule`] set names the closed conditions that
//! gate M5 promotion — one per [`NarrowingReason`] — and the register records the
//! proceed/hold verdict.
//!
//! The register is checked in at the path named by
//! [`SHIP_M5_BOUNDARY_SKEW_INSPECTORS_PATH`] and embedded here, so this typed
//! consumer and the CI gate agree on every inspector without a cargo build in CI.
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
pub const SHIP_M5_BOUNDARY_SKEW_INSPECTORS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const SHIP_M5_BOUNDARY_SKEW_INSPECTORS_RECORD_KIND: &str =
    "ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries";

/// Repo-relative path to the checked-in register.
pub const SHIP_M5_BOUNDARY_SKEW_INSPECTORS_PATH: &str =
    "artifacts/release/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.json";

/// Embedded checked-in register JSON.
pub const SHIP_M5_BOUNDARY_SKEW_INSPECTORS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.json"
));

/// M5 mixed-version boundary-crossing flow a skew inspector guards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryKind {
    /// Desktop↔remote helper/agent attach boundary.
    HelperAgentAttach,
    /// Extension host / SDK / manifest runtime-load boundary.
    ExtensionRuntimeLoad,
    /// Workspace/schema/save-state import or restore boundary.
    StateImportRestore,
    /// Provider snapshot or imported-object open boundary.
    ProviderSnapshotOpen,
}

impl BoundaryKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HelperAgentAttach,
        Self::ExtensionRuntimeLoad,
        Self::StateImportRestore,
        Self::ProviderSnapshotOpen,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelperAgentAttach => "helper_agent_attach",
            Self::ExtensionRuntimeLoad => "extension_runtime_load",
            Self::StateImportRestore => "state_import_restore",
            Self::ProviderSnapshotOpen => "provider_snapshot_open",
        }
    }

    /// The mutating-or-privileged action this boundary-crossing flow gates.
    pub const fn gated_action(self) -> GatedAction {
        match self {
            Self::HelperAgentAttach => GatedAction::Attach,
            Self::ExtensionRuntimeLoad => GatedAction::Load,
            Self::StateImportRestore => GatedAction::Restore,
            Self::ProviderSnapshotOpen => GatedAction::Open,
        }
    }
}

/// The helper/agent/host/schema/provider subject an inspector downgrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeSubject {
    /// Remote helper process.
    Helper,
    /// Remote execution agent.
    Agent,
    /// Extension host / runtime.
    Host,
    /// Wire/state schema.
    Schema,
    /// Provider snapshot / imported object.
    Provider,
}

impl DowngradeSubject {
    /// Every subject, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Helper,
        Self::Agent,
        Self::Host,
        Self::Schema,
        Self::Provider,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Helper => "helper",
            Self::Agent => "agent",
            Self::Host => "host",
            Self::Schema => "schema",
            Self::Provider => "provider",
        }
    }
}

/// The mutating-or-privileged action a boundary inspector gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GatedAction {
    /// Attach a helper/agent session.
    Attach,
    /// Load an extension/runtime.
    Load,
    /// Restore imported state.
    Restore,
    /// Open a provider snapshot/imported object.
    Open,
}

impl GatedAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 4] = [Self::Attach, Self::Load, Self::Restore, Self::Open];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attach => "attach",
            Self::Load => "load",
            Self::Restore => "restore",
            Self::Open => "open",
        }
    }
}

/// Whether the gated action mutates state, is privileged, or both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRisk {
    /// The action mutates persistent state.
    Mutating,
    /// The action exercises a privileged capability.
    Privileged,
    /// The action both mutates state and is privileged.
    MutatingAndPrivileged,
}

impl ActionRisk {
    /// Every risk, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Mutating,
        Self::Privileged,
        Self::MutatingAndPrivileged,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mutating => "mutating",
            Self::Privileged => "privileged",
            Self::MutatingAndPrivileged => "mutating_and_privileged",
        }
    }
}

/// Declared supported skew class for a boundary.
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
    /// The declared skew is outside any supported window.
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

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LockstepOnly => "lockstep_only",
            Self::BoundedSkew => "bounded_skew",
            Self::BackwardCompatible => "backward_compatible",
            Self::ForwardCompatible => "forward_compatible",
            Self::UnsupportedSkew => "unsupported_skew",
        }
    }

    /// Whether a boundary declared in this class supports any skew window.
    pub const fn is_supported(self) -> bool {
        !matches!(self, Self::UnsupportedSkew)
    }
}

/// Verdict an inspector reports before the gated action runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorVerdict {
    /// The peer is inside the supported skew window; the gated action proceeds.
    InsideWindow,
    /// The peer is outside any supported skew window; fail closed.
    UnsupportedSkew,
    /// The client must reconnect after upgrading to a supported version.
    ReconnectRequired,
    /// The client must reinstall to reach a supported version.
    ReinstallRequired,
    /// Imported state must be migrated before the restore can proceed.
    MigrationNeeded,
    /// The boundary changed and must be retested before it can re-qualify.
    RetestPending,
}

impl InspectorVerdict {
    /// Every verdict, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InsideWindow,
        Self::UnsupportedSkew,
        Self::ReconnectRequired,
        Self::ReinstallRequired,
        Self::MigrationNeeded,
        Self::RetestPending,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InsideWindow => "inside_window",
            Self::UnsupportedSkew => "unsupported_skew",
            Self::ReconnectRequired => "reconnect_required",
            Self::ReinstallRequired => "reinstall_required",
            Self::MigrationNeeded => "migration_needed",
            Self::RetestPending => "retest_pending",
        }
    }

    /// Whether the verdict lets the gated action proceed.
    pub const fn is_inside_window(self) -> bool {
        matches!(self, Self::InsideWindow)
    }

    /// Whether recovering from this verdict requires a version change with an
    /// explicit upgrade order (as opposed to a retest or evidence refresh).
    pub const fn requires_upgrade_guide(self) -> bool {
        matches!(
            self,
            Self::UnsupportedSkew
                | Self::ReconnectRequired
                | Self::ReinstallRequired
                | Self::MigrationNeeded
        )
    }

    /// The narrowing reason a non-inside-window verdict must name. Returns `None`
    /// for the inside-window verdict.
    pub const fn reason(self) -> Option<NarrowingReason> {
        match self {
            Self::InsideWindow => None,
            Self::UnsupportedSkew => Some(NarrowingReason::SkewWindowExceeded),
            Self::ReconnectRequired => Some(NarrowingReason::ReconnectRequired),
            Self::ReinstallRequired => Some(NarrowingReason::ReinstallRequired),
            Self::MigrationNeeded => Some(NarrowingReason::MigrationNeeded),
            Self::RetestPending => Some(NarrowingReason::RetestPending),
        }
    }
}

/// Posture the boundary applies to the gated action given the verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GatePosture {
    /// The mutating-or-privileged action proceeds.
    Allow,
    /// The boundary fails closed rather than act on unsupported skew.
    FailClosed,
}

impl GatePosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::Allow, Self::FailClosed];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::FailClosed => "fail_closed",
        }
    }
}

/// Which side leads an upgrade-order guide (or a step within it).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeLeadSide {
    /// No upgrade is required.
    NoneRequired,
    /// The local side upgrades first.
    LocalFirst,
    /// The peer side upgrades first.
    PeerFirst,
    /// Both ends upgrade together.
    Coordinated,
}

impl UpgradeLeadSide {
    /// Every lead side, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoneRequired,
        Self::LocalFirst,
        Self::PeerFirst,
        Self::Coordinated,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::LocalFirst => "local_first",
            Self::PeerFirst => "peer_first",
            Self::Coordinated => "coordinated",
        }
    }

    /// Whether this lead side prescribes an actual upgrade.
    pub const fn requires_upgrade(self) -> bool {
        !matches!(self, Self::NoneRequired)
    }
}

/// Overall state an inspector earned for its claimed support label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorState {
    /// The boundary is inside its window and the claim holds.
    InsideWindow,
    /// Holds the claim with a recorded compatibility caveat.
    Limited,
    /// Holds the claim provisionally under an active, unexpired waiver.
    OnWaiver,
    /// The boundary fails closed on an out-of-window skew verdict.
    FailClosed,
    /// The boundary changed and a retest is pending.
    RetestPending,
    /// The inspector's proof packet went stale.
    EvidenceStale,
    /// Inspector evidence, owner sign-off, or claim publication is incomplete.
    Incomplete,
}

impl InspectorState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InsideWindow,
        Self::Limited,
        Self::OnWaiver,
        Self::FailClosed,
        Self::RetestPending,
        Self::EvidenceStale,
        Self::Incomplete,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InsideWindow => "inside_window",
            Self::Limited => "limited",
            Self::OnWaiver => "on_waiver",
            Self::FailClosed => "fail_closed",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::Incomplete => "incomplete",
        }
    }

    /// Whether the state lets the inspector carry its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::InsideWindow | Self::Limited | Self::OnWaiver)
    }
}

/// Closed reason an inspector's support claim narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The peer is outside the supported skew window.
    SkewWindowExceeded,
    /// The client must reconnect after upgrading.
    ReconnectRequired,
    /// The client must reinstall to reach a supported version.
    ReinstallRequired,
    /// Imported state must be migrated before restore.
    MigrationNeeded,
    /// The boundary requires a retest.
    RetestPending,
    /// The inspector's proof packet has gone stale.
    EvidenceStale,
    /// The inspector has no captured proof packet.
    EvidenceMissing,
    /// A waiver the inspector relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// The backing claim publication is missing.
    ClaimPublicationMissing,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::SkewWindowExceeded,
        Self::ReconnectRequired,
        Self::ReinstallRequired,
        Self::MigrationNeeded,
        Self::RetestPending,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::ClaimPublicationMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SkewWindowExceeded => "skew_window_exceeded",
            Self::ReconnectRequired => "reconnect_required",
            Self::ReinstallRequired => "reinstall_required",
            Self::MigrationNeeded => "migration_needed",
            Self::RetestPending => "retest_pending",
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
    /// Widen or document the supported skew window.
    WidenOrDocumentSkew,
    /// Guide the client through the reconnect upgrade order.
    GuideReconnect,
    /// Guide the client through the reinstall upgrade order.
    GuideReinstall,
    /// Guide the operator through the state-migration upgrade order.
    GuideMigration,
    /// Retest the boundary.
    RetestBoundary,
    /// Refresh the inspector evidence packet.
    RefreshEvidence,
    /// Capture the inspector evidence packet.
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
        Self::WidenOrDocumentSkew,
        Self::GuideReconnect,
        Self::GuideReinstall,
        Self::GuideMigration,
        Self::RetestBoundary,
        Self::RefreshEvidence,
        Self::CaptureEvidence,
        Self::NarrowLabel,
        Self::RequestOwnerSignoff,
        Self::RepublishClaim,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenOrDocumentSkew => "widen_or_document_skew",
            Self::GuideReconnect => "guide_reconnect",
            Self::GuideReinstall => "guide_reinstall",
            Self::GuideMigration => "guide_migration",
            Self::RetestBoundary => "retest_boundary",
            Self::RefreshEvidence => "refresh_evidence",
            Self::CaptureEvidence => "capture_evidence",
            Self::NarrowLabel => "narrow_label",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RepublishClaim => "republish_claim",
        }
    }
}

/// The version skew an inspector inspects across a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkewWindow {
    /// The declared supported skew class.
    pub skew_window_class: SkewWindowClass,
    /// The local-side version at the boundary.
    pub local_version: String,
    /// The peer-side version at the boundary.
    pub peer_version: String,
    /// Inclusive version floor of the supported window.
    pub min_supported_version: String,
    /// Inclusive version ceiling of the supported window.
    pub max_supported_version: String,
    /// Negotiated wire/state fields the boundary exchanges.
    #[serde(default)]
    pub negotiated_fields: Vec<String>,
    /// Ref to the reviewer-facing skew-window record.
    pub skew_window_ref: String,
}

/// One step of an upgrade-order guide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpgradeStep {
    /// One-based order of the step.
    pub order: u32,
    /// Which side the step acts on.
    pub side: UpgradeLeadSide,
    /// Reviewable instruction for the step.
    pub instruction: String,
}

/// The upgrade-order guide that recovers an out-of-window boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpgradeOrderGuide {
    /// Which side upgrades first.
    pub lead_side: UpgradeLeadSide,
    /// Ordered upgrade steps. Empty only when no upgrade is required.
    #[serde(default)]
    pub steps: Vec<UpgradeStep>,
    /// Ref to the reviewer-facing upgrade-order guide.
    pub guide_ref: String,
}

/// One inspector-claim stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InspectorStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched inspector fires this rule.
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

/// One M5 boundary skew inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryInspector {
    /// Stable inspector id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The boundary-crossing flow this inspector guards.
    pub boundary_kind: BoundaryKind,
    /// The boundary ref this inspector speaks about.
    pub boundary_ref: String,
    /// Reviewable one-line statement of the boundary.
    pub boundary_summary: String,
    /// Whether the boundary is part of the release-blocking set.
    pub release_blocking: bool,
    /// The helper/agent/host/schema/provider subject this inspector downgrades.
    pub downgrade_subject: DowngradeSubject,
    /// The mutating-or-privileged action gated by this inspector.
    pub gated_action: GatedAction,
    /// Whether the gated action mutates, is privileged, or both.
    pub action_risk: ActionRisk,
    /// The local side of the boundary.
    pub local_role: String,
    /// The peer side of the boundary.
    pub peer_role: String,
    /// The stable-claim entry id whose claim this boundary backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall inspector state earned for the row.
    pub inspector_state: InspectorState,
    /// The version skew this inspector inspects.
    pub skew_window: SkewWindow,
    /// The verdict reported before the gated action runs.
    pub verdict: InspectorVerdict,
    /// The posture applied to the gated action.
    pub gate_posture: GatePosture,
    /// The upgrade-order guide that recovers an out-of-window boundary.
    pub upgrade_order_guide: UpgradeOrderGuide,
    /// Recorded compatibility caveats. Non-empty when the inspector is limited.
    #[serde(default)]
    pub compatibility_caveats: Vec<String>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the inspector below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the boundary effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this inspector's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the inspector carries this posture.
    pub rationale: String,
}

impl BoundaryInspector {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the inspector's state lets the boundary carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.inspector_state.holds_label()
    }

    /// True when a narrowing reason is active on the inspector.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// True when the gate lets the mutating-or-privileged action proceed.
    pub fn action_allowed(&self) -> bool {
        matches!(self.gate_posture, GatePosture::Allow)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundarySkewInspectorRegisterSummary {
    /// Total number of inspectors.
    pub total_inspectors: usize,
    /// Distinct boundaries covered.
    pub total_boundaries: usize,
    /// Inspectors publishing a label at or above the cutline.
    pub inspectors_publishing_stable: usize,
    /// Inspectors narrowed below the cutline.
    pub inspectors_narrowed: usize,
    /// Inspectors in a label-holding state.
    pub inspectors_holding: usize,
    /// Inspectors holding their label via an active waiver.
    pub inspectors_on_waiver: usize,
    /// Inspectors holding their label with a recorded caveat.
    pub inspectors_limited: usize,
    /// Inspectors that fail closed on an out-of-window skew verdict.
    pub inspectors_fail_closed: usize,
    /// Inspectors narrowed because a retest is pending.
    pub inspectors_retest_pending: usize,
    /// Inspectors narrowed because evidence is stale.
    pub inspectors_evidence_stale: usize,
    /// Inspectors narrowed because evidence or sign-off is incomplete.
    pub inspectors_incomplete: usize,
    /// Inspectors whose gate allows the mutating-or-privileged action.
    pub gate_allow: usize,
    /// Inspectors whose gate fails closed.
    pub gate_fail_closed: usize,
    /// Total release-blocking inspectors.
    pub release_blocking_total: usize,
    /// Release-blocking inspectors publishing a label at or above the cutline.
    pub release_blocking_publishing_stable: usize,
    /// Release-blocking inspectors narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Helper/agent attach inspectors.
    pub helper_agent_attach_inspectors: usize,
    /// Extension/runtime load inspectors.
    pub extension_runtime_load_inspectors: usize,
    /// State import/restore inspectors.
    pub state_import_restore_inspectors: usize,
    /// Provider snapshot/open inspectors.
    pub provider_snapshot_open_inspectors: usize,
    /// Inspectors downgrading the helper subject.
    pub helper_subject_inspectors: usize,
    /// Inspectors downgrading the agent subject.
    pub agent_subject_inspectors: usize,
    /// Inspectors downgrading the host subject.
    pub host_subject_inspectors: usize,
    /// Inspectors downgrading the schema subject.
    pub schema_subject_inspectors: usize,
    /// Inspectors downgrading the provider subject.
    pub provider_subject_inspectors: usize,
    /// Inspectors with the inside-window verdict.
    pub verdict_inside_window: usize,
    /// Inspectors with the unsupported-skew verdict.
    pub verdict_unsupported_skew: usize,
    /// Inspectors with the reconnect-required verdict.
    pub verdict_reconnect_required: usize,
    /// Inspectors with the reinstall-required verdict.
    pub verdict_reinstall_required: usize,
    /// Inspectors with the migration-needed verdict.
    pub verdict_migration_needed: usize,
    /// Inspectors with the retest-pending verdict.
    pub verdict_retest_pending: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all inspectors.
    pub total_active_narrowing_reasons: usize,
    /// Total upgrade steps across all inspectors.
    pub total_upgrade_steps: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundarySkewInspectorExportRow {
    /// Stable inspector id.
    pub entry_id: String,
    /// The boundary-crossing flow this inspector guards.
    pub boundary_kind: BoundaryKind,
    /// The boundary ref this inspector speaks about.
    pub boundary_ref: String,
    /// The helper/agent/host/schema/provider subject this inspector downgrades.
    pub downgrade_subject: DowngradeSubject,
    /// The mutating-or-privileged action gated by this inspector.
    pub gated_action: GatedAction,
    /// Whether the boundary is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim entry id this boundary backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the inspector publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall inspector state earned.
    pub inspector_state: InspectorState,
    /// The declared supported skew class.
    pub skew_window_class: SkewWindowClass,
    /// The verdict reported before the gated action runs.
    pub verdict: InspectorVerdict,
    /// The posture applied to the gated action.
    pub gate_posture: GatePosture,
    /// Which side upgrades first to recover the boundary.
    pub upgrade_lead_side: UpgradeLeadSide,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, release-center, service-health, support, and
/// export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundarySkewInspectorExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<BoundarySkewInspectorExportRow>,
}

/// The typed M5 mixed-version boundary skew-inspector register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundarySkewInspectorRegister {
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
    /// Ref to the qualification/skew matrix whose vocabulary this register reuses.
    pub qualification_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed boundary-kind vocabulary.
    pub boundary_kinds: Vec<BoundaryKind>,
    /// Closed downgrade-subject vocabulary.
    pub downgrade_subjects: Vec<DowngradeSubject>,
    /// Closed gated-action vocabulary.
    pub gated_actions: Vec<GatedAction>,
    /// Closed action-risk vocabulary.
    pub action_risks: Vec<ActionRisk>,
    /// Closed skew-window-class vocabulary.
    pub skew_window_classes: Vec<SkewWindowClass>,
    /// Closed inspector-verdict vocabulary.
    pub inspector_verdicts: Vec<InspectorVerdict>,
    /// Closed gate-posture vocabulary.
    pub gate_postures: Vec<GatePosture>,
    /// Closed upgrade-lead-side vocabulary.
    pub upgrade_lead_sides: Vec<UpgradeLeadSide>,
    /// Closed inspector-state vocabulary.
    pub inspector_states: Vec<InspectorState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking boundary refs this register must cover.
    pub release_blocking_boundary_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<InspectorStopRule>,
    /// Boundary inspectors.
    pub inspectors: Vec<BoundaryInspector>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: BoundarySkewInspectorRegisterSummary,
}

impl BoundarySkewInspectorRegister {
    /// Returns the inspector registered for `entry_id`.
    pub fn inspector(&self, entry_id: &str) -> Option<&BoundaryInspector> {
        self.inspectors.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the inspectors publishing a label at or above the cutline.
    pub fn inspectors_publishing_stable(&self) -> Vec<&BoundaryInspector> {
        self.inspectors
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the inspectors narrowed below the cutline.
    pub fn inspectors_narrowed(&self) -> Vec<&BoundaryInspector> {
        self.inspectors
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking inspectors.
    pub fn release_blocking_inspectors(&self) -> Vec<&BoundaryInspector> {
        self.inspectors
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the inspectors for one boundary kind.
    pub fn inspectors_for_kind(&self, kind: BoundaryKind) -> Vec<&BoundaryInspector> {
        self.inspectors
            .iter()
            .filter(|row| row.boundary_kind == kind)
            .collect()
    }

    /// Returns the inspectors for one downgrade subject.
    pub fn inspectors_for_subject(&self, subject: DowngradeSubject) -> Vec<&BoundaryInspector> {
        self.inspectors
            .iter()
            .filter(|row| row.downgrade_subject == subject)
            .collect()
    }

    /// Distinct boundaries (by boundary ref) the register covers.
    pub fn boundaries(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.inspectors {
            set.insert(row.boundary_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched inspector carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &InspectorStopRule) -> bool {
        self.inspectors.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the inspectors and stop rules.
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

    /// Inspector ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only inspectors whose claim is at or above the cutline count: an inspector
    /// whose claim is already canonically narrowed is not a *promotion* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.inspectors {
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

    /// Recomputes the summary block from the inspectors and stop rules.
    pub fn computed_summary(&self) -> BoundarySkewInspectorRegisterSummary {
        let packets = |state: FreshnessSloState| {
            self.inspectors
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: BoundaryKind| self.inspectors_for_kind(kind).len();
        let subject = |subject: DowngradeSubject| self.inspectors_for_subject(subject).len();
        let state = |state: InspectorState| {
            self.inspectors
                .iter()
                .filter(|row| row.inspector_state == state)
                .count()
        };
        let verdict = |verdict: InspectorVerdict| {
            self.inspectors
                .iter()
                .filter(|row| row.verdict == verdict)
                .count()
        };
        let gate = |posture: GatePosture| {
            self.inspectors
                .iter()
                .filter(|row| row.gate_posture == posture)
                .count()
        };
        let release_blocking: Vec<&BoundaryInspector> = self.release_blocking_inspectors();
        BoundarySkewInspectorRegisterSummary {
            total_inspectors: self.inspectors.len(),
            total_boundaries: self.boundaries().len(),
            inspectors_publishing_stable: self.inspectors_publishing_stable().len(),
            inspectors_narrowed: self.inspectors_narrowed().len(),
            inspectors_holding: self
                .inspectors
                .iter()
                .filter(|row| row.holds_label())
                .count(),
            inspectors_on_waiver: state(InspectorState::OnWaiver),
            inspectors_limited: state(InspectorState::Limited),
            inspectors_fail_closed: state(InspectorState::FailClosed),
            inspectors_retest_pending: state(InspectorState::RetestPending),
            inspectors_evidence_stale: state(InspectorState::EvidenceStale),
            inspectors_incomplete: state(InspectorState::Incomplete),
            gate_allow: gate(GatePosture::Allow),
            gate_fail_closed: gate(GatePosture::FailClosed),
            release_blocking_total: release_blocking.len(),
            release_blocking_publishing_stable: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            helper_agent_attach_inspectors: kind(BoundaryKind::HelperAgentAttach),
            extension_runtime_load_inspectors: kind(BoundaryKind::ExtensionRuntimeLoad),
            state_import_restore_inspectors: kind(BoundaryKind::StateImportRestore),
            provider_snapshot_open_inspectors: kind(BoundaryKind::ProviderSnapshotOpen),
            helper_subject_inspectors: subject(DowngradeSubject::Helper),
            agent_subject_inspectors: subject(DowngradeSubject::Agent),
            host_subject_inspectors: subject(DowngradeSubject::Host),
            schema_subject_inspectors: subject(DowngradeSubject::Schema),
            provider_subject_inspectors: subject(DowngradeSubject::Provider),
            verdict_inside_window: verdict(InspectorVerdict::InsideWindow),
            verdict_unsupported_skew: verdict(InspectorVerdict::UnsupportedSkew),
            verdict_reconnect_required: verdict(InspectorVerdict::ReconnectRequired),
            verdict_reinstall_required: verdict(InspectorVerdict::ReinstallRequired),
            verdict_migration_needed: verdict(InspectorVerdict::MigrationNeeded),
            verdict_retest_pending: verdict(InspectorVerdict::RetestPending),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .inspectors
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_upgrade_steps: self
                .inspectors
                .iter()
                .map(|row| row.upgrade_order_guide.steps.len())
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
    pub fn support_export_projection(&self) -> BoundarySkewInspectorExportProjection {
        BoundarySkewInspectorExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .inspectors
                .iter()
                .map(|row| BoundarySkewInspectorExportRow {
                    entry_id: row.entry_id.clone(),
                    boundary_kind: row.boundary_kind,
                    boundary_ref: row.boundary_ref.clone(),
                    downgrade_subject: row.downgrade_subject,
                    gated_action: row.gated_action,
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    inspector_state: row.inspector_state,
                    skew_window_class: row.skew_window.skew_window_class,
                    verdict: row.verdict,
                    gate_posture: row.gate_posture,
                    upgrade_lead_side: row.upgrade_order_guide.lead_side,
                    slo_state: row.proof_packet.slo_state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<BoundarySkewInspectorViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.inspectors {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(BoundarySkewInspectorViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_inspector(row, &mut violations);
        }
        if self.inspectors.is_empty() {
            violations.push(BoundarySkewInspectorViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(BoundarySkewInspectorViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<BoundarySkewInspectorViolation>) {
        if self.schema_version != SHIP_M5_BOUNDARY_SKEW_INSPECTORS_SCHEMA_VERSION {
            violations.push(BoundarySkewInspectorViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != SHIP_M5_BOUNDARY_SKEW_INSPECTORS_RECORD_KIND {
            violations.push(BoundarySkewInspectorViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(BoundarySkewInspectorViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.boundary_kinds != BoundaryKind::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "boundary_kinds",
            });
        }
        if self.downgrade_subjects != DowngradeSubject::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "downgrade_subjects",
            });
        }
        if self.gated_actions != GatedAction::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "gated_actions",
            });
        }
        if self.action_risks != ActionRisk::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "action_risks",
            });
        }
        if self.skew_window_classes != SkewWindowClass::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "skew_window_classes",
            });
        }
        if self.inspector_verdicts != InspectorVerdict::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "inspector_verdicts",
            });
        }
        if self.gate_postures != GatePosture::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "gate_postures",
            });
        }
        if self.upgrade_lead_sides != UpgradeLeadSide::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "upgrade_lead_sides",
            });
        }
        if self.inspector_states != InspectorState::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "inspector_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(BoundarySkewInspectorViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(BoundarySkewInspectorViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<BoundarySkewInspectorViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(BoundarySkewInspectorViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(BoundarySkewInspectorViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(BoundarySkewInspectorViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(BoundarySkewInspectorViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(
                    BoundarySkewInspectorViolation::NarrowingReasonWithoutStopRule { reason },
                );
            }
        }
    }

    fn validate_inspector(
        &self,
        row: &BoundaryInspector,
        violations: &mut Vec<BoundarySkewInspectorViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("boundary_ref", &row.boundary_ref),
            ("boundary_summary", &row.boundary_summary),
            ("local_role", &row.local_role),
            ("peer_role", &row.peer_role),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("skew_window.local_version", &row.skew_window.local_version),
            ("skew_window.peer_version", &row.skew_window.peer_version),
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
                "upgrade_order_guide.guide_ref",
                &row.upgrade_order_guide.guide_ref,
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
                violations.push(BoundarySkewInspectorViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // The gated action must match the boundary-crossing flow it guards.
        if row.gated_action != row.boundary_kind.gated_action() {
            violations.push(BoundarySkewInspectorViolation::GatedActionMismatch {
                entry_id: row.entry_id.clone(),
                boundary_kind: row.boundary_kind,
                gated_action: row.gated_action,
            });
        }

        // The gate allows the action exactly when the verdict is inside-window.
        let should_allow = row.verdict.is_inside_window();
        if row.action_allowed() != should_allow {
            violations.push(BoundarySkewInspectorViolation::GatePostureIncoherent {
                entry_id: row.entry_id.clone(),
                verdict: row.verdict,
                gate_posture: row.gate_posture,
            });
        }

        self.validate_upgrade_guide(row, violations);

        // The ceiling: no boundary may carry a label wider than the claim's label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(BoundarySkewInspectorViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(BoundarySkewInspectorViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(BoundarySkewInspectorViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_skew_window(row, violations);
        self.validate_caveats(row, violations);

        // A claim whose canonical label is below the cutline forces the boundary to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(BoundarySkewInspectorViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(BoundarySkewInspectorViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.inspector_state,
                });
            }
        }

        // A non-inside-window verdict must drop the claim below the cutline and name
        // the reason the verdict maps to.
        if !row.verdict.is_inside_window() {
            if row.holds_label() {
                violations.push(BoundarySkewInspectorViolation::FailClosedHeld {
                    entry_id: row.entry_id.clone(),
                    verdict: row.verdict,
                });
            }
            if let Some(reason) = row.verdict.reason() {
                if !row.has_active_reason(reason) {
                    violations.push(BoundarySkewInspectorViolation::VerdictReasonNotActive {
                        entry_id: row.entry_id.clone(),
                        verdict: row.verdict,
                        reason,
                    });
                }
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A backed boundary carries the claim's canonical label, carries no
            // active reason, runs a current verdict, rides a captured within-SLO
            // packet, and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(BoundarySkewInspectorViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(BoundarySkewInspectorViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(BoundarySkewInspectorViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(BoundarySkewInspectorViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(BoundarySkewInspectorViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(BoundarySkewInspectorViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.inspector_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(BoundarySkewInspectorViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.inspector_state,
                });
            }
            // A narrowing boundary whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::EvidenceStale)
            {
                violations.push(
                    BoundarySkewInspectorViolation::BreachedPacketWithoutReason {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::EvidenceMissing)
            {
                violations.push(BoundarySkewInspectorViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_coherence(row, violations);
    }

    fn validate_upgrade_guide(
        &self,
        row: &BoundaryInspector,
        violations: &mut Vec<BoundarySkewInspectorViolation>,
    ) {
        let guide = &row.upgrade_order_guide;
        for (index, step) in guide.steps.iter().enumerate() {
            if step.order == 0 {
                violations.push(BoundarySkewInspectorViolation::UpgradeStepOrderZero {
                    entry_id: row.entry_id.clone(),
                    index,
                });
            }
            if step.instruction.trim().is_empty() {
                violations.push(BoundarySkewInspectorViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "upgrade_order_guide.steps[].instruction",
                });
            }
        }
        // A skew-recovery verdict must carry an upgrade-order guide with a leading
        // side and at least one step; any other verdict must carry none.
        if row.verdict.requires_upgrade_guide() {
            if !guide.lead_side.requires_upgrade() || guide.steps.is_empty() {
                violations.push(BoundarySkewInspectorViolation::UpgradeGuideMissing {
                    entry_id: row.entry_id.clone(),
                    verdict: row.verdict,
                });
            }
        } else if guide.lead_side.requires_upgrade() || !guide.steps.is_empty() {
            violations.push(BoundarySkewInspectorViolation::UpgradeGuideUnexpected {
                entry_id: row.entry_id.clone(),
                verdict: row.verdict,
            });
        }
    }

    fn validate_skew_window(
        &self,
        row: &BoundaryInspector,
        violations: &mut Vec<BoundarySkewInspectorViolation>,
    ) {
        // A boundary declared in an unsupported skew class must narrow and name the
        // skew reason.
        if !row.skew_window.skew_window_class.is_supported() {
            if row.holds_label() {
                violations.push(BoundarySkewInspectorViolation::UnsupportedSkewHeld {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.has_active_reason(NarrowingReason::SkewWindowExceeded) {
                violations.push(BoundarySkewInspectorViolation::SkewExceededWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_caveats(
        &self,
        row: &BoundaryInspector,
        violations: &mut Vec<BoundarySkewInspectorViolation>,
    ) {
        // A limited inspector must record at least one compatibility caveat.
        if row.inspector_state == InspectorState::Limited
            && row
                .compatibility_caveats
                .iter()
                .all(|c| c.trim().is_empty())
        {
            violations.push(BoundarySkewInspectorViolation::LimitedWithoutCaveat {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_state_coherence(
        &self,
        row: &BoundaryInspector,
        violations: &mut Vec<BoundarySkewInspectorViolation>,
    ) {
        let push_verdict = |violations: &mut Vec<BoundarySkewInspectorViolation>| {
            violations.push(BoundarySkewInspectorViolation::StateVerdictIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.inspector_state,
                verdict: row.verdict,
            });
        };
        let push_reason = |violations: &mut Vec<BoundarySkewInspectorViolation>,
                           expected: NarrowingReason| {
            violations.push(BoundarySkewInspectorViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.inspector_state,
                expected_reason: expected,
            });
        };

        match row.inspector_state {
            InspectorState::InsideWindow | InspectorState::Limited => {
                if !row.verdict.is_inside_window() {
                    push_verdict(violations);
                }
            }
            InspectorState::OnWaiver => {
                if !row.verdict.is_inside_window() {
                    push_verdict(violations);
                }
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(BoundarySkewInspectorViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.inspector_state,
                    });
                }
            }
            InspectorState::FailClosed => {
                if !row.verdict.requires_upgrade_guide() {
                    push_verdict(violations);
                }
            }
            InspectorState::RetestPending => {
                if row.verdict != InspectorVerdict::RetestPending {
                    push_verdict(violations);
                }
                if !row.has_active_reason(NarrowingReason::RetestPending) {
                    push_reason(violations, NarrowingReason::RetestPending);
                }
            }
            InspectorState::EvidenceStale => {
                if !row.has_active_reason(NarrowingReason::EvidenceStale) {
                    push_reason(violations, NarrowingReason::EvidenceStale);
                }
            }
            InspectorState::Incomplete => {
                if !row.has_active_reason(NarrowingReason::EvidenceMissing)
                    && !row.has_active_reason(NarrowingReason::OwnerSignoffMissing)
                    && !row.has_active_reason(NarrowingReason::ClaimPublicationMissing)
                {
                    push_reason(violations, NarrowingReason::EvidenceMissing);
                }
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<BoundarySkewInspectorViolation>) {
        let covered: BTreeSet<String> = self
            .inspectors
            .iter()
            .map(|row| row.boundary_ref.clone())
            .collect();
        for declared in &self.release_blocking_boundary_refs {
            if !covered.contains(declared) {
                violations.push(
                    BoundarySkewInspectorViolation::ReleaseBlockingBoundaryUncovered {
                        boundary_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.inspectors {
            if row.release_blocking
                && !self
                    .release_blocking_boundary_refs
                    .contains(&row.boundary_ref)
            {
                violations.push(
                    BoundarySkewInspectorViolation::ReleaseBlockingRowNotDeclared {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<BoundarySkewInspectorViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(BoundarySkewInspectorViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(BoundarySkewInspectorViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(
                BoundarySkewInspectorViolation::PromotionDecisionInconsistent {
                    declared: self.promotion.decision,
                    computed,
                },
            );
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                BoundarySkewInspectorViolation::PromotionBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(
                BoundarySkewInspectorViolation::PromotionBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 boundary skew-inspector register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundarySkewInspectorViolation {
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
    /// The register has no inspectors.
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
    /// An inspector id appears more than once.
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
    /// The gated action does not match the boundary-crossing flow.
    GatedActionMismatch {
        /// Row id.
        entry_id: String,
        /// Boundary kind.
        boundary_kind: BoundaryKind,
        /// Declared gated action.
        gated_action: GatedAction,
    },
    /// The gate posture disagrees with the verdict.
    GatePostureIncoherent {
        /// Row id.
        entry_id: String,
        /// Verdict.
        verdict: InspectorVerdict,
        /// Gate posture.
        gate_posture: GatePosture,
    },
    /// A skew-recovery verdict carries no upgrade-order guide.
    UpgradeGuideMissing {
        /// Row id.
        entry_id: String,
        /// Verdict.
        verdict: InspectorVerdict,
    },
    /// A non-skew-recovery verdict carries an unexpected upgrade-order guide.
    UpgradeGuideUnexpected {
        /// Row id.
        entry_id: String,
        /// Verdict.
        verdict: InspectorVerdict,
    },
    /// An upgrade step has a zero order.
    UpgradeStepOrderZero {
        /// Row id.
        entry_id: String,
        /// Step index.
        index: usize,
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
    /// An inspector holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A non-inside-window verdict holds its label.
    FailClosedHeld {
        /// Row id.
        entry_id: String,
        /// Verdict.
        verdict: InspectorVerdict,
    },
    /// A non-inside-window verdict does not name its narrowing reason.
    VerdictReasonNotActive {
        /// Row id.
        entry_id: String,
        /// Verdict.
        verdict: InspectorVerdict,
        /// The reason the verdict requires.
        reason: NarrowingReason,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// Inspector state.
        state: InspectorState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// Inspector state.
        state: InspectorState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held inspector carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held inspector carries active narrowing reasons.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held inspector has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held inspector rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held inspector lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing inspector with a breached proof packet does not name the stale
    /// reason.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowing inspector with a missing proof packet does not name the missing
    /// reason.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A boundary declared in an unsupported skew class holds its label.
    UnsupportedSkewHeld {
        /// Row id.
        entry_id: String,
    },
    /// A boundary declared in an unsupported skew class does not name the skew
    /// reason.
    SkewExceededWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A limited inspector records no compatibility caveat.
    LimitedWithoutCaveat {
        /// Row id.
        entry_id: String,
    },
    /// An inspector state is incoherent with its verdict.
    StateVerdictIncoherent {
        /// Row id.
        entry_id: String,
        /// Inspector state.
        state: InspectorState,
        /// Verdict.
        verdict: InspectorVerdict,
    },
    /// An inspector state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// Inspector state.
        state: InspectorState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// Inspector state.
        state: InspectorState,
    },
    /// A release-blocking boundary ref has no covering inspector.
    ReleaseBlockingBoundaryUncovered {
        /// Boundary ref.
        boundary_ref: String,
    },
    /// A release-blocking inspector is not declared in the release-blocking list.
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
    /// The summary counts disagree with the inspectors.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for BoundarySkewInspectorViolation {
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
            Self::EmptyRegister => write!(f, "register has no inspectors"),
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
            Self::GatedActionMismatch {
                entry_id,
                boundary_kind,
                gated_action,
            } => write!(
                f,
                "inspector {entry_id} boundary {} does not gate action {}",
                boundary_kind.as_str(),
                gated_action.as_str()
            ),
            Self::GatePostureIncoherent {
                entry_id,
                verdict,
                gate_posture,
            } => write!(
                f,
                "inspector {entry_id} gate {} is incoherent with verdict {}",
                gate_posture.as_str(),
                verdict.as_str()
            ),
            Self::UpgradeGuideMissing { entry_id, verdict } => write!(
                f,
                "inspector {entry_id} verdict {} carries no upgrade-order guide",
                verdict.as_str()
            ),
            Self::UpgradeGuideUnexpected { entry_id, verdict } => write!(
                f,
                "inspector {entry_id} verdict {} carries an unexpected upgrade-order guide",
                verdict.as_str()
            ),
            Self::UpgradeStepOrderZero { entry_id, index } => {
                write!(f, "inspector {entry_id} upgrade step {index} has order 0")
            }
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "inspector {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "inspector {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::FailClosedHeld { entry_id, verdict } => write!(
                f,
                "inspector {entry_id} holds label on fail-closed verdict {}",
                verdict.as_str()
            ),
            Self::VerdictReasonNotActive {
                entry_id,
                verdict,
                reason,
            } => write!(
                f,
                "inspector {entry_id} verdict {} requires active reason {}",
                verdict.as_str(),
                reason.as_str()
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "inspector {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "inspector {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "inspector {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "inspector {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "inspector {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "inspector {entry_id} holds stable on stale packet {slo_state:?}"
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "inspector {entry_id} holds stable without owner signoff")
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "inspector {entry_id} breached packet without evidence_stale reason"
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "inspector {entry_id} missing packet without evidence_missing reason"
                )
            }
            Self::UnsupportedSkewHeld { entry_id } => {
                write!(f, "inspector {entry_id} holds label on unsupported skew")
            }
            Self::SkewExceededWithoutReason { entry_id } => {
                write!(
                    f,
                    "inspector {entry_id} unsupported skew without skew_window_exceeded reason"
                )
            }
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(
                    f,
                    "inspector {entry_id} is limited without a compatibility caveat"
                )
            }
            Self::StateVerdictIncoherent {
                entry_id,
                state,
                verdict,
            } => write!(
                f,
                "inspector {entry_id} state {state:?} is incoherent with verdict {}",
                verdict.as_str()
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "inspector {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "inspector {entry_id} state {state:?} names no waiver")
            }
            Self::ReleaseBlockingBoundaryUncovered { boundary_ref } => {
                write!(
                    f,
                    "release-blocking boundary {boundary_ref} has no covering inspector"
                )
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => {
                write!(
                    f,
                    "release-blocking inspector {entry_id} is not declared in release_blocking_boundary_refs"
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
            Self::SummaryMismatch => write!(f, "summary counts disagree with inspectors"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(
                    f,
                    "inspector {entry_id} freshness SLO window is inconsistent"
                )
            }
        }
    }
}

impl Error for BoundarySkewInspectorViolation {}

/// Loads the embedded M5 boundary skew-inspector register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`BoundarySkewInspectorRegister`].
pub fn current_m5_boundary_skew_inspectors(
) -> Result<BoundarySkewInspectorRegister, serde_json::Error> {
    serde_json::from_str(SHIP_M5_BOUNDARY_SKEW_INSPECTORS_JSON)
}

#[cfg(test)]
mod tests;
