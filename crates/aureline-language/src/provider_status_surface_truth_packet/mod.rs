//! Provider-status strip, capability-negotiation drawer, and
//! result-provenance pill surface truth packet.
//!
//! This module is the language-owned contract for the three reusable UI
//! objects that keep code-understanding provider truth inspectable across
//! the M5 framework, notebook, generated-source, preview, docs-linked, and
//! structured-artifact surfaces:
//!
//! - a **provider-status strip** naming which provider lane is currently
//!   active, where it runs, what lifecycle state it is in, and the route
//!   to inspect why a capability is partial or unavailable;
//! - a **capability-negotiation drawer** listing the participating
//!   providers, the selected winner / fused result, scope limits,
//!   freshness, and retry / restart recovery actions; and
//! - a **result-provenance pill** that keeps provenance attached to
//!   definitions, references, completions, rename previews, and
//!   framework-aware results without forcing users into raw logs.
//!
//! The packet does not re-mint provider vocabulary. It reads the closed
//! provider-family, capability-negotiation, conflict, result-provenance,
//! preview-completeness, and downgrade-label vocabularies frozen by the
//! sibling [`crate::provider_refactor_matrix_truth_packet`] matrix, and
//! adds only the UI-object vocabulary (surface, object kind, provider
//! locality, lifecycle state, display label, capability-detail route,
//! participant role, selected-result form, scope limit, freshness, and
//! recovery action) that those surfaces need on top.
//!
//! Every row binds a closed `surface_lane`, `object_kind`, `row_class`,
//! `support_class`, plus the per-dimension class owned by its row class,
//! an `evidence_class`, a `known_limit_class`, a `downgrade_automation_class`,
//! and a `confidence_class`. A row carries an `evidence_refs` array and a
//! `disclosure_ref` whenever it is narrowed below certified, declares a
//! non-`none_declared` known limit, or binds a non-`none` downgrade
//! automation.
//!
//! The packet preserves — it never weakens — the launch-language refactor
//! safety model: a result-provenance pill that anchors a rename preview
//! must still bind a typed preview completeness, so AI-planned,
//! organize-imports, schema/codegen, and notebook/generated edits cannot
//! bypass preview, completeness labeling, or rollback checkpoints. The
//! validator also refuses an opaque loading spinner standing in for a
//! capability-detail route, a raw internal process name used as the only
//! provider label, and a provider disagreement that drops its losing
//! provider — each of which would hide truth the source documents require
//! to stay inspectable.
//!
//! The packet is intentionally metadata-only — it never admits raw source
//! bodies, refactor diffs, generated artifact bodies, notebook cell
//! outputs, provider payloads, secrets, ambient credentials, or any other
//! private material past the boundary. A row that claims `certified`
//! while leaving a required binding unbound is refused; the validator
//! narrows below certified instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::provider_refactor_matrix_truth_packet::{
    CapabilityNegotiationClass, CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface,
    DowngradeAutomationClass, DowngradeLabelClass, EvidenceClass, FindingSeverity, KnownLimitClass,
    PromotionState, ProviderFamilyClass, ResultProvenanceClass, SupportClass,
};

/// Stable record-kind tag for [`ProviderStatusSurfaceTruthPacket`].
pub const PROVIDER_STATUS_SURFACE_TRUTH_PACKET_RECORD_KIND: &str =
    "provider_status_surface_truth_stable_packet";

/// Stable record-kind tag for [`ProviderStatusSurfaceTruthSupportExport`].
pub const PROVIDER_STATUS_SURFACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_status_surface_truth_support_export";

/// Integer schema version for the provider-status surface truth packet.
pub const PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_REF: &str =
    "schemas/language/provider_status_surface_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const PROVIDER_STATUS_SURFACE_TRUTH_DOC_REF: &str =
    "docs/m5/provider-status-strips-capability-negotiation-drawers-and-result-provenance-pills.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PROVIDER_STATUS_SURFACE_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/provider-status-strips-capability-negotiation-drawers-and-result-provenance-pills.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PROVIDER_STATUS_SURFACE_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/provider_status_surface_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const PROVIDER_STATUS_SURFACE_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/provider_status_surface_truth_packet.json";

/// Repo-relative path of the matrix packet this surface packet reads.
pub const PROVIDER_STATUS_SURFACE_MATRIX_SOURCE_REF: &str =
    "artifacts/language/m5/provider_refactor_matrix_truth_packet.json";

/// Closed surface vocabulary. Every required surface MUST have rows in
/// any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Framework-pack / framework-analyzer surface.
    FrameworkSurface,
    /// Notebook-aware surface.
    NotebookSurface,
    /// Generated / scaffolded source surface.
    GeneratedSourceSurface,
    /// Refactor / change preview surface.
    PreviewSurface,
    /// Docs-linked hover / definition surface.
    DocsLinkedSurface,
    /// Structured API / infra / preview artifact surface.
    StructuredArtifactSurface,
}

impl SurfaceClass {
    /// Every required surface, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::FrameworkSurface,
        Self::NotebookSurface,
        Self::GeneratedSourceSurface,
        Self::PreviewSurface,
        Self::DocsLinkedSurface,
        Self::StructuredArtifactSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkSurface => "framework_surface",
            Self::NotebookSurface => "notebook_surface",
            Self::GeneratedSourceSurface => "generated_source_surface",
            Self::PreviewSurface => "preview_surface",
            Self::DocsLinkedSurface => "docs_linked_surface",
            Self::StructuredArtifactSurface => "structured_artifact_surface",
        }
    }
}

/// Closed UI-object kind vocabulary. The packet certifies three reusable
/// objects across every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceObjectKind {
    /// Provider-status strip.
    ProviderStatusStrip,
    /// Capability-negotiation drawer.
    CapabilityNegotiationDrawer,
    /// Result-provenance pill.
    ResultProvenancePill,
    /// Row is a meta row (gap / known-limit / automation) and names no object.
    NotApplicable,
}

impl SurfaceObjectKind {
    /// Every concrete object kind a certified surface must carry.
    pub const REQUIRED: [Self; 3] = [
        Self::ProviderStatusStrip,
        Self::CapabilityNegotiationDrawer,
        Self::ResultProvenancePill,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderStatusStrip => "provider_status_strip",
            Self::CapabilityNegotiationDrawer => "capability_negotiation_drawer",
            Self::ResultProvenancePill => "result_provenance_pill",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this kind names a concrete object.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// Closed object-row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectRowClass {
    /// The surface declares it renders an object kind, at a support grade,
    /// naming the acting provider family and a human-readable label.
    SurfaceObjectPresence,
    /// Provider-status strip row binding provider locality and lifecycle state.
    ProviderLaneStateAdmission,
    /// Provider-status strip row binding the capability-detail inspection route.
    CapabilityDetailRouteAdmission,
    /// Drawer row binding one participating provider's role and conflict class.
    ParticipatingProviderAdmission,
    /// Drawer row binding the selected winner / fused result form.
    NegotiationResultAdmission,
    /// Drawer row binding scope limit and freshness.
    ScopeAndFreshnessAdmission,
    /// Drawer row binding the retry / restart recovery action.
    DrawerRecoveryActionAdmission,
    /// Pill row binding the provenance anchor target and result provenance.
    ProvenanceAnchorAdmission,
    /// Pill row binding the allowed downgrade label.
    ProvenanceDowngradeAdmission,
    /// Precisely labeled unsupported-gap row on a surface.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a surface.
    KnownLimit,
    /// Downgrade-automation rule row attached to a surface.
    DowngradeAutomation,
}

impl ObjectRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceObjectPresence => "surface_object_presence",
            Self::ProviderLaneStateAdmission => "provider_lane_state_admission",
            Self::CapabilityDetailRouteAdmission => "capability_detail_route_admission",
            Self::ParticipatingProviderAdmission => "participating_provider_admission",
            Self::NegotiationResultAdmission => "negotiation_result_admission",
            Self::ScopeAndFreshnessAdmission => "scope_and_freshness_admission",
            Self::DrawerRecoveryActionAdmission => "drawer_recovery_action_admission",
            Self::ProvenanceAnchorAdmission => "provenance_anchor_admission",
            Self::ProvenanceDowngradeAdmission => "provenance_downgrade_admission",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// Object kind a row of this class must carry, if any.
    pub const fn expected_object_kind(self) -> Option<SurfaceObjectKind> {
        match self {
            Self::ProviderLaneStateAdmission | Self::CapabilityDetailRouteAdmission => {
                Some(SurfaceObjectKind::ProviderStatusStrip)
            }
            Self::ParticipatingProviderAdmission
            | Self::NegotiationResultAdmission
            | Self::ScopeAndFreshnessAdmission
            | Self::DrawerRecoveryActionAdmission => {
                Some(SurfaceObjectKind::CapabilityNegotiationDrawer)
            }
            Self::ProvenanceAnchorAdmission | Self::ProvenanceDowngradeAdmission => {
                Some(SurfaceObjectKind::ResultProvenancePill)
            }
            Self::SurfaceObjectPresence
            | Self::UnsupportedGap
            | Self::KnownLimit
            | Self::DowngradeAutomation => None,
        }
    }

    /// True when a row of this class must name a concrete acting provider family.
    pub const fn requires_provider_family(self) -> bool {
        matches!(
            self,
            Self::SurfaceObjectPresence | Self::ProviderLaneStateAdmission
        )
    }

    /// True when a row of this class must carry a concrete, human-readable
    /// provider label.
    pub const fn is_label_bearing(self) -> bool {
        matches!(
            self,
            Self::SurfaceObjectPresence | Self::ProviderLaneStateAdmission
        )
    }
}

/// Closed provider-locality vocabulary: where the acting provider runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderLocalityClass {
    /// Runs in the editor process.
    InProcessEngine,
    /// Runs as a local-host subprocess.
    LocalHostSubprocess,
    /// Runs as a workspace-local process / container.
    WorkspaceLocalProcess,
    /// Runs in a notebook kernel session.
    NotebookKernelSession,
    /// Runs as a remote managed service.
    RemoteManagedService,
    /// Row does not bind a locality.
    NotApplicable,
    /// Row has no bound locality; this never qualifies certified for an
    /// owning row class.
    LocalityUnbound,
}

impl ProviderLocalityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcessEngine => "in_process_engine",
            Self::LocalHostSubprocess => "local_host_subprocess",
            Self::WorkspaceLocalProcess => "workspace_local_process",
            Self::NotebookKernelSession => "notebook_kernel_session",
            Self::RemoteManagedService => "remote_managed_service",
            Self::NotApplicable => "not_applicable",
            Self::LocalityUnbound => "locality_unbound",
        }
    }

    /// True when this locality is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::LocalityUnbound)
    }

    /// True when this locality is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::LocalityUnbound)
    }
}

/// Closed provider-lifecycle-state vocabulary: what state the provider is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderLifecycleStateClass {
    /// Provider is starting up.
    StartingUp,
    /// Provider is ready and serving live results.
    ReadyLive,
    /// Provider is degraded to a partial capability.
    DegradedPartial,
    /// Provider is restarting.
    Restarting,
    /// Provider is unavailable.
    Unavailable,
    /// Row does not bind a lifecycle state.
    NotApplicable,
    /// Row has no bound lifecycle state; this never qualifies certified for
    /// an owning row class.
    LifecycleUnbound,
}

impl ProviderLifecycleStateClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartingUp => "starting_up",
            Self::ReadyLive => "ready_live",
            Self::DegradedPartial => "degraded_partial",
            Self::Restarting => "restarting",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
            Self::LifecycleUnbound => "lifecycle_unbound",
        }
    }

    /// True when this lifecycle state is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::LifecycleUnbound)
    }

    /// True when this lifecycle state is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::LifecycleUnbound)
    }
}

/// Closed provider-display-label vocabulary. A raw internal process name
/// is never allowed as the only user-facing provider label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderDisplayLabelClass {
    /// A human-readable lane label.
    HumanReadableLaneLabel,
    /// A human-readable provider-family label paired with its locality.
    ProviderFamilyWithLocalityLabel,
    /// A raw internal process name used as the only label. Always refused.
    RawProcessNameOnly,
    /// Row does not bind a display label.
    NotApplicable,
    /// Row has no bound display label; this never qualifies certified for a
    /// label-bearing row class.
    LabelUnbound,
}

impl ProviderDisplayLabelClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanReadableLaneLabel => "human_readable_lane_label",
            Self::ProviderFamilyWithLocalityLabel => "provider_family_with_locality_label",
            Self::RawProcessNameOnly => "raw_process_name_only",
            Self::NotApplicable => "not_applicable",
            Self::LabelUnbound => "label_unbound",
        }
    }

    /// True when this label is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::LabelUnbound)
    }

    /// True when this label is a safe, human-readable value.
    pub const fn is_human_readable(self) -> bool {
        matches!(
            self,
            Self::HumanReadableLaneLabel | Self::ProviderFamilyWithLocalityLabel
        )
    }
}

/// Closed capability-detail route vocabulary: how a user inspects why a
/// capability is partial or unavailable. An opaque spinner is never a
/// valid inspection route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityDetailRouteClass {
    /// Opens the capability-negotiation drawer.
    OpenNegotiationDrawer,
    /// Opens a capability inspector listing negotiated capabilities.
    OpenCapabilityInspector,
    /// Opens the provider-health panel.
    OpenProviderHealthPanel,
    /// Opens the scope-limit detail.
    OpenScopeLimitDetail,
    /// A generic loading spinner with no inspection route. Always refused.
    OpaqueSpinnerOnly,
    /// Row does not bind a capability-detail route.
    NotApplicable,
    /// Row has no bound route; this never qualifies certified for an
    /// owning row class.
    RouteUnbound,
}

impl CapabilityDetailRouteClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenNegotiationDrawer => "open_negotiation_drawer",
            Self::OpenCapabilityInspector => "open_capability_inspector",
            Self::OpenProviderHealthPanel => "open_provider_health_panel",
            Self::OpenScopeLimitDetail => "open_scope_limit_detail",
            Self::OpaqueSpinnerOnly => "opaque_spinner_only",
            Self::NotApplicable => "not_applicable",
            Self::RouteUnbound => "route_unbound",
        }
    }

    /// True when this route is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::RouteUnbound)
    }

    /// True when this route is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::RouteUnbound)
    }

    /// True when this route actually leads to an inspectable detail.
    pub const fn is_inspectable(self) -> bool {
        matches!(
            self,
            Self::OpenNegotiationDrawer
                | Self::OpenCapabilityInspector
                | Self::OpenProviderHealthPanel
                | Self::OpenScopeLimitDetail
        )
    }
}

/// Closed participating-provider role vocabulary inside a drawer. The
/// losing provider stays inspectable; disagreement is never collapsed to a
/// ranking-only result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantRoleClass {
    /// The selected winner provider.
    SelectedWinner,
    /// A preserved losing provider whose result stays inspectable.
    PreservedLoser,
    /// A provider whose result is fused into the selected result.
    FusedContributor,
    /// A provider that declined the requested capability.
    DeclinedParticipant,
    /// Row does not bind a participant role.
    NotApplicable,
    /// Row has no bound role; this never qualifies certified for an owning
    /// row class.
    RoleUnbound,
}

impl ParticipantRoleClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedWinner => "selected_winner",
            Self::PreservedLoser => "preserved_loser",
            Self::FusedContributor => "fused_contributor",
            Self::DeclinedParticipant => "declined_participant",
            Self::NotApplicable => "not_applicable",
            Self::RoleUnbound => "role_unbound",
        }
    }

    /// True when this role is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::RoleUnbound)
    }

    /// True when this role is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::RoleUnbound)
    }

    /// True when this role keeps a non-winning provider inspectable.
    pub const fn preserves_non_winner(self) -> bool {
        matches!(
            self,
            Self::PreservedLoser | Self::FusedContributor | Self::DeclinedParticipant
        )
    }
}

/// Closed selected-result form vocabulary inside a drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectedResultFormClass {
    /// A single provider answered; the result is its output.
    SingleProviderResult,
    /// An arbitrated winner was selected over a preserved loser.
    ArbitratedWinnerResult,
    /// Multiple provider results were fused.
    FusedResult,
    /// Disagreement was surfaced unresolved.
    UnresolvedDisagreementResult,
    /// A policy / trust override decided the result.
    PolicyOverrideResult,
    /// A text / heuristic fallback result.
    TextFallbackResult,
    /// Row does not bind a selected-result form.
    NotApplicable,
    /// Row has no bound result form; this never qualifies certified for an
    /// owning row class.
    FormUnbound,
}

impl SelectedResultFormClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleProviderResult => "single_provider_result",
            Self::ArbitratedWinnerResult => "arbitrated_winner_result",
            Self::FusedResult => "fused_result",
            Self::UnresolvedDisagreementResult => "unresolved_disagreement_result",
            Self::PolicyOverrideResult => "policy_override_result",
            Self::TextFallbackResult => "text_fallback_result",
            Self::NotApplicable => "not_applicable",
            Self::FormUnbound => "form_unbound",
        }
    }

    /// True when this result form is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::FormUnbound)
    }

    /// True when this result form is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::FormUnbound)
    }
}

/// Closed scope-limit vocabulary. Notebook / generated / workset / sparse
/// scope limits are never hidden behind a generic loading spinner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeLimitClass {
    /// Full-workspace scope.
    FullWorkspaceScope,
    /// Single-file scope.
    SingleFileScope,
    /// Open-cells scope (notebook).
    OpenCellsScope,
    /// Sparse-index scope.
    SparseIndexScope,
    /// Workset-subset scope.
    WorksetSubsetScope,
    /// Row does not bind a scope limit.
    NotApplicable,
    /// Row has no bound scope limit; this never qualifies certified for an
    /// owning row class.
    ScopeUnbound,
}

impl ScopeLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullWorkspaceScope => "full_workspace_scope",
            Self::SingleFileScope => "single_file_scope",
            Self::OpenCellsScope => "open_cells_scope",
            Self::SparseIndexScope => "sparse_index_scope",
            Self::WorksetSubsetScope => "workset_subset_scope",
            Self::NotApplicable => "not_applicable",
            Self::ScopeUnbound => "scope_unbound",
        }
    }

    /// True when this scope limit is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ScopeUnbound)
    }

    /// True when this scope limit is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ScopeUnbound)
    }
}

/// Closed freshness vocabulary for a drawer's selected result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// A fresh, live result.
    FreshLive,
    /// A recently cached result.
    CachedRecent,
    /// An imported snapshot.
    ImportedSnapshot,
    /// A stale result pending refresh.
    StalePendingRefresh,
    /// Row does not bind a freshness.
    NotApplicable,
    /// Row has no bound freshness; this never qualifies certified for an
    /// owning row class.
    FreshnessUnbound,
}

impl FreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshLive => "fresh_live",
            Self::CachedRecent => "cached_recent",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::StalePendingRefresh => "stale_pending_refresh",
            Self::NotApplicable => "not_applicable",
            Self::FreshnessUnbound => "freshness_unbound",
        }
    }

    /// True when this freshness is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::FreshnessUnbound)
    }

    /// True when this freshness is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::FreshnessUnbound)
    }
}

/// Closed recovery-action vocabulary a drawer offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// Retry the request.
    RetryRequest,
    /// Restart the provider.
    RestartProvider,
    /// Regenerate from source.
    RegenerateFromSource,
    /// Rerun the preview.
    RerunPreview,
    /// Refresh the result.
    RefreshResult,
    /// Row does not bind a recovery action.
    NotApplicable,
    /// Row has no bound recovery action; this never qualifies certified for
    /// an owning row class.
    ActionUnbound,
}

impl RecoveryActionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryRequest => "retry_request",
            Self::RestartProvider => "restart_provider",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::RerunPreview => "rerun_preview",
            Self::RefreshResult => "refresh_result",
            Self::NotApplicable => "not_applicable",
            Self::ActionUnbound => "action_unbound",
        }
    }

    /// True when this recovery action is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::ActionUnbound)
    }

    /// True when this recovery action is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::ActionUnbound)
    }
}

/// Closed provenance-anchor target vocabulary: which result a provenance
/// pill is attached to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceAnchorTargetClass {
    /// A go-to-definition result.
    DefinitionResult,
    /// A find-references result.
    ReferenceResult,
    /// A completion result.
    CompletionResult,
    /// A rename preview.
    RenamePreview,
    /// A framework-aware result.
    FrameworkAwareResult,
    /// A hover / documentation result.
    HoverDocResult,
    /// Row does not bind a provenance anchor target.
    NotApplicable,
    /// Row has no bound anchor target; this never qualifies certified for
    /// an owning row class.
    AnchorUnbound,
}

impl ProvenanceAnchorTargetClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefinitionResult => "definition_result",
            Self::ReferenceResult => "reference_result",
            Self::CompletionResult => "completion_result",
            Self::RenamePreview => "rename_preview",
            Self::FrameworkAwareResult => "framework_aware_result",
            Self::HoverDocResult => "hover_doc_result",
            Self::NotApplicable => "not_applicable",
            Self::AnchorUnbound => "anchor_unbound",
        }
    }

    /// True when this anchor target is a concrete, bound value.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::AnchorUnbound)
    }

    /// True when this anchor target is allowed on a non-owner row.
    pub const fn is_inactive(self) -> bool {
        matches!(self, Self::NotApplicable | Self::AnchorUnbound)
    }

    /// True when this anchor target mutates source and so demands a typed
    /// preview completeness.
    pub const fn is_mutating_preview(self) -> bool {
        matches!(self, Self::RenamePreview)
    }
}

/// Closed validation-finding vocabulary for the surface packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required surface has no row.
    MissingSurfaceLaneCoverage,
    /// A covered surface is missing a presence row for an object kind.
    MissingObjectKindPresence,
    /// A certified status strip is missing its provider-lane-state row.
    MissingProviderLaneStateCoverage,
    /// A certified status strip is missing its capability-detail route row.
    MissingCapabilityDetailRouteCoverage,
    /// A certified drawer is missing a participating-provider row.
    MissingParticipatingProviderCoverage,
    /// A certified drawer is missing its negotiation-result row.
    MissingNegotiationResultCoverage,
    /// A certified drawer is missing its scope-and-freshness row.
    MissingScopeAndFreshnessCoverage,
    /// A certified drawer is missing its recovery-action row.
    MissingDrawerRecoveryActionCoverage,
    /// A certified pill is missing its provenance-anchor row.
    MissingProvenanceAnchorCoverage,
    /// A certified pill is missing its provenance-downgrade row.
    MissingProvenanceDowngradeCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row that must name a provider family has no concrete provider.
    MissingProviderFamily,
    /// A label-bearing row has no concrete provider label.
    MissingProviderDisplayLabel,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A status-strip lane-state row has no bound provider locality.
    MissingProviderLocalityClass,
    /// A status-strip lane-state row has no bound provider lifecycle state.
    MissingProviderLifecycleStateClass,
    /// A capability-detail route row has no bound route.
    MissingCapabilityDetailRouteClass,
    /// A capability-detail route row has no bound capability-negotiation outcome.
    MissingCapabilityNegotiationClass,
    /// A participating-provider row has no bound role.
    MissingParticipantRoleClass,
    /// A participating-provider row has no bound conflict class.
    MissingConflictClass,
    /// A negotiation-result row has no bound selected-result form.
    MissingSelectedResultFormClass,
    /// A scope-and-freshness row has no bound scope limit.
    MissingScopeLimitClass,
    /// A scope-and-freshness row has no bound freshness.
    MissingFreshnessClass,
    /// A recovery-action row has no bound recovery action.
    MissingRecoveryActionClass,
    /// A provenance-anchor row has no bound anchor target.
    MissingProvenanceAnchorTargetClass,
    /// A provenance-anchor row has no bound result provenance.
    MissingResultProvenanceClass,
    /// A provenance-downgrade row has no bound downgrade label.
    MissingDowngradeLabelClass,
    /// A rename-preview anchor row has no bound completeness.
    MissingCompletenessClass,
    /// A row claims certified while one or more bindings is unbound.
    CertifiedWithUnboundBinding,
    /// A row narrowed below certified drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A row's object kind disagrees with its row class.
    ObjectKindRowClassMismatch,
    /// A status-strip lane-state row binds a provider locality on a row that may not.
    ProviderLocalityNotPermittedOnRowClass,
    /// A row binds a provider lifecycle state on a row class that may not.
    ProviderLifecycleStateNotPermittedOnRowClass,
    /// A row binds a capability-detail route on a row class that may not.
    CapabilityDetailRouteNotPermittedOnRowClass,
    /// A row binds a capability-negotiation outcome on a row class that may not.
    CapabilityNegotiationNotPermittedOnRowClass,
    /// A row binds a participant role on a row class that may not.
    ParticipantRoleNotPermittedOnRowClass,
    /// A row binds a conflict class on a row class that may not.
    ConflictNotPermittedOnRowClass,
    /// A row binds a selected-result form on a row class that may not.
    SelectedResultFormNotPermittedOnRowClass,
    /// A row binds a scope limit on a row class that may not.
    ScopeLimitNotPermittedOnRowClass,
    /// A row binds a freshness on a row class that may not.
    FreshnessNotPermittedOnRowClass,
    /// A row binds a recovery action on a row class that may not.
    RecoveryActionNotPermittedOnRowClass,
    /// A row binds a provenance anchor target on a row class that may not.
    ProvenanceAnchorTargetNotPermittedOnRowClass,
    /// A row binds a result provenance on a row class that may not.
    ResultProvenanceNotPermittedOnRowClass,
    /// A row binds a downgrade label on a row class that may not.
    DowngradeLabelNotPermittedOnRowClass,
    /// A row binds a completeness on a row class that may not.
    CompletenessNotPermittedOnRowClass,
    /// A capability-detail route row stands in an opaque spinner for inspection.
    CapabilityDetailRouteIsOpaqueSpinner,
    /// A provider disagreement is shown without preserving the losing provider.
    LosingProviderNotPreserved,
    /// A raw internal process name is used as the only provider label.
    RawProcessNameOnlyLabel,
    /// A rename-preview provenance pill bypasses typed preview / completeness.
    PreviewAnchorBypassesTypedPreview,
    /// A provenance pill forces users into raw logs to read provenance.
    ProvenanceRequiresRawLogs,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops surface truth.
    ConsumerProjectionDrift,
    /// A projection collapses the surface vocabulary.
    SurfaceLaneVocabularyCollapsed,
    /// A projection collapses the object-kind vocabulary.
    ObjectKindVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the provider-family vocabulary.
    ProviderFamilyVocabularyCollapsed,
    /// A projection collapses the provider-locality vocabulary.
    ProviderLocalityVocabularyCollapsed,
    /// A projection collapses the provider-lifecycle-state vocabulary.
    ProviderLifecycleStateVocabularyCollapsed,
    /// A projection collapses the provider-display-label vocabulary.
    ProviderDisplayLabelVocabularyCollapsed,
    /// A projection collapses the capability-negotiation vocabulary.
    CapabilityNegotiationVocabularyCollapsed,
    /// A projection collapses the capability-detail route vocabulary.
    CapabilityDetailRouteVocabularyCollapsed,
    /// A projection collapses the participant-role vocabulary.
    ParticipantRoleVocabularyCollapsed,
    /// A projection collapses the conflict vocabulary.
    ConflictVocabularyCollapsed,
    /// A projection collapses the selected-result form vocabulary.
    SelectedResultFormVocabularyCollapsed,
    /// A projection collapses the scope-limit vocabulary.
    ScopeLimitVocabularyCollapsed,
    /// A projection collapses the freshness vocabulary.
    FreshnessVocabularyCollapsed,
    /// A projection collapses the recovery-action vocabulary.
    RecoveryActionVocabularyCollapsed,
    /// A projection collapses the provenance-anchor target vocabulary.
    ProvenanceAnchorTargetVocabularyCollapsed,
    /// A projection collapses the result-provenance vocabulary.
    ResultProvenanceVocabularyCollapsed,
    /// A projection collapses the completeness vocabulary.
    CompletenessVocabularyCollapsed,
    /// A projection collapses the downgrade-label vocabulary.
    DowngradeLabelVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSurfaceLaneCoverage => "missing_surface_lane_coverage",
            Self::MissingObjectKindPresence => "missing_object_kind_presence",
            Self::MissingProviderLaneStateCoverage => "missing_provider_lane_state_coverage",
            Self::MissingCapabilityDetailRouteCoverage => {
                "missing_capability_detail_route_coverage"
            }
            Self::MissingParticipatingProviderCoverage => "missing_participating_provider_coverage",
            Self::MissingNegotiationResultCoverage => "missing_negotiation_result_coverage",
            Self::MissingScopeAndFreshnessCoverage => "missing_scope_and_freshness_coverage",
            Self::MissingDrawerRecoveryActionCoverage => "missing_drawer_recovery_action_coverage",
            Self::MissingProvenanceAnchorCoverage => "missing_provenance_anchor_coverage",
            Self::MissingProvenanceDowngradeCoverage => "missing_provenance_downgrade_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingProviderFamily => "missing_provider_family",
            Self::MissingProviderDisplayLabel => "missing_provider_display_label",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingProviderLocalityClass => "missing_provider_locality_class",
            Self::MissingProviderLifecycleStateClass => "missing_provider_lifecycle_state_class",
            Self::MissingCapabilityDetailRouteClass => "missing_capability_detail_route_class",
            Self::MissingCapabilityNegotiationClass => "missing_capability_negotiation_class",
            Self::MissingParticipantRoleClass => "missing_participant_role_class",
            Self::MissingConflictClass => "missing_conflict_class",
            Self::MissingSelectedResultFormClass => "missing_selected_result_form_class",
            Self::MissingScopeLimitClass => "missing_scope_limit_class",
            Self::MissingFreshnessClass => "missing_freshness_class",
            Self::MissingRecoveryActionClass => "missing_recovery_action_class",
            Self::MissingProvenanceAnchorTargetClass => "missing_provenance_anchor_target_class",
            Self::MissingResultProvenanceClass => "missing_result_provenance_class",
            Self::MissingDowngradeLabelClass => "missing_downgrade_label_class",
            Self::MissingCompletenessClass => "missing_completeness_class",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::ObjectKindRowClassMismatch => "object_kind_row_class_mismatch",
            Self::ProviderLocalityNotPermittedOnRowClass => {
                "provider_locality_not_permitted_on_row_class"
            }
            Self::ProviderLifecycleStateNotPermittedOnRowClass => {
                "provider_lifecycle_state_not_permitted_on_row_class"
            }
            Self::CapabilityDetailRouteNotPermittedOnRowClass => {
                "capability_detail_route_not_permitted_on_row_class"
            }
            Self::CapabilityNegotiationNotPermittedOnRowClass => {
                "capability_negotiation_not_permitted_on_row_class"
            }
            Self::ParticipantRoleNotPermittedOnRowClass => {
                "participant_role_not_permitted_on_row_class"
            }
            Self::ConflictNotPermittedOnRowClass => "conflict_not_permitted_on_row_class",
            Self::SelectedResultFormNotPermittedOnRowClass => {
                "selected_result_form_not_permitted_on_row_class"
            }
            Self::ScopeLimitNotPermittedOnRowClass => "scope_limit_not_permitted_on_row_class",
            Self::FreshnessNotPermittedOnRowClass => "freshness_not_permitted_on_row_class",
            Self::RecoveryActionNotPermittedOnRowClass => {
                "recovery_action_not_permitted_on_row_class"
            }
            Self::ProvenanceAnchorTargetNotPermittedOnRowClass => {
                "provenance_anchor_target_not_permitted_on_row_class"
            }
            Self::ResultProvenanceNotPermittedOnRowClass => {
                "result_provenance_not_permitted_on_row_class"
            }
            Self::DowngradeLabelNotPermittedOnRowClass => {
                "downgrade_label_not_permitted_on_row_class"
            }
            Self::CompletenessNotPermittedOnRowClass => "completeness_not_permitted_on_row_class",
            Self::CapabilityDetailRouteIsOpaqueSpinner => {
                "capability_detail_route_is_opaque_spinner"
            }
            Self::LosingProviderNotPreserved => "losing_provider_not_preserved",
            Self::RawProcessNameOnlyLabel => "raw_process_name_only_label",
            Self::PreviewAnchorBypassesTypedPreview => "preview_anchor_bypasses_typed_preview",
            Self::ProvenanceRequiresRawLogs => "provenance_requires_raw_logs",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::SurfaceLaneVocabularyCollapsed => "surface_lane_vocabulary_collapsed",
            Self::ObjectKindVocabularyCollapsed => "object_kind_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::ProviderFamilyVocabularyCollapsed => "provider_family_vocabulary_collapsed",
            Self::ProviderLocalityVocabularyCollapsed => "provider_locality_vocabulary_collapsed",
            Self::ProviderLifecycleStateVocabularyCollapsed => {
                "provider_lifecycle_state_vocabulary_collapsed"
            }
            Self::ProviderDisplayLabelVocabularyCollapsed => {
                "provider_display_label_vocabulary_collapsed"
            }
            Self::CapabilityNegotiationVocabularyCollapsed => {
                "capability_negotiation_vocabulary_collapsed"
            }
            Self::CapabilityDetailRouteVocabularyCollapsed => {
                "capability_detail_route_vocabulary_collapsed"
            }
            Self::ParticipantRoleVocabularyCollapsed => "participant_role_vocabulary_collapsed",
            Self::ConflictVocabularyCollapsed => "conflict_vocabulary_collapsed",
            Self::SelectedResultFormVocabularyCollapsed => {
                "selected_result_form_vocabulary_collapsed"
            }
            Self::ScopeLimitVocabularyCollapsed => "scope_limit_vocabulary_collapsed",
            Self::FreshnessVocabularyCollapsed => "freshness_vocabulary_collapsed",
            Self::RecoveryActionVocabularyCollapsed => "recovery_action_vocabulary_collapsed",
            Self::ProvenanceAnchorTargetVocabularyCollapsed => {
                "provenance_anchor_target_vocabulary_collapsed"
            }
            Self::ResultProvenanceVocabularyCollapsed => "result_provenance_vocabulary_collapsed",
            Self::CompletenessVocabularyCollapsed => "completeness_vocabulary_collapsed",
            Self::DowngradeLabelVocabularyCollapsed => "downgrade_label_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One surface-object row binding a surface and object kind to the truth it
/// must show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceObjectRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Host surface this row renders on.
    pub surface_lane: SurfaceClass,
    /// UI object kind this row contributes to.
    pub object_kind: SurfaceObjectKind,
    /// Row class.
    pub row_class: ObjectRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting provider family (or `not_applicable`).
    pub provider_family_class: ProviderFamilyClass,
    /// Provider locality (or `not_applicable`).
    pub provider_locality_class: ProviderLocalityClass,
    /// Provider lifecycle state (or `not_applicable`).
    pub provider_lifecycle_state_class: ProviderLifecycleStateClass,
    /// Provider display label (or `not_applicable`).
    pub provider_display_label_class: ProviderDisplayLabelClass,
    /// Capability-negotiation outcome (or `not_applicable`).
    pub capability_negotiation_class: CapabilityNegotiationClass,
    /// Capability-detail inspection route (or `not_applicable`).
    pub capability_detail_route_class: CapabilityDetailRouteClass,
    /// Participating-provider role (or `not_applicable`).
    pub participant_role_class: ParticipantRoleClass,
    /// Provider-conflict class (or `not_applicable`).
    pub conflict_class: ConflictClass,
    /// Selected-result form (or `not_applicable`).
    pub selected_result_form_class: SelectedResultFormClass,
    /// Scope limit (or `not_applicable`).
    pub scope_limit_class: ScopeLimitClass,
    /// Freshness (or `not_applicable`).
    pub freshness_class: FreshnessClass,
    /// Recovery action (or `not_applicable`).
    pub recovery_action_class: RecoveryActionClass,
    /// Provenance anchor target (or `not_applicable`).
    pub provenance_anchor_target_class: ProvenanceAnchorTargetClass,
    /// Result-provenance class (or `not_applicable`).
    pub result_provenance_class: ResultProvenanceClass,
    /// Preview-completeness class for a rename-preview anchor (or `not_applicable`).
    pub completeness_class: CompletenessClass,
    /// Allowed downgrade label (or `not_applicable`).
    pub downgrade_label_class: DowngradeLabelClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not `certified`,
    /// declares a non-`none_declared` known limit, or binds a non-`none`
    /// automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when the row would force users into raw logs to read provenance;
    /// a certified provenance pill must keep this false.
    pub provenance_requires_raw_logs: bool,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl SurfaceObjectRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.provider_family_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceObjectConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Surface packet id consumed by the projection.
    pub surface_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface vocabulary is preserved verbatim.
    pub preserves_surface_lane_vocabulary: bool,
    /// True when the object-kind vocabulary is preserved verbatim.
    pub preserves_object_kind_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the provider-family vocabulary is preserved verbatim.
    pub preserves_provider_family_vocabulary: bool,
    /// True when the provider-locality vocabulary is preserved verbatim.
    pub preserves_provider_locality_vocabulary: bool,
    /// True when the provider-lifecycle-state vocabulary is preserved verbatim.
    pub preserves_provider_lifecycle_state_vocabulary: bool,
    /// True when the provider-display-label vocabulary is preserved verbatim.
    pub preserves_provider_display_label_vocabulary: bool,
    /// True when the capability-negotiation vocabulary is preserved verbatim.
    pub preserves_capability_negotiation_vocabulary: bool,
    /// True when the capability-detail route vocabulary is preserved verbatim.
    pub preserves_capability_detail_route_vocabulary: bool,
    /// True when the participant-role vocabulary is preserved verbatim.
    pub preserves_participant_role_vocabulary: bool,
    /// True when the conflict vocabulary is preserved verbatim.
    pub preserves_conflict_vocabulary: bool,
    /// True when the selected-result form vocabulary is preserved verbatim.
    pub preserves_selected_result_form_vocabulary: bool,
    /// True when the scope-limit vocabulary is preserved verbatim.
    pub preserves_scope_limit_vocabulary: bool,
    /// True when the freshness vocabulary is preserved verbatim.
    pub preserves_freshness_vocabulary: bool,
    /// True when the recovery-action vocabulary is preserved verbatim.
    pub preserves_recovery_action_vocabulary: bool,
    /// True when the provenance-anchor target vocabulary is preserved verbatim.
    pub preserves_provenance_anchor_target_vocabulary: bool,
    /// True when the result-provenance vocabulary is preserved verbatim.
    pub preserves_result_provenance_vocabulary: bool,
    /// True when the completeness vocabulary is preserved verbatim.
    pub preserves_completeness_vocabulary: bool,
    /// True when the downgrade-label vocabulary is preserved verbatim.
    pub preserves_downgrade_label_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl SurfaceObjectConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.surface_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_surface_lane_vocabulary
            && self.preserves_object_kind_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_provider_family_vocabulary
            && self.preserves_provider_locality_vocabulary
            && self.preserves_provider_lifecycle_state_vocabulary
            && self.preserves_provider_display_label_vocabulary
            && self.preserves_capability_negotiation_vocabulary
            && self.preserves_capability_detail_route_vocabulary
            && self.preserves_participant_role_vocabulary
            && self.preserves_conflict_vocabulary
            && self.preserves_selected_result_form_vocabulary
            && self.preserves_scope_limit_vocabulary
            && self.preserves_freshness_vocabulary
            && self.preserves_recovery_action_vocabulary
            && self.preserves_provenance_anchor_target_vocabulary
            && self.preserves_result_provenance_vocabulary
            && self.preserves_completeness_vocabulary
            && self.preserves_downgrade_label_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`ProviderStatusSurfaceTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStatusSurfaceTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<SurfaceClass>,
    /// Surface-object rows.
    #[serde(default)]
    pub rows: Vec<SurfaceObjectRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SurfaceObjectConsumerProjection>,
    /// Source contracts (matrix packet/docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet binding the provider-status strip,
/// capability-negotiation drawer, and result-provenance pill across the M5
/// framework, notebook, generated-source, preview, docs-linked, and
/// structured-artifact surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStatusSurfaceTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<SurfaceClass>,
    /// Surface-object rows.
    #[serde(default)]
    pub rows: Vec<SurfaceObjectRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SurfaceObjectConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl ProviderStatusSurfaceTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: ProviderStatusSurfaceTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PROVIDER_STATUS_SURFACE_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_surfaces: input.covered_surfaces,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable surface invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique surface tokens observed across rows.
    pub fn surface_lane_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.surface_lane.as_str())
    }

    /// Returns the unique object-kind tokens observed across rows.
    pub fn object_kind_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.object_kind.as_str())
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.row_class.as_str())
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.support_class.as_str())
    }

    /// Returns the unique provider-family tokens observed across rows.
    pub fn provider_family_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_family_class.as_str())
    }

    /// Returns the unique provider-locality tokens observed across rows.
    pub fn provider_locality_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_locality_class.as_str())
    }

    /// Returns the unique provider-lifecycle-state tokens observed across rows.
    pub fn provider_lifecycle_state_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_lifecycle_state_class.as_str())
    }

    /// Returns the unique provider-display-label tokens observed across rows.
    pub fn provider_display_label_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provider_display_label_class.as_str())
    }

    /// Returns the unique capability-negotiation tokens observed across rows.
    pub fn capability_negotiation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.capability_negotiation_class.as_str())
    }

    /// Returns the unique capability-detail route tokens observed across rows.
    pub fn capability_detail_route_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.capability_detail_route_class.as_str())
    }

    /// Returns the unique participant-role tokens observed across rows.
    pub fn participant_role_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.participant_role_class.as_str())
    }

    /// Returns the unique conflict tokens observed across rows.
    pub fn conflict_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.conflict_class.as_str())
    }

    /// Returns the unique selected-result form tokens observed across rows.
    pub fn selected_result_form_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.selected_result_form_class.as_str())
    }

    /// Returns the unique scope-limit tokens observed across rows.
    pub fn scope_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.scope_limit_class.as_str())
    }

    /// Returns the unique freshness tokens observed across rows.
    pub fn freshness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.freshness_class.as_str())
    }

    /// Returns the unique recovery-action tokens observed across rows.
    pub fn recovery_action_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.recovery_action_class.as_str())
    }

    /// Returns the unique provenance-anchor target tokens observed across rows.
    pub fn provenance_anchor_target_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.provenance_anchor_target_class.as_str())
    }

    /// Returns the unique result-provenance tokens observed across rows.
    pub fn result_provenance_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.result_provenance_class.as_str())
    }

    /// Returns the unique completeness tokens observed across rows.
    pub fn completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.completeness_class.as_str())
    }

    /// Returns the unique downgrade-label tokens observed across rows.
    pub fn downgrade_label_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_label_class.as_str())
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.known_limit_class.as_str())
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_automation_class.as_str())
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_class.as_str())
    }

    fn unique_tokens(
        &self,
        project: impl Fn(&SurfaceObjectRow) -> &'static str,
    ) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ProviderStatusSurfaceTruthSupportExport {
        ProviderStatusSurfaceTruthSupportExport {
            record_kind: PROVIDER_STATUS_SURFACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            surface_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            surface_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != PROVIDER_STATUS_SURFACE_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "surface packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "surface packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_surfaces.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSurfaceLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered surface",
            ));
        }

        for surface in &self.covered_surfaces {
            let present = self.rows.iter().any(|row| row.surface_lane == *surface);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSurfaceLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers surface {}", surface.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
        }

        for surface in &self.covered_surfaces {
            self.append_per_surface_coverage_findings(*surface, &mut findings);
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            self.append_projection_findings(projection, &mut findings);
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }

    fn append_per_row_findings(
        &self,
        row: &SurfaceObjectRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }
        if !row.raw_source_material_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::RawSourceMaterialPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits raw source bodies past the boundary",
                    row.row_id
                ),
            ));
        }
        if !row.secrets_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::SecretsPresent,
                FindingSeverity::Blocker,
                format!("row {} admits secrets past the boundary", row.row_id),
            ));
        }
        if !row.ambient_authority_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::AmbientAuthorityPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits ambient authority/credentials past the boundary",
                    row.row_id
                ),
            ));
        }

        if !row.support_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSupportClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound support class", row.row_id),
            ));
        }
        if !row.known_limit_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingKnownLimit,
                FindingSeverity::Blocker,
                format!("row {} has no bound known-limit class", row.row_id),
            ));
        }
        if !row.downgrade_automation_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeAutomation,
                FindingSeverity::Blocker,
                format!("row {} has no bound downgrade-automation class", row.row_id),
            ));
        }
        if !row.evidence_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound evidence class", row.row_id),
            ));
        }
        if row.row_class.requires_provider_family() && !row.provider_family_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingProviderFamily,
                FindingSeverity::Blocker,
                format!(
                    "row {} must name a concrete acting provider family",
                    row.row_id
                ),
            ));
        }

        // Provider display label discipline: a label-bearing row must carry a
        // concrete, human-readable label, never a raw internal process name.
        if row.row_class.is_label_bearing() {
            if !row.provider_display_label_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingProviderDisplayLabel,
                    FindingSeverity::Blocker,
                    format!("row {} must carry a concrete provider label", row.row_id),
                ));
            } else if !row.provider_display_label_class.is_human_readable() {
                findings.push(ValidationFinding::new(
                    FindingKind::RawProcessNameOnlyLabel,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} uses a raw internal process name as the only provider label",
                        row.row_id
                    ),
                ));
            }
        } else if matches!(
            row.provider_display_label_class,
            ProviderDisplayLabelClass::RawProcessNameOnly
        ) {
            findings.push(ValidationFinding::new(
                FindingKind::RawProcessNameOnlyLabel,
                FindingSeverity::Blocker,
                format!(
                    "row {} uses a raw internal process name as the only provider label",
                    row.row_id
                ),
            ));
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, provider family, known limit, downgrade automation, or evidence) is unbound",
                    row.row_id
                ),
            ));
        }

        if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::NarrowedRowMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} has support class {} without a disclosure ref",
                    row.row_id,
                    row.support_class.as_str()
                ),
            ));
        }
        if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::KnownLimitMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} discloses known limit {} without a disclosure ref",
                    row.row_id,
                    row.known_limit_class.as_str()
                ),
            ));
        }
        if row
            .downgrade_automation_class
            .requires_explicit_disclosure()
            && row.disclosure_ref.is_none()
        {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeAutomationMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} binds downgrade automation {} without a disclosure ref",
                    row.row_id,
                    row.downgrade_automation_class.as_str()
                ),
            ));
        }

        if row.evidence_refs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceRefs,
                FindingSeverity::Blocker,
                format!("row {} carries no evidence refs", row.row_id),
            ));
        }

        // Object-kind / row-class agreement.
        match row.row_class.expected_object_kind() {
            Some(expected) if row.object_kind != expected => {
                findings.push(ValidationFinding::new(
                    FindingKind::ObjectKindRowClassMismatch,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but object kind {}",
                        row.row_id,
                        row.row_class.as_str(),
                        row.object_kind.as_str()
                    ),
                ));
            }
            _ => {}
        }
        if matches!(row.row_class, ObjectRowClass::SurfaceObjectPresence)
            && !row.object_kind.is_concrete()
        {
            findings.push(ValidationFinding::new(
                FindingKind::ObjectKindRowClassMismatch,
                FindingSeverity::Blocker,
                format!(
                    "presence row {} must name a concrete object kind",
                    row.row_id
                ),
            ));
        }

        self.append_dimension_gating_findings(row, findings);

        if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Warning,
                format!(
                    "row {} claims certified at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_dimension_gating_findings(
        &self,
        row: &SurfaceObjectRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let is_lane_state = matches!(row.row_class, ObjectRowClass::ProviderLaneStateAdmission);
        let is_route = matches!(
            row.row_class,
            ObjectRowClass::CapabilityDetailRouteAdmission
        );
        let is_participant = matches!(
            row.row_class,
            ObjectRowClass::ParticipatingProviderAdmission
        );
        let is_result = matches!(row.row_class, ObjectRowClass::NegotiationResultAdmission);
        let is_scope = matches!(row.row_class, ObjectRowClass::ScopeAndFreshnessAdmission);
        let is_recovery = matches!(row.row_class, ObjectRowClass::DrawerRecoveryActionAdmission);
        let is_anchor = matches!(row.row_class, ObjectRowClass::ProvenanceAnchorAdmission);
        let is_downgrade = matches!(row.row_class, ObjectRowClass::ProvenanceDowngradeAdmission);

        // Provider locality (status strip lane-state).
        gate_dimension(
            findings,
            &row.row_id,
            is_lane_state,
            row.provider_locality_class.is_concrete(),
            row.provider_locality_class.is_inactive(),
            FindingKind::MissingProviderLocalityClass,
            FindingKind::ProviderLocalityNotPermittedOnRowClass,
            "provider locality",
            row.provider_locality_class.as_str(),
        );
        // Provider lifecycle state (status strip lane-state).
        gate_dimension(
            findings,
            &row.row_id,
            is_lane_state,
            row.provider_lifecycle_state_class.is_concrete(),
            row.provider_lifecycle_state_class.is_inactive(),
            FindingKind::MissingProviderLifecycleStateClass,
            FindingKind::ProviderLifecycleStateNotPermittedOnRowClass,
            "provider lifecycle state",
            row.provider_lifecycle_state_class.as_str(),
        );
        // Capability-detail route (status strip).
        gate_dimension(
            findings,
            &row.row_id,
            is_route,
            row.capability_detail_route_class.is_concrete(),
            row.capability_detail_route_class.is_inactive(),
            FindingKind::MissingCapabilityDetailRouteClass,
            FindingKind::CapabilityDetailRouteNotPermittedOnRowClass,
            "capability-detail route",
            row.capability_detail_route_class.as_str(),
        );
        // Capability negotiation outcome (status strip route).
        gate_dimension(
            findings,
            &row.row_id,
            is_route,
            row.capability_negotiation_class.is_concrete(),
            row.capability_negotiation_class.is_inactive(),
            FindingKind::MissingCapabilityNegotiationClass,
            FindingKind::CapabilityNegotiationNotPermittedOnRowClass,
            "capability negotiation",
            row.capability_negotiation_class.as_str(),
        );
        // Participant role (drawer).
        gate_dimension(
            findings,
            &row.row_id,
            is_participant,
            row.participant_role_class.is_concrete(),
            row.participant_role_class.is_inactive(),
            FindingKind::MissingParticipantRoleClass,
            FindingKind::ParticipantRoleNotPermittedOnRowClass,
            "participant role",
            row.participant_role_class.as_str(),
        );
        // Conflict (drawer participant).
        gate_dimension(
            findings,
            &row.row_id,
            is_participant,
            row.conflict_class.is_concrete(),
            row.conflict_class.is_inactive(),
            FindingKind::MissingConflictClass,
            FindingKind::ConflictNotPermittedOnRowClass,
            "conflict",
            row.conflict_class.as_str(),
        );
        // Selected-result form (drawer).
        gate_dimension(
            findings,
            &row.row_id,
            is_result,
            row.selected_result_form_class.is_concrete(),
            row.selected_result_form_class.is_inactive(),
            FindingKind::MissingSelectedResultFormClass,
            FindingKind::SelectedResultFormNotPermittedOnRowClass,
            "selected-result form",
            row.selected_result_form_class.as_str(),
        );
        // Scope limit (drawer).
        gate_dimension(
            findings,
            &row.row_id,
            is_scope,
            row.scope_limit_class.is_concrete(),
            row.scope_limit_class.is_inactive(),
            FindingKind::MissingScopeLimitClass,
            FindingKind::ScopeLimitNotPermittedOnRowClass,
            "scope limit",
            row.scope_limit_class.as_str(),
        );
        // Freshness (drawer scope-and-freshness).
        gate_dimension(
            findings,
            &row.row_id,
            is_scope,
            row.freshness_class.is_concrete(),
            row.freshness_class.is_inactive(),
            FindingKind::MissingFreshnessClass,
            FindingKind::FreshnessNotPermittedOnRowClass,
            "freshness",
            row.freshness_class.as_str(),
        );
        // Recovery action (drawer).
        gate_dimension(
            findings,
            &row.row_id,
            is_recovery,
            row.recovery_action_class.is_concrete(),
            row.recovery_action_class.is_inactive(),
            FindingKind::MissingRecoveryActionClass,
            FindingKind::RecoveryActionNotPermittedOnRowClass,
            "recovery action",
            row.recovery_action_class.as_str(),
        );
        // Provenance anchor target (pill).
        gate_dimension(
            findings,
            &row.row_id,
            is_anchor,
            row.provenance_anchor_target_class.is_concrete(),
            row.provenance_anchor_target_class.is_inactive(),
            FindingKind::MissingProvenanceAnchorTargetClass,
            FindingKind::ProvenanceAnchorTargetNotPermittedOnRowClass,
            "provenance anchor target",
            row.provenance_anchor_target_class.as_str(),
        );
        // Result provenance (pill).
        gate_dimension(
            findings,
            &row.row_id,
            is_anchor,
            row.result_provenance_class.is_concrete(),
            row.result_provenance_class.is_inactive(),
            FindingKind::MissingResultProvenanceClass,
            FindingKind::ResultProvenanceNotPermittedOnRowClass,
            "result provenance",
            row.result_provenance_class.as_str(),
        );
        // Downgrade label (pill downgrade).
        gate_dimension(
            findings,
            &row.row_id,
            is_downgrade,
            row.downgrade_label_class.is_concrete(),
            row.downgrade_label_class.is_inactive(),
            FindingKind::MissingDowngradeLabelClass,
            FindingKind::DowngradeLabelNotPermittedOnRowClass,
            "downgrade label",
            row.downgrade_label_class.as_str(),
        );

        // Completeness — only a rename-preview provenance anchor may bind it.
        let is_preview_anchor =
            is_anchor && row.provenance_anchor_target_class.is_mutating_preview();
        if is_preview_anchor {
            let preview_unsafe = !row.completeness_class.is_concrete()
                || matches!(
                    row.completeness_class,
                    CompletenessClass::Unsupported | CompletenessClass::Blocked
                );
            if preview_unsafe {
                findings.push(ValidationFinding::new(
                    FindingKind::PreviewAnchorBypassesTypedPreview,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} anchors a rename preview without a typed, complete preview",
                        row.row_id
                    ),
                ));
            }
            if !row.completeness_class.is_concrete() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingCompletenessClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} anchors a rename preview but has no bound completeness",
                        row.row_id
                    ),
                ));
            }
        } else if !row.completeness_class.is_inactive() {
            findings.push(ValidationFinding::new(
                FindingKind::CompletenessNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} binds completeness {} without anchoring a rename preview",
                    row.row_id,
                    row.completeness_class.as_str()
                ),
            ));
        }

        // Capability-detail route must lead somewhere inspectable.
        if is_route
            && row.capability_detail_route_class.is_concrete()
            && !row.capability_detail_route_class.is_inspectable()
        {
            findings.push(ValidationFinding::new(
                FindingKind::CapabilityDetailRouteIsOpaqueSpinner,
                FindingSeverity::Blocker,
                format!(
                    "row {} offers a generic loading spinner with no capability-detail route",
                    row.row_id
                ),
            ));
        }

        // Provenance must stay attached without forcing users into raw logs.
        if is_anchor && row.provenance_requires_raw_logs {
            findings.push(ValidationFinding::new(
                FindingKind::ProvenanceRequiresRawLogs,
                FindingSeverity::Blocker,
                format!(
                    "row {} forces users into raw logs to read provenance",
                    row.row_id
                ),
            ));
        }
    }

    fn append_per_surface_coverage_findings(
        &self,
        surface: SurfaceClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // Every covered surface must carry a presence row for each object kind.
        for kind in SurfaceObjectKind::REQUIRED {
            let present = self.rows.iter().any(|row| {
                row.surface_lane == surface
                    && matches!(row.row_class, ObjectRowClass::SurfaceObjectPresence)
                    && row.object_kind == kind
            });
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingObjectKindPresence,
                    FindingSeverity::Blocker,
                    format!(
                        "surface {} has no {} presence row",
                        surface.as_str(),
                        kind.as_str()
                    ),
                ));
            }
        }

        let strip_certified =
            self.presence_certified(surface, SurfaceObjectKind::ProviderStatusStrip);
        let drawer_certified =
            self.presence_certified(surface, SurfaceObjectKind::CapabilityNegotiationDrawer);
        let pill_certified =
            self.presence_certified(surface, SurfaceObjectKind::ResultProvenancePill);

        if strip_certified {
            self.require_admission(
                surface,
                SurfaceObjectKind::ProviderStatusStrip,
                ObjectRowClass::ProviderLaneStateAdmission,
                FindingKind::MissingProviderLaneStateCoverage,
                findings,
            );
            self.require_admission(
                surface,
                SurfaceObjectKind::ProviderStatusStrip,
                ObjectRowClass::CapabilityDetailRouteAdmission,
                FindingKind::MissingCapabilityDetailRouteCoverage,
                findings,
            );
        }
        if drawer_certified {
            self.require_admission(
                surface,
                SurfaceObjectKind::CapabilityNegotiationDrawer,
                ObjectRowClass::ParticipatingProviderAdmission,
                FindingKind::MissingParticipatingProviderCoverage,
                findings,
            );
            self.require_admission(
                surface,
                SurfaceObjectKind::CapabilityNegotiationDrawer,
                ObjectRowClass::NegotiationResultAdmission,
                FindingKind::MissingNegotiationResultCoverage,
                findings,
            );
            self.require_admission(
                surface,
                SurfaceObjectKind::CapabilityNegotiationDrawer,
                ObjectRowClass::ScopeAndFreshnessAdmission,
                FindingKind::MissingScopeAndFreshnessCoverage,
                findings,
            );
            self.require_admission(
                surface,
                SurfaceObjectKind::CapabilityNegotiationDrawer,
                ObjectRowClass::DrawerRecoveryActionAdmission,
                FindingKind::MissingDrawerRecoveryActionCoverage,
                findings,
            );
            self.append_loser_preservation_finding(surface, findings);
        }
        if pill_certified {
            self.require_admission(
                surface,
                SurfaceObjectKind::ResultProvenancePill,
                ObjectRowClass::ProvenanceAnchorAdmission,
                FindingKind::MissingProvenanceAnchorCoverage,
                findings,
            );
            self.require_admission(
                surface,
                SurfaceObjectKind::ResultProvenancePill,
                ObjectRowClass::ProvenanceDowngradeAdmission,
                FindingKind::MissingProvenanceDowngradeCoverage,
                findings,
            );
        }
    }

    fn require_admission(
        &self,
        surface: SurfaceClass,
        kind: SurfaceObjectKind,
        row_class: ObjectRowClass,
        finding_kind: FindingKind,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let covered = self.rows.iter().any(|row| {
            row.surface_lane == surface && row.object_kind == kind && row.row_class == row_class
        });
        if !covered {
            findings.push(ValidationFinding::new(
                finding_kind,
                FindingSeverity::Blocker,
                format!(
                    "surface {} certifies {} but has no {} row",
                    surface.as_str(),
                    kind.as_str(),
                    row_class.as_str()
                ),
            ));
        }
    }

    fn presence_certified(&self, surface: SurfaceClass, kind: SurfaceObjectKind) -> bool {
        self.rows.iter().any(|row| {
            row.surface_lane == surface
                && row.object_kind == kind
                && matches!(row.row_class, ObjectRowClass::SurfaceObjectPresence)
                && matches!(row.support_class, SupportClass::Certified)
        })
    }

    fn append_loser_preservation_finding(
        &self,
        surface: SurfaceClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let has_conflict = self.rows.iter().any(|row| {
            row.surface_lane == surface
                && matches!(
                    row.row_class,
                    ObjectRowClass::ParticipatingProviderAdmission
                )
                && matches!(
                    row.conflict_class,
                    ConflictClass::ArbitratedWinnerLoserPreserved
                        | ConflictClass::UnresolvedDisagreementSurfaced
                )
        });
        if !has_conflict {
            return;
        }
        let preserves_loser = self.rows.iter().any(|row| {
            row.surface_lane == surface
                && matches!(
                    row.row_class,
                    ObjectRowClass::ParticipatingProviderAdmission
                )
                && row.participant_role_class.preserves_non_winner()
        });
        if !preserves_loser {
            findings.push(ValidationFinding::new(
                FindingKind::LosingProviderNotPreserved,
                FindingSeverity::Blocker,
                format!(
                    "surface {} arbitrates a provider conflict without preserving the losing provider",
                    surface.as_str()
                ),
            ));
        }
    }

    fn append_projection_findings(
        &self,
        projection: &SurfaceObjectConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve surface truth",
                    projection.projection_ref
                ),
            ));
        }
        let collapses: [(bool, FindingKind, &str); 24] = [
            (
                projection.preserves_surface_lane_vocabulary,
                FindingKind::SurfaceLaneVocabularyCollapsed,
                "surface",
            ),
            (
                projection.preserves_object_kind_vocabulary,
                FindingKind::ObjectKindVocabularyCollapsed,
                "object-kind",
            ),
            (
                projection.preserves_row_class_vocabulary,
                FindingKind::RowClassVocabularyCollapsed,
                "row-class",
            ),
            (
                projection.preserves_support_class_vocabulary,
                FindingKind::SupportClassVocabularyCollapsed,
                "support-class",
            ),
            (
                projection.preserves_provider_family_vocabulary,
                FindingKind::ProviderFamilyVocabularyCollapsed,
                "provider-family",
            ),
            (
                projection.preserves_provider_locality_vocabulary,
                FindingKind::ProviderLocalityVocabularyCollapsed,
                "provider-locality",
            ),
            (
                projection.preserves_provider_lifecycle_state_vocabulary,
                FindingKind::ProviderLifecycleStateVocabularyCollapsed,
                "provider-lifecycle-state",
            ),
            (
                projection.preserves_provider_display_label_vocabulary,
                FindingKind::ProviderDisplayLabelVocabularyCollapsed,
                "provider-display-label",
            ),
            (
                projection.preserves_capability_negotiation_vocabulary,
                FindingKind::CapabilityNegotiationVocabularyCollapsed,
                "capability-negotiation",
            ),
            (
                projection.preserves_capability_detail_route_vocabulary,
                FindingKind::CapabilityDetailRouteVocabularyCollapsed,
                "capability-detail-route",
            ),
            (
                projection.preserves_participant_role_vocabulary,
                FindingKind::ParticipantRoleVocabularyCollapsed,
                "participant-role",
            ),
            (
                projection.preserves_conflict_vocabulary,
                FindingKind::ConflictVocabularyCollapsed,
                "conflict",
            ),
            (
                projection.preserves_selected_result_form_vocabulary,
                FindingKind::SelectedResultFormVocabularyCollapsed,
                "selected-result-form",
            ),
            (
                projection.preserves_scope_limit_vocabulary,
                FindingKind::ScopeLimitVocabularyCollapsed,
                "scope-limit",
            ),
            (
                projection.preserves_freshness_vocabulary,
                FindingKind::FreshnessVocabularyCollapsed,
                "freshness",
            ),
            (
                projection.preserves_recovery_action_vocabulary,
                FindingKind::RecoveryActionVocabularyCollapsed,
                "recovery-action",
            ),
            (
                projection.preserves_provenance_anchor_target_vocabulary,
                FindingKind::ProvenanceAnchorTargetVocabularyCollapsed,
                "provenance-anchor-target",
            ),
            (
                projection.preserves_result_provenance_vocabulary,
                FindingKind::ResultProvenanceVocabularyCollapsed,
                "result-provenance",
            ),
            (
                projection.preserves_completeness_vocabulary,
                FindingKind::CompletenessVocabularyCollapsed,
                "completeness",
            ),
            (
                projection.preserves_downgrade_label_vocabulary,
                FindingKind::DowngradeLabelVocabularyCollapsed,
                "downgrade-label",
            ),
            (
                projection.preserves_known_limit_vocabulary,
                FindingKind::KnownLimitVocabularyCollapsed,
                "known-limit",
            ),
            (
                projection.preserves_downgrade_automation_vocabulary,
                FindingKind::DowngradeAutomationVocabularyCollapsed,
                "downgrade-automation",
            ),
            (
                projection.preserves_evidence_class_vocabulary,
                FindingKind::EvidenceClassVocabularyCollapsed,
                "evidence-class",
            ),
            // Sentinel kept last so the table stays a fixed shape; always true.
            (true, FindingKind::ConsumerProjectionDrift, "_sentinel"),
        ];
        for (preserved, finding_kind, label) in collapses {
            if label == "_sentinel" {
                continue;
            }
            if !preserved {
                findings.push(ValidationFinding::new(
                    finding_kind,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the {} vocabulary",
                        projection.projection_ref, label
                    ),
                ));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn gate_dimension(
    findings: &mut Vec<ValidationFinding>,
    row_id: &str,
    is_owner: bool,
    is_concrete: bool,
    is_inactive: bool,
    missing_kind: FindingKind,
    not_permitted_kind: FindingKind,
    dim_label: &str,
    token: &str,
) {
    if is_owner && !is_concrete {
        findings.push(ValidationFinding::new(
            missing_kind,
            FindingSeverity::Blocker,
            format!("row {row_id} owns the {dim_label} dimension but binds no concrete value"),
        ));
    }
    if !is_owner && !is_inactive {
        findings.push(ValidationFinding::new(
            not_permitted_kind,
            FindingSeverity::Blocker,
            format!("row {row_id} may not bind {dim_label} {token} on its row class"),
        ));
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStatusSurfaceTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub surface_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub surface_packet: ProviderStatusSurfaceTruthPacket,
}

impl ProviderStatusSurfaceTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == PROVIDER_STATUS_SURFACE_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_VERSION
            && self.surface_packet_id_ref == self.surface_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.surface_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable surface packet.
#[derive(Debug)]
pub enum ProviderStatusSurfaceTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for ProviderStatusSurfaceTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "surface packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "surface packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for ProviderStatusSurfaceTruthArtifactError {}

/// Returns the checked-in stable provider-status surface truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_provider_status_surface_truth_packet(
) -> Result<ProviderStatusSurfaceTruthPacket, ProviderStatusSurfaceTruthArtifactError> {
    let packet: ProviderStatusSurfaceTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m5/provider_status_surface_truth_packet.json"
    )))
    .map_err(ProviderStatusSurfaceTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderStatusSurfaceTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
