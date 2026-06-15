//! M5 storage-class, low-disk, clear-data, and pin/retention certification.
//!
//! This module composes the already-landed M5 heavy-artifact storage lanes into
//! one certification index that release, Help/About, service-health, and
//! support-export surfaces can ingest verbatim. It mints no new storage
//! behavior; it binds the storage-governance matrix and the clear-data,
//! low-disk-pressure, pin/retention, cache-repair, and offboarding/continuity
//! lanes into one shared decision surface and proves that no heavy-artifact
//! profile can stay green while any of those proofs is stale.
//!
//! The packet answers six questions for every claimed M5 heavy-artifact family
//! on every claimed M5 profile:
//!
//! - which storage class governs the family and whether it is disposable,
//!   rebuildable, durable evidence, or user-owned recovery state (the
//!   storage-class truth proof);
//! - which class-selective clear-data review proves a generic clear can never
//!   erase protected or user-owned state (the clear-data proof);
//! - which low-disk / managed-quota pressure proof shows the frozen eviction
//!   order and the no-authoritative-state-loss guards (the disk-pressure proof);
//! - which pin/retention audit proves pinned evidence stays attributable and
//!   protected (the pin/retention proof);
//! - which corruption-repair drill proves a corrupt cache or index gets a
//!   targeted, no-reset-everything repair (the corruption-repair proof); and
//! - which offboarding/continuity plan proves export-before-delete runs before
//!   any protected removal (the export-before-delete proof).
//!
//! Rows that lose any of those proofs narrow automatically instead of
//! inheriting a greener neighboring claim, and rows that would blur cache versus
//! authoritative state or hide pressure behavior downgrade by construction.

use serde::{Deserialize, Serialize};

use crate::m5_cache_repair::M5_CACHE_REPAIR_SCHEMA_REF;
use crate::m5_clear_data_review::M5_CLEAR_DATA_REVIEW_SCHEMA_REF;
use crate::m5_fault_crash_certification::ClaimedM5Profile;
use crate::m5_offboarding_continuity::M5_OFFBOARDING_CONTINUITY_SCHEMA_REF;
use crate::m5_pin_retention::M5_PIN_RETENTION_SCHEMA_REF;
use crate::m5_storage_governance::{
    current_m5_artifact_family_storage_matrix, ArtifactFamilyId, AuthorityClass, GcPolicyClass,
    M5ArtifactFamilyRow, M5ArtifactFamilyStorageMatrix, M5_ARTIFACT_FAMILY_MATRIX_REF,
    M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF,
};
use crate::m5_storage_pressure::M5_STORAGE_PRESSURE_SCHEMA_REF;
use crate::storage_inspector::StorageClassId;

// Golden support-export projections each proof lane already checks in. The
// certification cites the same evidence its sibling lanes regenerate so a stale
// or deleted lane fixture is visible from the certification, not just inside the
// owning lane.
const STORAGE_CLASS_TRUTH_GOLDEN_REF: &str =
    "fixtures/storage/m5_artifact_family_storage_matrix/support_export.golden.json";
const CLEAR_DATA_REVIEW_GOLDEN_REF: &str =
    "fixtures/storage/m5_clear_data_review/support_export.golden.json";
const LOW_DISK_PRESSURE_GOLDEN_REF: &str =
    "fixtures/storage/m5_storage_pressure/support_export.golden.json";
const PIN_RETENTION_GOLDEN_REF: &str =
    "fixtures/storage/m5_pin_retention/support_export.golden.json";
const CORRUPTION_REPAIR_GOLDEN_REF: &str =
    "fixtures/storage/m5_cache_repair/support_export.golden.json";
const EXPORT_BEFORE_DELETE_GOLDEN_REF: &str =
    "fixtures/storage/m5_offboarding_continuity/support_export.golden.json";

// Checked consumer surfaces that must ingest the certification index verbatim.
const SERVICE_HEALTH_CONSUMER_REF: &str =
    "crates/aureline-service-health/src/finalize_service_health_destination_truth/mod.rs";
const SUPPORT_EXPORT_CONSUMER_REF: &str = "schemas/support/support_bundle_manifest.schema.json";
const RELEASE_MANIFEST_CONSUMER_REF: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";

const REQUIRED_PROJECTION_FIELDS: &[&str] = &[
    "certification_row_id",
    "family_id",
    "profile",
    "published_state",
    "stale_proof_tokens",
    "downgrade_rule_ids",
];

/// Stable record-kind tag carried by [`M5StorageCertificationPacket`].
pub const M5_STORAGE_CERTIFICATION_PACKET_RECORD_KIND: &str = "m5_storage_certification_packet";

/// Frozen schema version for the M5 storage certification packet.
pub const M5_STORAGE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema.
pub const M5_STORAGE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/storage/m5_storage_certification.schema.json";

/// Repository-relative path of the reviewer-facing contract document.
pub const M5_STORAGE_CERTIFICATION_DOC_REF: &str =
    "docs/storage/m5_storage_certification_contract.md";

/// Repository-relative path of the checked review artifact.
pub const M5_STORAGE_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/storage/m5_storage_certification.md";

/// Repository-relative path of the protected fixture directory.
pub const M5_STORAGE_CERTIFICATION_FIXTURE_DIR: &str = "fixtures/storage/m5_storage_certification";

/// Stable packet identifier reused by every surface binding.
pub const M5_STORAGE_CERTIFICATION_PACKET_ID: &str = "storage.m5.storage_certification.v1";

/// Certification result published for one family/profile row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageCertificationStateClass {
    /// All six storage proofs are current on the claimed profile.
    Qualified,
    /// The family keeps a narrower, class-scoped storage claim only.
    LimitedClassScoped,
    /// The family may only be touched through an explicit protected review.
    ProtectedReviewGatedOnly,
    /// The broad storage claim is blocked pending fresh proof.
    BlockedUnverified,
}

impl StorageCertificationStateClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::LimitedClassScoped => "limited_class_scoped",
            Self::ProtectedReviewGatedOnly => "protected_review_gated_only",
            Self::BlockedUnverified => "blocked_unverified",
        }
    }
}

/// Pressure source that governs eviction for one family on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureSourcePostureClass {
    /// Only local disk pressure can trim the family; no managed quota applies.
    LocalDiskOnly,
    /// Both local disk pressure and a managed quota ceiling apply, in the same
    /// frozen eviction order.
    DiskAndManagedQuota,
    /// A protected family that managed quota may never auto-delete; only
    /// explicit, reviewed removal can free it.
    ManagedQuotaProtectedExcluded,
}

impl PressureSourcePostureClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDiskOnly => "local_disk_only",
            Self::DiskAndManagedQuota => "disk_and_managed_quota",
            Self::ManagedQuotaProtectedExcluded => "managed_quota_protected_excluded",
        }
    }
}

/// Downgrade trigger automated by the certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageCertificationDowngradeTriggerClass {
    /// Storage-class truth is stale, so cache versus authoritative state blurs.
    StorageClassTruthStale,
    /// The class-selective clear-data review proof is stale or missing.
    ClearDataReviewStale,
    /// The low-disk / managed-quota pressure proof is stale, hiding behavior.
    LowDiskPressureProofStale,
    /// The pin/retention audit is stale, so pinned evidence is unverified.
    PinRetentionEvidenceStale,
    /// The corruption-repair drill is stale or missing.
    CorruptionRepairDrillStale,
    /// The export-before-delete validation is stale or missing.
    ExportBeforeDeleteValidationStale,
    /// One downstream surface stopped ingesting the certification by reference.
    ConsumerBindingMissing,
}

impl StorageCertificationDowngradeTriggerClass {
    /// All downgrade triggers in canonical order.
    pub const ALL: [Self; 7] = [
        Self::StorageClassTruthStale,
        Self::ClearDataReviewStale,
        Self::LowDiskPressureProofStale,
        Self::PinRetentionEvidenceStale,
        Self::CorruptionRepairDrillStale,
        Self::ExportBeforeDeleteValidationStale,
        Self::ConsumerBindingMissing,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StorageClassTruthStale => "storage_class_truth_stale",
            Self::ClearDataReviewStale => "clear_data_review_stale",
            Self::LowDiskPressureProofStale => "low_disk_pressure_proof_stale",
            Self::PinRetentionEvidenceStale => "pin_retention_evidence_stale",
            Self::CorruptionRepairDrillStale => "corruption_repair_drill_stale",
            Self::ExportBeforeDeleteValidationStale => "export_before_delete_validation_stale",
            Self::ConsumerBindingMissing => "consumer_binding_missing",
        }
    }
}

/// Stable consumer surface that ingests the certification result verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageCertificationSurfaceClass {
    /// Help/About storage proof and provenance cards.
    HelpAbout,
    /// Service-health and storage-pressure truth surfaces.
    ServiceHealth,
    /// Support-export and storage handoff surfaces.
    SupportExport,
    /// Release manifest and publication control surfaces.
    ReleaseManifest,
}

impl StorageCertificationSurfaceClass {
    /// All consumer surfaces in canonical order.
    pub const ALL: [Self; 4] = [
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::ReleaseManifest,
    ];

    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::ReleaseManifest => "release_manifest",
        }
    }
}

/// One family/profile certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageProfileCertificationRow {
    /// Stable row identifier.
    pub certification_row_id: String,
    /// Heavy-artifact family covered by the row.
    pub family_id: ArtifactFamilyId,
    /// Human-readable family label.
    pub family_label: String,
    /// Claimed M5 profile covered by the row.
    pub profile: ClaimedM5Profile,
    /// Published certification state for the row.
    pub published_state: StorageCertificationStateClass,
    /// Governing storage class, quoted from the storage-governance matrix.
    pub storage_class_id: StorageClassId,
    /// Authority posture, quoted from the storage-governance matrix.
    pub authority_class: AuthorityClass,
    /// True when the family is protected evidence or user-owned recovery state.
    pub protected_continuity: bool,
    /// Pressure source that governs eviction for the family on this profile.
    pub pressure_source_posture: PressureSourcePostureClass,
    /// Storage-class truth proof backing the row.
    pub storage_class_truth_ref: String,
    /// Class-selective clear-data review proof backing the row.
    pub clear_data_review_ref: String,
    /// Low-disk / managed-quota pressure proof backing the row.
    pub low_disk_pressure_ref: String,
    /// Pin/retention audit proof backing the row.
    pub pin_retention_ref: String,
    /// Corruption-repair drill proof backing the row.
    pub corruption_repair_ref: String,
    /// Export-before-delete / offboarding-continuity proof backing the row.
    pub export_before_delete_ref: String,
    /// Active stale or capability-loss tokens narrowing the row.
    pub stale_proof_tokens: Vec<String>,
    /// Active downgrade-rule identifiers explaining the published state.
    pub downgrade_rule_ids: Vec<String>,
    /// Review-safe summary for downstream surfaces.
    pub summary: String,
}

/// One downgrade rule published by the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCertificationDowngradeRuleRow {
    /// Stable rule identifier.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger_class: StorageCertificationDowngradeTriggerClass,
    /// Source certification state before the downgrade.
    pub source_state: StorageCertificationStateClass,
    /// Resulting certification state after the downgrade.
    pub downgraded_state: StorageCertificationStateClass,
    /// User-visible effect of the downgrade.
    pub required_effect: String,
    /// Reviewable rationale for the downgrade.
    pub rationale: String,
    /// Supporting evidence or contract refs used to inspect the rule.
    pub evidence_refs: Vec<String>,
}

/// One consumer-surface binding proving the same certification result is reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCertificationSurfaceBinding {
    /// Consumer surface that ingests the certification.
    pub surface: StorageCertificationSurfaceClass,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet identifier the consumer ingests verbatim.
    pub ingested_packet_id: String,
    /// Number of certification rows the consumer exposes by reference.
    pub certification_row_count: usize,
    /// Fields the consumer must preserve verbatim from the packet.
    pub required_verbatim_fields: Vec<String>,
    /// True when the consumer narrows immediately on stale proof or blocked rows.
    pub narrow_on_stale_proof: bool,
    /// True when limited or review-gated states stay labeled explicitly.
    pub explicit_limited_state_labels_required: bool,
    /// Review-safe summary of the binding contract.
    pub summary: String,
}

/// One validation error returned by [`M5StorageCertificationPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StorageCertificationViolation {
    /// Field or collection path that failed validation.
    pub path: String,
    /// Reviewable explanation of the failure.
    pub message: String,
}

/// Canonical M5 storage certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StorageCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Frozen schema version.
    pub schema_version: u32,
    /// Stable packet identifier.
    pub packet_id: String,
    /// RFC 3339 UTC generation time.
    pub generated_at: String,
    /// Reviewer-facing contract document ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Checked review artifact ref.
    pub artifact_ref: String,
    /// Authoritative spec sections quoted by the packet.
    pub source_spec_refs: Vec<String>,
    /// Existing packets and contracts this certification composes.
    pub supporting_contract_refs: Vec<String>,
    /// Claimed M5 profiles covered by the packet.
    pub claimed_profiles: Vec<ClaimedM5Profile>,
    /// Canonical family/profile certification rows.
    pub certification_rows: Vec<StorageProfileCertificationRow>,
    /// Automatic downgrade rules used by the packet.
    pub downgrade_rules: Vec<StorageCertificationDowngradeRuleRow>,
    /// Consumer-surface bindings that prove one certification index is reused.
    pub surface_bindings: Vec<StorageCertificationSurfaceBinding>,
    /// Metadata-safe summary safe for support and release surfaces.
    pub export_safe_summary: String,
}

impl M5StorageCertificationPacket {
    /// Validates profile coverage, downgrade automation, matrix consistency,
    /// and shared-surface bindings.
    pub fn validate(&self) -> Vec<M5StorageCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_STORAGE_CERTIFICATION_PACKET_RECORD_KIND {
            push(&mut violations, "record_kind", "unexpected record_kind");
        }
        if self.schema_version != M5_STORAGE_CERTIFICATION_SCHEMA_VERSION {
            push(
                &mut violations,
                "schema_version",
                "unexpected schema_version",
            );
        }
        if self.doc_ref != M5_STORAGE_CERTIFICATION_DOC_REF {
            push(
                &mut violations,
                "doc_ref",
                "packet must quote the canonical reviewer doc",
            );
        }
        if self.schema_ref != M5_STORAGE_CERTIFICATION_SCHEMA_REF {
            push(
                &mut violations,
                "schema_ref",
                "packet must quote the canonical schema ref",
            );
        }
        if self.artifact_ref != M5_STORAGE_CERTIFICATION_ARTIFACT_REF {
            push(
                &mut violations,
                "artifact_ref",
                "packet must quote the checked review artifact ref",
            );
        }
        if self.supporting_contract_refs.is_empty() {
            push(
                &mut violations,
                "supporting_contract_refs",
                "packet must cite the composed lane contracts",
            );
        }

        for required in ClaimedM5Profile::ALL {
            if !self.claimed_profiles.contains(&required) {
                push(
                    &mut violations,
                    "claimed_profiles",
                    &format!("missing claimed profile {}", required.as_str()),
                );
            }
        }

        // Load the storage-governance matrix so each row is provably consistent
        // with the canonical storage-class truth source rather than a local copy.
        let matrix = current_m5_artifact_family_storage_matrix().ok();
        if matrix.is_none() {
            push(
                &mut violations,
                "storage_governance_matrix",
                "could not load the storage-governance matrix to cross-check rows",
            );
        }

        for family in M5ArtifactFamilyStorageMatrix::required_families() {
            for profile in ClaimedM5Profile::ALL {
                if !self
                    .certification_rows
                    .iter()
                    .any(|row| row.family_id == *family && row.profile == profile)
                {
                    push(
                        &mut violations,
                        "certification_rows",
                        &format!(
                            "missing certification row for family {} on profile {}",
                            family.as_str(),
                            profile.as_str()
                        ),
                    );
                }
            }
        }

        let rule_ids: Vec<&str> = self
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();

        for row in &self.certification_rows {
            self.validate_row(&mut violations, row, matrix.as_ref(), &rule_ids);
        }

        for required in StorageCertificationDowngradeTriggerClass::ALL {
            if !self
                .downgrade_rules
                .iter()
                .any(|rule| rule.trigger_class == required)
            {
                push(
                    &mut violations,
                    "downgrade_rules",
                    &format!("missing downgrade trigger {}", required.as_str()),
                );
            }
        }
        for rule in &self.downgrade_rules {
            if rule.evidence_refs.is_empty() {
                push(
                    &mut violations,
                    &format!("downgrade_rules.{}", rule.rule_id),
                    "downgrade rule must cite at least one evidence ref",
                );
            }
        }

        for required in StorageCertificationSurfaceClass::ALL {
            let Some(binding) = self
                .surface_bindings
                .iter()
                .find(|binding| binding.surface == required)
            else {
                push(
                    &mut violations,
                    "surface_bindings",
                    &format!("missing surface binding {}", required.as_str()),
                );
                continue;
            };
            let base = format!("surface_bindings.{}", binding.surface.as_str());
            if binding.ingested_packet_id != self.packet_id {
                push(
                    &mut violations,
                    &base,
                    "surface binding must ingest the canonical packet id",
                );
            }
            if binding.certification_row_count != self.certification_rows.len() {
                push(
                    &mut violations,
                    &base,
                    "surface binding row count must match certification rows",
                );
            }
            for field in REQUIRED_PROJECTION_FIELDS {
                if !binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field)
                {
                    push(
                        &mut violations,
                        &base,
                        &format!("surface binding must preserve {field}"),
                    );
                }
            }
        }

        violations
    }

    fn validate_row(
        &self,
        violations: &mut Vec<M5StorageCertificationViolation>,
        row: &StorageProfileCertificationRow,
        matrix: Option<&M5ArtifactFamilyStorageMatrix>,
        rule_ids: &[&str],
    ) {
        let base = format!("certification_rows.{}", row.certification_row_id);
        for (field, value) in [
            ("family_label", row.family_label.as_str()),
            (
                "storage_class_truth_ref",
                row.storage_class_truth_ref.as_str(),
            ),
            ("clear_data_review_ref", row.clear_data_review_ref.as_str()),
            ("low_disk_pressure_ref", row.low_disk_pressure_ref.as_str()),
            ("pin_retention_ref", row.pin_retention_ref.as_str()),
            ("corruption_repair_ref", row.corruption_repair_ref.as_str()),
            (
                "export_before_delete_ref",
                row.export_before_delete_ref.as_str(),
            ),
            ("summary", row.summary.as_str()),
        ] {
            if value.trim().is_empty() {
                push(
                    violations,
                    &format!("{base}.{field}"),
                    "row field may not be empty",
                );
            }
        }

        if row.published_state == StorageCertificationStateClass::Qualified
            && !row.stale_proof_tokens.is_empty()
        {
            push(
                violations,
                &format!("{base}.stale_proof_tokens"),
                "qualified rows may not carry stale proof tokens",
            );
        }
        if row.published_state != StorageCertificationStateClass::Qualified
            && row.downgrade_rule_ids.is_empty()
        {
            push(
                violations,
                &format!("{base}.downgrade_rule_ids"),
                "non-qualified rows must cite downgrade rules",
            );
        }
        for rule_id in &row.downgrade_rule_ids {
            if !rule_ids.contains(&rule_id.as_str()) {
                push(
                    violations,
                    &format!("{base}.downgrade_rule_ids"),
                    &format!("row cites unknown downgrade rule {rule_id}"),
                );
            }
        }

        // Cross-check the storage-class, authority, and protection posture
        // against the canonical matrix: the certification may not invent a
        // storage truth the governance lane does not also publish.
        if let Some(matrix) = matrix {
            if let Some(matrix_row) = matrix.family(row.family_id) {
                if row.storage_class_id != matrix_row.storage_class_id {
                    push(
                        violations,
                        &format!("{base}.storage_class_id"),
                        "storage_class_id must match the storage-governance matrix",
                    );
                }
                if row.authority_class != matrix_row.authority_class {
                    push(
                        violations,
                        &format!("{base}.authority_class"),
                        "authority_class must match the storage-governance matrix",
                    );
                }
                if row.protected_continuity != matrix_row.protected_continuity {
                    push(
                        violations,
                        &format!("{base}.protected_continuity"),
                        "protected_continuity must match the storage-governance matrix",
                    );
                }
            } else {
                push(
                    violations,
                    &format!("{base}.family_id"),
                    "family has no row in the storage-governance matrix",
                );
            }
        }

        // Managed quota may never auto-delete a protected family: a protected
        // family on a managed-cloud profile must declare the excluded posture.
        if row.profile == ClaimedM5Profile::ManagedCloud
            && row.protected_continuity
            && row.pressure_source_posture
                != PressureSourcePostureClass::ManagedQuotaProtectedExcluded
        {
            push(
                violations,
                &format!("{base}.pressure_source_posture"),
                "protected families on managed_cloud must be excluded from managed-quota deletion",
            );
        }
        // A non-managed profile has no managed quota; it must stay disk-only.
        if row.profile != ClaimedM5Profile::ManagedCloud
            && row.pressure_source_posture != PressureSourcePostureClass::LocalDiskOnly
        {
            push(
                violations,
                &format!("{base}.pressure_source_posture"),
                "non-managed profiles carry only local disk pressure",
            );
        }
    }

    /// Returns true when the packet remains metadata-safe by construction.
    pub fn is_export_safe(&self) -> bool {
        self.export_safe_summary.contains("metadata-safe")
            && self
                .surface_bindings
                .iter()
                .all(|binding| binding.narrow_on_stale_proof)
    }
}

/// Returns the canonical seeded M5 storage certification packet.
pub fn seeded_m5_storage_certification_packet() -> M5StorageCertificationPacket {
    build_packet(CertificationVariant::Canonical)
}

/// Returns a seeded packet where the pin/retention audit is stale, gating every
/// protected family behind an explicit review.
pub fn seeded_stale_pin_retention_m5_storage_certification_packet() -> M5StorageCertificationPacket
{
    build_packet(CertificationVariant::StalePinRetention)
}

/// Returns a seeded packet where storage-class truth is stale and pressure
/// behavior is hidden, blocking authoritative families and narrowing disposable
/// ones.
pub fn seeded_blurred_cache_authority_m5_storage_certification_packet(
) -> M5StorageCertificationPacket {
    build_packet(CertificationVariant::BlurredCacheAuthority)
}

#[derive(Debug, Clone, Copy)]
enum CertificationVariant {
    Canonical,
    StalePinRetention,
    BlurredCacheAuthority,
}

fn build_packet(variant: CertificationVariant) -> M5StorageCertificationPacket {
    let matrix = current_m5_artifact_family_storage_matrix()
        .expect("load checked-in storage-governance matrix");

    let mut certification_rows = Vec::new();
    for family in M5ArtifactFamilyStorageMatrix::required_families() {
        let matrix_row = matrix
            .family(*family)
            .expect("matrix row for required family");
        for profile in ClaimedM5Profile::ALL {
            certification_rows.push(seed_row(matrix_row, profile, variant));
        }
    }

    let row_count = certification_rows.len();

    M5StorageCertificationPacket {
        record_kind: M5_STORAGE_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_STORAGE_CERTIFICATION_SCHEMA_VERSION,
        packet_id: M5_STORAGE_CERTIFICATION_PACKET_ID.to_owned(),
        generated_at: "2026-06-14T00:00:00Z".to_owned(),
        doc_ref: M5_STORAGE_CERTIFICATION_DOC_REF.to_owned(),
        schema_ref: M5_STORAGE_CERTIFICATION_SCHEMA_REF.to_owned(),
        artifact_ref: M5_STORAGE_CERTIFICATION_ARTIFACT_REF.to_owned(),
        source_spec_refs: vec![
            ".t2/docs/Aureline_Milestones_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md".to_owned(),
        ],
        supporting_contract_refs: vec![
            M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF.to_owned(),
            M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
            M5_CLEAR_DATA_REVIEW_SCHEMA_REF.to_owned(),
            M5_STORAGE_PRESSURE_SCHEMA_REF.to_owned(),
            M5_PIN_RETENTION_SCHEMA_REF.to_owned(),
            M5_CACHE_REPAIR_SCHEMA_REF.to_owned(),
            M5_OFFBOARDING_CONTINUITY_SCHEMA_REF.to_owned(),
            STORAGE_CLASS_TRUTH_GOLDEN_REF.to_owned(),
            CLEAR_DATA_REVIEW_GOLDEN_REF.to_owned(),
            LOW_DISK_PRESSURE_GOLDEN_REF.to_owned(),
            PIN_RETENTION_GOLDEN_REF.to_owned(),
            CORRUPTION_REPAIR_GOLDEN_REF.to_owned(),
            EXPORT_BEFORE_DELETE_GOLDEN_REF.to_owned(),
        ],
        claimed_profiles: ClaimedM5Profile::ALL.to_vec(),
        certification_rows,
        downgrade_rules: seeded_downgrade_rules(),
        surface_bindings: seeded_surface_bindings(row_count),
        export_safe_summary:
            "This metadata-safe certification index binds every claimed M5 heavy-artifact family and profile to explicit storage-class, clear-data, low-disk, pin/retention, corruption-repair, and export-before-delete proof; stale or blurred storage truth narrows instead of inheriting adjacent maturity, and no raw payloads cross the boundary."
                .to_owned(),
    }
}

fn seed_row(
    matrix_row: &M5ArtifactFamilyRow,
    profile: ClaimedM5Profile,
    variant: CertificationVariant,
) -> StorageProfileCertificationRow {
    let family_id = matrix_row.family_id;
    let mut row = StorageProfileCertificationRow {
        certification_row_id: format!("m5_storage:{}:{}", family_id.as_str(), profile.as_str()),
        family_id,
        family_label: matrix_row.label.clone(),
        profile,
        published_state: StorageCertificationStateClass::Qualified,
        storage_class_id: matrix_row.storage_class_id,
        authority_class: matrix_row.authority_class,
        protected_continuity: matrix_row.protected_continuity,
        pressure_source_posture: pressure_source_posture(profile, matrix_row),
        storage_class_truth_ref: STORAGE_CLASS_TRUTH_GOLDEN_REF.to_owned(),
        clear_data_review_ref: CLEAR_DATA_REVIEW_GOLDEN_REF.to_owned(),
        low_disk_pressure_ref: LOW_DISK_PRESSURE_GOLDEN_REF.to_owned(),
        pin_retention_ref: PIN_RETENTION_GOLDEN_REF.to_owned(),
        corruption_repair_ref: CORRUPTION_REPAIR_GOLDEN_REF.to_owned(),
        export_before_delete_ref: EXPORT_BEFORE_DELETE_GOLDEN_REF.to_owned(),
        stale_proof_tokens: Vec::new(),
        downgrade_rule_ids: Vec::new(),
        summary: format!(
            "{} on {} reuses the canonical storage-class, clear-data, low-disk, pin/retention, corruption-repair, and export-before-delete proofs.",
            matrix_row.label,
            profile.as_str()
        ),
    };

    match variant {
        CertificationVariant::Canonical => {}
        CertificationVariant::StalePinRetention => {
            if matrix_row.protected_continuity {
                apply_downgrade(
                    &mut row,
                    StorageCertificationStateClass::ProtectedReviewGatedOnly,
                    "stale_pin_retention_audit",
                    "pin_retention_stale_gates_protected_family",
                    &format!(
                        "{} on {} narrows to explicit protected review because the pin/retention audit is stale; pinned-evidence integrity cannot be certified.",
                        matrix_row.label,
                        profile.as_str()
                    ),
                );
            }
        }
        CertificationVariant::BlurredCacheAuthority => {
            if matrix_row.protected_continuity {
                apply_downgrade(
                    &mut row,
                    StorageCertificationStateClass::BlockedUnverified,
                    "blurred_cache_versus_authoritative_state",
                    "storage_class_truth_stale_blocks_authority_claim",
                    &format!(
                        "{} on {} is blocked because storage-class truth is stale; cache versus authoritative state can no longer be told apart.",
                        matrix_row.label,
                        profile.as_str()
                    ),
                );
            } else if matrix_row.gc_policy_class == GcPolicyClass::GcOnPressureIfUnpinned {
                apply_downgrade(
                    &mut row,
                    StorageCertificationStateClass::LimitedClassScoped,
                    "hidden_low_disk_pressure_behavior",
                    "low_disk_pressure_proof_stale_narrows_disposable_claim",
                    &format!(
                        "{} on {} narrows to a class-scoped claim because the low-disk pressure proof is stale and the eviction behavior is hidden.",
                        matrix_row.label,
                        profile.as_str()
                    ),
                );
            }
        }
    }

    row
}

/// Derives the pressure source posture for one family on one profile. Only the
/// managed-cloud profile adds a managed quota ceiling; protected families stay
/// explicitly excluded from quota-driven deletion.
fn pressure_source_posture(
    profile: ClaimedM5Profile,
    matrix_row: &M5ArtifactFamilyRow,
) -> PressureSourcePostureClass {
    if profile != ClaimedM5Profile::ManagedCloud {
        return PressureSourcePostureClass::LocalDiskOnly;
    }
    if matrix_row.protected_continuity {
        PressureSourcePostureClass::ManagedQuotaProtectedExcluded
    } else {
        PressureSourcePostureClass::DiskAndManagedQuota
    }
}

fn apply_downgrade(
    row: &mut StorageProfileCertificationRow,
    state: StorageCertificationStateClass,
    token: &str,
    rule_id: &str,
    summary: &str,
) {
    row.published_state = state;
    row.stale_proof_tokens.push(token.to_owned());
    row.downgrade_rule_ids.push(rule_id.to_owned());
    row.summary = summary.to_owned();
}

fn seeded_downgrade_rules() -> Vec<StorageCertificationDowngradeRuleRow> {
    vec![
        StorageCertificationDowngradeRuleRow {
            rule_id: "storage_class_truth_stale_blocks_authority_claim".to_owned(),
            trigger_class: StorageCertificationDowngradeTriggerClass::StorageClassTruthStale,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::BlockedUnverified,
            required_effect: "When storage-class truth is stale, surfaces must stop claiming the cache-versus-authoritative distinction and block the broad storage claim until the matrix proof is refreshed.".to_owned(),
            rationale: "A row that blurs cache versus authoritative state cannot stay green.".to_owned(),
            evidence_refs: vec![
                M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF.to_owned(),
                STORAGE_CLASS_TRUTH_GOLDEN_REF.to_owned(),
            ],
        },
        StorageCertificationDowngradeRuleRow {
            rule_id: "clear_data_review_stale_narrows_clear_claim".to_owned(),
            trigger_class: StorageCertificationDowngradeTriggerClass::ClearDataReviewStale,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::LimitedClassScoped,
            required_effect: "When the class-selective clear-data review proof is stale, surfaces must narrow to a class-scoped claim and may not advertise a generic clear that could reach protected or user-owned state.".to_owned(),
            rationale: "A generic clear-data path is only safe while the class-selective review proof is current.".to_owned(),
            evidence_refs: vec![
                M5_CLEAR_DATA_REVIEW_SCHEMA_REF.to_owned(),
                CLEAR_DATA_REVIEW_GOLDEN_REF.to_owned(),
            ],
        },
        StorageCertificationDowngradeRuleRow {
            rule_id: "low_disk_pressure_proof_stale_narrows_disposable_claim".to_owned(),
            trigger_class: StorageCertificationDowngradeTriggerClass::LowDiskPressureProofStale,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::LimitedClassScoped,
            required_effect: "When the low-disk / managed-quota pressure proof is stale, surfaces must narrow the eviction claim and stop implying the frozen ladder and no-authoritative-state-loss guards are proven.".to_owned(),
            rationale: "Hiding pressure behavior behind a stale proof is exactly the blur this lane exists to prevent.".to_owned(),
            evidence_refs: vec![
                M5_STORAGE_PRESSURE_SCHEMA_REF.to_owned(),
                LOW_DISK_PRESSURE_GOLDEN_REF.to_owned(),
            ],
        },
        StorageCertificationDowngradeRuleRow {
            rule_id: "pin_retention_stale_gates_protected_family".to_owned(),
            trigger_class: StorageCertificationDowngradeTriggerClass::PinRetentionEvidenceStale,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::ProtectedReviewGatedOnly,
            required_effect: "When the pin/retention audit is stale, protected families must be gated behind an explicit review and may not publish a green pinned-evidence integrity claim.".to_owned(),
            rationale: "Stale pin/retention evidence may not keep a protected family's row green.".to_owned(),
            evidence_refs: vec![
                M5_PIN_RETENTION_SCHEMA_REF.to_owned(),
                PIN_RETENTION_GOLDEN_REF.to_owned(),
            ],
        },
        StorageCertificationDowngradeRuleRow {
            rule_id: "corruption_repair_drill_stale_narrows_repair_claim".to_owned(),
            trigger_class: StorageCertificationDowngradeTriggerClass::CorruptionRepairDrillStale,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::LimitedClassScoped,
            required_effect: "When the corruption-repair drill is stale, surfaces must narrow the repair claim and may not promise a targeted, no-reset-everything repair across the family's storage class.".to_owned(),
            rationale: "A targeted-repair claim requires a current corruption-repair drill.".to_owned(),
            evidence_refs: vec![
                M5_CACHE_REPAIR_SCHEMA_REF.to_owned(),
                CORRUPTION_REPAIR_GOLDEN_REF.to_owned(),
            ],
        },
        StorageCertificationDowngradeRuleRow {
            rule_id: "export_before_delete_validation_stale_gates_protected_family".to_owned(),
            trigger_class:
                StorageCertificationDowngradeTriggerClass::ExportBeforeDeleteValidationStale,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::ProtectedReviewGatedOnly,
            required_effect: "When the export-before-delete validation is stale, protected families must be gated behind an explicit review so no offboarding or reset can remove them without an exported copy.".to_owned(),
            rationale: "Export-before-delete is the guard that makes protected removal honest; a stale proof gates it.".to_owned(),
            evidence_refs: vec![
                M5_OFFBOARDING_CONTINUITY_SCHEMA_REF.to_owned(),
                EXPORT_BEFORE_DELETE_GOLDEN_REF.to_owned(),
            ],
        },
        StorageCertificationDowngradeRuleRow {
            rule_id: "consumer_binding_missing_blocks_shared_truth".to_owned(),
            trigger_class: StorageCertificationDowngradeTriggerClass::ConsumerBindingMissing,
            source_state: StorageCertificationStateClass::Qualified,
            downgraded_state: StorageCertificationStateClass::BlockedUnverified,
            required_effect: "If Help/About, service health, support export, or release manifest stops ingesting this packet by reference, the broad storage claim blocks until parity is restored.".to_owned(),
            rationale: "The task requires one storage certification index; broken consumer bindings invalidate that promise.".to_owned(),
            evidence_refs: vec![
                M5_STORAGE_CERTIFICATION_DOC_REF.to_owned(),
                SERVICE_HEALTH_CONSUMER_REF.to_owned(),
                SUPPORT_EXPORT_CONSUMER_REF.to_owned(),
                RELEASE_MANIFEST_CONSUMER_REF.to_owned(),
            ],
        },
    ]
}

fn seeded_surface_bindings(row_count: usize) -> Vec<StorageCertificationSurfaceBinding> {
    let verbatim_fields: Vec<String> = REQUIRED_PROJECTION_FIELDS
        .iter()
        .map(|field| (*field).to_owned())
        .collect();
    vec![
        StorageCertificationSurfaceBinding {
            surface: StorageCertificationSurfaceClass::HelpAbout,
            consumer_ref: M5_STORAGE_CERTIFICATION_DOC_REF.to_owned(),
            ingested_packet_id: M5_STORAGE_CERTIFICATION_PACKET_ID.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: verbatim_fields.clone(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Help/About reuses the certification row ids, family tokens, profile tokens, published state, and stale-proof tokens verbatim instead of paraphrasing storage maturity.".to_owned(),
        },
        StorageCertificationSurfaceBinding {
            surface: StorageCertificationSurfaceClass::ServiceHealth,
            consumer_ref: SERVICE_HEALTH_CONSUMER_REF.to_owned(),
            ingested_packet_id: M5_STORAGE_CERTIFICATION_PACKET_ID.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: verbatim_fields.clone(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Service-health surfaces ingest the same certification index so low-disk and managed-quota pressure degradations do not drift from Help/About or support-export truth.".to_owned(),
        },
        StorageCertificationSurfaceBinding {
            surface: StorageCertificationSurfaceClass::SupportExport,
            consumer_ref: SUPPORT_EXPORT_CONSUMER_REF.to_owned(),
            ingested_packet_id: M5_STORAGE_CERTIFICATION_PACKET_ID.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: verbatim_fields.clone(),
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Support-export packets attach the same row ids and downgrade tokens instead of inventing a parallel storage-support badge.".to_owned(),
        },
        StorageCertificationSurfaceBinding {
            surface: StorageCertificationSurfaceClass::ReleaseManifest,
            consumer_ref: RELEASE_MANIFEST_CONSUMER_REF.to_owned(),
            ingested_packet_id: M5_STORAGE_CERTIFICATION_PACKET_ID.to_owned(),
            certification_row_count: row_count,
            required_verbatim_fields: verbatim_fields,
            narrow_on_stale_proof: true,
            explicit_limited_state_labels_required: true,
            summary: "Release manifests consume the same certification index so stale clear-data, pressure, or pin/retention proof cannot keep a broader release claim green.".to_owned(),
        },
    ]
}

fn push(violations: &mut Vec<M5StorageCertificationViolation>, path: &str, message: &str) {
    violations.push(M5StorageCertificationViolation {
        path: path.to_owned(),
        message: message.to_owned(),
    });
}
