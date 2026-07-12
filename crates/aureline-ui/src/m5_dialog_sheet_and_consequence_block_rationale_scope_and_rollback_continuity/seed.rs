//! Canonical seed builders for the M5 dialog / consequence controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean dialogs
//! and consequence blocks are built so the shared dialog-action, rationale/scope/explicit-action,
//! safe-focus/cancel/focus-return, and blast-radius/rollback grammar is proven across review,
//! settings, update/install, repair, shell, and support surfaces without any generic Yes/No ambiguity,
//! rationale-less confirmation, unsafe focus, broken focus return, unnamed blast radius, or
//! screenshot-only consequence.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_PACKET_ID: &str =
    "m5-dialog-sheet-and-consequence-block-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn dialog(input: M5DialogResolutionInput) -> M5ResolvedDialog {
    resolve_dialog(input).expect("seed dialog input resolves")
}

fn consequence(input: M5ConsequenceResolutionInput) -> M5ResolvedConsequence {
    resolve_consequence(input).expect("seed consequence input resolves")
}

// -- Clean dialog examples (action-model grammar across surfaces) --------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_dialog_base(
    dialog_id: &str,
    title: &str,
    action_model: M5DialogActionModel,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5DecisionActionSurfaceContext,
    focus_target: M5DialogFocusTarget,
    reopen_origin: M5DialogReopenOrigin,
) -> M5DialogResolutionInput {
    M5DialogResolutionInput {
        dialog_id: dialog_id.to_owned(),
        dialog_title: title.to_owned(),
        action_model,
        disposition,
        surface_context: surface,
        focus_target,
        reopen_origin,
        rationale_present: true,
        scope_named: true,
        actions_explicitly_named: true,
        initial_focus_is_safe: true,
        cancel_path_present: true,
        focus_returns_on_reopen: true,
        help_or_docs_hook_present: true,
        proof_fresh: true,
    }
}

/// Clean review-confirmation dialog naming specific actions.
fn dialog_review_named() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:review:approve",
        "Approve this change set?",
        M5DialogActionModel::NamedSpecificActions,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::ReviewConfirmation,
        M5DialogFocusTarget::FocusesLeastDestructiveAction,
        M5DialogReopenOrigin::FreshInvocation,
    ))
}

/// Clean settings dialog with a primary action plus an explicit cancel.
fn dialog_settings_primary_cancel() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:settings:grant",
        "Grant this capability?",
        M5DialogActionModel::PrimaryAndCancel,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5DialogFocusTarget::FocusesCancelControl,
        M5DialogReopenOrigin::ReopenedFromStatus,
    ))
}

/// Clean update / install dialog naming its destructive confirm.
fn dialog_updates_destructive_named() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:updates:replace",
        "Replace the installed version?",
        M5DialogActionModel::DestructiveConfirmNamed,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::UpdateOrInstall,
        M5DialogFocusTarget::FocusesCancelControl,
        M5DialogReopenOrigin::ReopenedFromActivityCenter,
    ))
}

/// Clean repair dialog stating rationale and scope.
fn dialog_support_rationale_scope() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:support:repair",
        "Run the repair routine?",
        M5DialogActionModel::RationaleAndScopeStated,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::RepairConfirmation,
        M5DialogFocusTarget::FocusesRationaleHeading,
        M5DialogReopenOrigin::ReopenedFromSupport,
    ))
}

/// Clean shell trust dialog that is dismissible with a safe default.
fn dialog_shell_dismissible_safe() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:shell:trust",
        "Trust this workspace?",
        M5DialogActionModel::DismissibleSafe,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5DialogFocusTarget::FocusesNamedPrimaryAction,
        M5DialogReopenOrigin::ReopenedFromDeepLink,
    ))
}

/// Clean support-export delete dialog naming specific actions (used by the support-export row).
fn dialog_support_export_named() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:support:delete",
        "Delete this saved bundle?",
        M5DialogActionModel::NamedSpecificActions,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::DestructiveDelete,
        M5DialogFocusTarget::FocusesLeastDestructiveAction,
        M5DialogReopenOrigin::FreshInvocation,
    ))
}

// -- Degraded dialog examples -------------------------------------------------------------------

/// Degraded dialog: it uses the disallowed generic-yes-no action model.
fn dialog_generic_yes_no() -> M5ResolvedDialog {
    dialog(clean_dialog_base(
        "dialog:review:generic",
        "Are you sure?",
        M5DialogActionModel::GenericYesNoDisallowed,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::ReviewConfirmation,
        M5DialogFocusTarget::FocusesCancelControl,
        M5DialogReopenOrigin::FreshInvocation,
    ))
}

/// Degraded dialog: the rationale is unstated.
fn dialog_rationale_missing() -> M5ResolvedDialog {
    let mut input = clean_dialog_base(
        "dialog:settings:no-rationale",
        "Grant this capability?",
        M5DialogActionModel::NamedSpecificActions,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5DialogFocusTarget::FocusesCancelControl,
        M5DialogReopenOrigin::ReopenedFromStatus,
    );
    input.rationale_present = false;
    dialog(input)
}

/// Degraded dialog: the named scope is unstated.
fn dialog_scope_missing() -> M5ResolvedDialog {
    let mut input = clean_dialog_base(
        "dialog:updates:no-scope",
        "Replace the installed version?",
        M5DialogActionModel::DestructiveConfirmNamed,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::UpdateOrInstall,
        M5DialogFocusTarget::FocusesCancelControl,
        M5DialogReopenOrigin::ReopenedFromActivityCenter,
    );
    input.scope_named = false;
    dialog(input)
}

/// Degraded dialog: the initial focus is unsafe.
fn dialog_safe_focus_missing() -> M5ResolvedDialog {
    let mut input = clean_dialog_base(
        "dialog:support:unsafe-focus",
        "Run the repair routine?",
        M5DialogActionModel::RationaleAndScopeStated,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::RepairConfirmation,
        M5DialogFocusTarget::FocusesNamedPrimaryAction,
        M5DialogReopenOrigin::ReopenedFromSupport,
    );
    input.initial_focus_is_safe = false;
    dialog(input)
}

/// Degraded dialog: the cancel / escape path is missing.
fn dialog_cancel_missing() -> M5ResolvedDialog {
    let mut input = clean_dialog_base(
        "dialog:shell:no-cancel",
        "Trust this workspace?",
        M5DialogActionModel::PrimaryAndCancel,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5DialogFocusTarget::FocusesNamedPrimaryAction,
        M5DialogReopenOrigin::ReopenedFromDeepLink,
    );
    input.cancel_path_present = false;
    dialog(input)
}

/// Degraded dialog: focus does not return to the invoker when reopened.
fn dialog_focus_return_broken() -> M5ResolvedDialog {
    let mut input = clean_dialog_base(
        "dialog:support:no-focus-return",
        "Delete this saved bundle?",
        M5DialogActionModel::NamedSpecificActions,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::DestructiveDelete,
        M5DialogFocusTarget::FocusesLeastDestructiveAction,
        M5DialogReopenOrigin::ReopenedFromStatus,
    );
    input.focus_returns_on_reopen = false;
    dialog(input)
}

// -- Clean consequence examples ------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_consequence_base(
    consequence_id: &str,
    label: &str,
    disclosure: M5ConsequenceDisclosure,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5DecisionActionSurfaceContext,
    blast_radius: M5ConsequenceBlastRadius,
    reversibility: M5ConsequenceReversibility,
) -> M5ConsequenceResolutionInput {
    M5ConsequenceResolutionInput {
        consequence_id: consequence_id.to_owned(),
        consequence_label: label.to_owned(),
        disclosure,
        disposition,
        surface_context: surface,
        blast_radius,
        reversibility,
        affected_object_named: true,
        blast_radius_named: true,
        rollback_or_help_posture_stated: true,
        partial_or_irreversible_noted: true,
        avoids_generic_yes_no: true,
        explanation_reachable_by_keyboard_sr_export: true,
        proof_fresh: true,
    }
}

/// Clean review consequence naming its blast radius.
fn consequence_review_blast() -> M5ResolvedConsequence {
    consequence(clean_consequence_base(
        "consequence:review:blast",
        "3 approved files will be rewritten",
        M5ConsequenceDisclosure::NamedBlastRadius,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::ReviewConfirmation,
        M5ConsequenceBlastRadius::MultipleObjects,
        M5ConsequenceReversibility::RollbackWithNamedSteps,
    ))
}

/// Clean settings consequence stating rollback is available.
fn consequence_settings_rollback() -> M5ResolvedConsequence {
    consequence(clean_consequence_base(
        "consequence:settings:rollback",
        "This capability grant can be revoked in Settings",
        M5ConsequenceDisclosure::RollbackAvailable,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5ConsequenceBlastRadius::SingleObject,
        M5ConsequenceReversibility::FullyReversible,
    ))
}

/// Clean update consequence stating rollback is unavailable and irreversible.
fn consequence_updates_irreversible() -> M5ResolvedConsequence {
    consequence(clean_consequence_base(
        "consequence:updates:irreversible",
        "Every workspace on this deployment updates and cannot be rolled back",
        M5ConsequenceDisclosure::RollbackUnavailableStated,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::UpdateOrInstall,
        M5ConsequenceBlastRadius::DeploymentWide,
        M5ConsequenceReversibility::IrreversibleAndStated,
    ))
}

/// Clean repair consequence with a help path.
fn consequence_support_help() -> M5ResolvedConsequence {
    consequence(clean_consequence_base(
        "consequence:support:help",
        "The affected profile is rebuilt; recovery steps are linked",
        M5ConsequenceDisclosure::HelpPathPresent,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::RepairConfirmation,
        M5ConsequenceBlastRadius::SingleObject,
        M5ConsequenceReversibility::RollbackWithNamedSteps,
    ))
}

/// Clean shell consequence naming explicit actions.
fn consequence_shell_explicit() -> M5ResolvedConsequence {
    consequence(clean_consequence_base(
        "consequence:shell:explicit",
        "Trusting this workspace enables its tasks",
        M5ConsequenceDisclosure::ExplicitNamedActions,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5ConsequenceBlastRadius::SingleObject,
        M5ConsequenceReversibility::FullyReversible,
    ))
}

/// Clean support-export consequence naming an irreversible external blast radius.
fn consequence_export_clean() -> M5ResolvedConsequence {
    consequence(clean_consequence_base(
        "consequence:support:export",
        "Deleting this bundle removes the exported evidence permanently",
        M5ConsequenceDisclosure::NamedBlastRadius,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::DestructiveDelete,
        M5ConsequenceBlastRadius::IrreversibleExternal,
        M5ConsequenceReversibility::IrreversibleAndStated,
    ))
}

// -- Degraded consequence examples --------------------------------------------------------------

/// Degraded consequence: the blast radius cannot be resolved.
fn consequence_blast_unresolved() -> M5ResolvedConsequence {
    let mut input = clean_consequence_base(
        "consequence:review:no-blast",
        "Some files will be rewritten",
        M5ConsequenceDisclosure::NamedBlastRadius,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::ReviewConfirmation,
        M5ConsequenceBlastRadius::RadiusUnknown,
        M5ConsequenceReversibility::RollbackWithNamedSteps,
    );
    input.blast_radius = M5ConsequenceBlastRadius::RadiusUnknown;
    consequence(input)
}

/// Degraded consequence: the rollback / help posture is unstated.
fn consequence_rollback_unstated() -> M5ResolvedConsequence {
    let mut input = clean_consequence_base(
        "consequence:settings:no-rollback",
        "This capability grant changes access",
        M5ConsequenceDisclosure::RollbackAvailable,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5ConsequenceBlastRadius::SingleObject,
        M5ConsequenceReversibility::FullyReversible,
    );
    input.rollback_or_help_posture_stated = false;
    consequence(input)
}

/// Degraded consequence: the reversibility posture cannot be resolved.
fn consequence_reversibility_unresolved() -> M5ResolvedConsequence {
    let mut input = clean_consequence_base(
        "consequence:updates:no-reversibility",
        "Every workspace on this deployment updates",
        M5ConsequenceDisclosure::RollbackUnavailableStated,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::UpdateOrInstall,
        M5ConsequenceBlastRadius::DeploymentWide,
        M5ConsequenceReversibility::ReversibilityUnknown,
    );
    input.reversibility = M5ConsequenceReversibility::ReversibilityUnknown;
    consequence(input)
}

/// Degraded consequence: the explanation is reachable only via a screenshot.
fn consequence_screenshot_only() -> M5ResolvedConsequence {
    let mut input = clean_consequence_base(
        "consequence:support:screenshot",
        "The affected profile is rebuilt",
        M5ConsequenceDisclosure::HelpPathPresent,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::RepairConfirmation,
        M5ConsequenceBlastRadius::SingleObject,
        M5ConsequenceReversibility::RollbackWithNamedSteps,
    );
    input.explanation_reachable_by_keyboard_sr_export = false;
    consequence(input)
}

/// Degraded consequence: it reduces to generic Yes/No ambiguity.
fn consequence_generic_yes_no() -> M5ResolvedConsequence {
    let mut input = clean_consequence_base(
        "consequence:shell:generic",
        "This affects the workspace",
        M5ConsequenceDisclosure::ExplicitNamedActions,
        M5DecisionFeedbackDisposition::Info,
        M5DecisionActionSurfaceContext::TrustPrompt,
        M5ConsequenceBlastRadius::SingleObject,
        M5ConsequenceReversibility::FullyReversible,
    );
    input.avoids_generic_yes_no = false;
    consequence(input)
}

/// Degraded consequence: the affected-object / scope label is unstated.
fn consequence_label_unstated() -> M5ResolvedConsequence {
    let mut input = clean_consequence_base(
        "consequence:support:no-label",
        "  ",
        M5ConsequenceDisclosure::NamedBlastRadius,
        M5DecisionFeedbackDisposition::Warning,
        M5DecisionActionSurfaceContext::DestructiveDelete,
        M5ConsequenceBlastRadius::IrreversibleExternal,
        M5ConsequenceReversibility::IrreversibleAndStated,
    );
    input.consequence_label = "  ".to_owned();
    consequence(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DialogConsequenceConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    dialog_examples: Vec<M5ResolvedDialog>,
    consequence_examples: Vec<M5ResolvedConsequence>,
) -> M5DialogConsequenceControlsRow {
    M5DialogConsequenceControlsRow {
        consumer_surface,
        qualification: M5DecisionFeedbackQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5DecisionFeedbackDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5DecisionFeedbackRequiredLabel::Identity,
            M5DecisionFeedbackRequiredLabel::State,
            M5DecisionFeedbackRequiredLabel::KeyboardRoute,
            M5DecisionFeedbackRequiredLabel::Rationale,
            M5DecisionFeedbackRequiredLabel::Scope,
            M5DecisionFeedbackRequiredLabel::RecoveryPath,
        ],
        accessibility_routes: M5DecisionFeedbackAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5DialogConsequenceAnatomyPart::ALL.to_vec(),
        export_fields: M5DialogConsequenceExportField::ALL.to_vec(),
        downgrade_triggers,
        dialog_examples,
        consequence_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_REF,
            M5_DIALOG_SHEET_SCHEMA_REF,
            M5_CONSEQUENCE_BLOCK_SCHEMA_REF,
        ]),
        dialog_uses_generic_yes_no_in_high_risk: false,
        dialog_focus_fails_to_return_on_reopen: false,
        consequence_omits_named_blast_radius: false,
        consequence_reduces_to_generic_yes_no: false,
    }
}

fn controls_rows() -> Vec<M5DialogConsequenceControlsRow> {
    use M5DecisionFeedbackConsumerSurface as C;
    use M5DecisionFeedbackDowngradeTrigger as D;

    vec![
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review confirmation names its rationale, scope, and specific actions and carries a consequence block that names the blast radius; both degrade honestly when the dialog reduces to generic Yes/No or the consequence block cannot resolve its blast radius",
            "evidence:m5-dialog-consequence-review-ui:001",
            vec![
                D::GenericYesNoUsedInHighRiskDialog,
                D::ScopeUnstated,
                D::ProofStale,
            ],
            vec![dialog_review_named(), dialog_generic_yes_no()],
            vec![consequence_review_blast(), consequence_blast_unresolved()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings trust dialog names a primary action plus an explicit cancel and states its rationale, and its consequence block states rollback availability; both degrade honestly when the rationale is unstated or the rollback posture is unstated",
            "evidence:m5-dialog-consequence-settings-ui:001",
            vec![
                D::RationaleUnstated,
                D::RecoveryPathUnstated,
                D::ProofStale,
            ],
            vec![dialog_settings_primary_cancel(), dialog_rationale_missing()],
            vec![
                consequence_settings_rollback(),
                consequence_rollback_unstated(),
            ],
        ),
        base_row(
            C::UpdatesUi,
            "Update / install owner",
            "The update / install dialog names its destructive confirm and its consequence block states the deployment-wide, irreversible blast radius; both degrade honestly when the scope is unstated or the reversibility posture cannot be resolved",
            "evidence:m5-dialog-consequence-updates-ui:001",
            vec![
                D::ScopeUnstated,
                D::RecoveryPathUnstated,
                D::ProofStale,
            ],
            vec![
                dialog_updates_destructive_named(),
                dialog_scope_missing(),
            ],
            vec![
                consequence_updates_irreversible(),
                consequence_reversibility_unresolved(),
            ],
        ),
        base_row(
            C::SupportUi,
            "Repair / support surface owner",
            "The repair confirmation states rationale and scope with a safe initial focus, and its consequence block carries a help path reachable off-screenshot; both degrade honestly when the initial focus is unsafe or the consequence explanation is screenshot-only",
            "evidence:m5-dialog-consequence-support-ui:001",
            vec![
                D::RecoveryPathUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                dialog_support_rationale_scope(),
                dialog_safe_focus_missing(),
            ],
            vec![consequence_support_help(), consequence_screenshot_only()],
        ),
        base_row(
            C::ShellUi,
            "Shell / entry surface owner",
            "The shell trust dialog is dismissible with a safe default and a cancel path, and its consequence block names explicit actions; both degrade honestly when the cancel path is missing or the consequence block reduces to generic Yes/No ambiguity",
            "evidence:m5-dialog-consequence-shell-ui:001",
            vec![
                D::RecoveryPathUnstated,
                D::GenericYesNoUsedInHighRiskDialog,
                D::ProofStale,
            ],
            vec![dialog_shell_dismissible_safe(), dialog_cancel_missing()],
            vec![consequence_shell_explicit(), consequence_generic_yes_no()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved dialog and consequence truth, so a broken focus return on reopen or an unstated consequence label is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-dialog-consequence-support-export:001",
            vec![
                D::GenericChromeWordingUsed,
                D::ScopeUnstated,
                D::ProofStale,
            ],
            vec![dialog_support_export_named(), dialog_focus_return_broken()],
            vec![consequence_export_clean(), consequence_label_unstated()],
        ),
    ]
}

fn governance_review() -> M5DialogConsequenceGovernanceReview {
    M5DialogConsequenceGovernanceReview {
        dialog_names_title_rationale_and_scope: true,
        dialog_names_explicit_actions_no_generic_yes_no: true,
        dialog_has_safe_initial_focus_and_cancel_path: true,
        dialog_returns_focus_on_reopen_from_notification: true,
        dialog_offers_help_or_docs_hook: true,
        consequence_names_affected_object_and_blast_radius: true,
        consequence_states_rollback_or_help_posture: true,
        consequence_notes_partial_success_or_irreversibility: true,
        consequence_never_reduces_to_generic_yes_no: true,
        consequence_explainable_without_screenshots: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5DialogConsequenceConsumerProjection {
    M5DialogConsequenceConsumerProjection {
        review_surfaces_consume_dialog_and_consequence_vocabulary: true,
        settings_surfaces_consume_dialog_vocabulary: true,
        updates_surfaces_consume_dialog_and_consequence_vocabulary: true,
        repair_surfaces_consume_consequence_vocabulary: true,
        dialog_and_consequence_trace_to_single_component_contract: true,
        support_export_reads_single_dialog_consequence_source: true,
    }
}

fn proof_freshness() -> M5DialogConsequenceProofFreshness {
    M5DialogConsequenceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DialogConsequenceReleasePosture {
    M5DialogConsequenceReleasePosture {
        proof_packet_ref: M5_DIALOG_CONSEQUENCE_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_DIALOG_CONSEQUENCE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_REF,
        M5_DIALOG_CONSEQUENCE_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_DIALOG_SHEET_SCHEMA_REF,
        M5_CONSEQUENCE_BLOCK_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 dialog / consequence controls packet.
pub fn seeded_m5_dialog_consequence_controls() -> M5DialogConsequenceControlsPacket {
    M5DialogConsequenceControlsPacket::new(M5DialogConsequenceControlsPacketInput {
        packet_id: M5_DIALOG_CONSEQUENCE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 dialog / sheet and consequence-block controls with stable title/rationale/scope anatomy, explicit action labels, safe initial focus, cancel paths, help/docs hooks off generic Yes/No, focus-return and reopen continuity, and consequence blocks naming affected object, blast radius, and rollback/help posture across review, settings, update/install, repair, shell, and support surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5DialogConsequenceVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the review-UI row is held at Beta pending dialog rationale/scope parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_dialog_consequence_controls_review_ui_beta_narrowed(
) -> M5DialogConsequenceControlsPacket {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.packet_id =
        "m5-dialog-sheet-and-consequence-block-controls:review-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the updates-UI row is narrowed to Preview pending consequence blast-radius /
/// rollback parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_dialog_consequence_controls_updates_ui_preview_narrowed(
) -> M5DialogConsequenceControlsPacket {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.packet_id =
        "m5-dialog-sheet-and-consequence-block-controls:updates-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::UpdatesUi)
        .expect("updates-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Preview;
    packet
}
