//! Policy-governance snapshot for durable record families.
//!
//! This module projects the stable policy-simulation and expiry page into a
//! smaller metadata-only snapshot keyed by durable artifact family. Support and
//! export consumers use it to verify that records-governance rows claiming
//! policy simulation, exception preview, and remembered-decision revalidation
//! all point at one shared policy truth source.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::policy_simulation_and_expiry::{
    seeded_policy_simulation_and_expiry_page, PolicySimulationAndExpiryPage,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the policy-governance snapshot.
pub const RECORDS_POLICY_GOVERNANCE_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the snapshot packet.
pub const RECORDS_POLICY_GOVERNANCE_SNAPSHOT_RECORD_KIND: &str =
    "records_policy_governance_snapshot";

/// Stable record kind for snapshot rows.
pub const RECORDS_POLICY_GOVERNANCE_SCOPE_ROW_RECORD_KIND: &str =
    "records_policy_governance_scope_row";

/// Artifact-family token covered by the policy-governance snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyGovernanceFamily {
    AiEvidencePacket,
    ProviderLinkedWorkItem,
    CompanionContinuityPacket,
    IncidentSupportPacket,
    SyncMirrorLedger,
    OffboardingRecord,
    BrowserHandoffManifest,
    SupportExportPacket,
}

impl PolicyGovernanceFamily {
    /// Every family covered by the snapshot.
    pub const ALL: [Self; 8] = [
        Self::AiEvidencePacket,
        Self::ProviderLinkedWorkItem,
        Self::CompanionContinuityPacket,
        Self::IncidentSupportPacket,
        Self::SyncMirrorLedger,
        Self::OffboardingRecord,
        Self::BrowserHandoffManifest,
        Self::SupportExportPacket,
    ];
}

/// One metadata-only policy coverage row for a durable artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyGovernanceCoverageRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Durable artifact family governed by this row.
    pub family: PolicyGovernanceFamily,
    /// `true` when policy simulation must be available for this family.
    pub policy_simulation_supported: bool,
    /// `true` when exception preview must be available for this family.
    pub exception_preview_supported: bool,
    /// `true` when remembered-decision revalidation must be available.
    pub remembered_decision_revalidation_supported: bool,
    /// Stable trigger tokens that force reapproval.
    pub required_reapproval_trigger_tokens: Vec<String>,
    /// Chronology references preserved by the underlying policy page.
    pub chronology_refs: Vec<String>,
    /// Evidence refs preserved by the underlying policy page.
    pub evidence_refs: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Snapshot packet joining policy coverage rows for durable record families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyGovernanceScopeSnapshot {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Source page record id backing this snapshot.
    pub source_page_ref: String,
    /// UTC timestamp or date this snapshot reflects.
    pub as_of: String,
    /// Covered rows.
    pub rows: Vec<PolicyGovernanceCoverageRow>,
}

/// Defect kind emitted while checking the snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyGovernanceSnapshotDefectKind {
    /// One durable family has no coverage row.
    FamilyUncovered,
    /// Remembered-decision revalidation omits a required trigger.
    ReapprovalTriggerMissing,
    /// Policy coverage row omits chronology lineage.
    ChronologyMissing,
    /// Policy coverage row omits evidence linkage.
    EvidenceMissing,
}

/// One validation defect emitted by the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyGovernanceSnapshotDefect {
    /// Defect kind.
    pub defect_kind: PolicyGovernanceSnapshotDefectKind,
    /// Row id or family token affected by the defect.
    pub scope_ref: String,
    /// Reviewable message.
    pub message: String,
}

impl PolicyGovernanceScopeSnapshot {
    /// Validates that the snapshot covers every family and preserves policy truth.
    pub fn validate(&self) -> Vec<PolicyGovernanceSnapshotDefect> {
        let mut defects = Vec::new();

        let covered: BTreeSet<PolicyGovernanceFamily> =
            self.rows.iter().map(|row| row.family).collect();
        for family in PolicyGovernanceFamily::ALL {
            if !covered.contains(&family) {
                defects.push(PolicyGovernanceSnapshotDefect {
                    defect_kind: PolicyGovernanceSnapshotDefectKind::FamilyUncovered,
                    scope_ref: format!("{family:?}"),
                    message: "governed family is missing from the policy snapshot".to_owned(),
                });
            }
        }

        for row in &self.rows {
            if row.remembered_decision_revalidation_supported {
                for trigger in [
                    "target_drift",
                    "policy_drift",
                    "version_drift",
                    "authority_drift",
                ] {
                    if !row
                        .required_reapproval_trigger_tokens
                        .iter()
                        .any(|candidate| candidate == trigger)
                    {
                        defects.push(PolicyGovernanceSnapshotDefect {
                            defect_kind:
                                PolicyGovernanceSnapshotDefectKind::ReapprovalTriggerMissing,
                            scope_ref: row.row_id.clone(),
                            message: format!("missing required reapproval trigger {trigger}"),
                        });
                    }
                }
            }
            if row.chronology_refs.is_empty() {
                defects.push(PolicyGovernanceSnapshotDefect {
                    defect_kind: PolicyGovernanceSnapshotDefectKind::ChronologyMissing,
                    scope_ref: row.row_id.clone(),
                    message: "policy coverage row must preserve chronology refs".to_owned(),
                });
            }
            if row.evidence_refs.is_empty() {
                defects.push(PolicyGovernanceSnapshotDefect {
                    defect_kind: PolicyGovernanceSnapshotDefectKind::EvidenceMissing,
                    scope_ref: row.row_id.clone(),
                    message: "policy coverage row must preserve evidence refs".to_owned(),
                });
            }
        }

        defects
    }

    /// Returns the row registered for `family`, if present.
    pub fn row_for_family(
        &self,
        family: PolicyGovernanceFamily,
    ) -> Option<&PolicyGovernanceCoverageRow> {
        self.rows.iter().find(|row| row.family == family)
    }
}

/// Builds the seeded policy-governance snapshot from the stable policy page.
pub fn seeded_records_policy_governance_snapshot() -> PolicyGovernanceScopeSnapshot {
    let page = seeded_policy_simulation_and_expiry_page();
    snapshot_from_page(page)
}

fn snapshot_from_page(page: PolicySimulationAndExpiryPage) -> PolicyGovernanceScopeSnapshot {
    let chronology_refs = page.review_packet.chronology_refs.clone();
    let evidence_refs = page
        .review_packet
        .policy_diff_summary_refs
        .iter()
        .chain(page.review_packet.simulation_outcome_refs.iter())
        .chain(page.review_packet.expiry_banner_refs.iter())
        .cloned()
        .collect::<Vec<_>>();
    let triggers = page.review_packet.reapproval_trigger_tokens.clone();

    let rows = PolicyGovernanceFamily::ALL
        .into_iter()
        .map(|family| PolicyGovernanceCoverageRow {
            record_kind: RECORDS_POLICY_GOVERNANCE_SCOPE_ROW_RECORD_KIND.to_owned(),
            schema_version: RECORDS_POLICY_GOVERNANCE_SNAPSHOT_SCHEMA_VERSION,
            row_id: format!("records-policy-scope:{}", family_token(family)),
            family,
            policy_simulation_supported: true,
            exception_preview_supported: true,
            remembered_decision_revalidation_supported: true,
            required_reapproval_trigger_tokens: triggers.clone(),
            chronology_refs: chronology_refs.clone(),
            evidence_refs: evidence_refs.clone(),
            rationale:
                "Family inherits the stable policy simulation, exception preview, and remembered-decision review packet."
                    .to_owned(),
        })
        .collect();

    PolicyGovernanceScopeSnapshot {
        schema_version: RECORDS_POLICY_GOVERNANCE_SNAPSHOT_SCHEMA_VERSION,
        record_kind: RECORDS_POLICY_GOVERNANCE_SNAPSHOT_RECORD_KIND.to_owned(),
        snapshot_id: "records_policy_governance_snapshot:v1".to_owned(),
        source_page_ref: page.page_id,
        as_of: page.generated_at,
        rows,
    }
}

fn family_token(family: PolicyGovernanceFamily) -> &'static str {
    match family {
        PolicyGovernanceFamily::AiEvidencePacket => "ai_evidence_packet",
        PolicyGovernanceFamily::ProviderLinkedWorkItem => "provider_linked_work_item",
        PolicyGovernanceFamily::CompanionContinuityPacket => "companion_continuity_packet",
        PolicyGovernanceFamily::IncidentSupportPacket => "incident_support_packet",
        PolicyGovernanceFamily::SyncMirrorLedger => "sync_mirror_ledger",
        PolicyGovernanceFamily::OffboardingRecord => "offboarding_record",
        PolicyGovernanceFamily::BrowserHandoffManifest => "browser_handoff_manifest",
        PolicyGovernanceFamily::SupportExportPacket => "support_export_packet",
    }
}
