//! Prebuild fingerprint, invalidation, and disclosure alpha records.
//!
//! A prebuild is an accelerator, not an authority. A
//! [`PrebuildFingerprintRecord`] freezes the dimensions that make one
//! prepared environment reusable — source identity, environment capsule,
//! toolchain, host, policy and trust posture, secret-handle posture, port
//! and route posture, cache artifacts, freshness, and redaction posture.
//! A [`PrebuildReuseDecisionRecord`] binds that fingerprint to a
//! requested entry path (`resume_live_workspace`, `start_from_snapshot`,
//! `clone_fresh`, or `reuse_cached_prebuild`) and resolves whether the
//! prebuild can be reused, partially reused, or must be rebuilt. A
//! [`PrebuildDisclosureRecord`] is the user / support readable
//! projection that Start Center, CLI / headless entry, docs, and support
//! packets quote before any networked setup runs.
//!
//! The schemas these records validate against live at:
//!
//! - `schemas/workspace/prebuild_fingerprint.schema.json`
//! - `schemas/workspace/prebuild_invalidation_reason.schema.json`
//!
//! The reviewer doc lives at
//! `docs/workspace/m3/prebuild_fingerprint_alpha.md`. The contract this
//! module enforces is documented in
//! `docs/workspace/prebuild_fingerprint_contract.md`.
//!
//! Frozen guarantees enforced here:
//!
//! 1. Every fingerprint declares freshness, producer, and signer posture
//!    and excludes raw secret material, raw credential bodies, raw
//!    environment values, machine-unique trust anchors, and uncommitted
//!    workspace edits from export.
//! 2. A `reuse_allowed` decision MUST carry no invalidation bundle refs
//!    and no required revalidations.
//! 3. A request for `resume_live_workspace` that resolves against a
//!    `prebuilt_snapshot` or `stale_prebuild_snapshot` materialization MUST
//!    produce `resume_live_denied` — a stale snapshot cannot masquerade as
//!    a live resume.
//! 4. Every disclosure record asserts
//!    `stale_snapshot_must_not_be_labeled_live_resume = true` and excludes
//!    raw secret, raw credential, and uncommitted edit residue from
//!    support export.
//! 5. The closed `resume_live_workspace` / `start_from_snapshot` /
//!    `clone_fresh` / `reuse_cached_prebuild` path vocabulary keeps the
//!    three user choices distinct; no decision collapses two paths into
//!    the same outcome.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Schema version for the alpha prebuild fingerprint record family.
pub const PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema version for the alpha prebuild invalidation reason family.
pub const PREBUILD_INVALIDATION_REASON_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for a [`PrebuildFingerprintRecord`].
pub const PREBUILD_FINGERPRINT_RECORD_KIND: &str = "prebuild_fingerprint_record";
/// Record-kind discriminator for a [`PrebuildReuseDecisionRecord`].
pub const PREBUILD_REUSE_DECISION_RECORD_KIND: &str = "prebuild_reuse_decision_record";
/// Record-kind discriminator for a [`PrebuildDisclosureRecord`].
pub const PREBUILD_DISCLOSURE_RECORD_KIND: &str = "prebuild_disclosure_record";

/// Closed set of entry paths a reuse decision can be requested against.
pub const PREBUILD_PATH_CLASSES: &[&str] = &[
    "resume_live_workspace",
    "start_from_snapshot",
    "clone_fresh",
    "reuse_cached_prebuild",
];

/// Closed set of materialization classes a candidate fingerprint can describe.
pub const SOURCE_MATERIALIZATION_CLASSES: &[&str] = &[
    "live_materialized_workspace",
    "prebuilt_snapshot",
    "stale_prebuild_snapshot",
    "fresh_clone_materialization",
    "local_override_materialization",
];

/// Closed set of reuse outcomes a decision can resolve to.
pub const REUSE_OUTCOME_CLASSES: &[&str] = &[
    "reuse_allowed",
    "reuse_after_revalidation",
    "partial_warm_reuse_only",
    "rebuild_required",
    "clone_fresh_required",
    "resume_live_denied",
];

/// Closed set of revalidation classes a decision can require.
pub const REQUIRED_REVALIDATION_CLASSES: &[&str] = &[
    "credentials",
    "ports",
    "indexes",
    "policy",
    "trust",
    "feature_flags",
    "source",
    "environment",
    "toolchain",
    "host",
    "schema",
    "redaction_review",
    "none",
];

/// Closed set of disclosure states a disclosure record may render.
pub const DISCLOSURE_STATE_CLASSES: &[&str] = &[
    "live_materialized",
    "prebuilt_reused",
    "prebuilt_reused_after_revalidation",
    "partially_warm",
    "stale_prebuild_rebuild_required",
    "fresh_clone",
    "local_override_rebuild_required",
    "resume_denied_snapshot_available",
];

/// Closed set of freshness age classes.
pub const FRESHNESS_AGE_CLASSES: &[&str] = &[
    "fresh_under_window",
    "near_expiry",
    "stale_over_window",
    "expired",
    "unknown_requires_revalidation",
];

/// Closed set of producer classes for a fingerprint.
pub const PRODUCER_CLASSES: &[&str] = &[
    "local_user_materializer",
    "first_party_template_pipeline",
    "managed_workspace_service",
    "self_hosted_prebuild_service",
    "enterprise_mirror_pipeline",
    "imported_bundle",
    "producer_unknown_requires_review",
];

/// Closed set of signer posture classes.
pub const SIGNER_POSTURE_CLASSES: &[&str] = &[
    "signed_verified",
    "signed_rotation_preauthorized",
    "signed_review_required",
    "unsigned_allowed_local_only",
    "signature_missing_blocked",
    "signature_mismatch_blocked",
    "not_applicable",
];

/// Closed set of host classes a fingerprint can declare.
pub const HOST_CLASSES: &[&str] = &[
    "local_host",
    "devcontainer",
    "container_local",
    "ssh_remote",
    "remote_workspace_vm",
    "managed_workspace",
    "prebuild_runtime",
];

/// Closed set of platform / architecture identifiers.
pub const PLATFORM_ARCH_CLASSES: &[&str] = &[
    "darwin-aarch64",
    "darwin-x86_64",
    "linux-aarch64",
    "linux-x86_64",
    "windows-x86_64",
    "platform_unknown_requires_review",
];

/// Closed set of cache classes a fingerprint can declare.
pub const CACHE_CLASSES: &[&str] = &[
    "toolchain_layer",
    "dependency_store",
    "index_shard",
    "artifact_mirror",
    "extension_package",
    "docs_pack",
    "service_image_layer",
    "environment_capsule_manifest",
];

/// Closed set of residue classes that MUST stay excluded from export.
pub const EXCLUDED_RESIDUE_CLASSES: &[&str] = &[
    "raw_secret_material",
    "raw_credential_bodies",
    "raw_environment_values",
    "raw_command_lines",
    "os_keychain_items",
    "enterprise_vault_bodies",
    "ssh_agent_sockets",
    "host_absolute_paths",
    "machine_unique_trust_anchors",
    "local_uid_gid",
    "runtime_process_ids",
    "port_allocations",
    "socket_paths",
    "terminal_history",
    "clipboard_contents",
    "dirty_buffer_journals",
    "uncommitted_workspace_edits",
    "local_only_logs",
    "cache_tempdirs",
];

/// Residue classes the fingerprint and disclosure MUST exclude in every record.
pub const REQUIRED_EXCLUDED_RESIDUE_FOR_FINGERPRINT: &[&str] = &[
    "raw_secret_material",
    "raw_credential_bodies",
    "raw_environment_values",
    "machine_unique_trust_anchors",
    "uncommitted_workspace_edits",
];

/// Residue classes the disclosure record MUST exclude.
pub const REQUIRED_EXCLUDED_RESIDUE_FOR_DISCLOSURE: &[&str] = &[
    "raw_secret_material",
    "raw_credential_bodies",
    "uncommitted_workspace_edits",
];

/// Closed set of allowed export-field classes.
pub const EXPORT_FIELD_CLASSES: &[&str] = &[
    "opaque_refs",
    "digests",
    "schema_versions",
    "producer_and_signer_posture",
    "cache_class_labels",
    "freshness_age_class",
    "redaction_summary_counts",
    "invalidation_reason_refs",
    "revalidation_requirements",
];

/// Closed set of support-export postures.
pub const SUPPORT_EXPORT_POSTURES: &[&str] = &[
    "metadata_safe_default",
    "redacted_support_default",
    "operator_only_restricted",
    "blocked_until_redaction_review",
];

/// Closed set of credential-expiry postures.
pub const CREDENTIAL_EXPIRY_POSTURES: &[&str] = &[
    "current",
    "expiring_revalidate_before_use",
    "expired_reauth_required",
    "store_locked_reauth_required",
    "not_configured",
];

/// Closed set of route-dependency classes for the port/route identity.
pub const ROUTE_DEPENDENCY_CLASSES: &[&str] = &[
    "direct",
    "tunneled",
    "managed_proxy",
    "policy_mirror",
    "not_applicable",
];

/// Closed set of trust states.
pub const TRUST_STATES: &[&str] = &[
    "trusted",
    "restricted",
    "pending_evaluation",
    "revoked",
    "managed_locked",
];

/// Closed set of support-packet inclusion postures on a decision.
pub const SUPPORT_PACKET_INCLUSION_CLASSES: &[&str] = &[
    "include_metadata_and_reasons",
    "include_metadata_marked_stale",
    "include_denial_only",
    "blocked_until_redaction_review",
];

/// Closed set of consumer surfaces the alpha bundle wires.
pub const PREBUILD_FINGERPRINT_CONSUMER_SURFACES: &[&str] = &[
    "start_center",
    "cli_headless_entry",
    "docs_workspace",
    "support_export",
];

// ---------------------------------------------------------------------------
// Sub-records
// ---------------------------------------------------------------------------

/// Source identity block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildSourceIdentity {
    pub source_repo_ref: String,
    pub workspace_root_ref: String,
    pub commit_or_tree_identity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_or_ref_intent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submodule_lfs_sparse_digest_ref: Option<String>,
    #[serde(default)]
    pub dependency_lock_digest_refs: Vec<String>,
}

/// Environment identity block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildEnvironmentIdentity {
    pub environment_capsule_ref: String,
    pub capsule_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_template_ref: Option<String>,
    pub base_image_or_host_ref: String,
    pub host_class: String,
    pub platform_arch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_topology_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_model_ref: Option<String>,
    pub materializer_version_ref: String,
}

/// Toolchain identity block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildToolchainIdentity {
    pub critical_toolchain_digest_refs: Vec<String>,
    pub extension_lock_digest_ref: String,
    #[serde(default)]
    pub package_manager_lock_digest_refs: Vec<String>,
    #[serde(default)]
    pub known_unsupported_gap_refs: Vec<String>,
}

/// Policy / feature / trust identity block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildPolicyFeatureIdentity {
    pub trust_state: String,
    pub policy_epoch: u64,
    pub policy_bundle_ref: String,
    pub feature_flag_digest_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entitlement_snapshot_ref: Option<String>,
    pub sandbox_profile_ref: String,
    pub egress_posture_ref: String,
}

/// Secret-handle identity block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildSecretHandleIdentity {
    pub secret_handle_set_digest_ref: String,
    #[serde(default)]
    pub projection_mode_refs: Vec<String>,
    pub trust_store_epoch_ref: String,
    pub credential_expiry_posture: String,
}

/// Port / route identity block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildPortRouteIdentity {
    pub declared_port_set_digest_ref: String,
    pub route_dependency_class: String,
    pub collision_key_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exposure_policy_ref: Option<String>,
}

/// One cache artifact carried by a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildCacheArtifact {
    pub cache_class: String,
    pub artifact_ref: String,
    pub digest_ref: String,
    pub included_in_full_reuse: bool,
    #[serde(default = "default_rebuild_if_missing")]
    pub rebuild_if_missing: bool,
}

fn default_rebuild_if_missing() -> bool {
    true
}

/// Redaction / portability posture of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildRedactionPortability {
    pub allowed_export_fields: Vec<String>,
    pub excluded_residue_classes: Vec<String>,
    pub support_export_posture: String,
    pub broadened_capture_approved: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_summary_ref: Option<String>,
}

/// Freshness block of a fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildFreshness {
    pub created_at: String,
    pub last_validated_at: String,
    pub max_reuse_age_seconds: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age_seconds_at_decision: Option<u64>,
    pub freshness_age_class: String,
    pub producer_class: String,
    pub signer_posture: String,
}

// ---------------------------------------------------------------------------
// Top-level records
// ---------------------------------------------------------------------------

/// One alpha prebuild fingerprint record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildFingerprintRecord {
    pub record_kind: String,
    #[serde(rename = "prebuild_fingerprint_schema_version")]
    pub schema_version: u32,
    pub fingerprint_id: String,
    pub prebuild_artifact_ref: String,
    pub source_identity: PrebuildSourceIdentity,
    pub environment_identity: PrebuildEnvironmentIdentity,
    pub toolchain_identity: PrebuildToolchainIdentity,
    pub policy_feature_identity: PrebuildPolicyFeatureIdentity,
    pub secret_handle_identity: PrebuildSecretHandleIdentity,
    pub port_route_identity: PrebuildPortRouteIdentity,
    pub cache_artifacts: Vec<PrebuildCacheArtifact>,
    pub redaction_and_portability: PrebuildRedactionPortability,
    pub freshness: PrebuildFreshness,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_packet_ref: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<JsonValue>,
}

/// One alpha prebuild reuse decision record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildReuseDecisionRecord {
    pub record_kind: String,
    #[serde(rename = "prebuild_fingerprint_schema_version")]
    pub schema_version: u32,
    pub decision_id: String,
    pub requested_path: String,
    pub source_materialization_class: String,
    pub candidate_fingerprint_ref: String,
    pub current_fingerprint_ref: String,
    pub reuse_outcome: String,
    #[serde(default)]
    pub invalidation_bundle_refs: Vec<String>,
    #[serde(default)]
    pub required_revalidations: Vec<String>,
    #[serde(default)]
    pub allowed_partial_cache_classes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_authority_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
    pub disclosure_record_ref: String,
    pub support_packet_inclusion: String,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<JsonValue>,
}

/// One alpha prebuild disclosure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildDisclosureRecord {
    pub record_kind: String,
    #[serde(rename = "prebuild_fingerprint_schema_version")]
    pub schema_version: u32,
    pub disclosure_id: String,
    pub decision_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint_ref: Option<String>,
    pub disclosure_state: String,
    pub requested_path: String,
    pub source_materialization_class: String,
    pub freshness_age_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age_seconds_at_decision: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_reuse_age_seconds: Option<u64>,
    pub host_class: String,
    pub platform_arch: String,
    #[serde(default)]
    pub required_revalidations: Vec<String>,
    pub rebuild_required: bool,
    pub fresh_clone_required: bool,
    pub local_override_disclosed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
    pub stale_snapshot_must_not_be_labeled_live_resume: bool,
    #[serde(default)]
    pub alternative_lane_refs: Vec<String>,
    pub excluded_residue_classes: Vec<String>,
    pub summary: String,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<JsonValue>,
}

/// Sum type discriminated on `record_kind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrebuildAlphaRecord {
    /// A fingerprint record.
    Fingerprint(PrebuildFingerprintRecord),
    /// A reuse decision record.
    ReuseDecision(PrebuildReuseDecisionRecord),
    /// A user / support disclosure record.
    Disclosure(PrebuildDisclosureRecord),
}

impl PrebuildAlphaRecord {
    /// Returns the record_kind discriminator.
    pub fn record_kind(&self) -> &str {
        match self {
            Self::Fingerprint(_) => PREBUILD_FINGERPRINT_RECORD_KIND,
            Self::ReuseDecision(_) => PREBUILD_REUSE_DECISION_RECORD_KIND,
            Self::Disclosure(_) => PREBUILD_DISCLOSURE_RECORD_KIND,
        }
    }

    /// Validates the record against the alpha contract.
    ///
    /// # Errors
    ///
    /// Returns [`PrebuildFingerprintValidationError`] when any frozen
    /// guarantee is violated.
    pub fn validate(&self) -> Result<(), PrebuildFingerprintValidationError> {
        match self {
            Self::Fingerprint(record) => record.validate(),
            Self::ReuseDecision(record) => record.validate(),
            Self::Disclosure(record) => record.validate(),
        }
    }

    /// Projects the record into the compact projection.
    pub fn project(&self) -> PrebuildFingerprintProjection {
        match self {
            Self::Fingerprint(record) => record.project(),
            Self::ReuseDecision(record) => record.project(),
            Self::Disclosure(record) => record.project(),
        }
    }
}

// ---------------------------------------------------------------------------
// Projection
// ---------------------------------------------------------------------------

/// Compact projection of one alpha record for shell / CLI / docs consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrebuildFingerprintProjection {
    /// Stable record id (fingerprint id, decision id, or disclosure id).
    pub record_id: String,
    /// Record kind token.
    pub record_kind: String,
    /// Disclosure label rendered on cards / rows.
    pub display_label: String,
    /// Short reviewable summary.
    pub summary: String,
    /// Requested entry path (`resume_live_workspace`, ...).
    pub requested_path: String,
    /// Source materialization class.
    pub source_materialization_class: String,
    /// Reuse outcome class.
    pub reuse_outcome: String,
    /// Disclosure state class (when projecting a disclosure record).
    pub disclosure_state: String,
    /// Freshness age class.
    pub freshness_age_class: String,
    /// Producer class.
    pub producer_class: String,
    /// Signer posture.
    pub signer_posture: String,
    /// Host class.
    pub host_class: String,
    /// Platform/arch.
    pub platform_arch: String,
    /// Required revalidation classes.
    pub required_revalidations: Vec<String>,
    /// Cache class labels included in the fingerprint or disclosure.
    pub cache_class_labels: Vec<String>,
    /// Excluded residue classes.
    pub excluded_residue_classes: Vec<String>,
    /// Support-export posture.
    pub support_export_posture: String,
    /// Whether rebuild is required.
    pub rebuild_required: bool,
    /// Whether fresh clone is required.
    pub fresh_clone_required: bool,
    /// Whether a local override was disclosed.
    pub local_override_disclosed: bool,
    /// Local override ref when present.
    pub local_override_ref: Option<String>,
    /// Alternative entry-path lanes the user may still take.
    pub alternative_lane_refs: Vec<String>,
    /// Invariant: stale snapshots are never labelled live resume.
    pub stale_snapshot_must_not_be_labeled_live_resume: bool,
}

impl PrebuildFingerprintRecord {
    /// Validates the fingerprint record.
    ///
    /// # Errors
    ///
    /// Returns [`PrebuildFingerprintValidationError`] when any frozen
    /// guarantee is violated.
    pub fn validate(&self) -> Result<(), PrebuildFingerprintValidationError> {
        validate_fingerprint(self)
    }

    /// Projects the fingerprint into the compact projection.
    pub fn project(&self) -> PrebuildFingerprintProjection {
        let cache_class_labels: Vec<String> = self
            .cache_artifacts
            .iter()
            .map(|artifact| artifact.cache_class.clone())
            .collect();
        PrebuildFingerprintProjection {
            record_id: self.fingerprint_id.clone(),
            record_kind: self.record_kind.clone(),
            display_label: format!("Prebuild fingerprint {}", self.fingerprint_id),
            summary: format!(
                "{} on {}; capsule {}",
                self.fingerprint_id,
                self.environment_identity.host_class,
                self.environment_identity.environment_capsule_ref,
            ),
            requested_path: "reuse_cached_prebuild".to_string(),
            source_materialization_class: "prebuilt_snapshot".to_string(),
            reuse_outcome: String::new(),
            disclosure_state: String::new(),
            freshness_age_class: self.freshness.freshness_age_class.clone(),
            producer_class: self.freshness.producer_class.clone(),
            signer_posture: self.freshness.signer_posture.clone(),
            host_class: self.environment_identity.host_class.clone(),
            platform_arch: self.environment_identity.platform_arch.clone(),
            required_revalidations: Vec::new(),
            cache_class_labels,
            excluded_residue_classes: self
                .redaction_and_portability
                .excluded_residue_classes
                .clone(),
            support_export_posture: self
                .redaction_and_portability
                .support_export_posture
                .clone(),
            rebuild_required: false,
            fresh_clone_required: false,
            local_override_disclosed: false,
            local_override_ref: None,
            alternative_lane_refs: Vec::new(),
            stale_snapshot_must_not_be_labeled_live_resume: true,
        }
    }
}

impl PrebuildReuseDecisionRecord {
    /// Validates the decision record.
    ///
    /// # Errors
    ///
    /// Returns [`PrebuildFingerprintValidationError`] when any frozen
    /// guarantee is violated.
    pub fn validate(&self) -> Result<(), PrebuildFingerprintValidationError> {
        validate_reuse_decision(self)
    }

    /// Projects the decision into the compact projection.
    pub fn project(&self) -> PrebuildFingerprintProjection {
        PrebuildFingerprintProjection {
            record_id: self.decision_id.clone(),
            record_kind: self.record_kind.clone(),
            display_label: format!("Reuse decision {}", self.decision_id),
            summary: self
                .blocked_authority_summary
                .clone()
                .unwrap_or_else(|| format!("{} ⇒ {}", self.requested_path, self.reuse_outcome)),
            requested_path: self.requested_path.clone(),
            source_materialization_class: self.source_materialization_class.clone(),
            reuse_outcome: self.reuse_outcome.clone(),
            disclosure_state: String::new(),
            freshness_age_class: String::new(),
            producer_class: String::new(),
            signer_posture: String::new(),
            host_class: String::new(),
            platform_arch: String::new(),
            required_revalidations: self.required_revalidations.clone(),
            cache_class_labels: self.allowed_partial_cache_classes.clone(),
            excluded_residue_classes: Vec::new(),
            support_export_posture: self.support_packet_inclusion.clone(),
            rebuild_required: matches!(self.reuse_outcome.as_str(), "rebuild_required"),
            fresh_clone_required: matches!(self.reuse_outcome.as_str(), "clone_fresh_required"),
            local_override_disclosed: self.local_override_ref.is_some(),
            local_override_ref: self.local_override_ref.clone(),
            alternative_lane_refs: Vec::new(),
            stale_snapshot_must_not_be_labeled_live_resume: true,
        }
    }
}

impl PrebuildDisclosureRecord {
    /// Validates the disclosure record.
    ///
    /// # Errors
    ///
    /// Returns [`PrebuildFingerprintValidationError`] when any frozen
    /// guarantee is violated.
    pub fn validate(&self) -> Result<(), PrebuildFingerprintValidationError> {
        validate_disclosure(self)
    }

    /// Projects the disclosure into the compact projection.
    pub fn project(&self) -> PrebuildFingerprintProjection {
        PrebuildFingerprintProjection {
            record_id: self.disclosure_id.clone(),
            record_kind: self.record_kind.clone(),
            display_label: format!("Prebuild disclosure {}", self.disclosure_id),
            summary: self.summary.clone(),
            requested_path: self.requested_path.clone(),
            source_materialization_class: self.source_materialization_class.clone(),
            reuse_outcome: String::new(),
            disclosure_state: self.disclosure_state.clone(),
            freshness_age_class: self.freshness_age_class.clone(),
            producer_class: String::new(),
            signer_posture: String::new(),
            host_class: self.host_class.clone(),
            platform_arch: self.platform_arch.clone(),
            required_revalidations: self.required_revalidations.clone(),
            cache_class_labels: Vec::new(),
            excluded_residue_classes: self.excluded_residue_classes.clone(),
            support_export_posture: String::new(),
            rebuild_required: self.rebuild_required,
            fresh_clone_required: self.fresh_clone_required,
            local_override_disclosed: self.local_override_disclosed,
            local_override_ref: self.local_override_ref.clone(),
            alternative_lane_refs: self.alternative_lane_refs.clone(),
            stale_snapshot_must_not_be_labeled_live_resume: self
                .stale_snapshot_must_not_be_labeled_live_resume,
        }
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validation failure for a prebuild fingerprint family record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrebuildFingerprintValidationError {
    message: String,
}

impl PrebuildFingerprintValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for PrebuildFingerprintValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "prebuild fingerprint validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for PrebuildFingerprintValidationError {}

/// Error returned when an alpha record payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrebuildFingerprintError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha contract.
    Validation(PrebuildFingerprintValidationError),
}

impl PrebuildFingerprintError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for PrebuildFingerprintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "prebuild fingerprint JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PrebuildFingerprintError {}

/// Parses an alpha JSON payload, dispatches by `record_kind`, validates, and
/// returns the compact projection.
///
/// # Errors
///
/// Returns [`PrebuildFingerprintError`] when the payload cannot be parsed,
/// the record kind is unknown, or the alpha contract is violated.
pub fn project_prebuild_fingerprint_alpha(
    payload: &str,
) -> Result<PrebuildFingerprintProjection, PrebuildFingerprintError> {
    let record = parse_prebuild_alpha_record(payload)?;
    record
        .validate()
        .map_err(PrebuildFingerprintError::Validation)?;
    Ok(record.project())
}

/// Parses an alpha payload into a [`PrebuildAlphaRecord`] without validating.
///
/// # Errors
///
/// Returns [`PrebuildFingerprintError::Json`] for JSON parse failures and for
/// unknown `record_kind` tokens.
pub fn parse_prebuild_alpha_record(
    payload: &str,
) -> Result<PrebuildAlphaRecord, PrebuildFingerprintError> {
    let value: JsonValue = serde_json::from_str(payload)
        .map_err(|err| PrebuildFingerprintError::Json(err.to_string()))?;
    let object = value
        .as_object()
        .ok_or_else(|| PrebuildFingerprintError::Json("payload must be a JSON object".into()))?;
    let record_kind = object
        .get("record_kind")
        .and_then(JsonValue::as_str)
        .ok_or_else(|| {
            PrebuildFingerprintError::Json("payload missing record_kind discriminator".into())
        })?;
    match record_kind {
        PREBUILD_FINGERPRINT_RECORD_KIND => {
            let record: PrebuildFingerprintRecord = serde_json::from_value(value)
                .map_err(|err| PrebuildFingerprintError::Json(err.to_string()))?;
            Ok(PrebuildAlphaRecord::Fingerprint(record))
        }
        PREBUILD_REUSE_DECISION_RECORD_KIND => {
            let record: PrebuildReuseDecisionRecord = serde_json::from_value(value)
                .map_err(|err| PrebuildFingerprintError::Json(err.to_string()))?;
            Ok(PrebuildAlphaRecord::ReuseDecision(record))
        }
        PREBUILD_DISCLOSURE_RECORD_KIND => {
            let record: PrebuildDisclosureRecord = serde_json::from_value(value)
                .map_err(|err| PrebuildFingerprintError::Json(err.to_string()))?;
            Ok(PrebuildAlphaRecord::Disclosure(record))
        }
        other => Err(PrebuildFingerprintError::Json(format!(
            "unknown record_kind {other}"
        ))),
    }
}

fn validate_fingerprint(
    record: &PrebuildFingerprintRecord,
) -> Result<(), PrebuildFingerprintValidationError> {
    require_equal(
        "record_kind",
        PREBUILD_FINGERPRINT_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION {
        return Err(PrebuildFingerprintValidationError::new(format!(
            "prebuild_fingerprint_schema_version is {}, expected {}",
            record.schema_version, PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION
        )));
    }
    require_non_empty("fingerprint_id", &record.fingerprint_id)?;
    require_non_empty("prebuild_artifact_ref", &record.prebuild_artifact_ref)?;
    require_non_empty(
        "source_identity.source_repo_ref",
        &record.source_identity.source_repo_ref,
    )?;
    require_non_empty(
        "source_identity.workspace_root_ref",
        &record.source_identity.workspace_root_ref,
    )?;
    require_non_empty(
        "source_identity.commit_or_tree_identity",
        &record.source_identity.commit_or_tree_identity,
    )?;
    require_one_of(
        "environment_identity.host_class",
        HOST_CLASSES,
        &record.environment_identity.host_class,
    )?;
    require_one_of(
        "environment_identity.platform_arch",
        PLATFORM_ARCH_CLASSES,
        &record.environment_identity.platform_arch,
    )?;
    require_non_empty(
        "environment_identity.environment_capsule_ref",
        &record.environment_identity.environment_capsule_ref,
    )?;
    require_non_empty(
        "environment_identity.capsule_hash",
        &record.environment_identity.capsule_hash,
    )?;
    require_non_empty(
        "environment_identity.materializer_version_ref",
        &record.environment_identity.materializer_version_ref,
    )?;

    if record
        .toolchain_identity
        .critical_toolchain_digest_refs
        .is_empty()
    {
        return Err(PrebuildFingerprintValidationError::new(
            "toolchain_identity.critical_toolchain_digest_refs must list at least one digest",
        ));
    }
    require_unique(
        "toolchain_identity.critical_toolchain_digest_refs",
        &record.toolchain_identity.critical_toolchain_digest_refs,
    )?;
    require_non_empty(
        "toolchain_identity.extension_lock_digest_ref",
        &record.toolchain_identity.extension_lock_digest_ref,
    )?;

    require_one_of(
        "policy_feature_identity.trust_state",
        TRUST_STATES,
        &record.policy_feature_identity.trust_state,
    )?;
    require_non_empty(
        "policy_feature_identity.policy_bundle_ref",
        &record.policy_feature_identity.policy_bundle_ref,
    )?;
    require_non_empty(
        "policy_feature_identity.feature_flag_digest_ref",
        &record.policy_feature_identity.feature_flag_digest_ref,
    )?;
    require_non_empty(
        "policy_feature_identity.sandbox_profile_ref",
        &record.policy_feature_identity.sandbox_profile_ref,
    )?;
    require_non_empty(
        "policy_feature_identity.egress_posture_ref",
        &record.policy_feature_identity.egress_posture_ref,
    )?;

    require_one_of(
        "secret_handle_identity.credential_expiry_posture",
        CREDENTIAL_EXPIRY_POSTURES,
        &record.secret_handle_identity.credential_expiry_posture,
    )?;
    require_non_empty(
        "secret_handle_identity.secret_handle_set_digest_ref",
        &record.secret_handle_identity.secret_handle_set_digest_ref,
    )?;

    require_one_of(
        "port_route_identity.route_dependency_class",
        ROUTE_DEPENDENCY_CLASSES,
        &record.port_route_identity.route_dependency_class,
    )?;
    require_non_empty(
        "port_route_identity.declared_port_set_digest_ref",
        &record.port_route_identity.declared_port_set_digest_ref,
    )?;
    require_non_empty(
        "port_route_identity.collision_key_ref",
        &record.port_route_identity.collision_key_ref,
    )?;

    if record.cache_artifacts.is_empty() {
        return Err(PrebuildFingerprintValidationError::new(
            "cache_artifacts must list at least one cache artifact",
        ));
    }
    let mut seen_artifact_refs: BTreeSet<&str> = BTreeSet::new();
    for artifact in &record.cache_artifacts {
        require_one_of(
            "cache_artifacts[].cache_class",
            CACHE_CLASSES,
            &artifact.cache_class,
        )?;
        require_non_empty("cache_artifacts[].artifact_ref", &artifact.artifact_ref)?;
        require_non_empty("cache_artifacts[].digest_ref", &artifact.digest_ref)?;
        if !seen_artifact_refs.insert(artifact.artifact_ref.as_str()) {
            return Err(PrebuildFingerprintValidationError::new(format!(
                "cache_artifacts contains a duplicate artifact_ref: {}",
                artifact.artifact_ref
            )));
        }
    }

    validate_redaction(
        &record.redaction_and_portability,
        REQUIRED_EXCLUDED_RESIDUE_FOR_FINGERPRINT,
    )?;

    require_one_of(
        "freshness.freshness_age_class",
        FRESHNESS_AGE_CLASSES,
        &record.freshness.freshness_age_class,
    )?;
    require_one_of(
        "freshness.producer_class",
        PRODUCER_CLASSES,
        &record.freshness.producer_class,
    )?;
    require_one_of(
        "freshness.signer_posture",
        SIGNER_POSTURE_CLASSES,
        &record.freshness.signer_posture,
    )?;
    require_non_empty("freshness.created_at", &record.freshness.created_at)?;
    require_non_empty(
        "freshness.last_validated_at",
        &record.freshness.last_validated_at,
    )?;
    Ok(())
}

fn validate_reuse_decision(
    record: &PrebuildReuseDecisionRecord,
) -> Result<(), PrebuildFingerprintValidationError> {
    require_equal(
        "record_kind",
        PREBUILD_REUSE_DECISION_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION {
        return Err(PrebuildFingerprintValidationError::new(format!(
            "prebuild_fingerprint_schema_version is {}, expected {}",
            record.schema_version, PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION
        )));
    }
    require_non_empty("decision_id", &record.decision_id)?;
    require_non_empty(
        "candidate_fingerprint_ref",
        &record.candidate_fingerprint_ref,
    )?;
    require_non_empty("current_fingerprint_ref", &record.current_fingerprint_ref)?;
    require_non_empty("disclosure_record_ref", &record.disclosure_record_ref)?;
    require_one_of(
        "requested_path",
        PREBUILD_PATH_CLASSES,
        &record.requested_path,
    )?;
    require_one_of(
        "source_materialization_class",
        SOURCE_MATERIALIZATION_CLASSES,
        &record.source_materialization_class,
    )?;
    require_one_of(
        "reuse_outcome",
        REUSE_OUTCOME_CLASSES,
        &record.reuse_outcome,
    )?;
    require_one_of(
        "support_packet_inclusion",
        SUPPORT_PACKET_INCLUSION_CLASSES,
        &record.support_packet_inclusion,
    )?;

    for cls in &record.required_revalidations {
        require_one_of(
            "required_revalidations[]",
            REQUIRED_REVALIDATION_CLASSES,
            cls,
        )?;
    }
    require_unique("required_revalidations", &record.required_revalidations)?;
    require_unique("invalidation_bundle_refs", &record.invalidation_bundle_refs)?;
    for cls in &record.allowed_partial_cache_classes {
        require_one_of("allowed_partial_cache_classes[]", CACHE_CLASSES, cls)?;
    }
    require_unique(
        "allowed_partial_cache_classes",
        &record.allowed_partial_cache_classes,
    )?;

    if record.requested_path == "resume_live_workspace"
        && matches!(
            record.source_materialization_class.as_str(),
            "prebuilt_snapshot" | "stale_prebuild_snapshot"
        )
        && record.reuse_outcome != "resume_live_denied"
    {
        return Err(PrebuildFingerprintValidationError::new(
            "resume_live_workspace against a snapshot must resolve to resume_live_denied",
        ));
    }

    if record.reuse_outcome == "reuse_allowed" {
        if !record.invalidation_bundle_refs.is_empty() {
            return Err(PrebuildFingerprintValidationError::new(
                "reuse_allowed decision must not list invalidation_bundle_refs",
            ));
        }
        if !record.required_revalidations.is_empty() {
            return Err(PrebuildFingerprintValidationError::new(
                "reuse_allowed decision must not require revalidation",
            ));
        }
    }

    if record.requested_path == "clone_fresh"
        && record.source_materialization_class != "fresh_clone_materialization"
        && record.reuse_outcome != "resume_live_denied"
    {
        return Err(PrebuildFingerprintValidationError::new(
            "clone_fresh path must materialize as fresh_clone_materialization",
        ));
    }

    Ok(())
}

fn validate_disclosure(
    record: &PrebuildDisclosureRecord,
) -> Result<(), PrebuildFingerprintValidationError> {
    require_equal(
        "record_kind",
        PREBUILD_DISCLOSURE_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION {
        return Err(PrebuildFingerprintValidationError::new(format!(
            "prebuild_fingerprint_schema_version is {}, expected {}",
            record.schema_version, PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION
        )));
    }
    require_non_empty("disclosure_id", &record.disclosure_id)?;
    require_non_empty("decision_ref", &record.decision_ref)?;
    require_non_empty("summary", &record.summary)?;
    require_one_of(
        "disclosure_state",
        DISCLOSURE_STATE_CLASSES,
        &record.disclosure_state,
    )?;
    require_one_of(
        "requested_path",
        PREBUILD_PATH_CLASSES,
        &record.requested_path,
    )?;
    require_one_of(
        "source_materialization_class",
        SOURCE_MATERIALIZATION_CLASSES,
        &record.source_materialization_class,
    )?;
    require_one_of(
        "freshness_age_class",
        FRESHNESS_AGE_CLASSES,
        &record.freshness_age_class,
    )?;
    require_one_of("host_class", HOST_CLASSES, &record.host_class)?;
    require_one_of(
        "platform_arch",
        PLATFORM_ARCH_CLASSES,
        &record.platform_arch,
    )?;
    for cls in &record.required_revalidations {
        require_one_of(
            "required_revalidations[]",
            REQUIRED_REVALIDATION_CLASSES,
            cls,
        )?;
    }
    require_unique("required_revalidations", &record.required_revalidations)?;
    for cls in &record.alternative_lane_refs {
        require_one_of("alternative_lane_refs[]", PREBUILD_PATH_CLASSES, cls)?;
    }
    require_unique("alternative_lane_refs", &record.alternative_lane_refs)?;
    require_unique("excluded_residue_classes", &record.excluded_residue_classes)?;
    for cls in &record.excluded_residue_classes {
        require_one_of("excluded_residue_classes[]", EXCLUDED_RESIDUE_CLASSES, cls)?;
    }
    for required in REQUIRED_EXCLUDED_RESIDUE_FOR_DISCLOSURE {
        if !record
            .excluded_residue_classes
            .iter()
            .any(|c| c == required)
        {
            return Err(PrebuildFingerprintValidationError::new(format!(
                "excluded_residue_classes must include {required}"
            )));
        }
    }

    if !record.stale_snapshot_must_not_be_labeled_live_resume {
        return Err(PrebuildFingerprintValidationError::new(
            "stale_snapshot_must_not_be_labeled_live_resume must remain true",
        ));
    }

    if record.local_override_disclosed && record.local_override_ref.is_none() {
        return Err(PrebuildFingerprintValidationError::new(
            "local_override_disclosed=true must bind a local_override_ref",
        ));
    }

    match record.disclosure_state.as_str() {
        "fresh_clone" => {
            if !record.fresh_clone_required {
                return Err(PrebuildFingerprintValidationError::new(
                    "fresh_clone disclosure must set fresh_clone_required = true",
                ));
            }
        }
        "stale_prebuild_rebuild_required" | "local_override_rebuild_required" => {
            if !record.rebuild_required {
                return Err(PrebuildFingerprintValidationError::new(
                    "rebuild-required disclosure must set rebuild_required = true",
                ));
            }
        }
        "resume_denied_snapshot_available" => {
            if record.requested_path != "resume_live_workspace" {
                return Err(PrebuildFingerprintValidationError::new(
                    "resume_denied_snapshot_available disclosure must be on a resume_live_workspace request",
                ));
            }
        }
        _ => {}
    }

    Ok(())
}

fn validate_redaction(
    redaction: &PrebuildRedactionPortability,
    required: &[&str],
) -> Result<(), PrebuildFingerprintValidationError> {
    if redaction.broadened_capture_approved {
        return Err(PrebuildFingerprintValidationError::new(
            "redaction_and_portability.broadened_capture_approved must remain false",
        ));
    }
    require_one_of(
        "redaction_and_portability.support_export_posture",
        SUPPORT_EXPORT_POSTURES,
        &redaction.support_export_posture,
    )?;
    if redaction.allowed_export_fields.is_empty() {
        return Err(PrebuildFingerprintValidationError::new(
            "redaction_and_portability.allowed_export_fields must list at least one export field class",
        ));
    }
    require_unique(
        "redaction_and_portability.allowed_export_fields",
        &redaction.allowed_export_fields,
    )?;
    for field in &redaction.allowed_export_fields {
        require_one_of(
            "redaction_and_portability.allowed_export_fields[]",
            EXPORT_FIELD_CLASSES,
            field,
        )?;
    }
    require_unique(
        "redaction_and_portability.excluded_residue_classes",
        &redaction.excluded_residue_classes,
    )?;
    for cls in &redaction.excluded_residue_classes {
        require_one_of(
            "redaction_and_portability.excluded_residue_classes[]",
            EXCLUDED_RESIDUE_CLASSES,
            cls,
        )?;
    }
    for required_cls in required {
        if !redaction
            .excluded_residue_classes
            .iter()
            .any(|c| c == required_cls)
        {
            return Err(PrebuildFingerprintValidationError::new(format!(
                "redaction_and_portability.excluded_residue_classes must include {required_cls}"
            )));
        }
    }
    Ok(())
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), PrebuildFingerprintValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(PrebuildFingerprintValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), PrebuildFingerprintValidationError> {
    if value.trim().is_empty() {
        Err(PrebuildFingerprintValidationError::new(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn require_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), PrebuildFingerprintValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(PrebuildFingerprintValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), PrebuildFingerprintValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(PrebuildFingerprintValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_FINGERPRINT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/prebuild_fingerprint/valid_cached_prebuild_fingerprint.json"
    ));

    const FIXTURE_REUSE_ALLOWED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/prebuild_fingerprint/reuse_allowed_decision.json"
    ));

    const FIXTURE_RESUME_DENIED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/prebuild_fingerprint/stale_snapshot_resume_denied_decision.json"
    ));

    const FIXTURE_LOCAL_OVERRIDE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/prebuild_fingerprint/local_override_rebuild_disclosure.json"
    ));

    const FIXTURE_FRESH_CLONE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/prebuild_fingerprint/fresh_clone_disclosure.json"
    ));

    #[test]
    fn fingerprint_fixture_projects() {
        let projection = project_prebuild_fingerprint_alpha(FIXTURE_FINGERPRINT)
            .expect("fingerprint fixture must project");
        assert_eq!(projection.record_kind, PREBUILD_FINGERPRINT_RECORD_KIND);
        assert_eq!(projection.freshness_age_class, "fresh_under_window");
        assert!(projection.stale_snapshot_must_not_be_labeled_live_resume);
        assert!(projection
            .excluded_residue_classes
            .iter()
            .any(|c| c == "raw_secret_material"));
    }

    #[test]
    fn reuse_allowed_fixture_projects() {
        let projection = project_prebuild_fingerprint_alpha(FIXTURE_REUSE_ALLOWED)
            .expect("reuse-allowed fixture must project");
        assert_eq!(projection.requested_path, "reuse_cached_prebuild");
        assert_eq!(projection.reuse_outcome, "reuse_allowed");
        assert!(projection.required_revalidations.is_empty());
    }

    #[test]
    fn resume_denied_fixture_projects() {
        let projection = project_prebuild_fingerprint_alpha(FIXTURE_RESUME_DENIED)
            .expect("resume-denied fixture must project");
        assert_eq!(projection.requested_path, "resume_live_workspace");
        assert_eq!(projection.reuse_outcome, "resume_live_denied");
    }

    #[test]
    fn local_override_disclosure_projects() {
        let projection = project_prebuild_fingerprint_alpha(FIXTURE_LOCAL_OVERRIDE)
            .expect("local override fixture must project");
        assert!(projection.rebuild_required);
        assert!(projection.local_override_disclosed);
        assert_eq!(
            projection.disclosure_state,
            "local_override_rebuild_required"
        );
    }

    #[test]
    fn fresh_clone_disclosure_projects() {
        let projection = project_prebuild_fingerprint_alpha(FIXTURE_FRESH_CLONE)
            .expect("fresh clone fixture must project");
        assert_eq!(projection.disclosure_state, "fresh_clone");
        assert!(projection.fresh_clone_required);
        assert!(!projection.rebuild_required);
    }

    #[test]
    fn rejects_resume_live_on_snapshot_with_allowed_outcome() {
        let payload = FIXTURE_RESUME_DENIED.replace(
            "\"reuse_outcome\": \"resume_live_denied\"",
            "\"reuse_outcome\": \"reuse_allowed\"",
        );
        let err = project_prebuild_fingerprint_alpha(&payload)
            .expect_err("resume on snapshot cannot be allowed");
        assert!(err.message().to_lowercase().contains("resume_live_denied"));
    }

    #[test]
    fn rejects_disclosure_with_widened_resume_invariant() {
        let payload = FIXTURE_LOCAL_OVERRIDE.replace(
            "\"stale_snapshot_must_not_be_labeled_live_resume\": true",
            "\"stale_snapshot_must_not_be_labeled_live_resume\": false",
        );
        let err = project_prebuild_fingerprint_alpha(&payload)
            .expect_err("stale-snapshot invariant cannot be widened");
        assert!(err
            .message()
            .to_lowercase()
            .contains("stale_snapshot_must_not_be_labeled_live_resume"));
    }

    #[test]
    fn rejects_reuse_allowed_with_revalidations() {
        let mut record: PrebuildReuseDecisionRecord =
            serde_json::from_str(FIXTURE_REUSE_ALLOWED).expect("fixture must parse");
        record.required_revalidations.push("trust".to_string());
        let err = record
            .validate()
            .expect_err("reuse_allowed cannot require revalidations");
        assert!(err.message().contains("reuse_allowed"));
    }

    #[test]
    fn rejects_fingerprint_with_broadened_capture() {
        let mut record: PrebuildFingerprintRecord =
            serde_json::from_str(FIXTURE_FINGERPRINT).expect("fingerprint fixture must parse");
        record.redaction_and_portability.broadened_capture_approved = true;
        let err = record
            .validate()
            .expect_err("broadened capture must be denied");
        assert!(err.message().contains("broadened_capture_approved"));
    }
}
