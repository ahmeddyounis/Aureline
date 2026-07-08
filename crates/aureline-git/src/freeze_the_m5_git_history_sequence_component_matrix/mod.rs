//! Frozen M5 Git-history and risky-mutation component matrix.
//!
//! This module locks the reusable Git-history and history-surgery component
//! family that claimed M5 Git surfaces render: commit-graph headers, history
//! graph rows, branch-comparison chips, worktree rows, stash entries, reflog
//! recovery banners, rebase todo rows, sequence-editor headers, cherry-pick /
//! revert review sheets, patch-apply review sheets, conflict-checkpoint cards,
//! and force-push review dialogs. Each row binds one component to the exact
//! repo/worktree/ref identity it must preserve, the recovery checkpoint or
//! destination that must stay reachable, the approval-invalidation rule it must
//! honor, the browser/provider handoff boundary it must respect, the shared
//! downgrade vocabulary it may surface, the mutation-review class its verb
//! requires, and the consumer surfaces that must express the same truth.
//!
//! The matrix is the single source of truth for whether an M5 Git-history
//! surface may reuse a shared component instead of copying per-screen chrome.
//! It references the canonical commit-history, topology, stash, recovery,
//! sequence-edit, history-surgery, conflict-session, and ref-update contracts by
//! id rather than redefining them, so review, shell, help, support, export,
//! provider-overlay, AI-context, and CLI flows all read one vocabulary.
//!
//! Component truth is never reduced to a badge: rows control which identity,
//! recovery, approval, and handoff facts a component must keep visible. Multiple
//! Git verbs are never collapsed into one ambiguous confirm, exact target
//! refs/worktrees are never hidden, and conflict/recovery state never disappears
//! after a risky mutation. Local-only recovery stays explicit even when a
//! provider-linked review state also exists. Raw paths, raw object bytes, raw
//! branch names, raw patch/reflog/stash bodies, raw provider payloads, and
//! credentials stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-git-history-sequence-component-matrix.schema.json`](../../../../schemas/ui/m5-git-history-sequence-component-matrix.schema.json).
//! The contract doc is
//! [`docs/git/m5/freeze_the_m5_git_history_sequence_component_matrix.md`](../../../../docs/git/m5/freeze_the_m5_git_history_sequence_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-git-history-sequence-components/`](../../../../fixtures/ui/m5-git-history-sequence-components/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5GitHistoryComponentMatrixPacket`].
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_git_history_sequence_component_matrix";

/// Schema version for M5 Git-history component-matrix records.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-git-history-sequence-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/git/m5/freeze_the_m5_git_history_sequence_component_matrix.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-git-history-sequence-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/release/m5-git-history-sequence-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-git-history-sequence-components";

/// Repo-relative path of the canonical commit-history review contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_COMMIT_HISTORY_CONTRACT_REF: &str =
    "schemas/git/git_history_review.schema.json";

/// Repo-relative path of the canonical repository-topology contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_TOPOLOGY_CONTRACT_REF: &str =
    "schemas/review/repository-topology.schema.json";

/// Repo-relative path of the canonical stash-entry contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_STASH_CONTRACT_REF: &str =
    "schemas/git/stash_entry.schema.json";

/// Repo-relative path of the canonical recovery-checkpoint contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF: &str =
    "schemas/git/recovery_checkpoint.schema.json";

/// Repo-relative path of the canonical sequence-edit-session contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_SEQUENCE_EDIT_CONTRACT_REF: &str =
    "schemas/git/sequence_edit_session.schema.json";

/// Repo-relative path of the canonical history-surgery review contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_HISTORY_SURGERY_CONTRACT_REF: &str =
    "schemas/git/history-surgery-review.schema.json";

/// Repo-relative path of the canonical conflict-session contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_CONFLICT_SESSION_CONTRACT_REF: &str =
    "schemas/git/conflict_session.schema.json";

/// Repo-relative path of the canonical ref-update lineage contract.
pub const M5_GIT_HISTORY_COMPONENT_MATRIX_REF_UPDATE_CONTRACT_REF: &str =
    "schemas/review/sequence-edit-conflict-session-stash-entry-ref-update.schema.json";

/// Reusable Git-history / risky-mutation component frozen by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GitHistoryComponent {
    /// Header summarizing a commit-graph view (branch/ref anchor, range).
    CommitGraphHeader,
    /// One commit row within a history graph.
    HistoryGraphRow,
    /// Chip comparing two branches/refs (ahead/behind, base).
    BranchComparisonChip,
    /// Row describing one linked worktree and its root/branch scope.
    WorktreeRow,
    /// A single stash-shelf entry with its restore scope.
    StashEntry,
    /// Banner offering a reflog-based recovery destination.
    ReflogRecoveryBanner,
    /// One todo line inside an interactive-rebase / sequence edit.
    RebaseTodoRow,
    /// Header framing a sequence-editor (interactive rebase) session.
    SequenceEditorHeader,
    /// Review sheet for a cherry-pick or revert before it runs.
    CherryPickRevertReviewSheet,
    /// Review sheet for applying a patch/mailbox before it runs.
    PatchApplyReviewSheet,
    /// Card exposing a conflict checkpoint captured during a risky mutation.
    ConflictCheckpointCard,
    /// Dialog reviewing a force-push (ref rewrite) before it runs.
    ForcePushReviewDialog,
}

impl M5GitHistoryComponent {
    /// Every frozen component, in canonical order.
    pub const ALL: [Self; 12] = [
        Self::CommitGraphHeader,
        Self::HistoryGraphRow,
        Self::BranchComparisonChip,
        Self::WorktreeRow,
        Self::StashEntry,
        Self::ReflogRecoveryBanner,
        Self::RebaseTodoRow,
        Self::SequenceEditorHeader,
        Self::CherryPickRevertReviewSheet,
        Self::PatchApplyReviewSheet,
        Self::ConflictCheckpointCard,
        Self::ForcePushReviewDialog,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommitGraphHeader => "commit_graph_header",
            Self::HistoryGraphRow => "history_graph_row",
            Self::BranchComparisonChip => "branch_comparison_chip",
            Self::WorktreeRow => "worktree_row",
            Self::StashEntry => "stash_entry",
            Self::ReflogRecoveryBanner => "reflog_recovery_banner",
            Self::RebaseTodoRow => "rebase_todo_row",
            Self::SequenceEditorHeader => "sequence_editor_header",
            Self::CherryPickRevertReviewSheet => "cherry_pick_revert_review_sheet",
            Self::PatchApplyReviewSheet => "patch_apply_review_sheet",
            Self::ConflictCheckpointCard => "conflict_checkpoint_card",
            Self::ForcePushReviewDialog => "force_push_review_dialog",
        }
    }

    /// Canonical source contract this component binds to by id.
    ///
    /// The matrix references the existing Git-history/recovery contracts rather
    /// than redefining them, so every surface reads one vocabulary.
    pub const fn canonical_source_contract_ref(self) -> &'static str {
        match self {
            Self::CommitGraphHeader | Self::HistoryGraphRow => {
                M5_GIT_HISTORY_COMPONENT_MATRIX_COMMIT_HISTORY_CONTRACT_REF
            }
            Self::BranchComparisonChip | Self::WorktreeRow => {
                M5_GIT_HISTORY_COMPONENT_MATRIX_TOPOLOGY_CONTRACT_REF
            }
            Self::StashEntry => M5_GIT_HISTORY_COMPONENT_MATRIX_STASH_CONTRACT_REF,
            Self::ReflogRecoveryBanner => {
                M5_GIT_HISTORY_COMPONENT_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF
            }
            Self::RebaseTodoRow | Self::SequenceEditorHeader => {
                M5_GIT_HISTORY_COMPONENT_MATRIX_SEQUENCE_EDIT_CONTRACT_REF
            }
            Self::CherryPickRevertReviewSheet | Self::PatchApplyReviewSheet => {
                M5_GIT_HISTORY_COMPONENT_MATRIX_HISTORY_SURGERY_CONTRACT_REF
            }
            Self::ConflictCheckpointCard => {
                M5_GIT_HISTORY_COMPONENT_MATRIX_CONFLICT_SESSION_CONTRACT_REF
            }
            Self::ForcePushReviewDialog => M5_GIT_HISTORY_COMPONENT_MATRIX_REF_UPDATE_CONTRACT_REF,
        }
    }

    /// Whether this component drives a risky, history-mutating verb (and so must
    /// carry a non-display mutation-review class that keeps recovery reachable).
    pub const fn is_risky_mutation_surface(self) -> bool {
        matches!(
            self,
            Self::StashEntry
                | Self::RebaseTodoRow
                | Self::SequenceEditorHeader
                | Self::CherryPickRevertReviewSheet
                | Self::PatchApplyReviewSheet
                | Self::ForcePushReviewDialog
        )
    }
}

/// Maturity posture of a frozen component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentMaturityPosture {
    /// Frozen as stable, reusable M5 truth.
    Stable,
    /// Reusable but still narrowed while its surface hardens.
    Beta,
    /// Preview-only; consumable but claim is narrowed the most.
    Preview,
}

impl ComponentMaturityPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
        }
    }
}

/// Shared downgrade vocabulary a component may surface.
///
/// These states are distinct and may not collapse into a single "degraded"
/// badge; each narrows a claim and stays visible after a risky mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GitHistoryDowngradeState {
    /// A provider overlay is stale relative to local Git truth.
    StaleProviderOverlay,
    /// The target ref is detached or missing; identity must be spelled out.
    DetachedOrMissingRef,
    /// The worktree is dirty or conflicted at the operation target.
    DirtyOrConflictedWorktree,
    /// Topology is shallow/partial/sparse, so history is incomplete here.
    ShallowOrPartialTopology,
    /// No checkpoint exists; only a reflog-only recovery fallback is offered.
    ReflogOnlyFallback,
    /// A prior approval was invalidated by this component's change.
    ApprovalInvalidated,
    /// Operating offline / local-only; provider handoff is unavailable.
    OfflineLocalOnly,
}

impl GitHistoryDowngradeState {
    /// Every downgrade state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::StaleProviderOverlay,
        Self::DetachedOrMissingRef,
        Self::DirtyOrConflictedWorktree,
        Self::ShallowOrPartialTopology,
        Self::ReflogOnlyFallback,
        Self::ApprovalInvalidated,
        Self::OfflineLocalOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleProviderOverlay => "stale_provider_overlay",
            Self::DetachedOrMissingRef => "detached_or_missing_ref",
            Self::DirtyOrConflictedWorktree => "dirty_or_conflicted_worktree",
            Self::ShallowOrPartialTopology => "shallow_or_partial_topology",
            Self::ReflogOnlyFallback => "reflog_only_fallback",
            Self::ApprovalInvalidated => "approval_invalidated",
            Self::OfflineLocalOnly => "offline_local_only",
        }
    }
}

/// Mutation-review class a component's verb requires before it executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationReviewClass {
    /// Read-only display / navigation; no mutation and so no confirm.
    DisplayOnlyNoMutation,
    /// An explicit single-verb confirm (cherry-pick vs revert stays distinct).
    ExplicitVerbConfirm,
    /// A full sequence-rewrite plan confirm (interactive rebase / sequence).
    SequenceRewriteConfirm,
    /// A stash restore confirm (apply/pop/drop/branch stays distinct).
    StashRestoreConfirm,
    /// A patch-apply confirm against an explicit target.
    PatchApplyConfirm,
    /// A force-push (ref rewrite) confirm with rollback disclosed.
    ForcePushConfirm,
}

impl MutationReviewClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayOnlyNoMutation => "display_only_no_mutation",
            Self::ExplicitVerbConfirm => "explicit_verb_confirm",
            Self::SequenceRewriteConfirm => "sequence_rewrite_confirm",
            Self::StashRestoreConfirm => "stash_restore_confirm",
            Self::PatchApplyConfirm => "patch_apply_confirm",
            Self::ForcePushConfirm => "force_push_confirm",
        }
    }

    /// Whether this class actually gates a history-mutating verb.
    pub const fn is_risky_mutation(self) -> bool {
        !matches!(self, Self::DisplayOnlyNoMutation)
    }
}

/// Consumer surface that must be able to express the component vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentConsumerSurface {
    /// Review diff/summary/publish/history-edit surfaces.
    Review,
    /// Shell chrome, activity center, and status rows.
    Shell,
    /// In-product help / about surfaces.
    Help,
    /// Support inspector surfaces.
    Support,
    /// Redaction-safe support/export packets.
    SupportExport,
    /// Hosted provider overlay layered over local truth.
    ProviderOverlay,
    /// CLI / headless replay or JSON output.
    Cli,
    /// AI-context assembly and evidence inspectors.
    AiContext,
}

impl ComponentConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Review,
        Self::Shell,
        Self::Help,
        Self::Support,
        Self::SupportExport,
        Self::ProviderOverlay,
        Self::Cli,
        Self::AiContext,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Review => "review",
            Self::Shell => "shell",
            Self::Help => "help",
            Self::Support => "support",
            Self::SupportExport => "support_export",
            Self::ProviderOverlay => "provider_overlay",
            Self::Cli => "cli",
            Self::AiContext => "ai_context",
        }
    }
}

/// One component row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRow {
    /// Canonical component governed by this row.
    pub component: M5GitHistoryComponent,
    /// Human-readable component label.
    pub label: String,
    /// Maturity posture of the frozen component.
    pub maturity: ComponentMaturityPosture,
    /// Canonical source contract this component binds to by id.
    pub canonical_source_contract_ref: String,
    /// How the component preserves exact repo/worktree/ref identity.
    pub identity_preservation: String,
    /// The recovery checkpoint / destination that must stay reachable.
    pub recovery_checkpoint_rule: String,
    /// How approval invalidation is surfaced (never silent).
    pub approval_invalidation_rule: String,
    /// The browser/provider handoff boundary the component respects.
    pub browser_provider_handoff_rule: String,
    /// Shared downgrade states this component may surface.
    pub downgrade_vocab: Vec<GitHistoryDowngradeState>,
    /// Mutation-review class the component's verb requires.
    pub mutation_review_class: MutationReviewClass,
    /// Whether the component keeps its Git verb distinct (never collapsed).
    pub preserves_distinct_verb: bool,
    /// Consumer surfaces that must project this component.
    pub consumer_surfaces: Vec<ComponentConsumerSurface>,
}

/// One downgrade-vocabulary row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeStateRow {
    /// Downgrade state defined by this row.
    pub state: GitHistoryDowngradeState,
    /// Human-readable meaning of the state.
    pub meaning: String,
    /// Whether this state narrows a claim (never reduced to a badge).
    pub narrows_claim: bool,
    /// Whether this state must stay visible after a risky mutation.
    pub must_stay_visible_after_mutation: bool,
}

/// Governance review block proving the matrix controls truth, not badges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixGovernanceReview {
    /// Every claimed surface consumes one shared component matrix.
    pub every_surface_consumes_one_shared_matrix: bool,
    /// Exact repo/worktree/ref identity is preserved by every component.
    pub exact_ref_worktree_identity_preserved: bool,
    /// A recovery destination is always explicit for risky components.
    pub recovery_destination_always_explicit: bool,
    /// Approval invalidation is never surfaced as a silent/generic warning.
    pub approval_invalidation_never_silent: bool,
    /// No Git verb is collapsed into one ambiguous confirm.
    pub no_verb_collapsed_into_ambiguous_confirm: bool,
    /// Conflict/recovery state survives a risky mutation.
    pub conflict_recovery_state_survives_mutation: bool,
    /// Provider overlays never overwrite local Git truth.
    pub provider_overlay_never_overwrites_local_truth: bool,
    /// Local-only recovery stays explicit even with provider review state.
    pub local_only_recovery_stays_explicit: bool,
    /// The downgrade vocabulary is shared across every surface.
    pub downgrade_vocabulary_shared_across_surfaces: bool,
}

/// Consumer-parity review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixConsumerParity {
    /// Review can express the component family and its downgrade rules.
    pub review_expresses_family: bool,
    /// Shell can express the component family and its downgrade rules.
    pub shell_expresses_family: bool,
    /// Help can express the component family and its downgrade rules.
    pub help_expresses_family: bool,
    /// Support/export can express the component family and its downgrade rules.
    pub support_export_expresses_family: bool,
    /// CLI/headless can express the component family and its downgrade rules.
    pub cli_expresses_family: bool,
    /// Provider overlay can express the component family and downgrade rules.
    pub provider_overlay_expresses_family: bool,
}

/// Freeze posture block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixFreezePosture {
    /// True when the matrix is frozen as canonical M5 truth.
    pub frozen: bool,
    /// Review SLO in hours.
    pub review_slo_hours: u32,
    /// RFC 3339 timestamp of the last review.
    pub last_reviewed_at: String,
    /// True when stale review automatically narrows claims.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5GitHistoryComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GitHistoryComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<ComponentRow>,
    /// Downgrade-vocabulary rows.
    pub downgrade_state_rows: Vec<DowngradeStateRow>,
    /// Governance review block.
    pub governance_review: MatrixGovernanceReview,
    /// Consumer-parity block.
    pub consumer_parity: MatrixConsumerParity,
    /// Freeze posture block.
    pub freeze_posture: MatrixFreezePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 Git-history component-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GitHistoryComponentMatrixPacket {
    /// Record kind; must equal [`M5_GIT_HISTORY_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<ComponentRow>,
    /// Downgrade-vocabulary rows.
    pub downgrade_state_rows: Vec<DowngradeStateRow>,
    /// Governance review block.
    pub governance_review: MatrixGovernanceReview,
    /// Consumer-parity block.
    pub consumer_parity: MatrixConsumerParity,
    /// Freeze posture block.
    pub freeze_posture: MatrixFreezePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GitHistoryComponentMatrixPacket {
    /// Builds a matrix packet from frozen input.
    pub fn new(input: M5GitHistoryComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_GIT_HISTORY_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            downgrade_state_rows: input.downgrade_state_rows,
            governance_review: input.governance_review,
            consumer_parity: input.consumer_parity,
            freeze_posture: input.freeze_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the matrix invariants.
    ///
    /// Returns every violation found rather than stopping at the first, so a
    /// regenerator or CI gate can report the full set at once.
    pub fn validate(&self) -> Vec<M5GitHistoryComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GIT_HISTORY_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5GitHistoryComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5GitHistoryComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GitHistoryComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_downgrade_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_parity(self, &mut violations);
        validate_freeze_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 git history component matrix packet serializes"),
        ) {
            violations.push(M5GitHistoryComponentMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 git history component matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Git-History and Risky-Mutation Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Frozen: {} (review SLO: {} hours, last reviewed: {})\n",
            self.freeze_posture.frozen,
            self.freeze_posture.review_slo_hours,
            self.freeze_posture.last_reviewed_at
        ));
        out.push_str(&format!(
            "- Rows: {} components / {} downgrade states\n",
            self.component_rows.len(),
            self.downgrade_state_rows.len(),
        ));

        out.push_str("\n## Components\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}** ({}, `{}`): review `{}`, binds `{}`\n",
                row.component.as_str(),
                row.maturity.as_str(),
                if row.preserves_distinct_verb {
                    "distinct-verb"
                } else {
                    "collapsed-verb"
                },
                row.mutation_review_class.as_str(),
                row.canonical_source_contract_ref,
            ));
        }

        out.push_str("\n## Downgrade vocabulary\n\n");
        for row in &self.downgrade_state_rows {
            out.push_str(&format!("- **{}**: {}\n", row.state.as_str(), row.meaning));
        }
        out
    }
}

/// Errors emitted when reading the checked-in matrix export.
#[derive(Debug)]
pub enum M5GitHistoryComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GitHistoryComponentMatrixViolation>),
}

impl fmt::Display for M5GitHistoryComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 git history component matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 git history component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GitHistoryComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5GitHistoryComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GitHistoryComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required component is missing from the matrix.
    RequiredComponentMissing,
    /// A component is listed more than once.
    DuplicateComponent,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row does not bind its canonical source contract.
    ComponentSourceContractMismatch,
    /// A component row does not name how it preserves exact identity.
    IdentityPreservationMissing,
    /// A component row does not name its recovery checkpoint/destination.
    RecoveryCheckpointRuleMissing,
    /// A component row does not name its approval-invalidation rule.
    ApprovalInvalidationRuleMissing,
    /// A component row does not name its browser/provider handoff rule.
    BrowserProviderHandoffRuleMissing,
    /// A risky component lacks a real mutation-review class.
    RiskyComponentMissingMutationReview,
    /// A mutating component collapses its Git verb into an ambiguous confirm.
    RiskyComponentCollapsesVerbs,
    /// A required downgrade state is missing from the matrix.
    RequiredDowngradeStateMissing,
    /// A downgrade row is incomplete (e.g. reduced to a badge).
    DowngradeStateRowIncomplete,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer parity does not satisfy required invariants.
    ConsumerParityIncomplete,
    /// Freeze posture block is incomplete.
    FreezePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5GitHistoryComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::DuplicateComponent => "duplicate_component",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::ComponentSourceContractMismatch => "component_source_contract_mismatch",
            Self::IdentityPreservationMissing => "identity_preservation_missing",
            Self::RecoveryCheckpointRuleMissing => "recovery_checkpoint_rule_missing",
            Self::ApprovalInvalidationRuleMissing => "approval_invalidation_rule_missing",
            Self::BrowserProviderHandoffRuleMissing => "browser_provider_handoff_rule_missing",
            Self::RiskyComponentMissingMutationReview => "risky_component_missing_mutation_review",
            Self::RiskyComponentCollapsesVerbs => "risky_component_collapses_verbs",
            Self::RequiredDowngradeStateMissing => "required_downgrade_state_missing",
            Self::DowngradeStateRowIncomplete => "downgrade_state_row_incomplete",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerParityIncomplete => "consumer_parity_incomplete",
            Self::FreezePostureIncomplete => "freeze_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable matrix export.
///
/// # Errors
///
/// Returns [`M5GitHistoryComponentMatrixArtifactError`] when the checked-in
/// export fails to parse or violates the frozen contract.
pub fn current_stable_m5_git_history_component_matrix_export(
) -> Result<M5GitHistoryComponentMatrixPacket, M5GitHistoryComponentMatrixArtifactError> {
    let packet: M5GitHistoryComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-git-history-sequence-proof/support_export.json"
    )))
    .map_err(M5GitHistoryComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GitHistoryComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5GitHistoryComponentMatrixPacket,
    violations: &mut Vec<M5GitHistoryComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GIT_HISTORY_COMPONENT_MATRIX_SCHEMA_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_DOC_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_COMMIT_HISTORY_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_TOPOLOGY_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_STASH_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_RECOVERY_CHECKPOINT_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_SEQUENCE_EDIT_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_HISTORY_SURGERY_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_CONFLICT_SESSION_CONTRACT_REF,
        M5_GIT_HISTORY_COMPONENT_MATRIX_REF_UPDATE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GitHistoryComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_component_rows(
    packet: &M5GitHistoryComponentMatrixPacket,
    violations: &mut Vec<M5GitHistoryComponentMatrixViolation>,
) {
    let mut seen: HashSet<M5GitHistoryComponent> = HashSet::new();
    for row in &packet.component_rows {
        if !seen.insert(row.component) {
            violations.push(M5GitHistoryComponentMatrixViolation::DuplicateComponent);
        }
        if row.label.trim().is_empty()
            || row.downgrade_vocab.is_empty()
            || row.consumer_surfaces.is_empty()
        {
            violations.push(M5GitHistoryComponentMatrixViolation::ComponentRowIncomplete);
        }
        if row.canonical_source_contract_ref != row.component.canonical_source_contract_ref() {
            violations.push(M5GitHistoryComponentMatrixViolation::ComponentSourceContractMismatch);
        }
        if row.identity_preservation.trim().is_empty() {
            violations.push(M5GitHistoryComponentMatrixViolation::IdentityPreservationMissing);
        }
        if row.recovery_checkpoint_rule.trim().is_empty() {
            violations.push(M5GitHistoryComponentMatrixViolation::RecoveryCheckpointRuleMissing);
        }
        if row.approval_invalidation_rule.trim().is_empty() {
            violations.push(M5GitHistoryComponentMatrixViolation::ApprovalInvalidationRuleMissing);
        }
        if row.browser_provider_handoff_rule.trim().is_empty() {
            violations
                .push(M5GitHistoryComponentMatrixViolation::BrowserProviderHandoffRuleMissing);
        }
        // A risky, history-mutating component must carry a real mutation-review
        // class; a display component must not claim a risky verb.
        if row.component.is_risky_mutation_surface()
            != row.mutation_review_class.is_risky_mutation()
        {
            violations
                .push(M5GitHistoryComponentMatrixViolation::RiskyComponentMissingMutationReview);
        }
        // Guardrail: a mutating verb may never be collapsed into an ambiguous
        // confirm; its verb must stay distinct.
        if row.mutation_review_class.is_risky_mutation() && !row.preserves_distinct_verb {
            violations.push(M5GitHistoryComponentMatrixViolation::RiskyComponentCollapsesVerbs);
        }
    }
    for required in M5GitHistoryComponent::ALL {
        if !seen.contains(&required) {
            violations.push(M5GitHistoryComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }
}

fn validate_downgrade_rows(
    packet: &M5GitHistoryComponentMatrixPacket,
    violations: &mut Vec<M5GitHistoryComponentMatrixViolation>,
) {
    let present: BTreeSet<GitHistoryDowngradeState> = packet
        .downgrade_state_rows
        .iter()
        .map(|row| row.state)
        .collect();
    for required in GitHistoryDowngradeState::ALL {
        if !present.contains(&required) {
            violations.push(M5GitHistoryComponentMatrixViolation::RequiredDowngradeStateMissing);
            return;
        }
    }

    for row in &packet.downgrade_state_rows {
        // Guardrail: a downgrade state may not be reduced to a badge; it must
        // narrow the claim. Recovery-critical states must survive a mutation.
        if row.meaning.trim().is_empty() || !row.narrows_claim {
            violations.push(M5GitHistoryComponentMatrixViolation::DowngradeStateRowIncomplete);
        }
        if row.state == GitHistoryDowngradeState::ReflogOnlyFallback
            && !row.must_stay_visible_after_mutation
        {
            violations.push(M5GitHistoryComponentMatrixViolation::DowngradeStateRowIncomplete);
        }
    }
}

fn validate_governance_review(
    packet: &M5GitHistoryComponentMatrixPacket,
    violations: &mut Vec<M5GitHistoryComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.every_surface_consumes_one_shared_matrix,
        review.exact_ref_worktree_identity_preserved,
        review.recovery_destination_always_explicit,
        review.approval_invalidation_never_silent,
        review.no_verb_collapsed_into_ambiguous_confirm,
        review.conflict_recovery_state_survives_mutation,
        review.provider_overlay_never_overwrites_local_truth,
        review.local_only_recovery_stays_explicit,
        review.downgrade_vocabulary_shared_across_surfaces,
    ] {
        if !ok {
            violations.push(M5GitHistoryComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_parity(
    packet: &M5GitHistoryComponentMatrixPacket,
    violations: &mut Vec<M5GitHistoryComponentMatrixViolation>,
) {
    let parity = &packet.consumer_parity;
    for ok in [
        parity.review_expresses_family,
        parity.shell_expresses_family,
        parity.help_expresses_family,
        parity.support_export_expresses_family,
        parity.cli_expresses_family,
        parity.provider_overlay_expresses_family,
    ] {
        if !ok {
            violations.push(M5GitHistoryComponentMatrixViolation::ConsumerParityIncomplete);
            return;
        }
    }
}

fn validate_freeze_posture(
    packet: &M5GitHistoryComponentMatrixPacket,
    violations: &mut Vec<M5GitHistoryComponentMatrixViolation>,
) {
    if !packet.freeze_posture.frozen
        || packet.freeze_posture.review_slo_hours == 0
        || packet.freeze_posture.last_reviewed_at.trim().is_empty()
    {
        violations.push(M5GitHistoryComponentMatrixViolation::FreezePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
