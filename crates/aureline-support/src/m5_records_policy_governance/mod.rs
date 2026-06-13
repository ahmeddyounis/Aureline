//! Support/export projection joining M5 hold/retention, exception/expiry, and
//! pre-apply policy-simulation truth.
//!
//! This module consumes the authoritative legal-hold and retention packet from
//! `aureline-records::m5_records_policy`, the policy exception/expiry packet
//! from `aureline-policy::m5_exception_expiry`, and the pre-apply policy
//! simulation packet from `aureline-records::m5_policy_simulation`, then exposes
//! one metadata-only support/export object. It lets support surfaces inspect
//! hold notices, hold selector scopes, retention owners, archive state, and the
//! pre-action delete/export truth in one place, see what a draft policy would
//! change before it publishes, and prove every records-side exception reference
//! resolves to a live, bounded policy exception.

use serde::{Deserialize, Serialize};

use aureline_policy::m5_exception_expiry::{
    seeded_m5_exception_expiry_packet, ApprovalHistoryRow, ExceptionRequestSheet, ExpiryBanner,
    M5ExceptionExpiryPacket, M5ExceptionExpiryViolation, RememberedDecisionRevalidation,
};
use aureline_records::m5_policy_simulation::{
    seeded_m5_policy_simulation_packet, M5PolicyImpactSimulationPacket,
    M5PolicySimulationViolation, SupportExportSimulationRow,
};
use aureline_records::m5_records_policy::{
    seeded_m5_records_policy_packet, M5RecordsPolicyPacket, M5RecordsPolicyViolation,
    SupportExportInspectorRow, M5_RECORDS_POLICY_ARTIFACT_REF, M5_RECORDS_POLICY_DOC_REF,
};

#[cfg(test)]
mod tests;

/// Schema version for the support-side M5 records-policy export.
pub const M5_RECORDS_POLICY_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for the support-side export.
pub const M5_RECORDS_POLICY_GOVERNANCE_RECORD_KIND: &str =
    "m5_records_policy_governance_support_export";

/// Cross-packet validation issue surfaced to support consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum M5RecordsPolicyGovernanceViolation {
    /// A records-side exception reference does not resolve to a policy exception.
    ExceptionRefUnresolved { exception_id: String },
    /// The records and policy packets disagree on the shared contract pairing.
    ContractRefMismatch {
        records_expects: String,
        policy_provides: String,
    },
    /// The simulation packet references the wrong runtime hold/retention contract.
    SimulationRuntimeContractMismatch {
        simulation_expects: String,
        runtime_provides: String,
    },
    /// A governed family present in one packet is absent from the simulation pairing.
    SimulationFamilyCoverageMismatch { entry_id: String },
}

/// Support-side packet joining M5 hold/retention and exception/expiry truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RecordsPolicyGovernanceSupportExport {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Hold/retention contract doc ref.
    pub hold_retention_doc_ref: String,
    /// Hold/retention artifact summary ref.
    pub hold_retention_artifact_ref: String,
    /// Embedded authoritative hold/retention packet.
    pub hold_retention_packet: M5RecordsPolicyPacket,
    /// Embedded authoritative exception/expiry packet.
    pub exception_expiry_packet: M5ExceptionExpiryPacket,
    /// Embedded authoritative pre-apply policy-simulation packet.
    pub policy_simulation_packet: M5PolicyImpactSimulationPacket,
    /// Support/export projection rows.
    pub projection_rows: Vec<SupportExportInspectorRow>,
    /// Pre-apply policy-simulation support/export projection rows.
    pub simulation_projection_rows: Vec<SupportExportSimulationRow>,
    /// Exception/waiver request sheets showing exact variance, scope, and expiry.
    pub exception_request_sheets: Vec<ExceptionRequestSheet>,
    /// Approval-history rows kept visible in the support export.
    pub exception_approval_history: Vec<ApprovalHistoryRow>,
    /// Expiry banners computed against the exception packet's `as_of`.
    pub exception_expiry_banners: Vec<ExpiryBanner>,
    /// Remembered-decision revalidation outcomes at the packet's `as_of`.
    pub remembered_decision_revalidations: Vec<RememberedDecisionRevalidation>,
    /// Validation issues inherited from the hold/retention packet.
    pub hold_retention_violations: Vec<M5RecordsPolicyViolation>,
    /// Validation issues inherited from the exception/expiry packet.
    pub exception_expiry_violations: Vec<M5ExceptionExpiryViolation>,
    /// Validation issues inherited from the policy-simulation packet.
    pub policy_simulation_violations: Vec<M5PolicySimulationViolation>,
    /// Cross-packet validation issues.
    pub cross_packet_violations: Vec<M5RecordsPolicyGovernanceViolation>,
    /// Metadata-only redaction invariant.
    pub raw_private_material_excluded: bool,
    /// Review-safe summary.
    pub summary: String,
}

impl M5RecordsPolicyGovernanceSupportExport {
    /// Cross-validates that every referenced exception resolves, contracts
    /// agree, and the simulation previews the same families as the runtime
    /// surface it affects.
    pub fn cross_validate(
        hold_retention_packet: &M5RecordsPolicyPacket,
        exception_expiry_packet: &M5ExceptionExpiryPacket,
        policy_simulation_packet: &M5PolicyImpactSimulationPacket,
    ) -> Vec<M5RecordsPolicyGovernanceViolation> {
        let mut violations = Vec::new();

        let known_exceptions = exception_expiry_packet.exception_ids();
        let mut referenced = hold_retention_packet.referenced_exception_ids();
        referenced.extend(policy_simulation_packet.referenced_exception_ids());
        referenced.sort();
        referenced.dedup();
        for exception_id in referenced {
            if !known_exceptions.contains(&exception_id) {
                violations.push(M5RecordsPolicyGovernanceViolation::ExceptionRefUnresolved {
                    exception_id,
                });
            }
        }

        if hold_retention_packet.exception_expiry_contract_ref
            != exception_expiry_packet.shared_contract_ref
        {
            violations.push(M5RecordsPolicyGovernanceViolation::ContractRefMismatch {
                records_expects: hold_retention_packet.exception_expiry_contract_ref.clone(),
                policy_provides: exception_expiry_packet.shared_contract_ref.clone(),
            });
        }

        // The simulation must point at the runtime hold/retention contract so
        // both surfaces share one set of object identities.
        if policy_simulation_packet.runtime_contract_ref
            != hold_retention_packet.shared_contract_ref
        {
            violations.push(
                M5RecordsPolicyGovernanceViolation::SimulationRuntimeContractMismatch {
                    simulation_expects: policy_simulation_packet.runtime_contract_ref.clone(),
                    runtime_provides: hold_retention_packet.shared_contract_ref.clone(),
                },
            );
        }

        // Every runtime family must be previewed by a simulation row with the
        // same entry id, and vice versa.
        for runtime_row in &hold_retention_packet.rows {
            if !policy_simulation_packet
                .rows
                .iter()
                .any(|sim| sim.entry_id == runtime_row.entry_id)
            {
                violations.push(
                    M5RecordsPolicyGovernanceViolation::SimulationFamilyCoverageMismatch {
                        entry_id: runtime_row.entry_id.clone(),
                    },
                );
            }
        }
        for sim_row in &policy_simulation_packet.rows {
            if !hold_retention_packet
                .rows
                .iter()
                .any(|runtime| runtime.entry_id == sim_row.entry_id)
            {
                violations.push(
                    M5RecordsPolicyGovernanceViolation::SimulationFamilyCoverageMismatch {
                        entry_id: sim_row.entry_id.clone(),
                    },
                );
            }
        }

        violations
    }

    /// Returns the current support-side export from the seeded authoritative packets.
    pub fn current() -> Self {
        let hold_retention_packet = seeded_m5_records_policy_packet();
        let exception_expiry_packet = seeded_m5_exception_expiry_packet();
        let policy_simulation_packet = seeded_m5_policy_simulation_packet();
        let projection_rows = hold_retention_packet.support_export_projection();
        let simulation_projection_rows = policy_simulation_packet.support_export_projection();
        let exception_request_sheets = exception_expiry_packet.request_sheets();
        let exception_approval_history = exception_expiry_packet.approval_history();
        let exception_expiry_banners = exception_expiry_packet.expiry_banners();
        let remembered_decision_revalidations = exception_expiry_packet.self_revalidation();
        let hold_retention_violations = hold_retention_packet.validate();
        let exception_expiry_violations = exception_expiry_packet.validate();
        let policy_simulation_violations = policy_simulation_packet.validate();
        let cross_packet_violations = Self::cross_validate(
            &hold_retention_packet,
            &exception_expiry_packet,
            &policy_simulation_packet,
        );
        let summary = format!(
            "Support export covering {} hold/retention families, {} bounded policy exceptions, \
             and {} pre-apply policy-simulation rows.",
            projection_rows.len(),
            exception_expiry_packet.rows.len(),
            simulation_projection_rows.len()
        );

        Self {
            schema_version: M5_RECORDS_POLICY_GOVERNANCE_SCHEMA_VERSION,
            record_kind: M5_RECORDS_POLICY_GOVERNANCE_RECORD_KIND.to_owned(),
            export_id: "support-export:m5-records-policy-governance:v1".to_owned(),
            generated_at: hold_retention_packet.as_of.clone(),
            hold_retention_doc_ref: M5_RECORDS_POLICY_DOC_REF.to_owned(),
            hold_retention_artifact_ref: M5_RECORDS_POLICY_ARTIFACT_REF.to_owned(),
            hold_retention_packet,
            exception_expiry_packet,
            policy_simulation_packet,
            projection_rows,
            simulation_projection_rows,
            exception_request_sheets,
            exception_approval_history,
            exception_expiry_banners,
            remembered_decision_revalidations,
            hold_retention_violations,
            exception_expiry_violations,
            policy_simulation_violations,
            cross_packet_violations,
            raw_private_material_excluded: true,
            summary,
        }
    }

    /// Returns true when no packet or cross-packet violation is present.
    pub fn is_clean(&self) -> bool {
        self.hold_retention_violations.is_empty()
            && self.exception_expiry_violations.is_empty()
            && self.policy_simulation_violations.is_empty()
            && self.cross_packet_violations.is_empty()
    }
}
