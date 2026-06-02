//! Stabilize mirror / manual import, offline catalog, publisher-transfer
//! continuity, and namespace-trust flows for the stable ecosystem line — one
//! conformance-backed, mirrorable, automatically-narrowing packet.
//!
//! The beta-level [`crate::mirror_import`] module owns the per-row baseline that
//! keeps primary-catalog, approved-mirror, offline-bundle, and manual-artifact
//! imports aligned on artifact identity, source visibility, and per-claim trust
//! downgrade metadata. This module owns the layer above it — the **stable,
//! evidence-backed import truth** a claimed stable imported / mirrored / offline
//! row carries, and the **stability qualification** that truth is allowed to claim.
//!
//! A stable import-truth row must bind, machine-readably:
//!
//! - the **identity** (catalog descriptor ref, row identity, package identity, the
//!   pinned import-truth version, the publisher namespace, the source artifact ref,
//!   the publisher trust tier, and the lifecycle state),
//! - the **source class** (`primary_catalog` / `approved_mirror` / `offline_bundle`
//!   / `manual_artifact`), the source visibility, whether the publisher / source
//!   class is preserved, whether a last-known-good is pinned, and whether the row
//!   stays explainable offline so a revoked or rehomed package never goes dark,
//! - the **publisher-transfer continuity** binding — the continuity event
//!   (`namespace_reservation` / `ownership_transfer` / `signer_key_rotation` /
//!   `orphaning_succession` / `approved_mirror_promotion`), the continuity state,
//!   whether the cooldown / delay is satisfied, the audit-trail ref and whether the
//!   audit lineage is preserved, whether the user and admin were notified, whether
//!   high-trust auto-update was gated behind delay / audit / notification before it
//!   resumed, and whether the transfer history is preserved,
//! - the **mapping outcome** (`exact` / `translated` / `partial` / `shimmed` /
//!   `unsupported`) generated from the real imported artifact, with the
//!   rollback-checkpoint ref and the diagnostics preserved when a mapping fails,
//! - the **permission posture** (declared-vs-effective refs, whether import widened
//!   authority, whether re-consent is required) so a mirror / manual import can
//!   never silently widen privilege,
//! - the **compatibility** label (parity band, evidence source, verified flag),
//! - the **activation-budget** instrumentation (so an unbounded activation cost can
//!   never ride a stable claim),
//! - the **install posture** (install scope and disclosure, revocation posture,
//!   mirrorability, rollback support), and
//! - the **stability qualification** after the posture is applied.
//!
//! The central rule mirrors the rest of the stable line: a **stable** import-truth
//! claim may never be implied from a catalog or mirror row alone. A row that renders
//! a `stable` badge must pin the published import-truth version, be evidence-backed
//! (not catalog-asserted), keep its publisher trust tier out of quarantine, stay on
//! an installable lifecycle, preserve its source class, stay explainable offline,
//! pin a last-known-good, keep its publisher-transfer continuity current with a
//! preserved audit lineage and transfer history (and, for any transfer event, keep
//! high-trust auto-update gated behind delay / audit / notification), map exactly or
//! by a verified translation from the real artifact with no missing rollback
//! checkpoint, never widen permissions on import, keep its compatibility verified and
//! not parity-limited / inherited / unsupported, keep its activation cost bounded and
//! within budget, disclose its install scope, keep a clean revocation posture, stay
//! mirrorable, and be fully attributed. When any of those fails, the visible tier is
//! **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with
//! machine-readable reasons rather than left asserting an import readiness the
//! evidence cannot back.
//!
//! Three guardrails are encoded so they cannot be papered over:
//!
//! - **No catalog-only trust.** A `catalog_asserted_only` claim basis can never back
//!   a stable import-truth claim; it narrows below Stable.
//! - **No ambient privilege.** A permission set widened on import withdraws the row
//!   outright.
//! - **No unbounded activation cost.** An `unbounded` activation budget withdraws the
//!   row outright; an `over_budget` budget narrows to `beta`.
//!
//! The companion schema lives at
//! [`schemas/extensions/stable_mirror_import_truth.schema.json`](../../../../schemas/extensions/stable_mirror_import_truth.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/`.

use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every stable mirror/manual import-truth record.
pub const STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// The published, stable import-truth version. A `stable` claim must pin exactly this
/// version; any other version narrows below Stable.
pub const STABLE_MIRROR_IMPORT_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_REF: &str =
    "schemas/extensions/stable_mirror_import_truth.schema.json";

/// Record-kind tag for [`StableMirrorImportTruthPacket`].
pub const STABLE_MIRROR_IMPORT_TRUTH_PACKET_RECORD_KIND: &str = "stable_mirror_import_truth_packet";

/// Record-kind tag for [`MirrorImportTruthIdentity`].
pub const MIRROR_IMPORT_TRUTH_IDENTITY_RECORD_KIND: &str = "stable_mirror_import_truth_identity";

/// Record-kind tag for [`MirrorImportSourceClass`].
pub const MIRROR_IMPORT_SOURCE_CLASS_RECORD_KIND: &str = "stable_mirror_import_source_class";

/// Record-kind tag for [`MirrorImportContinuity`].
pub const MIRROR_IMPORT_CONTINUITY_RECORD_KIND: &str = "stable_mirror_import_continuity";

/// Record-kind tag for [`MirrorImportMappingOutcome`].
pub const MIRROR_IMPORT_MAPPING_OUTCOME_RECORD_KIND: &str = "stable_mirror_import_mapping_outcome";

/// Record-kind tag for [`MirrorImportPermissionPosture`].
pub const MIRROR_IMPORT_PERMISSION_POSTURE_RECORD_KIND: &str =
    "stable_mirror_import_permission_posture";

/// Record-kind tag for [`MirrorImportCompatibility`].
pub const MIRROR_IMPORT_COMPATIBILITY_RECORD_KIND: &str = "stable_mirror_import_compatibility";

/// Record-kind tag for [`MirrorImportActivationBudget`].
pub const MIRROR_IMPORT_ACTIVATION_BUDGET_RECORD_KIND: &str =
    "stable_mirror_import_activation_budget";

/// Record-kind tag for [`MirrorImportInstallPosture`].
pub const MIRROR_IMPORT_INSTALL_POSTURE_RECORD_KIND: &str = "stable_mirror_import_install_posture";

/// Record-kind tag for [`MirrorImportTruthQualificationClaim`].
pub const MIRROR_IMPORT_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND: &str =
    "stable_mirror_import_truth_qualification_claim";

/// Record-kind tag for [`DowngradedImportBanner`].
pub const DOWNGRADED_IMPORT_BANNER_RECORD_KIND: &str = "stable_mirror_import_downgraded_banner";

/// Record-kind tag for [`StableMirrorImportTruthInspection`].
pub const STABLE_MIRROR_IMPORT_TRUTH_INSPECTION_RECORD_KIND: &str =
    "stable_mirror_import_truth_inspection";

/// Record-kind tag for [`StableMirrorImportTruthSupportExport`].
pub const STABLE_MIRROR_IMPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_mirror_import_truth_support_export";

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

/// Lifecycle states a stable import-truth claim may keep (installable / runnable).
pub const INSTALLABLE_LIFECYCLE_STATES: &[&str] =
    &["installed", "pending_activation", "active", "recovered"];

/// Closed import-route vocabulary.
pub const IMPORT_ROUTE_CLASSES: &[&str] = &[
    "primary_catalog",
    "approved_mirror",
    "offline_bundle",
    "manual_artifact",
];

/// Closed source-visibility vocabulary.
pub const SOURCE_VISIBILITY_CLASSES: &[&str] = &["fully_disclosed", "source_class_only", "opaque"];

/// Closed publisher-transfer continuity-event vocabulary. `none` means no transfer
/// event is in flight for this row.
pub const CONTINUITY_EVENT_CLASSES: &[&str] = &[
    "none",
    "namespace_reservation",
    "ownership_transfer",
    "signer_key_rotation",
    "orphaning_succession",
    "approved_mirror_promotion",
];

/// Continuity events that move publisher / signer authority and therefore require
/// delay, audit, and notification before high-trust auto-update may resume.
pub const TRANSFER_CONTINUITY_EVENTS: &[&str] = &[
    "namespace_reservation",
    "ownership_transfer",
    "signer_key_rotation",
    "orphaning_succession",
    "approved_mirror_promotion",
];

/// Closed continuity-state vocabulary. `current` is the only state a stable claim may
/// keep.
pub const CONTINUITY_STATE_CLASSES: &[&str] = &[
    "current",
    "in_cooldown",
    "pending_notification",
    "disputed",
    "orphaned",
    "revoked",
    "stale",
    "missing",
];

/// Closed mapping-outcome vocabulary, generated from the real imported artifact.
pub const MAPPING_OUTCOME_CLASSES: &[&str] =
    &["exact", "translated", "partial", "shimmed", "unsupported"];

/// Mapping outcomes a stable claim may keep (a faithful or verified-translation map).
pub const STABLE_GRADE_MAPPING_OUTCOMES: &[&str] = &["exact", "translated"];

/// Closed compatibility-label vocabulary.
pub const COMPATIBILITY_LABEL_CLASSES: &[&str] = &[
    "full_parity",
    "high_parity",
    "partial_parity",
    "limited_parity",
    "unsupported",
];

/// Closed compatibility evidence-source vocabulary. `inherited_from_adjacent` may
/// never back a stable claim.
pub const COMPATIBILITY_EVIDENCE_SOURCE_CLASSES: &[&str] = &[
    "conformance_suite",
    "certified_workspace",
    "bridge_matrix",
    "vendor_attested",
    "inherited_from_adjacent",
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

/// Closed set of switch-readiness stability tiers.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Tiers that count as a *stable* import-truth claim.
pub const STABLE_TIERS: &[&str] = &["stable"];

/// Closed claim-basis vocabulary. `catalog_asserted_only` may never back stable.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary a tier may imply.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_import_truth_claim",
    "beta_import_truth_partial_claim",
    "preview_import_truth_experimental_claim",
    "withdrawn_no_import_truth_claim",
];

/// Closed set of reasons that narrow a stable import-truth claim below Stable.
pub const MIRROR_IMPORT_DOWNGRADE_REASONS: &[&str] = &[
    "import_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "lifecycle_not_installable",
    "source_class_not_preserved",
    "offline_not_explainable",
    "last_known_good_not_pinned",
    "continuity_event_in_cooldown",
    "continuity_pending_notification",
    "continuity_disputed",
    "continuity_orphaned",
    "continuity_revoked",
    "continuity_stale",
    "continuity_missing",
    "high_trust_auto_update_not_gated",
    "audit_lineage_incomplete",
    "transfer_history_not_preserved",
    "user_admin_not_notified",
    "mapping_outcome_unsupported",
    "mapping_outcome_shimmed",
    "mapping_outcome_partial",
    "mapping_not_from_real_artifact",
    "mapping_rollback_checkpoint_missing",
    "permission_widened_on_import",
    "compatibility_unsupported",
    "compatibility_parity_limited",
    "compatibility_evidence_inherited",
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

/// Reasons that narrow all the way to `withdrawn` (the row cannot be trusted as stable
/// import truth at all).
const WITHDRAWN_CLASS_REASONS: &[&str] = &[
    "lifecycle_not_installable",
    "continuity_disputed",
    "continuity_revoked",
    "high_trust_auto_update_not_gated",
    "mapping_outcome_unsupported",
    "mapping_rollback_checkpoint_missing",
    "permission_widened_on_import",
    "compatibility_unsupported",
    "activation_cost_unbounded",
    "revocation_posture_quarantined",
    "revocation_posture_revoked",
];

/// Reasons that narrow to `preview` (a structural / trust / disclosure shortfall).
const PREVIEW_CLASS_REASONS: &[&str] = &[
    "import_version_not_published",
    "catalog_only_trust_not_evidence_backed",
    "trust_tier_quarantined",
    "source_class_not_preserved",
    "offline_not_explainable",
    "last_known_good_not_pinned",
    "continuity_pending_notification",
    "continuity_orphaned",
    "continuity_missing",
    "audit_lineage_incomplete",
    "transfer_history_not_preserved",
    "user_admin_not_notified",
    "mapping_not_from_real_artifact",
    "compatibility_evidence_inherited",
    "compatibility_not_verified",
    "activation_cost_not_measured",
    "install_scope_not_disclosed",
    "attribution_incomplete",
];

/// Reasons whose only effect is a safe narrowing to `beta`.
const BETA_CLASS_REASONS: &[&str] = &[
    "continuity_event_in_cooldown",
    "continuity_stale",
    "mapping_outcome_shimmed",
    "mapping_outcome_partial",
    "compatibility_parity_limited",
    "activation_cost_over_budget",
    "revocation_posture_advisory",
    "not_mirrorable",
];

/// Closed set of consumer surfaces that ingest this packet.
pub const STABLE_MIRROR_IMPORT_CONSUMER_SURFACES: &[&str] = &[
    "marketplace_result_row",
    "marketplace_detail_page",
    "install_review",
    "side_load_review",
    "mirror_bundle_review",
    "offline_catalog_view",
    "diagnostics",
    "support_export",
    "docs_help_surface",
    "release_packet",
    "cli_inspector",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a stable mirror/manual import-truth packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMirrorImportTruthInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity input.
    pub identity: MirrorImportTruthIdentityInput,
    /// Source-class input.
    pub source_class: MirrorImportSourceClassInput,
    /// Publisher-transfer continuity input.
    pub continuity: MirrorImportContinuityInput,
    /// Mapping-outcome input.
    pub mapping_outcome: MirrorImportMappingOutcomeInput,
    /// Permission-posture input.
    pub permission_posture: MirrorImportPermissionPostureInput,
    /// Compatibility input.
    pub compatibility: MirrorImportCompatibilityInput,
    /// Activation-budget input.
    pub activation_budget: MirrorImportActivationBudgetInput,
    /// Install-posture input.
    pub install_posture: MirrorImportInstallPostureInput,
    /// Stability qualification claim input.
    pub claim: MirrorImportTruthQualificationClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`MirrorImportTruthIdentity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportTruthIdentityInput {
    /// Ref to the catalog descriptor record this row stabilizes.
    pub catalog_descriptor_ref: String,
    /// Opaque marketplace row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published import-truth version this row pins.
    pub import_version: u32,
    /// Publisher namespace the row asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the imported source artifact (bundle, mirror object, manual file).
    pub source_artifact_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

/// Input for [`MirrorImportSourceClass`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportSourceClassInput {
    /// Import route.
    pub import_route_class: String,
    /// Source visibility.
    pub source_visibility_class: String,
    /// Whether the publisher / source class is preserved from the origin.
    pub source_class_preserved: bool,
    /// Ref to the last-known-good pinned artifact for this row.
    pub last_known_good_ref: String,
    /// Whether a last-known-good is pinned so a revoked / rehomed row stays usable.
    pub last_known_good_pinned: bool,
    /// Whether the row stays explainable offline (no live status fetch required).
    pub offline_explainable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportContinuity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportContinuityInput {
    /// Continuity event in flight for this row (or `none`).
    pub continuity_event_class: String,
    /// Continuity state.
    pub continuity_state_class: String,
    /// Whether the continuity-event cooldown / delay has been satisfied.
    pub cooldown_satisfied: bool,
    /// Ref to the continuity audit trail.
    pub audit_trail_ref: String,
    /// Whether the audit lineage is preserved across the transfer.
    pub audit_lineage_preserved: bool,
    /// Whether the user was notified of the continuity event.
    pub user_notified: bool,
    /// Whether the admin was notified of the continuity event.
    pub admin_notified: bool,
    /// Whether high-trust auto-update was gated behind delay / audit / notification
    /// (and only resumed after they were satisfied).
    pub high_trust_auto_update_gated: bool,
    /// Whether the publisher / signer transfer history is preserved.
    pub transfer_history_preserved: bool,
    /// Opaque ref to the publisher-continuity packet, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_packet_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportMappingOutcome`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportMappingOutcomeInput {
    /// Mapping outcome class.
    pub outcome_class: String,
    /// Whether the outcome was generated from the real imported artifact.
    pub generated_from_real_artifact: bool,
    /// Ref to the preserved rollback checkpoint.
    pub rollback_checkpoint_ref: String,
    /// Whether the rollback checkpoint is preserved (always required when a mapping
    /// fails).
    pub checkpoint_preserved: bool,
    /// Ref to the mapping diagnostics.
    pub diagnostics_ref: String,
    /// Whether the mapping failed.
    pub mapping_failed: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportPermissionPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportPermissionPostureInput {
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after import resolution.
    pub effective_permission_ref: String,
    /// Whether import widened authority beyond the declared set.
    pub widened_on_import: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportCompatibility`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportCompatibilityInput {
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the machine-readable compatibility window.
    pub compatibility_window_ref: String,
    /// Compatibility evidence source.
    pub evidence_source_class: String,
    /// Whether compatibility was verified against the imported artifact.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportActivationBudget`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportActivationBudgetInput {
    /// Activation-budget posture for the worst-case surface.
    pub budget_class: String,
    /// Ref to the measured activation cost.
    pub measured_cost_ref: String,
    /// Ref to the declared activation-budget ceiling.
    pub budget_ceiling_ref: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportInstallPosture`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportInstallPostureInput {
    /// Install scope.
    pub install_scope_class: String,
    /// Whether the install scope is disclosed.
    pub install_scope_disclosed: bool,
    /// Revocation posture.
    pub revocation_posture_class: String,
    /// Mirrorability.
    pub mirrorability_class: String,
    /// Whether rollback to last-known-good is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`MirrorImportTruthQualificationClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportTruthQualificationClaimInput {
    /// Import-truth tier claimed by the row.
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
pub struct MirrorImportTruthIdentity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the catalog descriptor record this row stabilizes.
    pub catalog_descriptor_ref: String,
    /// Opaque marketplace row identity ref.
    pub row_identity_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Published import-truth version this row pins.
    pub import_version: u32,
    /// Publisher namespace the row asserts authority under.
    pub publisher_namespace: String,
    /// Ref to the imported source artifact.
    pub source_artifact_ref: String,
    /// Publisher trust tier.
    pub publisher_trust_tier_class: String,
    /// Current lifecycle state.
    pub lifecycle_state_class: String,
}

impl MirrorImportTruthIdentity {
    /// Returns true when the row pins the published stable import-truth version.
    pub fn import_version_current(&self) -> bool {
        self.import_version == STABLE_MIRROR_IMPORT_PUBLISHED_VERSION
    }

    /// Returns true when the lifecycle is installable.
    pub fn lifecycle_installable(&self) -> bool {
        INSTALLABLE_LIFECYCLE_STATES.contains(&self.lifecycle_state_class.as_str())
    }
}

/// Source class for the imported row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportSourceClass {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Import route.
    pub import_route_class: String,
    /// Source visibility.
    pub source_visibility_class: String,
    /// Whether the publisher / source class is preserved from the origin.
    pub source_class_preserved: bool,
    /// Ref to the last-known-good pinned artifact for this row.
    pub last_known_good_ref: String,
    /// Whether a last-known-good is pinned.
    pub last_known_good_pinned: bool,
    /// Whether the row stays explainable offline.
    pub offline_explainable: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Publisher-transfer continuity binding for the imported row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportContinuity {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Continuity event in flight for this row (or `none`).
    pub continuity_event_class: String,
    /// Continuity state.
    pub continuity_state_class: String,
    /// Whether the continuity-event cooldown / delay has been satisfied.
    pub cooldown_satisfied: bool,
    /// Ref to the continuity audit trail.
    pub audit_trail_ref: String,
    /// Whether the audit lineage is preserved across the transfer.
    pub audit_lineage_preserved: bool,
    /// Whether the user was notified of the continuity event.
    pub user_notified: bool,
    /// Whether the admin was notified of the continuity event.
    pub admin_notified: bool,
    /// Whether high-trust auto-update was gated behind delay / audit / notification.
    pub high_trust_auto_update_gated: bool,
    /// Whether the publisher / signer transfer history is preserved.
    pub transfer_history_preserved: bool,
    /// Opaque ref to the publisher-continuity packet, when one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_packet_ref: Option<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MirrorImportContinuity {
    /// Returns true when the continuity is current.
    pub fn current(&self) -> bool {
        self.continuity_state_class == "current"
    }

    /// Returns true when a publisher / signer transfer event is in flight for this row.
    pub fn has_transfer_event(&self) -> bool {
        TRANSFER_CONTINUITY_EVENTS.contains(&self.continuity_event_class.as_str())
    }

    /// Returns true when, for any transfer event, high-trust auto-update was gated
    /// behind delay, audit, and notification and only resumed after they were
    /// satisfied.
    pub fn auto_update_safely_gated(&self) -> bool {
        !self.has_transfer_event() || self.high_trust_auto_update_gated
    }

    /// Returns true when, for any transfer event, both the user and admin were notified.
    pub fn notification_satisfied(&self) -> bool {
        !self.has_transfer_event() || (self.user_notified && self.admin_notified)
    }
}

/// Mapping-outcome record generated from the real imported artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportMappingOutcome {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Mapping outcome class.
    pub outcome_class: String,
    /// Whether the outcome was generated from the real imported artifact.
    pub generated_from_real_artifact: bool,
    /// Ref to the preserved rollback checkpoint.
    pub rollback_checkpoint_ref: String,
    /// Whether the rollback checkpoint is preserved.
    pub checkpoint_preserved: bool,
    /// Ref to the mapping diagnostics.
    pub diagnostics_ref: String,
    /// Whether the mapping failed.
    pub mapping_failed: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MirrorImportMappingOutcome {
    /// Returns true when the outcome is a faithful or verified-translation map.
    pub fn stable_grade(&self) -> bool {
        STABLE_GRADE_MAPPING_OUTCOMES.contains(&self.outcome_class.as_str())
    }

    /// Returns true when the outcome is unsupported.
    pub fn unsupported(&self) -> bool {
        self.outcome_class == "unsupported"
    }

    /// Returns true when a failed mapping did not preserve a rollback checkpoint.
    pub fn checkpoint_missing_on_failure(&self) -> bool {
        self.mapping_failed && !self.checkpoint_preserved
    }
}

/// Permission posture for the imported row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportPermissionPosture {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Ref to the declared permission set.
    pub declared_permission_ref: String,
    /// Ref to the effective permission set after import resolution.
    pub effective_permission_ref: String,
    /// Whether import widened authority beyond the declared set.
    pub widened_on_import: bool,
    /// Whether re-consent is required before activation.
    pub reconsent_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compatibility binding for the imported row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportCompatibility {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Compatibility label.
    pub compatibility_label_class: String,
    /// Ref to the machine-readable compatibility window.
    pub compatibility_window_ref: String,
    /// Compatibility evidence source.
    pub evidence_source_class: String,
    /// Whether compatibility was verified against the imported artifact.
    pub compatibility_verified: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MirrorImportCompatibility {
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

    /// Returns true when compatibility evidence inherits parity from an adjacent claim.
    pub fn evidence_inherited(&self) -> bool {
        self.evidence_source_class == "inherited_from_adjacent"
    }
}

/// Activation-budget instrumentation for the row's worst-case surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportActivationBudget {
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

impl MirrorImportActivationBudget {
    /// Returns true when the activation cost is bounded and within budget.
    pub fn within_budget(&self) -> bool {
        self.budget_class == "within_budget"
    }

    /// Returns true when the activation cost is unbounded.
    pub fn unbounded(&self) -> bool {
        self.budget_class == "unbounded"
    }
}

/// Install / mirror / revocation posture for the imported row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportInstallPosture {
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
    /// Whether rollback to last-known-good is supported.
    pub rollback_supported: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

impl MirrorImportInstallPosture {
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
pub struct MirrorImportTruthQualificationClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Import-truth tier claimed by the row.
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

/// Downgraded-row banner requirement. Raised whenever a reviewer must see an import
/// shortfall before relying on the row before install or enablement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedImportBanner {
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
pub struct StableMirrorImportTruthInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Effective import-truth tier.
    pub effective_tier: String,
    /// True when the claim is a stable import-truth claim.
    pub stable_claim: bool,
    /// True when the row pins the published import-truth version.
    pub import_version_current: bool,
    /// Publisher trust tier.
    pub trust_tier_class: String,
    /// Import route.
    pub import_route_class: String,
    /// True when the lifecycle is installable.
    pub lifecycle_installable: bool,
    /// True when the source class is preserved.
    pub source_class_preserved: bool,
    /// True when the row stays explainable offline.
    pub offline_explainable: bool,
    /// True when a last-known-good is pinned.
    pub last_known_good_pinned: bool,
    /// Continuity event.
    pub continuity_event_class: String,
    /// Continuity state.
    pub continuity_state_class: String,
    /// True when, for any transfer event, high-trust auto-update was safely gated.
    pub auto_update_safely_gated: bool,
    /// True when the audit lineage is preserved.
    pub audit_lineage_preserved: bool,
    /// True when the transfer history is preserved.
    pub transfer_history_preserved: bool,
    /// True when, for any transfer event, both user and admin were notified.
    pub notification_satisfied: bool,
    /// Mapping outcome.
    pub mapping_outcome_class: String,
    /// True when the mapping is stable-grade (exact / translated).
    pub mapping_stable_grade: bool,
    /// True when a failed mapping preserved a rollback checkpoint.
    pub rollback_checkpoint_preserved: bool,
    /// True when import did not widen permissions.
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

/// Stable mirror/manual import-truth packet consumed by marketplace result and detail
/// rows, install / side-load / mirror-bundle review, the offline catalog view,
/// diagnostics, support export, docs/help, release packets, and the CLI inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMirrorImportTruthPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity.
    pub identity: MirrorImportTruthIdentity,
    /// Source class.
    pub source_class: MirrorImportSourceClass,
    /// Publisher-transfer continuity.
    pub continuity: MirrorImportContinuity,
    /// Mapping outcome.
    pub mapping_outcome: MirrorImportMappingOutcome,
    /// Permission posture.
    pub permission_posture: MirrorImportPermissionPosture,
    /// Compatibility.
    pub compatibility: MirrorImportCompatibility,
    /// Activation-budget instrumentation.
    pub activation_budget: MirrorImportActivationBudget,
    /// Install posture.
    pub install_posture: MirrorImportInstallPosture,
    /// Stability qualification claim after the posture is applied.
    pub claim: MirrorImportTruthQualificationClaim,
    /// Downgraded-row banner requirement.
    pub downgraded_banner: DowngradedImportBanner,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// False so a catalog / mirror row can never imply stable trust on its own.
    pub allows_catalog_only_trust: bool,
    /// False so a mirror / manual import can never widen permissions and ride a
    /// stable row.
    pub allows_ambient_privilege: bool,
    /// False so an unbounded activation cost can never ride a stable row.
    pub allows_unbounded_activation_cost: bool,
    /// Inspection row.
    pub inspection: StableMirrorImportTruthInspection,
}

impl StableMirrorImportTruthPacket {
    /// Builds a stable mirror/manual import-truth packet from input, applying the
    /// import posture to the claimed tier so any required downgrade below Stable is
    /// automatic.
    ///
    /// # Errors
    ///
    /// Returns [`StableMirrorImportTruthValidationError`] when the input violates an
    /// identity, source, continuity, mapping, permission, compatibility, budget,
    /// install, or claim invariant.
    pub fn from_input(
        input: StableMirrorImportTruthInput,
    ) -> Result<Self, StableMirrorImportTruthValidationError> {
        validate_input(&input)?;

        let identity = identity_record(&input.identity);
        let source_class = source_class_record(&input.source_class);
        let continuity = continuity_record(&input.continuity);
        let mapping_outcome = mapping_outcome_record(&input.mapping_outcome);
        let permission_posture = permission_posture_record(&input.permission_posture);
        let compatibility = compatibility_record(&input.compatibility);
        let activation_budget = activation_budget_record(&input.activation_budget);
        let install_posture = install_posture_record(&input.install_posture);
        let attribution_complete =
            attribution_is_complete(&identity, &source_class, &continuity, &mapping_outcome);

        let posture = ImportPosture {
            identity: &identity,
            source_class: &source_class,
            continuity: &continuity,
            mapping_outcome: &mapping_outcome,
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
            record_kind: STABLE_MIRROR_IMPORT_TRUTH_PACKET_RECORD_KIND.to_string(),
            schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity,
            source_class,
            continuity,
            mapping_outcome,
            permission_posture,
            compatibility,
            activation_budget,
            install_posture,
            claim,
            downgraded_banner,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_REF.to_string()],
            allows_catalog_only_trust: false,
            allows_ambient_privilege: false,
            allows_unbounded_activation_cost: false,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the stable import-truth invariants.
    ///
    /// # Errors
    ///
    /// Returns [`StableMirrorImportTruthValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), StableMirrorImportTruthValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            STABLE_MIRROR_IMPORT_TRUTH_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_identity(&self.identity)?;
        validate_source_class(&self.source_class)?;
        validate_continuity(&self.continuity)?;
        validate_mapping_outcome(&self.mapping_outcome)?;
        validate_permission_posture(&self.permission_posture)?;
        validate_compatibility(&self.compatibility)?;
        validate_activation_budget(&self.activation_budget)?;
        validate_install_posture(&self.install_posture)?;
        validate_claim(&self.claim)?;
        validate_banner(&self.downgraded_banner)?;

        for surface in &self.consumer_surfaces {
            ensure_token(
                STABLE_MIRROR_IMPORT_CONSUMER_SURFACES,
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
            .any(|r| r == STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_REF)
        {
            return Err(err("packet must cite its source schema"));
        }

        // No catalog-only trust, ambient privilege, or unbounded activation cost may
        // ride a published stable import row.
        if self.allows_catalog_only_trust
            || self.allows_ambient_privilege
            || self.allows_unbounded_activation_cost
        {
            return Err(err(
                "a stable import-truth packet must not allow catalog-only trust, ambient privilege, or unbounded activation cost",
            ));
        }

        // Stable-claim binding.
        if STABLE_TIERS.contains(&self.claim.effective_tier.as_str()) {
            if !self.identity.import_version_current() {
                return Err(err(
                    "stable effective tier must pin the published import-truth version",
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
            if !self.identity.lifecycle_installable() {
                return Err(err(
                    "stable effective tier must stay on an installable lifecycle",
                ));
            }
            if !self.source_class.source_class_preserved {
                return Err(err("stable effective tier must preserve its source class"));
            }
            if !self.source_class.offline_explainable {
                return Err(err("stable effective tier must stay explainable offline"));
            }
            if !self.source_class.last_known_good_pinned {
                return Err(err("stable effective tier must pin a last-known-good"));
            }
            if !self.continuity.current() {
                return Err(err(
                    "stable effective tier must keep its publisher-transfer continuity current",
                ));
            }
            if !self.continuity.auto_update_safely_gated() {
                return Err(err(
                    "stable effective tier must gate high-trust auto-update behind delay, audit, and notification",
                ));
            }
            if !self.continuity.notification_satisfied() {
                return Err(err(
                    "stable effective tier must notify the user and admin of any transfer event",
                ));
            }
            if !self.continuity.audit_lineage_preserved
                || !self.continuity.transfer_history_preserved
            {
                return Err(err(
                    "stable effective tier must preserve its audit lineage and transfer history",
                ));
            }
            if !self.mapping_outcome.stable_grade() {
                return Err(err(
                    "stable effective tier must map exactly or by a verified translation",
                ));
            }
            if !self.mapping_outcome.generated_from_real_artifact {
                return Err(err(
                    "stable effective tier must generate its outcome from the real imported artifact",
                ));
            }
            if self.mapping_outcome.checkpoint_missing_on_failure() {
                return Err(err(
                    "stable effective tier must preserve a rollback checkpoint on a failed mapping",
                ));
            }
            if self.permission_posture.widened_on_import {
                return Err(err(
                    "stable effective tier must not widen permissions on import",
                ));
            }
            if self.compatibility.unsupported()
                || self.compatibility.parity_limited()
                || self.compatibility.evidence_inherited()
                || !self.compatibility.compatibility_verified
            {
                return Err(err(
                    "stable effective tier must carry verified, non-inherited, non-parity-limited compatibility",
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

        // Re-derive the effective tier and downgrade verdict so the stored claim cannot
        // drift from the posture truth.
        let posture = ImportPosture {
            identity: &self.identity,
            source_class: &self.source_class,
            continuity: &self.continuity,
            mapping_outcome: &self.mapping_outcome,
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
        let banner_required = import_requires_warning(&posture);
        if self.downgraded_banner.must_display != banner_required {
            return Err(err(
                "downgraded-row banner must_display does not match the import posture",
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

    /// Returns true when identity, source class, continuity, and mapping are fully
    /// attributed.
    pub fn attribution_complete(&self) -> bool {
        attribution_is_complete(
            &self.identity,
            &self.source_class,
            &self.continuity,
            &self.mapping_outcome,
        )
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableMirrorImportTruthProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Marketplace row identity.
    pub row_identity_ref: String,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after the posture is applied.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// True when the claim is a stable import-truth claim.
    pub stable_claim: bool,
    /// True when the claimed tier was narrowed.
    pub downgraded: bool,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// True when a downgraded-row banner is required.
    pub downgraded_banner_required: bool,
    /// Import route.
    pub import_route_class: String,
    /// Continuity event.
    pub continuity_event_class: String,
    /// Continuity state.
    pub continuity_state_class: String,
    /// Mapping outcome.
    pub mapping_outcome_class: String,
    /// Revocation posture.
    pub revocation_posture_class: String,
}

impl From<StableMirrorImportTruthPacket> for StableMirrorImportTruthProjection {
    fn from(packet: StableMirrorImportTruthPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            row_identity_ref: packet.identity.row_identity_ref,
            claimed_tier: packet.claim.claimed_tier,
            effective_tier: packet.claim.effective_tier,
            support_claim_class: packet.claim.support_claim_class,
            stable_claim: packet.inspection.stable_claim,
            downgraded: packet.claim.downgraded,
            downgrade_reasons: packet.claim.downgrade_reasons,
            downgraded_banner_required: packet.downgraded_banner.must_display,
            import_route_class: packet.source_class.import_route_class,
            continuity_event_class: packet.continuity.continuity_event_class,
            continuity_state_class: packet.continuity.continuity_state_class,
            mapping_outcome_class: packet.mapping_outcome.outcome_class,
            revocation_posture_class: packet.install_posture.revocation_posture_class,
        }
    }
}

/// Parses and validates a materialized packet, returning the compact projection.
///
/// # Errors
///
/// Returns [`StableMirrorImportTruthError`] when the payload fails to parse or violates
/// the stable import-truth invariants.
pub fn project_stable_mirror_import_truth(
    payload: &str,
) -> Result<StableMirrorImportTruthProjection, StableMirrorImportTruthError> {
    let packet: StableMirrorImportTruthPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(StableMirrorImportTruthProjection::from(packet))
}

// ---------------------------------------------------------------------------
// Support export projection
// ---------------------------------------------------------------------------

/// Metadata-safe support / partner / mirror export row that quotes the same closed
/// tokens as the packet without leaking raw artifact, evidence, or publisher-private
/// bytes, and preserves the source class, publisher continuity, transfer history, and
/// last-known-good pinning so a revoked or rehomed package remains explainable offline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableMirrorImportTruthSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Ref to the packet this export quotes.
    pub packet_ref: String,
    /// Marketplace row identity.
    pub row_identity_ref: String,
    /// Catalog descriptor ref.
    pub catalog_descriptor_ref: String,
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
    /// Import route.
    pub import_route_class: String,
    /// Source visibility.
    pub source_visibility_class: String,
    /// True when the source class is preserved.
    pub source_class_preserved: bool,
    /// True when the row stays explainable offline.
    pub offline_explainable: bool,
    /// True when a last-known-good is pinned.
    pub last_known_good_pinned: bool,
    /// Last-known-good ref.
    pub last_known_good_ref: String,
    /// Continuity event.
    pub continuity_event_class: String,
    /// Continuity state.
    pub continuity_state_class: String,
    /// True when, for any transfer event, high-trust auto-update was safely gated.
    pub auto_update_safely_gated: bool,
    /// True when the audit lineage is preserved.
    pub audit_lineage_preserved: bool,
    /// True when the transfer history is preserved.
    pub transfer_history_preserved: bool,
    /// Mapping outcome.
    pub mapping_outcome_class: String,
    /// True when a failed mapping preserved a rollback checkpoint.
    pub rollback_checkpoint_preserved: bool,
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
    /// True when the effective tier blocks the row as stable import truth (withdrawn).
    pub blocks_stable_import_truth: bool,
    /// Export-safe summary suitable for support / partner / mirror consumers.
    pub export_safe_summary: String,
}

/// Projects a packet into the metadata-safe support / partner / mirror export row.
pub fn project_stable_mirror_import_truth_support_export(
    packet: &StableMirrorImportTruthPacket,
) -> StableMirrorImportTruthSupportExport {
    let blocks = packet.claim.effective_tier == "withdrawn";
    let export_safe_summary = format!(
        "{} Route={} visibility={} source_preserved={} offline={} lkg_pinned={}. Continuity event={} state={} gated={} audit={} history={}. Mapping={} checkpoint={}. Compatibility={}. Activation={}. Revocation={} mirrorability={}. Tier claimed={} effective={} (downgraded={}). Banner required={}.",
        packet.claim.summary_label,
        packet.source_class.import_route_class,
        packet.source_class.source_visibility_class,
        packet.source_class.source_class_preserved,
        packet.source_class.offline_explainable,
        packet.source_class.last_known_good_pinned,
        packet.continuity.continuity_event_class,
        packet.continuity.continuity_state_class,
        packet.continuity.auto_update_safely_gated(),
        packet.continuity.audit_lineage_preserved,
        packet.continuity.transfer_history_preserved,
        packet.mapping_outcome.outcome_class,
        !packet.mapping_outcome.checkpoint_missing_on_failure(),
        packet.compatibility.compatibility_label_class,
        packet.activation_budget.budget_class,
        packet.install_posture.revocation_posture_class,
        packet.install_posture.mirrorability_class,
        packet.claim.claimed_tier,
        packet.claim.effective_tier,
        packet.claim.downgraded,
        packet.downgraded_banner.must_display,
    );

    StableMirrorImportTruthSupportExport {
        record_kind: STABLE_MIRROR_IMPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        export_id: format!(
            "stable_mirror_import_truth_support_export:{}",
            packet.packet_id
        ),
        packet_ref: packet.packet_id.clone(),
        row_identity_ref: packet.identity.row_identity_ref.clone(),
        catalog_descriptor_ref: packet.identity.catalog_descriptor_ref.clone(),
        extension_identity: packet.identity.extension_identity.clone(),
        extension_version: packet.identity.extension_version.clone(),
        publisher_namespace: packet.identity.publisher_namespace.clone(),
        trust_tier_class: packet.identity.publisher_trust_tier_class.clone(),
        lifecycle_state_class: packet.identity.lifecycle_state_class.clone(),
        import_route_class: packet.source_class.import_route_class.clone(),
        source_visibility_class: packet.source_class.source_visibility_class.clone(),
        source_class_preserved: packet.source_class.source_class_preserved,
        offline_explainable: packet.source_class.offline_explainable,
        last_known_good_pinned: packet.source_class.last_known_good_pinned,
        last_known_good_ref: packet.source_class.last_known_good_ref.clone(),
        continuity_event_class: packet.continuity.continuity_event_class.clone(),
        continuity_state_class: packet.continuity.continuity_state_class.clone(),
        auto_update_safely_gated: packet.continuity.auto_update_safely_gated(),
        audit_lineage_preserved: packet.continuity.audit_lineage_preserved,
        transfer_history_preserved: packet.continuity.transfer_history_preserved,
        mapping_outcome_class: packet.mapping_outcome.outcome_class.clone(),
        rollback_checkpoint_preserved: !packet.mapping_outcome.checkpoint_missing_on_failure(),
        compatibility_label_class: packet.compatibility.compatibility_label_class.clone(),
        activation_budget_class: packet.activation_budget.budget_class.clone(),
        revocation_posture_class: packet.install_posture.revocation_posture_class.clone(),
        mirrorability_class: packet.install_posture.mirrorability_class.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgraded: packet.claim.downgraded,
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        downgraded_banner_required: packet.downgraded_banner.must_display,
        blocks_stable_import_truth: blocks,
        export_safe_summary,
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for stable import-truth operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StableMirrorImportTruthError {
    /// Validation failed.
    Validation(StableMirrorImportTruthValidationError),
    /// Packet construction failed.
    Construction(String),
}

impl fmt::Display for StableMirrorImportTruthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for StableMirrorImportTruthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for stable import-truth packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StableMirrorImportTruthValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for StableMirrorImportTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for StableMirrorImportTruthValidationError {}

impl StableMirrorImportTruthValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for StableMirrorImportTruthError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(StableMirrorImportTruthValidationError {
            message: err.to_string(),
        })
    }
}

impl From<StableMirrorImportTruthValidationError> for StableMirrorImportTruthError {
    fn from(err: StableMirrorImportTruthValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Effective-tier derivation (the automatic narrowing)
// ---------------------------------------------------------------------------

/// Bundle of derived records used to apply the import posture.
struct ImportPosture<'a> {
    identity: &'a MirrorImportTruthIdentity,
    source_class: &'a MirrorImportSourceClass,
    continuity: &'a MirrorImportContinuity,
    mapping_outcome: &'a MirrorImportMappingOutcome,
    permission_posture: &'a MirrorImportPermissionPosture,
    compatibility: &'a MirrorImportCompatibility,
    activation_budget: &'a MirrorImportActivationBudget,
    install_posture: &'a MirrorImportInstallPosture,
    attribution_complete: bool,
}

struct DerivedTier {
    effective_tier: String,
    support_claim: String,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
}

/// Collects the narrowing reasons triggered by the import posture.
fn posture_reasons(posture: &ImportPosture<'_>) -> Vec<String> {
    let mut reasons: Vec<String> = Vec::new();

    // Identity.
    if !posture.identity.import_version_current() {
        reasons.push("import_version_not_published".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        reasons.push("trust_tier_quarantined".to_string());
    }
    if !posture.identity.lifecycle_installable() {
        reasons.push("lifecycle_not_installable".to_string());
    }

    // Source class.
    if !posture.source_class.source_class_preserved {
        reasons.push("source_class_not_preserved".to_string());
    }
    if !posture.source_class.offline_explainable {
        reasons.push("offline_not_explainable".to_string());
    }
    if !posture.source_class.last_known_good_pinned {
        reasons.push("last_known_good_not_pinned".to_string());
    }

    // Continuity state.
    match posture.continuity.continuity_state_class.as_str() {
        "in_cooldown" => reasons.push("continuity_event_in_cooldown".to_string()),
        "pending_notification" => reasons.push("continuity_pending_notification".to_string()),
        "disputed" => reasons.push("continuity_disputed".to_string()),
        "orphaned" => reasons.push("continuity_orphaned".to_string()),
        "revoked" => reasons.push("continuity_revoked".to_string()),
        "stale" => reasons.push("continuity_stale".to_string()),
        "missing" => reasons.push("continuity_missing".to_string()),
        _ => {}
    }
    if !posture.continuity.auto_update_safely_gated() {
        reasons.push("high_trust_auto_update_not_gated".to_string());
    }
    if !posture.continuity.audit_lineage_preserved {
        reasons.push("audit_lineage_incomplete".to_string());
    }
    if !posture.continuity.transfer_history_preserved {
        reasons.push("transfer_history_not_preserved".to_string());
    }
    if !posture.continuity.notification_satisfied() {
        reasons.push("user_admin_not_notified".to_string());
    }

    // Mapping outcome.
    match posture.mapping_outcome.outcome_class.as_str() {
        "unsupported" => reasons.push("mapping_outcome_unsupported".to_string()),
        "shimmed" => reasons.push("mapping_outcome_shimmed".to_string()),
        "partial" => reasons.push("mapping_outcome_partial".to_string()),
        _ => {}
    }
    if !posture.mapping_outcome.generated_from_real_artifact {
        reasons.push("mapping_not_from_real_artifact".to_string());
    }
    if posture.mapping_outcome.checkpoint_missing_on_failure() {
        reasons.push("mapping_rollback_checkpoint_missing".to_string());
    }

    // Permissions.
    if posture.permission_posture.widened_on_import {
        reasons.push("permission_widened_on_import".to_string());
    }

    // Compatibility.
    if posture.compatibility.unsupported() {
        reasons.push("compatibility_unsupported".to_string());
    } else if posture.compatibility.parity_limited() {
        reasons.push("compatibility_parity_limited".to_string());
    }
    if posture.compatibility.evidence_inherited() {
        reasons.push("compatibility_evidence_inherited".to_string());
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

/// Applies the import posture to a claimed tier, narrowing automatically below Stable
/// when the evidence can no longer back it. The claim basis is folded in separately so
/// a `catalog_asserted_only` basis can never back a stable claim.
fn derive_effective_tier(
    claimed_tier: &str,
    claim_basis: &str,
    posture: &ImportPosture<'_>,
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
        "stable" => "stable_import_truth_claim",
        "beta" => "beta_import_truth_partial_claim",
        "preview" => "preview_import_truth_experimental_claim",
        "withdrawn" => "withdrawn_no_import_truth_claim",
        _ => "preview_import_truth_experimental_claim",
    }
    .to_string()
}

/// Returns true when identity, source class, continuity, and mapping are fully
/// attributed.
fn attribution_is_complete(
    identity: &MirrorImportTruthIdentity,
    source_class: &MirrorImportSourceClass,
    continuity: &MirrorImportContinuity,
    mapping_outcome: &MirrorImportMappingOutcome,
) -> bool {
    !identity.catalog_descriptor_ref.trim().is_empty()
        && !identity.row_identity_ref.trim().is_empty()
        && !identity.source_artifact_ref.trim().is_empty()
        && !identity.publisher_namespace.trim().is_empty()
        && !source_class.last_known_good_ref.trim().is_empty()
        && !continuity.audit_trail_ref.trim().is_empty()
        && !mapping_outcome.rollback_checkpoint_ref.trim().is_empty()
        && !mapping_outcome.diagnostics_ref.trim().is_empty()
}

/// Returns true when the import posture requires a pre-trust warning banner.
fn import_requires_warning(posture: &ImportPosture<'_>) -> bool {
    posture.identity.publisher_trust_tier_class == "quarantined"
        || !posture.identity.lifecycle_installable()
        || !posture.source_class.offline_explainable
        || matches!(
            posture.continuity.continuity_state_class.as_str(),
            "disputed" | "revoked" | "orphaned" | "missing"
        )
        || !posture.continuity.auto_update_safely_gated()
        || posture.mapping_outcome.unsupported()
        || posture.mapping_outcome.checkpoint_missing_on_failure()
        || posture.permission_posture.widened_on_import
        || posture.compatibility.unsupported()
        || posture.activation_budget.unbounded()
        || matches!(
            posture.install_posture.revocation_posture_class.as_str(),
            "quarantined" | "revoked"
        )
}

/// Picks the most-severe banner reason for a row that requires a warning.
fn banner_reason_for(posture: &ImportPosture<'_>) -> Option<String> {
    if posture.permission_posture.widened_on_import {
        return Some("permission_widened_on_import".to_string());
    }
    if posture.continuity.continuity_state_class == "revoked" {
        return Some("continuity_revoked".to_string());
    }
    if posture.continuity.continuity_state_class == "disputed" {
        return Some("continuity_disputed".to_string());
    }
    if !posture.continuity.auto_update_safely_gated() {
        return Some("high_trust_auto_update_not_gated".to_string());
    }
    if posture.mapping_outcome.unsupported() {
        return Some("mapping_outcome_unsupported".to_string());
    }
    if posture.mapping_outcome.checkpoint_missing_on_failure() {
        return Some("mapping_rollback_checkpoint_missing".to_string());
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
    if !posture.identity.lifecycle_installable() {
        return Some("lifecycle_not_installable".to_string());
    }
    if posture.continuity.continuity_state_class == "orphaned" {
        return Some("continuity_orphaned".to_string());
    }
    if posture.continuity.continuity_state_class == "missing" {
        return Some("continuity_missing".to_string());
    }
    if !posture.source_class.offline_explainable {
        return Some("offline_not_explainable".to_string());
    }
    if posture.identity.publisher_trust_tier_class == "quarantined" {
        return Some("trust_tier_quarantined".to_string());
    }
    None
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn identity_record(input: &MirrorImportTruthIdentityInput) -> MirrorImportTruthIdentity {
    MirrorImportTruthIdentity {
        record_kind: MIRROR_IMPORT_TRUTH_IDENTITY_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        catalog_descriptor_ref: input.catalog_descriptor_ref.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        extension_identity: input.extension_identity.clone(),
        extension_version: input.extension_version.clone(),
        package_id: input.package_id.clone(),
        import_version: input.import_version,
        publisher_namespace: input.publisher_namespace.clone(),
        source_artifact_ref: input.source_artifact_ref.clone(),
        publisher_trust_tier_class: input.publisher_trust_tier_class.clone(),
        lifecycle_state_class: input.lifecycle_state_class.clone(),
    }
}

fn source_class_record(input: &MirrorImportSourceClassInput) -> MirrorImportSourceClass {
    MirrorImportSourceClass {
        record_kind: MIRROR_IMPORT_SOURCE_CLASS_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        import_route_class: input.import_route_class.clone(),
        source_visibility_class: input.source_visibility_class.clone(),
        source_class_preserved: input.source_class_preserved,
        last_known_good_ref: input.last_known_good_ref.clone(),
        last_known_good_pinned: input.last_known_good_pinned,
        offline_explainable: input.offline_explainable,
        summary_label: input.summary_label.clone(),
    }
}

fn continuity_record(input: &MirrorImportContinuityInput) -> MirrorImportContinuity {
    MirrorImportContinuity {
        record_kind: MIRROR_IMPORT_CONTINUITY_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        continuity_event_class: input.continuity_event_class.clone(),
        continuity_state_class: input.continuity_state_class.clone(),
        cooldown_satisfied: input.cooldown_satisfied,
        audit_trail_ref: input.audit_trail_ref.clone(),
        audit_lineage_preserved: input.audit_lineage_preserved,
        user_notified: input.user_notified,
        admin_notified: input.admin_notified,
        high_trust_auto_update_gated: input.high_trust_auto_update_gated,
        transfer_history_preserved: input.transfer_history_preserved,
        continuity_packet_ref: input.continuity_packet_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn mapping_outcome_record(input: &MirrorImportMappingOutcomeInput) -> MirrorImportMappingOutcome {
    MirrorImportMappingOutcome {
        record_kind: MIRROR_IMPORT_MAPPING_OUTCOME_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        outcome_class: input.outcome_class.clone(),
        generated_from_real_artifact: input.generated_from_real_artifact,
        rollback_checkpoint_ref: input.rollback_checkpoint_ref.clone(),
        checkpoint_preserved: input.checkpoint_preserved,
        diagnostics_ref: input.diagnostics_ref.clone(),
        mapping_failed: input.mapping_failed,
        summary_label: input.summary_label.clone(),
    }
}

fn permission_posture_record(
    input: &MirrorImportPermissionPostureInput,
) -> MirrorImportPermissionPosture {
    MirrorImportPermissionPosture {
        record_kind: MIRROR_IMPORT_PERMISSION_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        declared_permission_ref: input.declared_permission_ref.clone(),
        effective_permission_ref: input.effective_permission_ref.clone(),
        widened_on_import: input.widened_on_import,
        reconsent_required: input.reconsent_required,
        summary_label: input.summary_label.clone(),
    }
}

fn compatibility_record(input: &MirrorImportCompatibilityInput) -> MirrorImportCompatibility {
    MirrorImportCompatibility {
        record_kind: MIRROR_IMPORT_COMPATIBILITY_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        compatibility_label_class: input.compatibility_label_class.clone(),
        compatibility_window_ref: input.compatibility_window_ref.clone(),
        evidence_source_class: input.evidence_source_class.clone(),
        compatibility_verified: input.compatibility_verified,
        summary_label: input.summary_label.clone(),
    }
}

fn activation_budget_record(
    input: &MirrorImportActivationBudgetInput,
) -> MirrorImportActivationBudget {
    MirrorImportActivationBudget {
        record_kind: MIRROR_IMPORT_ACTIVATION_BUDGET_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        budget_class: input.budget_class.clone(),
        measured_cost_ref: input.measured_cost_ref.clone(),
        budget_ceiling_ref: input.budget_ceiling_ref.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn install_posture_record(input: &MirrorImportInstallPostureInput) -> MirrorImportInstallPosture {
    MirrorImportInstallPosture {
        record_kind: MIRROR_IMPORT_INSTALL_POSTURE_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        install_scope_class: input.install_scope_class.clone(),
        install_scope_disclosed: input.install_scope_disclosed,
        revocation_posture_class: input.revocation_posture_class.clone(),
        mirrorability_class: input.mirrorability_class.clone(),
        rollback_supported: input.rollback_supported,
        summary_label: input.summary_label.clone(),
    }
}

fn claim_record(
    input: &MirrorImportTruthQualificationClaimInput,
    posture: &ImportPosture<'_>,
) -> MirrorImportTruthQualificationClaim {
    let derived = derive_effective_tier(&input.claimed_tier, &input.claim_basis_class, posture);
    MirrorImportTruthQualificationClaim {
        record_kind: MIRROR_IMPORT_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        claimed_tier: input.claimed_tier.clone(),
        effective_tier: derived.effective_tier,
        support_claim_class: derived.support_claim,
        claim_basis_class: input.claim_basis_class.clone(),
        downgraded: derived.downgraded,
        downgrade_reasons: derived.downgrade_reasons,
        summary_label: input.summary_label.clone(),
    }
}

fn banner_record(posture: &ImportPosture<'_>) -> DowngradedImportBanner {
    let must_display = import_requires_warning(posture);
    let banner_reason_class = if must_display {
        banner_reason_for(posture)
    } else {
        None
    };
    let summary_label = if must_display {
        format!(
            "Imported row requires review before install or enablement ({}).",
            banner_reason_class.clone().unwrap_or_default()
        )
    } else {
        "Import truth stabilized: source class, publisher continuity, mapping, compatibility, activation, and revocation all current."
            .to_string()
    };
    DowngradedImportBanner {
        record_kind: DOWNGRADED_IMPORT_BANNER_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        must_display,
        banner_reason_class,
        summary_label,
    }
}

fn inspection_record(
    packet_id: &str,
    posture: &ImportPosture<'_>,
    claim: &MirrorImportTruthQualificationClaim,
    banner: &DowngradedImportBanner,
) -> StableMirrorImportTruthInspection {
    let stable_claim = STABLE_TIERS.contains(&claim.effective_tier.as_str());

    StableMirrorImportTruthInspection {
        record_kind: STABLE_MIRROR_IMPORT_TRUTH_INSPECTION_RECORD_KIND.to_string(),
        schema_version: STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim,
        import_version_current: posture.identity.import_version_current(),
        trust_tier_class: posture.identity.publisher_trust_tier_class.clone(),
        import_route_class: posture.source_class.import_route_class.clone(),
        lifecycle_installable: posture.identity.lifecycle_installable(),
        source_class_preserved: posture.source_class.source_class_preserved,
        offline_explainable: posture.source_class.offline_explainable,
        last_known_good_pinned: posture.source_class.last_known_good_pinned,
        continuity_event_class: posture.continuity.continuity_event_class.clone(),
        continuity_state_class: posture.continuity.continuity_state_class.clone(),
        auto_update_safely_gated: posture.continuity.auto_update_safely_gated(),
        audit_lineage_preserved: posture.continuity.audit_lineage_preserved,
        transfer_history_preserved: posture.continuity.transfer_history_preserved,
        notification_satisfied: posture.continuity.notification_satisfied(),
        mapping_outcome_class: posture.mapping_outcome.outcome_class.clone(),
        mapping_stable_grade: posture.mapping_outcome.stable_grade(),
        rollback_checkpoint_preserved: !posture.mapping_outcome.checkpoint_missing_on_failure(),
        permissions_not_widened: !posture.permission_posture.widened_on_import,
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
    input: &StableMirrorImportTruthInput,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let id = &input.identity;
    ensure_nonempty(
        &id.catalog_descriptor_ref,
        "identity.catalog_descriptor_ref",
    )?;
    if !id.catalog_descriptor_ref.starts_with("catalog_descriptor:") {
        return Err(err(
            "identity.catalog_descriptor_ref must start with 'catalog_descriptor:'",
        ));
    }
    ensure_nonempty(&id.row_identity_ref, "identity.row_identity_ref")?;
    ensure_nonempty(&id.extension_identity, "identity.extension_identity")?;
    ensure_nonempty(&id.extension_version, "identity.extension_version")?;
    ensure_nonempty(&id.package_id, "identity.package_id")?;
    ensure_nonempty(&id.publisher_namespace, "identity.publisher_namespace")?;
    ensure_nonempty(&id.source_artifact_ref, "identity.source_artifact_ref")?;
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

    let src = &input.source_class;
    ensure_token(
        IMPORT_ROUTE_CLASSES,
        &src.import_route_class,
        "source_class.import_route_class",
    )?;
    ensure_token(
        SOURCE_VISIBILITY_CLASSES,
        &src.source_visibility_class,
        "source_class.source_visibility_class",
    )?;
    ensure_nonempty(&src.last_known_good_ref, "source_class.last_known_good_ref")?;

    let cont = &input.continuity;
    ensure_token(
        CONTINUITY_EVENT_CLASSES,
        &cont.continuity_event_class,
        "continuity.continuity_event_class",
    )?;
    ensure_token(
        CONTINUITY_STATE_CLASSES,
        &cont.continuity_state_class,
        "continuity.continuity_state_class",
    )?;
    ensure_nonempty(&cont.audit_trail_ref, "continuity.audit_trail_ref")?;
    if cont.continuity_state_class == "current"
        && cont
            .continuity_packet_ref
            .as_ref()
            .map(|r| r.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(err(
            "a current publisher continuity must bind a continuity_packet_ref",
        ));
    }

    let map = &input.mapping_outcome;
    ensure_token(
        MAPPING_OUTCOME_CLASSES,
        &map.outcome_class,
        "mapping_outcome.outcome_class",
    )?;
    ensure_nonempty(
        &map.rollback_checkpoint_ref,
        "mapping_outcome.rollback_checkpoint_ref",
    )?;
    ensure_nonempty(&map.diagnostics_ref, "mapping_outcome.diagnostics_ref")?;

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
    ensure_nonempty(
        &compat.compatibility_window_ref,
        "compatibility.compatibility_window_ref",
    )?;
    ensure_token(
        COMPATIBILITY_EVIDENCE_SOURCE_CLASSES,
        &compat.evidence_source_class,
        "compatibility.evidence_source_class",
    )?;

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
            STABLE_MIRROR_IMPORT_CONSUMER_SURFACES,
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
    identity: &MirrorImportTruthIdentity,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        identity.record_kind.as_str(),
        MIRROR_IMPORT_TRUTH_IDENTITY_RECORD_KIND,
        "identity record_kind",
    )?;
    ensure_eq_u32(
        identity.schema_version,
        STABLE_MIRROR_IMPORT_TRUTH_SCHEMA_VERSION,
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

fn validate_source_class(
    source: &MirrorImportSourceClass,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        source.record_kind.as_str(),
        MIRROR_IMPORT_SOURCE_CLASS_RECORD_KIND,
        "source_class record_kind",
    )?;
    ensure_token(
        IMPORT_ROUTE_CLASSES,
        &source.import_route_class,
        "source_class import_route_class",
    )?;
    ensure_token(
        SOURCE_VISIBILITY_CLASSES,
        &source.source_visibility_class,
        "source_class source_visibility_class",
    )?;
    Ok(())
}

fn validate_continuity(
    continuity: &MirrorImportContinuity,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        continuity.record_kind.as_str(),
        MIRROR_IMPORT_CONTINUITY_RECORD_KIND,
        "continuity record_kind",
    )?;
    ensure_token(
        CONTINUITY_EVENT_CLASSES,
        &continuity.continuity_event_class,
        "continuity continuity_event_class",
    )?;
    ensure_token(
        CONTINUITY_STATE_CLASSES,
        &continuity.continuity_state_class,
        "continuity continuity_state_class",
    )?;
    Ok(())
}

fn validate_mapping_outcome(
    mapping: &MirrorImportMappingOutcome,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        mapping.record_kind.as_str(),
        MIRROR_IMPORT_MAPPING_OUTCOME_RECORD_KIND,
        "mapping_outcome record_kind",
    )?;
    ensure_token(
        MAPPING_OUTCOME_CLASSES,
        &mapping.outcome_class,
        "mapping_outcome outcome_class",
    )?;
    ensure_nonempty(
        &mapping.rollback_checkpoint_ref,
        "mapping_outcome rollback_checkpoint_ref",
    )?;
    ensure_nonempty(&mapping.diagnostics_ref, "mapping_outcome diagnostics_ref")?;
    Ok(())
}

fn validate_permission_posture(
    perm: &MirrorImportPermissionPosture,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        perm.record_kind.as_str(),
        MIRROR_IMPORT_PERMISSION_POSTURE_RECORD_KIND,
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
    compat: &MirrorImportCompatibility,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        compat.record_kind.as_str(),
        MIRROR_IMPORT_COMPATIBILITY_RECORD_KIND,
        "compatibility record_kind",
    )?;
    ensure_token(
        COMPATIBILITY_LABEL_CLASSES,
        &compat.compatibility_label_class,
        "compatibility compatibility_label_class",
    )?;
    ensure_token(
        COMPATIBILITY_EVIDENCE_SOURCE_CLASSES,
        &compat.evidence_source_class,
        "compatibility evidence_source_class",
    )?;
    Ok(())
}

fn validate_activation_budget(
    activation: &MirrorImportActivationBudget,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        activation.record_kind.as_str(),
        MIRROR_IMPORT_ACTIVATION_BUDGET_RECORD_KIND,
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
    inst: &MirrorImportInstallPosture,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        inst.record_kind.as_str(),
        MIRROR_IMPORT_INSTALL_POSTURE_RECORD_KIND,
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
    claim: &MirrorImportTruthQualificationClaim,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        MIRROR_IMPORT_TRUTH_QUALIFICATION_CLAIM_RECORD_KIND,
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
            MIRROR_IMPORT_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reason",
        )?;
    }
    Ok(())
}

fn validate_banner(
    banner: &DowngradedImportBanner,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        banner.record_kind.as_str(),
        DOWNGRADED_IMPORT_BANNER_RECORD_KIND,
        "banner record_kind",
    )?;
    if let Some(reason) = &banner.banner_reason_class {
        ensure_token(
            MIRROR_IMPORT_DOWNGRADE_REASONS,
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
    inspection: &StableMirrorImportTruthInspection,
    packet: &StableMirrorImportTruthPacket,
) -> Result<(), StableMirrorImportTruthValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        STABLE_MIRROR_IMPORT_TRUTH_INSPECTION_RECORD_KIND,
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
    if inspection.auto_update_safely_gated != packet.continuity.auto_update_safely_gated() {
        return Err(err("inspection auto_update_safely_gated is inconsistent"));
    }
    if inspection.mapping_stable_grade != packet.mapping_outcome.stable_grade() {
        return Err(err("inspection mapping_stable_grade is inconsistent"));
    }
    if inspection.permissions_not_widened == packet.permission_posture.widened_on_import {
        return Err(err("inspection permissions_not_widened is inconsistent"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation utilities
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> StableMirrorImportTruthValidationError {
    StableMirrorImportTruthValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(
    left: T,
    right: T,
    field: &str,
) -> Result<(), StableMirrorImportTruthValidationError>
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
) -> Result<(), StableMirrorImportTruthValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), StableMirrorImportTruthValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), StableMirrorImportTruthValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
