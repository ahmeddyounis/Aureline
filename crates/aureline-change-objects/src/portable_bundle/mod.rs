//! Portable bundle and shelf records for offline review and handoff.
//!
//! A [`PortableBundleRecord`] is a read-only, evidence-bearing change object.
//! It carries the refs and labels needed for offline review, browser companion
//! handoff, incident follow-up, shelf resume, and support export without
//! transporting live provider authority or secret material.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind discriminator for portable bundle records.
pub const PORTABLE_BUNDLE_RECORD_KIND: &str = "portable_change_bundle_record";

/// Frozen schema version for the portable bundle contract.
pub const PORTABLE_BUNDLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PORTABLE_BUNDLE_SCHEMA_REF: &str = "schemas/change/portable_bundle.schema.json";

/// Repo-relative path of the UX contract.
pub const PORTABLE_BUNDLE_DOC_REF: &str = "docs/ux/m3/portable_bundle_and_shelf_beta.md";

/// Repo-relative path of the canonical fixture directory.
pub const PORTABLE_BUNDLE_FIXTURE_DIR: &str = "fixtures/review/m3/portable_bundle";

/// Repo-relative path of the support-review artifact.
pub const PORTABLE_BUNDLE_SUPPORT_ARTIFACT_REF: &str =
    "artifacts/support/m3/portable_bundle_handoff_review.md";

/// Closed set of portable change-object classes.
pub const PORTABLE_BUNDLE_OBJECT_CLASSES: &[&str] = &["portable_bundle", "shelf_entry"];

/// Closed set of handoff purpose classes.
pub const PORTABLE_BUNDLE_HANDOFF_PURPOSE_CLASSES: &[&str] = &[
    "offline_review_handoff",
    "browser_companion_handoff",
    "incident_follow_up",
    "support_export",
    "review_export",
];

/// Closed set of portable bundle lifecycle states.
pub const PORTABLE_BUNDLE_STATE_CLASSES: &[&str] = &[
    "exported_read_only",
    "shelf_entry_saved",
    "imported_compare_only",
    "desktop_resume_pending_revalidation",
    "browser_companion_read_only",
    "destroyed_tombstone",
];

/// Closed set of target identity classes.
pub const PORTABLE_BUNDLE_TARGET_KIND_CLASSES: &[&str] = &[
    "local_worktree",
    "linked_worktree",
    "managed_workspace",
    "provider_review_target",
    "incident_workspace",
    "target_kind_unknown_requires_review",
];

/// Closed set of diff reference classes.
pub const PORTABLE_BUNDLE_DIFF_REF_CLASSES: &[&str] = &[
    "patch_ref",
    "structured_diff_ref",
    "review_workspace_diff_ref",
    "evidence_only_diff_ref",
    "pathset_manifest_ref",
];

/// Closed set of evidence reference classes.
pub const PORTABLE_BUNDLE_EVIDENCE_REF_CLASSES: &[&str] = &[
    "review_pack_result",
    "ai_evidence_packet",
    "incident_packet",
    "support_export_manifest",
    "validation_result",
    "browser_handoff_record",
    "mutation_journal_slice",
    "runbook_packet",
    "provider_status_snapshot",
];

/// Closed set of review-pack parity classes.
pub const PORTABLE_BUNDLE_REVIEW_PACK_PARITY_CLASSES: &[&str] = &[
    "review_pack_current",
    "review_pack_stale",
    "review_pack_unavailable_offline",
    "review_pack_unknown_requires_review",
];

/// Closed set of validation freshness classes.
pub const PORTABLE_BUNDLE_VALIDATION_FRESHNESS_CLASSES: &[&str] = &[
    "validation_current",
    "stale_base_changed",
    "stale_worktree_scope_changed",
    "stale_review_pack_version_changed",
    "stale_environment_capsule_changed",
    "provider_overlay_unavailable_local_continues",
    "imported_compare_only_validation",
];

/// Closed set of stale-validation labels.
pub const PORTABLE_BUNDLE_STALE_VALIDATION_LABELS: &[&str] = &[
    "base_revision_changed",
    "worktree_scope_changed",
    "review_pack_version_changed",
    "environment_capsule_changed",
    "provider_overlay_unavailable",
    "evidence_snapshot_stale",
    "imported_bundle_not_live",
];

/// Closed set of portable authority classes.
pub const PORTABLE_BUNDLE_AUTHORITY_CLASSES: &[&str] = &[
    "no_live_provider_authority",
    "imported_stale_provider_snapshot",
    "desktop_reauth_required",
    "local_resume_only",
    "authority_unknown_requires_review",
];

/// Closed set of reopen and inspection modes.
pub const PORTABLE_BUNDLE_OPEN_MODE_CLASSES: &[&str] = &[
    "inspect_offline",
    "compare_only_reopen",
    "desktop_resume_after_revalidation",
    "browser_companion_read_only",
    "support_export_inspect",
];

/// Closed set of redaction classes.
pub const PORTABLE_BUNDLE_REDACTION_CLASSES: &[&str] = &[
    "metadata_safe_default",
    "redacted_diff_summary",
    "operator_only_restricted",
    "internal_support_restricted",
];

/// Closed set of support lineage classes.
pub const PORTABLE_BUNDLE_SUPPORT_LINEAGE_CLASSES: &[&str] = &[
    "review_export_lineage",
    "support_export_lineage",
    "incident_handoff_lineage",
    "browser_handoff_lineage",
];

/// Closed set of consumer surfaces that can inspect the bundle.
pub const PORTABLE_BUNDLE_CONSUMER_SURFACES: &[&str] = &[
    "portable_bundle_inspector",
    "review_preview",
    "browser_companion",
    "cli_headless_entry",
    "support_export",
    "incident_workspace",
    "docs_review",
];

const DESTRUCTION_SEMANTICS_CLASSES: &[&str] = &[
    "retention_policy_bound",
    "destroy_on_import_request",
    "tombstone_after_destroy",
];

const FIXTURE_SOURCES: &[(&str, &str)] = &[
    (
        "fixtures/review/m3/portable_bundle/offline_review_handoff.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/portable_bundle/offline_review_handoff.json"
        )),
    ),
    (
        "fixtures/review/m3/portable_bundle/browser_companion_handoff.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/portable_bundle/browser_companion_handoff.json"
        )),
    ),
    (
        "fixtures/review/m3/portable_bundle/incident_follow_up_stale_validation.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/portable_bundle/incident_follow_up_stale_validation.json"
        )),
    ),
    (
        "fixtures/review/m3/portable_bundle/support_export_shelf_desktop_resume.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/portable_bundle/support_export_shelf_desktop_resume.json"
        )),
    ),
];

/// Target and worktree identity pinned inside a portable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleTargetIdentity {
    /// Opaque workspace identity.
    pub workspace_ref: String,
    /// Opaque repository identity.
    pub repo_ref: String,
    /// Opaque worktree identity.
    pub worktree_ref: String,
    /// Opaque base revision or dirty-tree fingerprint.
    pub base_ref: String,
    /// Opaque head revision when the bundle has one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_ref: Option<String>,
    /// Opaque review, shelf, or resume target.
    pub target_ref: String,
    /// Closed target-kind class.
    pub target_kind_class: String,
    /// Environment capsule ref used to decide validation freshness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_capsule_ref: Option<String>,
}

/// Diff ref carried by a portable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleDiffRef {
    /// Opaque diff identity.
    pub diff_ref: String,
    /// Closed diff-ref class.
    pub diff_ref_class: String,
    /// Opaque scope token for affected paths.
    pub path_scope_ref: String,
    /// Optional ref to a redacted body artifact outside this record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_ref: Option<String>,
    /// True would mean a raw diff body crossed this boundary.
    pub raw_diff_body_included: bool,
    /// True when the ref can only be reopened for compare.
    pub compare_only: bool,
}

/// Evidence ref carried by a portable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleEvidenceRef {
    /// Opaque evidence identity.
    pub evidence_ref: String,
    /// Closed evidence-ref class.
    pub evidence_ref_class: String,
    /// Freshness state copied from the owning evidence record.
    pub freshness_class: String,
    /// Redaction state copied from the owning evidence record.
    pub redaction_class: String,
}

/// Review-pack binding frozen into the portable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleReviewPackBinding {
    /// Opaque review-pack identity.
    pub review_pack_ref: String,
    /// Version label rendered to review and support surfaces.
    pub review_pack_version: String,
    /// Opaque digest ref for the evaluated pack body.
    pub review_pack_digest_ref: String,
    /// Closed parity class for the pack binding.
    pub parity_class: String,
}

/// Validation freshness state for the bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleValidationState {
    /// Closed validation freshness class.
    pub freshness_class: String,
    /// Stale-validation labels rendered on import and reopen.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub staleness_labels: Vec<String>,
    /// Time the validation snapshot was produced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validated_at: Option<String>,
    /// True when desktop resume must revalidate before mutation.
    pub revalidation_required_before_resume: bool,
}

/// Authority state proving the bundle is not a live credential container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleAuthorityState {
    /// Closed authority class.
    pub authority_class: String,
    /// Opaque provider snapshot ref when the bundle cites provider state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_snapshot_ref: Option<String>,
    /// Must be false; live bearer authority never enters bundles.
    pub live_bearer_authority_included: bool,
    /// Must be false; ambient credentials never enter bundles.
    pub ambient_credentials_included: bool,
    /// Must be false; raw secret material never enters bundles.
    pub secret_material_included: bool,
    /// True when a desktop mutation needs fresh auth after import.
    pub desktop_reauth_required_before_mutation: bool,
}

/// Redaction and destruction posture for a portable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleRedactionProfile {
    /// Closed redaction class.
    pub redaction_class: String,
    /// Opaque redaction-profile identity.
    pub profile_ref: String,
    /// Must be false; raw absolute paths never cross this boundary.
    pub raw_path_export_allowed: bool,
    /// Must be false; raw remote URLs never cross this boundary.
    pub raw_remote_url_export_allowed: bool,
    /// Must be false; raw secrets never cross this boundary.
    pub raw_secret_export_allowed: bool,
    /// Must be false; raw credentials never cross this boundary.
    pub raw_credential_export_allowed: bool,
    /// Closed destruction semantic.
    pub destruction_semantics_class: String,
    /// Opaque destruction receipt refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destruction_receipt_refs: Vec<String>,
    /// True when the importer offers a post-import destroy action.
    pub destroy_after_import_supported: bool,
}

/// Support and incident lineage carried by a portable bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleSupportExportLineage {
    /// Closed lineage class.
    pub lineage_class: String,
    /// Opaque support-export identity.
    pub support_export_ref: String,
    /// Parent export ref when the bundle derives from another packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_export_ref: Option<String>,
    /// Review workspace ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_workspace_ref: Option<String>,
    /// Incident ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incident_ref: Option<String>,
    /// Browser-handoff ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_ref: Option<String>,
    /// Mutation journal refs that establish change lineage.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutation_journal_refs: Vec<String>,
}

/// Invariants every portable bundle must claim before export or import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleReviewInvariants {
    /// The same record can be exported, imported, opened, and inspected.
    pub open_import_export_parity: bool,
    /// Target/worktree identity is pinned by opaque refs.
    pub target_identity_pinned: bool,
    /// Diff refs survive import and support export.
    pub diff_refs_preserved: bool,
    /// Evidence refs survive import and support export.
    pub evidence_refs_preserved: bool,
    /// Live provider authority is excluded.
    pub no_live_provider_authority: bool,
    /// Secret material is excluded.
    pub secrets_excluded: bool,
    /// Stale validation is visibly labeled.
    pub stale_validation_labeled: bool,
    /// Redaction posture is declared.
    pub redaction_profile_declared: bool,
}

/// Portable bundle or shelf record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleRecord {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable bundle identity.
    pub bundle_id: String,
    /// `portable_bundle` or `shelf_entry`.
    pub object_class: String,
    /// Handoff purpose class.
    pub handoff_purpose_class: String,
    /// Bundle lifecycle state.
    pub bundle_state_class: String,
    /// Reviewable display label.
    pub display_label: String,
    /// Reviewable summary.
    pub summary: String,
    /// Target and worktree identity.
    pub target_identity: PortableBundleTargetIdentity,
    /// Review-pack binding.
    pub review_pack: PortableBundleReviewPackBinding,
    /// Diff refs preserved by the bundle.
    pub diff_refs: Vec<PortableBundleDiffRef>,
    /// Evidence refs preserved by the bundle.
    pub evidence_refs: Vec<PortableBundleEvidenceRef>,
    /// Validation freshness state.
    pub validation_state: PortableBundleValidationState,
    /// Authority exclusion state.
    pub authority_state: PortableBundleAuthorityState,
    /// Reopen and inspection modes.
    pub open_modes: Vec<String>,
    /// Redaction and destruction posture.
    pub redaction_profile: PortableBundleRedactionProfile,
    /// Support-export lineage.
    pub support_export_lineage: PortableBundleSupportExportLineage,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// Contract invariants.
    pub review_invariants: PortableBundleReviewInvariants,
    /// Mint time of the bundle record.
    pub minted_at: String,
}

/// Compact row consumed by shell, review, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableBundleProjection {
    /// Stable bundle identity.
    pub bundle_id: String,
    /// Portable object class.
    pub object_class: String,
    /// Handoff purpose class.
    pub handoff_purpose_class: String,
    /// Bundle lifecycle state.
    pub bundle_state_class: String,
    /// Reviewable display label.
    pub display_label: String,
    /// Reviewable summary.
    pub summary: String,
    /// Opaque worktree identity.
    pub worktree_ref: String,
    /// Opaque target identity.
    pub target_ref: String,
    /// Closed target kind.
    pub target_kind_class: String,
    /// Review-pack version label.
    pub review_pack_version: String,
    /// Review-pack parity class.
    pub review_pack_parity_class: String,
    /// Count of diff refs.
    pub diff_ref_count: usize,
    /// Count of evidence refs.
    pub evidence_ref_count: usize,
    /// Validation freshness class.
    pub validation_freshness_class: String,
    /// Stale-validation labels.
    pub staleness_labels: Vec<String>,
    /// True when compare-only reopen is available.
    pub compare_only_reopen_available: bool,
    /// True when desktop resume is available after revalidation.
    pub desktop_resume_available: bool,
    /// True when browser companion read-only inspection is available.
    pub browser_companion_read_only_available: bool,
    /// Authority class.
    pub authority_class: String,
    /// True when the bundle excludes live bearer authority.
    pub no_live_provider_authority: bool,
    /// Redaction class.
    pub redaction_class: String,
    /// Support-export lineage class.
    pub support_lineage_class: String,
    /// Support-export ref.
    pub support_export_ref: String,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
    /// True when raw paths are blocked.
    pub raw_path_export_allowed: bool,
    /// True when raw remote URLs are blocked.
    pub raw_remote_url_export_allowed: bool,
    /// True when raw secrets are blocked.
    pub raw_secret_export_allowed: bool,
    /// True when raw credentials are blocked.
    pub raw_credential_export_allowed: bool,
}

impl PortableBundleRecord {
    /// Validates this record against the portable bundle contract.
    ///
    /// # Errors
    ///
    /// Returns [`PortableBundleValidationError`] when a frozen invariant is
    /// violated.
    pub fn validate(&self) -> Result<(), PortableBundleValidationError> {
        validate_record(self)
    }

    /// Projects this record into a compact portable-bundle row.
    pub fn project(&self) -> PortableBundleProjection {
        PortableBundleProjection {
            bundle_id: self.bundle_id.clone(),
            object_class: self.object_class.clone(),
            handoff_purpose_class: self.handoff_purpose_class.clone(),
            bundle_state_class: self.bundle_state_class.clone(),
            display_label: self.display_label.clone(),
            summary: self.summary.clone(),
            worktree_ref: self.target_identity.worktree_ref.clone(),
            target_ref: self.target_identity.target_ref.clone(),
            target_kind_class: self.target_identity.target_kind_class.clone(),
            review_pack_version: self.review_pack.review_pack_version.clone(),
            review_pack_parity_class: self.review_pack.parity_class.clone(),
            diff_ref_count: self.diff_refs.len(),
            evidence_ref_count: self.evidence_refs.len(),
            validation_freshness_class: self.validation_state.freshness_class.clone(),
            staleness_labels: self.validation_state.staleness_labels.clone(),
            compare_only_reopen_available: self
                .open_modes
                .iter()
                .any(|mode| mode == "compare_only_reopen"),
            desktop_resume_available: self
                .open_modes
                .iter()
                .any(|mode| mode == "desktop_resume_after_revalidation"),
            browser_companion_read_only_available: self
                .open_modes
                .iter()
                .any(|mode| mode == "browser_companion_read_only"),
            authority_class: self.authority_state.authority_class.clone(),
            no_live_provider_authority: !self.authority_state.live_bearer_authority_included
                && !self.authority_state.ambient_credentials_included
                && !self.authority_state.secret_material_included,
            redaction_class: self.redaction_profile.redaction_class.clone(),
            support_lineage_class: self.support_export_lineage.lineage_class.clone(),
            support_export_ref: self.support_export_lineage.support_export_ref.clone(),
            consumer_surfaces: self.consumer_surfaces.clone(),
            raw_path_export_allowed: self.redaction_profile.raw_path_export_allowed,
            raw_remote_url_export_allowed: self.redaction_profile.raw_remote_url_export_allowed,
            raw_secret_export_allowed: self.redaction_profile.raw_secret_export_allowed,
            raw_credential_export_allowed: self.redaction_profile.raw_credential_export_allowed,
        }
    }
}

/// Validation failure for a portable bundle record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableBundleValidationError {
    message: String,
}

impl PortableBundleValidationError {
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

impl fmt::Display for PortableBundleValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "portable bundle validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for PortableBundleValidationError {}

/// Error returned when a portable bundle JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortableBundleError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the portable bundle contract.
    Validation(PortableBundleValidationError),
}

impl PortableBundleError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for PortableBundleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "portable bundle JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PortableBundleError {}

/// Parses and validates a portable bundle JSON payload.
///
/// # Errors
///
/// Returns [`PortableBundleError`] when the payload is not valid JSON or the
/// decoded record violates the portable bundle contract.
pub fn project_portable_bundle(
    payload: &str,
) -> Result<PortableBundleProjection, PortableBundleError> {
    let record: PortableBundleRecord =
        serde_json::from_str(payload).map_err(|err| PortableBundleError::Json(err.to_string()))?;
    record.validate().map_err(PortableBundleError::Validation)?;
    Ok(record.project())
}

/// Projects every checked-in portable bundle fixture.
///
/// # Errors
///
/// Returns [`PortableBundleError`] when any fixture fails to parse or validate.
pub fn current_portable_bundle_fixture_projections(
) -> Result<Vec<PortableBundleProjection>, PortableBundleError> {
    let mut projections = Vec::with_capacity(FIXTURE_SOURCES.len());
    for (_, payload) in FIXTURE_SOURCES {
        projections.push(project_portable_bundle(payload)?);
    }
    Ok(projections)
}

fn validate_record(record: &PortableBundleRecord) -> Result<(), PortableBundleValidationError> {
    require_equal(
        "record_kind",
        PORTABLE_BUNDLE_RECORD_KIND,
        &record.record_kind,
    )?;
    if record.schema_version != PORTABLE_BUNDLE_SCHEMA_VERSION {
        return Err(PortableBundleValidationError::new(format!(
            "schema_version is {}, expected {}",
            record.schema_version, PORTABLE_BUNDLE_SCHEMA_VERSION
        )));
    }
    require_non_empty("bundle_id", &record.bundle_id)?;
    require_one_of(
        "object_class",
        PORTABLE_BUNDLE_OBJECT_CLASSES,
        &record.object_class,
    )?;
    require_one_of(
        "handoff_purpose_class",
        PORTABLE_BUNDLE_HANDOFF_PURPOSE_CLASSES,
        &record.handoff_purpose_class,
    )?;
    require_one_of(
        "bundle_state_class",
        PORTABLE_BUNDLE_STATE_CLASSES,
        &record.bundle_state_class,
    )?;
    require_non_empty("display_label", &record.display_label)?;
    require_non_empty("summary", &record.summary)?;
    require_non_empty("minted_at", &record.minted_at)?;
    validate_target_identity(&record.target_identity)?;
    validate_review_pack(&record.review_pack)?;
    validate_diff_refs(&record.diff_refs)?;
    validate_evidence_refs(&record.evidence_refs)?;
    validate_validation_state(&record.validation_state, &record.open_modes)?;
    validate_authority_state(&record.authority_state)?;
    validate_open_modes(&record.open_modes)?;
    validate_redaction_profile(&record.redaction_profile)?;
    validate_support_lineage(&record.support_export_lineage)?;
    validate_consumer_surfaces(&record.consumer_surfaces)?;
    validate_review_invariants(&record.review_invariants)?;
    cross_check_record(record)?;
    Ok(())
}

fn validate_target_identity(
    target: &PortableBundleTargetIdentity,
) -> Result<(), PortableBundleValidationError> {
    require_non_empty("target_identity.workspace_ref", &target.workspace_ref)?;
    require_non_empty("target_identity.repo_ref", &target.repo_ref)?;
    require_non_empty("target_identity.worktree_ref", &target.worktree_ref)?;
    require_non_empty("target_identity.base_ref", &target.base_ref)?;
    if let Some(head_ref) = &target.head_ref {
        require_non_empty("target_identity.head_ref", head_ref)?;
    }
    require_non_empty("target_identity.target_ref", &target.target_ref)?;
    require_one_of(
        "target_identity.target_kind_class",
        PORTABLE_BUNDLE_TARGET_KIND_CLASSES,
        &target.target_kind_class,
    )?;
    if let Some(environment_capsule_ref) = &target.environment_capsule_ref {
        require_non_empty(
            "target_identity.environment_capsule_ref",
            environment_capsule_ref,
        )?;
    }
    Ok(())
}

fn validate_review_pack(
    review_pack: &PortableBundleReviewPackBinding,
) -> Result<(), PortableBundleValidationError> {
    require_non_empty("review_pack.review_pack_ref", &review_pack.review_pack_ref)?;
    require_non_empty(
        "review_pack.review_pack_version",
        &review_pack.review_pack_version,
    )?;
    require_non_empty(
        "review_pack.review_pack_digest_ref",
        &review_pack.review_pack_digest_ref,
    )?;
    require_one_of(
        "review_pack.parity_class",
        PORTABLE_BUNDLE_REVIEW_PACK_PARITY_CLASSES,
        &review_pack.parity_class,
    )
}

fn validate_diff_refs(
    diff_refs: &[PortableBundleDiffRef],
) -> Result<(), PortableBundleValidationError> {
    if diff_refs.is_empty() {
        return Err(PortableBundleValidationError::new(
            "diff_refs must contain at least one diff ref",
        ));
    }
    let mut seen = BTreeSet::new();
    for diff_ref in diff_refs {
        require_non_empty("diff_refs[].diff_ref", &diff_ref.diff_ref)?;
        if !seen.insert(diff_ref.diff_ref.as_str()) {
            return Err(PortableBundleValidationError::new(format!(
                "diff_refs contains a duplicate diff_ref: {}",
                diff_ref.diff_ref
            )));
        }
        require_one_of(
            "diff_refs[].diff_ref_class",
            PORTABLE_BUNDLE_DIFF_REF_CLASSES,
            &diff_ref.diff_ref_class,
        )?;
        require_non_empty("diff_refs[].path_scope_ref", &diff_ref.path_scope_ref)?;
        if let Some(body_ref) = &diff_ref.body_ref {
            require_non_empty("diff_refs[].body_ref", body_ref)?;
        }
        if diff_ref.raw_diff_body_included {
            return Err(PortableBundleValidationError::new(
                "diff_refs[].raw_diff_body_included must be false",
            ));
        }
    }
    Ok(())
}

fn validate_evidence_refs(
    evidence_refs: &[PortableBundleEvidenceRef],
) -> Result<(), PortableBundleValidationError> {
    if evidence_refs.is_empty() {
        return Err(PortableBundleValidationError::new(
            "evidence_refs must contain at least one evidence ref",
        ));
    }
    let mut seen = BTreeSet::new();
    for evidence_ref in evidence_refs {
        require_non_empty("evidence_refs[].evidence_ref", &evidence_ref.evidence_ref)?;
        if !seen.insert(evidence_ref.evidence_ref.as_str()) {
            return Err(PortableBundleValidationError::new(format!(
                "evidence_refs contains a duplicate evidence_ref: {}",
                evidence_ref.evidence_ref
            )));
        }
        require_one_of(
            "evidence_refs[].evidence_ref_class",
            PORTABLE_BUNDLE_EVIDENCE_REF_CLASSES,
            &evidence_ref.evidence_ref_class,
        )?;
        require_one_of(
            "evidence_refs[].freshness_class",
            PORTABLE_BUNDLE_VALIDATION_FRESHNESS_CLASSES,
            &evidence_ref.freshness_class,
        )?;
        require_one_of(
            "evidence_refs[].redaction_class",
            PORTABLE_BUNDLE_REDACTION_CLASSES,
            &evidence_ref.redaction_class,
        )?;
    }
    Ok(())
}

fn validate_validation_state(
    validation: &PortableBundleValidationState,
    open_modes: &[String],
) -> Result<(), PortableBundleValidationError> {
    require_one_of(
        "validation_state.freshness_class",
        PORTABLE_BUNDLE_VALIDATION_FRESHNESS_CLASSES,
        &validation.freshness_class,
    )?;
    validate_unique_closed_values(
        "validation_state.staleness_labels",
        PORTABLE_BUNDLE_STALE_VALIDATION_LABELS,
        &validation.staleness_labels,
    )?;
    if validation.freshness_class != "validation_current" {
        if validation.staleness_labels.is_empty() {
            return Err(PortableBundleValidationError::new(
                "stale validation states must carry at least one staleness label",
            ));
        }
        if !contains(open_modes, "compare_only_reopen") {
            return Err(PortableBundleValidationError::new(
                "stale validation states must allow compare_only_reopen",
            ));
        }
    }
    Ok(())
}

fn validate_authority_state(
    authority: &PortableBundleAuthorityState,
) -> Result<(), PortableBundleValidationError> {
    require_one_of(
        "authority_state.authority_class",
        PORTABLE_BUNDLE_AUTHORITY_CLASSES,
        &authority.authority_class,
    )?;
    if let Some(provider_snapshot_ref) = &authority.provider_snapshot_ref {
        require_non_empty(
            "authority_state.provider_snapshot_ref",
            provider_snapshot_ref,
        )?;
    }
    if authority.live_bearer_authority_included
        || authority.ambient_credentials_included
        || authority.secret_material_included
    {
        return Err(PortableBundleValidationError::new(
            "portable bundles must not include live bearer authority, ambient credentials, or secret material",
        ));
    }
    Ok(())
}

fn validate_open_modes(open_modes: &[String]) -> Result<(), PortableBundleValidationError> {
    if open_modes.is_empty() {
        return Err(PortableBundleValidationError::new(
            "open_modes must contain at least one mode",
        ));
    }
    validate_unique_closed_values("open_modes", PORTABLE_BUNDLE_OPEN_MODE_CLASSES, open_modes)?;
    if !contains(open_modes, "inspect_offline") {
        return Err(PortableBundleValidationError::new(
            "open_modes must include inspect_offline",
        ));
    }
    Ok(())
}

fn validate_redaction_profile(
    redaction: &PortableBundleRedactionProfile,
) -> Result<(), PortableBundleValidationError> {
    require_one_of(
        "redaction_profile.redaction_class",
        PORTABLE_BUNDLE_REDACTION_CLASSES,
        &redaction.redaction_class,
    )?;
    require_non_empty("redaction_profile.profile_ref", &redaction.profile_ref)?;
    require_one_of(
        "redaction_profile.destruction_semantics_class",
        DESTRUCTION_SEMANTICS_CLASSES,
        &redaction.destruction_semantics_class,
    )?;
    if redaction.raw_path_export_allowed
        || redaction.raw_remote_url_export_allowed
        || redaction.raw_secret_export_allowed
        || redaction.raw_credential_export_allowed
    {
        return Err(PortableBundleValidationError::new(
            "redaction_profile raw path, remote URL, secret, and credential export must be false",
        ));
    }
    validate_unique_non_empty(
        "redaction_profile.destruction_receipt_refs",
        &redaction.destruction_receipt_refs,
    )
}

fn validate_support_lineage(
    lineage: &PortableBundleSupportExportLineage,
) -> Result<(), PortableBundleValidationError> {
    require_one_of(
        "support_export_lineage.lineage_class",
        PORTABLE_BUNDLE_SUPPORT_LINEAGE_CLASSES,
        &lineage.lineage_class,
    )?;
    require_non_empty(
        "support_export_lineage.support_export_ref",
        &lineage.support_export_ref,
    )?;
    if let Some(parent_export_ref) = &lineage.parent_export_ref {
        require_non_empty(
            "support_export_lineage.parent_export_ref",
            parent_export_ref,
        )?;
    }
    if let Some(review_workspace_ref) = &lineage.review_workspace_ref {
        require_non_empty(
            "support_export_lineage.review_workspace_ref",
            review_workspace_ref,
        )?;
    }
    if let Some(incident_ref) = &lineage.incident_ref {
        require_non_empty("support_export_lineage.incident_ref", incident_ref)?;
    }
    if let Some(browser_handoff_ref) = &lineage.browser_handoff_ref {
        require_non_empty(
            "support_export_lineage.browser_handoff_ref",
            browser_handoff_ref,
        )?;
    }
    validate_unique_non_empty(
        "support_export_lineage.mutation_journal_refs",
        &lineage.mutation_journal_refs,
    )
}

fn validate_consumer_surfaces(
    consumer_surfaces: &[String],
) -> Result<(), PortableBundleValidationError> {
    if consumer_surfaces.is_empty() {
        return Err(PortableBundleValidationError::new(
            "consumer_surfaces must contain at least one surface",
        ));
    }
    validate_unique_closed_values(
        "consumer_surfaces",
        PORTABLE_BUNDLE_CONSUMER_SURFACES,
        consumer_surfaces,
    )?;
    if !contains(consumer_surfaces, "portable_bundle_inspector") {
        return Err(PortableBundleValidationError::new(
            "consumer_surfaces must include portable_bundle_inspector",
        ));
    }
    if !contains(consumer_surfaces, "support_export") {
        return Err(PortableBundleValidationError::new(
            "consumer_surfaces must include support_export",
        ));
    }
    Ok(())
}

fn validate_review_invariants(
    invariants: &PortableBundleReviewInvariants,
) -> Result<(), PortableBundleValidationError> {
    let checks = [
        (
            "open_import_export_parity",
            invariants.open_import_export_parity,
        ),
        ("target_identity_pinned", invariants.target_identity_pinned),
        ("diff_refs_preserved", invariants.diff_refs_preserved),
        (
            "evidence_refs_preserved",
            invariants.evidence_refs_preserved,
        ),
        (
            "no_live_provider_authority",
            invariants.no_live_provider_authority,
        ),
        ("secrets_excluded", invariants.secrets_excluded),
        (
            "stale_validation_labeled",
            invariants.stale_validation_labeled,
        ),
        (
            "redaction_profile_declared",
            invariants.redaction_profile_declared,
        ),
    ];
    for (name, value) in checks {
        if !value {
            return Err(PortableBundleValidationError::new(format!(
                "review_invariants.{name} must be true"
            )));
        }
    }
    Ok(())
}

fn cross_check_record(record: &PortableBundleRecord) -> Result<(), PortableBundleValidationError> {
    if record.object_class == "shelf_entry"
        && !matches!(
            record.bundle_state_class.as_str(),
            "shelf_entry_saved" | "desktop_resume_pending_revalidation" | "destroyed_tombstone"
        )
    {
        return Err(PortableBundleValidationError::new(
            "shelf_entry must use a shelf or desktop-resume bundle_state_class",
        ));
    }
    if record.bundle_state_class == "browser_companion_read_only"
        && !contains(&record.open_modes, "browser_companion_read_only")
    {
        return Err(PortableBundleValidationError::new(
            "browser companion bundles must include browser_companion_read_only open mode",
        ));
    }
    if record.bundle_state_class == "desktop_resume_pending_revalidation" {
        if !contains(&record.open_modes, "desktop_resume_after_revalidation") {
            return Err(PortableBundleValidationError::new(
                "desktop resume bundles must include desktop_resume_after_revalidation",
            ));
        }
        if !record.validation_state.revalidation_required_before_resume {
            return Err(PortableBundleValidationError::new(
                "desktop resume bundles must require revalidation before resume",
            ));
        }
    }
    if record.authority_state.authority_class == "imported_stale_provider_snapshot"
        && record.authority_state.provider_snapshot_ref.is_none()
    {
        return Err(PortableBundleValidationError::new(
            "imported stale provider snapshots must carry provider_snapshot_ref",
        ));
    }
    Ok(())
}

fn require_equal(
    field: &str,
    expected: &str,
    actual: &str,
) -> Result<(), PortableBundleValidationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(PortableBundleValidationError::new(format!(
            "{field} is {actual:?}, expected {expected:?}"
        )))
    }
}

fn require_non_empty(field: &str, value: &str) -> Result<(), PortableBundleValidationError> {
    if value.trim().is_empty() {
        Err(PortableBundleValidationError::new(format!(
            "{field} must be non-empty"
        )))
    } else {
        Ok(())
    }
}

fn require_one_of(
    field: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), PortableBundleValidationError> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(PortableBundleValidationError::new(format!(
            "{field} has unsupported value {value:?}"
        )))
    }
}

fn validate_unique_closed_values(
    field: &str,
    allowed: &[&str],
    values: &[String],
) -> Result<(), PortableBundleValidationError> {
    let mut seen = BTreeSet::new();
    for value in values {
        require_one_of(field, allowed, value)?;
        if !seen.insert(value.as_str()) {
            return Err(PortableBundleValidationError::new(format!(
                "{field} contains duplicate value {value:?}"
            )));
        }
    }
    Ok(())
}

fn validate_unique_non_empty(
    field: &str,
    values: &[String],
) -> Result<(), PortableBundleValidationError> {
    let mut seen = BTreeSet::new();
    for value in values {
        require_non_empty(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(PortableBundleValidationError::new(format!(
                "{field} contains duplicate value {value:?}"
            )));
        }
    }
    Ok(())
}

fn contains(values: &[String], needle: &str) -> bool {
    values.iter().any(|value| value == needle)
}
