//! Stable runtime-class disclosure, active contribution inspectors, downgrade
//! banners, and hosted-surface boundary truth for extension ecosystem lanes.
//!
//! This module owns the stable packet consumed by marketplace rows, install and
//! update review, active contribution inspectors, extension-authored hosted
//! surfaces, diagnostics, local development / sideload / publish-preview flows,
//! and support exports. It keeps the runtime class, execution locus, host
//! boundary, contribution attribution, downgrade state, and hosted-surface
//! boundary facts in one machine-readable record so a contributed surface cannot
//! inherit trust from visual similarity or a generic extension badge.
//!
//! The companion schema lives at
//! [`schemas/extensions/runtime-class.schema.json`](../../../../schemas/extensions/runtime-class.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for every stable runtime-class truth packet.
pub const RUNTIME_CLASS_SCHEMA_VERSION: u32 = 1;

/// Canonical schema ref cited by packets, docs, diagnostics, and support exports.
pub const RUNTIME_CLASS_SCHEMA_REF: &str = "schemas/extensions/runtime-class.schema.json";

/// Record-kind tag for [`RuntimeClassTruthPacket`].
pub const RUNTIME_CLASS_TRUTH_PACKET_RECORD_KIND: &str = "extension_runtime_class_truth_packet";

/// Record-kind tag for [`RuntimeClassDisclosure`].
pub const RUNTIME_CLASS_DISCLOSURE_RECORD_KIND: &str = "extension_runtime_class_disclosure";

/// Record-kind tag for [`ActiveContributionInspector`].
pub const ACTIVE_CONTRIBUTION_INSPECTOR_RECORD_KIND: &str =
    "extension_active_contribution_inspector";

/// Record-kind tag for [`DowngradedHostBanner`].
pub const DOWNGRADED_HOST_BANNER_RECORD_KIND: &str = "extension_downgraded_host_banner";

/// Record-kind tag for [`HostedSurfaceBoundary`].
pub const HOSTED_SURFACE_BOUNDARY_RECORD_KIND: &str = "extension_hosted_surface_boundary";

/// Record-kind tag for [`AuthoringFlowDisclosure`].
pub const AUTHORING_FLOW_DISCLOSURE_RECORD_KIND: &str = "extension_authoring_flow_disclosure";

/// Record-kind tag for [`SurfaceTruthCoverage`].
pub const SURFACE_TRUTH_COVERAGE_RECORD_KIND: &str = "extension_surface_truth_coverage";

/// Record-kind tag for [`RuntimeClassTruthClaim`].
pub const RUNTIME_CLASS_TRUTH_CLAIM_RECORD_KIND: &str = "extension_runtime_class_truth_claim";

/// Record-kind tag for [`RuntimeClassTruthInspection`].
pub const RUNTIME_CLASS_TRUTH_INSPECTION_RECORD_KIND: &str =
    "extension_runtime_class_truth_inspection";

/// Record-kind tag for [`RuntimeClassTruthSupportExport`].
pub const RUNTIME_CLASS_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_runtime_class_truth_support_export";

/// Closed runtime-class vocabulary shared by marketplace, install review,
/// runtime inspectors, diagnostics, support exports, and authoring flows.
pub const RUNTIME_CLASSES: &[&str] = &[
    "passive_package",
    "wasm_capability_sandbox",
    "declarative_host_rendered_view",
    "external_host",
    "compatibility_bridge",
    "remote_side_component",
];

/// Closed execution-locus vocabulary.
pub const EXECUTION_LOCUS_CLASSES: &[&str] = &[
    "no_code_execution",
    "local_machine",
    "remote_target",
    "managed_host",
    "external_process",
    "browser_runtime",
];

/// Closed trust-tier vocabulary rendered independently from runtime class.
pub const TRUST_TIER_CLASSES: &[&str] = &[
    "verified_publisher",
    "enterprise_approved",
    "known_publisher",
    "community_unverified",
    "unsigned_local",
    "quarantined",
];

/// Closed publisher or signature-state vocabulary.
pub const SIGNATURE_STATE_CLASSES: &[&str] = &[
    "signed_verified",
    "signed_unverified",
    "unsigned_local_dev",
    "unsigned_sideload",
    "revoked_signature",
];

/// Closed host-health vocabulary shown by active contribution inspectors.
pub const HOST_HEALTH_CLASSES: &[&str] = &[
    "healthy",
    "degraded",
    "host_missing",
    "restarting",
    "quarantined",
    "unavailable",
];

/// Required inspector actions for every active contribution.
pub const REQUIRED_INSPECTOR_ACTIONS: &[&str] = &["pause", "restart", "quarantine"];

/// Closed active-inspector action vocabulary.
pub const INSPECTOR_ACTION_CLASSES: &[&str] = &[
    "pause",
    "restart",
    "quarantine",
    "open_logs",
    "disable",
    "review_permissions",
    "revert",
    "migrate",
];

/// Closed downgrade-reason vocabulary.
pub const DOWNGRADED_HOST_REASON_CLASSES: &[&str] = &[
    "native_host_unavailable",
    "platform_runtime_unavailable",
    "policy_blocked_native_runtime",
    "host_crash_loop",
    "sandbox_incompatible",
    "remote_host_unreachable",
    "browser_content_required",
];

/// Runtime classes that represent explicit fallback from a stronger native or
/// host-rendered contribution.
pub const FALLBACK_RUNTIME_CLASSES: &[&str] = &[
    "external_host",
    "compatibility_bridge",
    "remote_side_component",
];

/// Closed hosted-surface vocabulary.
pub const HOSTED_SURFACE_CLASSES: &[&str] = &[
    "extension_webview",
    "hosted_dashboard",
    "account_pane",
    "browser_runtime_bridge",
    "documentation_pane",
];

/// Closed open-in-browser fallback vocabulary.
pub const OPEN_IN_BROWSER_FALLBACK_CLASSES: &[&str] = &[
    "available",
    "recommended_safer",
    "blocked_by_policy",
    "unavailable_offline",
    "not_applicable",
];

/// Closed local-dev / sideload / publish-preview flow vocabulary.
pub const AUTHORING_FLOW_CLASSES: &[&str] =
    &["local_dev_workspace", "sideload_review", "publish_preview"];

/// Closed registry-binding vocabulary shared by public and local authoring flows.
pub const REGISTRY_BINDING_CLASSES: &[&str] = &[
    "public_registry",
    "approved_mirror",
    "offline_bundle",
    "stay_local",
    "bind_to_registry_later",
    "publish_preview_pending",
];

/// Required surfaces that must consume the packet before a stable claim can hold.
pub const REQUIRED_CONSUMER_SURFACES: &[&str] = &[
    "marketplace_result_row",
    "install_review",
    "active_contribution_inspector",
    "diagnostics",
    "support_export",
];

/// Closed consumer-surface vocabulary.
pub const CONSUMER_SURFACE_CLASSES: &[&str] = &[
    "marketplace_result_row",
    "marketplace_detail_page",
    "install_review",
    "active_ui_surface",
    "active_contribution_inspector",
    "diagnostics",
    "support_export",
    "hosted_surface_chrome",
    "local_dev_workspace",
    "sideload_review",
    "publish_preview",
    "docs_help_surface",
];

/// Closed support tiers for the derived claim.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Closed claim-basis vocabulary.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_runtime_class_and_hosted_surface_truth_claim",
    "beta_runtime_class_and_hosted_surface_partial_claim",
    "preview_runtime_class_and_hosted_surface_experimental_claim",
    "withdrawn_no_runtime_class_or_hosted_surface_truth_claim",
];

/// Closed downgrade-reason vocabulary for automatic narrowing.
pub const RUNTIME_CLASS_TRUTH_DOWNGRADE_REASONS: &[&str] = &[
    "catalog_only_truth_not_evidence_backed",
    "missing_required_consumer_surface",
    "runtime_class_unverified",
    "runtime_class_vocabulary_missing",
    "surface_truth_incomplete",
    "active_contribution_inspector_missing",
    "inspector_attribution_incomplete",
    "inspector_actions_incomplete",
    "downgraded_host_banner_missing",
    "downgraded_host_active",
    "hosted_surface_boundary_undisclosed",
    "hosted_surface_handoff_missing",
    "authoring_vocabulary_drift",
    "support_export_missing",
];

const WITHDRAWN_REASONS: &[&str] = &[
    "runtime_class_vocabulary_missing",
    "runtime_class_unverified",
    "active_contribution_inspector_missing",
    "hosted_surface_boundary_undisclosed",
    "support_export_missing",
];

const PREVIEW_REASONS: &[&str] = &[
    "catalog_only_truth_not_evidence_backed",
    "missing_required_consumer_surface",
    "surface_truth_incomplete",
    "inspector_attribution_incomplete",
    "inspector_actions_incomplete",
    "downgraded_host_banner_missing",
    "hosted_surface_handoff_missing",
    "authoring_vocabulary_drift",
];

/// Returns the stable user-facing label for a runtime-class token.
pub fn runtime_class_label(runtime_class: &str) -> Option<&'static str> {
    match runtime_class {
        "passive_package" => Some("Passive package"),
        "wasm_capability_sandbox" => Some("Wasm capability sandbox"),
        "declarative_host_rendered_view" => Some("Declarative/host-rendered view"),
        "external_host" => Some("External host"),
        "compatibility_bridge" => Some("Compatibility bridge"),
        "remote_side_component" => Some("Remote-side component"),
        _ => None,
    }
}

/// Input describing a stable runtime-class truth packet to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Runtime-class disclosures used by marketplace, install review, live
    /// surfaces, diagnostics, and support exports.
    #[serde(default)]
    pub runtime_disclosures: Vec<RuntimeClassDisclosureInput>,
    /// Active contribution inspectors for currently installed or running
    /// contributions.
    #[serde(default)]
    pub active_inspectors: Vec<ActiveContributionInspectorInput>,
    /// Downgraded-host banners shown for fallback scenarios.
    #[serde(default)]
    pub downgraded_host_banners: Vec<DowngradedHostBannerInput>,
    /// Hosted-surface boundary rows.
    #[serde(default)]
    pub hosted_surfaces: Vec<HostedSurfaceBoundaryInput>,
    /// Local development, sideload, and publish-preview rows.
    #[serde(default)]
    pub authoring_flows: Vec<AuthoringFlowDisclosureInput>,
    /// Cross-surface coverage assertions.
    #[serde(default)]
    pub surface_coverage: Vec<SurfaceTruthCoverageInput>,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Claim input for the visible tier.
    pub claim: RuntimeClassTruthClaimInput,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`RuntimeClassDisclosure`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassDisclosureInput {
    /// Package id.
    pub package_id: String,
    /// Package version.
    pub package_version: String,
    /// Publisher or signature state.
    pub publisher_or_signature_state: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Whether the runtime class is verified for this package and profile.
    pub runtime_class_verified: bool,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Trust tier.
    pub trust_tier_class: String,
    /// Plain-language host boundary summary.
    pub host_boundary_summary: String,
    /// Plain-language permission summary.
    pub permission_summary: String,
    /// Registry or local binding class.
    pub registry_binding_class: String,
    /// Previous runtime class when the contribution has downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_from_runtime_class: Option<String>,
}

/// Input for [`ActiveContributionInspector`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveContributionInspectorInput {
    /// Stable inspector id.
    pub inspector_id: String,
    /// Surface or contribution ref being inspected.
    pub surface_ref: String,
    /// Package id.
    pub package_id: String,
    /// Package version.
    pub package_version: String,
    /// Publisher or signature state.
    pub publisher_or_signature_state: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Trust tier.
    pub trust_tier_class: String,
    /// Permissions used during the current session.
    #[serde(default)]
    pub permissions_used_this_session: Vec<String>,
    /// Current host health.
    pub current_host_health_class: String,
    /// Current host identity or last-known-good host ref.
    pub host_identity_ref: String,
    /// Recent event refs.
    #[serde(default)]
    pub recent_event_refs: Vec<String>,
    /// Inspector actions available to the user.
    pub actions: Vec<String>,
}

/// Input for [`DowngradedHostBanner`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedHostBannerInput {
    /// Stable banner id.
    pub banner_id: String,
    /// Affected package id.
    pub package_id: String,
    /// Surface where the banner renders.
    pub surface_ref: String,
    /// Previous runtime class.
    pub previous_runtime_class: String,
    /// Current runtime class.
    pub current_runtime_class: String,
    /// Reason class.
    pub reason_class: String,
    /// Plain-language feature-loss summary.
    pub feature_loss_summary: String,
    /// Recovery choices shown to the user.
    pub recovery_choices: Vec<String>,
    /// Whether migrate is available.
    pub migrate_choice_available: bool,
    /// Whether revert is available.
    pub revert_choice_available: bool,
}

/// Input for [`HostedSurfaceBoundary`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostedSurfaceBoundaryInput {
    /// Stable surface id.
    pub surface_id: String,
    /// Hosted-surface class.
    pub hosted_surface_class: String,
    /// Owning package id.
    pub owner_package_id: String,
    /// Publisher label or state.
    pub publisher_or_signature_state: String,
    /// Exact origin or opaque origin ref.
    pub origin_ref: String,
    /// Runtime class backing the hosted surface.
    pub runtime_class: String,
    /// Whether owner/origin chrome is visible.
    pub owner_origin_chrome_visible: bool,
    /// Boundary and egress summary.
    pub boundary_egress_summary: String,
    /// Storage/cookie posture.
    pub storage_cookie_posture: String,
    /// Accessibility note.
    pub accessibility_note: String,
    /// Theming note.
    pub theming_note: String,
    /// Open-in-browser fallback posture.
    pub open_in_browser_fallback_class: String,
    /// Whether a safer external handoff is available or required.
    pub safer_external_handoff_available: bool,
}

/// Input for [`AuthoringFlowDisclosure`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoringFlowDisclosureInput {
    /// Stable flow id.
    pub flow_id: String,
    /// Local-dev, sideload, or publish-preview class.
    pub flow_class: String,
    /// Package id.
    pub package_id: String,
    /// Runtime class.
    pub runtime_class: String,
    /// Permission summary.
    pub permission_summary: String,
    /// Rollback or last-known-good binding.
    pub rollback_binding_ref: String,
    /// Registry or local binding class.
    pub registry_binding_class: String,
    /// Whether the flow uses the same runtime, permission, rollback, and
    /// registry-binding vocabulary as public packages.
    pub uses_public_package_vocabulary: bool,
}

/// Input for [`SurfaceTruthCoverage`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthCoverageInput {
    /// Consumer surface.
    pub surface_class: String,
    /// Whether runtime class is visible on the surface.
    pub runtime_class_visible: bool,
    /// Whether host-boundary truth is visible on the surface.
    pub host_boundary_visible: bool,
    /// Whether contribution attribution is visible on the surface.
    pub contribution_attribution_visible: bool,
}

/// Input for [`RuntimeClassTruthClaim`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthClaimInput {
    /// Claimed tier.
    pub claimed_tier: String,
    /// Claim basis.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Runtime-class disclosure shared across public and local package surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassDisclosure {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Package id.
    pub package_id: String,
    /// Package version.
    pub package_version: String,
    /// Publisher or signature state.
    pub publisher_or_signature_state: String,
    /// Runtime class.
    pub runtime_class: String,
    /// User-facing runtime label.
    pub runtime_class_label: String,
    /// Whether the runtime class is verified for this package and profile.
    pub runtime_class_verified: bool,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Trust tier.
    pub trust_tier_class: String,
    /// Plain-language host boundary summary.
    pub host_boundary_summary: String,
    /// Plain-language permission summary.
    pub permission_summary: String,
    /// Registry or local binding class.
    pub registry_binding_class: String,
    /// Previous runtime class when the contribution has downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_from_runtime_class: Option<String>,
}

impl RuntimeClassDisclosure {
    /// Builds a disclosure row from fixture or caller input.
    pub fn from_input(
        input: RuntimeClassDisclosureInput,
    ) -> Result<Self, RuntimeClassTruthValidationError> {
        let Some(label) = runtime_class_label(&input.runtime_class) else {
            return Err(err(format!(
                "runtime_class must be one of {RUNTIME_CLASSES:?}, got {}",
                input.runtime_class
            )));
        };
        Ok(Self {
            record_kind: RUNTIME_CLASS_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            package_id: input.package_id,
            package_version: input.package_version,
            publisher_or_signature_state: input.publisher_or_signature_state,
            runtime_class: input.runtime_class,
            runtime_class_label: label.to_string(),
            runtime_class_verified: input.runtime_class_verified,
            execution_locus_class: input.execution_locus_class,
            trust_tier_class: input.trust_tier_class,
            host_boundary_summary: input.host_boundary_summary,
            permission_summary: input.permission_summary,
            registry_binding_class: input.registry_binding_class,
            downgraded_from_runtime_class: input.downgraded_from_runtime_class,
        })
    }

    /// Returns true when the disclosure names an active downgraded host.
    pub fn downgraded(&self) -> bool {
        self.downgraded_from_runtime_class.is_some()
            && FALLBACK_RUNTIME_CLASSES.contains(&self.runtime_class.as_str())
    }
}

/// Active contribution inspector row used from live surfaces and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveContributionInspector {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Surface or contribution ref being inspected.
    pub surface_ref: String,
    /// Package id.
    pub package_id: String,
    /// Package version.
    pub package_version: String,
    /// Publisher or signature state.
    pub publisher_or_signature_state: String,
    /// Runtime class.
    pub runtime_class: String,
    /// User-facing runtime label.
    pub runtime_class_label: String,
    /// Execution locus.
    pub execution_locus_class: String,
    /// Trust tier.
    pub trust_tier_class: String,
    /// Permissions used during the current session.
    pub permissions_used_this_session: Vec<String>,
    /// Current host health.
    pub current_host_health_class: String,
    /// Current host identity or last-known-good host ref.
    pub host_identity_ref: String,
    /// Recent event refs.
    pub recent_event_refs: Vec<String>,
    /// Inspector actions available to the user.
    pub actions: Vec<String>,
}

impl ActiveContributionInspector {
    /// Builds an active inspector row from fixture or caller input.
    pub fn from_input(
        input: ActiveContributionInspectorInput,
    ) -> Result<Self, RuntimeClassTruthValidationError> {
        let Some(label) = runtime_class_label(&input.runtime_class) else {
            return Err(err(format!(
                "inspector runtime_class must be one of {RUNTIME_CLASSES:?}, got {}",
                input.runtime_class
            )));
        };
        Ok(Self {
            record_kind: ACTIVE_CONTRIBUTION_INSPECTOR_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            inspector_id: input.inspector_id,
            surface_ref: input.surface_ref,
            package_id: input.package_id,
            package_version: input.package_version,
            publisher_or_signature_state: input.publisher_or_signature_state,
            runtime_class: input.runtime_class,
            runtime_class_label: label.to_string(),
            execution_locus_class: input.execution_locus_class,
            trust_tier_class: input.trust_tier_class,
            permissions_used_this_session: input.permissions_used_this_session,
            current_host_health_class: input.current_host_health_class,
            host_identity_ref: input.host_identity_ref,
            recent_event_refs: input.recent_event_refs,
            actions: input.actions,
        })
    }

    /// Returns true when package identity, version, publisher or signature state,
    /// runtime class, execution locus, trust tier, permissions, host state, and
    /// events are available to the user.
    pub fn attribution_complete(&self) -> bool {
        !self.package_id.trim().is_empty()
            && !self.package_version.trim().is_empty()
            && !self.publisher_or_signature_state.trim().is_empty()
            && !self.runtime_class.trim().is_empty()
            && !self.execution_locus_class.trim().is_empty()
            && !self.trust_tier_class.trim().is_empty()
            && !self.permissions_used_this_session.is_empty()
            && !self.host_identity_ref.trim().is_empty()
            && !self.recent_event_refs.is_empty()
    }

    /// Returns true when pause, restart, and quarantine actions are all present.
    pub fn required_actions_present(&self) -> bool {
        REQUIRED_INSPECTOR_ACTIONS
            .iter()
            .all(|action| self.actions.iter().any(|got| got == action))
    }
}

/// Banner shown when an extension contribution falls back to a lower-fidelity host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedHostBanner {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable banner id.
    pub banner_id: String,
    /// Affected package id.
    pub package_id: String,
    /// Surface where the banner renders.
    pub surface_ref: String,
    /// Previous runtime class.
    pub previous_runtime_class: String,
    /// Previous user-facing runtime label.
    pub previous_runtime_label: String,
    /// Current runtime class.
    pub current_runtime_class: String,
    /// Current user-facing runtime label.
    pub current_runtime_label: String,
    /// Reason class.
    pub reason_class: String,
    /// Plain-language feature-loss summary.
    pub feature_loss_summary: String,
    /// Recovery choices shown to the user.
    pub recovery_choices: Vec<String>,
    /// Whether migrate is available.
    pub migrate_choice_available: bool,
    /// Whether revert is available.
    pub revert_choice_available: bool,
}

impl DowngradedHostBanner {
    /// Builds a downgraded-host banner from fixture or caller input.
    pub fn from_input(
        input: DowngradedHostBannerInput,
    ) -> Result<Self, RuntimeClassTruthValidationError> {
        let Some(previous_label) = runtime_class_label(&input.previous_runtime_class) else {
            return Err(err(format!(
                "previous_runtime_class must be one of {RUNTIME_CLASSES:?}, got {}",
                input.previous_runtime_class
            )));
        };
        let Some(current_label) = runtime_class_label(&input.current_runtime_class) else {
            return Err(err(format!(
                "current_runtime_class must be one of {RUNTIME_CLASSES:?}, got {}",
                input.current_runtime_class
            )));
        };
        Ok(Self {
            record_kind: DOWNGRADED_HOST_BANNER_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            banner_id: input.banner_id,
            package_id: input.package_id,
            surface_ref: input.surface_ref,
            previous_runtime_class: input.previous_runtime_class,
            previous_runtime_label: previous_label.to_string(),
            current_runtime_class: input.current_runtime_class,
            current_runtime_label: current_label.to_string(),
            reason_class: input.reason_class,
            feature_loss_summary: input.feature_loss_summary,
            recovery_choices: input.recovery_choices,
            migrate_choice_available: input.migrate_choice_available,
            revert_choice_available: input.revert_choice_available,
        })
    }

    /// Returns true when the banner names a runtime change, reason, feature loss,
    /// and at least one recovery choice.
    pub fn complete(&self) -> bool {
        self.previous_runtime_class != self.current_runtime_class
            && !self.reason_class.trim().is_empty()
            && !self.feature_loss_summary.trim().is_empty()
            && !self.recovery_choices.is_empty()
            && (self.migrate_choice_available || self.revert_choice_available)
    }
}

/// Hosted-surface boundary row for webviews, dashboards, account panes, and
/// browser-runtime bridges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostedSurfaceBoundary {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable surface id.
    pub surface_id: String,
    /// Hosted-surface class.
    pub hosted_surface_class: String,
    /// Owning package id.
    pub owner_package_id: String,
    /// Publisher label or state.
    pub publisher_or_signature_state: String,
    /// Exact origin or opaque origin ref.
    pub origin_ref: String,
    /// Runtime class backing the hosted surface.
    pub runtime_class: String,
    /// User-facing runtime label.
    pub runtime_class_label: String,
    /// Whether owner/origin chrome is visible.
    pub owner_origin_chrome_visible: bool,
    /// Boundary and egress summary.
    pub boundary_egress_summary: String,
    /// Storage/cookie posture.
    pub storage_cookie_posture: String,
    /// Accessibility note.
    pub accessibility_note: String,
    /// Theming note.
    pub theming_note: String,
    /// Open-in-browser fallback posture.
    pub open_in_browser_fallback_class: String,
    /// Whether a safer external handoff is available or required.
    pub safer_external_handoff_available: bool,
}

impl HostedSurfaceBoundary {
    /// Builds a hosted-surface boundary row from fixture or caller input.
    pub fn from_input(
        input: HostedSurfaceBoundaryInput,
    ) -> Result<Self, RuntimeClassTruthValidationError> {
        let Some(label) = runtime_class_label(&input.runtime_class) else {
            return Err(err(format!(
                "hosted runtime_class must be one of {RUNTIME_CLASSES:?}, got {}",
                input.runtime_class
            )));
        };
        Ok(Self {
            record_kind: HOSTED_SURFACE_BOUNDARY_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            surface_id: input.surface_id,
            hosted_surface_class: input.hosted_surface_class,
            owner_package_id: input.owner_package_id,
            publisher_or_signature_state: input.publisher_or_signature_state,
            origin_ref: input.origin_ref,
            runtime_class: input.runtime_class,
            runtime_class_label: label.to_string(),
            owner_origin_chrome_visible: input.owner_origin_chrome_visible,
            boundary_egress_summary: input.boundary_egress_summary,
            storage_cookie_posture: input.storage_cookie_posture,
            accessibility_note: input.accessibility_note,
            theming_note: input.theming_note,
            open_in_browser_fallback_class: input.open_in_browser_fallback_class,
            safer_external_handoff_available: input.safer_external_handoff_available,
        })
    }

    /// Returns true when owner/origin chrome, boundary and egress, storage/cookie,
    /// accessibility, theming, and fallback facts are all present.
    pub fn boundary_complete(&self) -> bool {
        self.owner_origin_chrome_visible
            && !self.owner_package_id.trim().is_empty()
            && !self.origin_ref.trim().is_empty()
            && !self.boundary_egress_summary.trim().is_empty()
            && !self.storage_cookie_posture.trim().is_empty()
            && !self.accessibility_note.trim().is_empty()
            && !self.theming_note.trim().is_empty()
            && OPEN_IN_BROWSER_FALLBACK_CLASSES
                .contains(&self.open_in_browser_fallback_class.as_str())
    }

    /// Returns true when a safer external handoff has a usable in-product action.
    pub fn handoff_complete(&self) -> bool {
        !self.safer_external_handoff_available
            || matches!(
                self.open_in_browser_fallback_class.as_str(),
                "available" | "recommended_safer"
            )
    }
}

/// Local-dev, sideload, and publish-preview disclosure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoringFlowDisclosure {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable flow id.
    pub flow_id: String,
    /// Local-dev, sideload, or publish-preview class.
    pub flow_class: String,
    /// Package id.
    pub package_id: String,
    /// Runtime class.
    pub runtime_class: String,
    /// User-facing runtime label.
    pub runtime_class_label: String,
    /// Permission summary.
    pub permission_summary: String,
    /// Rollback or last-known-good binding.
    pub rollback_binding_ref: String,
    /// Registry or local binding class.
    pub registry_binding_class: String,
    /// Whether the flow uses the same runtime, permission, rollback, and
    /// registry-binding vocabulary as public packages.
    pub uses_public_package_vocabulary: bool,
}

impl AuthoringFlowDisclosure {
    /// Builds an authoring-flow disclosure row from fixture or caller input.
    pub fn from_input(
        input: AuthoringFlowDisclosureInput,
    ) -> Result<Self, RuntimeClassTruthValidationError> {
        let Some(label) = runtime_class_label(&input.runtime_class) else {
            return Err(err(format!(
                "authoring runtime_class must be one of {RUNTIME_CLASSES:?}, got {}",
                input.runtime_class
            )));
        };
        Ok(Self {
            record_kind: AUTHORING_FLOW_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            flow_id: input.flow_id,
            flow_class: input.flow_class,
            package_id: input.package_id,
            runtime_class: input.runtime_class,
            runtime_class_label: label.to_string(),
            permission_summary: input.permission_summary,
            rollback_binding_ref: input.rollback_binding_ref,
            registry_binding_class: input.registry_binding_class,
            uses_public_package_vocabulary: input.uses_public_package_vocabulary,
        })
    }
}

/// Cross-surface coverage row proving runtime and contribution truth remains
/// visible before install, after install, after downgrade, and in export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthCoverage {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Consumer surface.
    pub surface_class: String,
    /// Whether runtime class is visible on the surface.
    pub runtime_class_visible: bool,
    /// Whether host-boundary truth is visible on the surface.
    pub host_boundary_visible: bool,
    /// Whether contribution attribution is visible on the surface.
    pub contribution_attribution_visible: bool,
}

impl SurfaceTruthCoverage {
    /// Builds a surface-coverage row.
    pub fn from_input(input: SurfaceTruthCoverageInput) -> Self {
        Self {
            record_kind: SURFACE_TRUTH_COVERAGE_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            surface_class: input.surface_class,
            runtime_class_visible: input.runtime_class_visible,
            host_boundary_visible: input.host_boundary_visible,
            contribution_attribution_visible: input.contribution_attribution_visible,
        }
    }

    /// Returns true when this surface carries all required truth.
    pub fn complete(&self) -> bool {
        self.runtime_class_visible
            && self.host_boundary_visible
            && self.contribution_attribution_visible
    }
}

/// Derived visible claim for runtime-class and hosted-surface truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthClaim {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after automatic narrowing.
    pub effective_tier: String,
    /// Claim basis.
    pub claim_basis_class: String,
    /// Support-claim vocabulary.
    pub support_claim_class: String,
    /// Automatic downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// Whether the claimed tier was narrowed.
    pub downgraded: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Derived inspection row for conformance, diagnostics, and tests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthInspection {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Whether all required consumers are present.
    pub required_consumers_present: bool,
    /// Whether every runtime class is verified and has a closed label.
    pub runtime_classes_verified: bool,
    /// Whether active inspectors are present.
    pub active_inspectors_present: bool,
    /// Whether all inspector attribution is complete.
    pub inspector_attribution_complete: bool,
    /// Whether all required inspector actions are present.
    pub inspector_actions_complete: bool,
    /// Whether every active downgrade has a complete banner.
    pub downgrade_banners_complete: bool,
    /// Whether a downgraded host is currently active.
    pub downgraded_host_active: bool,
    /// Whether hosted-surface boundaries are complete.
    pub hosted_surface_boundaries_complete: bool,
    /// Whether hosted-surface handoffs are complete.
    pub hosted_surface_handoffs_complete: bool,
    /// Whether authoring flows use the public package vocabulary.
    pub authoring_flows_use_public_vocabulary: bool,
    /// Whether surface coverage is complete.
    pub surface_coverage_complete: bool,
    /// Number of runtime disclosures.
    pub runtime_disclosure_count: usize,
    /// Number of active inspectors.
    pub active_inspector_count: usize,
    /// Number of downgrade banners.
    pub downgraded_banner_count: usize,
    /// Number of hosted surfaces.
    pub hosted_surface_count: usize,
}

/// Stable packet preserving runtime-class and hosted-surface truth across all
/// ecosystem consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Schema ref.
    pub schema_ref: String,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Runtime-class disclosures.
    pub runtime_disclosures: Vec<RuntimeClassDisclosure>,
    /// Active contribution inspectors.
    pub active_inspectors: Vec<ActiveContributionInspector>,
    /// Downgraded-host banners.
    pub downgraded_host_banners: Vec<DowngradedHostBanner>,
    /// Hosted-surface boundary rows.
    pub hosted_surfaces: Vec<HostedSurfaceBoundary>,
    /// Local development, sideload, and publish-preview rows.
    pub authoring_flows: Vec<AuthoringFlowDisclosure>,
    /// Cross-surface coverage rows.
    pub surface_coverage: Vec<SurfaceTruthCoverage>,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Derived claim.
    pub claim: RuntimeClassTruthClaim,
    /// Derived inspection.
    pub inspection: RuntimeClassTruthInspection,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
    /// Guardrail flag: catalog-only truth may never back Stable.
    pub allows_catalog_only_truth: bool,
    /// Guardrail flag: hidden hosted boundaries may never back Stable.
    pub allows_hidden_hosted_surface_boundary: bool,
    /// Guardrail flag: generic extension wording may never hide runtime class.
    pub allows_generic_extension_badge_only: bool,
}

impl RuntimeClassTruthPacket {
    /// Builds a runtime-class truth packet and derives its effective tier.
    pub fn from_input(input: RuntimeClassTruthInput) -> Result<Self, RuntimeClassTruthError> {
        let runtime_disclosures = input
            .runtime_disclosures
            .into_iter()
            .map(RuntimeClassDisclosure::from_input)
            .collect::<Result<Vec<_>, _>>()?;
        let active_inspectors = input
            .active_inspectors
            .into_iter()
            .map(ActiveContributionInspector::from_input)
            .collect::<Result<Vec<_>, _>>()?;
        let downgraded_host_banners = input
            .downgraded_host_banners
            .into_iter()
            .map(DowngradedHostBanner::from_input)
            .collect::<Result<Vec<_>, _>>()?;
        let hosted_surfaces = input
            .hosted_surfaces
            .into_iter()
            .map(HostedSurfaceBoundary::from_input)
            .collect::<Result<Vec<_>, _>>()?;
        let authoring_flows = input
            .authoring_flows
            .into_iter()
            .map(AuthoringFlowDisclosure::from_input)
            .collect::<Result<Vec<_>, _>>()?;
        let surface_coverage = input
            .surface_coverage
            .into_iter()
            .map(SurfaceTruthCoverage::from_input)
            .collect::<Vec<_>>();

        let downgrade_reasons = derive_downgrade_reasons(
            &runtime_disclosures,
            &active_inspectors,
            &downgraded_host_banners,
            &hosted_surfaces,
            &authoring_flows,
            &surface_coverage,
            &input.consumer_surfaces,
            &input.claim,
        );
        let effective_tier = derive_effective_tier(&input.claim.claimed_tier, &downgrade_reasons);
        let support_claim_class = support_claim_for_tier(&effective_tier).to_string();
        let claim = RuntimeClassTruthClaim {
            record_kind: RUNTIME_CLASS_TRUTH_CLAIM_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            claimed_tier: input.claim.claimed_tier,
            effective_tier,
            claim_basis_class: input.claim.claim_basis_class,
            support_claim_class,
            downgraded: !downgrade_reasons.is_empty(),
            downgrade_reasons,
            summary_label: input.claim.summary_label,
        };
        let inspection = derive_inspection(
            &input.packet_id,
            &claim,
            &runtime_disclosures,
            &active_inspectors,
            &downgraded_host_banners,
            &hosted_surfaces,
            &authoring_flows,
            &surface_coverage,
            &input.consumer_surfaces,
        );
        let packet = Self {
            record_kind: RUNTIME_CLASS_TRUTH_PACKET_RECORD_KIND.to_string(),
            schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
            schema_ref: RUNTIME_CLASS_SCHEMA_REF.to_string(),
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            runtime_disclosures,
            active_inspectors,
            downgraded_host_banners,
            hosted_surfaces,
            authoring_flows,
            surface_coverage,
            consumer_surfaces: input.consumer_surfaces,
            claim,
            inspection,
            summary_label: input.summary_label,
            allows_catalog_only_truth: false,
            allows_hidden_hosted_surface_boundary: false,
            allows_generic_extension_badge_only: false,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates packet consistency and closed vocabulary.
    pub fn validate(&self) -> Result<(), RuntimeClassTruthValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            RUNTIME_CLASS_TRUTH_PACKET_RECORD_KIND,
            "packet record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            RUNTIME_CLASS_SCHEMA_VERSION,
            "packet schema_version",
        )?;
        ensure_eq(
            self.schema_ref.as_str(),
            RUNTIME_CLASS_SCHEMA_REF,
            "packet schema_ref",
        )?;
        ensure_nonempty(&self.packet_id, "packet packet_id")?;
        ensure_nonempty(&self.generated_at, "packet generated_at")?;
        ensure_nonempty(&self.summary_label, "packet summary_label")?;
        for surface in &self.consumer_surfaces {
            ensure_token(CONSUMER_SURFACE_CLASSES, surface, "consumer_surfaces")?;
        }
        for row in &self.runtime_disclosures {
            validate_runtime_disclosure(row)?;
        }
        for inspector in &self.active_inspectors {
            validate_active_inspector(inspector)?;
        }
        for banner in &self.downgraded_host_banners {
            validate_downgraded_banner(banner)?;
        }
        for surface in &self.hosted_surfaces {
            validate_hosted_surface(surface)?;
        }
        for flow in &self.authoring_flows {
            validate_authoring_flow(flow)?;
        }
        for coverage in &self.surface_coverage {
            validate_surface_coverage(coverage)?;
        }
        validate_claim(&self.claim)?;
        validate_inspection(self)?;
        if self.allows_catalog_only_truth {
            return Err(err("allows_catalog_only_truth must remain false"));
        }
        if self.allows_hidden_hosted_surface_boundary {
            return Err(err(
                "allows_hidden_hosted_surface_boundary must remain false",
            ));
        }
        if self.allows_generic_extension_badge_only {
            return Err(err("allows_generic_extension_badge_only must remain false"));
        }
        Ok(())
    }

    /// Returns true when every active downgrade has a banner naming the package
    /// and old/new runtime class.
    pub fn every_downgrade_has_banner(&self) -> bool {
        every_downgrade_has_banner(&self.runtime_disclosures, &self.downgraded_host_banners)
    }
}

/// Projection returned by [`project_runtime_class_truth`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthProjection {
    /// Packet id.
    pub packet_id: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Runtime classes observed in the packet.
    pub runtime_classes: Vec<String>,
    /// Whether a downgraded host is active.
    pub downgraded_host_active: bool,
    /// Whether hosted boundaries are complete.
    pub hosted_surface_boundaries_complete: bool,
    /// Whether the row blocks a stable runtime-truth claim.
    pub blocks_stable_runtime_truth: bool,
}

/// Metadata-safe support export for runtime-class and hosted-surface truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClassTruthSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id ref.
    pub packet_id_ref: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Runtime class tokens included in the export.
    pub runtime_classes: Vec<String>,
    /// Active inspector ids included in the export.
    pub active_inspector_refs: Vec<String>,
    /// Downgrade banner ids included in the export.
    pub downgraded_banner_refs: Vec<String>,
    /// Hosted surface ids included in the export.
    pub hosted_surface_refs: Vec<String>,
    /// Downgrade reasons.
    pub downgrade_reasons: Vec<String>,
    /// Whether the support export blocks a stable runtime-truth claim.
    pub blocks_stable_runtime_truth: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Error returned when input construction or projection fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeClassTruthError {
    /// Error message.
    pub message: String,
}

impl fmt::Display for RuntimeClassTruthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for RuntimeClassTruthError {}

impl From<RuntimeClassTruthValidationError> for RuntimeClassTruthError {
    fn from(value: RuntimeClassTruthValidationError) -> Self {
        Self {
            message: value.message,
        }
    }
}

/// Error returned when a runtime-class truth packet fails validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeClassTruthValidationError {
    /// Error message.
    pub message: String,
}

impl fmt::Display for RuntimeClassTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for RuntimeClassTruthValidationError {}

/// Projects a JSON packet into the minimal consumer-facing runtime-truth view.
pub fn project_runtime_class_truth(
    json: &str,
) -> Result<RuntimeClassTruthProjection, RuntimeClassTruthError> {
    let packet: RuntimeClassTruthPacket =
        serde_json::from_str(json).map_err(|e| RuntimeClassTruthError {
            message: format!("runtime-class truth packet JSON did not parse: {e}"),
        })?;
    packet.validate()?;
    let mut runtime_classes = packet
        .runtime_disclosures
        .iter()
        .map(|row| row.runtime_class.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    runtime_classes.sort();
    Ok(RuntimeClassTruthProjection {
        packet_id: packet.packet_id,
        effective_tier: packet.claim.effective_tier.clone(),
        runtime_classes,
        downgraded_host_active: packet.inspection.downgraded_host_active,
        hosted_surface_boundaries_complete: packet.inspection.hosted_surface_boundaries_complete,
        blocks_stable_runtime_truth: packet.claim.effective_tier != "stable",
    })
}

/// Projects a metadata-safe support export from a packet.
pub fn project_runtime_class_truth_support_export(
    packet: &RuntimeClassTruthPacket,
) -> RuntimeClassTruthSupportExport {
    let mut runtime_classes = packet
        .runtime_disclosures
        .iter()
        .map(|row| row.runtime_class.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    runtime_classes.sort();
    RuntimeClassTruthSupportExport {
        record_kind: RUNTIME_CLASS_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
        packet_id_ref: packet.packet_id.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        runtime_classes,
        active_inspector_refs: packet
            .active_inspectors
            .iter()
            .map(|row| row.inspector_id.clone())
            .collect(),
        downgraded_banner_refs: packet
            .downgraded_host_banners
            .iter()
            .map(|row| row.banner_id.clone())
            .collect(),
        hosted_surface_refs: packet
            .hosted_surfaces
            .iter()
            .map(|row| row.surface_id.clone())
            .collect(),
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        blocks_stable_runtime_truth: packet.claim.effective_tier != "stable",
        summary_label: packet.summary_label.clone(),
    }
}

fn derive_downgrade_reasons(
    runtime_disclosures: &[RuntimeClassDisclosure],
    active_inspectors: &[ActiveContributionInspector],
    banners: &[DowngradedHostBanner],
    hosted_surfaces: &[HostedSurfaceBoundary],
    authoring_flows: &[AuthoringFlowDisclosure],
    surface_coverage: &[SurfaceTruthCoverage],
    consumer_surfaces: &[String],
    claim: &RuntimeClassTruthClaimInput,
) -> Vec<String> {
    let mut reasons = BTreeSet::new();
    if claim.claim_basis_class == "catalog_asserted_only" {
        reasons.insert("catalog_only_truth_not_evidence_backed".to_string());
    }
    for required in REQUIRED_CONSUMER_SURFACES {
        if !consumer_surfaces.iter().any(|surface| surface == required) {
            reasons.insert("missing_required_consumer_surface".to_string());
        }
    }
    if !consumer_surfaces
        .iter()
        .any(|surface| surface == "support_export")
    {
        reasons.insert("support_export_missing".to_string());
    }
    if runtime_disclosures.is_empty() {
        reasons.insert("runtime_class_vocabulary_missing".to_string());
    }
    if runtime_disclosures
        .iter()
        .any(|row| !row.runtime_class_verified || runtime_class_label(&row.runtime_class).is_none())
    {
        reasons.insert("runtime_class_unverified".to_string());
    }
    if surface_coverage.is_empty() || surface_coverage.iter().any(|row| !row.complete()) {
        reasons.insert("surface_truth_incomplete".to_string());
    }
    if active_inspectors.is_empty() {
        reasons.insert("active_contribution_inspector_missing".to_string());
    }
    if active_inspectors
        .iter()
        .any(|inspector| !inspector.attribution_complete())
    {
        reasons.insert("inspector_attribution_incomplete".to_string());
    }
    if active_inspectors
        .iter()
        .any(|inspector| !inspector.required_actions_present())
    {
        reasons.insert("inspector_actions_incomplete".to_string());
    }
    if runtime_disclosures
        .iter()
        .any(RuntimeClassDisclosure::downgraded)
    {
        reasons.insert("downgraded_host_active".to_string());
        if !every_downgrade_has_banner(runtime_disclosures, banners) {
            reasons.insert("downgraded_host_banner_missing".to_string());
        }
    }
    if hosted_surfaces
        .iter()
        .any(|surface| !surface.boundary_complete())
    {
        reasons.insert("hosted_surface_boundary_undisclosed".to_string());
    }
    if hosted_surfaces
        .iter()
        .any(|surface| !surface.handoff_complete())
    {
        reasons.insert("hosted_surface_handoff_missing".to_string());
    }
    if authoring_flows
        .iter()
        .any(|flow| !flow.uses_public_package_vocabulary)
    {
        reasons.insert("authoring_vocabulary_drift".to_string());
    }
    reasons.into_iter().collect()
}

fn derive_effective_tier(claimed_tier: &str, reasons: &[String]) -> String {
    if reasons.is_empty() {
        return claimed_tier.to_string();
    }
    if reasons
        .iter()
        .any(|reason| WITHDRAWN_REASONS.contains(&reason.as_str()))
    {
        "withdrawn".to_string()
    } else if reasons
        .iter()
        .any(|reason| PREVIEW_REASONS.contains(&reason.as_str()))
    {
        "preview".to_string()
    } else {
        "beta".to_string()
    }
}

fn support_claim_for_tier(tier: &str) -> &'static str {
    match tier {
        "stable" => "stable_runtime_class_and_hosted_surface_truth_claim",
        "beta" => "beta_runtime_class_and_hosted_surface_partial_claim",
        "preview" => "preview_runtime_class_and_hosted_surface_experimental_claim",
        _ => "withdrawn_no_runtime_class_or_hosted_surface_truth_claim",
    }
}

fn derive_inspection(
    packet_id: &str,
    claim: &RuntimeClassTruthClaim,
    runtime_disclosures: &[RuntimeClassDisclosure],
    active_inspectors: &[ActiveContributionInspector],
    banners: &[DowngradedHostBanner],
    hosted_surfaces: &[HostedSurfaceBoundary],
    authoring_flows: &[AuthoringFlowDisclosure],
    surface_coverage: &[SurfaceTruthCoverage],
    consumer_surfaces: &[String],
) -> RuntimeClassTruthInspection {
    RuntimeClassTruthInspection {
        record_kind: RUNTIME_CLASS_TRUTH_INSPECTION_RECORD_KIND.to_string(),
        schema_version: RUNTIME_CLASS_SCHEMA_VERSION,
        packet_id_ref: packet_id.to_string(),
        effective_tier: claim.effective_tier.clone(),
        required_consumers_present: REQUIRED_CONSUMER_SURFACES
            .iter()
            .all(|required| consumer_surfaces.iter().any(|surface| surface == required)),
        runtime_classes_verified: !runtime_disclosures.is_empty()
            && runtime_disclosures.iter().all(|row| {
                row.runtime_class_verified && runtime_class_label(&row.runtime_class).is_some()
            }),
        active_inspectors_present: !active_inspectors.is_empty(),
        inspector_attribution_complete: !active_inspectors.is_empty()
            && active_inspectors
                .iter()
                .all(ActiveContributionInspector::attribution_complete),
        inspector_actions_complete: !active_inspectors.is_empty()
            && active_inspectors
                .iter()
                .all(ActiveContributionInspector::required_actions_present),
        downgrade_banners_complete: every_downgrade_has_banner(runtime_disclosures, banners),
        downgraded_host_active: runtime_disclosures
            .iter()
            .any(RuntimeClassDisclosure::downgraded),
        hosted_surface_boundaries_complete: hosted_surfaces
            .iter()
            .all(HostedSurfaceBoundary::boundary_complete),
        hosted_surface_handoffs_complete: hosted_surfaces
            .iter()
            .all(HostedSurfaceBoundary::handoff_complete),
        authoring_flows_use_public_vocabulary: authoring_flows
            .iter()
            .all(|flow| flow.uses_public_package_vocabulary),
        surface_coverage_complete: !surface_coverage.is_empty()
            && surface_coverage.iter().all(SurfaceTruthCoverage::complete),
        runtime_disclosure_count: runtime_disclosures.len(),
        active_inspector_count: active_inspectors.len(),
        downgraded_banner_count: banners.len(),
        hosted_surface_count: hosted_surfaces.len(),
    }
}

fn every_downgrade_has_banner(
    runtime_disclosures: &[RuntimeClassDisclosure],
    banners: &[DowngradedHostBanner],
) -> bool {
    runtime_disclosures
        .iter()
        .filter(|row| row.downgraded())
        .all(|row| {
            let previous = row
                .downgraded_from_runtime_class
                .as_deref()
                .unwrap_or_default();
            banners.iter().any(|banner| {
                banner.package_id == row.package_id
                    && banner.previous_runtime_class == previous
                    && banner.current_runtime_class == row.runtime_class
                    && banner.complete()
            })
        })
}

fn validate_runtime_disclosure(
    row: &RuntimeClassDisclosure,
) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        row.record_kind.as_str(),
        RUNTIME_CLASS_DISCLOSURE_RECORD_KIND,
        "runtime_disclosure record_kind",
    )?;
    ensure_eq_u32(
        row.schema_version,
        RUNTIME_CLASS_SCHEMA_VERSION,
        "runtime_disclosure schema_version",
    )?;
    ensure_nonempty(&row.package_id, "runtime_disclosure package_id")?;
    ensure_nonempty(&row.package_version, "runtime_disclosure package_version")?;
    ensure_token(
        SIGNATURE_STATE_CLASSES,
        &row.publisher_or_signature_state,
        "runtime_disclosure publisher_or_signature_state",
    )?;
    ensure_token(
        RUNTIME_CLASSES,
        &row.runtime_class,
        "runtime_disclosure runtime_class",
    )?;
    ensure_eq(
        row.runtime_class_label.as_str(),
        runtime_class_label(&row.runtime_class).unwrap_or_default(),
        "runtime_disclosure runtime_class_label",
    )?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &row.execution_locus_class,
        "runtime_disclosure execution_locus_class",
    )?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &row.trust_tier_class,
        "runtime_disclosure trust_tier_class",
    )?;
    ensure_token(
        REGISTRY_BINDING_CLASSES,
        &row.registry_binding_class,
        "runtime_disclosure registry_binding_class",
    )?;
    ensure_nonempty(
        &row.host_boundary_summary,
        "runtime_disclosure host_boundary_summary",
    )?;
    ensure_nonempty(
        &row.permission_summary,
        "runtime_disclosure permission_summary",
    )?;
    if let Some(previous) = &row.downgraded_from_runtime_class {
        ensure_token(
            RUNTIME_CLASSES,
            previous,
            "runtime_disclosure downgraded_from_runtime_class",
        )?;
    }
    Ok(())
}

fn validate_active_inspector(
    row: &ActiveContributionInspector,
) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        row.record_kind.as_str(),
        ACTIVE_CONTRIBUTION_INSPECTOR_RECORD_KIND,
        "active_inspector record_kind",
    )?;
    ensure_nonempty(&row.inspector_id, "active_inspector inspector_id")?;
    ensure_nonempty(&row.surface_ref, "active_inspector surface_ref")?;
    ensure_nonempty(&row.package_id, "active_inspector package_id")?;
    ensure_nonempty(&row.package_version, "active_inspector package_version")?;
    ensure_token(
        SIGNATURE_STATE_CLASSES,
        &row.publisher_or_signature_state,
        "active_inspector publisher_or_signature_state",
    )?;
    ensure_token(
        RUNTIME_CLASSES,
        &row.runtime_class,
        "active_inspector runtime_class",
    )?;
    ensure_token(
        EXECUTION_LOCUS_CLASSES,
        &row.execution_locus_class,
        "active_inspector execution_locus_class",
    )?;
    ensure_token(
        TRUST_TIER_CLASSES,
        &row.trust_tier_class,
        "active_inspector trust_tier_class",
    )?;
    ensure_token(
        HOST_HEALTH_CLASSES,
        &row.current_host_health_class,
        "active_inspector current_host_health_class",
    )?;
    for action in &row.actions {
        ensure_token(INSPECTOR_ACTION_CLASSES, action, "active_inspector actions")?;
    }
    Ok(())
}

fn validate_downgraded_banner(
    row: &DowngradedHostBanner,
) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        row.record_kind.as_str(),
        DOWNGRADED_HOST_BANNER_RECORD_KIND,
        "downgraded_banner record_kind",
    )?;
    ensure_nonempty(&row.banner_id, "downgraded_banner banner_id")?;
    ensure_nonempty(&row.package_id, "downgraded_banner package_id")?;
    ensure_nonempty(&row.surface_ref, "downgraded_banner surface_ref")?;
    ensure_token(
        RUNTIME_CLASSES,
        &row.previous_runtime_class,
        "downgraded_banner previous_runtime_class",
    )?;
    ensure_token(
        RUNTIME_CLASSES,
        &row.current_runtime_class,
        "downgraded_banner current_runtime_class",
    )?;
    ensure_token(
        DOWNGRADED_HOST_REASON_CLASSES,
        &row.reason_class,
        "downgraded_banner reason_class",
    )?;
    ensure_nonempty(
        &row.feature_loss_summary,
        "downgraded_banner feature_loss_summary",
    )?;
    if row.recovery_choices.is_empty() {
        return Err(err("downgraded_banner recovery_choices must not be empty"));
    }
    Ok(())
}

fn validate_hosted_surface(
    row: &HostedSurfaceBoundary,
) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        row.record_kind.as_str(),
        HOSTED_SURFACE_BOUNDARY_RECORD_KIND,
        "hosted_surface record_kind",
    )?;
    ensure_nonempty(&row.surface_id, "hosted_surface surface_id")?;
    ensure_token(
        HOSTED_SURFACE_CLASSES,
        &row.hosted_surface_class,
        "hosted_surface hosted_surface_class",
    )?;
    ensure_nonempty(&row.owner_package_id, "hosted_surface owner_package_id")?;
    ensure_token(
        SIGNATURE_STATE_CLASSES,
        &row.publisher_or_signature_state,
        "hosted_surface publisher_or_signature_state",
    )?;
    ensure_nonempty(&row.origin_ref, "hosted_surface origin_ref")?;
    ensure_token(
        RUNTIME_CLASSES,
        &row.runtime_class,
        "hosted_surface runtime_class",
    )?;
    ensure_token(
        OPEN_IN_BROWSER_FALLBACK_CLASSES,
        &row.open_in_browser_fallback_class,
        "hosted_surface open_in_browser_fallback_class",
    )?;
    Ok(())
}

fn validate_authoring_flow(
    row: &AuthoringFlowDisclosure,
) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        row.record_kind.as_str(),
        AUTHORING_FLOW_DISCLOSURE_RECORD_KIND,
        "authoring_flow record_kind",
    )?;
    ensure_nonempty(&row.flow_id, "authoring_flow flow_id")?;
    ensure_token(
        AUTHORING_FLOW_CLASSES,
        &row.flow_class,
        "authoring_flow flow_class",
    )?;
    ensure_nonempty(&row.package_id, "authoring_flow package_id")?;
    ensure_token(
        RUNTIME_CLASSES,
        &row.runtime_class,
        "authoring_flow runtime_class",
    )?;
    ensure_nonempty(&row.permission_summary, "authoring_flow permission_summary")?;
    ensure_nonempty(
        &row.rollback_binding_ref,
        "authoring_flow rollback_binding_ref",
    )?;
    ensure_token(
        REGISTRY_BINDING_CLASSES,
        &row.registry_binding_class,
        "authoring_flow registry_binding_class",
    )?;
    Ok(())
}

fn validate_surface_coverage(
    row: &SurfaceTruthCoverage,
) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        row.record_kind.as_str(),
        SURFACE_TRUTH_COVERAGE_RECORD_KIND,
        "surface_coverage record_kind",
    )?;
    ensure_token(
        CONSUMER_SURFACE_CLASSES,
        &row.surface_class,
        "surface_coverage surface_class",
    )?;
    Ok(())
}

fn validate_claim(claim: &RuntimeClassTruthClaim) -> Result<(), RuntimeClassTruthValidationError> {
    ensure_eq(
        claim.record_kind.as_str(),
        RUNTIME_CLASS_TRUTH_CLAIM_RECORD_KIND,
        "claim record_kind",
    )?;
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claim claimed_tier")?;
    ensure_token(
        STABILITY_TIERS,
        &claim.effective_tier,
        "claim effective_tier",
    )?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim claim_basis_class",
    )?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "claim support_claim_class",
    )?;
    for reason in &claim.downgrade_reasons {
        ensure_token(
            RUNTIME_CLASS_TRUTH_DOWNGRADE_REASONS,
            reason,
            "claim downgrade_reasons",
        )?;
    }
    Ok(())
}

fn validate_inspection(
    packet: &RuntimeClassTruthPacket,
) -> Result<(), RuntimeClassTruthValidationError> {
    let inspection = &packet.inspection;
    ensure_eq(
        inspection.record_kind.as_str(),
        RUNTIME_CLASS_TRUTH_INSPECTION_RECORD_KIND,
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
    if inspection.required_consumers_present
        != REQUIRED_CONSUMER_SURFACES.iter().all(|required| {
            packet
                .consumer_surfaces
                .iter()
                .any(|surface| surface == required)
        })
    {
        return Err(err("inspection required_consumers_present is inconsistent"));
    }
    if inspection.runtime_disclosure_count != packet.runtime_disclosures.len() {
        return Err(err("inspection runtime_disclosure_count is inconsistent"));
    }
    if inspection.active_inspector_count != packet.active_inspectors.len() {
        return Err(err("inspection active_inspector_count is inconsistent"));
    }
    if inspection.downgraded_banner_count != packet.downgraded_host_banners.len() {
        return Err(err("inspection downgraded_banner_count is inconsistent"));
    }
    if inspection.hosted_surface_count != packet.hosted_surfaces.len() {
        return Err(err("inspection hosted_surface_count is inconsistent"));
    }
    if inspection.downgrade_banners_complete != packet.every_downgrade_has_banner() {
        return Err(err("inspection downgrade_banners_complete is inconsistent"));
    }
    Ok(())
}

fn err(message: impl Into<String>) -> RuntimeClassTruthValidationError {
    RuntimeClassTruthValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T>(left: T, right: T, field: &str) -> Result<(), RuntimeClassTruthValidationError>
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
) -> Result<(), RuntimeClassTruthValidationError> {
    if left != right {
        return Err(err(format!(
            "{field} mismatch: expected {right}, got {left}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), RuntimeClassTruthValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), RuntimeClassTruthValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}
