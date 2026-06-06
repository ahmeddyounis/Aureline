//! Stabilize extension dependency resolution, effective permissions, publisher
//! continuity, revocation pins, and deprecation propagation into one exportable
//! stable ecosystem packet.
//!
//! Existing stable extension lanes prove individual slices: manifest hardening
//! exposes dependency declarations, lifecycle-flow hardening proves install and
//! rollback review, mirror-import truth proves publisher continuity for mirrored
//! rows, and SDK deprecation policy proves migration metadata. This module owns
//! the cross-surface packet that ties those facts together for public-registry,
//! approved-mirror, and enterprise-curated installs so UI, CLI/headless, mirror,
//! migration, release-evidence, and support-export consumers do not need side
//! channel knowledge.
//!
//! A stable packet must expose hard dependencies, optional integrations,
//! API/runtime ranges, deterministic lock/export state, the effective permission
//! set after dependency resolution, re-consent on authority widening, publisher
//! continuity workflow state, yanks/revocations/last-known-good pinning, and
//! deprecation propagation. The effective tier is derived from the posture every
//! time the packet is built or validated, so stale proof narrows automatically.
//!
//! The companion schema lives at
//! [`schemas/extensions/extension-dependency-resolution.schema.json`](../../../../schemas/extensions/extension-dependency-resolution.schema.json).
//! Canonical fixtures live under
//! `fixtures/extensions/m4/stabilize-extension-dependency-resolution-and-publisher-continuity/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the stable dependency-resolution and continuity packet.
pub const EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_VERSION: u32 = 1;

/// Published resolver/continuity contract version required for a stable claim.
pub const EXTENSION_DEPENDENCY_RESOLUTION_PUBLISHED_VERSION: u32 = 1;

/// Canonical schema path the packet cites.
pub const EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_REF: &str =
    "schemas/extensions/extension-dependency-resolution.schema.json";

/// Record kind for [`ExtensionDependencyResolutionPacket`].
pub const EXTENSION_DEPENDENCY_RESOLUTION_PACKET_RECORD_KIND: &str =
    "extension_dependency_resolution_packet";

/// Record kind for [`ExtensionDependencyResolutionSupportExport`].
pub const EXTENSION_DEPENDENCY_RESOLUTION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_dependency_resolution_support_export";

/// Closed install-flow vocabulary.
pub const INSTALL_FLOW_CLASSES: &[&str] = &[
    "public_install",
    "public_update",
    "mirrored_install",
    "mirrored_update",
    "enterprise_curated_install",
    "enterprise_curated_update",
    "rollback",
    "mirror_promotion",
];

/// Closed source-class vocabulary.
pub const SOURCE_CLASSES: &[&str] = &["public_registry", "approved_mirror", "enterprise_curated"];

/// Closed resolver-determinism vocabulary.
pub const RESOLVER_DETERMINISM_CLASSES: &[&str] =
    &["deterministic", "nondeterministic", "not_resolved"];

/// Closed dependency-edge vocabulary.
pub const DEPENDENCY_EDGE_CLASSES: &[&str] = &["root", "hard_dependency", "optional_integration"];

/// Closed dependency resolution-state vocabulary.
pub const DEPENDENCY_RESOLUTION_STATE_CLASSES: &[&str] = &[
    "resolved",
    "unresolved_missing",
    "version_conflict",
    "optional_absent",
];

/// Closed re-consent vocabulary.
pub const RECONSENT_STATE_CLASSES: &[&str] = &[
    "not_required",
    "required_obtained",
    "required_pending",
    "required_missing",
];

/// Closed publisher-continuity workflow vocabulary.
pub const CONTINUITY_WORKFLOW_CLASSES: &[&str] = &[
    "none",
    "key_rotation",
    "ownership_transfer",
    "namespace_dispute",
    "maintainer_removal",
    "orphan_adoption",
    "approved_mirror_succession",
];

/// Continuity workflows that move package authority and must gate high-trust update.
pub const AUTHORITY_MOVING_CONTINUITY_WORKFLOWS: &[&str] = &[
    "key_rotation",
    "ownership_transfer",
    "maintainer_removal",
    "orphan_adoption",
    "approved_mirror_succession",
];

/// Closed publisher-continuity state vocabulary.
pub const CONTINUITY_STATE_CLASSES: &[&str] = &[
    "current",
    "in_cooldown",
    "pending_review",
    "pending_notification",
    "disputed",
    "revoked",
    "stale",
    "missing",
];

/// Closed yank/revocation vocabulary.
pub const YANK_REVOCATION_STATE_CLASSES: &[&str] =
    &["clean", "advisory", "yanked", "quarantined", "revoked"];

/// Closed revocation propagation vocabulary.
pub const REVOCATION_PROPAGATION_CLASSES: &[&str] = &[
    "not_applicable",
    "propagated_all_sources",
    "partial",
    "not_propagated",
];

/// Closed downgrade/hold behavior vocabulary.
pub const DOWNGRADE_HOLD_BEHAVIOR_CLASSES: &[&str] = &[
    "explicit_policy_allowed",
    "explicit_policy_denied",
    "hidden_or_implicit",
];

/// Closed SDK/API deprecation state vocabulary.
pub const API_DEPRECATION_STATE_CLASSES: &[&str] =
    &["none", "warning", "sunset_scheduled", "removed"];

/// Closed stability-tier vocabulary.
pub const STABILITY_TIERS: &[&str] = &["stable", "beta", "preview", "withdrawn"];

/// Closed claim-basis vocabulary.
pub const CLAIM_BASIS_CLASSES: &[&str] = &["evidence_backed", "catalog_asserted_only"];

/// Closed support-claim vocabulary.
pub const SUPPORT_CLAIM_CLASSES: &[&str] = &[
    "stable_dependency_resolution_claim",
    "beta_dependency_resolution_partial_claim",
    "preview_dependency_resolution_experimental_claim",
    "withdrawn_no_dependency_resolution_claim",
];

/// Closed downgrade-reason vocabulary.
pub const DEPENDENCY_RESOLUTION_DOWNGRADE_REASONS: &[&str] = &[
    "contract_version_not_published",
    "catalog_only_claim_basis",
    "resolver_nondeterministic",
    "resolver_not_resolved",
    "unresolved_hard_dependency",
    "version_conflict_dependency",
    "lock_export_unavailable",
    "team_rollout_unsupported",
    "air_gapped_rollout_unsupported",
    "permission_widening_without_reconsent",
    "publisher_continuity_in_cooldown",
    "publisher_continuity_pending_review",
    "publisher_continuity_pending_notification",
    "publisher_continuity_disputed",
    "publisher_continuity_revoked",
    "publisher_continuity_missing_or_stale",
    "continuity_audit_missing",
    "continuity_notification_missing",
    "high_trust_auto_update_not_gated",
    "transfer_history_missing",
    "package_identity_not_preserved",
    "revocation_not_propagated",
    "last_known_good_pin_missing",
    "hold_or_downgrade_policy_hidden",
    "deprecation_not_propagated",
    "removed_api_without_shim",
    "claim_packet_stale_or_incomplete",
    "identity_parity_missing",
];

/// Consumer surfaces that ingest this packet.
pub const EXTENSION_DEPENDENCY_RESOLUTION_CONSUMER_SURFACES: &[&str] = &[
    "install_review",
    "update_review",
    "mirror_review",
    "rollback_review",
    "extension_inspector",
    "cli_headless",
    "support_export",
    "migration_docs",
    "claim_packet",
    "enterprise_admin_export",
];

/// Input for [`ExtensionDependencyResolutionPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDependencyResolutionInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity and install-flow posture.
    pub identity: ResolutionIdentityInput,
    /// Resolver output and lock/export posture.
    pub resolution: DependencyResolutionInput,
    /// Effective permission review input.
    pub permissions: EffectivePermissionReviewInput,
    /// Publisher continuity workflow input.
    pub continuity: PublisherContinuityInput,
    /// Yank, revocation, last-known-good pin, and rollback/hold input.
    pub revocation: RevocationPinInput,
    /// SDK/API deprecation propagation input.
    pub deprecation: DeprecationPropagationInput,
    /// Claimed tier and basis.
    pub claim: DependencyResolutionClaimInput,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Identity and install-flow input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionIdentityInput {
    /// Canonical package identity preserved across public, mirror, and enterprise lanes.
    pub package_identity: String,
    /// Package version under review.
    pub package_version: String,
    /// Install/update/mirror/rollback flow class.
    pub install_flow_class: String,
    /// Published resolver/continuity contract version pinned by the row.
    pub resolver_contract_version: u32,
    /// True when public, mirrored, and enterprise-curated projections preserve identity.
    pub identity_parity_across_sources: bool,
    /// Source classes proven by this packet.
    pub source_classes: Vec<String>,
    /// Public-row ref when applicable.
    pub public_row_ref: String,
    /// Mirror-row ref when applicable.
    pub mirror_row_ref: String,
    /// Enterprise-curated row ref when applicable.
    pub enterprise_row_ref: String,
}

/// Dependency-edge input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdgeInput {
    /// Stable edge id.
    pub edge_id: String,
    /// Edge class.
    pub edge_class: String,
    /// Target package ref.
    pub target_ref: String,
    /// Version range ref.
    pub version_range_ref: String,
    /// Resolution state for this edge.
    pub resolution_state_class: String,
    /// Permissions contributed by the edge when it resolves.
    #[serde(default)]
    pub contributed_permission_refs: Vec<String>,
    /// API/runtime range implication for this edge.
    pub compatibility_range_ref: String,
}

/// Resolver output and lock/export input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyResolutionInput {
    /// Resolver determinism class.
    pub determinism_class: String,
    /// API compatibility range ref.
    pub api_range_ref: String,
    /// Runtime compatibility range ref.
    pub runtime_range_ref: String,
    /// Resolution digest ref.
    pub resolution_digest_ref: String,
    /// Resolver input ref.
    pub resolver_input_ref: String,
    /// Hard dependencies.
    #[serde(default)]
    pub hard_dependencies: Vec<DependencyEdgeInput>,
    /// Optional integrations.
    #[serde(default)]
    pub optional_integrations: Vec<DependencyEdgeInput>,
    /// Deterministic lockfile ref.
    pub lockfile_ref: String,
    /// Team/air-gap install export ref.
    pub install_export_ref: String,
    /// Whether the lock/export object is available.
    pub lock_export_available: bool,
    /// Whether team rollout is supported.
    pub supports_team_rollout: bool,
    /// Whether air-gapped rollout is supported.
    pub supports_air_gapped_rollout: bool,
}

/// Effective permission review input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionReviewInput {
    /// Top-level manifest permission refs.
    #[serde(default)]
    pub declared_permission_refs: Vec<String>,
    /// Prior installed effective permission refs.
    #[serde(default)]
    pub prior_effective_permission_refs: Vec<String>,
    /// Re-consent state.
    pub reconsent_state_class: String,
    /// Consent record ref when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_record_ref: Option<String>,
}

/// Publisher continuity workflow input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublisherContinuityInput {
    /// Continuity workflow class.
    pub workflow_class: String,
    /// Continuity state.
    pub continuity_state_class: String,
    /// Delay/cooldown/window satisfaction flag.
    pub delay_satisfied: bool,
    /// Audit trail ref.
    pub audit_trail_ref: String,
    /// Whether audit lineage is preserved and inspectable.
    pub audit_trail_preserved: bool,
    /// Whether affected users were notified.
    pub user_notified: bool,
    /// Whether affected admins were notified.
    pub admin_notified: bool,
    /// Whether high-trust auto-update is gated until delay, audit, and notification pass.
    pub high_trust_auto_update_gated: bool,
    /// Whether transfer history remains preserved.
    pub transfer_history_preserved: bool,
    /// Whether the same package identity is preserved after continuity workflow.
    pub package_identity_preserved: bool,
    /// Continuity packet ref.
    pub continuity_packet_ref: String,
}

/// Yank, revocation, last-known-good, and rollback/hold input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationPinInput {
    /// Yank/revocation state.
    pub yank_revocation_state_class: String,
    /// Revocation propagation class across public, mirror, and offline metadata.
    pub propagation_class: String,
    /// Last-known-good pin ref when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_ref: Option<String>,
    /// Whether policy allows hold/downgrade to the last-known-good pin.
    pub policy_allows_hold_or_downgrade: bool,
    /// Downgrade/hold behavior class.
    pub downgrade_hold_behavior_class: String,
    /// Rollback export ref.
    pub rollback_export_ref: String,
}

/// SDK/API deprecation propagation input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecationPropagationInput {
    /// SDK/API deprecation state.
    pub api_deprecation_state_class: String,
    /// Deprecated API refs.
    #[serde(default)]
    pub deprecated_api_refs: Vec<String>,
    /// Whether deprecation flows into resolver output.
    pub flows_to_resolution_output: bool,
    /// Whether deprecation flows into install-time warnings.
    pub flows_to_install_warning: bool,
    /// Whether deprecation flows into migration docs.
    pub flows_to_migration_docs: bool,
    /// Whether a compatibility shim is available where feasible.
    pub compatibility_shim_available: bool,
    /// Whether migration docs ref is available.
    pub migration_docs_ref: String,
    /// Whether ecosystem claim packet proof is current and complete.
    pub claim_packet_current: bool,
}

/// Claimed tier input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyResolutionClaimInput {
    /// Claimed stability tier.
    pub claimed_tier: String,
    /// Claim basis.
    pub claim_basis_class: String,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Dependency edge after normalization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Stable edge id.
    pub edge_id: String,
    /// Edge class.
    pub edge_class: String,
    /// Target package ref.
    pub target_ref: String,
    /// Version range ref.
    pub version_range_ref: String,
    /// Resolution state.
    pub resolution_state_class: String,
    /// Permissions contributed by this edge.
    pub contributed_permission_refs: Vec<String>,
    /// API/runtime range implication.
    pub compatibility_range_ref: String,
}

/// Derived resolver output and lock/export posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyResolution {
    /// Resolver determinism class.
    pub determinism_class: String,
    /// API compatibility range ref.
    pub api_range_ref: String,
    /// Runtime compatibility range ref.
    pub runtime_range_ref: String,
    /// Resolution digest ref.
    pub resolution_digest_ref: String,
    /// Resolver input ref.
    pub resolver_input_ref: String,
    /// Hard dependencies.
    pub hard_dependencies: Vec<DependencyEdge>,
    /// Optional integrations.
    pub optional_integrations: Vec<DependencyEdge>,
    /// Deterministic lockfile ref.
    pub lockfile_ref: String,
    /// Team/air-gap install export ref.
    pub install_export_ref: String,
    /// Whether lock/export object is available.
    pub lock_export_available: bool,
    /// Whether team rollout is supported.
    pub supports_team_rollout: bool,
    /// Whether air-gapped rollout is supported.
    pub supports_air_gapped_rollout: bool,
    /// Derived hard dependency count.
    pub hard_dependency_count: usize,
    /// Derived optional integration count.
    pub optional_integration_count: usize,
    /// Derived unresolved hard dependency count.
    pub unresolved_hard_dependency_count: usize,
    /// Derived version conflict count.
    pub version_conflict_count: usize,
}

impl DependencyResolution {
    /// Returns true when the resolver output is deterministic.
    pub fn deterministic(&self) -> bool {
        self.determinism_class == "deterministic"
    }

    /// Returns true when every hard dependency resolved cleanly.
    pub fn all_hard_dependencies_resolved(&self) -> bool {
        self.unresolved_hard_dependency_count == 0 && self.version_conflict_count == 0
    }
}

/// Effective permission set after dependency resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectivePermissionResolution {
    /// Top-level manifest permission refs.
    pub declared_permission_refs: Vec<String>,
    /// Hard-dependency inherited permission refs.
    pub inherited_permission_refs: Vec<String>,
    /// Optional integration permission refs, surfaced separately.
    pub optional_integration_permission_refs: Vec<String>,
    /// Effective permission refs after dependency resolution.
    pub effective_permission_refs: Vec<String>,
    /// Prior installed effective permission refs.
    pub prior_effective_permission_refs: Vec<String>,
    /// Newly expanded permission refs compared with prior effective set.
    pub expanded_permission_refs: Vec<String>,
    /// Re-consent state.
    pub reconsent_state_class: String,
    /// Whether re-consent was triggered by dependency permission expansion.
    pub triggered_by_dependency_permission_expansion: bool,
    /// Consent record ref when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_record_ref: Option<String>,
}

impl EffectivePermissionResolution {
    /// Returns true when the dependency resolution widened authority.
    pub fn widened(&self) -> bool {
        !self.expanded_permission_refs.is_empty()
    }

    /// Returns true when re-consent has been satisfied.
    pub fn reconsent_satisfied(&self) -> bool {
        !self.widened() || self.reconsent_state_class == "required_obtained"
    }
}

/// Stability qualification claim after posture derivation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyResolutionQualificationClaim {
    /// Claimed tier.
    pub claimed_tier: String,
    /// Effective tier after posture derivation.
    pub effective_tier: String,
    /// Support claim class the effective tier may imply.
    pub support_claim_class: String,
    /// Claim basis.
    pub claim_basis_class: String,
    /// Whether claimed tier narrowed below Stable.
    pub downgraded: bool,
    /// Machine-readable narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Compact inspection row shared by UI, CLI/headless, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyResolutionInspection {
    /// Packet inspected by this row.
    pub packet_id_ref: String,
    /// Package identity.
    pub package_identity: String,
    /// Install flow class.
    pub install_flow_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Whether this row remains a stable claim.
    pub stable_claim: bool,
    /// Whether resolver output is deterministic.
    pub resolution_deterministic: bool,
    /// Whether every hard dependency resolved.
    pub all_hard_dependencies_resolved: bool,
    /// Hard dependency count.
    pub hard_dependency_count: usize,
    /// Optional integration count.
    pub optional_integration_count: usize,
    /// Effective permission count.
    pub effective_permission_count: usize,
    /// Expanded permission count.
    pub expanded_permission_count: usize,
    /// Whether re-consent is satisfied.
    pub reconsent_satisfied: bool,
    /// Publisher continuity workflow.
    pub continuity_workflow_class: String,
    /// Publisher continuity state.
    pub continuity_state_class: String,
    /// Whether high-trust auto-update may resume.
    pub high_trust_auto_update_may_resume: bool,
    /// Yank/revocation state.
    pub yank_revocation_state_class: String,
    /// Revocation propagation class.
    pub revocation_propagation_class: String,
    /// Last-known-good pin ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_ref: Option<String>,
    /// API deprecation state.
    pub api_deprecation_state_class: String,
    /// Whether claim-packet proof is current.
    pub claim_packet_current: bool,
    /// Whether public/mirror/enterprise identity parity is proven.
    pub identity_parity_across_sources: bool,
    /// Whether a banner must display before trust resumes.
    pub banner_required: bool,
}

/// Stable dependency-resolution and publisher-continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDependencyResolutionPacket {
    /// Record kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Identity and install-flow posture.
    pub identity: ResolutionIdentityInput,
    /// Resolver output and lock/export posture.
    pub resolution: DependencyResolution,
    /// Effective permission set after dependency resolution.
    pub permissions: EffectivePermissionResolution,
    /// Publisher continuity workflow posture.
    pub continuity: PublisherContinuityInput,
    /// Yank/revocation/last-known-good/rollback posture.
    pub revocation: RevocationPinInput,
    /// SDK/API deprecation propagation posture.
    pub deprecation: DeprecationPropagationInput,
    /// Stability qualification claim after derivation.
    pub claim: DependencyResolutionQualificationClaim,
    /// Consumer surfaces bound to this packet.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the packet cites.
    pub source_schema_refs: Vec<String>,
    /// True when a banner must be shown before relying on the row.
    pub banner_required: bool,
    /// False so top-level-manifest-only reasoning can never pass as complete truth.
    pub allows_top_level_manifest_only_risk: bool,
    /// False so dependency trees cannot widen authority silently.
    pub allows_silent_dependency_permission_widening: bool,
    /// False so high-trust auto-update cannot resume without continuity review.
    pub allows_ungated_high_trust_auto_update_after_continuity_change: bool,
    /// Compact inspection row.
    pub inspection: DependencyResolutionInspection,
    /// Reviewable summary.
    pub summary_label: String,
}

impl ExtensionDependencyResolutionPacket {
    /// Builds a packet from input and derives effective permissions, narrowing
    /// reasons, and inspection truth.
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionDependencyResolutionValidationError`] when the input or
    /// derived packet violates the stable dependency-resolution contract.
    pub fn from_input(
        input: ExtensionDependencyResolutionInput,
    ) -> Result<Self, ExtensionDependencyResolutionValidationError> {
        validate_input(&input)?;
        let resolution = resolution_record(&input.resolution);
        let permissions = permission_record(&input.permissions, &resolution);
        let derived = derive_claim(
            &input.identity,
            &resolution,
            &permissions,
            &input.continuity,
            &input.revocation,
            &input.deprecation,
            &input.claim,
        );
        let banner_required = !derived.downgrade_reasons.is_empty()
            || permissions.widened()
            || continuity_requires_review(&input.continuity);
        let inspection = inspection_record(
            &input.packet_id,
            &input.identity,
            &resolution,
            &permissions,
            &input.continuity,
            &input.revocation,
            &input.deprecation,
            &derived,
            banner_required,
        );
        let packet = Self {
            record_kind: EXTENSION_DEPENDENCY_RESOLUTION_PACKET_RECORD_KIND.to_string(),
            schema_version: EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            identity: input.identity,
            resolution,
            permissions,
            continuity: input.continuity,
            revocation: input.revocation,
            deprecation: input.deprecation,
            claim: derived,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_REF.to_string()],
            banner_required,
            allows_top_level_manifest_only_risk: false,
            allows_silent_dependency_permission_widening: false,
            allows_ungated_high_trust_auto_update_after_continuity_change: false,
            inspection,
            summary_label: input.summary_label,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates the packet invariants and re-derives the stored truth.
    ///
    /// # Errors
    ///
    /// Returns [`ExtensionDependencyResolutionValidationError`] when a packet
    /// field drifts from derived resolver, permission, continuity, revocation, or
    /// deprecation truth.
    pub fn validate(&self) -> Result<(), ExtensionDependencyResolutionValidationError> {
        ensure_eq(
            &self.record_kind,
            EXTENSION_DEPENDENCY_RESOLUTION_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;
        validate_identity(&self.identity)?;
        validate_resolution(&self.resolution)?;
        validate_permissions(&self.permissions)?;
        validate_continuity(&self.continuity)?;
        validate_revocation(&self.revocation)?;
        validate_deprecation(&self.deprecation)?;
        validate_claim_fields(&self.claim)?;
        for surface in &self.consumer_surfaces {
            ensure_token(
                EXTENSION_DEPENDENCY_RESOLUTION_CONSUMER_SURFACES,
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
            .any(|schema| schema == EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_REF)
        {
            return Err(err("packet must cite the dependency-resolution schema"));
        }
        if self.allows_top_level_manifest_only_risk
            || self.allows_silent_dependency_permission_widening
            || self.allows_ungated_high_trust_auto_update_after_continuity_change
        {
            return Err(err(
                "packet must not allow top-level-only risk, silent dependency widening, or ungated continuity auto-update",
            ));
        }

        let derived_resolution = resolution_record(&DependencyResolutionInput {
            determinism_class: self.resolution.determinism_class.clone(),
            api_range_ref: self.resolution.api_range_ref.clone(),
            runtime_range_ref: self.resolution.runtime_range_ref.clone(),
            resolution_digest_ref: self.resolution.resolution_digest_ref.clone(),
            resolver_input_ref: self.resolution.resolver_input_ref.clone(),
            hard_dependencies: self
                .resolution
                .hard_dependencies
                .iter()
                .map(edge_input_from_edge)
                .collect(),
            optional_integrations: self
                .resolution
                .optional_integrations
                .iter()
                .map(edge_input_from_edge)
                .collect(),
            lockfile_ref: self.resolution.lockfile_ref.clone(),
            install_export_ref: self.resolution.install_export_ref.clone(),
            lock_export_available: self.resolution.lock_export_available,
            supports_team_rollout: self.resolution.supports_team_rollout,
            supports_air_gapped_rollout: self.resolution.supports_air_gapped_rollout,
        });
        if derived_resolution != self.resolution {
            return Err(err(
                "stored resolution counts drifted from dependency edges",
            ));
        }

        let derived_permissions = permission_record(
            &EffectivePermissionReviewInput {
                declared_permission_refs: self.permissions.declared_permission_refs.clone(),
                prior_effective_permission_refs: self
                    .permissions
                    .prior_effective_permission_refs
                    .clone(),
                reconsent_state_class: self.permissions.reconsent_state_class.clone(),
                consent_record_ref: self.permissions.consent_record_ref.clone(),
            },
            &self.resolution,
        );
        if derived_permissions != self.permissions {
            return Err(err(
                "stored effective permissions drifted from declared plus hard-dependency permissions",
            ));
        }
        if self.permissions.widened() && !self.permissions.reconsent_satisfied() {
            if self.claim.effective_tier == "stable" {
                return Err(err(
                    "stable tier cannot keep widened effective permissions without obtained re-consent",
                ));
            }
        }
        if high_trust_update_may_resume(&self.continuity)
            && continuity_requires_review(&self.continuity)
        {
            return Err(err(
                "high-trust auto-update cannot resume while continuity still requires review",
            ));
        }
        if self.claim.effective_tier == "stable" {
            if self.identity.resolver_contract_version
                != EXTENSION_DEPENDENCY_RESOLUTION_PUBLISHED_VERSION
            {
                return Err(err("stable tier must pin the published contract version"));
            }
            if self.claim.claim_basis_class != "evidence_backed" {
                return Err(err("stable tier must be evidence-backed"));
            }
            if !self.resolution.deterministic()
                || !self.resolution.all_hard_dependencies_resolved()
                || !self.resolution.lock_export_available
                || !self.resolution.supports_team_rollout
                || !self.resolution.supports_air_gapped_rollout
            {
                return Err(err(
                    "stable tier requires deterministic resolved exportable lock truth",
                ));
            }
            if !self.permissions.reconsent_satisfied() {
                return Err(err(
                    "stable tier requires re-consent after permission widening",
                ));
            }
            if !continuity_satisfies_stable(&self.continuity) {
                return Err(err("stable tier requires settled publisher continuity"));
            }
            if !revocation_satisfies_stable(&self.revocation) {
                return Err(err(
                    "stable tier requires explicit revocation and pin posture",
                ));
            }
            if !deprecation_satisfies_stable(&self.deprecation) {
                return Err(err("stable tier requires complete deprecation propagation"));
            }
            if !self.identity.identity_parity_across_sources {
                return Err(err(
                    "stable tier requires public/mirror/enterprise identity parity",
                ));
            }
        }

        let derived_claim = derive_claim(
            &self.identity,
            &self.resolution,
            &self.permissions,
            &self.continuity,
            &self.revocation,
            &self.deprecation,
            &DependencyResolutionClaimInput {
                claimed_tier: self.claim.claimed_tier.clone(),
                claim_basis_class: self.claim.claim_basis_class.clone(),
                summary_label: self.claim.summary_label.clone(),
            },
        );
        if derived_claim != self.claim {
            return Err(err("stored claim does not match posture-derived claim"));
        }
        let derived_banner = !derived_claim.downgrade_reasons.is_empty()
            || self.permissions.widened()
            || continuity_requires_review(&self.continuity);
        if derived_banner != self.banner_required {
            return Err(err("stored banner state does not match derived posture"));
        }
        let derived_inspection = inspection_record(
            &self.packet_id,
            &self.identity,
            &self.resolution,
            &self.permissions,
            &self.continuity,
            &self.revocation,
            &self.deprecation,
            &self.claim,
            self.banner_required,
        );
        if derived_inspection != self.inspection {
            return Err(err("stored inspection row does not match derived posture"));
        }
        Ok(())
    }
}

/// Compact projection consumed by CLI/headless and dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionDependencyResolutionProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Package identity.
    pub package_identity: String,
    /// Install flow class.
    pub install_flow_class: String,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// Whether a banner must display.
    pub banner_required: bool,
    /// Effective permission count.
    pub effective_permission_count: usize,
    /// Expanded permission count.
    pub expanded_permission_count: usize,
    /// Continuity workflow class.
    pub continuity_workflow_class: String,
    /// Continuity state class.
    pub continuity_state_class: String,
}

/// Parses and validates a materialized packet, returning a compact projection.
///
/// # Errors
///
/// Returns [`ExtensionDependencyResolutionError`] when payload parsing or packet
/// validation fails.
pub fn project_extension_dependency_resolution(
    payload: &str,
) -> Result<ExtensionDependencyResolutionProjection, ExtensionDependencyResolutionError> {
    let packet: ExtensionDependencyResolutionPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(ExtensionDependencyResolutionProjection {
        packet_id: packet.packet_id,
        package_identity: packet.identity.package_identity,
        install_flow_class: packet.identity.install_flow_class,
        effective_tier: packet.claim.effective_tier,
        support_claim_class: packet.claim.support_claim_class,
        downgrade_reasons: packet.claim.downgrade_reasons,
        banner_required: packet.banner_required,
        effective_permission_count: packet.permissions.effective_permission_refs.len(),
        expanded_permission_count: packet.permissions.expanded_permission_refs.len(),
        continuity_workflow_class: packet.continuity.workflow_class,
        continuity_state_class: packet.continuity.continuity_state_class,
    })
}

/// Metadata-safe support export for dependency resolution and continuity truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDependencyResolutionSupportExport {
    /// Record kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Export identity.
    pub export_id: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Package identity.
    pub package_identity: String,
    /// Install flow class.
    pub install_flow_class: String,
    /// Source classes proven by the packet.
    pub source_classes: Vec<String>,
    /// Effective tier.
    pub effective_tier: String,
    /// Support claim class.
    pub support_claim_class: String,
    /// Narrowing reasons.
    pub downgrade_reasons: Vec<String>,
    /// Resolver determinism class.
    pub determinism_class: String,
    /// Hard dependency count.
    pub hard_dependency_count: usize,
    /// Optional integration count.
    pub optional_integration_count: usize,
    /// Effective permission count.
    pub effective_permission_count: usize,
    /// Expanded permission count.
    pub expanded_permission_count: usize,
    /// Re-consent state class.
    pub reconsent_state_class: String,
    /// Continuity workflow class.
    pub continuity_workflow_class: String,
    /// Continuity state class.
    pub continuity_state_class: String,
    /// Whether high-trust auto-update may resume.
    pub high_trust_auto_update_may_resume: bool,
    /// Yank/revocation state class.
    pub yank_revocation_state_class: String,
    /// Revocation propagation class.
    pub revocation_propagation_class: String,
    /// Last-known-good ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_ref: Option<String>,
    /// API deprecation state class.
    pub api_deprecation_state_class: String,
    /// Whether claim proof is current.
    pub claim_packet_current: bool,
    /// Whether banner is required.
    pub banner_required: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

/// Projects a packet into a metadata-safe support export.
pub fn project_extension_dependency_resolution_support_export(
    packet: &ExtensionDependencyResolutionPacket,
) -> ExtensionDependencyResolutionSupportExport {
    ExtensionDependencyResolutionSupportExport {
        record_kind: EXTENSION_DEPENDENCY_RESOLUTION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: EXTENSION_DEPENDENCY_RESOLUTION_SCHEMA_VERSION,
        export_id: format!("support_export:{}", packet.packet_id),
        packet_ref: packet.packet_id.clone(),
        package_identity: packet.identity.package_identity.clone(),
        install_flow_class: packet.identity.install_flow_class.clone(),
        source_classes: packet.identity.source_classes.clone(),
        effective_tier: packet.claim.effective_tier.clone(),
        support_claim_class: packet.claim.support_claim_class.clone(),
        downgrade_reasons: packet.claim.downgrade_reasons.clone(),
        determinism_class: packet.resolution.determinism_class.clone(),
        hard_dependency_count: packet.resolution.hard_dependency_count,
        optional_integration_count: packet.resolution.optional_integration_count,
        effective_permission_count: packet.permissions.effective_permission_refs.len(),
        expanded_permission_count: packet.permissions.expanded_permission_refs.len(),
        reconsent_state_class: packet.permissions.reconsent_state_class.clone(),
        continuity_workflow_class: packet.continuity.workflow_class.clone(),
        continuity_state_class: packet.continuity.continuity_state_class.clone(),
        high_trust_auto_update_may_resume: high_trust_update_may_resume(&packet.continuity),
        yank_revocation_state_class: packet.revocation.yank_revocation_state_class.clone(),
        revocation_propagation_class: packet.revocation.propagation_class.clone(),
        last_known_good_ref: packet.revocation.last_known_good_ref.clone(),
        api_deprecation_state_class: packet.deprecation.api_deprecation_state_class.clone(),
        claim_packet_current: packet.deprecation.claim_packet_current,
        banner_required: packet.banner_required,
        export_safe_summary: format!(
            "{} {} resolved {} hard dependencies and {} optional integrations with effective tier {}. Effective permissions={} expanded={} reconsent={}. Continuity {} is {} and high-trust auto-update may resume={}. Revocation={} propagation={} last-known-good={}. Deprecation={} claim_packet_current={}.",
            packet.identity.package_identity,
            packet.identity.install_flow_class,
            packet.resolution.hard_dependency_count,
            packet.resolution.optional_integration_count,
            packet.claim.effective_tier,
            packet.permissions.effective_permission_refs.len(),
            packet.permissions.expanded_permission_refs.len(),
            packet.permissions.reconsent_state_class,
            packet.continuity.workflow_class,
            packet.continuity.continuity_state_class,
            high_trust_update_may_resume(&packet.continuity),
            packet.revocation.yank_revocation_state_class,
            packet.revocation.propagation_class,
            packet
                .revocation
                .last_known_good_ref
                .as_deref()
                .unwrap_or("none"),
            packet.deprecation.api_deprecation_state_class,
            packet.deprecation.claim_packet_current,
        ),
    }
}

/// Error raised while parsing or projecting dependency-resolution packets.
#[derive(Debug)]
pub enum ExtensionDependencyResolutionError {
    /// JSON payload failed to parse.
    Json(serde_json::Error),
    /// Parsed packet failed validation.
    Validation(ExtensionDependencyResolutionValidationError),
}

impl fmt::Display for ExtensionDependencyResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(err) => write!(f, "invalid dependency-resolution JSON: {err}"),
            Self::Validation(err) => write!(f, "invalid dependency-resolution packet: {err}"),
        }
    }
}

impl std::error::Error for ExtensionDependencyResolutionError {}

impl From<serde_json::Error> for ExtensionDependencyResolutionError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<ExtensionDependencyResolutionValidationError> for ExtensionDependencyResolutionError {
    fn from(value: ExtensionDependencyResolutionValidationError) -> Self {
        Self::Validation(value)
    }
}

/// Validation error for stable dependency-resolution packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionDependencyResolutionValidationError {
    message: String,
}

impl fmt::Display for ExtensionDependencyResolutionValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ExtensionDependencyResolutionValidationError {}

fn resolution_record(input: &DependencyResolutionInput) -> DependencyResolution {
    let hard_dependencies: Vec<_> = input.hard_dependencies.iter().map(edge_record).collect();
    let optional_integrations: Vec<_> = input
        .optional_integrations
        .iter()
        .map(edge_record)
        .collect();
    let unresolved_hard_dependency_count = hard_dependencies
        .iter()
        .filter(|edge| edge.resolution_state_class == "unresolved_missing")
        .count();
    let version_conflict_count = hard_dependencies
        .iter()
        .filter(|edge| edge.resolution_state_class == "version_conflict")
        .count();
    DependencyResolution {
        determinism_class: input.determinism_class.clone(),
        api_range_ref: input.api_range_ref.clone(),
        runtime_range_ref: input.runtime_range_ref.clone(),
        resolution_digest_ref: input.resolution_digest_ref.clone(),
        resolver_input_ref: input.resolver_input_ref.clone(),
        hard_dependency_count: hard_dependencies.len(),
        optional_integration_count: optional_integrations.len(),
        unresolved_hard_dependency_count,
        version_conflict_count,
        hard_dependencies,
        optional_integrations,
        lockfile_ref: input.lockfile_ref.clone(),
        install_export_ref: input.install_export_ref.clone(),
        lock_export_available: input.lock_export_available,
        supports_team_rollout: input.supports_team_rollout,
        supports_air_gapped_rollout: input.supports_air_gapped_rollout,
    }
}

fn edge_record(input: &DependencyEdgeInput) -> DependencyEdge {
    DependencyEdge {
        edge_id: input.edge_id.clone(),
        edge_class: input.edge_class.clone(),
        target_ref: input.target_ref.clone(),
        version_range_ref: input.version_range_ref.clone(),
        resolution_state_class: input.resolution_state_class.clone(),
        contributed_permission_refs: sorted(input.contributed_permission_refs.clone()),
        compatibility_range_ref: input.compatibility_range_ref.clone(),
    }
}

fn edge_input_from_edge(edge: &DependencyEdge) -> DependencyEdgeInput {
    DependencyEdgeInput {
        edge_id: edge.edge_id.clone(),
        edge_class: edge.edge_class.clone(),
        target_ref: edge.target_ref.clone(),
        version_range_ref: edge.version_range_ref.clone(),
        resolution_state_class: edge.resolution_state_class.clone(),
        contributed_permission_refs: edge.contributed_permission_refs.clone(),
        compatibility_range_ref: edge.compatibility_range_ref.clone(),
    }
}

fn permission_record(
    input: &EffectivePermissionReviewInput,
    resolution: &DependencyResolution,
) -> EffectivePermissionResolution {
    let declared = sorted(input.declared_permission_refs.clone());
    let inherited = sorted(
        resolution
            .hard_dependencies
            .iter()
            .filter(|edge| edge.resolution_state_class == "resolved")
            .flat_map(|edge| edge.contributed_permission_refs.clone())
            .collect(),
    );
    let optional = sorted(
        resolution
            .optional_integrations
            .iter()
            .flat_map(|edge| edge.contributed_permission_refs.clone())
            .collect(),
    );
    let prior = sorted(input.prior_effective_permission_refs.clone());
    let mut effective_set: BTreeSet<String> = declared.iter().cloned().collect();
    effective_set.extend(inherited.iter().cloned());
    let effective: Vec<_> = effective_set.into_iter().collect();
    let prior_set: BTreeSet<_> = prior.iter().cloned().collect();
    let expanded = effective
        .iter()
        .filter(|permission| !prior_set.contains(*permission))
        .cloned()
        .collect();
    EffectivePermissionResolution {
        declared_permission_refs: declared,
        inherited_permission_refs: inherited,
        optional_integration_permission_refs: optional,
        effective_permission_refs: effective,
        prior_effective_permission_refs: prior,
        expanded_permission_refs: expanded,
        reconsent_state_class: input.reconsent_state_class.clone(),
        triggered_by_dependency_permission_expansion: false,
        consent_record_ref: input.consent_record_ref.clone(),
    }
    .with_trigger()
}

trait WithTrigger {
    fn with_trigger(self) -> Self;
}

impl WithTrigger for EffectivePermissionResolution {
    fn with_trigger(mut self) -> Self {
        self.triggered_by_dependency_permission_expansion = self.widened();
        self
    }
}

fn derive_claim(
    identity: &ResolutionIdentityInput,
    resolution: &DependencyResolution,
    permissions: &EffectivePermissionResolution,
    continuity: &PublisherContinuityInput,
    revocation: &RevocationPinInput,
    deprecation: &DeprecationPropagationInput,
    claim: &DependencyResolutionClaimInput,
) -> DependencyResolutionQualificationClaim {
    let mut reasons = Vec::new();
    push_if(
        &mut reasons,
        identity.resolver_contract_version != EXTENSION_DEPENDENCY_RESOLUTION_PUBLISHED_VERSION,
        "contract_version_not_published",
    );
    push_if(
        &mut reasons,
        claim.claim_basis_class == "catalog_asserted_only",
        "catalog_only_claim_basis",
    );
    push_if(
        &mut reasons,
        resolution.determinism_class == "nondeterministic",
        "resolver_nondeterministic",
    );
    push_if(
        &mut reasons,
        resolution.determinism_class == "not_resolved",
        "resolver_not_resolved",
    );
    push_if(
        &mut reasons,
        resolution.unresolved_hard_dependency_count > 0,
        "unresolved_hard_dependency",
    );
    push_if(
        &mut reasons,
        resolution.version_conflict_count > 0,
        "version_conflict_dependency",
    );
    push_if(
        &mut reasons,
        !resolution.lock_export_available,
        "lock_export_unavailable",
    );
    push_if(
        &mut reasons,
        !resolution.supports_team_rollout,
        "team_rollout_unsupported",
    );
    push_if(
        &mut reasons,
        !resolution.supports_air_gapped_rollout,
        "air_gapped_rollout_unsupported",
    );
    push_if(
        &mut reasons,
        permissions.widened() && !permissions.reconsent_satisfied(),
        "permission_widening_without_reconsent",
    );
    add_continuity_reasons(&mut reasons, continuity);
    add_revocation_reasons(&mut reasons, revocation);
    add_deprecation_reasons(&mut reasons, deprecation);
    push_if(
        &mut reasons,
        !identity.identity_parity_across_sources,
        "identity_parity_missing",
    );
    let reasons = sorted(reasons);
    let effective_tier = effective_tier(&claim.claimed_tier, &reasons);
    let downgraded = effective_tier != claim.claimed_tier;
    DependencyResolutionQualificationClaim {
        claimed_tier: claim.claimed_tier.clone(),
        support_claim_class: support_claim(&effective_tier),
        effective_tier,
        claim_basis_class: claim.claim_basis_class.clone(),
        downgraded,
        downgrade_reasons: reasons,
        summary_label: claim.summary_label.clone(),
    }
}

fn add_continuity_reasons(reasons: &mut Vec<String>, continuity: &PublisherContinuityInput) {
    match continuity.continuity_state_class.as_str() {
        "in_cooldown" => reasons.push("publisher_continuity_in_cooldown".to_string()),
        "pending_review" => reasons.push("publisher_continuity_pending_review".to_string()),
        "pending_notification" => {
            reasons.push("publisher_continuity_pending_notification".to_string())
        }
        "disputed" => reasons.push("publisher_continuity_disputed".to_string()),
        "revoked" => reasons.push("publisher_continuity_revoked".to_string()),
        "stale" | "missing" => reasons.push("publisher_continuity_missing_or_stale".to_string()),
        _ => {}
    }
    push_if(
        reasons,
        !continuity.audit_trail_preserved,
        "continuity_audit_missing",
    );
    push_if(
        reasons,
        !continuity.user_notified || !continuity.admin_notified,
        "continuity_notification_missing",
    );
    push_if(
        reasons,
        continuity_requires_gate(continuity) && !continuity.high_trust_auto_update_gated,
        "high_trust_auto_update_not_gated",
    );
    push_if(
        reasons,
        !continuity.transfer_history_preserved,
        "transfer_history_missing",
    );
    push_if(
        reasons,
        !continuity.package_identity_preserved,
        "package_identity_not_preserved",
    );
}

fn add_revocation_reasons(reasons: &mut Vec<String>, revocation: &RevocationPinInput) {
    let needs_pin = matches!(
        revocation.yank_revocation_state_class.as_str(),
        "advisory" | "yanked" | "quarantined" | "revoked"
    );
    push_if(
        reasons,
        revocation.propagation_class == "partial"
            || revocation.propagation_class == "not_propagated",
        "revocation_not_propagated",
    );
    push_if(
        reasons,
        needs_pin
            && revocation
                .last_known_good_ref
                .as_deref()
                .unwrap_or("")
                .is_empty(),
        "last_known_good_pin_missing",
    );
    push_if(
        reasons,
        needs_pin
            && (!revocation.policy_allows_hold_or_downgrade
                || revocation.downgrade_hold_behavior_class == "hidden_or_implicit"),
        "hold_or_downgrade_policy_hidden",
    );
}

fn add_deprecation_reasons(reasons: &mut Vec<String>, deprecation: &DeprecationPropagationInput) {
    let deprecation_active = deprecation.api_deprecation_state_class != "none";
    push_if(
        reasons,
        deprecation_active
            && (!deprecation.flows_to_resolution_output
                || !deprecation.flows_to_install_warning
                || !deprecation.flows_to_migration_docs),
        "deprecation_not_propagated",
    );
    push_if(
        reasons,
        deprecation.api_deprecation_state_class == "removed"
            && !deprecation.compatibility_shim_available,
        "removed_api_without_shim",
    );
    push_if(
        reasons,
        !deprecation.claim_packet_current,
        "claim_packet_stale_or_incomplete",
    );
}

fn effective_tier(claimed: &str, reasons: &[String]) -> String {
    if claimed != "stable" || reasons.is_empty() {
        return claimed.to_string();
    }
    if reasons.iter().any(|reason| {
        matches!(
            reason.as_str(),
            "resolver_not_resolved"
                | "unresolved_hard_dependency"
                | "version_conflict_dependency"
                | "publisher_continuity_disputed"
                | "publisher_continuity_revoked"
                | "high_trust_auto_update_not_gated"
                | "package_identity_not_preserved"
                | "revocation_not_propagated"
                | "last_known_good_pin_missing"
                | "hold_or_downgrade_policy_hidden"
                | "removed_api_without_shim"
                | "identity_parity_missing"
        )
    }) {
        return "withdrawn".to_string();
    }
    if reasons.iter().any(|reason| {
        matches!(
            reason.as_str(),
            "publisher_continuity_in_cooldown"
                | "team_rollout_unsupported"
                | "air_gapped_rollout_unsupported"
        )
    }) {
        return "beta".to_string();
    }
    "preview".to_string()
}

fn support_claim(effective_tier: &str) -> String {
    match effective_tier {
        "stable" => "stable_dependency_resolution_claim",
        "beta" => "beta_dependency_resolution_partial_claim",
        "preview" => "preview_dependency_resolution_experimental_claim",
        _ => "withdrawn_no_dependency_resolution_claim",
    }
    .to_string()
}

fn inspection_record(
    packet_id: &str,
    identity: &ResolutionIdentityInput,
    resolution: &DependencyResolution,
    permissions: &EffectivePermissionResolution,
    continuity: &PublisherContinuityInput,
    revocation: &RevocationPinInput,
    deprecation: &DeprecationPropagationInput,
    claim: &DependencyResolutionQualificationClaim,
    banner_required: bool,
) -> DependencyResolutionInspection {
    DependencyResolutionInspection {
        packet_id_ref: packet_id.to_string(),
        package_identity: identity.package_identity.clone(),
        install_flow_class: identity.install_flow_class.clone(),
        effective_tier: claim.effective_tier.clone(),
        stable_claim: claim.effective_tier == "stable",
        resolution_deterministic: resolution.deterministic(),
        all_hard_dependencies_resolved: resolution.all_hard_dependencies_resolved(),
        hard_dependency_count: resolution.hard_dependency_count,
        optional_integration_count: resolution.optional_integration_count,
        effective_permission_count: permissions.effective_permission_refs.len(),
        expanded_permission_count: permissions.expanded_permission_refs.len(),
        reconsent_satisfied: permissions.reconsent_satisfied(),
        continuity_workflow_class: continuity.workflow_class.clone(),
        continuity_state_class: continuity.continuity_state_class.clone(),
        high_trust_auto_update_may_resume: high_trust_update_may_resume(continuity),
        yank_revocation_state_class: revocation.yank_revocation_state_class.clone(),
        revocation_propagation_class: revocation.propagation_class.clone(),
        last_known_good_ref: revocation.last_known_good_ref.clone(),
        api_deprecation_state_class: deprecation.api_deprecation_state_class.clone(),
        claim_packet_current: deprecation.claim_packet_current,
        identity_parity_across_sources: identity.identity_parity_across_sources,
        banner_required,
    }
}

fn high_trust_update_may_resume(continuity: &PublisherContinuityInput) -> bool {
    continuity.continuity_state_class == "current"
        && continuity.delay_satisfied
        && continuity.audit_trail_preserved
        && continuity.user_notified
        && continuity.admin_notified
        && continuity.high_trust_auto_update_gated
        && continuity.transfer_history_preserved
        && continuity.package_identity_preserved
}

fn continuity_satisfies_stable(continuity: &PublisherContinuityInput) -> bool {
    continuity.continuity_state_class == "current"
        && continuity.delay_satisfied
        && continuity.audit_trail_preserved
        && continuity.user_notified
        && continuity.admin_notified
        && continuity.transfer_history_preserved
        && continuity.package_identity_preserved
        && (!continuity_requires_gate(continuity) || continuity.high_trust_auto_update_gated)
}

fn continuity_requires_review(continuity: &PublisherContinuityInput) -> bool {
    !continuity_satisfies_stable(continuity)
}

fn continuity_requires_gate(continuity: &PublisherContinuityInput) -> bool {
    AUTHORITY_MOVING_CONTINUITY_WORKFLOWS.contains(&continuity.workflow_class.as_str())
}

fn revocation_satisfies_stable(revocation: &RevocationPinInput) -> bool {
    let needs_pin = revocation.yank_revocation_state_class != "clean";
    let propagation_ok = revocation.propagation_class == "not_applicable"
        || revocation.propagation_class == "propagated_all_sources";
    propagation_ok
        && (!needs_pin
            || (revocation.last_known_good_ref.is_some()
                && revocation.policy_allows_hold_or_downgrade
                && revocation.downgrade_hold_behavior_class == "explicit_policy_allowed"))
}

fn deprecation_satisfies_stable(deprecation: &DeprecationPropagationInput) -> bool {
    let deprecation_active = deprecation.api_deprecation_state_class != "none";
    deprecation.claim_packet_current
        && (!deprecation_active
            || (deprecation.flows_to_resolution_output
                && deprecation.flows_to_install_warning
                && deprecation.flows_to_migration_docs))
        && (deprecation.api_deprecation_state_class != "removed"
            || deprecation.compatibility_shim_available)
}

fn validate_input(
    input: &ExtensionDependencyResolutionInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;
    validate_identity(&input.identity)?;
    validate_resolution_input(&input.resolution)?;
    ensure_token(
        RECONSENT_STATE_CLASSES,
        &input.permissions.reconsent_state_class,
        "reconsent_state_class",
    )?;
    validate_continuity(&input.continuity)?;
    validate_revocation(&input.revocation)?;
    validate_deprecation(&input.deprecation)?;
    ensure_token(STABILITY_TIERS, &input.claim.claimed_tier, "claimed_tier")?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &input.claim.claim_basis_class,
        "claim_basis_class",
    )?;
    Ok(())
}

fn validate_identity(
    identity: &ResolutionIdentityInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_nonempty(&identity.package_identity, "package_identity")?;
    ensure_nonempty(&identity.package_version, "package_version")?;
    ensure_token(
        INSTALL_FLOW_CLASSES,
        &identity.install_flow_class,
        "install_flow_class",
    )?;
    for source in &identity.source_classes {
        ensure_token(SOURCE_CLASSES, source, "source_class")?;
    }
    if identity.source_classes.is_empty() {
        return Err(err("identity must bind at least one source class"));
    }
    ensure_nonempty(&identity.public_row_ref, "public_row_ref")?;
    ensure_nonempty(&identity.mirror_row_ref, "mirror_row_ref")?;
    ensure_nonempty(&identity.enterprise_row_ref, "enterprise_row_ref")?;
    Ok(())
}

fn validate_resolution_input(
    resolution: &DependencyResolutionInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_token(
        RESOLVER_DETERMINISM_CLASSES,
        &resolution.determinism_class,
        "determinism_class",
    )?;
    ensure_nonempty(&resolution.api_range_ref, "api_range_ref")?;
    ensure_nonempty(&resolution.runtime_range_ref, "runtime_range_ref")?;
    ensure_nonempty(&resolution.resolution_digest_ref, "resolution_digest_ref")?;
    ensure_nonempty(&resolution.resolver_input_ref, "resolver_input_ref")?;
    ensure_nonempty(&resolution.lockfile_ref, "lockfile_ref")?;
    ensure_nonempty(&resolution.install_export_ref, "install_export_ref")?;
    for edge in resolution
        .hard_dependencies
        .iter()
        .chain(resolution.optional_integrations.iter())
    {
        validate_edge_input(edge)?;
    }
    Ok(())
}

fn validate_edge_input(
    edge: &DependencyEdgeInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_nonempty(&edge.edge_id, "edge_id")?;
    ensure_token(DEPENDENCY_EDGE_CLASSES, &edge.edge_class, "edge_class")?;
    ensure_nonempty(&edge.target_ref, "target_ref")?;
    ensure_nonempty(&edge.version_range_ref, "version_range_ref")?;
    ensure_token(
        DEPENDENCY_RESOLUTION_STATE_CLASSES,
        &edge.resolution_state_class,
        "resolution_state_class",
    )?;
    ensure_nonempty(&edge.compatibility_range_ref, "compatibility_range_ref")?;
    Ok(())
}

fn validate_resolution(
    resolution: &DependencyResolution,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    validate_resolution_input(&DependencyResolutionInput {
        determinism_class: resolution.determinism_class.clone(),
        api_range_ref: resolution.api_range_ref.clone(),
        runtime_range_ref: resolution.runtime_range_ref.clone(),
        resolution_digest_ref: resolution.resolution_digest_ref.clone(),
        resolver_input_ref: resolution.resolver_input_ref.clone(),
        hard_dependencies: resolution
            .hard_dependencies
            .iter()
            .map(edge_input_from_edge)
            .collect(),
        optional_integrations: resolution
            .optional_integrations
            .iter()
            .map(edge_input_from_edge)
            .collect(),
        lockfile_ref: resolution.lockfile_ref.clone(),
        install_export_ref: resolution.install_export_ref.clone(),
        lock_export_available: resolution.lock_export_available,
        supports_team_rollout: resolution.supports_team_rollout,
        supports_air_gapped_rollout: resolution.supports_air_gapped_rollout,
    })
}

fn validate_permissions(
    permissions: &EffectivePermissionResolution,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_token(
        RECONSENT_STATE_CLASSES,
        &permissions.reconsent_state_class,
        "reconsent_state_class",
    )?;
    if permissions.widened() && permissions.reconsent_state_class == "not_required" {
        return Err(err("permission widening must trigger re-consent"));
    }
    if !permissions.widened() && permissions.reconsent_state_class != "not_required" {
        return Err(err(
            "re-consent must be not_required when authority did not widen",
        ));
    }
    Ok(())
}

fn validate_continuity(
    continuity: &PublisherContinuityInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_token(
        CONTINUITY_WORKFLOW_CLASSES,
        &continuity.workflow_class,
        "workflow_class",
    )?;
    ensure_token(
        CONTINUITY_STATE_CLASSES,
        &continuity.continuity_state_class,
        "continuity_state_class",
    )?;
    ensure_nonempty(&continuity.audit_trail_ref, "audit_trail_ref")?;
    ensure_nonempty(&continuity.continuity_packet_ref, "continuity_packet_ref")?;
    Ok(())
}

fn validate_revocation(
    revocation: &RevocationPinInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_token(
        YANK_REVOCATION_STATE_CLASSES,
        &revocation.yank_revocation_state_class,
        "yank_revocation_state_class",
    )?;
    ensure_token(
        REVOCATION_PROPAGATION_CLASSES,
        &revocation.propagation_class,
        "propagation_class",
    )?;
    ensure_token(
        DOWNGRADE_HOLD_BEHAVIOR_CLASSES,
        &revocation.downgrade_hold_behavior_class,
        "downgrade_hold_behavior_class",
    )?;
    ensure_nonempty(&revocation.rollback_export_ref, "rollback_export_ref")?;
    Ok(())
}

fn validate_deprecation(
    deprecation: &DeprecationPropagationInput,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_token(
        API_DEPRECATION_STATE_CLASSES,
        &deprecation.api_deprecation_state_class,
        "api_deprecation_state_class",
    )?;
    ensure_nonempty(&deprecation.migration_docs_ref, "migration_docs_ref")?;
    Ok(())
}

fn validate_claim_fields(
    claim: &DependencyResolutionQualificationClaim,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    ensure_token(STABILITY_TIERS, &claim.claimed_tier, "claimed_tier")?;
    ensure_token(STABILITY_TIERS, &claim.effective_tier, "effective_tier")?;
    ensure_token(
        SUPPORT_CLAIM_CLASSES,
        &claim.support_claim_class,
        "support_claim_class",
    )?;
    ensure_token(
        CLAIM_BASIS_CLASSES,
        &claim.claim_basis_class,
        "claim_basis_class",
    )?;
    for reason in &claim.downgrade_reasons {
        ensure_token(
            DEPENDENCY_RESOLUTION_DOWNGRADE_REASONS,
            reason,
            "downgrade_reason",
        )?;
    }
    Ok(())
}

fn push_if(reasons: &mut Vec<String>, condition: bool, reason: &str) {
    if condition {
        reasons.push(reason.to_string());
    }
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.retain(|value| !value.trim().is_empty());
    values.sort();
    values.dedup();
    values
}

fn ensure_nonempty(
    value: &str,
    field: &str,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    allowed: &[&str],
    value: &str,
    field: &str,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    if !allowed.contains(&value) {
        return Err(err(format!("{field} contains unsupported token {value:?}")));
    }
    Ok(())
}

fn ensure_eq(
    actual: &str,
    expected: &str,
    field: &str,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    if actual != expected {
        return Err(err(format!("{field} must be {expected:?}, got {actual:?}")));
    }
    Ok(())
}

fn ensure_eq_u32(
    actual: u32,
    expected: u32,
    field: &str,
) -> Result<(), ExtensionDependencyResolutionValidationError> {
    if actual != expected {
        return Err(err(format!("{field} must be {expected}, got {actual}")));
    }
    Ok(())
}

fn err(message: impl Into<String>) -> ExtensionDependencyResolutionValidationError {
    ExtensionDependencyResolutionValidationError {
        message: message.into(),
    }
}
