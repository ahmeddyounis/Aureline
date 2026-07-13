//! Canonical seed builders for the M5 toast / loading-state controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code controls, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean toasts and loading states are built
//! so the shared acknowledgement, durable-backlink, loading-treatment, and readiness grammar is proven
//! across shell, review, settings, help, support, and support-export surfaces without any toast-only
//! truth, missing durable backlink, blanked pane, full-screen spinner, readiness overclaim, or
//! non-reconstructable explanation.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_TOAST_LOADING_CONTROLS_PACKET_ID: &str =
    "m5-toast-and-loading-state-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn toast(input: M5ToastResolutionInput) -> M5ResolvedToast {
    resolve_toast(input).expect("seed toast input resolves")
}

fn loading_state(input: M5LoadingStateResolutionInput) -> M5ResolvedLoadingState {
    resolve_loading_state(input).expect("seed loading-state input resolves")
}

// -- Clean toast examples (acknowledgement scope and durable-backlink coverage across surfaces) --

#[allow(clippy::too_many_arguments)]
fn clean_toast_base(
    toast_id: &str,
    label: &str,
    toast_durability: M5ToastDurability,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5TransientSurfaceContext,
    acknowledgement_scope: M5ToastAcknowledgementScope,
    backlink_target: M5ToastBacklinkTarget,
    outcome_matters: bool,
    durable_backlink_present: bool,
) -> M5ToastResolutionInput {
    M5ToastResolutionInput {
        toast_id: toast_id.to_owned(),
        toast_label: label.to_owned(),
        toast_durability,
        disposition,
        surface_context: surface,
        acknowledgement_scope,
        backlink_target,
        acknowledges_transiently: true,
        outcome_matters_after_dismissal: outcome_matters,
        durable_backlink_present,
        bounded_action_present: false,
        action_is_bounded: false,
        avoids_toast_only_truth: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

/// Clean shell toast: a transient confirmation whose outcome does not matter after dismissal.
fn toast_shell_transient() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:shell:saved",
        "Layout preference saved",
        M5ToastDurability::TransientAcknowledgment,
        M5DecisionFeedbackDisposition::Success,
        M5TransientSurfaceContext::ShellEntry,
        M5ToastAcknowledgementScope::TransientConfirmation,
        M5ToastBacklinkTarget::TargetUnknown,
        false,
        false,
    ))
}

/// Clean review toast: a durable-outcome acknowledgement backed by the review queue.
fn toast_review_durable() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:review:approved",
        "Change approved — it is now in the review queue",
        M5ToastDurability::MirroredToActivityCenter,
        M5DecisionFeedbackDisposition::Success,
        M5TransientSurfaceContext::ReviewWorkspace,
        M5ToastAcknowledgementScope::DurableOutcomeAck,
        M5ToastBacklinkTarget::ReviewQueue,
        true,
        true,
    ))
}

/// Clean settings toast: a reversible action acknowledgement with a bounded undo backed by a record.
fn toast_settings_reversible() -> M5ResolvedToast {
    let mut input = clean_toast_base(
        "toast:settings:revoked",
        "Capability revoked — open settings to restore it",
        M5ToastDurability::ActionRetainedElsewhere,
        M5DecisionFeedbackDisposition::Info,
        M5TransientSurfaceContext::SettingsArea,
        M5ToastAcknowledgementScope::ReversibleActionAck,
        M5ToastBacklinkTarget::SettingsRecord,
        true,
        true,
    );
    input.bounded_action_present = true;
    input.action_is_bounded = true;
    toast(input)
}

/// Clean help toast: a non-blocking notice whose outcome does not matter after dismissal.
fn toast_help_notice() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:help:copied",
        "Help article link copied",
        M5ToastDurability::AutoDismissTimed,
        M5DecisionFeedbackDisposition::Info,
        M5TransientSurfaceContext::HelpArea,
        M5ToastAcknowledgementScope::NonBlockingNotice,
        M5ToastBacklinkTarget::TargetUnknown,
        false,
        false,
    ))
}

/// Clean support toast: a background handoff acknowledgement backed by a support record.
fn toast_support_background() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:support:report",
        "Diagnostic report queued — track it in the support record",
        M5ToastDurability::MirroredToActivityCenter,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SupportArea,
        M5ToastAcknowledgementScope::BackgroundHandoff,
        M5ToastBacklinkTarget::SupportRecord,
        true,
        true,
    ))
}

/// Clean support-export toast: a durable-outcome acknowledgement backed by the activity center.
fn toast_export_durable() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:support:export",
        "Support bundle exported — its record lives in the activity center",
        M5ToastDurability::MirroredToActivityCenter,
        M5DecisionFeedbackDisposition::Success,
        M5TransientSurfaceContext::SupportArea,
        M5ToastAcknowledgementScope::DurableOutcomeAck,
        M5ToastBacklinkTarget::ActivityCenter,
        true,
        true,
    ))
}

// -- Degraded toast examples ---------------------------------------------------------------------

/// Degraded toast: the outcome matters but the durable backlink is missing.
fn toast_backlink_missing() -> M5ResolvedToast {
    let mut input = clean_toast_base(
        "toast:shell:no-backlink",
        "Change approved",
        M5ToastDurability::TransientAcknowledgment,
        M5DecisionFeedbackDisposition::Success,
        M5TransientSurfaceContext::ShellEntry,
        M5ToastAcknowledgementScope::DurableOutcomeAck,
        M5ToastBacklinkTarget::TargetUnknown,
        true,
        false,
    );
    input.durable_backlink_present = false;
    toast(input)
}

/// Degraded toast: it is used as the only durable truth for reviewable work.
fn toast_only_truth() -> M5ResolvedToast {
    let mut input = clean_toast_base(
        "toast:review:only-truth",
        "Merge finished (this toast is the only record)",
        M5ToastDurability::MirroredToActivityCenter,
        M5DecisionFeedbackDisposition::Success,
        M5TransientSurfaceContext::ReviewWorkspace,
        M5ToastAcknowledgementScope::DurableOutcomeAck,
        M5ToastBacklinkTarget::ReviewQueue,
        true,
        true,
    );
    input.avoids_toast_only_truth = false;
    toast(input)
}

/// Degraded toast: the toast durability is the disallowed toast-only-truth token.
fn toast_durability_disallowed() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:settings:durability",
        "Capability revoked",
        M5ToastDurability::ToastOnlyTruthDisallowed,
        M5DecisionFeedbackDisposition::Warning,
        M5TransientSurfaceContext::SettingsArea,
        M5ToastAcknowledgementScope::ReversibleActionAck,
        M5ToastBacklinkTarget::SettingsRecord,
        true,
        true,
    ))
}

/// Degraded toast: the acknowledgement scope cannot be resolved.
fn toast_scope_unresolved() -> M5ResolvedToast {
    toast(clean_toast_base(
        "toast:help:no-scope",
        "Done",
        M5ToastDurability::AutoDismissTimed,
        M5DecisionFeedbackDisposition::Info,
        M5TransientSurfaceContext::HelpArea,
        M5ToastAcknowledgementScope::ScopeUnknown,
        M5ToastBacklinkTarget::TargetUnknown,
        false,
        false,
    ))
}

/// Degraded toast: a present action is not bounded to a single safe action.
fn toast_action_unbounded() -> M5ResolvedToast {
    let mut input = clean_toast_base(
        "toast:support:unbounded",
        "Report queued — track it in the support record",
        M5ToastDurability::MirroredToActivityCenter,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SupportArea,
        M5ToastAcknowledgementScope::BackgroundHandoff,
        M5ToastBacklinkTarget::SupportRecord,
        true,
        true,
    );
    input.bounded_action_present = true;
    input.action_is_bounded = false;
    toast(input)
}

/// Degraded toast: the explanation cannot be reconstructed from the export.
fn toast_not_reconstructable() -> M5ResolvedToast {
    let mut input = clean_toast_base(
        "toast:support:screenshot-only",
        "Support bundle exported",
        M5ToastDurability::MirroredToActivityCenter,
        M5DecisionFeedbackDisposition::Success,
        M5TransientSurfaceContext::SupportArea,
        M5ToastAcknowledgementScope::DurableOutcomeAck,
        M5ToastBacklinkTarget::ActivityCenter,
        true,
        true,
    );
    input.reconstructable_from_export = false;
    toast(input)
}

// -- Clean loading-state examples (treatment grammar across surfaces) ----------------------------

#[allow(clippy::too_many_arguments)]
fn clean_loading_base(
    loading_state_id: &str,
    label: &str,
    loading_fidelity: M5LoadingFidelity,
    disposition: M5DecisionFeedbackDisposition,
    surface: M5TransientSurfaceContext,
    loading_treatment: M5LoadingTreatment,
    readiness_posture: M5LoadingReadinessPosture,
    partial_content_available: bool,
) -> M5LoadingStateResolutionInput {
    M5LoadingStateResolutionInput {
        loading_state_id: loading_state_id.to_owned(),
        loading_label: label.to_owned(),
        loading_fidelity,
        disposition,
        surface_context: surface,
        loading_treatment,
        readiness_posture,
        partial_content_available,
        partial_content_preserved: partial_content_available,
        pane_blanked: false,
        overclaims_readiness: false,
        purpose_stated: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

/// Clean shell loading state: a skeleton that preserves the layout while first data warms.
fn loading_shell_skeleton() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:shell:skeleton",
        "Loading your workspaces — the layout is reserved while data warms",
        M5LoadingFidelity::SkeletonPreservesLayout,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::ShellEntry,
        M5LoadingTreatment::Skeleton,
        M5LoadingReadinessPosture::WarmingNotReady,
        false,
    ))
}

/// Clean review loading state: retained previous content shown while a refresh runs.
fn loading_review_retained() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:review:retained",
        "Refreshing the review index — the previous results stay visible",
        M5LoadingFidelity::PartialDataRetained,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::ReviewWorkspace,
        M5LoadingTreatment::RetainedPreviousContent,
        M5LoadingReadinessPosture::PartiallyReady,
        true,
    ))
}

/// Clean settings loading state: a stable placeholder that reserves space without implying data.
fn loading_settings_placeholder() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:settings:placeholder",
        "Checking capabilities — a stable placeholder holds the space",
        M5LoadingFidelity::InlineProgressScoped,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SettingsArea,
        M5LoadingTreatment::StablePlaceholder,
        M5LoadingReadinessPosture::WarmingNotReady,
        false,
    ))
}

/// Clean help loading state: partial results streaming in as they arrive.
fn loading_help_streaming() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:help:streaming",
        "Searching help — results stream in as they are found",
        M5LoadingFidelity::DeterminateProgress,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::HelpArea,
        M5LoadingTreatment::PartialResultsStreaming,
        M5LoadingReadinessPosture::PartiallyReady,
        true,
    ))
}

/// Clean support loading state: a blocked-waiting state that needs an action to proceed.
fn loading_support_blocked() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:support:blocked",
        "Diagnostics are waiting on a reconnect before they can finish",
        M5LoadingFidelity::IndeterminateSpinnerScoped,
        M5DecisionFeedbackDisposition::Blocked,
        M5TransientSurfaceContext::SupportArea,
        M5LoadingTreatment::BlockedWaiting,
        M5LoadingReadinessPosture::BlockedNeedsAction,
        false,
    ))
}

/// Clean support-export loading state: a skeleton that preserves the layout.
fn loading_export_skeleton() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:support:export",
        "Assembling the support bundle — the layout is reserved while it warms",
        M5LoadingFidelity::SkeletonPreservesLayout,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SupportArea,
        M5LoadingTreatment::Skeleton,
        M5LoadingReadinessPosture::WarmingNotReady,
        false,
    ))
}

// -- Degraded loading-state examples -------------------------------------------------------------

/// Degraded loading state: a useful pane is blanked while partial content is available.
fn loading_pane_blanked() -> M5ResolvedLoadingState {
    let mut input = clean_loading_base(
        "loading:shell:blanked",
        "Loading your workspaces",
        M5LoadingFidelity::SkeletonPreservesLayout,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::ShellEntry,
        M5LoadingTreatment::Skeleton,
        M5LoadingReadinessPosture::WarmingNotReady,
        true,
    );
    input.pane_blanked = true;
    loading_state(input)
}

/// Degraded loading state: the loading fidelity is the disallowed full-screen-spinner token.
fn loading_full_screen_spinner() -> M5ResolvedLoadingState {
    loading_state(clean_loading_base(
        "loading:review:spinner",
        "Loading the review index",
        M5LoadingFidelity::FullScreenSpinnerDisallowed,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::ReviewWorkspace,
        M5LoadingTreatment::Skeleton,
        M5LoadingReadinessPosture::WarmingNotReady,
        false,
    ))
}

/// Degraded loading state: readiness is overclaimed while data is warming.
fn loading_readiness_overclaimed() -> M5ResolvedLoadingState {
    let mut input = clean_loading_base(
        "loading:settings:overclaim",
        "Capabilities ready",
        M5LoadingFidelity::InlineProgressScoped,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SettingsArea,
        M5LoadingTreatment::StablePlaceholder,
        M5LoadingReadinessPosture::WarmingNotReady,
        false,
    );
    input.overclaims_readiness = true;
    loading_state(input)
}

/// Degraded loading state: useful partial content is not preserved.
fn loading_partial_not_preserved() -> M5ResolvedLoadingState {
    let mut input = clean_loading_base(
        "loading:help:dropped",
        "Searching help",
        M5LoadingFidelity::DeterminateProgress,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::HelpArea,
        M5LoadingTreatment::PartialResultsStreaming,
        M5LoadingReadinessPosture::PartiallyReady,
        true,
    );
    input.partial_content_preserved = false;
    loading_state(input)
}

/// Degraded loading state: what the pane is loading and why is unstated.
fn loading_purpose_unstated() -> M5ResolvedLoadingState {
    let mut input = clean_loading_base(
        "loading:support:no-purpose",
        "Please wait",
        M5LoadingFidelity::IndeterminateSpinnerScoped,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SupportArea,
        M5LoadingTreatment::BlockedWaiting,
        M5LoadingReadinessPosture::BlockedNeedsAction,
        false,
    );
    input.purpose_stated = false;
    loading_state(input)
}

/// Degraded loading state: the explanation cannot be reconstructed from the export.
fn loading_not_reconstructable() -> M5ResolvedLoadingState {
    let mut input = clean_loading_base(
        "loading:support:screenshot-only",
        "Assembling the support bundle",
        M5LoadingFidelity::SkeletonPreservesLayout,
        M5DecisionFeedbackDisposition::Pending,
        M5TransientSurfaceContext::SupportArea,
        M5LoadingTreatment::Skeleton,
        M5LoadingReadinessPosture::WarmingNotReady,
        false,
    );
    input.reconstructable_from_export = false;
    loading_state(input)
}

// -- Row builders --------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ToastLoadingConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    toast_examples: Vec<M5ResolvedToast>,
    loading_state_examples: Vec<M5ResolvedLoadingState>,
) -> M5ToastLoadingControlsRow {
    M5ToastLoadingControlsRow {
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
        anatomy_parts: M5ToastLoadingAnatomyPart::ALL.to_vec(),
        export_fields: M5ToastLoadingExportField::ALL.to_vec(),
        downgrade_triggers,
        toast_examples,
        loading_state_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TOAST_LOADING_CONTROLS_SCHEMA_REF,
            M5_TOAST_SCHEMA_REF,
            M5_LOADING_STATE_SCHEMA_REF,
        ]),
        toast_represents_durable_work_as_toast_only: false,
        toast_lacks_durable_backlink_when_outcome_matters: false,
        loading_blanks_useful_pane: false,
        loading_uses_full_screen_spinner_when_partial_capable: false,
    }
}

fn controls_rows() -> Vec<M5ToastLoadingControlsRow> {
    use M5DecisionFeedbackConsumerSurface as C;
    use M5DecisionFeedbackDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell / entry surface owner",
            "The shell toast is a transient confirmation, and its loading state is a layout-preserving skeleton; both degrade honestly when a durable backlink is missing or a useful pane is blanked",
            "evidence:m5-toast-loading-shell-ui:001",
            vec![D::DurableWorkShownAsToastOnly, D::UsefulPaneBlankedDuringLoading, D::ProofStale],
            vec![toast_shell_transient(), toast_backlink_missing()],
            vec![loading_shell_skeleton(), loading_pane_blanked()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review toast is a durable-outcome acknowledgement backed by the review queue, and its loading state retains the previous results while refreshing; both degrade honestly when the toast becomes the only durable truth or a full-screen spinner is used",
            "evidence:m5-toast-loading-review-ui:001",
            vec![D::DurableWorkShownAsToastOnly, D::FullScreenSpinnerWhenPartialCapable, D::ProofStale],
            vec![toast_review_durable(), toast_only_truth()],
            vec![loading_review_retained(), loading_full_screen_spinner()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings toast is a reversible-action acknowledgement with one bounded action backed by a record, and its loading state is a stable placeholder; both degrade honestly when the durability is toast-only or readiness is overclaimed",
            "evidence:m5-toast-loading-settings-ui:001",
            vec![D::DurableWorkShownAsToastOnly, D::StateTaxonomyDrifted, D::ProofStale],
            vec![toast_settings_reversible(), toast_durability_disallowed()],
            vec![loading_settings_placeholder(), loading_readiness_overclaimed()],
        ),
        base_row(
            C::HelpUi,
            "Help surface owner",
            "The help toast is a non-blocking notice, and its loading state streams partial search results; both degrade honestly when the acknowledgement scope is unresolved or partial content is not preserved",
            "evidence:m5-toast-loading-help-ui:001",
            vec![D::ScopeUnstated, D::UsefulPaneBlankedDuringLoading, D::ProofStale],
            vec![toast_help_notice(), toast_scope_unresolved()],
            vec![loading_help_streaming(), loading_partial_not_preserved()],
        ),
        base_row(
            C::SupportUi,
            "Support surface owner",
            "The support toast is a background handoff backed by a support record, and its loading state names a blocked-waiting state; both degrade honestly when a present action is unbounded or the loading purpose is unstated",
            "evidence:m5-toast-loading-support-ui:001",
            vec![D::RecoveryPathUnstated, D::RationaleUnstated, D::ProofStale],
            vec![toast_support_background(), toast_action_unbounded()],
            vec![loading_support_blocked(), loading_purpose_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved toast and loading-state truth, so a screenshot-only toast or loading state is visible in evidence rather than hidden, and the reason a toast appeared or a loading state persisted can be reconstructed at capture time without losing object identity",
            "evidence:m5-toast-loading-support-export:001",
            vec![D::GenericChromeWordingUsed, D::DurableWorkShownAsToastOnly, D::ProofStale],
            vec![toast_export_durable(), toast_not_reconstructable()],
            vec![loading_export_skeleton(), loading_not_reconstructable()],
        ),
    ]
}

fn governance_review() -> M5ToastLoadingGovernanceReview {
    M5ToastLoadingGovernanceReview {
        toast_acknowledges_transiently_with_named_scope: true,
        toast_points_back_to_durable_object_when_outcome_matters: true,
        toast_keeps_present_action_bounded: true,
        toast_never_only_durable_truth: true,
        loading_state_distinguishes_treatments: true,
        loading_state_preserves_partial_content: true,
        loading_state_never_blanks_useful_pane: true,
        loading_state_never_full_screen_spinner_when_partial_capable: true,
        loading_state_never_overclaims_readiness: true,
        both_reconstructable_from_export: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ToastLoadingConsumerProjection {
    M5ToastLoadingConsumerProjection {
        shell_surfaces_consume_toast_and_loading_vocabulary: true,
        review_surfaces_consume_toast_and_loading_vocabulary: true,
        settings_surfaces_consume_loading_vocabulary: true,
        help_surfaces_consume_loading_vocabulary: true,
        toast_and_loading_trace_to_single_component_contract: true,
        support_export_reads_single_toast_loading_source: true,
    }
}

fn proof_freshness() -> M5ToastLoadingProofFreshness {
    M5ToastLoadingProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ToastLoadingReleasePosture {
    M5ToastLoadingReleasePosture {
        proof_packet_ref: M5_TOAST_LOADING_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_TOAST_LOADING_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TOAST_LOADING_CONTROLS_SCHEMA_REF,
        M5_TOAST_LOADING_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_TOAST_SCHEMA_REF,
        M5_LOADING_STATE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 toast / loading-state controls packet.
pub fn seeded_m5_toast_loading_controls() -> M5ToastLoadingControlsPacket {
    M5ToastLoadingControlsPacket::new(M5ToastLoadingControlsPacketInput {
        packet_id: M5_TOAST_LOADING_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 toast and loading-state controls with acknowledgement-only semantics, one bounded action where appropriate, durable-object back-links whenever the outcome matters after dismissal, skeleton / retained-content / stable-placeholder / partial-streaming / blocked-waiting loading treatments rather than one spinner, and no toast-only truth or full-screen spinner across shell, review, settings, help, support, and support-export surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ToastLoadingVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the review-UI row is held at Beta pending toast durable-backlink parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_toast_loading_controls_review_ui_beta_narrowed() -> M5ToastLoadingControlsPacket {
    let mut packet = seeded_m5_toast_loading_controls();
    packet.packet_id = "m5-toast-and-loading-state-controls:review-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support-UI row is narrowed to Preview pending loading-treatment / readiness
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_toast_loading_controls_support_ui_preview_narrowed() -> M5ToastLoadingControlsPacket
{
    let mut packet = seeded_m5_toast_loading_controls();
    packet.packet_id = "m5-toast-and-loading-state-controls:support-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionFeedbackConsumerSurface::SupportUi)
        .expect("support-ui row present");
    row.qualification = M5DecisionFeedbackQualificationClass::Preview;
    packet
}
