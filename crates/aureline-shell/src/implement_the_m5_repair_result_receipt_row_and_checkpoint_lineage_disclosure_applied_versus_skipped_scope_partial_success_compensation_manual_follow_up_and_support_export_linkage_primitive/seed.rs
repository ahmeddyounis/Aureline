//! Canonical seed builders for the M5 repair-result-receipt-row / checkpoint-lineage-disclosure
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_PACKET_ID: &str =
    "m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn receipt(input: M5RepairResultReceiptRowResolutionInput) -> M5ResolvedRepairResultReceiptRow {
    resolve_repair_result_receipt_row(input).expect("seed repair-result receipt input resolves")
}

fn lineage(
    input: M5CheckpointLineageDisclosureResolutionInput,
) -> M5ResolvedCheckpointLineageDisclosure {
    resolve_checkpoint_lineage_disclosure(input).expect("seed checkpoint-lineage input resolves")
}

// -- Canonical receipt inputs ------------------------------------------------------------------

/// A fully valid exact-reversal receipt input the builders mutate a single field of.
fn clean_receipt_input() -> M5RepairResultReceiptRowResolutionInput {
    M5RepairResultReceiptRowResolutionInput {
        receipt_id: "receipt:clean".to_owned(),
        repair_id: "repair:workspace-settings-rewrite".to_owned(),
        linked_finding_ids: vec!["finding:settings-drift".to_owned()],
        outcome_class: M5RepairOutcomeClass::RepairAppliedExact,
        applied_scope: vec!["file: .aureline/settings.json".to_owned()],
        skipped_scope: Vec::new(),
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_ref: "checkpoint:pre-settings-rewrite".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        follow_up_stated: true,
        reads_as_complete: true,
        reads_as_generic_success: false,
        support_export_available: true,
        proof_fresh: true,
    }
}

/// Clean receipt: an exact repair that applied and can be reversed exactly.
fn receipt_exact_clean() -> M5ResolvedRepairResultReceiptRow {
    receipt(clean_receipt_input())
}

/// Clean receipt: a first-class partial success naming applied and skipped scope, not read as
/// complete.
fn receipt_partial_clean() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:partial".to_owned();
    input.repair_id = "repair:dependency-realign".to_owned();
    input.linked_finding_ids = strings(&["finding:dep-a", "finding:dep-b"]);
    input.outcome_class = M5RepairOutcomeClass::RepairPartialSuccess;
    input.applied_scope = strings(&["dependency: lib-a"]);
    input.skipped_scope = strings(&["dependency: lib-b (locked)"]);
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.reads_as_complete = false;
    input.follow_up_stated = true;
    receipt(input)
}

/// Clean receipt: a compensated repair naming its compensation follow-up state.
fn receipt_compensated_clean() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:compensated".to_owned();
    input.repair_id = "repair:remote-config-realign".to_owned();
    input.linked_finding_ids = strings(&["finding:remote-config-drift"]);
    input.outcome_class = M5RepairOutcomeClass::RepairCompensated;
    input.applied_scope = strings(&["config: remote-host profile"]);
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.follow_up_stated = true;
    receipt(input)
}

/// Clean receipt: a manual-required repair leaving a stated follow-up.
fn receipt_manual_clean() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:manual".to_owned();
    input.repair_id = "repair:schema-migration".to_owned();
    input.linked_finding_ids = strings(&["finding:schema-outdated"]);
    input.outcome_class = M5RepairOutcomeClass::RepairManualRequired;
    input.applied_scope = strings(&["migration: staged"]);
    input.skipped_scope = strings(&["migration: finalize (needs approval)"]);
    input.reversal_class = M5RepairReversalClass::ManualFollowUp;
    input.reads_as_complete = false;
    input.follow_up_stated = true;
    receipt(input)
}

/// Clean receipt: a regenerated repair.
fn receipt_regenerated_clean() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:regenerated".to_owned();
    input.repair_id = "repair:index-rebuild".to_owned();
    input.linked_finding_ids = strings(&["finding:corrupt-index"]);
    input.outcome_class = M5RepairOutcomeClass::RepairRegenerated;
    input.applied_scope = strings(&["cache: search-index"]);
    input.reversal_class = M5RepairReversalClass::RegenerateReversal;
    receipt(input)
}

/// Clean receipt: a failed repair where nothing changed, kept honest with skipped scope.
fn receipt_failed_clean() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:failed".to_owned();
    input.repair_id = "repair:permissions-fix".to_owned();
    input.linked_finding_ids = strings(&["finding:permission-denied"]);
    input.outcome_class = M5RepairOutcomeClass::RepairFailed;
    input.applied_scope = Vec::new();
    input.skipped_scope = strings(&["path: /managed/root (read-only)"]);
    input.reversal_class = M5RepairReversalClass::AuditOnly;
    input.reads_as_complete = false;
    receipt(input)
}

/// Degraded receipt: distinct outcomes collapsed into a generic success.
fn receipt_generic_collapsed() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:generic-collapsed".to_owned();
    input.reads_as_generic_success = true;
    receipt(input)
}

/// Degraded receipt: a partial success read as complete.
fn receipt_partial_shown_complete() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:partial-shown-complete".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairPartialSuccess;
    input.applied_scope = strings(&["dependency: lib-a"]);
    input.skipped_scope = strings(&["dependency: lib-b"]);
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.follow_up_stated = true;
    input.reads_as_complete = true;
    receipt(input)
}

/// Degraded receipt: the checkpoint ref is unstated while a checkpoint is present.
fn receipt_checkpoint_ref_unstated() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:checkpoint-ref-unstated".to_owned();
    input.checkpoint_ref = "   ".to_owned();
    receipt(input)
}

/// Degraded receipt: the reversal class could not be resolved.
fn receipt_reversal_unresolved() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:reversal-unresolved".to_owned();
    input.reversal_class = M5RepairReversalClass::ReversalUnknown;
    receipt(input)
}

/// Degraded receipt: a required follow-up state is unstated.
fn receipt_follow_up_unstated() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:follow-up-unstated".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairManualRequired;
    input.applied_scope = strings(&["migration: staged"]);
    input.reversal_class = M5RepairReversalClass::ManualFollowUp;
    input.reads_as_complete = false;
    input.follow_up_stated = false;
    receipt(input)
}

/// Degraded receipt: no command-backed support-export path is reachable.
fn receipt_export_missing() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:export-missing".to_owned();
    input.support_export_available = false;
    receipt(input)
}

/// Degraded receipt: the repair id is unstated.
fn receipt_repair_id_unstated() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:repair-id-unstated".to_owned();
    input.repair_id = "  ".to_owned();
    receipt(input)
}

/// Degraded receipt: a non-failure outcome names no applied scope.
fn receipt_applied_scope_unstated() -> M5ResolvedRepairResultReceiptRow {
    let mut input = clean_receipt_input();
    input.receipt_id = "receipt:applied-scope-unstated".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairRegenerated;
    input.applied_scope = Vec::new();
    input.reversal_class = M5RepairReversalClass::RegenerateReversal;
    receipt(input)
}

// -- Canonical lineage inputs ------------------------------------------------------------------

/// A fully valid exact-outcome lineage input the builders mutate a single field of.
fn clean_lineage_input() -> M5CheckpointLineageDisclosureResolutionInput {
    M5CheckpointLineageDisclosureResolutionInput {
        disclosure_id: "lineage:clean".to_owned(),
        repair_id: "repair:workspace-settings-rewrite".to_owned(),
        linked_finding_ids: vec!["finding:settings-drift".to_owned()],
        preview_ref: "preview:settings-rewrite".to_owned(),
        checkpoint_ref: "checkpoint:pre-settings-rewrite".to_owned(),
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        apply_ref: "apply:settings-rewrite".to_owned(),
        receipt_ref: "receipt:clean".to_owned(),
        outcome_class: M5RepairOutcomeClass::RepairAppliedExact,
        reversal_class: M5RepairReversalClass::ExactReversal,
        reads_as_single_status: false,
        support_export_available: true,
        proof_fresh: true,
    }
}

/// Clean lineage: an exact repair traced finding to result.
fn lineage_exact_clean() -> M5ResolvedCheckpointLineageDisclosure {
    lineage(clean_lineage_input())
}

/// Clean lineage: a partial success traced finding to result.
fn lineage_partial_clean() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:partial".to_owned();
    input.repair_id = "repair:dependency-realign".to_owned();
    input.linked_finding_ids = strings(&["finding:dep-a", "finding:dep-b"]);
    input.preview_ref = "preview:dependency-realign".to_owned();
    input.checkpoint_ref = "checkpoint:pre-dep-realign".to_owned();
    input.apply_ref = "apply:dependency-realign".to_owned();
    input.receipt_ref = "receipt:partial".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairPartialSuccess;
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    lineage(input)
}

/// Clean lineage: a compensated repair traced finding to result.
fn lineage_compensate_clean() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:compensate".to_owned();
    input.repair_id = "repair:remote-config-realign".to_owned();
    input.linked_finding_ids = strings(&["finding:remote-config-drift"]);
    input.preview_ref = "preview:remote-config-realign".to_owned();
    input.checkpoint_ref = "checkpoint:pre-remote-config".to_owned();
    input.apply_ref = "apply:remote-config-realign".to_owned();
    input.receipt_ref = "receipt:compensated".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairCompensated;
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    lineage(input)
}

/// Clean lineage: a manual-required repair traced finding to result.
fn lineage_manual_clean() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:manual".to_owned();
    input.repair_id = "repair:schema-migration".to_owned();
    input.linked_finding_ids = strings(&["finding:schema-outdated"]);
    input.preview_ref = "preview:schema-migration".to_owned();
    input.checkpoint_ref = "checkpoint:pre-schema-migration".to_owned();
    input.apply_ref = "apply:schema-migration".to_owned();
    input.receipt_ref = "receipt:manual".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairManualRequired;
    input.reversal_class = M5RepairReversalClass::ManualFollowUp;
    lineage(input)
}

/// Clean lineage: a regenerated repair traced finding to result.
fn lineage_regenerated_clean() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:regenerated".to_owned();
    input.repair_id = "repair:index-rebuild".to_owned();
    input.linked_finding_ids = strings(&["finding:corrupt-index"]);
    input.preview_ref = "preview:index-rebuild".to_owned();
    input.checkpoint_ref = "checkpoint:pre-index-rebuild".to_owned();
    input.apply_ref = "apply:index-rebuild".to_owned();
    input.receipt_ref = "receipt:regenerated".to_owned();
    input.outcome_class = M5RepairOutcomeClass::RepairRegenerated;
    input.reversal_class = M5RepairReversalClass::RegenerateReversal;
    lineage(input)
}

/// Degraded lineage: the four stages collapsed into a single opaque status.
fn lineage_stages_collapsed() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:stages-collapsed".to_owned();
    input.reads_as_single_status = true;
    lineage(input)
}

/// Degraded lineage: the result (canonical receipt) ref is unstated.
fn lineage_result_unstated() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:result-unstated".to_owned();
    input.receipt_ref = "  ".to_owned();
    lineage(input)
}

/// Degraded lineage: the preview stage ref is unstated.
fn lineage_preview_unstated() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:preview-unstated".to_owned();
    input.preview_ref = "".to_owned();
    lineage(input)
}

/// Degraded lineage: the apply stage ref is unstated.
fn lineage_apply_unstated() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:apply-unstated".to_owned();
    input.apply_ref = "".to_owned();
    lineage(input)
}

/// Degraded lineage: no finding is linked; the lineage does not start at a finding.
fn lineage_finding_unstated() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:finding-unstated".to_owned();
    input.linked_finding_ids = Vec::new();
    lineage(input)
}

/// Degraded lineage: no command-backed support-export / lineage path is reachable.
fn lineage_path_missing() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:path-missing".to_owned();
    input.support_export_available = false;
    lineage(input)
}

/// Degraded lineage: the repair id is unstated.
fn lineage_repair_id_unstated() -> M5ResolvedCheckpointLineageDisclosure {
    let mut input = clean_lineage_input();
    input.disclosure_id = "lineage:repair-id-unstated".to_owned();
    input.repair_id = "".to_owned();
    lineage(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5RepairReceiptLineageConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    repair_result_receipt_row_examples: Vec<M5ResolvedRepairResultReceiptRow>,
    checkpoint_lineage_disclosure_examples: Vec<M5ResolvedCheckpointLineageDisclosure>,
) -> M5RepairReceiptLineageControlsRow {
    M5RepairReceiptLineageControlsRow {
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
        anatomy_parts: M5RepairReceiptLineageAnatomyPart::ALL.to_vec(),
        export_fields: M5RepairReceiptLineageExportField::ALL.to_vec(),
        downgrade_triggers,
        repair_result_receipt_row_examples,
        checkpoint_lineage_disclosure_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_REF,
            M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
        ]),
        collapses_outcomes_into_generic_success: false,
        hides_partial_success_or_follow_up: false,
        severs_receipt_from_checkpoint_lineage: false,
        requires_feature_local_translation_for_support_export: false,
    }
}

fn controls_rows() -> Vec<M5RepairReceiptLineageControlsRow> {
    use M5WorkspaceTrustRepairConsumerSurface as C;
    use M5WorkspaceTrustRepairDowngradeTrigger as D;

    vec![
        base_row(
            C::DoctorUi,
            "Project Doctor owner",
            "Project Doctor renders repair-result receipt rows for an exact success and a first-class partial success naming applied-versus-skipped scope, plus checkpoint-lineage disclosures tracing finding to preview to apply to result, and degrades honestly when an outcome collapses into a generic success, a partial success reads as complete, the lineage stages collapse into one status, or the canonical result ref is unstated",
            "evidence:m5-repair-receipt-lineage-doctor-ui:001",
            vec![
                D::ReversalClassCollapsedIntoGenericSuccess,
                D::PartialSuccessShownAsComplete,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                receipt_exact_clean(),
                receipt_partial_clean(),
                receipt_generic_collapsed(),
                receipt_partial_shown_complete(),
            ],
            vec![
                lineage_exact_clean(),
                lineage_partial_clean(),
                lineage_stages_collapsed(),
                lineage_result_unstated(),
            ],
        ),
        base_row(
            C::RemoteUi,
            "Remote workspace owner",
            "The remote workspace UI carries the compensated receipt with its stated compensation follow-up and a lineage traced finding to result, degrading honestly when the checkpoint ref is unstated or the lineage skips the preview stage",
            "evidence:m5-repair-receipt-lineage-remote-ui:001",
            vec![
                D::CheckpointAbsenceHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![receipt_compensated_clean(), receipt_checkpoint_ref_unstated()],
            vec![lineage_compensate_clean(), lineage_preview_unstated()],
        ),
        base_row(
            C::SafeModeUi,
            "Safe mode owner",
            "Safe mode shows the manual-required receipt leaving a stated follow-up and a lineage traced finding to result, degrading honestly when the manual follow-up state is unstated or the lineage skips the apply stage",
            "evidence:m5-repair-receipt-lineage-safe-mode-ui:001",
            vec![
                D::PartialSuccessShownAsComplete,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![receipt_manual_clean(), receipt_follow_up_unstated()],
            vec![lineage_manual_clean(), lineage_apply_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved receipt and lineage truth, so a receipt with no command-backed support-export path, an unresolved reversal class, a missing lineage path, or an unlinked finding is visible in evidence rather than hidden behind feature-local translation",
            "evidence:m5-repair-receipt-lineage-support-export:001",
            vec![
                D::GenericChromeWordingUsed,
                D::RepairTargetIdsUnstated,
                D::ProofStale,
            ],
            vec![receipt_export_missing(), receipt_reversal_unresolved()],
            vec![lineage_path_missing(), lineage_finding_unstated()],
        ),
        base_row(
            C::ProductUi,
            "In-product repair owner",
            "In-product surfaces reuse the same receipt and lineage grammar the Doctor UI shows for a regenerated and a failed outcome, keeping failure honest with skipped scope, and degrading honestly when the repair id or the applied scope of a non-failure outcome is unstated",
            "evidence:m5-repair-receipt-lineage-product-ui:001",
            vec![
                D::RepairTargetIdsUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                receipt_regenerated_clean(),
                receipt_failed_clean(),
                receipt_repair_id_unstated(),
                receipt_applied_scope_unstated(),
            ],
            vec![lineage_regenerated_clean(), lineage_repair_id_unstated()],
        ),
    ]
}

fn governance_review() -> M5RepairReceiptLineageGovernanceReview {
    M5RepairReceiptLineageGovernanceReview {
        receipt_names_repair_id_and_linked_findings: true,
        receipt_shows_applied_versus_skipped_scope: true,
        partial_success_and_follow_up_visible_first_class: true,
        outcome_never_collapsed_into_generic_success: true,
        checkpoint_lineage_traceable_finding_to_result: true,
        receipt_joins_support_export_without_local_translation: true,
        receipt_vocabulary_shared_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5RepairReceiptLineageConsumerProjection {
    M5RepairReceiptLineageConsumerProjection {
        receipts_expose_same_outcome_grammar: true,
        partial_success_legible_without_logs: true,
        repair_result_traces_to_single_component_contract: true,
        support_export_reads_single_receipt_lineage_source: true,
    }
}

fn proof_freshness() -> M5RepairReceiptLineageProofFreshness {
    M5RepairReceiptLineageProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RepairReceiptLineageReleasePosture {
    M5RepairReceiptLineageReleasePosture {
        proof_packet_ref: M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_REF,
        M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 repair-result-receipt-row / checkpoint-lineage-disclosure controls packet.
pub fn seeded_m5_repair_receipt_lineage_controls() -> M5RepairReceiptLineageControlsPacket {
    M5RepairReceiptLineageControlsPacket::new(M5RepairReceiptLineageControlsPacketInput {
        packet_id: M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 repair-result receipt rows and checkpoint-lineage disclosures with applied-versus-skipped scope, partial-success markers, compensation and manual follow-up state, and support-export linkage"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5RepairReceiptLineageVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the Doctor-UI row is held at Beta pending lineage parity on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_repair_receipt_lineage_controls_doctor_ui_beta_narrowed(
) -> M5RepairReceiptLineageControlsPacket {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.packet_id =
        "m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls:doctor-ui-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::DoctorUi)
        .expect("doctor-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Beta;
    packet
}

/// Narrowed variant: the safe-mode-UI row is narrowed to Preview pending follow-up parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_repair_receipt_lineage_controls_safe_mode_ui_preview_narrowed(
) -> M5RepairReceiptLineageControlsPacket {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.packet_id =
        "m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls:safe-mode-ui-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .expect("safe-mode-ui row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Preview;
    packet
}
