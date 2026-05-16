//! Mirror and manual-import baseline records for extension artifacts.
//!
//! This module evaluates extension artifacts that arrive from the primary
//! catalog, an approved mirror, an offline bundle, or a manually supplied
//! archive. The baseline preserves source visibility, publisher continuity,
//! permission, compatibility, lifecycle, and trust-claim metadata in one
//! inspectable record before install review or support export consumes it.

use serde::{Deserialize, Serialize};

use crate::install_review::{CompatibilityClaimClass, CompatibilityLabel};
use crate::manifest_baseline::{RedactionClass, SummaryFreshnessClass};
use crate::publication::PublicationContentAddress;
use crate::registry::{
    CatalogCompatibilityMetadata, CatalogLifecycleMetadata, CatalogPublisherContinuityMetadata,
    CatalogRegistrySourceClass,
};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`MirrorImportBaselineRecord`] payloads.
pub const MIRROR_IMPORT_BASELINE_RECORD_KIND: &str = "mirror_import_baseline_record";

/// Record-kind tag carried on serialized [`MirrorImportSupportExportRecord`] payloads.
pub const MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND: &str = "mirror_import_support_export_record";

/// Schema version for mirror and manual-import baseline payloads.
pub const MIRROR_IMPORT_BASELINE_SCHEMA_VERSION: u32 = 1;

/// Delivery route used by an extension artifact before install review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportRouteClass {
    /// Artifact was read from the primary catalog path.
    PrimaryCatalog,
    /// Artifact was read from an approved enterprise or private mirror.
    ApprovedMirror,
    /// Artifact was read from a sealed offline bundle.
    OfflineBundle,
    /// Artifact was supplied as a local manual archive.
    ManualArtifact,
}

/// Metadata family that must remain visible before import or install review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportDisclosureClass {
    /// Publisher continuity state is rendered.
    PublisherContinuity,
    /// Source lane and transport path are rendered.
    SourceLane,
    /// Artifact digest identity is rendered.
    ArtifactIdentity,
    /// Permission-manifest metadata is rendered.
    PermissionManifest,
    /// Compatibility metadata is rendered.
    CompatibilityMetadata,
    /// Lifecycle metadata is rendered.
    LifecycleMetadata,
    /// Trust-claim states and downgrades are rendered.
    TrustClaims,
    /// Revocation-snapshot state is rendered.
    RevocationSnapshot,
    /// Native install-review handoff is rendered.
    NativeInstallReview,
}

/// Trust claim evaluated during mirror or manual import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportTrustClaimClass {
    PublisherContinuity,
    ArtifactDigest,
    Signature,
    Attestation,
    RevocationSnapshot,
    CompatibilityEvidence,
    LifecycleMetadata,
    PermissionManifest,
}

/// State of one trust claim after source-specific import evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportTrustClaimStateClass {
    /// Claim is verified against the delivered source.
    Verified,
    /// Claim is preserved from the origin and quoted by the delivered source.
    PreservedFromOrigin,
    /// Claim is present but downgraded because the route cannot prove full trust.
    Downgraded,
    /// Claim is missing but does not by itself prove artifact mutation.
    Missing,
    /// Claim is refused and blocks import.
    Refused,
}

/// Reason a trust claim was downgraded, missed, or refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportDowngradeReasonClass {
    NoDowngrade,
    MirrorLimitation,
    ManualArtifactUnverified,
    DegradedCachedSnapshot,
    MissingEvidence,
    ArtifactIdentityMismatch,
    PolicyBlocked,
}

/// Decision emitted by mirror/manual import baseline evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportDecisionClass {
    /// Import can continue to install review without source-specific limits.
    ReadyForImport,
    /// Import can continue, but one or more trust claims is visibly downgraded.
    ReadyWithDowngradedTrustClaims,
    /// Import needs an admin or mirror-operator review before install review.
    AwaitingAdminReview,
    /// Import must not continue to install review.
    Refused,
}

/// Typed reason paired with [`MirrorImportDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportReasonClass {
    ReadyPrimaryCatalogBaseline,
    ReadyMirrorSemanticParity,
    ReadyManualImportUnverified,
    LimitedByMirrorTrustDowngrade,
    AwaitingAdminOutOfBandVerification,
    RefusedIdentityMissing,
    RefusedArtifactIdentityMismatch,
    RefusedPublisherContinuityMissing,
    RefusedPermissionMetadataMissing,
    RefusedCompatibilityMetadataMissing,
    RefusedLifecycleMetadataMissing,
    RefusedRequiredDisclosureMissing,
    RefusedTrustClaimBlocksInstall,
}

/// Export-facing explanation class for support and docs/help consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportSupportExplanationClass {
    Ready,
    Limited,
    AwaitingAdmin,
    Refused,
}

/// Action a support, headless, or review consumer may offer for the baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorImportActionOfferClass {
    OpenSourceDetails,
    OpenPublisherContinuity,
    OpenPermissionManifest,
    OpenCompatibilityReport,
    OpenLifecycleMetadata,
    OpenTrustClaimDetails,
    RequestAdminReview,
    OpenNativeInstallReview,
    ExportSupportPacket,
}

/// Permission metadata preserved across mirror or manual import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportPermissionMetadata {
    pub permission_manifest_ref: String,
    pub declared_permission_refs: Vec<String>,
    pub effective_permission_refs: Vec<String>,
    pub permission_delta_ref: String,
    pub freshness_class: SummaryFreshnessClass,
}

/// One evaluated trust claim in a mirror or manual-import baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportTrustClaimEntry {
    pub claim_class: MirrorImportTrustClaimClass,
    pub state_class: MirrorImportTrustClaimStateClass,
    pub source_registry_class: CatalogRegistrySourceClass,
    pub downgrade_reason_class: MirrorImportDowngradeReasonClass,
    pub evidence_refs: Vec<String>,
    pub rendered_label: String,
}

/// Input consumed to evaluate one mirror or manual-import baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportBaselineInput {
    pub baseline_id: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub display_name: String,
    pub route_class: MirrorImportRouteClass,
    pub origin_registry_source_class: CatalogRegistrySourceClass,
    pub delivered_registry_source_class: CatalogRegistrySourceClass,
    pub source_label: String,
    pub origin_catalog_ref: String,
    pub delivered_artifact_ref: String,
    pub registry_manifest_ref: String,
    pub compatibility_report_ref: String,
    pub lifecycle_metadata_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_archive_import_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_import_receipt_ref: Option<String>,
    pub content_address: PublicationContentAddress,
    pub origin_content_address: PublicationContentAddress,
    pub publisher: CatalogPublisherContinuityMetadata,
    pub lifecycle: CatalogLifecycleMetadata,
    pub permission: MirrorImportPermissionMetadata,
    pub compatibility: Vec<CatalogCompatibilityMetadata>,
    pub trust_claims: Vec<MirrorImportTrustClaimEntry>,
    pub rendered_disclosures: Vec<MirrorImportDisclosureClass>,
    pub generated_at: String,
}

/// Evaluated baseline consumed by install review, support export, and docs/help.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportBaselineRecord {
    pub record_kind: String,
    pub mirror_import_baseline_schema_version: u32,
    pub baseline_id: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub display_name: String,
    pub route_class: MirrorImportRouteClass,
    pub origin_registry_source_class: CatalogRegistrySourceClass,
    pub delivered_registry_source_class: CatalogRegistrySourceClass,
    pub source_label: String,
    pub origin_catalog_ref: String,
    pub delivered_artifact_ref: String,
    pub registry_manifest_ref: String,
    pub compatibility_report_ref: String,
    pub lifecycle_metadata_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_archive_import_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_import_receipt_ref: Option<String>,
    pub content_address: PublicationContentAddress,
    pub origin_content_address: PublicationContentAddress,
    pub publisher: CatalogPublisherContinuityMetadata,
    pub lifecycle: CatalogLifecycleMetadata,
    pub permission: MirrorImportPermissionMetadata,
    pub compatibility: Vec<CatalogCompatibilityMetadata>,
    pub trust_claims: Vec<MirrorImportTrustClaimEntry>,
    pub required_disclosures: Vec<MirrorImportDisclosureClass>,
    pub rendered_disclosures: Vec<MirrorImportDisclosureClass>,
    pub trust_claim_count: u32,
    pub downgraded_trust_claim_count: u32,
    pub refused_trust_claim_count: u32,
    pub artifact_identity_preserved: bool,
    pub publisher_continuity_preserved: bool,
    pub permission_metadata_preserved: bool,
    pub compatibility_metadata_preserved: bool,
    pub lifecycle_metadata_preserved: bool,
    pub source_visible_to_users_admins: bool,
    pub install_lane_continues: bool,
    pub decision_class: MirrorImportDecisionClass,
    pub reason_class: MirrorImportReasonClass,
    pub support_explanation_class: MirrorImportSupportExplanationClass,
    pub decision_summary: String,
    pub generated_at: String,
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support/export projection for a mirror-import baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorImportSupportExportRecord {
    pub record_kind: String,
    pub mirror_import_baseline_schema_version: u32,
    pub export_id: String,
    pub baseline_ref: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub route_class: MirrorImportRouteClass,
    pub origin_registry_source_class: CatalogRegistrySourceClass,
    pub delivered_registry_source_class: CatalogRegistrySourceClass,
    pub source_label: String,
    pub publisher_continuity_ref: String,
    pub permission_manifest_ref: String,
    pub compatibility_report_ref: String,
    pub lifecycle_metadata_ref: String,
    pub content_address: PublicationContentAddress,
    pub artifact_identity_preserved: bool,
    pub downgraded_trust_claim_count: u32,
    pub refused_trust_claim_count: u32,
    pub source_visible_to_users_admins: bool,
    pub install_lane_continues: bool,
    pub decision_class: MirrorImportDecisionClass,
    pub reason_class: MirrorImportReasonClass,
    pub support_explanation_class: MirrorImportSupportExplanationClass,
    pub offered_actions: Vec<MirrorImportActionOfferClass>,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by mirror-import baseline validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorImportFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl MirrorImportFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate one source artifact into the mirror/manual import baseline.
pub fn evaluate_mirror_import_baseline(
    input: MirrorImportBaselineInput,
) -> MirrorImportBaselineRecord {
    let required_disclosures = required_disclosures_for_mirror_import();
    let trust_claim_count = input.trust_claims.len() as u32;
    let downgraded_trust_claim_count = input
        .trust_claims
        .iter()
        .filter(|claim| claim.state_class == MirrorImportTrustClaimStateClass::Downgraded)
        .count() as u32;
    let refused_trust_claim_count = input
        .trust_claims
        .iter()
        .filter(|claim| claim.state_class == MirrorImportTrustClaimStateClass::Refused)
        .count() as u32;
    let artifact_identity_preserved = input.content_address == input.origin_content_address;
    let publisher_continuity_preserved = publisher_continuity_present(&input.publisher);
    let permission_metadata_preserved = permission_metadata_present(&input.permission);
    let compatibility_metadata_preserved = compatibility_metadata_present(&input.compatibility);
    let lifecycle_metadata_preserved = lifecycle_metadata_present(&input.lifecycle);
    let source_visible_to_users_admins = source_visible(&input);

    let (decision_class, reason_class, support_explanation_class, decision_summary) =
        decide_mirror_import(
            &input,
            &required_disclosures,
            artifact_identity_preserved,
            publisher_continuity_preserved,
            permission_metadata_preserved,
            compatibility_metadata_preserved,
            lifecycle_metadata_preserved,
            downgraded_trust_claim_count,
            refused_trust_claim_count,
        );
    let install_lane_continues = matches!(
        decision_class,
        MirrorImportDecisionClass::ReadyForImport
            | MirrorImportDecisionClass::ReadyWithDowngradedTrustClaims
    );

    MirrorImportBaselineRecord {
        record_kind: MIRROR_IMPORT_BASELINE_RECORD_KIND.to_string(),
        mirror_import_baseline_schema_version: MIRROR_IMPORT_BASELINE_SCHEMA_VERSION,
        baseline_id: input.baseline_id,
        extension_identity: input.extension_identity,
        extension_version: input.extension_version,
        package_id: input.package_id,
        display_name: input.display_name,
        route_class: input.route_class,
        origin_registry_source_class: input.origin_registry_source_class,
        delivered_registry_source_class: input.delivered_registry_source_class,
        source_label: input.source_label,
        origin_catalog_ref: input.origin_catalog_ref,
        delivered_artifact_ref: input.delivered_artifact_ref,
        registry_manifest_ref: input.registry_manifest_ref,
        compatibility_report_ref: input.compatibility_report_ref,
        lifecycle_metadata_ref: input.lifecycle_metadata_ref,
        local_archive_import_ref: input.local_archive_import_ref,
        manual_import_receipt_ref: input.manual_import_receipt_ref,
        content_address: input.content_address,
        origin_content_address: input.origin_content_address,
        publisher: input.publisher,
        lifecycle: input.lifecycle,
        permission: input.permission,
        compatibility: input.compatibility,
        trust_claims: input.trust_claims,
        required_disclosures,
        rendered_disclosures: input.rendered_disclosures,
        trust_claim_count,
        downgraded_trust_claim_count,
        refused_trust_claim_count,
        artifact_identity_preserved,
        publisher_continuity_preserved,
        permission_metadata_preserved,
        compatibility_metadata_preserved,
        lifecycle_metadata_preserved,
        source_visible_to_users_admins,
        install_lane_continues,
        decision_class,
        reason_class,
        support_explanation_class,
        decision_summary,
        generated_at: input.generated_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a baseline into the support/export consumer shape.
pub fn project_mirror_import_support_export(
    record: &MirrorImportBaselineRecord,
    export_id: &str,
) -> MirrorImportSupportExportRecord {
    let mut offered_actions = vec![
        MirrorImportActionOfferClass::OpenSourceDetails,
        MirrorImportActionOfferClass::OpenPublisherContinuity,
        MirrorImportActionOfferClass::OpenPermissionManifest,
        MirrorImportActionOfferClass::OpenCompatibilityReport,
        MirrorImportActionOfferClass::OpenLifecycleMetadata,
        MirrorImportActionOfferClass::OpenTrustClaimDetails,
        MirrorImportActionOfferClass::ExportSupportPacket,
    ];

    if record.install_lane_continues {
        offered_actions.push(MirrorImportActionOfferClass::OpenNativeInstallReview);
    } else {
        offered_actions.push(MirrorImportActionOfferClass::RequestAdminReview);
    }

    MirrorImportSupportExportRecord {
        record_kind: MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        mirror_import_baseline_schema_version: MIRROR_IMPORT_BASELINE_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        baseline_ref: record.baseline_id.clone(),
        extension_identity: record.extension_identity.clone(),
        extension_version: record.extension_version.clone(),
        package_id: record.package_id.clone(),
        route_class: record.route_class,
        origin_registry_source_class: record.origin_registry_source_class,
        delivered_registry_source_class: record.delivered_registry_source_class,
        source_label: record.source_label.clone(),
        publisher_continuity_ref: record.publisher.publisher_continuity_ref.clone(),
        permission_manifest_ref: record.permission.permission_manifest_ref.clone(),
        compatibility_report_ref: record.compatibility_report_ref.clone(),
        lifecycle_metadata_ref: record.lifecycle_metadata_ref.clone(),
        content_address: record.content_address.clone(),
        artifact_identity_preserved: record.artifact_identity_preserved,
        downgraded_trust_claim_count: record.downgraded_trust_claim_count,
        refused_trust_claim_count: record.refused_trust_claim_count,
        source_visible_to_users_admins: record.source_visible_to_users_admins,
        install_lane_continues: record.install_lane_continues,
        decision_class: record.decision_class,
        reason_class: record.reason_class,
        support_explanation_class: record.support_explanation_class,
        offered_actions,
        export_safe_summary: format!(
            "{} {} route={:?}; delivered={:?}; decision={:?}; downgraded_claims={}",
            record.extension_identity,
            record.extension_version,
            record.route_class,
            record.delivered_registry_source_class,
            record.decision_class,
            record.downgraded_trust_claim_count
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a mirror/manual import baseline.
pub fn validate_mirror_import_baseline_record(
    record: &MirrorImportBaselineRecord,
) -> Vec<MirrorImportFinding> {
    let mut findings = Vec::new();

    if record.record_kind != MIRROR_IMPORT_BASELINE_RECORD_KIND {
        findings.push(MirrorImportFinding::new(
            "mirror_import.record_kind_wrong",
            format!(
                "record_kind must be '{MIRROR_IMPORT_BASELINE_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.mirror_import_baseline_schema_version != MIRROR_IMPORT_BASELINE_SCHEMA_VERSION {
        findings.push(MirrorImportFinding::new(
            "mirror_import.schema_version_wrong",
            format!(
                "mirror_import_baseline_schema_version must be {MIRROR_IMPORT_BASELINE_SCHEMA_VERSION}; got {}",
                record.mirror_import_baseline_schema_version
            ),
        ));
    }
    if !record.baseline_id.starts_with("mirror_import_baseline:") {
        findings.push(MirrorImportFinding::new(
            "mirror_import.id_unprefixed",
            "baseline_id must start with 'mirror_import_baseline:'",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(MirrorImportFinding::new(
            "mirror_import.redaction_class_must_be_metadata_safe",
            "mirror import baseline records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    if let Some(missing) =
        first_missing_disclosure(&record.required_disclosures, &record.rendered_disclosures)
    {
        findings.push(MirrorImportFinding::new(
            "mirror_import.required_disclosure_missing",
            format!("required disclosure '{missing:?}' was not rendered"),
        ));
    }
    if record.trust_claim_count != record.trust_claims.len() as u32 {
        findings.push(MirrorImportFinding::new(
            "mirror_import.trust_claim_count_inconsistent",
            "trust_claim_count must equal trust_claims.len()",
        ));
    }
    if record.downgraded_trust_claim_count != downgraded_claim_count(&record.trust_claims) {
        findings.push(MirrorImportFinding::new(
            "mirror_import.downgraded_claim_count_inconsistent",
            "downgraded_trust_claim_count must equal downgraded trust claims",
        ));
    }
    if record.refused_trust_claim_count != refused_claim_count(&record.trust_claims) {
        findings.push(MirrorImportFinding::new(
            "mirror_import.refused_claim_count_inconsistent",
            "refused_trust_claim_count must equal refused trust claims",
        ));
    }
    if record.artifact_identity_preserved
        != (record.content_address == record.origin_content_address)
    {
        findings.push(MirrorImportFinding::new(
            "mirror_import.artifact_identity_preservation_inconsistent",
            "artifact_identity_preserved must compare delivered and origin content addresses",
        ));
    }
    if !record.artifact_identity_preserved
        && record.decision_class != MirrorImportDecisionClass::Refused
    {
        findings.push(MirrorImportFinding::new(
            "mirror_import.artifact_identity_mismatch_must_refuse",
            "artifact identity mismatch must refuse the import baseline",
        ));
    }
    if record.source_visible_to_users_admins != source_visible_from_record(record) {
        findings.push(MirrorImportFinding::new(
            "mirror_import.source_visibility_inconsistent",
            "source_visible_to_users_admins must reflect route, source classes, and source label",
        ));
    }
    if !route_matches_source(record.route_class, record.delivered_registry_source_class) {
        findings.push(MirrorImportFinding::new(
            "mirror_import.route_source_class_mismatch",
            "route_class must match delivered_registry_source_class",
        ));
    }
    for claim in &record.trust_claims {
        if claim.rendered_label.trim().is_empty() {
            findings.push(MirrorImportFinding::new(
                "mirror_import.trust_claim_label_missing",
                "each trust claim must carry a rendered_label",
            ));
        }
        if matches!(
            claim.state_class,
            MirrorImportTrustClaimStateClass::Verified
                | MirrorImportTrustClaimStateClass::PreservedFromOrigin
        ) && claim.evidence_refs.is_empty()
        {
            findings.push(MirrorImportFinding::new(
                "mirror_import.verified_claim_evidence_missing",
                "verified or preserved trust claims must cite evidence refs",
            ));
        }
    }
    if record.install_lane_continues
        != matches!(
            record.decision_class,
            MirrorImportDecisionClass::ReadyForImport
                | MirrorImportDecisionClass::ReadyWithDowngradedTrustClaims
        )
    {
        findings.push(MirrorImportFinding::new(
            "mirror_import.install_lane_continuity_inconsistent",
            "install_lane_continues must reflect the decision class",
        ));
    }

    findings
}

/// Validate structural invariants for a mirror-import support export.
pub fn validate_mirror_import_support_export_record(
    record: &MirrorImportSupportExportRecord,
) -> Vec<MirrorImportFinding> {
    let mut findings = Vec::new();

    if record.record_kind != MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.record_kind_wrong",
            format!(
                "record_kind must be '{MIRROR_IMPORT_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.mirror_import_baseline_schema_version != MIRROR_IMPORT_BASELINE_SCHEMA_VERSION {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.schema_version_wrong",
            format!(
                "mirror_import_baseline_schema_version must be {MIRROR_IMPORT_BASELINE_SCHEMA_VERSION}; got {}",
                record.mirror_import_baseline_schema_version
            ),
        ));
    }
    if !record
        .export_id
        .starts_with("mirror_import_support_export:")
    {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.id_unprefixed",
            "export_id must start with 'mirror_import_support_export:'",
        ));
    }
    if !record.baseline_ref.starts_with("mirror_import_baseline:") {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.baseline_ref_unprefixed",
            "baseline_ref must start with 'mirror_import_baseline:'",
        ));
    }
    if record.source_label.trim().is_empty() {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.source_label_required",
            "source_label must be visible in support export",
        ));
    }
    if record.export_safe_summary.trim().is_empty() {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.summary_required",
            "export_safe_summary must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(MirrorImportFinding::new(
            "mirror_import_support_export.redaction_class_must_be_metadata_safe",
            "support export records must emit RedactionClass::MetadataSafeDefault",
        ));
    }

    findings
}

fn required_disclosures_for_mirror_import() -> Vec<MirrorImportDisclosureClass> {
    vec![
        MirrorImportDisclosureClass::PublisherContinuity,
        MirrorImportDisclosureClass::SourceLane,
        MirrorImportDisclosureClass::ArtifactIdentity,
        MirrorImportDisclosureClass::PermissionManifest,
        MirrorImportDisclosureClass::CompatibilityMetadata,
        MirrorImportDisclosureClass::LifecycleMetadata,
        MirrorImportDisclosureClass::TrustClaims,
        MirrorImportDisclosureClass::RevocationSnapshot,
        MirrorImportDisclosureClass::NativeInstallReview,
    ]
}

#[allow(clippy::too_many_arguments)]
fn decide_mirror_import(
    input: &MirrorImportBaselineInput,
    required_disclosures: &[MirrorImportDisclosureClass],
    artifact_identity_preserved: bool,
    publisher_continuity_preserved: bool,
    permission_metadata_preserved: bool,
    compatibility_metadata_preserved: bool,
    lifecycle_metadata_preserved: bool,
    downgraded_trust_claim_count: u32,
    refused_trust_claim_count: u32,
) -> (
    MirrorImportDecisionClass,
    MirrorImportReasonClass,
    MirrorImportSupportExplanationClass,
    String,
) {
    if import_identity_missing(input) {
        return refused(
            MirrorImportReasonClass::RefusedIdentityMissing,
            "mirror import baseline is missing extension, source, artifact, or registry identity",
        );
    }
    if let Some(missing) =
        first_missing_disclosure(required_disclosures, &input.rendered_disclosures)
    {
        return refused(
            MirrorImportReasonClass::RefusedRequiredDisclosureMissing,
            format!("mirror import baseline did not render required disclosure '{missing:?}'"),
        );
    }
    if !artifact_identity_preserved {
        return refused(
            MirrorImportReasonClass::RefusedArtifactIdentityMismatch,
            "delivered artifact content address does not match the origin content address",
        );
    }
    if !publisher_continuity_preserved {
        return refused(
            MirrorImportReasonClass::RefusedPublisherContinuityMissing,
            "publisher continuity, signing key, or freshness metadata is missing",
        );
    }
    if !permission_metadata_preserved {
        return refused(
            MirrorImportReasonClass::RefusedPermissionMetadataMissing,
            "permission manifest, declared permissions, or effective-permission refs are missing",
        );
    }
    if !compatibility_metadata_preserved {
        return refused(
            MirrorImportReasonClass::RefusedCompatibilityMetadataMissing,
            "compatibility metadata does not preserve host, capability, target, and report refs",
        );
    }
    if !lifecycle_metadata_preserved {
        return refused(
            MirrorImportReasonClass::RefusedLifecycleMetadataMissing,
            "lifecycle state, source endpoint, support class, or lifecycle refs are missing",
        );
    }
    if refused_trust_claim_count > 0 {
        return refused(
            MirrorImportReasonClass::RefusedTrustClaimBlocksInstall,
            "one or more trust claims refused the import",
        );
    }
    if input.route_class == MirrorImportRouteClass::ManualArtifact
        && input.manual_import_receipt_ref.is_none()
    {
        return (
            MirrorImportDecisionClass::AwaitingAdminReview,
            MirrorImportReasonClass::AwaitingAdminOutOfBandVerification,
            MirrorImportSupportExplanationClass::AwaitingAdmin,
            "manual artifact import needs an out-of-band verification receipt before install review"
                .to_string(),
        );
    }
    if downgraded_trust_claim_count > 0 {
        let reason = if input.route_class == MirrorImportRouteClass::ManualArtifact {
            MirrorImportReasonClass::ReadyManualImportUnverified
        } else {
            MirrorImportReasonClass::LimitedByMirrorTrustDowngrade
        };
        return (
            MirrorImportDecisionClass::ReadyWithDowngradedTrustClaims,
            reason,
            MirrorImportSupportExplanationClass::Limited,
            "import can continue because semantic metadata is preserved and only named trust claims are downgraded"
                .to_string(),
        );
    }

    let reason = if input.route_class == MirrorImportRouteClass::PrimaryCatalog {
        MirrorImportReasonClass::ReadyPrimaryCatalogBaseline
    } else {
        MirrorImportReasonClass::ReadyMirrorSemanticParity
    };
    (
        MirrorImportDecisionClass::ReadyForImport,
        reason,
        MirrorImportSupportExplanationClass::Ready,
        "import preserves artifact identity, source visibility, publisher continuity, permissions, compatibility, and lifecycle metadata"
            .to_string(),
    )
}

fn refused(
    reason: MirrorImportReasonClass,
    summary: impl Into<String>,
) -> (
    MirrorImportDecisionClass,
    MirrorImportReasonClass,
    MirrorImportSupportExplanationClass,
    String,
) {
    (
        MirrorImportDecisionClass::Refused,
        reason,
        MirrorImportSupportExplanationClass::Refused,
        summary.into(),
    )
}

fn first_missing_disclosure(
    required: &[MirrorImportDisclosureClass],
    rendered: &[MirrorImportDisclosureClass],
) -> Option<MirrorImportDisclosureClass> {
    required
        .iter()
        .find(|required| !rendered.contains(required))
        .copied()
}

fn import_identity_missing(input: &MirrorImportBaselineInput) -> bool {
    input.baseline_id.trim().is_empty()
        || input.extension_identity.trim().is_empty()
        || input.extension_version.trim().is_empty()
        || input.package_id.trim().is_empty()
        || input.display_name.trim().is_empty()
        || input.source_label.trim().is_empty()
        || input.origin_catalog_ref.trim().is_empty()
        || input.delivered_artifact_ref.trim().is_empty()
        || input.registry_manifest_ref.trim().is_empty()
        || input.compatibility_report_ref.trim().is_empty()
        || input.lifecycle_metadata_ref.trim().is_empty()
        || !route_matches_source(input.route_class, input.delivered_registry_source_class)
}

fn publisher_continuity_present(publisher: &CatalogPublisherContinuityMetadata) -> bool {
    let fresh_enough = matches!(
        publisher.freshness_class,
        SummaryFreshnessClass::AuthoritativeLive
            | SummaryFreshnessClass::WarmCached
            | SummaryFreshnessClass::DegradedCached
    );
    !publisher.publisher_id.trim().is_empty()
        && !publisher.publisher_display_label.trim().is_empty()
        && !publisher.publisher_continuity_ref.trim().is_empty()
        && fresh_enough
        && !publisher.active_signing_key_refs.is_empty()
}

fn permission_metadata_present(permission: &MirrorImportPermissionMetadata) -> bool {
    let fresh_enough = matches!(
        permission.freshness_class,
        SummaryFreshnessClass::AuthoritativeLive
            | SummaryFreshnessClass::WarmCached
            | SummaryFreshnessClass::DegradedCached
    );
    !permission.permission_manifest_ref.trim().is_empty()
        && !permission.declared_permission_refs.is_empty()
        && !permission.effective_permission_refs.is_empty()
        && !permission.permission_delta_ref.trim().is_empty()
        && fresh_enough
}

fn compatibility_metadata_present(compatibility: &[CatalogCompatibilityMetadata]) -> bool {
    !compatibility.is_empty()
        && compatibility.iter().all(|row| {
            !matches!(
                row.compatibility_claim_class,
                CompatibilityClaimClass::IncompatibleBlockedOnPolicy
            ) && !matches!(row.rendered_label, CompatibilityLabel::Unsupported)
                && !row.host_contract_family_refs.is_empty()
                && !row.capability_world_refs.is_empty()
                && !row.target_platforms.is_empty()
                && row
                    .caveat_labels
                    .iter()
                    .any(|label| label.starts_with("compatibility_report:"))
        })
}

fn lifecycle_metadata_present(lifecycle: &CatalogLifecycleMetadata) -> bool {
    !lifecycle.source_endpoint_ref.trim().is_empty()
        && !lifecycle.support_class.trim().is_empty()
        && !lifecycle.lifecycle_event_refs.is_empty()
}

fn source_visible(input: &MirrorImportBaselineInput) -> bool {
    !input.source_label.trim().is_empty()
        && route_matches_source(input.route_class, input.delivered_registry_source_class)
}

fn source_visible_from_record(record: &MirrorImportBaselineRecord) -> bool {
    !record.source_label.trim().is_empty()
        && route_matches_source(record.route_class, record.delivered_registry_source_class)
}

fn route_matches_source(
    route_class: MirrorImportRouteClass,
    source_class: CatalogRegistrySourceClass,
) -> bool {
    match route_class {
        MirrorImportRouteClass::PrimaryCatalog => matches!(
            source_class,
            CatalogRegistrySourceClass::PublicRegistry
                | CatalogRegistrySourceClass::PrivateRegistry
        ),
        MirrorImportRouteClass::ApprovedMirror => {
            source_class == CatalogRegistrySourceClass::ApprovedMirror
        }
        MirrorImportRouteClass::OfflineBundle => {
            source_class == CatalogRegistrySourceClass::OfflineBundle
        }
        MirrorImportRouteClass::ManualArtifact => matches!(
            source_class,
            CatalogRegistrySourceClass::LocalArchive
                | CatalogRegistrySourceClass::QuarantinedLocalCopy
        ),
    }
}

fn downgraded_claim_count(claims: &[MirrorImportTrustClaimEntry]) -> u32 {
    claims
        .iter()
        .filter(|claim| claim.state_class == MirrorImportTrustClaimStateClass::Downgraded)
        .count() as u32
}

fn refused_claim_count(claims: &[MirrorImportTrustClaimEntry]) -> u32 {
    claims
        .iter()
        .filter(|claim| claim.state_class == MirrorImportTrustClaimStateClass::Refused)
        .count() as u32
}
