use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::descriptor::{CommandId, CommandRevisionRef, OpaqueId, PolicyContext, RepairHookRef};
use crate::enablement::{DisabledReasonCode, EnablementDecisionClass};

static SESSION_SEQ: AtomicUsize = AtomicUsize::new(1);
static ATTEMPT_SEQ: AtomicUsize = AtomicUsize::new(1);
static RESULT_SEQ: AtomicUsize = AtomicUsize::new(1);
static PREVIEW_SEQ: AtomicUsize = AtomicUsize::new(1);
static APPROVAL_SEQ: AtomicUsize = AtomicUsize::new(1);
static BASIS_SEQ: AtomicUsize = AtomicUsize::new(1);

fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn now_rfc3339() -> String {
    now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

fn timestamp_fragment(now: OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}-{:02}-{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

fn next_seq(counter: &AtomicUsize) -> String {
    let next = counter.fetch_add(1, Ordering::Relaxed);
    format!("{next:02}")
}

pub fn mint_invocation_session_id(canonical_verb: &str) -> OpaqueId {
    let now = now_utc();
    format!(
        "inv:{canonical_verb}:{}:{}",
        timestamp_fragment(now),
        next_seq(&SESSION_SEQ)
    )
}

pub fn mint_invocation_attempt_id(canonical_verb: &str) -> OpaqueId {
    let now = now_utc();
    format!(
        "inv-attempt:{canonical_verb}:{}:{}",
        timestamp_fragment(now),
        next_seq(&ATTEMPT_SEQ)
    )
}

pub fn mint_result_packet_id(canonical_verb: &str) -> OpaqueId {
    let now = now_utc();
    format!(
        "result:{canonical_verb}:{}:{}",
        timestamp_fragment(now),
        next_seq(&RESULT_SEQ)
    )
}

pub fn mint_preview_record_ref(canonical_verb: &str) -> OpaqueId {
    let now = now_utc();
    format!(
        "preview:{canonical_verb}:{}:{}",
        timestamp_fragment(now),
        next_seq(&PREVIEW_SEQ)
    )
}

pub fn mint_approval_ticket_ref(canonical_verb: &str) -> OpaqueId {
    let now = now_utc();
    format!(
        "approval-ticket:{canonical_verb}:{}:{}",
        timestamp_fragment(now),
        next_seq(&APPROVAL_SEQ)
    )
}

pub fn mint_basis_snapshot_ref(canonical_verb: &str) -> OpaqueId {
    let now = now_utc();
    format!(
        "basis:{canonical_verb}:{}:{}",
        timestamp_fragment(now),
        next_seq(&BASIS_SEQ)
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnablementDecisionBlock {
    pub decision_class: EnablementDecisionClass,
    pub disabled_reason_code: Option<DisabledReasonCode>,
    pub repair_hook_ref: Option<RepairHookRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewPostureBlock {
    pub preview_class_declared: String,
    pub preview_shown: bool,
    pub preview_record_ref: Option<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalPostureBlock {
    pub approval_posture_class_declared: String,
    pub approval_state: String,
    pub approval_ticket_ref: Option<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArgumentProvenanceEntry {
    pub argument_name: String,
    pub provenance: String,
    pub resolved_value_ref: Option<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationContextSnapshot {
    pub focused_entity_ref: Option<OpaqueId>,
    pub selection_ref: Option<OpaqueId>,
    pub workspace_trust_state: String,
    pub execution_context_id: Option<OpaqueId>,
    pub scope_filter_class_ref: Option<OpaqueId>,
    pub basis_snapshot_ref: OpaqueId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationOutcomeBlock {
    pub outcome_class: String,
    pub disabled_reason_code: Option<DisabledReasonCode>,
    pub warnings_summary_refs: Vec<OpaqueId>,
    pub partially_applied_artifact_refs: Vec<OpaqueId>,
    pub unapplied_artifact_refs: Vec<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationCreatedArtifactRefEntry {
    pub result_contract_class: String,
    pub artifact_ref: OpaqueId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRefEntry {
    pub evidence_ref_class: String,
    pub evidence_id: OpaqueId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvocationSessionPacketRecord {
    pub record_kind: String,
    pub command_descriptor_schema_version: u32,

    pub invocation_session_id: OpaqueId,
    pub command_id: CommandId,
    pub command_revision_ref: CommandRevisionRef,

    pub issuing_surface: String,
    pub authority_class: String,

    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    pub context_snapshot: InvocationContextSnapshot,

    pub enablement_decision: EnablementDecisionBlock,
    pub preview_posture: PreviewPostureBlock,
    pub approval_posture: ApprovalPostureBlock,

    pub execution_intent: String,

    pub outcome: InvocationOutcomeBlock,

    pub created_artifact_refs: Vec<InvocationCreatedArtifactRefEntry>,
    pub evidence_refs: Vec<EvidenceRefEntry>,

    pub policy_context: PolicyContext,
    pub redaction_class: String,
    pub minted_at: String,
}

impl InvocationSessionPacketRecord {
    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasUsedBlock {
    pub alias_kind: String,
    pub alias_id: Option<OpaqueId>,
    pub alias_state: String,
    pub resolves_to_canonical_command_id: CommandId,
    pub migration_trace_ref: Option<OpaqueId>,
    pub support_window_ref: Option<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextRefsBlock {
    pub focused_entity_ref: Option<OpaqueId>,
    pub selection_ref: Option<OpaqueId>,
    pub workspace_ref: Option<OpaqueId>,
    pub workspace_trust_state: String,
    pub execution_context_id: Option<OpaqueId>,
    pub scope_filter_class_ref: Option<OpaqueId>,
    pub basis_snapshot_ref: OpaqueId,
    pub context_object_refs: Vec<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRefEntry {
    pub result_contract_class: String,
    pub artifact_ref: OpaqueId,
    pub artifact_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRefEntry {
    pub notification_ref: OpaqueId,
    pub delivery_posture: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRefEntry {
    pub activity_ref: OpaqueId,
    pub activity_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackHandleRefBlock {
    pub rollback_handle_posture: String,
    pub rollback_handle_id: Option<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointRefEntry {
    pub checkpoint_class: String,
    pub checkpoint_ref: Option<OpaqueId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportPostureBlock {
    pub export_posture_class: String,
    pub redaction_class: String,
    pub export_review_ref: Option<OpaqueId>,
    pub portable_profile_allowed: bool,
    pub support_bundle_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoBypassGuards {
    pub trust_revalidation_required: bool,
    pub policy_revalidation_required: bool,
    pub permission_prompt_revalidation_required: bool,
    pub preview_path_preserved: bool,
    pub approval_path_preserved: bool,
    pub credential_broker_revalidation_required: bool,
    pub execution_context_revalidation_required: bool,
    pub freshness_floor_revalidation_required: bool,
    pub capability_class_may_not_widen: bool,
    pub result_schema_may_not_be_replaced: bool,
}

impl NoBypassGuards {
    pub const fn strict() -> Self {
        Self {
            trust_revalidation_required: true,
            policy_revalidation_required: true,
            permission_prompt_revalidation_required: true,
            preview_path_preserved: true,
            approval_path_preserved: true,
            credential_broker_revalidation_required: true,
            execution_context_revalidation_required: true,
            freshness_floor_revalidation_required: true,
            capability_class_may_not_widen: true,
            result_schema_may_not_be_replaced: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultBodyBlock {
    pub outcome_code: String,
    pub warning_codes: Vec<String>,
    pub error_codes: Vec<String>,
    pub created_artifact_refs: Vec<ArtifactRefEntry>,
    pub notification_refs: Vec<NotificationRefEntry>,
    pub activity_refs: Vec<ActivityRefEntry>,
    pub rollback_handle_ref: RollbackHandleRefBlock,
    pub checkpoint_refs: Vec<CheckpointRefEntry>,
    pub evidence_refs: Vec<EvidenceRefEntry>,
    pub export_posture: ExportPostureBlock,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvocationMinimumBlock {
    pub invocation_session_id: OpaqueId,
    pub invocation_attempt_id: OpaqueId,
    pub issuing_surface: String,
    pub authority_class: String,
    pub canonical_command_id: CommandId,
    pub command_revision_ref: CommandRevisionRef,
    pub canonical_verb: String,
    pub alias_used: AliasUsedBlock,
    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    pub context_refs: ContextRefsBlock,
    pub enablement_decision: EnablementDecisionBlock,
    pub preview_posture: PreviewPostureBlock,
    pub approval_posture: ApprovalPostureBlock,
    pub execution_intent: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandResultPacketRecord {
    pub record_kind: String,
    pub result_packet_schema_version: u32,
    pub result_packet_id: OpaqueId,

    pub invocation: InvocationMinimumBlock,
    pub result: ResultBodyBlock,

    pub parity_expectation_ref: OpaqueId,
    pub no_bypass_guards: NoBypassGuards,
    pub policy_context: PolicyContext,
    pub redaction_class: String,
    pub minted_at: String,
}

impl CommandResultPacketRecord {
    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandInvocationSession {
    pub invocation_session_id: OpaqueId,
    pub canonical_command_id: CommandId,
    pub command_revision_ref: CommandRevisionRef,
    pub canonical_verb: String,

    pub issuing_surface: String,
    pub authority_class: String,
    pub alias_used: AliasUsedBlock,

    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    pub context_snapshot: InvocationContextSnapshot,
    pub context_refs: ContextRefsBlock,

    pub enablement_decision: EnablementDecisionBlock,
    pub preview_posture: PreviewPostureBlock,
    pub approval_posture: ApprovalPostureBlock,
    pub execution_intent: String,

    pub policy_context: PolicyContext,
    pub redaction_class: String,
}

impl CommandInvocationSession {
    pub fn mint_attempt_id(&self) -> OpaqueId {
        mint_invocation_attempt_id(&self.canonical_verb)
    }

    pub fn mint_result_packet_id(&self) -> OpaqueId {
        mint_result_packet_id(&self.canonical_verb)
    }

    pub fn invocation_session_packet(
        &self,
        outcome: InvocationOutcomeBlock,
        created_artifact_refs: Vec<InvocationCreatedArtifactRefEntry>,
        evidence_refs: Vec<EvidenceRefEntry>,
    ) -> InvocationSessionPacketRecord {
        InvocationSessionPacketRecord {
            record_kind: "invocation_session_packet_record".to_string(),
            command_descriptor_schema_version: 1,
            invocation_session_id: self.invocation_session_id.clone(),
            command_id: self.canonical_command_id.clone(),
            command_revision_ref: self.command_revision_ref.clone(),
            issuing_surface: self.issuing_surface.clone(),
            authority_class: self.authority_class.clone(),
            argument_provenance_map: self.argument_provenance_map.clone(),
            context_snapshot: self.context_snapshot.clone(),
            enablement_decision: self.enablement_decision.clone(),
            preview_posture: self.preview_posture.clone(),
            approval_posture: self.approval_posture.clone(),
            execution_intent: self.execution_intent.clone(),
            outcome,
            created_artifact_refs,
            evidence_refs,
            policy_context: self.policy_context.clone(),
            redaction_class: self.redaction_class.clone(),
            minted_at: now_rfc3339(),
        }
    }

    pub fn command_result_packet(
        &self,
        invocation_attempt_id: OpaqueId,
        result_packet_id: OpaqueId,
        result: ResultBodyBlock,
        parity_expectation_ref: OpaqueId,
        no_bypass_guards: NoBypassGuards,
    ) -> CommandResultPacketRecord {
        CommandResultPacketRecord {
            record_kind: "command_result_packet_record".to_string(),
            result_packet_schema_version: 1,
            result_packet_id,
            invocation: InvocationMinimumBlock {
                invocation_session_id: self.invocation_session_id.clone(),
                invocation_attempt_id,
                issuing_surface: self.issuing_surface.clone(),
                authority_class: self.authority_class.clone(),
                canonical_command_id: self.canonical_command_id.clone(),
                command_revision_ref: self.command_revision_ref.clone(),
                canonical_verb: self.canonical_verb.clone(),
                alias_used: self.alias_used.clone(),
                argument_provenance_map: self.argument_provenance_map.clone(),
                context_refs: self.context_refs.clone(),
                enablement_decision: self.enablement_decision.clone(),
                preview_posture: self.preview_posture.clone(),
                approval_posture: self.approval_posture.clone(),
                execution_intent: self.execution_intent.clone(),
            },
            result,
            parity_expectation_ref,
            no_bypass_guards,
            policy_context: self.policy_context.clone(),
            redaction_class: self.redaction_class.clone(),
            minted_at: now_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_fixture(path: &str) -> String {
        let base = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
        std::fs::read_to_string(format!("{base}{path}")).expect("fixture must read")
    }

    #[test]
    fn parses_invocation_session_fixtures() {
        let dir = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/commands/command_descriptor_examples"
        );
        for entry in std::fs::read_dir(dir).expect("fixture dir must exist") {
            let entry = entry.expect("dir entry must read");
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !name.starts_with("invocation_session_") || !name.ends_with(".json") {
                continue;
            }
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let record: InvocationSessionPacketRecord =
                serde_json::from_str(&payload).expect("invocation session fixture must parse");
            assert_eq!(record.record_kind, "invocation_session_packet_record");
            assert_eq!(record.command_descriptor_schema_version, 1);
            assert!(!record.invocation_session_id.trim().is_empty());
            assert!(!record.command_id.trim().is_empty());
        }
    }

    #[test]
    fn parses_result_packet_fixture_json() {
        let payload = read_fixture(
            "fixtures/commands/invocation_packets/palette_import_profile_cancelled.result.json",
        );
        let record: CommandResultPacketRecord =
            serde_json::from_str(&payload).expect("result packet fixture must parse");
        assert_eq!(record.record_kind, "command_result_packet_record");
        assert_eq!(record.result_packet_schema_version, 1);
        assert!(!record.result_packet_id.trim().is_empty());
        assert!(!record.invocation.invocation_session_id.trim().is_empty());
    }

    #[test]
    fn mints_reasonably_shaped_ids() {
        let session_id = mint_invocation_session_id("workspace.open_folder");
        let attempt_id = mint_invocation_attempt_id("workspace.open_folder");
        let result_id = mint_result_packet_id("workspace.open_folder");
        let preview_ref = mint_preview_record_ref("workspace.open_folder");
        let approval_ref = mint_approval_ticket_ref("workspace.open_folder");
        let basis_ref = mint_basis_snapshot_ref("workspace.open_folder");
        assert!(session_id.starts_with("inv:workspace.open_folder:"));
        assert!(attempt_id.starts_with("inv-attempt:workspace.open_folder:"));
        assert!(result_id.starts_with("result:workspace.open_folder:"));
        assert!(preview_ref.starts_with("preview:workspace.open_folder:"));
        assert!(approval_ref.starts_with("approval-ticket:workspace.open_folder:"));
        assert!(basis_ref.starts_with("basis:workspace.open_folder:"));
    }

    #[test]
    fn pretty_json_helpers_work() {
        let payload = read_fixture(
            "fixtures/commands/command_descriptor_examples/invocation_session_open_folder_succeeded.json",
        );
        let record: InvocationSessionPacketRecord =
            serde_json::from_str(&payload).expect("fixture must parse");
        let _ = record.to_pretty_json().expect("pretty json must serialize");
    }
}
