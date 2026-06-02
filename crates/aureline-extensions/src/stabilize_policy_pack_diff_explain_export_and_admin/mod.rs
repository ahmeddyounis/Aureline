//! Stabilize policy-pack diff / explain / export and admin-facing ecosystem
//! governance on claimed enterprise lanes — one evidence-backed,
//! automatically-narrowing governance packet whose stability qualification is
//! derived, not asserted.
//!
//! The beta install-review, runtime, marketplace-catalog, and mirror-import lanes
//! own per-row admission, catalog truth, and import truth. The stable runtime-ABI,
//! manifest, lifecycle-flow, catalog-truth, bridge-certification, and
//! performance-budget lanes own the published truth a claimed stable row carries on
//! each of those axes. This module owns the layer that makes the **admin-facing
//! governance** of an extension on a claimed **enterprise lane** inspectable,
//! reviewable, and exportable:
//!
//! - the **policy-pack diff** — the base / target policy-pack versions, the
//!   added / removed / modified rule counts, the diff completeness
//!   (`complete` / `partial` / `missing`), and whether a breaking change was
//!   acknowledged, so an admin can review exactly what a pack revision changes,
//! - the **policy-decision explain** — the governance decision the active pack
//!   produces for the governed row (`allowed` / `constrained` / `blocked` /
//!   `quarantined`), the typed reason, the governing-rule ref, and whether a
//!   user-readable explanation is actually attached, so a decision is never
//!   unexplained,
//! - the **admin-facing export** — the export scope (`full` / `summary` /
//!   `decision_only`), whether the export is mechanically generated rather than
//!   hand-authored, the redaction posture (`metadata_safe` / `contains_private`),
//!   and the export ref, so a governance export is supportable and never leaks
//!   private bytes,
//! - the **enterprise-lane binding** — the lane class, the tenancy scope, and
//!   whether the enterprise-lane claim is attested rather than asserted,
//! - the **permission posture** (declared / effective / policy-cap refs and a
//!   no-widening flag) so no ambient privilege rides a governance claim,
//! - the **compatibility** label (scorecard ref, verified flag),
//! - the **install posture** (install scope and disclosure, activation cost class,
//!   revocation posture, mirrorability, rollback support), and
//! - the **stability qualification** after the posture is applied.
//!
//! The central rule mirrors the rest of the stable line: a **stable** governance
//! claim may never be implied from a catalog row or an adjacent green row. A row
//! that renders a `stable` badge must pin the published governance-profile version,
//! be evidence-backed (not catalog-asserted), keep its publisher trust tier out of
//! quarantine, stay on a runnable lifecycle, carry a complete diff with no
//! unacknowledged breaking change, attach an explanation to its decision, emit a
//! mechanically-generated metadata-safe export at full / summary scope, attest its
//! enterprise lane, never widen permissions beyond the declared manifest or the
//! policy cap, keep its activation cost bounded, keep its compatibility verified and
//! not parity-limited / unsupported, disclose its install scope, keep a clean
//! revocation posture, stay mirrorable, and be fully attributed. When any of those
//! fails, the visible tier is **automatically narrowed below Stable** (`beta`,
//! `preview`, or `withdrawn`) with machine-readable reasons.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No unbounded activation cost.** An `unbounded` activation-cost class
//!   withdraws the row outright.
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never
//!   back a stable governance claim; it narrows below Stable. A hand-authored
//!   export — a governance claim with no mechanical source behind it — narrows to
//!   `preview`.
//! - **No ambient extension privilege.** A permission set widened beyond the
//!   declared manifest or the policy cap withdraws the row outright. A
//!   `contains_private` export also withdraws the row.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_policy_pack_governance.schema.json`](../../../../schemas/extensions/stable_policy_pack_governance.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/`.

use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable policy-pack governance record.
pub const STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// The published, stable governance-profile version. A `stable` claim must pin
/// exactly this version; any other version narrows below Stable.
pub const STABLE_POLICY_PACK_GOVERNANCE_PUBLISHED_PROFILE_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/extensions/stable_policy_pack_governance.schema.json";

/// Record-kind tag for [`StablePolicyPackGovernancePacket`].
pub const STABLE_POLICY_PACK_GOVERNANCE_PACKET_RECORD_KIND: &str =
    "stable_policy_pack_governance_packet";

/// Record-kind tag for [`PolicyGovernanceIdentity`].
pub const POLICY_GOVERNANCE_IDENTITY_RECORD_KIND: &str = "stable_policy_governance_identity";

/// Record-kind tag for [`PolicyPackDiff`].
pub const POLICY_PACK_DIFF_RECORD_KIND: &str = "stable_policy_pack_diff";

/// Record-kind tag for [`PolicyDecisionExplain`].
pub const POLICY_DECISION_EXPLAIN_RECORD_KIND: &str = "stable_policy_decision_explain";

/// Record-kind tag for [`AdminGovernanceExport`].
pub const ADMIN_GOVERNANCE_EXPORT_RECORD_KIND: &str = "stable_admin_governance_export";

/// Record-kind tag for [`EnterpriseLaneBinding`].
pub const ENTERPRISE_LANE_BINDING_RECORD_KIND: &str = "stable_enterprise_lane_binding";

/// Record-kind tag for [`PolicyGovernancePermissionPosture`].
pub const POLICY_GOVERNANCE_PERMISSION_POSTURE_RECORD_KIND: &str =
    "stable_policy_governance_permission_posture";

/// Record-kind tag for [`PolicyGovernanceCompatibility`].
pub const POLICY_GOVERNANCE_COMPATIBILITY_RECORD_KIND: &str =
    "stable_policy_governance_compatibility";

/// Record-kind tag for [`PolicyGovernanceInstallPosture`].
pub const POLICY_GOVERNANCE_INSTALL_POSTURE_RECORD_KIND: &str =
    "stable_policy_governance_install_posture";

/// Record-kind tag for [`PolicyGovernanceQualificationClaim`].
pub const POLICY_GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_policy_governance_qualification_claim";

/// Record-kind tag for [`PolicyGovernanceDowngradedBanner`].
pub const POLICY_GOVERNANCE_DOWNGRADED_BANNER_RECORD_KIND: &str =
    "stable_policy_governance_downgraded_banner";

/// Record-kind tag for [`StablePolicyPackGovernanceInspection`].
pub const STABLE_POLICY_PACK_GOVERNANCE_INSPECTION_RECORD_KIND: &str =
    "stable_policy_pack_governance_inspection";

/// Record-kind tag for [`StablePolicyPackGovernanceSupportExport`].
pub const STABLE_POLICY_PACK_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_policy_pack_governance_support_export";

/// Closed publisher-trust-tier vocabulary, shared with the rest of the stable line.
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

/// Lifecycle states a stable governance claim may keep (installable / runnable).
pub const RUNNABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed diff-completeness vocabulary. `complete` is the only completeness a stable
/// claim may keep.
pub const DIFF_COMPLETENESS_CLASSES: &[&str] = &["complete", "partial", "missing"];

/// Closed governance-decision vocabulary the active pack produces for the row.
pub const DECISION_CLASSES: &[&str] = &["allowed", "constrained", "blocked", "quarantined"];

/// Closed governance-reason vocabulary explaining why the decision was reached.
pub const DECISION_REASON_CLASSES: &[&str] = &[
    "within_policy",
    "permission_capped",
    "publisher_below_trust_floor",
    "version_out_of_range",
    "on_admin_denylist",
    "superseded_by_pack",
];

/// Closed export-scope vocabulary. `decision_only` narrows a stable claim to `beta`.
pub const EXPORT_SCOPE_CLASSES: &[&str] = &["full", "summary", "decision_only"];

/// Closed export-source vocabulary. `hand_authored` may never back a stable claim.
pub const EXPORT_SOURCE_CLASSES: &[&str] = &["mechanically_generated", "hand_authored"];

/// Closed export-redaction vocabulary. `contains_private` withdraws the row.
pub const EXPORT_REDACTION_CLASSES: &[&str] = &["metadata_safe", "contains_private"];

/// Closed enterprise-lane vocabulary the governance row is claimed on.
pub const ENTERPRISE_LANE_CLASSES: &[&str] = &[
    "enterprise_managed",
    "enterprise_self_hosted",
    "enterprise_air_gapped",
    "standard",
];

/// Closed lane-claim-basis vocabulary. `asserted` narrows below Stable.
pub const LANE_CLAIM_BASIS_CLASSES: &[&str] = &["attested", "asserted"];

/// Closed tenancy-scope vocabulary.
pub const TENANCY_SCOPE_CLASSES: &[&str] = &["single_tenant", "multi_tenant", "per_workspace"];

/// Closed activation-cost vocabulary — the headline cost the governed row carries.
/// `unbounded` may never ride a stable claim.
pub const ACTIVATION_COST_CLASSES: &[&str] =
    &["negligible", "light", "moderate", "heavy", "unbounded"];

/// Closed compatibility-label vocabulary.
pub const COMPATIBILITY_LABEL_CLASSES: &[&str] = &[
    "full_parity",
    "high_parity",
    "partial_parity",
    "limited_parity",
    "unsupported",
];

/// Closed install-scope vocabulary.
pub const INSTALL_SCOPE_CLASSES: &[&str] = &["user", "workspace", "machine", "portable"];

/// Closed revocation-posture vocabulary. `clean` is the only posture a stable claim
/// may keep.
pub const REVOCATION_POSTURE_CLASSES: &[&str] = &["clean", "advisory", "quarantined", "revoked"];

/// Closed mirrorability vocabulary. `not_mirrorable` narrows a stable claim.
pub const MIRRORABILITY_CLASSES: &[&str] = &["mirrorable", "mirror_pinned", "not_mirrorable"];

/// Closed set of stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* governance claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_governance_claim",
    "beta_governance_partial_claim",
    "preview_governance_experimental_claim",
    "withdrawn_no_governance_claim",
];

/// Closed set of reasons that narrow a stable governance claim below Stable.
pub const POLICY_PACK_GOVERNANCE_DOWNGRADE_REASONS: &[&str] = &[
    "governance_profile_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_runnable",
    "diff_missing",
    "diff_partial",
    "diff_unacknowledged_breaking_change",
    "decision_not_explained",
    "export_not_mechanically_sourced",
    "export_scope_limited",
    "export_contains_private_data",
    "enterprise_lane_not_attested",
    "permission_widened",
    "activation_cost_unbounded",
    "compatibility_unsupported",
    "compatibility_parity_limited",
    "compatibility_not_verified",
    "install_scope_not_disclosed",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
    "revocation_posture_advisory",
    "not_mirrorable",
    "attribution_incomplete",
];

/// Reasons that narrow all the way to `withdrawn` (the governance row cannot be
/// trusted as stable at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_runnable",
    "export_contains_private_data",
    "permission_widened",
    "activation_cost_unbounded",
    "compatibility_unsupported",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
];

/// Reasons that narrow to `preview` (a structural / disclosure / evidence shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "governance_profile_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "diff_missing",
    "decision_not_explained",
    "export_not_mechanically_sourced",
    "enterprise_lane_not_attested",
    "compatibility_not_verified",
    "install_scope_not_disclosed",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "diff_partial",
    "diff_unacknowledged_breaking_change",
    "export_scope_limited",
    "compatibility_parity_limited",
    "revocation_posture_advisory",
    "not_mirrorable",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_POLICY_PACK_GOVERNANCE_CONSUMER_SURFACES: &[&str] = &[
    "admin_governance_console",
    "policy_pack_diff_view",
    "policy_decision_explainer",
    "install_review",
    "extension_detail_view",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable policy-pack governance packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePolicyPackGovernanceInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: PolicyGovernanceIdentityInput,
    /// Policy-pack diff input.
    pub diff: PolicyPackDiffInput,
    /// Policy-decision explain input.
    pub explain: PolicyDecisionExplainInput,
    /// Admin-facing export input.
    pub export: AdminGovernanceExportInput,
    /// Enterprise-lane binding input.
    pub enterprise_lane: EnterpriseLaneBindingInput,
    /// Permission-posture input.
    pub permission_posture: PolicyGovernancePermissionPostureInput,
    /// Compatibility input.
    pub compatibility: PolicyGovernanceCompatibilityInput,
    /// Install-posture input.
    pub install_posture: PolicyGovernanceInstallPostureInput,
    /// Stability qualification claim input.
    pub claim: PolicyGovernanceQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`PolicyGovernanceIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceIdentityInput {
    /// Ref to the governance-profile descriptor this row stabilizes.
    pub governance_profile_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Policy-pack id.
    pub policy_pack_id: String,
    /// Policy-pack version the active governance decision was taken under.
    pub policy_pack_version: u32,
    /// Published governance-profile version this row pins.
    pub governance_profile_version: u32,
    /// Admin / governance-authority namespace the pack is published under.
    pub admin_namespace: String,
    /// Publisher namespace the governed extension asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned governance-evidence bundle.
    pub governance_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`PolicyPackDiff`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackDiffInput {
    /// Base policy-pack version the diff is taken from.
    pub base_pack_version: u32,
    /// Target policy-pack version the diff is taken to.
    pub target_pack_version: u32,
    /// Diff completeness.
    pub diff_completeness_class: String,
    /// Number of rules added between base and target.
    pub rules_added: u32,
    /// Number of rules removed between base and target.
    pub rules_removed: u32,
    /// Number of rules modified between base and target.
    pub rules_modified: u32,
    /// Whether the diff introduces a breaking change for governed rows.
    pub breaking_change: bool,
    /// Whether an admin acknowledged the breaking change.
    pub breaking_change_acknowledged: bool,
    /// Ref to the full diff record.
    pub diff_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PolicyDecisionExplain`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecisionExplainInput {
    /// Governance decision the active pack produces for the row.
    pub decision_class: String,
    /// Typed reason for the decision.
    pub reason_class: String,
    /// Ref to the rule that drove the decision.
    pub governing_rule_ref: String,
    /// Whether a user-readable decision explanation is attached.
    pub decision_explained: bool,
    /// Ref to the user-visible explanation copy.
    pub explanation_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`AdminGovernanceExport`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminGovernanceExportInput {
    /// Export scope.
    pub export_scope_class: String,
    /// Export source.
    pub export_source_class: String,
    /// Export redaction posture.
    pub export_redaction_class: String,
    /// Ref to the export bundle.
    pub export_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`EnterpriseLaneBinding`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseLaneBindingInput {
    /// Enterprise-lane class the row is claimed on.
    pub enterprise_lane_class: String,
    /// Whether the enterprise-lane claim is attested or merely asserted.
    pub lane_claim_basis_class: String,
    /// Tenancy scope.
    pub tenancy_scope_class: String,
    /// Ref to the lane-attestation record.
    pub lane_attestation_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PolicyGovernancePermissionPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernancePermissionPostureInput {
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after resolution.
    pub effective_permission_ref: String,
    /// Ref to the policy-pack permission cap applied to the row.
    pub policy_cap_ref: String,
    /// Whether authority was widened beyond the declared set or the policy cap.
    pub widened: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PolicyGovernanceCompatibility`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceCompatibilityInput {
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PolicyGovernanceInstallPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceInstallPostureInput {
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Headline activation-cost class for the governed row.
    pub activation_cost_class: String,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`PolicyGovernanceQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceQualificationClaimInput {
    /// Governance tier claimed by the row.
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
pub struct PolicyGovernanceIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the governance-profile descriptor this row stabilizes.
    pub governance_profile_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Policy-pack id.
    pub policy_pack_id: String,
    /// Policy-pack version the active governance decision was taken under.
    pub policy_pack_version: u32,
    /// Published governance-profile version this row pins.
    pub governance_profile_version: u32,
    /// Admin / governance-authority namespace the pack is published under.
    pub admin_namespace: String,
    /// Publisher namespace the governed extension asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned governance-evidence bundle.
    pub governance_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl PolicyGovernanceIdentity {
    /// Returns true when the row pins the published governance-profile version.
    pub fn profile_version_current(&self) -> bool {
        self.governance_profile_version == STABLE_POLICY_PACK_GOVERNANCE_PUBLISHED_PROFILE_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Policy-pack diff between a base and target pack version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPackDiff {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Base policy-pack version.
    pub base_pack_version: u32,
    /// Target policy-pack version.
    pub target_pack_version: u32,
    /// Diff completeness.
    pub diff_completeness_class: String,
    /// Number of rules added.
    pub rules_added: u32,
    /// Number of rules removed.
    pub rules_removed: u32,
    /// Number of rules modified.
    pub rules_modified: u32,
    /// Whether the diff introduces a breaking change.
    pub breaking_change: bool,
    /// Whether an admin acknowledged the breaking change.
    pub breaking_change_acknowledged: bool,
    /// Ref to the full diff record.
    pub diff_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PolicyPackDiff {
    /// Returns true when the diff is complete and reviewable.
    pub fn complete(&self) -> bool {
        self.diff_completeness_class == "complete"
    }

    /// Returns true when the diff is missing entirely.
    pub fn missing(&self) -> bool {
        self.diff_completeness_class == "missing"
    }

    /// Returns true when the diff is only partial.
    pub fn partial(&self) -> bool {
        self.diff_completeness_class == "partial"
    }

    /// Returns true when a breaking change exists but no admin acknowledged it.
    pub fn unacknowledged_breaking(&self) -> bool {
        self.breaking_change && !self.breaking_change_acknowledged
    }

    /// Returns the total number of changed rules.
    pub fn changed_rules(&self) -> u32 {
        self.rules_added
            .saturating_add(self.rules_removed)
            .saturating_add(self.rules_modified)
    }
}

/// Policy-decision explanation the active pack produces for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecisionExplain {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Governance decision.
    pub decision_class: String,
    /// Typed reason for the decision.
    pub reason_class: String,
    /// Ref to the governing rule.
    pub governing_rule_ref: String,
    /// Whether a user-readable explanation is attached.
    pub decision_explained: bool,
    /// Ref to the user-visible explanation copy.
    pub explanation_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PolicyDecisionExplain {
    /// Returns true when the governed row is blocked or quarantined by policy.
    pub fn blocks_governed_row(&self) -> bool {
        matches!(self.decision_class.as_str(), "blocked" | "quarantined")
    }
}

/// Admin-facing governance export posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminGovernanceExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Export scope.
    pub export_scope_class: String,
    /// Export source.
    pub export_source_class: String,
    /// Export redaction posture.
    pub export_redaction_class: String,
    /// Ref to the export bundle.
    pub export_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl AdminGovernanceExport {
    /// Returns true when the export is mechanically generated, not hand-authored.
    pub fn mechanically_sourced(&self) -> bool {
        self.export_source_class == "mechanically_generated"
    }

    /// Returns true when the export carries raw private bytes.
    pub fn contains_private(&self) -> bool {
        self.export_redaction_class == "contains_private"
    }

    /// Returns true when the export scope is reduced to decision-only.
    pub fn scope_limited(&self) -> bool {
        self.export_scope_class == "decision_only"
    }
}

/// Enterprise-lane binding for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseLaneBinding {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Enterprise-lane class.
    pub enterprise_lane_class: String,
    /// Lane-claim basis.
    pub lane_claim_basis_class: String,
    /// Tenancy scope.
    pub tenancy_scope_class: String,
    /// Ref to the lane-attestation record.
    pub lane_attestation_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl EnterpriseLaneBinding {
    /// Returns true when the enterprise-lane claim is attested.
    pub fn attested(&self) -> bool {
        self.lane_claim_basis_class == "attested"
    }
}

/// Permission posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernancePermissionPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after resolution.
    pub effective_permission_ref: String,
    /// Ref to the policy-pack permission cap.
    pub policy_cap_ref: String,
    /// Whether authority was widened beyond the declared set or the policy cap.
    pub widened: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compatibility binding for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceCompatibility {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PolicyGovernanceCompatibility {
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

/// Install / mirror / revocation posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceInstallPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Headline activation-cost class for the governed row.
    pub activation_cost_class: String,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl PolicyGovernanceInstallPosture {
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

    /// Returns true when the activation cost is unbounded.
    pub fn activation_cost_unbounded(&self) -> bool {
        self.activation_cost_class == "unbounded"
    }
}

/// Stability qualification claim after the posture is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Governance tier claimed by the row.
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

/// Downgraded-row banner requirement. Raised whenever an admin / reviewer must see a
/// governance shortfall before relying on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceDowngradedBanner {
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
pub struct StablePolicyPackGovernanceInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective governance tier.
    pub effective_tier: String,
    /// True when the claim is a stable governance claim.
    pub stable_claim: bool,
    /// True when the row pins the published profile version.
    pub profile_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is runnable.
    pub lifecycle_runnable: bool,
    /// Base policy-pack version.
    pub base_pack_version: u32,
    /// Target policy-pack version.
    pub target_pack_version: u32,
    /// Diff completeness.
    pub diff_completeness_class: String,
    /// True when the diff is complete and reviewable.
    pub diff_complete: bool,
    /// Governance decision.
    pub decision_class: String,
    /// True when the decision is explained to the user.
    pub decision_explained: bool,
    /// Export scope.
    pub export_scope_class: String,
    /// True when the export is mechanically generated.
    pub export_mechanically_sourced: bool,
    /// Export redaction posture.
    pub export_redaction_class: String,
    /// Enterprise-lane class.
    pub enterprise_lane_class: String,
    /// True when the enterprise-lane claim is attested.
    pub enterprise_lane_attested: bool,
    /// True when permissions were not widened.
    pub permissions_not_widened: bool,
    /// Headline activation-cost class.
    pub activation_cost_class: String,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// True when compatibility was verified.
    pub compatibility_verified: bool,
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

/// Stable policy-pack governance packet consumed by the admin governance console,
/// the policy-pack diff view, the policy-decision explainer, install review, the
/// extension detail view, diagnostics, support export, docs/help, release packets,
/// and the CLI inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePolicyPackGovernancePacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: PolicyGovernanceIdentity,
    /// Policy-pack diff.
    pub diff: PolicyPackDiff,
    /// Policy-decision explanation.
    pub explain: PolicyDecisionExplain,
    /// Admin-facing export.
    pub export: AdminGovernanceExport,
    /// Enterprise-lane binding.
    pub enterprise_lane: EnterpriseLaneBinding,
    /// Permission posture.
    pub permission_posture: PolicyGovernancePermissionPosture,
    /// Compatibility.
    pub compatibility: PolicyGovernanceCompatibility,
    /// Install posture.
    pub install_posture: PolicyGovernanceInstallPosture,
    /// Stability qualification claim after the posture is applied.
    pub claim: PolicyGovernanceQualificationClaim,
    /// Downgraded-row banner requirement.
    pub downgraded_banner: PolicyGovernanceDowngradedBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a catalog row can never imply a stable governance claim on its own.
    pub allows_catalog_only_trust: bool,
    /// False so a widened permission set can never ride a stable governance row.
    pub allows_ambient_extension_privilege: bool,
    /// False so an unbounded activation cost can never ride a stable governance row.
    pub allows_unbounded_activation_cost: bool,
    /// Inspection row.
    pub inspection: StablePolicyPackGovernanceInspection,
}

impl StablePolicyPackGovernancePacket {
    /// Builds a stable policy-pack governance packet from input, applying the
    /// governance posture to the claimed tier so any required downgrade below Stable
    /// is automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StablePolicyPackGovernanceValidationError`] when the input violates
    /// an identity, diff, explain, export, lane, permission, compatibility, install,
    /// or claim invariant.
    pub fn from_input(
        input: StablePolicyPackGovernanceInput,
    ) -> Result<Self, StablePolicyPackGovernanceValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let diff = diff_record(&input.diff);
        let explain = explain_record(&input.explain);
        let export = export_record(&input.export);
        let enterprise_lane = enterprise_lane_record(&input.enterprise_lane);
        let permission_posture = permission_posture_record(&input.permission_posture);
        let compatibility = compatibility_record(&input.compatibility);
        let install_posture = install_posture_record(&input.install_posture);
        let attribution_complete = attribution_is_complete(
            &identity,
            &diff,
            &explain,
            &export,
            &enterprise_lane,
            &compatibility,
        );

        let posture = GovernancePosture {
            identity: &identity,
            diff: &diff,
            explain: &explain,
            export: &export,
            enterprise_lane: &enterprise_lane,
            permission_posture: &permission_posture,
            compatibility: &compatibility,
            install_posture: &install_posture,
            attribution_complete,
        };

        let claim = claim_record(&input.claim, &posture);
        let downgraded_banner = banner_record(&posture);
        let inspection = inspection_record(&input.packet_id, &posture, &claim, &downgraded_banner);

        let packet = Self {
            record_kind: STABLE_POLICY_PACK_GOVERNANCE_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            diff,
            explain,
            export,
            enterprise_lane,
            permission_posture,
            compatibility,
            install_posture,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_REF.to_string()],
            allows_catalog_only_trust: false,
            allows_ambient_extension_privilege: false,
            allows_unbounded_activation_cost: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable governance invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StablePolicyPackGovernanceValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), StablePolicyPackGovernanceValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_POLICY_PACK_GOVERNANCE_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_diff(&self.diff)?;
        validate_explain(&self.explain)?;
        validate_export(&self.export)?;
        validate_enterprise_lane(&self.enterprise_lane)?;
        validate_permission_posture(&self.permission_posture)?;
        validate_compatibility(&self.compatibility)?;
        validate_install_posture(&self.install_posture)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;
        validate_diff_counts(&self.diff)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_POLICY_PACK_GOVERNANCE_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No catalog-only trust, ambient extension privilege, or unbounded activation
        // cost may ride a published stable governance row.
        if self.allows_catalog_only_trust
            || self.allows_ambient_extension_privilege
            || self.allows_unbounded_activation_cost
        {
            return Err(err(
                "a stable governance packet must not allow catalog-only trust, ambient extension privilege, or unbounded activation cost",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.profile_version_current() {
                return Err(err(
                    "stable effective tier must pin the published governance-profile version",
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
            if !self.diff.complete() {
                return Err(err(
                    "stable effective tier must carry a complete policy-pack diff",
                ));
            }
            if self.diff.unacknowledged_breaking() {
                return Err(err(
                    "stable effective tier must acknowledge any breaking policy-pack change",
                ));
            }
            if !self.explain.decision_explained {
                return Err(err(
                    "stable effective tier must explain its governance decision",
                ));
            }
            if !self.export.mechanically_sourced() {
                return Err(err(
                    "stable effective tier must emit a mechanically-generated governance export",
                ));
            }
            if self.export.contains_private() {
                return Err(err(
                    "stable effective tier must not export raw private data",
                ));
            }
            if self.export.scope_limited() {
                return Err(err(
                    "stable effective tier must export at full or summary scope, not decision-only",
                ));
            }
            if !self.enterprise_lane.attested() {
                return Err(err("stable effective tier must attest its enterprise lane"));
            }
            if self.permission_posture.widened {
                return Err(err("stable effective tier must not widen permissions"));
            }
            if self.install_posture.activation_cost_unbounded() {
                return Err(err(
                    "stable effective tier must not carry an unbounded activation cost",
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
        let posture = GovernancePosture {
            identity: &self.identity,
            diff: &self.diff,
            explain: &self.explain,
            export: &self.export,
            enterprise_lane: &self.enterprise_lane,
            permission_posture: &self.permission_posture,
            compatibility: &self.compatibility,
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
        let banner_required = governance_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-row banner must_display does not match the governance posture",
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

    /// Returns true when an unbounded activation cost is never left rendering stable.
    pub fn unbounded_cost_never_stable(&self) -> bool {
        if !self.install_posture.activation_cost_unbounded() {
            return true;
        }
        !STABLE_TIERS.contains(&self.claim.effective_tier.as_str())
    }

    /// Returns true when a private export is never left rendering stable.
    pub fn private_export_never_stable(&self) -> bool {
        if !self.export.contains_private() {
            return true;
        }
        !STABLE_TIERS.contains(&self.claim.effective_tier.as_str())
    }

    /// Returns true when identity, the diff, the explanation, the export, the lane,
    /// and compatibility are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(
            &self.identity,
            &self.diff,
            &self.explain,
            &self.export,
            &self.enterprise_lane,
            &self.compatibility,
        )
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StablePolicyPackGovernanceProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Marketplace / catalog row identity.
    pub row_identity_ref: String,
    /// Policy-pack id.
    pub policy_pack_id: String,
    /// Base policy-pack version.
    pub base_pack_version: u32,
    /// Target policy-pack version.
    pub target_pack_version: u32,
    /// Diff completeness.
    pub diff_completeness_class: String,
    /// Governance decision.
    pub decision_class: String,
    /// Export scope.
    pub export_scope_class: String,
    /// Enterprise-lane class.
    pub enterprise_lane_class: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable governance claim.
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

impl From<StablePolicyPackGovernancePacket> for StablePolicyPackGovernanceProjection {
    fn from(packet: StablePolicyPackGovernancePacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            row_identity_ref: packet.identity.row_identity_ref,
            policy_pack_id: packet.identity.policy_pack_id,
            base_pack_version: packet.diff.base_pack_version,
            target_pack_version: packet.diff.target_pack_version,
            diff_completeness_class: packet.diff.diff_completeness_class,
            decision_class: packet.explain.decision_class,
            export_scope_class: packet.export.export_scope_class,
            enterprise_lane_class: packet.enterprise_lane.enterprise_lane_class,
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
/// Returns [`StablePolicyPackGovernanceError`] when the payload fails to parse or
/// violates the stable governance invariants.
pub fn project_stable_policy_pack_governance(
    payload: &str,
) -> Result<StablePolicyPackGovernanceProjection, StablePolicyPackGovernanceError> {
    let packet: StablePolicyPackGovernancePacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StablePolicyPackGovernanceProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support / partner / mirror export row that quotes the same closed
/// tokens as the packet without leaking raw policy-pack, decision, permission, or
/// publisher-private bytes, and preserves the diff / decision / export posture so a
/// reviewer can see why a row is or is not a stable governance claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePolicyPackGovernanceSupportExport {
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
    /// Governance-profile descriptor ref.
    pub governance_profile_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Policy-pack id.
    pub policy_pack_id: String,
    /// Admin / governance-authority namespace.
    pub admin_namespace: String,
    /// Publisher namespace.
    pub publisher_namespace: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// Base policy-pack version.
    pub base_pack_version: u32,
    /// Target policy-pack version.
    pub target_pack_version: u32,
    /// Diff completeness.
    pub diff_completeness_class: String,
    /// Total changed rules.
    pub changed_rules: u32,
    /// Governance decision.
    pub decision_class: String,
    /// Governance reason.
    pub reason_class: String,
    /// True when the decision is explained.
    pub decision_explained: bool,
    /// Export scope.
    pub export_scope_class: String,
    /// Export source.
    pub export_source_class: String,
    /// Export redaction posture.
    pub export_redaction_class: String,
    /// Enterprise-lane class.
    pub enterprise_lane_class: String,
    /// Lane-claim basis.
    pub lane_claim_basis_class: String,
    /// Tenancy scope.
    pub tenancy_scope_class: String,
    /// Headline activation-cost class.
    pub activation_cost_class: String,
    /// Compatibility label.
    pub compatibility_label_class: String,
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
    /// True when the effective tier blocks the row as a stable governance claim (withdrawn).
    pub blocks_stable_governance: bool,
    /// Export-safe summary suitable for support / partner / mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support / partner / mirror export row.
pub fn project_stable_policy_pack_governance_support_export(
    packet: &StablePolicyPackGovernancePacket,
) -> StablePolicyPackGovernanceSupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Diff={} {}→{} (changed={}). Decision={} reason={} explained={}. Export scope={} source={} redaction={}. Lane={} basis={} tenancy={}. Cost={}. Compatibility={}. Revocation={} mirrorability={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.diff.diff_completeness_class,
        packet.diff.base_pack_version,
        packet.diff.target_pack_version,
        packet.diff.changed_rules(),
        packet.explain.decision_class,
        packet.explain.reason_class,
        packet.explain.decision_explained,
        packet.export.export_scope_class,
        packet.export.export_source_class,
        packet.export.export_redaction_class,
        packet.enterprise_lane.enterprise_lane_class,
        packet.enterprise_lane.lane_claim_basis_class,
        packet.enterprise_lane.tenancy_scope_class,
        packet.install_posture.activation_cost_class,
        packet.compatibility.compatibility_label_class,
        packet.install_posture.revocation_posture_class,
        packet.install_posture.mirrorability_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StablePolicyPackGovernanceSupportExport {
        record_kind: STABLE_POLICY_PACK_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        export_id: format!(
            "stable_policy_pack_governance_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        row_identity_ref: packet.identity.row_identity_ref.clone(),
        governance_profile_ref: packet.identity.governance_profile_ref.clone(),
        extension_identity: packet.identity.extension_identity.clone(),
        extension_version: packet.identity.extension_version.clone(),
        policy_pack_id: packet.identity.policy_pack_id.clone(),
        admin_namespace: packet.identity.admin_namespace.clone(),
        publisher_namespace: packet.identity.publisher_namespace.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        base_pack_version: packet.diff.base_pack_version,
        target_pack_version: packet.diff.target_pack_version,
        diff_completeness_class: packet.diff.diff_completeness_class.clone(),
        changed_rules: packet.diff.changed_rules(),
        decision_class: packet.explain.decision_class.clone(),
        reason_class: packet.explain.reason_class.clone(),
        decision_explained: packet.explain.decision_explained,
        export_scope_class: packet.export.export_scope_class.clone(),
        export_source_class: packet.export.export_source_class.clone(),
        export_redaction_class: packet.export.export_redaction_class.clone(),
        enterprise_lane_class: packet.enterprise_lane.enterprise_lane_class.clone(),
        lane_claim_basis_class: packet.enterprise_lane.lane_claim_basis_class.clone(),
        tenancy_scope_class: packet.enterprise_lane.tenancy_scope_class.clone(),
        activation_cost_class: packet.install_posture.activation_cost_class.clone(),
        compatibility_label_class: packet.compatibility.compatibility_label_class.clone(),
        revocation_posture_class: packet.install_posture.revocation_posture_class.clone(),
        mirrorability_class: packet.install_posture.mirrorability_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_governance: blocks,
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable governance operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StablePolicyPackGovernanceError {
    /// Validation failed.
    Validation(StablePolicyPackGovernanceValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StablePolicyPackGovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StablePolicyPackGovernanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable governance packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StablePolicyPackGovernanceValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StablePolicyPackGovernanceValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StablePolicyPackGovernanceValidationError {}

impl StablePolicyPackGovernanceValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StablePolicyPackGovernanceError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StablePolicyPackGovernanceValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StablePolicyPackGovernanceValidationError> for StablePolicyPackGovernanceError {
    fn from(err: StablePolicyPackGovernanceValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the governance posture.
struct GovernancePosture<'a> {
    identity: &'a PolicyGovernanceIdentity,
    diff: &'a PolicyPackDiff,
    explain: &'a PolicyDecisionExplain,
    export: &'a AdminGovernanceExport,
    enterprise_lane: &'a EnterpriseLaneBinding,
    permission_posture: &'a PolicyGovernancePermissionPosture,
    compatibility: &'a PolicyGovernanceCompatibility,
    install_posture: &'a PolicyGovernanceInstallPosture,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the governance posture.
fn posture_reasons(posture: &GovernancePosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();

    // Identity.
    if !posture.identity.profile_version_current() {
        reasons.push("governance_profile_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        reasons.push("lifecycle_not_runnable".to_string());
    }

    // Diff.
    if posture.diff.missing() {
        reasons.push("diff_missing".to_string());
    } else if posture.diff.partial() {
        reasons.push("diff_partial".to_string());
    }
    if posture.diff.unacknowledged_breaking() {
        reasons.push("diff_unacknowledged_breaking_change".to_string());
    }

    // Explain.
    if !posture.explain.decision_explained {
        reasons.push("decision_not_explained".to_string());
    }

    // Export.
    if posture.export.contains_private() {
        reasons.push("export_contains_private_data".to_string());
    }
    if !posture.export.mechanically_sourced() {
        reasons.push("export_not_mechanically_sourced".to_string());
    }
    if posture.export.scope_limited() {
        reasons.push("export_scope_limited".to_string());
    }

    // Enterprise lane.
    if !posture.enterprise_lane.attested() {
        reasons.push("enterprise_lane_not_attested".to_string());
    }

    // Permissions.
    if posture.permission_posture.widened {
        reasons.push("permission_widened".to_string());
    }

    // Activation cost.
    if posture.install_posture.activation_cost_unbounded() {
        reasons.push("activation_cost_unbounded".to_string());
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

/// Applies the governance posture to a claimed tier, narrowing automatically below
/// Stable when the evidence can no longer back it. The claim basis is folded in
/// separately so a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &GovernancePosture<'_>,
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
        "stable" => "stable_governance_claim",
        "beta" => "beta_governance_partial_claim",
        "preview" => "preview_governance_experimental_claim",
        "withdrawn" => "withdrawn_no_governance_claim",
        _ => "preview_governance_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity, the diff, the explanation, the export, the lane, and
/// compatibility are fully attributed.
fn attribution_is_complete(
    identity: &PolicyGovernanceIdentity,
    diff: &PolicyPackDiff,
    explain: &PolicyDecisionExplain,
    export: &AdminGovernanceExport,
    enterprise_lane: &EnterpriseLaneBinding,
    compatibility: &PolicyGovernanceCompatibility,
) -> bool {
    !identity.governance_profile_ref.trim().is_empty()
        && !identity.row_identity_ref.trim().is_empty()
        && !identity.governance_evidence_ref.trim().is_empty()
        && !identity.admin_namespace.trim().is_empty()
        && !identity.publisher_namespace.trim().is_empty()
        && !diff.diff_ref.trim().is_empty()
        && !explain.governing_rule_ref.trim().is_empty()
        && !explain.explanation_ref.trim().is_empty()
        && !export.export_ref.trim().is_empty()
        && !enterprise_lane.lane_attestation_ref.trim().is_empty()
        && !compatibility.scorecard_ref.trim().is_empty()
}

/// Returns true when the governance posture requires a pre-trust warning banner.
fn governance_requires_warning(posture: &GovernancePosture<'_>) -> bool {
    posture.identity.publisher_trust_tier_class == "quarantined"
        || !posture.identity.lifecycle_runnable()
        || posture.export.contains_private()
        || posture.permission_posture.widened
        || posture.install_posture.activation_cost_unbounded()
        || posture.compatibility.unsupported()
        || matches!(
            posture.install_posture.revocation_posture_class.as_str(),
            "quarantined" | "revoked"
        )
        || posture.diff.unacknowledged_breaking()
}

/// Picks the most-severe banner reason for a row that requires a warning.
fn banner_reason_for(posture: &GovernancePosture<'_>) -> Option<String> {
    if posture.permission_posture.widened {
        return Some("permission_widened".to_string());
    }
    if posture.export.contains_private() {
        return Some("export_contains_private_data".to_string());
    }
    if posture.install_posture.activation_cost_unbounded() {
        return Some("activation_cost_unbounded".to_string());
    }
    if posture.compatibility.unsupported() {
        return Some("compatibility_unsupported".to_string());
    }
    if posture.install_posture.revocation_posture_class == "revoked" {
        return Some("revocation_posture_revoked".to_string());
    }
    if posture.install_posture.revocation_posture_class == "quarantined" {
        return Some("revocation_posture_quarantined".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        return Some("lifecycle_not_runnable".to_string());
    }
    if posture.diff.unacknowledged_breaking() {
        return Some("diff_unacknowledged_breaking_change".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &PolicyGovernanceIdentityInput) -> PolicyGovernanceIdentity {
    PolicyGovernanceIdentity {
        record_kind: POLICY_GOVERNANCE_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        governance_profile_ref: input.governance_profile_ref.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        extension_identity: input.extension_identity.clone(),
        extension_version: input.extension_version.clone(),
        package_id: input.package_id.clone(),
        policy_pack_id: input.policy_pack_id.clone(),
        policy_pack_version: input.policy_pack_version,
        governance_profile_version: input.governance_profile_version,
        admin_namespace: input.admin_namespace.clone(),
        publisher_namespace: input.publisher_namespace.clone(),
        governance_evidence_ref: input.governance_evidence_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn diff_record(input: &PolicyPackDiffInput) -> PolicyPackDiff {
    PolicyPackDiff {
        record_kind: POLICY_PACK_DIFF_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        base_pack_version: input.base_pack_version,
        target_pack_version: input.target_pack_version,
        diff_completeness_class: input.diff_completeness_class.clone(),
        rules_added: input.rules_added,
        rules_removed: input.rules_removed,
        rules_modified: input.rules_modified,
        breaking_change: input.breaking_change,
        breaking_change_acknowledged: input.breaking_change_acknowledged,
        diff_ref: input.diff_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn explain_record(input: &PolicyDecisionExplainInput) -> PolicyDecisionExplain {
    PolicyDecisionExplain {
        record_kind: POLICY_DECISION_EXPLAIN_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        decision_class: input.decision_class.clone(),
        reason_class: input.reason_class.clone(),
        governing_rule_ref: input.governing_rule_ref.clone(),
        decision_explained: input.decision_explained,
        explanation_ref: input.explanation_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn export_record(input: &AdminGovernanceExportInput) -> AdminGovernanceExport {
    AdminGovernanceExport {
        record_kind: ADMIN_GOVERNANCE_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        export_scope_class: input.export_scope_class.clone(),
        export_source_class: input.export_source_class.clone(),
        export_redaction_class: input.export_redaction_class.clone(),
        export_ref: input.export_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn enterprise_lane_record(input: &EnterpriseLaneBindingInput) -> EnterpriseLaneBinding {
    EnterpriseLaneBinding {
        record_kind: ENTERPRISE_LANE_BINDING_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        enterprise_lane_class: input.enterprise_lane_class.clone(),
        lane_claim_basis_class: input.lane_claim_basis_class.clone(),
        tenancy_scope_class: input.tenancy_scope_class.clone(),
        lane_attestation_ref: input.lane_attestation_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn permission_posture_record(
    input: &PolicyGovernancePermissionPostureInput,
) -> PolicyGovernancePermissionPosture {
    PolicyGovernancePermissionPosture {
        record_kind: POLICY_GOVERNANCE_PERMISSION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        declared_permission_ref: input.declared_permission_ref.clone(),
        effective_permission_ref: input.effective_permission_ref.clone(),
        policy_cap_ref: input.policy_cap_ref.clone(),
        widened: input.widened,
        reconsent_required: input.reconsent_required,
        summary_label: input.summary_label.clone(),
    }
}

fn compatibility_record(
    input: &PolicyGovernanceCompatibilityInput,
) -> PolicyGovernanceCompatibility {
    PolicyGovernanceCompatibility {
        record_kind: POLICY_GOVERNANCE_COMPATIBILITY_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        compatibility_label_class: input.compatibility_label_class.clone(),
        scorecard_ref: input.scorecard_ref.clone(),
        compatibility_verified: input.compatibility_verified,
        summary_label: input.summary_label.clone(),
    }
}

fn install_posture_record(
    input: &PolicyGovernanceInstallPostureInput,
) -> PolicyGovernanceInstallPosture {
    PolicyGovernanceInstallPosture {
        record_kind: POLICY_GOVERNANCE_INSTALL_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        install_scope_class: input.install_scope_class.clone(),
        install_scope_disclosed: input.install_scope_disclosed,
        activation_cost_class: input.activation_cost_class.clone(),
        revocation_posture_class: input.revocation_posture_class.clone(),
        mirrorability_class: input.mirrorability_class.clone(),
        rollback_supported: input.rollback_supported,
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &PolicyGovernanceQualificationClaimInput,
    posture: &GovernancePosture<'_>,
) -> PolicyGovernanceQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    PolicyGovernanceQualificationClaim {
        record_kind: POLICY_GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &GovernancePosture<'_>) -> PolicyGovernanceDowngradedBanner {
    let must_display = governance_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Governance row requires admin review before relying on the policy decision ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Governance hardened: complete diff, explained decision, mechanically-generated metadata-safe export, attested enterprise lane."
            .to_string()
    };
    PolicyGovernanceDowngradedBanner {
        record_kind: POLICY_GOVERNANCE_DOWNGRADED_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &GovernancePosture<'_>,
    claim: &PolicyGovernanceQualificationClaim,
    banner: &PolicyGovernanceDowngradedBanner,
) -> StablePolicyPackGovernanceInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());

    StablePolicyPackGovernanceInspection {
        record_kind: STABLE_POLICY_PACK_GOVERNANCE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        profile_version_current: posture.identity.profile_version_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: posture.identity.lifecycle_runnable(),
        base_pack_version: posture.diff.base_pack_version,
        target_pack_version: posture.diff.target_pack_version,
        diff_completeness_class: posture.diff.diff_completeness_class.clone(),
        diff_complete: posture.diff.complete(),
        decision_class: posture.explain.decision_class.clone(),
        decision_explained: posture.explain.decision_explained,
        export_scope_class: posture.export.export_scope_class.clone(),
        export_mechanically_sourced: posture.export.mechanically_sourced(),
        export_redaction_class: posture.export.export_redaction_class.clone(),
        enterprise_lane_class: posture.enterprise_lane.enterprise_lane_class.clone(),
        enterprise_lane_attested: posture.enterprise_lane.attested(),
        permissions_not_widened: !posture.permission_posture.widened,
        activation_cost_class: posture.install_posture.activation_cost_class.clone(),
        compatibility_label_class: posture.compatibility.compatibility_label_class.clone(),
        compatibility_verified: posture.compatibility.compatibility_verified,
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
    input: &StablePolicyPackGovernanceInput,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(
        &id.governance_profile_ref,
        "identity.governance_profile_ref",
    )?;
    if !id.governance_profile_ref.starts_with("governance_profile:") {
        return Err(err(
            "identity.governance_profile_ref must start with 'governance_profile:'",
        ));
    }
    ensure_nonempty(&id.row_identity_ref, "identity.row_identity_ref")?;
    ensure_nonempty(&id.extension_identity, "identity.extension_identity")?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.package_id, "identity.package_id")?;
    ensure_nonempty(&id.policy_pack_id, "identity.policy_pack_id")?;
    ensure_nonempty(&id.admin_namespace, "identity.admin_namespace")?;
    ensure_nonempty(&id.publisher_namespace, "identity.publisher_namespace")?;
    ensure_nonempty(
        &id.governance_evidence_ref,
        "identity.governance_evidence_ref",
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

    let d = &input.diff;
    ensure_token(
        DIFF_COMPLETENESS_CLASSES,
        &d.diff_completeness_class,
        "diff.diff_completeness_class",
    )?;
    ensure_nonempty(&d.diff_ref, "diff.diff_ref")?;

    let ex = &input.explain;
    ensure_token(
        DECISION_CLASSES,
        &ex.decision_class,
        "explain.decision_class",
    )?;
    ensure_token(
        DECISION_REASON_CLASSES,
        &ex.reason_class,
        "explain.reason_class",
    )?;
    ensure_nonempty(&ex.governing_rule_ref, "explain.governing_rule_ref")?;
    ensure_nonempty(&ex.explanation_ref, "explain.explanation_ref")?;

    let exp = &input.export;
    ensure_token(
        EXPORT_SCOPE_CLASSES,
        &exp.export_scope_class,
        "export.export_scope_class",
    )?;
    ensure_token(
        EXPORT_SOURCE_CLASSES,
        &exp.export_source_class,
        "export.export_source_class",
    )?;
    ensure_token(
        EXPORT_REDACTION_CLASSES,
        &exp.export_redaction_class,
        "export.export_redaction_class",
    )?;
    ensure_nonempty(&exp.export_ref, "export.export_ref")?;

    let lane = &input.enterprise_lane;
    ensure_token(
        ENTERPRISE_LANE_CLASSES,
        &lane.enterprise_lane_class,
        "enterprise_lane.enterprise_lane_class",
    )?;
    ensure_token(
        LANE_CLAIM_BASIS_CLASSES,
        &lane.lane_claim_basis_class,
        "enterprise_lane.lane_claim_basis_class",
    )?;
    ensure_token(
        TENANCY_SCOPE_CLASSES,
        &lane.tenancy_scope_class,
        "enterprise_lane.tenancy_scope_class",
    )?;
    ensure_nonempty(
        &lane.lane_attestation_ref,
        "enterprise_lane.lane_attestation_ref",
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
    ensure_nonempty(&perm.policy_cap_ref, "permission_posture.policy_cap_ref")?;

    let compat = &input.compatibility;
    ensure_token(
        COMPATIBILITY_LABEL_CLASSES,
        &compat.compatibility_label_class,
        "compatibility.compatibility_label_class",
    )?;
    ensure_nonempty(&compat.scorecard_ref, "compatibility.scorecard_ref")?;

    let inst = &input.install_posture;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &inst.install_scope_class,
        "install_posture.install_scope_class",
    )?;
    ensure_token(
        ACTIVATION_COST_CLASSES,
        &inst.activation_cost_class,
        "install_posture.activation_cost_class",
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
            STABLE_POLICY_PACK_GOVERNANCE_CONSUMER_SURFACES,
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
    identity: &PolicyGovernanceIdentity,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        POLICY_GOVERNANCE_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_POLICY_PACK_GOVERNANCE_SCHEMA_VERSION,
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

fn validate_diff(d: &PolicyPackDiff) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        d.record_kind.as_str(),
        POLICY_PACK_DIFF_RECORD_KIND,
        "diff record_kind",
    )?;
    ensure_token(
        DIFF_COMPLETENESS_CLASSES,
        &d.diff_completeness_class,
        "diff diff_completeness_class",
    )?;
    ensure_nonempty(&d.diff_ref, "diff diff_ref")?;
    Ok(())
}

fn validate_explain(
    ex: &PolicyDecisionExplain,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        ex.record_kind.as_str(),
        POLICY_DECISION_EXPLAIN_RECORD_KIND,
        "explain record_kind",
    )?;
    ensure_token(
        DECISION_CLASSES,
        &ex.decision_class,
        "explain decision_class",
    )?;
    ensure_token(
        DECISION_REASON_CLASSES,
        &ex.reason_class,
        "explain reason_class",
    )?;
    ensure_nonempty(&ex.governing_rule_ref, "explain governing_rule_ref")?;
    ensure_nonempty(&ex.explanation_ref, "explain explanation_ref")?;
    Ok(())
}

fn validate_export(
    exp: &AdminGovernanceExport,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        exp.record_kind.as_str(),
        ADMIN_GOVERNANCE_EXPORT_RECORD_KIND,
        "export record_kind",
    )?;
    ensure_token(
        EXPORT_SCOPE_CLASSES,
        &exp.export_scope_class,
        "export export_scope_class",
    )?;
    ensure_token(
        EXPORT_SOURCE_CLASSES,
        &exp.export_source_class,
        "export export_source_class",
    )?;
    ensure_token(
        EXPORT_REDACTION_CLASSES,
        &exp.export_redaction_class,
        "export export_redaction_class",
    )?;
    ensure_nonempty(&exp.export_ref, "export export_ref")?;
    Ok(())
}

fn validate_enterprise_lane(
    lane: &EnterpriseLaneBinding,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        lane.record_kind.as_str(),
        ENTERPRISE_LANE_BINDING_RECORD_KIND,
        "enterprise_lane record_kind",
    )?;
    ensure_token(
        ENTERPRISE_LANE_CLASSES,
        &lane.enterprise_lane_class,
        "enterprise_lane enterprise_lane_class",
    )?;
    ensure_token(
        LANE_CLAIM_BASIS_CLASSES,
        &lane.lane_claim_basis_class,
        "enterprise_lane lane_claim_basis_class",
    )?;
    ensure_token(
        TENANCY_SCOPE_CLASSES,
        &lane.tenancy_scope_class,
        "enterprise_lane tenancy_scope_class",
    )?;
    ensure_nonempty(
        &lane.lane_attestation_ref,
        "enterprise_lane lane_attestation_ref",
    )?;
    Ok(())
}

fn validate_permission_posture(
    perm: &PolicyGovernancePermissionPosture,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        perm.record_kind.as_str(),
        POLICY_GOVERNANCE_PERMISSION_POSTURE_RECORD_KIND,
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
    ensure_nonempty(&perm.policy_cap_ref, "permission_posture policy_cap_ref")?;
    Ok(())
}

fn validate_compatibility(
    compat: &PolicyGovernanceCompatibility,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        compat.record_kind.as_str(),
        POLICY_GOVERNANCE_COMPATIBILITY_RECORD_KIND,
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

fn validate_install_posture(
    inst: &PolicyGovernanceInstallPosture,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        inst.record_kind.as_str(),
        POLICY_GOVERNANCE_INSTALL_POSTURE_RECORD_KIND,
        "install_posture record_kind",
    )?;
    ensure_token(
        INSTALL_SCOPE_CLASSES,
        &inst.install_scope_class,
        "install_posture install_scope_class",
    )?;
    ensure_token(
        ACTIVATION_COST_CLASSES,
        &inst.activation_cost_class,
        "install_posture activation_cost_class",
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
    claim: &PolicyGovernanceQualificationClaim,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        POLICY_GOVERNANCE_QUALIFICATION_CLAIM_RECORD_KIND,
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
            POLICY_PACK_GOVERNANCE_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &PolicyGovernanceDowngradedBanner,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        POLICY_GOVERNANCE_DOWNGRADED_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            POLICY_PACK_GOVERNANCE_DOWNGRADE_REASONS,
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

/// Cross-checks the diff counts against the diff completeness and base / target
/// versions so a diff record cannot be internally inconsistent.
fn validate_diff_counts(
    d: &PolicyPackDiff,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    if d.target_pack_version < d.base_pack_version {
        return Err(err(
            "diff target_pack_version must not precede base_pack_version",
        ));
    }
    if d.breaking_change && d.rules_removed == 0 && d.rules_modified == 0 {
        return Err(err(
            "a breaking_change diff must remove or modify at least one rule",
        ));
    }
    match d.diff_completeness_class.as_str() {
        "complete" => {
            // A complete diff with no changed rules must be an identity diff
            // (base == target); otherwise the counts are missing.
            if d.changed_rules() == 0 && d.base_pack_version != d.target_pack_version {
                return Err(err(
                    "a complete diff across differing pack versions must report at least one changed rule",
                ));
            }
        }
        "missing" => {
            if d.changed_rules() != 0 {
                return Err(err("a missing diff must not report changed rules"));
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_inspection(
    inspection: &StablePolicyPackGovernanceInspection,
    packet: &StablePolicyPackGovernancePacket,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_POLICY_PACK_GOVERNANCE_INSPECTION_RECORD_KIND,
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
    if inspection.diff_complete != packet.diff.complete() {
        return Err(err("inspection diff_complete is inconsistent"));
    }
    if inspection.export_mechanically_sourced != packet.export.mechanically_sourced() {
        return Err(err(
            "inspection export_mechanically_sourced is inconsistent",
        ));
    }
    if inspection.enterprise_lane_attested != packet.enterprise_lane.attested() {
        return Err(err("inspection enterprise_lane_attested is inconsistent"));
    }
    if inspection.permissions_not_widened == packet.permission_posture.widened {
        return Err(err("inspection permissions_not_widened is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StablePolicyPackGovernanceValidationError {
    StablePolicyPackGovernanceValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StablePolicyPackGovernanceValidationError>
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
) -> Result<(), StablePolicyPackGovernanceValidationError> {
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
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StablePolicyPackGovernanceValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
