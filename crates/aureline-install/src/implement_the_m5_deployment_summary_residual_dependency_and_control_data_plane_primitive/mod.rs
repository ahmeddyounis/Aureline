//! Implements the reusable deployment-summary primitive: a deployment summary card,
//! a set of residual-dependency rows, and a control-plane/data-plane status strip that
//! all resolve from one deployment context and share one deployment identity, so
//! managed, self-hosted, mirrored, sovereign, and local-only surfaces stay explicit
//! about the boundary the running deployment actually provides and about local-safe
//! continuity *before* users or admins act.
//!
//! Where
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix`] *freezes* the
//! reusable deployment / continuity component families as a governed contract, and
//! [`crate::implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive`]
//! narrows the install-profile / side-by-side / rollout-ring families, this module
//! *narrows* the remaining three operational families —
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::DeploymentSummaryCard`],
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::ResidualDependencyRow`],
//! and
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip`]
//! — into one working primitive with a real **resolver**. A single deployment context
//! projects onto a summary card, its residual-dependency rows, and a plane status strip
//! that all carry one deployment identity, so the operating boundary, the residual
//! vendor dependency, and the split between control-plane health and local-runtime
//! continuity never blur across them.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — a self-hosted or sovereign surface never implies a stronger boundary than
//!   the running deployment provides.** The deployment summary card names the
//!   deployment scope, tenant / org, region, mirror / offline posture, and last
//!   control-plane sync, and a boundary that claims reduced vendor dependency
//!   (self-hosted, sovereign, local-only) may never hide a required residual vendor
//!   dependency.
//! - **AC2 — control-plane degradation is distinguishable from local-runtime
//!   continuity without opening raw diagnostics.** The status strip keeps the
//!   control-plane and data-plane states distinct, keeps a local-safe next step
//!   visible, and never masks a control-plane impairment as a local-runtime failure.
//! - **AC3 — residual vendor dependency is explicit and exportable.** Every
//!   residual-dependency row names the still-vendor-hosted service, its exact failure
//!   consequence, and its disable / alternative path, and is carried in the export.
//!
//! Raw config bytes, credentials, license keys, mirror URLs, provider cursors, and
//! device identifiers never cross this boundary; the resolver carries only opaque refs,
//! typed class tokens, booleans, and redacted labels, so support and diagnostics
//! exports reconstruct exactly what a surface would have shown without leaking source
//! or live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-deployment-summary-primitive.schema.json`](../../../../schemas/ui/m5-deployment-summary-primitive.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_deployment_summary_primitive.md`](../../../../docs/deployment/m5_deployment_summary_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_deployment_continuity_component_matrix::{
    DegradedState, M5DeploymentDowngradeTrigger, M5DeploymentMode, M5DeploymentTruthMode,
    M5PlaneState, M5ResidualDependencyClass,
};

/// Stable record-kind tag carried by [`M5DeploymentSummaryPrimitivePacket`].
pub const M5_DEPLOYMENT_SUMMARY_RECORD_KIND: &str = "m5_deployment_summary_primitive";

/// Schema version for the deployment-summary primitive packet.
pub const M5_DEPLOYMENT_SUMMARY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DEPLOYMENT_SUMMARY_SCHEMA_REF: &str =
    "schemas/ui/m5-deployment-summary-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DEPLOYMENT_SUMMARY_DOC_REF: &str =
    "docs/deployment/m5_deployment_summary_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive narrows.
pub const M5_DEPLOYMENT_SUMMARY_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DEPLOYMENT_SUMMARY_FIXTURE_DIR: &str = "fixtures/ui/m5-deployment-summary-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_DEPLOYMENT_SUMMARY_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-summary-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_DEPLOYMENT_SUMMARY_CSV_REF: &str =
    "artifacts/release/m5-deployment-summary-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DEPLOYMENT_SUMMARY_REPORT_REF: &str =
    "artifacts/release/m5-deployment-summary-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed deployment-summary surface family. Each family is one parity surface that
/// ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentSummarySurfaceFamily {
    /// The About / deployment summary card on the desktop or web app.
    AboutDeploymentCard,
    /// The admin deployment console governing a managed / self-hosted deployment.
    AdminDeploymentConsole,
    /// The service-health panel separating control-plane from data-plane health.
    ServiceHealthPanel,
    /// The diagnostics deployment pane.
    DiagnosticsDeployment,
    /// The support / export replay surface reconstructing deployment truth.
    SupportExportReplay,
    /// The docs / help deployment-scope reference surface.
    DocsDeploymentReference,
}

impl M5DeploymentSummarySurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AboutDeploymentCard,
        Self::AdminDeploymentConsole,
        Self::ServiceHealthPanel,
        Self::DiagnosticsDeployment,
        Self::SupportExportReplay,
        Self::DocsDeploymentReference,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutDeploymentCard => "about_deployment_card",
            Self::AdminDeploymentConsole => "admin_deployment_console",
            Self::ServiceHealthPanel => "service_health_panel",
            Self::DiagnosticsDeployment => "diagnostics_deployment",
            Self::SupportExportReplay => "support_export_replay",
            Self::DocsDeploymentReference => "docs_deployment_reference",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AboutDeploymentCard => "About / deployment summary card",
            Self::AdminDeploymentConsole => "Admin deployment console",
            Self::ServiceHealthPanel => "Service-health panel",
            Self::DiagnosticsDeployment => "Diagnostics deployment pane",
            Self::SupportExportReplay => "Support / export replay",
            Self::DocsDeploymentReference => "Docs deployment reference",
        }
    }
}

/// Closed deployment-scope vocabulary. Names the boundary a deployment claims so a
/// self-hosted, sovereign, or local-only surface never implies a stronger boundary than
/// the running deployment provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeClass {
    /// A shared, multi-tenant, vendor-managed deployment.
    SharedManaged,
    /// A dedicated, single-tenant, vendor-managed deployment.
    DedicatedManaged,
    /// A self-hosted deployment operated by the customer.
    SelfHosted,
    /// A sovereign / air-gapped deployment with no live vendor control plane.
    Sovereign,
    /// A local-only desktop deployment with no control plane at all.
    LocalOnly,
}

impl M5DeploymentScopeClass {
    /// Every deployment scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SharedManaged,
        Self::DedicatedManaged,
        Self::SelfHosted,
        Self::Sovereign,
        Self::LocalOnly,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedManaged => "shared_managed",
            Self::DedicatedManaged => "dedicated_managed",
            Self::SelfHosted => "self_hosted",
            Self::Sovereign => "sovereign",
            Self::LocalOnly => "local_only",
        }
    }

    /// True when this scope claims a boundary that keeps the vendor out of the running
    /// deployment (self-hosted, sovereign, or local-only) and therefore may never hide
    /// a required residual vendor dependency.
    pub const fn claims_reduced_vendor_dependency(self) -> bool {
        matches!(self, Self::SelfHosted | Self::Sovereign | Self::LocalOnly)
    }
}

/// Closed residual-failure-consequence vocabulary. Names the exact consequence if a
/// still-vendor-hosted service is unreachable so a residual dependency is never left as
/// a vague "may not work".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResidualFailureConsequence {
    /// License activation / entitlement is blocked.
    BlocksActivation,
    /// Update delivery is blocked.
    BlocksUpdates,
    /// Sign-in / identity is blocked.
    BlocksSignIn,
    /// An optional feature is degraded but core local work continues.
    DegradesOptionalFeature,
    /// No user-visible impact (e.g. telemetry only).
    NoUserImpact,
}

impl M5ResidualFailureConsequence {
    /// Every failure consequence, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::BlocksActivation,
        Self::BlocksUpdates,
        Self::BlocksSignIn,
        Self::DegradesOptionalFeature,
        Self::NoUserImpact,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlocksActivation => "blocks_activation",
            Self::BlocksUpdates => "blocks_updates",
            Self::BlocksSignIn => "blocks_sign_in",
            Self::DegradesOptionalFeature => "degrades_optional_feature",
            Self::NoUserImpact => "no_user_impact",
        }
    }
}

/// Closed residual-mitigation vocabulary. Names the disable / alternative path a
/// residual-dependency row offers so a vendor dependency never reads as an unavoidable
/// dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResidualMitigation {
    /// The dependent feature can be disabled.
    DisableFeature,
    /// An offline / cached fallback covers the dependency.
    OfflineFallback,
    /// A self-hosted alternative can replace the vendor service.
    SelfHostAlternative,
    /// An admin can provision an on-prem / mirrored equivalent.
    AdminProvisioned,
    /// No alternative exists; the dependency is required as-is.
    NoAlternative,
}

impl M5ResidualMitigation {
    /// Every mitigation path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DisableFeature,
        Self::OfflineFallback,
        Self::SelfHostAlternative,
        Self::AdminProvisioned,
        Self::NoAlternative,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisableFeature => "disable_feature",
            Self::OfflineFallback => "offline_fallback",
            Self::SelfHostAlternative => "self_host_alternative",
            Self::AdminProvisioned => "admin_provisioned",
            Self::NoAlternative => "no_alternative",
        }
    }
}

/// Closed local-safe-next-step vocabulary. Names the local-safe step that always stays
/// visible on the status strip so a control-plane outage never leaves the user without
/// a next action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalSafeNextStep {
    /// Continue local work; the local runtime is unaffected.
    ContinueLocalWork,
    /// Work offline against cached content while the control plane recovers.
    WorkOfflineCached,
    /// Retry the control plane / re-establish the connection.
    RetryControlPlane,
    /// Restore local state from a checkpoint.
    RestoreFromCheckpoint,
    /// Contact the deployment admin for a control-plane resolution.
    ContactAdmin,
}

impl M5LocalSafeNextStep {
    /// Every local-safe next step, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ContinueLocalWork,
        Self::WorkOfflineCached,
        Self::RetryControlPlane,
        Self::RestoreFromCheckpoint,
        Self::ContactAdmin,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueLocalWork => "continue_local_work",
            Self::WorkOfflineCached => "work_offline_cached",
            Self::RetryControlPlane => "retry_control_plane",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::ContactAdmin => "contact_admin",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet must
/// carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentSummaryExportField {
    /// The stable deployment identity shared across surfaces.
    DeploymentId,
    /// The deployment scope / boundary the deployment claims.
    DeploymentScope,
    /// The operating / install mode.
    OperatingMode,
    /// The tenant / org and region the deployment serves.
    TenantRegion,
    /// The mirror / offline posture.
    MirrorOfflinePosture,
    /// The last control-plane sync marker.
    LastControlPlaneSync,
    /// The control-plane and data-plane status.
    PlaneStatus,
    /// The residual vendor dependencies, their consequences, and mitigations.
    ResidualDependencies,
    /// The local-safe next step.
    LocalSafeNextStep,
}

impl M5DeploymentSummaryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DeploymentId,
        Self::DeploymentScope,
        Self::OperatingMode,
        Self::TenantRegion,
        Self::MirrorOfflinePosture,
        Self::LastControlPlaneSync,
        Self::PlaneStatus,
        Self::ResidualDependencies,
        Self::LocalSafeNextStep,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::DeploymentId,
        Self::DeploymentScope,
        Self::OperatingMode,
        Self::PlaneStatus,
        Self::ResidualDependencies,
        Self::LocalSafeNextStep,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeploymentId => "deployment_id",
            Self::DeploymentScope => "deployment_scope",
            Self::OperatingMode => "operating_mode",
            Self::TenantRegion => "tenant_region",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
            Self::LastControlPlaneSync => "last_control_plane_sync",
            Self::PlaneStatus => "plane_status",
            Self::ResidualDependencies => "residual_dependencies",
            Self::LocalSafeNextStep => "local_safe_next_step",
        }
    }
}

// --- resolver input ---

/// One residual vendor dependency the deployment still relies on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResidualDependencyInput {
    /// Opaque ref to the still-vendor-hosted service; never a raw URL.
    pub vendor_dependency_ref: String,
    /// What kind of residual dependency remains.
    pub dependency_class: M5ResidualDependencyClass,
    /// Whether the dependency is required for operation (versus optional / opt-in).
    pub required_for_operation: bool,
    /// The exact consequence if the vendor service is unreachable.
    pub failure_consequence: M5ResidualFailureConsequence,
    /// The disable / alternative path offered for the dependency.
    pub mitigation: M5ResidualMitigation,
    /// The dependency is disclosed on the surface; a residual row that is not disclosed
    /// is a contradiction and is rejected.
    pub disclosed: bool,
}

/// The full input to the deployment-summary resolver for one deployment context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryInput {
    /// The stable deployment identity that must survive across the summary card, the
    /// residual-dependency rows, and the plane status strip.
    pub deployment_id: String,
    /// Human-readable surface / context label.
    pub surface_label: String,
    /// The deployment scope / boundary the deployment claims.
    pub deployment_scope: M5DeploymentScopeClass,
    /// The operating / install mode.
    pub operating_mode: M5DeploymentMode,
    /// Opaque ref to the tenant / org the deployment serves.
    pub tenant_org_ref: String,
    /// Opaque ref to the region the deployment serves.
    pub region_ref: String,
    /// The mirror / offline posture the summary renders.
    pub mirror_offline_posture: M5DeploymentTruthMode,
    /// Opaque ref / label of the last control-plane sync marker.
    pub last_control_plane_sync_ref: String,
    /// The provenance / freshness truth class the summary binds to.
    pub truth_mode: M5DeploymentTruthMode,
    /// The control-plane (identity / policy / control) health.
    pub control_plane_state: M5PlaneState,
    /// The data-plane (workspace / runtime / data) health.
    pub data_plane_state: M5PlaneState,
    /// Whether the local runtime is actually impaired.
    pub local_runtime_impaired: bool,
    /// A control-plane impairment is flagged as a local-runtime failure; must be
    /// `false` — a control-plane outage never masquerades as a broken local runtime.
    pub control_plane_impairment_flagged_as_local: bool,
    /// The local-safe next step to keep visible on the status strip.
    pub local_safe_next_step: M5LocalSafeNextStep,
    /// The residual vendor dependencies the deployment still relies on.
    #[serde(default)]
    pub residual_dependencies: Vec<M5ResidualDependencyInput>,
    /// The summary card offers an open-details action.
    pub open_details_available: bool,
    /// The summary card offers an export action.
    pub export_available: bool,
    /// An externally-observed narrowing (control-plane outage, stale mirror, residual
    /// dependency) that degrades the surface before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved deployment summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDeploymentSummaryCard {
    /// The deployment identity — identical to the residual rows and status strip.
    pub deployment_id: String,
    /// The deployment scope / boundary the deployment claims.
    pub deployment_scope: M5DeploymentScopeClass,
    /// The operating / install mode.
    pub operating_mode: M5DeploymentMode,
    /// The opaque tenant / org ref.
    pub tenant_org_ref: String,
    /// The opaque region ref.
    pub region_ref: String,
    /// The mirror / offline posture.
    pub mirror_offline_posture: M5DeploymentTruthMode,
    /// The opaque last control-plane sync ref.
    pub last_control_plane_sync_ref: String,
    /// The provenance / freshness truth class.
    pub truth_mode: M5DeploymentTruthMode,
    /// The claimed boundary is honestly scoped: it discloses every required residual
    /// vendor dependency rather than implying a stronger boundary (AC1).
    pub boundary_honestly_scoped: bool,
    /// The card discloses its deployment scope; always holds.
    pub discloses_scope: bool,
    /// The card offers an open-details action.
    pub open_details_available: bool,
    /// The card offers an export action.
    pub export_available: bool,
}

/// The resolved residual-dependency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedResidualDependencyRow {
    /// The deployment identity — identical to every other surface.
    pub deployment_id: String,
    /// The opaque vendor-dependency ref.
    pub vendor_dependency_ref: String,
    /// What kind of residual dependency remains.
    pub dependency_class: M5ResidualDependencyClass,
    /// Whether the dependency is required for operation.
    pub required_for_operation: bool,
    /// The exact consequence if the vendor service is unreachable.
    pub failure_consequence: M5ResidualFailureConsequence,
    /// The disable / alternative path offered.
    pub mitigation: M5ResidualMitigation,
    /// The dependency is disclosed; always holds for a resolved row.
    pub disclosed: bool,
    /// The row names both a failure consequence and a mitigation path; always holds.
    pub names_failure_and_path: bool,
    /// The row is export-safe and carried in the support export (AC3).
    pub exportable: bool,
}

/// The resolved control-plane/data-plane status strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedControlDataPlaneStatusStrip {
    /// The deployment identity — identical to every other surface.
    pub deployment_id: String,
    /// The control-plane (identity / policy / control) health.
    pub control_plane_state: M5PlaneState,
    /// The data-plane (workspace / runtime / data) health.
    pub data_plane_state: M5PlaneState,
    /// Whether the local runtime is actually impaired.
    pub local_runtime_impaired: bool,
    /// The two planes are recorded distinctly; always holds.
    pub planes_distinct: bool,
    /// A control-plane impairment is never masked as a local-runtime failure (AC2);
    /// always holds.
    pub control_impairment_not_masked_as_local: bool,
    /// The local-safe next step kept visible on the strip.
    pub local_safe_next_step: M5LocalSafeNextStep,
    /// The local-safe next step stays visible; always holds.
    pub local_safe_next_step_visible: bool,
}

/// The resolved deployment-summary truth shared across the summary card, the
/// residual-dependency rows, and the plane status strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDeploymentSummary {
    /// The stable deployment identity.
    pub deployment_id: String,
    /// The resolved deployment summary card.
    pub summary_card: M5ResolvedDeploymentSummaryCard,
    /// The resolved residual-dependency rows.
    pub residual_rows: Vec<M5ResolvedResidualDependencyRow>,
    /// The resolved control-plane/data-plane status strip.
    pub status_strip: M5ResolvedControlDataPlaneStatusStrip,
    /// The claimed boundary is not stronger than the running deployment provides (AC1).
    pub boundary_not_overclaimed: bool,
    /// Control-plane degradation is distinguishable from local-runtime continuity
    /// without opening raw diagnostics (AC2).
    pub planes_distinguishable: bool,
    /// Residual vendor dependency is explicit and exportable (AC3).
    pub residual_dependency_exportable: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedDeploymentSummary {
    /// True when the deployment identity is identical across the summary card, every
    /// residual row, and the status strip.
    pub fn identity_consistent(&self) -> bool {
        self.summary_card.deployment_id == self.deployment_id
            && self.status_strip.deployment_id == self.deployment_id
            && self
                .residual_rows
                .iter()
                .all(|row| row.deployment_id == self.deployment_id)
    }

    /// True when the summary card and status strip bind to the same deployment scope /
    /// mode without inventing a second deployment story.
    pub fn scope_claims_reduced_vendor_dependency(&self) -> bool {
        self.summary_card
            .deployment_scope
            .claims_reduced_vendor_dependency()
    }

    /// True when the resolved summary carries at least one required residual dependency.
    pub fn has_required_residual(&self) -> bool {
        self.residual_rows
            .iter()
            .any(|row| row.required_for_operation)
    }

    /// True when the claimed boundary is not stronger than reality (AC1).
    pub fn boundary_not_overclaimed(&self) -> bool {
        self.boundary_not_overclaimed
    }

    /// True when control-plane degradation is distinguishable from local-runtime
    /// continuity (AC2).
    pub fn planes_distinguishable(&self) -> bool {
        self.planes_distinguishable
    }

    /// True when residual vendor dependency is explicit and exportable (AC3).
    pub fn residual_dependency_exportable(&self) -> bool {
        self.residual_dependency_exportable
    }
}

/// Errors returned by [`resolve_deployment_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DeploymentSummaryResolutionError {
    /// The deployment identity was empty.
    EmptyDeploymentId,
    /// The tenant / org ref was empty.
    EmptyTenantOrgRef,
    /// The region ref was empty.
    EmptyRegionRef,
    /// The last control-plane sync ref was empty.
    EmptySyncRef,
    /// A residual dependency ref was empty.
    EmptyResidualRef,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// A scope that claims reduced vendor dependency hid a required residual vendor
    /// dependency, implying a stronger boundary than the running deployment provides.
    BoundaryOverclaimed,
    /// A residual dependency was carried as a row but not disclosed on the surface.
    ResidualDependencyUndisclosed,
    /// A control-plane impairment was flagged as a local-runtime failure.
    ControlPlaneMaskedAsLocal,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5DeploymentSummaryResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDeploymentId => "empty_deployment_id",
            Self::EmptyTenantOrgRef => "empty_tenant_org_ref",
            Self::EmptyRegionRef => "empty_region_ref",
            Self::EmptySyncRef => "empty_sync_ref",
            Self::EmptyResidualRef => "empty_residual_ref",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::BoundaryOverclaimed => "boundary_overclaimed",
            Self::ResidualDependencyUndisclosed => "residual_dependency_undisclosed",
            Self::ControlPlaneMaskedAsLocal => "control_plane_masked_as_local",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5DeploymentSummaryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "deployment-summary resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DeploymentSummaryResolutionError {}

/// Resolves one deployment context into its shared deployment summary card,
/// residual-dependency rows, and control-plane/data-plane status strip.
///
/// The three surfaces share one deployment identity, so the operating boundary, the
/// residual vendor dependency, and the split between control-plane health and
/// local-runtime continuity never blur across them. A boundary that claims reduced
/// vendor dependency never hides a required residual dependency; a control-plane
/// impairment never masks the local runtime; and a degraded input narrows the surface
/// before action rather than after.
pub fn resolve_deployment_summary(
    input: &M5DeploymentSummaryInput,
) -> Result<M5ResolvedDeploymentSummary, M5DeploymentSummaryResolutionError> {
    if input.deployment_id.trim().is_empty() {
        return Err(M5DeploymentSummaryResolutionError::EmptyDeploymentId);
    }
    if input.tenant_org_ref.trim().is_empty() {
        return Err(M5DeploymentSummaryResolutionError::EmptyTenantOrgRef);
    }
    if input.region_ref.trim().is_empty() {
        return Err(M5DeploymentSummaryResolutionError::EmptyRegionRef);
    }
    if input.last_control_plane_sync_ref.trim().is_empty() {
        return Err(M5DeploymentSummaryResolutionError::EmptySyncRef);
    }

    let mut forbidden_scan: Vec<&str> = vec![
        input.deployment_id.as_str(),
        input.surface_label.as_str(),
        input.tenant_org_ref.as_str(),
        input.region_ref.as_str(),
        input.last_control_plane_sync_ref.as_str(),
    ];
    for dep in &input.residual_dependencies {
        forbidden_scan.push(dep.vendor_dependency_ref.as_str());
    }
    for value in forbidden_scan {
        if value_is_forbidden(value) {
            return Err(M5DeploymentSummaryResolutionError::ForbiddenMaterial);
        }
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5DeploymentSummaryResolutionError::DegradedLabelGeneric);
        }
    }

    // A control-plane impairment never masquerades as a local-runtime failure (AC2).
    if input.control_plane_impairment_flagged_as_local {
        return Err(M5DeploymentSummaryResolutionError::ControlPlaneMaskedAsLocal);
    }

    let strong_boundary = input.deployment_scope.claims_reduced_vendor_dependency();

    // Every residual dependency named as a row must be disclosed; a scope that claims
    // reduced vendor dependency may never hide a required residual dependency (AC1/AC3).
    let mut residual_rows = Vec::with_capacity(input.residual_dependencies.len());
    for dep in &input.residual_dependencies {
        if dep.vendor_dependency_ref.trim().is_empty() {
            return Err(M5DeploymentSummaryResolutionError::EmptyResidualRef);
        }
        if !dep.disclosed {
            if dep.required_for_operation && strong_boundary {
                return Err(M5DeploymentSummaryResolutionError::BoundaryOverclaimed);
            }
            return Err(M5DeploymentSummaryResolutionError::ResidualDependencyUndisclosed);
        }
        residual_rows.push(M5ResolvedResidualDependencyRow {
            deployment_id: input.deployment_id.clone(),
            vendor_dependency_ref: dep.vendor_dependency_ref.clone(),
            dependency_class: dep.dependency_class,
            required_for_operation: dep.required_for_operation,
            failure_consequence: dep.failure_consequence,
            mitigation: dep.mitigation,
            disclosed: dep.disclosed,
            names_failure_and_path: true,
            exportable: true,
        });
    }

    // A strong-boundary claim is honestly scoped when it discloses every required
    // residual vendor dependency (all rows are disclosed after the guard above).
    let boundary_honestly_scoped = input
        .residual_dependencies
        .iter()
        .all(|dep| !dep.required_for_operation || dep.disclosed);
    let boundary_not_overclaimed = boundary_honestly_scoped;

    let control_impairment_not_masked_as_local = !input.control_plane_impairment_flagged_as_local;
    let planes_distinguishable = control_impairment_not_masked_as_local;

    let residual_dependency_exportable = residual_rows.iter().all(|row| row.exportable);

    let summary_card = M5ResolvedDeploymentSummaryCard {
        deployment_id: input.deployment_id.clone(),
        deployment_scope: input.deployment_scope,
        operating_mode: input.operating_mode,
        tenant_org_ref: input.tenant_org_ref.clone(),
        region_ref: input.region_ref.clone(),
        mirror_offline_posture: input.mirror_offline_posture,
        last_control_plane_sync_ref: input.last_control_plane_sync_ref.clone(),
        truth_mode: input.truth_mode,
        boundary_honestly_scoped,
        discloses_scope: true,
        open_details_available: input.open_details_available,
        export_available: input.export_available,
    };

    let status_strip = M5ResolvedControlDataPlaneStatusStrip {
        deployment_id: input.deployment_id.clone(),
        control_plane_state: input.control_plane_state,
        data_plane_state: input.data_plane_state,
        local_runtime_impaired: input.local_runtime_impaired,
        planes_distinct: true,
        control_impairment_not_masked_as_local,
        local_safe_next_step: input.local_safe_next_step,
        local_safe_next_step_visible: true,
    };

    Ok(M5ResolvedDeploymentSummary {
        deployment_id: input.deployment_id.clone(),
        summary_card,
        residual_rows,
        status_strip,
        boundary_not_overclaimed,
        planes_distinguishable,
        residual_dependency_exportable,
        degraded: input.degraded.clone(),
    })
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs deployment truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryCase {
    /// The resolver input.
    pub input: M5DeploymentSummaryInput,
    /// The resolved deployment truth. Must equal
    /// `resolve_deployment_summary(&input)`.
    pub resolved: M5ResolvedDeploymentSummary,
}

impl M5DeploymentSummaryCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DeploymentSummaryInput) -> Self {
        let resolved = resolve_deployment_summary(&input).expect("seed deployment case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_deployment_summary(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one deployment surface family bound to the shared
/// deployment-summary contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummarySurfaceRow {
    /// The deployment surface family.
    pub surface_family: M5DeploymentSummarySurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment scopes this surface can disclose (must be non-empty).
    pub deployment_scopes: Vec<M5DeploymentScopeClass>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<M5DeploymentTruthMode>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5DeploymentSummaryExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5DeploymentDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be non-empty).
    pub example_summaries: Vec<M5DeploymentSummaryCase>,
    /// Hard invariant: this row never overclaims its boundary. MUST be `false`.
    pub overclaims_boundary: bool,
    /// Hard invariant: this row never masks control-plane impairment as local failure.
    /// MUST be `false`.
    pub masks_control_plane_as_local: bool,
    /// Hard invariant: this row never hides a residual dependency. MUST be `false`.
    pub hides_residual_dependency: bool,
    /// Hard invariant: this row never drops the local-safe next step. MUST be `false`.
    pub drops_local_safe_step: bool,
}

impl M5DeploymentSummarySurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DeploymentSummaryExportField> =
            self.export_fields.iter().copied().collect();
        M5DeploymentSummaryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.overclaims_boundary
            && !self.masks_control_plane_as_local
            && !self.hides_residual_dependency
            && !self.drops_local_safe_step
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryVocabularySet {
    /// Deployment surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-scope tokens.
    pub deployment_scopes: Vec<String>,
    /// Residual-failure-consequence tokens.
    pub failure_consequences: Vec<String>,
    /// Residual-mitigation tokens.
    pub mitigations: Vec<String>,
    /// Local-safe-next-step tokens.
    pub local_safe_next_steps: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Deployment-mode tokens (reused from the frozen matrix).
    pub deployment_modes: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Residual-dependency-class tokens (reused from the frozen matrix).
    pub residual_dependency_classes: Vec<String>,
    /// Plane-state tokens (reused from the frozen matrix).
    pub plane_states: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5DeploymentSummaryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(
                &M5DeploymentSummarySurfaceFamily::ALL,
                M5DeploymentSummarySurfaceFamily::as_str,
            ),
            deployment_scopes: tokens(&M5DeploymentScopeClass::ALL, M5DeploymentScopeClass::as_str),
            failure_consequences: tokens(
                &M5ResidualFailureConsequence::ALL,
                M5ResidualFailureConsequence::as_str,
            ),
            mitigations: tokens(&M5ResidualMitigation::ALL, M5ResidualMitigation::as_str),
            local_safe_next_steps: tokens(&M5LocalSafeNextStep::ALL, M5LocalSafeNextStep::as_str),
            export_fields: tokens(
                &M5DeploymentSummaryExportField::ALL,
                M5DeploymentSummaryExportField::as_str,
            ),
            deployment_modes: tokens(&DEPLOYMENT_MODE_ALL, M5DeploymentMode::as_str),
            truth_modes: tokens(&TRUTH_MODE_ALL, M5DeploymentTruthMode::as_str),
            residual_dependency_classes: tokens(
                &RESIDUAL_CLASS_ALL,
                M5ResidualDependencyClass::as_str,
            ),
            plane_states: tokens(&PLANE_STATE_ALL, M5PlaneState::as_str),
            downgrade_triggers: tokens(
                &DOWNGRADE_TRIGGER_ALL,
                M5DeploymentDowngradeTrigger::as_str,
            ),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The deployment modes reused from the frozen matrix, in a stable order.
const DEPLOYMENT_MODE_ALL: [M5DeploymentMode; 5] = [
    M5DeploymentMode::Desktop,
    M5DeploymentMode::Managed,
    M5DeploymentMode::SelfHosted,
    M5DeploymentMode::Portable,
    M5DeploymentMode::AirGapped,
];

/// The truth classes reused from the frozen matrix, in a stable order.
const TRUTH_MODE_ALL: [M5DeploymentTruthMode; 5] = [
    M5DeploymentTruthMode::Live,
    M5DeploymentTruthMode::Mirrored,
    M5DeploymentTruthMode::CachedOffline,
    M5DeploymentTruthMode::Imported,
    M5DeploymentTruthMode::ProviderReported,
];

/// The residual-dependency classes reused from the frozen matrix, in a stable order.
const RESIDUAL_CLASS_ALL: [M5ResidualDependencyClass; 5] = [
    M5ResidualDependencyClass::LicenseActivation,
    M5ResidualDependencyClass::UpdateDelivery,
    M5ResidualDependencyClass::IdentityProvider,
    M5ResidualDependencyClass::TelemetryChannel,
    M5ResidualDependencyClass::ModelInferenceService,
];

/// The plane states reused from the frozen matrix, in a stable order.
const PLANE_STATE_ALL: [M5PlaneState; 4] = [
    M5PlaneState::Operational,
    M5PlaneState::Degraded,
    M5PlaneState::Unavailable,
    M5PlaneState::Unknown,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5DeploymentDowngradeTrigger; 9] = [
    M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
    M5DeploymentDowngradeTrigger::MirrorStale,
    M5DeploymentDowngradeTrigger::OfflineCacheOnly,
    M5DeploymentDowngradeTrigger::SignatureUnverified,
    M5DeploymentDowngradeTrigger::RolloutPaused,
    M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
    M5DeploymentDowngradeTrigger::StateRootUnavailable,
    M5DeploymentDowngradeTrigger::ResidualVendorDependency,
    M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryGovernanceReview {
    /// One primitive carries summary-card / residual-row / status-strip truth on every
    /// surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Deployment identity is preserved across the card, rows, and strip.
    pub deployment_identity_preserved_across_surfaces: bool,
    /// A self-hosted or sovereign boundary is never overclaimed.
    pub boundary_never_overclaimed: bool,
    /// Control-plane health stays distinct from local-runtime continuity.
    pub control_plane_distinct_from_local_runtime: bool,
    /// Residual vendor dependency is always explicit and exportable.
    pub residual_dependency_always_explicit_and_exportable: bool,
    /// The support / export packet reconstructs deployment truth.
    pub support_export_reconstructs_deployment: bool,
    /// Later M5 rows cannot invent parallel deployment-summary vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryConsumerProjection {
    /// About / admin / service-health / diagnostics surfaces all consume the shared
    /// primitive.
    pub deployment_surfaces_consume_shared_primitive: bool,
    /// The deployment resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The status strip reads a single canonical plane-health source.
    pub status_strip_reads_single_plane_source: bool,
    /// Support / export reads a single canonical deployment source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the deployment-summary primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting deployment audit.
    pub deployment_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DeploymentSummaryPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DeploymentSummaryPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DeploymentSummarySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DeploymentSummaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DeploymentSummaryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DeploymentSummaryConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5DeploymentSummaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 deployment-summary primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentSummaryPrimitivePacket {
    /// Record kind; must equal [`M5_DEPLOYMENT_SUMMARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DEPLOYMENT_SUMMARY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DeploymentSummarySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DeploymentSummaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DeploymentSummaryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DeploymentSummaryConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5DeploymentSummaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DeploymentSummaryPrimitivePacket {
    /// Builds an M5 deployment-summary primitive packet from stable-lane input.
    pub fn new(input: M5DeploymentSummaryPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DEPLOYMENT_SUMMARY_RECORD_KIND.to_owned(),
            schema_version: M5_DEPLOYMENT_SUMMARY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 deployment-summary primitive invariants.
    pub fn validate(&self) -> Vec<M5DeploymentSummaryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DEPLOYMENT_SUMMARY_RECORD_KIND {
            violations.push(M5DeploymentSummaryViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DEPLOYMENT_SUMMARY_SCHEMA_VERSION {
            violations.push(M5DeploymentSummaryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DeploymentSummaryViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 deployment-summary primitive packet serializes"),
        ) {
            violations.push(M5DeploymentSummaryViolation::RawMaterialInExport);
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
            .expect("m5 deployment-summary primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,deployment_scopes,truth_modes,export_fields,residual_rows,example_count\n",
        );
        for row in &self.surface_rows {
            let residual_rows: usize = row
                .example_summaries
                .iter()
                .map(|case| case.resolved.residual_rows.len())
                .sum();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.deployment_scopes, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                residual_rows,
                row.example_summaries.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Deployment-Summary Primitive: Deployment Summary Card, Residual-Dependency Rows, and Control/Data-Plane Status Strip\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Deployment surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5DeploymentSummarySurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Deployment scopes: {}\n",
            self.vocabulary_set.deployment_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Failure consequences: {}\n",
            self.vocabulary_set.failure_consequences.join(", ")
        ));
        out.push_str(&format!(
            "- Local-safe next steps: {}\n",
            self.vocabulary_set.local_safe_next_steps.join(", ")
        ));
        out.push_str("\n## Deployment surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_summaries.len()
            ));
            for case in &row.example_summaries {
                out.push_str(&format!(
                    "    - `{}` → scope `{}`, mode `{}`, planes `{}`/`{}`, residual `{}`, next `{}`\n",
                    case.resolved.deployment_id,
                    case.resolved.summary_card.deployment_scope.as_str(),
                    case.resolved.summary_card.operating_mode.as_str(),
                    case.resolved.status_strip.control_plane_state.as_str(),
                    case.resolved.status_strip.data_plane_state.as_str(),
                    case.resolved.residual_rows.len(),
                    case.resolved.status_strip.local_safe_next_step.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 deployment-summary export.
#[derive(Debug)]
pub enum M5DeploymentSummaryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DeploymentSummaryViolation>),
}

impl fmt::Display for M5DeploymentSummaryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 deployment-summary primitive export parse failed: {error}"
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
                    "m5 deployment-summary primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DeploymentSummaryArtifactError {}

/// Validation failures emitted by [`M5DeploymentSummaryPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DeploymentSummaryViolation {
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
    /// A required deployment surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no deployment scopes.
    DeploymentScopeMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked deployment cases.
    ExampleSummariesMissing,
    /// A worked deployment case does not match a fresh resolve of its input.
    ExampleSummaryDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves a self-hosted / sovereign boundary disclosed rather than
    /// overclaimed (AC1).
    BoundaryHonestyUnproven,
    /// No worked case proves control-plane degradation distinguishable from local
    /// runtime (AC2).
    PlaneDistinctionUnproven,
    /// No worked case proves residual vendor dependency explicit and exportable (AC3).
    ResidualDependencyUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DeploymentSummaryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::DeploymentScopeMissing => "deployment_scope_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleSummariesMissing => "example_summaries_missing",
            Self::ExampleSummaryDrift => "example_summary_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::BoundaryHonestyUnproven => "boundary_honesty_unproven",
            Self::PlaneDistinctionUnproven => "plane_distinction_unproven",
            Self::ResidualDependencyUnproven => "residual_dependency_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 deployment-summary export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stable_m5_deployment_summary_export(
) -> Result<M5DeploymentSummaryPrimitivePacket, M5DeploymentSummaryArtifactError> {
    let packet: M5DeploymentSummaryPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-summary-primitive-proof/support_export.json"
    )))
    .map_err(M5DeploymentSummaryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DeploymentSummaryArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DEPLOYMENT_SUMMARY_SCHEMA_REF,
        M5_DEPLOYMENT_SUMMARY_DOC_REF,
        M5_DEPLOYMENT_SUMMARY_COMPONENT_MATRIX_REF,
        M5_DEPLOYMENT_SUMMARY_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DeploymentSummaryViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DeploymentSummaryViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    let present: BTreeSet<M5DeploymentSummarySurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5DeploymentSummarySurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DeploymentSummaryViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5DeploymentSummaryViolation::SurfaceRowIncomplete);
        }
        if row.deployment_scopes.is_empty() {
            violations.push(M5DeploymentSummaryViolation::DeploymentScopeMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5DeploymentSummaryViolation::TruthModeMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DeploymentSummaryViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DeploymentSummaryViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DeploymentSummaryViolation::ConsumerSurfacesMissing);
        }
        if row.example_summaries.is_empty() {
            violations.push(M5DeploymentSummaryViolation::ExampleSummariesMissing);
        }
        if row
            .example_summaries
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DeploymentSummaryViolation::ExampleSummaryDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5DeploymentSummaryViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case across
/// the matrix: a self-hosted / sovereign boundary disclosed rather than overclaimed
/// (AC1), control-plane degradation distinguishable from local-runtime continuity
/// (AC2), and residual vendor dependency explicit and exportable (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    let cases: Vec<&M5ResolvedDeploymentSummary> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_summaries.iter().map(|case| &case.resolved))
        .collect();

    // AC1: at least one case is a strong-boundary scope that discloses a required
    // residual vendor dependency (so it cannot imply a stronger boundary than reality),
    // and every case keeps its boundary honestly scoped and identity consistent.
    let boundary_proven = cases.iter().any(|resolved| {
        resolved.scope_claims_reduced_vendor_dependency()
            && resolved.has_required_residual()
            && resolved.boundary_not_overclaimed()
    }) && cases
        .iter()
        .all(|resolved| resolved.identity_consistent() && resolved.boundary_not_overclaimed());
    if !boundary_proven {
        violations.push(M5DeploymentSummaryViolation::BoundaryHonestyUnproven);
    }

    // AC2: at least one case has an impaired control plane while the local runtime is
    // unaffected and a local-safe next step stays visible, and every case keeps the
    // planes distinguishable.
    let plane_proven = cases.iter().any(|resolved| {
        resolved.status_strip.control_plane_state.is_impaired()
            && !resolved.status_strip.local_runtime_impaired
            && resolved.status_strip.local_safe_next_step_visible
    }) && cases
        .iter()
        .all(|resolved| resolved.planes_distinguishable());
    if !plane_proven {
        violations.push(M5DeploymentSummaryViolation::PlaneDistinctionUnproven);
    }

    // AC3: at least one case carries a residual row and every residual row is
    // exportable across the matrix.
    let residual_proven = cases
        .iter()
        .any(|resolved| !resolved.residual_rows.is_empty())
        && cases.iter().all(|resolved| {
            resolved.residual_dependency_exportable()
                && resolved.residual_rows.iter().all(|row| row.exportable)
        });
    if !residual_proven {
        violations.push(M5DeploymentSummaryViolation::ResidualDependencyUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.deployment_identity_preserved_across_surfaces,
        review.boundary_never_overclaimed,
        review.control_plane_distinct_from_local_runtime,
        review.residual_dependency_always_explicit_and_exportable,
        review.support_export_reconstructs_deployment,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DeploymentSummaryViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.deployment_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.status_strip_reads_single_plane_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DeploymentSummaryViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5DeploymentSummaryPrimitivePacket,
    violations: &mut Vec<M5DeploymentSummaryViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.deployment_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DeploymentSummaryViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
