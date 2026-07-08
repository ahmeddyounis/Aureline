//! Canonical seed builders for the M5 state-distinction-explanation-helper primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical state-distinction-explanation-helper primitive packet.
pub const M5_STATE_EXPLANATION_PACKET_ID: &str =
    "m5-state-distinction-explanation-helper-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked state-distinction-explanation resolution case from a full explanation.
#[allow(clippy::too_many_arguments)]
fn explanation_case(
    surface: M5ExplanationConsumerSurface,
    distinction: M5ConfusableStateDistinction,
    delivery: M5ExplanationDelivery,
    recovery_class: M5RecoveryDisclosureClass,
    state_cause: M5StateCauseClass,
    recovery_available: bool,
    high_contrast_active: bool,
    explanation_identity_ref: &str,
    taxonomy_ref: &str,
    distinction_copy_ref: &str,
    blocked_limited_copy_ref: &str,
) -> M5StateExplanationCase {
    M5StateExplanationCase::resolved(M5StateExplanationInput {
        surface,
        distinction,
        delivery,
        recovery_class,
        state_cause,
        recovery_available,
        high_contrast_active,
        explanation_identity_ref: explanation_identity_ref.to_owned(),
        taxonomy_ref: taxonomy_ref.to_owned(),
        distinction_copy_ref: distinction_copy_ref.to_owned(),
        blocked_limited_copy_ref: blocked_limited_copy_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full explanation anatomy, distinctions,
/// precedence rules, deliveries, non-color cues, required disclosures, recovery-disclosure classes,
/// state cause classes, export fields, labels, and accessibility parity every surface carries.
fn base_row(
    surface: M5ExplanationConsumerSurface,
    qualification: M5ComponentStateQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    explanation_examples: Vec<M5StateExplanationCase>,
) -> M5ExplanationSurfaceRow {
    M5ExplanationSurfaceRow {
        surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComponentStateSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComponentStateDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ExplanationAnatomyPart::ALL.to_vec(),
        distinctions: M5ConfusableStateDistinction::ALL.to_vec(),
        precedence_rules: M5StatePrecedenceRule::ALL.to_vec(),
        deliveries: M5ExplanationDelivery::ALL.to_vec(),
        non_color_cues: M5ExplanationCue::ALL.to_vec(),
        required_disclosures: M5StateDisclosureTrigger::ALL.to_vec(),
        recovery_disclosure_classes: M5RecoveryDisclosureClass::ALL.to_vec(),
        state_cause_classes: M5StateCauseClass::ALL.to_vec(),
        export_fields: M5ExplanationExportField::ALL.to_vec(),
        accessibility_routes: M5ComponentStateAccessibilityRoute::ALL.to_vec(),
        required_labels: M5ComponentStateRequiredLabel::ALL.to_vec(),
        consumer_surfaces: M5ComponentStateConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ComponentStateDowngradeTrigger::AlternateStateLabelInvented,
            M5ComponentStateDowngradeTrigger::CurrentSelectedCollapsed,
            M5ComponentStateDowngradeTrigger::PendingShownAsLoading,
            M5ComponentStateDowngradeTrigger::ColorOnlyTreatment,
            M5ComponentStateDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STATE_EXPLANATION_SCHEMA_REF,
            M5_STATE_EXPLANATION_STATE_CLASS_REF,
            M5_STATE_EXPLANATION_COMPONENT_MATRIX_REF,
        ]),
        explanation_examples,
        invents_one_off_state_language: false,
        contradicts_shared_taxonomy: false,
        collapses_the_two_states: false,
        misaligns_blocked_action_help: false,
    }
}

fn rows() -> Vec<M5ExplanationSurfaceRow> {
    use M5ComponentStateQualificationClass as Qual;
    use M5ConfusableStateDistinction as Distinction;
    use M5ExplanationConsumerSurface as Surface;
    use M5ExplanationDelivery as Delivery;
    use M5RecoveryDisclosureClass as Recovery;
    use M5StateCauseClass as Cause;

    // The canonical shared-taxonomy reference every explanation links back to, so no surface floats
    // a one-off label divorced from the frozen state classes.
    let taxonomy = M5_STATE_EXPLANATION_COMPONENT_MATRIX_REF;

    vec![
        // 1. Onboarding / help — the drawer that teaches current-vs-selected the first time a user
        //    meets both, and the inline chip that names a pending action so it never reads as
        //    generic loading.
        base_row(
            Surface::OnboardingHelp,
            Qual::Stable,
            "Learnability owner",
            "The onboarding / help surface teaches the confusable distinctions in place, using the same taxonomy words the components expose: an expanded drawer explains current-vs-selected and links back to the canonical taxonomy, and an inline chip names a pending action so a first-time user never mistakes it for generic background loading",
            "evidence:m5-state-explanation-onboarding:001",
            vec![
                explanation_case(
                    Surface::OnboardingHelp,
                    Distinction::CurrentVsSelected,
                    Delivery::ExpandedDrawer,
                    Recovery::NamesConsequence,
                    Cause::UnknownCause,
                    true,
                    false,
                    "explain:onboarding.current-vs-selected",
                    taxonomy,
                    "copy:onboarding.current-vs-selected-drawer",
                    "",
                ),
                explanation_case(
                    Surface::OnboardingHelp,
                    Distinction::PendingVsLoading,
                    Delivery::InlineChip,
                    Recovery::NamesRetryPath,
                    Cause::ConnectivityCause,
                    true,
                    false,
                    "explain:onboarding.pending-vs-loading",
                    taxonomy,
                    "copy:onboarding.pending-vs-loading-chip",
                    "",
                ),
            ],
        ),
        // 2. Blocked-action row — the blocked/limited copy objects that keep blocked-action help
        //    aligned with the same state truth: a policy lock explained as locked (not merely
        //    disabled) with its recovery path, and a read-only projection explained as read-only
        //    (not disabled) even when no recovery is available.
        base_row(
            Surface::BlockedActionRow,
            Qual::Stable,
            "Blocked-action help owner",
            "The blocked-action explanation row uses blocked/limited copy objects that stay aligned with the component-state truth: a locked-vs-disabled explanation names the policy owner, the block reason, and the recovery path so a lock never hides behind a bare disabled control, and a read-only-vs-disabled explanation preserves inspectability and states honestly when no recovery is available",
            "evidence:m5-state-explanation-blocked-action:001",
            vec![
                explanation_case(
                    Surface::BlockedActionRow,
                    Distinction::LockedVsDisabled,
                    Delivery::BlockedLimitedCopy,
                    Recovery::NamesRecoveryAction,
                    Cause::PolicyCause,
                    true,
                    false,
                    "explain:blocked-action.locked-vs-disabled",
                    taxonomy,
                    "copy:blocked-action.locked-vs-disabled-body",
                    "copy:blocked-action.locked-by-policy-owner-and-recovery",
                ),
                explanation_case(
                    Surface::BlockedActionRow,
                    Distinction::ReadOnlyVsDisabled,
                    Delivery::BlockedLimitedCopy,
                    Recovery::NoRecoveryAvailable,
                    Cause::PermissionCause,
                    false,
                    false,
                    "explain:blocked-action.read-only-vs-disabled",
                    taxonomy,
                    "copy:blocked-action.read-only-vs-disabled-body",
                    "copy:blocked-action.read-only-projection-still-inspectable",
                ),
            ],
        ),
        // 3. Settings row — the inline chip that names a read-only effective value (not disabled) and
        //    the drawer that explains a locked setting versus a plain disabled one.
        base_row(
            Surface::SettingsRow,
            Qual::Stable,
            "Settings surface owner",
            "The settings row explains its confusable states in place: an inline chip names a read-only effective value so it never reads as a plain disabled control, and an expanded drawer explains a locked setting against a disabled one — naming the owner, the reason, and the recovery path — so a policy lock stays explainable rather than collapsing into a silent disabled row",
            "evidence:m5-state-explanation-settings:001",
            vec![
                explanation_case(
                    Surface::SettingsRow,
                    Distinction::ReadOnlyVsDisabled,
                    Delivery::InlineChip,
                    Recovery::NamesFreshness,
                    Cause::PreconditionCause,
                    true,
                    false,
                    "explain:settings.read-only-vs-disabled",
                    taxonomy,
                    "copy:settings.read-only-vs-disabled-chip",
                    "",
                ),
                explanation_case(
                    Surface::SettingsRow,
                    Distinction::LockedVsDisabled,
                    Delivery::ExpandedDrawer,
                    Recovery::NamesRecoveryAction,
                    Cause::PolicyCause,
                    true,
                    false,
                    "explain:settings.locked-vs-disabled",
                    taxonomy,
                    "copy:settings.locked-vs-disabled-drawer",
                    "",
                ),
            ],
        ),
        // 4. Activity row — the blocked/limited copy that names a pending submission in flight (so it
        //    never reads as generic loading) and the drawer that teaches pending-vs-loading with its
        //    consequence.
        base_row(
            Surface::ActivityRow,
            Qual::Stable,
            "Activity center owner",
            "The activity row keeps pending distinct from loading: a blocked/limited copy object attributes a pending submission to the exact user action in flight, with its consequence and retry path, and an expanded drawer teaches pending-vs-loading so a submitted action in the activity center never masquerades as generic background work",
            "evidence:m5-state-explanation-activity:001",
            vec![
                explanation_case(
                    Surface::ActivityRow,
                    Distinction::PendingVsLoading,
                    Delivery::BlockedLimitedCopy,
                    Recovery::NamesRetryPath,
                    Cause::ConnectivityCause,
                    true,
                    false,
                    "explain:activity.pending-vs-loading-copy",
                    taxonomy,
                    "copy:activity.pending-vs-loading-body",
                    "copy:activity.pending-submission-in-flight-and-retry",
                ),
                explanation_case(
                    Surface::ActivityRow,
                    Distinction::PendingVsLoading,
                    Delivery::ExpandedDrawer,
                    Recovery::NamesConsequence,
                    Cause::PreconditionCause,
                    true,
                    false,
                    "explain:activity.pending-vs-loading-drawer",
                    taxonomy,
                    "copy:activity.pending-vs-loading-drawer-body",
                    "",
                ),
            ],
        ),
        // 5. Workspace entry — the inline chip that names the current workspace (not merely a
        //    selected one) and the drawer that teaches current-vs-selected across the entry list.
        base_row(
            Surface::WorkspaceEntry,
            Qual::Stable,
            "Workspace entry owner",
            "The workspace-entry surface keeps current distinct from selected: an inline chip names the current workspace / live context owner so it never collapses into a merely selected entry, and an expanded drawer teaches current-vs-selected across the entry list with a fallback-scope recovery path when the live context is unavailable",
            "evidence:m5-state-explanation-workspace-entry:001",
            vec![
                explanation_case(
                    Surface::WorkspaceEntry,
                    Distinction::CurrentVsSelected,
                    Delivery::InlineChip,
                    Recovery::NamesConsequence,
                    Cause::UnknownCause,
                    true,
                    false,
                    "explain:workspace-entry.current-vs-selected-chip",
                    taxonomy,
                    "copy:workspace-entry.current-vs-selected-chip",
                    "",
                ),
                explanation_case(
                    Surface::WorkspaceEntry,
                    Distinction::CurrentVsSelected,
                    Delivery::ExpandedDrawer,
                    Recovery::NamesFallbackScope,
                    Cause::FreshnessCause,
                    true,
                    false,
                    "explain:workspace-entry.current-vs-selected-drawer",
                    taxonomy,
                    "copy:workspace-entry.current-vs-selected-drawer",
                    "",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5ExplanationGovernanceReview {
    M5ExplanationGovernanceReview {
        surfaces_explain_state_semantics_in_place: true,
        current_and_selected_never_collapse: true,
        read_only_and_disabled_never_collapse: true,
        locked_and_disabled_never_collapse: true,
        pending_and_loading_never_collapse: true,
        no_one_off_language_invented: true,
        aligned_with_shared_taxonomy: true,
        contextual_teaching_aligned_with_component_truth: true,
        blocked_action_help_aligned_with_component_truth: true,
        state_meaning_never_color_only: true,
        explanations_keyboard_and_screen_reader_explainable: true,
        explanations_driven_by_shared_contract_and_tokens: true,
        no_one_off_per_surface_copy: true,
        explanations_stable_across_deployment_lines: true,
        explanations_stable_across_consumer_surfaces: true,
        every_surface_declares_accessibility_route: true,
        support_export_reconstructs_explanation_truth: true,
        later_rows_cannot_invent_parallel_state_vocabulary: true,
    }
}

fn consumer_projection() -> M5ExplanationConsumerProjection {
    M5ExplanationConsumerProjection {
        surfaces_consume_state_vocabulary: true,
        cue_set_reads_single_source: true,
        disclosure_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5ExplanationProofFreshness {
    M5ExplanationProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ExplanationReleasePosture {
    M5ExplanationReleasePosture {
        release_packet_ref: M5_STATE_EXPLANATION_ARTIFACT_REF.to_owned(),
        explanation_audit_ref: M5_STATE_EXPLANATION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STATE_EXPLANATION_SCHEMA_REF,
        M5_STATE_EXPLANATION_DOC_REF,
        M5_STATE_EXPLANATION_COMPONENT_MATRIX_REF,
        M5_STATE_EXPLANATION_STATE_CLASS_REF,
        M5_STATE_EXPLANATION_BLOCKED_ACTION_REF,
        M5_STATE_EXPLANATION_CONTEXTUAL_TEACHING_REF,
    ])
}

/// Builds the canonical M5 state-distinction-explanation-helper packet.
pub fn seeded_m5_state_explanation_packet() -> M5StateExplanationPacket {
    M5StateExplanationPacket::new(M5StateExplanationPacketInput {
        packet_id: M5_STATE_EXPLANATION_PACKET_ID.to_owned(),
        matrix_label:
            "M5 state-distinction explanation helper primitive: consumer surface, confusable distinction (current-vs-selected / read-only-vs-disabled / locked-vs-disabled / pending-vs-loading), frozen precedence rule, primary and contrasted states, delivery form (inline chip / expanded drawer / blocked-limited copy), required non-color cues, required disclosures (state cause / owner / block reason / recovery action), recovery-disclosure class, and the stay-distinct, no-one-off-language, taxonomy-alignment, and blocked-action-alignment guarantees"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5ExplanationVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the blocked-action row is held at Beta because a slice of blocked-action
/// surfaces does not yet name the owner on every profile; every surface stays visible.
pub fn seeded_m5_state_explanation_blocked_action_beta_narrowed() -> M5StateExplanationPacket {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.packet_id =
        "m5-state-distinction-explanation-helper-primitive:blocked-action-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5ExplanationConsumerSurface::BlockedActionRow)
        .expect("blocked-action row present");
    row.qualification = M5ComponentStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the workspace-entry surface is narrowed to Preview pending current-vs-selected
/// parity proof across every density; every surface stays visible.
pub fn seeded_m5_state_explanation_workspace_entry_preview_narrowed() -> M5StateExplanationPacket {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.packet_id =
        "m5-state-distinction-explanation-helper-primitive:workspace-entry-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == M5ExplanationConsumerSurface::WorkspaceEntry)
        .expect("workspace-entry row present");
    row.qualification = M5ComponentStateQualificationClass::Preview;
    packet
}
