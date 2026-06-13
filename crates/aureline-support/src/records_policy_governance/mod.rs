//! Support-export projection for records/policy governance truth.
//!
//! This module consumes the checked-in records-governance matrix and the stable
//! policy-governance snapshot, then emits one support-export packet that can
//! prove durable managed/provider/support artifact families share one truthful
//! answer for record class, delete/export honesty, chronology, and policy
//! simulation.

use serde::{Deserialize, Serialize};

use aureline_policy::{
    seeded_records_policy_governance_snapshot, PolicyGovernanceFamily,
    PolicyGovernanceScopeSnapshot,
};
use aureline_records::records_policy_simulation_matrix::{
    current_records_policy_matrix, GovernedArtifactFamily, RecordsPolicyGapReason,
    RecordsPolicyPublicationDecision, RecordsPolicySimulationMatrix,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the support-export packet.
pub const RECORDS_POLICY_GOVERNANCE_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the support-export packet.
pub const RECORDS_POLICY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "records_policy_governance_support_export";

/// One support-side coherence issue between the records matrix and policy snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordsPolicyGovernanceIssue {
    /// Affected matrix row id.
    pub entry_id: String,
    /// Stable issue code.
    pub issue_code: String,
    /// Reviewable message.
    pub message: String,
}

/// Metadata-only support-export packet joining records and policy truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordsPolicyGovernanceSupportExport {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC timestamp when the export was generated.
    pub generated_at: String,
    /// Source matrix reference.
    pub matrix_ref: String,
    /// Source policy snapshot reference.
    pub policy_snapshot_ref: String,
    /// Embedded checked-in matrix.
    pub matrix: RecordsPolicySimulationMatrix,
    /// Embedded policy-governance snapshot.
    pub policy_snapshot: PolicyGovernanceScopeSnapshot,
    /// Release-blocking rows that currently force publication hold.
    pub blocking_row_ids: Vec<String>,
    /// Narrowed rows that support surfaces must disclose.
    pub narrowed_row_ids: Vec<String>,
    /// Cross-packet coherence issues.
    pub coherence_issues: Vec<RecordsPolicyGovernanceIssue>,
    /// Metadata-only redaction posture.
    pub raw_private_material_excluded: bool,
    /// Reviewable summary.
    pub support_summary: String,
}

impl RecordsPolicyGovernanceSupportExport {
    /// Builds a support-export packet from the current matrix and policy snapshot.
    pub fn current() -> Result<Self, RecordsPolicyGovernanceLoadError> {
        let matrix =
            current_records_policy_matrix().map_err(RecordsPolicyGovernanceLoadError::Matrix)?;
        let policy_snapshot = seeded_records_policy_governance_snapshot();
        Ok(Self::from_parts(matrix, policy_snapshot))
    }

    /// Builds a support-export packet from caller-supplied parts.
    pub fn from_parts(
        matrix: RecordsPolicySimulationMatrix,
        policy_snapshot: PolicyGovernanceScopeSnapshot,
    ) -> Self {
        let blocking_row_ids = matrix
            .rows
            .iter()
            .filter(|row| row.release_blocking && row.needs_review())
            .map(|row| row.entry_id.clone())
            .collect::<Vec<_>>();
        let narrowed_row_ids = matrix
            .rows
            .iter()
            .filter(|row| row.needs_review())
            .map(|row| row.entry_id.clone())
            .collect::<Vec<_>>();
        let coherence_issues = coherence_issues(&matrix, &policy_snapshot);
        let support_summary = format!(
            "Records/policy governance export covering {} governed rows; {} narrowed rows; publication decision {}.",
            matrix.rows.len(),
            narrowed_row_ids.len(),
            matrix.publication.decision.as_str()
        );

        Self {
            schema_version: RECORDS_POLICY_GOVERNANCE_SUPPORT_EXPORT_SCHEMA_VERSION,
            record_kind: RECORDS_POLICY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            export_id: "support-export:records-policy-governance:v1".to_owned(),
            generated_at: matrix.as_of.clone(),
            matrix_ref: aureline_records::records_policy_simulation_matrix::RECORDS_POLICY_SIMULATION_MATRIX_PATH
                .to_owned(),
            policy_snapshot_ref: "policy:records_policy_governance_snapshot:v1".to_owned(),
            matrix,
            policy_snapshot,
            blocking_row_ids,
            narrowed_row_ids,
            coherence_issues,
            raw_private_material_excluded: true,
            support_summary,
        }
    }

    /// Validates structural and cross-packet invariants.
    pub fn validate(&self) -> Vec<RecordsPolicyGovernanceIssue> {
        let mut issues = Vec::new();

        for violation in self.matrix.validate() {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: "matrix".to_owned(),
                issue_code: "matrix_violation".to_owned(),
                message: format!("{violation:?}"),
            });
        }
        for defect in self.policy_snapshot.validate() {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: defect.scope_ref,
                issue_code: "policy_snapshot_defect".to_owned(),
                message: defect.message,
            });
        }
        if !self.raw_private_material_excluded {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: self.export_id.clone(),
                issue_code: "raw_private_material_exposed".to_owned(),
                message: "support export must remain metadata-only".to_owned(),
            });
        }
        issues.extend(coherence_issues(&self.matrix, &self.policy_snapshot));
        issues
    }
}

/// Load error emitted while constructing the support-export packet.
#[derive(Debug)]
pub enum RecordsPolicyGovernanceLoadError {
    /// The checked-in matrix could not be parsed.
    Matrix(serde_yaml::Error),
}

impl std::fmt::Display for RecordsPolicyGovernanceLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Matrix(error) => write!(f, "records policy matrix: {error}"),
        }
    }
}

impl std::error::Error for RecordsPolicyGovernanceLoadError {}

fn coherence_issues(
    matrix: &RecordsPolicySimulationMatrix,
    policy_snapshot: &PolicyGovernanceScopeSnapshot,
) -> Vec<RecordsPolicyGovernanceIssue> {
    let mut issues = Vec::new();

    for row in &matrix.rows {
        let family = family_for_row(row.artifact_family);
        let Some(policy_row) = policy_snapshot.row_for_family(family) else {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: row.entry_id.clone(),
                issue_code: "missing_policy_snapshot_row".to_owned(),
                message: "matrix row has no matching policy snapshot row".to_owned(),
            });
            continue;
        };

        if row.policy_contract.policy_simulation_required && !policy_row.policy_simulation_supported
        {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: row.entry_id.clone(),
                issue_code: "policy_simulation_missing".to_owned(),
                message: "matrix row requires policy simulation but snapshot does not support it"
                    .to_owned(),
            });
        }
        if row.policy_contract.exception_preview_required && !policy_row.exception_preview_supported
        {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: row.entry_id.clone(),
                issue_code: "exception_preview_missing".to_owned(),
                message: "matrix row requires exception preview but snapshot does not support it"
                    .to_owned(),
            });
        }
        if row
            .policy_contract
            .remembered_decision_revalidation_required
            && !policy_row.remembered_decision_revalidation_supported
        {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: row.entry_id.clone(),
                issue_code: "remembered_decision_missing".to_owned(),
                message:
                    "matrix row requires remembered-decision revalidation but snapshot does not support it"
                        .to_owned(),
            });
        }
        for trigger in &row.policy_contract.required_reapproval_trigger_tokens {
            if !policy_row
                .required_reapproval_trigger_tokens
                .iter()
                .any(|candidate| candidate == trigger)
            {
                issues.push(RecordsPolicyGovernanceIssue {
                    entry_id: row.entry_id.clone(),
                    issue_code: "reapproval_trigger_missing".to_owned(),
                    message: format!("policy snapshot is missing required trigger {trigger}"),
                });
            }
        }
        if policy_row.chronology_refs.is_empty() {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: row.entry_id.clone(),
                issue_code: "chronology_refs_missing".to_owned(),
                message: "policy snapshot must preserve chronology refs".to_owned(),
            });
        }
        if row.release_blocking
            && matches!(
                row.active_gap_reasons.first(),
                Some(RecordsPolicyGapReason::ProofStale | RecordsPolicyGapReason::ProofMissing)
            )
            && matrix.publication.decision != RecordsPolicyPublicationDecision::Hold
        {
            issues.push(RecordsPolicyGovernanceIssue {
                entry_id: row.entry_id.clone(),
                issue_code: "release_hold_mismatch".to_owned(),
                message: "release-blocking stale or missing proof must force a publication hold"
                    .to_owned(),
            });
        }
    }

    issues
}

fn family_for_row(family: GovernedArtifactFamily) -> PolicyGovernanceFamily {
    match family {
        GovernedArtifactFamily::AiEvidencePacket => PolicyGovernanceFamily::AiEvidencePacket,
        GovernedArtifactFamily::ProviderLinkedWorkItem => {
            PolicyGovernanceFamily::ProviderLinkedWorkItem
        }
        GovernedArtifactFamily::CompanionContinuityPacket => {
            PolicyGovernanceFamily::CompanionContinuityPacket
        }
        GovernedArtifactFamily::IncidentSupportPacket => {
            PolicyGovernanceFamily::IncidentSupportPacket
        }
        GovernedArtifactFamily::SyncMirrorLedger => PolicyGovernanceFamily::SyncMirrorLedger,
        GovernedArtifactFamily::OffboardingRecord => PolicyGovernanceFamily::OffboardingRecord,
        GovernedArtifactFamily::BrowserHandoffManifest => {
            PolicyGovernanceFamily::BrowserHandoffManifest
        }
        GovernedArtifactFamily::SupportExportPacket => PolicyGovernanceFamily::SupportExportPacket,
    }
}
