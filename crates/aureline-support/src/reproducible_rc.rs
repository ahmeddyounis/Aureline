//! Beta reproducible release-candidate support/export projection.
//!
//! This module consumes the generated support projection at
//! `/artifacts/release/m3/reproducible_rc_packet/support_export_projection.json`.
//! Support and partner-proof packets use this projection instead of parsing
//! release notes or clean-room rebuild prose, so exact-build identity,
//! rebuilt artifact graph comparisons, and blocking publication checks stay
//! machine-readable.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for generated reproducible release-candidate support exports.
pub const REPRODUCIBLE_RC_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for generated reproducible release-candidate support exports.
pub const REPRODUCIBLE_RC_SUPPORT_EXPORT_RECORD_KIND: &str = "reproducible_rc_support_export";

/// Repository-relative path to the generated reproducible release-candidate support export.
pub const CURRENT_REPRODUCIBLE_RC_SUPPORT_EXPORT_PATH: &str =
    "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json";

const CURRENT_REPRODUCIBLE_RC_SUPPORT_EXPORT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m3/reproducible_rc_packet/support_export_projection.json"
));

const REQUIRED_CONSUMER_REFS: &[&str] = &[
    "docs/release/m3/reproducible_rc_beta.md",
    "docs/help/m3/release_truth_surfaces.md",
    "docs/release/m3/packaging_and_signing_beta.md",
    "fixtures/release/m3/reproducible_rc/manifest.yaml",
];

/// Loads the checked-in reproducible release-candidate support export.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in projection no longer matches
/// [`ReproducibleRcSupportExport`].
pub fn current_reproducible_rc_support_export(
) -> Result<ReproducibleRcSupportExport, serde_json::Error> {
    serde_json::from_str(CURRENT_REPRODUCIBLE_RC_SUPPORT_EXPORT_JSON)
}

/// Metadata-only support export generated from reproducible release-candidate evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproducibleRcSupportExport {
    /// Integer schema version for this support-export shape.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable support projection id.
    pub projection_id: String,
    /// Source packet id that generated this projection.
    pub packet_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Repository-relative packet ref.
    pub source_packet_ref: String,
    /// Rebuilt artifact graph snapshot ref.
    pub rebuilt_artifact_graph_ref: String,
    /// Release candidate ref under comparison.
    pub release_candidate_ref: String,
    /// Artifact bundle ref under comparison.
    pub artifact_bundle_ref: String,
    /// Exact-build identity shared by the candidate and comparison rows.
    pub exact_build_identity_ref: String,
    /// Clean-room rebuild packet ref.
    pub clean_room_rebuild_packet_ref: String,
    /// Clean-room rebuild validation capture ref.
    pub clean_room_rebuild_capture_ref: String,
    /// Redaction class applied to every row.
    pub redaction_class: String,
    /// Whether private raw material is excluded from the projection.
    pub raw_private_material_excluded: bool,
    /// Publication claim state exposed by this projection.
    pub claim_state: String,
    /// Aggregate comparison and publication-check counts.
    pub summary: ReproducibleRcSummary,
    /// Clean-room evidence row shared with support exports.
    pub clean_room_evidence: CleanRoomEvidenceProjection,
    /// Artifact graph comparison rows.
    pub artifact_graph_checks: Vec<ArtifactGraphCheck>,
    /// Blocking publication checks.
    pub publication_checks: Vec<PublicationCheckProjection>,
    /// Consumer refs that are allowed to quote this projection.
    pub consumer_refs: Vec<String>,
}

impl ReproducibleRcSupportExport {
    /// Validates support/export invariants before a caller exposes the packet.
    pub fn validate(&self) -> Vec<ReproducibleRcViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REPRODUCIBLE_RC_SUPPORT_EXPORT_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "support_export.schema_version",
                &self.packet_id,
                "reproducible RC support export schema_version must be 1",
            );
        }
        if self.record_kind != REPRODUCIBLE_RC_SUPPORT_EXPORT_RECORD_KIND {
            push_violation(
                &mut violations,
                "support_export.record_kind",
                &self.packet_id,
                "reproducible RC support export record_kind is not supported",
            );
        }
        if self.redaction_class != "metadata_only_no_package_bytes" {
            push_violation(
                &mut violations,
                "support_export.redaction_class",
                &self.redaction_class,
                "reproducible RC support export must remain metadata-only",
            );
        }
        if !self.raw_private_material_excluded {
            push_violation(
                &mut violations,
                "support_export.raw_private_material_excluded",
                &self.packet_id,
                "reproducible RC support export must exclude raw private material",
            );
        }
        if !self
            .exact_build_identity_ref
            .starts_with("build-id:aureline:")
        {
            push_violation(
                &mut violations,
                "support_export.exact_build_identity_ref",
                &self.exact_build_identity_ref,
                "exact-build identity must use the Aureline build-id vocabulary",
            );
        }
        if !self
            .rebuilt_artifact_graph_ref
            .starts_with("artifacts/release/m3/reproducible_rc_packet/")
        {
            push_violation(
                &mut violations,
                "support_export.rebuilt_artifact_graph_ref",
                &self.rebuilt_artifact_graph_ref,
                "rebuilt graph ref must resolve through the reproducible RC packet directory",
            );
        }

        self.summary.validate(
            self.artifact_graph_checks.len(),
            self.publication_checks.len(),
            &mut violations,
        );
        self.clean_room_evidence
            .validate(&self.exact_build_identity_ref, &mut violations);
        self.validate_artifact_checks(&mut violations);
        self.validate_publication_checks(&mut violations);
        self.validate_consumer_refs(&mut violations);

        violations
    }

    fn validate_artifact_checks(&self, violations: &mut Vec<ReproducibleRcViolation>) {
        let mut row_ids = BTreeSet::new();
        for row in &self.artifact_graph_checks {
            row.validate(&self.exact_build_identity_ref, violations);
            if !row_ids.insert(row.node_id.as_str()) {
                push_violation(
                    violations,
                    "artifact_graph_check.duplicate_node_id",
                    &row.node_id,
                    "artifact graph comparison rows must have unique node ids",
                );
            }
        }
    }

    fn validate_publication_checks(&self, violations: &mut Vec<ReproducibleRcViolation>) {
        let mut check_ids = BTreeSet::new();
        for row in &self.publication_checks {
            row.validate(violations);
            if !check_ids.insert(row.check_id.as_str()) {
                push_violation(
                    violations,
                    "publication_check.duplicate_check_id",
                    &row.check_id,
                    "publication check ids must be unique",
                );
            }
        }
    }

    fn validate_consumer_refs(&self, violations: &mut Vec<ReproducibleRcViolation>) {
        let refs = self
            .consumer_refs
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for required in REQUIRED_CONSUMER_REFS {
            if !refs.contains(required) {
                push_violation(
                    violations,
                    "consumer_refs.required_missing",
                    required,
                    "support export must name every release, help, and fixture consumer",
                );
            }
        }
    }
}

/// Aggregate comparison and publication-check counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproducibleRcSummary {
    /// Candidate-required artifact rows compared.
    pub required_artifact_count: u32,
    /// Artifact rows whose rebuilt digest matches the promoted digest.
    pub matched_artifact_count: u32,
    /// Artifact rows whose rebuilt digest differs from the promoted digest.
    pub mismatched_artifact_count: u32,
    /// Artifact rows intentionally handled by non-digest comparison.
    pub non_comparable_artifact_count: u32,
    /// Blocking publication checks evaluated.
    pub publication_check_count: u32,
    /// Blocking publication checks currently failing.
    pub blocking_failure_count: u32,
    /// Clean-room evidence state.
    pub clean_room_evidence_state: String,
    /// Rebuild comparison result state.
    pub rebuild_result_state: String,
    /// Whether binary byte identity is claimed.
    pub byte_identity_claimed: bool,
}

impl ReproducibleRcSummary {
    fn validate(
        &self,
        artifact_check_count: usize,
        publication_check_count: usize,
        violations: &mut Vec<ReproducibleRcViolation>,
    ) {
        let total = self.matched_artifact_count
            + self.mismatched_artifact_count
            + self.non_comparable_artifact_count;
        if total != self.required_artifact_count {
            push_violation(
                violations,
                "summary.artifact_count_mismatch",
                "summary",
                "matched, mismatched, and non-comparable artifact counts must add to required_artifact_count",
            );
        }
        if self.required_artifact_count as usize != artifact_check_count {
            push_violation(
                violations,
                "summary.required_artifact_count",
                "summary",
                "required_artifact_count must equal artifact_graph_checks length",
            );
        }
        if self.publication_check_count as usize != publication_check_count {
            push_violation(
                violations,
                "summary.publication_check_count",
                "summary",
                "publication_check_count must equal publication_checks length",
            );
        }
        if self.mismatched_artifact_count != 0 {
            push_violation(
                violations,
                "summary.mismatched_artifact_count",
                "summary",
                "support export cannot expose mismatched clean-room artifact rows",
            );
        }
        if self.blocking_failure_count != 0 {
            push_violation(
                violations,
                "summary.blocking_failure_count",
                "summary",
                "support export cannot expose failing publication checks",
            );
        }
        if self.clean_room_evidence_state != "clean_room_rebuild_rehearsal_current" {
            push_violation(
                violations,
                "summary.clean_room_evidence_state",
                &self.clean_room_evidence_state,
                "clean-room evidence must be current",
            );
        }
        if self.rebuild_result_state != "rebuilt_artifact_graph_matches_promoted_candidate" {
            push_violation(
                violations,
                "summary.rebuild_result_state",
                &self.rebuild_result_state,
                "rebuilt artifact graph must match the promoted candidate",
            );
        }
        if self.byte_identity_claimed {
            push_violation(
                violations,
                "summary.byte_identity_claimed",
                "summary",
                "beta reproducibility packet must not claim binary byte identity",
            );
        }
    }
}

/// Clean-room evidence row shared with support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanRoomEvidenceProjection {
    /// Clean-room rebuild command.
    pub lane_command: String,
    /// Offline clean-room rebuild command.
    pub offline_command: String,
    /// Current evidence state.
    pub evidence_state: String,
    /// Rebuild result state.
    pub rebuild_result_state: String,
    /// Accepted rebuild states from the packet.
    pub accepted_rebuild_states: Vec<String>,
    /// Build identity anchor ref.
    pub build_identity_ref: String,
    /// Clean-room packet ref.
    pub packet_ref: String,
    /// Clean-room validation capture ref.
    pub capture_ref: String,
}

impl CleanRoomEvidenceProjection {
    fn validate(
        &self,
        _exact_build_identity_ref: &str,
        violations: &mut Vec<ReproducibleRcViolation>,
    ) {
        if self.evidence_state != "clean_room_rebuild_rehearsal_current" {
            push_violation(
                violations,
                "clean_room_evidence.evidence_state",
                &self.evidence_state,
                "clean-room evidence must be current before support export",
            );
        }
        if self.rebuild_result_state != "rebuilt_artifact_graph_matches_promoted_candidate" {
            push_violation(
                violations,
                "clean_room_evidence.rebuild_result_state",
                &self.rebuild_result_state,
                "clean-room rebuild result must match the promoted graph",
            );
        }
        if !self
            .accepted_rebuild_states
            .iter()
            .any(|state| state == "rehearsal_only")
        {
            push_violation(
                violations,
                "clean_room_evidence.accepted_rebuild_states",
                &self.packet_ref,
                "accepted rebuild states must include the current clean-room rehearsal state",
            );
        }
        if self.build_identity_ref != "artifacts/build/build_identity.json" {
            push_violation(
                violations,
                "clean_room_evidence.build_identity_ref",
                &self.build_identity_ref,
                "clean-room evidence must compare against the checked-in build identity anchor",
            );
        }
        if !self
            .packet_ref
            .ends_with("clean_room_rebuild_rehearsal/packet.md")
        {
            push_violation(
                violations,
                "clean_room_evidence.packet_ref",
                &self.packet_ref,
                "clean-room evidence must quote the rebuild rehearsal packet",
            );
        }
        if !self
            .capture_ref
            .ends_with("clean_room_rebuild_rehearsal_validation_capture.json")
        {
            push_violation(
                violations,
                "clean_room_evidence.capture_ref",
                &self.capture_ref,
                "clean-room evidence must quote the validation capture",
            );
        }
    }
}

/// Artifact graph comparison row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactGraphCheck {
    /// Artifact node id from the promoted graph.
    pub node_id: String,
    /// Artifact family class from the promoted graph.
    pub family_class: String,
    /// Artifact role from the promoted graph.
    pub artifact_role: String,
    /// Exact-build identity ref for this row.
    pub exact_build_identity_ref: String,
    /// Digest recorded by the promoted graph projection.
    pub promoted_digest: Option<String>,
    /// Digest recorded by the rebuilt graph snapshot.
    pub rebuilt_digest: Option<String>,
    /// Whether the digests match.
    pub digest_match: bool,
    /// Comparison state such as `matched` or `self_referential_projection`.
    pub comparison_state: String,
    /// Support/export ref for this row.
    pub support_ref: String,
}

impl ArtifactGraphCheck {
    fn validate(
        &self,
        exact_build_identity_ref: &str,
        violations: &mut Vec<ReproducibleRcViolation>,
    ) {
        if self.exact_build_identity_ref != exact_build_identity_ref {
            push_violation(
                violations,
                "artifact_graph_check.exact_build_identity_ref",
                &self.node_id,
                "artifact graph check must use the support export exact-build identity",
            );
        }
        if !self
            .support_ref
            .starts_with(CURRENT_REPRODUCIBLE_RC_SUPPORT_EXPORT_PATH)
        {
            push_violation(
                violations,
                "artifact_graph_check.support_ref",
                &self.node_id,
                "artifact graph check support_ref must point into the generated support export",
            );
        }
        if self.comparison_state == "matched" {
            if !self.digest_match {
                push_violation(
                    violations,
                    "artifact_graph_check.digest_match",
                    &self.node_id,
                    "matched artifact graph checks must set digest_match",
                );
            }
            if self.promoted_digest.is_none() || self.promoted_digest != self.rebuilt_digest {
                push_violation(
                    violations,
                    "artifact_graph_check.digest_mismatch",
                    &self.node_id,
                    "rebuilt digest must equal promoted digest for matched rows",
                );
            }
        } else if self.comparison_state == "self_referential_projection" {
            if self.digest_match || self.promoted_digest.is_some() || self.rebuilt_digest.is_some()
            {
                push_violation(
                    violations,
                    "artifact_graph_check.self_referential_digest",
                    &self.node_id,
                    "self-referential projection rows must not claim digest identity",
                );
            }
        } else {
            push_violation(
                violations,
                "artifact_graph_check.comparison_state",
                &self.node_id,
                "artifact graph check comparison_state is not supported",
            );
        }
    }
}

/// Blocking publication check projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationCheckProjection {
    /// Stable check id.
    pub check_id: String,
    /// Check class.
    pub check_class: String,
    /// Evidence source ref.
    pub source_ref: String,
    /// State required for publication.
    pub required_state: String,
    /// State observed by the validator.
    pub actual_state: String,
    /// Whether this row blocks publication on failure.
    pub blocks_publication: bool,
    /// Support/export ref for this check.
    pub support_ref: String,
}

impl PublicationCheckProjection {
    fn validate(&self, violations: &mut Vec<ReproducibleRcViolation>) {
        if self.actual_state != self.required_state {
            push_violation(
                violations,
                "publication_check.state_mismatch",
                &self.check_id,
                "publication check must meet its required state",
            );
        }
        if !self.blocks_publication {
            push_violation(
                violations,
                "publication_check.not_blocking",
                &self.check_id,
                "publication check must block publication on failure",
            );
        }
        if !self
            .support_ref
            .starts_with(CURRENT_REPRODUCIBLE_RC_SUPPORT_EXPORT_PATH)
        {
            push_violation(
                violations,
                "publication_check.support_ref",
                &self.check_id,
                "publication check support_ref must point into the generated support export",
            );
        }
    }
}

/// Validation failure emitted while checking a reproducible release-candidate support export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibleRcViolation {
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe ref associated with the violation.
    pub ref_id: String,
    /// Redaction-safe validation message.
    pub message: String,
}

fn push_violation(
    violations: &mut Vec<ReproducibleRcViolation>,
    check_id: &str,
    ref_id: &str,
    message: &str,
) {
    violations.push(ReproducibleRcViolation {
        check_id: check_id.to_owned(),
        ref_id: ref_id.to_owned(),
        message: message.to_owned(),
    });
}
