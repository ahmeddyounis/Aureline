//! SDK and public-interface lifecycle metadata for beta extension surfaces.
//!
//! The packet owned here is the canonical machine-readable source for
//! versioning, support windows, deprecation posture, replacement paths,
//! and removal guidance on declared beta extension surfaces. SDK docs,
//! compatibility reports, publication tooling, and support exports cite
//! the same row ids instead of copying local support-window prose.

use serde::{Deserialize, Serialize};

use crate::manifest_baseline::RedactionClass;

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`LifecycleMetadataPacket`] payloads.
pub const LIFECYCLE_METADATA_PACKET_RECORD_KIND: &str = "extension_lifecycle_metadata_packet";

/// Record-kind tag carried on serialized
/// [`LifecycleMetadataSupportExportRecord`] payloads.
pub const LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_lifecycle_metadata_support_export";

/// Schema version for lifecycle metadata payloads.
pub const LIFECYCLE_METADATA_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path to the checked lifecycle metadata packet.
pub const CURRENT_EXTENSION_LIFECYCLE_METADATA_PACKET_PATH: &str =
    "artifacts/extensions/m3/lifecycle_metadata_packet.json";

const CURRENT_EXTENSION_LIFECYCLE_METADATA_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/extensions/m3/lifecycle_metadata_packet.json"
));

/// Loads the checked lifecycle metadata packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked artifact does not match
/// [`LifecycleMetadataPacket`].
pub fn current_extension_lifecycle_metadata_packet(
) -> Result<LifecycleMetadataPacket, serde_json::Error> {
    serde_json::from_str(CURRENT_EXTENSION_LIFECYCLE_METADATA_PACKET_JSON)
}

/// Closed vocabulary for the public surface family a lifecycle row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleSurfaceKind {
    /// Extension SDK typed API surface.
    SdkApiSurface,
    /// Extension manifest schema or manifest field.
    ManifestSchema,
    /// WIT package or world reference.
    WitWorld,
    /// Permission vocabulary row.
    PermissionVocabulary,
    /// Extension publication packet or registry promotion schema.
    PublicationPipeline,
    /// Compatibility bridge profile or shim profile.
    BridgeProfile,
}

/// Stability label rendered for a public lifecycle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStabilityLabel {
    /// Internal implementation detail with no public promise.
    Internal,
    /// Preview-only surface with best-effort compatibility.
    Experimental,
    /// Beta public surface with release-family compatibility intent.
    Beta,
    /// Stable public surface with SemVer and migration obligations.
    Stable,
    /// Contractual slow-moving public surface.
    LtsSurface,
    /// Supported surface with an active deprecation clock.
    Deprecated,
    /// Retired surface unavailable for new publication or activation.
    Retired,
}

impl LifecycleStabilityLabel {
    const fn requires_support_window(self) -> bool {
        matches!(
            self,
            Self::Beta | Self::Stable | Self::LtsSurface | Self::Deprecated
        )
    }
}

/// Versioning mechanism used by a governed lifecycle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleVersioningScheme {
    /// Semantic Versioning for SDK/helper packages.
    Semver,
    /// Integer JSON schema epoch.
    JsonSchemaEpoch,
    /// WIT package versioning.
    WitPackageVersion,
    /// Permission vocabulary epoch.
    PermissionVocabularyEpoch,
    /// Date or named compatibility profile epoch.
    BridgeProfileEpoch,
    /// Publication packet schema epoch.
    PublicationSchemaEpoch,
}

/// Deprecation posture for a lifecycle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleDeprecationPostureClass {
    /// Surface is not deprecated.
    NotDeprecated,
    /// Surface is deprecated and has a direct replacement.
    DeprecatedWithReplacement,
    /// Surface is deprecated and has no direct replacement.
    DeprecatedNoDirectReplacement,
    /// Surface is already retired.
    Retired,
}

/// Decision emitted after evaluating a lifecycle metadata packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMetadataDecisionClass {
    /// Every row is governed and no deprecated row needs special disclosure.
    ReadyForBetaUse,
    /// Packet is governed, and consumers must show deprecation guidance.
    DeprecatedMigrationRequired,
    /// Packet is structurally incomplete and must block publication widening.
    RefusedIncompleteMetadata,
}

/// Typed reason paired with [`LifecycleMetadataDecisionClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMetadataReasonClass {
    /// All lifecycle rows satisfy governance requirements.
    AllRowsGoverned,
    /// Deprecated rows include replacement, expiry, and migration guidance.
    DeprecatedRowsCarryMigration,
    /// The packet id is outside the lifecycle metadata namespace.
    RefusedPacketIdUnprefixed,
    /// The packet does not cite the versioning and deprecation policy.
    RefusedPolicyRefMissing,
    /// The packet does not contain governed surface rows.
    RefusedRowsMissing,
    /// A governed public row is missing support-window metadata.
    RefusedSupportWindowMissing,
    /// A deprecated row lacks a replacement or no-direct-replacement reason.
    RefusedDeprecatedRowMissingReplacementOrNoDirectReplacement,
    /// A deprecated row lacks a removal target version or date.
    RefusedDeprecatedRowMissingExpiry,
    /// A deprecated row lacks migration guidance.
    RefusedDeprecatedRowMissingMigrationGuide,
    /// A row does not name any consuming surface.
    RefusedConsumerRefsMissing,
    /// A packet, row, or support export is not metadata-safe to disclose.
    RefusedRedactionClassNotMetadataSafe,
}

/// Support window attached to a governed public surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleSupportWindow {
    /// Version where the surface first became available.
    pub introduced_in_version: String,
    /// Minimum overlap promised before incompatible removal.
    pub minimum_overlap: String,
    /// Earliest version where the surface can be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_not_before_version: Option<String>,
    /// Earliest calendar date where the surface can be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_not_before_date: Option<String>,
    /// Human-readable support-window summary safe for docs and support export.
    pub support_window_summary: String,
}

/// Deprecation metadata attached to a lifecycle row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDeprecationMetadata {
    /// Current deprecation posture for the surface.
    pub deprecation_posture_class: LifecycleDeprecationPostureClass,
    /// Version where the deprecation notice became active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated_since_version: Option<String>,
    /// Replacement surface authors should migrate to, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_surface_ref: Option<String>,
    /// Reason no direct replacement exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_direct_replacement_reason: Option<String>,
    /// Documentation that explains the migration or retirement path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_guide_ref: Option<String>,
    /// Target version where the deprecated surface can be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_target_version: Option<String>,
    /// Target date where the deprecated surface can be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_target_date: Option<String>,
    /// Reader, writer, alias, downgrade, or rollback behavior during overlap.
    pub alias_or_downgrade_behavior: String,
    /// Author-visible impact of the deprecation posture.
    pub author_impact: String,
}

/// One governed SDK/public-interface lifecycle row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleMetadataRow {
    /// Stable lifecycle row id referenced by docs, tooling, and artifacts.
    pub row_id: String,
    /// Public surface governed by this row.
    pub surface_ref: String,
    /// Surface family used to choose versioning and support rules.
    pub surface_kind: LifecycleSurfaceKind,
    /// Stability label and public compatibility promise for the row.
    pub stability_label: LifecycleStabilityLabel,
    /// Versioning scheme applied to the governed surface.
    pub versioning_scheme: LifecycleVersioningScheme,
    /// Currently published version of the surface.
    pub current_version: String,
    /// Oldest supported version in the active compatibility window.
    pub min_supported_version: String,
    /// Newest version exercised by the current conformance evidence.
    pub max_tested_version: String,
    /// Support-window metadata for governed public rows.
    pub support_window: LifecycleSupportWindow,
    /// Deprecation posture and migration metadata for this row.
    pub deprecation: LifecycleDeprecationMetadata,
    /// Author-facing documentation that defines the surface.
    pub docs_ref: String,
    /// Schema or typed contract that validates the surface.
    pub schema_ref: String,
    /// Compatibility report that consumes or verifies the row.
    pub compatibility_report_ref: String,
    /// Optional conformance report for executable or fixture-backed evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conformance_report_ref: Option<String>,
    /// Accountable owner for lifecycle updates.
    pub owner: String,
    /// Docs, tools, reports, or runtime consumers that cite the row.
    pub consumer_refs: Vec<String>,
    /// Redaction class proving the row is safe for support export.
    pub redaction_class: RedactionClass,
}

/// Input to evaluate a lifecycle metadata packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleMetadataPacketInput {
    /// Stable packet id used before adding derived counters and decisions.
    pub packet_id: String,
    /// Versioning and deprecation policy cited by the packet.
    pub policy_ref: String,
    /// UTC timestamp for the packet generation.
    pub generated_at: String,
    /// Accountable owner for the packet.
    pub owner: String,
    /// SDK line governed by this packet.
    pub sdk_line_id: String,
    /// SemVer label for the governed SDK line.
    pub sdk_line_semver: String,
    /// Lifecycle rows to evaluate.
    pub rows: Vec<LifecycleMetadataRow>,
}

/// Canonical lifecycle metadata packet for extension beta surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleMetadataPacket {
    /// Record-kind tag for serialized lifecycle metadata packets.
    pub record_kind: String,
    /// Schema version for the lifecycle metadata packet shape.
    pub lifecycle_metadata_schema_version: u32,
    /// Stable packet id for the governed SDK line.
    pub packet_id: String,
    /// Versioning and deprecation policy cited by the packet.
    pub policy_ref: String,
    /// UTC timestamp for the packet generation.
    pub generated_at: String,
    /// Accountable owner for the packet.
    pub owner: String,
    /// SDK line governed by this packet.
    pub sdk_line_id: String,
    /// SemVer label for the governed SDK line.
    pub sdk_line_semver: String,
    /// Governed lifecycle rows.
    pub rows: Vec<LifecycleMetadataRow>,
    /// Number of governed lifecycle rows.
    pub row_count: u32,
    /// Number of rows with active deprecation or retirement posture.
    pub deprecated_row_count: u32,
    /// Number of beta, stable, LTS, or deprecated rows with support windows.
    pub beta_or_stable_row_count: u32,
    /// Publication decision derived from packet completeness.
    pub decision_class: LifecycleMetadataDecisionClass,
    /// Typed reason for the derived decision.
    pub reason_class: LifecycleMetadataReasonClass,
    /// Human-readable summary of the derived decision.
    pub decision_summary: String,
    /// Redaction class proving the packet is safe for support export.
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support export for lifecycle metadata packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleMetadataSupportExportRecord {
    /// Record-kind tag for serialized lifecycle support exports.
    pub record_kind: String,
    /// Schema version for the lifecycle support-export shape.
    pub lifecycle_metadata_schema_version: u32,
    /// Stable export id for this projection.
    pub export_id: String,
    /// Lifecycle metadata packet id that produced this export.
    pub packet_ref: String,
    /// Versioning and deprecation policy cited by the source packet.
    pub policy_ref: String,
    /// SDK line governed by the source packet.
    pub sdk_line_id: String,
    /// SemVer label for the governed SDK line.
    pub sdk_line_semver: String,
    /// Number of governed lifecycle rows.
    pub row_count: u32,
    /// Number of rows with active deprecation or retirement posture.
    pub deprecated_row_count: u32,
    /// Number of beta, stable, LTS, or deprecated rows with support windows.
    pub beta_or_stable_row_count: u32,
    /// Publication decision derived from packet completeness.
    pub decision_class: LifecycleMetadataDecisionClass,
    /// Typed reason for the derived decision.
    pub reason_class: LifecycleMetadataReasonClass,
    /// Whether incomplete lifecycle metadata blocks publication.
    pub blocks_publication: bool,
    /// Whether consumers must show deprecation migration guidance.
    pub deprecation_disclosure_required: bool,
    /// Metadata-safe summary for support bundles and registry review.
    pub export_safe_summary: String,
    /// Redaction class proving the export is safe for support bundles.
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by lifecycle metadata validators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleMetadataFinding {
    /// Stable check id for the failed invariant.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
    /// Optional row id when the finding applies to one lifecycle row.
    pub row_id: Option<String>,
}

impl LifecycleMetadataFinding {
    fn packet(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
            row_id: None,
        }
    }

    fn row(row_id: &str, check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
            row_id: Some(row_id.to_string()),
        }
    }
}

/// Evaluate one [`LifecycleMetadataPacketInput`] into a canonical
/// [`LifecycleMetadataPacket`].
pub fn evaluate_lifecycle_metadata_packet(
    input: LifecycleMetadataPacketInput,
) -> LifecycleMetadataPacket {
    let LifecycleMetadataPacketInput {
        packet_id,
        policy_ref,
        generated_at,
        owner,
        sdk_line_id,
        sdk_line_semver,
        rows,
    } = input;

    let row_count = rows.len() as u32;
    let deprecated_row_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.stability_label,
                LifecycleStabilityLabel::Deprecated | LifecycleStabilityLabel::Retired
            ) || row.deprecation.deprecation_posture_class
                != LifecycleDeprecationPostureClass::NotDeprecated
        })
        .count() as u32;
    let beta_or_stable_row_count = rows
        .iter()
        .filter(|row| row.stability_label.requires_support_window())
        .count() as u32;

    let (decision_class, reason_class, decision_summary) =
        decide_lifecycle_metadata_packet(&packet_id, &policy_ref, &rows);

    LifecycleMetadataPacket {
        record_kind: LIFECYCLE_METADATA_PACKET_RECORD_KIND.to_string(),
        lifecycle_metadata_schema_version: LIFECYCLE_METADATA_SCHEMA_VERSION,
        packet_id,
        policy_ref,
        generated_at,
        owner,
        sdk_line_id,
        sdk_line_semver,
        rows,
        row_count,
        deprecated_row_count,
        beta_or_stable_row_count,
        decision_class,
        reason_class,
        decision_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Project a lifecycle metadata packet into the support-export consumer.
pub fn project_lifecycle_metadata_support_export(
    record: &LifecycleMetadataPacket,
    export_id: &str,
) -> LifecycleMetadataSupportExportRecord {
    let blocks_publication =
        record.decision_class == LifecycleMetadataDecisionClass::RefusedIncompleteMetadata;
    let deprecation_disclosure_required =
        record.decision_class == LifecycleMetadataDecisionClass::DeprecatedMigrationRequired;
    let export_safe_summary = format!(
        "{} rows ({} beta/stable, {} deprecated); decision={:?}; policy={}",
        record.row_count,
        record.beta_or_stable_row_count,
        record.deprecated_row_count,
        record.decision_class,
        record.policy_ref
    );

    LifecycleMetadataSupportExportRecord {
        record_kind: LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        lifecycle_metadata_schema_version: LIFECYCLE_METADATA_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        packet_ref: record.packet_id.clone(),
        policy_ref: record.policy_ref.clone(),
        sdk_line_id: record.sdk_line_id.clone(),
        sdk_line_semver: record.sdk_line_semver.clone(),
        row_count: record.row_count,
        deprecated_row_count: record.deprecated_row_count,
        beta_or_stable_row_count: record.beta_or_stable_row_count,
        decision_class: record.decision_class,
        reason_class: record.reason_class,
        blocks_publication,
        deprecation_disclosure_required,
        export_safe_summary,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a lifecycle metadata packet.
pub fn validate_lifecycle_metadata_packet(
    record: &LifecycleMetadataPacket,
) -> Vec<LifecycleMetadataFinding> {
    let mut findings = Vec::new();
    if record.record_kind != LIFECYCLE_METADATA_PACKET_RECORD_KIND {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.record_kind_wrong",
            format!(
                "record_kind must be '{LIFECYCLE_METADATA_PACKET_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.lifecycle_metadata_schema_version != LIFECYCLE_METADATA_SCHEMA_VERSION {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.schema_version_wrong",
            format!(
                "lifecycle_metadata_schema_version must be {LIFECYCLE_METADATA_SCHEMA_VERSION}; got {}",
                record.lifecycle_metadata_schema_version
            ),
        ));
    }
    if !record
        .packet_id
        .starts_with("extension_lifecycle_metadata_packet:")
    {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.packet_id_unprefixed",
            "packet_id must start with 'extension_lifecycle_metadata_packet:'",
        ));
    }
    if record.policy_ref.trim().is_empty() {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.policy_ref_missing",
            "policy_ref must cite the SDK versioning and deprecation policy",
        ));
    }
    if record.rows.is_empty() {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.rows_missing",
            "packet must carry at least one governed surface row",
        ));
    }
    if record.row_count != record.rows.len() as u32 {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.row_count_inconsistent",
            "row_count must equal rows.len()",
        ));
    }
    let expected_deprecated = record
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.stability_label,
                LifecycleStabilityLabel::Deprecated | LifecycleStabilityLabel::Retired
            ) || row.deprecation.deprecation_posture_class
                != LifecycleDeprecationPostureClass::NotDeprecated
        })
        .count() as u32;
    if record.deprecated_row_count != expected_deprecated {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.deprecated_row_count_inconsistent",
            "deprecated_row_count must reflect row deprecation posture",
        ));
    }
    let expected_beta_stable = record
        .rows
        .iter()
        .filter(|row| row.stability_label.requires_support_window())
        .count() as u32;
    if record.beta_or_stable_row_count != expected_beta_stable {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.beta_or_stable_count_inconsistent",
            "beta_or_stable_row_count must reflect governed public rows",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata.redaction_class_must_be_metadata_safe",
            "lifecycle metadata records must emit RedactionClass::MetadataSafeDefault",
        ));
    }

    for row in &record.rows {
        validate_lifecycle_row(row, &mut findings);
    }

    findings
}

/// Validate structural invariants for a lifecycle metadata support export.
pub fn validate_lifecycle_metadata_support_export(
    record: &LifecycleMetadataSupportExportRecord,
) -> Vec<LifecycleMetadataFinding> {
    let mut findings = Vec::new();
    if record.record_kind != LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata_support_export.record_kind_wrong",
            format!(
                "record_kind must be '{LIFECYCLE_METADATA_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.lifecycle_metadata_schema_version != LIFECYCLE_METADATA_SCHEMA_VERSION {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata_support_export.schema_version_wrong",
            format!(
                "lifecycle_metadata_schema_version must be {LIFECYCLE_METADATA_SCHEMA_VERSION}; got {}",
                record.lifecycle_metadata_schema_version
            ),
        ));
    }
    if !record
        .export_id
        .starts_with("extension_lifecycle_metadata_support_export:")
    {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata_support_export.id_unprefixed",
            "export_id must start with 'extension_lifecycle_metadata_support_export:'",
        ));
    }
    if !record
        .packet_ref
        .starts_with("extension_lifecycle_metadata_packet:")
    {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata_support_export.packet_ref_unprefixed",
            "packet_ref must start with 'extension_lifecycle_metadata_packet:'",
        ));
    }
    if record.export_safe_summary.trim().is_empty() {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata_support_export.summary_required",
            "export_safe_summary must be non-empty",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(LifecycleMetadataFinding::packet(
            "lifecycle_metadata_support_export.redaction_class_must_be_metadata_safe",
            "support exports must emit RedactionClass::MetadataSafeDefault",
        ));
    }
    findings
}

fn validate_lifecycle_row(
    row: &LifecycleMetadataRow,
    findings: &mut Vec<LifecycleMetadataFinding>,
) {
    if !row.row_id.starts_with("lifecycle_row:") {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.row_id_unprefixed",
            "row_id must start with 'lifecycle_row:'",
        ));
    }
    if row.surface_ref.trim().is_empty()
        || row.current_version.trim().is_empty()
        || row.min_supported_version.trim().is_empty()
        || row.max_tested_version.trim().is_empty()
        || row.docs_ref.trim().is_empty()
        || row.schema_ref.trim().is_empty()
        || row.compatibility_report_ref.trim().is_empty()
        || row.owner.trim().is_empty()
    {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.row_required_field_missing",
            "surface, version, docs, schema, compatibility report, and owner fields are required",
        ));
    }
    if row.consumer_refs.is_empty() {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.consumer_refs_missing",
            "row must name at least one consuming surface",
        ));
    }
    if row.stability_label.requires_support_window() && support_window_missing(&row.support_window)
    {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.support_window_missing",
            "beta, stable, LTS, and deprecated rows must publish a support window",
        ));
    }
    if row.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.row_redaction_class_must_be_metadata_safe",
            "row redaction_class must be MetadataSafeDefault",
        ));
    }
    if row_is_deprecated(row) {
        validate_deprecated_row(row, findings);
    }
}

fn decide_lifecycle_metadata_packet(
    packet_id: &str,
    policy_ref: &str,
    rows: &[LifecycleMetadataRow],
) -> (
    LifecycleMetadataDecisionClass,
    LifecycleMetadataReasonClass,
    String,
) {
    if !packet_id.starts_with("extension_lifecycle_metadata_packet:") {
        return refused(
            LifecycleMetadataReasonClass::RefusedPacketIdUnprefixed,
            "packet id is not in the lifecycle metadata namespace",
        );
    }
    if policy_ref.trim().is_empty() {
        return refused(
            LifecycleMetadataReasonClass::RefusedPolicyRefMissing,
            "lifecycle metadata packet must cite the SDK versioning policy",
        );
    }
    if rows.is_empty() {
        return refused(
            LifecycleMetadataReasonClass::RefusedRowsMissing,
            "lifecycle metadata packet has no governed rows",
        );
    }
    for row in rows {
        if row.consumer_refs.is_empty() {
            return refused(
                LifecycleMetadataReasonClass::RefusedConsumerRefsMissing,
                "every lifecycle row must name at least one consuming surface",
            );
        }
        if row.stability_label.requires_support_window()
            && support_window_missing(&row.support_window)
        {
            return refused(
                LifecycleMetadataReasonClass::RefusedSupportWindowMissing,
                "every beta, stable, LTS, or deprecated row must publish a support window",
            );
        }
        if row.redaction_class != RedactionClass::MetadataSafeDefault {
            return refused(
                LifecycleMetadataReasonClass::RefusedRedactionClassNotMetadataSafe,
                "lifecycle rows must stay metadata-safe",
            );
        }
        if row_is_deprecated(row) {
            if replacement_or_no_direct_missing(&row.deprecation) {
                return refused(
                    LifecycleMetadataReasonClass::RefusedDeprecatedRowMissingReplacementOrNoDirectReplacement,
                    "deprecated rows must name a replacement or an explicit no-direct-replacement reason",
                );
            }
            if expiry_missing(&row.deprecation) {
                return refused(
                    LifecycleMetadataReasonClass::RefusedDeprecatedRowMissingExpiry,
                    "deprecated rows must publish removal target version or date",
                );
            }
            if row
                .deprecation
                .migration_guide_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                return refused(
                    LifecycleMetadataReasonClass::RefusedDeprecatedRowMissingMigrationGuide,
                    "deprecated rows must cite migration guidance",
                );
            }
        }
    }
    if rows.iter().any(row_is_deprecated) {
        return (
            LifecycleMetadataDecisionClass::DeprecatedMigrationRequired,
            LifecycleMetadataReasonClass::DeprecatedRowsCarryMigration,
            "lifecycle metadata is complete and deprecated rows carry replacement and expiry guidance".to_string(),
        );
    }
    (
        LifecycleMetadataDecisionClass::ReadyForBetaUse,
        LifecycleMetadataReasonClass::AllRowsGoverned,
        "lifecycle metadata is complete for declared beta surfaces".to_string(),
    )
}

fn validate_deprecated_row(
    row: &LifecycleMetadataRow,
    findings: &mut Vec<LifecycleMetadataFinding>,
) {
    if replacement_or_no_direct_missing(&row.deprecation) {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.deprecated_replacement_or_no_direct_missing",
            "deprecated rows must name a replacement or an explicit no-direct-replacement reason",
        ));
    }
    if expiry_missing(&row.deprecation) {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.deprecated_expiry_missing",
            "deprecated rows must publish removal target version or date",
        ));
    }
    if row
        .deprecation
        .migration_guide_ref
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
    {
        findings.push(LifecycleMetadataFinding::row(
            &row.row_id,
            "lifecycle_metadata.deprecated_migration_guide_missing",
            "deprecated rows must cite migration guidance",
        ));
    }
}

fn support_window_missing(window: &LifecycleSupportWindow) -> bool {
    window.introduced_in_version.trim().is_empty()
        || window.minimum_overlap.trim().is_empty()
        || window.support_window_summary.trim().is_empty()
}

fn row_is_deprecated(row: &LifecycleMetadataRow) -> bool {
    matches!(
        row.stability_label,
        LifecycleStabilityLabel::Deprecated | LifecycleStabilityLabel::Retired
    ) || row.deprecation.deprecation_posture_class
        != LifecycleDeprecationPostureClass::NotDeprecated
}

fn replacement_or_no_direct_missing(deprecation: &LifecycleDeprecationMetadata) -> bool {
    deprecation
        .replacement_surface_ref
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
        && deprecation
            .no_direct_replacement_reason
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
}

fn expiry_missing(deprecation: &LifecycleDeprecationMetadata) -> bool {
    deprecation
        .removal_target_version
        .as_deref()
        .unwrap_or("")
        .trim()
        .is_empty()
        && deprecation
            .removal_target_date
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
}

fn refused(
    reason: LifecycleMetadataReasonClass,
    summary: &str,
) -> (
    LifecycleMetadataDecisionClass,
    LifecycleMetadataReasonClass,
    String,
) {
    (
        LifecycleMetadataDecisionClass::RefusedIncompleteMetadata,
        reason,
        summary.to_string(),
    )
}
