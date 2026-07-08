use super::*;

const PACKET_ID: &str = "git-history-component-accessibility:stable:0001";

fn trust_review() -> GitHistoryComponentAccessibilityTrustReview {
    GitHistoryComponentAccessibilityTrustReview {
        keyboard_reachable_on_every_claim: true,
        screen_reader_labeled_on_every_claim: true,
        cli_enum_exposed_on_every_claim: true,
        export_enum_exposed_on_every_claim: true,
        explanation_field_present_on_every_claim: true,
        no_component_pointer_only: true,
        no_component_export_opaque: true,
        desktop_never_stronger_than_cli: true,
        claim_narrows_when_topology_or_recovery_or_provider_degrades: true,
        recovery_or_mutation_safety_never_overstated_under_weakening: true,
        recovery_destination_kept_explicit: true,
        local_continue_preserved_under_degraded_truth: true,
    }
}

fn projection() -> GitHistoryComponentAccessibilityProjection {
    GitHistoryComponentAccessibilityProjection {
        exposes_keyboard_and_screen_reader_labels: true,
        exposes_cli_and_export_enums: true,
        exposes_explanation_fields: true,
        auto_narrows_on_partial_repo_topology: true,
        auto_narrows_on_unavailable_checkpoint_recovery: true,
        auto_narrows_on_stale_provider_review_state: true,
        auto_narrows_on_offline_local_only: true,
        desktop_cli_export_semantics_identical: true,
        narrowing_prevents_overstated_recovery_or_mutation_safety: true,
        every_component_reachable_non_visually: true,
    }
}

fn proof_freshness() -> GitHistoryComponentAccessibilityProofFreshness {
    GitHistoryComponentAccessibilityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<GitHistoryAccessibilityDowngradeTrigger> {
    vec![
        GitHistoryAccessibilityDowngradeTrigger::ProofStale,
        GitHistoryAccessibilityDowngradeTrigger::ProviderReviewStateStale,
        GitHistoryAccessibilityDowngradeTrigger::RepoTopologyPartial,
        GitHistoryAccessibilityDowngradeTrigger::CheckpointRecoveryUnavailable,
        GitHistoryAccessibilityDowngradeTrigger::OfflineLocalOnly,
        GitHistoryAccessibilityDowngradeTrigger::ClaimOverstated,
    ]
}

fn rendering_surfaces() -> Vec<GitHistoryRenderingSurface> {
    GitHistoryRenderingSurface::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GIT_HISTORY_ACCESSIBILITY_SCHEMA_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_DOC_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_CONSUMER_CONTRACT_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_IDENTITY_CONTRACT_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_STASH_RECOVERY_CONTRACT_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_SEQUENCE_EDIT_CONTRACT_REF.to_owned(),
        GIT_HISTORY_ACCESSIBILITY_MUTATION_REVIEW_CONTRACT_REF.to_owned(),
    ]
}

fn row_refs(component: M5GitHistoryComponent) -> Vec<String> {
    vec![
        GIT_HISTORY_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_schema_ref(component).to_owned(),
    ]
}

fn human_component(component: M5GitHistoryComponent) -> &'static str {
    match component {
        M5GitHistoryComponent::CommitGraphHeader => "Commit graph header",
        M5GitHistoryComponent::HistoryGraphRow => "History graph row",
        M5GitHistoryComponent::BranchComparisonChip => "Branch comparison chip",
        M5GitHistoryComponent::WorktreeRow => "Worktree row",
        M5GitHistoryComponent::StashEntry => "Stash entry",
        M5GitHistoryComponent::ReflogRecoveryBanner => "Reflog recovery banner",
        M5GitHistoryComponent::RebaseTodoRow => "Rebase todo row",
        M5GitHistoryComponent::SequenceEditorHeader => "Sequence editor header",
        M5GitHistoryComponent::CherryPickRevertReviewSheet => "Cherry-pick / revert review sheet",
        M5GitHistoryComponent::PatchApplyReviewSheet => "Patch-apply review sheet",
        M5GitHistoryComponent::ConflictCheckpointCard => "Conflict checkpoint card",
        M5GitHistoryComponent::ForcePushReviewDialog => "Force-push review dialog",
    }
}

fn claim_phrase(tier: GitHistoryClaimTier) -> &'static str {
    match tier {
        GitHistoryClaimTier::RecoverableInProduct => {
            "safe in-product history surgery with recoverable depth"
        }
        GitHistoryClaimTier::LocallyRecoverable => {
            "recoverable from local checkpoints while provider review state is stale"
        }
        GitHistoryClaimTier::PartialHistoryOnly => {
            "showing partial history only; full depth is not loaded here"
        }
        GitHistoryClaimTier::ReflogOnlyRecovery => {
            "recoverable only through the reflog fallback; no checkpoint exists"
        }
        GitHistoryClaimTier::LocalContinueOnly => {
            "local-only; publish and approval parity cannot be claimed offline"
        }
    }
}

fn condition_phrase(condition: GitHistoryClaimCondition) -> &'static str {
    match condition {
        GitHistoryClaimCondition::LocalTruthAligned => "local Git truth is aligned",
        GitHistoryClaimCondition::ProviderReviewStateStale => {
            "provider-linked review state is stale"
        }
        GitHistoryClaimCondition::RepoTopologyPartial => {
            "repo topology is shallow, partial, or sparse"
        }
        GitHistoryClaimCondition::CheckpointRecoveryUnavailable => {
            "no checkpoint exists and only reflog recovery remains"
        }
        GitHistoryClaimCondition::OfflineLocalOnly => "the surface is offline / local-only",
    }
}

fn next_action_label(action: GitHistoryClaimNextAction) -> String {
    match action {
        GitHistoryClaimNextAction::ReconcileProviderReviewState => {
            "Reconcile the provider-linked review state before claiming parity".to_owned()
        }
        GitHistoryClaimNextAction::CompleteRepoTopology => {
            "Unshallow or fetch to restore the full history depth".to_owned()
        }
        GitHistoryClaimNextAction::OpenRecoveryCheckpoint => {
            "Open the reflog recovery destination to restore this work".to_owned()
        }
        GitHistoryClaimNextAction::ContinueLocalHistory => {
            "Continue the history work locally while offline".to_owned()
        }
    }
}

/// Builds one accessibility row, deriving the claim, narrowing, notes, and labels from
/// the component and condition so the fixture stays self-consistent.
fn row(
    row_id: &str,
    component: M5GitHistoryComponent,
    condition: GitHistoryClaimCondition,
) -> GitHistoryComponentAccessibilityRow {
    let resolution = resolve_git_history_component_claim_narrowing(condition);
    let effective_claim = resolution.permitted_ceiling;

    let narrowing = if resolution.requires_narrowing {
        Some(GitHistoryComponentClaimNarrowing {
            trigger: resolution
                .expected_trigger
                .expect("weakening condition has a trigger"),
            narrowed_to: resolution.permitted_ceiling,
            preserved_truth_note: format!(
                "{} stays keyboard-reachable, screen-reader labelled, and export-legible; only the recovery / mutation-safety claim is narrowed",
                human_component(component)
            ),
            next_action: resolution.expected_next_action,
            next_action_label: next_action_label(resolution.expected_next_action),
        })
    } else {
        None
    };

    let topology_note = if resolution.needs_topology_note {
        format!(
            "The {} shows partial history only; run the next action to load the full depth",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };
    let recovery_note = if resolution.needs_recovery_note {
        format!(
            "The {} keeps its reflog recovery destination named; no checkpoint exists yet",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };
    let local_continue_note = if resolution.needs_local_continue_note {
        format!(
            "Your local history work on the {} continues here even without provider-linked state",
            human_component(component).to_lowercase()
        )
    } else {
        String::new()
    };

    GitHistoryComponentAccessibilityRow {
        row_id: row_id.to_owned(),
        component,
        condition,
        effective_claim,
        keyboard_label: format!(
            "{}: focusable, Enter opens, Space toggles detail",
            human_component(component)
        ),
        screen_reader_label: format!(
            "{}, {}",
            human_component(component),
            claim_phrase(effective_claim)
        ),
        cli_enum_token: format!("{}:{}", component.as_str(), effective_claim.as_str()),
        export_enum_token: effective_claim.as_str().to_owned(),
        explanation_field: format!(
            "{} — {}",
            claim_phrase(effective_claim),
            condition_phrase(condition)
        ),
        rendering_surfaces: rendering_surfaces(),
        narrowing,
        topology_note,
        recovery_note,
        local_continue_note,
        is_pointer_only: false,
        is_export_opaque: false,
        desktop_stronger_than_cli: false,
        source_contract_refs: row_refs(component),
    }
}

/// The canonical row set: all twelve components, covering all five conditions and all
/// five claim tiers.
fn accessibility_rows() -> Vec<GitHistoryComponentAccessibilityRow> {
    vec![
        row(
            "row:commit-graph-aligned",
            M5GitHistoryComponent::CommitGraphHeader,
            GitHistoryClaimCondition::LocalTruthAligned,
        ),
        row(
            "row:history-graph-topology-partial",
            M5GitHistoryComponent::HistoryGraphRow,
            GitHistoryClaimCondition::RepoTopologyPartial,
        ),
        row(
            "row:branch-comparison-provider-stale",
            M5GitHistoryComponent::BranchComparisonChip,
            GitHistoryClaimCondition::ProviderReviewStateStale,
        ),
        row(
            "row:worktree-aligned",
            M5GitHistoryComponent::WorktreeRow,
            GitHistoryClaimCondition::LocalTruthAligned,
        ),
        row(
            "row:stash-aligned",
            M5GitHistoryComponent::StashEntry,
            GitHistoryClaimCondition::LocalTruthAligned,
        ),
        row(
            "row:reflog-checkpoint-unavailable",
            M5GitHistoryComponent::ReflogRecoveryBanner,
            GitHistoryClaimCondition::CheckpointRecoveryUnavailable,
        ),
        row(
            "row:rebase-todo-topology-partial",
            M5GitHistoryComponent::RebaseTodoRow,
            GitHistoryClaimCondition::RepoTopologyPartial,
        ),
        row(
            "row:sequence-editor-provider-stale",
            M5GitHistoryComponent::SequenceEditorHeader,
            GitHistoryClaimCondition::ProviderReviewStateStale,
        ),
        row(
            "row:cherry-pick-offline",
            M5GitHistoryComponent::CherryPickRevertReviewSheet,
            GitHistoryClaimCondition::OfflineLocalOnly,
        ),
        row(
            "row:patch-apply-checkpoint-unavailable",
            M5GitHistoryComponent::PatchApplyReviewSheet,
            GitHistoryClaimCondition::CheckpointRecoveryUnavailable,
        ),
        row(
            "row:conflict-checkpoint-offline",
            M5GitHistoryComponent::ConflictCheckpointCard,
            GitHistoryClaimCondition::OfflineLocalOnly,
        ),
        row(
            "row:force-push-provider-stale",
            M5GitHistoryComponent::ForcePushReviewDialog,
            GitHistoryClaimCondition::ProviderReviewStateStale,
        ),
    ]
}

fn packet_with(
    rows: Vec<GitHistoryComponentAccessibilityRow>,
) -> GitHistoryComponentAccessibilityPacket {
    GitHistoryComponentAccessibilityPacket::new(GitHistoryComponentAccessibilityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Git-history component accessibility, headless, and export parity"
            .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: downgrade_triggers(),
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn packet() -> GitHistoryComponentAccessibilityPacket {
    packet_with(accessibility_rows())
}

#[test]
fn accessibility_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_canonical_row_is_honest() {
    for row in accessibility_rows() {
        assert!(row.claim_is_honest(), "row not honest: {}", row.row_id);
    }
}

#[test]
fn claim_narrowing_maps_condition_to_ceiling() {
    let aligned =
        resolve_git_history_component_claim_narrowing(GitHistoryClaimCondition::LocalTruthAligned);
    assert_eq!(
        aligned.permitted_ceiling,
        GitHistoryClaimTier::RecoverableInProduct
    );
    assert!(!aligned.requires_narrowing);
    assert!(aligned.expected_trigger.is_none());
    assert!(!aligned.needs_local_continue_note);
    assert!(!aligned.needs_topology_note);
    assert!(!aligned.needs_recovery_note);

    let provider = resolve_git_history_component_claim_narrowing(
        GitHistoryClaimCondition::ProviderReviewStateStale,
    );
    assert_eq!(
        provider.permitted_ceiling,
        GitHistoryClaimTier::LocallyRecoverable
    );
    assert!(provider.requires_narrowing);
    assert!(provider.needs_local_continue_note);
    assert!(!provider.needs_topology_note);

    let topology = resolve_git_history_component_claim_narrowing(
        GitHistoryClaimCondition::RepoTopologyPartial,
    );
    assert_eq!(
        topology.permitted_ceiling,
        GitHistoryClaimTier::PartialHistoryOnly
    );
    assert_eq!(
        topology.expected_trigger,
        Some(GitHistoryAccessibilityDowngradeTrigger::RepoTopologyPartial)
    );
    assert!(topology.needs_topology_note);

    let checkpoint = resolve_git_history_component_claim_narrowing(
        GitHistoryClaimCondition::CheckpointRecoveryUnavailable,
    );
    assert_eq!(
        checkpoint.permitted_ceiling,
        GitHistoryClaimTier::ReflogOnlyRecovery
    );
    assert!(checkpoint.needs_recovery_note);

    let offline =
        resolve_git_history_component_claim_narrowing(GitHistoryClaimCondition::OfflineLocalOnly);
    assert_eq!(
        offline.permitted_ceiling,
        GitHistoryClaimTier::LocalContinueOnly
    );
    assert!(offline.needs_local_continue_note);
}

// --- AC2: narrowing prevents overstated recovery / mutation safety ------------

#[test]
fn recoverable_claim_never_survives_a_weakening_condition() {
    // A component that keeps asserting full recoverable-in-product safety while its
    // repo topology is partial overstates its truth and must be caught.
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::RepoTopologyPartial)
        .expect("topology-partial row present");
    packet.accessibility_rows[index].effective_claim = GitHistoryClaimTier::RecoverableInProduct;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn claim_ceiling_exceeded_on_checkpoint_unavailable_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::CheckpointRecoveryUnavailable)
        .expect("checkpoint-unavailable row present");
    // Claim partial-history (rank 3) above the reflog-only-recovery ceiling (rank 2).
    packet.accessibility_rows[index].effective_claim = GitHistoryClaimTier::PartialHistoryOnly;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ClaimCeilingExceeded));
}

#[test]
fn weakening_condition_without_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].narrowing = None;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ClaimNarrowingMissing));
}

#[test]
fn baseline_condition_with_narrowing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::LocalTruthAligned)
        .expect("aligned row present");
    packet.accessibility_rows[index].narrowing = Some(GitHistoryComponentClaimNarrowing {
        trigger: GitHistoryAccessibilityDowngradeTrigger::RepoTopologyPartial,
        narrowed_to: GitHistoryClaimTier::RecoverableInProduct,
        preserved_truth_note: "note".to_owned(),
        next_action: GitHistoryClaimNextAction::CompleteRepoTopology,
        next_action_label: "Fetch".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ClaimNarrowingUnexpected));
}

#[test]
fn narrowed_to_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.narrowed_to = GitHistoryClaimTier::LocalContinueOnly;
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::NarrowedToMismatch));
}

#[test]
fn narrow_trigger_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::RepoTopologyPartial)
        .expect("topology-partial row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.trigger = GitHistoryAccessibilityDowngradeTrigger::OfflineLocalOnly;
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::NarrowTriggerMismatch));
}

#[test]
fn narrow_next_action_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::CheckpointRecoveryUnavailable)
        .expect("checkpoint-unavailable row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action = GitHistoryClaimNextAction::ContinueLocalHistory;
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::NarrowNextActionMismatch));
}

#[test]
fn narrow_missing_preserved_truth_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.preserved_truth_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::NarrowPreservedTruthMissing));
}

#[test]
fn narrow_missing_next_action_label_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    if let Some(narrowing) = packet.accessibility_rows[index].narrowing.as_mut() {
        narrowing.next_action_label = String::new();
    }
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::NarrowNextActionMissing));
}

// --- AC1: parity across keyboard / screen-reader / CLI / export ---------------

#[test]
fn keyboard_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].keyboard_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::KeyboardLabelMissing));
}

#[test]
fn screen_reader_label_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].screen_reader_label = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ScreenReaderLabelMissing));
}

#[test]
fn cli_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].cli_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::CliEnumTokenMissing));
}

#[test]
fn export_enum_token_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].export_enum_token = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ExportEnumTokenMissing));
}

#[test]
fn explanation_field_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].explanation_field = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ExplanationFieldMissing));
}

#[test]
fn rendering_surface_coverage_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].rendering_surfaces = vec![GitHistoryRenderingSurface::DesktopFull];
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::RenderingSurfaceCoverageMissing));
}

#[test]
fn pointer_only_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_pointer_only = true;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::PointerOnlyComponent));
}

#[test]
fn export_opaque_component_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].is_export_opaque = true;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ExportOpaqueComponent));
}

#[test]
fn desktop_stronger_than_cli_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].desktop_stronger_than_cli = true;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::DesktopStrongerThanCli));
}

#[test]
fn topology_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::RepoTopologyPartial)
        .expect("topology-partial row present");
    packet.accessibility_rows[index].topology_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::TopologyNoteMissing));
}

#[test]
fn recovery_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.condition == GitHistoryClaimCondition::CheckpointRecoveryUnavailable)
        .expect("checkpoint-unavailable row present");
    packet.accessibility_rows[index].recovery_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::RecoveryNoteMissing));
}

#[test]
fn local_continue_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .accessibility_rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("narrowed row present");
    packet.accessibility_rows[index].local_continue_note = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::LocalContinueNoteMissing));
}

#[test]
fn canonical_contract_reference_missing_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].source_contract_refs =
        vec![GIT_HISTORY_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::CanonicalContractReferenceMissing));
}

// --- Coverage -----------------------------------------------------------------

#[test]
fn missing_component_coverage_fails() {
    let mut rows = accessibility_rows();
    rows.retain(|r| r.component != M5GitHistoryComponent::ForcePushReviewDialog);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ComponentCoverageMissing));
}

#[test]
fn missing_condition_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only offline / local-only rows.
    rows.retain(|r| r.condition != GitHistoryClaimCondition::OfflineLocalOnly);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ConditionCoverageMissing));
}

#[test]
fn missing_claim_tier_coverage_fails() {
    let mut rows = accessibility_rows();
    // Drop the only local-continue-only rows; that tier is then unreachable.
    rows.retain(|r| r.effective_claim != GitHistoryClaimTier::LocalContinueOnly);
    let packet = packet_with(rows);
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ClaimTierCoverageMissing));
}

// --- Structural ---------------------------------------------------------------

#[test]
fn row_incomplete_fails() {
    let mut packet = packet();
    packet.accessibility_rows[0].row_id = String::new();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::RowIncomplete));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.accessibility_rows.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::AccessibilityRowsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .recovery_or_mutation_safety_never_overstated_under_weakening = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::TrustReviewIncomplete));
}

#[test]
fn projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .projection
        .narrowing_prevents_overstated_recovery_or_mutation_safety = false;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GitHistoryComponentAccessibilityViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Accessibility rows"));
    assert!(summary.contains("commit_graph_header"));
    assert!(summary.contains("force_push_review_dialog"));
    assert!(summary.contains("reflog_only_recovery"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_git_history_component_accessibility_export()
        .expect("checked git-history component accessibility export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-git-history-component-accessibility-parity/repo_topology_partial_and_checkpoint_unavailable_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-git-history-component-accessibility-parity/provider_review_stale_and_offline_local_only_narrowed.json"
        )),
    ] {
        let packet: GitHistoryComponentAccessibilityPacket = serde_json::from_str(raw)
            .expect("fixture parses as git-history component accessibility packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// The canonical rows plus extra scenario rows that demonstrate a normally-aligned
/// component auto-narrowing under partial topology and an unavailable checkpoint. The
/// base rows keep full component / condition / tier coverage; the extra rows show the
/// narrowing.
fn fixture_repo_topology_partial_and_checkpoint_unavailable_narrowed(
) -> GitHistoryComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:commit-graph-topology-partial-narrowed",
        M5GitHistoryComponent::CommitGraphHeader,
        GitHistoryClaimCondition::RepoTopologyPartial,
    ));
    rows.push(row(
        "row:worktree-checkpoint-unavailable-narrowed",
        M5GitHistoryComponent::WorktreeRow,
        GitHistoryClaimCondition::CheckpointRecoveryUnavailable,
    ));
    GitHistoryComponentAccessibilityPacket::new(GitHistoryComponentAccessibilityPacketInput {
        packet_id:
            "git-history-component-accessibility:fixture:repo-topology-partial-and-checkpoint-unavailable-narrowed"
                .to_owned(),
        surface_label:
            "Git-history component accessibility: repo topology partial and checkpoint recovery unavailable, claim auto-narrowed"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            GitHistoryAccessibilityDowngradeTrigger::RepoTopologyPartial,
            GitHistoryAccessibilityDowngradeTrigger::CheckpointRecoveryUnavailable,
            GitHistoryAccessibilityDowngradeTrigger::ClaimOverstated,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// The canonical rows plus extra scenario rows for a branch-comparison chip losing
/// provider-linked review state and a force-push dialog dropping to offline / local-only.
fn fixture_provider_review_stale_and_offline_local_only_narrowed(
) -> GitHistoryComponentAccessibilityPacket {
    let mut rows = accessibility_rows();
    rows.push(row(
        "row:commit-graph-provider-stale-narrowed",
        M5GitHistoryComponent::CommitGraphHeader,
        GitHistoryClaimCondition::ProviderReviewStateStale,
    ));
    rows.push(row(
        "row:force-push-offline-narrowed",
        M5GitHistoryComponent::ForcePushReviewDialog,
        GitHistoryClaimCondition::OfflineLocalOnly,
    ));
    GitHistoryComponentAccessibilityPacket::new(GitHistoryComponentAccessibilityPacketInput {
        packet_id:
            "git-history-component-accessibility:fixture:provider-review-stale-and-offline-local-only-narrowed"
                .to_owned(),
        surface_label:
            "Git-history component accessibility: provider review state stale and offline / local-only, claim auto-narrowed"
                .to_owned(),
        accessibility_rows: rows,
        downgrade_triggers: vec![
            GitHistoryAccessibilityDowngradeTrigger::ProviderReviewStateStale,
            GitHistoryAccessibilityDowngradeTrigger::OfflineLocalOnly,
        ],
        rendering_surfaces: rendering_surfaces(),
        trust_review: trust_review(),
        projection: projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_GIT_HISTORY_ACCESSIBILITY_ARTIFACTS` so it never writes during a
/// normal test run. Run with the env var set to refresh the artifacts after a contract
/// change, then review the diff.
#[test]
fn regenerate_git_history_component_accessibility_artifacts() {
    if std::env::var("GEN_GIT_HISTORY_ACCESSIBILITY_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir =
        format!("{root}/artifacts/release/m5-git-history-component-accessibility-proof");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{artifact_dir}/summary.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = format!("{root}/fixtures/ui/m5-git-history-component-accessibility-parity");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "repo_topology_partial_and_checkpoint_unavailable_narrowed.json",
            fixture_repo_topology_partial_and_checkpoint_unavailable_narrowed(),
        ),
        (
            "provider_review_stale_and_offline_local_only_narrowed.json",
            fixture_provider_review_stale_and_offline_local_only_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
