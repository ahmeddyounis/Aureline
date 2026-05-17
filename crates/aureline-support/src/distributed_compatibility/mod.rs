//! Beta distributed-compatibility support/export projection.
//!
//! This module consumes the generated support projection at
//! `/artifacts/release/m3/distributed_compatibility/support_export_projection.json`.
//! The projection is derived from the generated distributed compatibility
//! manifests, not release-note prose, so support exports quote the same
//! compatibility rows, skew cases, out-of-window postures, and repair hints as
//! the release packet.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version for generated distributed compatibility support exports.
pub const DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for generated distributed compatibility support exports.
pub const DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "distributed_compatibility_support_export";

/// Repository-relative path to the generated distributed compatibility support export.
pub const CURRENT_DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_PATH: &str =
    "artifacts/release/m3/distributed_compatibility/support_export_projection.json";

const CURRENT_DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m3/distributed_compatibility/support_export_projection.json"
));

const REQUIRED_FAMILIES: &[&str] = &["client_helper", "client_extension", "schema", "provider"];

/// Loads the checked-in generated distributed compatibility support export.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in projection no longer matches
/// [`DistributedCompatibilitySupportExport`].
pub fn current_distributed_compatibility_support_export(
) -> Result<DistributedCompatibilitySupportExport, serde_json::Error> {
    serde_json::from_str(CURRENT_DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_JSON)
}

/// Metadata-only support export generated from distributed compatibility manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedCompatibilitySupportExport {
    /// Integer schema version for this support-export shape.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable support-export id.
    pub export_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Manifest index ref that produced this projection.
    pub source_index_ref: String,
    /// Release packet ref that consumes the same manifests.
    pub release_packet_ref: String,
    /// Redaction class applied to every row.
    pub redaction_class: String,
    /// Whether private raw material is excluded from the projection.
    pub raw_private_material_excluded: bool,
    /// Manifest family summaries covered by the export.
    pub manifest_families: Vec<DistributedManifestFamily>,
    /// Skew harness summary shared with the release packet.
    pub harness_summary: DistributedSkewHarnessSummary,
    /// Row-level support/export projections.
    pub support_rows: Vec<DistributedCompatibilitySupportRow>,
}

impl DistributedCompatibilitySupportExport {
    /// Validates support/export invariants before a caller exposes the packet.
    pub fn validate(&self) -> Vec<DistributedCompatibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "support_export.schema_version",
                &self.export_id,
                "distributed compatibility support export schema_version must be 1",
            );
        }
        if self.record_kind != DISTRIBUTED_COMPATIBILITY_SUPPORT_EXPORT_RECORD_KIND {
            push_violation(
                &mut violations,
                "support_export.record_kind",
                &self.export_id,
                "distributed compatibility support export record_kind is not supported",
            );
        }
        if self.redaction_class != "metadata_safe_default" {
            push_violation(
                &mut violations,
                "support_export.redaction_class",
                &self.export_id,
                "distributed compatibility support export must use metadata_safe_default",
            );
        }
        if !self.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "support_export.raw_private_material_excluded",
                &self.export_id,
                "distributed compatibility support export must exclude raw private material",
            );
        }
        if !self
            .source_index_ref
            .starts_with("artifacts/compat/m3/distributed_manifests/")
        {
            push_violation(
                &mut violations,
                "support_export.source_index_ref",
                &self.source_index_ref,
                "source index must resolve through the generated distributed manifest directory",
            );
        }

        self.validate_family_coverage(&mut violations);
        self.harness_summary
            .validate(&self.export_id, &mut violations);
        self.validate_rows(&mut violations);

        violations
    }

    fn validate_family_coverage(&self, violations: &mut Vec<DistributedCompatibilityViolation>) {
        let families = self
            .manifest_families
            .iter()
            .map(|family| family.manifest_family.as_str())
            .collect::<BTreeSet<_>>();
        for required in REQUIRED_FAMILIES {
            if !families.contains(required) {
                push_violation(
                    violations,
                    "manifest_families.required_missing",
                    required,
                    "support export must include every generated distributed compatibility family",
                );
            }
        }
        for family in &self.manifest_families {
            family.validate(violations);
        }
    }

    fn validate_rows(&self, violations: &mut Vec<DistributedCompatibilityViolation>) {
        let family_refs = self
            .manifest_families
            .iter()
            .map(|family| {
                (
                    family.manifest_family.as_str(),
                    family.manifest_ref.as_str(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut support_row_ids = BTreeSet::new();
        for row in &self.support_rows {
            row.validate(&self.release_packet_ref, &family_refs, violations);
            if !support_row_ids.insert(row.support_row_id.as_str()) {
                push_violation(
                    violations,
                    "support_rows.duplicate_id",
                    &row.support_row_id,
                    "support row ids must be unique",
                );
            }
        }
    }
}

/// Summary for one generated distributed compatibility manifest family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedManifestFamily {
    /// Family token such as `client_helper`.
    pub manifest_family: String,
    /// Repository-relative manifest ref.
    pub manifest_ref: String,
    /// Stable generated manifest id.
    pub manifest_id: String,
    /// Number of compatibility rows in the family manifest.
    pub row_count: u32,
}

impl DistributedManifestFamily {
    fn validate(&self, violations: &mut Vec<DistributedCompatibilityViolation>) {
        if self.row_count == 0 {
            push_violation(
                violations,
                "manifest_family.row_count_empty",
                &self.manifest_family,
                "manifest family must expose at least one support row",
            );
        }
        if !self
            .manifest_ref
            .starts_with("artifacts/compat/m3/distributed_manifests/")
        {
            push_violation(
                violations,
                "manifest_family.ref_not_generated",
                &self.manifest_ref,
                "manifest family ref must point at the generated manifest directory",
            );
        }
        if !self
            .manifest_id
            .starts_with(&format!("distributed_compat:{}.", self.manifest_family))
        {
            push_violation(
                violations,
                "manifest_family.id_mismatch",
                &self.manifest_id,
                "manifest id must carry the same family token as manifest_family",
            );
        }
    }
}

/// Skew-harness totals shared by release and support/export projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedSkewHarnessSummary {
    /// Number of harness cases loaded.
    pub total_cases: u32,
    /// Number of passing harness cases.
    pub passed_cases: u32,
    /// Number of failing harness cases.
    pub failed_cases: u32,
    /// Number of cases exercising supported skew windows.
    pub supported_case_count: u32,
    /// Number of cases exercising unsupported skew windows.
    pub unsupported_case_count: u32,
    /// Number of cases that block mutation.
    pub mutation_blocked_case_count: u32,
}

impl DistributedSkewHarnessSummary {
    fn validate(&self, owner: &str, violations: &mut Vec<DistributedCompatibilityViolation>) {
        if self.total_cases != self.passed_cases + self.failed_cases {
            push_violation(
                violations,
                "harness_summary.total_mismatch",
                owner,
                "harness total must equal passed plus failed cases",
            );
        }
        if self.failed_cases != 0 {
            push_violation(
                violations,
                "harness_summary.failed_cases",
                owner,
                "distributed compatibility support export cannot expose failed harness cases",
            );
        }
        if self.supported_case_count < REQUIRED_FAMILIES.len() as u32 {
            push_violation(
                violations,
                "harness_summary.supported_coverage",
                owner,
                "harness must include at least one supported case per required family",
            );
        }
        if self.unsupported_case_count < REQUIRED_FAMILIES.len() as u32 {
            push_violation(
                violations,
                "harness_summary.unsupported_coverage",
                owner,
                "harness must include at least one unsupported case per required family",
            );
        }
    }
}

/// Row-level support/export projection for one distributed compatibility row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DistributedCompatibilitySupportRow {
    /// Stable support-export row id.
    pub support_row_id: String,
    /// Family manifest ref that produced this row.
    pub manifest_ref: String,
    /// Family manifest row id.
    pub manifest_row_id: String,
    /// Manifest family token.
    pub manifest_family: String,
    /// Seeded compatibility row ref.
    pub compatibility_row_ref: String,
    /// Generated compatibility-report row ref.
    pub compatibility_report_row_ref: String,
    /// Effective support class after downgrades.
    pub support_class_effective: String,
    /// Skew-window class for the row.
    pub skew_window_class: String,
    /// Skew-window declaration ref when the window is not inline.
    pub skew_window_ref: Option<String>,
    /// Current supported skew case ref.
    pub current_skew_case_ref: String,
    /// Unsupported skew case refs projected into support/export.
    pub unsupported_case_refs: Vec<String>,
    /// Out-of-window posture.
    pub out_of_window_posture: String,
    /// Typed repair hints or safe continuation actions.
    pub repair_hints: Vec<String>,
    /// Release packet ref that consumes the same generated manifest row.
    pub release_packet_ref: String,
}

impl DistributedCompatibilitySupportRow {
    fn validate(
        &self,
        release_packet_ref: &str,
        family_refs: &BTreeMap<&str, &str>,
        violations: &mut Vec<DistributedCompatibilityViolation>,
    ) {
        if !self
            .support_row_id
            .starts_with("support_export:distributed_compatibility.")
        {
            push_violation(
                violations,
                "support_row.id_prefix",
                &self.support_row_id,
                "support row id must use the distributed compatibility support-export prefix",
            );
        }
        match family_refs.get(self.manifest_family.as_str()) {
            Some(expected_ref) if *expected_ref == self.manifest_ref => {}
            _ => push_violation(
                violations,
                "support_row.manifest_ref_mismatch",
                &self.support_row_id,
                "support row manifest_ref must match its manifest family summary",
            ),
        }
        if self.release_packet_ref != release_packet_ref {
            push_violation(
                violations,
                "support_row.release_packet_ref_mismatch",
                &self.support_row_id,
                "support row release_packet_ref must match the export release packet ref",
            );
        }
        if !self.compatibility_row_ref.starts_with("compat_row:") {
            push_violation(
                violations,
                "support_row.compatibility_row_ref",
                &self.compatibility_row_ref,
                "support row must cite a seeded compatibility row ref",
            );
        }
        if !self.current_skew_case_ref.starts_with("skew_case:") {
            push_violation(
                violations,
                "support_row.current_skew_case_ref",
                &self.current_skew_case_ref,
                "support row must cite the current skew case",
            );
        }
        if self.unsupported_case_refs.is_empty() {
            push_violation(
                violations,
                "support_row.unsupported_case_refs_empty",
                &self.support_row_id,
                "support row must expose unsupported skew cases",
            );
        }
        if self.repair_hints.is_empty() {
            push_violation(
                violations,
                "support_row.repair_hints_empty",
                &self.support_row_id,
                "support row must expose at least one repair or continuation hint",
            );
        }
    }
}

/// Validation finding for distributed compatibility support/export data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedCompatibilityViolation {
    /// Stable check id.
    pub check_id: String,
    /// Row, packet, or ref associated with the finding.
    pub subject: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<DistributedCompatibilityViolation>,
    check_id: &str,
    subject: &str,
    message: &str,
) {
    violations.push(DistributedCompatibilityViolation {
        check_id: check_id.to_owned(),
        subject: subject.to_owned(),
        message: message.to_owned(),
    });
}
