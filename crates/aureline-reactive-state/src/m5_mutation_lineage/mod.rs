//! Canonical M5 mutation-lineage packet for cross-surface history,
//! support export, and checkpoint lineage.
//!
//! This module freezes the shared M5 mutation-lineage vocabulary for the
//! notebook, request, data, preview, sync, repair, provider, workflow,
//! profiler, AI-evidence, and incident surfaces that now mutate material
//! state outside the core editor's ordinary buffer model.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/m5_mutation_lineage.schema.json`](../../../../schemas/state/m5_mutation_lineage.schema.json)
//! - [`/docs/state/m5_mutation_lineage.md`](../../../../docs/state/m5_mutation_lineage.md)
//! - [`/artifacts/state/m5_mutation_lineage.json`](../../../../artifacts/state/m5_mutation_lineage.json)
//! - [`/artifacts/state/m5_mutation_lineage.md`](../../../../artifacts/state/m5_mutation_lineage.md)
//! - [`/fixtures/state/m5_mutation_lineage/`](../../../../fixtures/state/m5_mutation_lineage/)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const M5_MUTATION_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const M5_MUTATION_LINEAGE_PACKET_RECORD_KIND: &str =
    "m5_mutation_lineage_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND: &str =
    "m5_mutation_lineage_fixture_record";

/// Repo-relative schema ref.
pub const M5_MUTATION_LINEAGE_SCHEMA_REF: &str =
    "schemas/state/m5_mutation_lineage.schema.json";

/// Repo-relative reviewer doc ref.
pub const M5_MUTATION_LINEAGE_DOC_REF: &str = "docs/state/m5_mutation_lineage.md";

/// Repo-relative machine-readable artifact packet.
pub const M5_MUTATION_LINEAGE_PACKET_REF: &str = "artifacts/state/m5_mutation_lineage.json";

/// Repo-relative reviewer artifact report.
pub const M5_MUTATION_LINEAGE_REPORT_REF: &str = "artifacts/state/m5_mutation_lineage.md";

/// Repo-relative fixture directory.
pub const M5_MUTATION_LINEAGE_FIXTURE_DIR: &str = "fixtures/state/m5_mutation_lineage";

/// Repo-relative fixture manifest.
pub const M5_MUTATION_LINEAGE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/m5_mutation_lineage/manifest.yaml";

/// One M5 mutation-bearing surface class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationSurfaceClass {
    NotebookDocument,
    NotebookOutput,
    RequestWorkspace,
    DataExportArtifact,
    PreviewOutput,
    SyncPacket,
    RepairTransaction,
    ProviderDraft,
    WorkflowBundle,
    ProfilerTrace,
    AiEvidencePacket,
    IncidentAction,
}

impl MutationSurfaceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookDocument => "notebook_document",
            Self::NotebookOutput => "notebook_output",
            Self::RequestWorkspace => "request_workspace",
            Self::DataExportArtifact => "data_export_artifact",
            Self::PreviewOutput => "preview_output",
            Self::SyncPacket => "sync_packet",
            Self::RepairTransaction => "repair_transaction",
            Self::ProviderDraft => "provider_draft",
            Self::WorkflowBundle => "workflow_bundle",
            Self::ProfilerTrace => "profiler_trace",
            Self::AiEvidencePacket => "ai_evidence_packet",
            Self::IncidentAction => "incident_action",
        }
    }
}

/// Artifact class the mutation touched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    NotebookFile,
    NotebookOutputBundle,
    RequestDocument,
    QueryExport,
    PreviewSnapshot,
    SyncManifest,
    RepairReceipt,
    ProviderDraft,
    WorkflowBundle,
    TraceCapture,
    AiEvidencePacket,
    IncidentPacket,
}

impl ArtifactClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookFile => "notebook_file",
            Self::NotebookOutputBundle => "notebook_output_bundle",
            Self::RequestDocument => "request_document",
            Self::QueryExport => "query_export",
            Self::PreviewSnapshot => "preview_snapshot",
            Self::SyncManifest => "sync_manifest",
            Self::RepairReceipt => "repair_receipt",
            Self::ProviderDraft => "provider_draft",
            Self::WorkflowBundle => "workflow_bundle",
            Self::TraceCapture => "trace_capture",
            Self::AiEvidencePacket => "ai_evidence_packet",
            Self::IncidentPacket => "incident_packet",
        }
    }
}

/// Explicit reversal vocabulary for M5 rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversalClass {
    Exact,
    GroupedExact,
    Compensate,
    Regenerate,
    Manual,
    AuditOnly,
}

impl ReversalClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::GroupedExact => "grouped_exact",
            Self::Compensate => "compensate",
            Self::Regenerate => "regenerate",
            Self::Manual => "manual",
            Self::AuditOnly => "audit_only",
        }
    }

    const fn severity(self) -> u8 {
        match self {
            Self::Exact => 0,
            Self::GroupedExact => 1,
            Self::Compensate => 2,
            Self::Regenerate => 3,
            Self::Manual => 4,
            Self::AuditOnly => 5,
        }
    }
}

/// Actor or executor class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    InteractiveUser,
    NotebookRuntime,
    QueryRunner,
    PreviewPublisher,
    SyncEngine,
    RepairExecutor,
    ProviderPublisher,
    WorkflowBundleRunner,
    ProfilerCaptureService,
    AiAssistant,
    IncidentResponder,
}

impl ActorClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InteractiveUser => "interactive_user",
            Self::NotebookRuntime => "notebook_runtime",
            Self::QueryRunner => "query_runner",
            Self::PreviewPublisher => "preview_publisher",
            Self::SyncEngine => "sync_engine",
            Self::RepairExecutor => "repair_executor",
            Self::ProviderPublisher => "provider_publisher",
            Self::WorkflowBundleRunner => "workflow_bundle_runner",
            Self::ProfilerCaptureService => "profiler_capture_service",
            Self::AiAssistant => "ai_assistant",
            Self::IncidentResponder => "incident_responder",
        }
    }
}

/// Source or provenance class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    HumanLocal,
    MachineLocal,
    MachineRemoteAgent,
    AiHostedProvider,
    PolicyDriven,
    ImportedEvidence,
}

impl SourceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanLocal => "human_local",
            Self::MachineLocal => "machine_local",
            Self::MachineRemoteAgent => "machine_remote_agent",
            Self::AiHostedProvider => "ai_hosted_provider",
            Self::PolicyDriven => "policy_driven",
            Self::ImportedEvidence => "imported_evidence",
        }
    }
}

/// Scope class the mutation touched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    Workspace,
    Notebook,
    RequestWorkspace,
    DataWorkspace,
    PreviewRoute,
    SyncLane,
    RepairScope,
    ProviderDraft,
    WorkflowBundle,
    PerformanceSession,
    IncidentWorkspace,
}

impl ScopeClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Notebook => "notebook",
            Self::RequestWorkspace => "request_workspace",
            Self::DataWorkspace => "data_workspace",
            Self::PreviewRoute => "preview_route",
            Self::SyncLane => "sync_lane",
            Self::RepairScope => "repair_scope",
            Self::ProviderDraft => "provider_draft",
            Self::WorkflowBundle => "workflow_bundle",
            Self::PerformanceSession => "performance_session",
            Self::IncidentWorkspace => "incident_workspace",
        }
    }
}

/// Checkpoint class used by the lineage packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointClass {
    PreMutation,
    ExecutionBundle,
    ExportSnapshot,
    ReconciliationCheckpoint,
    RepairCheckpoint,
    IncidentCapture,
}

impl CheckpointClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreMutation => "pre_mutation",
            Self::ExecutionBundle => "execution_bundle",
            Self::ExportSnapshot => "export_snapshot",
            Self::ReconciliationCheckpoint => "reconciliation_checkpoint",
            Self::RepairCheckpoint => "repair_checkpoint",
            Self::IncidentCapture => "incident_capture",
        }
    }
}

/// Role a checkpoint plays in the lineage chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointRole {
    CreatedBeforeMutation,
    ProducedByMutation,
    RequiredForManualRecovery,
}

impl CheckpointRole {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreatedBeforeMutation => "created_before_mutation",
            Self::ProducedByMutation => "produced_by_mutation",
            Self::RequiredForManualRecovery => "required_for_manual_recovery",
        }
    }
}

/// Automation influence that shaped the mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationInfluence {
    None,
    NotebookRuntime,
    QueryPlanAutomation,
    PreviewPublishAutomation,
    SyncReconciliation,
    WorkflowBundle,
    RepairTransaction,
    IncidentCapture,
    AiEvidenceCapture,
}

impl AutomationInfluence {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NotebookRuntime => "notebook_runtime",
            Self::QueryPlanAutomation => "query_plan_automation",
            Self::PreviewPublishAutomation => "preview_publish_automation",
            Self::SyncReconciliation => "sync_reconciliation",
            Self::WorkflowBundle => "workflow_bundle",
            Self::RepairTransaction => "repair_transaction",
            Self::IncidentCapture => "incident_capture",
            Self::AiEvidenceCapture => "ai_evidence_capture",
        }
    }
}

/// Policy influence that narrowed or admitted the mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyInfluence {
    None,
    PolicyChecked,
    ApprovalBound,
    ReauthGate,
    IncidentRetention,
    ProviderPublishRules,
}

impl PolicyInfluence {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PolicyChecked => "policy_checked",
            Self::ApprovalBound => "approval_bound",
            Self::ReauthGate => "reauth_gate",
            Self::IncidentRetention => "incident_retention",
            Self::ProviderPublishRules => "provider_publish_rules",
        }
    }
}

/// Group phase inside one lineage root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupPhaseClass {
    PrimaryMutation,
    DerivedArtifactFollowOn,
    AuditCapture,
    RepairFollowOn,
    IncidentFollowOn,
}

impl GroupPhaseClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryMutation => "primary_mutation",
            Self::DerivedArtifactFollowOn => "derived_artifact_follow_on",
            Self::AuditCapture => "audit_capture",
            Self::RepairFollowOn => "repair_follow_on",
            Self::IncidentFollowOn => "incident_follow_on",
        }
    }
}

/// One checkpoint ref attached to a mutation or lineage root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointRef {
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Checkpoint class.
    pub checkpoint_class: CheckpointClass,
    /// Role the checkpoint plays.
    pub checkpoint_role: CheckpointRole,
}

/// One mutation entry in the unified M5 journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationEntry {
    /// Stable mutation id.
    pub mutation_id: String,
    /// Mutation group id for the exact apply step.
    pub group_id: String,
    /// Shared lineage root that ties follow-on evidence and repair work back to
    /// the original change thread.
    pub lineage_root_id: String,
    /// Surface class that emitted this entry.
    pub surface_class: MutationSurfaceClass,
    /// Artifact class the mutation touched.
    pub artifact_class: ArtifactClass,
    /// Actor or executor class.
    pub actor_class: ActorClass,
    /// Provenance class.
    pub source_class: SourceClass,
    /// Scope class under mutation.
    pub scope_class: ScopeClass,
    /// Stable scope id.
    pub scope_id: String,
    /// Checkpoints visible from creation through follow-on repair or export.
    pub checkpoint_refs: Vec<CheckpointRef>,
    /// Explicit reversal class.
    pub reversal_class: ReversalClass,
    /// Count of affected file-like objects surfaced to users and support.
    pub affected_file_count: u32,
    /// Automation influence preserved in history and export.
    pub automation_influence: AutomationInfluence,
    /// Policy influence preserved in history and export.
    pub policy_influence: PolicyInfluence,
    /// Consumers that quote this mutation directly.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One group record for a visible mutation phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGroupRecord {
    /// Stable group id.
    pub group_id: String,
    /// Shared lineage root id.
    pub lineage_root_id: String,
    /// Phase class within the lineage root.
    pub phase_class: GroupPhaseClass,
    /// Reviewer-facing title.
    pub title: String,
    /// Primary surface class for the group.
    pub primary_surface_class: MutationSurfaceClass,
    /// Member mutation ids in stable order.
    pub member_mutation_ids: Vec<String>,
    /// Group reversal class.
    pub reversal_class: ReversalClass,
    /// Aggregated file count across member entries.
    pub total_file_count: u32,
    /// Aggregated artifact classes across member entries.
    pub artifact_classes: Vec<ArtifactClass>,
    /// Aggregated automation influences across member entries.
    pub automation_influences: Vec<AutomationInfluence>,
    /// Aggregated policy influences across member entries.
    pub policy_influences: Vec<PolicyInfluence>,
    /// Group-visible checkpoint lineage.
    pub checkpoint_refs: Vec<CheckpointRef>,
    /// Consumer refs that reopen or export this group directly.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// Cross-surface history inspector row aggregated by lineage root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryInspectorRow {
    /// Stable row id.
    pub row_id: String,
    /// Shared lineage root id.
    pub lineage_root_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Primary surface class that anchors the row.
    pub primary_surface_class: MutationSurfaceClass,
    /// All related groups.
    pub group_ids: Vec<String>,
    /// Mutation ids visible from the row.
    pub mutation_ids: Vec<String>,
    /// Highest-risk reversal class on the row.
    pub highest_risk_reversal_class: ReversalClass,
    /// Every reversal class present on the row.
    pub reversal_classes: Vec<ReversalClass>,
    /// Aggregated file count.
    pub total_file_count: u32,
    /// Artifact classes preserved on the row.
    pub artifact_classes: Vec<ArtifactClass>,
    /// Automation influences preserved on the row.
    pub automation_influences: Vec<AutomationInfluence>,
    /// Policy influences preserved on the row.
    pub policy_influences: Vec<PolicyInfluence>,
    /// Checkpoint ids a user can inspect from the row.
    pub checkpoint_ids: Vec<String>,
    /// Reviewer-facing reopen action label.
    pub reopen_action_label: String,
    /// Short reviewer note.
    pub notes: String,
}

/// Support-export manifest row aggregated by lineage root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportManifestRow {
    /// Stable row id.
    pub row_id: String,
    /// Shared lineage root id.
    pub lineage_root_id: String,
    /// All related groups.
    pub group_ids: Vec<String>,
    /// All mutation ids preserved by the manifest.
    pub mutation_ids: Vec<String>,
    /// Highest-risk reversal class carried by the manifest.
    pub highest_risk_reversal_class: ReversalClass,
    /// Every reversal class preserved by the manifest.
    pub reversal_classes: Vec<ReversalClass>,
    /// Aggregated file count.
    pub total_file_count: u32,
    /// Artifact classes preserved without raw payloads.
    pub artifact_classes: Vec<ArtifactClass>,
    /// Automation influences preserved without raw payloads.
    pub automation_influences: Vec<AutomationInfluence>,
    /// Policy influences preserved without raw payloads.
    pub policy_influences: Vec<PolicyInfluence>,
    /// Metadata-safe export invariant.
    pub raw_payload_excluded: bool,
    /// Metadata-safe export invariant.
    pub raw_private_material_excluded: bool,
    /// Metadata-safe export invariant.
    pub ambient_authority_excluded: bool,
    /// True when the grouped row preserves one journal lineage thread.
    pub single_lineage_thread_preserved: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the unified M5 mutation-lineage contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationLineagePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Mutation entries.
    pub entries: Vec<MutationEntry>,
    /// Mutation groups.
    pub groups: Vec<MutationGroupRecord>,
    /// History inspector rows.
    pub history_inspector_rows: Vec<HistoryInspectorRow>,
    /// Support-export manifest rows.
    pub support_export_rows: Vec<SupportExportManifestRow>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture row bound to a lineage root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationLineageFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Shared lineage root under test.
    pub expected_lineage_root_id: String,
    /// Expected primary surface class.
    pub primary_surface_class: MutationSurfaceClass,
    /// Expected highest-risk reversal class.
    pub highest_risk_reversal_class: ReversalClass,
    /// Expected file count.
    pub total_file_count: u32,
    /// Expected artifact classes.
    pub artifact_classes: Vec<ArtifactClass>,
    /// Expected automation influences.
    pub automation_influences: Vec<AutomationInfluence>,
    /// Expected policy influences.
    pub policy_influences: Vec<PolicyInfluence>,
    /// Expected history inspector row.
    pub history_inspector_row_id: String,
    /// Expected support-export row.
    pub support_export_row_id: String,
    /// One consumer that quotes this lineage root.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "m5 mutation lineage validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// Returns the checked-in packet this lane freezes.
pub fn seeded_m5_mutation_lineage_packet() -> M5MutationLineagePacket {
    let entries = vec![
        MutationEntry {
            mutation_id: "mutation:m5:notebook_document:0001".to_owned(),
            group_id: "group:m5:notebook_document:0001".to_owned(),
            lineage_root_id: "lineage:m5:notebook_execution:0001".to_owned(),
            surface_class: MutationSurfaceClass::NotebookDocument,
            artifact_class: ArtifactClass::NotebookFile,
            actor_class: ActorClass::InteractiveUser,
            source_class: SourceClass::HumanLocal,
            scope_class: ScopeClass::Notebook,
            scope_id: "scope:notebook:daily-retention".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:notebook:pre_edit:0001",
                    CheckpointClass::PreMutation,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:notebook:post_edit:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::Exact,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::None,
            policy_influence: PolicyInfluence::PolicyChecked,
            consumer_refs: vec![
                "crates/aureline-notebook/src/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/mod.rs".to_owned(),
                "crates/aureline-shell/src/notebook_alpha/mod.rs".to_owned(),
            ],
            notes: "Notebook document edits stay attributable to the local user and preserve the pre-edit checkpoint.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:notebook_output:0001".to_owned(),
            group_id: "group:m5:notebook_output:0001".to_owned(),
            lineage_root_id: "lineage:m5:notebook_execution:0001".to_owned(),
            surface_class: MutationSurfaceClass::NotebookOutput,
            artifact_class: ArtifactClass::NotebookOutputBundle,
            actor_class: ActorClass::NotebookRuntime,
            source_class: SourceClass::MachineLocal,
            scope_class: ScopeClass::Notebook,
            scope_id: "scope:notebook:daily-retention".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:notebook:post_edit:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:notebook:output_refresh:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::Regenerate,
            affected_file_count: 2,
            automation_influence: AutomationInfluence::NotebookRuntime,
            policy_influence: PolicyInfluence::PolicyChecked,
            consumer_refs: vec![
                "crates/aureline-notebook/src/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories/mod.rs".to_owned(),
                "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            ],
            notes: "Notebook output refreshes preserve the input checkpoint but only promise regeneration, not exact undo.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:ai_evidence:0001".to_owned(),
            group_id: "group:m5:ai_evidence:0001".to_owned(),
            lineage_root_id: "lineage:m5:notebook_execution:0001".to_owned(),
            surface_class: MutationSurfaceClass::AiEvidencePacket,
            artifact_class: ArtifactClass::AiEvidencePacket,
            actor_class: ActorClass::AiAssistant,
            source_class: SourceClass::AiHostedProvider,
            scope_class: ScopeClass::Notebook,
            scope_id: "scope:notebook:daily-retention".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:notebook:output_refresh:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:notebook:evidence_packet:0001",
                    CheckpointClass::ExportSnapshot,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::AuditOnly,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::AiEvidenceCapture,
            policy_influence: PolicyInfluence::ApprovalBound,
            consumer_refs: vec![
                "crates/aureline-ai/src/evidence/mod.rs".to_owned(),
                "crates/aureline-support/src/m5_mutation_lineage/mod.rs".to_owned(),
            ],
            notes: "AI evidence packets stay on the same lineage root as the notebook mutation without claiming an editable undo path.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:request_workspace:0001".to_owned(),
            group_id: "group:m5:request_batch:0001".to_owned(),
            lineage_root_id: "lineage:m5:request_batch:0001".to_owned(),
            surface_class: MutationSurfaceClass::RequestWorkspace,
            artifact_class: ArtifactClass::RequestDocument,
            actor_class: ActorClass::InteractiveUser,
            source_class: SourceClass::HumanLocal,
            scope_class: ScopeClass::RequestWorkspace,
            scope_id: "scope:request:catalog-sync".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:request:pre_apply:0001",
                    CheckpointClass::PreMutation,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:request:batched_apply:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::GroupedExact,
            affected_file_count: 2,
            automation_influence: AutomationInfluence::None,
            policy_influence: PolicyInfluence::ApprovalBound,
            consumer_refs: vec![
                "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                "crates/aureline-shell/src/review_preview/mod.rs".to_owned(),
            ],
            notes: "Request-workspace apply stays preview-first and grouped under one exact rollback boundary.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:data_export:0001".to_owned(),
            group_id: "group:m5:request_batch:0001".to_owned(),
            lineage_root_id: "lineage:m5:request_batch:0001".to_owned(),
            surface_class: MutationSurfaceClass::DataExportArtifact,
            artifact_class: ArtifactClass::QueryExport,
            actor_class: ActorClass::QueryRunner,
            source_class: SourceClass::MachineLocal,
            scope_class: ScopeClass::DataWorkspace,
            scope_id: "scope:data:catalog-sync".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:request:batched_apply:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:data:export_snapshot:0001",
                    CheckpointClass::ExportSnapshot,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::GroupedExact,
            affected_file_count: 2,
            automation_influence: AutomationInfluence::QueryPlanAutomation,
            policy_influence: PolicyInfluence::ApprovalBound,
            consumer_refs: vec![
                "crates/aureline-data/src/lib.rs".to_owned(),
                "crates/aureline-shell/src/m5_activity_objects/mod.rs".to_owned(),
            ],
            notes: "The request batch and its paired export stay on one grouped-exact lineage root.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:preview_output:0001".to_owned(),
            group_id: "group:m5:preview_publish:0001".to_owned(),
            lineage_root_id: "lineage:m5:preview_publish:0001".to_owned(),
            surface_class: MutationSurfaceClass::PreviewOutput,
            artifact_class: ArtifactClass::PreviewSnapshot,
            actor_class: ActorClass::PreviewPublisher,
            source_class: SourceClass::MachineRemoteAgent,
            scope_class: ScopeClass::PreviewRoute,
            scope_id: "scope:preview:release-notes".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:preview:pre_publish:0001",
                    CheckpointClass::PreMutation,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:preview:publish_snapshot:0001",
                    CheckpointClass::ExportSnapshot,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::Regenerate,
            affected_file_count: 3,
            automation_influence: AutomationInfluence::PreviewPublishAutomation,
            policy_influence: PolicyInfluence::PolicyChecked,
            consumer_refs: vec![
                "crates/aureline-preview/src/preview_origin/mod.rs".to_owned(),
                "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            ],
            notes: "Preview publish artifacts preserve their source checkpoint but require regeneration instead of exact undo.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:sync_packet:0001".to_owned(),
            group_id: "group:m5:sync_provider_stage:0001".to_owned(),
            lineage_root_id: "lineage:m5:provider_sync:0001".to_owned(),
            surface_class: MutationSurfaceClass::SyncPacket,
            artifact_class: ArtifactClass::SyncManifest,
            actor_class: ActorClass::SyncEngine,
            source_class: SourceClass::MachineRemoteAgent,
            scope_class: ScopeClass::SyncLane,
            scope_id: "scope:sync:provider-draft".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:sync:queued_state:0001",
                    CheckpointClass::ReconciliationCheckpoint,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:sync:stage_manifest:0001",
                    CheckpointClass::ReconciliationCheckpoint,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::Compensate,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::SyncReconciliation,
            policy_influence: PolicyInfluence::ReauthGate,
            consumer_refs: vec![
                "crates/aureline-sync/src/lib.rs".to_owned(),
                "crates/aureline-shell/src/activity_center/deferred_publish.rs".to_owned(),
            ],
            notes: "Sync packet staging never replays invisibly and advertises compensation instead of exact undo.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:provider_draft:0001".to_owned(),
            group_id: "group:m5:sync_provider_stage:0001".to_owned(),
            lineage_root_id: "lineage:m5:provider_sync:0001".to_owned(),
            surface_class: MutationSurfaceClass::ProviderDraft,
            artifact_class: ArtifactClass::ProviderDraft,
            actor_class: ActorClass::ProviderPublisher,
            source_class: SourceClass::MachineRemoteAgent,
            scope_class: ScopeClass::ProviderDraft,
            scope_id: "scope:provider:draft:release-notes".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:sync:stage_manifest:0001",
                    CheckpointClass::ReconciliationCheckpoint,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:provider:draft_stage:0001",
                    CheckpointClass::ReconciliationCheckpoint,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::Compensate,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::SyncReconciliation,
            policy_influence: PolicyInfluence::ProviderPublishRules,
            consumer_refs: vec![
                "crates/aureline-provider/src/lib.rs".to_owned(),
                "crates/aureline-shell/src/m5_activity_objects/mod.rs".to_owned(),
            ],
            notes: "Provider-local drafts stay local-first but compensation is the only honest publish reversal.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:repair_transaction:0001".to_owned(),
            group_id: "group:m5:repair_follow_on:0001".to_owned(),
            lineage_root_id: "lineage:m5:provider_sync:0001".to_owned(),
            surface_class: MutationSurfaceClass::RepairTransaction,
            artifact_class: ArtifactClass::RepairReceipt,
            actor_class: ActorClass::RepairExecutor,
            source_class: SourceClass::PolicyDriven,
            scope_class: ScopeClass::RepairScope,
            scope_id: "scope:repair:provider-sync".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:provider:draft_stage:0001",
                    CheckpointClass::RepairCheckpoint,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:repair:provider_sync:0001",
                    CheckpointClass::RepairCheckpoint,
                    CheckpointRole::ProducedByMutation,
                ),
                checkpoint(
                    "ckp:m5:repair:manual_rebind:0001",
                    CheckpointClass::RepairCheckpoint,
                    CheckpointRole::RequiredForManualRecovery,
                ),
            ],
            reversal_class: ReversalClass::Manual,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::RepairTransaction,
            policy_influence: PolicyInfluence::ReauthGate,
            consumer_refs: vec![
                "crates/aureline-support/src/repair/mod.rs".to_owned(),
                "crates/aureline-support/src/repair_transactions/mod.rs".to_owned(),
            ],
            notes: "Repair follow-on work stays in the same lineage root but narrows the visible recovery promise to manual.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:workflow_bundle:0001".to_owned(),
            group_id: "group:m5:repair_follow_on:0001".to_owned(),
            lineage_root_id: "lineage:m5:provider_sync:0001".to_owned(),
            surface_class: MutationSurfaceClass::WorkflowBundle,
            artifact_class: ArtifactClass::WorkflowBundle,
            actor_class: ActorClass::WorkflowBundleRunner,
            source_class: SourceClass::MachineLocal,
            scope_class: ScopeClass::WorkflowBundle,
            scope_id: "scope:workflow:provider-sync".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:repair:provider_sync:0001",
                    CheckpointClass::RepairCheckpoint,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:workflow:bundle_projection:0001",
                    CheckpointClass::ExportSnapshot,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::Manual,
            affected_file_count: 2,
            automation_influence: AutomationInfluence::WorkflowBundle,
            policy_influence: PolicyInfluence::ApprovalBound,
            consumer_refs: vec![
                "crates/aureline-templates/src/lib.rs".to_owned(),
                "crates/aureline-shell/src/portable_bundle_inspector/mod.rs".to_owned(),
            ],
            notes: "Workflow bundles preserve provider/sync lineage without implying the bundle import can be silently undone.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:profiler_trace:0001".to_owned(),
            group_id: "group:m5:profiler_capture:0001".to_owned(),
            lineage_root_id: "lineage:m5:incident_capture:0001".to_owned(),
            surface_class: MutationSurfaceClass::ProfilerTrace,
            artifact_class: ArtifactClass::TraceCapture,
            actor_class: ActorClass::ProfilerCaptureService,
            source_class: SourceClass::MachineLocal,
            scope_class: ScopeClass::PerformanceSession,
            scope_id: "scope:perf:release-smoke".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:profiler:capture_basis:0001",
                    CheckpointClass::ExecutionBundle,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:profiler:trace_capture:0001",
                    CheckpointClass::IncidentCapture,
                    CheckpointRole::ProducedByMutation,
                ),
            ],
            reversal_class: ReversalClass::AuditOnly,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::IncidentCapture,
            policy_influence: PolicyInfluence::IncidentRetention,
            consumer_refs: vec![
                "crates/aureline-profiler/src/lib.rs".to_owned(),
                "crates/aureline-support/src/m5_forensic_packet/mod.rs".to_owned(),
            ],
            notes: "Trace capture is evidence-bearing and attributable, but not user-undoable.".to_owned(),
        },
        MutationEntry {
            mutation_id: "mutation:m5:incident_action:0001".to_owned(),
            group_id: "group:m5:incident_follow_on:0001".to_owned(),
            lineage_root_id: "lineage:m5:incident_capture:0001".to_owned(),
            surface_class: MutationSurfaceClass::IncidentAction,
            artifact_class: ArtifactClass::IncidentPacket,
            actor_class: ActorClass::IncidentResponder,
            source_class: SourceClass::ImportedEvidence,
            scope_class: ScopeClass::IncidentWorkspace,
            scope_id: "scope:incident:release-smoke".to_owned(),
            checkpoint_refs: vec![
                checkpoint(
                    "ckp:m5:profiler:trace_capture:0001",
                    CheckpointClass::IncidentCapture,
                    CheckpointRole::CreatedBeforeMutation,
                ),
                checkpoint(
                    "ckp:m5:incident:triage_packet:0001",
                    CheckpointClass::IncidentCapture,
                    CheckpointRole::ProducedByMutation,
                ),
                checkpoint(
                    "ckp:m5:incident:operator_reopen:0001",
                    CheckpointClass::IncidentCapture,
                    CheckpointRole::RequiredForManualRecovery,
                ),
            ],
            reversal_class: ReversalClass::Manual,
            affected_file_count: 1,
            automation_influence: AutomationInfluence::IncidentCapture,
            policy_influence: PolicyInfluence::IncidentRetention,
            consumer_refs: vec![
                "crates/aureline-incident/src/lib.rs".to_owned(),
                "crates/aureline-support/src/incident_workspace/mod.rs".to_owned(),
            ],
            notes: "Incident actions stay on the same lineage root as the trace capture but require operator-driven recovery.".to_owned(),
        },
    ];

    let groups = vec![
        mutation_group(
            "group:m5:notebook_document:0001",
            "lineage:m5:notebook_execution:0001",
            GroupPhaseClass::PrimaryMutation,
            "Notebook document apply",
            MutationSurfaceClass::NotebookDocument,
            vec!["mutation:m5:notebook_document:0001"],
            &entries,
            vec![
                "crates/aureline-notebook/src/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/mod.rs".to_owned(),
            ],
            "The authored notebook edit anchors the lineage thread.",
        ),
        mutation_group(
            "group:m5:notebook_output:0001",
            "lineage:m5:notebook_execution:0001",
            GroupPhaseClass::DerivedArtifactFollowOn,
            "Notebook output refresh",
            MutationSurfaceClass::NotebookOutput,
            vec!["mutation:m5:notebook_output:0001"],
            &entries,
            vec![
                "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            ],
            "Derived notebook output follows the same lineage root with regenerate-only recovery.",
        ),
        mutation_group(
            "group:m5:ai_evidence:0001",
            "lineage:m5:notebook_execution:0001",
            GroupPhaseClass::AuditCapture,
            "AI evidence capture",
            MutationSurfaceClass::AiEvidencePacket,
            vec!["mutation:m5:ai_evidence:0001"],
            &entries,
            vec![
                "crates/aureline-ai/src/evidence/mod.rs".to_owned(),
            ],
            "AI evidence capture remains attributable on the notebook lineage thread.",
        ),
        mutation_group(
            "group:m5:request_batch:0001",
            "lineage:m5:request_batch:0001",
            GroupPhaseClass::PrimaryMutation,
            "Request batch apply",
            MutationSurfaceClass::RequestWorkspace,
            vec![
                "mutation:m5:request_workspace:0001",
                "mutation:m5:data_export:0001",
            ],
            &entries,
            vec![
                "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                "crates/aureline-data/src/lib.rs".to_owned(),
            ],
            "Request edits and paired export artifacts share one grouped-exact rollback boundary.",
        ),
        mutation_group(
            "group:m5:preview_publish:0001",
            "lineage:m5:preview_publish:0001",
            GroupPhaseClass::DerivedArtifactFollowOn,
            "Preview publish refresh",
            MutationSurfaceClass::PreviewOutput,
            vec!["mutation:m5:preview_output:0001"],
            &entries,
            vec![
                "crates/aureline-preview/src/preview_origin/mod.rs".to_owned(),
            ],
            "Preview publication captures its checkpoint lineage but narrows to regeneration.",
        ),
        mutation_group(
            "group:m5:sync_provider_stage:0001",
            "lineage:m5:provider_sync:0001",
            GroupPhaseClass::PrimaryMutation,
            "Sync and provider draft stage",
            MutationSurfaceClass::SyncPacket,
            vec![
                "mutation:m5:sync_packet:0001",
                "mutation:m5:provider_draft:0001",
            ],
            &entries,
            vec![
                "crates/aureline-shell/src/activity_center/deferred_publish.rs".to_owned(),
                "crates/aureline-provider/src/lib.rs".to_owned(),
            ],
            "Queued provider-facing work stays compensating and reviewable under reauth and publish rules.",
        ),
        mutation_group(
            "group:m5:repair_follow_on:0001",
            "lineage:m5:provider_sync:0001",
            GroupPhaseClass::RepairFollowOn,
            "Repair and workflow follow-on",
            MutationSurfaceClass::RepairTransaction,
            vec![
                "mutation:m5:repair_transaction:0001",
                "mutation:m5:workflow_bundle:0001",
            ],
            &entries,
            vec![
                "crates/aureline-support/src/repair/mod.rs".to_owned(),
                "crates/aureline-shell/src/portable_bundle_inspector/mod.rs".to_owned(),
            ],
            "Repair and workflow bundle follow-on share the same provider/sync lineage but reduce recovery to manual.",
        ),
        mutation_group(
            "group:m5:profiler_capture:0001",
            "lineage:m5:incident_capture:0001",
            GroupPhaseClass::AuditCapture,
            "Profiler trace capture",
            MutationSurfaceClass::ProfilerTrace,
            vec!["mutation:m5:profiler_trace:0001"],
            &entries,
            vec![
                "crates/aureline-profiler/src/lib.rs".to_owned(),
            ],
            "Trace capture seeds the incident lineage thread as audit-only evidence.",
        ),
        mutation_group(
            "group:m5:incident_follow_on:0001",
            "lineage:m5:incident_capture:0001",
            GroupPhaseClass::IncidentFollowOn,
            "Incident packet follow-on",
            MutationSurfaceClass::IncidentAction,
            vec!["mutation:m5:incident_action:0001"],
            &entries,
            vec![
                "crates/aureline-incident/src/lib.rs".to_owned(),
                "crates/aureline-support/src/incident_workspace/mod.rs".to_owned(),
            ],
            "Incident actions remain attributable to the original trace capture and demand manual operator recovery.",
        ),
    ];

    let history_inspector_rows = vec![
        inspector_row(
            "history:m5:notebook_execution:0001",
            "lineage:m5:notebook_execution:0001",
            "Notebook execution lineage",
            MutationSurfaceClass::NotebookDocument,
            &groups,
            &entries,
            "Open notebook lineage",
            "Notebook edits, generated outputs, and AI evidence stay joinable on one history thread.",
        ),
        inspector_row(
            "history:m5:request_batch:0001",
            "lineage:m5:request_batch:0001",
            "Request batch lineage",
            MutationSurfaceClass::RequestWorkspace,
            &groups,
            &entries,
            "Open request batch review",
            "Request changes and paired exports surface grouped-exact rollback instead of a vague undo promise.",
        ),
        inspector_row(
            "history:m5:preview_publish:0001",
            "lineage:m5:preview_publish:0001",
            "Preview publish lineage",
            MutationSurfaceClass::PreviewOutput,
            &groups,
            &entries,
            "Open preview publish history",
            "Preview outputs remain attributable to their source checkpoint and show regeneration as the visible recovery path.",
        ),
        inspector_row(
            "history:m5:provider_sync:0001",
            "lineage:m5:provider_sync:0001",
            "Provider sync and repair lineage",
            MutationSurfaceClass::SyncPacket,
            &groups,
            &entries,
            "Open provider sync recovery",
            "Queued publish work, repair, and workflow follow-on stay on one lineage root while preserving compensate versus manual recovery classes.",
        ),
        inspector_row(
            "history:m5:incident_capture:0001",
            "lineage:m5:incident_capture:0001",
            "Profiler and incident lineage",
            MutationSurfaceClass::ProfilerTrace,
            &groups,
            &entries,
            "Open incident lineage",
            "Trace capture and incident follow-on stay joined without pretending evidence capture is directly undoable.",
        ),
    ];

    let support_export_rows = vec![
        support_export_row(
            "support:m5:notebook_execution:0001",
            "lineage:m5:notebook_execution:0001",
            &groups,
            &entries,
            "Notebook support export preserves authored, generated, and evidence-bearing mutations on one lineage thread without raw notebook payloads.",
        ),
        support_export_row(
            "support:m5:request_batch:0001",
            "lineage:m5:request_batch:0001",
            &groups,
            &entries,
            "Request support export preserves the grouped batch and its export side effects with exact file counts and artifact classes.",
        ),
        support_export_row(
            "support:m5:preview_publish:0001",
            "lineage:m5:preview_publish:0001",
            &groups,
            &entries,
            "Preview support export preserves regeneration posture and publish automation without raw preview bytes.",
        ),
        support_export_row(
            "support:m5:provider_sync:0001",
            "lineage:m5:provider_sync:0001",
            &groups,
            &entries,
            "Provider/sync support export preserves queued publish, repair, and workflow lineage while keeping reauth and policy influence explicit.",
        ),
        support_export_row(
            "support:m5:incident_capture:0001",
            "lineage:m5:incident_capture:0001",
            &groups,
            &entries,
            "Incident support export preserves evidence capture and incident follow-on lineage without raw trace payloads.",
        ),
    ];

    M5MutationLineagePacket {
        record_kind: M5_MUTATION_LINEAGE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_MUTATION_LINEAGE_SCHEMA_VERSION,
        packet_id: "state.m5_mutation_lineage.v1".to_owned(),
        title: "Unified M5 mutation journal, reversal classes, checkpoint lineage, and history inspectors"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: M5_MUTATION_LINEAGE_DOC_REF.to_owned(),
            schema_ref: M5_MUTATION_LINEAGE_SCHEMA_REF.to_owned(),
            packet_ref: M5_MUTATION_LINEAGE_PACKET_REF.to_owned(),
            report_ref: M5_MUTATION_LINEAGE_REPORT_REF.to_owned(),
            fixture_manifest_ref: M5_MUTATION_LINEAGE_FIXTURE_MANIFEST_REF.to_owned(),
        },
        entries,
        groups,
        history_inspector_rows,
        support_export_rows,
        invariants: vec![
            "Every material M5 mutation emits one attributable journal entry with actor, source, scope, checkpoint lineage, and explicit reversal class.".to_owned(),
            "Cross-surface history inspectors aggregate by lineage root so notebook, request, preview, sync, repair, provider, profiler, AI-evidence, workflow, and incident follow-on actions do not invent parallel history models.".to_owned(),
            "Support-export rows preserve file count, artifact class, automation influence, and policy influence without embedding raw payloads or ambient authority.".to_owned(),
            "Exact language is reserved for exact or grouped-exact rows; compensate, regenerate, manual, and audit-only remain visibly narrower.".to_owned(),
            "Deferred or managed follow-on work never replays invisibly; reauth, approval, and manual-recovery requirements stay attached to the same lineage root.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture rows this lane freezes.
pub fn seeded_m5_mutation_lineage_fixtures() -> Vec<M5MutationLineageFixture> {
    vec![
        M5MutationLineageFixture {
            record_kind: M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: M5_MUTATION_LINEAGE_SCHEMA_VERSION,
            fixture_id: "fixture:m5:notebook_execution".to_owned(),
            expected_lineage_root_id: "lineage:m5:notebook_execution:0001".to_owned(),
            primary_surface_class: MutationSurfaceClass::NotebookDocument,
            highest_risk_reversal_class: ReversalClass::AuditOnly,
            total_file_count: 4,
            artifact_classes: vec![
                ArtifactClass::NotebookFile,
                ArtifactClass::NotebookOutputBundle,
                ArtifactClass::AiEvidencePacket,
            ],
            automation_influences: vec![
                AutomationInfluence::AiEvidenceCapture,
                AutomationInfluence::NotebookRuntime,
                AutomationInfluence::None,
            ],
            policy_influences: vec![
                PolicyInfluence::ApprovalBound,
                PolicyInfluence::PolicyChecked,
            ],
            history_inspector_row_id: "history:m5:notebook_execution:0001".to_owned(),
            support_export_row_id: "support:m5:notebook_execution:0001".to_owned(),
            consumer_ref: "crates/aureline-shell/src/notebook_alpha/mod.rs".to_owned(),
            notes: "Notebook lineage proves authored edit, generated output, and AI evidence join the same checkpoint thread.".to_owned(),
        },
        M5MutationLineageFixture {
            record_kind: M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: M5_MUTATION_LINEAGE_SCHEMA_VERSION,
            fixture_id: "fixture:m5:request_batch".to_owned(),
            expected_lineage_root_id: "lineage:m5:request_batch:0001".to_owned(),
            primary_surface_class: MutationSurfaceClass::RequestWorkspace,
            highest_risk_reversal_class: ReversalClass::GroupedExact,
            total_file_count: 4,
            artifact_classes: vec![ArtifactClass::QueryExport, ArtifactClass::RequestDocument],
            automation_influences: vec![
                AutomationInfluence::None,
                AutomationInfluence::QueryPlanAutomation,
            ],
            policy_influences: vec![PolicyInfluence::ApprovalBound],
            history_inspector_row_id: "history:m5:request_batch:0001".to_owned(),
            support_export_row_id: "support:m5:request_batch:0001".to_owned(),
            consumer_ref: "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
            notes: "Request batch lineage proves grouped exact recovery survives paired data exports.".to_owned(),
        },
        M5MutationLineageFixture {
            record_kind: M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: M5_MUTATION_LINEAGE_SCHEMA_VERSION,
            fixture_id: "fixture:m5:preview_publish".to_owned(),
            expected_lineage_root_id: "lineage:m5:preview_publish:0001".to_owned(),
            primary_surface_class: MutationSurfaceClass::PreviewOutput,
            highest_risk_reversal_class: ReversalClass::Regenerate,
            total_file_count: 3,
            artifact_classes: vec![ArtifactClass::PreviewSnapshot],
            automation_influences: vec![AutomationInfluence::PreviewPublishAutomation],
            policy_influences: vec![PolicyInfluence::PolicyChecked],
            history_inspector_row_id: "history:m5:preview_publish:0001".to_owned(),
            support_export_row_id: "support:m5:preview_publish:0001".to_owned(),
            consumer_ref: "crates/aureline-preview/src/preview_origin/mod.rs".to_owned(),
            notes: "Preview publish lineage proves regeneration stays explicit and does not collapse into generic undo.".to_owned(),
        },
        M5MutationLineageFixture {
            record_kind: M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: M5_MUTATION_LINEAGE_SCHEMA_VERSION,
            fixture_id: "fixture:m5:provider_sync".to_owned(),
            expected_lineage_root_id: "lineage:m5:provider_sync:0001".to_owned(),
            primary_surface_class: MutationSurfaceClass::SyncPacket,
            highest_risk_reversal_class: ReversalClass::Manual,
            total_file_count: 5,
            artifact_classes: vec![
                ArtifactClass::ProviderDraft,
                ArtifactClass::RepairReceipt,
                ArtifactClass::SyncManifest,
                ArtifactClass::WorkflowBundle,
            ],
            automation_influences: vec![
                AutomationInfluence::RepairTransaction,
                AutomationInfluence::SyncReconciliation,
                AutomationInfluence::WorkflowBundle,
            ],
            policy_influences: vec![
                PolicyInfluence::ApprovalBound,
                PolicyInfluence::ProviderPublishRules,
                PolicyInfluence::ReauthGate,
            ],
            history_inspector_row_id: "history:m5:provider_sync:0001".to_owned(),
            support_export_row_id: "support:m5:provider_sync:0001".to_owned(),
            consumer_ref: "crates/aureline-support/src/repair/mod.rs".to_owned(),
            notes: "Provider, sync, repair, and workflow follow-on remain on one lineage root while preserving compensate versus manual recovery.".to_owned(),
        },
        M5MutationLineageFixture {
            record_kind: M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: M5_MUTATION_LINEAGE_SCHEMA_VERSION,
            fixture_id: "fixture:m5:incident_capture".to_owned(),
            expected_lineage_root_id: "lineage:m5:incident_capture:0001".to_owned(),
            primary_surface_class: MutationSurfaceClass::ProfilerTrace,
            highest_risk_reversal_class: ReversalClass::AuditOnly,
            total_file_count: 2,
            artifact_classes: vec![ArtifactClass::IncidentPacket, ArtifactClass::TraceCapture],
            automation_influences: vec![AutomationInfluence::IncidentCapture],
            policy_influences: vec![PolicyInfluence::IncidentRetention],
            history_inspector_row_id: "history:m5:incident_capture:0001".to_owned(),
            support_export_row_id: "support:m5:incident_capture:0001".to_owned(),
            consumer_ref: "crates/aureline-support/src/incident_workspace/mod.rs".to_owned(),
            notes: "Profiler and incident follow-on lineage proves evidence capture and operator actions stay attributable without raw payload exports.".to_owned(),
        },
    ]
}

/// Validates the seeded packet or an on-disk copy of it.
pub fn validate_m5_mutation_lineage_packet(
    packet: &M5MutationLineagePacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != M5_MUTATION_LINEAGE_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            format!(
                "record_kind must be {M5_MUTATION_LINEAGE_PACKET_RECORD_KIND}, got {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != M5_MUTATION_LINEAGE_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            format!(
                "schema_version must be {}, got {}",
                M5_MUTATION_LINEAGE_SCHEMA_VERSION, packet.schema_version
            ),
        );
    }
    if packet.source_contract_refs.doc_ref != M5_MUTATION_LINEAGE_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted");
    }
    if packet.source_contract_refs.schema_ref != M5_MUTATION_LINEAGE_SCHEMA_REF {
        report.push("packet.schema_ref", "schema_ref drifted");
    }
    if packet.source_contract_refs.packet_ref != M5_MUTATION_LINEAGE_PACKET_REF {
        report.push("packet.packet_ref", "packet_ref drifted");
    }
    if packet.source_contract_refs.report_ref != M5_MUTATION_LINEAGE_REPORT_REF {
        report.push("packet.report_ref", "report_ref drifted");
    }
    if packet.source_contract_refs.fixture_manifest_ref != M5_MUTATION_LINEAGE_FIXTURE_MANIFEST_REF
    {
        report.push("packet.fixture_manifest_ref", "fixture_manifest_ref drifted");
    }

    let mut mutation_ids = BTreeSet::new();
    let mut group_ids = BTreeSet::new();
    let mut lineage_roots = BTreeSet::new();
    let mut surface_classes = BTreeSet::new();
    let mut reversal_classes = BTreeSet::new();
    let mut consumer_refs_ok = true;

    for entry in &packet.entries {
        if !mutation_ids.insert(entry.mutation_id.as_str()) {
            report.push(
                "entry.duplicate_mutation_id",
                format!("duplicate mutation id {}", entry.mutation_id),
            );
        }
        surface_classes.insert(entry.surface_class);
        reversal_classes.insert(entry.reversal_class);
        lineage_roots.insert(entry.lineage_root_id.as_str());
        if entry.scope_id.trim().is_empty() {
            report.push(
                "entry.scope_id",
                format!("entry {} must carry a non-empty scope_id", entry.mutation_id),
            );
        }
        if entry.affected_file_count == 0 {
            report.push(
                "entry.file_count",
                format!("entry {} must affect at least one file-like object", entry.mutation_id),
            );
        }
        if entry.checkpoint_refs.is_empty() {
            report.push(
                "entry.checkpoints",
                format!("entry {} must cite checkpoint lineage", entry.mutation_id),
            );
        }
        if !entry
            .checkpoint_refs
            .iter()
            .any(|checkpoint| checkpoint.checkpoint_role == CheckpointRole::CreatedBeforeMutation)
        {
            report.push(
                "entry.creation_checkpoint",
                format!(
                    "entry {} must cite a created_before_mutation checkpoint",
                    entry.mutation_id
                ),
            );
        }
        if entry.consumer_refs.is_empty() {
            report.push(
                "entry.consumer_refs",
                format!("entry {} must carry at least one consumer ref", entry.mutation_id),
            );
            consumer_refs_ok = false;
        }
    }

    for required in [
        MutationSurfaceClass::NotebookDocument,
        MutationSurfaceClass::NotebookOutput,
        MutationSurfaceClass::RequestWorkspace,
        MutationSurfaceClass::DataExportArtifact,
        MutationSurfaceClass::PreviewOutput,
        MutationSurfaceClass::SyncPacket,
        MutationSurfaceClass::RepairTransaction,
        MutationSurfaceClass::ProviderDraft,
        MutationSurfaceClass::WorkflowBundle,
        MutationSurfaceClass::ProfilerTrace,
        MutationSurfaceClass::AiEvidencePacket,
        MutationSurfaceClass::IncidentAction,
    ] {
        if !surface_classes.contains(&required) {
            report.push(
                "packet.required_surface_missing",
                format!("required surface {} missing", required.as_str()),
            );
        }
    }

    for required in [
        ReversalClass::Exact,
        ReversalClass::GroupedExact,
        ReversalClass::Compensate,
        ReversalClass::Regenerate,
        ReversalClass::Manual,
        ReversalClass::AuditOnly,
    ] {
        if !reversal_classes.contains(&required)
            && !packet.groups.iter().any(|group| group.reversal_class == required)
        {
            report.push(
                "packet.required_reversal_missing",
                format!("required reversal class {} missing", required.as_str()),
            );
        }
    }

    let entry_by_id: BTreeMap<_, _> = packet
        .entries
        .iter()
        .map(|entry| (entry.mutation_id.as_str(), entry))
        .collect();

    for group in &packet.groups {
        if !group_ids.insert(group.group_id.as_str()) {
            report.push(
                "group.duplicate_group_id",
                format!("duplicate group id {}", group.group_id),
            );
        }
        if group.member_mutation_ids.is_empty() {
            report.push(
                "group.members_empty",
                format!("group {} must cite at least one member mutation", group.group_id),
            );
            continue;
        }
        let mut total_file_count = 0;
        let mut artifact_classes = BTreeSet::new();
        let mut automation_influences = BTreeSet::new();
        let mut policy_influences = BTreeSet::new();
        let mut max_reversal = ReversalClass::Exact;
        for mutation_id in &group.member_mutation_ids {
            let Some(entry) = entry_by_id.get(mutation_id.as_str()) else {
                report.push(
                    "group.member_missing",
                    format!("group {} cites missing mutation {}", group.group_id, mutation_id),
                );
                continue;
            };
            if entry.group_id != group.group_id {
                report.push(
                    "group.member_group_mismatch",
                    format!(
                        "entry {} points to group {} but is listed under {}",
                        entry.mutation_id, entry.group_id, group.group_id
                    ),
                );
            }
            if entry.lineage_root_id != group.lineage_root_id {
                report.push(
                    "group.member_lineage_root_mismatch",
                    format!(
                        "entry {} points to lineage root {} but group {} uses {}",
                        entry.mutation_id, entry.lineage_root_id, group.group_id, group.lineage_root_id
                    ),
                );
            }
            total_file_count += entry.affected_file_count;
            artifact_classes.insert(entry.artifact_class);
            automation_influences.insert(entry.automation_influence);
            policy_influences.insert(entry.policy_influence);
            if entry.reversal_class.severity() > max_reversal.severity() {
                max_reversal = entry.reversal_class;
            }
        }
        if group.total_file_count != total_file_count {
            report.push(
                "group.total_file_count",
                format!(
                    "group {} expected total_file_count {} but members sum to {}",
                    group.group_id, group.total_file_count, total_file_count
                ),
            );
        }
        if group.reversal_class != max_reversal {
            report.push(
                "group.reversal_class",
                format!(
                    "group {} reversal {} must equal highest-severity member {}",
                    group.group_id,
                    group.reversal_class.as_str(),
                    max_reversal.as_str()
                ),
            );
        }
        if collect_vec(group.artifact_classes.iter().copied()) != collect_vec(artifact_classes) {
            report.push(
                "group.artifact_classes",
                format!("group {} artifact_classes drifted from member entries", group.group_id),
            );
        }
        if collect_vec(group.automation_influences.iter().copied())
            != collect_vec(automation_influences)
        {
            report.push(
                "group.automation_influences",
                format!(
                    "group {} automation_influences drifted from member entries",
                    group.group_id
                ),
            );
        }
        if collect_vec(group.policy_influences.iter().copied()) != collect_vec(policy_influences) {
            report.push(
                "group.policy_influences",
                format!(
                    "group {} policy_influences drifted from member entries",
                    group.group_id
                ),
            );
        }
        if group.checkpoint_refs.is_empty() {
            report.push(
                "group.checkpoints",
                format!("group {} must carry checkpoint lineage", group.group_id),
            );
        }
        if group.consumer_refs.is_empty() {
            report.push(
                "group.consumer_refs",
                format!("group {} must carry at least one consumer ref", group.group_id),
            );
            consumer_refs_ok = false;
        }
    }

    if consumer_refs_ok && packet
        .invariants
        .iter()
        .all(|invariant| invariant.trim().is_empty())
    {
        report.push("packet.invariants", "invariants must be non-empty");
    }

    validate_thread_rows(packet, &entry_by_id, &mut report);
    validate_required_thread_linkage(packet, &mut report);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one fixture against the packet.
pub fn validate_m5_mutation_lineage_fixture(
    packet: &M5MutationLineagePacket,
    fixture: &M5MutationLineageFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!(
                "fixture {} record_kind must be {}",
                fixture.fixture_id, M5_MUTATION_LINEAGE_FIXTURE_RECORD_KIND
            ),
        );
    }
    if fixture.schema_version != M5_MUTATION_LINEAGE_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!(
                "fixture {} schema_version must be {}",
                fixture.fixture_id, M5_MUTATION_LINEAGE_SCHEMA_VERSION
            ),
        );
    }
    let Some(history_row) = packet
        .history_inspector_rows
        .iter()
        .find(|row| row.lineage_root_id == fixture.expected_lineage_root_id)
    else {
        report.push(
            "fixture.lineage_root_missing",
            format!(
                "fixture {} points to missing lineage root {}",
                fixture.fixture_id, fixture.expected_lineage_root_id
            ),
        );
        return Err(report);
    };
    let Some(support_row) = packet
        .support_export_rows
        .iter()
        .find(|row| row.lineage_root_id == fixture.expected_lineage_root_id)
    else {
        report.push(
            "fixture.support_row_missing",
            format!(
                "fixture {} points to missing support row for {}",
                fixture.fixture_id, fixture.expected_lineage_root_id
            ),
        );
        return Err(report);
    };
    if history_row.primary_surface_class != fixture.primary_surface_class {
        report.push(
            "fixture.primary_surface_class",
            format!(
                "fixture {} expected primary surface {} but history row shows {}",
                fixture.fixture_id,
                fixture.primary_surface_class.as_str(),
                history_row.primary_surface_class.as_str()
            ),
        );
    }
    if history_row.highest_risk_reversal_class != fixture.highest_risk_reversal_class {
        report.push(
            "fixture.highest_risk_reversal_class",
            format!(
                "fixture {} expected highest reversal {} but history row shows {}",
                fixture.fixture_id,
                fixture.highest_risk_reversal_class.as_str(),
                history_row.highest_risk_reversal_class.as_str()
            ),
        );
    }
    if history_row.total_file_count != fixture.total_file_count {
        report.push(
            "fixture.total_file_count",
            format!(
                "fixture {} expected total_file_count {} but history row shows {}",
                fixture.fixture_id, fixture.total_file_count, history_row.total_file_count
            ),
        );
    }
    if collect_vec(history_row.artifact_classes.iter().copied())
        != collect_vec(fixture.artifact_classes.iter().copied())
    {
        report.push(
            "fixture.artifact_classes",
            format!("fixture {} artifact_classes drifted", fixture.fixture_id),
        );
    }
    if collect_vec(history_row.automation_influences.iter().copied())
        != collect_vec(fixture.automation_influences.iter().copied())
    {
        report.push(
            "fixture.automation_influences",
            format!("fixture {} automation_influences drifted", fixture.fixture_id),
        );
    }
    if collect_vec(history_row.policy_influences.iter().copied())
        != collect_vec(fixture.policy_influences.iter().copied())
    {
        report.push(
            "fixture.policy_influences",
            format!("fixture {} policy_influences drifted", fixture.fixture_id),
        );
    }
    if history_row.row_id != fixture.history_inspector_row_id {
        report.push(
            "fixture.history_inspector_row_id",
            format!(
                "fixture {} expected history row {} but saw {}",
                fixture.fixture_id, fixture.history_inspector_row_id, history_row.row_id
            ),
        );
    }
    if support_row.row_id != fixture.support_export_row_id {
        report.push(
            "fixture.support_export_row_id",
            format!(
                "fixture {} expected support row {} but saw {}",
                fixture.fixture_id, fixture.support_export_row_id, support_row.row_id
            ),
        );
    }
    let consumer_visible = packet
        .entries
        .iter()
        .any(|entry| entry.lineage_root_id == fixture.expected_lineage_root_id
            && entry.consumer_refs.iter().any(|reference| reference == &fixture.consumer_ref))
        || packet
            .groups
            .iter()
            .any(|group| group.lineage_root_id == fixture.expected_lineage_root_id
                && group.consumer_refs.iter().any(|reference| reference == &fixture.consumer_ref));
    if !consumer_visible {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} consumer_ref {} must be declared by an entry or group on the lineage root",
                fixture.fixture_id, fixture.consumer_ref
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_thread_rows(
    packet: &M5MutationLineagePacket,
    entry_by_id: &BTreeMap<&str, &MutationEntry>,
    report: &mut ValidationReport,
) {
    let group_by_id: BTreeMap<_, _> = packet
        .groups
        .iter()
        .map(|group| (group.group_id.as_str(), group))
        .collect();
    let mut thread_roots = BTreeSet::new();
    for group in &packet.groups {
        thread_roots.insert(group.lineage_root_id.as_str());
    }

    for row in &packet.history_inspector_rows {
        if !thread_roots.contains(row.lineage_root_id.as_str()) {
            report.push(
                "history.lineage_root_missing",
                format!(
                    "history row {} points to missing lineage root {}",
                    row.row_id, row.lineage_root_id
                ),
            );
            continue;
        }
        let thread_groups: Vec<_> = packet
            .groups
            .iter()
            .filter(|group| group.lineage_root_id == row.lineage_root_id)
            .collect();
        validate_thread_row(
            "history",
            row.lineage_root_id.as_str(),
            &row.group_ids,
            &row.mutation_ids,
            row.highest_risk_reversal_class,
            &row.reversal_classes,
            row.total_file_count,
            &row.artifact_classes,
            &row.automation_influences,
            &row.policy_influences,
            &row.checkpoint_ids,
            &thread_groups,
            entry_by_id,
            group_by_id.clone(),
            report,
        );
    }

    for row in &packet.support_export_rows {
        if !thread_roots.contains(row.lineage_root_id.as_str()) {
            report.push(
                "support.lineage_root_missing",
                format!(
                    "support row {} points to missing lineage root {}",
                    row.row_id, row.lineage_root_id
                ),
            );
            continue;
        }
        if !row.raw_payload_excluded
            || !row.raw_private_material_excluded
            || !row.ambient_authority_excluded
            || !row.single_lineage_thread_preserved
        {
            report.push(
                "support.safety_flags",
                format!("support row {} violated metadata-safe export invariants", row.row_id),
            );
        }
        let thread_groups: Vec<_> = packet
            .groups
            .iter()
            .filter(|group| group.lineage_root_id == row.lineage_root_id)
            .collect();
        validate_thread_row(
            "support",
            row.lineage_root_id.as_str(),
            &row.group_ids,
            &row.mutation_ids,
            row.highest_risk_reversal_class,
            &row.reversal_classes,
            row.total_file_count,
            &row.artifact_classes,
            &row.automation_influences,
            &row.policy_influences,
            &Vec::new(),
            &thread_groups,
            entry_by_id,
            group_by_id.clone(),
            report,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_thread_row(
    prefix: &'static str,
    lineage_root_id: &str,
    group_ids: &[String],
    mutation_ids: &[String],
    highest_risk_reversal_class: ReversalClass,
    reversal_classes: &[ReversalClass],
    total_file_count: u32,
    artifact_classes: &[ArtifactClass],
    automation_influences: &[AutomationInfluence],
    policy_influences: &[PolicyInfluence],
    checkpoint_ids: &[String],
    thread_groups: &[&MutationGroupRecord],
    entry_by_id: &BTreeMap<&str, &MutationEntry>,
    group_by_id: BTreeMap<&str, &MutationGroupRecord>,
    report: &mut ValidationReport,
) {
    let expected_group_ids: Vec<_> = collect_vec(thread_groups.iter().map(|group| group.group_id.clone()));
    if collect_vec(group_ids.iter().cloned()) != expected_group_ids {
        report.push(
            match prefix {
                "history" => "history.group_ids",
                _ => "support.group_ids",
            },
            format!("{} row for {} drifted from grouped lineage ids", prefix, lineage_root_id),
        );
    }

    let mut expected_mutation_ids = BTreeSet::new();
    let mut expected_artifact_classes = BTreeSet::new();
    let mut expected_automation_influences = BTreeSet::new();
    let mut expected_policy_influences = BTreeSet::new();
    let mut expected_checkpoint_ids = BTreeSet::new();
    let mut expected_total_file_count = 0;
    let mut max_reversal = ReversalClass::Exact;
    let mut expected_reversal_classes = BTreeSet::new();

    for group in thread_groups {
        if !group_by_id.contains_key(group.group_id.as_str()) {
            report.push(
                "thread.group_id_missing",
                format!("thread {} references unknown group {}", lineage_root_id, group.group_id),
            );
        }
        for checkpoint in &group.checkpoint_refs {
            expected_checkpoint_ids.insert(checkpoint.checkpoint_id.clone());
        }
        for mutation_id in &group.member_mutation_ids {
            expected_mutation_ids.insert(mutation_id.clone());
            let entry = entry_by_id
                .get(mutation_id.as_str())
                .expect("group validation already proved entries exist");
            expected_total_file_count += entry.affected_file_count;
            expected_artifact_classes.insert(entry.artifact_class);
            expected_automation_influences.insert(entry.automation_influence);
            expected_policy_influences.insert(entry.policy_influence);
            for checkpoint in &entry.checkpoint_refs {
                expected_checkpoint_ids.insert(checkpoint.checkpoint_id.clone());
            }
            expected_reversal_classes.insert(entry.reversal_class);
            if entry.reversal_class.severity() > max_reversal.severity() {
                max_reversal = entry.reversal_class;
            }
        }
    }

    if collect_vec(mutation_ids.iter().cloned()) != collect_vec(expected_mutation_ids) {
        report.push(
            match prefix {
                "history" => "history.mutation_ids",
                _ => "support.mutation_ids",
            },
            format!("{} row for {} drifted from mutation ids", prefix, lineage_root_id),
        );
    }
    if collect_vec(artifact_classes.iter().copied()) != collect_vec(expected_artifact_classes) {
        report.push(
            match prefix {
                "history" => "history.artifact_classes",
                _ => "support.artifact_classes",
            },
            format!("{} row for {} drifted from artifact classes", prefix, lineage_root_id),
        );
    }
    if collect_vec(automation_influences.iter().copied())
        != collect_vec(expected_automation_influences)
    {
        report.push(
            match prefix {
                "history" => "history.automation_influences",
                _ => "support.automation_influences",
            },
            format!(
                "{} row for {} drifted from automation influences",
                prefix, lineage_root_id
            ),
        );
    }
    if collect_vec(policy_influences.iter().copied()) != collect_vec(expected_policy_influences) {
        report.push(
            match prefix {
                "history" => "history.policy_influences",
                _ => "support.policy_influences",
            },
            format!(
                "{} row for {} drifted from policy influences",
                prefix, lineage_root_id
            ),
        );
    }
    if total_file_count != expected_total_file_count {
        report.push(
            match prefix {
                "history" => "history.total_file_count",
                _ => "support.total_file_count",
            },
            format!(
                "{} row for {} expected total_file_count {} but computed {}",
                prefix, lineage_root_id, total_file_count, expected_total_file_count
            ),
        );
    }
    if highest_risk_reversal_class != max_reversal {
        report.push(
            match prefix {
                "history" => "history.highest_risk_reversal_class",
                _ => "support.highest_risk_reversal_class",
            },
            format!(
                "{} row for {} expected highest reversal {} but computed {}",
                prefix,
                lineage_root_id,
                highest_risk_reversal_class.as_str(),
                max_reversal.as_str()
            ),
        );
    }
    if collect_vec(reversal_classes.iter().copied()) != collect_vec(expected_reversal_classes) {
        report.push(
            match prefix {
                "history" => "history.reversal_classes",
                _ => "support.reversal_classes",
            },
            format!("{} row for {} drifted from reversal classes", prefix, lineage_root_id),
        );
    }
    if prefix == "history"
        && collect_vec(checkpoint_ids.iter().cloned()) != collect_vec(expected_checkpoint_ids)
    {
        report.push(
            "history.checkpoint_ids",
            format!("history row for {} drifted from checkpoint ids", lineage_root_id),
        );
    }
}

fn validate_required_thread_linkage(packet: &M5MutationLineagePacket, report: &mut ValidationReport) {
    let notebook_thread_entries: BTreeSet<_> = packet
        .entries
        .iter()
        .filter(|entry| entry.lineage_root_id == "lineage:m5:notebook_execution:0001")
        .map(|entry| entry.surface_class)
        .collect();
    for required in [
        MutationSurfaceClass::NotebookDocument,
        MutationSurfaceClass::NotebookOutput,
        MutationSurfaceClass::AiEvidencePacket,
    ] {
        if !notebook_thread_entries.contains(&required) {
            report.push(
                "thread.notebook_linkage",
                format!(
                    "notebook lineage thread must include surface {}",
                    required.as_str()
                ),
            );
        }
    }

    let provider_thread_entries: BTreeSet<_> = packet
        .entries
        .iter()
        .filter(|entry| entry.lineage_root_id == "lineage:m5:provider_sync:0001")
        .map(|entry| entry.surface_class)
        .collect();
    for required in [
        MutationSurfaceClass::SyncPacket,
        MutationSurfaceClass::ProviderDraft,
        MutationSurfaceClass::RepairTransaction,
        MutationSurfaceClass::WorkflowBundle,
    ] {
        if !provider_thread_entries.contains(&required) {
            report.push(
                "thread.provider_sync_linkage",
                format!(
                    "provider/sync lineage thread must include surface {}",
                    required.as_str()
                ),
            );
        }
    }

    let incident_thread_entries: BTreeSet<_> = packet
        .entries
        .iter()
        .filter(|entry| entry.lineage_root_id == "lineage:m5:incident_capture:0001")
        .map(|entry| entry.surface_class)
        .collect();
    for required in [
        MutationSurfaceClass::ProfilerTrace,
        MutationSurfaceClass::IncidentAction,
    ] {
        if !incident_thread_entries.contains(&required) {
            report.push(
                "thread.incident_linkage",
                format!(
                    "incident lineage thread must include surface {}",
                    required.as_str()
                ),
            );
        }
    }
}

fn checkpoint(
    checkpoint_id: &str,
    checkpoint_class: CheckpointClass,
    checkpoint_role: CheckpointRole,
) -> CheckpointRef {
    CheckpointRef {
        checkpoint_id: checkpoint_id.to_owned(),
        checkpoint_class,
        checkpoint_role,
    }
}

fn mutation_group(
    group_id: &str,
    lineage_root_id: &str,
    phase_class: GroupPhaseClass,
    title: &str,
    primary_surface_class: MutationSurfaceClass,
    member_mutation_ids: Vec<&str>,
    entries: &[MutationEntry],
    consumer_refs: Vec<String>,
    notes: &str,
) -> MutationGroupRecord {
    let members: Vec<_> = member_mutation_ids
        .iter()
        .map(|mutation_id| {
            entries
                .iter()
                .find(|entry| entry.mutation_id == *mutation_id)
                .expect("seeded group members must exist")
        })
        .collect();
    let total_file_count = members.iter().map(|entry| entry.affected_file_count).sum();
    let artifact_classes = collect_vec(members.iter().map(|entry| entry.artifact_class));
    let automation_influences =
        collect_vec(members.iter().map(|entry| entry.automation_influence));
    let policy_influences = collect_vec(members.iter().map(|entry| entry.policy_influence));
    let reversal_class = members
        .iter()
        .map(|entry| entry.reversal_class)
        .max_by_key(|reversal| reversal.severity())
        .expect("seeded group must have members");
    let mut checkpoint_refs = BTreeMap::new();
    for member in &members {
        for checkpoint in &member.checkpoint_refs {
            checkpoint_refs
                .entry(checkpoint.checkpoint_id.clone())
                .or_insert_with(|| checkpoint.clone());
        }
    }
    MutationGroupRecord {
        group_id: group_id.to_owned(),
        lineage_root_id: lineage_root_id.to_owned(),
        phase_class,
        title: title.to_owned(),
        primary_surface_class,
        member_mutation_ids: member_mutation_ids.into_iter().map(str::to_owned).collect(),
        reversal_class,
        total_file_count,
        artifact_classes,
        automation_influences,
        policy_influences,
        checkpoint_refs: checkpoint_refs.into_values().collect(),
        consumer_refs,
        notes: notes.to_owned(),
    }
}

fn inspector_row(
    row_id: &str,
    lineage_root_id: &str,
    title: &str,
    primary_surface_class: MutationSurfaceClass,
    groups: &[MutationGroupRecord],
    entries: &[MutationEntry],
    reopen_action_label: &str,
    notes: &str,
) -> HistoryInspectorRow {
    let thread_groups: Vec<_> = groups
        .iter()
        .filter(|group| group.lineage_root_id == lineage_root_id)
        .collect();
    let group_ids = thread_groups.iter().map(|group| group.group_id.clone()).collect();
    let mutation_ids = thread_groups
        .iter()
        .flat_map(|group| group.member_mutation_ids.iter().cloned())
        .collect::<Vec<_>>();
    let thread_entries: Vec<_> = entries
        .iter()
        .filter(|entry| entry.lineage_root_id == lineage_root_id)
        .collect();
    let highest_risk_reversal_class = thread_entries
        .iter()
        .map(|entry| entry.reversal_class)
        .max_by_key(|reversal| reversal.severity())
        .expect("thread must have entries");
    let reversal_classes = collect_vec(thread_entries.iter().map(|entry| entry.reversal_class));
    let total_file_count = thread_entries
        .iter()
        .map(|entry| entry.affected_file_count)
        .sum();
    let artifact_classes = collect_vec(thread_entries.iter().map(|entry| entry.artifact_class));
    let automation_influences =
        collect_vec(thread_entries.iter().map(|entry| entry.automation_influence));
    let policy_influences = collect_vec(thread_entries.iter().map(|entry| entry.policy_influence));
    let checkpoint_ids = collect_vec(thread_entries.iter().flat_map(|entry| {
        entry
            .checkpoint_refs
            .iter()
            .map(|checkpoint| checkpoint.checkpoint_id.clone())
    }));
    HistoryInspectorRow {
        row_id: row_id.to_owned(),
        lineage_root_id: lineage_root_id.to_owned(),
        title: title.to_owned(),
        primary_surface_class,
        group_ids,
        mutation_ids,
        highest_risk_reversal_class,
        reversal_classes,
        total_file_count,
        artifact_classes,
        automation_influences,
        policy_influences,
        checkpoint_ids,
        reopen_action_label: reopen_action_label.to_owned(),
        notes: notes.to_owned(),
    }
}

fn support_export_row(
    row_id: &str,
    lineage_root_id: &str,
    groups: &[MutationGroupRecord],
    entries: &[MutationEntry],
    notes: &str,
) -> SupportExportManifestRow {
    let history_row = inspector_row(
        row_id,
        lineage_root_id,
        row_id,
        entries
            .iter()
            .find(|entry| entry.lineage_root_id == lineage_root_id)
            .expect("lineage root must exist")
            .surface_class,
        groups,
        entries,
        "n/a",
        notes,
    );
    SupportExportManifestRow {
        row_id: row_id.to_owned(),
        lineage_root_id: lineage_root_id.to_owned(),
        group_ids: history_row.group_ids,
        mutation_ids: history_row.mutation_ids,
        highest_risk_reversal_class: history_row.highest_risk_reversal_class,
        reversal_classes: history_row.reversal_classes,
        total_file_count: history_row.total_file_count,
        artifact_classes: history_row.artifact_classes,
        automation_influences: history_row.automation_influences,
        policy_influences: history_row.policy_influences,
        raw_payload_excluded: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
        single_lineage_thread_preserved: true,
        notes: notes.to_owned(),
    }
}

fn collect_vec<T>(items: impl IntoIterator<Item = T>) -> Vec<T>
where
    T: Ord,
{
    let mut items: Vec<_> = items.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
    items.sort();
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_m5_mutation_lineage_packet();
        validate_m5_mutation_lineage_packet(&packet)
            .expect("seeded packet must satisfy the frozen contract");
    }

    #[test]
    fn seeded_fixtures_validate() {
        let packet = seeded_m5_mutation_lineage_packet();
        let fixtures = seeded_m5_mutation_lineage_fixtures();
        assert_eq!(fixtures.len(), 5);
        for fixture in &fixtures {
            validate_m5_mutation_lineage_fixture(&packet, fixture)
                .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        }
    }

    #[test]
    fn required_reversal_classes_are_visible() {
        let packet = seeded_m5_mutation_lineage_packet();
        let classes = collect_vec(
            packet
                .entries
                .iter()
                .map(|entry| entry.reversal_class)
                .chain(packet.groups.iter().map(|group| group.reversal_class)),
        );
        for required in [
            ReversalClass::Exact,
            ReversalClass::GroupedExact,
            ReversalClass::Compensate,
            ReversalClass::Regenerate,
            ReversalClass::Manual,
            ReversalClass::AuditOnly,
        ] {
            assert!(
                classes.contains(&required),
                "required reversal class {} missing",
                required.as_str()
            );
        }
    }

    #[test]
    fn thread_level_support_rows_preserve_metadata_safe_flags() {
        let packet = seeded_m5_mutation_lineage_packet();
        for row in &packet.support_export_rows {
            assert!(row.raw_payload_excluded);
            assert!(row.raw_private_material_excluded);
            assert!(row.ambient_authority_excluded);
            assert!(row.single_lineage_thread_preserved);
        }
    }
}
