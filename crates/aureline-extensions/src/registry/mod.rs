//! Registry catalog descriptors for moderated, mirrorable extension rows.
//!
//! This module joins publication metadata, publisher continuity,
//! moderation state, revocation posture, and mirror-compatible trust
//! labels into one [`CatalogDescriptorRecord`]. Discovery, install review,
//! support export, and mirror import consumers read this descriptor instead
//! of reconstructing registry truth from install links or listing copy.

use serde::{Deserialize, Serialize};

use crate::install_review::{BridgeStateClass, CompatibilityClaimClass, CompatibilityLabel};
use crate::manifest_baseline::{PublisherTrustTierClass, RedactionClass, SummaryFreshnessClass};
use crate::publication::{PublicationChannelClass, PublicationContentAddress};
use crate::review_alpha::{PublisherContinuityStateClass, RevocationStateClass};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`CatalogDescriptorRecord`] payloads.
pub const CATALOG_DESCRIPTOR_RECORD_KIND: &str = "catalog_descriptor_record";

/// Record-kind tag carried on serialized [`CatalogDescriptorSupportExportRecord`] payloads.
pub const CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "catalog_descriptor_support_export_record";

/// Schema version for catalog-descriptor payloads.
pub const CATALOG_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Registry or artifact source class preserved by catalog descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogRegistrySourceClass {
    /// Public Aureline extension registry.
    PublicRegistry,
    /// Approved private or enterprise mirror.
    ApprovedMirror,
    /// Private extension registry.
    PrivateRegistry,
    /// Sealed offline bundle import.
    OfflineBundle,
    /// Local archive or side-loaded package.
    LocalArchive,
    /// Local copy retained only for quarantine or forensic continuity.
    QuarantinedLocalCopy,
}

/// Lifecycle state rendered for a catalog row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogLifecycleStateClass {
    /// Row is visible only as a staged or review-only catalog row.
    Staged,
    /// Row is approved for the declared channel.
    Approved,
    /// Row remains installable or inspectable with constrained capability.
    Limited,
    /// Row is retained for migration or compatibility but no longer preferred.
    Deprecated,
    /// Row is revoked and cannot be installed or updated.
    Revoked,
    /// Row is quarantined pending repair, review, or removal.
    Quarantined,
}

/// Moderation state assigned to a catalog row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogModerationStateClass {
    /// Row is awaiting moderation review.
    PendingReview,
    /// Row is admitted by moderation.
    Admitted,
    /// Row is admitted with documented limits.
    Limited,
    /// Row is revoked by registry, policy, or publisher authority.
    Revoked,
    /// Row is quarantined pending trust review.
    Quarantined,
}

/// Trust-badge inheritance rule carried into mirrorable catalog metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogTrustBadgeInheritanceRuleClass {
    /// Catalog row inherits the origin publisher tier.
    InheritsOriginTier,
    /// Private-registry rows cap rendered trust at organisational.
    CappedAtOrganisationalOnPrivateRegistry,
    /// Approved-mirror and offline rows cap rendered trust at community.
    CappedAtCommunityOnApprovedMirror,
    /// Local archive rows cap rendered trust at unverified.
    CappedAtUnverifiedOnLocalArchive,
    /// Quarantined rows cannot inherit trust from the origin.
    QuarantinedCannotInherit,
}

/// Revocation snapshot freshness carried by catalog descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogRevocationSnapshotAgeClass {
    /// Live registry or mirror snapshot is fresh.
    Fresh,
    /// Cached snapshot is warm enough for review.
    WarmCached,
    /// Cached snapshot is degraded but still inspectable.
    DegradedCached,
    /// Snapshot is stale and cannot support install or update.
    Stale,
    /// No verifiable revocation snapshot is present.
    UnverifiedNoSnapshot,
}

/// Mirror import posture for the same catalog descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogMirrorabilityClass {
    /// Mirrors can ingest the row with verified origin trust.
    MirrorableVerified,
    /// Mirrors can ingest the row but must cap rendered trust.
    MirrorableCappedTrust,
    /// Mirrors can ingest only after re-verification.
    MirrorablePendingReverify,
    /// Mirror import is blocked by digest or signature drift.
    MirrorBlockedDigestOrSignature,
    /// Mirror import is blocked because the source class is not eligible.
    MirrorBlockedSourceNotAllowed,
}

/// Disclosure class rendered before a descriptor is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogDisclosureClass {
    /// Publisher identity and continuity state are visible.
    PublisherContinuity,
    /// Catalog lifecycle state is visible.
    LifecycleState,
    /// Moderation state and reason refs are visible.
    ModerationState,
    /// Revocation state and snapshot age are visible.
    RevocationPosture,
    /// Compatibility labels are visible.
    CompatibilityLabels,
    /// Mirror, source, and parity metadata are visible.
    MirrorMetadata,
    /// Permission manifest ref or digest is visible.
    PermissionManifest,
    /// Rollback or last-known-good posture is visible.
    RollbackPosture,
}

/// Decision emitted by catalog descriptor evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogDescriptorDecisionClass {
    /// Row is ready for catalog publication and mirror ingestion.
    ReadyForCatalog,
    /// Row is structurally valid but remains staged for review.
    StagedForReview,
    /// Row can be shown or installed only with explicit limitations.
    Limited,
    /// Row is refused for install or update catalog mutation.
    Refused,
}

/// Typed reason paired with [`CatalogDescriptorDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogDescriptorReasonClass {
    ReadyMirrorableTrustComplete,
    StagedPendingModeration,
    LimitedByPolicyOrCompatibility,
    RefusedCatalogIdentityMissing,
    RefusedPublisherContinuityMissing,
    RefusedRequiredDisclosureMissing,
    RefusedRevoked,
    RefusedQuarantined,
    RefusedMirrorMetadataMissing,
    RefusedMirrorBlocked,
    RefusedRevocationSnapshotStale,
    RefusedRollbackUnavailable,
}

/// Export-facing explanation class for support and docs/help surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogSupportExplanationClass {
    Ready,
    Staged,
    Limited,
    Revoked,
    Quarantined,
    MirrorBlocked,
    Incomplete,
}

/// Action a support, docs, or review surface may offer for a descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogActionOfferClass {
    ApproveCatalogRow,
    OpenModerationReview,
    OpenPublisherContinuity,
    OpenRevocationNotice,
    OpenMirrorDetails,
    OpenCompatibilityReport,
    KeepPinned,
    RequestAdminReview,
    RemoveOrDisable,
    ExportSupportPacket,
}

/// Publisher continuity metadata carried with a catalog descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogPublisherContinuityMetadata {
    pub publisher_id: String,
    pub publisher_display_label: String,
    pub publisher_continuity_ref: String,
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    pub continuity_state_class: PublisherContinuityStateClass,
    pub active_signing_key_refs: Vec<String>,
    pub lifecycle_event_refs: Vec<String>,
    pub freshness_class: SummaryFreshnessClass,
}

/// Lifecycle metadata rendered on catalog rows and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogLifecycleMetadata {
    pub lifecycle_state_class: CatalogLifecycleStateClass,
    pub source_registry_class: CatalogRegistrySourceClass,
    pub source_endpoint_ref: String,
    pub channel_class: PublicationChannelClass,
    pub support_class: String,
    pub lifecycle_event_refs: Vec<String>,
}

/// Moderation metadata assigned by registry operators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogModerationMetadata {
    pub moderation_state_class: CatalogModerationStateClass,
    pub moderation_review_ref: String,
    pub moderation_reason_refs: Vec<String>,
    pub anti_abuse_signal_refs: Vec<String>,
    pub operator_handoff_refs: Vec<String>,
    pub primary_operator_ref: String,
    pub backup_operator_ref: String,
}

/// Revocation-ready metadata attached to a catalog row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogRevocationMetadata {
    pub revocation_state_class: RevocationStateClass,
    pub revocation_snapshot_ref: String,
    pub revocation_snapshot_age_class: CatalogRevocationSnapshotAgeClass,
    pub last_known_good_version: String,
    pub rollback_manifest_ref: String,
    pub emergency_disable_refs: Vec<String>,
    pub revocation_event_refs: Vec<String>,
}

/// Compatibility label carried in mirrorable catalog metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogCompatibilityMetadata {
    pub compatibility_claim_class: CompatibilityClaimClass,
    pub bridge_state_class: BridgeStateClass,
    pub rendered_label: CompatibilityLabel,
    pub host_contract_family_refs: Vec<String>,
    pub capability_world_refs: Vec<String>,
    pub target_platforms: Vec<String>,
    pub caveat_labels: Vec<String>,
}

/// Mirror-compatible descriptor metadata for catalog import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogMirrorMetadata {
    pub mirrorability_class: CatalogMirrorabilityClass,
    pub mirror_descriptor_ref: String,
    pub mirror_registry_source_class: CatalogRegistrySourceClass,
    pub content_address: PublicationContentAddress,
    pub signature_ref: String,
    pub trust_badge_inheritance_rule_class: CatalogTrustBadgeInheritanceRuleClass,
    pub parity_assertion_refs: Vec<String>,
    pub compatibility_labels: Vec<CatalogCompatibilityMetadata>,
}

/// Input consumed to evaluate one catalog descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogDescriptorInput {
    pub descriptor_id: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub display_name: String,
    pub publication_ref: String,
    pub registry_manifest_ref: String,
    pub permission_manifest_ref: String,
    pub runtime_contract_ref: String,
    pub compatibility_report_ref: String,
    pub publisher: CatalogPublisherContinuityMetadata,
    pub lifecycle: CatalogLifecycleMetadata,
    pub moderation: CatalogModerationMetadata,
    pub revocation: CatalogRevocationMetadata,
    pub mirror: CatalogMirrorMetadata,
    pub rendered_disclosures: Vec<CatalogDisclosureClass>,
    pub generated_at: String,
}

/// Evaluated descriptor consumed by catalog, mirror, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogDescriptorRecord {
    pub record_kind: String,
    pub catalog_descriptor_schema_version: u32,
    pub descriptor_id: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub display_name: String,
    pub publication_ref: String,
    pub registry_manifest_ref: String,
    pub permission_manifest_ref: String,
    pub runtime_contract_ref: String,
    pub compatibility_report_ref: String,
    pub publisher: CatalogPublisherContinuityMetadata,
    pub lifecycle: CatalogLifecycleMetadata,
    pub moderation: CatalogModerationMetadata,
    pub revocation: CatalogRevocationMetadata,
    pub mirror: CatalogMirrorMetadata,
    pub required_disclosures: Vec<CatalogDisclosureClass>,
    pub rendered_disclosures: Vec<CatalogDisclosureClass>,
    pub compatibility_label_count: u32,
    pub parity_assertion_count: u32,
    pub publisher_lifecycle_event_count: u32,
    pub moderation_reason_count: u32,
    pub mirrorable_catalog_metadata: bool,
    pub revocation_ready: bool,
    pub publisher_continuity_ready: bool,
    pub decision_class: CatalogDescriptorDecisionClass,
    pub reason_class: CatalogDescriptorReasonClass,
    pub support_explanation_class: CatalogSupportExplanationClass,
    pub decision_summary: String,
    pub generated_at: String,
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support export for a catalog descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogDescriptorSupportExportRecord {
    pub record_kind: String,
    pub catalog_descriptor_schema_version: u32,
    pub export_id: String,
    pub descriptor_ref: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub package_id: String,
    pub publisher_continuity_ref: String,
    pub registry_source_class: CatalogRegistrySourceClass,
    pub lifecycle_state_class: CatalogLifecycleStateClass,
    pub moderation_state_class: CatalogModerationStateClass,
    pub revocation_state_class: RevocationStateClass,
    pub mirrorability_class: CatalogMirrorabilityClass,
    pub decision_class: CatalogDescriptorDecisionClass,
    pub reason_class: CatalogDescriptorReasonClass,
    pub support_explanation_class: CatalogSupportExplanationClass,
    pub blocks_install_or_update: bool,
    pub offered_actions: Vec<CatalogActionOfferClass>,
    pub export_safe_summary: String,
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by catalog-descriptor validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogDescriptorFinding {
    pub check_id: &'static str,
    pub message: String,
}

impl CatalogDescriptorFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate catalog metadata into a moderated, mirrorable descriptor.
pub fn evaluate_catalog_descriptor(input: CatalogDescriptorInput) -> CatalogDescriptorRecord {
    let required_disclosures = required_disclosures_for_catalog_descriptor();
    let compatibility_label_count = input.mirror.compatibility_labels.len() as u32;
    let parity_assertion_count = input.mirror.parity_assertion_refs.len() as u32;
    let publisher_lifecycle_event_count = input.publisher.lifecycle_event_refs.len() as u32;
    let moderation_reason_count = input.moderation.moderation_reason_refs.len() as u32;
    let mirrorable_catalog_metadata = mirrorable_metadata_present(&input.mirror);
    let revocation_ready = revocation_metadata_ready(&input.revocation);
    let publisher_continuity_ready = publisher_continuity_ready(&input.publisher);

    let (decision_class, reason_class, support_explanation_class, decision_summary) =
        decide_catalog_descriptor(
            &input,
            &required_disclosures,
            mirrorable_catalog_metadata,
            revocation_ready,
            publisher_continuity_ready,
        );

    CatalogDescriptorRecord {
        record_kind: CATALOG_DESCRIPTOR_RECORD_KIND.to_string(),
        catalog_descriptor_schema_version: CATALOG_DESCRIPTOR_SCHEMA_VERSION,
        descriptor_id: input.descriptor_id,
        extension_identity: input.extension_identity,
        extension_version: input.extension_version,
        package_id: input.package_id,
        display_name: input.display_name,
        publication_ref: input.publication_ref,
        registry_manifest_ref: input.registry_manifest_ref,
        permission_manifest_ref: input.permission_manifest_ref,
        runtime_contract_ref: input.runtime_contract_ref,
        compatibility_report_ref: input.compatibility_report_ref,
        publisher: input.publisher,
        lifecycle: input.lifecycle,
        moderation: input.moderation,
        revocation: input.revocation,
        mirror: input.mirror,
        required_disclosures,
        rendered_disclosures: input.rendered_disclosures,
        compatibility_label_count,
        parity_assertion_count,
        publisher_lifecycle_event_count,
        moderation_reason_count,
        mirrorable_catalog_metadata,
        revocation_ready,
        publisher_continuity_ready,
        decision_class,
        reason_class,
        support_explanation_class,
        decision_summary,
        generated_at: input.generated_at,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a descriptor into the support/export consumer shape.
pub fn project_catalog_descriptor_support_export(
    record: &CatalogDescriptorRecord,
    export_id: &str,
) -> CatalogDescriptorSupportExportRecord {
    let blocks_install_or_update = matches!(
        record.decision_class,
        CatalogDescriptorDecisionClass::Refused
    );
    let mut offered_actions = vec![
        CatalogActionOfferClass::OpenPublisherContinuity,
        CatalogActionOfferClass::OpenMirrorDetails,
        CatalogActionOfferClass::OpenCompatibilityReport,
        CatalogActionOfferClass::ExportSupportPacket,
    ];

    match record.decision_class {
        CatalogDescriptorDecisionClass::ReadyForCatalog => {
            offered_actions.insert(0, CatalogActionOfferClass::ApproveCatalogRow);
        }
        CatalogDescriptorDecisionClass::StagedForReview => {
            offered_actions.insert(0, CatalogActionOfferClass::OpenModerationReview);
            offered_actions.push(CatalogActionOfferClass::RequestAdminReview);
        }
        CatalogDescriptorDecisionClass::Limited => {
            offered_actions.push(CatalogActionOfferClass::KeepPinned);
            offered_actions.push(CatalogActionOfferClass::RequestAdminReview);
        }
        CatalogDescriptorDecisionClass::Refused => {
            offered_actions.insert(0, CatalogActionOfferClass::OpenRevocationNotice);
            offered_actions.push(CatalogActionOfferClass::RemoveOrDisable);
            offered_actions.push(CatalogActionOfferClass::KeepPinned);
        }
    }

    CatalogDescriptorSupportExportRecord {
        record_kind: CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        catalog_descriptor_schema_version: CATALOG_DESCRIPTOR_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        descriptor_ref: record.descriptor_id.clone(),
        extension_identity: record.extension_identity.clone(),
        extension_version: record.extension_version.clone(),
        package_id: record.package_id.clone(),
        publisher_continuity_ref: record.publisher.publisher_continuity_ref.clone(),
        registry_source_class: record.lifecycle.source_registry_class,
        lifecycle_state_class: record.lifecycle.lifecycle_state_class,
        moderation_state_class: record.moderation.moderation_state_class,
        revocation_state_class: record.revocation.revocation_state_class,
        mirrorability_class: record.mirror.mirrorability_class,
        decision_class: record.decision_class,
        reason_class: record.reason_class,
        support_explanation_class: record.support_explanation_class,
        blocks_install_or_update,
        offered_actions,
        export_safe_summary: format!(
            "{} {} decision={:?}; lifecycle={:?}; moderation={:?}; revocation={:?}; mirror={:?}",
            record.extension_identity,
            record.extension_version,
            record.decision_class,
            record.lifecycle.lifecycle_state_class,
            record.moderation.moderation_state_class,
            record.revocation.revocation_state_class,
            record.mirror.mirrorability_class
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a catalog descriptor.
pub fn validate_catalog_descriptor_record(
    record: &CatalogDescriptorRecord,
) -> Vec<CatalogDescriptorFinding> {
    let mut findings = Vec::new();

    if record.record_kind != CATALOG_DESCRIPTOR_RECORD_KIND {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.record_kind_wrong",
            format!(
                "record_kind must be '{CATALOG_DESCRIPTOR_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.catalog_descriptor_schema_version != CATALOG_DESCRIPTOR_SCHEMA_VERSION {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.schema_version_wrong",
            format!(
                "catalog_descriptor_schema_version must be {CATALOG_DESCRIPTOR_SCHEMA_VERSION}; got {}",
                record.catalog_descriptor_schema_version
            ),
        ));
    }
    if !record.descriptor_id.starts_with("catalog_descriptor:") {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.id_unprefixed",
            "descriptor_id must start with 'catalog_descriptor:'",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.redaction_class_must_be_metadata_safe",
            "catalog descriptor records must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    if let Some(missing) =
        first_missing_disclosure(&record.required_disclosures, &record.rendered_disclosures)
    {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.required_disclosure_missing",
            format!("required disclosure '{missing:?}' was not rendered"),
        ));
    }
    if record.compatibility_label_count != record.mirror.compatibility_labels.len() as u32 {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.compatibility_label_count_inconsistent",
            "compatibility_label_count must equal mirror.compatibility_labels.len()",
        ));
    }
    if record.parity_assertion_count != record.mirror.parity_assertion_refs.len() as u32 {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.parity_assertion_count_inconsistent",
            "parity_assertion_count must equal mirror.parity_assertion_refs.len()",
        ));
    }
    if record.publisher_lifecycle_event_count != record.publisher.lifecycle_event_refs.len() as u32
    {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.publisher_lifecycle_event_count_inconsistent",
            "publisher_lifecycle_event_count must equal publisher.lifecycle_event_refs.len()",
        ));
    }
    if record.moderation_reason_count != record.moderation.moderation_reason_refs.len() as u32 {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.moderation_reason_count_inconsistent",
            "moderation_reason_count must equal moderation.moderation_reason_refs.len()",
        ));
    }
    if record.mirrorable_catalog_metadata != mirrorable_metadata_present(&record.mirror) {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.mirrorable_metadata_inconsistent",
            "mirrorable_catalog_metadata must reflect mirror metadata presence and mirrorability",
        ));
    }
    if record.revocation_ready != revocation_metadata_ready(&record.revocation) {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.revocation_ready_inconsistent",
            "revocation_ready must reflect revocation snapshot and rollback metadata",
        ));
    }
    if record.publisher_continuity_ready != publisher_continuity_ready(&record.publisher) {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor.publisher_continuity_ready_inconsistent",
            "publisher_continuity_ready must reflect publisher identity and signing metadata",
        ));
    }

    findings
}

/// Validate structural invariants for a catalog-descriptor support export.
pub fn validate_catalog_descriptor_support_export_record(
    record: &CatalogDescriptorSupportExportRecord,
) -> Vec<CatalogDescriptorFinding> {
    let mut findings = Vec::new();

    if record.record_kind != CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor_support_export.record_kind_wrong",
            format!(
                "record_kind must be '{CATALOG_DESCRIPTOR_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.catalog_descriptor_schema_version != CATALOG_DESCRIPTOR_SCHEMA_VERSION {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor_support_export.schema_version_wrong",
            format!(
                "catalog_descriptor_schema_version must be {CATALOG_DESCRIPTOR_SCHEMA_VERSION}; got {}",
                record.catalog_descriptor_schema_version
            ),
        ));
    }
    if !record
        .export_id
        .starts_with("catalog_descriptor_support_export:")
    {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor_support_export.id_unprefixed",
            "export_id must start with 'catalog_descriptor_support_export:'",
        ));
    }
    if !record.descriptor_ref.starts_with("catalog_descriptor:") {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor_support_export.descriptor_ref_unprefixed",
            "descriptor_ref must start with 'catalog_descriptor:'",
        ));
    }
    if record.export_safe_summary.trim().is_empty() {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor_support_export.summary_required",
            "export_safe_summary must be a non-empty string",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(CatalogDescriptorFinding::new(
            "catalog_descriptor_support_export.redaction_class_must_be_metadata_safe",
            "support export records must emit RedactionClass::MetadataSafeDefault",
        ));
    }

    findings
}

fn required_disclosures_for_catalog_descriptor() -> Vec<CatalogDisclosureClass> {
    vec![
        CatalogDisclosureClass::PublisherContinuity,
        CatalogDisclosureClass::LifecycleState,
        CatalogDisclosureClass::ModerationState,
        CatalogDisclosureClass::RevocationPosture,
        CatalogDisclosureClass::CompatibilityLabels,
        CatalogDisclosureClass::MirrorMetadata,
        CatalogDisclosureClass::PermissionManifest,
        CatalogDisclosureClass::RollbackPosture,
    ]
}

fn decide_catalog_descriptor(
    input: &CatalogDescriptorInput,
    required_disclosures: &[CatalogDisclosureClass],
    mirrorable_catalog_metadata: bool,
    revocation_ready: bool,
    publisher_continuity_ready: bool,
) -> (
    CatalogDescriptorDecisionClass,
    CatalogDescriptorReasonClass,
    CatalogSupportExplanationClass,
    String,
) {
    if catalog_identity_missing(input) {
        return refused(
            CatalogDescriptorReasonClass::RefusedCatalogIdentityMissing,
            CatalogSupportExplanationClass::Incomplete,
            "catalog descriptor is missing package, artifact, manifest, runtime, or compatibility identity",
        );
    }
    if !publisher_continuity_ready {
        return refused(
            CatalogDescriptorReasonClass::RefusedPublisherContinuityMissing,
            CatalogSupportExplanationClass::Incomplete,
            "catalog descriptor lacks publisher continuity, signing, or fresh-enough publisher metadata",
        );
    }
    if let Some(missing) =
        first_missing_disclosure(required_disclosures, &input.rendered_disclosures)
    {
        return refused(
            CatalogDescriptorReasonClass::RefusedRequiredDisclosureMissing,
            CatalogSupportExplanationClass::Incomplete,
            format!("catalog descriptor did not render required disclosure '{missing:?}'"),
        );
    }
    if !mirrorable_catalog_metadata {
        let reason = if matches!(
            input.mirror.mirrorability_class,
            CatalogMirrorabilityClass::MirrorBlockedDigestOrSignature
                | CatalogMirrorabilityClass::MirrorBlockedSourceNotAllowed
        ) {
            CatalogDescriptorReasonClass::RefusedMirrorBlocked
        } else {
            CatalogDescriptorReasonClass::RefusedMirrorMetadataMissing
        };
        return refused(
            reason,
            CatalogSupportExplanationClass::MirrorBlocked,
            "catalog descriptor lacks mirrorable trust, compatibility, signature, or parity metadata",
        );
    }
    if revocation_snapshot_blocks(&input.revocation) {
        return refused(
            CatalogDescriptorReasonClass::RefusedRevocationSnapshotStale,
            CatalogSupportExplanationClass::Incomplete,
            "catalog descriptor cannot be admitted because revocation metadata is stale or unverified",
        );
    }
    if !revocation_ready {
        return refused(
            CatalogDescriptorReasonClass::RefusedRollbackUnavailable,
            CatalogSupportExplanationClass::Incomplete,
            "catalog descriptor is not revocation-ready because rollback or last-known-good metadata is missing",
        );
    }
    if row_revoked(input) {
        return refused(
            CatalogDescriptorReasonClass::RefusedRevoked,
            CatalogSupportExplanationClass::Revoked,
            "catalog descriptor is revoked or emergency-disabled; install and update remain blocked",
        );
    }
    if row_quarantined(input) {
        return refused(
            CatalogDescriptorReasonClass::RefusedQuarantined,
            CatalogSupportExplanationClass::Quarantined,
            "catalog descriptor is quarantined pending trust review, repair, or removal",
        );
    }
    if row_staged(input) {
        return (
            CatalogDescriptorDecisionClass::StagedForReview,
            CatalogDescriptorReasonClass::StagedPendingModeration,
            CatalogSupportExplanationClass::Staged,
            "catalog descriptor is staged pending moderation review while preserving mirror and rollback metadata"
                .to_string(),
        );
    }
    if row_limited(input) {
        return (
            CatalogDescriptorDecisionClass::Limited,
            CatalogDescriptorReasonClass::LimitedByPolicyOrCompatibility,
            CatalogSupportExplanationClass::Limited,
            "catalog descriptor is limited by policy, compatibility, lifecycle, or mirror re-verification"
                .to_string(),
        );
    }

    (
        CatalogDescriptorDecisionClass::ReadyForCatalog,
        CatalogDescriptorReasonClass::ReadyMirrorableTrustComplete,
        CatalogSupportExplanationClass::Ready,
        "catalog descriptor is moderated, revocation-ready, and mirrorable without losing trust vocabulary"
            .to_string(),
    )
}

fn refused(
    reason: CatalogDescriptorReasonClass,
    explanation: CatalogSupportExplanationClass,
    summary: impl Into<String>,
) -> (
    CatalogDescriptorDecisionClass,
    CatalogDescriptorReasonClass,
    CatalogSupportExplanationClass,
    String,
) {
    (
        CatalogDescriptorDecisionClass::Refused,
        reason,
        explanation,
        summary.into(),
    )
}

fn first_missing_disclosure(
    required: &[CatalogDisclosureClass],
    rendered: &[CatalogDisclosureClass],
) -> Option<CatalogDisclosureClass> {
    required
        .iter()
        .find(|required| !rendered.contains(required))
        .copied()
}

fn catalog_identity_missing(input: &CatalogDescriptorInput) -> bool {
    input.descriptor_id.trim().is_empty()
        || input.extension_identity.trim().is_empty()
        || input.extension_version.trim().is_empty()
        || input.package_id.trim().is_empty()
        || input.display_name.trim().is_empty()
        || input.publication_ref.trim().is_empty()
        || input.registry_manifest_ref.trim().is_empty()
        || input.permission_manifest_ref.trim().is_empty()
        || input.runtime_contract_ref.trim().is_empty()
        || input.compatibility_report_ref.trim().is_empty()
}

fn publisher_continuity_ready(publisher: &CatalogPublisherContinuityMetadata) -> bool {
    let fresh_enough = matches!(
        publisher.freshness_class,
        SummaryFreshnessClass::AuthoritativeLive
            | SummaryFreshnessClass::WarmCached
            | SummaryFreshnessClass::DegradedCached
    );
    publisher.publisher_id.trim().is_empty().not()
        && publisher.publisher_display_label.trim().is_empty().not()
        && publisher.publisher_continuity_ref.trim().is_empty().not()
        && fresh_enough
        && !matches!(
            publisher.publisher_trust_tier_class,
            PublisherTrustTierClass::AnonymousPublisherClass
        )
        && !publisher.active_signing_key_refs.is_empty()
}

fn mirrorable_metadata_present(mirror: &CatalogMirrorMetadata) -> bool {
    !matches!(
        mirror.mirrorability_class,
        CatalogMirrorabilityClass::MirrorBlockedDigestOrSignature
            | CatalogMirrorabilityClass::MirrorBlockedSourceNotAllowed
    ) && mirror.mirror_descriptor_ref.trim().is_empty().not()
        && mirror.signature_ref.trim().is_empty().not()
        && !content_address_missing(&mirror.content_address)
        && !mirror.parity_assertion_refs.is_empty()
        && !mirror.compatibility_labels.is_empty()
        && mirror.compatibility_labels.iter().all(compatibility_ready)
}

fn compatibility_ready(label: &CatalogCompatibilityMetadata) -> bool {
    !matches!(
        label.compatibility_claim_class,
        CompatibilityClaimClass::IncompatibleBlockedOnPolicy
    ) && !matches!(label.rendered_label, CompatibilityLabel::Unsupported)
        && !label.host_contract_family_refs.is_empty()
        && !label.capability_world_refs.is_empty()
        && !label.target_platforms.is_empty()
}

fn content_address_missing(address: &PublicationContentAddress) -> bool {
    address.digest_algorithm.trim().is_empty()
        || address.digest_hex.trim().is_empty()
        || address.digest_size_bytes == 0
}

fn revocation_metadata_ready(revocation: &CatalogRevocationMetadata) -> bool {
    !revocation_snapshot_blocks(revocation)
        && revocation.revocation_snapshot_ref.trim().is_empty().not()
        && revocation.last_known_good_version.trim().is_empty().not()
        && revocation.rollback_manifest_ref.trim().is_empty().not()
}

fn revocation_snapshot_blocks(revocation: &CatalogRevocationMetadata) -> bool {
    matches!(
        revocation.revocation_snapshot_age_class,
        CatalogRevocationSnapshotAgeClass::Stale
            | CatalogRevocationSnapshotAgeClass::UnverifiedNoSnapshot
    )
}

fn row_revoked(input: &CatalogDescriptorInput) -> bool {
    matches!(
        input.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Revoked
    ) || matches!(
        input.moderation.moderation_state_class,
        CatalogModerationStateClass::Revoked
    ) || matches!(
        input.revocation.revocation_state_class,
        RevocationStateClass::Revoked
            | RevocationStateClass::EmergencyDisabled
            | RevocationStateClass::MirrorPromotionRevoked
    )
}

fn row_quarantined(input: &CatalogDescriptorInput) -> bool {
    matches!(
        input.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Quarantined
    ) || matches!(
        input.moderation.moderation_state_class,
        CatalogModerationStateClass::Quarantined
    ) || matches!(
        input.revocation.revocation_state_class,
        RevocationStateClass::Quarantined
    ) || matches!(
        input.publisher.publisher_trust_tier_class,
        PublisherTrustTierClass::QuarantinedPublisher
    ) || matches!(
        input.publisher.continuity_state_class,
        PublisherContinuityStateClass::Retired
    )
}

fn row_staged(input: &CatalogDescriptorInput) -> bool {
    matches!(
        input.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Staged
    ) || matches!(
        input.moderation.moderation_state_class,
        CatalogModerationStateClass::PendingReview
    ) || matches!(
        input.lifecycle.channel_class,
        PublicationChannelClass::Quarantine
    )
}

fn row_limited(input: &CatalogDescriptorInput) -> bool {
    matches!(
        input.lifecycle.lifecycle_state_class,
        CatalogLifecycleStateClass::Limited | CatalogLifecycleStateClass::Deprecated
    ) || matches!(
        input.moderation.moderation_state_class,
        CatalogModerationStateClass::Limited
    ) || matches!(
        input.mirror.mirrorability_class,
        CatalogMirrorabilityClass::MirrorablePendingReverify
            | CatalogMirrorabilityClass::MirrorableCappedTrust
    ) || input.mirror.compatibility_labels.iter().any(|label| {
        matches!(
            label.compatibility_claim_class,
            CompatibilityClaimClass::CompatibleOnSubsetOfDeclaredTargets
                | CompatibilityClaimClass::CompatibilityBridgeRequired
                | CompatibilityClaimClass::CompatibilityUnknownPendingReverification
        ) || matches!(
            label.rendered_label,
            CompatibilityLabel::Translated
                | CompatibilityLabel::Partial
                | CompatibilityLabel::Shimmed
                | CompatibilityLabel::Unknown
        ) || !matches!(
            label.bridge_state_class,
            BridgeStateClass::NoBridgeRequiredNativeMatch
        )
    })
}

trait BoolExt {
    fn not(self) -> bool;
}

impl BoolExt for bool {
    fn not(self) -> bool {
        !self
    }
}
