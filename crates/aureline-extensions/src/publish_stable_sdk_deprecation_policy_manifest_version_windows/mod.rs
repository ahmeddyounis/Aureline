//! Publish the stable SDK / deprecation policy, manifest version windows, and
//! ecosystem migration guidance for a claimed stable extension row — one
//! evidence-backed, automatically-narrowing packet whose stability qualification is
//! derived, not asserted.
//!
//! The beta install-review, runtime, marketplace-catalog, and mirror-import lanes own
//! per-row admission, catalog truth, and import truth. The stable runtime-ABI,
//! manifest, lifecycle-flow, catalog-truth, bridge-certification, performance-budget,
//! and policy-pack-governance lanes own the published truth a claimed stable row
//! carries on each of those axes. This module owns the layer that makes the
//! **SDK / API deprecation policy**, the **manifest version windows**, and the
//! **ecosystem migration guidance** of an extension on a claimed stable row
//! inspectable across every surface an author, admin, or user reaches:
//!
//! - the **SDK / deprecation policy** — the SDK deprecation stage
//!   (`active` / `deprecated` / `sunset` / `removed`), the last-supported version
//!   window, the replacement package or API (or an explicit
//!   `superseded_no_replacement`), whether pinning to a last-known-good version
//!   remains allowed by policy (`pin_allowed` / `pin_discouraged` / `pin_blocked`),
//!   and the named affected dependency edges, so a deprecation packet is never a
//!   release-note-only afterthought,
//! - the **deprecation propagation** — whether the deprecation actually flows into
//!   install-time warnings, marketplace cards, dependency-resolution output, the
//!   migration docs, and a compatibility shim, so a deprecation can never live only
//!   in release notes,
//! - the **manifest version window** — the min / max supported manifest version, the
//!   published manifest version, and the row's own manifest version, so a manifest
//!   version that falls outside the supported window is visible before install /
//!   upgrade / mirror promotion,
//! - the **ecosystem migration guidance** — the migration outcome label
//!   (`exact` / `translated` / `partial` / `shimmed` / `unsupported`) generated from
//!   the real imported artifact, the migration-doc ref, the compatibility-shim
//!   availability, and whether a rollback checkpoint and diagnostics were preserved
//!   when a mapping was partial / shimmed / unsupported,
//! - the **permission posture** (declared / effective / policy-cap refs and a
//!   no-widening flag) so no ambient privilege rides the SDK-policy claim,
//! - the **compatibility** label (scorecard ref, verified flag),
//! - the **install posture** (install scope and disclosure, activation cost class,
//!   revocation posture, mirrorability, rollback support), and
//! - the **stability qualification** after the posture is applied.
//!
//! The central rule mirrors the rest of the stable line: a **stable** SDK-policy
//! claim may never be implied from a catalog row or an adjacent green row. A row that
//! renders a `stable` badge must pin the published SDK-policy profile version, be
//! evidence-backed (not catalog-asserted), keep its publisher trust tier out of
//! quarantine, stay on a runnable lifecycle, keep its SDK out of the sunset window and
//! out of removal, name a replacement and a last-supported window and the affected
//! dependency edges whenever the SDK is deprecated, propagate that deprecation into
//! the install warning / marketplace card / dependency-resolution output and the
//! migration docs, keep the row's manifest version inside the supported window,
//! surface migration guidance that is `exact` / `translated` / `shimmed` (never
//! `partial` / `unsupported`) and preserve a rollback checkpoint when a mapping is not
//! exact, never widen permissions beyond the declared manifest or the policy cap, keep
//! its activation cost bounded, keep its compatibility verified and not parity-limited
//! / unsupported, disclose its install scope, keep a clean revocation posture, stay
//! mirrorable, and be fully attributed. When any of those fails, the visible tier is
//! **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with
//! machine-readable reasons.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No unbounded activation cost.** An `unbounded` activation-cost class withdraws
//!   the row outright.
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never back a
//!   stable SDK-policy claim; it narrows below Stable.
//! - **No ambient extension privilege.** A permission set widened beyond the declared
//!   manifest or the policy cap withdraws the row outright.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_sdk_deprecation_policy.schema.json`](../../../../schemas/extensions/stable_sdk_deprecation_policy.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/publish-stable-sdk-deprecation-policy-manifest-version-windows/`.

use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable SDK / deprecation policy record.
pub const STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION: u32 = 1;

/// The published, stable SDK-policy profile version. A `stable` claim must pin exactly
/// this version; any other version narrows below Stable.
pub const STABLE_SDK_DEPRECATION_POLICY_PUBLISHED_PROFILE_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_SDK_DEPRECATION_POLICY_SCHEMA_REF: &str =
    "schemas/extensions/stable_sdk_deprecation_policy.schema.json";

/// Record-kind tag for [`StableSdkDeprecationPolicyPacket`].
pub const STABLE_SDK_DEPRECATION_POLICY_PACKET_RECORD_KIND: &str =
    "stable_sdk_deprecation_policy_packet";

/// Record-kind tag for [`SdkPolicyIdentity`].
pub const SDK_POLICY_IDENTITY_RECORD_KIND: &str = "stable_sdk_policy_identity";

/// Record-kind tag for [`SdkDeprecationPolicy`].
pub const SDK_DEPRECATION_POLICY_RECORD_KIND: &str = "stable_sdk_deprecation_policy";

/// Record-kind tag for [`DeprecationPropagation`].
pub const DEPRECATION_PROPAGATION_RECORD_KIND: &str = "stable_deprecation_propagation";

/// Record-kind tag for [`ManifestVersionWindow`].
pub const MANIFEST_VERSION_WINDOW_RECORD_KIND: &str = "stable_manifest_version_window";

/// Record-kind tag for [`EcosystemMigrationGuidance`].
pub const ECOSYSTEM_MIGRATION_GUIDANCE_RECORD_KIND: &str = "stable_ecosystem_migration_guidance";

/// Record-kind tag for [`SdkPolicyPermissionPosture`].
pub const SDK_POLICY_PERMISSION_POSTURE_RECORD_KIND: &str = "stable_sdk_policy_permission_posture";

/// Record-kind tag for [`SdkPolicyCompatibility`].
pub const SDK_POLICY_COMPATIBILITY_RECORD_KIND: &str = "stable_sdk_policy_compatibility";

/// Record-kind tag for [`SdkPolicyInstallPosture`].
pub const SDK_POLICY_INSTALL_POSTURE_RECORD_KIND: &str = "stable_sdk_policy_install_posture";

/// Record-kind tag for [`SdkPolicyQualificationClaim`].
pub const SDK_POLICY_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_sdk_policy_qualification_claim";

/// Record-kind tag for [`SdkPolicyDowngradedBanner`].
pub const SDK_POLICY_DOWNGRADED_BANNER_RECORD_KIND: &str = "stable_sdk_policy_downgraded_banner";

/// Record-kind tag for [`StableSdkDeprecationPolicyInspection`].
pub const STABLE_SDK_DEPRECATION_POLICY_INSPECTION_RECORD_KIND: &str =
    "stable_sdk_deprecation_policy_inspection";

/// Record-kind tag for [`StableSdkDeprecationPolicySupportExport`].
pub const STABLE_SDK_DEPRECATION_POLICY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_sdk_deprecation_policy_support_export";

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

/// Lifecycle states a stable SDK-policy claim may keep (installable / runnable).
pub const RUNNABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed SDK deprecation-stage vocabulary. `active` and `deprecated` are the only
/// stages a stable claim may keep; `sunset` narrows to `beta`, `removed` withdraws.
pub const DEPRECATION_STAGE_CLASSES: &[&str] = &["active", "deprecated", "sunset", "removed"];

/// Closed replacement-kind vocabulary. `none` (an unspecified replacement) narrows a
/// deprecated row below Stable; `superseded_no_replacement` is an explicit answer that
/// may ride Stable.
pub const REPLACEMENT_KIND_CLASSES: &[&str] = &[
    "replacement_package",
    "replacement_api",
    "superseded_no_replacement",
    "none",
];

/// Closed pin-policy vocabulary — whether pinning to a last-known-good version remains
/// allowed by policy.
pub const PIN_POLICY_CLASSES: &[&str] = &["pin_allowed", "pin_discouraged", "pin_blocked"];

/// Closed migration-outcome vocabulary generated from the real imported artifact.
/// `partial` narrows to `beta`; `unsupported` withdraws.
pub const MIGRATION_OUTCOME_CLASSES: &[&str] =
    &["exact", "translated", "partial", "shimmed", "unsupported"];

/// Closed compatibility-shim-availability vocabulary.
pub const SHIM_AVAILABILITY_CLASSES: &[&str] =
    &["no_shim_needed", "shim_available", "shim_unavailable"];

/// Closed activation-cost vocabulary — the headline cost the row carries.
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

/// Tiers that count as a *stable* SDK-policy claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_sdk_policy_claim",
    "beta_sdk_policy_partial_claim",
    "preview_sdk_policy_experimental_claim",
    "withdrawn_no_sdk_policy_claim",
];

/// Closed set of reasons that narrow a stable SDK-policy claim below Stable.
pub const SDK_DEPRECATION_POLICY_DOWNGRADE_REASONS: &[&str] = &[
    "sdk_policy_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_runnable",
    "deprecation_stage_removed",
    "deprecation_in_sunset_window",
    "replacement_path_missing",
    "last_supported_window_missing",
    "affected_dependency_edges_unnamed",
    "deprecation_propagation_incomplete",
    "migration_docs_missing",
    "manifest_version_out_of_window",
    "migration_outcome_unsupported",
    "migration_outcome_partial",
    "rollback_checkpoint_not_preserved",
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

/// Reasons that narrow all the way to `withdrawn` (the row cannot be trusted as a
/// stable SDK-policy claim at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_runnable",
    "deprecation_stage_removed",
    "migration_outcome_unsupported",
    "permission_widened",
    "activation_cost_unbounded",
    "compatibility_unsupported",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
];

/// Reasons that narrow to `preview` (a structural / disclosure / evidence shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "sdk_policy_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "replacement_path_missing",
    "last_supported_window_missing",
    "migration_docs_missing",
    "manifest_version_out_of_window",
    "rollback_checkpoint_not_preserved",
    "compatibility_not_verified",
    "install_scope_not_disclosed",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "deprecation_in_sunset_window",
    "affected_dependency_edges_unnamed",
    "deprecation_propagation_incomplete",
    "migration_outcome_partial",
    "compatibility_parity_limited",
    "revocation_posture_advisory",
    "not_mirrorable",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_SDK_DEPRECATION_POLICY_CONSUMER_SURFACES: &[&str] = &[
    "sdk_migration_console",
    "deprecation_policy_view",
    "dependency_resolution_output",
    "install_review",
    "marketplace_card",
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

/// Input describing a stable SDK / deprecation policy packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkDeprecationPolicyInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: SdkPolicyIdentityInput,
    /// SDK / deprecation policy input.
    pub deprecation: SdkDeprecationPolicyInput,
    /// Deprecation-propagation input.
    pub propagation: DeprecationPropagationInput,
    /// Manifest-version-window input.
    pub manifest_window: ManifestVersionWindowInput,
    /// Ecosystem-migration-guidance input.
    pub migration: EcosystemMigrationGuidanceInput,
    /// Permission-posture input.
    pub permission_posture: SdkPolicyPermissionPostureInput,
    /// Compatibility input.
    pub compatibility: SdkPolicyCompatibilityInput,
    /// Install-posture input.
    pub install_posture: SdkPolicyInstallPostureInput,
    /// Stability qualification claim input.
    pub claim: SdkPolicyQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`SdkPolicyIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyIdentityInput {
    /// Ref to the SDK-policy descriptor this row stabilizes.
    pub sdk_policy_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// SDK release channel id the row tracks.
    pub sdk_channel_id: String,
    /// SDK-policy version the active deprecation decision was taken under.
    pub sdk_policy_version: u32,
    /// Published SDK-policy profile version this row pins.
    pub published_policy_version: u32,
    /// Publisher namespace the extension asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the pinned SDK-policy evidence bundle.
    pub policy_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`SdkDeprecationPolicy`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkDeprecationPolicyInput {
    /// SDK deprecation stage.
    pub deprecation_stage_class: String,
    /// Last-supported SDK version (empty while the SDK is `active`).
    pub last_supported_version: String,
    /// Replacement package / API kind.
    pub replacement_kind_class: String,
    /// Ref to the replacement package or API (empty when none / not yet deprecated).
    pub replacement_ref: String,
    /// Whether pinning to a last-known-good version remains allowed by policy.
    pub pin_policy_class: String,
    /// Ref to the published support-window record (empty while `active`).
    pub support_window_ref: String,
    /// Number of dependency edges affected by the deprecation.
    pub affected_dependency_edge_count: u32,
    /// Ref to the named affected-dependency-edge list.
    pub dependency_edges_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`DeprecationPropagation`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecationPropagationInput {
    /// Whether the deprecation surfaces in install-time warnings.
    pub surfaces_in_install_warning: bool,
    /// Whether the deprecation surfaces on the marketplace card.
    pub surfaces_in_marketplace_card: bool,
    /// Whether the deprecation surfaces in dependency-resolution output.
    pub surfaces_in_dependency_resolution: bool,
    /// Whether the deprecation surfaces in the migration docs.
    pub surfaces_in_migration_docs: bool,
    /// Whether a compatibility shim is wired for the deprecated surface.
    pub surfaces_in_compat_shim: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ManifestVersionWindow`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestVersionWindowInput {
    /// Minimum supported manifest version.
    pub min_supported_manifest_version: u32,
    /// Maximum supported manifest version.
    pub max_supported_manifest_version: u32,
    /// Published (current) manifest version.
    pub published_manifest_version: u32,
    /// The row's own manifest version.
    pub row_manifest_version: u32,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`EcosystemMigrationGuidance`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemMigrationGuidanceInput {
    /// Migration outcome generated from the real imported artifact.
    pub migration_outcome_class: String,
    /// Ref to the migration docs.
    pub migration_doc_ref: String,
    /// Compatibility-shim availability.
    pub shim_availability_class: String,
    /// Whether a rollback checkpoint was preserved for a non-exact mapping.
    pub rollback_checkpoint_preserved: bool,
    /// Ref to the migration diagnostics bundle.
    pub diagnostics_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`SdkPolicyPermissionPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyPermissionPostureInput {
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after resolution.
    pub effective_permission_ref: String,
    /// Ref to the policy permission cap applied to the row.
    pub policy_cap_ref: String,
    /// Whether authority was widened beyond the declared set or the policy cap.
    pub widened: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`SdkPolicyCompatibility`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyCompatibilityInput {
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the compatibility scorecard.
    pub scorecard_ref: String,
    /// Whether compatibility was verified.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`SdkPolicyInstallPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyInstallPostureInput {
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Headline activation-cost class for the row.
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

/// Input for [`SdkPolicyQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyQualificationClaimInput {
    /// SDK-policy tier claimed by the row.
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
pub struct SdkPolicyIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the SDK-policy descriptor this row stabilizes.
    pub sdk_policy_ref: String,
    /// Opaque marketplace / catalog row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// SDK release channel id the row tracks.
    pub sdk_channel_id: String,
    /// SDK-policy version the active deprecation decision was taken under.
    pub sdk_policy_version: u32,
    /// Published SDK-policy profile version this row pins.
    pub published_policy_version: u32,
    /// Publisher namespace.
    pub publisher_namespace: String,
    /// Ref to the pinned SDK-policy evidence bundle.
    pub policy_evidence_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl SdkPolicyIdentity {
    /// Returns true when the row pins the published SDK-policy profile version.
    pub fn profile_version_current(&self) -> bool {
        self.published_policy_version == STABLE_SDK_DEPRECATION_POLICY_PUBLISHED_PROFILE_VERSION
    }

    /// Returns true when the lifecycle is runnable.
    pub fn lifecycle_runnable(&self) -> bool {
        RUNNABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// SDK / API deprecation policy carried on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkDeprecationPolicy {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// SDK deprecation stage.
    pub deprecation_stage_class: String,
    /// Last-supported SDK version (empty while `active`).
    pub last_supported_version: String,
    /// Replacement package / API kind.
    pub replacement_kind_class: String,
    /// Ref to the replacement package or API.
    pub replacement_ref: String,
    /// Whether pinning to a last-known-good version remains allowed by policy.
    pub pin_policy_class: String,
    /// Ref to the published support-window record.
    pub support_window_ref: String,
    /// Number of dependency edges affected by the deprecation.
    pub affected_dependency_edge_count: u32,
    /// Ref to the named affected-dependency-edge list.
    pub dependency_edges_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl SdkDeprecationPolicy {
    /// Returns true when the SDK is current (no deprecation in force).
    pub fn active(&self) -> bool {
        self.deprecation_stage_class == "active"
    }

    /// Returns true when the SDK is deprecated, sunsetting, or removed.
    pub fn deprecated_or_later(&self) -> bool {
        matches!(
            self.deprecation_stage_class.as_str(),
            "deprecated" | "sunset" | "removed"
        )
    }

    /// Returns true when the SDK is in the sunset window.
    pub fn in_sunset(&self) -> bool {
        self.deprecation_stage_class == "sunset"
    }

    /// Returns true when the SDK has been removed.
    pub fn removed(&self) -> bool {
        self.deprecation_stage_class == "removed"
    }

    /// Returns true when a replacement package / API (or explicit supersession) is
    /// named.
    pub fn replacement_named(&self) -> bool {
        self.replacement_kind_class != "none"
    }

    /// Returns true when the last-supported version window is named.
    pub fn last_supported_named(&self) -> bool {
        !self.last_supported_version.trim().is_empty()
    }

    /// Returns true when the affected dependency edges are named.
    pub fn dependency_edges_named(&self) -> bool {
        self.affected_dependency_edge_count > 0 && !self.dependency_edges_ref.trim().is_empty()
    }
}

/// Whether the deprecation actually flows into every required surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecationPropagation {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Whether the deprecation surfaces in install-time warnings.
    pub surfaces_in_install_warning: bool,
    /// Whether the deprecation surfaces on the marketplace card.
    pub surfaces_in_marketplace_card: bool,
    /// Whether the deprecation surfaces in dependency-resolution output.
    pub surfaces_in_dependency_resolution: bool,
    /// Whether the deprecation surfaces in the migration docs.
    pub surfaces_in_migration_docs: bool,
    /// Whether a compatibility shim is wired for the deprecated surface.
    pub surfaces_in_compat_shim: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl DeprecationPropagation {
    /// Returns true when the deprecation reaches install warnings, the marketplace
    /// card, and dependency-resolution output.
    pub fn core_propagation_complete(&self) -> bool {
        self.surfaces_in_install_warning
            && self.surfaces_in_marketplace_card
            && self.surfaces_in_dependency_resolution
    }
}

/// Manifest version window the row's manifest must sit inside.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestVersionWindow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Minimum supported manifest version.
    pub min_supported_manifest_version: u32,
    /// Maximum supported manifest version.
    pub max_supported_manifest_version: u32,
    /// Published (current) manifest version.
    pub published_manifest_version: u32,
    /// The row's own manifest version.
    pub row_manifest_version: u32,
    /// Reviewable summary.
    pub summary_label: String,
}

impl ManifestVersionWindow {
    /// Returns true when the row's manifest version sits inside the supported window.
    pub fn within_window(&self) -> bool {
        self.row_manifest_version >= self.min_supported_manifest_version
            && self.row_manifest_version <= self.max_supported_manifest_version
    }
}

/// Ecosystem migration guidance derived from the real imported artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemMigrationGuidance {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Migration outcome.
    pub migration_outcome_class: String,
    /// Ref to the migration docs.
    pub migration_doc_ref: String,
    /// Compatibility-shim availability.
    pub shim_availability_class: String,
    /// Whether a rollback checkpoint was preserved for a non-exact mapping.
    pub rollback_checkpoint_preserved: bool,
    /// Ref to the migration diagnostics bundle.
    pub diagnostics_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

impl EcosystemMigrationGuidance {
    /// Returns true when migration cannot be carried at all.
    pub fn outcome_unsupported(&self) -> bool {
        self.migration_outcome_class == "unsupported"
    }

    /// Returns true when migration is only partial.
    pub fn outcome_partial(&self) -> bool {
        self.migration_outcome_class == "partial"
    }

    /// Returns true when migration is bridged through a compatibility shim.
    pub fn shimmed(&self) -> bool {
        self.migration_outcome_class == "shimmed"
    }

    /// Returns true when a mapping was not exact and therefore must preserve a
    /// rollback checkpoint and diagnostics.
    pub fn needs_rollback_checkpoint(&self) -> bool {
        matches!(
            self.migration_outcome_class.as_str(),
            "partial" | "shimmed" | "unsupported"
        )
    }
}

/// Permission posture for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyPermissionPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after resolution.
    pub effective_permission_ref: String,
    /// Ref to the policy permission cap.
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
pub struct SdkPolicyCompatibility {
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

impl SdkPolicyCompatibility {
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
pub struct SdkPolicyInstallPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Headline activation-cost class for the row.
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

impl SdkPolicyInstallPosture {
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
pub struct SdkPolicyQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// SDK-policy tier claimed by the row.
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

/// Downgraded-row banner requirement. Raised whenever an admin / author / user must see
/// an SDK-policy shortfall before relying on the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SdkPolicyDowngradedBanner {
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
pub struct StableSdkDeprecationPolicyInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective SDK-policy tier.
    pub effective_tier: String,
    /// True when the claim is a stable SDK-policy claim.
    pub stable_claim: bool,
    /// True when the row pins the published profile version.
    pub profile_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// True when the lifecycle is runnable.
    pub lifecycle_runnable: bool,
    /// SDK deprecation stage.
    pub deprecation_stage_class: String,
    /// True when a replacement package / API is named (or explicit supersession).
    pub replacement_named: bool,
    /// Pin policy.
    pub pin_policy_class: String,
    /// Affected dependency edge count.
    pub affected_dependency_edge_count: u32,
    /// True when the deprecation reaches install warnings, marketplace card, and
    /// dependency-resolution output.
    pub core_propagation_complete: bool,
    /// True when the deprecation surfaces in the migration docs.
    pub surfaces_in_migration_docs: bool,
    /// Minimum supported manifest version.
    pub min_supported_manifest_version: u32,
    /// Maximum supported manifest version.
    pub max_supported_manifest_version: u32,
    /// The row's manifest version.
    pub row_manifest_version: u32,
    /// True when the row's manifest version sits inside the supported window.
    pub manifest_within_window: bool,
    /// Migration outcome.
    pub migration_outcome_class: String,
    /// Compatibility-shim availability.
    pub shim_availability_class: String,
    /// True when a rollback checkpoint was preserved (when one was required).
    pub rollback_checkpoint_preserved: bool,
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

/// Stable SDK / deprecation policy packet consumed by the SDK migration console, the
/// deprecation-policy view, dependency-resolution output, install review, the
/// marketplace card, the extension detail view, diagnostics, support export, docs/help,
/// release packets, and the CLI inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkDeprecationPolicyPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: SdkPolicyIdentity,
    /// SDK / deprecation policy.
    pub deprecation: SdkDeprecationPolicy,
    /// Deprecation propagation.
    pub propagation: DeprecationPropagation,
    /// Manifest version window.
    pub manifest_window: ManifestVersionWindow,
    /// Ecosystem migration guidance.
    pub migration: EcosystemMigrationGuidance,
    /// Permission posture.
    pub permission_posture: SdkPolicyPermissionPosture,
    /// Compatibility.
    pub compatibility: SdkPolicyCompatibility,
    /// Install posture.
    pub install_posture: SdkPolicyInstallPosture,
    /// Stability qualification claim after the posture is applied.
    pub claim: SdkPolicyQualificationClaim,
    /// Downgraded-row banner requirement.
    pub downgraded_banner: SdkPolicyDowngradedBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a catalog row can never imply a stable SDK-policy claim on its own.
    pub allows_catalog_only_trust: bool,
    /// False so a widened permission set can never ride a stable SDK-policy row.
    pub allows_ambient_extension_privilege: bool,
    /// False so an unbounded activation cost can never ride a stable SDK-policy row.
    pub allows_unbounded_activation_cost: bool,
    /// Inspection row.
    pub inspection: StableSdkDeprecationPolicyInspection,
}

impl StableSdkDeprecationPolicyPacket {
    /// Builds a stable SDK / deprecation policy packet from input, applying the
    /// posture to the claimed tier so any required downgrade below Stable is
    /// automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableSdkDeprecationPolicyValidationError`] when the input violates an
    /// identity, deprecation, propagation, manifest-window, migration, permission,
    /// compatibility, install, or claim invariant.
    pub fn from_input(
        input: StableSdkDeprecationPolicyInput,
    ) -> Result<Self, StableSdkDeprecationPolicyValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let deprecation = deprecation_record(&input.deprecation);
        let propagation = propagation_record(&input.propagation);
        let manifest_window = manifest_window_record(&input.manifest_window);
        let migration = migration_record(&input.migration);
        let permission_posture = permission_posture_record(&input.permission_posture);
        let compatibility = compatibility_record(&input.compatibility);
        let install_posture = install_posture_record(&input.install_posture);
        let attribution_complete = attribution_is_complete(
            &identity,
            &deprecation,
            &migration,
            &compatibility,
            &permission_posture,
        );

        let posture = SdkPolicyPosture {
            identity: &identity,
            deprecation: &deprecation,
            propagation: &propagation,
            manifest_window: &manifest_window,
            migration: &migration,
            permission_posture: &permission_posture,
            compatibility: &compatibility,
            install_posture: &install_posture,
            attribution_complete,
        };

        let claim = claim_record(&input.claim, &posture);
        let downgraded_banner = banner_record(&posture);
        let inspection = inspection_record(&input.packet_id, &posture, &claim, &downgraded_banner);

        let packet = Self {
            record_kind: STABLE_SDK_DEPRECATION_POLICY_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            deprecation,
            propagation,
            manifest_window,
            migration,
            permission_posture,
            compatibility,
            install_posture,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_SDK_DEPRECATION_POLICY_SCHEMA_REF.to_string()],
            allows_catalog_only_trust: false,
            allows_ambient_extension_privilege: false,
            allows_unbounded_activation_cost: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable SDK-policy invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableSdkDeprecationPolicyValidationError`] when an invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), StableSdkDeprecationPolicyValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_SDK_DEPRECATION_POLICY_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_deprecation(&self.deprecation)?;
        validate_propagation(&self.propagation)?;
        validate_manifest_window(&self.manifest_window)?;
        validate_migration(&self.migration)?;
        validate_permission_posture(&self.permission_posture)?;
        validate_compatibility(&self.compatibility)?;
        validate_install_posture(&self.install_posture)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;
        validate_manifest_window_bounds(&self.manifest_window)?;
        validate_migration_consistency(&self.migration)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_SDK_DEPRECATION_POLICY_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_SDK_DEPRECATION_POLICY_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No catalog-only trust, ambient extension privilege, or unbounded activation
        // cost may ride a published stable SDK-policy row.
        if self.allows_catalog_only_trust
            || self.allows_ambient_extension_privilege
            || self.allows_unbounded_activation_cost
        {
            return Err(err(
                "a stable SDK-policy packet must not allow catalog-only trust, ambient extension privilege, or unbounded activation cost",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.profile_version_current() {
                return Err(err(
                    "stable effective tier must pin the published SDK-policy profile version",
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
            if self.deprecation.removed() {
                return Err(err("stable effective tier must not carry a removed SDK"));
            }
            if self.deprecation.in_sunset() {
                return Err(err(
                    "stable effective tier must not be in the SDK sunset window",
                ));
            }
            if self.deprecation.deprecated_or_later() {
                if !self.deprecation.replacement_named() {
                    return Err(err(
                        "stable effective tier must name a replacement for a deprecated SDK",
                    ));
                }
                if !self.deprecation.last_supported_named() {
                    return Err(err(
                        "stable effective tier must name a last-supported window for a deprecated SDK",
                    ));
                }
                if !self.deprecation.dependency_edges_named() {
                    return Err(err(
                        "stable effective tier must name affected dependency edges for a deprecated SDK",
                    ));
                }
                if !self.propagation.core_propagation_complete() {
                    return Err(err(
                        "stable effective tier must propagate the deprecation into install warning, marketplace card, and dependency resolution",
                    ));
                }
                if !self.propagation.surfaces_in_migration_docs {
                    return Err(err(
                        "stable effective tier must propagate the deprecation into the migration docs",
                    ));
                }
            }
            if !self.manifest_window.within_window() {
                return Err(err(
                    "stable effective tier must keep the row manifest version inside the supported window",
                ));
            }
            if self.migration.outcome_unsupported() {
                return Err(err(
                    "stable effective tier must not carry an unsupported migration",
                ));
            }
            if self.migration.outcome_partial() {
                return Err(err(
                    "stable effective tier must not carry a partial migration",
                ));
            }
            if self.migration.needs_rollback_checkpoint()
                && !self.migration.rollback_checkpoint_preserved
            {
                return Err(err(
                    "stable effective tier must preserve a rollback checkpoint for a non-exact migration",
                ));
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

        // Re-derive the effective tier and downgrade verdict so the stored claim cannot
        // drift from the posture truth.
        let posture = SdkPolicyPosture {
            identity: &self.identity,
            deprecation: &self.deprecation,
            propagation: &self.propagation,
            manifest_window: &self.manifest_window,
            migration: &self.migration,
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
        let banner_required = sdk_policy_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-row banner must_display does not match the SDK-policy posture",
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

    /// Returns true when an unsupported migration is never left rendering stable.
    pub fn unsupported_migration_never_stable(&self) -> bool {
        if !self.migration.outcome_unsupported() {
            return true;
        }
        !STABLE_TIERS.contains(&self.claim.effective_tier.as_str())
    }

    /// Returns true when identity, the deprecation policy, the migration guidance,
    /// compatibility, and the permission posture are fully attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(
            &self.identity,
            &self.deprecation,
            &self.migration,
            &self.compatibility,
            &self.permission_posture,
        )
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableSdkDeprecationPolicyProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Marketplace / catalog row identity.
    pub row_identity_ref: String,
    /// SDK release channel id.
    pub sdk_channel_id: String,
    /// SDK deprecation stage.
    pub deprecation_stage_class: String,
    /// Pin policy.
    pub pin_policy_class: String,
    /// Migration outcome.
    pub migration_outcome_class: String,
    /// True when the row manifest version sits inside the supported window.
    pub manifest_within_window: bool,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable SDK-policy claim.
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

impl From<StableSdkDeprecationPolicyPacket> for StableSdkDeprecationPolicyProjection {
    fn from(packet: StableSdkDeprecationPolicyPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            row_identity_ref: packet.identity.row_identity_ref,
            sdk_channel_id: packet.identity.sdk_channel_id,
            deprecation_stage_class: packet.deprecation.deprecation_stage_class,
            pin_policy_class: packet.deprecation.pin_policy_class,
            migration_outcome_class: packet.migration.migration_outcome_class,
            manifest_within_window: packet.manifest_window.within_window(),
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
/// Returns [`StableSdkDeprecationPolicyError`] when the payload fails to parse or
/// violates the stable SDK-policy invariants.
pub fn project_stable_sdk_deprecation_policy(
    payload: &str,
) -> Result<StableSdkDeprecationPolicyProjection, StableSdkDeprecationPolicyError> {
    let packet: StableSdkDeprecationPolicyPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableSdkDeprecationPolicyProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support / partner / mirror export row that quotes the same closed
/// tokens as the packet without leaking raw SDK, manifest, permission, or
/// publisher-private bytes, and preserves the deprecation / manifest-window / migration
/// posture so a reviewer can see why a row is or is not a stable SDK-policy claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSdkDeprecationPolicySupportExport {
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
    /// SDK-policy descriptor ref.
    pub sdk_policy_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// SDK release channel id.
    pub sdk_channel_id: String,
    /// Publisher namespace.
    pub publisher_namespace: String,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Lifecycle state.
    pub lifecycle_state_class: String,
    /// SDK deprecation stage.
    pub deprecation_stage_class: String,
    /// Last-supported version.
    pub last_supported_version: String,
    /// Replacement kind.
    pub replacement_kind_class: String,
    /// Pin policy.
    pub pin_policy_class: String,
    /// Affected dependency edge count.
    pub affected_dependency_edge_count: u32,
    /// True when the deprecation reaches install warnings, marketplace card, and
    /// dependency-resolution output.
    pub core_propagation_complete: bool,
    /// True when the deprecation surfaces in the migration docs.
    pub surfaces_in_migration_docs: bool,
    /// Minimum supported manifest version.
    pub min_supported_manifest_version: u32,
    /// Maximum supported manifest version.
    pub max_supported_manifest_version: u32,
    /// Row manifest version.
    pub row_manifest_version: u32,
    /// True when the row manifest version sits inside the supported window.
    pub manifest_within_window: bool,
    /// Migration outcome.
    pub migration_outcome_class: String,
    /// Compatibility-shim availability.
    pub shim_availability_class: String,
    /// True when a rollback checkpoint was preserved.
    pub rollback_checkpoint_preserved: bool,
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
    /// True when the effective tier blocks the row as a stable SDK-policy claim
    /// (withdrawn).
    pub blocks_stable_sdk_policy: bool,
    /// Export-safe summary suitable for support / partner / mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support / partner / mirror export row.
pub fn project_stable_sdk_deprecation_policy_support_export(
    packet: &StableSdkDeprecationPolicyPacket,
) -> StableSdkDeprecationPolicySupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} SDK stage={} last_supported={} replacement={} pin={} edges={}. Propagation core={} docs={}. Manifest window=[{}..{}] row={} (within={}). Migration={} shim={} rollback={}. Cost={}. Compatibility={}. Revocation={} mirrorability={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.deprecation.deprecation_stage_class,
        display_or_none(&packet.deprecation.last_supported_version),
        packet.deprecation.replacement_kind_class,
        packet.deprecation.pin_policy_class,
        packet.deprecation.affected_dependency_edge_count,
        packet.propagation.core_propagation_complete(),
        packet.propagation.surfaces_in_migration_docs,
        packet.manifest_window.min_supported_manifest_version,
        packet.manifest_window.max_supported_manifest_version,
        packet.manifest_window.row_manifest_version,
        packet.manifest_window.within_window(),
        packet.migration.migration_outcome_class,
        packet.migration.shim_availability_class,
        packet.migration.rollback_checkpoint_preserved,
        packet.install_posture.activation_cost_class,
        packet.compatibility.compatibility_label_class,
        packet.install_posture.revocation_posture_class,
        packet.install_posture.mirrorability_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StableSdkDeprecationPolicySupportExport {
        record_kind: STABLE_SDK_DEPRECATION_POLICY_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        export_id: format!(
            "stable_sdk_deprecation_policy_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        row_identity_ref: packet.identity.row_identity_ref.clone(),
        sdk_policy_ref: packet.identity.sdk_policy_ref.clone(),
        extension_identity: packet.identity.extension_identity.clone(),
        extension_version: packet.identity.extension_version.clone(),
        sdk_channel_id: packet.identity.sdk_channel_id.clone(),
        publisher_namespace: packet.identity.publisher_namespace.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        deprecation_stage_class: packet.deprecation.deprecation_stage_class.clone(),
        last_supported_version: packet.deprecation.last_supported_version.clone(),
        replacement_kind_class: packet.deprecation.replacement_kind_class.clone(),
        pin_policy_class: packet.deprecation.pin_policy_class.clone(),
        affected_dependency_edge_count: packet.deprecation.affected_dependency_edge_count,
        core_propagation_complete: packet.propagation.core_propagation_complete(),
        surfaces_in_migration_docs: packet.propagation.surfaces_in_migration_docs,
        min_supported_manifest_version: packet.manifest_window.min_supported_manifest_version,
        max_supported_manifest_version: packet.manifest_window.max_supported_manifest_version,
        row_manifest_version: packet.manifest_window.row_manifest_version,
        manifest_within_window: packet.manifest_window.within_window(),
        migration_outcome_class: packet.migration.migration_outcome_class.clone(),
        shim_availability_class: packet.migration.shim_availability_class.clone(),
        rollback_checkpoint_preserved: packet.migration.rollback_checkpoint_preserved,
        activation_cost_class: packet.install_posture.activation_cost_class.clone(),
        compatibility_label_class: packet.compatibility.compatibility_label_class.clone(),
        revocation_posture_class: packet.install_posture.revocation_posture_class.clone(),
        mirrorability_class: packet.install_posture.mirrorability_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_sdk_policy: blocks,
        export_safe_summary,
    }
}

fn display_or_none(value: &str) -> String {
    if value.trim().is_empty() {
        "none".to_string()
    } else {
        value.to_string()
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable SDK-policy operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableSdkDeprecationPolicyError {
    /// Validation failed.
    Validation(StableSdkDeprecationPolicyValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableSdkDeprecationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableSdkDeprecationPolicyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable SDK-policy packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableSdkDeprecationPolicyValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableSdkDeprecationPolicyValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableSdkDeprecationPolicyValidationError {}

impl StableSdkDeprecationPolicyValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableSdkDeprecationPolicyError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableSdkDeprecationPolicyValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableSdkDeprecationPolicyValidationError> for StableSdkDeprecationPolicyError {
    fn from(err: StableSdkDeprecationPolicyValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the SDK-policy posture.
struct SdkPolicyPosture<'a> {
    identity: &'a SdkPolicyIdentity,
    deprecation: &'a SdkDeprecationPolicy,
    propagation: &'a DeprecationPropagation,
    manifest_window: &'a ManifestVersionWindow,
    migration: &'a EcosystemMigrationGuidance,
    permission_posture: &'a SdkPolicyPermissionPosture,
    compatibility: &'a SdkPolicyCompatibility,
    install_posture: &'a SdkPolicyInstallPosture,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the SDK-policy posture.
fn posture_reasons(posture: &SdkPolicyPosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();

    // Identity.
    if !posture.identity.profile_version_current() {
        reasons.push("sdk_policy_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        reasons.push("lifecycle_not_runnable".to_string());
    }

    // Deprecation policy.
    if posture.deprecation.removed() {
        reasons.push("deprecation_stage_removed".to_string());
    } else if posture.deprecation.in_sunset() {
        reasons.push("deprecation_in_sunset_window".to_string());
    }
    if posture.deprecation.deprecated_or_later() {
        if !posture.deprecation.replacement_named() {
            reasons.push("replacement_path_missing".to_string());
        }
        if !posture.deprecation.last_supported_named() {
            reasons.push("last_supported_window_missing".to_string());
        }
        if !posture.deprecation.dependency_edges_named() {
            reasons.push("affected_dependency_edges_unnamed".to_string());
        }
        if !posture.propagation.core_propagation_complete() {
            reasons.push("deprecation_propagation_incomplete".to_string());
        }
        if !posture.propagation.surfaces_in_migration_docs {
            reasons.push("migration_docs_missing".to_string());
        }
    }

    // Manifest window.
    if !posture.manifest_window.within_window() {
        reasons.push("manifest_version_out_of_window".to_string());
    }

    // Migration guidance.
    if posture.migration.outcome_unsupported() {
        reasons.push("migration_outcome_unsupported".to_string());
    } else if posture.migration.outcome_partial() {
        reasons.push("migration_outcome_partial".to_string());
    }
    if posture.migration.needs_rollback_checkpoint()
        && !posture.migration.rollback_checkpoint_preserved
    {
        reasons.push("rollback_checkpoint_not_preserved".to_string());
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

/// Applies the SDK-policy posture to a claimed tier, narrowing automatically below
/// Stable when the evidence can no longer back it. The claim basis is folded in
/// separately so a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &SdkPolicyPosture<'_>,
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
        "stable" => "stable_sdk_policy_claim",
        "beta" => "beta_sdk_policy_partial_claim",
        "preview" => "preview_sdk_policy_experimental_claim",
        "withdrawn" => "withdrawn_no_sdk_policy_claim",
        _ => "preview_sdk_policy_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity, the deprecation policy, the migration guidance,
/// compatibility, and the permission posture are fully attributed.
fn attribution_is_complete(
    identity: &SdkPolicyIdentity,
    deprecation: &SdkDeprecationPolicy,
    migration: &EcosystemMigrationGuidance,
    compatibility: &SdkPolicyCompatibility,
    permission_posture: &SdkPolicyPermissionPosture,
) -> bool {
    let deprecation_attributed = if deprecation.deprecated_or_later() {
        // A deprecated SDK must attribute its support window and edge list.
        !deprecation.support_window_ref.trim().is_empty()
            && !deprecation.dependency_edges_ref.trim().is_empty()
    } else {
        true
    };
    !identity.sdk_policy_ref.trim().is_empty()
        && !identity.row_identity_ref.trim().is_empty()
        && !identity.policy_evidence_ref.trim().is_empty()
        && !identity.publisher_namespace.trim().is_empty()
        && !identity.sdk_channel_id.trim().is_empty()
        && deprecation_attributed
        && !migration.migration_doc_ref.trim().is_empty()
        && !migration.diagnostics_ref.trim().is_empty()
        && !compatibility.scorecard_ref.trim().is_empty()
        && !permission_posture.declared_permission_ref.trim().is_empty()
        && !permission_posture
            .effective_permission_ref
            .trim()
            .is_empty()
}

/// Returns true when the SDK-policy posture requires a pre-trust warning banner.
fn sdk_policy_requires_warning(posture: &SdkPolicyPosture<'_>) -> bool {
    posture.identity.publisher_trust_tier_class == "quarantined"
        || !posture.identity.lifecycle_runnable()
        || posture.deprecation.removed()
        || posture.deprecation.in_sunset()
        || posture.permission_posture.widened
        || posture.install_posture.activation_cost_unbounded()
        || posture.compatibility.unsupported()
        || posture.migration.outcome_unsupported()
        || !posture.manifest_window.within_window()
        || matches!(
            posture.install_posture.revocation_posture_class.as_str(),
            "quarantined" | "revoked"
        )
}

/// Picks the most-severe banner reason for a row that requires a warning.
fn banner_reason_for(posture: &SdkPolicyPosture<'_>) -> Option<String> {
    if posture.permission_posture.widened {
        return Some("permission_widened".to_string());
    }
    if posture.install_posture.activation_cost_unbounded() {
        return Some("activation_cost_unbounded".to_string());
    }
    if posture.migration.outcome_unsupported() {
        return Some("migration_outcome_unsupported".to_string());
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
    if posture.deprecation.removed() {
        return Some("deprecation_stage_removed".to_string());
    }
    if !posture.identity.lifecycle_runnable() {
        return Some("lifecycle_not_runnable".to_string());
    }
    if !posture.manifest_window.within_window() {
        return Some("manifest_version_out_of_window".to_string());
    }
    if posture.deprecation.in_sunset() {
        return Some("deprecation_in_sunset_window".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &SdkPolicyIdentityInput) -> SdkPolicyIdentity {
    SdkPolicyIdentity {
        record_kind: SDK_POLICY_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        sdk_policy_ref: input.sdk_policy_ref.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        extension_identity: input.extension_identity.clone(),
        extension_version: input.extension_version.clone(),
        package_id: input.package_id.clone(),
        sdk_channel_id: input.sdk_channel_id.clone(),
        sdk_policy_version: input.sdk_policy_version,
        published_policy_version: input.published_policy_version,
        publisher_namespace: input.publisher_namespace.clone(),
        policy_evidence_ref: input.policy_evidence_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn deprecation_record(input: &SdkDeprecationPolicyInput) -> SdkDeprecationPolicy {
    SdkDeprecationPolicy {
        record_kind: SDK_DEPRECATION_POLICY_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        deprecation_stage_class: input.deprecation_stage_class.clone(),
        last_supported_version: input.last_supported_version.clone(),
        replacement_kind_class: input.replacement_kind_class.clone(),
        replacement_ref: input.replacement_ref.clone(),
        pin_policy_class: input.pin_policy_class.clone(),
        support_window_ref: input.support_window_ref.clone(),
        affected_dependency_edge_count: input.affected_dependency_edge_count,
        dependency_edges_ref: input.dependency_edges_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn propagation_record(input: &DeprecationPropagationInput) -> DeprecationPropagation {
    DeprecationPropagation {
        record_kind: DEPRECATION_PROPAGATION_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        surfaces_in_install_warning: input.surfaces_in_install_warning,
        surfaces_in_marketplace_card: input.surfaces_in_marketplace_card,
        surfaces_in_dependency_resolution: input.surfaces_in_dependency_resolution,
        surfaces_in_migration_docs: input.surfaces_in_migration_docs,
        surfaces_in_compat_shim: input.surfaces_in_compat_shim,
        summary_label: input.summary_label.clone(),
    }
}

fn manifest_window_record(input: &ManifestVersionWindowInput) -> ManifestVersionWindow {
    ManifestVersionWindow {
        record_kind: MANIFEST_VERSION_WINDOW_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        min_supported_manifest_version: input.min_supported_manifest_version,
        max_supported_manifest_version: input.max_supported_manifest_version,
        published_manifest_version: input.published_manifest_version,
        row_manifest_version: input.row_manifest_version,
        summary_label: input.summary_label.clone(),
    }
}

fn migration_record(input: &EcosystemMigrationGuidanceInput) -> EcosystemMigrationGuidance {
    EcosystemMigrationGuidance {
        record_kind: ECOSYSTEM_MIGRATION_GUIDANCE_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        migration_outcome_class: input.migration_outcome_class.clone(),
        migration_doc_ref: input.migration_doc_ref.clone(),
        shim_availability_class: input.shim_availability_class.clone(),
        rollback_checkpoint_preserved: input.rollback_checkpoint_preserved,
        diagnostics_ref: input.diagnostics_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn permission_posture_record(
    input: &SdkPolicyPermissionPostureInput,
) -> SdkPolicyPermissionPosture {
    SdkPolicyPermissionPosture {
        record_kind: SDK_POLICY_PERMISSION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        declared_permission_ref: input.declared_permission_ref.clone(),
        effective_permission_ref: input.effective_permission_ref.clone(),
        policy_cap_ref: input.policy_cap_ref.clone(),
        widened: input.widened,
        reconsent_required: input.reconsent_required,
        summary_label: input.summary_label.clone(),
    }
}

fn compatibility_record(input: &SdkPolicyCompatibilityInput) -> SdkPolicyCompatibility {
    SdkPolicyCompatibility {
        record_kind: SDK_POLICY_COMPATIBILITY_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        compatibility_label_class: input.compatibility_label_class.clone(),
        scorecard_ref: input.scorecard_ref.clone(),
        compatibility_verified: input.compatibility_verified,
        summary_label: input.summary_label.clone(),
    }
}

fn install_posture_record(input: &SdkPolicyInstallPostureInput) -> SdkPolicyInstallPosture {
    SdkPolicyInstallPosture {
        record_kind: SDK_POLICY_INSTALL_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
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
    input: &SdkPolicyQualificationClaimInput,
    posture: &SdkPolicyPosture<'_>,
) -> SdkPolicyQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    SdkPolicyQualificationClaim {
        record_kind: SDK_POLICY_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &SdkPolicyPosture<'_>) -> SdkPolicyDowngradedBanner {
    let must_display = sdk_policy_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "SDK-policy row requires review before relying on the deprecation guidance ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "SDK-policy hardened: named replacement and window, propagated deprecation, manifest version in window, supportable migration guidance."
            .to_string()
    };
    SdkPolicyDowngradedBanner {
        record_kind: SDK_POLICY_DOWNGRADED_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &SdkPolicyPosture<'_>,
    claim: &SdkPolicyQualificationClaim,
    banner: &SdkPolicyDowngradedBanner,
) -> StableSdkDeprecationPolicyInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());

    StableSdkDeprecationPolicyInspection {
        record_kind: STABLE_SDK_DEPRECATION_POLICY_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        profile_version_current: posture.identity.profile_version_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        lifecycle_runnable: posture.identity.lifecycle_runnable(),
        deprecation_stage_class: posture.deprecation.deprecation_stage_class.clone(),
        replacement_named: posture.deprecation.replacement_named(),
        pin_policy_class: posture.deprecation.pin_policy_class.clone(),
        affected_dependency_edge_count: posture.deprecation.affected_dependency_edge_count,
        core_propagation_complete: posture.propagation.core_propagation_complete(),
        surfaces_in_migration_docs: posture.propagation.surfaces_in_migration_docs,
        min_supported_manifest_version: posture.manifest_window.min_supported_manifest_version,
        max_supported_manifest_version: posture.manifest_window.max_supported_manifest_version,
        row_manifest_version: posture.manifest_window.row_manifest_version,
        manifest_within_window: posture.manifest_window.within_window(),
        migration_outcome_class: posture.migration.migration_outcome_class.clone(),
        shim_availability_class: posture.migration.shim_availability_class.clone(),
        rollback_checkpoint_preserved: posture.migration.rollback_checkpoint_preserved,
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
    input: &StableSdkDeprecationPolicyInput,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(&id.sdk_policy_ref, "identity.sdk_policy_ref")?;
    if !id.sdk_policy_ref.starts_with("sdk_policy:") {
        return Err(err("identity.sdk_policy_ref must start with 'sdk_policy:'"));
    }
    ensure_nonempty(&id.row_identity_ref, "identity.row_identity_ref")?;
    ensure_nonempty(&id.extension_identity, "identity.extension_identity")?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.package_id, "identity.package_id")?;
    ensure_nonempty(&id.sdk_channel_id, "identity.sdk_channel_id")?;
    ensure_nonempty(&id.publisher_namespace, "identity.publisher_namespace")?;
    ensure_nonempty(&id.policy_evidence_ref, "identity.policy_evidence_ref")?;
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

    let dep = &input.deprecation;
    ensure_token(
        DEPRECATION_STAGE_CLASSES,
        &dep.deprecation_stage_class,
        "deprecation.deprecation_stage_class",
    )?;
    ensure_token(
        REPLACEMENT_KIND_CLASSES,
        &dep.replacement_kind_class,
        "deprecation.replacement_kind_class",
    )?;
    ensure_token(
        PIN_POLICY_CLASSES,
        &dep.pin_policy_class,
        "deprecation.pin_policy_class",
    )?;

    let mig = &input.migration;
    ensure_token(
        MIGRATION_OUTCOME_CLASSES,
        &mig.migration_outcome_class,
        "migration.migration_outcome_class",
    )?;
    ensure_token(
        SHIM_AVAILABILITY_CLASSES,
        &mig.shim_availability_class,
        "migration.shim_availability_class",
    )?;
    ensure_nonempty(&mig.migration_doc_ref, "migration.migration_doc_ref")?;
    ensure_nonempty(&mig.diagnostics_ref, "migration.diagnostics_ref")?;

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
            STABLE_SDK_DEPRECATION_POLICY_CONSUMER_SURFACES,
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
    identity: &SdkPolicyIdentity,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        SDK_POLICY_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_SDK_DEPRECATION_POLICY_SCHEMA_VERSION,
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

fn validate_deprecation(
    dep: &SdkDeprecationPolicy,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        dep.record_kind.as_str(),
        SDK_DEPRECATION_POLICY_RECORD_KIND,
        "deprecation record_kind",
    )?;
    ensure_token(
        DEPRECATION_STAGE_CLASSES,
        &dep.deprecation_stage_class,
        "deprecation deprecation_stage_class",
    )?;
    ensure_token(
        REPLACEMENT_KIND_CLASSES,
        &dep.replacement_kind_class,
        "deprecation replacement_kind_class",
    )?;
    ensure_token(
        PIN_POLICY_CLASSES,
        &dep.pin_policy_class,
        "deprecation pin_policy_class",
    )?;
    Ok(())
}

fn validate_propagation(
    _prop: &DeprecationPropagation,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        _prop.record_kind.as_str(),
        DEPRECATION_PROPAGATION_RECORD_KIND,
        "propagation record_kind",
    )?;
    Ok(())
}

fn validate_manifest_window(
    win: &ManifestVersionWindow,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        win.record_kind.as_str(),
        MANIFEST_VERSION_WINDOW_RECORD_KIND,
        "manifest_window record_kind",
    )?;
    Ok(())
}

fn validate_migration(
    mig: &EcosystemMigrationGuidance,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        mig.record_kind.as_str(),
        ECOSYSTEM_MIGRATION_GUIDANCE_RECORD_KIND,
        "migration record_kind",
    )?;
    ensure_token(
        MIGRATION_OUTCOME_CLASSES,
        &mig.migration_outcome_class,
        "migration migration_outcome_class",
    )?;
    ensure_token(
        SHIM_AVAILABILITY_CLASSES,
        &mig.shim_availability_class,
        "migration shim_availability_class",
    )?;
    ensure_nonempty(&mig.migration_doc_ref, "migration migration_doc_ref")?;
    ensure_nonempty(&mig.diagnostics_ref, "migration diagnostics_ref")?;
    Ok(())
}

fn validate_permission_posture(
    perm: &SdkPolicyPermissionPosture,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        perm.record_kind.as_str(),
        SDK_POLICY_PERMISSION_POSTURE_RECORD_KIND,
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
    compat: &SdkPolicyCompatibility,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        compat.record_kind.as_str(),
        SDK_POLICY_COMPATIBILITY_RECORD_KIND,
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
    inst: &SdkPolicyInstallPosture,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        inst.record_kind.as_str(),
        SDK_POLICY_INSTALL_POSTURE_RECORD_KIND,
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
    claim: &SdkPolicyQualificationClaim,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        SDK_POLICY_QUALIFICATION_CLAIM_RECORD_KIND,
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
            SDK_DEPRECATION_POLICY_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &SdkPolicyDowngradedBanner,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        SDK_POLICY_DOWNGRADED_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            SDK_DEPRECATION_POLICY_DOWNGRADE_REASONS,
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

/// Cross-checks the manifest window bounds so a window record cannot be internally
/// inconsistent.
fn validate_manifest_window_bounds(
    win: &ManifestVersionWindow,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    if win.max_supported_manifest_version < win.min_supported_manifest_version {
        return Err(err(
            "manifest_window max_supported_manifest_version must not precede min_supported_manifest_version",
        ));
    }
    if win.published_manifest_version < win.min_supported_manifest_version
        || win.published_manifest_version > win.max_supported_manifest_version
    {
        return Err(err(
            "manifest_window published_manifest_version must sit inside the supported window",
        ));
    }
    Ok(())
}

/// Cross-checks the migration outcome against the shim availability so a migration
/// record cannot be internally inconsistent.
fn validate_migration_consistency(
    mig: &EcosystemMigrationGuidance,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    if mig.migration_outcome_class == "shimmed" && mig.shim_availability_class != "shim_available" {
        return Err(err(
            "a shimmed migration outcome must carry an available compatibility shim",
        ));
    }
    Ok(())
}

fn validate_inspection(
    inspection: &StableSdkDeprecationPolicyInspection,
    packet: &StableSdkDeprecationPolicyPacket,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_SDK_DEPRECATION_POLICY_INSPECTION_RECORD_KIND,
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
    if inspection.manifest_within_window != packet.manifest_window.within_window() {
        return Err(err("inspection manifest_within_window is inconsistent"));
    }
    if inspection.core_propagation_complete != packet.propagation.core_propagation_complete() {
        return Err(err("inspection core_propagation_complete is inconsistent"));
    }
    if inspection.replacement_named != packet.deprecation.replacement_named() {
        return Err(err("inspection replacement_named is inconsistent"));
    }
    if inspection.permissions_not_widened == packet.permission_posture.widened {
        return Err(err("inspection permissions_not_widened is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableSdkDeprecationPolicyValidationError {
    StableSdkDeprecationPolicyValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StableSdkDeprecationPolicyValidationError>
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
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
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
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableSdkDeprecationPolicyValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
