//! Frozen M5 adapter-confidence-chip, discovery-diff-card, host-boundary-strip,
//! execution-origin-receipt-row, managed-workspace-lifecycle-card,
//! suspend-resume-rebuild-review-sheet, workspace-expiry-banner, and
//! local-safe-continuation-card component matrix.
//!
//! This module locks Aureline's reusable build / remote / managed-workspace boundary UI
//! components into one export-safe packet. Every build-intelligence, host-boundary, or
//! provisioned-workspace surface M5 claims that still ships its own confidence, host-ownership, or
//! lifecycle copy — the adapter-confidence chip, the discovery-diff card, the host-boundary strip,
//! the execution-origin receipt row, the managed-workspace lifecycle card, the
//! suspend/resume/rebuild review sheet, the workspace-expiry banner, and the local-safe
//! continuation card — is named once here and constrained by the same adapter-confidence,
//! discovery-drift, host-ownership, execution-origin, suspend/resume/rebuild/recreate, expiry,
//! changed-persistence, and local-safe continuation vocabulary regardless of the surface family
//! that renders it.
//!
//! The matrix does not re-architect the remote control planes, the target-discovery engines, or
//! the managed-workspace orchestration backends that already own those records — it is the shared
//! build/remote boundary-honesty component contract layered on top of them. It binds directly to
//! the frozen M5 execution and managed-workspace object models so no later consumer can fork its
//! own confidence, host, or continuity wording: the confidence-bearing components reuse the
//! [`AdapterConfidence`] and [`DiscoveryConfidence`] vocabularies, the host-bearing components
//! reuse the [`HostKind`] and [`OriginLocus`] vocabularies, and the lifecycle-bearing components
//! reuse the [`LifecycleStateClass`], [`PersistenceClass`], [`ContinuityClass`], and
//! [`ExpiryClass`] vocabularies.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5BuildRemoteBoundaryVocabularySet`] rather than minted per surface. The single controlled
//! boundary-disposition vocabulary consumers bind to — local, SSH, container, devcontainer,
//! managed-workspace, browser-bridge, service-plane, suspended, rebuilt, recreated, expired,
//! local-safe continuation, and not-evaluated — keeps a rebuilt, recreated, or expired workspace
//! from ever reading as exact continuity, keeps local-safe continuation and companion handoff from
//! hiding behind overflow-only affordances, and keeps lower-confidence discovery from overwriting
//! higher-confidence resolved target truth without an explicit review state. Raw secret values and
//! private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_build_remote_boundary_component_matrix,
    seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed,
    seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed,
    M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_execution::m5_build_and_host_governance::{
    AdapterConfidence, M5_BUILD_AND_HOST_GOVERNANCE_PATH,
};
use aureline_execution::m5_host_boundary::{HostKind, OriginLocus, M5_HOST_BOUNDARY_PATH};
use aureline_execution::m5_target_discovery::{DiscoveryConfidence, M5_TARGET_DISCOVERY_PATH};

use crate::managed_workspace_lifecycle::{
    ContinuityClass, ExpiryClass, LifecycleStateClass, PersistenceClass,
    MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
};

/// Stable record-kind tag carried by [`M5BuildRemoteBoundaryComponentMatrixPacket`].
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix";

/// Schema version for M5 build/remote-boundary component-matrix records.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined build/remote-boundary component-matrix schema.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-build-remote-boundary-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF: &str =
    "docs/remote/m5_build_remote_boundary_components_contract.md";

/// Repo-relative path of the adapter-confidence-chip canonical component schema.
pub const M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF: &str =
    "schemas/ui/m5-adapter-confidence-chip.schema.json";

/// Repo-relative path of the discovery-diff-card canonical component schema.
pub const M5_DISCOVERY_DIFF_CARD_SCHEMA_REF: &str = "schemas/ui/m5-discovery-diff-card.schema.json";

/// Repo-relative path of the host-boundary-strip canonical component schema.
pub const M5_HOST_BOUNDARY_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-host-boundary-strip.schema.json";

/// Repo-relative path of the execution-origin-receipt-row canonical component schema.
pub const M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-execution-origin-receipt-row.schema.json";

/// Repo-relative path of the managed-workspace-lifecycle-card canonical component schema.
pub const M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-managed-workspace-lifecycle-card.schema.json";

/// Repo-relative path of the suspend-resume-rebuild-review-sheet canonical component schema.
pub const M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-suspend-resume-rebuild-review-sheet.schema.json";

/// Repo-relative path of the workspace-expiry-banner canonical component schema.
pub const M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-expiry-banner.schema.json";

/// Repo-relative path of the local-safe-continuation-card canonical component schema.
pub const M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-local-safe-continuation-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-build-remote-boundary-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-build-remote-boundary-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-build-remote-boundary-component-matrix.md";

/// One of the eight governed build / remote / managed-workspace boundary component families this
/// matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteBoundaryComponentFamily {
    /// An adapter-confidence chip naming the build/runtime adapter's confidence in the resolved
    /// target and the claim ceiling it permits.
    AdapterConfidenceChip,
    /// A discovery-diff card naming heuristic-vs-resolved target drift and the review state that
    /// governs it.
    DiscoveryDiffCard,
    /// A host-boundary strip naming which host kind the work ran on (local, SSH, container,
    /// managed, browser-bridge, or service-plane).
    HostBoundaryStrip,
    /// An execution-origin receipt row naming the origin locus where the work actually ran.
    ExecutionOriginReceiptRow,
    /// A managed-workspace lifecycle card naming the workspace lifecycle state.
    ManagedWorkspaceLifecycleCard,
    /// A suspend/resume/rebuild review sheet naming the lifecycle state, the changed persistence
    /// class, and the claimed continuity relative to the prior runtime.
    SuspendResumeRebuildReviewSheet,
    /// A workspace-expiry banner naming the expiry timing that governs the workspace.
    WorkspaceExpiryBanner,
    /// A local-safe continuation card naming the local-safe continuation offered when managed
    /// continuity is unavailable.
    LocalSafeContinuationCard,
}

impl M5BuildRemoteBoundaryComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AdapterConfidenceChip,
        Self::DiscoveryDiffCard,
        Self::HostBoundaryStrip,
        Self::ExecutionOriginReceiptRow,
        Self::ManagedWorkspaceLifecycleCard,
        Self::SuspendResumeRebuildReviewSheet,
        Self::WorkspaceExpiryBanner,
        Self::LocalSafeContinuationCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterConfidenceChip => "adapter_confidence_chip",
            Self::DiscoveryDiffCard => "discovery_diff_card",
            Self::HostBoundaryStrip => "host_boundary_strip",
            Self::ExecutionOriginReceiptRow => "execution_origin_receipt_row",
            Self::ManagedWorkspaceLifecycleCard => "managed_workspace_lifecycle_card",
            Self::SuspendResumeRebuildReviewSheet => "suspend_resume_rebuild_review_sheet",
            Self::WorkspaceExpiryBanner => "workspace_expiry_banner",
            Self::LocalSafeContinuationCard => "local_safe_continuation_card",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating this
    /// component's boundary truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::AdapterConfidenceChip => M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
            Self::DiscoveryDiffCard => M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
            Self::HostBoundaryStrip => M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
            Self::ExecutionOriginReceiptRow => M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
            Self::ManagedWorkspaceLifecycleCard => M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
            Self::SuspendResumeRebuildReviewSheet => {
                M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF
            }
            Self::WorkspaceExpiryBanner => M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
            Self::LocalSafeContinuationCard => M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled adapter-confidence level.
    pub const fn declares_adapter_confidence(self) -> bool {
        matches!(self, Self::AdapterConfidenceChip)
    }

    /// `true` when this family must name a controlled discovery-confidence level.
    pub const fn declares_discovery_confidence(self) -> bool {
        matches!(self, Self::DiscoveryDiffCard)
    }

    /// `true` when this family must name a controlled host kind.
    pub const fn declares_host_kind(self) -> bool {
        matches!(self, Self::HostBoundaryStrip)
    }

    /// `true` when this family must name a controlled origin locus.
    pub const fn declares_origin_locus(self) -> bool {
        matches!(self, Self::ExecutionOriginReceiptRow)
    }

    /// `true` when this family must name a controlled lifecycle state.
    pub const fn declares_lifecycle_state(self) -> bool {
        matches!(
            self,
            Self::ManagedWorkspaceLifecycleCard | Self::SuspendResumeRebuildReviewSheet
        )
    }

    /// `true` when this family must name a controlled persistence class.
    pub const fn declares_persistence_class(self) -> bool {
        matches!(self, Self::SuspendResumeRebuildReviewSheet)
    }

    /// `true` when this family must name a controlled continuity class.
    pub const fn declares_continuity_class(self) -> bool {
        matches!(
            self,
            Self::SuspendResumeRebuildReviewSheet | Self::LocalSafeContinuationCard
        )
    }

    /// `true` when this family must name a controlled expiry class.
    pub const fn declares_expiry_class(self) -> bool {
        matches!(self, Self::WorkspaceExpiryBanner)
    }
}

/// The single controlled boundary-disposition vocabulary every build / remote / managed-workspace
/// consumer binds to. These are the exact acceptance-criteria tokens that keep a rebuilt,
/// recreated, or expired workspace from reading as exact continuity, and that keep local, SSH,
/// container, devcontainer, managed, browser-bridge, and service-plane execution distinguishable.
/// No build/remote surface invents a parallel word for any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteBoundaryDisposition {
    /// Work ran on the local desktop host.
    LocalExecution,
    /// Work ran on a remote host reached over SSH.
    SshExecution,
    /// Work ran in a container runtime.
    ContainerExecution,
    /// Work ran in a devcontainer runtime.
    DevcontainerExecution,
    /// Work ran in a managed cloud workspace.
    ManagedWorkspace,
    /// Work ran across a browser / companion bridge.
    BrowserBridge,
    /// Work ran on a connector-backed service plane.
    ServicePlane,
    /// The workspace is suspended; the persistent volume survives but the runtime is not executing.
    Suspended,
    /// The workspace was rebuilt on a successor image; prior scratch state is gone.
    Rebuilt,
    /// The workspace was recreated under a new identity with no carried-over state.
    Recreated,
    /// The workspace expired after an idle, hibernation, or hard-deadline window.
    Expired,
    /// Work continues against a local-safe mirror with explicit caveats.
    LocalSafeContinuation,
    /// The disposition cannot currently be evaluated.
    NotEvaluated,
}

impl M5BuildRemoteBoundaryDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::LocalExecution,
        Self::SshExecution,
        Self::ContainerExecution,
        Self::DevcontainerExecution,
        Self::ManagedWorkspace,
        Self::BrowserBridge,
        Self::ServicePlane,
        Self::Suspended,
        Self::Rebuilt,
        Self::Recreated,
        Self::Expired,
        Self::LocalSafeContinuation,
        Self::NotEvaluated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalExecution => "local_execution",
            Self::SshExecution => "ssh_execution",
            Self::ContainerExecution => "container_execution",
            Self::DevcontainerExecution => "devcontainer_execution",
            Self::ManagedWorkspace => "managed_workspace",
            Self::BrowserBridge => "browser_bridge",
            Self::ServicePlane => "service_plane",
            Self::Suspended => "suspended",
            Self::Rebuilt => "rebuilt",
            Self::Recreated => "recreated",
            Self::Expired => "expired",
            Self::LocalSafeContinuation => "local_safe_continuation",
            Self::NotEvaluated => "not_evaluated",
        }
    }

    /// Whether this disposition is the one clean first-party local-execution truth state.
    pub const fn is_local_first_party(self) -> bool {
        matches!(self, Self::LocalExecution)
    }

    /// Whether this disposition materially breaks exact continuity with a prior runtime, so a
    /// reused card must never present it as exact continuity.
    pub const fn breaks_exact_continuity(self) -> bool {
        matches!(
            self,
            Self::Rebuilt | Self::Recreated | Self::Expired | Self::LocalSafeContinuation
        )
    }
}

/// Claimed M5 surface family that renders / consumes a build/remote-boundary component. No
/// component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteSurfaceFamily {
    /// The shell status / activity surface.
    Shell,
    /// The run / test / debug surface.
    RunTestDebug,
    /// The notebook surface.
    Notebook,
    /// The preview surface.
    Preview,
    /// The AI surface.
    Ai,
    /// The companion surface.
    Companion,
    /// The incident / diagnostics surface.
    Incident,
    /// The support export.
    SupportExport,
}

impl M5BuildRemoteSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Shell,
        Self::RunTestDebug,
        Self::Notebook,
        Self::Preview,
        Self::Ai,
        Self::Companion,
        Self::Incident,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::RunTestDebug => "run_test_debug",
            Self::Notebook => "notebook",
            Self::Preview => "preview",
            Self::Ai => "ai",
            Self::Companion => "companion",
            Self::Incident => "incident",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's confidence,
/// host-ownership, lifecycle, or continuity truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5BuildRemoteDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The run / test / debug UI.
    RunTestDebugUi,
    /// The notebook UI.
    NotebookUi,
    /// The preview UI.
    PreviewUi,
    /// The companion UI.
    CompanionUi,
    /// The incident / diagnostics UI.
    IncidentUi,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5BuildRemoteConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ShellUi,
        Self::RunTestDebugUi,
        Self::NotebookUi,
        Self::PreviewUi,
        Self::CompanionUi,
        Self::IncidentUi,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellUi => "shell_ui",
            Self::RunTestDebugUi => "run_test_debug_ui",
            Self::NotebookUi => "notebook_ui",
            Self::PreviewUi => "preview_ui",
            Self::CompanionUi => "companion_ui",
            Self::IncidentUi => "incident_ui",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no boundary truth is hover-only,
/// pointer-only, menu-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet, never menu-only.
    SupportExportable,
}

impl M5BuildRemoteAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Reason a build/remote-boundary component has degraded below its qualified state. Required on
/// every row so a stale, unattributed, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteDegradedReason {
    /// The discovery proof has gone stale.
    DiscoveryProofStale,
    /// The host attribution is unavailable.
    HostAttributionUnavailable,
    /// The lifecycle state is unknown.
    LifecycleStateUnknown,
    /// The expiry timing is unavailable.
    ExpiryTimingUnavailable,
    /// Continuity relative to the prior runtime is unverified.
    ContinuityUnverified,
    /// An upstream boundary lane narrowed.
    UpstreamBoundaryNarrowed,
}

impl M5BuildRemoteDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DiscoveryProofStale,
        Self::HostAttributionUnavailable,
        Self::LifecycleStateUnknown,
        Self::ExpiryTimingUnavailable,
        Self::ContinuityUnverified,
        Self::UpstreamBoundaryNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscoveryProofStale => "discovery_proof_stale",
            Self::HostAttributionUnavailable => "host_attribution_unavailable",
            Self::LifecycleStateUnknown => "lifecycle_state_unknown",
            Self::ExpiryTimingUnavailable => "expiry_timing_unavailable",
            Self::ContinuityUnverified => "continuity_unverified",
            Self::UpstreamBoundaryNarrowed => "upstream_boundary_narrowed",
        }
    }
}

/// Mandatory label a claimed build/remote-boundary component must be able to show. The first three
/// are hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about confidence and discovery drift, host ownership and execution origin, and
/// lifecycle plus continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The adapter/discovery confidence and any discovery drift behind the component.
    ConfidenceAndDiscovery,
    /// The host ownership and execution origin behind the component.
    HostAndExecutionOrigin,
    /// The lifecycle state and claimed continuity behind the component.
    LifecycleAndContinuity,
}

impl M5BuildRemoteRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ConfidenceAndDiscovery,
        Self::HostAndExecutionOrigin,
        Self::LifecycleAndContinuity,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ConfidenceAndDiscovery => "confidence_and_discovery",
            Self::HostAndExecutionOrigin => "host_and_execution_origin",
            Self::LifecycleAndContinuity => "lifecycle_and_continuity",
        }
    }
}

/// Qualification class for an M5 build/remote-boundary component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5BuildRemoteQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a build/remote-boundary component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteDowngradeTrigger {
    /// A component left its adapter confidence unstated.
    AdapterConfidenceUnstated,
    /// A component hid heuristic-vs-resolved discovery drift.
    DiscoveryDriftHidden,
    /// A component left its host boundary unstated.
    HostBoundaryUnstated,
    /// A component left its execution origin unstated.
    ExecutionOriginUnstated,
    /// A component left its lifecycle state unstated.
    LifecycleStateUnstated,
    /// A component left its expiry timing unstated.
    ExpiryTimingUnstated,
    /// A component hid a material change in persistence class.
    PersistenceChangeHidden,
    /// A component claimed exact continuity over a material change.
    ExactContinuityOverclaimed,
    /// A component hid local-safe continuation or companion handoff in overflow-only affordances.
    LocalSafeOrCompanionHandoffOverflowOnly,
    /// A component let lower-confidence discovery overwrite a higher-confidence resolved target.
    LowerConfidenceOverwroteResolvedTarget,
    /// Generic status wording concealed confidence, host, lifecycle, or continuity truth.
    GenericStatusWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5BuildRemoteDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::AdapterConfidenceUnstated,
        Self::DiscoveryDriftHidden,
        Self::HostBoundaryUnstated,
        Self::ExecutionOriginUnstated,
        Self::LifecycleStateUnstated,
        Self::ExpiryTimingUnstated,
        Self::PersistenceChangeHidden,
        Self::ExactContinuityOverclaimed,
        Self::LocalSafeOrCompanionHandoffOverflowOnly,
        Self::LowerConfidenceOverwroteResolvedTarget,
        Self::GenericStatusWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterConfidenceUnstated => "adapter_confidence_unstated",
            Self::DiscoveryDriftHidden => "discovery_drift_hidden",
            Self::HostBoundaryUnstated => "host_boundary_unstated",
            Self::ExecutionOriginUnstated => "execution_origin_unstated",
            Self::LifecycleStateUnstated => "lifecycle_state_unstated",
            Self::ExpiryTimingUnstated => "expiry_timing_unstated",
            Self::PersistenceChangeHidden => "persistence_change_hidden",
            Self::ExactContinuityOverclaimed => "exact_continuity_overclaimed",
            Self::LocalSafeOrCompanionHandoffOverflowOnly => {
                "local_safe_or_companion_handoff_overflow_only"
            }
            Self::LowerConfidenceOverwroteResolvedTarget => {
                "lower_confidence_overwrote_resolved_target"
            }
            Self::GenericStatusWordingUsed => "generic_status_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// The canonical managed-workspace lifecycle states bound from the shared managed-workspace object
/// model, in canonical order. [`LifecycleStateClass`] does not export its own `ALL`, so the matrix
/// pins the full set here to keep the frozen vocabulary stable and complete.
pub const BOUND_LIFECYCLE_STATES: [LifecycleStateClass; 10] = [
    LifecycleStateClass::Provision,
    LifecycleStateClass::Warm,
    LifecycleStateClass::Ready,
    LifecycleStateClass::Suspended,
    LifecycleStateClass::Resumed,
    LifecycleStateClass::Reconnecting,
    LifecycleStateClass::RebuildRequired,
    LifecycleStateClass::RecreateRequired,
    LifecycleStateClass::Expired,
    LifecycleStateClass::LocalSafeContinuation,
];

/// The canonical persistence classes bound from the shared managed-workspace object model, in
/// canonical order.
pub const BOUND_PERSISTENCE_CLASSES: [PersistenceClass; 6] = [
    PersistenceClass::PersistentVolume,
    PersistenceClass::EphemeralScratch,
    PersistenceClass::SnapshotRestored,
    PersistenceClass::RebuiltFresh,
    PersistenceClass::RecreatedNew,
    PersistenceClass::LocalMirror,
];

/// The canonical continuity classes bound from the shared managed-workspace object model, in
/// canonical order.
pub const BOUND_CONTINUITY_CLASSES: [ContinuityClass; 4] = [
    ContinuityClass::ExactContinuity,
    ContinuityClass::MaterialChange,
    ContinuityClass::FreshNoContinuity,
    ContinuityClass::LocalSafeOnly,
];

/// The canonical expiry classes bound from the shared managed-workspace object model, in canonical
/// order.
pub const BOUND_EXPIRY_CLASSES: [ExpiryClass; 5] = [
    ExpiryClass::None,
    ExpiryClass::IdleWindow,
    ExpiryClass::HibernationWindow,
    ExpiryClass::HardDeadline,
    ExpiryClass::ControlPlaneOutage,
];

/// One row in the matrix: one governed build/remote-boundary component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildRemoteBoundaryComponentRow {
    /// Governed component family.
    pub component_family: M5BuildRemoteBoundaryComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5BuildRemoteQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5BuildRemoteSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5BuildRemoteDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5BuildRemoteRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5BuildRemoteRequiredLabel>,
    /// Boundary dispositions this component can carry (the frozen AC vocabulary; required on every
    /// component).
    pub boundary_dispositions: Vec<M5BuildRemoteBoundaryDisposition>,
    /// Adapter-confidence levels this component names (confidence families only). Bound from the M5
    /// build/host-governance object model.
    pub adapter_confidences: Vec<AdapterConfidence>,
    /// Discovery-confidence levels this component names (discovery families only). Bound from the
    /// M5 target-discovery object model.
    pub discovery_confidences: Vec<DiscoveryConfidence>,
    /// Host kinds this component names (host families only). Bound from the M5 host-boundary object
    /// model.
    pub host_kinds: Vec<HostKind>,
    /// Origin loci this component names (origin families only). Bound from the M5 host-boundary
    /// object model.
    pub origin_loci: Vec<OriginLocus>,
    /// Lifecycle states this component names (lifecycle families only). Bound from the M5
    /// managed-workspace lifecycle object model.
    pub lifecycle_states: Vec<LifecycleStateClass>,
    /// Persistence classes this component names (persistence families only). Bound from the M5
    /// managed-workspace lifecycle object model.
    pub persistence_classes: Vec<PersistenceClass>,
    /// Continuity classes this component names (continuity families only). Bound from the M5
    /// managed-workspace lifecycle object model.
    pub continuity_classes: Vec<ContinuityClass>,
    /// Expiry classes this component names (expiry families only). Bound from the M5
    /// managed-workspace lifecycle object model.
    pub expiry_classes: Vec<ExpiryClass>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5BuildRemoteDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5BuildRemoteAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5BuildRemoteConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical component
    /// schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never implies exact continuity after a material change in
    /// target identity, image, template, or persistence class. MUST be `false`.
    pub implies_exact_continuity_after_material_change: bool,
    /// Hard invariant: this component never hides local-safe continuation or browser/companion
    /// handoff behind overflow-only affordances. MUST be `false`.
    pub hides_local_safe_or_companion_handoff_in_overflow_only: bool,
    /// Hard invariant: this component never lets lower-confidence discovery overwrite a
    /// higher-confidence resolved target without an explicit review state. MUST be `false`.
    pub lower_confidence_overwrites_resolved_target_without_review: bool,
}

impl M5BuildRemoteBoundaryComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5BuildRemoteRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5BuildRemoteRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.implies_exact_continuity_after_material_change
            && !self.hides_local_safe_or_companion_handoff_in_overflow_only
            && !self.lower_confidence_overwrites_resolved_target_without_review
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildRemoteBoundaryVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Boundary-disposition tokens.
    pub boundary_dispositions: Vec<String>,
    /// Adapter-confidence tokens (bound from the build/host-governance object model).
    pub adapter_confidences: Vec<String>,
    /// Discovery-confidence tokens (bound from the target-discovery object model).
    pub discovery_confidences: Vec<String>,
    /// Host-kind tokens (bound from the host-boundary object model).
    pub host_kinds: Vec<String>,
    /// Origin-locus tokens (bound from the host-boundary object model).
    pub origin_loci: Vec<String>,
    /// Lifecycle-state tokens (bound from the managed-workspace lifecycle object model).
    pub lifecycle_states: Vec<String>,
    /// Persistence-class tokens (bound from the managed-workspace lifecycle object model).
    pub persistence_classes: Vec<String>,
    /// Continuity-class tokens (bound from the managed-workspace lifecycle object model).
    pub continuity_classes: Vec<String>,
    /// Expiry-class tokens (bound from the managed-workspace lifecycle object model).
    pub expiry_classes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5BuildRemoteBoundaryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5BuildRemoteBoundaryComponentFamily::ALL, |v| v.as_str()),
            boundary_dispositions: tokens(&M5BuildRemoteBoundaryDisposition::ALL, |v| v.as_str()),
            adapter_confidences: tokens(&AdapterConfidence::ALL, |v| v.as_str()),
            discovery_confidences: tokens(&DiscoveryConfidence::ALL, |v| v.as_str()),
            host_kinds: tokens(&HostKind::ALL, |v| v.as_str()),
            origin_loci: tokens(&OriginLocus::ALL, |v| v.as_str()),
            lifecycle_states: tokens(&BOUND_LIFECYCLE_STATES, |v| v.as_str()),
            persistence_classes: tokens(&BOUND_PERSISTENCE_CLASSES, |v| v.as_str()),
            continuity_classes: tokens(&BOUND_CONTINUITY_CLASSES, |v| v.as_str()),
            expiry_classes: tokens(&BOUND_EXPIRY_CLASSES, |v| v.as_str()),
            surface_families: tokens(&M5BuildRemoteSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5BuildRemoteDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BuildRemoteConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BuildRemoteAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5BuildRemoteDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5BuildRemoteRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5BuildRemoteBoundaryComponentGovernanceReview {
    /// The adapter-confidence chip names the adapter confidence and its claim ceiling.
    pub adapter_confidence_chip_names_confidence_and_ceiling: bool,
    /// The discovery-diff card shows heuristic-vs-resolved drift and its review state.
    pub discovery_diff_card_shows_drift_and_review_state: bool,
    /// The host-boundary strip names the host kind.
    pub host_boundary_strip_names_host_kind: bool,
    /// The execution-origin receipt row names the origin locus.
    pub execution_origin_receipt_row_names_origin_locus: bool,
    /// The managed-workspace lifecycle card names the lifecycle state.
    pub managed_workspace_lifecycle_card_names_lifecycle_state: bool,
    /// The suspend/resume/rebuild review sheet names continuity and changed persistence.
    pub suspend_resume_rebuild_review_sheet_names_continuity_and_persistence: bool,
    /// The workspace-expiry banner names the expiry timing.
    pub workspace_expiry_banner_names_expiry_timing: bool,
    /// The local-safe continuation card names the local-safe continuation.
    pub local_safe_continuation_card_names_local_safe_continuation: bool,
    /// No card implies exact continuity after a material change.
    pub no_card_implies_exact_continuity_after_material_change: bool,
    /// Host ownership and execution origin are always explicit.
    pub host_ownership_and_execution_origin_always_explicit: bool,
    /// Discovery confidence and drift are always explicit.
    pub discovery_confidence_and_drift_always_explicit: bool,
    /// Local-safe continuation and companion handoff are never overflow-only.
    pub local_safe_and_companion_handoff_never_overflow_only: bool,
    /// Lower-confidence discovery never overwrites a resolved target without review.
    pub lower_confidence_never_overwrites_resolved_without_review: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel build/remote boundary vocabulary.
    pub later_rows_cannot_invent_parallel_build_remote_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildRemoteBoundaryComponentConsumerProjection {
    /// Run / test / debug surfaces consume the shared confidence vocabulary.
    pub run_test_debug_surfaces_consume_confidence_vocabulary: bool,
    /// Remote and preview surfaces consume the shared host-and-origin vocabulary.
    pub remote_and_preview_surfaces_consume_host_and_origin_vocabulary: bool,
    /// Managed-workspace surfaces consume the shared lifecycle vocabulary.
    pub managed_workspace_surfaces_consume_lifecycle_vocabulary: bool,
    /// Companion surfaces consume the shared continuity vocabulary.
    pub companion_surfaces_consume_continuity_vocabulary: bool,
    /// Incident surfaces consume the shared expiry vocabulary.
    pub incident_surfaces_consume_expiry_vocabulary: bool,
    /// Support / export reads a single canonical build/remote boundary source.
    pub support_export_reads_single_build_remote_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildRemoteBoundaryComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the build/remote-boundary component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildRemoteBoundaryComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BuildRemoteBoundaryComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BuildRemoteBoundaryComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5BuildRemoteBoundaryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildRemoteBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildRemoteBoundaryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildRemoteBoundaryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BuildRemoteBoundaryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BuildRemoteBoundaryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 build/remote-boundary component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildRemoteBoundaryComponentMatrixPacket {
    /// Record kind; must equal [`M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5BuildRemoteBoundaryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BuildRemoteBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BuildRemoteBoundaryComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BuildRemoteBoundaryComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BuildRemoteBoundaryComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BuildRemoteBoundaryComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BuildRemoteBoundaryComponentMatrixPacket {
    /// Builds an M5 build/remote-boundary component matrix packet from stable-lane input.
    pub fn new(input: M5BuildRemoteBoundaryComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 build/remote-boundary component matrix invariants.
    pub fn validate(&self) -> Vec<M5BuildRemoteBoundaryComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 build/remote-boundary component matrix serializes"),
        ) {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 build/remote-boundary component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Adapter-Confidence-Chip, Discovery-Diff-Card, Host-Boundary-Strip, Execution-Origin-Receipt-Row, Managed-Workspace-Lifecycle-Card, Suspend-Resume-Rebuild-Review-Sheet, Workspace-Expiry-Banner, and Local-Safe-Continuation-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Boundary dispositions: {}\n",
            self.vocabulary_set.boundary_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Host kinds: {}\n",
            self.vocabulary_set.host_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 build/remote-boundary matrix export.
#[derive(Debug)]
pub enum M5BuildRemoteBoundaryComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BuildRemoteBoundaryComponentMatrixViolation>),
}

impl fmt::Display for M5BuildRemoteBoundaryComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 build/remote-boundary component matrix export parse failed: {error}"
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
                    "m5 build/remote-boundary component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BuildRemoteBoundaryComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5BuildRemoteBoundaryComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BuildRemoteBoundaryComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no boundary dispositions.
    BoundaryDispositionMissing,
    /// A confidence component declares no adapter-confidence levels.
    AdapterConfidenceMissing,
    /// A discovery component declares no discovery-confidence levels.
    DiscoveryConfidenceMissing,
    /// A host component declares no host kinds.
    HostKindMissing,
    /// An origin component declares no origin loci.
    OriginLocusMissing,
    /// A lifecycle component declares no lifecycle states.
    LifecycleStateMissing,
    /// A persistence component declares no persistence classes.
    PersistenceClassMissing,
    /// A continuity component declares no continuity classes.
    ContinuityClassMissing,
    /// An expiry component declares no expiry classes.
    ExpiryClassMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (implies exact continuity after a material change,
    /// hides local-safe continuation or companion handoff in overflow-only affordances, or lets
    /// lower-confidence discovery overwrite a resolved target without review).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BuildRemoteBoundaryComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::BoundaryDispositionMissing => "boundary_disposition_missing",
            Self::AdapterConfidenceMissing => "adapter_confidence_missing",
            Self::DiscoveryConfidenceMissing => "discovery_confidence_missing",
            Self::HostKindMissing => "host_kind_missing",
            Self::OriginLocusMissing => "origin_locus_missing",
            Self::LifecycleStateMissing => "lifecycle_state_missing",
            Self::PersistenceClassMissing => "persistence_class_missing",
            Self::ContinuityClassMissing => "continuity_class_missing",
            Self::ExpiryClassMissing => "expiry_class_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 build/remote-boundary matrix export.
pub fn current_stable_m5_build_remote_boundary_component_matrix_export() -> Result<
    M5BuildRemoteBoundaryComponentMatrixPacket,
    M5BuildRemoteBoundaryComponentMatrixArtifactError,
> {
    let packet: M5BuildRemoteBoundaryComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-build-remote-boundary-proof/support_export.json"
        )))
        .map_err(M5BuildRemoteBoundaryComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BuildRemoteBoundaryComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
        M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
        M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
        M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
        M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
        M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
        M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
        M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    let present: BTreeSet<M5BuildRemoteBoundaryComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5BuildRemoteBoundaryComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.boundary_dispositions.is_empty() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::BoundaryDispositionMissing);
        }
        if family.declares_adapter_confidence() && row.adapter_confidences.is_empty() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::AdapterConfidenceMissing);
        }
        if family.declares_discovery_confidence() && row.discovery_confidences.is_empty() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::DiscoveryConfidenceMissing);
        }
        if family.declares_host_kind() && row.host_kinds.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::HostKindMissing);
        }
        if family.declares_origin_locus() && row.origin_loci.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::OriginLocusMissing);
        }
        if family.declares_lifecycle_state() && row.lifecycle_states.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::LifecycleStateMissing);
        }
        if family.declares_persistence_class() && row.persistence_classes.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::PersistenceClassMissing);
        }
        if family.declares_continuity_class() && row.continuity_classes.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::ContinuityClassMissing);
        }
        if family.declares_expiry_class() && row.expiry_classes.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::ExpiryClassMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.adapter_confidence_chip_names_confidence_and_ceiling,
        review.discovery_diff_card_shows_drift_and_review_state,
        review.host_boundary_strip_names_host_kind,
        review.execution_origin_receipt_row_names_origin_locus,
        review.managed_workspace_lifecycle_card_names_lifecycle_state,
        review.suspend_resume_rebuild_review_sheet_names_continuity_and_persistence,
        review.workspace_expiry_banner_names_expiry_timing,
        review.local_safe_continuation_card_names_local_safe_continuation,
        review.no_card_implies_exact_continuity_after_material_change,
        review.host_ownership_and_execution_origin_always_explicit,
        review.discovery_confidence_and_drift_always_explicit,
        review.local_safe_and_companion_handoff_never_overflow_only,
        review.lower_confidence_never_overwrites_resolved_without_review,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_build_remote_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.run_test_debug_surfaces_consume_confidence_vocabulary,
        projection.remote_and_preview_surfaces_consume_host_and_origin_vocabulary,
        projection.managed_workspace_surfaces_consume_lifecycle_vocabulary,
        projection.companion_surfaces_consume_continuity_vocabulary,
        projection.incident_surfaces_consume_expiry_vocabulary,
        projection.support_export_reads_single_build_remote_source,
    ] {
        if !ok {
            violations
                .push(M5BuildRemoteBoundaryComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BuildRemoteBoundaryComponentMatrixPacket,
    violations: &mut Vec<M5BuildRemoteBoundaryComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BuildRemoteBoundaryComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses boundary words; what is rejected is a raw secret *value* shape — a
/// pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Repo-relative refs of the execution and managed-workspace object models this matrix binds
/// against, so no consumer forks its own confidence, host, or continuity vocabulary. Re-exported
/// for callers that assemble the full source-contract set.
pub const M5_BUILD_REMOTE_BOUNDARY_BINDING_REFS: [&str; 4] = [
    M5_BUILD_AND_HOST_GOVERNANCE_PATH,
    M5_HOST_BOUNDARY_PATH,
    M5_TARGET_DISCOVERY_PATH,
    MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
];
