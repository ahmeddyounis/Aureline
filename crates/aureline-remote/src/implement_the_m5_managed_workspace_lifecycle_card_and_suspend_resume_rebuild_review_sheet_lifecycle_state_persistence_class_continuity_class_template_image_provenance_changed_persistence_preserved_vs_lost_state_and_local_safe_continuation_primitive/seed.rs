//! Canonical seed builders for the M5 managed-workspace-lifecycle-card /
//! suspend-resume-rebuild-review-sheet controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

use crate::managed_workspace_lifecycle::{
    CaveatClass, ContinuityClass, ExpiryClass, LifecycleStateClass, PersistenceClass,
    ProvenanceClass, RecoveryOptionClass,
};

/// Stable packet id for the canonical controls packet.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_PACKET_ID: &str =
    "m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card(
    input: M5ManagedWorkspaceLifecycleCardResolutionInput,
) -> M5ResolvedManagedWorkspaceLifecycleCard {
    resolve_managed_workspace_lifecycle_card(input).expect("seed lifecycle card input resolves")
}

fn sheet(
    input: M5SuspendResumeRebuildReviewSheetResolutionInput,
) -> M5ResolvedSuspendResumeRebuildReviewSheet {
    resolve_suspend_resume_rebuild_review_sheet(input).expect("seed review sheet input resolves")
}

// -- Canonical managed-workspace lifecycle card examples -----------------------------------------

#[allow(clippy::too_many_arguments)]
fn card_input(
    card_id: &str,
    workspace_label: &str,
    lifecycle_state: LifecycleStateClass,
    persistence_class: PersistenceClass,
    continuity_class: ContinuityClass,
    expiry_class: ExpiryClass,
    expiry_disclosed: bool,
    recovery_options: Vec<RecoveryOptionClass>,
    local_safe_offered: bool,
    material_change_present: bool,
) -> M5ManagedWorkspaceLifecycleCardResolutionInput {
    M5ManagedWorkspaceLifecycleCardResolutionInput {
        card_id: card_id.to_owned(),
        workspace_label: workspace_label.to_owned(),
        lifecycle_state,
        state_disclosed: true,
        persistence_class,
        persistence_disclosed: true,
        continuity_class,
        continuity_disclosed: true,
        expiry_class,
        expiry_disclosed,
        recovery_options,
        local_safe_offered,
        material_change_present,
        proof_fresh: true,
    }
}

/// Clean card: the control plane is provisioning a fresh workspace.
fn card_provision() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:provision",
        "web-frontend@managed-ws",
        LifecycleStateClass::Provision,
        PersistenceClass::RebuiltFresh,
        ContinuityClass::FreshNoContinuity,
        ExpiryClass::None,
        false,
        vec![],
        false,
        false,
    ))
}

/// Clean card: the workspace is warming.
fn card_warm() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:warm",
        "web-frontend@managed-ws",
        LifecycleStateClass::Warm,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::None,
        false,
        vec![],
        false,
        false,
    ))
}

/// Clean card: the workspace is ready for interactive work under an idle window.
fn card_ready() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:ready",
        "web-frontend@managed-ws",
        LifecycleStateClass::Ready,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::IdleWindow,
        true,
        vec![],
        false,
        false,
    ))
}

/// Clean card: the workspace is suspended under a hibernation window.
fn card_suspended() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:suspended",
        "web-frontend@managed-ws",
        LifecycleStateClass::Suspended,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::HibernationWindow,
        true,
        vec![RecoveryOptionClass::Resume],
        false,
        false,
    ))
}

/// Clean card: the workspace resumed with no material change, so exact continuity is honest.
fn card_resumed() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:resumed",
        "web-frontend@managed-ws",
        LifecycleStateClass::Resumed,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::None,
        false,
        vec![],
        false,
        false,
    ))
}

/// Clean card: the connection dropped and is being re-established; local-safe continuation applies.
fn card_reconnecting() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:reconnecting",
        "web-frontend@managed-ws",
        LifecycleStateClass::Reconnecting,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::ControlPlaneOutage,
        true,
        vec![
            RecoveryOptionClass::Reconnect,
            RecoveryOptionClass::LocalSafeContinue,
        ],
        true,
        false,
    ))
}

/// Clean card: a successor image requires a rebuild; the material change is named, not implied away.
fn card_rebuild_required() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:rebuild-required",
        "web-frontend@managed-ws",
        LifecycleStateClass::RebuildRequired,
        PersistenceClass::RebuiltFresh,
        ContinuityClass::MaterialChange,
        ExpiryClass::None,
        false,
        vec![
            RecoveryOptionClass::Rebuild,
            RecoveryOptionClass::LocalSafeContinue,
        ],
        true,
        true,
    ))
}

/// Clean card: the workspace must be recreated from scratch; there is no continuity.
fn card_recreate_required() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:recreate-required",
        "web-frontend@managed-ws",
        LifecycleStateClass::RecreateRequired,
        PersistenceClass::RecreatedNew,
        ContinuityClass::FreshNoContinuity,
        ExpiryClass::None,
        false,
        vec![
            RecoveryOptionClass::Recreate,
            RecoveryOptionClass::LocalSafeContinue,
        ],
        true,
        true,
    ))
}

/// Clean card: the workspace expired under a hard deadline; only a local-safe mirror remains.
fn card_expired() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:expired",
        "web-frontend@managed-ws",
        LifecycleStateClass::Expired,
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        ExpiryClass::HardDeadline,
        true,
        vec![
            RecoveryOptionClass::Recreate,
            RecoveryOptionClass::LocalSafeContinue,
        ],
        true,
        true,
    ))
}

/// Clean card: the control plane is unreachable; work continues against a local-safe mirror.
fn card_local_safe_continuation() -> M5ResolvedManagedWorkspaceLifecycleCard {
    card(card_input(
        "lifecycle-card:local-safe-continuation",
        "web-frontend@managed-ws",
        LifecycleStateClass::LocalSafeContinuation,
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        ExpiryClass::ControlPlaneOutage,
        true,
        vec![
            RecoveryOptionClass::LocalSafeContinue,
            RecoveryOptionClass::ContactOperator,
        ],
        true,
        true,
    ))
}

/// Degraded card: the lifecycle state is undisclosed — proves AC1's state half.
fn card_state_unstated() -> M5ResolvedManagedWorkspaceLifecycleCard {
    let mut input = card_input(
        "lifecycle-card:state-hidden",
        "web-frontend@managed-ws",
        LifecycleStateClass::Ready,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::IdleWindow,
        true,
        vec![],
        false,
        false,
    );
    input.state_disclosed = false;
    card(input)
}

/// Degraded card: the persistence class is undisclosed.
fn card_persistence_unstated() -> M5ResolvedManagedWorkspaceLifecycleCard {
    let mut input = card_input(
        "lifecycle-card:persistence-hidden",
        "web-frontend@managed-ws",
        LifecycleStateClass::Resumed,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::None,
        false,
        vec![],
        false,
        false,
    );
    input.persistence_disclosed = false;
    card(input)
}

/// Degraded card: the continuity class is undisclosed on a material change.
fn card_continuity_unstated() -> M5ResolvedManagedWorkspaceLifecycleCard {
    let mut input = card_input(
        "lifecycle-card:continuity-hidden",
        "web-frontend@managed-ws",
        LifecycleStateClass::RebuildRequired,
        PersistenceClass::RebuiltFresh,
        ContinuityClass::MaterialChange,
        ExpiryClass::None,
        false,
        vec![
            RecoveryOptionClass::Rebuild,
            RecoveryOptionClass::LocalSafeContinue,
        ],
        true,
        true,
    );
    input.continuity_disclosed = false;
    card(input)
}

/// Degraded card: an expiry window governs the state but the expiry timing is undisclosed.
fn card_expiry_unstated() -> M5ResolvedManagedWorkspaceLifecycleCard {
    let mut input = card_input(
        "lifecycle-card:expiry-hidden",
        "web-frontend@managed-ws",
        LifecycleStateClass::Suspended,
        PersistenceClass::PersistentVolume,
        ContinuityClass::ExactContinuity,
        ExpiryClass::HibernationWindow,
        false,
        vec![RecoveryOptionClass::Resume],
        false,
        false,
    );
    input.expiry_disclosed = false;
    card(input)
}

/// Degraded card: an expired outage state hides local-safe continuation — proves the local-safe
/// guardrail.
fn card_local_safe_unavailable() -> M5ResolvedManagedWorkspaceLifecycleCard {
    let mut input = card_input(
        "lifecycle-card:local-safe-hidden",
        "web-frontend@managed-ws",
        LifecycleStateClass::Expired,
        PersistenceClass::LocalMirror,
        ContinuityClass::LocalSafeOnly,
        ExpiryClass::HardDeadline,
        true,
        vec![RecoveryOptionClass::Recreate],
        false,
        true,
    );
    input.local_safe_offered = false;
    card(input)
}

// -- Canonical suspend / resume / rebuild review sheet examples ----------------------------------

#[allow(clippy::too_many_arguments)]
fn sheet_input(
    sheet_id: &str,
    workspace_label: &str,
    action: M5ManagedWorkspaceAction,
    provenance_class: ProvenanceClass,
    persistence_class: PersistenceClass,
    persistence_changed: bool,
    continuity_class: ContinuityClass,
    caveats: Vec<CaveatClass>,
    material_change_present: bool,
) -> M5SuspendResumeRebuildReviewSheetResolutionInput {
    M5SuspendResumeRebuildReviewSheetResolutionInput {
        sheet_id: sheet_id.to_owned(),
        workspace_label: workspace_label.to_owned(),
        action,
        action_disclosed: true,
        provenance_class,
        provenance_disclosed: true,
        persistence_class,
        persistence_changed,
        persistence_change_disclosed: true,
        continuity_class,
        preserved_state_disclosed: true,
        lost_state_disclosed: true,
        consequences_disclosed: true,
        shown_before_commit: true,
        caveats,
        material_change_present,
        proof_fresh: true,
    }
}

/// Clean sheet: a resume with no material change, shown before commit, exact continuity honest.
fn sheet_resume_clean() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    sheet(sheet_input(
        "review-sheet:resume-clean",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Resume,
        ProvenanceClass::PinnedDigest,
        PersistenceClass::PersistentVolume,
        false,
        ContinuityClass::ExactContinuity,
        vec![],
        false,
    ))
}

/// Clean sheet: a rebuild that names its successor-image provenance, changed persistence class, and
/// caveats before commit, so the materially different workspace is surfaced, not implied away.
fn sheet_rebuild_clean() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    sheet(sheet_input(
        "review-sheet:rebuild-clean",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Rebuild,
        ProvenanceClass::SuccessorImage,
        PersistenceClass::RebuiltFresh,
        true,
        ContinuityClass::MaterialChange,
        vec![
            CaveatClass::ImageChanged,
            CaveatClass::ScratchStateDiscarded,
        ],
        true,
    ))
}

/// Degraded sheet: the template / image provenance is undisclosed.
fn sheet_provenance_unstated() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    let mut input = sheet_input(
        "review-sheet:provenance-hidden",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Rebuild,
        ProvenanceClass::SuccessorImage,
        PersistenceClass::RebuiltFresh,
        true,
        ContinuityClass::MaterialChange,
        vec![CaveatClass::ImageChanged],
        true,
    );
    input.provenance_disclosed = false;
    sheet(input)
}

/// Degraded sheet: a changed persistence class is hidden.
fn sheet_persistence_change_hidden() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    let mut input = sheet_input(
        "review-sheet:persistence-hidden",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Rebuild,
        ProvenanceClass::SuccessorImage,
        PersistenceClass::RebuiltFresh,
        true,
        ContinuityClass::MaterialChange,
        vec![CaveatClass::PersistenceClassChanged],
        true,
    );
    input.persistence_change_disclosed = false;
    sheet(input)
}

/// Degraded sheet: the preserved-vs-lost state is undisclosed.
fn sheet_preserved_lost_unstated() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    let mut input = sheet_input(
        "review-sheet:preserved-lost-hidden",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Resume,
        ProvenanceClass::PinnedDigest,
        PersistenceClass::PersistentVolume,
        false,
        ContinuityClass::ExactContinuity,
        vec![],
        false,
    );
    input.lost_state_disclosed = false;
    sheet(input)
}

/// Degraded sheet: the reattach / rerun consequences are undisclosed.
fn sheet_consequences_unstated() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    let mut input = sheet_input(
        "review-sheet:consequences-hidden",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Resume,
        ProvenanceClass::PinnedDigest,
        PersistenceClass::PersistentVolume,
        false,
        ContinuityClass::ExactContinuity,
        vec![],
        false,
    );
    input.consequences_disclosed = false;
    sheet(input)
}

/// Degraded sheet: a materially changed runtime is presented as exact continuity — proves the
/// continuity guardrail.
fn sheet_exact_continuity_overclaimed() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    sheet(sheet_input(
        "review-sheet:continuity-overclaimed",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Resume,
        ProvenanceClass::SuccessorImage,
        PersistenceClass::SnapshotRestored,
        true,
        ContinuityClass::ExactContinuity,
        vec![CaveatClass::ImageChanged],
        true,
    ))
}

/// Degraded sheet: the review would appear after the action it gates — proves AC2.
fn sheet_shown_after_commit() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    let mut input = sheet_input(
        "review-sheet:after-the-fact",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Recreate,
        ProvenanceClass::DriftedUnpinned,
        PersistenceClass::RecreatedNew,
        true,
        ContinuityClass::FreshNoContinuity,
        vec![CaveatClass::TargetIdentityChanged],
        true,
    );
    input.shown_before_commit = false;
    sheet(input)
}

/// Degraded sheet: the action class is undisclosed.
fn sheet_action_unstated() -> M5ResolvedSuspendResumeRebuildReviewSheet {
    let mut input = sheet_input(
        "review-sheet:action-hidden",
        "web-frontend@managed-ws",
        M5ManagedWorkspaceAction::Resume,
        ProvenanceClass::PinnedDigest,
        PersistenceClass::PersistentVolume,
        false,
        ContinuityClass::ExactContinuity,
        vec![],
        false,
    );
    input.action_disclosed = false;
    sheet(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ManagedLifecycleConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    lifecycle_card_examples: Vec<M5ResolvedManagedWorkspaceLifecycleCard>,
    review_sheet_examples: Vec<M5ResolvedSuspendResumeRebuildReviewSheet>,
) -> M5ManagedLifecycleControlsRow {
    M5ManagedLifecycleControlsRow {
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
        anatomy_parts: M5ManagedLifecycleAnatomyPart::ALL.to_vec(),
        export_fields: M5ManagedLifecycleExportField::ALL.to_vec(),
        downgrade_triggers,
        lifecycle_card_examples,
        review_sheet_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_REF,
            M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
            M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
        ]),
        implies_exact_continuity_after_material_change: false,
        hides_local_safe_or_companion_handoff_in_overflow_only: false,
        review_sheet_appears_after_the_fact: false,
        conceals_lifecycle_or_continuity_in_generic_status_wording: false,
    }
}

fn controls_rows() -> Vec<M5ManagedLifecycleControlsRow> {
    use M5BuildRemoteConsumerSurface as C;
    use M5BuildRemoteDowngradeTrigger as D;

    vec![
        base_row(
            C::RunTestDebugUi,
            "Run/test/debug surface owner",
            "Every run, test, and debug target renders a managed-workspace lifecycle card naming its lifecycle state, persistence class, continuity class, and expiry timing before the user trusts a target; the suspend/resume/rebuild review sheet names its action class, template/image provenance, changed persistence, preserved-vs-lost state, and reattach/rerun consequences before commit",
            "evidence:m5-managed-lifecycle-run-test-debug:001",
            vec![
                D::LifecycleStateUnstated,
                D::ExpiryTimingUnstated,
                D::ExactContinuityOverclaimed,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![card_provision(), card_warm(), card_ready(), card_state_unstated()],
            vec![sheet_resume_clean(), sheet_action_unstated()],
        ),
        base_row(
            C::PreviewUi,
            "Preview surface owner",
            "Preview targets reuse the same lifecycle card and review-sheet vocabulary, distinguishing suspended and resumed states and degrading honestly when the persistence class or expiry timing is unstated; the rebuild review sheet names its successor-image provenance and changed persistence before commit",
            "evidence:m5-managed-lifecycle-preview:001",
            vec![
                D::LifecycleStateUnstated,
                D::PersistenceChangeHidden,
                D::ExpiryTimingUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_suspended(),
                card_resumed(),
                card_persistence_unstated(),
                card_expiry_unstated(),
            ],
            vec![sheet_rebuild_clean(), sheet_persistence_change_hidden()],
        ),
        base_row(
            C::CompanionUi,
            "Companion surface owner",
            "Companion handoff reuses the same lifecycle cards and review language so a reconnecting or rebuild-required workspace is distinguishable before the user acts, and the review sheet degrades rather than present a materially changed runtime as exact continuity",
            "evidence:m5-managed-lifecycle-companion:001",
            vec![
                D::LifecycleStateUnstated,
                D::ExactContinuityOverclaimed,
                D::LocalSafeOrCompanionHandoffOverflowOnly,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_reconnecting(),
                card_rebuild_required(),
                card_continuity_unstated(),
            ],
            vec![sheet_provenance_unstated(), sheet_exact_continuity_overclaimed()],
        ),
        base_row(
            C::IncidentUi,
            "Incident/ops surface owner",
            "Incident and ops surfaces keep the same lifecycle language, distinguishing recreate-required and expired states and degrading honestly when an outage state hides local-safe continuation; the review sheet degrades rather than appear after a destructive action",
            "evidence:m5-managed-lifecycle-incident:001",
            vec![
                D::LifecycleStateUnstated,
                D::LocalSafeOrCompanionHandoffOverflowOnly,
                D::ExactContinuityOverclaimed,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_recreate_required(),
                card_expired(),
                card_local_safe_unavailable(),
            ],
            vec![sheet_shown_after_commit(), sheet_resume_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved lifecycle card and review-sheet truth, so a hidden persistence change, an unstated preserved-vs-lost state, or an undisclosed consequence is visible in evidence rather than hidden behind feature-local prose, and local-safe continuation stays legible",
            "evidence:m5-managed-lifecycle-support-export:001",
            vec![
                D::LifecycleStateUnstated,
                D::PersistenceChangeHidden,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![card_local_safe_continuation()],
            vec![
                sheet_preserved_lost_unstated(),
                sheet_consequences_unstated(),
                sheet_rebuild_clean(),
            ],
        ),
    ]
}

fn governance_review() -> M5ManagedLifecycleGovernanceReview {
    M5ManagedLifecycleGovernanceReview {
        card_names_lifecycle_state_and_persistence_class: true,
        card_names_continuity_and_expiry_timing: true,
        lifecycle_state_always_explicit: true,
        review_sheet_names_action_and_provenance: true,
        review_sheet_names_preserved_vs_lost_and_consequences: true,
        review_sheet_appears_before_destructive_action: true,
        material_change_never_implies_exact_continuity: true,
        local_safe_continuation_never_overflow_only: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ManagedLifecycleConsumerProjection {
    M5ManagedLifecycleConsumerProjection {
        run_test_debug_surfaces_consume_lifecycle_vocabulary: true,
        preview_surfaces_consume_lifecycle_vocabulary: true,
        companion_surfaces_reuse_lifecycle_cards_and_review_language: true,
        incident_ops_consumes_lifecycle_vocabulary: true,
        support_export_reads_single_lifecycle_source: true,
        lifecycle_language_consistent_across_surfaces: true,
    }
}

fn proof_freshness() -> M5ManagedLifecycleProofFreshness {
    M5ManagedLifecycleProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ManagedLifecycleReleasePosture {
    M5ManagedLifecycleReleasePosture {
        proof_packet_ref: M5_MANAGED_LIFECYCLE_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_MANAGED_LIFECYCLE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_REF,
        M5_MANAGED_LIFECYCLE_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
        M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
        M5_MANAGED_LIFECYCLE_OBJECT_MODEL_DOC_REF,
    ])
}

/// Builds the canonical M5 managed-workspace-lifecycle-card / suspend-resume-rebuild-review-sheet
/// controls packet.
pub fn seeded_m5_managed_lifecycle_controls() -> M5ManagedLifecycleControlsPacket {
    M5ManagedLifecycleControlsPacket::new(M5ManagedLifecycleControlsPacketInput {
        packet_id: M5_MANAGED_LIFECYCLE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 managed-workspace-lifecycle-card and suspend-resume-rebuild-review-sheet controls with lifecycle state, persistence class, continuity class, expiry timing, template/image provenance, changed persistence, preserved-vs-lost state, reattach/rerun consequences, and local-safe continuation truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5ManagedLifecycleVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the run/test/debug row is held at Beta pending lifecycle-card parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_managed_lifecycle_controls_lifecycle_card_beta_narrowed(
) -> M5ManagedLifecycleControlsPacket {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.packet_id =
        "m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls:lifecycle-card-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::RunTestDebugUi)
        .expect("run/test/debug row present");
    row.qualification = M5BuildRemoteQualificationClass::Beta;
    packet
}

/// Narrowed variant: the preview row is narrowed to Preview pending review-sheet parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_managed_lifecycle_controls_review_sheet_preview_narrowed(
) -> M5ManagedLifecycleControlsPacket {
    let mut packet = seeded_m5_managed_lifecycle_controls();
    packet.packet_id =
        "m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls:review-sheet-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .expect("preview row present");
    row.qualification = M5BuildRemoteQualificationClass::Preview;
    packet
}
