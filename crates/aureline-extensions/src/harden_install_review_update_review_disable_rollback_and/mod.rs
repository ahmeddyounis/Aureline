//! Harden the install-review, update-review, disable/rollback, and revocation
//! flows for extensions and policy packs into one stable, evidence-backed,
//! automatically-narrowing lifecycle-flow packet.
//!
//! The beta-level [`crate::install_review`], [`crate::review_alpha`], and
//! [`crate::revocation`] modules own the per-flow alpha projections. This module
//! owns the layer above them — the **stable lifecycle-flow truth** a claimed
//! stable ecosystem row carries when it opens install review, update review,
//! disable/rollback, or revocation, and the **stability qualification** that
//! truth is allowed to claim.
//!
//! A stable lifecycle-flow row must bind, machine-readably:
//!
//! - the **identity** (the subject — `extension` or `policy_pack` — the flow
//!   class, the package identity, the pinned flow-contract version, the
//!   publisher trust tier, and the lifecycle state),
//! - the **deterministic resolver output** (a determinism class, a resolution
//!   digest, the install scope — public / mirrored / offline — and the resolved
//!   dependency tree with one node per hard dependency and optional integration)
//!   so an install or update can never ride a nondeterministic or unresolved
//!   resolution,
//! - the **effective permission inheritance** (declared, transitive, effective,
//!   and optional-integration permission sets, plus the diff against the prior
//!   installed effective set) so authority is never widened silently,
//! - the **re-consent requirement**, which is raised whenever dependency
//!   resolution **expands** the effective permission set, not only when the
//!   top-level package manifest changes,
//! - the **lock / export plan** (a lockfile ref and an install-plan ref, with
//!   whether the plan supports team and air-gapped rollout) so a mirrored or
//!   offline rollout can be reproduced,
//! - the **disable / rollback posture** (whether disable is reversible and
//!   whether a pinned rollback target and rollback manifest exist), and
//! - the **revocation posture** (the revocation state and whether it has
//!   propagated across the primary registry, approved mirrors, and offline
//!   bundles).
//!
//! The central rule mirrors the rest of the stable line: a **stable**
//! lifecycle-flow claim may never be implied from a catalog row or an adjacent
//! green flow alone. A row that renders a `stable` badge must pin the published
//! flow-contract version, be evidence-backed (not catalog-asserted), keep its
//! publisher trust tier out of quarantine, carry a deterministic resolution with
//! every hard dependency resolved, obtain re-consent whenever the effective
//! permission set expanded, expose an exportable lock/install plan, and — per the
//! flow being hardened — stay installable (install / update), keep a reversible
//! rollback (disable / rollback), or have a fully-propagated revocation
//! (revocation). When any of those fails, the visible tier is **automatically
//! narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with
//! machine-readable reasons.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No ambient privilege expansion.** An effective-permission expansion with
//!   no obtained re-consent can never back a stable flow claim; it narrows below
//!   Stable and raises a flow banner, while the expanded permissions stay
//!   surfaced.
//! - **No nondeterministic install truth.** A nondeterministic or not-yet-run
//!   resolution can never ride a stable install / update claim.
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never
//!   back a stable lifecycle-flow claim; it narrows below Stable.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_lifecycle_flow_hardening.schema.json`](../../../../schemas/extensions/stable_lifecycle_flow_hardening.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable lifecycle-flow hardening record.
pub const STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION: u32 = 1;

/// The published, stable flow-contract version. A `stable` claim must pin exactly
/// this version; any other version narrows below Stable.
pub const STABLE_LIFECYCLE_FLOW_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_LIFECYCLE_FLOW_SCHEMA_REF: &str =
    "schemas/extensions/stable_lifecycle_flow_hardening.schema.json";

/// Record-kind tag for [`StableLifecycleFlowPacket`].
pub const STABLE_LIFECYCLE_FLOW_PACKET_RECORD_KIND: &str = "stable_lifecycle_flow_hardening_packet";

/// Record-kind tag for [`LifecycleFlowIdentity`].
pub const LIFECYCLE_FLOW_IDENTITY_RECORD_KIND: &str = "stable_lifecycle_flow_identity";

/// Record-kind tag for [`DeterministicResolution`].
pub const DETERMINISTIC_RESOLUTION_RECORD_KIND: &str =
    "stable_lifecycle_flow_deterministic_resolution";

/// Record-kind tag for [`DependencyNode`].
pub const DEPENDENCY_NODE_RECORD_KIND: &str = "stable_lifecycle_flow_dependency_node";

/// Record-kind tag for [`EffectivePermissionInheritance`].
pub const EFFECTIVE_PERMISSION_INHERITANCE_RECORD_KIND: &str =
    "stable_lifecycle_flow_effective_permission_inheritance";

/// Record-kind tag for [`ReConsentRequirement`].
pub const RECONSENT_REQUIREMENT_RECORD_KIND: &str = "stable_lifecycle_flow_reconsent_requirement";

/// Record-kind tag for [`LockExportPlan`].
pub const LOCK_EXPORT_PLAN_RECORD_KIND: &str = "stable_lifecycle_flow_lock_export_plan";

/// Record-kind tag for [`DisableRollbackPosture`].
pub const DISABLE_ROLLBACK_POSTURE_RECORD_KIND: &str =
    "stable_lifecycle_flow_disable_rollback_posture";

/// Record-kind tag for [`RevocationPosture`].
pub const REVOCATION_POSTURE_RECORD_KIND: &str = "stable_lifecycle_flow_revocation_posture";

/// Record-kind tag for [`LifecycleFlowQualificationClaim`].
pub const LIFECYCLE_FLOW_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_lifecycle_flow_qualification_claim";

/// Record-kind tag for [`DowngradedFlowBanner`].
pub const DOWNGRADED_FLOW_BANNER_RECORD_KIND: &str = "stable_lifecycle_flow_downgraded_banner";

/// Record-kind tag for [`LifecycleFlowInspection`].
pub const LIFECYCLE_FLOW_INSPECTION_RECORD_KIND: &str = "stable_lifecycle_flow_inspection";

/// Record-kind tag for [`StableLifecycleFlowSupportExport`].
pub const STABLE_LIFECYCLE_FLOW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_lifecycle_flow_hardening_support_export";

/// Closed subject vocabulary — extensions and policy packs share one lane.
pub const SUBJECT_CLASSES: &[&str] = &["extension", "policy_pack"];

/// Closed lifecycle-flow vocabulary.
pub const FLOW_CLASSES: &[&str] = &[
    "install_review",
    "update_review",
    "disable",
    "rollback",
    "revocation",
];

/// Flows that are install-shaped (a resolution lands a runnable artifact).
pub const INSTALL_SHAPED_FLOWS: &[&str] = &["install_review", "update_review"];

/// Closed publisher-trust-tier vocabulary, shared with the rest of the lane.
pub const TRUST_TIER_CLASSES: &[&str] = &[
    "verified_publisher",
    "known_publisher",
    "community_unverified",
    "quarantined",
];

/// Closed lifecycle-state vocabulary mirrored from the extension lifecycle lane.
pub const LIFECYCLE_STATE_CLASSES: &[&str] = &[
    "installed",
    "pending_activation",
    "active",
    "recovered",
    "degraded",
    "disabled",
    "quarantined",
    "removed",
    "publisher_blocked",
];

/// Lifecycle states an install-shaped stable flow may keep (installable / runnable).
pub const INSTALLABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed install-scope vocabulary — public, mirrored, and offline installs.
pub const INSTALL_SCOPE_CLASSES: &[&str] =
    &["public_registry", "approved_mirror", "offline_bundle"];

/// Closed resolver-determinism vocabulary. `deterministic` is the only state a
/// stable install-shaped claim may keep.
pub const RESOLVER_DETERMINISM_CLASSES: &[&str] =
    &["deterministic", "nondeterministic", "not_resolved"];

/// Closed dependency-node-kind vocabulary.
pub const DEPENDENCY_NODE_KIND_CLASSES: &[&str] =
    &["root", "hard_dependency", "optional_integration"];

/// Closed dependency-resolution-state vocabulary.
pub const DEPENDENCY_RESOLUTION_STATE_CLASSES: &[&str] = &[
    "resolved",
    "unresolved_missing",
    "version_conflict",
    "optional_absent",
];

/// Closed effective-permission-expansion vocabulary, derived from the prior-vs-effective diff.
pub const PERMISSION_EXPANSION_CLASSES: &[&str] = &["no_change", "expanded", "reduced"];

/// Closed re-consent-state vocabulary.
pub const RECONSENT_STATE_CLASSES: &[&str] = &[
    "not_required",
    "required_obtained",
    "required_pending",
    "required_missing",
];

/// Closed lock/export-state vocabulary.
pub const LOCK_EXPORT_STATE_CLASSES: &[&str] = &["exported", "exportable", "unavailable"];

/// Closed disable/rollback-state vocabulary.
pub const ROLLBACK_STATE_CLASSES: &[&str] = &[
    "not_applicable",
    "reversible_target_pinned",
    "reversible_no_target",
    "irreversible",
];

/// Closed revocation-state vocabulary, mirrored from the incident lane.
pub const REVOCATION_STATE_CLASSES: &[&str] = &[
    "none",
    "advisory",
    "emergency_disabled",
    "quarantined",
    "revoked",
];

/// Closed revocation-propagation vocabulary.
pub const REVOCATION_PROPAGATION_CLASSES: &[&str] = &[
    "not_applicable",
    "propagated_all_sources",
    "partial",
    "not_propagated",
];

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* lifecycle-flow claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_lifecycle_flow_claim",
    "beta_lifecycle_flow_partial_claim",
    "preview_lifecycle_flow_experimental_claim",
    "withdrawn_no_lifecycle_flow_claim",
];

/// Closed set of reasons that narrow a stable lifecycle-flow claim below Stable.
pub const LIFECYCLE_FLOW_DOWNGRADE_REASONS: &[&str] = &[
    "flow_contract_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_installable",
    "resolver_nondeterministic",
    "resolver_not_resolved",
    "unresolved_hard_dependency",
    "version_conflict_dependency",
    "permission_expansion_without_reconsent",
    "reconsent_pending",
    "lock_export_unavailable",
    "team_rollout_unsupported",
    "air_gapped_rollout_unsupported",
    "rollback_irreversible",
    "rollback_target_missing",
    "revocation_not_propagated",
    "revocation_state_missing",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the flow cannot be trusted as
/// stable at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_installable",
    "resolver_not_resolved",
    "unresolved_hard_dependency",
    "version_conflict_dependency",
    "rollback_irreversible",
    "revocation_not_propagated",
    "revocation_state_missing",
];

/// Reasons that narrow to `preview` (a structural / trust / disclosure shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "flow_contract_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "resolver_nondeterministic",
    "permission_expansion_without_reconsent",
    "lock_export_unavailable",
    "rollback_target_missing",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "reconsent_pending",
    "team_rollout_unsupported",
    "air_gapped_rollout_unsupported",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_LIFECYCLE_FLOW_CONSUMER_SURFACES: &[&str] = &[
    "install_review",
    "update_review",
    "disable_rollback_panel",
    "revocation_panel",
    "extension_inspector",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
    "mirror_packet",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable lifecycle-flow hardening packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLifecycleFlowInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: LifecycleFlowIdentityInput,
    /// Deterministic resolver-output input.
    pub resolution: DeterministicResolutionInput,
    /// Effective-permission-inheritance input.
    pub permissions: EffectivePermissionInheritanceInput,
    /// Re-consent-requirement input.
    pub reconsent: ReConsentRequirementInput,
    /// Lock / export plan input.
    pub lock_export: LockExportPlanInput,
    /// Disable / rollback posture input.
    pub disable_rollback: DisableRollbackPostureInput,
    /// Revocation posture input.
    pub revocation: RevocationPostureInput,
    /// Stability qualification claim input.
    pub claim: LifecycleFlowQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`LifecycleFlowIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFlowIdentityInput {
    /// Subject class — extension or policy pack.
    pub subject_class: String,
    /// Flow class being hardened.
    pub flow_class: String,
    /// Ref to the install-review / review alpha record this row stabilizes.
    pub source_review_ref: String,
    /// Extension or policy-pack identity.
    pub subject_identity: String,
    /// Target version this flow lands on.
    pub subject_version: String,
    /// Prior installed version, when one exists (update / rollback flows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_version_ref: Option<String>,
    /// Source package the flow operates on.
    pub source_package_ref: String,
    /// Published flow-contract version this row pins.
    pub flow_contract_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`DeterministicResolution`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicResolutionInput {
    /// Resolver determinism class.
    pub determinism_class: String,
    /// Install scope the resolution was run for.
    pub install_scope_class: String,
    /// Ref to the deterministic resolution digest.
    pub resolution_digest_ref: String,
    /// Ref to the resolver input set.
    pub resolver_input_ref: String,
    /// Resolved dependency tree nodes.
    #[serde(default)]
    pub nodes: Vec<DependencyNodeInput>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for one [`DependencyNode`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyNodeInput {
    /// Stable node id.
    pub node_id: String,
    /// Node kind class.
    pub node_kind_class: String,
    /// Target extension / package ref.
    pub target_ref: String,
    /// Resolved version range ref.
    pub version_range_ref: String,
    /// Resolution state class.
    pub resolution_state_class: String,
    /// Permission refs this node contributes to the effective set.
    #[serde(default)]
    pub contributed_permission_refs: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`EffectivePermissionInheritance`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionInheritanceInput {
    /// Top-level declared permission refs.
    #[serde(default)]
    pub declared_permission_refs: Vec<String>,
    /// Transitive permission refs from resolved hard dependencies.
    #[serde(default)]
    pub transitive_permission_refs: Vec<String>,
    /// Optional-integration permission refs, surfaced separately.
    #[serde(default)]
    pub optional_integration_permission_refs: Vec<String>,
    /// Prior installed effective permission refs (empty for a fresh install).
    #[serde(default)]
    pub prior_effective_permission_refs: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ReConsentRequirement`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReConsentRequirementInput {
    /// Re-consent state.
    pub reconsent_state_class: String,
    /// Whether the top-level package manifest changed.
    pub manifest_changed: bool,
    /// Ref to the obtained consent record, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_record_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`LockExportPlan`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockExportPlanInput {
    /// Lock/export state.
    pub lock_export_state_class: String,
    /// Ref to the lockfile artifact.
    pub lock_plan_ref: String,
    /// Ref to the install-plan artifact.
    pub install_plan_ref: String,
    /// Whether the plan supports a team rollout.
    pub supports_team_rollout: bool,
    /// Whether the plan supports an air-gapped rollout.
    pub supports_air_gapped_rollout: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`DisableRollbackPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisableRollbackPostureInput {
    /// Rollback state.
    pub rollback_state_class: String,
    /// Ref to the pinned rollback target, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_target_ref: Option<String>,
    /// Ref to the rollback manifest, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_manifest_ref: Option<String>,
    /// Whether a disable is reversible without data loss.
    pub disable_reversible: bool,
    /// Whether user/extension data is retained across a disable.
    pub data_retained_on_disable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`RevocationPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationPostureInput {
    /// Revocation state.
    pub revocation_state_class: String,
    /// Revocation-propagation class across primary / mirror / offline sources.
    pub propagation_class: String,
    /// Ref to the revocation/incident record, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_source_ref: Option<String>,
    /// Ref to the recovery-guidance record, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_guidance_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`LifecycleFlowQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFlowQualificationClaimInput {
    /// Lifecycle-flow tier claimed by the row.
    pub claimed_tier: String,
    /// Claim basis: evidence-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFlowIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Subject class — extension or policy pack.
    pub subject_class: String,
    /// Flow class being hardened.
    pub flow_class: String,
    /// Ref to the install-review / review alpha record this row stabilizes.
    pub source_review_ref: String,
    /// Extension or policy-pack identity.
    pub subject_identity: String,
    /// Target version this flow lands on.
    pub subject_version: String,
    /// Prior installed version, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_version_ref: Option<String>,
    /// Source package the flow operates on.
    pub source_package_ref: String,
    /// Published flow-contract version this row pins.
    pub flow_contract_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl LifecycleFlowIdentity {
    /// Returns true when the row pins the published stable flow-contract version.
    pub fn flow_contract_version_current(&self) -> bool {
        self.flow_contract_version == STABLE_LIFECYCLE_FLOW_PUBLISHED_VERSION
    }

    /// Returns true when the lifecycle is installable.
    pub fn lifecycle_installable(&self) -> bool {
        INSTALLABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }

    /// Returns true when this flow is install-shaped (a resolution lands an artifact).
    pub fn install_shaped(&self) -> bool {
        INSTALL_SHAPED_FLOWS.contains(&self.flow_class.as_str())
    }
}

/// One resolved dependency-tree node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyNode {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable node id.
    pub node_id: String,
    /// Node kind class.
    pub node_kind_class: String,
    /// Target extension / package ref.
    pub target_ref: String,
    /// Resolved version range ref.
    pub version_range_ref: String,
    /// Resolution state class.
    pub resolution_state_class: String,
    /// Permission refs this node contributes to the effective set.
    pub contributed_permission_refs: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl DependencyNode {
    /// Returns true when this node is a hard dependency.
    pub fn is_hard_dependency(&self) -> bool {
        self.node_kind_class == "hard_dependency"
    }

    /// Returns true when this node is an optional integration.
    pub fn is_optional_integration(&self) -> bool {
        self.node_kind_class == "optional_integration"
    }

    /// Returns true when a hard dependency failed to resolve.
    pub fn unresolved_hard(&self) -> bool {
        self.is_hard_dependency() && self.resolution_state_class == "unresolved_missing"
    }

    /// Returns true when a hard dependency hit a version conflict.
    pub fn version_conflict_hard(&self) -> bool {
        self.is_hard_dependency() && self.resolution_state_class == "version_conflict"
    }
}

/// Deterministic resolver output and dependency tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicResolution {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Resolver determinism class.
    pub determinism_class: String,
    /// Install scope the resolution was run for.
    pub install_scope_class: String,
    /// Ref to the deterministic resolution digest.
    pub resolution_digest_ref: String,
    /// Ref to the resolver input set.
    pub resolver_input_ref: String,
    /// Resolved dependency tree nodes.
    pub nodes: Vec<DependencyNode>,
    /// Total node count (derived).
    pub node_count: usize,
    /// Hard-dependency count (derived).
    pub hard_dependency_count: usize,
    /// Optional-integration count (derived).
    pub optional_integration_count: usize,
    /// Unresolved hard-dependency count (derived).
    pub unresolved_hard_dependency_count: usize,
    /// Version-conflict hard-dependency count (derived).
    pub version_conflict_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

impl DeterministicResolution {
    /// Returns true when the resolution is deterministic.
    pub fn deterministic(&self) -> bool {
        self.determinism_class == "deterministic"
    }

    /// Returns true when the resolution has not run.
    pub fn not_resolved(&self) -> bool {
        self.determinism_class == "not_resolved"
    }

    /// Returns true when every hard dependency resolved cleanly.
    pub fn all_hard_dependencies_resolved(&self) -> bool {
        self.unresolved_hard_dependency_count == 0 && self.version_conflict_count == 0
    }
}

/// Effective permission inheritance after dependency resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionInheritance {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Top-level declared permission refs (sorted, deduped).
    pub declared_permission_refs: Vec<String>,
    /// Transitive permission refs from resolved hard dependencies (sorted, deduped).
    pub transitive_permission_refs: Vec<String>,
    /// Effective permission refs — declared ∪ transitive (sorted, deduped, derived).
    pub effective_permission_refs: Vec<String>,
    /// Optional-integration permission refs, surfaced separately (sorted, deduped).
    pub optional_integration_permission_refs: Vec<String>,
    /// Prior installed effective permission refs (sorted, deduped).
    pub prior_effective_permission_refs: Vec<String>,
    /// Permission refs newly added relative to the prior effective set (derived).
    pub expanded_permission_refs: Vec<String>,
    /// Effective-permission-expansion class (derived).
    pub expansion_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl EffectivePermissionInheritance {
    /// Returns true when the effective permission set expanded over the prior set.
    pub fn expanded(&self) -> bool {
        self.expansion_class == "expanded"
    }
}

/// Re-consent requirement raised whenever resolution expands the effective set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReConsentRequirement {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Re-consent state.
    pub reconsent_state_class: String,
    /// Whether re-consent was triggered by an effective-permission expansion (derived).
    pub triggered_by_permission_expansion: bool,
    /// Whether re-consent was triggered by a top-level package manifest change.
    pub triggered_by_manifest_change: bool,
    /// Ref to the obtained consent record, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_record_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl ReConsentRequirement {
    /// Returns true when re-consent is required by either trigger.
    pub fn required(&self) -> bool {
        self.triggered_by_permission_expansion || self.triggered_by_manifest_change
    }

    /// Returns true when a required re-consent has been obtained.
    pub fn satisfied(&self) -> bool {
        !self.required() || self.reconsent_state_class == "required_obtained"
    }
}

/// Lock / export plan binding for reproducible team or air-gapped rollout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockExportPlan {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Lock/export state.
    pub lock_export_state_class: String,
    /// Ref to the lockfile artifact.
    pub lock_plan_ref: String,
    /// Ref to the install-plan artifact.
    pub install_plan_ref: String,
    /// Whether the plan supports a team rollout.
    pub supports_team_rollout: bool,
    /// Whether the plan supports an air-gapped rollout.
    pub supports_air_gapped_rollout: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl LockExportPlan {
    /// Returns true when an exportable lock/install plan is available.
    pub fn available(&self) -> bool {
        matches!(
            self.lock_export_state_class.as_str(),
            "exported" | "exportable"
        )
    }
}

/// Disable / rollback posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisableRollbackPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Rollback state.
    pub rollback_state_class: String,
    /// Ref to the pinned rollback target, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_target_ref: Option<String>,
    /// Ref to the rollback manifest, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_manifest_ref: Option<String>,
    /// Whether a disable is reversible without data loss.
    pub disable_reversible: bool,
    /// Whether user/extension data is retained across a disable.
    pub data_retained_on_disable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl DisableRollbackPosture {
    /// Returns true when rollback is reversible (target pinned or not).
    pub fn reversible(&self) -> bool {
        matches!(
            self.rollback_state_class.as_str(),
            "reversible_target_pinned" | "reversible_no_target"
        )
    }

    /// Returns true when rollback is explicitly irreversible.
    pub fn irreversible(&self) -> bool {
        self.rollback_state_class == "irreversible"
    }

    /// Returns true when a reversible rollback carries a pinned target.
    pub fn target_pinned(&self) -> bool {
        self.rollback_state_class == "reversible_target_pinned"
            && self
                .rollback_target_ref
                .as_ref()
                .is_some_and(|r| !r.trim().is_empty())
    }
}

/// Revocation posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Revocation state.
    pub revocation_state_class: String,
    /// Revocation-propagation class across primary / mirror / offline sources.
    pub propagation_class: String,
    /// Ref to the revocation/incident record, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_source_ref: Option<String>,
    /// Ref to the recovery-guidance record, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_guidance_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl RevocationPosture {
    /// Returns true when a revocation action is in force (not merely advisory/none).
    pub fn revocation_in_force(&self) -> bool {
        matches!(
            self.revocation_state_class.as_str(),
            "emergency_disabled" | "quarantined" | "revoked"
        )
    }

    /// Returns true when the revocation has propagated across every source.
    pub fn fully_propagated(&self) -> bool {
        self.propagation_class == "propagated_all_sources"
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFlowQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Lifecycle-flow tier claimed by the row.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: evidence-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Downgraded-flow banner requirement. Raised whenever a reviewer must see a flow
/// shortfall before relying on the flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedFlowBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-flow banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFlowInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Subject class.
    pub subject_class: String,
    /// Flow class.
    pub flow_class: String,
    /// Effective lifecycle-flow tier.
    pub effective_tier: String,
    /// True when the claim is a stable lifecycle-flow claim.
    pub stable_claim: bool,
    /// True when the row pins the published flow-contract version.
    pub flow_contract_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is installable.
    pub lifecycle_installable: bool,
    /// Install scope class.
    pub install_scope_class: String,
    /// Resolver determinism class.
    pub determinism_class: String,
    /// True when the resolution is deterministic.
    pub resolution_deterministic: bool,
    /// True when every hard dependency resolved.
    pub all_hard_dependencies_resolved: bool,
    /// Effective-permission-expansion class.
    pub expansion_class: String,
    /// True when a required re-consent has been satisfied.
    pub reconsent_satisfied: bool,
    /// Lock/export state.
    pub lock_export_state_class: String,
    /// True when an exportable lock/install plan is available.
    pub lock_export_available: bool,
    /// True when team and air-gapped rollout are both supported.
    pub team_and_air_gapped_rollout_supported: bool,
    /// Rollback state.
    pub rollback_state_class: String,
    /// True when rollback is reversible.
    pub rollback_reversible: bool,
    /// Revocation state.
    pub revocation_state_class: String,
    /// Revocation-propagation class.
    pub revocation_propagation_class: String,
    /// True when revocation has propagated across every source.
    pub revocation_fully_propagated: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-flow banner is required.
    pub downgraded_banner_required: bool,
    /// True when identity and every artifact are fully attributed.
    pub attribution_complete: bool,
    /// Dependency-tree node count.
    pub node_count: usize,
    /// Hard-dependency count.
    pub hard_dependency_count: usize,
    /// Optional-integration count.
    pub optional_integration_count: usize,
    /// Declared permission count.
    pub declared_permission_count: usize,
    /// Effective permission count.
    pub effective_permission_count: usize,
    /// Number of permissions newly expanded relative to the prior set.
    pub expanded_permission_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable lifecycle-flow hardening packet consumed by install review, update
/// review, the disable/rollback panel, the revocation panel, the extension
/// inspector, diagnostics, support export, docs/help, release packets, the CLI
/// inspector, and mirror packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLifecycleFlowPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: LifecycleFlowIdentity,
    /// Deterministic resolver output.
    pub resolution: DeterministicResolution,
    /// Effective permission inheritance.
    pub permissions: EffectivePermissionInheritance,
    /// Re-consent requirement.
    pub reconsent: ReConsentRequirement,
    /// Lock / export plan.
    pub lock_export: LockExportPlan,
    /// Disable / rollback posture.
    pub disable_rollback: DisableRollbackPosture,
    /// Revocation posture.
    pub revocation: RevocationPosture,
    /// Stability qualification claim after the posture is applied.
    pub claim: LifecycleFlowQualificationClaim,
    /// Downgraded-flow banner requirement.
    pub downgraded_banner: DowngradedFlowBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so an ambient/implicit permission expansion can never ride a stable flow.
    pub allows_ambient_permission_expansion: bool,
    /// False so a nondeterministic resolution can never ride a stable install/update.
    pub allows_nondeterministic_install: bool,
    /// False so a catalog row can never imply stable trust on its own.
    pub allows_catalog_only_trust: bool,
    /// Inspection row.
    pub inspection: LifecycleFlowInspection,
}

impl StableLifecycleFlowPacket {
    /// Builds a stable lifecycle-flow packet from input, deriving the dependency
    /// counts, the effective-permission set and expansion, the re-consent trigger,
    /// and applying the flow posture to the claimed tier so any required downgrade
    /// below Stable is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableLifecycleFlowValidationError`] when the input violates an
    /// identity, resolution, permission, re-consent, lock/export, rollback,
    /// revocation, or claim invariant.
    pub fn from_input(
        input: StableLifecycleFlowInput,
    ) -> Result<Self, StableLifecycleFlowValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let resolution = resolution_record(&input.resolution);
        let permissions = permissions_record(&input.permissions);
        let reconsent = reconsent_record(&input.reconsent, &permissions);
        let lock_export = lock_export_record(&input.lock_export);
        let disable_rollback = disable_rollback_record(&input.disable_rollback);
        let revocation = revocation_record(&input.revocation);
        let attribution_complete = attribution_is_complete(&identity, &resolution);

        let posture = FlowPosture {
            identity: &identity,
            resolution: &resolution,
            permissions: &permissions,
            reconsent: &reconsent,
            lock_export: &lock_export,
            disable_rollback: &disable_rollback,
            revocation: &revocation,
            attribution_complete,
        };

        let claim = claim_record(&input.claim, &posture);
        let downgraded_banner = banner_record(&posture);
        let inspection = inspection_record(&input.packet_id, &posture, &claim, &downgraded_banner);

        let packet = Self {
            record_kind: STABLE_LIFECYCLE_FLOW_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            resolution,
            permissions,
            reconsent,
            lock_export,
            disable_rollback,
            revocation,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_LIFECYCLE_FLOW_SCHEMA_REF.to_string()],
            allows_ambient_permission_expansion: false,
            allows_nondeterministic_install: false,
            allows_catalog_only_trust: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable lifecycle-flow invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableLifecycleFlowValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableLifecycleFlowValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_LIFECYCLE_FLOW_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_resolution(&self.resolution)?;
        validate_permissions(&self.permissions)?;
        validate_reconsent(&self.reconsent)?;
        validate_lock_export(&self.lock_export)?;
        validate_disable_rollback(&self.disable_rollback)?;
        validate_revocation(&self.revocation)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_LIFECYCLE_FLOW_CONSUMER_SURFACES,
                surface,
                "consumer_surface",
            )?;
        }
        if self.consumer_surfaces.is_empty() {
            return Err(err("packet must bind at least one consumer surface"));
        }
        if !self
            .source_schema_refs
            .iter()
            .any(|r| r == STABLE_LIFECYCLE_FLOW_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No ambient permission expansion, nondeterministic install, or catalog-only
        // trust may ride a published stable lifecycle-flow row.
        if self.allows_ambient_permission_expansion
            || self.allows_nondeterministic_install
            || self.allows_catalog_only_trust
        {
            return Err(err(
                "a stable lifecycle-flow packet must not allow ambient permission expansion, nondeterministic install, or catalog-only trust",
            ));
        }

        // The dependency counts are re-derived from the nodes so a stored packet
        // cannot hide an unresolved or conflicting hard dependency.
        let derived_resolution = derive_resolution_counts(&self.resolution);
        if derived_resolution != counts_of(&self.resolution) {
            return Err(err(
                "stored dependency counts do not match the node-derived truth",
            ));
        }

        // The effective permission set is re-derived from declared ∪ transitive, and
        // the expansion is re-derived from the prior-vs-effective diff, so a stored
        // packet cannot hide a transitive permission or a silent expansion.
        let derived_perms = derive_permissions(
            &self.permissions.declared_permission_refs,
            &self.permissions.transitive_permission_refs,
            &self.permissions.optional_integration_permission_refs,
            &self.permissions.prior_effective_permission_refs,
        );
        if derived_perms.effective_permission_refs != self.permissions.effective_permission_refs {
            return Err(err(
                "stored effective permission set does not match the declared-union-transitive truth",
            ));
        }
        if derived_perms.expanded_permission_refs != self.permissions.expanded_permission_refs {
            return Err(err(
                "stored expanded permission set does not match the prior-vs-effective diff",
            ));
        }
        if derived_perms.expansion_class != self.permissions.expansion_class {
            return Err(err(
                "stored expansion class does not match the prior-vs-effective diff",
            ));
        }

        // The re-consent permission-expansion trigger is re-derived from the diff so
        // re-consent is required whenever the effective set expands, not only on a
        // top-level manifest change.
        if self.reconsent.triggered_by_permission_expansion != self.permissions.expanded() {
            return Err(err(
                "re-consent permission-expansion trigger must match the effective-permission expansion",
            ));
        }
        if self.reconsent.required() && self.reconsent.reconsent_state_class == "not_required" {
            return Err(err(
                "a required re-consent must not carry a not_required state",
            ));
        }
        if !self.reconsent.required() && self.reconsent.reconsent_state_class != "not_required" {
            return Err(err(
                "re-consent state must be not_required when no trigger is present",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.flow_contract_version_current() {
                return Err(err(
                    "stable effective tier must pin the published flow-contract version",
                ));
            }
            if self.claim.claim_basis_class != "evidence_backed" {
                return Err(err(
                    "stable effective tier must be evidence-backed, not catalog-asserted",
                ));
            }
            if self.identity.publisher_trust_tier_class == "quarantined" {
                return Err(err(
                    "stable effective tier must not carry a quarantined trust tier",
                ));
            }
            if !self.resolution.deterministic() {
                return Err(err(
                    "stable effective tier must carry a deterministic resolution",
                ));
            }
            if !self.resolution.all_hard_dependencies_resolved() {
                return Err(err(
                    "stable effective tier must resolve every hard dependency",
                ));
            }
            if !self.reconsent.satisfied() {
                return Err(err(
                    "stable effective tier must obtain re-consent whenever the effective permission set expanded",
                ));
            }
            if !self.lock_export.available() {
                return Err(err(
                    "stable effective tier must expose an exportable lock/install plan",
                ));
            }
            if self.identity.install_shaped() {
                if !self.identity.lifecycle_installable() {
                    return Err(err(
                        "stable install-shaped tier must stay on an installable lifecycle",
                    ));
                }
                if !self.lock_export.supports_team_rollout
                    || !self.lock_export.supports_air_gapped_rollout
                {
                    return Err(err(
                        "stable install-shaped tier must support team and air-gapped rollout",
                    ));
                }
            }
            if self.identity.flow_class == "disable" || self.identity.flow_class == "rollback" {
                if self.disable_rollback.irreversible() {
                    return Err(err(
                        "stable disable/rollback tier must keep a reversible rollback",
                    ));
                }
                if !self.disable_rollback.reversible() {
                    return Err(err(
                        "stable disable/rollback tier must declare a reversible rollback posture",
                    ));
                }
            }
            if self.identity.flow_class == "revocation" {
                if !self.revocation.revocation_in_force() {
                    return Err(err(
                        "stable revocation tier must carry a revocation action in force",
                    ));
                }
                if !self.revocation.fully_propagated() {
                    return Err(err(
                        "stable revocation tier must propagate across every source",
                    ));
                }
            }
            if !self.attribution_complete() {
                return Err(err("stable effective tier must be fully attributed"));
            }
            if self.claim.downgraded {
                return Err(err(
                    "a stable effective tier must not also be marked downgraded",
                ));
            }
        }

        // Downgrade truth.
        if self.claim.downgraded {
            if self.claim.downgrade_reasons.is_empty() {
                return Err(err("a downgraded claim must carry at least one reason"));
            }
            if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
                return Err(err("a downgraded claim must not keep a stable tier"));
            }
        }

        // Re-derive the effective tier and downgrade verdict so the stored claim
        // cannot drift from the posture truth.
        let posture = FlowPosture {
            identity: &self.identity,
            resolution: &self.resolution,
            permissions: &self.permissions,
            reconsent: &self.reconsent,
            lock_export: &self.lock_export,
            disable_rollback: &self.disable_rollback,
            revocation: &self.revocation,
            attribution_complete: self.attribution_complete(),
        };
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &posture,
        );
        if derived.effective_tier != self.claim.effective_tier {
            return Err(err(
                "stored effective tier does not match the posture-derived tier",
            ));
        }
        if derived.downgraded != self.claim.downgraded {
            return Err(err(
                "stored downgrade flag does not match the posture-derived verdict",
            ));
        }
        let stored: BTreeSet<&str> = self
            .claim
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        let expected: BTreeSet<&str> = derived
            .downgrade_reasons
            .iter()
            .map(String::as_str)
            .collect();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Banner truth.
        let banner_required = flow_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-flow banner must_display does not match the flow posture",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from catalog-only trust.
    pub fn no_catalog_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "evidence_backed";
        }
        true
    }

    /// Returns true when identity and the resolution are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(&self.identity, &self.resolution)
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableLifecycleFlowProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Subject class.
    pub subject_class: String,
    /// Flow class.
    pub flow_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable lifecycle-flow claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-flow banner is required.
    pub downgraded_banner_required: bool,
    /// Install scope class.
    pub install_scope_class: String,
    /// Effective-permission-expansion class.
    pub expansion_class: String,
    /// Re-consent state class.
    pub reconsent_state_class: String,
    /// Rollback state class.
    pub rollback_state_class: String,
    /// Revocation state class.
    pub revocation_state_class: String,
    /// Effective permission count.
    pub effective_permission_count: usize,
}

impl From<StableLifecycleFlowPacket> for StableLifecycleFlowProjection {
    fn from(packet: StableLifecycleFlowPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            subject_class: packet.identity.subject_class,
            flow_class: packet.identity.flow_class,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_banner_required: packet.downgraded_banner.must_display,
            install_scope_class: packet.resolution.install_scope_class,
            expansion_class: packet.permissions.expansion_class,
            reconsent_state_class: packet.reconsent.reconsent_state_class,
            rollback_state_class: packet.disable_rollback.rollback_state_class,
            revocation_state_class: packet.revocation.revocation_state_class,
            effective_permission_count: packet.permissions.effective_permission_refs.len(),
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableLifecycleFlowError`] when the payload fails to parse or violates
/// the stable lifecycle-flow invariants.
pub fn project_stable_lifecycle_flow(
    payload: &str,
) -> Result<StableLifecycleFlowProjection, StableLifecycleFlowError> {
    let packet: StableLifecycleFlowPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableLifecycleFlowProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support/partner/mirror export row that quotes the same closed
/// tokens as the packet without leaking raw lockfile, digest, consent, or
/// publisher-private bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableLifecycleFlowSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Subject class.
    pub subject_class: String,
    /// Flow class.
    pub flow_class: String,
    /// Subject identity.
    pub subject_identity: String,
    /// Subject version.
    pub subject_version: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Install scope class.
    pub install_scope_class: String,
    /// Resolver determinism class.
    pub determinism_class: String,
    /// True when every hard dependency resolved.
    pub all_hard_dependencies_resolved: bool,
    /// Effective-permission-expansion class.
    pub expansion_class: String,
    /// Re-consent state.
    pub reconsent_state_class: String,
    /// True when a required re-consent has been satisfied.
    pub reconsent_satisfied: bool,
    /// Lock/export state.
    pub lock_export_state_class: String,
    /// True when team and air-gapped rollout are both supported.
    pub team_and_air_gapped_rollout_supported: bool,
    /// Rollback state.
    pub rollback_state_class: String,
    /// Revocation state.
    pub revocation_state_class: String,
    /// Revocation-propagation class.
    pub revocation_propagation_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-flow banner is required.
    pub downgraded_banner_required: bool,
    /// True when the effective tier blocks the flow as stable (withdrawn).
    pub blocks_stable_flow: bool,
    /// Dependency-tree node count.
    pub node_count: usize,
    /// Effective permission count.
    pub effective_permission_count: usize,
    /// Number of permissions newly expanded relative to the prior set.
    pub expanded_permission_count: usize,
    /// Export-safe summary suitable for support/partner/mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support/partner/mirror export row.
pub fn project_stable_lifecycle_flow_support_export(
    packet: &StableLifecycleFlowPacket,
) -> StableLifecycleFlowSupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let team_and_air_gapped =
        packet.lock_export.supports_team_rollout && packet.lock_export.supports_air_gapped_rollout;
    let export_safe_summary = format!(
        "{} Subject={} flow={} trust={} lifecycle={}. Scope={} resolver={} (hard_resolved={}). Permissions effective={} expansion={} expanded={}. Re-consent={} (satisfied={}). Lock/export={} team_air_gapped={}. Rollback={}. Revocation={} propagation={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.identity.subject_class,
        packet.identity.flow_class,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        packet.resolution.install_scope_class,
        packet.resolution.determinism_class,
        packet.resolution.all_hard_dependencies_resolved(),
        packet.permissions.effective_permission_refs.len(),
        packet.permissions.expansion_class,
        packet.permissions.expanded_permission_refs.len(),
        packet.reconsent.reconsent_state_class,
        packet.reconsent.satisfied(),
        packet.lock_export.lock_export_state_class,
        team_and_air_gapped,
        packet.disable_rollback.rollback_state_class,
        packet.revocation.revocation_state_class,
        packet.revocation.propagation_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StableLifecycleFlowSupportExport {
        record_kind: STABLE_LIFECYCLE_FLOW_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        export_id: format!("stable_lifecycle_flow_support_export:{}", packet.packet_id),
        packet_ref: packet.packet_id.clone(),
        subject_class: packet.identity.subject_class.clone(),
        flow_class: packet.identity.flow_class.clone(),
        subject_identity: packet.identity.subject_identity.clone(),
        subject_version: packet.identity.subject_version.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        install_scope_class: packet.resolution.install_scope_class.clone(),
        determinism_class: packet.resolution.determinism_class.clone(),
        all_hard_dependencies_resolved: packet.resolution.all_hard_dependencies_resolved(),
        expansion_class: packet.permissions.expansion_class.clone(),
        reconsent_state_class: packet.reconsent.reconsent_state_class.clone(),
        reconsent_satisfied: packet.reconsent.satisfied(),
        lock_export_state_class: packet.lock_export.lock_export_state_class.clone(),
        team_and_air_gapped_rollout_supported: team_and_air_gapped,
        rollback_state_class: packet.disable_rollback.rollback_state_class.clone(),
        revocation_state_class: packet.revocation.revocation_state_class.clone(),
        revocation_propagation_class: packet.revocation.propagation_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_flow: blocks,
        node_count: packet.resolution.node_count,
        effective_permission_count: packet.permissions.effective_permission_refs.len(),
        expanded_permission_count: packet.permissions.expanded_permission_refs.len(),
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable lifecycle-flow operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableLifecycleFlowError {
    /// Validation failed.
    Validation(StableLifecycleFlowValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableLifecycleFlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableLifecycleFlowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable lifecycle-flow packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableLifecycleFlowValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableLifecycleFlowValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableLifecycleFlowValidationError {}

impl StableLifecycleFlowValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableLifecycleFlowError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableLifecycleFlowValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableLifecycleFlowValidationError> for StableLifecycleFlowError {
    fn from(err: StableLifecycleFlowValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Derivation helpers
// ---------------------------------------------------------------------------

/// Sorted, deduped clone of a ref list.
fn normalized(refs: &[String]) -> Vec<String> {
    let set: BTreeSet<String> = refs.iter().map(|r| r.trim().to_string()).collect();
    set.into_iter().collect()
}

struct DerivedPermissions {
    declared: Vec<String>,
    transitive: Vec<String>,
    optional: Vec<String>,
    prior: Vec<String>,
    effective_permission_refs: Vec<String>,
    expanded_permission_refs: Vec<String>,
    expansion_class: String,
}

/// Derives the effective set (declared ∪ transitive), the optional set, and the
/// expansion against the prior installed effective set. The effective set and the
/// expansion are pure functions of their inputs so they cannot drift from evidence.
fn derive_permissions(
    declared: &[String],
    transitive: &[String],
    optional: &[String],
    prior: &[String],
) -> DerivedPermissions {
    let declared = normalized(declared);
    let transitive = normalized(transitive);
    let optional = normalized(optional);
    let prior = normalized(prior);

    let effective_set: BTreeSet<String> =
        declared.iter().chain(transitive.iter()).cloned().collect();
    let effective_permission_refs: Vec<String> = effective_set.iter().cloned().collect();

    let prior_set: BTreeSet<String> = prior.iter().cloned().collect();
    let expanded_permission_refs: Vec<String> = effective_set
        .iter()
        .filter(|p| !prior_set.contains(*p))
        .cloned()
        .collect();
    let dropped = prior_set.iter().any(|p| !effective_set.contains(p));

    let expansion_class = if !expanded_permission_refs.is_empty() {
        "expanded"
    } else if dropped {
        "reduced"
    } else {
        "no_change"
    }
    .to_string();

    DerivedPermissions {
        declared,
        transitive,
        optional,
        prior,
        effective_permission_refs,
        expanded_permission_refs,
        expansion_class,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ResolutionCounts {
    node_count: usize,
    hard_dependency_count: usize,
    optional_integration_count: usize,
    unresolved_hard_dependency_count: usize,
    version_conflict_count: usize,
}

fn counts_of(resolution: &DeterministicResolution) -> ResolutionCounts {
    ResolutionCounts {
        node_count: resolution.node_count,
        hard_dependency_count: resolution.hard_dependency_count,
        optional_integration_count: resolution.optional_integration_count,
        unresolved_hard_dependency_count: resolution.unresolved_hard_dependency_count,
        version_conflict_count: resolution.version_conflict_count,
    }
}

fn derive_resolution_counts(resolution: &DeterministicResolution) -> ResolutionCounts {
    ResolutionCounts {
        node_count: resolution.nodes.len(),
        hard_dependency_count: resolution
            .nodes
            .iter()
            .filter(|n| n.is_hard_dependency())
            .count(),
        optional_integration_count: resolution
            .nodes
            .iter()
            .filter(|n| n.is_optional_integration())
            .count(),
        unresolved_hard_dependency_count: resolution
            .nodes
            .iter()
            .filter(|n| n.unresolved_hard())
            .count(),
        version_conflict_count: resolution
            .nodes
            .iter()
            .filter(|n| n.version_conflict_hard())
            .count(),
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the flow posture.
struct FlowPosture<'a> {
    identity: &'a LifecycleFlowIdentity,
    resolution: &'a DeterministicResolution,
    permissions: &'a EffectivePermissionInheritance,
    reconsent: &'a ReConsentRequirement,
    lock_export: &'a LockExportPlan,
    disable_rollback: &'a DisableRollbackPosture,
    revocation: &'a RevocationPosture,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the flow posture. Several reasons
/// are flow-aware: a lifecycle / rollout shortfall only matters for an
/// install-shaped flow, a rollback shortfall only for disable/rollback, and a
/// revocation shortfall only for the revocation flow.
fn posture_reasons(posture: &FlowPosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();
    let install_shaped = posture.identity.install_shaped();
    let flow = posture.identity.flow_class.as_str();

    if !posture.identity.flow_contract_version_current() {
        reasons.push("flow_contract_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if install_shaped && !posture.identity.lifecycle_installable() {
        reasons.push("lifecycle_not_installable".to_string());
    }

    if posture.resolution.not_resolved() {
        reasons.push("resolver_not_resolved".to_string());
    } else if !posture.resolution.deterministic() {
        reasons.push("resolver_nondeterministic".to_string());
    }
    if posture.resolution.unresolved_hard_dependency_count > 0 {
        reasons.push("unresolved_hard_dependency".to_string());
    }
    if posture.resolution.version_conflict_count > 0 {
        reasons.push("version_conflict_dependency".to_string());
    }

    if posture.permissions.expanded() {
        match posture.reconsent.reconsent_state_class.as_str() {
            "required_obtained" => {}
            "required_pending" => reasons.push("reconsent_pending".to_string()),
            _ => reasons.push("permission_expansion_without_reconsent".to_string()),
        }
    } else if posture.reconsent.required() && !posture.reconsent.satisfied() {
        // A manifest-change-only re-consent that has not been obtained.
        match posture.reconsent.reconsent_state_class.as_str() {
            "required_pending" => reasons.push("reconsent_pending".to_string()),
            _ => reasons.push("permission_expansion_without_reconsent".to_string()),
        }
    }

    if !posture.lock_export.available() {
        reasons.push("lock_export_unavailable".to_string());
    }
    if install_shaped {
        if !posture.lock_export.supports_team_rollout {
            reasons.push("team_rollout_unsupported".to_string());
        }
        if !posture.lock_export.supports_air_gapped_rollout {
            reasons.push("air_gapped_rollout_unsupported".to_string());
        }
    }

    if flow == "disable" || flow == "rollback" {
        if posture.disable_rollback.irreversible() {
            reasons.push("rollback_irreversible".to_string());
        } else if posture.disable_rollback.reversible()
            && !posture.disable_rollback.target_pinned()
            && posture.disable_rollback.rollback_state_class == "reversible_target_pinned"
        {
            reasons.push("rollback_target_missing".to_string());
        }
    }

    if flow == "revocation" {
        if !posture.revocation.revocation_in_force() {
            reasons.push("revocation_state_missing".to_string());
        }
        if !posture.revocation.fully_propagated() {
            reasons.push("revocation_not_propagated".to_string());
        }
    }

    if !posture.attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();
    reasons
}

/// Applies the flow posture to a claimed tier, narrowing automatically below Stable
/// when the evidence can no longer back it. The claim basis is folded in separately
/// so a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &FlowPosture<'_>,
) -> DerivedTier {
    // Non-stable claims are already honest; they pass through unchanged.
    if !STABLE_TIERS.contains(&claimed_tier) {
        return DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        };
    }

    let mut reasons = posture_reasons(posture);
    if claim_basis != "evidence_backed" {
        reasons.push("catalog_only_trust_not_evidence_backed".to_string());
        reasons.sort();
        reasons.dedup();
    }

    if reasons.is_empty() {
        DerivedTier {
            effective_tier: claimed_tier.to_string(),
            support_claim: support_claim_for(claimed_tier),
            downgraded: false,
            downgrade_reasons: Vec::new(),
        }
    } else {
        let effective_tier = narrow_tier_for(&reasons);
        DerivedTier {
            effective_tier: effective_tier.to_string(),
            support_claim: support_claim_for(effective_tier),
            downgraded: true,
            downgrade_reasons: reasons,
        }
    }
}

/// Picks the effective tier given the active narrowing reasons.
fn narrow_tier_for(reasons: &[String]) -> &'static str {
    if reasons
        .iter()
        .any(|r| WITHDRAWN_CLASS_REASONS.contains(&r.as_str()))
    {
        "withdrawn"
    } else if reasons
        .iter()
        .any(|r| PREVIEW_CLASS_REASONS.contains(&r.as_str()))
    {
        "preview"
    } else {
        debug_assert!(reasons
            .iter()
            .all(|r| BETA_CLASS_REASONS.contains(&r.as_str())));
        "beta"
    }
}

/// Maps an effective tier to the support claim it may imply.
fn support_claim_for(tier: &str) -> String {
    match tier {
        "stable" => "stable_lifecycle_flow_claim",
        "beta" => "beta_lifecycle_flow_partial_claim",
        "preview" => "preview_lifecycle_flow_experimental_claim",
        "withdrawn" => "withdrawn_no_lifecycle_flow_claim",
        _ => "preview_lifecycle_flow_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity and the resolution are fully attributed.
fn attribution_is_complete(
    identity: &LifecycleFlowIdentity,
    resolution: &DeterministicResolution,
) -> bool {
    !identity.source_review_ref.trim().is_empty()
        && !identity.source_package_ref.trim().is_empty()
        && !resolution.resolution_digest_ref.trim().is_empty()
        && !resolution.resolver_input_ref.trim().is_empty()
        && resolution
            .nodes
            .iter()
            .all(|n| !n.node_id.trim().is_empty() && !n.target_ref.trim().is_empty())
}

/// Returns true when the flow posture requires a pre-trust warning banner.
fn flow_requires_warning(posture: &FlowPosture<'_>) -> bool {
    let install_shaped = posture.identity.install_shaped();
    let flow = posture.identity.flow_class.as_str();
    posture.identity.publisher_trust_tier_class == "quarantined"
        || (install_shaped && !posture.identity.lifecycle_installable())
        || posture.resolution.not_resolved()
        || !posture.resolution.deterministic()
        || posture.resolution.unresolved_hard_dependency_count > 0
        || posture.resolution.version_conflict_count > 0
        || (posture.permissions.expanded() && !posture.reconsent.satisfied())
        || !posture.lock_export.available()
        || ((flow == "disable" || flow == "rollback") && posture.disable_rollback.irreversible())
        || (flow == "revocation"
            && (!posture.revocation.revocation_in_force()
                || !posture.revocation.fully_propagated()))
}

/// Picks the most-severe banner reason for a flow that requires a warning.
fn banner_reason_for(posture: &FlowPosture<'_>) -> Option<String> {
    let flow = posture.identity.flow_class.as_str();
    if posture.resolution.not_resolved() {
        return Some("resolver_not_resolved".to_string());
    }
    if posture.resolution.unresolved_hard_dependency_count > 0 {
        return Some("unresolved_hard_dependency".to_string());
    }
    if posture.resolution.version_conflict_count > 0 {
        return Some("version_conflict_dependency".to_string());
    }
    if (flow == "disable" || flow == "rollback") && posture.disable_rollback.irreversible() {
        return Some("rollback_irreversible".to_string());
    }
    if flow == "revocation" && !posture.revocation.fully_propagated() {
        return Some("revocation_not_propagated".to_string());
    }
    if flow == "revocation" && !posture.revocation.revocation_in_force() {
        return Some("revocation_state_missing".to_string());
    }
    if posture.identity.install_shaped() && !posture.identity.lifecycle_installable() {
        return Some("lifecycle_not_installable".to_string());
    }
    if !posture.resolution.deterministic() {
        return Some("resolver_nondeterministic".to_string());
    }
    if posture.permissions.expanded() && !posture.reconsent.satisfied() {
        return Some("permission_expansion_without_reconsent".to_string());
    }
    if !posture.lock_export.available() {
        return Some("lock_export_unavailable".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &LifecycleFlowIdentityInput) -> LifecycleFlowIdentity {
    LifecycleFlowIdentity {
        record_kind: LIFECYCLE_FLOW_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        subject_class: input.subject_class.clone(),
        flow_class: input.flow_class.clone(),
        source_review_ref: input.source_review_ref.clone(),
        subject_identity: input.subject_identity.clone(),
        subject_version: input.subject_version.clone(),
        prior_version_ref: input.prior_version_ref.clone(),
        source_package_ref: input.source_package_ref.clone(),
        flow_contract_version: input.flow_contract_version,
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn node_record(input: &DependencyNodeInput) -> DependencyNode {
    DependencyNode {
        record_kind: DEPENDENCY_NODE_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        node_id: input.node_id.clone(),
        node_kind_class: input.node_kind_class.clone(),
        target_ref: input.target_ref.clone(),
        version_range_ref: input.version_range_ref.clone(),
        resolution_state_class: input.resolution_state_class.clone(),
        contributed_permission_refs: input.contributed_permission_refs.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn resolution_record(input: &DeterministicResolutionInput) -> DeterministicResolution {
    let nodes: Vec<DependencyNode> = input.nodes.iter().map(node_record).collect();
    let mut resolution = DeterministicResolution {
        record_kind: DETERMINISTIC_RESOLUTION_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        determinism_class: input.determinism_class.clone(),
        install_scope_class: input.install_scope_class.clone(),
        resolution_digest_ref: input.resolution_digest_ref.clone(),
        resolver_input_ref: input.resolver_input_ref.clone(),
        nodes,
        node_count: 0,
        hard_dependency_count: 0,
        optional_integration_count: 0,
        unresolved_hard_dependency_count: 0,
        version_conflict_count: 0,
        summary_label: input.summary_label.clone(),
    };
    let counts = derive_resolution_counts(&resolution);
    resolution.node_count = counts.node_count;
    resolution.hard_dependency_count = counts.hard_dependency_count;
    resolution.optional_integration_count = counts.optional_integration_count;
    resolution.unresolved_hard_dependency_count = counts.unresolved_hard_dependency_count;
    resolution.version_conflict_count = counts.version_conflict_count;
    resolution
}

fn permissions_record(
    input: &EffectivePermissionInheritanceInput,
) -> EffectivePermissionInheritance {
    let derived = derive_permissions(
        &input.declared_permission_refs,
        &input.transitive_permission_refs,
        &input.optional_integration_permission_refs,
        &input.prior_effective_permission_refs,
    );
    EffectivePermissionInheritance {
        record_kind: EFFECTIVE_PERMISSION_INHERITANCE_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        declared_permission_refs: derived.declared,
        transitive_permission_refs: derived.transitive,
        effective_permission_refs: derived.effective_permission_refs,
        optional_integration_permission_refs: derived.optional,
        prior_effective_permission_refs: derived.prior,
        expanded_permission_refs: derived.expanded_permission_refs,
        expansion_class: derived.expansion_class,
        summary_label: input.summary_label.clone(),
    }
}

fn reconsent_record(
    input: &ReConsentRequirementInput,
    permissions: &EffectivePermissionInheritance,
) -> ReConsentRequirement {
    ReConsentRequirement {
        record_kind: RECONSENT_REQUIREMENT_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        reconsent_state_class: input.reconsent_state_class.clone(),
        triggered_by_permission_expansion: permissions.expanded(),
        triggered_by_manifest_change: input.manifest_changed,
        consent_record_ref: input.consent_record_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn lock_export_record(input: &LockExportPlanInput) -> LockExportPlan {
    LockExportPlan {
        record_kind: LOCK_EXPORT_PLAN_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        lock_export_state_class: input.lock_export_state_class.clone(),
        lock_plan_ref: input.lock_plan_ref.clone(),
        install_plan_ref: input.install_plan_ref.clone(),
        supports_team_rollout: input.supports_team_rollout,
        supports_air_gapped_rollout: input.supports_air_gapped_rollout,
        summary_label: input.summary_label.clone(),
    }
}

fn disable_rollback_record(input: &DisableRollbackPostureInput) -> DisableRollbackPosture {
    DisableRollbackPosture {
        record_kind: DISABLE_ROLLBACK_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        rollback_state_class: input.rollback_state_class.clone(),
        rollback_target_ref: input.rollback_target_ref.clone(),
        rollback_manifest_ref: input.rollback_manifest_ref.clone(),
        disable_reversible: input.disable_reversible,
        data_retained_on_disable: input.data_retained_on_disable,
        summary_label: input.summary_label.clone(),
    }
}

fn revocation_record(input: &RevocationPostureInput) -> RevocationPosture {
    RevocationPosture {
        record_kind: REVOCATION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        revocation_state_class: input.revocation_state_class.clone(),
        propagation_class: input.propagation_class.clone(),
        revocation_source_ref: input.revocation_source_ref.clone(),
        recovery_guidance_ref: input.recovery_guidance_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &LifecycleFlowQualificationClaimInput,
    posture: &FlowPosture<'_>,
) -> LifecycleFlowQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    LifecycleFlowQualificationClaim {
        record_kind: LIFECYCLE_FLOW_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &FlowPosture<'_>) -> DowngradedFlowBanner {
    let must_display = flow_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Lifecycle flow requires review before it can be trusted ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Lifecycle flow hardened: deterministic resolution, effective permissions, re-consent, lock/export, rollback, and revocation posture all current."
            .to_string()
    };
    DowngradedFlowBanner {
        record_kind: DOWNGRADED_FLOW_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &FlowPosture<'_>,
    claim: &LifecycleFlowQualificationClaim,
    banner: &DowngradedFlowBanner,
) -> LifecycleFlowInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());
    let team_and_air_gapped = posture.lock_export.supports_team_rollout
        && posture.lock_export.supports_air_gapped_rollout;

    LifecycleFlowInspection {
        record_kind: LIFECYCLE_FLOW_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        subject_class: posture.identity.subject_class.clone(),
        flow_class: posture.identity.flow_class.clone(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        flow_contract_version_current: posture.identity.flow_contract_version_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        lifecycle_installable: posture.identity.lifecycle_installable(),
        install_scope_class: posture.resolution.install_scope_class.clone(),
        determinism_class: posture.resolution.determinism_class.clone(),
        resolution_deterministic: posture.resolution.deterministic(),
        all_hard_dependencies_resolved: posture.resolution.all_hard_dependencies_resolved(),
        expansion_class: posture.permissions.expansion_class.clone(),
        reconsent_satisfied: posture.reconsent.satisfied(),
        lock_export_state_class: posture.lock_export.lock_export_state_class.clone(),
        lock_export_available: posture.lock_export.available(),
        team_and_air_gapped_rollout_supported: team_and_air_gapped,
        rollback_state_class: posture.disable_rollback.rollback_state_class.clone(),
        rollback_reversible: posture.disable_rollback.reversible(),
        revocation_state_class: posture.revocation.revocation_state_class.clone(),
        revocation_propagation_class: posture.revocation.propagation_class.clone(),
        revocation_fully_propagated: posture.revocation.fully_propagated(),
        downgraded: claim.downgraded,
        downgraded_banner_required: banner.must_display,
        attribution_complete: posture.attribution_complete,
        node_count: posture.resolution.node_count,
        hard_dependency_count: posture.resolution.hard_dependency_count,
        optional_integration_count: posture.resolution.optional_integration_count,
        declared_permission_count: posture.permissions.declared_permission_refs.len(),
        effective_permission_count: posture.permissions.effective_permission_refs.len(),
        expanded_permission_count: posture.permissions.expanded_permission_refs.len(),
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableLifecycleFlowInput,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_token(SUBJECT_CLASSES, &id.subject_class, "identity.subject_class")?;
    ensure_token(FLOW_CLASSES, &id.flow_class, "identity.flow_class")?;
    ensure_nonempty(&id.source_review_ref, "identity.source_review_ref")?;
    ensure_nonempty(&id.subject_identity, "identity.subject_identity")?;
    ensure_nonempty(&id.subject_version, "identity.subject_version")?;
    ensure_nonempty(&id.source_package_ref, "identity.source_package_ref")?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &id.publisher_trust_tier_class,
        "identity.publisher_trust_tier_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &id.lifecycle_state_class,
        "identity.lifecycle_state_class",
    )?;

    let res = &input.resolution;
    ensure_token(
        RESOLVER_DETERMINISM_CLASSES,
        &res.determinism_class,
        "resolution.determinism_class",
    )?;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &res.install_scope_class,
        "resolution.install_scope_class",
    )?;
    ensure_nonempty(
        &res.resolution_digest_ref,
        "resolution.resolution_digest_ref",
    )?;
    ensure_nonempty(&res.resolver_input_ref, "resolution.resolver_input_ref")?;
    let mut node_ids = BTreeSet::new();
    for n in &res.nodes {
        ensure_nonempty(&n.node_id, "node.node_id")?;
        if !node_ids.insert(&n.node_id) {
            return Err(err(format!("duplicate node_id: {}", n.node_id)));
        }
        ensure_token(
            DEPENDENCY_NODE_KIND_CLASSES,
            &n.node_kind_class,
            "node.node_kind_class",
        )?;
        ensure_nonempty(&n.target_ref, "node.target_ref")?;
        ensure_token(
            DEPENDENCY_RESOLUTION_STATE_CLASSES,
            &n.resolution_state_class,
            "node.resolution_state_class",
        )?;
    }

    let rc = &input.reconsent;
    ensure_token(
        RECONSENT_STATE_CLASSES,
        &rc.reconsent_state_class,
        "reconsent.reconsent_state_class",
    )?;
    if rc.reconsent_state_class == "required_obtained"
        && rc
            .consent_record_ref
            .as_ref()
            .map(|r| r.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(err("an obtained re-consent must bind a consent_record_ref"));
    }

    let lx = &input.lock_export;
    ensure_token(
        LOCK_EXPORT_STATE_CLASSES,
        &lx.lock_export_state_class,
        "lock_export.lock_export_state_class",
    )?;
    if matches!(
        lx.lock_export_state_class.as_str(),
        "exported" | "exportable"
    ) {
        ensure_nonempty(&lx.lock_plan_ref, "lock_export.lock_plan_ref")?;
        ensure_nonempty(&lx.install_plan_ref, "lock_export.install_plan_ref")?;
    }

    let dr = &input.disable_rollback;
    ensure_token(
        ROLLBACK_STATE_CLASSES,
        &dr.rollback_state_class,
        "disable_rollback.rollback_state_class",
    )?;
    if dr.rollback_state_class == "reversible_target_pinned"
        && dr
            .rollback_target_ref
            .as_ref()
            .map(|r| r.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(err(
            "a target-pinned rollback must bind a rollback_target_ref",
        ));
    }

    let rev = &input.revocation;
    ensure_token(
        REVOCATION_STATE_CLASSES,
        &rev.revocation_state_class,
        "revocation.revocation_state_class",
    )?;
    ensure_token(
        REVOCATION_PROPAGATION_CLASSES,
        &rev.propagation_class,
        "revocation.propagation_class",
    )?;

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim.claim_basis_class",
    )?;

    for surface in &input.consumer_surfaces {
        ensure_token(
            STABLE_LIFECYCLE_FLOW_CONSUMER_SURFACES,
            surface,
            "consumer_surface",
        )?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

fn validate_identity(
    identity: &LifecycleFlowIdentity,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        LIFECYCLE_FLOW_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_LIFECYCLE_FLOW_SCHEMA_VERSION,
        "identity schema_version",
    )?;
    ensure_token(
        SUBJECT_CLASSES,
        &identity.subject_class,
        "identity subject_class",
    )?;
    ensure_token(FLOW_CLASSES, &identity.flow_class, "identity flow_class")?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &identity.publisher_trust_tier_class,
        "identity publisher_trust_tier_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &identity.lifecycle_state_class,
        "identity lifecycle_state_class",
    )?;
    Ok(())
}

fn validate_resolution(
    resolution: &DeterministicResolution,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        resolution.record_kind.as_str(),
        DETERMINISTIC_RESOLUTION_RECORD_KIND,
        "resolution record_kind",
    )?;
    ensure_token(
        RESOLVER_DETERMINISM_CLASSES,
        &resolution.determinism_class,
        "resolution determinism_class",
    )?;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &resolution.install_scope_class,
        "resolution install_scope_class",
    )?;
    ensure_nonempty(
        &resolution.resolution_digest_ref,
        "resolution resolution_digest_ref",
    )?;
    ensure_nonempty(
        &resolution.resolver_input_ref,
        "resolution resolver_input_ref",
    )?;
    let mut node_ids = BTreeSet::new();
    for node in &resolution.nodes {
        ensure_eq(
            node.record_kind.as_str(),
            DEPENDENCY_NODE_RECORD_KIND,
            "node record_kind",
        )?;
        ensure_nonempty(&node.node_id, "node node_id")?;
        if !node_ids.insert(node.node_id.as_str()) {
            return Err(err(format!("duplicate node_id: {}", node.node_id)));
        }
        ensure_token(
            DEPENDENCY_NODE_KIND_CLASSES,
            &node.node_kind_class,
            "node node_kind_class",
        )?;
        ensure_token(
            DEPENDENCY_RESOLUTION_STATE_CLASSES,
            &node.resolution_state_class,
            "node resolution_state_class",
        )?;
    }
    Ok(())
}

fn validate_permissions(
    permissions: &EffectivePermissionInheritance,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        permissions.record_kind.as_str(),
        EFFECTIVE_PERMISSION_INHERITANCE_RECORD_KIND,
        "permissions record_kind",
    )?;
    ensure_token(
        PERMISSION_EXPANSION_CLASSES,
        &permissions.expansion_class,
        "permissions expansion_class",
    )?;
    Ok(())
}

fn validate_reconsent(
    reconsent: &ReConsentRequirement,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        reconsent.record_kind.as_str(),
        RECONSENT_REQUIREMENT_RECORD_KIND,
        "reconsent record_kind",
    )?;
    ensure_token(
        RECONSENT_STATE_CLASSES,
        &reconsent.reconsent_state_class,
        "reconsent reconsent_state_class",
    )?;
    Ok(())
}

fn validate_lock_export(
    lock_export: &LockExportPlan,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        lock_export.record_kind.as_str(),
        LOCK_EXPORT_PLAN_RECORD_KIND,
        "lock_export record_kind",
    )?;
    ensure_token(
        LOCK_EXPORT_STATE_CLASSES,
        &lock_export.lock_export_state_class,
        "lock_export lock_export_state_class",
    )?;
    Ok(())
}

fn validate_disable_rollback(
    disable_rollback: &DisableRollbackPosture,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        disable_rollback.record_kind.as_str(),
        DISABLE_ROLLBACK_POSTURE_RECORD_KIND,
        "disable_rollback record_kind",
    )?;
    ensure_token(
        ROLLBACK_STATE_CLASSES,
        &disable_rollback.rollback_state_class,
        "disable_rollback rollback_state_class",
    )?;
    Ok(())
}

fn validate_revocation(
    revocation: &RevocationPosture,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        revocation.record_kind.as_str(),
        REVOCATION_POSTURE_RECORD_KIND,
        "revocation record_kind",
    )?;
    ensure_token(
        REVOCATION_STATE_CLASSES,
        &revocation.revocation_state_class,
        "revocation revocation_state_class",
    )?;
    ensure_token(
        REVOCATION_PROPAGATION_CLASSES,
        &revocation.propagation_class,
        "revocation propagation_class",
    )?;
    Ok(())
}

fn validate_claim(
    claim: &LifecycleFlowQualificationClaim,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        LIFECYCLE_FLOW_QUALIFICATION_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim claimed_tier")?;
    ensure_token(
        STABILITY_TIERS,
        &claim.effective_tier,
        "claim effective_tier",
    )?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim claim_basis_class",
    )?;
    for reason in &claim.downgrade_reasons {
        ensure_token(
            LIFECYCLE_FLOW_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &DowngradedFlowBanner,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        DOWNGRADED_FLOW_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            LIFECYCLE_FLOW_DOWNGRADE_REASONS,
            reason,
            "banner banner_reason_class",
        )?;
        if !banner.must_display {
            return Err(err("banner_reason_class is set but must_display is false"));
        }
    } else if banner.must_display {
        return Err(err(
            "must_display is true but no banner_reason_class is set",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &LifecycleFlowInspection,
    packet: &StableLifecycleFlowPacket,
) -> Result<(), StableLifecycleFlowValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        LIFECYCLE_FLOW_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.packet_id_ref.as_str(),
        packet.packet_id.as_str(),
        "inspection packet_id_ref",
    )?;
    ensure_eq(
        inspection.effective_tier.as_str(),
        packet.claim.effective_tier.as_str(),
        "inspection effective_tier",
    )?;
    if inspection.downgraded != packet.claim.downgraded {
        return Err(err("inspection downgraded is inconsistent"));
    }
    if inspection.downgraded_banner_required != packet.downgraded_banner.must_display {
        return Err(err("inspection downgraded_banner_required is inconsistent"));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.node_count != packet.resolution.node_count {
        return Err(err("inspection node_count is inconsistent"));
    }
    if inspection.effective_permission_count != packet.permissions.effective_permission_refs.len() {
        return Err(err("inspection effective_permission_count is inconsistent"));
    }
    if inspection.expanded_permission_count != packet.permissions.expanded_permission_refs.len() {
        return Err(err("inspection expanded_permission_count is inconsistent"));
    }
    if inspection.reconsent_satisfied != packet.reconsent.satisfied() {
        return Err(err("inspection reconsent_satisfied is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableLifecycleFlowValidationError {
    StableLifecycleFlowValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), StableLifecycleFlowValidationError>
where
    T: PartialEq + fmt::Display,
{
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_eq_u32(
    left: u32,
    right: u32,
    field: &str,
) -> Result<(), StableLifecycleFlowValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableLifecycleFlowValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableLifecycleFlowValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
