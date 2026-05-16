//! Extension publication pipeline records for signed, provenance-bound,
//! rollback-safe registry promotion.
//!
//! This module owns the bounded beta publication lane that turns an
//! authored extension manifest plus a built artifact into one inspectable
//! record set. The pipeline binds version metadata, compatibility
//! metadata, signer metadata, provenance metadata, monotone channel
//! promotion, a rollback plan, and transactional catalog-write guards into
//! one [`ExtensionPublicationPipelineRecord`].
//!
//! The first consumers are the headless publication CLI at
//! [`/tools/extensions/m3/publish_extension.py`](../../../../tools/extensions/m3/publish_extension.py),
//! the checked-in packet under
//! [`/artifacts/extensions/m3/publication_pipeline/`](../../../../artifacts/extensions/m3/publication_pipeline/),
//! and the reviewer-facing guide at
//! [`/docs/extensions/m3/publication_pipeline_beta.md`](../../../../docs/extensions/m3/publication_pipeline_beta.md).

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::RedactionClass;

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`ExtensionPublicationPipelineRecord`] payloads.
pub const EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND: &str =
    "extension_publication_pipeline_record";

/// Record-kind tag carried on serialized
/// [`ExtensionPublicationSupportExportRecord`] payloads.
pub const EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_publication_support_export_record";

/// Schema version for the extension publication pipeline payloads.
///
/// Bumped on breaking payload changes. Additive enum members or optional
/// fields are additive-minor and require boundary consumers to preserve
/// unknown fields.
pub const EXTENSION_PUBLICATION_SCHEMA_VERSION: u32 = 1;

/// Content address for an artifact or metadata envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationContentAddress {
    pub digest_algorithm: String,
    pub digest_hex: String,
    pub digest_size_bytes: u64,
}

/// Closed signature posture for the publication lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationSignatureClass {
    /// Artifact carries a publisher signature.
    PublisherSignature,
    /// Artifact carries an attestation bundle.
    AttestationBundle,
    /// Artifact carries both a publisher signature and attestation bundle.
    DualSignedPublisherAndAttestation,
    /// Unsigned artifacts are represented only to make denial drills explicit.
    UnsignedDeniedOnPolicy,
}

/// Closed provenance posture for the publication lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationProvenanceClass {
    /// Provenance was produced by a verified build identity.
    VerifiedBuildProvenance,
    /// Provenance was asserted by a registered publisher and needs review.
    PublisherAssertedProvenance,
    /// Provenance is missing and the row must not publish.
    MissingProvenance,
}

/// Closed channel lane vocabulary for publication promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationChannelClass {
    Quarantine,
    Approved,
    Production,
}

/// Closed transaction-write posture for catalog mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationTransactionWriteClass {
    /// Artifacts and sidecars are verified first, then the catalog pointer
    /// swaps atomically.
    AtomicCatalogSwap,
    /// The packet is a dry run and mutates no catalog state.
    DryRunOnly,
    /// Partial writes are represented only to make denial drills explicit.
    UnsafePartialWrites,
}

/// Closed publication decision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecisionClass {
    /// The artifact is signed, provenance-bound, compatible, and safe to promote.
    ReadyForPromotion,
    /// The artifact is valid but remains held outside the production lane.
    HeldForReview,
    /// The publication is structurally unsafe and must not mutate catalog state.
    Refused,
}

/// Closed publication reason class paired with [`PublicationDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationReasonClass {
    ReadySignedProvenanceRollbackSafe,
    HeldNoProductionPromotionRequested,
    RefusedPublicationIdUnprefixed,
    RefusedVersionMetadataMissing,
    RefusedArtifactMetadataMissing,
    RefusedSignerMissing,
    RefusedUnsignedArtifact,
    RefusedProvenanceMissing,
    RefusedCompatibilityMissing,
    RefusedPromotionIdentityMutation,
    RefusedPromotionEvidenceMissing,
    RefusedRollbackTargetMissing,
    RefusedTransactionalWriteGuardMissing,
    RefusedOrphanedRevocationState,
    RefusedRedactionClassNotMetadataSafe,
}

/// Version and compatibility metadata extracted from an extension manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationVersionMetadata {
    pub extension_identity: String,
    pub package_id: String,
    pub publisher_id: String,
    pub extension_version: String,
    pub manifest_schema_version: u32,
    pub sdk_line_id: String,
    pub sdk_line_semver: String,
    pub aureline_version_min: String,
    pub aureline_version_max: String,
    pub support_class: String,
    pub bridge_state: String,
}

/// Artifact metadata produced by packaging before registry admission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationArtifactMetadata {
    pub artifact_ref: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub content_address: PublicationContentAddress,
    pub registry_manifest_ref: String,
    pub permission_manifest_ref: String,
    pub runtime_contract_ref: String,
}

/// Signer metadata attached to a built artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSignerMetadata {
    pub signer_ref: String,
    pub signer_key_fingerprint: String,
    pub signature_ref: String,
    pub signature_class: PublicationSignatureClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transparency_log_ref: Option<String>,
    pub signed_content_address: PublicationContentAddress,
    pub signed_at: String,
}

/// Provenance metadata attached to a built artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationProvenanceMetadata {
    pub provenance_ref: String,
    pub provenance_class: PublicationProvenanceClass,
    pub builder_id: String,
    pub source_manifest_ref: String,
    pub source_revision_ref: String,
    pub build_run_ref: String,
    pub conformance_report_ref: String,
    pub sdk_release_bundle_ref: String,
    pub subject_content_address: PublicationContentAddress,
    pub generated_at: String,
}

/// Compatibility metadata carried with the publication packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationCompatibilityMetadata {
    pub compatibility_report_ref: String,
    pub host_contract_family_refs: Vec<String>,
    pub capability_world_refs: Vec<String>,
    pub target_platforms: Vec<String>,
    pub support_class: String,
    pub bridge_state: String,
}

/// One monotone channel-promotion step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationPromotionStep {
    pub promotion_step_id: String,
    pub registry_manifest_ref: String,
    pub source_channel_class: PublicationChannelClass,
    pub target_channel_class: PublicationChannelClass,
    pub subject_content_address: PublicationContentAddress,
    pub subject_signature_ref: String,
    pub preserves_artifact_identity: bool,
    pub required_evidence_refs: Vec<String>,
    pub approver_refs: Vec<String>,
}

/// Rollback plan bound to a publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationRollbackPlan {
    pub rollback_plan_id: String,
    pub previous_extension_version: String,
    pub previous_registry_manifest_ref: String,
    pub previous_content_address: PublicationContentAddress,
    pub rollback_manifest_ref: String,
    pub preserves_prior_installable_artifact: bool,
    pub rollback_does_not_delete_prior_artifact: bool,
}

/// Transactional catalog-write guard for the publication lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationFailureAtomicityGuard {
    pub staging_catalog_ref: String,
    pub target_catalog_ref: String,
    pub transaction_write_class: PublicationTransactionWriteClass,
    pub writes_catalog_after_artifacts_verified: bool,
    pub revocation_state_requires_catalog_commit: bool,
    pub orphaned_revocation_state_count: u32,
    pub retry_idempotency_key: String,
}

/// Input to evaluate an extension publication packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPublicationPipelineInput {
    pub publication_id: String,
    pub version_metadata: PublicationVersionMetadata,
    pub artifact_metadata: PublicationArtifactMetadata,
    pub signer_metadata: PublicationSignerMetadata,
    pub provenance_metadata: PublicationProvenanceMetadata,
    pub compatibility_metadata: PublicationCompatibilityMetadata,
    pub promotion_steps: Vec<PublicationPromotionStep>,
    pub rollback_plan: PublicationRollbackPlan,
    pub failure_atomicity_guard: PublicationFailureAtomicityGuard,
    pub generated_at: String,
}

/// One bundled, inspectable extension publication pipeline record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPublicationPipelineRecord {
    pub record_kind: String,
    pub extension_publication_schema_version: u32,
    pub publication_id: String,
    pub version_metadata: PublicationVersionMetadata,
    pub artifact_metadata: PublicationArtifactMetadata,
    pub signer_metadata: PublicationSignerMetadata,
    pub provenance_metadata: PublicationProvenanceMetadata,
    pub compatibility_metadata: PublicationCompatibilityMetadata,
    pub promotion_steps: Vec<PublicationPromotionStep>,
    pub rollback_plan: PublicationRollbackPlan,
    pub failure_atomicity_guard: PublicationFailureAtomicityGuard,
    pub promotion_step_count: u32,
    pub required_evidence_ref_count: u32,
    pub approver_ref_count: u32,
    pub preserves_prior_installable_artifact: bool,
    pub transactional_catalog_update: bool,
    pub decision_class: PublicationDecisionClass,
    pub reason_class: PublicationReasonClass,
    pub decision_summary: String,
    pub generated_at: String,
    pub redaction_class: RedactionClass,
}

/// First consumer projection: a metadata-safe support export that quotes
/// publication truth without raw artifact bytes or signing material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPublicationSupportExportRecord {
    pub record_kind: String,
    pub extension_publication_schema_version: u32,
    pub export_id: String,
    pub publication_ref: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub artifact_ref: String,
    pub registry_manifest_ref: String,
    pub signature_ref: String,
    pub provenance_ref: String,
    pub compatibility_report_ref: String,
    pub rollback_manifest_ref: String,
    pub decision_class: PublicationDecisionClass,
    pub reason_class: PublicationReasonClass,
    pub blocks_catalog_mutation: bool,
    pub rollback_available: bool,
    pub transactional_catalog_update: bool,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by publication-pipeline validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationPipelineFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl PublicationPipelineFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate one [`ExtensionPublicationPipelineInput`] into a typed
/// [`ExtensionPublicationPipelineRecord`].
pub fn evaluate_extension_publication_pipeline(
    input: ExtensionPublicationPipelineInput,
) -> ExtensionPublicationPipelineRecord {
    let ExtensionPublicationPipelineInput {
        publication_id,
        version_metadata,
        artifact_metadata,
        signer_metadata,
        provenance_metadata,
        compatibility_metadata,
        promotion_steps,
        rollback_plan,
        failure_atomicity_guard,
        generated_at,
    } = input;

    let promotion_step_count = promotion_steps.len() as u32;
    let required_evidence_ref_count = promotion_steps
        .iter()
        .map(|step| step.required_evidence_refs.len() as u32)
        .sum();
    let approver_ref_count = promotion_steps
        .iter()
        .map(|step| step.approver_refs.len() as u32)
        .sum();
    let preserves_prior_installable_artifact = rollback_plan.preserves_prior_installable_artifact
        && rollback_plan.rollback_does_not_delete_prior_artifact;
    let transactional_catalog_update = matches!(
        failure_atomicity_guard.transaction_write_class,
        PublicationTransactionWriteClass::AtomicCatalogSwap
            | PublicationTransactionWriteClass::DryRunOnly
    ) && failure_atomicity_guard
        .writes_catalog_after_artifacts_verified
        && failure_atomicity_guard.revocation_state_requires_catalog_commit
        && failure_atomicity_guard.orphaned_revocation_state_count == 0;

    let (decision_class, reason_class, decision_summary) = decide_publication_pipeline(
        &publication_id,
        &version_metadata,
        &artifact_metadata,
        &signer_metadata,
        &provenance_metadata,
        &compatibility_metadata,
        &promotion_steps,
        &rollback_plan,
        &failure_atomicity_guard,
    );

    ExtensionPublicationPipelineRecord {
        record_kind: EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND.to_string(),
        extension_publication_schema_version: EXTENSION_PUBLICATION_SCHEMA_VERSION,
        publication_id,
        version_metadata,
        artifact_metadata,
        signer_metadata,
        provenance_metadata,
        compatibility_metadata,
        promotion_steps,
        rollback_plan,
        failure_atomicity_guard,
        promotion_step_count,
        required_evidence_ref_count,
        approver_ref_count,
        preserves_prior_installable_artifact,
        transactional_catalog_update,
        decision_class,
        reason_class,
        decision_summary,
        generated_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a publication pipeline record into the first consumer support export.
pub fn project_extension_publication_support_export(
    record: &ExtensionPublicationPipelineRecord,
    export_id: &str,
) -> ExtensionPublicationSupportExportRecord {
    let blocks_catalog_mutation = record.decision_class == PublicationDecisionClass::Refused;
    let rollback_available = record.preserves_prior_installable_artifact
        && !record
            .rollback_plan
            .previous_registry_manifest_ref
            .trim()
            .is_empty();
    let export_safe_summary = format!(
        "{} {} decision={:?}; signer={}; provenance={}; promotion_steps={}; rollback={}",
        record.version_metadata.extension_identity,
        record.version_metadata.extension_version,
        record.decision_class,
        record.signer_metadata.signer_ref,
        record.provenance_metadata.provenance_ref,
        record.promotion_step_count,
        rollback_available,
    );

    ExtensionPublicationSupportExportRecord {
        record_kind: EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        extension_publication_schema_version: EXTENSION_PUBLICATION_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        publication_ref: record.publication_id.clone(),
        extension_identity: record.version_metadata.extension_identity.clone(),
        extension_version: record.version_metadata.extension_version.clone(),
        artifact_ref: record.artifact_metadata.artifact_ref.clone(),
        registry_manifest_ref: record.artifact_metadata.registry_manifest_ref.clone(),
        signature_ref: record.signer_metadata.signature_ref.clone(),
        provenance_ref: record.provenance_metadata.provenance_ref.clone(),
        compatibility_report_ref: record
            .compatibility_metadata
            .compatibility_report_ref
            .clone(),
        rollback_manifest_ref: record.rollback_plan.rollback_manifest_ref.clone(),
        decision_class: record.decision_class,
        reason_class: record.reason_class,
        blocks_catalog_mutation,
        rollback_available,
        transactional_catalog_update: record.transactional_catalog_update,
        export_safe_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate the structural invariants of an
/// [`ExtensionPublicationPipelineRecord`].
pub fn validate_extension_publication_pipeline_record(
    record: &ExtensionPublicationPipelineRecord,
) -> Vec<PublicationPipelineFinding> {
    let mut findings = Vec::new();
    if record.record_kind != EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_PUBLICATION_PIPELINE_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.extension_publication_schema_version != EXTENSION_PUBLICATION_SCHEMA_VERSION {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.schema_version_wrong",
            format!(
                "extension_publication_schema_version must be {EXTENSION_PUBLICATION_SCHEMA_VERSION}; got {}",
                record.extension_publication_schema_version
            ),
        ));
    }
    if !record.publication_id.starts_with("extension_publication:") {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.id_unprefixed",
            "publication_id must start with 'extension_publication:'",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.redaction_class_must_be_metadata_safe",
            "publication pipeline records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    if record.promotion_step_count != record.promotion_steps.len() as u32 {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.promotion_step_count_inconsistent",
            "promotion_step_count must equal promotion_steps.len()",
        ));
    }
    let evidence_count: u32 = record
        .promotion_steps
        .iter()
        .map(|step| step.required_evidence_refs.len() as u32)
        .sum();
    if record.required_evidence_ref_count != evidence_count {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.required_evidence_ref_count_inconsistent",
            "required_evidence_ref_count must equal promotion-step evidence refs",
        ));
    }
    let approver_count: u32 = record
        .promotion_steps
        .iter()
        .map(|step| step.approver_refs.len() as u32)
        .sum();
    if record.approver_ref_count != approver_count {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.approver_ref_count_inconsistent",
            "approver_ref_count must equal promotion-step approver refs",
        ));
    }
    let expected_prior = record.rollback_plan.preserves_prior_installable_artifact
        && record.rollback_plan.rollback_does_not_delete_prior_artifact;
    if record.preserves_prior_installable_artifact != expected_prior {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.rollback_preservation_inconsistent",
            "preserves_prior_installable_artifact must reflect the rollback plan flags",
        ));
    }
    let expected_transactional = matches!(
        record.failure_atomicity_guard.transaction_write_class,
        PublicationTransactionWriteClass::AtomicCatalogSwap
            | PublicationTransactionWriteClass::DryRunOnly
    ) && record
        .failure_atomicity_guard
        .writes_catalog_after_artifacts_verified
        && record
            .failure_atomicity_guard
            .revocation_state_requires_catalog_commit
        && record
            .failure_atomicity_guard
            .orphaned_revocation_state_count
            == 0;
    if record.transactional_catalog_update != expected_transactional {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication.transactional_catalog_update_inconsistent",
            "transactional_catalog_update must reflect the atomicity guard",
        ));
    }
    findings
}

/// Validate the structural invariants of an
/// [`ExtensionPublicationSupportExportRecord`].
pub fn validate_extension_publication_support_export_record(
    record: &ExtensionPublicationSupportExportRecord,
) -> Vec<PublicationPipelineFinding> {
    let mut findings = Vec::new();
    if record.record_kind != EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication_support_export.record_kind_wrong",
            format!(
                "record_kind must be '{EXTENSION_PUBLICATION_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.extension_publication_schema_version != EXTENSION_PUBLICATION_SCHEMA_VERSION {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication_support_export.schema_version_wrong",
            format!(
                "extension_publication_schema_version must be {EXTENSION_PUBLICATION_SCHEMA_VERSION}; got {}",
                record.extension_publication_schema_version
            ),
        ));
    }
    if !record
        .export_id
        .starts_with("extension_publication_support_export:")
    {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication_support_export.id_unprefixed",
            "export_id must start with 'extension_publication_support_export:'",
        ));
    }
    if !record.publication_ref.starts_with("extension_publication:") {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication_support_export.publication_ref_unprefixed",
            "publication_ref must start with 'extension_publication:'",
        ));
    }
    if record.export_safe_summary.trim().is_empty() {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication_support_export.summary_required",
            "export_safe_summary must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(PublicationPipelineFinding::new(
            "extension_publication_support_export.redaction_class_must_be_metadata_safe",
            "support export records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    findings
}

fn decide_publication_pipeline(
    publication_id: &str,
    version: &PublicationVersionMetadata,
    artifact: &PublicationArtifactMetadata,
    signer: &PublicationSignerMetadata,
    provenance: &PublicationProvenanceMetadata,
    compatibility: &PublicationCompatibilityMetadata,
    promotion_steps: &[PublicationPromotionStep],
    rollback: &PublicationRollbackPlan,
    atomicity: &PublicationFailureAtomicityGuard,
) -> (PublicationDecisionClass, PublicationReasonClass, String) {
    if !publication_id.starts_with("extension_publication:") {
        return refused(
            PublicationReasonClass::RefusedPublicationIdUnprefixed,
            "publication id is not in the extension publication namespace",
        );
    }
    if version.extension_identity.trim().is_empty()
        || version.package_id.trim().is_empty()
        || version.publisher_id.trim().is_empty()
        || version.extension_version.trim().is_empty()
        || version.sdk_line_id.trim().is_empty()
        || version.sdk_line_semver.trim().is_empty()
        || version.aureline_version_min.trim().is_empty()
        || version.aureline_version_max.trim().is_empty()
        || version.support_class.trim().is_empty()
        || version.bridge_state.trim().is_empty()
    {
        return refused(
            PublicationReasonClass::RefusedVersionMetadataMissing,
            "version metadata is incomplete",
        );
    }
    if artifact.artifact_ref.trim().is_empty()
        || artifact.registry_manifest_ref.trim().is_empty()
        || artifact.permission_manifest_ref.trim().is_empty()
        || artifact.runtime_contract_ref.trim().is_empty()
        || artifact.extension_identity != version.extension_identity
        || artifact.extension_version != version.extension_version
        || content_address_missing(&artifact.content_address)
    {
        return refused(
            PublicationReasonClass::RefusedArtifactMetadataMissing,
            "artifact metadata does not match version metadata or lacks a content address",
        );
    }
    if signer.signer_ref.trim().is_empty()
        || signer.signer_key_fingerprint.trim().is_empty()
        || signer.signature_ref.trim().is_empty()
        || content_address_missing(&signer.signed_content_address)
    {
        return refused(
            PublicationReasonClass::RefusedSignerMissing,
            "signer metadata is incomplete",
        );
    }
    if signer.signature_class == PublicationSignatureClass::UnsignedDeniedOnPolicy {
        return refused(
            PublicationReasonClass::RefusedUnsignedArtifact,
            "unsigned artifacts cannot publish",
        );
    }
    if signer.signed_content_address != artifact.content_address {
        return refused(
            PublicationReasonClass::RefusedPromotionIdentityMutation,
            "signer content address differs from artifact content address",
        );
    }
    if provenance.provenance_ref.trim().is_empty()
        || provenance.builder_id.trim().is_empty()
        || provenance.source_manifest_ref.trim().is_empty()
        || provenance.source_revision_ref.trim().is_empty()
        || provenance.build_run_ref.trim().is_empty()
        || provenance.conformance_report_ref.trim().is_empty()
        || provenance.sdk_release_bundle_ref.trim().is_empty()
        || provenance.subject_content_address != artifact.content_address
        || provenance.provenance_class == PublicationProvenanceClass::MissingProvenance
    {
        return refused(
            PublicationReasonClass::RefusedProvenanceMissing,
            "provenance metadata is missing or does not bind to the artifact content address",
        );
    }
    if compatibility.compatibility_report_ref.trim().is_empty()
        || compatibility.host_contract_family_refs.is_empty()
        || compatibility.capability_world_refs.is_empty()
        || compatibility.target_platforms.is_empty()
        || compatibility.support_class.trim().is_empty()
        || compatibility.bridge_state.trim().is_empty()
    {
        return refused(
            PublicationReasonClass::RefusedCompatibilityMissing,
            "compatibility metadata is incomplete",
        );
    }
    if promotion_steps.is_empty()
        || promotion_steps.iter().any(|step| {
            step.promotion_step_id.trim().is_empty()
                || step.registry_manifest_ref.trim().is_empty()
                || step.subject_signature_ref != signer.signature_ref
                || step.subject_content_address != artifact.content_address
                || !step.preserves_artifact_identity
                || !is_monotone_promotion(step.source_channel_class, step.target_channel_class)
        })
    {
        return refused(
            PublicationReasonClass::RefusedPromotionIdentityMutation,
            "promotion steps must be monotone and preserve artifact identity",
        );
    }
    if promotion_steps
        .iter()
        .any(|step| step.required_evidence_refs.is_empty() || step.approver_refs.is_empty())
    {
        return refused(
            PublicationReasonClass::RefusedPromotionEvidenceMissing,
            "promotion steps require evidence and approver refs",
        );
    }
    let production_requested = promotion_steps
        .iter()
        .any(|step| step.target_channel_class == PublicationChannelClass::Production);
    if rollback.rollback_plan_id.trim().is_empty()
        || rollback.previous_extension_version.trim().is_empty()
        || rollback.previous_registry_manifest_ref.trim().is_empty()
        || rollback.rollback_manifest_ref.trim().is_empty()
        || content_address_missing(&rollback.previous_content_address)
        || !rollback.preserves_prior_installable_artifact
        || !rollback.rollback_does_not_delete_prior_artifact
    {
        return refused(
            PublicationReasonClass::RefusedRollbackTargetMissing,
            "rollback plan must preserve a previous installable artifact",
        );
    }
    if atomicity.staging_catalog_ref.trim().is_empty()
        || atomicity.target_catalog_ref.trim().is_empty()
        || atomicity.retry_idempotency_key.trim().is_empty()
        || atomicity.transaction_write_class
            == PublicationTransactionWriteClass::UnsafePartialWrites
        || !atomicity.writes_catalog_after_artifacts_verified
        || !atomicity.revocation_state_requires_catalog_commit
    {
        return refused(
            PublicationReasonClass::RefusedTransactionalWriteGuardMissing,
            "publication must use a guarded catalog transaction",
        );
    }
    if atomicity.orphaned_revocation_state_count != 0 {
        return refused(
            PublicationReasonClass::RefusedOrphanedRevocationState,
            "publication leaves orphaned revocation state",
        );
    }
    if production_requested {
        (
            PublicationDecisionClass::ReadyForPromotion,
            PublicationReasonClass::ReadySignedProvenanceRollbackSafe,
            "artifact is signed, provenance-bound, compatible, transaction-safe, and rollback-ready"
                .to_string(),
        )
    } else {
        (
            PublicationDecisionClass::HeldForReview,
            PublicationReasonClass::HeldNoProductionPromotionRequested,
            "artifact is valid but no production promotion was requested".to_string(),
        )
    }
}

fn refused(
    reason: PublicationReasonClass,
    summary: impl Into<String>,
) -> (PublicationDecisionClass, PublicationReasonClass, String) {
    (PublicationDecisionClass::Refused, reason, summary.into())
}

fn content_address_missing(address: &PublicationContentAddress) -> bool {
    address.digest_algorithm.trim().is_empty()
        || address.digest_hex.trim().is_empty()
        || address.digest_size_bytes == 0
}

fn is_monotone_promotion(source: PublicationChannelClass, target: PublicationChannelClass) -> bool {
    matches!(
        (source, target),
        (
            PublicationChannelClass::Quarantine,
            PublicationChannelClass::Approved
        ) | (
            PublicationChannelClass::Approved,
            PublicationChannelClass::Production
        ) | (
            PublicationChannelClass::Production,
            PublicationChannelClass::Production
        )
    )
}
