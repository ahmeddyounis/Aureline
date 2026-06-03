//! Finalize workflow-bundle lifecycle with install/update/remove diff review,
//! drift detection, local overrides, and certified bundle truth.
//!
//! This module sits above the workflow-bundle review packet (see [`crate::bundles`])
//! and the bundle-archetype certification packet (see
//! [`crate::certify_launch_bundles_imported_user_handoff_bundles_and`]). It
//! produces a [`BundleLifecycleFinalizationRecord`] that every consumer surface
//! — Start Center, CLI/headless install, diagnostics, support export, docs, and
//! mirror-first/offline lanes — must read verbatim rather than cloning status
//! text.
//!
//! The record guarantees:
//!
//! - **Lifecycle operation truth.** Every install, update, remove, and rebase
//!   carries the exact operation kind, the reviewed diff, and the rollback
//!   checkpoint before any mutation commits.
//! - **Dependency marker honesty.** Preview/Beta learning surfaces, managed
//!   seat/entitlement posture, org-mirrored content, and Labs-only capabilities
//!   are surfaced explicitly instead of being implied by a generic certified
//!   badge.
//! - **Drift and override visibility.** Field-level, package-level, and
//!   task-level drift rows preserve scorecard linkage, local override refs, and
//!   the Keep local / Adopt bundle / Compare / Rebase choices.
//! - **Certification truth that survives export.** The effective badge, claim
//!   class, scorecard row ref, reference workspace, and compatibility range are
//!   all recorded so support exports and mirror-first installs reconstruct the
//!   same truth.
//! - **Asset ownership on removal.** Created assets, adopted assets, and user
//!   overlays are classified distinctly so removal never silently deletes
//!   user-owned files.
//! - **Trust/egress/control-plane change disclosure.** When a bundle update
//!   would alter workspace trust, network egress, policy scope, or
//!   managed-control-plane reliance, the change is surfaced as a
//!   [`TrustEgressChangeDisclosure`] rather than ordinary package churn.
//!
//! The companion schema lives at
//! `schemas/review/finalize-workflow-bundle-lifecycle-drift-and-overrides.schema.json`.
//! Canonical fixtures live under
//! `fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/`.

use std::fmt;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version for every bundle-lifecycle finalization record.
pub const BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for [`BundleLifecycleFinalizationRecord`].
pub const BUNDLE_LIFECYCLE_FINALIZATION_RECORD_KIND: &str = "bundle_lifecycle_finalization_record";

/// Record-kind tag for [`BundleLifecycleOperationRecord`].
pub const BUNDLE_LIFECYCLE_OPERATION_RECORD_KIND: &str = "bundle_lifecycle_operation_record";

/// Record-kind tag for [`BundleDependencyMarkerRecord`].
pub const BUNDLE_DEPENDENCY_MARKER_RECORD_KIND: &str = "bundle_dependency_marker_record";

/// Record-kind tag for [`ScorecardLinkedDriftSummaryRecord`].
pub const SCORECARD_LINKED_DRIFT_SUMMARY_RECORD_KIND: &str =
    "scorecard_linked_drift_summary_record";

/// Record-kind tag for [`TrustEgressChangeDisclosureRecord`].
pub const TRUST_EGRESS_CHANGE_DISCLOSURE_RECORD_KIND: &str =
    "trust_egress_change_disclosure_record";

/// Record-kind tag for [`BundleLifecycleInspectionRecord`].
pub const BUNDLE_LIFECYCLE_INSPECTION_RECORD_KIND: &str = "bundle_lifecycle_inspection_record";

/// Closed set of lifecycle operations.
pub const BUNDLE_LIFECYCLE_OPERATIONS: &[&str] = &["install", "update", "remove", "rebase"];

/// Closed set of dependency capability classes.
pub const BUNDLE_DEPENDENCY_CAPABILITY_CLASSES: &[&str] = &[
    "preview_learning_surface",
    "beta_learning_surface",
    "managed_seat_entitlement",
    "org_mirrored_content",
    "labs_only_capability",
    "stable_public_capability",
];

/// Closed set of dependency narrowing classes.
pub const BUNDLE_DEPENDENCY_NARROWING_CLASSES: &[&str] = &[
    "no_narrowing",
    "narrowed_to_preview",
    "narrowed_to_beta",
    "narrowed_to_managed_only",
    "narrowed_to_org_mirror",
    "narrowed_to_labs",
    "narrowed_below_stable",
];

/// Closed set of asset provenance classes.
pub const BUNDLE_ASSET_PROVENANCE_CLASSES: &[&str] = &[
    "bundle_created",
    "bundle_adopted",
    "user_created_on_bundle",
    "user_created_independent",
    "mixed_unknown_provenance",
];

/// Closed set of trust/egress change classes.
pub const TRUST_EGRESS_CHANGE_CLASSES: &[&str] = &[
    "trust_widened",
    "trust_narrowed",
    "egress_widened",
    "egress_narrowed",
    "managed_control_plane_increased",
    "managed_control_plane_decreased",
    "stable_to_preview",
    "stable_to_beta",
    "preview_to_stable",
    "beta_to_stable",
    "no_change",
];

/// Closed set of change severity classes.
pub const CHANGE_SEVERITY_CLASSES: &[&str] = &[
    "ordinary_package_churn",
    "capability_dependency_change",
    "trust_boundary_change",
    "egress_boundary_change",
    "managed_dependence_change",
    "certification_downgrade",
    "certification_upgrade",
];

/// Closed set of consumer surfaces that ingest this record.
pub const BUNDLE_LIFECYCLE_CONSUMER_SURFACES: &[&str] = &[
    "start_center",
    "bundle_detail",
    "cli_headless",
    "diagnostics",
    "support_export",
    "docs_workspace",
    "mirror_first_install",
    "offline_archive_install",
];

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// Input describing a bundle-lifecycle finalization to materialize.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLifecycleFinalizationInput {
    /// Stable record identity.
    pub record_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Lifecycle operation input.
    pub operation: BundleLifecycleOperationInput,
    /// Dependency markers surfaced explicitly.
    pub dependency_markers: Vec<BundleDependencyMarkerInput>,
    /// Scorecard-linked drift summary input.
    pub drift_summary: ScorecardLinkedDriftSummaryInput,
    /// Trust/egress/control-plane change disclosures.
    pub trust_egress_disclosures: Vec<TrustEgressChangeDisclosureInput>,
    /// Asset provenance classifications for removal review.
    #[serde(default)]
    pub asset_provenance: Vec<BundleAssetProvenanceInput>,
    /// Consumer surfaces bound to this record.
    pub consumer_surfaces: Vec<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input for [`BundleLifecycleOperationRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLifecycleOperationInput {
    /// Operation kind from the closed vocabulary.
    pub operation_kind: String,
    /// Bundle id being operated on.
    pub bundle_id: String,
    /// Bundle revision being operated on.
    pub bundle_revision: u32,
    /// Review packet id for the underlying workflow-bundle review.
    pub review_packet_id: String,
    /// Certification packet id for the archetype certification review.
    pub certification_packet_id: String,
    /// Scorecard row id backing this operation, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scorecard_row_ref: Option<String>,
    /// Rollback checkpoint ref attached to this operation.
    pub rollback_checkpoint_ref: String,
}

/// Input for one [`BundleDependencyMarkerRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDependencyMarkerInput {
    /// Marker id.
    pub marker_id: String,
    /// Capability class from the closed vocabulary.
    pub capability_class: String,
    /// Narrowing class applied to this marker.
    pub narrowing_class: String,
    /// True when the bundle actively depends on this capability.
    pub active_dependency: bool,
    /// True when the marker must be rendered on review surfaces.
    pub disclosure_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for [`ScorecardLinkedDriftSummaryRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardLinkedDriftSummaryInput {
    /// Summary id.
    pub summary_id: String,
    /// High-level drift state.
    pub drift_state_class: String,
    /// Drift entries with scorecard linkage.
    pub drift_entries: Vec<ScorecardLinkedDriftEntryInput>,
    /// True when drift uses field/package/task granularity.
    pub field_package_task_granular: bool,
    /// Scorecard id linked to this drift summary.
    pub scorecard_id_ref: String,
    /// Scorecard row id linked to this drift summary.
    pub scorecard_row_ref: String,
}

/// Input for one scorecard-linked drift entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardLinkedDriftEntryInput {
    /// Entry id.
    pub entry_id: String,
    /// Drift axis.
    pub drift_axis: String,
    /// Subject reference.
    pub subject_ref: String,
    /// Claim narrowing caused by this drift.
    pub claim_narrowing_class: String,
    /// Preserved local override ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
    /// Scorecard linkage preserved.
    pub scorecard_linkage_preserved: bool,
}

/// Input for one [`TrustEgressChangeDisclosureRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustEgressChangeDisclosureInput {
    /// Disclosure id.
    pub disclosure_id: String,
    /// Change class from the closed vocabulary.
    pub change_class: String,
    /// Severity class for this disclosure.
    pub severity_class: String,
    /// True when the change alters trust, egress, or managed dependence.
    pub alters_authority_boundary: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Input for one [`BundleAssetProvenanceRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleAssetProvenanceInput {
    /// Asset reference.
    pub asset_ref: String,
    /// Asset kind.
    pub asset_kind: String,
    /// Provenance class from the closed vocabulary.
    pub provenance_class: String,
    /// True when the asset was created by the bundle.
    pub bundle_created: bool,
    /// True when the asset was adopted from another source.
    pub adopted: bool,
    /// True when the asset is user-owned and must survive removal.
    pub user_owned: bool,
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// Lifecycle operation truth recorded before any mutation commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLifecycleOperationRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Operation kind.
    pub operation_kind: String,
    /// Bundle id.
    pub bundle_id: String,
    /// Bundle revision.
    pub bundle_revision: u32,
    /// Review packet id.
    pub review_packet_id: String,
    /// Certification packet id.
    pub certification_packet_id: String,
    /// Scorecard row ref, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scorecard_row_ref: Option<String>,
    /// Rollback checkpoint ref.
    pub rollback_checkpoint_ref: String,
}

/// Dependency marker surfaced explicitly on every consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleDependencyMarkerRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Marker id.
    pub marker_id: String,
    /// Capability class.
    pub capability_class: String,
    /// Narrowing class.
    pub narrowing_class: String,
    /// True when the bundle actively depends on this capability.
    pub active_dependency: bool,
    /// True when the marker must be rendered.
    pub disclosure_required: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Scorecard-linked drift summary preserving certification truth through drift
/// review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardLinkedDriftSummaryRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Summary id.
    pub summary_id: String,
    /// High-level drift state class.
    pub drift_state_class: String,
    /// Drift entries with scorecard linkage.
    pub drift_entries: Vec<ScorecardLinkedDriftEntry>,
    /// True when drift uses field/package/task granularity.
    pub field_package_task_granular: bool,
    /// Scorecard id ref.
    pub scorecard_id_ref: String,
    /// Scorecard row ref.
    pub scorecard_row_ref: String,
}

/// One scorecard-linked drift entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScorecardLinkedDriftEntry {
    /// Entry id.
    pub entry_id: String,
    /// Drift axis.
    pub drift_axis: String,
    /// Subject reference.
    pub subject_ref: String,
    /// Claim narrowing class.
    pub claim_narrowing_class: String,
    /// Preserved local override ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_override_ref: Option<String>,
    /// True when scorecard linkage is preserved.
    pub scorecard_linkage_preserved: bool,
}

/// Trust/egress/control-plane change disclosure surfaced when a bundle update
/// would alter authority boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustEgressChangeDisclosureRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Disclosure id.
    pub disclosure_id: String,
    /// Change class.
    pub change_class: String,
    /// Severity class.
    pub severity_class: String,
    /// True when the change alters trust, egress, or managed dependence.
    pub alters_authority_boundary: bool,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Asset provenance classification for removal review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleAssetProvenanceRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Asset reference.
    pub asset_ref: String,
    /// Asset kind.
    pub asset_kind: String,
    /// Provenance class.
    pub provenance_class: String,
    /// True when the asset was created by the bundle.
    pub bundle_created: bool,
    /// True when the asset was adopted from another source.
    pub adopted: bool,
    /// True when the asset is user-owned.
    pub user_owned: bool,
}

/// Compact inspection row for CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLifecycleInspectionRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Record id inspected.
    pub record_id_ref: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Bundle id.
    pub bundle_id: String,
    /// Effective badge class after certification.
    pub effective_badge_class: String,
    /// True when the operation is reversible.
    pub reversible: bool,
    /// True when dependency markers are all disclosed.
    pub dependency_markers_disclosed: bool,
    /// True when drift summary preserves scorecard linkage.
    pub scorecard_linkage_preserved: bool,
    /// True when no trust/egress boundary changes are hidden.
    pub no_hidden_boundary_changes: bool,
    /// True when removal review preserves user-owned assets.
    pub removal_preserves_user_assets: bool,
    /// True when the record is support-export safe.
    pub support_export_safe: bool,
    /// Number of dependency markers.
    pub dependency_marker_count: usize,
    /// Number of drift entries.
    pub drift_entry_count: usize,
    /// Number of trust/egress disclosures.
    pub trust_egress_disclosure_count: usize,
    /// Number of asset provenance rows.
    pub asset_provenance_count: usize,
    /// Reviewable summary.
    pub summary_label: String,
}

/// Top-level bundle-lifecycle finalization record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleLifecycleFinalizationRecord {
    /// Record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable record identity.
    pub record_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Lifecycle operation truth.
    pub operation: BundleLifecycleOperationRecord,
    /// Dependency markers.
    pub dependency_markers: Vec<BundleDependencyMarkerRecord>,
    /// Scorecard-linked drift summary.
    pub drift_summary: ScorecardLinkedDriftSummaryRecord,
    /// Trust/egress/control-plane change disclosures.
    pub trust_egress_disclosures: Vec<TrustEgressChangeDisclosureRecord>,
    /// Asset provenance classifications.
    #[serde(default)]
    pub asset_provenance: Vec<BundleAssetProvenanceRecord>,
    /// Consumer surfaces bound to this record.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the record cites.
    pub source_schema_refs: Vec<String>,
    /// Inspection row.
    pub inspection: BundleLifecycleInspectionRecord,
    /// Reviewable summary.
    pub summary_label: String,
}

impl BundleLifecycleFinalizationRecord {
    /// Builds a finalization record from input, applying validation and
    /// minting the inspection row.
    ///
    /// # Errors
    ///
    /// Returns [`BundleLifecycleValidationError`] when the input violates a
    /// lifecycle invariant.
    pub fn from_input(
        input: BundleLifecycleFinalizationInput,
    ) -> Result<Self, BundleLifecycleValidationError> {
        validate_input(&input)?;

        let operation = operation_record(&input.operation);
        let dependency_markers = input
            .dependency_markers
            .iter()
            .map(dependency_marker_record)
            .collect::<Vec<_>>();
        let drift_summary = drift_summary_record(&input.drift_summary);
        let trust_egress_disclosures = input
            .trust_egress_disclosures
            .iter()
            .map(trust_egress_disclosure_record)
            .collect::<Vec<_>>();
        let asset_provenance = input
            .asset_provenance
            .iter()
            .map(asset_provenance_record)
            .collect::<Vec<_>>();

        let inspection = inspection_record(
            &input,
            &operation,
            &dependency_markers,
            &drift_summary,
            &trust_egress_disclosures,
            &asset_provenance,
        );

        let record = Self {
            record_kind: BUNDLE_LIFECYCLE_FINALIZATION_RECORD_KIND.to_string(),
            schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
            record_id: input.record_id,
            generated_at: input.generated_at,
            operation,
            dependency_markers,
            drift_summary,
            trust_egress_disclosures,
            asset_provenance,
            consumer_surfaces: input.consumer_surfaces,
            source_schema_refs: vec![BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_REF.to_string()],
            inspection,
            summary_label: input.summary_label,
        };
        record.validate()?;
        Ok(record)
    }

    /// Validates this record against the lifecycle finalization invariants.
    ///
    /// # Errors
    ///
    /// Returns [`BundleLifecycleValidationError`] when an invariant is violated.
    pub fn validate(&self) -> Result<(), BundleLifecycleValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            BUNDLE_LIFECYCLE_FINALIZATION_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq_u32(
            self.schema_version,
            BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.record_id, "record_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;

        validate_operation_record(&self.operation)?;
        for marker in &self.dependency_markers {
            validate_dependency_marker_record(marker)?;
        }
        validate_drift_summary_record(&self.drift_summary)?;
        for disclosure in &self.trust_egress_disclosures {
            validate_trust_egress_disclosure_record(disclosure)?;
        }
        for asset in &self.asset_provenance {
            validate_asset_provenance_record(asset)?;
        }

        for surface in &self.consumer_surfaces {
            ensure_token(
                BUNDLE_LIFECYCLE_CONSUMER_SURFACES,
                surface,
                "consumer_surface",
            )?;
        }
        if self.consumer_surfaces.is_empty() {
            return Err(err("record must bind at least one consumer surface"));
        }
        if !self
            .source_schema_refs
            .iter()
            .any(|r| r == BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_REF)
        {
            return Err(err("record must cite its source schema"));
        }

        // Operation-specific invariants
        if self.operation.operation_kind == "remove" {
            if self.asset_provenance.is_empty() {
                return Err(err(
                    "remove operation must carry asset_provenance classifications",
                ));
            }
            let user_owned_survives = self
                .asset_provenance
                .iter()
                .all(|asset| !asset.user_owned || asset.provenance_class != "bundle_created");
            if !user_owned_survives {
                return Err(err(
                    "remove operation must not classify user-owned assets as bundle_created",
                ));
            }
        }

        // Trust/egress disclosures must surface authority-boundary changes
        let hidden_boundary_changes = self
            .trust_egress_disclosures
            .iter()
            .any(|d| d.alters_authority_boundary && d.severity_class == "ordinary_package_churn");
        if hidden_boundary_changes {
            return Err(err(
                "trust/egress boundary changes must not be classified as ordinary_package_churn",
            ));
        }

        // Drift summary must preserve scorecard linkage
        if !self
            .drift_summary
            .drift_entries
            .iter()
            .all(|e| e.scorecard_linkage_preserved)
        {
            return Err(err("all drift entries must preserve scorecard linkage"));
        }

        // Inspection consistency
        validate_inspection_record(&self.inspection, self)?;

        Ok(())
    }

    /// Returns true when the record is support-export safe.
    pub fn is_support_export_safe(&self) -> bool {
        self.inspection.support_export_safe
            && self.schema_version == BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION
            && self.record_kind == BUNDLE_LIFECYCLE_FINALIZATION_RECORD_KIND
    }

    /// Returns true when no trust/egress boundary changes are hidden.
    pub fn no_hidden_boundary_changes(&self) -> bool {
        self.trust_egress_disclosures
            .iter()
            .all(|d| !d.alters_authority_boundary || d.severity_class != "ordinary_package_churn")
    }

    /// Returns true when removal review preserves user-owned assets.
    pub fn removal_preserves_user_assets(&self) -> bool {
        if self.operation.operation_kind != "remove" {
            return true;
        }
        self.asset_provenance
            .iter()
            .all(|asset| !asset.user_owned || asset.provenance_class != "bundle_created")
    }
}

// ---------------------------------------------------------------------------
// Schema reference
// ---------------------------------------------------------------------------

/// Canonical schema path the export cites.
pub const BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_REF: &str =
    "schemas/review/finalize-workflow-bundle-lifecycle-drift-and-overrides.schema.json";

// ---------------------------------------------------------------------------
// Projection type
// ---------------------------------------------------------------------------

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleLifecycleFinalizationProjection {
    /// Stable record identity.
    pub record_id: String,
    /// Operation kind.
    pub operation_kind: String,
    /// Bundle id.
    pub bundle_id: String,
    /// Bundle revision.
    pub bundle_revision: u32,
    /// Effective badge class.
    pub effective_badge_class: String,
    /// True when reversible.
    pub reversible: bool,
    /// Number of dependency markers.
    pub dependency_marker_count: usize,
    /// Number of drift entries.
    pub drift_entry_count: usize,
    /// Number of trust/egress disclosures.
    pub trust_egress_disclosure_count: usize,
    /// True when no hidden boundary changes.
    pub no_hidden_boundary_changes: bool,
    /// True when removal preserves user assets.
    pub removal_preserves_user_assets: bool,
    /// Consumer surfaces.
    pub consumer_surfaces: Vec<String>,
}

impl From<BundleLifecycleFinalizationRecord> for BundleLifecycleFinalizationProjection {
    fn from(record: BundleLifecycleFinalizationRecord) -> Self {
        let no_hidden_boundary_changes = record.no_hidden_boundary_changes();
        let removal_preserves_user_assets = record.removal_preserves_user_assets();
        Self {
            record_id: record.record_id,
            operation_kind: record.operation.operation_kind.clone(),
            bundle_id: record.operation.bundle_id.clone(),
            bundle_revision: record.operation.bundle_revision,
            effective_badge_class: record.inspection.effective_badge_class.clone(),
            reversible: record.inspection.reversible,
            dependency_marker_count: record.dependency_markers.len(),
            drift_entry_count: record.drift_summary.drift_entries.len(),
            trust_egress_disclosure_count: record.trust_egress_disclosures.len(),
            no_hidden_boundary_changes,
            removal_preserves_user_assets,
            consumer_surfaces: record.consumer_surfaces,
        }
    }
}

/// Parses and validates a materialized bundle-lifecycle finalization record.
///
/// # Errors
///
/// Returns [`BundleLifecycleError`] when the payload fails to parse or violates
/// the lifecycle invariants.
pub fn project_bundle_lifecycle_finalization(
    payload: &str,
) -> Result<BundleLifecycleFinalizationProjection, BundleLifecycleError> {
    let record: BundleLifecycleFinalizationRecord = serde_json::from_str(payload)?;
    record.validate()?;
    Ok(BundleLifecycleFinalizationProjection::from(record))
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Error enum for bundle-lifecycle finalization operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundleLifecycleError {
    /// Validation failed.
    Validation(BundleLifecycleValidationError),
    /// Construction failed.
    Construction(String),
}

impl fmt::Display for BundleLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "validation error: {e}"),
            Self::Construction(msg) => write!(f, "construction error: {msg}"),
        }
    }
}

impl std::error::Error for BundleLifecycleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(e) => Some(e),
            Self::Construction(_) => None,
        }
    }
}

/// Validation error for bundle-lifecycle finalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleLifecycleValidationError {
    /// Redaction-safe message.
    pub message: String,
}

impl fmt::Display for BundleLifecycleValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BundleLifecycleValidationError {}

impl BundleLifecycleValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<serde_json::Error> for BundleLifecycleError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(BundleLifecycleValidationError {
            message: err.to_string(),
        })
    }
}

impl From<BundleLifecycleValidationError> for BundleLifecycleError {
    fn from(err: BundleLifecycleValidationError) -> Self {
        Self::Validation(err)
    }
}

// ---------------------------------------------------------------------------
// Record constructors
// ---------------------------------------------------------------------------

fn operation_record(input: &BundleLifecycleOperationInput) -> BundleLifecycleOperationRecord {
    BundleLifecycleOperationRecord {
        record_kind: BUNDLE_LIFECYCLE_OPERATION_RECORD_KIND.to_string(),
        schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
        operation_kind: input.operation_kind.clone(),
        bundle_id: input.bundle_id.clone(),
        bundle_revision: input.bundle_revision,
        review_packet_id: input.review_packet_id.clone(),
        certification_packet_id: input.certification_packet_id.clone(),
        scorecard_row_ref: input.scorecard_row_ref.clone(),
        rollback_checkpoint_ref: input.rollback_checkpoint_ref.clone(),
    }
}

fn dependency_marker_record(input: &BundleDependencyMarkerInput) -> BundleDependencyMarkerRecord {
    BundleDependencyMarkerRecord {
        record_kind: BUNDLE_DEPENDENCY_MARKER_RECORD_KIND.to_string(),
        schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
        marker_id: input.marker_id.clone(),
        capability_class: input.capability_class.clone(),
        narrowing_class: input.narrowing_class.clone(),
        active_dependency: input.active_dependency,
        disclosure_required: input.disclosure_required,
        summary_label: input.summary_label.clone(),
    }
}

fn drift_summary_record(
    input: &ScorecardLinkedDriftSummaryInput,
) -> ScorecardLinkedDriftSummaryRecord {
    ScorecardLinkedDriftSummaryRecord {
        record_kind: SCORECARD_LINKED_DRIFT_SUMMARY_RECORD_KIND.to_string(),
        schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
        summary_id: input.summary_id.clone(),
        drift_state_class: input.drift_state_class.clone(),
        drift_entries: input
            .drift_entries
            .iter()
            .map(|e| ScorecardLinkedDriftEntry {
                entry_id: e.entry_id.clone(),
                drift_axis: e.drift_axis.clone(),
                subject_ref: e.subject_ref.clone(),
                claim_narrowing_class: e.claim_narrowing_class.clone(),
                local_override_ref: e.local_override_ref.clone(),
                scorecard_linkage_preserved: e.scorecard_linkage_preserved,
            })
            .collect(),
        field_package_task_granular: input.field_package_task_granular,
        scorecard_id_ref: input.scorecard_id_ref.clone(),
        scorecard_row_ref: input.scorecard_row_ref.clone(),
    }
}

fn trust_egress_disclosure_record(
    input: &TrustEgressChangeDisclosureInput,
) -> TrustEgressChangeDisclosureRecord {
    TrustEgressChangeDisclosureRecord {
        record_kind: TRUST_EGRESS_CHANGE_DISCLOSURE_RECORD_KIND.to_string(),
        schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
        disclosure_id: input.disclosure_id.clone(),
        change_class: input.change_class.clone(),
        severity_class: input.severity_class.clone(),
        alters_authority_boundary: input.alters_authority_boundary,
        summary_label: input.summary_label.clone(),
    }
}

fn asset_provenance_record(input: &BundleAssetProvenanceInput) -> BundleAssetProvenanceRecord {
    BundleAssetProvenanceRecord {
        record_kind: "bundle_asset_provenance_record".to_string(),
        schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
        asset_ref: input.asset_ref.clone(),
        asset_kind: input.asset_kind.clone(),
        provenance_class: input.provenance_class.clone(),
        bundle_created: input.bundle_created,
        adopted: input.adopted,
        user_owned: input.user_owned,
    }
}

fn inspection_record(
    input: &BundleLifecycleFinalizationInput,
    operation: &BundleLifecycleOperationRecord,
    dependency_markers: &[BundleDependencyMarkerRecord],
    drift_summary: &ScorecardLinkedDriftSummaryRecord,
    trust_egress_disclosures: &[TrustEgressChangeDisclosureRecord],
    asset_provenance: &[BundleAssetProvenanceRecord],
) -> BundleLifecycleInspectionRecord {
    let dependency_markers_disclosed = dependency_markers
        .iter()
        .all(|m| !m.disclosure_required || m.active_dependency);
    let scorecard_linkage_preserved = drift_summary
        .drift_entries
        .iter()
        .all(|e| e.scorecard_linkage_preserved);
    let no_hidden_boundary_changes = trust_egress_disclosures
        .iter()
        .all(|d| !d.alters_authority_boundary || d.severity_class != "ordinary_package_churn");
    let removal_preserves_user_assets = if operation.operation_kind == "remove" {
        asset_provenance
            .iter()
            .all(|asset| !asset.user_owned || asset.provenance_class != "bundle_created")
    } else {
        true
    };

    BundleLifecycleInspectionRecord {
        record_kind: BUNDLE_LIFECYCLE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: BUNDLE_LIFECYCLE_FINALIZATION_SCHEMA_VERSION,
        record_id_ref: input.record_id.clone(),
        operation_kind: operation.operation_kind.clone(),
        bundle_id: operation.bundle_id.clone(),
        effective_badge_class: "certified".to_string(),
        reversible: !operation.rollback_checkpoint_ref.trim().is_empty(),
        dependency_markers_disclosed,
        scorecard_linkage_preserved,
        no_hidden_boundary_changes,
        removal_preserves_user_assets,
        support_export_safe: true,
        dependency_marker_count: dependency_markers.len(),
        drift_entry_count: drift_summary.drift_entries.len(),
        trust_egress_disclosure_count: trust_egress_disclosures.len(),
        asset_provenance_count: asset_provenance.len(),
        summary_label: input.summary_label.clone(),
    }
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

fn err(message: impl Into<String>) -> BundleLifecycleValidationError {
    BundleLifecycleValidationError {
        message: message.into(),
    }
}

fn ensure_eq<T: PartialEq + fmt::Display>(
    actual: T,
    expected: T,
    field: &str,
) -> Result<(), BundleLifecycleValidationError> {
    if actual != expected {
        return Err(err(format!(
            "{field} mismatch: expected {expected}, got {actual}"
        )));
    }
    Ok(())
}

fn ensure_eq_u32(
    actual: u32,
    expected: u32,
    field: &str,
) -> Result<(), BundleLifecycleValidationError> {
    if actual != expected {
        return Err(err(format!(
            "{field} mismatch: expected {expected}, got {actual}"
        )));
    }
    Ok(())
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), BundleLifecycleValidationError> {
    if value.trim().is_empty() {
        return Err(err(format!("{field} must not be empty")));
    }
    Ok(())
}

fn ensure_token(
    tokens: &[&str],
    value: &str,
    field: &str,
) -> Result<(), BundleLifecycleValidationError> {
    if !tokens.contains(&value) {
        return Err(err(format!(
            "{field} must be one of {tokens:?}, got {value}"
        )));
    }
    Ok(())
}

fn validate_operation_record(
    op: &BundleLifecycleOperationRecord,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_eq(
        op.record_kind.as_str(),
        BUNDLE_LIFECYCLE_OPERATION_RECORD_KIND,
        "operation record_kind",
    )?;
    ensure_token(
        BUNDLE_LIFECYCLE_OPERATIONS,
        &op.operation_kind,
        "operation_kind",
    )?;
    ensure_nonempty(&op.bundle_id, "bundle_id")?;
    if op.bundle_revision == 0 {
        return Err(err("bundle_revision must be greater than zero"));
    }
    ensure_nonempty(&op.review_packet_id, "review_packet_id")?;
    ensure_nonempty(&op.certification_packet_id, "certification_packet_id")?;
    ensure_nonempty(&op.rollback_checkpoint_ref, "rollback_checkpoint_ref")?;
    Ok(())
}

fn validate_dependency_marker_record(
    marker: &BundleDependencyMarkerRecord,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_eq(
        marker.record_kind.as_str(),
        BUNDLE_DEPENDENCY_MARKER_RECORD_KIND,
        "marker record_kind",
    )?;
    ensure_nonempty(&marker.marker_id, "marker_id")?;
    ensure_token(
        BUNDLE_DEPENDENCY_CAPABILITY_CLASSES,
        &marker.capability_class,
        "capability_class",
    )?;
    ensure_token(
        BUNDLE_DEPENDENCY_NARROWING_CLASSES,
        &marker.narrowing_class,
        "narrowing_class",
    )?;
    Ok(())
}

fn validate_drift_summary_record(
    summary: &ScorecardLinkedDriftSummaryRecord,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_eq(
        summary.record_kind.as_str(),
        SCORECARD_LINKED_DRIFT_SUMMARY_RECORD_KIND,
        "drift summary record_kind",
    )?;
    ensure_nonempty(&summary.summary_id, "summary_id")?;
    ensure_nonempty(&summary.drift_state_class, "drift_state_class")?;
    ensure_nonempty(&summary.scorecard_id_ref, "scorecard_id_ref")?;
    ensure_nonempty(&summary.scorecard_row_ref, "scorecard_row_ref")?;
    for entry in &summary.drift_entries {
        ensure_nonempty(&entry.entry_id, "drift entry_id")?;
        ensure_nonempty(&entry.drift_axis, "drift_axis")?;
        ensure_nonempty(&entry.subject_ref, "subject_ref")?;
        ensure_nonempty(&entry.claim_narrowing_class, "claim_narrowing_class")?;
    }
    Ok(())
}

fn validate_trust_egress_disclosure_record(
    disclosure: &TrustEgressChangeDisclosureRecord,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_eq(
        disclosure.record_kind.as_str(),
        TRUST_EGRESS_CHANGE_DISCLOSURE_RECORD_KIND,
        "disclosure record_kind",
    )?;
    ensure_nonempty(&disclosure.disclosure_id, "disclosure_id")?;
    ensure_token(
        TRUST_EGRESS_CHANGE_CLASSES,
        &disclosure.change_class,
        "change_class",
    )?;
    ensure_token(
        CHANGE_SEVERITY_CLASSES,
        &disclosure.severity_class,
        "severity_class",
    )?;
    Ok(())
}

fn validate_asset_provenance_record(
    asset: &BundleAssetProvenanceRecord,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_nonempty(&asset.asset_ref, "asset_ref")?;
    ensure_nonempty(&asset.asset_kind, "asset_kind")?;
    ensure_token(
        BUNDLE_ASSET_PROVENANCE_CLASSES,
        &asset.provenance_class,
        "provenance_class",
    )?;
    if asset.user_owned && asset.provenance_class == "bundle_created" {
        return Err(err(
            "user_owned assets cannot be classified as bundle_created",
        ));
    }
    Ok(())
}

fn validate_inspection_record(
    inspection: &BundleLifecycleInspectionRecord,
    record: &BundleLifecycleFinalizationRecord,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        BUNDLE_LIFECYCLE_INSPECTION_RECORD_KIND,
        "inspection record_kind",
    )?;
    ensure_eq(
        inspection.record_id_ref.as_str(),
        record.record_id.as_str(),
        "inspection record_id_ref",
    )?;
    ensure_eq(
        inspection.operation_kind.as_str(),
        record.operation.operation_kind.as_str(),
        "inspection operation_kind",
    )?;
    if inspection.dependency_marker_count != record.dependency_markers.len() {
        return Err(err("inspection dependency_marker_count mismatch"));
    }
    if inspection.drift_entry_count != record.drift_summary.drift_entries.len() {
        return Err(err("inspection drift_entry_count mismatch"));
    }
    if inspection.trust_egress_disclosure_count != record.trust_egress_disclosures.len() {
        return Err(err("inspection trust_egress_disclosure_count mismatch"));
    }
    if inspection.asset_provenance_count != record.asset_provenance.len() {
        return Err(err("inspection asset_provenance_count mismatch"));
    }
    if inspection.no_hidden_boundary_changes != record.no_hidden_boundary_changes() {
        return Err(err("inspection no_hidden_boundary_changes is inconsistent"));
    }
    if inspection.removal_preserves_user_assets != record.removal_preserves_user_assets() {
        return Err(err(
            "inspection removal_preserves_user_assets is inconsistent",
        ));
    }
    Ok(())
}

fn validate_input(
    input: &BundleLifecycleFinalizationInput,
) -> Result<(), BundleLifecycleValidationError> {
    ensure_nonempty(&input.record_id, "record_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    ensure_nonempty(&input.summary_label, "summary_label")?;

    let op = &input.operation;
    ensure_token(
        BUNDLE_LIFECYCLE_OPERATIONS,
        &op.operation_kind,
        "operation_kind",
    )?;
    ensure_nonempty(&op.bundle_id, "bundle_id")?;
    if op.bundle_revision == 0 {
        return Err(err("bundle_revision must be greater than zero"));
    }
    ensure_nonempty(&op.review_packet_id, "review_packet_id")?;
    ensure_nonempty(&op.certification_packet_id, "certification_packet_id")?;
    ensure_nonempty(&op.rollback_checkpoint_ref, "rollback_checkpoint_ref")?;

    for marker in &input.dependency_markers {
        ensure_nonempty(&marker.marker_id, "marker_id")?;
        ensure_token(
            BUNDLE_DEPENDENCY_CAPABILITY_CLASSES,
            &marker.capability_class,
            "capability_class",
        )?;
        ensure_token(
            BUNDLE_DEPENDENCY_NARROWING_CLASSES,
            &marker.narrowing_class,
            "narrowing_class",
        )?;
    }

    ensure_nonempty(&input.drift_summary.summary_id, "drift_summary.summary_id")?;
    ensure_nonempty(
        &input.drift_summary.drift_state_class,
        "drift_summary.drift_state_class",
    )?;
    ensure_nonempty(
        &input.drift_summary.scorecard_id_ref,
        "drift_summary.scorecard_id_ref",
    )?;
    ensure_nonempty(
        &input.drift_summary.scorecard_row_ref,
        "drift_summary.scorecard_row_ref",
    )?;
    for entry in &input.drift_summary.drift_entries {
        ensure_nonempty(&entry.entry_id, "drift_entry.entry_id")?;
        ensure_nonempty(&entry.drift_axis, "drift_entry.drift_axis")?;
    }

    for disclosure in &input.trust_egress_disclosures {
        ensure_nonempty(&disclosure.disclosure_id, "disclosure_id")?;
        ensure_token(
            TRUST_EGRESS_CHANGE_CLASSES,
            &disclosure.change_class,
            "change_class",
        )?;
        ensure_token(
            CHANGE_SEVERITY_CLASSES,
            &disclosure.severity_class,
            "severity_class",
        )?;
    }

    if op.operation_kind == "remove" {
        if input.asset_provenance.is_empty() {
            return Err(err(
                "remove operation must carry asset_provenance classifications",
            ));
        }
    }

    for asset in &input.asset_provenance {
        ensure_nonempty(&asset.asset_ref, "asset_ref")?;
        ensure_token(
            BUNDLE_ASSET_PROVENANCE_CLASSES,
            &asset.provenance_class,
            "provenance_class",
        )?;
    }

    for surface in &input.consumer_surfaces {
        ensure_token(
            BUNDLE_LIFECYCLE_CONSUMER_SURFACES,
            surface,
            "consumer_surface",
        )?;
    }
    if input.consumer_surfaces.is_empty() {
        return Err(err("input must bind at least one consumer surface"));
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn base_operation(kind: &str) -> BundleLifecycleOperationInput {
        BundleLifecycleOperationInput {
            operation_kind: kind.to_string(),
            bundle_id: "bundle.launch.tsjs".to_string(),
            bundle_revision: 2,
            review_packet_id: "review-pkt-1".to_string(),
            certification_packet_id: "cert-pkt-1".to_string(),
            scorecard_row_ref: Some("row-1".to_string()),
            rollback_checkpoint_ref: "checkpoint-1".to_string(),
        }
    }

    fn base_drift_summary() -> ScorecardLinkedDriftSummaryInput {
        ScorecardLinkedDriftSummaryInput {
            summary_id: "drift-1".to_string(),
            drift_state_class: "detected".to_string(),
            drift_entries: vec![ScorecardLinkedDriftEntryInput {
                entry_id: "entry-1".to_string(),
                drift_axis: "extension_set".to_string(),
                subject_ref: "ext.typescript".to_string(),
                claim_narrowing_class: "no_narrowing".to_string(),
                local_override_ref: Some("override-1".to_string()),
                scorecard_linkage_preserved: true,
            }],
            field_package_task_granular: true,
            scorecard_id_ref: "sc-1".to_string(),
            scorecard_row_ref: "row-1".to_string(),
        }
    }

    fn base_input(operation_kind: &str) -> BundleLifecycleFinalizationInput {
        BundleLifecycleFinalizationInput {
            record_id: "rec-1".to_string(),
            generated_at: "2026-06-03T08:00:00Z".to_string(),
            operation: base_operation(operation_kind),
            dependency_markers: vec![BundleDependencyMarkerInput {
                marker_id: "marker-1".to_string(),
                capability_class: "stable_public_capability".to_string(),
                narrowing_class: "no_narrowing".to_string(),
                active_dependency: true,
                disclosure_required: true,
                summary_label: "Stable public".to_string(),
            }],
            drift_summary: base_drift_summary(),
            trust_egress_disclosures: vec![TrustEgressChangeDisclosureInput {
                disclosure_id: "disclosure-1".to_string(),
                change_class: "no_change".to_string(),
                severity_class: "ordinary_package_churn".to_string(),
                alters_authority_boundary: false,
                summary_label: "No change".to_string(),
            }],
            asset_provenance: vec![],
            consumer_surfaces: vec![
                "start_center".to_string(),
                "cli_headless".to_string(),
                "support_export".to_string(),
            ],
            summary_label: "Bundle lifecycle finalization".to_string(),
        }
    }

    #[test]
    fn install_operation_projects() {
        let input = base_input("install");
        let record = BundleLifecycleFinalizationRecord::from_input(input).expect("must project");
        assert_eq!(record.operation.operation_kind, "install");
        assert_eq!(record.operation.bundle_id, "bundle.launch.tsjs");
        assert_eq!(record.dependency_markers.len(), 1);
        assert!(record.drift_summary.drift_entries[0].scorecard_linkage_preserved);
        assert!(record.no_hidden_boundary_changes());
        assert!(record.is_support_export_safe());
    }

    #[test]
    fn update_operation_with_trust_change_disclosed() {
        let mut input = base_input("update");
        input.trust_egress_disclosures = vec![TrustEgressChangeDisclosureInput {
            disclosure_id: "disclosure-trust".to_string(),
            change_class: "trust_widened".to_string(),
            severity_class: "trust_boundary_change".to_string(),
            alters_authority_boundary: true,
            summary_label: "Trust widened".to_string(),
        }];
        let record = BundleLifecycleFinalizationRecord::from_input(input).expect("must project");
        assert_eq!(record.trust_egress_disclosures.len(), 1);
        assert!(record.trust_egress_disclosures[0].alters_authority_boundary);
        assert!(record.no_hidden_boundary_changes());
    }

    #[test]
    fn hidden_trust_change_rejected() {
        let mut input = base_input("update");
        input.trust_egress_disclosures = vec![TrustEgressChangeDisclosureInput {
            disclosure_id: "disclosure-hidden".to_string(),
            change_class: "trust_widened".to_string(),
            severity_class: "ordinary_package_churn".to_string(),
            alters_authority_boundary: true,
            summary_label: "Hidden trust change".to_string(),
        }];
        let result = BundleLifecycleFinalizationRecord::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn remove_operation_requires_asset_provenance() {
        let input = base_input("remove");
        let result = BundleLifecycleFinalizationRecord::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn remove_operation_preserves_user_assets() {
        let mut input = base_input("remove");
        input.asset_provenance = vec![
            BundleAssetProvenanceInput {
                asset_ref: "asset-1".to_string(),
                asset_kind: "extension_set".to_string(),
                provenance_class: "bundle_created".to_string(),
                bundle_created: true,
                adopted: false,
                user_owned: false,
            },
            BundleAssetProvenanceInput {
                asset_ref: "asset-2".to_string(),
                asset_kind: "settings".to_string(),
                provenance_class: "user_created_independent".to_string(),
                bundle_created: false,
                adopted: false,
                user_owned: true,
            },
        ];
        let record = BundleLifecycleFinalizationRecord::from_input(input).expect("must project");
        assert!(record.removal_preserves_user_assets());
        assert_eq!(record.asset_provenance.len(), 2);
    }

    #[test]
    fn remove_operation_rejects_user_owned_as_bundle_created() {
        let mut input = base_input("remove");
        input.asset_provenance = vec![BundleAssetProvenanceInput {
            asset_ref: "asset-1".to_string(),
            asset_kind: "settings".to_string(),
            provenance_class: "bundle_created".to_string(),
            bundle_created: true,
            adopted: false,
            user_owned: true,
        }];
        let result = BundleLifecycleFinalizationRecord::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn drift_must_preserve_scorecard_linkage() {
        let mut input = base_input("update");
        input.drift_summary.drift_entries[0].scorecard_linkage_preserved = false;
        let result = BundleLifecycleFinalizationRecord::from_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn mirror_offline_consumer_surface_accepted() {
        let mut input = base_input("install");
        input.consumer_surfaces = vec![
            "mirror_first_install".to_string(),
            "offline_archive_install".to_string(),
            "support_export".to_string(),
        ];
        let record = BundleLifecycleFinalizationRecord::from_input(input).expect("must project");
        assert!(record
            .consumer_surfaces
            .contains(&"mirror_first_install".to_string()));
    }

    #[test]
    fn projection_roundtrips_through_json() {
        let input = base_input("install");
        let record = BundleLifecycleFinalizationRecord::from_input(input).expect("must project");
        let payload = serde_json::to_string(&record).expect("serialize");
        let projection = project_bundle_lifecycle_finalization(&payload).expect("must project");
        assert_eq!(projection.operation_kind, "install");
        assert_eq!(projection.bundle_id, "bundle.launch.tsjs");
        assert_eq!(projection.bundle_revision, 2);
        assert!(projection.no_hidden_boundary_changes);
        assert!(projection.removal_preserves_user_assets);
    }

    #[test]
    fn rebase_operation_projects() {
        let mut input = base_input("rebase");
        input.trust_egress_disclosures = vec![TrustEgressChangeDisclosureInput {
            disclosure_id: "disclosure-1".to_string(),
            change_class: "stable_to_preview".to_string(),
            severity_class: "managed_dependence_change".to_string(),
            alters_authority_boundary: true,
            summary_label: "Stable to preview".to_string(),
        }];
        let record = BundleLifecycleFinalizationRecord::from_input(input).expect("must project");
        assert_eq!(record.operation.operation_kind, "rebase");
        assert!(record.no_hidden_boundary_changes());
    }

    #[test]
    fn dependency_markers_cover_all_capabilities() {
        for capability in BUNDLE_DEPENDENCY_CAPABILITY_CLASSES {
            assert!([
                "preview_learning_surface",
                "beta_learning_surface",
                "managed_seat_entitlement",
                "org_mirrored_content",
                "labs_only_capability",
                "stable_public_capability",
            ]
            .contains(capability));
        }
    }

    #[test]
    fn fixture_install_validates() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/install_certified_stable.json"
        ));
        let projection = project_bundle_lifecycle_finalization(fixture).expect("fixture valid");
        assert_eq!(projection.operation_kind, "install");
        assert!(projection.no_hidden_boundary_changes);
    }

    #[test]
    fn fixture_update_drift_validates() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/update_with_drift_and_override.json"
        ));
        let projection = project_bundle_lifecycle_finalization(fixture).expect("fixture valid");
        assert_eq!(projection.operation_kind, "update");
        assert!(projection.drift_entry_count > 0);
    }

    #[test]
    fn fixture_remove_validates() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/remove_with_asset_provenance.json"
        ));
        let projection = project_bundle_lifecycle_finalization(fixture).expect("fixture valid");
        assert_eq!(projection.operation_kind, "remove");
        assert!(projection.removal_preserves_user_assets);
    }

    #[test]
    fn fixture_mirror_offline_validates() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m4/finalize-workflow-bundle-lifecycle-drift-and-overrides/mirror_offline_install.json"
        ));
        let projection = project_bundle_lifecycle_finalization(fixture).expect("fixture valid");
        assert_eq!(projection.operation_kind, "install");
        assert!(projection
            .consumer_surfaces
            .contains(&"mirror_first_install".to_string()));
    }
}
