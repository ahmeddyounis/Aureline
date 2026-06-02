//! Harden the extension manifest, permission display, lifecycle labels, and
//! compatibility-range truth for the stable line.
//!
//! The manifest baseline (see [`crate::manifest_baseline`]) owns the first
//! inspectable manifest record and the declared-vs-effective permission diff for
//! a *single* extension. This module owns the layer above it — the **stable,
//! hardened manifest truth** a claimed stable ecosystem row carries once
//! dependency resolution is taken into account, and the **stability
//! qualification** that truth is allowed to claim.
//!
//! A hardened manifest must declare, machine-readably:
//!
//! - its **hard dependencies** and **optional integrations** (each with a
//!   dependency class, a resolution state, a lifecycle/deprecation marker, and
//!   the permissions it contributes),
//! - its compatible **API range** and **runtime range** (so a range conflict is
//!   visible *before* install, upgrade, or mirror promotion), and
//! - the **permission implications** of resolving those dependencies, so the
//!   **effective** permission set after dependency resolution is fully surfaced
//!   and authority is never widened implicitly.
//!
//! The central rule mirrors the rest of the stable line: a **stable** manifest
//! claim may never be implied from a catalog row alone. A row that renders a
//! `stable` manifest badge must pin the published manifest schema version, be
//! resolution-backed (not catalog-asserted), keep its publisher trust tier out
//! of quarantine, stay on an installable lifecycle, resolve every hard
//! dependency, satisfy its API and runtime ranges, surface every transitive
//! permission machine-readably (no implicit authority widening), keep the stored
//! effective-permission diff consistent with the resolved truth, and be fully
//! attributed. When any of those fails, the visible tier is **automatically
//! narrowed below Stable** (to `beta` or `preview`, or `withdrawn` when the row
//! cannot be installed at all) rather than left asserting a manifest readiness
//! the resolver cannot back.
//!
//! Two security guardrails are encoded so they cannot be papered over:
//!
//! - **No implicit authority widening.** When a hard dependency contributes
//!   permissions but does not declare them machine-readably
//!   (`permission_implications_machine_readable == false`), the effective
//!   authority widened without being surfaced; the claim narrows below Stable and
//!   a manifest-review banner is raised. The effective permission set is
//!   re-derived from the declared top-level set plus the resolved hard-dependency
//!   contributions at validation time, so a stored packet can never hide a
//!   transitive permission.
//! - **Range conflicts are visible before install.** An unsatisfied API or
//!   runtime range (`below_minimum` / `above_maximum` / `range_conflict`) is
//!   carried on the packet and narrows the claim; a `below_minimum` range or an
//!   unresolved/removed hard dependency withdraws the row outright.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_manifest_hardening.schema.json`](../../../../schemas/extensions/stable_manifest_hardening.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable manifest-hardening record.
pub const STABLE_MANIFEST_HARDENING_SCHEMA_VERSION: u32 = 1;

/// The published, stable manifest schema version. A `stable` manifest claim must
/// pin exactly this version; any other version narrows below Stable.
pub const STABLE_MANIFEST_HARDENING_PUBLISHED_MANIFEST_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_MANIFEST_HARDENING_SCHEMA_REF: &str =
    "schemas/extensions/stable_manifest_hardening.schema.json";

/// Record-kind tag for [`StableManifestHardeningPacket`].
pub const STABLE_MANIFEST_HARDENING_PACKET_RECORD_KIND: &str = "stable_manifest_hardening_packet";

/// Record-kind tag for [`ManifestHardeningIdentity`].
pub const MANIFEST_HARDENING_IDENTITY_RECORD_KIND: &str = "stable_manifest_hardening_identity";

/// Record-kind tag for [`ManifestCompatibilityRange`].
pub const MANIFEST_COMPATIBILITY_RANGE_RECORD_KIND: &str = "stable_manifest_compatibility_range";

/// Record-kind tag for [`ManifestDependencyEdge`].
pub const MANIFEST_DEPENDENCY_EDGE_RECORD_KIND: &str = "stable_manifest_dependency_edge";

/// Record-kind tag for [`EffectivePermissionResolution`].
pub const EFFECTIVE_PERMISSION_RESOLUTION_RECORD_KIND: &str =
    "stable_effective_permission_resolution";

/// Record-kind tag for [`ManifestLifecycleLabel`].
pub const MANIFEST_LIFECYCLE_LABEL_RECORD_KIND: &str = "stable_manifest_lifecycle_label";

/// Record-kind tag for [`ManifestHardeningQualificationClaim`].
pub const MANIFEST_HARDENING_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_manifest_hardening_qualification_claim";

/// Record-kind tag for [`DowngradedManifestBanner`].
pub const DOWNGRADED_MANIFEST_BANNER_RECORD_KIND: &str = "stable_downgraded_manifest_banner";

/// Record-kind tag for [`StableManifestHardeningInspection`].
pub const STABLE_MANIFEST_HARDENING_INSPECTION_RECORD_KIND: &str =
    "stable_manifest_hardening_inspection";

/// Record-kind tag for [`StableManifestHardeningSupportExport`].
pub const STABLE_MANIFEST_HARDENING_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_manifest_hardening_support_export";

/// Closed publisher-trust-tier vocabulary.
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

/// Lifecycle states a stable manifest claim may keep (installable / runnable).
pub const INSTALLABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed dependency-class vocabulary. A `hard_dependency` folds its permissions
/// into the effective set; an `optional_integration` is surfaced separately.
pub const DEPENDENCY_CLASSES: &[&str] = &["hard_dependency", "optional_integration"];

/// Closed dependency-resolution-state vocabulary.
pub const DEPENDENCY_RESOLUTION_STATE_CLASSES: &[&str] = &[
    "resolved",
    "unresolved_missing",
    "version_conflict",
    "optional_absent",
];

/// Closed deprecation-marker vocabulary carried on every dependency and on the
/// manifest's own lifecycle label.
pub const DEPRECATION_CLASSES: &[&str] = &["active", "deprecated", "removal_scheduled", "removed"];

/// Closed range-resolution vocabulary shared by the API range and runtime range.
/// `satisfied` is the only state a stable claim may keep.
pub const RANGE_RESOLUTION_CLASSES: &[&str] = &[
    "satisfied",
    "below_minimum",
    "above_maximum",
    "range_conflict",
];

/// Closed capability-class vocabulary every permission entry carries so the
/// machine-readable permission implication is named, not free text.
pub const PERMISSION_CAPABILITY_CLASSES: &[&str] = &[
    "filesystem_read",
    "filesystem_write",
    "network_access",
    "process_exec",
    "secret_access",
    "ui_contribution",
    "telemetry",
    "workspace_state",
    "passive_metadata",
];

/// Closed permission-source vocabulary recorded on every effective-permission
/// diff entry.
pub const PERMISSION_SOURCE_CLASSES: &[&str] =
    &["top_level_declared", "transitive_hard_dependency", "both"];

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* manifest claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["resolution_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_manifest_ready_claim",
    "beta_manifest_partial_claim",
    "preview_manifest_experimental_claim",
    "withdrawn_no_manifest_claim",
];

/// Closed set of reasons that narrow a stable manifest claim below Stable.
pub const MANIFEST_HARDENING_DOWNGRADE_REASONS: &[&str] = &[
    "manifest_schema_version_mismatch",
    "catalog_only_trust_not_resolution_backed",
    "trust_tier_quarantined",
    "lifecycle_not_installable",
    "runtime_range_below_min_unsupported",
    "runtime_range_above_max_unverified",
    "api_range_below_min_unsupported",
    "api_range_above_max_unverified",
    "unresolved_hard_dependency",
    "dependency_version_conflict",
    "dependency_removed",
    "dependency_deprecated",
    "dependency_removal_scheduled",
    "transitive_permission_not_machine_readable",
    "effective_permission_diff_inconsistent",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the row cannot be installed).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_installable",
    "unresolved_hard_dependency",
    "dependency_removed",
    "runtime_range_below_min_unsupported",
    "api_range_below_min_unsupported",
];

/// Reasons that narrow to `preview` (a structural/security shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "manifest_schema_version_mismatch",
    "catalog_only_trust_not_resolution_backed",
    "trust_tier_quarantined",
    "dependency_version_conflict",
    "transitive_permission_not_machine_readable",
    "effective_permission_diff_inconsistent",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "runtime_range_above_max_unverified",
    "api_range_above_max_unverified",
    "dependency_deprecated",
    "dependency_removal_scheduled",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_MANIFEST_HARDENING_CONSUMER_SURFACES: &[&str] = &[
    "install_review",
    "manifest_inspector",
    "upgrade_review",
    "mirror_promotion",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable manifest-hardening packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableManifestHardeningInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: ManifestHardeningIdentityInput,
    /// Compatibility-range input.
    pub compatibility_range: ManifestCompatibilityRangeInput,
    /// Top-level requested permissions.
    #[serde(default)]
    pub declared_permissions: Vec<ManifestPermissionEntryInput>,
    /// Dependency edges (hard dependencies and optional integrations).
    #[serde(default)]
    pub dependencies: Vec<ManifestDependencyEdgeInput>,
    /// Manifest lifecycle / deprecation label input.
    pub lifecycle_label: ManifestLifecycleLabelInput,
    /// Stability qualification claim input.
    pub claim: ManifestHardeningQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`ManifestHardeningIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestHardeningIdentityInput {
    /// Ref to the manifest baseline record this packet hardens.
    pub manifest_baseline_ref: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version declared by the manifest baseline.
    pub extension_version: String,
    /// Ref to the source package the manifest came from.
    pub source_package_ref: String,
    /// Manifest schema version this row pins.
    pub manifest_schema_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`ManifestCompatibilityRange`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestCompatibilityRangeInput {
    /// Declared minimum compatible host API version.
    pub declared_api_min_ref: String,
    /// Declared maximum compatible host API version.
    pub declared_api_max_ref: String,
    /// Declared minimum compatible runtime ABI version.
    pub declared_runtime_min_ref: String,
    /// Declared maximum compatible runtime ABI version.
    pub declared_runtime_max_ref: String,
    /// Resolution of the declared API range against the target host.
    pub api_range_resolution_class: String,
    /// Resolution of the declared runtime range against the target host.
    pub runtime_range_resolution_class: String,
    /// Host API version the ranges were resolved against.
    pub resolved_against_api_ref: String,
    /// Host runtime version the ranges were resolved against.
    pub resolved_against_runtime_ref: String,
}

/// Input for one top-level [`ManifestPermissionEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestPermissionEntryInput {
    /// Opaque permission ref.
    pub permission_ref: String,
    /// Capability class the permission implies.
    pub capability_class: String,
}

/// Input for one [`ManifestDependencyEdge`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDependencyEdgeInput {
    /// Stable dependency edge id.
    pub dependency_id: String,
    /// Dependency class: hard dependency vs optional integration.
    pub dependency_class: String,
    /// Opaque ref to the target extension.
    pub target_extension_ref: String,
    /// Opaque ref to the declared version range for the target.
    pub target_version_range_ref: String,
    /// Resolution state for the dependency.
    pub resolution_state_class: String,
    /// Lifecycle state of the resolved dependency.
    pub lifecycle_state_class: String,
    /// Deprecation marker for the dependency.
    pub deprecation_class: String,
    /// Whether the dependency declares its permission implications
    /// machine-readably. A hard dependency that contributes permissions while
    /// this is false widens authority implicitly.
    pub permission_implications_machine_readable: bool,
    /// Permissions the dependency contributes to the consuming extension.
    #[serde(default)]
    pub contributed_permissions: Vec<ManifestPermissionEntryInput>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ManifestLifecycleLabel`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestLifecycleLabelInput {
    /// Lifecycle state of the extension.
    pub lifecycle_state_class: String,
    /// Deprecation marker for the extension.
    pub deprecation_class: String,
    /// Opaque ref to the support-window metadata, when one is declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_window_ref: Option<String>,
    /// Opaque ref to a replacement extension, when one is declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ManifestHardeningQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestHardeningQualificationClaimInput {
    /// Manifest tier claimed by the row.
    pub claimed_tier: String,
    /// Claim basis: resolution-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Identity shared across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestHardeningIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the manifest baseline record this packet hardens.
    pub manifest_baseline_ref: String,
    /// Opaque extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Source package the manifest came from.
    pub source_package_ref: String,
    /// Manifest schema version this row pins.
    pub manifest_schema_version: u32,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl ManifestHardeningIdentity {
    /// Returns true when the row pins the published stable manifest schema version.
    pub fn manifest_version_current(&self) -> bool {
        self.manifest_schema_version == STABLE_MANIFEST_HARDENING_PUBLISHED_MANIFEST_VERSION
    }

    /// Returns true when the lifecycle is installable.
    pub fn lifecycle_installable(&self) -> bool {
        INSTALLABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Declared API and runtime compatibility range plus its resolution state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestCompatibilityRange {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Declared minimum compatible host API version.
    pub declared_api_min_ref: String,
    /// Declared maximum compatible host API version.
    pub declared_api_max_ref: String,
    /// Declared minimum compatible runtime ABI version.
    pub declared_runtime_min_ref: String,
    /// Declared maximum compatible runtime ABI version.
    pub declared_runtime_max_ref: String,
    /// Resolution of the declared API range against the target host.
    pub api_range_resolution_class: String,
    /// Resolution of the declared runtime range against the target host.
    pub runtime_range_resolution_class: String,
    /// Host API version the ranges were resolved against.
    pub resolved_against_api_ref: String,
    /// Host runtime version the ranges were resolved against.
    pub resolved_against_runtime_ref: String,
}

impl ManifestCompatibilityRange {
    /// Returns true when both the API and runtime ranges are satisfied.
    pub fn ranges_satisfied(&self) -> bool {
        self.api_range_resolution_class == "satisfied"
            && self.runtime_range_resolution_class == "satisfied"
    }
}

/// One top-level requested permission entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestPermissionEntry {
    /// Opaque permission ref.
    pub permission_ref: String,
    /// Capability class the permission implies.
    pub capability_class: String,
}

/// One dependency edge. A `hard_dependency` folds its contributed permissions
/// into the effective set; an `optional_integration` is surfaced separately.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDependencyEdge {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable dependency edge id.
    pub dependency_id: String,
    /// Dependency class.
    pub dependency_class: String,
    /// Opaque ref to the target extension.
    pub target_extension_ref: String,
    /// Opaque ref to the declared version range for the target.
    pub target_version_range_ref: String,
    /// Resolution state for the dependency.
    pub resolution_state_class: String,
    /// Lifecycle state of the resolved dependency.
    pub lifecycle_state_class: String,
    /// Deprecation marker for the dependency.
    pub deprecation_class: String,
    /// Whether the dependency declares its permission implications
    /// machine-readably.
    pub permission_implications_machine_readable: bool,
    /// Permissions the dependency contributes.
    pub contributed_permissions: Vec<ManifestPermissionEntry>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl ManifestDependencyEdge {
    /// Returns true when this edge is a hard dependency.
    pub fn is_hard(&self) -> bool {
        self.dependency_class == "hard_dependency"
    }

    /// Returns true when a hard dependency is unresolved.
    pub fn hard_unresolved(&self) -> bool {
        self.is_hard() && matches!(self.resolution_state_class.as_str(), "unresolved_missing")
    }

    /// Returns true when a hard dependency contributes permissions without
    /// declaring them machine-readably (implicit authority widening).
    pub fn widens_authority_implicitly(&self) -> bool {
        self.is_hard()
            && !self.contributed_permissions.is_empty()
            && !self.permission_implications_machine_readable
    }
}

/// One effective-permission diff entry after dependency resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionDiffEntry {
    /// Opaque permission ref.
    pub permission_ref: String,
    /// Capability class the permission implies.
    pub capability_class: String,
    /// Where the effective permission came from.
    pub source_class: String,
    /// Refs to the hard dependencies that contributed this permission, if any.
    pub contributed_by_refs: Vec<String>,
}

/// Effective permission set after dependency resolution. The top-level declared
/// set, the transitive set folded in from resolved hard dependencies, and the
/// per-permission diff are all surfaced; the optional-integration contributions
/// are surfaced separately and never silently folded into the effective set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionResolution {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Top-level requested permission refs.
    pub declared_permission_refs: Vec<String>,
    /// Transitive permission refs folded in from resolved hard dependencies.
    pub transitive_permission_refs: Vec<String>,
    /// Effective permission refs (declared ∪ resolved hard-dependency transitive).
    pub effective_permission_refs: Vec<String>,
    /// Permission refs an optional integration would add only when enabled.
    pub optional_integration_permission_refs: Vec<String>,
    /// Per-permission diff entries.
    pub diff_entries: Vec<EffectivePermissionDiffEntry>,
    /// True when a hard dependency contributes permissions without declaring
    /// them machine-readably.
    pub implicit_widening_present: bool,
}

impl EffectivePermissionResolution {
    /// Returns the number of effective permissions added purely by transitive
    /// hard-dependency resolution (not present in the top-level declared set).
    pub fn transitive_only_count(&self) -> usize {
        self.diff_entries
            .iter()
            .filter(|d| d.source_class == "transitive_hard_dependency")
            .count()
    }
}

/// Manifest lifecycle / deprecation label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestLifecycleLabel {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Lifecycle state of the extension.
    pub lifecycle_state_class: String,
    /// Deprecation marker for the extension.
    pub deprecation_class: String,
    /// Opaque ref to the support-window metadata, when one is declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_window_ref: Option<String>,
    /// Opaque ref to a replacement extension, when one is declared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestHardeningQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Manifest tier claimed by the row.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim the effective tier is allowed to imply.
    pub support_claim_class: String,
    /// Claim basis: resolution-backed vs catalog-asserted only.
    pub claim_basis_class: String,
    /// True when the claimed tier was narrowed below Stable.
    pub downgraded: bool,
    /// Reasons that narrowed the claim.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Downgraded-manifest banner requirement. Raised whenever a reviewer must see a
/// hardening shortfall before install, upgrade, or mirror promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedManifestBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-manifest banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableManifestHardeningInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective manifest tier.
    pub effective_tier: String,
    /// True when the claim is a stable manifest claim.
    pub stable_claim: bool,
    /// True when the row pins the published manifest schema version.
    pub manifest_version_current: bool,
    /// True when both the API and runtime ranges are satisfied.
    pub ranges_satisfied: bool,
    /// API range resolution.
    pub api_range_resolution_class: String,
    /// Runtime range resolution.
    pub runtime_range_resolution_class: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is installable.
    pub lifecycle_installable: bool,
    /// Manifest deprecation marker.
    pub deprecation_class: String,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-manifest banner is required.
    pub downgraded_manifest_banner_required: bool,
    /// True when every dependency and identity is fully attributed.
    pub attribution_complete: bool,
    /// Always false; surfaced so a reviewer can see implicit widening is forbidden.
    pub implicit_authority_widening_present: bool,
    /// Number of declared (top-level) permissions.
    pub declared_permission_count: usize,
    /// Number of transitive permissions folded in from hard dependencies.
    pub transitive_permission_count: usize,
    /// Number of effective permissions after resolution.
    pub effective_permission_count: usize,
    /// Number of permissions added purely by transitive resolution.
    pub transitive_only_permission_count: usize,
    /// Number of optional integrations.
    pub optional_integration_count: usize,
    /// Number of hard dependencies.
    pub hard_dependency_count: usize,
    /// Number of unresolved hard dependencies.
    pub unresolved_hard_dependency_count: usize,
    /// Number of deprecated/removal-scheduled/removed dependencies.
    pub deprecated_dependency_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable manifest-hardening packet consumed by install review, the manifest
/// inspector, upgrade review, mirror promotion, diagnostics, support export,
/// docs/help, and release packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableManifestHardeningPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: ManifestHardeningIdentity,
    /// Compatibility range.
    pub compatibility_range: ManifestCompatibilityRange,
    /// Top-level requested permissions.
    pub declared_permissions: Vec<ManifestPermissionEntry>,
    /// Dependency edges.
    pub dependencies: Vec<ManifestDependencyEdge>,
    /// Effective permission resolution after dependency resolution.
    pub effective_permissions: EffectivePermissionResolution,
    /// Manifest lifecycle / deprecation label.
    pub lifecycle_label: ManifestLifecycleLabel,
    /// Stability qualification claim after the posture is applied.
    pub claim: ManifestHardeningQualificationClaim,
    /// Downgraded-manifest banner requirement.
    pub downgraded_manifest_banner: DowngradedManifestBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so dependency resolution can never widen authority implicitly.
    pub allows_implicit_authority_widening: bool,
    /// False so catalog-only trust cannot back a stable manifest claim.
    pub allows_catalog_only_trust: bool,
    /// False so a runtime/API range conflict can never be hidden before install.
    pub allows_hidden_range_conflict: bool,
    /// False so a transitive permission can never go unsurfaced.
    pub allows_unsurfaced_transitive_permission: bool,
    /// Inspection row.
    pub inspection: StableManifestHardeningInspection,
}

impl StableManifestHardeningPacket {
    /// Builds a stable manifest-hardening packet from input, resolving the
    /// effective permission set and applying the manifest posture to the claimed
    /// tier so any required downgrade below Stable is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableManifestHardeningValidationError`] when the input violates
    /// an identity, range, dependency, permission, or claim invariant.
    pub fn from_input(
        input: StableManifestHardeningInput,
    ) -> Result<Self, StableManifestHardeningValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let compatibility_range = range_record(&input.compatibility_range);
        let declared_permissions: Vec<ManifestPermissionEntry> = input
            .declared_permissions
            .iter()
            .map(permission_record)
            .collect();
        let dependencies: Vec<ManifestDependencyEdge> =
            input.dependencies.iter().map(dependency_record).collect();
        let effective_permissions =
            resolve_effective_permissions(&declared_permissions, &dependencies);
        let lifecycle_label = lifecycle_label_record(&input.lifecycle_label);
        let claim = claim_record(
            &input.claim,
            &identity,
            &compatibility_range,
            &dependencies,
            &effective_permissions,
            attribution_is_complete(&identity, &dependencies),
        );
        let downgraded_manifest_banner = banner_record(
            &identity,
            &compatibility_range,
            &dependencies,
            &effective_permissions,
        );
        let inspection = inspection_record(
            &input.packet_id,
            &identity,
            &compatibility_range,
            &declared_permissions,
            &dependencies,
            &effective_permissions,
            &lifecycle_label,
            &claim,
            &downgraded_manifest_banner,
        );

        let packet = Self {
            record_kind: STABLE_MANIFEST_HARDENING_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            compatibility_range,
            declared_permissions,
            dependencies,
            effective_permissions,
            lifecycle_label,
            claim,
            downgraded_manifest_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_MANIFEST_HARDENING_SCHEMA_REF.to_string()],
            allows_implicit_authority_widening: false,
            allows_catalog_only_trust: false,
            allows_hidden_range_conflict: false,
            allows_unsurfaced_transitive_permission: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable manifest-hardening invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableManifestHardeningValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), StableManifestHardeningValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_MANIFEST_HARDENING_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_range(&self.compatibility_range)?;
        for entry in &self.declared_permissions {
            validate_permission(entry, "declared_permission")?;
        }
        for dep in &self.dependencies {
            validate_dependency(dep)?;
        }
        validate_effective_permissions(&self.effective_permissions)?;
        validate_lifecycle_label(&self.lifecycle_label)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_manifest_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_MANIFEST_HARDENING_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_MANIFEST_HARDENING_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No implicit widening, catalog-only trust, hidden range conflict, or
        // unsurfaced transitive permission may ride a published stable manifest row.
        if self.allows_implicit_authority_widening
            || self.allows_catalog_only_trust
            || self.allows_hidden_range_conflict
            || self.allows_unsurfaced_transitive_permission
        {
            return Err(err(
                "a stable manifest packet must not allow implicit authority widening, catalog-only trust, hidden range conflict, or unsurfaced transitive permission",
            ));
        }

        // The effective permission set, sources, and implicit-widening flag are
        // re-derived from the declared set and the resolved hard dependencies, so
        // a stored packet can never hide a transitive permission or its origin.
        let derived_resolution =
            resolve_effective_permissions(&self.declared_permissions, &self.dependencies);
        if derived_resolution != self.effective_permissions {
            return Err(err(
                "stored effective-permission resolution does not match the resolver-derived truth",
            ));
        }

        // Stable-claim binding: a stable effective tier must pin the published
        // manifest version, be resolution-backed, keep the trust tier out of
        // quarantine, stay installable, satisfy its ranges, resolve every hard
        // dependency, surface every transitive permission, and be fully attributed.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.manifest_version_current() {
                return Err(err(
                    "stable effective tier must pin the published manifest schema version",
                ));
            }
            if self.claim.claim_basis_class != "resolution_backed" {
                return Err(err(
                    "stable effective tier must be resolution-backed, not catalog-asserted",
                ));
            }
            if self.identity.publisher_trust_tier_class == "quarantined" {
                return Err(err(
                    "stable effective tier must not carry a quarantined trust tier",
                ));
            }
            if !self.identity.lifecycle_installable() {
                return Err(err(
                    "stable effective tier must stay on an installable lifecycle",
                ));
            }
            if !self.compatibility_range.ranges_satisfied() {
                return Err(err(
                    "stable effective tier must satisfy its API and runtime ranges",
                ));
            }
            if self.dependencies.iter().any(|d| d.hard_unresolved()) {
                return Err(err(
                    "stable effective tier must not carry an unresolved hard dependency",
                ));
            }
            if self
                .dependencies
                .iter()
                .any(|d| d.is_hard() && d.resolution_state_class == "version_conflict")
            {
                return Err(err(
                    "stable effective tier must not carry a hard-dependency version conflict",
                ));
            }
            if self.effective_permissions.implicit_widening_present {
                return Err(err(
                    "stable effective tier must not widen authority implicitly",
                ));
            }
            if self
                .dependencies
                .iter()
                .any(|d| matches!(d.deprecation_class.as_str(), "removed"))
            {
                return Err(err(
                    "stable effective tier must not depend on a removed dependency",
                ));
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

        // Downgrade truth: a downgraded claim carries at least one reason and
        // never keeps a stable effective tier.
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
        let derived = derive_effective_tier(
            &self.claim.claimed_tier,
            &self.claim.claim_basis_class,
            &self.identity,
            &self.compatibility_range,
            &self.dependencies,
            &self.effective_permissions,
            self.attribution_complete(),
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

        // Banner truth: a banner must be raised exactly when the manifest posture
        // requires a pre-install warning, and never silently suppressed.
        let banner_required = manifest_requires_warning(
            &self.identity,
            &self.compatibility_range,
            &self.dependencies,
            &self.effective_permissions,
        );
        if self.downgraded_manifest_banner.must_display != banner_required {
            return Err(err(
                "downgraded-manifest banner must_display does not match the manifest posture",
            ));
        }

        validate_inspection(&self.inspection, self)?;
        Ok(())
    }

    /// Returns true when no stable claim is implied from catalog-only trust.
    pub fn no_catalog_only_stable_claim(&self) -> bool {
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            return self.claim.claim_basis_class == "resolution_backed";
        }
        true
    }

    /// Returns true when identity and every dependency are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(&self.identity, &self.dependencies)
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableManifestHardeningProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Extension identity.
    pub extension_identity_ref: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable manifest claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-manifest banner is required.
    pub downgraded_manifest_banner_required: bool,
    /// Number of effective permissions after resolution.
    pub effective_permission_count: usize,
    /// Number of permissions added purely by transitive resolution.
    pub transitive_only_permission_count: usize,
    /// Number of hard dependencies.
    pub hard_dependency_count: usize,
}

impl From<StableManifestHardeningPacket> for StableManifestHardeningProjection {
    fn from(packet: StableManifestHardeningPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            extension_identity_ref: packet.identity.extension_identity_ref,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_manifest_banner_required: packet.downgraded_manifest_banner.must_display,
            effective_permission_count: packet.inspection.effective_permission_count,
            transitive_only_permission_count: packet.inspection.transitive_only_permission_count,
            hard_dependency_count: packet.inspection.hard_dependency_count,
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableManifestHardeningError`] when the payload fails to parse or
/// violates the stable manifest-hardening invariants.
pub fn project_stable_manifest_hardening(
    payload: &str,
) -> Result<StableManifestHardeningProjection, StableManifestHardeningError> {
    let packet: StableManifestHardeningPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableManifestHardeningProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support/partner export row that quotes the same closed tokens
/// as the packet without leaking raw manifest, permission, or version bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableManifestHardeningSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Extension identity.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Source package.
    pub source_package_ref: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Manifest deprecation marker.
    pub deprecation_class: String,
    /// API range resolution.
    pub api_range_resolution_class: String,
    /// Runtime range resolution.
    pub runtime_range_resolution_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-manifest banner is required.
    pub downgraded_manifest_banner_required: bool,
    /// True when the effective tier blocks install (withdrawn).
    pub blocks_install: bool,
    /// True when authority widened without machine-readable surfacing.
    pub implicit_authority_widening_present: bool,
    /// Number of declared (top-level) permissions.
    pub declared_permission_count: usize,
    /// Number of transitive permissions folded in from hard dependencies.
    pub transitive_permission_count: usize,
    /// Number of effective permissions after resolution.
    pub effective_permission_count: usize,
    /// Number of optional integrations.
    pub optional_integration_count: usize,
    /// Number of hard dependencies.
    pub hard_dependency_count: usize,
    /// Number of unresolved hard dependencies.
    pub unresolved_hard_dependency_count: usize,
    /// Export-safe summary suitable for support/partner consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support/partner export row.
pub fn project_stable_manifest_hardening_support_export(
    packet: &StableManifestHardeningPacket,
) -> StableManifestHardeningSupportExport {
    let blocks_install = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Trust={} lifecycle={} deprecation={}. Ranges api={} runtime={}. Tier claimed={} effective={} (downgraded={}). Banner required={}. Permissions: declared={} transitive={} effective={}. Dependencies: hard={} unresolved={} optional={}. Implicit widening={}.",
        packet.claim.summary_label,
        packet.identity.publisher_trust_tier_class,
        packet.identity.lifecycle_state_class,
        packet.lifecycle_label.deprecation_class,
        packet.compatibility_range.api_range_resolution_class,
        packet.compatibility_range.runtime_range_resolution_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_manifest_banner.must_display,
        packet.inspection.declared_permission_count,
        packet.inspection.transitive_permission_count,
        packet.inspection.effective_permission_count,
        packet.inspection.hard_dependency_count,
        packet.inspection.unresolved_hard_dependency_count,
        packet.inspection.optional_integration_count,
        packet.effective_permissions.implicit_widening_present,
    );

    StableManifestHardeningSupportExport {
        record_kind: STABLE_MANIFEST_HARDENING_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        export_id: format!(
            "stable_manifest_hardening_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        extension_identity_ref: packet.identity.extension_identity_ref.clone(),
        extension_version: packet.identity.extension_version.clone(),
        source_package_ref: packet.identity.source_package_ref.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        deprecation_class: packet.lifecycle_label.deprecation_class.clone(),
        api_range_resolution_class: packet
            .compatibility_range
            .api_range_resolution_class
            .clone(),
        runtime_range_resolution_class: packet
            .compatibility_range
            .runtime_range_resolution_class
            .clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_manifest_banner_required: packet.downgraded_manifest_banner.must_display,
        blocks_install,
        implicit_authority_widening_present: packet.effective_permissions.implicit_widening_present,
        declared_permission_count: packet.inspection.declared_permission_count,
        transitive_permission_count: packet.inspection.transitive_permission_count,
        effective_permission_count: packet.inspection.effective_permission_count,
        optional_integration_count: packet.inspection.optional_integration_count,
        hard_dependency_count: packet.inspection.hard_dependency_count,
        unresolved_hard_dependency_count: packet.inspection.unresolved_hard_dependency_count,
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable manifest-hardening operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableManifestHardeningError {
    /// Validation failed.
    Validation(StableManifestHardeningValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableManifestHardeningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableManifestHardeningError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable manifest-hardening packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableManifestHardeningValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableManifestHardeningValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableManifestHardeningValidationError {}

impl StableManifestHardeningValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableManifestHardeningError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableManifestHardeningValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableManifestHardeningValidationError> for StableManifestHardeningError {
    fn from(err: StableManifestHardeningValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-permission resolution
// ---------------------------------------------------------------------------

/// Resolves the effective permission set after dependency resolution. Top-level
/// declared permissions plus the contributions of *resolved* hard dependencies
/// form the effective set; optional-integration contributions are surfaced
/// separately and never silently folded in.
fn resolve_effective_permissions(
    declared: &[ManifestPermissionEntry],
    dependencies: &[ManifestDependencyEdge],
) -> EffectivePermissionResolution {
    use std::collections::BTreeMap;

    let declared_refs: BTreeSet<String> =
        declared.iter().map(|p| p.permission_ref.clone()).collect();

    // Capability classes per permission ref, plus contributing hard-dependency refs.
    let mut capability_for: BTreeMap<String, String> = BTreeMap::new();
    let mut contributed_by: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for p in declared {
        capability_for
            .entry(p.permission_ref.clone())
            .or_insert_with(|| p.capability_class.clone());
    }

    let mut transitive_refs: BTreeSet<String> = BTreeSet::new();
    let mut optional_refs: BTreeSet<String> = BTreeSet::new();
    let mut implicit_widening_present = false;

    for dep in dependencies {
        if dep.is_hard() {
            // A hard dependency that contributes permissions but does not declare
            // them machine-readably widens authority implicitly.
            if dep.widens_authority_implicitly() {
                implicit_widening_present = true;
            }
            // Only *resolved* hard dependencies fold their permissions into the
            // effective set; an unresolved/conflicting dependency contributes
            // nothing effective (its absence is a downgrade reason elsewhere).
            if dep.resolution_state_class == "resolved" {
                for p in &dep.contributed_permissions {
                    transitive_refs.insert(p.permission_ref.clone());
                    capability_for
                        .entry(p.permission_ref.clone())
                        .or_insert_with(|| p.capability_class.clone());
                    contributed_by
                        .entry(p.permission_ref.clone())
                        .or_default()
                        .insert(dep.dependency_id.clone());
                }
            }
        } else {
            for p in &dep.contributed_permissions {
                optional_refs.insert(p.permission_ref.clone());
                capability_for
                    .entry(p.permission_ref.clone())
                    .or_insert_with(|| p.capability_class.clone());
            }
        }
    }

    let effective_refs: BTreeSet<String> = declared_refs.union(&transitive_refs).cloned().collect();

    let mut diff_entries: Vec<EffectivePermissionDiffEntry> = Vec::new();
    for permission_ref in &effective_refs {
        let in_declared = declared_refs.contains(permission_ref);
        let in_transitive = transitive_refs.contains(permission_ref);
        let source_class = match (in_declared, in_transitive) {
            (true, true) => "both",
            (true, false) => "top_level_declared",
            (false, true) => "transitive_hard_dependency",
            (false, false) => "top_level_declared",
        };
        let contributed_by_refs: Vec<String> = contributed_by
            .get(permission_ref)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();
        diff_entries.push(EffectivePermissionDiffEntry {
            permission_ref: permission_ref.clone(),
            capability_class: capability_for
                .get(permission_ref)
                .cloned()
                .unwrap_or_else(|| "passive_metadata".to_string()),
            source_class: source_class.to_string(),
            contributed_by_refs,
        });
    }

    // Optional-integration permissions that are not already effective.
    let optional_only: Vec<String> = optional_refs
        .iter()
        .filter(|r| !effective_refs.contains(*r))
        .cloned()
        .collect();

    EffectivePermissionResolution {
        record_kind: EFFECTIVE_PERMISSION_RESOLUTION_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        declared_permission_refs: declared_refs.into_iter().collect(),
        transitive_permission_refs: transitive_refs.into_iter().collect(),
        effective_permission_refs: effective_refs.into_iter().collect(),
        optional_integration_permission_refs: optional_only,
        diff_entries,
        implicit_widening_present,
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Applies the manifest posture to a claimed tier, narrowing automatically below
/// Stable when the resolver can no longer back it.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    identity: &ManifestHardeningIdentity,
    range: &ManifestCompatibilityRange,
    dependencies: &[ManifestDependencyEdge],
    effective: &EffectivePermissionResolution,
    attribution_complete: bool,
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

    let mut reasons: Vec<String> = Vec::new();

    if !identity.manifest_version_current() {
        reasons.push("manifest_schema_version_mismatch".to_string());
    }
    if claim_basis != "resolution_backed" {
        reasons.push("catalog_only_trust_not_resolution_backed".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !identity.lifecycle_installable() {
        reasons.push("lifecycle_not_installable".to_string());
    }
    match range.api_range_resolution_class.as_str() {
        "below_minimum" | "range_conflict" => {
            reasons.push("api_range_below_min_unsupported".to_string())
        }
        "above_maximum" => reasons.push("api_range_above_max_unverified".to_string()),
        _ => {}
    }
    match range.runtime_range_resolution_class.as_str() {
        "below_minimum" | "range_conflict" => {
            reasons.push("runtime_range_below_min_unsupported".to_string())
        }
        "above_maximum" => reasons.push("runtime_range_above_max_unverified".to_string()),
        _ => {}
    }
    if dependencies.iter().any(|d| d.hard_unresolved()) {
        reasons.push("unresolved_hard_dependency".to_string());
    }
    if dependencies
        .iter()
        .any(|d| d.is_hard() && d.resolution_state_class == "version_conflict")
    {
        reasons.push("dependency_version_conflict".to_string());
    }
    if dependencies
        .iter()
        .any(|d| d.deprecation_class == "removed")
    {
        reasons.push("dependency_removed".to_string());
    }
    if dependencies
        .iter()
        .any(|d| d.deprecation_class == "deprecated")
    {
        reasons.push("dependency_deprecated".to_string());
    }
    if dependencies
        .iter()
        .any(|d| d.deprecation_class == "removal_scheduled")
    {
        reasons.push("dependency_removal_scheduled".to_string());
    }
    if effective.implicit_widening_present {
        reasons.push("transitive_permission_not_machine_readable".to_string());
    }
    if !attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();

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
        "stable" => "stable_manifest_ready_claim",
        "beta" => "beta_manifest_partial_claim",
        "preview" => "preview_manifest_experimental_claim",
        "withdrawn" => "withdrawn_no_manifest_claim",
        _ => "preview_manifest_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity and every dependency are fully attributed.
fn attribution_is_complete(
    identity: &ManifestHardeningIdentity,
    dependencies: &[ManifestDependencyEdge],
) -> bool {
    !identity.source_package_ref.trim().is_empty()
        && !identity.extension_identity_ref.trim().is_empty()
        && dependencies.iter().all(|d| {
            !d.dependency_id.trim().is_empty() && !d.target_extension_ref.trim().is_empty()
        })
}

/// Returns true when the manifest posture requires a pre-install warning banner.
fn manifest_requires_warning(
    identity: &ManifestHardeningIdentity,
    range: &ManifestCompatibilityRange,
    dependencies: &[ManifestDependencyEdge],
    effective: &EffectivePermissionResolution,
) -> bool {
    identity.publisher_trust_tier_class == "quarantined"
        || matches!(
            range.api_range_resolution_class.as_str(),
            "below_minimum" | "range_conflict"
        )
        || matches!(
            range.runtime_range_resolution_class.as_str(),
            "below_minimum" | "range_conflict"
        )
        || dependencies.iter().any(|d| d.hard_unresolved())
        || dependencies.iter().any(|d| {
            matches!(
                d.deprecation_class.as_str(),
                "removed" | "removal_scheduled"
            )
        })
        || effective.implicit_widening_present
}

/// Picks the most-severe banner reason for a manifest that requires a warning.
fn banner_reason_for(
    identity: &ManifestHardeningIdentity,
    range: &ManifestCompatibilityRange,
    dependencies: &[ManifestDependencyEdge],
    effective: &EffectivePermissionResolution,
) -> Option<String> {
    if effective.implicit_widening_present {
        return Some("transitive_permission_not_machine_readable".to_string());
    }
    if dependencies.iter().any(|d| d.hard_unresolved()) {
        return Some("unresolved_hard_dependency".to_string());
    }
    if dependencies
        .iter()
        .any(|d| d.deprecation_class == "removed")
    {
        return Some("dependency_removed".to_string());
    }
    if range.runtime_range_resolution_class == "range_conflict"
        || range.runtime_range_resolution_class == "below_minimum"
    {
        return Some("runtime_range_below_min_unsupported".to_string());
    }
    if range.api_range_resolution_class == "range_conflict"
        || range.api_range_resolution_class == "below_minimum"
    {
        return Some("api_range_below_min_unsupported".to_string());
    }
    if identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    if dependencies
        .iter()
        .any(|d| d.deprecation_class == "removal_scheduled")
    {
        return Some("dependency_removal_scheduled".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &ManifestHardeningIdentityInput) -> ManifestHardeningIdentity {
    ManifestHardeningIdentity {
        record_kind: MANIFEST_HARDENING_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        manifest_baseline_ref: input.manifest_baseline_ref.clone(),
        extension_identity_ref: input.extension_identity_ref.clone(),
        extension_version: input.extension_version.clone(),
        source_package_ref: input.source_package_ref.clone(),
        manifest_schema_version: input.manifest_schema_version,
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn range_record(input: &ManifestCompatibilityRangeInput) -> ManifestCompatibilityRange {
    ManifestCompatibilityRange {
        record_kind: MANIFEST_COMPATIBILITY_RANGE_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        declared_api_min_ref: input.declared_api_min_ref.clone(),
        declared_api_max_ref: input.declared_api_max_ref.clone(),
        declared_runtime_min_ref: input.declared_runtime_min_ref.clone(),
        declared_runtime_max_ref: input.declared_runtime_max_ref.clone(),
        api_range_resolution_class: input.api_range_resolution_class.clone(),
        runtime_range_resolution_class: input.runtime_range_resolution_class.clone(),
        resolved_against_api_ref: input.resolved_against_api_ref.clone(),
        resolved_against_runtime_ref: input.resolved_against_runtime_ref.clone(),
    }
}

fn permission_record(input: &ManifestPermissionEntryInput) -> ManifestPermissionEntry {
    ManifestPermissionEntry {
        permission_ref: input.permission_ref.clone(),
        capability_class: input.capability_class.clone(),
    }
}

fn dependency_record(input: &ManifestDependencyEdgeInput) -> ManifestDependencyEdge {
    ManifestDependencyEdge {
        record_kind: MANIFEST_DEPENDENCY_EDGE_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        dependency_id: input.dependency_id.clone(),
        dependency_class: input.dependency_class.clone(),
        target_extension_ref: input.target_extension_ref.clone(),
        target_version_range_ref: input.target_version_range_ref.clone(),
        resolution_state_class: input.resolution_state_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
        deprecation_class: input.deprecation_class.clone(),
        permission_implications_machine_readable: input.permission_implications_machine_readable,
        contributed_permissions: input
            .contributed_permissions
            .iter()
            .map(permission_record)
            .collect(),
        summary_label: input.summary_label.clone(),
    }
}

fn lifecycle_label_record(input: &ManifestLifecycleLabelInput) -> ManifestLifecycleLabel {
    ManifestLifecycleLabel {
        record_kind: MANIFEST_LIFECYCLE_LABEL_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        lifecycle_state_class: input.lifecycle_state_class.clone(),
        deprecation_class: input.deprecation_class.clone(),
        support_window_ref: input.support_window_ref.clone(),
        replacement_ref: input.replacement_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &ManifestHardeningQualificationClaimInput,
    identity: &ManifestHardeningIdentity,
    range: &ManifestCompatibilityRange,
    dependencies: &[ManifestDependencyEdge],
    effective: &EffectivePermissionResolution,
    attribution_complete: bool,
) -> ManifestHardeningQualificationClaim {
    let derived = derive_effective_tier(
        &input.claimed_tier,
        &input.claim_basis_class,
        identity,
        range,
        dependencies,
        effective,
        attribution_complete,
    );
    ManifestHardeningQualificationClaim {
        record_kind: MANIFEST_HARDENING_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(
    identity: &ManifestHardeningIdentity,
    range: &ManifestCompatibilityRange,
    dependencies: &[ManifestDependencyEdge],
    effective: &EffectivePermissionResolution,
) -> DowngradedManifestBanner {
    let must_display = manifest_requires_warning(identity, range, dependencies, effective);
    let banner_reason_class = if must_display {
        banner_reason_for(identity, range, dependencies, effective)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Manifest requires review before install ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Manifest hardened: ranges satisfied, dependencies resolved, no implicit widening."
            .to_string()
    };
    DowngradedManifestBanner {
        record_kind: DOWNGRADED_MANIFEST_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

#[allow(clippy::too_many_arguments)]
fn inspection_record(
    packet_id: &str,
    identity: &ManifestHardeningIdentity,
    range: &ManifestCompatibilityRange,
    declared: &[ManifestPermissionEntry],
    dependencies: &[ManifestDependencyEdge],
    effective: &EffectivePermissionResolution,
    lifecycle_label: &ManifestLifecycleLabel,
    claim: &ManifestHardeningQualificationClaim,
    banner: &DowngradedManifestBanner,
) -> StableManifestHardeningInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());
    let hard_dependency_count = dependencies.iter().filter(|d| d.is_hard()).count();
    let optional_integration_count = dependencies.iter().filter(|d| !d.is_hard()).count();
    let unresolved_hard_dependency_count =
        dependencies.iter().filter(|d| d.hard_unresolved()).count();
    let deprecated_dependency_count = dependencies
        .iter()
        .filter(|d| {
            matches!(
                d.deprecation_class.as_str(),
                "deprecated" | "removal_scheduled" | "removed"
            )
        })
        .count();

    StableManifestHardeningInspection {
        record_kind: STABLE_MANIFEST_HARDENING_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        manifest_version_current: identity.manifest_version_current(),
        ranges_satisfied: range.ranges_satisfied(),
        api_range_resolution_class: range.api_range_resolution_class.clone(),
        runtime_range_resolution_class: range.runtime_range_resolution_class.clone(),
        trust_tier_class: identity.publisher_trust_tier_class.clone(),
        lifecycle_installable: identity.lifecycle_installable(),
        deprecation_class: lifecycle_label.deprecation_class.clone(),
        downgraded: claim.downgraded,
        downgraded_manifest_banner_required: banner.must_display,
        attribution_complete: attribution_is_complete(identity, dependencies),
        implicit_authority_widening_present: effective.implicit_widening_present,
        declared_permission_count: declared.len(),
        transitive_permission_count: effective.transitive_permission_refs.len(),
        effective_permission_count: effective.effective_permission_refs.len(),
        transitive_only_permission_count: effective.transitive_only_count(),
        optional_integration_count,
        hard_dependency_count,
        unresolved_hard_dependency_count,
        deprecated_dependency_count,
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableManifestHardeningInput,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.manifest_baseline_ref, "identity.manifest_baseline_ref")?;
    if !id.manifest_baseline_ref.starts_with("manifest_baseline:") {
        return Err(err(
            "identity.manifest_baseline_ref must start with 'manifest_baseline:'",
        ));
    }
    ensure_nonempty(
        &id.extension_identity_ref,
        "identity.extension_identity_ref",
    )?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
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

    let r = &input.compatibility_range;
    ensure_nonempty(
        &r.declared_api_min_ref,
        "compatibility_range.declared_api_min_ref",
    )?;
    ensure_nonempty(
        &r.declared_api_max_ref,
        "compatibility_range.declared_api_max_ref",
    )?;
    ensure_nonempty(
        &r.declared_runtime_min_ref,
        "compatibility_range.declared_runtime_min_ref",
    )?;
    ensure_nonempty(
        &r.declared_runtime_max_ref,
        "compatibility_range.declared_runtime_max_ref",
    )?;
    ensure_token(
        RANGE_RESOLUTION_CLASSES,
        &r.api_range_resolution_class,
        "compatibility_range.api_range_resolution_class",
    )?;
    ensure_token(
        RANGE_RESOLUTION_CLASSES,
        &r.runtime_range_resolution_class,
        "compatibility_range.runtime_range_resolution_class",
    )?;
    ensure_nonempty(
        &r.resolved_against_api_ref,
        "compatibility_range.resolved_against_api_ref",
    )?;
    ensure_nonempty(
        &r.resolved_against_runtime_ref,
        "compatibility_range.resolved_against_runtime_ref",
    )?;

    let mut declared_perm_refs = BTreeSet::new();
    for p in &input.declared_permissions {
        ensure_nonempty(&p.permission_ref, "declared_permission.permission_ref")?;
        if !declared_perm_refs.insert(&p.permission_ref) {
            return Err(err(format!(
                "duplicate declared permission_ref: {}",
                p.permission_ref
            )));
        }
        ensure_token(
            PERMISSION_CAPABILITY_CLASSES,
            &p.capability_class,
            "declared_permission.capability_class",
        )?;
    }

    let mut dependency_ids = BTreeSet::new();
    for d in &input.dependencies {
        ensure_nonempty(&d.dependency_id, "dependency.dependency_id")?;
        if !dependency_ids.insert(&d.dependency_id) {
            return Err(err(format!("duplicate dependency_id: {}", d.dependency_id)));
        }
        ensure_token(
            DEPENDENCY_CLASSES,
            &d.dependency_class,
            "dependency.dependency_class",
        )?;
        ensure_nonempty(&d.target_extension_ref, "dependency.target_extension_ref")?;
        ensure_nonempty(
            &d.target_version_range_ref,
            "dependency.target_version_range_ref",
        )?;
        ensure_token(
            DEPENDENCY_RESOLUTION_STATE_CLASSES,
            &d.resolution_state_class,
            "dependency.resolution_state_class",
        )?;
        ensure_token(
            LIFECYCLE_STATE_CLASSES,
            &d.lifecycle_state_class,
            "dependency.lifecycle_state_class",
        )?;
        ensure_token(
            DEPRECATION_CLASSES,
            &d.deprecation_class,
            "dependency.deprecation_class",
        )?;
        // A hard dependency may never carry the optional-only resolution state,
        // and an optional integration may never be a missing hard requirement.
        if d.dependency_class == "hard_dependency" && d.resolution_state_class == "optional_absent"
        {
            return Err(err(
                "a hard_dependency must not use the optional_absent resolution state",
            ));
        }
        if d.dependency_class == "optional_integration"
            && d.resolution_state_class == "unresolved_missing"
        {
            return Err(err(
                "an optional_integration uses optional_absent, not unresolved_missing",
            ));
        }
        let mut contributed_refs = BTreeSet::new();
        for p in &d.contributed_permissions {
            ensure_nonempty(
                &p.permission_ref,
                "dependency.contributed_permission.permission_ref",
            )?;
            if !contributed_refs.insert(&p.permission_ref) {
                return Err(err(format!(
                    "duplicate contributed permission_ref on {}: {}",
                    d.dependency_id, p.permission_ref
                )));
            }
            ensure_token(
                PERMISSION_CAPABILITY_CLASSES,
                &p.capability_class,
                "dependency.contributed_permission.capability_class",
            )?;
        }
    }

    let lc = &input.lifecycle_label;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &lc.lifecycle_state_class,
        "lifecycle_label.lifecycle_state_class",
    )?;
    ensure_token(
        DEPRECATION_CLASSES,
        &lc.deprecation_class,
        "lifecycle_label.deprecation_class",
    )?;
    // The lifecycle label must agree with the identity lifecycle state.
    if lc.lifecycle_state_class != id.lifecycle_state_class {
        return Err(err(
            "lifecycle_label.lifecycle_state_class must match identity.lifecycle_state_class",
        ));
    }

    let claim = &input.claim;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim.claimed_tier")?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim.claim_basis_class",
    )?;

    for surface in &input.consumer_surfaces {
        ensure_token(
            STABLE_MANIFEST_HARDENING_CONSUMER_SURFACES,
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
    identity: &ManifestHardeningIdentity,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        MANIFEST_HARDENING_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_MANIFEST_HARDENING_SCHEMA_VERSION,
        "identity schema_version",
    )?;
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

fn validate_range(
    range: &ManifestCompatibilityRange,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        range.record_kind.as_str(),
        MANIFEST_COMPATIBILITY_RANGE_RECORD_KIND,
        "compatibility_range record_kind",
    )?;
    ensure_token(
        RANGE_RESOLUTION_CLASSES,
        &range.api_range_resolution_class,
        "compatibility_range api_range_resolution_class",
    )?;
    ensure_token(
        RANGE_RESOLUTION_CLASSES,
        &range.runtime_range_resolution_class,
        "compatibility_range runtime_range_resolution_class",
    )?;
    Ok(())
}

fn validate_permission(
    entry: &ManifestPermissionEntry,
    field: &str,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_nonempty(&entry.permission_ref, &format!("{field}.permission_ref"))?;
    ensure_token(
        PERMISSION_CAPABILITY_CLASSES,
        &entry.capability_class,
        &format!("{field}.capability_class"),
    )?;
    Ok(())
}

fn validate_dependency(
    dep: &ManifestDependencyEdge,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        dep.record_kind.as_str(),
        MANIFEST_DEPENDENCY_EDGE_RECORD_KIND,
        "dependency record_kind",
    )?;
    ensure_token(
        DEPENDENCY_CLASSES,
        &dep.dependency_class,
        "dependency dependency_class",
    )?;
    ensure_token(
        DEPENDENCY_RESOLUTION_STATE_CLASSES,
        &dep.resolution_state_class,
        "dependency resolution_state_class",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &dep.lifecycle_state_class,
        "dependency lifecycle_state_class",
    )?;
    ensure_token(
        DEPRECATION_CLASSES,
        &dep.deprecation_class,
        "dependency deprecation_class",
    )?;
    ensure_nonempty(&dep.dependency_id, "dependency dependency_id")?;
    ensure_nonempty(&dep.target_extension_ref, "dependency target_extension_ref")?;
    for p in &dep.contributed_permissions {
        validate_permission(p, "dependency.contributed_permission")?;
    }
    Ok(())
}

fn validate_effective_permissions(
    res: &EffectivePermissionResolution,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        res.record_kind.as_str(),
        EFFECTIVE_PERMISSION_RESOLUTION_RECORD_KIND,
        "effective_permissions record_kind",
    )?;
    // Effective must equal declared ∪ transitive.
    let declared: BTreeSet<&str> = res
        .declared_permission_refs
        .iter()
        .map(String::as_str)
        .collect();
    let transitive: BTreeSet<&str> = res
        .transitive_permission_refs
        .iter()
        .map(String::as_str)
        .collect();
    let effective: BTreeSet<&str> = res
        .effective_permission_refs
        .iter()
        .map(String::as_str)
        .collect();
    let union: BTreeSet<&str> = declared.union(&transitive).copied().collect();
    if effective != union {
        return Err(err(
            "effective_permission_refs must equal declared ∪ transitive permission refs",
        ));
    }
    for d in &res.diff_entries {
        ensure_nonempty(
            &d.permission_ref,
            "effective_permission diff.permission_ref",
        )?;
        ensure_token(
            PERMISSION_CAPABILITY_CLASSES,
            &d.capability_class,
            "effective_permission diff.capability_class",
        )?;
        ensure_token(
            PERMISSION_SOURCE_CLASSES,
            &d.source_class,
            "effective_permission diff.source_class",
        )?;
    }
    if res.diff_entries.len() != res.effective_permission_refs.len() {
        return Err(err(
            "effective_permission diff must carry one entry per effective permission",
        ));
    }
    Ok(())
}

fn validate_lifecycle_label(
    lc: &ManifestLifecycleLabel,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        lc.record_kind.as_str(),
        MANIFEST_LIFECYCLE_LABEL_RECORD_KIND,
        "lifecycle_label record_kind",
    )?;
    ensure_token(
        LIFECYCLE_STATE_CLASSES,
        &lc.lifecycle_state_class,
        "lifecycle_label lifecycle_state_class",
    )?;
    ensure_token(
        DEPRECATION_CLASSES,
        &lc.deprecation_class,
        "lifecycle_label deprecation_class",
    )?;
    Ok(())
}

fn validate_claim(
    claim: &ManifestHardeningQualificationClaim,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        MANIFEST_HARDENING_QUALIFICATION_CLAIM_RECORD_KIND,
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
            MANIFEST_HARDENING_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &DowngradedManifestBanner,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        DOWNGRADED_MANIFEST_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            MANIFEST_HARDENING_DOWNGRADE_REASONS,
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
    inspection: &StableManifestHardeningInspection,
    packet: &StableManifestHardeningPacket,
) -> Result<(), StableManifestHardeningValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_MANIFEST_HARDENING_INSPECTION_RECORD_KIND,
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
    if inspection.downgraded_manifest_banner_required
        != packet.downgraded_manifest_banner.must_display
    {
        return Err(err(
            "inspection downgraded_manifest_banner_required is inconsistent",
        ));
    }
    if inspection.attribution_complete != packet.attribution_complete() {
        return Err(err("inspection attribution_complete is inconsistent"));
    }
    if inspection.implicit_authority_widening_present
        != packet.effective_permissions.implicit_widening_present
    {
        return Err(err(
            "inspection implicit_authority_widening_present is inconsistent",
        ));
    }
    if inspection.effective_permission_count
        != packet.effective_permissions.effective_permission_refs.len()
    {
        return Err(err("inspection effective_permission_count is inconsistent"));
    }
    if inspection.declared_permission_count != packet.declared_permissions.len() {
        return Err(err("inspection declared_permission_count is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableManifestHardeningValidationError {
    StableManifestHardeningValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StableManifestHardeningValidationError>
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
) -> Result<(), StableManifestHardeningValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableManifestHardeningValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableManifestHardeningValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
