//! Canonical seed builders for the M5 workspace-expiry-banner / local-safe-continuation-card
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

use crate::managed_workspace_lifecycle::{
    ContinuityClass, ExpiryClass, PersistenceClass, RecoveryOptionClass, TransitionReasonClass,
};

/// Stable packet id for the canonical controls packet.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_PACKET_ID: &str =
    "m5-workspace-expiry-banner-local-safe-continuation-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn banner(input: M5WorkspaceExpiryBannerResolutionInput) -> M5ResolvedWorkspaceExpiryBanner {
    resolve_workspace_expiry_banner(input).expect("seed expiry banner input resolves")
}

fn card(input: M5LocalSafeContinuationCardResolutionInput) -> M5ResolvedLocalSafeContinuationCard {
    resolve_local_safe_continuation_card(input).expect("seed local-safe card input resolves")
}

// -- Canonical workspace-expiry banner examples --------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn banner_input(
    banner_id: &str,
    workspace_label: &str,
    expiry_class: ExpiryClass,
    triggering_reason: TransitionReasonClass,
    affected_capabilities: Vec<M5WorkspaceLiveCapability>,
    offered_actions: Vec<M5WorkspaceExpiryAction>,
    renew_reopen_allowed: bool,
    continuity_class: ContinuityClass,
    material_change_present: bool,
) -> M5WorkspaceExpiryBannerResolutionInput {
    M5WorkspaceExpiryBannerResolutionInput {
        banner_id: banner_id.to_owned(),
        workspace_label: workspace_label.to_owned(),
        expiry_class,
        expiry_disclosed: true,
        triggering_reason,
        triggering_source_disclosed: true,
        affected_capabilities,
        capabilities_disclosed: true,
        offered_actions,
        renew_reopen_allowed,
        continuity_class,
        material_change_present,
        proof_fresh: true,
    }
}

/// Clean banner: an idle window will suspend the workspace; export / renew are offered before loss.
fn banner_idle_window() -> M5ResolvedWorkspaceExpiryBanner {
    banner(banner_input(
        "expiry-banner:idle-window",
        "web-frontend@managed-ws",
        ExpiryClass::IdleWindow,
        TransitionReasonClass::IdleWindowElapsed,
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Kernels,
        ],
        vec![
            M5WorkspaceExpiryAction::ExportBeforeLoss,
            M5WorkspaceExpiryAction::Renew,
            M5WorkspaceExpiryAction::ContinueLocalSafe,
        ],
        true,
        ContinuityClass::ExactContinuity,
        false,
    ))
}

/// Clean banner: a hibernation window will expire the workspace after suspension.
fn banner_hibernation_window() -> M5ResolvedWorkspaceExpiryBanner {
    banner(banner_input(
        "expiry-banner:hibernation-window",
        "web-frontend@managed-ws",
        ExpiryClass::HibernationWindow,
        TransitionReasonClass::HibernationWindowElapsed,
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Ports,
            M5WorkspaceLiveCapability::BackgroundJobs,
        ],
        vec![
            M5WorkspaceExpiryAction::ExportBeforeLoss,
            M5WorkspaceExpiryAction::Reopen,
            M5WorkspaceExpiryAction::ContinueLocalSafe,
        ],
        true,
        ContinuityClass::MaterialChange,
        true,
    ))
}

/// Clean banner: a hard deadline expires the workspace regardless of activity; the runtime is gone.
fn banner_hard_deadline() -> M5ResolvedWorkspaceExpiryBanner {
    banner(banner_input(
        "expiry-banner:hard-deadline",
        "web-frontend@managed-ws",
        ExpiryClass::HardDeadline,
        TransitionReasonClass::ExpiryDeadlineReached,
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Ports,
            M5WorkspaceLiveCapability::Kernels,
            M5WorkspaceLiveCapability::Previews,
        ],
        vec![
            M5WorkspaceExpiryAction::ExportBeforeLoss,
            M5WorkspaceExpiryAction::ContinueLocalSafe,
            M5WorkspaceExpiryAction::ContactOperator,
        ],
        false,
        ContinuityClass::LocalSafeOnly,
        true,
    ))
}

/// Clean banner: a control-plane outage clock governs the local-safe grace window.
fn banner_control_plane_outage() -> M5ResolvedWorkspaceExpiryBanner {
    banner(banner_input(
        "expiry-banner:control-plane-outage",
        "web-frontend@managed-ws",
        ExpiryClass::ControlPlaneOutage,
        TransitionReasonClass::ControlPlaneFailure,
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Ports,
            M5WorkspaceLiveCapability::Kernels,
            M5WorkspaceLiveCapability::Previews,
            M5WorkspaceLiveCapability::BackgroundJobs,
        ],
        vec![
            M5WorkspaceExpiryAction::ExportBeforeLoss,
            M5WorkspaceExpiryAction::ContinueLocalSafe,
            M5WorkspaceExpiryAction::ContactOperator,
        ],
        false,
        ContinuityClass::LocalSafeOnly,
        true,
    ))
}

/// Degraded banner: the exact expiry timing is undisclosed — proves AC1's generic-disconnect half.
fn banner_timing_unstated() -> M5ResolvedWorkspaceExpiryBanner {
    let mut input = banner_input(
        "expiry-banner:timing-hidden",
        "web-frontend@managed-ws",
        ExpiryClass::IdleWindow,
        TransitionReasonClass::IdleWindowElapsed,
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![M5WorkspaceExpiryAction::ExportBeforeLoss],
        true,
        ContinuityClass::ExactContinuity,
        false,
    );
    input.expiry_disclosed = false;
    banner(input)
}

/// Degraded banner: the triggering owner / source is undisclosed — proves AC1's silent-loss half.
fn banner_source_unstated() -> M5ResolvedWorkspaceExpiryBanner {
    let mut input = banner_input(
        "expiry-banner:source-hidden",
        "web-frontend@managed-ws",
        ExpiryClass::HibernationWindow,
        TransitionReasonClass::HibernationWindowElapsed,
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![M5WorkspaceExpiryAction::ExportBeforeLoss],
        true,
        ContinuityClass::MaterialChange,
        true,
    );
    input.triggering_source_disclosed = false;
    banner(input)
}

/// Degraded banner: the affected capabilities are undisclosed.
fn banner_capabilities_unstated() -> M5ResolvedWorkspaceExpiryBanner {
    let mut input = banner_input(
        "expiry-banner:capabilities-hidden",
        "web-frontend@managed-ws",
        ExpiryClass::HibernationWindow,
        TransitionReasonClass::HibernationWindowElapsed,
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![M5WorkspaceExpiryAction::ExportBeforeLoss],
        true,
        ContinuityClass::MaterialChange,
        true,
    );
    input.capabilities_disclosed = false;
    banner(input)
}

/// Degraded banner: no export-before-loss or renew / reopen action is offered.
fn banner_export_action_missing() -> M5ResolvedWorkspaceExpiryBanner {
    banner(banner_input(
        "expiry-banner:no-action",
        "web-frontend@managed-ws",
        ExpiryClass::ControlPlaneOutage,
        TransitionReasonClass::ControlPlaneFailure,
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Ports,
        ],
        vec![],
        false,
        ContinuityClass::LocalSafeOnly,
        true,
    ))
}

/// Degraded banner: a gone runtime is presented as exact continuity — proves the continuity
/// guardrail.
fn banner_continuity_overclaimed() -> M5ResolvedWorkspaceExpiryBanner {
    banner(banner_input(
        "expiry-banner:continuity-overclaimed",
        "web-frontend@managed-ws",
        ExpiryClass::HardDeadline,
        TransitionReasonClass::ExpiryDeadlineReached,
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Kernels,
        ],
        vec![
            M5WorkspaceExpiryAction::ExportBeforeLoss,
            M5WorkspaceExpiryAction::ContinueLocalSafe,
        ],
        false,
        ContinuityClass::ExactContinuity,
        true,
    ))
}

// -- Canonical local-safe continuation card examples ---------------------------------------------

#[allow(clippy::too_many_arguments)]
fn card_input(
    card_id: &str,
    workspace_label: &str,
    persistence_class: PersistenceClass,
    continuity_class: ContinuityClass,
    preserved_context: Vec<M5PreservedContextClass>,
    lost_live_state: Vec<M5WorkspaceLiveCapability>,
    next_actions: Vec<RecoveryOptionClass>,
    material_change_present: bool,
) -> M5LocalSafeContinuationCardResolutionInput {
    M5LocalSafeContinuationCardResolutionInput {
        card_id: card_id.to_owned(),
        workspace_label: workspace_label.to_owned(),
        persistence_class,
        continuity_class,
        preserved_context,
        preserved_disclosed: true,
        lost_live_state,
        lost_disclosed: true,
        next_actions,
        next_actions_disclosed: true,
        material_change_present,
        proof_fresh: true,
    }
}

/// Clean card: a control-plane outage dropped the runtime; work continues locally while reconnecting.
fn card_reconnect_local_safe() -> M5ResolvedLocalSafeContinuationCard {
    card(card_input(
        "local-safe-card:reconnect",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        vec![
            M5PreservedContextClass::WorkingTreeFiles,
            M5PreservedContextClass::UnsavedEdits,
            M5PreservedContextClass::Checkpoints,
        ],
        vec![
            M5WorkspaceLiveCapability::Terminals,
            M5WorkspaceLiveCapability::Ports,
            M5WorkspaceLiveCapability::Kernels,
        ],
        vec![
            RecoveryOptionClass::LocalSafeContinue,
            RecoveryOptionClass::Reconnect,
        ],
        true,
    ))
}

/// Clean card: the workspace expired and must be rebuilt; the local mirror carries files forward.
fn card_rebuild_local_safe() -> M5ResolvedLocalSafeContinuationCard {
    card(card_input(
        "local-safe-card:rebuild",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        vec![
            M5PreservedContextClass::WorkingTreeFiles,
            M5PreservedContextClass::EnvironmentConfig,
            M5PreservedContextClass::CommandHistory,
        ],
        vec![
            M5WorkspaceLiveCapability::Kernels,
            M5WorkspaceLiveCapability::Previews,
            M5WorkspaceLiveCapability::BackgroundJobs,
            M5WorkspaceLiveCapability::DebugSessions,
        ],
        vec![
            RecoveryOptionClass::LocalSafeContinue,
            RecoveryOptionClass::Rebuild,
            RecoveryOptionClass::Recreate,
        ],
        true,
    ))
}

/// Degraded card: the preserved files / context are undisclosed — proves AC2's preserved half.
fn card_preserved_unstated() -> M5ResolvedLocalSafeContinuationCard {
    let mut input = card_input(
        "local-safe-card:preserved-hidden",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        vec![M5PreservedContextClass::WorkingTreeFiles],
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![RecoveryOptionClass::LocalSafeContinue],
        true,
    );
    input.preserved_disclosed = false;
    card(input)
}

/// Degraded card: the lost live state is undisclosed — proves AC2's lost half.
fn card_lost_unstated() -> M5ResolvedLocalSafeContinuationCard {
    let mut input = card_input(
        "local-safe-card:lost-hidden",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        vec![M5PreservedContextClass::WorkingTreeFiles],
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![RecoveryOptionClass::LocalSafeContinue],
        true,
    );
    input.lost_disclosed = false;
    card(input)
}

/// Degraded card: the next safe actions are undisclosed.
fn card_next_actions_unstated() -> M5ResolvedLocalSafeContinuationCard {
    let mut input = card_input(
        "local-safe-card:actions-hidden",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        vec![M5PreservedContextClass::WorkingTreeFiles],
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![RecoveryOptionClass::LocalSafeContinue],
        true,
    );
    input.next_actions_disclosed = false;
    card(input)
}

/// Degraded card: the card offers no local-safe continuation route — proves the local-safe guardrail.
fn card_local_safe_unavailable() -> M5ResolvedLocalSafeContinuationCard {
    card(card_input(
        "local-safe-card:local-safe-hidden",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        vec![M5PreservedContextClass::WorkingTreeFiles],
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![
            RecoveryOptionClass::Reconnect,
            RecoveryOptionClass::ContactOperator,
        ],
        true,
    ))
}

/// Degraded card: a materially changed / gone runtime is presented as exact continuity — proves the
/// continuity guardrail.
fn card_continuity_overclaimed() -> M5ResolvedLocalSafeContinuationCard {
    card(card_input(
        "local-safe-card:continuity-overclaimed",
        "web-frontend@managed-ws",
        PersistenceClass::LocalMirror,
        ContinuityClass::ExactContinuity,
        vec![M5PreservedContextClass::WorkingTreeFiles],
        vec![M5WorkspaceLiveCapability::Terminals],
        vec![
            RecoveryOptionClass::LocalSafeContinue,
            RecoveryOptionClass::Rebuild,
        ],
        true,
    ))
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ExpiryContinuationConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    expiry_banner_examples: Vec<M5ResolvedWorkspaceExpiryBanner>,
    local_safe_card_examples: Vec<M5ResolvedLocalSafeContinuationCard>,
) -> M5ExpiryContinuationControlsRow {
    M5ExpiryContinuationControlsRow {
        consumer_surface,
        qualification: M5BuildRemoteQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5BuildRemoteDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5BuildRemoteRequiredLabel::Identity,
            M5BuildRemoteRequiredLabel::State,
            M5BuildRemoteRequiredLabel::KeyboardRoute,
            M5BuildRemoteRequiredLabel::LifecycleAndContinuity,
        ],
        accessibility_routes: M5BuildRemoteAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ExpiryContinuationAnatomyPart::ALL.to_vec(),
        export_fields: M5ExpiryContinuationExportField::ALL.to_vec(),
        downgrade_triggers,
        expiry_banner_examples,
        local_safe_card_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_REF,
            M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
            M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
        ]),
        implies_exact_continuity_after_material_change: false,
        hides_local_safe_or_companion_handoff_in_overflow_only: false,
        expiry_appears_as_generic_disconnect_or_silent_loss: false,
        conceals_preserved_vs_lost_state_or_next_safe_actions: false,
    }
}

fn controls_rows() -> Vec<M5ExpiryContinuationControlsRow> {
    use M5BuildRemoteConsumerSurface as C;
    use M5BuildRemoteDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell renders a workspace-expiry banner naming its exact expiry timing, triggering owner/source, affected capabilities, and export-before-loss or renew/reopen actions, so an idle-window expiry never reads as a generic disconnect; the local-safe continuation card names what remains local-safe and what live state is lost",
            "evidence:m5-expiry-continuation-shell:001",
            vec![
                D::ExpiryTimingUnstated,
                D::LocalSafeOrCompanionHandoffOverflowOnly,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![banner_idle_window(), banner_timing_unstated()],
            vec![card_reconnect_local_safe(), card_preserved_unstated()],
        ),
        base_row(
            C::PreviewUi,
            "Preview surface owner",
            "Preview targets reuse the same expiry-banner and local-safe continuation vocabulary, distinguishing hibernation-window expiry and degrading honestly when the triggering source or affected capabilities are unstated; the continuation card names its lost previews and background jobs",
            "evidence:m5-expiry-continuation-preview:001",
            vec![
                D::ExpiryTimingUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![
                banner_hibernation_window(),
                banner_source_unstated(),
                banner_capabilities_unstated(),
            ],
            vec![card_rebuild_local_safe(), card_lost_unstated()],
        ),
        base_row(
            C::CompanionUi,
            "Companion surface owner",
            "Companion handoff reuses the same expiry banner and local-safe continuation cards so a hard-deadline expiry is distinguishable before the user loses context, and both components degrade rather than present a gone runtime as exact continuity",
            "evidence:m5-expiry-continuation-companion:001",
            vec![
                D::ExactContinuityOverclaimed,
                D::LocalSafeOrCompanionHandoffOverflowOnly,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![banner_hard_deadline(), banner_continuity_overclaimed()],
            vec![card_reconnect_local_safe(), card_continuity_overclaimed()],
        ),
        base_row(
            C::IncidentUi,
            "Incident/ops surface owner",
            "Incident and ops surfaces keep the same expiry and fallback language, distinguishing a control-plane outage and degrading honestly when a banner offers no export-before-loss route or a continuation card hides local-safe continuation",
            "evidence:m5-expiry-continuation-incident:001",
            vec![
                D::LocalSafeOrCompanionHandoffOverflowOnly,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![banner_control_plane_outage(), banner_export_action_missing()],
            vec![card_local_safe_unavailable(), card_next_actions_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved expiry-banner and local-safe continuation truth, so an unstated timing, an undisclosed preserved-vs-lost state, or a hidden local-safe continuation is visible in evidence rather than hidden behind feature-local prose",
            "evidence:m5-expiry-continuation-support-export:001",
            vec![
                D::ExpiryTimingUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![banner_idle_window()],
            vec![card_rebuild_local_safe(), card_preserved_unstated()],
        ),
    ]
}

fn governance_review() -> M5ExpiryContinuationGovernanceReview {
    M5ExpiryContinuationGovernanceReview {
        banner_names_expiry_timing_and_triggering_source: true,
        banner_names_affected_capabilities_and_actions: true,
        expiry_never_appears_as_generic_disconnect: true,
        card_names_preserved_and_lost_live_state: true,
        card_names_next_safe_actions: true,
        local_safe_continuation_never_overflow_only: true,
        material_change_never_implies_exact_continuity: true,
        export_before_loss_action_always_available: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ExpiryContinuationConsumerProjection {
    M5ExpiryContinuationConsumerProjection {
        shell_surfaces_consume_expiry_vocabulary: true,
        preview_surfaces_consume_expiry_vocabulary: true,
        companion_surfaces_reuse_expiry_banner_and_continuation_cards: true,
        incident_ops_consumes_expiry_vocabulary: true,
        support_export_reads_single_expiry_source: true,
        expiry_and_fallback_language_consistent_across_surfaces: true,
    }
}

fn proof_freshness() -> M5ExpiryContinuationProofFreshness {
    M5ExpiryContinuationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ExpiryContinuationReleasePosture {
    M5ExpiryContinuationReleasePosture {
        proof_packet_ref: M5_EXPIRY_CONTINUATION_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_EXPIRY_CONTINUATION_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_REF,
        M5_EXPIRY_CONTINUATION_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
        M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
        M5_EXPIRY_CONTINUATION_OBJECT_MODEL_DOC_REF,
    ])
}

/// Builds the canonical M5 workspace-expiry-banner / local-safe-continuation-card controls packet.
pub fn seeded_m5_expiry_continuation_controls() -> M5ExpiryContinuationControlsPacket {
    M5ExpiryContinuationControlsPacket::new(M5ExpiryContinuationControlsPacketInput {
        packet_id: M5_EXPIRY_CONTINUATION_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 workspace-expiry-banner and local-safe-continuation-card controls with expiry timing, triggering owner/source, affected capabilities, export-before-loss and renew/reopen actions, preserved files/context, lost live state, next safe actions, and no-exact-continuity-overclaim truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ExpiryContinuationVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell row is held at Beta pending expiry-banner parity on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_expiry_continuation_controls_expiry_banner_beta_narrowed(
) -> M5ExpiryContinuationControlsPacket {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.packet_id =
        "m5-workspace-expiry-banner-local-safe-continuation-card-controls:expiry-banner-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::ShellUi)
        .expect("shell row present");
    row.qualification = M5BuildRemoteQualificationClass::Beta;
    packet
}

/// Narrowed variant: the preview row is narrowed to Preview pending local-safe-card parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_expiry_continuation_controls_local_safe_card_preview_narrowed(
) -> M5ExpiryContinuationControlsPacket {
    let mut packet = seeded_m5_expiry_continuation_controls();
    packet.packet_id =
        "m5-workspace-expiry-banner-local-safe-continuation-card-controls:local-safe-card-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .expect("preview row present");
    row.qualification = M5BuildRemoteQualificationClass::Preview;
    packet
}
