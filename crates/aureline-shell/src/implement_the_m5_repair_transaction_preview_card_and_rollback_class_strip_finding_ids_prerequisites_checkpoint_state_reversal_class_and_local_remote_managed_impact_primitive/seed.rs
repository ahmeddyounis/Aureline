//! Canonical seed builders for the M5 repair-transaction-preview-card / rollback-class-strip
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_PACKET_ID: &str =
    "m5-repair-transaction-preview-card-rollback-class-strip-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card(
    input: M5RepairTransactionPreviewCardResolutionInput,
) -> M5ResolvedRepairTransactionPreviewCard {
    resolve_repair_transaction_preview_card(input).expect("seed preview card input resolves")
}

fn strip(input: M5RollbackClassStripResolutionInput) -> M5ResolvedRollbackClassStrip {
    resolve_rollback_class_strip(input).expect("seed rollback strip input resolves")
}

// -- Canonical repair-transaction preview card examples ---------------------------------------

/// Clean card for a local-workspace repair with a full checkpoint.
fn card_local_clean() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:local".to_owned(),
        repair_id: "repair-0007".to_owned(),
        linked_finding_ids: strings(&["finding-0031", "finding-0032"]),
        prerequisites: strings(&["workspace-trusted", "no-unsaved-edits"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "rewrites 3 files under src/".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean card for a remote-host repair with a partial checkpoint.
fn card_remote_clean() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:remote".to_owned(),
        repair_id: "repair-0008".to_owned(),
        linked_finding_ids: strings(&["finding-0040"]),
        prerequisites: strings(&["remote-connected"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointPartial,
        checkpoint_absence_disclosed: false,
        impact_scope: "updates remote package manifest".to_owned(),
        target_class: M5RepairTargetClass::RemoteHost,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean card for a managed-workspace repair with an external checkpoint.
fn card_managed_clean() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:managed".to_owned(),
        repair_id: "repair-0009".to_owned(),
        linked_finding_ids: strings(&["finding-0051"]),
        prerequisites: strings(&["policy-allows-repair"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointExternal,
        checkpoint_absence_disclosed: false,
        impact_scope: "reapplies managed settings".to_owned(),
        target_class: M5RepairTargetClass::ManagedWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean card for a local repair whose missing checkpoint is disclosed before apply.
fn card_checkpoint_absent_disclosed_clean() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:no-checkpoint".to_owned(),
        repair_id: "repair-0010".to_owned(),
        linked_finding_ids: strings(&["finding-0060"]),
        prerequisites: strings(&["workspace-trusted"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointMissing,
        checkpoint_absence_disclosed: true,
        impact_scope: "deletes stale lockfile (no restore point)".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: the stable repair id is unstated.
fn card_repair_id_unstated() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:no-id".to_owned(),
        repair_id: "  ".to_owned(),
        linked_finding_ids: strings(&["finding-0031"]),
        prerequisites: strings(&["workspace-trusted"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "rewrites config".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: no linked finding id is named.
fn card_linked_findings_unstated() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:no-findings".to_owned(),
        repair_id: "repair-0011".to_owned(),
        linked_finding_ids: Vec::new(),
        prerequisites: strings(&["workspace-trusted"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "rewrites config".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: the prerequisites are not stated.
fn card_prerequisites_unstated() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:no-prereqs".to_owned(),
        repair_id: "repair-0012".to_owned(),
        linked_finding_ids: strings(&["finding-0070"]),
        prerequisites: Vec::new(),
        prerequisites_stated: false,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "rewrites config".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: the checkpoint state cannot be resolved.
fn card_checkpoint_unresolved() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:checkpoint-unknown".to_owned(),
        repair_id: "repair-0013".to_owned(),
        linked_finding_ids: strings(&["finding-0080"]),
        prerequisites: strings(&["policy-allows-repair"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointUnknown,
        checkpoint_absence_disclosed: false,
        impact_scope: "reapplies managed settings".to_owned(),
        target_class: M5RepairTargetClass::ManagedWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: a missing checkpoint's absence is hidden before apply.
fn card_checkpoint_absence_hidden() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:checkpoint-hidden".to_owned(),
        repair_id: "repair-0014".to_owned(),
        linked_finding_ids: strings(&["finding-0090"]),
        prerequisites: strings(&["workspace-trusted"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointMissing,
        checkpoint_absence_disclosed: false,
        impact_scope: "deletes generated artifacts".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: the impact scope is unstated.
fn card_impact_scope_unstated() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:no-scope".to_owned(),
        repair_id: "repair-0015".to_owned(),
        linked_finding_ids: strings(&["finding-0100"]),
        prerequisites: strings(&["workspace-trusted"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "  ".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: the local / remote / managed target class cannot be resolved.
fn card_target_unresolved() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:target-unknown".to_owned(),
        repair_id: "repair-0016".to_owned(),
        linked_finding_ids: strings(&["finding-0110"]),
        prerequisites: strings(&["remote-connected"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointPartial,
        checkpoint_absence_disclosed: false,
        impact_scope: "updates package manifest".to_owned(),
        target_class: M5RepairTargetClass::TargetUnknown,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: an incomplete preview reads as ready to apply.
fn card_preview_not_ready() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:incomplete".to_owned(),
        repair_id: "repair-0017".to_owned(),
        linked_finding_ids: strings(&["finding-0120"]),
        prerequisites: strings(&["workspace-trusted"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "rewrites config".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewIncomplete,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: the local / remote / managed target collapsed into a generic target.
fn card_target_collapsed() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:target-collapsed".to_owned(),
        repair_id: "repair-0018".to_owned(),
        linked_finding_ids: strings(&["finding-0130"]),
        prerequisites: strings(&["remote-connected"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointPartial,
        checkpoint_absence_disclosed: false,
        impact_scope: "updates package manifest".to_owned(),
        target_class: M5RepairTargetClass::RemoteHost,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: true,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded card: no command-backed review path is reachable.
fn card_review_path_missing() -> M5ResolvedRepairTransactionPreviewCard {
    card(M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:no-review".to_owned(),
        repair_id: "repair-0019".to_owned(),
        linked_finding_ids: strings(&["finding-0140"]),
        prerequisites: strings(&["policy-allows-repair"]),
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointExternal,
        checkpoint_absence_disclosed: false,
        impact_scope: "reapplies managed settings".to_owned(),
        target_class: M5RepairTargetClass::ManagedWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: false,
        proof_fresh: true,
    })
}

// -- Canonical rollback-class strip examples ---------------------------------------------------

/// Clean strip for an exact reversal.
fn strip_exact_clean() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:exact".to_owned(),
        repair_id: "repair-0007".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a regenerate reversal, honestly reversible by regenerating state.
fn strip_regenerate_clean() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:regenerate".to_owned(),
        repair_id: "repair-0008".to_owned(),
        reversal_class: M5RepairReversalClass::RegenerateReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointPartial,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a compensating reversal, never implying full reversibility.
fn strip_compensate_clean() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:compensate".to_owned(),
        repair_id: "repair-0009".to_owned(),
        reversal_class: M5RepairReversalClass::CompensatingReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: false,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a manual-follow-up reversal.
fn strip_manual_clean() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:manual".to_owned(),
        repair_id: "repair-0010".to_owned(),
        reversal_class: M5RepairReversalClass::ManualFollowUp,
        checkpoint_state: M5RepairCheckpointState::CheckpointExternal,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: false,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for an audit-only change that cannot be reversed in-product.
fn strip_audit_clean() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:audit".to_owned(),
        repair_id: "repair-0009".to_owned(),
        reversal_class: M5RepairReversalClass::AuditOnly,
        checkpoint_state: M5RepairCheckpointState::CheckpointExternal,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: false,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean strip for a compensating reversal whose missing checkpoint is disclosed before apply.
fn strip_checkpoint_absent_disclosed_clean() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:no-checkpoint".to_owned(),
        repair_id: "repair-0010".to_owned(),
        reversal_class: M5RepairReversalClass::CompensatingReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointMissing,
        checkpoint_absence_disclosed: true,
        reads_as_reversible: false,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: the stable repair id is unstated.
fn strip_repair_id_unstated() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:no-id".to_owned(),
        repair_id: "".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: the reversal class cannot be resolved.
fn strip_reversal_unresolved() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:reversal-unknown".to_owned(),
        repair_id: "repair-0016".to_owned(),
        reversal_class: M5RepairReversalClass::ReversalUnknown,
        checkpoint_state: M5RepairCheckpointState::CheckpointPartial,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: false,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: the checkpoint state cannot be resolved.
fn strip_checkpoint_unresolved() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:checkpoint-unknown".to_owned(),
        repair_id: "repair-0013".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointUnknown,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: a missing checkpoint's absence is hidden before apply.
fn strip_checkpoint_absence_hidden() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:checkpoint-hidden".to_owned(),
        repair_id: "repair-0014".to_owned(),
        reversal_class: M5RepairReversalClass::CompensatingReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointMissing,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: false,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: reversibility is implied without an exact or regenerate reversal.
fn strip_reversibility_overclaimed() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:overclaimed".to_owned(),
        repair_id: "repair-0010".to_owned(),
        reversal_class: M5RepairReversalClass::ManualFollowUp,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: a non-exact reversal leaves its limit undisclosed.
fn strip_reversal_limit_hidden() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:limit-hidden".to_owned(),
        repair_id: "repair-0009".to_owned(),
        reversal_class: M5RepairReversalClass::CompensatingReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: false,
        reversal_limit_disclosed: false,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: distinct reversal classes collapse into a generic undo.
fn strip_generic_undo() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:generic-undo".to_owned(),
        repair_id: "repair-0008".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: true,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded strip: no command-backed review path is reachable.
fn strip_review_path_missing() -> M5ResolvedRollbackClassStrip {
    strip(M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:no-review".to_owned(),
        repair_id: "repair-0019".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: false,
        proof_fresh: true,
    })
}

// -- Row builders ----------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5RepairPreviewRollbackConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    repair_transaction_preview_card_examples: Vec<M5ResolvedRepairTransactionPreviewCard>,
    rollback_class_strip_examples: Vec<M5ResolvedRollbackClassStrip>,
) -> M5RepairPreviewRollbackControlsRow {
    M5RepairPreviewRollbackControlsRow {
        consumer_surface,
        qualification: M5WorkspaceTrustRepairQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5WorkspaceTrustRepairDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5WorkspaceTrustRepairRequiredLabel::Identity,
            M5WorkspaceTrustRepairRequiredLabel::State,
            M5WorkspaceTrustRepairRequiredLabel::KeyboardRoute,
            M5WorkspaceTrustRepairRequiredLabel::ReversalAndCheckpoint,
        ],
        accessibility_routes: M5WorkspaceTrustRepairAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5RepairPreviewRollbackAnatomyPart::ALL.to_vec(),
        export_fields: M5RepairPreviewRollbackExportField::ALL.to_vec(),
        downgrade_triggers,
        repair_transaction_preview_card_examples,
        rollback_class_strip_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_REF,
            M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
            M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
        ]),
        hides_checkpoint_absence_or_reversal_limits: false,
        collapses_reversal_classes_into_generic_success: false,
        implies_reversibility_without_exact_or_regenerate: false,
        hides_target_class_or_impact_scope: false,
    }
}

fn controls_rows() -> Vec<M5RepairPreviewRollbackControlsRow> {
    use M5WorkspaceTrustRepairConsumerSurface as C;
    use M5WorkspaceTrustRepairDowngradeTrigger as D;

    vec![
        base_row(
            C::DoctorUi,
            "Project Doctor owner",
            "Project Doctor renders one repair-transaction preview card naming repair id, linked findings, prerequisites, checkpoint state, impact scope, and local target class, and one rollback-class strip naming the controlled reversal class before anything is applied",
            "evidence:m5-repair-preview-rollback-doctor-ui:001",
            vec![
                D::RepairTargetIdsUnstated,
                D::CheckpointAbsenceHidden,
                D::ReversalLimitHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_local_clean(),
                card_checkpoint_absent_disclosed_clean(),
                card_checkpoint_absence_hidden(),
                card_preview_not_ready(),
            ],
            vec![
                strip_exact_clean(),
                strip_compensate_clean(),
                strip_reversibility_overclaimed(),
                strip_reversal_limit_hidden(),
            ],
        ),
        base_row(
            C::RemoteUi,
            "Remote repair owner",
            "The remote / workspace UI reuses the same transaction-preview grammar for a remote-host target, degrading honestly when the target class is unresolved or collapsed into a generic target",
            "evidence:m5-repair-preview-rollback-remote-ui:001",
            vec![
                D::CheckpointAbsenceHidden,
                D::ReversalClassCollapsedIntoGenericSuccess,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_remote_clean(),
                card_target_unresolved(),
                card_target_collapsed(),
            ],
            vec![strip_regenerate_clean(), strip_generic_undo()],
        ),
        base_row(
            C::SafeModeUi,
            "Safe mode owner",
            "Safe mode previews a managed-workspace repair with its checkpoint state and a manual-follow-up rollback class, degrading honestly when a checkpoint or review path cannot be resolved",
            "evidence:m5-repair-preview-rollback-safe-mode-ui:001",
            vec![
                D::CheckpointAbsenceHidden,
                D::ReversalLimitHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_managed_clean(),
                card_checkpoint_unresolved(),
                card_review_path_missing(),
            ],
            vec![
                strip_manual_clean(),
                strip_checkpoint_absence_hidden(),
                strip_review_path_missing(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved card and strip truth, so a repair with unstated ids, an unresolved reversal class, or a hidden checkpoint absence is visible in evidence rather than hidden",
            "evidence:m5-repair-preview-rollback-support-export:001",
            vec![
                D::RepairTargetIdsUnstated,
                D::CheckpointAbsenceHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_repair_id_unstated(),
                card_linked_findings_unstated(),
                card_impact_scope_unstated(),
            ],
            vec![
                strip_reversal_unresolved(),
                strip_checkpoint_unresolved(),
                strip_repair_id_unstated(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product repair owner",
            "In-product surfaces reuse the same transaction-preview grammar and reversal vocabulary a user sees in Project Doctor, keeping an audit-only change honest about its reversal limits and disclosing a missing checkpoint before apply",
            "evidence:m5-repair-preview-rollback-product-ui:001",
            vec![
                D::RepairTargetIdsUnstated,
                D::ReversalLimitHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![card_local_clean(), card_prerequisites_unstated()],
            vec![
                strip_audit_clean(),
                strip_checkpoint_absent_disclosed_clean(),
            ],
        ),
    ]
}

fn governance_review() -> M5RepairPreviewRollbackGovernanceReview {
    M5RepairPreviewRollbackGovernanceReview {
        preview_card_names_repair_id_and_linked_findings: true,
        preview_card_names_prerequisites_impact_and_target_class: true,
        checkpoint_presence_or_absence_visible_before_apply: true,
        rollback_strip_uses_controlled_reversal_vocabulary: true,
        reversibility_never_implied_without_exact_or_regenerate: true,
        reversal_limits_always_disclosed: true,
        repair_vocabulary_shared_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RepairPreviewRollbackConsumerProjection {
    M5RepairPreviewRollbackConsumerProjection {
        repair_previews_expose_same_transaction_grammar: true,
        reversal_class_legible_without_docs: true,
        repair_state_traces_to_single_component_contract: true,
        support_export_reads_single_repair_source: true,
    }
}

fn proof_freshness() -> M5RepairPreviewRollbackProofFreshness {
    M5RepairPreviewRollbackProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RepairPreviewRollbackReleasePosture {
    M5RepairPreviewRollbackReleasePosture {
        proof_packet_ref: M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_REF,
        M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
        M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 repair-transaction-preview-card / rollback-class-strip controls packet.
pub fn seeded_m5_repair_preview_rollback_controls() -> M5RepairPreviewRollbackControlsPacket {
    M5RepairPreviewRollbackControlsPacket::new(M5RepairPreviewRollbackControlsPacketInput {
        packet_id: M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 repair-transaction-preview-card and rollback-class-strip controls with repair ids, linked findings, prerequisites, checkpoint state, reversal class, and local/remote/managed impact truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5RepairPreviewRollbackVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the Project-Doctor row is held at Beta pending repair-preview parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed(
) -> M5RepairPreviewRollbackControlsPacket {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.packet_id =
        "m5-repair-transaction-preview-card-rollback-class-strip-controls:doctor-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::DoctorUi)
        .expect("doctor-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Beta;
    packet
}

/// Narrowed variant: the safe-mode row is narrowed to Preview pending rollback-class-strip parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed(
) -> M5RepairPreviewRollbackControlsPacket {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.packet_id =
        "m5-repair-transaction-preview-card-rollback-class-strip-controls:safe-mode-ui-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .expect("safe-mode-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Preview;
    packet
}
