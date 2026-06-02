//! Finalize the extension bridge contract and the certification scope for the
//! stable ecosystem line, and downgrade any non-qualified category to preview —
//! one conformance-backed, automatically-narrowing certification packet.
//!
//! The beta-level install-review, runtime, and external-host modules own the
//! per-row admission and host contracts. The stable runtime-ABI, manifest,
//! lifecycle-flow, catalog-truth, and mirror-import lanes own the published truth
//! a claimed stable row carries on each of those axes. This module owns the layer
//! that closes the ecosystem line: the **finalized extension bridge contract**
//! a category drives, and the **certification scope** — the set of extension
//! categories that have actually qualified for the stable certification — bound
//! into one evidence-backed packet whose stability qualification is derived, not
//! asserted.
//!
//! A stable bridge-certification row must bind, machine-readably:
//!
//! - the **identity** (the certification-scope descriptor ref, the row identity,
//!   the package identity, the pinned certification-scope version, the publisher
//!   namespace, the pinned certification-evidence ref, the publisher trust tier,
//!   and the lifecycle state),
//! - the **bridge surface binding** — the bridge kind the category drives
//!   (`language_bridge` / `debug_bridge` / `scm_bridge` / `task_bridge` /
//!   `terminal_bridge` / `filesystem_bridge` / `search_bridge` / `ui_panel_bridge`
//!   / `data_infra_bridge`), the pinned bridge ABI version, whether the bridge
//!   contract is **finalized** (frozen for the stable line), whether the bridge
//!   boundary is **enforcement-backed** (not catalog-asserted), and the
//!   control-plane boundary (`guarded` / `advisory` / `unguarded`),
//! - the **certification scope** — the certification category, the scope status
//!   (`certified` / `provisional` / `excluded` / `deprecated_scope`), the
//!   certification evidence source, and whether conformance passed,
//! - the **permission posture** (declared-vs-effective refs, whether the bridge
//!   widened authority, whether re-consent is required) so a bridge can never
//!   silently widen privilege,
//! - the **compatibility** label (bridge scorecard ref, verified flag),
//! - the **activation-budget** instrumentation (so an unbounded activation cost can
//!   never ride a stable claim),
//! - the **install posture** (install scope and disclosure, revocation posture,
//!   mirrorability, rollback support), and
//! - the **stability qualification** after the posture is applied.
//!
//! The central rule mirrors the rest of the stable line: a **stable**
//! bridge-certification claim may never be implied from a catalog row or an
//! adjacent green category. A row that renders a `stable` badge must pin the
//! published certification-scope version and bridge ABI version, be
//! evidence-backed (not catalog-asserted), keep its publisher trust tier out of
//! quarantine, stay on a runnable lifecycle, finalize its bridge contract, keep
//! the bridge enforcement-backed, keep the bridge control plane guarded, keep its
//! category inside the certified scope with conformance passed and non-inherited
//! certification evidence, never widen permissions across the bridge, keep its
//! compatibility verified and not parity-limited / unsupported, keep its activation
//! cost bounded and within budget, disclose its install scope, keep a clean
//! revocation posture, stay mirrorable, and be fully attributed. When any of those
//! fails, the visible tier is **automatically narrowed below Stable** (`beta`,
//! `preview`, or `withdrawn`) with machine-readable reasons.
//!
//! The headline behavior of this row is the **certification-scope gate**: a
//! category that is not in the certified scope — whether `provisional`,
//! `excluded`, or `deprecated_scope`, whether its conformance has not passed, or
//! whether its certification evidence is inherited rather than earned — is
//! **downgraded to preview**. A non-qualified category never inherits a stable
//! badge from a certified neighbor.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never
//!   back a stable bridge-certification claim; it narrows below Stable.
//! - **No ambient bridge privilege.** A permission set widened across the bridge
//!   withdraws the row outright.
//! - **No unbounded activation cost.** An `unbounded` activation budget withdraws
//!   the row outright; an `over_budget` budget narrows to `beta`.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_bridge_certification_scope.schema.json`](../../../../schemas/extensions/stable_bridge_certification_scope.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/`.

use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable bridge-certification record.
pub const STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// The published, stable certification-scope version. A `stable` claim must pin
/// exactly this version; any other version narrows below Stable.
pub const STABLE_BRIDGE_CERTIFICATION_PUBLISHED_SCOPE_VERSION: u32 = 1;

/// The published, stable bridge ABI contract version. A `stable` claim must pin
/// exactly this version; any other version narrows below Stable.
pub const STABLE_BRIDGE_PUBLISHED_ABI_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_BRIDGE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/extensions/stable_bridge_certification_scope.schema.json";

/// Record-kind tag for [`StableBridgeCertificationScopePacket`].
pub const STABLE_BRIDGE_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "stable_bridge_certification_scope_packet";

/// Record-kind tag for [`BridgeCertificationIdentity`].
pub const BRIDGE_CERTIFICATION_IDENTITY_RECORD_KIND: &str = "stable_bridge_certification_identity";

/// Record-kind tag for [`BridgeSurfaceBinding`].
pub const BRIDGE_SURFACE_BINDING_RECORD_KIND: &str = "stable_bridge_surface_binding";

/// Record-kind tag for [`BridgeCertificationScope`].
pub const BRIDGE_CERTIFICATION_SCOPE_RECORD_KIND: &str = "stable_bridge_certification_scope";

/// Record-kind tag for [`BridgeCertificationPermissionPosture`].
pub const BRIDGE_CERTIFICATION_PERMISSION_POSTURE_RECORD_KIND: &str =
    "stable_bridge_certification_permission_posture";

/// Record-kind tag for [`BridgeCertificationCompatibility`].
pub const BRIDGE_CERTIFICATION_COMPATIBILITY_RECORD_KIND: &str =
    "stable_bridge_certification_compatibility";

/// Record-kind tag for [`BridgeCertificationActivationBudget`].
pub const BRIDGE_CERTIFICATION_ACTIVATION_BUDGET_RECORD_KIND: &str =
    "stable_bridge_certification_activation_budget";

/// Record-kind tag for [`BridgeCertificationInstallPosture`].
pub const BRIDGE_CERTIFICATION_INSTALL_POSTURE_RECORD_KIND: &str =
    "stable_bridge_certification_install_posture";

/// Record-kind tag for [`BridgeCertificationQualificationClaim`].
pub const BRIDGE_CERTIFICATION_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_bridge_certification_qualification_claim";

/// Record-kind tag for [`BridgeCertificationDowngradedBanner`].
pub const BRIDGE_CERTIFICATION_DOWNGRADED_BANNER_RECORD_KIND: &str =
    "stable_bridge_certification_downgraded_banner";

/// Record-kind tag for [`StableBridgeCertificationScopeInspection`].
pub const STABLE_BRIDGE_CERTIFICATION_INSPECTION_RECORD_KIND: &str =
    "stable_bridge_certification_inspection";

/// Record-kind tag for [`StableBridgeCertificationScopeSupportExport`].
pub const STABLE_BRIDGE_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_bridge_certification_support_export";

/// Closed publisher-trust-tier vocabulary, shared with the catalog-truth lane.
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

/// Lifecycle states a stable certification claim may keep (installable / runnable).
pub const RUNNABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed bridge-kind vocabulary — the capability bridge a category drives.
pub const BRIDGE_KIND_CLASSES: &[&str] = &[
    "language_bridge",
    "debug_bridge",
    "scm_bridge",
    "task_bridge",
    "terminal_bridge",
    "filesystem_bridge",
    "search_bridge",
    "ui_panel_bridge",
    "data_infra_bridge",
];

/// Closed control-plane-boundary vocabulary. `guarded` is the only posture a stable
/// claim may keep.
pub const CONTROL_PLANE_BOUNDARY_CLASSES: &[&str] = &["guarded", "advisory", "unguarded"];

/// Closed certification-category vocabulary — the extension categories under the
/// certification scope.
pub const CERTIFICATION_CATEGORY_CLASSES: &[&str] = &[
    "language_tools",
    "debuggers",
    "formatters_linters",
    "scm_providers",
    "task_runners",
    "terminal_tools",
    "themes_appearance",
    "snippets",
    "keymaps",
    "data_infra_adapters",
    "ai_assist",
    "general_ui",
];

/// Closed scope-status vocabulary. `certified` is the only status a stable claim may
/// keep; every other status narrows the category to preview.
pub const SCOPE_STATUS_CLASSES: &[&str] =
    &["certified", "provisional", "excluded", "deprecated_scope"];

/// The only scope status that backs a stable certification claim.
pub const CERTIFIED_SCOPE_STATUS: &str = "certified";

/// Closed certification evidence-source vocabulary. `inherited_from_adjacent` may
/// never back a stable claim.
pub const CERTIFICATION_EVIDENCE_SOURCE_CLASSES: &[&str] = &[
    "conformance_suite",
    "certified_workspace",
    "bridge_matrix",
    "vendor_attested",
    "inherited_from_adjacent",
];

/// Closed compatibility-label vocabulary.
pub const COMPATIBILITY_LABEL_CLASSES: &[&str] = &[
    "full_parity",
    "high_parity",
    "partial_parity",
    "limited_parity",
    "unsupported",
];

/// Closed activation-budget vocabulary. `within_budget` is the only state a stable
/// claim may keep.
pub const ACTIVATION_BUDGET_CLASSES: &[&str] =
    &["within_budget", "over_budget", "unbounded", "not_measured"];

/// Closed install-scope vocabulary.
pub const INSTALL_SCOPE_CLASSES: &[&str] = &["user", "workspace", "machine", "portable"];

/// Closed revocation-posture vocabulary. `clean` is the only posture a stable claim
/// may keep.
pub const REVOCATION_POSTURE_CLASSES: &[&str] = &["clean", "advisory", "quarantined", "revoked"];

/// Closed mirrorability vocabulary. `not_mirrorable` narrows a stable claim.
pub const MIRRORABILITY_CLASSES: &[&str] = &["mirrorable", "mirror_pinned", "not_mirrorable"];

/// Closed set of stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* certification claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_bridge_certification_claim",
    "beta_bridge_certification_partial_claim",
    "preview_bridge_certification_experimental_claim",
    "withdrawn_no_bridge_certification_claim",
];

/// Closed set of reasons that narrow a stable bridge-certification claim below Stable.
pub const BRIDGE_CERTIFICATION_DOWNGRADE_REASONS: &[&str] = &[
    "certification_version_not_published",
    "bridge_abi_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_runnable",
    "bridge_contract_not_finalized",
    "bridge_not_enforcement_backed",
    "bridge_control_plane_unguarded",
    "bridge_control_plane_advisory",
    "category_scope_provisional",
    "category_scope_excluded",
    "category_scope_deprecated",
    "certification_conformance_failed",
    "certification_evidence_inherited",
    "bridge_permission_widened",
    "compatibility_unsupported",
    "compatibility_parity_limited",
    "compatibility_not_verified",
    "activation_cost_unbounded",
    "activation_cost_over_budget",
    "activation_cost_not_measured",
    "install_scope_not_disclosed",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
    "revocation_posture_advisory",
    "not_mirrorable",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the bridge / certification cannot
/// be trusted as stable at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_runnable",
    "bridge_control_plane_unguarded",
    "bridge_permission_widened",
    "compatibility_unsupported",
    "activation_cost_unbounded",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
];

/// Reasons that narrow to `preview` (a structural / scope / disclosure shortfall).
/// The certification-scope gate lives here: a non-qualified category lands at preview.
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "certification_version_not_published",
    "bridge_abi_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "bridge_contract_not_finalized",
    "bridge_not_enforcement_backed",
    "category_scope_provisional",
    "category_scope_excluded",
    "category_scope_deprecated",
    "certification_conformance_failed",
    "certification_evidence_inherited",
    "compatibility_not_verified",
    "activation_cost_not_measured",
    "install_scope_not_disclosed",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "bridge_control_plane_advisory",
    "compatibility_parity_limited",
    "activation_cost_over_budget",
    "revocation_posture_advisory",
    "not_mirrorable",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_BRIDGE_CERTIFICATION_CONSUMER_SURFACES: &[&str] = &[
    "marketplace_result_row",
    "marketplace_detail_page",
    "install_review",
    "extension_detail_view",
    "bridge_inspector",
    "certification_dashboard",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable bridge-certification packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBridgeCertificationScopeInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: BridgeCertificationIdentityInput,
    /// Bridge-surface binding input.
    pub bridge_surface: BridgeSurfaceBindingInput,
    /// Certification-scope input.
    pub certification_scope: BridgeCertificationScopeInput,
    /// Permission-posture input.
    pub permission_posture: BridgeCertificationPermissionPostureInput,
    /// Compatibility input.
    pub compatibility: BridgeCertificationCompatibilityInput,
    /// Activation-budget input.
    pub activation_budget: BridgeCertificationActivationBudgetInput,
    /// Install-posture input.
    pub install_posture: BridgeCertificationInstallPostureInput,
    /// Stability qualification claim input.
    pub claim: BridgeCertificationQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationIdentityInput {
    /// Ref to the certification-scope descriptor this row stabilizes.
    pub certification_scope_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published certification-scope version this row pins.
    pub certification_scope_version: u32,
    /// Publisher namespace the row asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned certification-evidence bundle.
    pub certification_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`BridgeSurfaceBinding`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeSurfaceBindingInput {
    /// Bridge kind the category drives.
    pub bridge_kind_class: String,
    /// Published bridge ABI version this row pins.
    pub bridge_abi_version: u32,
    /// Ref to the bridge surface contract.
    pub bridge_surface_ref: String,
    /// Whether the bridge contract is finalized (frozen for the stable line).
    pub bridge_contract_finalized: bool,
    /// Whether the bridge boundary is enforcement-backed (not catalog-asserted).
    pub bridge_enforcement_backed: bool,
    /// Control-plane boundary posture.
    pub control_plane_boundary_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationScope`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationScopeInput {
    /// Certification category.
    pub category_class: String,
    /// Scope status for the category.
    pub scope_status_class: String,
    /// Certification evidence source.
    pub certification_evidence_source_class: String,
    /// Whether the category's conformance suite passed.
    pub conformance_passed: bool,
    /// Ref to the conformance report backing the scope.
    pub conformance_report_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationPermissionPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationPermissionPostureInput {
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after bridge resolution.
    pub effective_permission_ref: String,
    /// Whether the bridge widened authority beyond the declared set.
    pub widened_on_bridge: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationCompatibility`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationCompatibilityInput {
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the bridge compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified against the bridge ABI.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationActivationBudget`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationActivationBudgetInput {
    /// Activation-budget posture for the worst-case surface.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationInstallPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationInstallPostureInput {
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`BridgeCertificationQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationQualificationClaimInput {
    /// Certification tier claimed by the row.
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
pub struct BridgeCertificationIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the certification-scope descriptor this row stabilizes.
    pub certification_scope_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published certification-scope version this row pins.
    pub certification_scope_version: u32,
    /// Publisher namespace the row asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned certification-evidence bundle.
    pub certification_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl BridgeCertificationIdentity {
    /// Returns true when the row pins the published certification-scope version.
    pub fn scope_version_current(&self) -> bool {
        self.certification_scope_version == STABLE_BRIDGE_CERTIFICATION_PUBLISHED_SCOPE_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Finalized bridge-surface binding for the certified category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeSurfaceBinding {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Bridge kind the category drives.
    pub bridge_kind_class: String,
    /// Published bridge ABI version this row pins.
    pub bridge_abi_version: u32,
    /// Ref to the bridge surface contract.
    pub bridge_surface_ref: String,
    /// Whether the bridge contract is finalized.
    pub bridge_contract_finalized: bool,
    /// Whether the bridge boundary is enforcement-backed.
    pub bridge_enforcement_backed: bool,
    /// Control-plane boundary posture.
    pub control_plane_boundary_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl BridgeSurfaceBinding {
    /// Returns true when the row pins the published bridge ABI version.
    pub fn bridge_abi_current(&self) -> bool {
        self.bridge_abi_version == STABLE_BRIDGE_PUBLISHED_ABI_VERSION
    }

    /// Returns true when the bridge control plane is guarded.
    pub fn control_plane_guarded(&self) -> bool {
        self.control_plane_boundary_class == "guarded"
    }

    /// Returns true when the bridge control plane is unguarded.
    pub fn control_plane_unguarded(&self) -> bool {
        self.control_plane_boundary_class == "unguarded"
    }
}

/// Certification scope for the category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationScope {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Certification category.
    pub category_class: String,
    /// Scope status for the category.
    pub scope_status_class: String,
    /// Certification evidence source.
    pub certification_evidence_source_class: String,
    /// Whether the category's conformance suite passed.
    pub conformance_passed: bool,
    /// Ref to the conformance report backing the scope.
    pub conformance_report_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl BridgeCertificationScope {
    /// Returns true when the category is inside the certified stable scope.
    pub fn in_certified_scope(&self) -> bool {
        self.scope_status_class == CERTIFIED_SCOPE_STATUS
    }

    /// Returns true when the certification evidence inherits from an adjacent claim.
    pub fn evidence_inherited(&self) -> bool {
        self.certification_evidence_source_class == "inherited_from_adjacent"
    }
}

/// Permission posture for the bridged category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationPermissionPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after bridge resolution.
    pub effective_permission_ref: String,
    /// Whether the bridge widened authority beyond the declared set.
    pub widened_on_bridge: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compatibility binding for the bridged category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationCompatibility {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the bridge compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified against the bridge ABI.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl BridgeCertificationCompatibility {
    /// Returns true when compatibility reports an unsupported parity.
    pub fn unsupported(&self) -> bool {
        self.compatibility_label_class == "unsupported"
    }

    /// Returns true when compatibility reports a parity-limited posture short of
    /// unsupported.
    pub fn parity_limited(&self) -> bool {
        matches!(
            self.compatibility_label_class.as_str(),
            "partial_parity" | "limited_parity"
        )
    }
}

/// Activation-budget instrumentation for the row's worst-case surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationActivationBudget {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Activation-budget posture.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl BridgeCertificationActivationBudget {
    /// Returns true when the activation cost is bounded and within budget.
    pub fn within_budget(&self) -> bool {
        self.budget_class == "within_budget"
    }

    /// Returns true when the activation cost is unbounded.
    pub fn unbounded(&self) -> bool {
        self.budget_class == "unbounded"
    }
}

/// Install / mirror / revocation posture for the bridged category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationInstallPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl BridgeCertificationInstallPosture {
    /// Returns true when the revocation posture is clean.
    pub fn revocation_clean(&self) -> bool {
        self.revocation_posture_class == "clean"
    }

    /// Returns true when the row stays mirrorable.
    pub fn mirrorable(&self) -> bool {
        matches!(
            self.mirrorability_class.as_str(),
            "mirrorable" | "mirror_pinned"
        )
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Certification tier claimed by the row.
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

/// Downgraded-row banner requirement. Raised whenever a reviewer must see a
/// certification or bridge shortfall before relying on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeCertificationDowngradedBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// True when a downgraded-row banner must be displayed.
    pub must_display: bool,
    /// Most-severe applicable banner reason, drawn from the downgrade vocabulary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_reason_class: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row for CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBridgeCertificationScopeInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective certification tier.
    pub effective_tier: String,
    /// True when the claim is a stable certification claim.
    pub stable_claim: bool,
    /// True when the row pins the published certification-scope version.
    pub scope_version_current: bool,
    /// True when the row pins the published bridge ABI version.
    pub bridge_abi_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is runnable.
    pub lifecycle_runnable: bool,
    /// Bridge kind.
    pub bridge_kind_class: String,
    /// True when the bridge contract is finalized.
    pub bridge_contract_finalized: bool,
    /// True when the bridge is enforcement-backed.
    pub bridge_enforcement_backed: bool,
    /// True when the bridge control plane is guarded.
    pub control_plane_guarded: bool,
    /// Certification category.
    pub category_class: String,
    /// Scope status.
    pub scope_status_class: String,
    /// True when the category is in the certified scope.
    pub in_certified_scope: bool,
    /// True when conformance passed.
    pub conformance_passed: bool,
    /// True when the bridge did not widen permissions.
    pub permissions_not_widened: bool,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// True when compatibility was verified.
    pub compatibility_verified: bool,
    /// Activation-budget posture.
    pub activation_budget_class: String,
    /// True when the activation cost is bounded and within budget.
    pub activation_within_budget: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// True when identity and every artifact are fully attributed.
    pub attribution_complete: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

// ---------------------------------------------------------------------------
// Packet type
// ---------------------------------------------------------------------------

/// Stable bridge-certification packet consumed by marketplace result / detail rows,
/// install review, the extension detail view, the bridge inspector, the
/// certification dashboard, diagnostics, support export, docs/help, release packets,
/// and the CLI inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBridgeCertificationScopePacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: BridgeCertificationIdentity,
    /// Finalized bridge-surface binding.
    pub bridge_surface: BridgeSurfaceBinding,
    /// Certification scope.
    pub certification_scope: BridgeCertificationScope,
    /// Permission posture.
    pub permission_posture: BridgeCertificationPermissionPosture,
    /// Compatibility.
    pub compatibility: BridgeCertificationCompatibility,
    /// Activation-budget instrumentation.
    pub activation_budget: BridgeCertificationActivationBudget,
    /// Install posture.
    pub install_posture: BridgeCertificationInstallPosture,
    /// Stability qualification claim after the posture is applied.
    pub claim: BridgeCertificationQualificationClaim,
    /// Downgraded-row banner requirement.
    pub downgraded_banner: BridgeCertificationDowngradedBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a catalog row can never imply stable certification on its own.
    pub allows_catalog_only_trust: bool,
    /// False so a bridge can never widen permissions and ride a stable row.
    pub allows_ambient_bridge_privilege: bool,
    /// False so an unbounded activation cost can never ride a stable row.
    pub allows_unbounded_activation_cost: bool,
    /// Inspection row.
    pub inspection: StableBridgeCertificationScopeInspection,
}

impl StableBridgeCertificationScopePacket {
    /// Builds a stable bridge-certification packet from input, applying the
    /// certification posture to the claimed tier so any required downgrade below
    /// Stable is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableBridgeCertificationScopeValidationError`] when the input
    /// violates an identity, bridge, scope, permission, compatibility, budget,
    /// install, or claim invariant.
    pub fn from_input(
        input: StableBridgeCertificationScopeInput,
    ) -> Result<Self, StableBridgeCertificationScopeValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let bridge_surface = bridge_surface_record(&input.bridge_surface);
        let certification_scope = scope_record(&input.certification_scope);
        let permission_posture = permission_posture_record(&input.permission_posture);
        let compatibility = compatibility_record(&input.compatibility);
        let activation_budget = activation_budget_record(&input.activation_budget);
        let install_posture = install_posture_record(&input.install_posture);
        let attribution_complete = attribution_is_complete(
            &identity,
            &bridge_surface,
            &certification_scope,
            &compatibility,
            &activation_budget,
        );

        let posture = CertificationPosture {
            identity: &identity,
            bridge_surface: &bridge_surface,
            certification_scope: &certification_scope,
            permission_posture: &permission_posture,
            compatibility: &compatibility,
            activation_budget: &activation_budget,
            install_posture: &install_posture,
            attribution_complete,
        };

        let claim = claim_record(&input.claim, &posture);
        let downgraded_banner = banner_record(&posture);
        let inspection = inspection_record(&input.packet_id, &posture, &claim, &downgraded_banner);

        let packet = Self {
            record_kind: STABLE_BRIDGE_CERTIFICATION_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            bridge_surface,
            certification_scope,
            permission_posture,
            compatibility,
            activation_budget,
            install_posture,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_BRIDGE_CERTIFICATION_SCHEMA_REF.to_string()],
            allows_catalog_only_trust: false,
            allows_ambient_bridge_privilege: false,
            allows_unbounded_activation_cost: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable bridge-certification invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableBridgeCertificationScopeValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), StableBridgeCertificationScopeValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_BRIDGE_CERTIFICATION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_bridge_surface(&self.bridge_surface)?;
        validate_scope(&self.certification_scope)?;
        validate_permission_posture(&self.permission_posture)?;
        validate_compatibility(&self.compatibility)?;
        validate_activation_budget(&self.activation_budget)?;
        validate_install_posture(&self.install_posture)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_BRIDGE_CERTIFICATION_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_BRIDGE_CERTIFICATION_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No catalog-only trust, ambient bridge privilege, or unbounded activation
        // cost may ride a published stable certification row.
        if self.allows_catalog_only_trust
            || self.allows_ambient_bridge_privilege
            || self.allows_unbounded_activation_cost
        {
            return Err(err(
                "a stable bridge-certification packet must not allow catalog-only trust, ambient bridge privilege, or unbounded activation cost",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.scope_version_current() {
                return Err(err(
                    "stable effective tier must pin the published certification-scope version",
                ));
            }
            if !self.bridge_surface.bridge_abi_current() {
                return Err(err(
                    "stable effective tier must pin the published bridge ABI version",
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
            if !self.identity.lifecycle_runnable() {
                return Err(err(
                    "stable effective tier must stay on a runnable lifecycle",
                ));
            }
            if !self.bridge_surface.bridge_contract_finalized {
                return Err(err(
                    "stable effective tier must finalize its bridge contract",
                ));
            }
            if !self.bridge_surface.bridge_enforcement_backed {
                return Err(err(
                    "stable effective tier must keep its bridge enforcement-backed",
                ));
            }
            if !self.bridge_surface.control_plane_guarded() {
                return Err(err(
                    "stable effective tier must keep its bridge control plane guarded",
                ));
            }
            if !self.certification_scope.in_certified_scope() {
                return Err(err(
                    "stable effective tier must keep its category in the certified scope",
                ));
            }
            if !self.certification_scope.conformance_passed {
                return Err(err(
                    "stable effective tier must pass its certification conformance",
                ));
            }
            if self.certification_scope.evidence_inherited() {
                return Err(err(
                    "stable effective tier must not inherit its certification evidence",
                ));
            }
            if self.permission_posture.widened_on_bridge {
                return Err(err(
                    "stable effective tier must not widen permissions across the bridge",
                ));
            }
            if self.compatibility.unsupported()
                || self.compatibility.parity_limited()
                || !self.compatibility.compatibility_verified
            {
                return Err(err(
                    "stable effective tier must carry verified, non-parity-limited compatibility",
                ));
            }
            if !self.activation_budget.within_budget() {
                return Err(err(
                    "stable effective tier must keep its activation cost bounded and within budget",
                ));
            }
            if !self.install_posture.install_scope_disclosed {
                return Err(err("stable effective tier must disclose its install scope"));
            }
            if !self.install_posture.revocation_clean() {
                return Err(err(
                    "stable effective tier must keep a clean revocation posture",
                ));
            }
            if !self.install_posture.mirrorable() {
                return Err(err("stable effective tier must stay mirrorable"));
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
        let posture = CertificationPosture {
            identity: &self.identity,
            bridge_surface: &self.bridge_surface,
            certification_scope: &self.certification_scope,
            permission_posture: &self.permission_posture,
            compatibility: &self.compatibility,
            activation_budget: &self.activation_budget,
            install_posture: &self.install_posture,
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
        let mut stored = self.claim.downgrade_reasons.clone();
        stored.sort();
        let mut expected = derived.downgrade_reasons.clone();
        expected.sort();
        if stored != expected {
            return Err(err(
                "stored downgrade reasons do not match the posture-derived reasons",
            ));
        }

        // Banner truth.
        let banner_required = certification_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-row banner must_display does not match the certification posture",
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

    /// Returns true when a non-certified category is never left rendering stable.
    pub fn non_qualified_category_never_stable(&self) -> bool {
        if self.certification_scope.in_certified_scope() {
            return true;
        }
        !STABLE_TIERS.contains(&self.claim.effective_tier.as_str())
    }

    /// Returns true when identity, bridge surface, scope, compatibility, and the
    /// activation budget are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(
            &self.identity,
            &self.bridge_surface,
            &self.certification_scope,
            &self.compatibility,
            &self.activation_budget,
        )
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableBridgeCertificationScopeProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Marketplace / catalog row identity.
    pub row_identity_ref: String,
    /// Certification category.
    pub category_class: String,
    /// Bridge kind.
    pub bridge_kind_class: String,
    /// Scope status.
    pub scope_status_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable certification claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
}

impl From<StableBridgeCertificationScopePacket> for StableBridgeCertificationScopeProjection {
    fn from(packet: StableBridgeCertificationScopePacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            row_identity_ref: packet.identity.row_identity_ref,
            category_class: packet.certification_scope.category_class,
            bridge_kind_class: packet.bridge_surface.bridge_kind_class,
            scope_status_class: packet.certification_scope.scope_status_class,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_banner_required: packet.downgraded_banner.must_display,
            revocation_posture_class: packet.install_posture.revocation_posture_class,
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableBridgeCertificationScopeError`] when the payload fails to parse or
/// violates the stable bridge-certification invariants.
pub fn project_stable_bridge_certification_scope(
    payload: &str,
) -> Result<StableBridgeCertificationScopeProjection, StableBridgeCertificationScopeError> {
    let packet: StableBridgeCertificationScopePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableBridgeCertificationScopeProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support / partner / mirror export row that quotes the same closed
/// tokens as the packet without leaking raw artifact, evidence, or publisher-private
/// bytes, and preserves the certification scope, bridge finalization, and
/// conformance posture so a reviewer can see why a category is or is not certified.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBridgeCertificationScopeSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Marketplace / catalog row identity.
    pub row_identity_ref: String,
    /// Certification-scope descriptor ref.
    pub certification_scope_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Publisher namespace.
    pub publisher_namespace: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Bridge kind.
    pub bridge_kind_class: String,
    /// True when the bridge contract is finalized.
    pub bridge_contract_finalized: bool,
    /// True when the bridge is enforcement-backed.
    pub bridge_enforcement_backed: bool,
    /// Control-plane boundary.
    pub control_plane_boundary_class: String,
    /// Certification category.
    pub category_class: String,
    /// Scope status.
    pub scope_status_class: String,
    /// True when the category is in the certified scope.
    pub in_certified_scope: bool,
    /// True when conformance passed.
    pub conformance_passed: bool,
    /// Certification evidence source.
    pub certification_evidence_source_class: String,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Activation-budget posture.
    pub activation_budget_class: String,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim was narrowed below Stable.
    pub downgraded: bool,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// True when the effective tier blocks the row as a stable certification (withdrawn).
    pub blocks_stable_certification: bool,
    /// Export-safe summary suitable for support / partner / mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support / partner / mirror export row.
pub fn project_stable_bridge_certification_scope_support_export(
    packet: &StableBridgeCertificationScopePacket,
) -> StableBridgeCertificationScopeSupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Bridge kind={} finalized={} enforced={} control_plane={}. Category={} scope={} certified={} conformance={} evidence={}. Compatibility={}. Activation={}. Revocation={} mirrorability={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.bridge_surface.bridge_kind_class,
        packet.bridge_surface.bridge_contract_finalized,
        packet.bridge_surface.bridge_enforcement_backed,
        packet.bridge_surface.control_plane_boundary_class,
        packet.certification_scope.category_class,
        packet.certification_scope.scope_status_class,
        packet.certification_scope.in_certified_scope(),
        packet.certification_scope.conformance_passed,
        packet.certification_scope.certification_evidence_source_class,
        packet.compatibility.compatibility_label_class,
        packet.activation_budget.budget_class,
        packet.install_posture.revocation_posture_class,
        packet.install_posture.mirrorability_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StableBridgeCertificationScopeSupportExport {
        record_kind: STABLE_BRIDGE_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        export_id: format!(
            "stable_bridge_certification_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        row_identity_ref: packet.identity.row_identity_ref.clone(),
        certification_scope_ref: packet.identity.certification_scope_ref.clone(),
        extension_identity: packet.identity.extension_identity.clone(),
        extension_version: packet.identity.extension_version.clone(),
        publisher_namespace: packet.identity.publisher_namespace.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        bridge_kind_class: packet.bridge_surface.bridge_kind_class.clone(),
        bridge_contract_finalized: packet.bridge_surface.bridge_contract_finalized,
        bridge_enforcement_backed: packet.bridge_surface.bridge_enforcement_backed,
        control_plane_boundary_class: packet.bridge_surface.control_plane_boundary_class.clone(),
        category_class: packet.certification_scope.category_class.clone(),
        scope_status_class: packet.certification_scope.scope_status_class.clone(),
        in_certified_scope: packet.certification_scope.in_certified_scope(),
        conformance_passed: packet.certification_scope.conformance_passed,
        certification_evidence_source_class: packet
            .certification_scope
            .certification_evidence_source_class
            .clone(),
        compatibility_label_class: packet.compatibility.compatibility_label_class.clone(),
        activation_budget_class: packet.activation_budget.budget_class.clone(),
        revocation_posture_class: packet.install_posture.revocation_posture_class.clone(),
        mirrorability_class: packet.install_posture.mirrorability_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_certification: blocks,
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable bridge-certification operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableBridgeCertificationScopeError {
    /// Validation failed.
    Validation(StableBridgeCertificationScopeValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableBridgeCertificationScopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableBridgeCertificationScopeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable bridge-certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableBridgeCertificationScopeValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableBridgeCertificationScopeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableBridgeCertificationScopeValidationError {}

impl StableBridgeCertificationScopeValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableBridgeCertificationScopeError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableBridgeCertificationScopeValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableBridgeCertificationScopeValidationError> for StableBridgeCertificationScopeError {
    fn from(err: StableBridgeCertificationScopeValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the certification posture.
struct CertificationPosture<'a> {
    identity: &'a BridgeCertificationIdentity,
    bridge_surface: &'a BridgeSurfaceBinding,
    certification_scope: &'a BridgeCertificationScope,
    permission_posture: &'a BridgeCertificationPermissionPosture,
    compatibility: &'a BridgeCertificationCompatibility,
    activation_budget: &'a BridgeCertificationActivationBudget,
    install_posture: &'a BridgeCertificationInstallPosture,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the certification posture.
fn posture_reasons(posture: &CertificationPosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();

    // Identity.
    if !posture.identity.scope_version_current() {
        reasons.push("certification_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        reasons.push("lifecycle_not_runnable".to_string());
    }

    // Bridge surface.
    if !posture.bridge_surface.bridge_abi_current() {
        reasons.push("bridge_abi_not_published".to_string());
    }
    if !posture.bridge_surface.bridge_contract_finalized {
        reasons.push("bridge_contract_not_finalized".to_string());
    }
    if !posture.bridge_surface.bridge_enforcement_backed {
        reasons.push("bridge_not_enforcement_backed".to_string());
    }
    match posture.bridge_surface.control_plane_boundary_class.as_str() {
        "unguarded" => reasons.push("bridge_control_plane_unguarded".to_string()),
        "advisory" => reasons.push("bridge_control_plane_advisory".to_string()),
        _ => {}
    }

    // Certification scope.
    match posture.certification_scope.scope_status_class.as_str() {
        "provisional" => reasons.push("category_scope_provisional".to_string()),
        "excluded" => reasons.push("category_scope_excluded".to_string()),
        "deprecated_scope" => reasons.push("category_scope_deprecated".to_string()),
        _ => {}
    }
    if !posture.certification_scope.conformance_passed {
        reasons.push("certification_conformance_failed".to_string());
    }
    if posture.certification_scope.evidence_inherited() {
        reasons.push("certification_evidence_inherited".to_string());
    }

    // Permissions.
    if posture.permission_posture.widened_on_bridge {
        reasons.push("bridge_permission_widened".to_string());
    }

    // Compatibility.
    if posture.compatibility.unsupported() {
        reasons.push("compatibility_unsupported".to_string());
    } else if posture.compatibility.parity_limited() {
        reasons.push("compatibility_parity_limited".to_string());
    }
    if !posture.compatibility.compatibility_verified {
        reasons.push("compatibility_not_verified".to_string());
    }

    // Activation budget.
    match posture.activation_budget.budget_class.as_str() {
        "unbounded" => reasons.push("activation_cost_unbounded".to_string()),
        "over_budget" => reasons.push("activation_cost_over_budget".to_string()),
        "not_measured" => reasons.push("activation_cost_not_measured".to_string()),
        _ => {}
    }

    // Install posture.
    if !posture.install_posture.install_scope_disclosed {
        reasons.push("install_scope_not_disclosed".to_string());
    }
    match posture.install_posture.revocation_posture_class.as_str() {
        "quarantined" => reasons.push("revocation_posture_quarantined".to_string()),
        "revoked" => reasons.push("revocation_posture_revoked".to_string()),
        "advisory" => reasons.push("revocation_posture_advisory".to_string()),
        _ => {}
    }
    if !posture.install_posture.mirrorable() {
        reasons.push("not_mirrorable".to_string());
    }

    if !posture.attribution_complete {
        reasons.push("attribution_incomplete".to_string());
    }

    reasons.sort();
    reasons.dedup();
    reasons
}

/// Applies the certification posture to a claimed tier, narrowing automatically below
/// Stable when the evidence can no longer back it. The claim basis is folded in
/// separately so a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &CertificationPosture<'_>,
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
        "stable" => "stable_bridge_certification_claim",
        "beta" => "beta_bridge_certification_partial_claim",
        "preview" => "preview_bridge_certification_experimental_claim",
        "withdrawn" => "withdrawn_no_bridge_certification_claim",
        _ => "preview_bridge_certification_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity, bridge surface, scope, compatibility, and the
/// activation budget are fully attributed.
fn attribution_is_complete(
    identity: &BridgeCertificationIdentity,
    bridge_surface: &BridgeSurfaceBinding,
    certification_scope: &BridgeCertificationScope,
    compatibility: &BridgeCertificationCompatibility,
    activation_budget: &BridgeCertificationActivationBudget,
) -> bool {
    !identity.certification_scope_ref.trim().is_empty()
        && !identity.row_identity_ref.trim().is_empty()
        && !identity.certification_evidence_ref.trim().is_empty()
        && !identity.publisher_namespace.trim().is_empty()
        && !bridge_surface.bridge_surface_ref.trim().is_empty()
        && !certification_scope.conformance_report_ref.trim().is_empty()
        && !compatibility.scorecard_ref.trim().is_empty()
        && !activation_budget.measured_cost_ref.trim().is_empty()
}

/// Returns true when the certification posture requires a pre-trust warning banner.
fn certification_requires_warning(posture: &CertificationPosture<'_>) -> bool {
    posture.identity.publisher_trust_tier_class == "quarantined"
        || !posture.identity.lifecycle_runnable()
        || posture.bridge_surface.control_plane_unguarded()
        || posture.permission_posture.widened_on_bridge
        || posture.compatibility.unsupported()
        || posture.activation_budget.unbounded()
        || matches!(
            posture.install_posture.revocation_posture_class.as_str(),
            "quarantined" | "revoked"
        )
        || matches!(
            posture.certification_scope.scope_status_class.as_str(),
            "excluded" | "deprecated_scope"
        )
}

/// Picks the most-severe banner reason for a row that requires a warning.
fn banner_reason_for(posture: &CertificationPosture<'_>) -> Option<String> {
    if posture.permission_posture.widened_on_bridge {
        return Some("bridge_permission_widened".to_string());
    }
    if posture.bridge_surface.control_plane_unguarded() {
        return Some("bridge_control_plane_unguarded".to_string());
    }
    if posture.compatibility.unsupported() {
        return Some("compatibility_unsupported".to_string());
    }
    if posture.activation_budget.unbounded() {
        return Some("activation_cost_unbounded".to_string());
    }
    if posture.install_posture.revocation_posture_class == "quarantined" {
        return Some("revocation_posture_quarantined".to_string());
    }
    if posture.install_posture.revocation_posture_class == "revoked" {
        return Some("revocation_posture_revoked".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        return Some("lifecycle_not_runnable".to_string());
    }
    if posture.certification_scope.scope_status_class == "excluded" {
        return Some("category_scope_excluded".to_string());
    }
    if posture.certification_scope.scope_status_class == "deprecated_scope" {
        return Some("category_scope_deprecated".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &BridgeCertificationIdentityInput) -> BridgeCertificationIdentity {
    BridgeCertificationIdentity {
        record_kind: BRIDGE_CERTIFICATION_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        certification_scope_ref: input.certification_scope_ref.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        extension_identity: input.extension_identity.clone(),
        extension_version: input.extension_version.clone(),
        package_id: input.package_id.clone(),
        certification_scope_version: input.certification_scope_version,
        publisher_namespace: input.publisher_namespace.clone(),
        certification_evidence_ref: input.certification_evidence_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn bridge_surface_record(input: &BridgeSurfaceBindingInput) -> BridgeSurfaceBinding {
    BridgeSurfaceBinding {
        record_kind: BRIDGE_SURFACE_BINDING_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        bridge_kind_class: input.bridge_kind_class.clone(),
        bridge_abi_version: input.bridge_abi_version,
        bridge_surface_ref: input.bridge_surface_ref.clone(),
        bridge_contract_finalized: input.bridge_contract_finalized,
        bridge_enforcement_backed: input.bridge_enforcement_backed,
        control_plane_boundary_class: input.control_plane_boundary_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn scope_record(input: &BridgeCertificationScopeInput) -> BridgeCertificationScope {
    BridgeCertificationScope {
        record_kind: BRIDGE_CERTIFICATION_SCOPE_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        category_class: input.category_class.clone(),
        scope_status_class: input.scope_status_class.clone(),
        certification_evidence_source_class: input.certification_evidence_source_class.clone(),
        conformance_passed: input.conformance_passed,
        conformance_report_ref: input.conformance_report_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn permission_posture_record(
    input: &BridgeCertificationPermissionPostureInput,
) -> BridgeCertificationPermissionPosture {
    BridgeCertificationPermissionPosture {
        record_kind: BRIDGE_CERTIFICATION_PERMISSION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        declared_permission_ref: input.declared_permission_ref.clone(),
        effective_permission_ref: input.effective_permission_ref.clone(),
        widened_on_bridge: input.widened_on_bridge,
        reconsent_required: input.reconsent_required,
        summary_label: input.summary_label.clone(),
    }
}

fn compatibility_record(
    input: &BridgeCertificationCompatibilityInput,
) -> BridgeCertificationCompatibility {
    BridgeCertificationCompatibility {
        record_kind: BRIDGE_CERTIFICATION_COMPATIBILITY_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        compatibility_label_class: input.compatibility_label_class.clone(),
        scorecard_ref: input.scorecard_ref.clone(),
        compatibility_verified: input.compatibility_verified,
        summary_label: input.summary_label.clone(),
    }
}

fn activation_budget_record(
    input: &BridgeCertificationActivationBudgetInput,
) -> BridgeCertificationActivationBudget {
    BridgeCertificationActivationBudget {
        record_kind: BRIDGE_CERTIFICATION_ACTIVATION_BUDGET_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        budget_class: input.budget_class.clone(),
        measured_cost_ref: input.measured_cost_ref.clone(),
        budget_ceiling_ref: input.budget_ceiling_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn install_posture_record(
    input: &BridgeCertificationInstallPostureInput,
) -> BridgeCertificationInstallPosture {
    BridgeCertificationInstallPosture {
        record_kind: BRIDGE_CERTIFICATION_INSTALL_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        install_scope_class: input.install_scope_class.clone(),
        install_scope_disclosed: input.install_scope_disclosed,
        revocation_posture_class: input.revocation_posture_class.clone(),
        mirrorability_class: input.mirrorability_class.clone(),
        rollback_supported: input.rollback_supported,
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &BridgeCertificationQualificationClaimInput,
    posture: &CertificationPosture<'_>,
) -> BridgeCertificationQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    BridgeCertificationQualificationClaim {
        record_kind: BRIDGE_CERTIFICATION_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &CertificationPosture<'_>) -> BridgeCertificationDowngradedBanner {
    let must_display = certification_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Certification row requires review before install or enablement ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Bridge certification stabilized: finalized bridge, certified category, conformance passed, compatibility and activation current."
            .to_string()
    };
    BridgeCertificationDowngradedBanner {
        record_kind: BRIDGE_CERTIFICATION_DOWNGRADED_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &CertificationPosture<'_>,
    claim: &BridgeCertificationQualificationClaim,
    banner: &BridgeCertificationDowngradedBanner,
) -> StableBridgeCertificationScopeInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());

    StableBridgeCertificationScopeInspection {
        record_kind: STABLE_BRIDGE_CERTIFICATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        scope_version_current: posture.identity.scope_version_current(),
        bridge_abi_current: posture.bridge_surface.bridge_abi_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: posture.identity.lifecycle_runnable(),
        bridge_kind_class: posture.bridge_surface.bridge_kind_class.clone(),
        bridge_contract_finalized: posture.bridge_surface.bridge_contract_finalized,
        bridge_enforcement_backed: posture.bridge_surface.bridge_enforcement_backed,
        control_plane_guarded: posture.bridge_surface.control_plane_guarded(),
        category_class: posture.certification_scope.category_class.clone(),
        scope_status_class: posture.certification_scope.scope_status_class.clone(),
        in_certified_scope: posture.certification_scope.in_certified_scope(),
        conformance_passed: posture.certification_scope.conformance_passed,
        permissions_not_widened: !posture.permission_posture.widened_on_bridge,
        compatibility_label_class: posture.compatibility.compatibility_label_class.clone(),
        compatibility_verified: posture.compatibility.compatibility_verified,
        activation_budget_class: posture.activation_budget.budget_class.clone(),
        activation_within_budget: posture.activation_budget.within_budget(),
        revocation_posture_class: posture.install_posture.revocation_posture_class.clone(),
        mirrorability_class: posture.install_posture.mirrorability_class.clone(),
        downgraded: claim.downgraded,
        downgraded_banner_required: banner.must_display,
        attribution_complete: posture.attribution_complete,
        summary_label: claim.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn validate_input(
    input: &StableBridgeCertificationScopeInput,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(
        &id.certification_scope_ref,
        "identity.certification_scope_ref",
    )?;
    if !id
        .certification_scope_ref
        .starts_with("certification_scope:")
    {
        return Err(err(
            "identity.certification_scope_ref must start with 'certification_scope:'",
        ));
    }
    ensure_nonempty(&id.row_identity_ref, "identity.row_identity_ref")?;
    ensure_nonempty(&id.extension_identity, "identity.extension_identity")?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.package_id, "identity.package_id")?;
    ensure_nonempty(&id.publisher_namespace, "identity.publisher_namespace")?;
    ensure_nonempty(
        &id.certification_evidence_ref,
        "identity.certification_evidence_ref",
    )?;
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

    let bridge = &input.bridge_surface;
    ensure_token(
        BRIDGE_KIND_CLASSES,
        &bridge.bridge_kind_class,
        "bridge_surface.bridge_kind_class",
    )?;
    ensure_nonempty(
        &bridge.bridge_surface_ref,
        "bridge_surface.bridge_surface_ref",
    )?;
    ensure_token(
        CONTROL_PLANE_BOUNDARY_CLASSES,
        &bridge.control_plane_boundary_class,
        "bridge_surface.control_plane_boundary_class",
    )?;

    let scope = &input.certification_scope;
    ensure_token(
        CERTIFICATION_CATEGORY_CLASSES,
        &scope.category_class,
        "certification_scope.category_class",
    )?;
    ensure_token(
        SCOPE_STATUS_CLASSES,
        &scope.scope_status_class,
        "certification_scope.scope_status_class",
    )?;
    ensure_token(
        CERTIFICATION_EVIDENCE_SOURCE_CLASSES,
        &scope.certification_evidence_source_class,
        "certification_scope.certification_evidence_source_class",
    )?;
    ensure_nonempty(
        &scope.conformance_report_ref,
        "certification_scope.conformance_report_ref",
    )?;

    let perm = &input.permission_posture;
    ensure_nonempty(
        &perm.declared_permission_ref,
        "permission_posture.declared_permission_ref",
    )?;
    ensure_nonempty(
        &perm.effective_permission_ref,
        "permission_posture.effective_permission_ref",
    )?;

    let compat = &input.compatibility;
    ensure_token(
        COMPATIBILITY_LABEL_CLASSES,
        &compat.compatibility_label_class,
        "compatibility.compatibility_label_class",
    )?;
    ensure_nonempty(&compat.scorecard_ref, "compatibility.scorecard_ref")?;

    let act = &input.activation_budget;
    ensure_token(
        ACTIVATION_BUDGET_CLASSES,
        &act.budget_class,
        "activation_budget.budget_class",
    )?;
    ensure_nonempty(
        &act.measured_cost_ref,
        "activation_budget.measured_cost_ref",
    )?;
    ensure_nonempty(
        &act.budget_ceiling_ref,
        "activation_budget.budget_ceiling_ref",
    )?;

    let inst = &input.install_posture;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &inst.install_scope_class,
        "install_posture.install_scope_class",
    )?;
    ensure_token(
        REVOCATION_POSTURE_CLASSES,
        &inst.revocation_posture_class,
        "install_posture.revocation_posture_class",
    )?;
    ensure_token(
        MIRRORABILITY_CLASSES,
        &inst.mirrorability_class,
        "install_posture.mirrorability_class",
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
            STABLE_BRIDGE_CERTIFICATION_CONSUMER_SURFACES,
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
    identity: &BridgeCertificationIdentity,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        BRIDGE_CERTIFICATION_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_BRIDGE_CERTIFICATION_SCHEMA_VERSION,
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

fn validate_bridge_surface(
    bridge: &BridgeSurfaceBinding,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        bridge.record_kind.as_str(),
        BRIDGE_SURFACE_BINDING_RECORD_KIND,
        "bridge_surface record_kind",
    )?;
    ensure_token(
        BRIDGE_KIND_CLASSES,
        &bridge.bridge_kind_class,
        "bridge_surface bridge_kind_class",
    )?;
    ensure_token(
        CONTROL_PLANE_BOUNDARY_CLASSES,
        &bridge.control_plane_boundary_class,
        "bridge_surface control_plane_boundary_class",
    )?;
    ensure_nonempty(
        &bridge.bridge_surface_ref,
        "bridge_surface bridge_surface_ref",
    )?;
    Ok(())
}

fn validate_scope(
    scope: &BridgeCertificationScope,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        scope.record_kind.as_str(),
        BRIDGE_CERTIFICATION_SCOPE_RECORD_KIND,
        "certification_scope record_kind",
    )?;
    ensure_token(
        CERTIFICATION_CATEGORY_CLASSES,
        &scope.category_class,
        "certification_scope category_class",
    )?;
    ensure_token(
        SCOPE_STATUS_CLASSES,
        &scope.scope_status_class,
        "certification_scope scope_status_class",
    )?;
    ensure_token(
        CERTIFICATION_EVIDENCE_SOURCE_CLASSES,
        &scope.certification_evidence_source_class,
        "certification_scope certification_evidence_source_class",
    )?;
    Ok(())
}

fn validate_permission_posture(
    perm: &BridgeCertificationPermissionPosture,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        perm.record_kind.as_str(),
        BRIDGE_CERTIFICATION_PERMISSION_POSTURE_RECORD_KIND,
        "permission_posture record_kind",
    )?;
    ensure_nonempty(
        &perm.declared_permission_ref,
        "permission_posture declared_permission_ref",
    )?;
    ensure_nonempty(
        &perm.effective_permission_ref,
        "permission_posture effective_permission_ref",
    )?;
    Ok(())
}

fn validate_compatibility(
    compat: &BridgeCertificationCompatibility,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        compat.record_kind.as_str(),
        BRIDGE_CERTIFICATION_COMPATIBILITY_RECORD_KIND,
        "compatibility record_kind",
    )?;
    ensure_token(
        COMPATIBILITY_LABEL_CLASSES,
        &compat.compatibility_label_class,
        "compatibility compatibility_label_class",
    )?;
    ensure_nonempty(&compat.scorecard_ref, "compatibility scorecard_ref")?;
    Ok(())
}

fn validate_activation_budget(
    activation: &BridgeCertificationActivationBudget,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        activation.record_kind.as_str(),
        BRIDGE_CERTIFICATION_ACTIVATION_BUDGET_RECORD_KIND,
        "activation_budget record_kind",
    )?;
    ensure_token(
        ACTIVATION_BUDGET_CLASSES,
        &activation.budget_class,
        "activation_budget budget_class",
    )?;
    ensure_nonempty(
        &activation.measured_cost_ref,
        "activation_budget measured_cost_ref",
    )?;
    ensure_nonempty(
        &activation.budget_ceiling_ref,
        "activation_budget budget_ceiling_ref",
    )?;
    Ok(())
}

fn validate_install_posture(
    inst: &BridgeCertificationInstallPosture,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        inst.record_kind.as_str(),
        BRIDGE_CERTIFICATION_INSTALL_POSTURE_RECORD_KIND,
        "install_posture record_kind",
    )?;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &inst.install_scope_class,
        "install_posture install_scope_class",
    )?;
    ensure_token(
        REVOCATION_POSTURE_CLASSES,
        &inst.revocation_posture_class,
        "install_posture revocation_posture_class",
    )?;
    ensure_token(
        MIRRORABILITY_CLASSES,
        &inst.mirrorability_class,
        "install_posture mirrorability_class",
    )?;
    Ok(())
}

fn validate_claim(
    claim: &BridgeCertificationQualificationClaim,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        BRIDGE_CERTIFICATION_QUALIFICATION_CLAIM_RECORD_KIND,
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
            BRIDGE_CERTIFICATION_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &BridgeCertificationDowngradedBanner,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        BRIDGE_CERTIFICATION_DOWNGRADED_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            BRIDGE_CERTIFICATION_DOWNGRADE_REASONS,
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
    inspection: &StableBridgeCertificationScopeInspection,
    packet: &StableBridgeCertificationScopePacket,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_BRIDGE_CERTIFICATION_INSPECTION_RECORD_KIND,
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
    if inspection.in_certified_scope != packet.certification_scope.in_certified_scope() {
        return Err(err("inspection in_certified_scope is inconsistent"));
    }
    if inspection.control_plane_guarded != packet.bridge_surface.control_plane_guarded() {
        return Err(err("inspection control_plane_guarded is inconsistent"));
    }
    if inspection.permissions_not_widened == packet.permission_posture.widened_on_bridge {
        return Err(err("inspection permissions_not_widened is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableBridgeCertificationScopeValidationError {
    StableBridgeCertificationScopeValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StableBridgeCertificationScopeValidationError>
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
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableBridgeCertificationScopeValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
