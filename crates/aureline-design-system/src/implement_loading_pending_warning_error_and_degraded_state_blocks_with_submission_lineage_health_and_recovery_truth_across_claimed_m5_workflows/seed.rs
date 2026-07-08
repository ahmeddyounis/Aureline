//! Canonical seed builders for the M5 degraded-state-contract primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical degraded-state-contract primitive packet.
pub const M5_DEGRADED_STATE_CONTRACT_PACKET_ID: &str =
    "m5-loading-pending-degraded-state-contract-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-08T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked degraded-state resolution case from a full block state.
#[allow(clippy::too_many_arguments)]
fn state_case(
    block_kind: M5DegradedStateBlockKind,
    degraded_state: M5SharedComponentStateClass,
    severity: M5DegradedStateSeverity,
    recovery_class: M5RecoveryDisclosureClass,
    state_cause: M5StateCauseClass,
    recovery_available: bool,
    retains_partial_capability: bool,
    high_contrast_active: bool,
    block_identity_ref: &str,
    state_style_ref: &str,
    submission_lineage_ref: &str,
    disclosure_ref: &str,
) -> M5DegradedStateResolutionCase {
    M5DegradedStateResolutionCase::resolved(M5DegradedStateResolutionInput {
        block_kind,
        degraded_state,
        severity,
        recovery_class,
        state_cause,
        recovery_available,
        retains_partial_capability,
        high_contrast_active,
        block_identity_ref: block_identity_ref.to_owned(),
        state_style_ref: state_style_ref.to_owned(),
        submission_lineage_ref: submission_lineage_ref.to_owned(),
        disclosure_ref: disclosure_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full degraded-state anatomy, states,
/// presentations, severities, non-color cues, required disclosures, recovery-disclosure classes,
/// state cause classes, export fields, labels, and accessibility parity every block carries.
fn base_row(
    block_kind: M5DegradedStateBlockKind,
    qualification: M5ComponentStateQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    state_examples: Vec<M5DegradedStateResolutionCase>,
) -> M5DegradedStateBlockRow {
    M5DegradedStateBlockRow {
        block_kind,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComponentStateSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComponentStateDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5DegradedStateAnatomyPart::ALL.to_vec(),
        degraded_states: degraded_states(),
        presentations: M5DegradedStatePresentation::ALL.to_vec(),
        severities: M5DegradedStateSeverity::ALL.to_vec(),
        non_color_cues: M5DegradedStateCue::ALL.to_vec(),
        required_disclosures: M5StateDisclosureTrigger::ALL.to_vec(),
        recovery_disclosure_classes: M5RecoveryDisclosureClass::ALL.to_vec(),
        state_cause_classes: M5StateCauseClass::ALL.to_vec(),
        export_fields: M5DegradedStateExportField::ALL.to_vec(),
        accessibility_routes: M5ComponentStateAccessibilityRoute::ALL.to_vec(),
        required_labels: M5ComponentStateRequiredLabel::ALL.to_vec(),
        consumer_surfaces: M5ComponentStateConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ComponentStateDowngradeTrigger::PendingShownAsLoading,
            M5ComponentStateDowngradeTrigger::ConsequenceOrRecoveryOmitted,
            M5ComponentStateDowngradeTrigger::ColorOnlyTreatment,
            M5ComponentStateDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF,
            M5_DEGRADED_STATE_CONTRACT_STATE_RECOVERY_REF,
            M5_DEGRADED_STATE_CONTRACT_SERVICE_HEALTH_REF,
        ]),
        state_examples,
        presents_pending_as_generic_loading: false,
        collapses_warning_and_error: false,
        omits_consequence_or_recovery: false,
        invents_private_state_name: false,
    }
}

fn rows() -> Vec<M5DegradedStateBlockRow> {
    use M5ComponentStateQualificationClass as Qual;
    use M5DegradedStateBlockKind as Block;
    use M5DegradedStateSeverity as Severity;
    use M5RecoveryDisclosureClass as Recovery;
    use M5SharedComponentStateClass as State;
    use M5StateCauseClass as Cause;

    vec![
        // 1. Form — the user-submitted pending treatment and the hard-error warning/error
        //    treatment, so a save-settings submit is attributed to the user action that triggered
        //    it (never a generic spinner) and a validation error names its consequence and the
        //    recovery path while preserving the same submission lineage.
        base_row(
            Block::Form,
            Qual::Stable,
            "Form workflow owner",
            "The form renders the shared degraded-state contract so a submitted save action shows as pending — attributed to the exact user action, not a generic background spinner — and a validation error names its consequence, its recovery path, and keeps the submission lineage so the activity center and support export can reconstruct what the user did",
            "evidence:m5-degraded-state-form:001",
            vec![
                state_case(
                    Block::Form,
                    State::Pending,
                    Severity::Informational,
                    Recovery::NamesRecoveryAction,
                    Cause::PreconditionCause,
                    true,
                    true,
                    false,
                    "block:settings-form.save-workspace",
                    "token:state.form.pending",
                    "submission:settings-form.save-workspace#req-1",
                    "",
                ),
                state_case(
                    Block::Form,
                    State::WarningError,
                    Severity::Error,
                    Recovery::NamesRecoveryAction,
                    Cause::PreconditionCause,
                    true,
                    true,
                    false,
                    "block:settings-form.save-workspace",
                    "token:state.form.error",
                    "submission:settings-form.save-workspace#req-1",
                    "error:settings-form.workspace-name-required",
                ),
            ],
        ),
        // 2. Background job row — the background loading treatment and the failed-job error
        //    treatment, so a running index rebuild shows generic background progress with no false
        //    submission claim, and a failed job names its connectivity consequence, its retry path,
        //    and the submission lineage of the run that failed.
        base_row(
            Block::JobRow,
            Qual::Stable,
            "Activity center owner",
            "The background job row renders the shared degraded-state contract so a running job shows background loading progress with no submission attribution it does not own, and a failed job names its consequence, its retry path, and the submission lineage of the run — a health regression the activity center and support export can reconstruct, never a bare spinner that hides the failure",
            "evidence:m5-degraded-state-job-row:001",
            vec![
                state_case(
                    Block::JobRow,
                    State::Loading,
                    Severity::Informational,
                    Recovery::NamesFreshness,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "block:activity.index-rebuild-run",
                    "token:state.job_row.loading",
                    "",
                    "",
                ),
                state_case(
                    Block::JobRow,
                    State::WarningError,
                    Severity::Error,
                    Recovery::NamesRetryPath,
                    Cause::ConnectivityCause,
                    true,
                    true,
                    false,
                    "block:activity.index-rebuild-run",
                    "token:state.job_row.error",
                    "submission:activity.index-rebuild-run#run-9",
                    "error:activity.index-rebuild.upstream-timeout",
                ),
            ],
        ),
        // 3. Banner — the warning-severity treatment and the reduced-capability degraded treatment,
        //    so a sync-behind banner warns with its consequence and recovery while the workflow can
        //    still proceed, and an offline banner names its degraded fallback scope and what still
        //    works.
        base_row(
            Block::Banner,
            Qual::Stable,
            "Shell banner owner",
            "The banner renders the shared degraded-state contract so a sync-behind warning names its consequence and recovery path without blocking the workflow, and an offline degraded banner names its reduced fallback scope and what still works — a warning glyph or a reduced-capability glyph with an explicit next safe action, never a color-only banner that collapses a warning into a hard error",
            "evidence:m5-degraded-state-banner:001",
            vec![
                state_case(
                    Block::Banner,
                    State::WarningError,
                    Severity::Warning,
                    Recovery::NamesConsequence,
                    Cause::FreshnessCause,
                    true,
                    true,
                    true,
                    "block:shell-banner.sync-behind",
                    "token:state.banner.warning",
                    "",
                    "warning:shell-banner.sync-behind-by-3-commits",
                ),
                state_case(
                    Block::Banner,
                    State::Degraded,
                    Severity::Reduced,
                    Recovery::NamesFallbackScope,
                    Cause::ConnectivityCause,
                    true,
                    true,
                    true,
                    "block:shell-banner.offline-mode",
                    "token:state.banner.degraded",
                    "",
                    "degraded:shell-banner.offline-read-only-cache",
                ),
            ],
        ),
        // 4. Card — the background loading treatment and the reduced-capability degraded treatment,
        //    so a metrics card shows honest background loading and a stale-metrics card names its
        //    freshness consequence and what still works rather than presenting stale numbers as
        //    fresh.
        base_row(
            Block::Card,
            Qual::Stable,
            "Dashboard card owner",
            "The card renders the shared degraded-state contract so a loading card shows honest background progress and a degraded card names its lowered freshness, what still works, and the refresh path — never presenting a stale or partial card as fully fresh, and never a color-only dimming that hides the reduced certainty",
            "evidence:m5-degraded-state-card:001",
            vec![
                state_case(
                    Block::Card,
                    State::Loading,
                    Severity::Informational,
                    Recovery::NamesRetryPath,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "block:dashboard.throughput-card",
                    "token:state.card.loading",
                    "",
                    "",
                ),
                state_case(
                    Block::Card,
                    State::Degraded,
                    Severity::Reduced,
                    Recovery::NamesFreshness,
                    Cause::FreshnessCause,
                    true,
                    true,
                    false,
                    "block:dashboard.throughput-card",
                    "token:state.card.degraded",
                    "",
                    "degraded:dashboard.throughput-card.stale-by-15m",
                ),
            ],
        ),
        // 5. Dense row — the user-submitted pending treatment and the reduced-capability degraded
        //    treatment, so an inline edit shows as pending attributed to the user action, and a
        //    partial-data row names its degraded scope and what still works.
        base_row(
            Block::Row,
            Qual::Stable,
            "Dense collection owner",
            "The dense row renders the shared degraded-state contract so an inline edit shows as pending attributed to the exact user action rather than generic loading, and a partial-data row names its degraded scope, what still works, and the recovery path — never a spinner that hides which action is in flight and never a color-only treatment that collapses pending into loading",
            "evidence:m5-degraded-state-row:001",
            vec![
                state_case(
                    Block::Row,
                    State::Pending,
                    Severity::Informational,
                    Recovery::NamesRecoveryAction,
                    Cause::PreconditionCause,
                    true,
                    true,
                    false,
                    "block:results-row.rename-item",
                    "token:state.row.pending",
                    "submission:results-row.rename-item#req-4",
                    "",
                ),
                state_case(
                    Block::Row,
                    State::Degraded,
                    Severity::Reduced,
                    Recovery::NamesRecoveryAction,
                    Cause::PreconditionCause,
                    true,
                    true,
                    false,
                    "block:results-row.partial-row",
                    "token:state.row.degraded",
                    "",
                    "degraded:results-row.partial-columns-available",
                ),
            ],
        ),
        // 6. Review sheet — the policy-blocked error treatment and the reduced-context degraded
        //    treatment, so a blocked approval names its policy consequence, recovery path, and the
        //    submission lineage of the approval attempt, and a reduced-context review names what
        //    still works even when no recovery is available.
        base_row(
            Block::ReviewSheet,
            Qual::Stable,
            "Review workflow owner",
            "The review sheet renders the shared degraded-state contract so a policy-blocked approval names its consequence, its recovery path, and the submission lineage of the approval attempt, and a reduced-context review names what still works and states honestly when no recovery is available — never an error toast that drops the submission lineage or a degraded sheet that hides how much context is missing",
            "evidence:m5-degraded-state-review-sheet:001",
            vec![
                state_case(
                    Block::ReviewSheet,
                    State::WarningError,
                    Severity::Error,
                    Recovery::NamesRecoveryAction,
                    Cause::PolicyCause,
                    true,
                    true,
                    false,
                    "block:review-sheet.approve-change",
                    "token:state.review_sheet.error",
                    "submission:review-sheet.approve-change#req-3",
                    "error:review-sheet.policy-requires-second-reviewer",
                ),
                state_case(
                    Block::ReviewSheet,
                    State::Degraded,
                    Severity::Reduced,
                    Recovery::NoRecoveryAvailable,
                    Cause::UnknownCause,
                    false,
                    true,
                    false,
                    "block:review-sheet.reduced-context",
                    "token:state.review_sheet.degraded",
                    "",
                    "degraded:review-sheet.diff-context-partially-unavailable",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5DegradedStateGovernanceReview {
    M5DegradedStateGovernanceReview {
        blocks_distinguish_loading_pending_warning_error_degraded: true,
        loading_and_pending_never_collapse: true,
        warning_and_error_never_collapse: true,
        error_and_degraded_never_collapse: true,
        pending_attributed_to_user_action: true,
        consequence_and_recovery_surfaced_when_explainable: true,
        submission_lineage_and_capability_preserved: true,
        state_meaning_never_color_only: true,
        states_keyboard_and_screen_reader_explainable: true,
        states_driven_by_shared_contract_and_tokens: true,
        no_one_off_per_surface_styling: true,
        states_stable_across_deployment_lines: true,
        states_stable_across_consumer_surfaces: true,
        every_block_declares_accessibility_route: true,
        support_export_reconstructs_state_truth: true,
        later_rows_cannot_invent_parallel_state_vocabulary: true,
    }
}

fn consumer_projection() -> M5DegradedStateConsumerProjection {
    M5DegradedStateConsumerProjection {
        blocks_consume_state_vocabulary: true,
        presentation_reads_single_source: true,
        disclosure_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5DegradedStateProofFreshness {
    M5DegradedStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DegradedStateReleasePosture {
    M5DegradedStateReleasePosture {
        release_packet_ref: M5_DEGRADED_STATE_CONTRACT_ARTIFACT_REF.to_owned(),
        degraded_state_audit_ref: M5_DEGRADED_STATE_CONTRACT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF,
        M5_DEGRADED_STATE_CONTRACT_DOC_REF,
        M5_DEGRADED_STATE_CONTRACT_COMPONENT_MATRIX_REF,
        M5_DEGRADED_STATE_CONTRACT_SERVICE_HEALTH_REF,
        M5_DEGRADED_STATE_CONTRACT_STATE_RECOVERY_REF,
        M5_DEGRADED_STATE_CONTRACT_ACTIVITY_ROW_REF,
    ])
}

/// Builds the canonical M5 degraded-state-contract packet.
pub fn seeded_m5_degraded_state_contract_packet() -> M5DegradedStateContractPacket {
    M5DegradedStateContractPacket::new(M5DegradedStateContractPacketInput {
        packet_id: M5_DEGRADED_STATE_CONTRACT_PACKET_ID.to_owned(),
        matrix_label:
            "M5 loading / pending / warning-error / degraded state-block contract primitive: block kind, degraded state (loading/pending/warning-error/degraded), derived presentation posture, warning-vs-error severity, required non-color cues, required disclosures (state cause / owner / block reason / recovery action), recovery-disclosure class, and the loading-vs-pending / warning-vs-error / error-vs-degraded distinctness plus submission-lineage, what-still-works, and next-safe-action guarantees"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5DegradedStateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the banner block is held at Beta because a slice of banner surfaces does not
/// yet name the fallback scope on every profile; every block stays visible.
pub fn seeded_m5_degraded_state_contract_banner_beta_narrowed() -> M5DegradedStateContractPacket {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.packet_id =
        "m5-loading-pending-degraded-state-contract-primitive:banner-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.block_kind == M5DegradedStateBlockKind::Banner)
        .expect("banner row present");
    row.qualification = M5ComponentStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review sheet block is narrowed to Preview pending submission-lineage
/// parity proof across every density; every block stays visible.
pub fn seeded_m5_degraded_state_contract_review_sheet_preview_narrowed(
) -> M5DegradedStateContractPacket {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.packet_id =
        "m5-loading-pending-degraded-state-contract-primitive:review-sheet-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.block_kind == M5DegradedStateBlockKind::ReviewSheet)
        .expect("review-sheet row present");
    row.qualification = M5ComponentStateQualificationClass::Preview;
    packet
}
