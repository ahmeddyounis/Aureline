//! Implemented M5 host-boundary-strip and execution-origin-receipt-row primitives.
//!
//! The frozen [build/remote-boundary component matrix][matrix] names the reusable build / remote /
//! managed-workspace boundary UI components and locks their controlled vocabulary. This module is
//! the second implement lane over that matrix: it turns the two runtime-ownership components — the
//! **host-boundary strip** and the **execution-origin receipt row** — into resolvers that produce
//! export-safe, honest projections instead of feature-local host or origin copy.
//!
//! Three acceptance criteria drive the resolvers:
//!
//! * **AC1 — users can distinguish local, SSH, container, devcontainer, managed, and browser-bridge
//!   execution without opening a separate inspector.** [`resolve_host_boundary_strip`] refuses to
//!   read as a clean strip unless it names its locality class, its target label, its owning
//!   runtime / service lane, and — whenever the connection is impaired — its reconnect / degraded
//!   state. A clean strip always carries one of the seven [`M5HostBoundaryLocality`] classes so a
//!   user can read where work is running before they trust logs, previews, shells, or actions.
//! * **AC2 — receipts remain stable enough for diagnostics, support exports, and release evidence to
//!   reuse them without rewriting target lineage.** [`resolve_execution_origin_receipt_row`] degrades
//!   to [`M5ExecutionOriginReceiptRowDegradeReason::LineageNotExportSafe`] the moment a receipt is
//!   presented without an export-safe, reusable target lineage, and never lets a row read as a clean
//!   receipt when its action class, resolved target identity, or execution-context provenance is
//!   missing.
//! * **Host ownership never disappears** — [`resolve_execution_origin_receipt_row`] degrades to
//!   [`M5ExecutionOriginReceiptRowDegradeReason::OwnershipDroppedOnRestore`] whenever a surface is
//!   restored, handed off, or degraded and the execution origin is dropped instead of carried
//!   through, so host ownership can never silently vanish on recovery.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5BuildRemoteBoundaryDisposition`] boundary-disposition vocabulary, the frozen
//! [`M5BuildRemoteDowngradeTrigger`] downgrade-trigger vocabulary — and bind the locality class,
//! origin locus, connection state, receipt state, attribution confidence, and host-narrowing reason
//! directly to the frozen M5 execution object model ([`HostKind`], [`OriginLocus`],
//! [`ConnectionState`], [`OriginReceiptState`], [`AttributionConfidence`], and
//! [`HostNarrowingReason`]), so this lane can never fork its own host, origin, or continuity wording.
//!
//! [matrix]: crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_host_origin_controls,
    seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed,
    seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed,
    M5_HOST_ORIGIN_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_execution::m5_host_boundary::{
    AttributionConfidence, ConnectionState, HostKind, HostNarrowingReason, OriginLocus,
    OriginReceiptState, M5_HOST_BOUNDARY_PATH,
};

use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::{
    M5BuildRemoteAccessibilityRoute, M5BuildRemoteBoundaryDisposition, M5BuildRemoteConsumerSurface,
    M5BuildRemoteDeploymentLine, M5BuildRemoteDowngradeTrigger, M5BuildRemoteQualificationClass,
    M5BuildRemoteRequiredLabel, M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
    M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF, M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
    M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5HostOriginControlsPacket`].
pub const M5_HOST_ORIGIN_CONTROLS_RECORD_KIND: &str =
    "implement_m5_host_boundary_strip_and_execution_origin_receipt_row_controls";

/// Schema version for M5 host-boundary-strip / execution-origin-receipt-row controls records.
pub const M5_HOST_ORIGIN_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_HOST_ORIGIN_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-host-boundary-strip-execution-origin-receipt-row-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_HOST_ORIGIN_CONTROLS_DOC_REF: &str =
    "docs/remote/m5_host_boundary_strip_and_execution_origin_receipt_row_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_HOST_ORIGIN_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-host-boundary-strip-execution-origin-receipt-row-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_HOST_ORIGIN_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-host-boundary-strip-execution-origin-receipt-row-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_HOST_ORIGIN_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-host-boundary-strip-execution-origin-receipt-row-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_HOST_ORIGIN_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-host-boundary-strip-execution-origin-receipt-row-controls";

/// Consumer surface a host-origin controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5HostOriginConsumerSurface = M5BuildRemoteConsumerSurface;

/// The single controlled locality class a resolved host-boundary strip carries. These are the exact
/// execution classes the spec requires a user to be able to tell apart before they trust logs,
/// previews, shells, or actions: local, SSH, container, devcontainer, managed, browser-bridge, or
/// service-plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostBoundaryLocality {
    /// Work runs on the local desktop host.
    Local,
    /// Work runs on a remote host reached over SSH.
    Ssh,
    /// Work runs in a container runtime.
    Container,
    /// Work runs in a development container.
    Devcontainer,
    /// Work runs on a managed cloud workspace.
    Managed,
    /// Work runs across a browser / companion bridge.
    BrowserBridge,
    /// Work runs on a connector-backed service plane.
    ServicePlane,
}

impl M5HostBoundaryLocality {
    /// Every locality class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Local,
        Self::Ssh,
        Self::Container,
        Self::Devcontainer,
        Self::Managed,
        Self::BrowserBridge,
        Self::ServicePlane,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Ssh => "ssh",
            Self::Container => "container",
            Self::Devcontainer => "devcontainer",
            Self::Managed => "managed",
            Self::BrowserBridge => "browser_bridge",
            Self::ServicePlane => "service_plane",
        }
    }

    /// Whether this is the one local, first-party locality.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::Local)
    }

    /// Whether work in this locality crossed a remote, managed, bridged, or service-plane boundary.
    pub const fn crosses_boundary(self) -> bool {
        !self.is_local()
    }
}

/// One mandatory rendered part a host-boundary strip or execution-origin receipt row must be able to
/// show, so no host-ownership or execution-origin truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostOriginAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The current locality class (strip).
    LocalityClass,
    /// The resolved target label (strip).
    TargetLabel,
    /// The owning runtime / service lane (strip).
    OwningRuntimeLane,
    /// The reconnect / degraded state (strip).
    ReconnectDegradedState,
    /// The open-details affordance (strip).
    OpenDetailsAffordance,
    /// The action class the receipt attests (row).
    ActionClass,
    /// The resolved target identity (row).
    ResolvedTargetIdentity,
    /// The execution-context provenance (row).
    ExecutionContextProvenance,
    /// The export-safe target lineage (row).
    ExportSafeLineage,
}

impl M5HostOriginAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::LocalityClass,
        Self::TargetLabel,
        Self::OwningRuntimeLane,
        Self::ReconnectDegradedState,
        Self::OpenDetailsAffordance,
        Self::ActionClass,
        Self::ResolvedTargetIdentity,
        Self::ExecutionContextProvenance,
        Self::ExportSafeLineage,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::LocalityClass => "locality_class",
            Self::TargetLabel => "target_label",
            Self::OwningRuntimeLane => "owning_runtime_lane",
            Self::ReconnectDegradedState => "reconnect_degraded_state",
            Self::OpenDetailsAffordance => "open_details_affordance",
            Self::ActionClass => "action_class",
            Self::ResolvedTargetIdentity => "resolved_target_identity",
            Self::ExecutionContextProvenance => "execution_context_provenance",
            Self::ExportSafeLineage => "export_safe_lineage",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostOriginNextAction {
    /// Open the host-boundary details.
    OpenBoundaryDetails,
    /// View the resolved execution origin.
    ViewExecutionOrigin,
    /// Reconnect the host.
    ReconnectHost,
    /// Review the degraded host context.
    ReviewDegradedContext,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5HostOriginNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenBoundaryDetails,
        Self::ViewExecutionOrigin,
        Self::ReconnectHost,
        Self::ReviewDegradedContext,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenBoundaryDetails => "open_boundary_details",
            Self::ViewExecutionOrigin => "view_execution_origin",
            Self::ReconnectHost => "reconnect_host",
            Self::ReviewDegradedContext => "review_degraded_context",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a host-origin controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostOriginExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The locality classes carried.
    Localities,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The origin confidences carried by the receipts.
    OriginConfidences,
    /// The host kind named by the strip.
    HostKind,
    /// The origin locus named by the strip / receipt.
    OriginLocus,
    /// The target label named by the strip.
    TargetLabel,
    /// The resolved target identity named by the receipt.
    ResolvedTargetIdentity,
    /// The accountable owner role.
    OwnerRole,
}

impl M5HostOriginExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Localities,
        Self::DegradeReasons,
        Self::Qualification,
        Self::OriginConfidences,
        Self::HostKind,
        Self::OriginLocus,
        Self::TargetLabel,
        Self::ResolvedTargetIdentity,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Localities,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Localities => "localities",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::OriginConfidences => "origin_confidences",
            Self::HostKind => "host_kind",
            Self::OriginLocus => "origin_locus",
            Self::TargetLabel => "target_label",
            Self::ResolvedTargetIdentity => "resolved_target_identity",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a host-boundary strip degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an under-labelled strip read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostBoundaryStripDegradeReason {
    /// The locality class is unstated on the strip (AC1 violation).
    LocalityClassUnstated,
    /// The resolved target label is unstated.
    TargetLabelUnstated,
    /// The owning runtime / service lane is unstated.
    OwningLaneUnstated,
    /// The connection is impaired but the reconnect / degraded state is unstated (host ownership
    /// disappears on degrade).
    ReconnectDegradedStateUnstated,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5HostBoundaryStripDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalityClassUnstated,
        Self::TargetLabelUnstated,
        Self::OwningLaneUnstated,
        Self::ReconnectDegradedStateUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalityClassUnstated => "locality_class_unstated",
            Self::TargetLabelUnstated => "target_label_unstated",
            Self::OwningLaneUnstated => "owning_lane_unstated",
            Self::ReconnectDegradedStateUnstated => "reconnect_degraded_state_unstated",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5HostOriginNextAction {
        match self {
            Self::LocalityClassUnstated | Self::TargetLabelUnstated => {
                M5HostOriginNextAction::OpenBoundaryDetails
            }
            Self::OwningLaneUnstated => M5HostOriginNextAction::ViewExecutionOrigin,
            Self::ReconnectDegradedStateUnstated => M5HostOriginNextAction::ReviewDegradedContext,
            Self::ProofStale => M5HostOriginNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::LocalityClassUnstated
            | Self::OwningLaneUnstated
            | Self::ReconnectDegradedStateUnstated => {
                M5BuildRemoteDowngradeTrigger::HostBoundaryUnstated
            }
            Self::TargetLabelUnstated => M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed,
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an execution-origin receipt row degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionOriginReceiptRowDegradeReason {
    /// The action class is unstated on the receipt.
    ActionClassUnstated,
    /// The resolved target identity is unstated.
    TargetIdentityUnstated,
    /// The execution-context provenance is unstated.
    ProvenanceUnstated,
    /// The target lineage is not export-safe / reusable for diagnostics, support, or evidence (AC2
    /// violation).
    LineageNotExportSafe,
    /// The execution origin was dropped when the surface was restored, handed off, or degraded (host
    /// ownership disappeared).
    OwnershipDroppedOnRestore,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ExecutionOriginReceiptRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActionClassUnstated,
        Self::TargetIdentityUnstated,
        Self::ProvenanceUnstated,
        Self::LineageNotExportSafe,
        Self::OwnershipDroppedOnRestore,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionClassUnstated => "action_class_unstated",
            Self::TargetIdentityUnstated => "target_identity_unstated",
            Self::ProvenanceUnstated => "provenance_unstated",
            Self::LineageNotExportSafe => "lineage_not_export_safe",
            Self::OwnershipDroppedOnRestore => "ownership_dropped_on_restore",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5HostOriginNextAction {
        match self {
            Self::ActionClassUnstated | Self::ProofStale => {
                M5HostOriginNextAction::ReviewDiagnostics
            }
            Self::TargetIdentityUnstated
            | Self::ProvenanceUnstated
            | Self::LineageNotExportSafe => M5HostOriginNextAction::ViewExecutionOrigin,
            Self::OwnershipDroppedOnRestore => M5HostOriginNextAction::ReconnectHost,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::ActionClassUnstated => M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed,
            Self::TargetIdentityUnstated
            | Self::ProvenanceUnstated
            | Self::LineageNotExportSafe => M5BuildRemoteDowngradeTrigger::ExecutionOriginUnstated,
            Self::OwnershipDroppedOnRestore => M5BuildRemoteDowngradeTrigger::HostBoundaryUnstated,
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
        }
    }
}

/// Maps a host kind and a devcontainer flag to the single controlled locality a user reads.
fn locality_for_strip(host_kind: HostKind, is_devcontainer: bool) -> M5HostBoundaryLocality {
    use M5HostBoundaryLocality as L;
    match host_kind {
        HostKind::Local => L::Local,
        HostKind::Ssh => L::Ssh,
        HostKind::Container => {
            if is_devcontainer {
                L::Devcontainer
            } else {
                L::Container
            }
        }
        HostKind::ManagedWorkspace => L::Managed,
        HostKind::BrowserBridge => L::BrowserBridge,
        HostKind::ServicePlane => L::ServicePlane,
    }
}

/// Whether a connection state represents an impaired (bridged, reconnecting, or stale) host that a
/// strip must disclose rather than let read as a live, direct connection.
fn connection_is_degraded(connection: ConnectionState) -> bool {
    !matches!(connection, ConnectionState::Connected)
}

/// Input to [`resolve_host_boundary_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HostBoundaryStripResolutionInput {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The host kind the work runs on.
    pub host_kind: HostKind,
    /// True when the container host is specifically a development container.
    pub is_devcontainer: bool,
    /// True when the locality class is disclosed on the strip, never inspector-only.
    pub locality_disclosed: bool,
    /// The resolved target label (empty means unstated).
    pub target_label: String,
    /// True when the target label is disclosed on the strip.
    pub target_label_disclosed: bool,
    /// The owning runtime / service lane (empty means unstated).
    pub owning_runtime_lane: String,
    /// True when the owning runtime / service lane is disclosed on the strip.
    pub owning_lane_disclosed: bool,
    /// The live connection state between the desktop and the host.
    pub connection_state: ConnectionState,
    /// True when the reconnect / degraded state is disclosed on the strip.
    pub reconnect_state_disclosed: bool,
    /// True when an open-details affordance is offered on the strip.
    pub open_details_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe host-boundary strip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedHostBoundaryStrip {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The single controlled locality class the strip carries.
    pub locality: M5HostBoundaryLocality,
    /// Host-kind token named by the strip.
    pub host_kind: String,
    /// Origin-locus token derived from the host kind.
    pub origin_locus: String,
    /// Target label named by the strip.
    pub target_label: String,
    /// Owning runtime / service lane named by the strip.
    pub owning_runtime_lane: String,
    /// Connection-state token named by the strip.
    pub connection_state: String,
    /// Whether the connection is impaired (bridged, reconnecting, or stale).
    pub is_degraded: bool,
    /// Whether an open-details affordance is offered.
    pub open_details_available: bool,
    /// Degrade reason, if the strip could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5HostBoundaryStripDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5HostOriginNextAction,
    /// AC1: whether the locality class is disclosed on the strip.
    pub locality_disclosed: bool,
    /// AC1: whether the target label is disclosed on the strip.
    pub target_label_disclosed: bool,
    /// AC1: whether the owning runtime / service lane is disclosed on the strip.
    pub owning_lane_disclosed: bool,
    /// AC1: whether the reconnect / degraded state is disclosed on the strip.
    pub reconnect_state_disclosed: bool,
}

impl M5ResolvedHostBoundaryStrip {
    /// Whether this strip reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this strip hides its locality class or owning runtime / service lane (a host-ownership
    /// violation).
    pub fn hides_host_boundary(&self) -> bool {
        !self.locality_disclosed || !self.owning_lane_disclosed
    }
}

/// Input to [`resolve_execution_origin_receipt_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExecutionOriginReceiptRowResolutionInput {
    /// Stable identity of the receipt instance.
    pub receipt_id: String,
    /// The action class the receipt attests (empty means unstated).
    pub action_class: String,
    /// True when the action class is disclosed on the receipt.
    pub action_class_disclosed: bool,
    /// The resolved target identity (empty means unstated).
    pub resolved_target_identity: String,
    /// True when the resolved target identity is disclosed on the receipt.
    pub target_identity_disclosed: bool,
    /// The host kind the work ran on.
    pub host_kind: HostKind,
    /// The origin-receipt state captured for the lane.
    pub receipt_state: OriginReceiptState,
    /// The live connection state between the desktop and the host.
    pub connection_state: ConnectionState,
    /// True when the execution-context provenance is disclosed on the receipt.
    pub provenance_disclosed: bool,
    /// True when an export-safe, reusable target lineage is present.
    pub export_safe_lineage_present: bool,
    /// True when the surface was restored, handed off, or degraded.
    pub restored_or_handed_off: bool,
    /// True when host ownership was retained across a restore / handoff / degrade.
    pub ownership_retained: bool,
    /// The host-narrowing reason attributed to the receipt, if any.
    pub host_narrowing_reason: Option<HostNarrowingReason>,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe execution-origin receipt row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedExecutionOriginReceiptRow {
    /// Stable identity of the receipt instance.
    pub receipt_id: String,
    /// Action-class token named by the receipt.
    pub action_class: String,
    /// Resolved target identity named by the receipt.
    pub resolved_target_identity: String,
    /// Host-kind token named by the receipt.
    pub host_kind: String,
    /// Origin-locus token derived from the host kind.
    pub origin_locus: String,
    /// Origin-receipt-state token named by the receipt.
    pub receipt_state: String,
    /// Connection-state token named by the receipt.
    pub connection_state: String,
    /// The origin confidence the receipt may publish.
    pub origin_confidence: AttributionConfidence,
    /// Host-narrowing-reason token attributed by the receipt, if any.
    pub host_narrowing_reason: Option<String>,
    /// Whether the surface was restored, handed off, or degraded.
    pub restored_or_handed_off: bool,
    /// Whether host ownership was retained across a restore / handoff / degrade.
    pub ownership_retained: bool,
    /// Whether an export-safe, reusable target lineage is present.
    pub export_safe_lineage_present: bool,
    /// Whether the target lineage is stable enough for diagnostics, support, and evidence to reuse.
    pub lineage_stable_for_reuse: bool,
    /// AC: whether the action class is disclosed on the receipt.
    pub action_class_disclosed: bool,
    /// AC: whether the resolved target identity is disclosed on the receipt.
    pub target_identity_disclosed: bool,
    /// AC: whether the execution-context provenance is disclosed on the receipt.
    pub provenance_disclosed: bool,
    /// Degrade reason, if the receipt could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5ExecutionOriginReceiptRowDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5HostOriginNextAction,
    /// Guardrail (MUST be `false` on a clean receipt): the execution origin was dropped when the
    /// surface was restored, handed off, or degraded.
    pub drops_ownership_on_restore: bool,
}

impl M5ResolvedExecutionOriginReceiptRow {
    /// Whether this receipt reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this receipt drops host ownership on restore, or carries a lineage that is not stable
    /// enough for reuse (an AC / guardrail violation).
    pub fn hides_origin_or_lineage(&self) -> bool {
        self.drops_ownership_on_restore || !self.lineage_stable_for_reuse
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5HostOriginResolutionError {
    /// The strip id was empty.
    EmptyStripId,
    /// The receipt id was empty.
    EmptyReceiptId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5HostOriginResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStripId => "empty_strip_id",
            Self::EmptyReceiptId => "empty_receipt_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5HostOriginResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 host-origin resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5HostOriginResolutionError {}

/// Resolves a host-boundary strip, proving AC1: a user can distinguish local, SSH, container,
/// devcontainer, managed, and browser-bridge execution — with its target label, owning runtime /
/// service lane, and reconnect / degraded state — without opening a separate inspector.
pub fn resolve_host_boundary_strip(
    input: M5HostBoundaryStripResolutionInput,
) -> Result<M5ResolvedHostBoundaryStrip, M5HostOriginResolutionError> {
    if input.strip_id.trim().is_empty() {
        return Err(M5HostOriginResolutionError::EmptyStripId);
    }
    if string_is_forbidden(&input.strip_id)
        || string_is_forbidden(&input.target_label)
        || string_is_forbidden(&input.owning_runtime_lane)
    {
        return Err(M5HostOriginResolutionError::ForbiddenMaterial);
    }

    let locality = locality_for_strip(input.host_kind, input.is_devcontainer);
    let is_degraded = connection_is_degraded(input.connection_state);

    let degrade_reason = if !input.locality_disclosed {
        Some(M5HostBoundaryStripDegradeReason::LocalityClassUnstated)
    } else if input.target_label.trim().is_empty() || !input.target_label_disclosed {
        Some(M5HostBoundaryStripDegradeReason::TargetLabelUnstated)
    } else if input.owning_runtime_lane.trim().is_empty() || !input.owning_lane_disclosed {
        Some(M5HostBoundaryStripDegradeReason::OwningLaneUnstated)
    } else if is_degraded && !input.reconnect_state_disclosed {
        Some(M5HostBoundaryStripDegradeReason::ReconnectDegradedStateUnstated)
    } else if !input.proof_fresh {
        Some(M5HostBoundaryStripDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None if is_degraded => M5HostOriginNextAction::ReviewDegradedContext,
        None => M5HostOriginNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedHostBoundaryStrip {
        strip_id: input.strip_id,
        locality,
        host_kind: input.host_kind.as_str().to_owned(),
        origin_locus: input.host_kind.locus().as_str().to_owned(),
        target_label: input.target_label,
        owning_runtime_lane: input.owning_runtime_lane,
        connection_state: input.connection_state.as_str().to_owned(),
        is_degraded,
        open_details_available: input.open_details_available,
        degrade_reason,
        next_action,
        locality_disclosed: input.locality_disclosed,
        target_label_disclosed: input.target_label_disclosed,
        owning_lane_disclosed: input.owning_lane_disclosed,
        reconnect_state_disclosed: input.reconnect_state_disclosed,
    })
}

/// Resolves an execution-origin receipt row, proving AC2 (the target lineage is export-safe and
/// stable enough for diagnostics, support, and evidence to reuse) and the host-ownership guardrail
/// (host ownership never disappears when a surface is restored, handed off, or degraded).
pub fn resolve_execution_origin_receipt_row(
    input: M5ExecutionOriginReceiptRowResolutionInput,
) -> Result<M5ResolvedExecutionOriginReceiptRow, M5HostOriginResolutionError> {
    if input.receipt_id.trim().is_empty() {
        return Err(M5HostOriginResolutionError::EmptyReceiptId);
    }
    if string_is_forbidden(&input.receipt_id)
        || string_is_forbidden(&input.action_class)
        || string_is_forbidden(&input.resolved_target_identity)
    {
        return Err(M5HostOriginResolutionError::ForbiddenMaterial);
    }

    let origin_confidence = input
        .receipt_state
        .confidence_ceiling()
        .min(input.connection_state.confidence_ceiling());
    let drops_ownership_on_restore = input.restored_or_handed_off && !input.ownership_retained;
    let lineage_stable_for_reuse = input.export_safe_lineage_present
        && input.target_identity_disclosed
        && input.provenance_disclosed;

    let degrade_reason = if input.action_class.trim().is_empty() || !input.action_class_disclosed {
        Some(M5ExecutionOriginReceiptRowDegradeReason::ActionClassUnstated)
    } else if input.resolved_target_identity.trim().is_empty() || !input.target_identity_disclosed {
        Some(M5ExecutionOriginReceiptRowDegradeReason::TargetIdentityUnstated)
    } else if !input.provenance_disclosed {
        Some(M5ExecutionOriginReceiptRowDegradeReason::ProvenanceUnstated)
    } else if !input.export_safe_lineage_present {
        Some(M5ExecutionOriginReceiptRowDegradeReason::LineageNotExportSafe)
    } else if drops_ownership_on_restore {
        Some(M5ExecutionOriginReceiptRowDegradeReason::OwnershipDroppedOnRestore)
    } else if !input.proof_fresh {
        Some(M5ExecutionOriginReceiptRowDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5HostOriginNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedExecutionOriginReceiptRow {
        receipt_id: input.receipt_id,
        action_class: input.action_class,
        resolved_target_identity: input.resolved_target_identity,
        host_kind: input.host_kind.as_str().to_owned(),
        origin_locus: input.host_kind.locus().as_str().to_owned(),
        receipt_state: input.receipt_state.as_str().to_owned(),
        connection_state: input.connection_state.as_str().to_owned(),
        origin_confidence,
        host_narrowing_reason: input.host_narrowing_reason.map(|r| r.as_str().to_owned()),
        restored_or_handed_off: input.restored_or_handed_off,
        ownership_retained: input.ownership_retained,
        export_safe_lineage_present: input.export_safe_lineage_present,
        lineage_stable_for_reuse,
        action_class_disclosed: input.action_class_disclosed,
        target_identity_disclosed: input.target_identity_disclosed,
        provenance_disclosed: input.provenance_disclosed,
        degrade_reason,
        next_action,
        drops_ownership_on_restore,
    })
}

/// One controls row: one consumer surface bound to the resolved host-boundary strip and
/// execution-origin receipt row examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5HostOriginConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5BuildRemoteQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5BuildRemoteDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5BuildRemoteRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5BuildRemoteAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5HostOriginAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5HostOriginExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    /// Resolved host-boundary strip examples.
    pub host_boundary_strip_examples: Vec<M5ResolvedHostBoundaryStrip>,
    /// Resolved execution-origin receipt row examples.
    pub execution_origin_receipt_row_examples: Vec<M5ResolvedExecutionOriginReceiptRow>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hide the host locality class or owning runtime / service lane.
    pub hides_host_locality_or_owning_lane: bool,
    /// Hard invariant: never drop the execution origin when a surface is restored, handed off, or
    /// degraded.
    pub drops_execution_origin_when_restored_or_degraded: bool,
    /// Hard invariant: never publish a receipt whose lineage is not stable enough for reuse.
    pub receipt_lineage_not_stable_for_reuse: bool,
    /// Hard invariant: never conceal a boundary or origin behind generic status wording.
    pub conceals_boundary_or_origin_in_generic_status_wording: bool,
}

impl M5HostOriginControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5HostOriginAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5HostOriginAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5HostOriginExportField> =
            self.export_fields.iter().copied().collect();
        M5HostOriginExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_host_locality_or_owning_lane
            && !self.drops_execution_origin_when_restored_or_degraded
            && !self.receipt_lineage_not_stable_for_reuse
            && !self.conceals_boundary_or_origin_in_generic_status_wording
    }

    /// True when every resolved example on this row is honest: no clean strip hides its host
    /// boundary, and no clean receipt drops ownership or carries an unstable lineage.
    fn examples_are_honest(&self) -> bool {
        self.host_boundary_strip_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.hides_host_boundary()))
            && self
                .execution_origin_receipt_row_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.hides_origin_or_lineage()))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Locality tokens.
    pub localities: Vec<String>,
    /// Host-kind tokens (bound from the host-boundary object model).
    pub host_kinds: Vec<String>,
    /// Origin-locus tokens (bound from the host-boundary object model).
    pub origin_loci: Vec<String>,
    /// Connection-state tokens (bound from the host-boundary object model).
    pub connection_states: Vec<String>,
    /// Origin-receipt-state tokens (bound from the host-boundary object model).
    pub origin_receipt_states: Vec<String>,
    /// Attribution-confidence tokens (bound from the host-boundary object model).
    pub attribution_confidences: Vec<String>,
    /// Host-narrowing-reason tokens (bound from the host-boundary object model).
    pub host_narrowing_reasons: Vec<String>,
    /// Strip degrade-reason tokens.
    pub strip_degrade_reasons: Vec<String>,
    /// Receipt degrade-reason tokens.
    pub receipt_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5HostOriginVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5BuildRemoteBoundaryDisposition::ALL, |v| v.as_str()),
            localities: tokens(&M5HostBoundaryLocality::ALL, |v| v.as_str()),
            host_kinds: tokens(&HostKind::ALL, |v| v.as_str()),
            origin_loci: tokens(&OriginLocus::ALL, |v| v.as_str()),
            connection_states: tokens(&ConnectionState::ALL, |v| v.as_str()),
            origin_receipt_states: tokens(&OriginReceiptState::ALL, |v| v.as_str()),
            attribution_confidences: tokens(&AttributionConfidence::ALL, |v| v.as_str()),
            host_narrowing_reasons: tokens(&HostNarrowingReason::ALL, |v| v.as_str()),
            strip_degrade_reasons: tokens(&M5HostBoundaryStripDegradeReason::ALL, |v| v.as_str()),
            receipt_degrade_reasons: tokens(&M5ExecutionOriginReceiptRowDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5HostOriginAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5HostOriginNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5HostOriginExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BuildRemoteConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginGovernanceReview {
    /// The strip always names its locality class and target label.
    pub strip_names_locality_and_target_label: bool,
    /// The strip always names its owning runtime / service lane and reconnect / degraded state.
    pub strip_names_owning_lane_and_reconnect_state: bool,
    /// Host locality and ownership are always explicit, never inspector-only.
    pub host_ownership_always_explicit: bool,
    /// The receipt always names its action class and resolved target identity.
    pub receipt_names_action_class_and_target_identity: bool,
    /// The receipt always names its execution-context provenance and export-safe lineage.
    pub receipt_names_provenance_and_export_safe_lineage: bool,
    /// Host ownership never disappears on restore, handoff, or degrade.
    pub host_ownership_never_disappears_on_restore: bool,
    /// Receipt lineage is always stable enough for diagnostics, support, and evidence to reuse.
    pub receipt_lineage_always_reusable: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginConsumerProjection {
    /// Run / test / debug surfaces consume the shared host-boundary vocabulary.
    pub run_test_debug_surfaces_consume_host_boundary_vocabulary: bool,
    /// Preview surfaces consume the shared host-boundary vocabulary.
    pub preview_surfaces_consume_host_boundary_vocabulary: bool,
    /// AI tool-routing surfaces consume the shared host-boundary vocabulary.
    pub ai_tool_routing_consumes_host_boundary_vocabulary: bool,
    /// Incident / ops surfaces consume the shared host-boundary vocabulary.
    pub incident_ops_consumes_host_boundary_vocabulary: bool,
    /// Support / export reads a single canonical execution-origin source.
    pub support_export_reads_single_origin_source: bool,
    /// Host-boundary language stays consistent across every surface.
    pub host_boundary_language_consistent_across_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5HostOriginControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HostOriginControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5HostOriginControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HostOriginVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HostOriginGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HostOriginConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HostOriginProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HostOriginReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 host-boundary-strip / execution-origin-receipt-row controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostOriginControlsPacket {
    /// Record kind; must equal [`M5_HOST_ORIGIN_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HOST_ORIGIN_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5HostOriginControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HostOriginVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HostOriginGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HostOriginConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HostOriginProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HostOriginReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5HostOriginControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5HostOriginControlsPacketInput) -> Self {
        Self {
            record_kind: M5_HOST_ORIGIN_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_HOST_ORIGIN_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5HostOriginControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HOST_ORIGIN_CONTROLS_RECORD_KIND {
            violations.push(M5HostOriginControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HOST_ORIGIN_CONTROLS_SCHEMA_VERSION {
            violations.push(M5HostOriginControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HostOriginControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5HostOriginControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 host-origin controls packet serializes"),
        ) {
            violations.push(M5HostOriginControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 host-origin controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,strip_examples,receipt_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .host_boundary_strip_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.execution_origin_receipt_row_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.host_boundary_strip_examples.len(),
                row.execution_origin_receipt_row_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Host-Boundary-Strip and Execution-Origin-Receipt-Row Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Localities: {}\n",
            self.vocabulary_set.localities.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Strip examples: {} / receipt examples: {}\n",
                row.host_boundary_strip_examples.len(),
                row.execution_origin_receipt_row_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5HostOriginControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HostOriginControlsViolation>),
}

impl fmt::Display for M5HostOriginControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 host-origin controls export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 host-origin controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5HostOriginControlsArtifactError {}

/// Validation failures emitted by [`M5HostOriginControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HostOriginControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (hidden host boundary, dropped ownership, or
    /// unstable lineage).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// AC1 is not proven: clean strips do not cover every locality, no locality/owning-lane/reconnect
    /// unstated strip degrades, or a clean strip hides its host boundary.
    Ac1NotProven,
    /// AC2 / host-ownership guardrail is not proven: no lineage-not-export-safe or
    /// ownership-dropped receipt degrades, no clean receipt shows a reusable lineage, or a clean
    /// receipt drops ownership / carries an unstable lineage.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5HostOriginControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::Ac1NotProven => "ac1_not_proven",
            Self::Ac2NotProven => "ac2_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_host_origin_controls_export(
) -> Result<M5HostOriginControlsPacket, M5HostOriginControlsArtifactError> {
    let packet: M5HostOriginControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-host-boundary-strip-execution-origin-receipt-row-controls-proof/support_export.json"
    )))
    .map_err(M5HostOriginControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HostOriginControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_HOST_ORIGIN_CONTROLS_SCHEMA_REF,
        M5_HOST_ORIGIN_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
        M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5HostOriginControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5HostOriginControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5HostOriginControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5HostOriginControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5HostOriginControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_HOST_BOUNDARY_STRIP_SCHEMA_REF)
            || !refs.contains(M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF)
        {
            violations.push(M5HostOriginControlsViolation::ComponentSchemaRefMissing);
        }
        if row.host_boundary_strip_examples.is_empty()
            || row.execution_origin_receipt_row_examples.is_empty()
        {
            violations.push(M5HostOriginControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5HostOriginControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5HostOriginControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.strip_names_locality_and_target_label,
        review.strip_names_owning_lane_and_reconnect_state,
        review.host_ownership_always_explicit,
        review.receipt_names_action_class_and_target_identity,
        review.receipt_names_provenance_and_export_safe_lineage,
        review.host_ownership_never_disappears_on_restore,
        review.receipt_lineage_always_reusable,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5HostOriginControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.run_test_debug_surfaces_consume_host_boundary_vocabulary,
        projection.preview_surfaces_consume_host_boundary_vocabulary,
        projection.ai_tool_routing_consumes_host_boundary_vocabulary,
        projection.incident_ops_consumes_host_boundary_vocabulary,
        projection.support_export_reads_single_origin_source,
        projection.host_boundary_language_consistent_across_surfaces,
    ] {
        if !ok {
            violations.push(M5HostOriginControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5HostOriginControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5HostOriginControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5HostOriginControlsPacket,
    violations: &mut Vec<M5HostOriginControlsViolation>,
) {
    let strip_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.host_boundary_strip_examples.iter())
    };
    let receipt_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.execution_origin_receipt_row_examples.iter())
    };

    // AC1: a user can distinguish local / SSH / container / devcontainer / managed / browser-bridge
    // (and service-plane) before invoking — clean strips cover every locality, a locality-unstated
    // strip degrades, an owning-lane-unstated strip degrades, a reconnect/degraded-unstated strip
    // degrades, and no clean strip hides its host boundary.
    let clean_localities: BTreeSet<&str> = strip_examples()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.locality.as_str())
        .collect();
    let covers_all_localities = M5HostBoundaryLocality::ALL
        .iter()
        .all(|locality| clean_localities.contains(locality.as_str()));
    let locality_unstated_degrades = strip_examples().any(|ex| {
        ex.degrade_reason == Some(M5HostBoundaryStripDegradeReason::LocalityClassUnstated)
    });
    let owning_lane_unstated_degrades = strip_examples()
        .any(|ex| ex.degrade_reason == Some(M5HostBoundaryStripDegradeReason::OwningLaneUnstated));
    let reconnect_unstated_degrades = strip_examples().any(|ex| {
        ex.degrade_reason == Some(M5HostBoundaryStripDegradeReason::ReconnectDegradedStateUnstated)
    });
    let no_clean_strip_hides =
        strip_examples().all(|ex| !(ex.is_clean() && ex.hides_host_boundary()));
    if !(covers_all_localities
        && locality_unstated_degrades
        && owning_lane_unstated_degrades
        && reconnect_unstated_degrades
        && no_clean_strip_hides)
    {
        violations.push(M5HostOriginControlsViolation::Ac1NotProven);
    }

    // AC2 + host-ownership guardrail: receipts stay reusable for diagnostics / support / evidence
    // and host ownership never disappears on restore — at least one lineage-not-export-safe receipt
    // degrades, at least one ownership-dropped receipt degrades, at least one clean receipt carries
    // a reusable lineage, and no clean receipt drops ownership or carries an unstable lineage.
    let lineage_not_safe_degrades = receipt_examples().any(|ex| {
        ex.degrade_reason == Some(M5ExecutionOriginReceiptRowDegradeReason::LineageNotExportSafe)
    });
    let ownership_dropped_degrades = receipt_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5ExecutionOriginReceiptRowDegradeReason::OwnershipDroppedOnRestore)
            && ex.drops_ownership_on_restore
    });
    let clean_receipt_shows_lineage =
        receipt_examples().any(|ex| ex.is_clean() && ex.lineage_stable_for_reuse);
    let no_clean_receipt_drops =
        receipt_examples().all(|ex| !(ex.is_clean() && ex.hides_origin_or_lineage()));
    if !(lineage_not_safe_degrades
        && ownership_dropped_degrades
        && clean_receipt_shows_lineage
        && no_clean_receipt_drops)
    {
        violations.push(M5HostOriginControlsViolation::Ac2NotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Repo-relative path of the M5 host-boundary object model bound by this lane.
pub const M5_HOST_ORIGIN_HOST_BOUNDARY_PATH: &str = M5_HOST_BOUNDARY_PATH;
