//! Workspace-template bundle alpha — pre-execution review record.
//!
//! A [`WorkspaceTemplateBundleRecord`] is the durable, exportable bundle that
//! Start Center, CLI / headless entry, docs, and support packets read **before
//! a template is used**. It restates the same source class, support class,
//! target runtime, side effects, trust posture, and open-without-starter
//! bypass routes named by the underlying signed manifest, and projects them
//! into one closed record so consumer surfaces never invent parallel
//! disclosure vocabulary.
//!
//! The bundle is intentionally narrower than the scaffold-run packet in
//! [`crate::generated_projects`]: it never describes the preflight write plan,
//! the scaffold run, or the lineage record. Those records still own
//! generation. The bundle owns **review**: what the user is shown about a
//! template's identity, side effects, and bypass options ahead of any choice
//! to apply.
//!
//! Frozen guarantees enforced here:
//!
//! 1. Every bundle cites a bound template-manifest reference, a stable
//!    template id, and a template version so support packets and CLI surfaces
//!    quote the same identity.
//! 2. The bypass review carries at least one
//!    `open_without_starter_route_id`, and its `bypass_continuity_class` is
//!    `equal_weight_with_apply` — the user can always open the workspace
//!    plainly when the product contract allows it.
//! 3. Side-effect review names the egress, extension-install, remote
//!    provisioning, managed-service, and credential classes. A
//!    `managed_cloud_required` target runtime is not allowed to claim
//!    `no_remote_provisioning_required` or `no_managed_service_required`.
//! 4. Community / uncertified source classes carry at least one trust note;
//!    the bundle never hides a missing signer behind a generic posture.
//! 5. Support export is closed: raw secrets, raw command lines, and raw URLs
//!    never cross this record's boundary.
//! 6. At least one consumer surface (Start Center, CLI / headless entry,
//!    docs, support) is bound, and the review invariants
//!    (`reviewed_before_use`, `inspectable_before_execution`,
//!    `no_writes_before_review`) are all true.
//!
//! The companion schema lives at
//! `schemas/workspace/template_bundle.schema.json`. The reviewer doc lives at
//! `docs/workspace/m3/template_bundle_alpha.md`. Canonical fixtures live under
//! `fixtures/workspace/m3/template_bundle/`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every alpha workspace-template bundle record.
pub const WORKSPACE_TEMPLATE_BUNDLE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for [`WorkspaceTemplateBundleRecord`].
pub const WORKSPACE_TEMPLATE_BUNDLE_ALPHA_RECORD_KIND: &str =
    "workspace_template_bundle_alpha_record";

/// Closed set of source classes the bundle re-exports from the manifest.
pub const TEMPLATE_BUNDLE_SOURCE_CLASSES: &[&str] = &[
    "first_party",
    "team_managed",
    "community",
    "local_only",
    "mirror_cached",
    "uncertified",
];

/// Closed set of support classes the bundle re-exports from the manifest.
pub const TEMPLATE_BUNDLE_SUPPORT_CLASSES: &[&str] = &[
    "officially_supported",
    "community_supported",
    "experimental",
    "legacy_deprecated",
    "unsupported",
    "support_unknown",
];

/// Closed set of runtime / toolchain scope classes.
pub const TEMPLATE_BUNDLE_RUNTIME_SCOPE_CLASSES: &[&str] = &[
    "local_only",
    "local_with_devcontainer",
    "local_with_container",
    "remote_image_required",
    "managed_cloud_required",
    "mixed_local_and_remote",
    "not_declared",
];

/// Closed set of host-boundary classes that must agree with runtime scope.
pub const TEMPLATE_BUNDLE_HOST_BOUNDARY_CLASSES: &[&str] = &[
    "host_local_device_only",
    "host_local_with_devcontainer_attached",
    "host_local_with_container_attached",
    "host_remote_image_required",
    "host_managed_workspace_required",
    "host_mixed_local_and_remote",
    "host_boundary_unknown_requires_review",
];

/// Closed set of bundle consumer surfaces.
pub const TEMPLATE_BUNDLE_CONSUMER_SURFACES: &[&str] = &[
    "start_center",
    "cli_headless_entry",
    "docs_workspace",
    "support_export",
];

/// Equal-weight bypass-continuity class required for every bundle.
pub const TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT: &str = "equal_weight_with_apply";

/// Closed set of network-egress classes.
pub const TEMPLATE_BUNDLE_NETWORK_EGRESS_CLASSES: &[&str] = &[
    "no_network_egress_required",
    "egress_to_first_party_origin_only",
    "egress_to_team_managed_mirror_only",
    "egress_to_community_origin_user_review_required",
    "egress_to_managed_workspace_envelope_only",
    "egress_envelope_unknown_requires_review",
];

/// Closed set of extension-install classes.
pub const TEMPLATE_BUNDLE_EXTENSION_INSTALL_CLASSES: &[&str] = &[
    "no_extension_install_required",
    "first_party_extension_install_required",
    "organization_curated_extension_install_required",
    "marketplace_extension_install_user_review_required",
    "managed_only_channel_extension_install_required",
    "extension_install_review_required_signature_unverified",
    "extension_install_class_unknown_requires_review",
];

/// Closed set of remote-provisioning classes.
pub const TEMPLATE_BUNDLE_REMOTE_PROVISIONING_CLASSES: &[&str] = &[
    "no_remote_provisioning_required",
    "devcontainer_attach_required",
    "container_attach_required",
    "remote_image_required",
    "managed_workspace_required",
    "mixed_local_and_remote_provisioning_required",
    "remote_provisioning_unknown_requires_review",
];

/// Closed set of managed-service classes.
pub const TEMPLATE_BUNDLE_MANAGED_SERVICE_CLASSES: &[&str] = &[
    "no_managed_service_required",
    "managed_workspace_envelope_required",
    "managed_only_channel_invocation_required",
    "third_party_connected_provider_required",
    "first_party_managed_service_required",
    "managed_service_class_unknown_requires_review",
];

/// Closed set of credential-provisioning classes.
pub const TEMPLATE_BUNDLE_CREDENTIAL_PROVISIONING_CLASSES: &[&str] = &[
    "no_credential_provisioning_required",
    "secret_broker_handle_required",
    "credential_provisioning_step_required",
    "remote_attach_handshake_required",
    "credential_provisioning_class_unknown_requires_review",
];

/// Closed set of bypass route ids the bundle may advertise.
pub const TEMPLATE_BUNDLE_BYPASS_ROUTE_IDS: &[&str] = &[
    "bypass.open_folder_without_starter",
    "bypass.open_workspace_without_starter",
    "bypass.clone_repository_without_starter",
    "bypass.create_empty_workspace",
    "bypass.open_prebuild_minimal",
    "bypass.set_up_later",
    "bypass.continue_without_starter",
];

/// Source disclosure block carried on the bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleSourceReview {
    /// `first_party`, `team_managed`, `community`, `local_only`,
    /// `mirror_cached`, or `uncertified`.
    pub source_class: String,
    /// Distribution channel for the source artifact.
    pub source_distribution_class: String,
    /// Signature state shown next to the title.
    pub signature_state: String,
    /// Stable opaque publisher reference.
    pub publisher_label: String,
    /// Trust-root reference quoted in support packets.
    pub trust_root_ref: String,
}

/// Support class disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleSupportReview {
    /// Support posture shown on the card.
    pub support_class: String,
    /// Lifecycle posture mirroring the manifest.
    pub lifecycle_class: String,
}

/// Target runtime disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleTargetRuntimeReview {
    /// `local_only`, `managed_cloud_required`, etc.
    pub runtime_scope_class: String,
    /// Host-boundary disclosure class.
    pub host_boundary_class: String,
    /// Supported ecosystem class names (closed vocabulary, opaque tokens).
    pub supported_ecosystems: Vec<String>,
    /// Supported platform class names (closed vocabulary, opaque tokens).
    pub supported_platforms: Vec<String>,
}

/// Side-effect disclosure block consumed by review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleSideEffectReview {
    pub required_network_egress_class: String,
    pub required_extension_install_class: String,
    pub required_remote_provisioning_class: String,
    pub required_managed_service_class: String,
    pub required_credential_provisioning_class: String,
    /// Count of declared scaffold hooks bound on the manifest.
    pub declared_hook_count: u32,
    /// Count of declared scaffold setup tasks bound on the manifest.
    pub declared_setup_task_count: u32,
    /// Short reviewable sentences quoting the side effects.
    pub side_effect_notes: Vec<String>,
}

/// Trust posture and egress posture disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleTrustReview {
    pub trust_posture_class: String,
    pub egress_posture_class: String,
    pub trust_notes: Vec<String>,
}

/// Bypass disclosure ensuring "open without a starter" stays at equal weight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleBypassReview {
    pub open_without_starter_route_ids: Vec<String>,
    /// Always `equal_weight_with_apply`.
    pub bypass_continuity_class: String,
    /// Optional bypass guidance shown next to the routes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_guidance: Option<String>,
}

/// Closed support-export disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleSupportExport {
    pub export_packet_refs: Vec<String>,
    pub raw_secret_export_allowed: bool,
    pub raw_command_export_allowed: bool,
    pub raw_url_export_allowed: bool,
    pub redaction_class: String,
}

/// Review invariants the bundle must claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleReviewInvariants {
    pub reviewed_before_use: bool,
    pub inspectable_before_execution: bool,
    pub no_writes_before_review: bool,
}

/// One alpha workspace-template bundle record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTemplateBundleRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub bundle_id: String,
    pub bound_template_manifest_ref: String,
    pub bound_template_id: String,
    pub bound_template_version: String,
    pub display_label: String,
    pub summary: String,
    pub source_review: WorkspaceTemplateBundleSourceReview,
    pub support_review: WorkspaceTemplateBundleSupportReview,
    pub target_runtime_review: WorkspaceTemplateBundleTargetRuntimeReview,
    pub side_effect_review: WorkspaceTemplateBundleSideEffectReview,
    pub trust_review: WorkspaceTemplateBundleTrustReview,
    pub bypass_review: WorkspaceTemplateBundleBypassReview,
    pub consumer_surfaces: Vec<String>,
    pub support_export: WorkspaceTemplateBundleSupportExport,
    pub review_invariants: WorkspaceTemplateBundleReviewInvariants,
    pub minted_at: String,
}

/// Compact projection of one bundle for shell / CLI / doc consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceTemplateBundleProjection {
    pub bundle_id: String,
    pub bound_template_id: String,
    pub bound_template_version: String,
    pub display_label: String,
    pub summary: String,
    pub source_class: String,
    pub source_distribution_class: String,
    pub signature_state: String,
    pub publisher_label: String,
    pub support_class: String,
    pub lifecycle_class: String,
    pub runtime_scope_class: String,
    pub host_boundary_class: String,
    pub supported_ecosystems: Vec<String>,
    pub supported_platforms: Vec<String>,
    pub required_network_egress_class: String,
    pub required_extension_install_class: String,
    pub required_remote_provisioning_class: String,
    pub required_managed_service_class: String,
    pub required_credential_provisioning_class: String,
    pub declared_hook_count: u32,
    pub declared_setup_task_count: u32,
    pub side_effect_notes: Vec<String>,
    pub trust_posture_class: String,
    pub egress_posture_class: String,
    pub trust_notes: Vec<String>,
    pub open_without_starter_route_ids: Vec<String>,
    pub bypass_continuity_class: String,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
    pub raw_secret_export_allowed: bool,
    pub raw_command_export_allowed: bool,
    pub raw_url_export_allowed: bool,
}

impl WorkspaceTemplateBundleRecord {
    /// Validates the record against the alpha bundle contract.
    ///
    /// # Errors
    ///
    /// Returns [`WorkspaceTemplateBundleValidationError`] when any frozen
    /// guarantee is violated.
    pub fn validate(&self) -> Result<(), WorkspaceTemplateBundleValidationError> {
        validate_record(self)
    }

    /// Projects the bundle into the compact shell / CLI / docs surface row.
    pub fn project(&self) -> WorkspaceTemplateBundleProjection {
        WorkspaceTemplateBundleProjection {
            bundle_id: self.bundle_id.clone(),
            bound_template_id: self.bound_template_id.clone(),
            bound_template_version: self.bound_template_version.clone(),
            display_label: self.display_label.clone(),
            summary: self.summary.clone(),
            source_class: self.source_review.source_class.clone(),
            source_distribution_class: self.source_review.source_distribution_class.clone(),
            signature_state: self.source_review.signature_state.clone(),
            publisher_label: self.source_review.publisher_label.clone(),
            support_class: self.support_review.support_class.clone(),
            lifecycle_class: self.support_review.lifecycle_class.clone(),
            runtime_scope_class: self.target_runtime_review.runtime_scope_class.clone(),
            host_boundary_class: self.target_runtime_review.host_boundary_class.clone(),
            supported_ecosystems: self.target_runtime_review.supported_ecosystems.clone(),
            supported_platforms: self.target_runtime_review.supported_platforms.clone(),
            required_network_egress_class: self
                .side_effect_review
                .required_network_egress_class
                .clone(),
            required_extension_install_class: self
                .side_effect_review
                .required_extension_install_class
                .clone(),
            required_remote_provisioning_class: self
                .side_effect_review
                .required_remote_provisioning_class
                .clone(),
            required_managed_service_class: self
                .side_effect_review
                .required_managed_service_class
                .clone(),
            required_credential_provisioning_class: self
                .side_effect_review
                .required_credential_provisioning_class
                .clone(),
            declared_hook_count: self.side_effect_review.declared_hook_count,
            declared_setup_task_count: self.side_effect_review.declared_setup_task_count,
            side_effect_notes: self.side_effect_review.side_effect_notes.clone(),
            trust_posture_class: self.trust_review.trust_posture_class.clone(),
            egress_posture_class: self.trust_review.egress_posture_class.clone(),
            trust_notes: self.trust_review.trust_notes.clone(),
            open_without_starter_route_ids: self
                .bypass_review
                .open_without_starter_route_ids
                .clone(),
            bypass_continuity_class: self.bypass_review.bypass_continuity_class.clone(),
            consumer_surfaces: self.consumer_surfaces.clone(),
            support_export_refs: self.support_export.export_packet_refs.clone(),
            redaction_class: self.support_export.redaction_class.clone(),
            raw_secret_export_allowed: self.support_export.raw_secret_export_allowed,
            raw_command_export_allowed: self.support_export.raw_command_export_allowed,
            raw_url_export_allowed: self.support_export.raw_url_export_allowed,
        }
    }
}

/// Validation failure for a workspace-template bundle record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceTemplateBundleValidationError {
    message: String,
}

impl WorkspaceTemplateBundleValidationError {
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

impl fmt::Display for WorkspaceTemplateBundleValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "workspace template bundle validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for WorkspaceTemplateBundleValidationError {}

/// Error returned when an alpha bundle payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceTemplateBundleError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the alpha bundle contract.
    Validation(WorkspaceTemplateBundleValidationError),
}

impl WorkspaceTemplateBundleError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for WorkspaceTemplateBundleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "template bundle JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for WorkspaceTemplateBundleError {}

/// Parses and validates an alpha workspace-template bundle JSON payload.
pub fn project_workspace_template_bundle(
    payload: &str,
) -> Result<WorkspaceTemplateBundleProjection, WorkspaceTemplateBundleError> {
    let record: WorkspaceTemplateBundleRecord = serde_json::from_str(payload)
        .map_err(|err| WorkspaceTemplateBundleError::Json(err.to_string()))?;
    record
        .validate()
        .map_err(WorkspaceTemplateBundleError::Validation)?;
    Ok(record.project())
}

fn validate_record(
    record: &WorkspaceTemplateBundleRecord,
) -> Result<(), WorkspaceTemplateBundleValidationError> {
    require_equal(
        "record_kind",
        WORKSPACE_TEMPLATE_BUNDLE_ALPHA_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != WORKSPACE_TEMPLATE_BUNDLE_ALPHA_SCHEMA_VERSION {
        return Err(WorkspaceTemplateBundleValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, WORKSPACE_TEMPLATE_BUNDLE_ALPHA_SCHEMA_VERSION
        )));
    }
    require_non_empty("bundle_id", &record.bundle_id)?;
    require_non_empty(
        "bound_template_manifest_ref",
        &record.bound_template_manifest_ref,
    )?;
    require_non_empty("bound_template_id", &record.bound_template_id)?;
    require_non_empty("bound_template_version", &record.bound_template_version)?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;

    require_one_of(
        "source_review.source_class",
        TEMPLATE_BUNDLE_SOURCE_CLASSES,
        &record.source_review.source_class,
    )?;
    require_non_empty(
        "source_review.source_distribution_class",
        &record.source_review.source_distribution_class,
    )?;
    require_non_empty(
        "source_review.signature_state",
        &record.source_review.signature_state,
    )?;
    require_non_empty(
        "source_review.publisher_label",
        &record.source_review.publisher_label,
    )?;
    require_non_empty(
        "source_review.trust_root_ref",
        &record.source_review.trust_root_ref,
    )?;

    require_one_of(
        "support_review.support_class",
        TEMPLATE_BUNDLE_SUPPORT_CLASSES,
        &record.support_review.support_class,
    )?;
    require_non_empty(
        "support_review.lifecycle_class",
        &record.support_review.lifecycle_class,
    )?;

    require_one_of(
        "target_runtime_review.runtime_scope_class",
        TEMPLATE_BUNDLE_RUNTIME_SCOPE_CLASSES,
        &record.target_runtime_review.runtime_scope_class,
    )?;
    require_one_of(
        "target_runtime_review.host_boundary_class",
        TEMPLATE_BUNDLE_HOST_BOUNDARY_CLASSES,
        &record.target_runtime_review.host_boundary_class,
    )?;
    if record.target_runtime_review.supported_ecosystems.is_empty() {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "target_runtime_review.supported_ecosystems must list at least one ecosystem class",
        ));
    }
    if record.target_runtime_review.supported_platforms.is_empty() {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "target_runtime_review.supported_platforms must list at least one platform class",
        ));
    }
    require_unique(
        "target_runtime_review.supported_ecosystems",
        &record.target_runtime_review.supported_ecosystems,
    )?;
    require_unique(
        "target_runtime_review.supported_platforms",
        &record.target_runtime_review.supported_platforms,
    )?;

    require_one_of(
        "side_effect_review.required_network_egress_class",
        TEMPLATE_BUNDLE_NETWORK_EGRESS_CLASSES,
        &record.side_effect_review.required_network_egress_class,
    )?;
    require_one_of(
        "side_effect_review.required_extension_install_class",
        TEMPLATE_BUNDLE_EXTENSION_INSTALL_CLASSES,
        &record.side_effect_review.required_extension_install_class,
    )?;
    require_one_of(
        "side_effect_review.required_remote_provisioning_class",
        TEMPLATE_BUNDLE_REMOTE_PROVISIONING_CLASSES,
        &record.side_effect_review.required_remote_provisioning_class,
    )?;
    require_one_of(
        "side_effect_review.required_managed_service_class",
        TEMPLATE_BUNDLE_MANAGED_SERVICE_CLASSES,
        &record.side_effect_review.required_managed_service_class,
    )?;
    require_one_of(
        "side_effect_review.required_credential_provisioning_class",
        TEMPLATE_BUNDLE_CREDENTIAL_PROVISIONING_CLASSES,
        &record
            .side_effect_review
            .required_credential_provisioning_class,
    )?;

    if record
        .bypass_review
        .open_without_starter_route_ids
        .is_empty()
    {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "bypass_review.open_without_starter_route_ids must list at least one route",
        ));
    }
    require_unique(
        "bypass_review.open_without_starter_route_ids",
        &record.bypass_review.open_without_starter_route_ids,
    )?;
    for route in &record.bypass_review.open_without_starter_route_ids {
        require_one_of(
            "bypass_review.open_without_starter_route_ids[]",
            TEMPLATE_BUNDLE_BYPASS_ROUTE_IDS,
            route,
        )?;
    }
    if record.bypass_review.bypass_continuity_class
        != TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT
    {
        return Err(WorkspaceTemplateBundleValidationError::new(format!(
            "bypass_review.bypass_continuity_class is {}, expected {}",
            record.bypass_review.bypass_continuity_class,
            TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT
        )));
    }

    if record.consumer_surfaces.is_empty() {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "consumer_surfaces must list at least one consumer surface",
        ));
    }
    require_unique("consumer_surfaces", &record.consumer_surfaces)?;
    for surface in &record.consumer_surfaces {
        require_one_of(
            "consumer_surfaces[]",
            TEMPLATE_BUNDLE_CONSUMER_SURFACES,
            surface,
        )?;
    }
    if !record.consumer_surfaces.iter().any(|s| s == "start_center") {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "consumer_surfaces must include start_center so the first product surface stays wired",
        ));
    }

    if record.support_export.raw_secret_export_allowed
        || record.support_export.raw_command_export_allowed
        || record.support_export.raw_url_export_allowed
    {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "support_export must keep raw_*_export_allowed false",
        ));
    }
    require_non_empty(
        "support_export.redaction_class",
        &record.support_export.redaction_class,
    )?;

    if !record.review_invariants.reviewed_before_use
        || !record.review_invariants.inspectable_before_execution
        || !record.review_invariants.no_writes_before_review
    {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "review_invariants must be true; the bundle is a pre-execution review record",
        ));
    }

    if matches!(
        record.source_review.source_class.as_str(),
        "community" | "uncertified"
    ) && record.trust_review.trust_notes.is_empty()
    {
        return Err(WorkspaceTemplateBundleValidationError::new(
            "community / uncertified source_class must carry at least one trust note",
        ));
    }

    cross_check_runtime_and_side_effects(record)?;
    Ok(())
}

fn cross_check_runtime_and_side_effects(
    record: &WorkspaceTemplateBundleRecord,
) -> Result<(), WorkspaceTemplateBundleValidationError> {
    let runtime = record.target_runtime_review.runtime_scope_class.as_str();
    let host = record.target_runtime_review.host_boundary_class.as_str();
    let remote = record
        .side_effect_review
        .required_remote_provisioning_class
        .as_str();
    let managed = record
        .side_effect_review
        .required_managed_service_class
        .as_str();
    let egress = record
        .side_effect_review
        .required_network_egress_class
        .as_str();

    match runtime {
        "local_only" => {
            if !matches!(
                host,
                "host_local_device_only" | "host_boundary_unknown_requires_review"
            ) {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_only runtime must pair with host_local_device_only host boundary",
                ));
            }
            if !matches!(
                remote,
                "no_remote_provisioning_required" | "remote_provisioning_unknown_requires_review"
            ) {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_only runtime cannot require remote provisioning",
                ));
            }
            if managed != "no_managed_service_required"
                && managed != "managed_service_class_unknown_requires_review"
            {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_only runtime cannot require a managed service",
                ));
            }
        }
        "local_with_devcontainer" => {
            if host != "host_local_with_devcontainer_attached" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_with_devcontainer runtime must use host_local_with_devcontainer_attached",
                ));
            }
            if !matches!(
                remote,
                "devcontainer_attach_required" | "remote_provisioning_unknown_requires_review"
            ) {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_with_devcontainer runtime must require devcontainer attach",
                ));
            }
        }
        "local_with_container" => {
            if host != "host_local_with_container_attached" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_with_container runtime must use host_local_with_container_attached",
                ));
            }
            if !matches!(
                remote,
                "container_attach_required" | "remote_provisioning_unknown_requires_review"
            ) {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "local_with_container runtime must require container attach",
                ));
            }
        }
        "remote_image_required" => {
            if host != "host_remote_image_required" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "remote_image_required runtime must use host_remote_image_required",
                ));
            }
            if remote != "remote_image_required" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "remote_image_required runtime must require remote_image_required provisioning",
                ));
            }
        }
        "managed_cloud_required" => {
            if host != "host_managed_workspace_required" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "managed_cloud_required runtime must use host_managed_workspace_required",
                ));
            }
            if remote != "managed_workspace_required" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "managed_cloud_required runtime must require managed_workspace_required provisioning",
                ));
            }
            if managed == "no_managed_service_required" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "managed_cloud_required runtime must declare a managed-service class",
                ));
            }
            if egress == "no_network_egress_required" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "managed_cloud_required runtime must declare network egress",
                ));
            }
        }
        "mixed_local_and_remote" => {
            if host != "host_mixed_local_and_remote" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "mixed_local_and_remote runtime must use host_mixed_local_and_remote",
                ));
            }
        }
        "not_declared" => {
            if host != "host_boundary_unknown_requires_review" {
                return Err(WorkspaceTemplateBundleValidationError::new(
                    "not_declared runtime must use host_boundary_unknown_requires_review",
                ));
            }
        }
        other => {
            return Err(WorkspaceTemplateBundleValidationError::new(format!(
                "unsupported runtime_scope_class {other}"
            )));
        }
    }
    Ok(())
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), WorkspaceTemplateBundleValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(WorkspaceTemplateBundleValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_non_empty(
    label: &str,
    value: &str,
) -> Result<(), WorkspaceTemplateBundleValidationError> {
    if value.trim().is_empty() {
        Err(WorkspaceTemplateBundleValidationError::new(format!(
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
) -> Result<(), WorkspaceTemplateBundleValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(WorkspaceTemplateBundleValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), WorkspaceTemplateBundleValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(WorkspaceTemplateBundleValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE_FIRST_PARTY: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspace/m3/template_bundle/first_party_local_starter.json"
    ));

    #[test]
    fn first_party_fixture_projects() {
        let projection =
            project_workspace_template_bundle(FIXTURE_FIRST_PARTY).expect("valid bundle");
        assert_eq!(projection.source_class, "first_party");
        assert_eq!(projection.support_class, "experimental");
        assert_eq!(projection.runtime_scope_class, "local_only");
        assert!(projection
            .open_without_starter_route_ids
            .contains(&"bypass.create_empty_workspace".to_string()));
        assert_eq!(
            projection.bypass_continuity_class,
            TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT
        );
        assert!(projection
            .consumer_surfaces
            .contains(&"start_center".to_string()));
        assert!(!projection.raw_secret_export_allowed);
        assert!(!projection.raw_command_export_allowed);
        assert!(!projection.raw_url_export_allowed);
    }

    #[test]
    fn rejects_missing_bypass_route() {
        let mut record: WorkspaceTemplateBundleRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.bypass_review.open_without_starter_route_ids.clear();
        let err = record.validate().expect_err("must reject empty bypass");
        assert!(err.message().contains("open_without_starter_route_ids"));
    }

    #[test]
    fn rejects_raw_secret_export() {
        let mut record: WorkspaceTemplateBundleRecord =
            serde_json::from_str(FIXTURE_FIRST_PARTY).expect("fixture must parse");
        record.support_export.raw_secret_export_allowed = true;
        let err = record
            .validate()
            .expect_err("must reject raw secret export");
        assert!(err.message().contains("raw_"));
    }
}
