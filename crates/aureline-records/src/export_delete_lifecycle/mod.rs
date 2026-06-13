//! Canonical export-job, request-case, and delete-case records for durable M5 artifacts.
//!
//! This module turns delete/export honesty from prose into typed product objects
//! that can be shared across AI evidence, provider-linked work items, sync,
//! incident, and offboarding lanes. Export jobs always carry an
//! [`ExportBundleManifest`]. Delete cases either carry a durable
//! [`DestructionReceipt`] or a typed blocker state. Local-only boundaries remain
//! explicit so no consumer can imply remote export, remote delete, or managed
//! hold coverage where the platform never possessed the artifact.

use serde::{Deserialize, Serialize};

use crate::records_policy_simulation_matrix::GovernedArtifactFamily;
use crate::stabilize_record_class_registry_legal_hold_delete_honesty::{
    ChronologyExport, DestructionReceipt, ExportBundleManifest, HoldEvaluation, OmissionReason,
    RecordOperationOutcome, RefChecksum, RefRecord,
};
use crate::RecordClassId;

#[cfg(test)]
mod tests;

/// Schema version for lifecycle packet records.
pub const RECORDS_EXPORT_DELETE_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the top-level lifecycle packet.
pub const RECORDS_EXPORT_DELETE_LIFECYCLE_RECORD_KIND: &str =
    "records_export_delete_lifecycle_packet";

/// Repo-relative doc reference for the lifecycle contract.
pub const RECORDS_EXPORT_DELETE_LIFECYCLE_DOC_REF: &str =
    "docs/governance/records_export_delete_lifecycle.md";

/// Repo-relative artifact summary for the lifecycle contract.
pub const RECORDS_EXPORT_DELETE_LIFECYCLE_ARTIFACT_REF: &str =
    "artifacts/governance/records_export_delete_lifecycle.md";

/// Repo-relative schema reference for the lifecycle contract.
pub const RECORDS_EXPORT_DELETE_LIFECYCLE_SCHEMA_REF: &str =
    "schemas/governance/records_export_delete_lifecycle.schema.json";

/// Request case kind surfaced to users/admins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestCaseKind {
    /// User or admin asked for an export/access package.
    AccessExport,
    /// User or admin asked for a privacy delete or redaction outcome.
    PrivacyDelete,
    /// User or admin asked for an offboarding/export-delete package.
    OffboardingDelete,
}

impl RequestCaseKind {
    /// Returns the stable token for this request kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessExport => "access_export",
            Self::PrivacyDelete => "privacy_delete",
            Self::OffboardingDelete => "offboarding_delete",
        }
    }
}

/// Typed blocker state shown when a delete or request case cannot finish cleanly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseBlockerState {
    /// Stable blocker id.
    pub blocker_id: String,
    /// Closed outcome token that explains the blocker.
    pub blocker_outcome: RecordOperationOutcome,
    /// Opaque proof or evidence ref backing the blocker.
    pub blocker_ref: String,
    /// Redaction-safe explanation suitable for UI, CLI, and support export.
    pub detail: String,
    /// Optional next expected state-change hint.
    pub next_state_change_hint: Option<String>,
}

/// Resumable export job with a mandatory manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportJobRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export job id.
    pub job_id: String,
    /// Requester ref.
    pub requester_ref: String,
    /// Linked request-case id.
    pub request_case_id: String,
    /// Scope selector or scope ref for this export.
    pub scope_selector: String,
    /// Record classes the job covered.
    pub record_class_ids: Vec<RecordClassId>,
    /// Applied redaction profile.
    pub redaction_profile: String,
    /// Manifest emitted by the export job.
    pub manifest: ExportBundleManifest,
    /// Primary outcome for the job.
    pub outcome: RecordOperationOutcome,
    /// Partial or omission reasons visible on the job.
    pub partial_or_omission_reasons: Vec<OmissionReason>,
    /// Export bundle refs produced by the job.
    pub bundle_refs: Vec<String>,
    /// Chronology exported for the job.
    pub chronology: ChronologyExport,
    /// Review-safe summary.
    pub summary: String,
}

/// Durable request case tracking export/delete posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyRequestCaseRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Request case kind.
    pub request_kind: RequestCaseKind,
    /// Requester or request-subject ref.
    pub requester_ref: String,
    /// Subject, workspace, or org scope.
    pub subject_scope_ref: String,
    /// Jurisdiction or effective policy class.
    pub jurisdiction_or_policy_class: String,
    /// Approval or audit refs.
    pub approval_log_refs: Vec<String>,
    /// Blocking state when the case cannot finish cleanly.
    pub blocker_state: Option<CaseBlockerState>,
    /// SLA target in RFC 3339 UTC.
    pub sla_target_at: String,
    /// Current durable status for the case.
    pub status: RecordOperationOutcome,
    /// Linked export jobs.
    pub export_job_ids: Vec<String>,
    /// Linked delete cases.
    pub delete_case_ids: Vec<String>,
    /// Review-safe chronology reference.
    pub chronology_ref: String,
    /// Review-safe summary.
    pub summary: String,
}

/// Durable delete case with either a receipt or a typed blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteCaseRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable delete case id.
    pub case_id: String,
    /// Request case kind.
    pub request_kind: RequestCaseKind,
    /// Requester ref.
    pub requester_ref: String,
    /// Subject, workspace, or org scope.
    pub subject_scope_ref: String,
    /// Policy version used by the delete path.
    pub policy_version: String,
    /// Planning-time hold evaluation.
    pub planning_hold: HoldEvaluation,
    /// Execution-time hold evaluation.
    pub execution_hold: HoldEvaluation,
    /// Export jobs that had to complete before delete could proceed.
    pub prerequisite_export_job_ids: Vec<String>,
    /// Durable delete outcome.
    pub outcome: RecordOperationOutcome,
    /// Remaining unresolved outcomes when the result is partial.
    pub unresolved_outcomes: Vec<RecordOperationOutcome>,
    /// Durable destruction receipt when delete reached a terminal destructive state.
    pub destruction_receipt: Option<DestructionReceipt>,
    /// Typed blocker state when delete could not finish.
    pub typed_blocker_state: Option<CaseBlockerState>,
    /// Review-safe summary.
    pub summary: String,
}

/// One authoritative lifecycle link for a governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleFamilyLink {
    /// Stable entry id.
    pub entry_id: String,
    /// Governed artifact family.
    pub artifact_family: GovernedArtifactFamily,
    /// Governing record class.
    pub record_class_id: RecordClassId,
    /// First consumer packet or lane ref.
    pub consumer_packet_ref: String,
    /// Primary request case id for the family.
    pub request_case_id: String,
    /// Primary export job id for the family.
    pub export_job_id: String,
    /// Primary delete case id for the family.
    pub delete_case_id: String,
    /// Primary export outcome for the family.
    pub export_outcome: RecordOperationOutcome,
    /// Primary delete outcome for the family.
    pub delete_outcome: RecordOperationOutcome,
    /// Optional local-only or outside-scope note.
    pub local_only_boundary_note: Option<String>,
    /// Review-safe policy owner ref.
    pub policy_owner_ref: String,
    /// Review-safe chronology ref.
    pub chronology_ref: String,
}

/// CLI/headless projection row for one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliHeadlessLifecycleRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Export job id.
    pub export_job_id: String,
    /// Delete case id.
    pub delete_case_id: String,
    /// Export outcome.
    pub export_outcome: RecordOperationOutcome,
    /// Delete outcome.
    pub delete_outcome: RecordOperationOutcome,
}

/// Help/docs projection row for one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpDocsLifecycleRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Consumer packet or lane reference.
    pub consumer_packet_ref: String,
    /// Review-safe summary.
    pub summary: String,
}

/// Support/export projection row for one governed family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportLifecycleRow {
    /// Governed family.
    pub artifact_family: GovernedArtifactFamily,
    /// Request case id.
    pub request_case_id: String,
    /// Export job id.
    pub export_job_id: String,
    /// Export manifest bundle id.
    pub manifest_bundle_id: String,
    /// Delete case id.
    pub delete_case_id: String,
    /// Receipt id when a receipt exists.
    pub destruction_receipt_id: Option<String>,
    /// Typed blocker outcome when a receipt does not exist.
    pub delete_blocker_outcome: Option<RecordOperationOutcome>,
    /// Optional local-only or outside-scope note.
    pub local_only_boundary_note: Option<String>,
}

/// Top-level canonical lifecycle packet for the M5 export/delete truth source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordsExportDeleteLifecyclePacket {
    /// Schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// UTC packet timestamp.
    pub as_of: String,
    /// Overview doc ref.
    pub overview_doc_ref: String,
    /// Artifact summary ref.
    pub artifact_summary_ref: String,
    /// Export jobs emitted by the packet.
    pub export_jobs: Vec<ExportJobRecord>,
    /// Request cases emitted by the packet.
    pub request_cases: Vec<PrivacyRequestCaseRecord>,
    /// Delete cases emitted by the packet.
    pub delete_cases: Vec<DeleteCaseRecord>,
    /// Governed family links.
    pub family_links: Vec<LifecycleFamilyLink>,
    /// Review-safe summary.
    pub summary: String,
}

/// Validation issue emitted by the lifecycle packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "detail")]
pub enum RecordsExportDeleteLifecycleViolation {
    /// Schema version mismatch.
    SchemaVersionMismatch { found: u32 },
    /// Record kind mismatch.
    RecordKindMismatch { found: String },
    /// Export job omitted a manifest.
    ExportJobMissingManifest { job_id: String },
    /// Export job finished but omitted bundle refs.
    ExportJobMissingBundleRef { job_id: String },
    /// Partial export job omitted partial reasons.
    ExportJobPartialReasonsMissing { job_id: String },
    /// Request case omitted any linked export or delete case.
    RequestCaseUnlinked { case_id: String },
    /// Delete case finished without a receipt.
    DeleteCaseMissingReceipt { case_id: String },
    /// Delete case blocked or scoped-out without a typed blocker.
    DeleteCaseMissingBlocker { case_id: String },
    /// Family link points at an unknown request case.
    FamilyLinkUnknownRequestCase { entry_id: String, case_id: String },
    /// Family link points at an unknown export job.
    FamilyLinkUnknownExportJob { entry_id: String, job_id: String },
    /// Family link points at an unknown delete case.
    FamilyLinkUnknownDeleteCase { entry_id: String, case_id: String },
    /// Family link outcome drifts from the linked export job.
    FamilyLinkExportOutcomeMismatch { entry_id: String },
    /// Family link outcome drifts from the linked delete case.
    FamilyLinkDeleteOutcomeMismatch { entry_id: String },
    /// Governed family coverage is missing.
    FamilyCoverageMissing {
        artifact_family: GovernedArtifactFamily,
    },
}

impl RecordsExportDeleteLifecyclePacket {
    /// Validates structural and cross-link invariants.
    pub fn validate(&self) -> Vec<RecordsExportDeleteLifecycleViolation> {
        let mut violations = Vec::new();

        if self.schema_version != RECORDS_EXPORT_DELETE_LIFECYCLE_SCHEMA_VERSION {
            violations.push(
                RecordsExportDeleteLifecycleViolation::SchemaVersionMismatch {
                    found: self.schema_version,
                },
            );
        }
        if self.record_kind != RECORDS_EXPORT_DELETE_LIFECYCLE_RECORD_KIND {
            violations.push(RecordsExportDeleteLifecycleViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }

        let export_job_ids = self
            .export_jobs
            .iter()
            .map(|job| (job.job_id.clone(), job))
            .collect::<std::collections::BTreeMap<_, _>>();
        let request_case_ids = self
            .request_cases
            .iter()
            .map(|case| (case.case_id.clone(), case))
            .collect::<std::collections::BTreeMap<_, _>>();
        let delete_case_ids = self
            .delete_cases
            .iter()
            .map(|case| (case.case_id.clone(), case))
            .collect::<std::collections::BTreeMap<_, _>>();

        for job in &self.export_jobs {
            if job.manifest.bundle_id.trim().is_empty() {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::ExportJobMissingManifest {
                        job_id: job.job_id.clone(),
                    },
                );
            }
            if matches!(
                job.outcome,
                RecordOperationOutcome::Completed | RecordOperationOutcome::Partial
            ) && job.bundle_refs.is_empty()
            {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::ExportJobMissingBundleRef {
                        job_id: job.job_id.clone(),
                    },
                );
            }
            if job.outcome == RecordOperationOutcome::Partial
                && job.partial_or_omission_reasons.is_empty()
            {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::ExportJobPartialReasonsMissing {
                        job_id: job.job_id.clone(),
                    },
                );
            }
        }

        for case in &self.request_cases {
            if case.export_job_ids.is_empty() && case.delete_case_ids.is_empty() {
                violations.push(RecordsExportDeleteLifecycleViolation::RequestCaseUnlinked {
                    case_id: case.case_id.clone(),
                });
            }
        }

        for delete_case in &self.delete_cases {
            match delete_case.outcome {
                RecordOperationOutcome::Completed | RecordOperationOutcome::Partial => {
                    if delete_case.destruction_receipt.is_none() {
                        violations.push(
                            RecordsExportDeleteLifecycleViolation::DeleteCaseMissingReceipt {
                                case_id: delete_case.case_id.clone(),
                            },
                        );
                    }
                }
                RecordOperationOutcome::BlockedByHold
                | RecordOperationOutcome::PolicyRetained
                | RecordOperationOutcome::OutsidePlatformScope
                | RecordOperationOutcome::ManualLocalCaptureRequired
                | RecordOperationOutcome::NotFound
                | RecordOperationOutcome::OmittedByRedaction => {
                    if delete_case.typed_blocker_state.is_none() {
                        violations.push(
                            RecordsExportDeleteLifecycleViolation::DeleteCaseMissingBlocker {
                                case_id: delete_case.case_id.clone(),
                            },
                        );
                    }
                }
                RecordOperationOutcome::Requested | RecordOperationOutcome::Queued => {}
            }
        }

        for family in required_families() {
            if !self
                .family_links
                .iter()
                .any(|row| row.artifact_family == family)
            {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::FamilyCoverageMissing {
                        artifact_family: family,
                    },
                );
            }
        }

        for link in &self.family_links {
            let Some(request_case) = request_case_ids.get(&link.request_case_id) else {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::FamilyLinkUnknownRequestCase {
                        entry_id: link.entry_id.clone(),
                        case_id: link.request_case_id.clone(),
                    },
                );
                continue;
            };
            let Some(export_job) = export_job_ids.get(&link.export_job_id) else {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::FamilyLinkUnknownExportJob {
                        entry_id: link.entry_id.clone(),
                        job_id: link.export_job_id.clone(),
                    },
                );
                continue;
            };
            let Some(delete_case) = delete_case_ids.get(&link.delete_case_id) else {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::FamilyLinkUnknownDeleteCase {
                        entry_id: link.entry_id.clone(),
                        case_id: link.delete_case_id.clone(),
                    },
                );
                continue;
            };

            if export_job.outcome != link.export_outcome {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::FamilyLinkExportOutcomeMismatch {
                        entry_id: link.entry_id.clone(),
                    },
                );
            }
            if delete_case.outcome != link.delete_outcome {
                violations.push(
                    RecordsExportDeleteLifecycleViolation::FamilyLinkDeleteOutcomeMismatch {
                        entry_id: link.entry_id.clone(),
                    },
                );
            }
            let request_links_export = request_case
                .export_job_ids
                .iter()
                .any(|candidate| candidate == &link.export_job_id);
            let request_links_delete = request_case
                .delete_case_ids
                .iter()
                .any(|candidate| candidate == &link.delete_case_id);
            if !request_links_export || !request_links_delete {
                violations.push(RecordsExportDeleteLifecycleViolation::RequestCaseUnlinked {
                    case_id: request_case.case_id.clone(),
                });
            }
        }

        violations
    }

    /// Returns a narrow CLI/headless projection.
    pub fn cli_headless_projection(&self) -> Vec<CliHeadlessLifecycleRow> {
        self.family_links
            .iter()
            .map(|link| CliHeadlessLifecycleRow {
                artifact_family: link.artifact_family,
                export_job_id: link.export_job_id.clone(),
                delete_case_id: link.delete_case_id.clone(),
                export_outcome: link.export_outcome,
                delete_outcome: link.delete_outcome,
            })
            .collect()
    }

    /// Returns a help/docs projection.
    pub fn help_docs_projection(&self) -> Vec<HelpDocsLifecycleRow> {
        self.family_links
            .iter()
            .map(|link| HelpDocsLifecycleRow {
                artifact_family: link.artifact_family,
                consumer_packet_ref: link.consumer_packet_ref.clone(),
                summary: format!(
                    "{} export={} delete={}",
                    link.consumer_packet_ref,
                    link.export_outcome.as_str(),
                    link.delete_outcome.as_str()
                ),
            })
            .collect()
    }

    /// Returns a support/export projection.
    pub fn support_export_projection(&self) -> Vec<SupportExportLifecycleRow> {
        self.family_links
            .iter()
            .filter_map(|link| {
                let export_job = self
                    .export_jobs
                    .iter()
                    .find(|job| job.job_id == link.export_job_id)?;
                let delete_case = self
                    .delete_cases
                    .iter()
                    .find(|case| case.case_id == link.delete_case_id)?;

                Some(SupportExportLifecycleRow {
                    artifact_family: link.artifact_family,
                    request_case_id: link.request_case_id.clone(),
                    export_job_id: link.export_job_id.clone(),
                    manifest_bundle_id: export_job.manifest.bundle_id.clone(),
                    delete_case_id: link.delete_case_id.clone(),
                    destruction_receipt_id: delete_case
                        .destruction_receipt
                        .as_ref()
                        .map(|receipt| receipt.receipt_id.clone()),
                    delete_blocker_outcome: delete_case
                        .typed_blocker_state
                        .as_ref()
                        .map(|state| state.blocker_outcome),
                    local_only_boundary_note: link.local_only_boundary_note.clone(),
                })
            })
            .collect()
    }
}

/// Returns the canonical seeded lifecycle packet for the first consumers.
pub fn seeded_records_export_delete_lifecycle_packet() -> RecordsExportDeleteLifecyclePacket {
    let export_jobs = vec![
        ai_export_job(),
        provider_export_job(),
        sync_export_job(),
        incident_export_job(),
        offboarding_export_job(),
    ];
    let request_cases = vec![
        ai_request_case(),
        provider_request_case(),
        sync_request_case(),
        incident_request_case(),
        offboarding_request_case(),
    ];
    let delete_cases = vec![
        ai_delete_case(),
        provider_delete_case(),
        sync_delete_case(),
        incident_delete_case(),
        offboarding_delete_case(),
    ];
    let family_links = vec![
        LifecycleFamilyLink {
            entry_id: "family-link:ai-evidence".to_owned(),
            artifact_family: GovernedArtifactFamily::AiEvidencePacket,
            record_class_id: RecordClassId::AiRetainedEvidencePacket,
            consumer_packet_ref: "ai-memory-state:stable:0001".to_owned(),
            request_case_id: "request-case:ai-evidence:0001".to_owned(),
            export_job_id: "export-job:ai-evidence:0001".to_owned(),
            delete_case_id: "delete-case:ai-evidence:0001".to_owned(),
            export_outcome: RecordOperationOutcome::Completed,
            delete_outcome: RecordOperationOutcome::Partial,
            local_only_boundary_note: Some(
                "Reviewed local prompt/result caches remain on-device until the user clears them."
                    .to_owned(),
            ),
            policy_owner_ref: "policy-owner:ai-evidence".to_owned(),
            chronology_ref: "chronology:ai-evidence:0001".to_owned(),
        },
        LifecycleFamilyLink {
            entry_id: "family-link:provider-work-item".to_owned(),
            artifact_family: GovernedArtifactFamily::ProviderLinkedWorkItem,
            record_class_id: RecordClassId::ProviderLinkedWorkItemRecord,
            consumer_packet_ref: "providers.work_item_object_rows.packet".to_owned(),
            request_case_id: "request-case:provider-work-item:0001".to_owned(),
            export_job_id: "export-job:provider-work-item:0001".to_owned(),
            delete_case_id: "delete-case:provider-work-item:0001".to_owned(),
            export_outcome: RecordOperationOutcome::OutsidePlatformScope,
            delete_outcome: RecordOperationOutcome::NotFound,
            local_only_boundary_note: Some(
                "Provider-origin records stayed on the provider and Aureline exported only local linkage metadata."
                    .to_owned(),
            ),
            policy_owner_ref: "policy-owner:provider-work-item".to_owned(),
            chronology_ref: "chronology:provider-work-item:0001".to_owned(),
        },
        LifecycleFamilyLink {
            entry_id: "family-link:sync-ledger".to_owned(),
            artifact_family: GovernedArtifactFamily::SyncMirrorLedger,
            record_class_id: RecordClassId::SyncMirrorLedger,
            consumer_packet_ref: "m5_sync_and_device_review:fully_synced_baseline".to_owned(),
            request_case_id: "request-case:sync-ledger:0001".to_owned(),
            export_job_id: "export-job:sync-ledger:0001".to_owned(),
            delete_case_id: "delete-case:sync-ledger:0001".to_owned(),
            export_outcome: RecordOperationOutcome::ManualLocalCaptureRequired,
            delete_outcome: RecordOperationOutcome::BlockedByHold,
            local_only_boundary_note: Some(
                "Per-device local revisions and recovery snapshots require manual local capture before export or delete covers them."
                    .to_owned(),
            ),
            policy_owner_ref: "policy-owner:sync-ledger".to_owned(),
            chronology_ref: "chronology:sync-ledger:0001".to_owned(),
        },
        LifecycleFamilyLink {
            entry_id: "family-link:incident-packet".to_owned(),
            artifact_family: GovernedArtifactFamily::IncidentSupportPacket,
            record_class_id: RecordClassId::IncidentSupportPacket,
            consumer_packet_ref: "incident-workspace:provider-unavailable".to_owned(),
            request_case_id: "request-case:incident-packet:0001".to_owned(),
            export_job_id: "export-job:incident-packet:0001".to_owned(),
            delete_case_id: "delete-case:incident-packet:0001".to_owned(),
            export_outcome: RecordOperationOutcome::OmittedByRedaction,
            delete_outcome: RecordOperationOutcome::Completed,
            local_only_boundary_note: None,
            policy_owner_ref: "policy-owner:incident-packet".to_owned(),
            chronology_ref: "chronology:incident-packet:0001".to_owned(),
        },
        LifecycleFamilyLink {
            entry_id: "family-link:offboarding-record".to_owned(),
            artifact_family: GovernedArtifactFamily::OffboardingRecord,
            record_class_id: RecordClassId::OffboardingExitPacket,
            consumer_packet_ref: "usage-export-offboarding-surface:stable:0001".to_owned(),
            request_case_id: "request-case:offboarding:0001".to_owned(),
            export_job_id: "export-job:offboarding:0001".to_owned(),
            delete_case_id: "delete-case:offboarding:0001".to_owned(),
            export_outcome: RecordOperationOutcome::Partial,
            delete_outcome: RecordOperationOutcome::PolicyRetained,
            local_only_boundary_note: Some(
                "Downloaded local workspace exports remain user-controlled after managed offboarding completes."
                    .to_owned(),
            ),
            policy_owner_ref: "policy-owner:offboarding".to_owned(),
            chronology_ref: "chronology:offboarding:0001".to_owned(),
        },
    ];

    RecordsExportDeleteLifecyclePacket {
        schema_version: RECORDS_EXPORT_DELETE_LIFECYCLE_SCHEMA_VERSION,
        record_kind: RECORDS_EXPORT_DELETE_LIFECYCLE_RECORD_KIND.to_owned(),
        packet_id: "records-export-delete-lifecycle:stable:0001".to_owned(),
        shared_contract_ref: "records:export_delete_lifecycle:v1".to_owned(),
        as_of: "2026-06-13T16:00:00Z".to_owned(),
        overview_doc_ref: RECORDS_EXPORT_DELETE_LIFECYCLE_DOC_REF.to_owned(),
        artifact_summary_ref: RECORDS_EXPORT_DELETE_LIFECYCLE_ARTIFACT_REF.to_owned(),
        export_jobs,
        request_cases,
        delete_cases,
        family_links,
        summary: "Canonical export jobs, request cases, and delete cases for AI evidence, provider-linked work items, sync, incident, and offboarding rows.".to_owned(),
    }
}

fn required_families() -> [GovernedArtifactFamily; 5] {
    [
        GovernedArtifactFamily::AiEvidencePacket,
        GovernedArtifactFamily::ProviderLinkedWorkItem,
        GovernedArtifactFamily::SyncMirrorLedger,
        GovernedArtifactFamily::IncidentSupportPacket,
        GovernedArtifactFamily::OffboardingRecord,
    ]
}

fn chronology(
    export_id: &str,
    scope_selector: &str,
    record_class_id: RecordClassId,
) -> ChronologyExport {
    ChronologyExport {
        export_id: export_id.to_owned(),
        scope_selector: scope_selector.to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        exported_at: "2026-06-13T16:00:00Z".to_owned(),
        actor_lineage_preserved: true,
        timezone_aware: true,
        entries: vec![
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::ChronologyEntry {
                event_id: format!("{export_id}:requested"),
                actor_ref: "actor:local-user".to_owned(),
                timestamp_utc: "2026-06-13T15:00:00Z".to_owned(),
                source_timezone_label: Some("America/Los_Angeles".to_owned()),
                event_class: "request_opened".to_owned(),
                record_class_id: Some(record_class_id),
                is_local_only: false,
                is_attributed: true,
            },
        ],
    }
}

fn hold_cleared() -> HoldEvaluation {
    HoldEvaluation {
        phase:
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::HoldPhase::Planning,
        status:
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::HoldStatus::Cleared,
        scope:
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::HoldScope::ManagedOnly,
        active_hold_refs: Vec::new(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        local_only_artifact_note: None,
    }
}

fn hold_active(note: &str) -> HoldEvaluation {
    HoldEvaluation {
        phase:
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::HoldPhase::Execution,
        status:
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::HoldStatus::Active,
        scope:
            crate::stabilize_record_class_registry_legal_hold_delete_honesty::HoldScope::ManagedOnly,
        active_hold_refs: vec!["hold:legal-77".to_owned()],
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        local_only_artifact_note: Some(note.to_owned()),
    }
}

fn manifest(
    bundle_id: &str,
    record_class_id: RecordClassId,
    outcome: RecordOperationOutcome,
    omission_reasons: Vec<OmissionReason>,
) -> ExportBundleManifest {
    ExportBundleManifest {
        bundle_id: bundle_id.to_owned(),
        created_at: "2026-06-13T16:00:00Z".to_owned(),
        scope_selectors: vec![format!("scope:{bundle_id}")],
        time_range_start: Some("2026-06-01T00:00:00Z".to_owned()),
        time_range_end: Some("2026-06-13T16:00:00Z".to_owned()),
        included_classes: vec![record_class_id],
        excluded_classes: omission_reasons
            .iter()
            .map(|reason| reason.record_class_id)
            .collect(),
        omission_reasons,
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        hash_checksum_manifest: vec![RefChecksum {
            ref_id: format!("ref:{bundle_id}:0001"),
            algorithm: "sha256".to_owned(),
            hash: "7f1f9d6c".to_owned(),
        }],
        redaction_profile: "metadata_safe_default".to_owned(),
        signer_ref: "signer:records-export-delete:v1".to_owned(),
        refs: vec![RefRecord {
            ref_id: format!("ref:{bundle_id}:0001"),
            outcome,
            reason: None,
            record_class_id: Some(record_class_id),
        }],
        collab_metadata: None,
    }
}

fn receipt(
    receipt_id: &str,
    action: &str,
    record_class_id: RecordClassId,
    retained_outcome: Option<RecordOperationOutcome>,
) -> DestructionReceipt {
    let retained_refs = retained_outcome
        .map(|outcome| {
            vec![RefRecord {
                ref_id: format!("ref:{receipt_id}:retained"),
                outcome,
                reason: Some("Evidence packet remains under policy retention.".to_owned()),
                record_class_id: Some(record_class_id),
            }]
        })
        .unwrap_or_default();
    DestructionReceipt {
        receipt_id: receipt_id.to_owned(),
        executed_action: action.to_owned(),
        executed_at: "2026-06-13T16:00:00Z".to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        scope_selectors: vec![format!("scope:{receipt_id}")],
        included_classes: vec![record_class_id],
        excluded_classes: Vec::new(),
        deleted_refs: vec![RefRecord {
            ref_id: format!("ref:{receipt_id}:deleted"),
            outcome: RecordOperationOutcome::Completed,
            reason: None,
            record_class_id: Some(record_class_id),
        }],
        skipped_refs: Vec::new(),
        retained_refs: retained_refs.clone(),
        held_refs: Vec::new(),
        outside_scope_refs: Vec::new(),
        total_destroyed_count: 1,
        total_retained_count: retained_refs.len() as u64,
        total_outside_scope_count: 0,
        hash_checksum_manifest: vec![RefChecksum {
            ref_id: format!("ref:{receipt_id}:deleted"),
            algorithm: "sha256".to_owned(),
            hash: "aa11bb22".to_owned(),
        }],
        redaction_profile: "metadata_safe_default".to_owned(),
        verifier_ref: "verifier:records-export-delete:v1".to_owned(),
        local_only_not_held_note: None,
        collab_metadata: None,
    }
}

fn ai_export_job() -> ExportJobRecord {
    ExportJobRecord {
        record_kind: "records_export_job_record".to_owned(),
        schema_version: 1,
        job_id: "export-job:ai-evidence:0001".to_owned(),
        requester_ref: "actor:local-user".to_owned(),
        request_case_id: "request-case:ai-evidence:0001".to_owned(),
        scope_selector: "scope:ai-evidence".to_owned(),
        record_class_ids: vec![RecordClassId::AiRetainedEvidencePacket],
        redaction_profile: "metadata_safe_default".to_owned(),
        manifest: manifest(
            "manifest:ai-evidence:0001",
            RecordClassId::AiRetainedEvidencePacket,
            RecordOperationOutcome::Completed,
            Vec::new(),
        ),
        outcome: RecordOperationOutcome::Completed,
        partial_or_omission_reasons: Vec::new(),
        bundle_refs: vec!["bundle:ai-evidence:0001".to_owned()],
        chronology: chronology(
            "chronology:ai-evidence:0001",
            "scope:ai-evidence",
            RecordClassId::AiRetainedEvidencePacket,
        ),
        summary: "AI evidence export completed with a manifest and a bounded redacted bundle."
            .to_owned(),
    }
}

fn provider_export_job() -> ExportJobRecord {
    let omission = OmissionReason {
        record_class_id: RecordClassId::ProviderLinkedWorkItemRecord,
        outcome: RecordOperationOutcome::OutsidePlatformScope,
        detail: Some(
            "Provider-owned issue bodies remained on the provider and were never possessed as managed data."
                .to_owned(),
        ),
    };
    ExportJobRecord {
        record_kind: "records_export_job_record".to_owned(),
        schema_version: 1,
        job_id: "export-job:provider-work-item:0001".to_owned(),
        requester_ref: "actor:local-user".to_owned(),
        request_case_id: "request-case:provider-work-item:0001".to_owned(),
        scope_selector: "scope:provider-work-item".to_owned(),
        record_class_ids: vec![RecordClassId::ProviderLinkedWorkItemRecord],
        redaction_profile: "metadata_safe_default".to_owned(),
        manifest: manifest(
            "manifest:provider-work-item:0001",
            RecordClassId::ProviderLinkedWorkItemRecord,
            RecordOperationOutcome::OutsidePlatformScope,
            vec![omission.clone()],
        ),
        outcome: RecordOperationOutcome::OutsidePlatformScope,
        partial_or_omission_reasons: vec![omission],
        bundle_refs: Vec::new(),
        chronology: chronology(
            "chronology:provider-work-item:0001",
            "scope:provider-work-item",
            RecordClassId::ProviderLinkedWorkItemRecord,
        ),
        summary: "Provider-linked work-item export emitted a manifest but disclosed outside_platform_scope for provider-owned payloads.".to_owned(),
    }
}

fn sync_export_job() -> ExportJobRecord {
    let omission = OmissionReason {
        record_class_id: RecordClassId::SyncMirrorLedger,
        outcome: RecordOperationOutcome::ManualLocalCaptureRequired,
        detail: Some(
            "Device-local revisions and cached recovery snapshots require manual local capture."
                .to_owned(),
        ),
    };
    ExportJobRecord {
        record_kind: "records_export_job_record".to_owned(),
        schema_version: 1,
        job_id: "export-job:sync-ledger:0001".to_owned(),
        requester_ref: "actor:local-user".to_owned(),
        request_case_id: "request-case:sync-ledger:0001".to_owned(),
        scope_selector: "scope:sync-ledger".to_owned(),
        record_class_ids: vec![RecordClassId::SyncMirrorLedger],
        redaction_profile: "metadata_safe_default".to_owned(),
        manifest: manifest(
            "manifest:sync-ledger:0001",
            RecordClassId::SyncMirrorLedger,
            RecordOperationOutcome::ManualLocalCaptureRequired,
            vec![omission.clone()],
        ),
        outcome: RecordOperationOutcome::ManualLocalCaptureRequired,
        partial_or_omission_reasons: vec![omission],
        bundle_refs: Vec::new(),
        chronology: chronology(
            "chronology:sync-ledger:0001",
            "scope:sync-ledger",
            RecordClassId::SyncMirrorLedger,
        ),
        summary: "Sync export emitted a manifest and guidance records for local-only device material that Aureline never possessed remotely.".to_owned(),
    }
}

fn incident_export_job() -> ExportJobRecord {
    let omission = OmissionReason {
        record_class_id: RecordClassId::IncidentSupportPacket,
        outcome: RecordOperationOutcome::OmittedByRedaction,
        detail: Some(
            "Incident export omitted redacted log slices and operator-only notes.".to_owned(),
        ),
    };
    ExportJobRecord {
        record_kind: "records_export_job_record".to_owned(),
        schema_version: 1,
        job_id: "export-job:incident-packet:0001".to_owned(),
        requester_ref: "actor:support".to_owned(),
        request_case_id: "request-case:incident-packet:0001".to_owned(),
        scope_selector: "scope:incident-packet".to_owned(),
        record_class_ids: vec![RecordClassId::IncidentSupportPacket],
        redaction_profile: "metadata_safe_default".to_owned(),
        manifest: manifest(
            "manifest:incident-packet:0001",
            RecordClassId::IncidentSupportPacket,
            RecordOperationOutcome::OmittedByRedaction,
            vec![omission.clone()],
        ),
        outcome: RecordOperationOutcome::OmittedByRedaction,
        partial_or_omission_reasons: vec![omission],
        bundle_refs: Vec::new(),
        chronology: chronology(
            "chronology:incident-packet:0001",
            "scope:incident-packet",
            RecordClassId::IncidentSupportPacket,
        ),
        summary: "Incident export emitted a manifest that preserved omitted_by_redaction rows instead of hiding them.".to_owned(),
    }
}

fn offboarding_export_job() -> ExportJobRecord {
    let omission = OmissionReason {
        record_class_id: RecordClassId::OffboardingExitPacket,
        outcome: RecordOperationOutcome::ManualLocalCaptureRequired,
        detail: Some(
            "Local workspace exports stay user-controlled and are referenced by pointer only."
                .to_owned(),
        ),
    };
    ExportJobRecord {
        record_kind: "records_export_job_record".to_owned(),
        schema_version: 1,
        job_id: "export-job:offboarding:0001".to_owned(),
        requester_ref: "actor:tenant-admin".to_owned(),
        request_case_id: "request-case:offboarding:0001".to_owned(),
        scope_selector: "scope:offboarding".to_owned(),
        record_class_ids: vec![RecordClassId::OffboardingExitPacket],
        redaction_profile: "metadata_safe_default".to_owned(),
        manifest: manifest(
            "manifest:offboarding:0001",
            RecordClassId::OffboardingExitPacket,
            RecordOperationOutcome::Partial,
            vec![omission.clone()],
        ),
        outcome: RecordOperationOutcome::Partial,
        partial_or_omission_reasons: vec![omission],
        bundle_refs: vec!["bundle:offboarding:0001".to_owned()],
        chronology: chronology(
            "chronology:offboarding:0001",
            "scope:offboarding",
            RecordClassId::OffboardingExitPacket,
        ),
        summary: "Offboarding export completed partially: managed bundles were emitted and local workspace captures remained manual and explicit.".to_owned(),
    }
}

fn ai_request_case() -> PrivacyRequestCaseRecord {
    PrivacyRequestCaseRecord {
        record_kind: "records_request_case_record".to_owned(),
        schema_version: 1,
        case_id: "request-case:ai-evidence:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:local-user".to_owned(),
        subject_scope_ref: "scope:ai-evidence".to_owned(),
        jurisdiction_or_policy_class: "privacy_policy_standard".to_owned(),
        approval_log_refs: vec!["approval:ai-evidence:0001".to_owned()],
        blocker_state: None,
        sla_target_at: "2026-06-20T00:00:00Z".to_owned(),
        status: RecordOperationOutcome::Partial,
        export_job_ids: vec!["export-job:ai-evidence:0001".to_owned()],
        delete_case_ids: vec!["delete-case:ai-evidence:0001".to_owned()],
        chronology_ref: "chronology:ai-evidence:0001".to_owned(),
        summary: "AI evidence privacy delete case completed partially because reviewed evidence copies remained policy-retained.".to_owned(),
    }
}

fn provider_request_case() -> PrivacyRequestCaseRecord {
    PrivacyRequestCaseRecord {
        record_kind: "records_request_case_record".to_owned(),
        schema_version: 1,
        case_id: "request-case:provider-work-item:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:local-user".to_owned(),
        subject_scope_ref: "scope:provider-work-item".to_owned(),
        jurisdiction_or_policy_class: "privacy_policy_standard".to_owned(),
        approval_log_refs: vec!["approval:provider-work-item:0001".to_owned()],
        blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:provider-work-item:not-found".to_owned(),
            blocker_outcome: RecordOperationOutcome::NotFound,
            blocker_ref: "provider-work-item:lookup:none".to_owned(),
            detail: "No locally retained managed copy matched the requested provider work-item deletion scope.".to_owned(),
            next_state_change_hint: None,
        }),
        sla_target_at: "2026-06-20T00:00:00Z".to_owned(),
        status: RecordOperationOutcome::NotFound,
        export_job_ids: vec!["export-job:provider-work-item:0001".to_owned()],
        delete_case_ids: vec!["delete-case:provider-work-item:0001".to_owned()],
        chronology_ref: "chronology:provider-work-item:0001".to_owned(),
        summary: "Provider-linked delete case remained inspectable and ended as not_found rather than implying provider-side deletion.".to_owned(),
    }
}

fn sync_request_case() -> PrivacyRequestCaseRecord {
    PrivacyRequestCaseRecord {
        record_kind: "records_request_case_record".to_owned(),
        schema_version: 1,
        case_id: "request-case:sync-ledger:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:local-user".to_owned(),
        subject_scope_ref: "scope:sync-ledger".to_owned(),
        jurisdiction_or_policy_class: "privacy_policy_standard".to_owned(),
        approval_log_refs: vec!["approval:sync-ledger:0001".to_owned()],
        blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:sync-ledger:hold".to_owned(),
            blocker_outcome: RecordOperationOutcome::BlockedByHold,
            blocker_ref: "hold:legal-77".to_owned(),
            detail: "Managed sync ledger deletion is blocked by an active hold while local device state remains outside managed hold scope.".to_owned(),
            next_state_change_hint: Some("Wait for hold release and replay the final execution-time check.".to_owned()),
        }),
        sla_target_at: "2026-06-20T00:00:00Z".to_owned(),
        status: RecordOperationOutcome::BlockedByHold,
        export_job_ids: vec!["export-job:sync-ledger:0001".to_owned()],
        delete_case_ids: vec!["delete-case:sync-ledger:0001".to_owned()],
        chronology_ref: "chronology:sync-ledger:0001".to_owned(),
        summary: "Sync delete case is blocked_by_hold and points at the local-only capture boundary explicitly.".to_owned(),
    }
}

fn incident_request_case() -> PrivacyRequestCaseRecord {
    PrivacyRequestCaseRecord {
        record_kind: "records_request_case_record".to_owned(),
        schema_version: 1,
        case_id: "request-case:incident-packet:0001".to_owned(),
        request_kind: RequestCaseKind::AccessExport,
        requester_ref: "actor:support".to_owned(),
        subject_scope_ref: "scope:incident-packet".to_owned(),
        jurisdiction_or_policy_class: "support_export_policy".to_owned(),
        approval_log_refs: vec!["approval:incident-packet:0001".to_owned()],
        blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:incident-packet:redaction".to_owned(),
            blocker_outcome: RecordOperationOutcome::OmittedByRedaction,
            blocker_ref: "redaction:incident-packet:policy".to_owned(),
            detail: "Incident export preserved redacted omissions rather than widening support visibility.".to_owned(),
            next_state_change_hint: None,
        }),
        sla_target_at: "2026-06-20T00:00:00Z".to_owned(),
        status: RecordOperationOutcome::OmittedByRedaction,
        export_job_ids: vec!["export-job:incident-packet:0001".to_owned()],
        delete_case_ids: vec!["delete-case:incident-packet:0001".to_owned()],
        chronology_ref: "chronology:incident-packet:0001".to_owned(),
        summary: "Incident export case carried omitted_by_redaction as a durable request-case outcome.".to_owned(),
    }
}

fn offboarding_request_case() -> PrivacyRequestCaseRecord {
    PrivacyRequestCaseRecord {
        record_kind: "records_request_case_record".to_owned(),
        schema_version: 1,
        case_id: "request-case:offboarding:0001".to_owned(),
        request_kind: RequestCaseKind::OffboardingDelete,
        requester_ref: "actor:tenant-admin".to_owned(),
        subject_scope_ref: "scope:offboarding".to_owned(),
        jurisdiction_or_policy_class: "offboarding_policy".to_owned(),
        approval_log_refs: vec!["approval:offboarding:0001".to_owned()],
        blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:offboarding:policy-retained".to_owned(),
            blocker_outcome: RecordOperationOutcome::PolicyRetained,
            blocker_ref: "retention-floor:offboarding:30d".to_owned(),
            detail: "Managed audit subsets remain policy-retained after offboarding and are disclosed immediately.".to_owned(),
            next_state_change_hint: Some("Re-evaluate at the retention floor horizon or after policy change.".to_owned()),
        }),
        sla_target_at: "2026-06-20T00:00:00Z".to_owned(),
        status: RecordOperationOutcome::PolicyRetained,
        export_job_ids: vec!["export-job:offboarding:0001".to_owned()],
        delete_case_ids: vec!["delete-case:offboarding:0001".to_owned()],
        chronology_ref: "chronology:offboarding:0001".to_owned(),
        summary: "Offboarding request case exported what the platform held and named policy-retained subsets instead of claiming clean deletion.".to_owned(),
    }
}

fn ai_delete_case() -> DeleteCaseRecord {
    DeleteCaseRecord {
        record_kind: "records_delete_case_record".to_owned(),
        schema_version: 1,
        case_id: "delete-case:ai-evidence:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:local-user".to_owned(),
        subject_scope_ref: "scope:ai-evidence".to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        planning_hold: hold_cleared(),
        execution_hold: hold_cleared(),
        prerequisite_export_job_ids: vec!["export-job:ai-evidence:0001".to_owned()],
        outcome: RecordOperationOutcome::Partial,
        unresolved_outcomes: vec![RecordOperationOutcome::PolicyRetained],
        destruction_receipt: Some(receipt(
            "receipt:ai-evidence:0001",
            "privacy_delete_ai_evidence",
            RecordClassId::AiRetainedEvidencePacket,
            Some(RecordOperationOutcome::PolicyRetained),
        )),
        typed_blocker_state: None,
        summary: "AI evidence delete case emitted a durable receipt and disclosed remaining policy-retained evidence copies.".to_owned(),
    }
}

fn provider_delete_case() -> DeleteCaseRecord {
    DeleteCaseRecord {
        record_kind: "records_delete_case_record".to_owned(),
        schema_version: 1,
        case_id: "delete-case:provider-work-item:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:local-user".to_owned(),
        subject_scope_ref: "scope:provider-work-item".to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        planning_hold: hold_cleared(),
        execution_hold: hold_cleared(),
        prerequisite_export_job_ids: vec!["export-job:provider-work-item:0001".to_owned()],
        outcome: RecordOperationOutcome::NotFound,
        unresolved_outcomes: Vec::new(),
        destruction_receipt: None,
        typed_blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:provider-work-item:not-found".to_owned(),
            blocker_outcome: RecordOperationOutcome::NotFound,
            blocker_ref: "provider-work-item:lookup:none".to_owned(),
            detail: "No managed copy or managed tombstone matched the requested provider work-item delete scope.".to_owned(),
            next_state_change_hint: None,
        }),
        summary: "Provider delete case returned not_found and never widened that into a provider-side delete claim.".to_owned(),
    }
}

fn sync_delete_case() -> DeleteCaseRecord {
    DeleteCaseRecord {
        record_kind: "records_delete_case_record".to_owned(),
        schema_version: 1,
        case_id: "delete-case:sync-ledger:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:local-user".to_owned(),
        subject_scope_ref: "scope:sync-ledger".to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        planning_hold: hold_active(
            "Local per-device snapshots are outside managed hold scope and remain user-controlled.",
        ),
        execution_hold: hold_active(
            "Local per-device snapshots are outside managed hold scope and remain user-controlled.",
        ),
        prerequisite_export_job_ids: vec!["export-job:sync-ledger:0001".to_owned()],
        outcome: RecordOperationOutcome::BlockedByHold,
        unresolved_outcomes: Vec::new(),
        destruction_receipt: None,
        typed_blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:sync-ledger:hold".to_owned(),
            blocker_outcome: RecordOperationOutcome::BlockedByHold,
            blocker_ref: "hold:legal-77".to_owned(),
            detail: "Final execution-time hold check blocked destructive completion for the managed sync ledger.".to_owned(),
            next_state_change_hint: Some("Re-run the delete after the active hold clears.".to_owned()),
        }),
        summary: "Sync delete case is explicitly blocked_by_hold; the platform does not imply local device material was deleted.".to_owned(),
    }
}

fn incident_delete_case() -> DeleteCaseRecord {
    DeleteCaseRecord {
        record_kind: "records_delete_case_record".to_owned(),
        schema_version: 1,
        case_id: "delete-case:incident-packet:0001".to_owned(),
        request_kind: RequestCaseKind::PrivacyDelete,
        requester_ref: "actor:support".to_owned(),
        subject_scope_ref: "scope:incident-packet".to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        planning_hold: hold_cleared(),
        execution_hold: hold_cleared(),
        prerequisite_export_job_ids: vec!["export-job:incident-packet:0001".to_owned()],
        outcome: RecordOperationOutcome::Completed,
        unresolved_outcomes: Vec::new(),
        destruction_receipt: Some(receipt(
            "receipt:incident-packet:0001",
            "delete_incident_packet",
            RecordClassId::IncidentSupportPacket,
            None,
        )),
        typed_blocker_state: None,
        summary: "Incident delete case completed and emitted a durable destruction receipt."
            .to_owned(),
    }
}

fn offboarding_delete_case() -> DeleteCaseRecord {
    DeleteCaseRecord {
        record_kind: "records_delete_case_record".to_owned(),
        schema_version: 1,
        case_id: "delete-case:offboarding:0001".to_owned(),
        request_kind: RequestCaseKind::OffboardingDelete,
        requester_ref: "actor:tenant-admin".to_owned(),
        subject_scope_ref: "scope:offboarding".to_owned(),
        policy_version: "policy-version:records-export-delete:v1".to_owned(),
        planning_hold: hold_cleared(),
        execution_hold: hold_cleared(),
        prerequisite_export_job_ids: vec!["export-job:offboarding:0001".to_owned()],
        outcome: RecordOperationOutcome::PolicyRetained,
        unresolved_outcomes: Vec::new(),
        destruction_receipt: None,
        typed_blocker_state: Some(CaseBlockerState {
            blocker_id: "blocker:offboarding:policy-retained".to_owned(),
            blocker_outcome: RecordOperationOutcome::PolicyRetained,
            blocker_ref: "retention-floor:offboarding:30d".to_owned(),
            detail: "Managed audit history and retained receipts remain under the offboarding retention floor.".to_owned(),
            next_state_change_hint: Some("Re-evaluate when the retention floor expires.".to_owned()),
        }),
        summary: "Offboarding delete case is policy_retained and cites the remaining-retention horizon instead of claiming completion.".to_owned(),
    }
}
