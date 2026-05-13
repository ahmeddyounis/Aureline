//! Preview/apply/revert lifecycle wedge for one destructive core path
//! (multi-target bulk replace).
//!
//! The shell hosts a bounded prototype wedge that proves the
//! "worktree-is-sacred" rule end-to-end on one destructive surface. A user
//! cannot mutate the target world without going through:
//!
//! - **Propose** — declare scope, basis digests, and the honest
//!   [`RevertClass`] the recovery surface will advertise after apply.
//! - **Preview** — compute a per-target diff against the captured basis and
//!   gate admission to apply on whether the basis has drifted.
//! - **Apply** — mint a named undo group plus a content-addressed
//!   checkpoint, write through to the target world, and record one
//!   [`aureline_history::MutationJournalEntryRecord`] per target.
//! - **Validate** — confirm the post-apply world matches what the preview
//!   advertised, byte-for-byte, so the wedge cannot silently widen scope.
//! - **Revert** — restore from the checkpoint and record a reversal
//!   mutation-journal entry, downgrading the realized [`RevertClass`] when
//!   the apply did not produce a checkpoint.
//!
//! The wedge intentionally does not generalize into a platform-wide
//! mutation framework. It reuses the shared
//! [`aureline_history`] vocabulary (mutation journal, local history
//! checkpoints) without forking, and projects one
//! [`MutationPacket`] record reviewers can quote verbatim.
//!
//! Lineage identifiers (`proposal_id`, `preview_id`, `apply_id`,
//! `mutation_group_id`, `local_history_group_id`, `validation_id`,
//! `revert_id`) survive every phase transition. Apply is **rejected**
//! when the basis has drifted since preview; the surface invalidates the
//! preview and reopens review rather than silently widening scope.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use aureline_history::checkpoints::{
    AliasSetRecord, CanonicalFilesystemObjectRecord, CaptureDescriptor, CaptureMode,
    CaptureOmissionReasonClass, FilesystemIdentityRecord, IdentityTokenRecord,
    LocalHistoryEntryRecord, LocalHistoryGroupKind, LocalHistoryGroupRecord,
    LocalHistoryGroupResolution, LocalHistoryStore, LogicalDocumentIdentity,
    LogicalWorkspaceIdentityRecord, MutationJournalLink, MutationJournalLinkActorClass,
    MutationJournalLinkKind, MutationJournalLinkReversalClass, PresentationPathRecord,
    RetentionScopeClass, SnapshotClass,
};
use aureline_history::mutation_journal::{MutationGroupKind, MutationGroupResolution};
use aureline_history::{
    body_object_id, ActorClass, ActorRef, DurableVsDisposable, HistoryError, IdSource,
    LocalHistoryAlphaPacket, LocalHistoryConsumerSurface, MutationGroupRecord,
    MutationJournalEntryRecord, MutationJournalStore, RedactionClass, ReversalClass,
    ReviewApplyLineageInput, ScopeClass, ScopeRef, SideEffectSummary, SourceClass, TargetKind,
    TargetRef,
};
use aureline_runtime::{
    ExecutionContext, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass,
};

/// Stable record-kind tag carried in serialized [`MutationPacket`] payloads.
pub const MUTATION_PACKET_RECORD_KIND: &str = "preview_apply_revert_mutation_packet";

/// Schema version for the [`MutationPacket`] payload shape.
pub const MUTATION_PACKET_SCHEMA_VERSION: u32 = 1;

/// Canonical command id used by the destructive core wedge.
pub const DESTRUCTIVE_CORE_COMMAND_ID: &str = "workspace.bulk_replace_in_files.apply";

/// Stable lifecycle phases for [`MutationPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewApplyRevertPhase {
    Propose,
    Preview,
    Apply,
    Validate,
    Revert,
    Keep,
}

impl PreviewApplyRevertPhase {
    /// Stable string token quoted by support exports and fixture replays.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Propose => "propose",
            Self::Preview => "preview",
            Self::Apply => "apply",
            Self::Validate => "validate",
            Self::Revert => "revert",
            Self::Keep => "keep",
        }
    }
}

/// Honest revert class advertised by the wedge, mirroring the frozen
/// `revert_class` vocabulary in
/// `docs/ux/preview_apply_revert_contract.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevertClass {
    ExactUndo,
    CompensatingAction,
    RegenerateFromSource,
    RestoreFromCheckpoint,
    EvidenceOnlyNoRerun,
    NoRecoveryAvailable,
}

impl RevertClass {
    /// Stable string token quoted on packets and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::CompensatingAction => "compensating_action",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::EvidenceOnlyNoRerun => "evidence_only_no_rerun",
            Self::NoRecoveryAvailable => "no_recovery_available",
        }
    }
}

/// Honest consequence class advertised at proposal time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceClass {
    /// Reversible by ordinary undo without a checkpoint.
    Reversible,
    /// Destructive but reversible after a minted checkpoint is bound.
    DestructiveReversibleWithCheckpoint,
    /// Irreversible high blast radius; checkpoint-restore is best-effort
    /// evidence only.
    IrreversibleHighBlast,
}

impl ConsequenceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "reversible",
            Self::DestructiveReversibleWithCheckpoint => "destructive_reversible_with_checkpoint",
            Self::IrreversibleHighBlast => "irreversible_high_blast",
        }
    }
}

/// Basis-drift state across the propose/preview/apply boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BasisDriftState {
    NoDrift,
    DriftDetected {
        previous_basis: String,
        current_basis: String,
    },
    BasisMissing,
}

impl BasisDriftState {
    /// True when the basis has not changed.
    pub fn is_clean(&self) -> bool {
        matches!(self, BasisDriftState::NoDrift)
    }
}

/// Per-row blocked reason rendered on a [`DiffPreviewRow`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffRowBlockReason {
    BasisDrifted,
    BasisMissing,
    TargetMissing,
    NoMatchesFound,
}

impl DiffRowBlockReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BasisDrifted => "basis_drifted",
            Self::BasisMissing => "basis_missing",
            Self::TargetMissing => "target_missing",
            Self::NoMatchesFound => "no_matches_found",
        }
    }
}

/// Admission decision for the preview-to-apply boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ApplyAdmissibility {
    Admitted,
    BlockedByBasisDrift { drifted_target_count: u32 },
    BlockedByTargetMissing { missing_target_count: u32 },
    BlockedByNoMatches,
}

impl ApplyAdmissibility {
    /// Stable string token.
    pub fn as_token(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::BlockedByBasisDrift { .. } => "blocked_by_basis_drift",
            Self::BlockedByTargetMissing { .. } => "blocked_by_target_missing",
            Self::BlockedByNoMatches => "blocked_by_no_matches",
        }
    }

    /// True when apply is admitted.
    pub fn is_admitted(&self) -> bool {
        matches!(self, ApplyAdmissibility::Admitted)
    }
}

/// One declared target spec at proposal time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetSpec {
    /// Stable logical reference (e.g. workspace-relative path) used as the
    /// addressable id across every phase.
    pub logical_ref: String,
    /// Content-addressed basis digest captured when the proposal opened.
    pub basis_digest: String,
}

/// One per-target diff row rendered by the preview surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffPreviewRow {
    pub logical_ref: String,
    pub basis_digest_at_propose: String,
    pub basis_digest_observed_at_preview: String,
    pub basis_drift: BasisDriftState,
    pub match_count: u32,
    pub proposed_post_apply_digest: String,
    pub proposed_bytes_preview_snippet: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<DiffRowBlockReason>,
}

/// Propose-phase record (frozen at proposal time).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposalRecord {
    pub proposal_id: String,
    pub command_id: String,
    pub command_label: String,
    pub actor_display_name: String,
    pub workspace_id: String,
    pub search_pattern: String,
    pub replacement_text: String,
    pub declared_consequence_class: ConsequenceClass,
    pub declared_revert_class: RevertClass,
    pub target_specs: Vec<TargetSpec>,
    pub proposed_at: String,
}

/// Preview-phase record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewRecord {
    pub preview_id: String,
    pub proposal_id: String,
    pub previewed_at: String,
    pub rows: Vec<DiffPreviewRow>,
    pub overall_basis_drift_state: BasisDriftState,
    pub apply_admissibility: ApplyAdmissibility,
    pub total_match_count: u32,
    pub revert_class_after_apply: RevertClass,
}

/// One mutation/checkpoint link bound at apply time per target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetMutationLink {
    pub logical_ref: String,
    pub mutation_id: String,
    pub local_history_entry_id: String,
    pub pre_apply_body_object_id: String,
    pub post_apply_body_object_id: String,
    pub bytes_written: u64,
}

/// Apply-phase record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyRecord {
    pub apply_id: String,
    pub preview_id: String,
    pub applied_at: String,
    pub mutation_group_id: String,
    pub mutation_group_label: String,
    pub local_history_group_id: String,
    pub local_history_checkpoint_label: String,
    pub per_target_links: Vec<TargetMutationLink>,
    pub bytes_written_total: u64,
    pub realized_revert_class: RevertClass,
}

/// Validate-phase record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidateRecord {
    pub validation_id: String,
    pub apply_id: String,
    pub validated_at: String,
    pub per_target_results: Vec<TargetValidationResult>,
    pub all_targets_matched: bool,
}

/// One per-target validation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetValidationResult {
    pub logical_ref: String,
    pub expected_post_apply_digest: String,
    pub observed_post_apply_digest: String,
    pub matches: bool,
}

/// Revert-phase record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevertRecord {
    pub revert_id: String,
    pub apply_id: String,
    pub reverted_at: String,
    pub realized_revert_class: RevertClass,
    pub restored_from_local_history_group_id: String,
    pub restored_target_count: u32,
    pub reverse_mutation_group_id: String,
    pub group_resolution: GroupResolution,
}

/// Resolution captured on the named undo group at the end of the
/// lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupResolution {
    Applied,
    Reverted,
    PartiallyAppliedAndRolledBack,
}

impl GroupResolution {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Reverted => "reverted",
            Self::PartiallyAppliedAndRolledBack => "partially_applied_and_rolled_back",
        }
    }
}

/// Top-level mutation packet projected across the entire lifecycle.
///
/// A single packet record is the canonical evidence row a reviewer quotes
/// or a support export carries; every per-phase record stays bound to its
/// parent packet so lineage is unambiguous.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub packet_id: String,
    pub current_phase: PreviewApplyRevertPhase,
    pub proposal: ProposalRecord,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<PreviewRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply: Option<ApplyRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidateRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert: Option<RevertRecord>,
    /// Review-lane provenance event carrying the execution context that opened the packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_provenance_event: Option<ExecutionProvenanceEvent>,
    /// True once the user has explicitly chosen `Keep` on a validated
    /// apply, retiring the packet without a revert.
    pub kept: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept_at: Option<String>,
}

impl MutationPacket {
    fn new(packet_id: String, proposal: ProposalRecord) -> Self {
        Self {
            record_kind: MUTATION_PACKET_RECORD_KIND.to_owned(),
            schema_version: MUTATION_PACKET_SCHEMA_VERSION,
            packet_id,
            current_phase: PreviewApplyRevertPhase::Propose,
            proposal,
            preview: None,
            apply: None,
            validation: None,
            revert: None,
            context_provenance_event: None,
            kept: false,
            kept_at: None,
        }
    }

    /// Lineage ids carried across every phase, in canonical reading order.
    pub fn lineage_ids(&self) -> LineageIds {
        LineageIds {
            packet_id: self.packet_id.clone(),
            proposal_id: self.proposal.proposal_id.clone(),
            preview_id: self
                .preview
                .as_ref()
                .map(|preview| preview.preview_id.clone()),
            apply_id: self.apply.as_ref().map(|apply| apply.apply_id.clone()),
            mutation_group_id: self
                .apply
                .as_ref()
                .map(|apply| apply.mutation_group_id.clone()),
            local_history_group_id: self
                .apply
                .as_ref()
                .map(|apply| apply.local_history_group_id.clone()),
            validation_id: self
                .validation
                .as_ref()
                .map(|validation| validation.validation_id.clone()),
            revert_id: self.revert.as_ref().map(|revert| revert.revert_id.clone()),
        }
    }

    /// Deterministic plaintext render the wedge quotes verbatim for support
    /// exports, fixture replays, and reviewer copy-paste.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Preview / apply / revert — destructive core wedge\n");
        out.push_str(&format!(
            "Packet: {packet}\nProposal: {proposal}\nCommand: {command} ({label})\nPhase: {phase}\nConsequence: {consequence}\nDeclared revert class: {declared}\n",
            packet = self.packet_id,
            proposal = self.proposal.proposal_id,
            command = self.proposal.command_id,
            label = self.proposal.command_label,
            phase = self.current_phase.as_str(),
            consequence = self.proposal.declared_consequence_class.as_str(),
            declared = self.proposal.declared_revert_class.as_str(),
        ));
        if let Some(event) = &self.context_provenance_event {
            out.push_str(&format!(
                "Context provenance: {}\n",
                event.context_provenance.context_provenance_id
            ));
        }

        out.push_str("\n[Preview]\n");
        if let Some(preview) = &self.preview {
            out.push_str(&format!(
                "  preview_id: {preview_id}\n  basis_drift: {drift}\n  admission: {admission}\n  total_matches: {total}\n",
                preview_id = preview.preview_id,
                drift = drift_token(&preview.overall_basis_drift_state),
                admission = preview.apply_admissibility.as_token(),
                total = preview.total_match_count,
            ));
            for row in &preview.rows {
                let blocked = row
                    .blocked_reason
                    .map(|r| format!(" [blocked: {}]", r.as_str()))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "    {logical}: matches={matches}, drift={drift}{blocked}\n",
                    logical = row.logical_ref,
                    matches = row.match_count,
                    drift = drift_token(&row.basis_drift),
                ));
            }
        } else {
            out.push_str("  (not yet previewed)\n");
        }

        out.push_str("\n[Apply]\n");
        if let Some(apply) = &self.apply {
            out.push_str(&format!(
                "  apply_id: {apply_id}\n  mutation_group_id: {group}\n  local_history_group_id: {checkpoint}\n  realized_revert_class: {realized}\n  bytes_written_total: {bytes}\n",
                apply_id = apply.apply_id,
                group = apply.mutation_group_id,
                checkpoint = apply.local_history_group_id,
                realized = apply.realized_revert_class.as_str(),
                bytes = apply.bytes_written_total,
            ));
            for link in &apply.per_target_links {
                out.push_str(&format!(
                    "    {logical}: mutation_id={mutation}, entry_id={entry}, pre={pre}, post={post}\n",
                    logical = link.logical_ref,
                    mutation = link.mutation_id,
                    entry = link.local_history_entry_id,
                    pre = link.pre_apply_body_object_id,
                    post = link.post_apply_body_object_id,
                ));
            }
        } else {
            out.push_str("  (not yet applied)\n");
        }

        out.push_str("\n[Validate]\n");
        if let Some(validation) = &self.validation {
            out.push_str(&format!(
                "  validation_id: {validation_id}\n  all_targets_matched: {matched}\n",
                validation_id = validation.validation_id,
                matched = validation.all_targets_matched,
            ));
            for row in &validation.per_target_results {
                out.push_str(&format!(
                    "    {logical}: expected={expected}, observed={observed}, matches={matches}\n",
                    logical = row.logical_ref,
                    expected = row.expected_post_apply_digest,
                    observed = row.observed_post_apply_digest,
                    matches = row.matches,
                ));
            }
        } else {
            out.push_str("  (not yet validated)\n");
        }

        out.push_str("\n[Revert]\n");
        if let Some(revert) = &self.revert {
            out.push_str(&format!(
                "  revert_id: {revert_id}\n  realized_revert_class: {realized}\n  restored_from: {checkpoint}\n  group_resolution: {resolution}\n",
                revert_id = revert.revert_id,
                realized = revert.realized_revert_class.as_str(),
                checkpoint = revert.restored_from_local_history_group_id,
                resolution = revert.group_resolution.as_str(),
            ));
        } else if self.kept {
            out.push_str(&format!(
                "  (kept: no revert; kept_at={})\n",
                self.kept_at.as_deref().unwrap_or("?")
            ));
        } else {
            out.push_str("  (not yet reverted)\n");
        }

        out
    }

    /// Projects the apply/revert lineage into the shared local-history alpha shape.
    pub fn local_history_alpha_packet(
        &self,
        produced_at: impl Into<String>,
    ) -> Option<LocalHistoryAlphaPacket> {
        let apply = self.apply.as_ref()?;
        let entry_refs = apply
            .per_target_links
            .iter()
            .map(|link| link.local_history_entry_id.clone())
            .collect();
        let row = aureline_history::ActorLineageRow::from_review_apply(ReviewApplyLineageInput {
            row_id: format!("{}.local_history_lineage", apply.apply_id),
            display_label: apply.local_history_checkpoint_label.clone(),
            local_history_group_ref: apply.local_history_group_id.clone(),
            local_history_entry_refs: entry_refs,
            mutation_group_ref: apply.mutation_group_id.clone(),
            command_id: self.proposal.command_id.clone(),
            reversal_class: "restore_from_checkpoint".to_owned(),
            side_effect_summary: apply.mutation_group_label.clone(),
        });
        Some(
            LocalHistoryAlphaPacket::new(
                format!("{}.local_history_alpha", self.packet_id),
                produced_at,
                LocalHistoryConsumerSurface::ReviewApply,
            )
            .with_actor_lineage_row(row),
        )
    }
}

fn drift_token(state: &BasisDriftState) -> &'static str {
    match state {
        BasisDriftState::NoDrift => "no_drift",
        BasisDriftState::DriftDetected { .. } => "drift_detected",
        BasisDriftState::BasisMissing => "basis_missing",
    }
}

/// Stable lineage identifiers carried across every phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageIds {
    pub packet_id: String,
    pub proposal_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_history_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_id: Option<String>,
}

/// Errors returned by the destructive-core engine.
#[derive(Debug)]
pub enum WedgeError {
    NotInPhase {
        expected: PreviewApplyRevertPhase,
        actual: PreviewApplyRevertPhase,
    },
    ApplyBlocked(ApplyAdmissibility),
    NoPreview,
    NoApply,
    AlreadyReverted,
    AlreadyKept,
    History(HistoryError),
}

impl std::fmt::Display for WedgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInPhase { expected, actual } => write!(
                f,
                "preview/apply/revert wedge expected phase {expected} but the packet is in {actual}",
                expected = expected.as_str(),
                actual = actual.as_str(),
            ),
            Self::ApplyBlocked(reason) => write!(
                f,
                "apply blocked by {reason} — re-open preview after the basis settles",
                reason = reason.as_token(),
            ),
            Self::NoPreview => write!(f, "apply requires a preview; none has been computed"),
            Self::NoApply => write!(f, "validate/revert require an apply; none has been recorded"),
            Self::AlreadyReverted => write!(f, "packet has already been reverted"),
            Self::AlreadyKept => write!(f, "packet has already been kept"),
            Self::History(err) => write!(f, "history backend error: {err}"),
        }
    }
}

impl std::error::Error for WedgeError {}

impl From<HistoryError> for WedgeError {
    fn from(value: HistoryError) -> Self {
        Self::History(value)
    }
}

/// Engine that owns the destructive-core target world plus the history
/// store handles, and drives a [`MutationPacket`] through every phase.
pub struct DestructiveCoreEngine {
    workspace_id: String,
    actor_display_name: String,
    journal: MutationJournalStore,
    history: LocalHistoryStore,
    packets: IdSource,
    proposals: IdSource,
    previews: IdSource,
    applies: IdSource,
    validations: IdSource,
    reverts: IdSource,
    world: HashMap<String, Vec<u8>>,
    context_provenance: Option<ExecutionEventProvenance>,
    clock: Clock,
}

enum Clock {
    Wall,
    Pinned(Vec<String>, usize),
}

impl Clock {
    fn now_iso(&mut self) -> String {
        match self {
            Clock::Wall => wall_clock_iso(),
            Clock::Pinned(values, cursor) => {
                if values.is_empty() {
                    return wall_clock_iso();
                }
                let idx = *cursor % values.len();
                let value = values[idx].clone();
                *cursor = cursor.saturating_add(1);
                value
            }
        }
    }
}

fn wall_clock_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("instant_{nanos:032}")
}

impl DestructiveCoreEngine {
    /// Creates a new engine that writes journal and local-history records
    /// through `journal` and `history`.
    pub fn new(
        workspace_id: impl Into<String>,
        actor_display_name: impl Into<String>,
        journal: MutationJournalStore,
        history: LocalHistoryStore,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            actor_display_name: actor_display_name.into(),
            journal,
            history,
            packets: IdSource::new("pkt"),
            proposals: IdSource::new("prop"),
            previews: IdSource::new("prev"),
            applies: IdSource::new("app"),
            validations: IdSource::new("val"),
            reverts: IdSource::new("rev"),
            world: HashMap::new(),
            context_provenance: None,
            clock: Clock::Wall,
        }
    }

    /// Attaches the resolved execution context that opened review packets.
    pub fn with_execution_context(mut self, context: &ExecutionContext) -> Self {
        self.context_provenance = Some(ExecutionEventProvenance::from_context(context));
        self
    }

    /// Pin a fixed sequence of ISO timestamps so the protected-walk test
    /// captures a stable packet shape.
    pub fn with_pinned_clock(mut self, values: Vec<String>) -> Self {
        self.clock = Clock::Pinned(values, 0);
        self
    }

    /// Seed a target into the destructive-core world.
    pub fn seed_target(&mut self, logical_ref: impl Into<String>, bytes: Vec<u8>) {
        self.world.insert(logical_ref.into(), bytes);
    }

    /// Simulate an external mutation against `logical_ref`. Used by the
    /// failure drill to drift the basis after preview.
    pub fn simulate_external_mutation(&mut self, logical_ref: &str, bytes: Vec<u8>) {
        self.world.insert(logical_ref.to_owned(), bytes);
    }

    /// Read the current bytes for `logical_ref` (or `None` when missing).
    pub fn read_target(&self, logical_ref: &str) -> Option<&[u8]> {
        self.world.get(logical_ref).map(|bytes| bytes.as_slice())
    }

    /// Phase 1 — capture the proposal. The basis digest for each target is
    /// frozen on the proposal so a later preview can detect drift.
    pub fn propose(
        &mut self,
        target_refs: &[&str],
        search_pattern: impl Into<String>,
        replacement_text: impl Into<String>,
    ) -> Result<MutationPacket, WedgeError> {
        let proposal_id = self.proposals.mint();
        let packet_id = self.packets.mint();
        let proposed_at = self.clock.now_iso();

        let target_specs = target_refs
            .iter()
            .map(|logical_ref| TargetSpec {
                logical_ref: (*logical_ref).to_owned(),
                basis_digest: self
                    .world
                    .get(*logical_ref)
                    .map(|bytes| body_object_id(bytes))
                    .unwrap_or_else(|| "obj:missing".to_owned()),
            })
            .collect();

        let proposal = ProposalRecord {
            proposal_id,
            command_id: DESTRUCTIVE_CORE_COMMAND_ID.to_owned(),
            command_label: "Bulk replace across selected files".to_owned(),
            actor_display_name: self.actor_display_name.clone(),
            workspace_id: self.workspace_id.clone(),
            search_pattern: search_pattern.into(),
            replacement_text: replacement_text.into(),
            declared_consequence_class: ConsequenceClass::DestructiveReversibleWithCheckpoint,
            declared_revert_class: RevertClass::RestoreFromCheckpoint,
            target_specs,
            proposed_at,
        };

        let mut packet = MutationPacket::new(packet_id, proposal);
        if let Some(context_provenance) = &self.context_provenance {
            packet.context_provenance_event = Some(ExecutionProvenanceEvent::new(
                format!("execution-provenance-event:review:{}", packet.packet_id),
                ExecutionProvenanceEventClass::Review,
                packet.packet_id.clone(),
                packet.proposal.proposed_at.clone(),
                context_provenance.clone(),
            ));
        }
        Ok(packet)
    }

    /// Phase 2 — compute a preview against the current target world. The
    /// preview surfaces basis drift per row and refuses admission to
    /// `apply` when any row's basis has drifted from the proposal.
    pub fn preview(&mut self, packet: &mut MutationPacket) -> Result<(), WedgeError> {
        if packet.current_phase != PreviewApplyRevertPhase::Propose {
            return Err(WedgeError::NotInPhase {
                expected: PreviewApplyRevertPhase::Propose,
                actual: packet.current_phase,
            });
        }

        let preview_id = self.previews.mint();
        let previewed_at = self.clock.now_iso();

        let mut rows = Vec::with_capacity(packet.proposal.target_specs.len());
        let mut overall_drift = BasisDriftState::NoDrift;
        let mut drifted_count: u32 = 0;
        let mut missing_count: u32 = 0;
        let mut total_match_count: u32 = 0;
        let mut any_matches = false;

        for spec in &packet.proposal.target_specs {
            let row = self.compute_diff_row(
                spec,
                &packet.proposal.search_pattern,
                &packet.proposal.replacement_text,
            );
            match row.blocked_reason {
                Some(DiffRowBlockReason::BasisDrifted) => {
                    drifted_count = drifted_count.saturating_add(1);
                    overall_drift = row.basis_drift.clone();
                }
                Some(DiffRowBlockReason::BasisMissing)
                | Some(DiffRowBlockReason::TargetMissing) => {
                    missing_count = missing_count.saturating_add(1);
                }
                _ => {}
            }
            total_match_count = total_match_count.saturating_add(row.match_count);
            if row.match_count > 0 {
                any_matches = true;
            }
            rows.push(row);
        }

        let apply_admissibility = if drifted_count > 0 {
            ApplyAdmissibility::BlockedByBasisDrift {
                drifted_target_count: drifted_count,
            }
        } else if missing_count > 0 {
            ApplyAdmissibility::BlockedByTargetMissing {
                missing_target_count: missing_count,
            }
        } else if !any_matches {
            ApplyAdmissibility::BlockedByNoMatches
        } else {
            ApplyAdmissibility::Admitted
        };

        let preview = PreviewRecord {
            preview_id,
            proposal_id: packet.proposal.proposal_id.clone(),
            previewed_at,
            rows,
            overall_basis_drift_state: overall_drift,
            apply_admissibility,
            total_match_count,
            revert_class_after_apply: packet.proposal.declared_revert_class,
        };
        packet.preview = Some(preview);
        packet.current_phase = PreviewApplyRevertPhase::Preview;
        Ok(())
    }

    /// Phase 3 — mint a named undo group plus a content-addressed
    /// checkpoint, then apply each per-target diff into the world.
    ///
    /// Apply is **rejected** when the preview's admissibility is anything
    /// other than [`ApplyAdmissibility::Admitted`]. The wedge does not
    /// silently widen scope after preview.
    pub fn apply(&mut self, packet: &mut MutationPacket) -> Result<(), WedgeError> {
        if packet.current_phase != PreviewApplyRevertPhase::Preview {
            return Err(WedgeError::NotInPhase {
                expected: PreviewApplyRevertPhase::Preview,
                actual: packet.current_phase,
            });
        }

        let preview = packet
            .preview
            .as_ref()
            .ok_or(WedgeError::NoPreview)?
            .clone();
        if !preview.apply_admissibility.is_admitted() {
            return Err(WedgeError::ApplyBlocked(preview.apply_admissibility));
        }

        let apply_id = self.applies.mint();
        let mutation_group_id = self.journal.mint_group_id();
        let local_history_group_id = self.history.mint_group_id();
        let applied_at = self.clock.now_iso();

        let mutation_group_label = format!(
            "Bulk replace `{search}` -> `{replacement}` across {count} target(s)",
            search = packet.proposal.search_pattern,
            replacement = packet.proposal.replacement_text,
            count = packet.proposal.target_specs.len()
        );
        let local_history_checkpoint_label =
            format!("Pre-apply checkpoint for {}", mutation_group_label);

        let mut links = Vec::with_capacity(preview.rows.len());
        let mut member_mutation_ids = Vec::with_capacity(preview.rows.len());
        let mut member_entry_ids = Vec::with_capacity(preview.rows.len());
        let mut bytes_written_total: u64 = 0;

        for row in &preview.rows {
            // Persist the pre-apply body to the content-addressed object
            // store; this is the bytes the checkpoint restores on revert.
            let pre_bytes = self
                .world
                .get(&row.logical_ref)
                .cloned()
                .unwrap_or_default();
            let pre_apply_body_object_id = self.history.write_body_object(&pre_bytes)?;

            // Compute the proposed post-apply bytes by applying the diff.
            let post_bytes = apply_replacement(
                &pre_bytes,
                &packet.proposal.search_pattern,
                &packet.proposal.replacement_text,
            );
            let post_apply_body_object_id = self.history.write_body_object(&post_bytes)?;
            let bytes_written = post_bytes.len() as u64;
            bytes_written_total = bytes_written_total.saturating_add(bytes_written);

            // Mint mutation-journal and local-history entry ids.
            let mutation_id = self.journal.mint_mutation_id();
            let local_history_entry_id = self.history.mint_entry_id();

            // Persist the per-target mutation-journal entry.
            let filesystem_identity = synthetic_filesystem_identity(
                &row.logical_ref,
                &self.workspace_id,
                &pre_apply_body_object_id,
            );
            let journal_entry = MutationJournalEntryRecord::new(
                mutation_id.clone(),
                packet.proposal.command_id.clone(),
                ActorClass::ReviewApply,
                SourceClass::HumanLocal,
                ActorRef {
                    display_name: self.actor_display_name.clone(),
                    stable_id: None,
                    role: Some("author".to_owned()),
                },
                ScopeRef {
                    class: ScopeClass::Workspace,
                    id: self.workspace_id.clone(),
                },
                vec![TargetRef {
                    target_kind: TargetKind::FilesystemObject,
                    filesystem_identity: Some(filesystem_identity.clone()),
                    logical_ref: Some(row.logical_ref.clone()),
                    affected_range: None,
                }],
                applied_at.clone(),
                applied_at.clone(),
                "bulk_replace".to_owned(),
                ReversalClass::RestoreFromCheckpoint,
                RedactionClass::CodeAdjacent,
                DurableVsDisposable::DurableUserAuthored,
                SideEffectSummary::new(format!(
                    "Bulk replace applied {match_count} match(es) in {logical}",
                    match_count = row.match_count,
                    logical = row.logical_ref
                )),
                Vec::new(),
            )
            .with_group_id(mutation_group_id.clone());
            self.journal.write_entry(&journal_entry)?;

            // Persist the per-target local-history entry.
            let logical_document_id = format!("ld:{}", row.logical_ref);
            let entry = LocalHistoryEntryRecord::new(
                local_history_entry_id.clone(),
                SnapshotClass::WorkspaceMutationCheckpoint,
                applied_at.clone(),
                LogicalDocumentIdentity {
                    logical_document_id,
                    current_filesystem_identity: filesystem_identity,
                    canonical_identity_drift: None,
                    rename_move_history: Vec::new(),
                },
                CaptureDescriptor {
                    capture_mode: CaptureMode::ContentAddressedSnapshot,
                    omission_reason: CaptureOmissionReasonClass::NotOmitted,
                    body_available: true,
                    body_object_refs: vec![pre_apply_body_object_id.clone()],
                    reference_digest: None,
                    bytes_estimated: Some(pre_bytes.len() as u64),
                    omission_note: None,
                },
                MutationJournalLink {
                    linked_kind: MutationJournalLinkKind::MutationJournalEntry,
                    linked_id: mutation_id.clone(),
                    actor_class: Some(MutationJournalLinkActorClass::ReviewApply),
                    source_class: Some(SourceClass::HumanLocal),
                    reversal_class: Some(MutationJournalLinkReversalClass::RestoreFromCheckpoint),
                    redaction_class: Some(RedactionClass::CodeAdjacent),
                },
                RetentionScopeClass::RetainedByEvidenceReference,
                Some(format!(
                    "pre-apply snapshot for {logical}",
                    logical = row.logical_ref
                )),
            )
            .with_group_id(local_history_group_id.clone());
            self.history.write_entry(&entry)?;

            // Swap the world's bytes for this target with the post-apply bytes.
            self.world.insert(row.logical_ref.clone(), post_bytes);

            member_mutation_ids.push(mutation_id.clone());
            member_entry_ids.push(local_history_entry_id.clone());
            links.push(TargetMutationLink {
                logical_ref: row.logical_ref.clone(),
                mutation_id,
                local_history_entry_id,
                pre_apply_body_object_id,
                post_apply_body_object_id,
                bytes_written,
            });
        }

        // Persist the mutation-group record.
        let group_record = MutationGroupRecord::new(
            mutation_group_id.clone(),
            MutationGroupKind::BulkReplace,
            packet.proposal.command_id.clone(),
            ActorClass::ReviewApply,
            SourceClass::HumanLocal,
            ActorRef {
                display_name: self.actor_display_name.clone(),
                stable_id: None,
                role: Some("author".to_owned()),
            },
            ScopeRef {
                class: ScopeClass::Workspace,
                id: self.workspace_id.clone(),
            },
            applied_at.clone(),
            applied_at.clone(),
            MutationGroupResolution::Applied,
            member_mutation_ids,
            ReversalClass::RestoreFromCheckpoint,
            RedactionClass::CodeAdjacent,
            DurableVsDisposable::DurableUserAuthored,
            SideEffectSummary::new(mutation_group_label.clone()),
            Vec::new(),
        );
        self.journal.write_group(&group_record)?;

        // Persist the local-history group record so the checkpoint is
        // inspectable from the recovery / history surface later.
        let history_group = LocalHistoryGroupRecord::new(
            local_history_group_id.clone(),
            LocalHistoryGroupKind::BulkReplace,
            SnapshotClass::WorkspaceMutationCheckpoint,
            applied_at.clone(),
            applied_at.clone(),
            LocalHistoryGroupResolution::Applied,
            member_entry_ids,
            MutationJournalLink {
                linked_kind: MutationJournalLinkKind::MutationGroupRecord,
                linked_id: mutation_group_id.clone(),
                actor_class: Some(MutationJournalLinkActorClass::ReviewApply),
                source_class: Some(SourceClass::HumanLocal),
                reversal_class: Some(MutationJournalLinkReversalClass::RestoreFromCheckpoint),
                redaction_class: Some(RedactionClass::CodeAdjacent),
            },
            RetentionScopeClass::RetainedByEvidenceReference,
            Some(local_history_checkpoint_label.clone()),
        );
        self.history.write_group(&history_group)?;

        let apply = ApplyRecord {
            apply_id,
            preview_id: preview.preview_id.clone(),
            applied_at,
            mutation_group_id,
            mutation_group_label,
            local_history_group_id,
            local_history_checkpoint_label,
            per_target_links: links,
            bytes_written_total,
            realized_revert_class: RevertClass::RestoreFromCheckpoint,
        };
        packet.apply = Some(apply);
        packet.current_phase = PreviewApplyRevertPhase::Apply;
        Ok(())
    }

    /// Phase 4 — re-read each target and confirm the post-apply digest
    /// matches the digest the preview advertised.
    pub fn validate(&mut self, packet: &mut MutationPacket) -> Result<(), WedgeError> {
        if packet.current_phase != PreviewApplyRevertPhase::Apply {
            return Err(WedgeError::NotInPhase {
                expected: PreviewApplyRevertPhase::Apply,
                actual: packet.current_phase,
            });
        }

        let preview = packet.preview.as_ref().ok_or(WedgeError::NoPreview)?;
        let apply = packet.apply.as_ref().ok_or(WedgeError::NoApply)?;
        let validation_id = self.validations.mint();
        let validated_at = self.clock.now_iso();

        let mut results = Vec::with_capacity(preview.rows.len());
        let mut all_matched = true;
        for row in &preview.rows {
            let observed = self
                .world
                .get(&row.logical_ref)
                .map(|bytes| body_object_id(bytes))
                .unwrap_or_else(|| "obj:missing".to_owned());
            let matches = observed == row.proposed_post_apply_digest;
            if !matches {
                all_matched = false;
            }
            results.push(TargetValidationResult {
                logical_ref: row.logical_ref.clone(),
                expected_post_apply_digest: row.proposed_post_apply_digest.clone(),
                observed_post_apply_digest: observed,
                matches,
            });
        }

        packet.validation = Some(ValidateRecord {
            validation_id,
            apply_id: apply.apply_id.clone(),
            validated_at,
            per_target_results: results,
            all_targets_matched: all_matched,
        });
        packet.current_phase = PreviewApplyRevertPhase::Validate;
        Ok(())
    }

    /// Phase 5 — restore each target from the bound checkpoint, write a
    /// reverse mutation-journal group, and downgrade the realized
    /// [`RevertClass`] when the checkpoint cannot be honoured.
    pub fn revert(&mut self, packet: &mut MutationPacket) -> Result<(), WedgeError> {
        if !matches!(
            packet.current_phase,
            PreviewApplyRevertPhase::Apply | PreviewApplyRevertPhase::Validate
        ) {
            return Err(WedgeError::NotInPhase {
                expected: PreviewApplyRevertPhase::Validate,
                actual: packet.current_phase,
            });
        }
        if packet.kept {
            return Err(WedgeError::AlreadyKept);
        }
        if packet.revert.is_some() {
            return Err(WedgeError::AlreadyReverted);
        }

        let apply = packet.apply.as_ref().ok_or(WedgeError::NoApply)?.clone();
        let revert_id = self.reverts.mint();
        let reverted_at = self.clock.now_iso();

        // Restore each target from its pre-apply body object.
        let mut restored: u32 = 0;
        let mut reverse_member_ids = Vec::with_capacity(apply.per_target_links.len());
        for link in &apply.per_target_links {
            let pre_bytes = self.read_body_object(&link.pre_apply_body_object_id);
            if let Some(bytes) = pre_bytes {
                self.world.insert(link.logical_ref.clone(), bytes);
                restored = restored.saturating_add(1);

                // Reverse mutation-journal entry per target.
                let reverse_mutation_id = self.journal.mint_mutation_id();
                let filesystem_identity = synthetic_filesystem_identity(
                    &link.logical_ref,
                    &self.workspace_id,
                    &link.post_apply_body_object_id,
                );
                let reverse_entry = MutationJournalEntryRecord::new(
                    reverse_mutation_id.clone(),
                    "workspace.bulk_replace_in_files.revert".to_owned(),
                    ActorClass::UserCommand,
                    SourceClass::HumanLocal,
                    ActorRef {
                        display_name: self.actor_display_name.clone(),
                        stable_id: None,
                        role: Some("author".to_owned()),
                    },
                    ScopeRef {
                        class: ScopeClass::Workspace,
                        id: self.workspace_id.clone(),
                    },
                    vec![TargetRef {
                        target_kind: TargetKind::FilesystemObject,
                        filesystem_identity: Some(filesystem_identity),
                        logical_ref: Some(link.logical_ref.clone()),
                        affected_range: None,
                    }],
                    reverted_at.clone(),
                    reverted_at.clone(),
                    "bulk_replace_revert".to_owned(),
                    ReversalClass::RestoreFromCheckpoint,
                    RedactionClass::CodeAdjacent,
                    DurableVsDisposable::DurableUserAuthored,
                    SideEffectSummary::new(format!(
                        "Restored {logical} from pre-apply checkpoint",
                        logical = link.logical_ref
                    )),
                    Vec::new(),
                );
                self.journal.write_entry(&reverse_entry)?;
                reverse_member_ids.push(reverse_mutation_id);
            }
        }

        let group_resolution = if restored == apply.per_target_links.len() as u32 {
            GroupResolution::Reverted
        } else if restored == 0 {
            GroupResolution::Applied
        } else {
            GroupResolution::PartiallyAppliedAndRolledBack
        };
        let realized = if restored == apply.per_target_links.len() as u32 {
            RevertClass::RestoreFromCheckpoint
        } else {
            RevertClass::CompensatingAction
        };

        // Persist the reverse mutation-group record so support exports
        // surface the restore as one named row.
        let reverse_group_id = self.journal.mint_group_id();
        let reverse_group_label = format!("Revert of {label}", label = apply.mutation_group_label);
        let reverse_group_record = MutationGroupRecord::new(
            reverse_group_id.clone(),
            MutationGroupKind::BulkReplace,
            "workspace.bulk_replace_in_files.revert".to_owned(),
            ActorClass::UserCommand,
            SourceClass::HumanLocal,
            ActorRef {
                display_name: self.actor_display_name.clone(),
                stable_id: None,
                role: Some("author".to_owned()),
            },
            ScopeRef {
                class: ScopeClass::Workspace,
                id: self.workspace_id.clone(),
            },
            reverted_at.clone(),
            reverted_at.clone(),
            match group_resolution {
                GroupResolution::Applied => MutationGroupResolution::Applied,
                GroupResolution::Reverted => MutationGroupResolution::Reverted,
                GroupResolution::PartiallyAppliedAndRolledBack => {
                    MutationGroupResolution::PartiallyAppliedAndRolledBack
                }
            },
            reverse_member_ids,
            ReversalClass::RestoreFromCheckpoint,
            RedactionClass::CodeAdjacent,
            DurableVsDisposable::DurableUserAuthored,
            SideEffectSummary::new(reverse_group_label),
            Vec::new(),
        );
        self.journal.write_group(&reverse_group_record)?;

        packet.revert = Some(RevertRecord {
            revert_id,
            apply_id: apply.apply_id.clone(),
            reverted_at,
            realized_revert_class: realized,
            restored_from_local_history_group_id: apply.local_history_group_id.clone(),
            restored_target_count: restored,
            reverse_mutation_group_id: reverse_group_id,
            group_resolution,
        });
        packet.current_phase = PreviewApplyRevertPhase::Revert;
        Ok(())
    }

    /// Retire the packet without a revert (after a successful validate).
    pub fn keep(&mut self, packet: &mut MutationPacket) -> Result<(), WedgeError> {
        if packet.current_phase != PreviewApplyRevertPhase::Validate {
            return Err(WedgeError::NotInPhase {
                expected: PreviewApplyRevertPhase::Validate,
                actual: packet.current_phase,
            });
        }
        if packet.kept {
            return Err(WedgeError::AlreadyKept);
        }
        let kept_at = self.clock.now_iso();
        packet.kept = true;
        packet.kept_at = Some(kept_at);
        packet.current_phase = PreviewApplyRevertPhase::Keep;
        Ok(())
    }

    /// Reopen the preview against the current world state. The wedge
    /// returns a fresh preview with the latest basis-drift observations so
    /// the caller can re-review before attempting apply again.
    pub fn reopen_preview(&mut self, packet: &mut MutationPacket) -> Result<(), WedgeError> {
        if !matches!(
            packet.current_phase,
            PreviewApplyRevertPhase::Preview | PreviewApplyRevertPhase::Propose
        ) {
            return Err(WedgeError::NotInPhase {
                expected: PreviewApplyRevertPhase::Preview,
                actual: packet.current_phase,
            });
        }
        packet.preview = None;
        packet.current_phase = PreviewApplyRevertPhase::Propose;
        self.preview(packet)
    }

    fn compute_diff_row(
        &self,
        spec: &TargetSpec,
        search: &str,
        replacement: &str,
    ) -> DiffPreviewRow {
        let bytes = match self.world.get(&spec.logical_ref) {
            Some(bytes) => bytes.clone(),
            None => {
                return DiffPreviewRow {
                    logical_ref: spec.logical_ref.clone(),
                    basis_digest_at_propose: spec.basis_digest.clone(),
                    basis_digest_observed_at_preview: "obj:missing".to_owned(),
                    basis_drift: BasisDriftState::BasisMissing,
                    match_count: 0,
                    proposed_post_apply_digest: "obj:missing".to_owned(),
                    proposed_bytes_preview_snippet: String::new(),
                    blocked_reason: Some(DiffRowBlockReason::TargetMissing),
                };
            }
        };
        let observed_basis = body_object_id(&bytes);

        if observed_basis != spec.basis_digest {
            return DiffPreviewRow {
                logical_ref: spec.logical_ref.clone(),
                basis_digest_at_propose: spec.basis_digest.clone(),
                basis_digest_observed_at_preview: observed_basis.clone(),
                basis_drift: BasisDriftState::DriftDetected {
                    previous_basis: spec.basis_digest.clone(),
                    current_basis: observed_basis,
                },
                match_count: 0,
                proposed_post_apply_digest: "obj:invalidated_by_drift".to_owned(),
                proposed_bytes_preview_snippet: String::new(),
                blocked_reason: Some(DiffRowBlockReason::BasisDrifted),
            };
        }

        let pre_text = std::str::from_utf8(&bytes).unwrap_or("");
        let match_count = if search.is_empty() {
            0
        } else {
            pre_text.matches(search).count() as u32
        };
        if match_count == 0 {
            return DiffPreviewRow {
                logical_ref: spec.logical_ref.clone(),
                basis_digest_at_propose: spec.basis_digest.clone(),
                basis_digest_observed_at_preview: observed_basis.clone(),
                basis_drift: BasisDriftState::NoDrift,
                match_count: 0,
                proposed_post_apply_digest: observed_basis,
                proposed_bytes_preview_snippet: String::new(),
                blocked_reason: Some(DiffRowBlockReason::NoMatchesFound),
            };
        }

        let post_bytes = apply_replacement(&bytes, search, replacement);
        let post_digest = body_object_id(&post_bytes);
        let snippet = preview_snippet(&post_bytes, 80);

        DiffPreviewRow {
            logical_ref: spec.logical_ref.clone(),
            basis_digest_at_propose: spec.basis_digest.clone(),
            basis_digest_observed_at_preview: observed_basis,
            basis_drift: BasisDriftState::NoDrift,
            match_count,
            proposed_post_apply_digest: post_digest,
            proposed_bytes_preview_snippet: snippet,
            blocked_reason: None,
        }
    }

    fn read_body_object(&self, object_id: &str) -> Option<Vec<u8>> {
        let prefix = "obj:blake3:";
        let hex = object_id.strip_prefix(prefix)?;
        let storage_root = self.history.objects_root_path();
        let path = storage_root.join(format!("{hex}.blob"));
        std::fs::read(path).ok()
    }
}

fn apply_replacement(bytes: &[u8], search: &str, replacement: &str) -> Vec<u8> {
    if search.is_empty() {
        return bytes.to_vec();
    }
    match std::str::from_utf8(bytes) {
        Ok(text) => text.replace(search, replacement).into_bytes(),
        Err(_) => bytes.to_vec(),
    }
}

fn preview_snippet(bytes: &[u8], max_len: usize) -> String {
    let text = String::from_utf8_lossy(bytes);
    let trimmed: String = text.chars().take(max_len).collect();
    trimmed
}

fn synthetic_filesystem_identity(
    logical_ref: &str,
    workspace_id: &str,
    body_object_id_value: &str,
) -> FilesystemIdentityRecord {
    FilesystemIdentityRecord {
        record_kind: "filesystem_identity_record".to_owned(),
        filesystem_identity_schema_version: 1,
        presentation_path: PresentationPathRecord {
            uri: format!("aureline-ws://{workspace_id}/{logical_ref}"),
            display_label: logical_ref.to_owned(),
            root_badge: "destructive_core_wedge".to_owned(),
        },
        logical_workspace_identity: LogicalWorkspaceIdentityRecord {
            workspace_id: workspace_id.to_owned(),
            root_id: "destructive_core_wedge".to_owned(),
            logical_uri: format!("aureline-ws://{workspace_id}/{logical_ref}"),
            trust_state: "trusted".to_owned(),
            policy_scope: None,
        },
        canonical_filesystem_object: CanonicalFilesystemObjectRecord {
            canonical_uri: format!("aureline-ws://{workspace_id}/{logical_ref}"),
            normalization_form: "posix".to_owned(),
            strongest_identity_token: IdentityTokenRecord {
                kind: "content_digest".to_owned(),
                value: body_object_id_value.to_owned(),
            },
            fallback_identity_tokens: Vec::new(),
        },
        alias_set: AliasSetRecord {
            aliases: Vec::new(),
        },
    }
}

#[cfg(test)]
mod tests;
