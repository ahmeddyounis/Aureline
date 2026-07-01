//! Canonical seed builders for the M5 critical-journey checkpoint proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code certification proof, the artifacts, and the fixtures never drift. Every
//! attribute each journey row certifies over — the driving object family, its explicit state machine
//! (admitted states), the named recovery affordance the next-safe-action anchors on, the declared
//! consumer surfaces, the applicable downgrade triggers, and the controlled last-failure reason
//! classes — is pulled straight from the frozen lifecycle matrix's seeded packet, so the
//! certification cannot audit a journey the matrix does not anchor, and the bindings are derived
//! from the matrix rather than restated by hand. Only the ordered milestone checkpoint sequence each
//! journey shows is authored here, drawn from the frozen [`M5JourneyCheckpoint`] vocabulary.

use super::*;
use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::{
    seeded_m5_lifecycle_matrix, M5ObjectStateRow, M5_LIFECYCLE_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so the
/// checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The checkpoint posture seeded for one protected journey.
struct JourneySpec {
    /// The object family the journey drives (its matrix anchor).
    object_family: M5LifecycleObjectFamily,
    /// The frozen matrix journey this protected journey binds to, when one exists.
    matrix_journey: Option<M5CriticalJourney>,
    /// Short journey scope summary.
    scope_summary: &'static str,
    /// The ordered milestone checkpoints the journey shows, drawn from the frozen vocabulary.
    checkpoint_sequence: Vec<M5JourneyCheckpoint>,
    /// When set, the evaluated-surface set used instead of the driving object's declared set
    /// (blocked fixtures use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    checkpoint_visibility: CheckpointVisibilityState,
    partial_truth_labeling: PartialTruthLabelingState,
    place_continuity: PlaceContinuityState,
    capture_parity: CaptureParityState,
    headless_parity_preserved: bool,
    waiver: Option<CriticalJourneyWaiver>,
    narrowing_reason: Option<&'static str>,
}

/// Short reviewer-facing label for a protected journey.
fn journey_label(journey: M5ProtectedJourney) -> &'static str {
    match journey {
        M5ProtectedJourney::WarmStartup => "Warm startup",
        M5ProtectedJourney::LargeRepoOpen => "Large-repo open",
        M5ProtectedJourney::AiMultiFileApply => "AI multi-file apply",
        M5ProtectedJourney::RemoteAttachRun => "Remote attach-and-run",
        M5ProtectedJourney::CollaborationJoinFollow => "Collaboration join-follow",
    }
}

/// Returns the frozen matrix object-state row for a family.
fn matrix_row(object_family: M5LifecycleObjectFamily) -> M5ObjectStateRow {
    seeded_m5_lifecycle_matrix()
        .object_state_rows
        .into_iter()
        .find(|row| row.object_family == object_family)
        .expect("frozen lifecycle matrix declares every governed object family")
}

/// Builds one certification row from a protected journey and a checkpoint posture. Every binding —
/// the driving object family's qualification, owner, state machine (admitted states), recovery
/// affordance, last-failure reason classes, declared consumer surfaces, and downgrade triggers — is
/// pulled from the frozen matrix object row for the family the journey drives.
fn row_from_journey(journey: M5ProtectedJourney, spec: JourneySpec) -> CriticalJourneyRow {
    let source = matrix_row(spec.object_family);
    let required_consumer_surfaces = source.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| source.consumer_surfaces.clone());
    let mut row = CriticalJourneyRow {
        journey,
        journey_label: journey_label(journey).to_owned(),
        object_family: spec.object_family,
        matrix_journey: spec.matrix_journey,
        qualification: source.qualification,
        owner_role: source.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        admitted_states: source.admitted_states.clone(),
        success_state: M5LifecycleState::Ready,
        recovery_affordance: source.recovery_affordance,
        last_failure_reason_classes: source.last_failure_reason_classes.clone(),
        checkpoint_sequence: spec.checkpoint_sequence,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        checkpoint_visibility: spec.checkpoint_visibility,
        partial_truth_labeling: spec.partial_truth_labeling,
        place_continuity: spec.place_continuity,
        capture_parity: spec.capture_parity,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: source.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: CriticalJourneyStatus::Green,
        journey_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.journey_causes = row.recompute_causes();
    row
}

/// Builds the collaboration reduced-next-action waiver carried by the seed.
fn collaboration_reduced_next_action_waiver() -> CriticalJourneyWaiver {
    CriticalJourneyWaiver {
        waiver_id: "waiver:collaboration-reduced-next-action:0001".to_owned(),
        journey: M5ProtectedJourney::CollaborationJoinFollow,
        reason:
            "When a collaboration join-follow session loses its shared connection mid-follow, the \
             journey keeps the user's place in the checkpoint sequence and a disclosed, still-safe \
             reduced next-safe-action: the rejoin affordance is offered immediately while the \
             control-transfer request is deferred until the session reconnects, rather than \
             dropping the user onto a generic shell. The reduced next-safe-action is disclosed, \
             never silent, and the full affordance set is restored the moment the collaboration \
             lane rejoins."
                .to_owned(),
        owner_role: "Collaboration owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-visibility posture: all four checkpoint dimensions hold and headless parity is preserved.
fn full(
    object_family: M5LifecycleObjectFamily,
    matrix_journey: Option<M5CriticalJourney>,
    scope_summary: &'static str,
    checkpoint_sequence: Vec<M5JourneyCheckpoint>,
) -> JourneySpec {
    JourneySpec {
        object_family,
        matrix_journey,
        scope_summary,
        checkpoint_sequence,
        evaluated_surfaces_override: None,
        checkpoint_visibility: CheckpointVisibilityState::NamedMilestonesReplaceSpinner,
        partial_truth_labeling: PartialTruthLabelingState::PartialStateLabeledAndAttributed,
        place_continuity: PlaceContinuityState::PlaceAndNextActionPreserved,
        capture_parity: CaptureParityState::CheckpointsCapturedInExportAndScreenshot,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded checkpoint posture for one protected journey.
fn journey_spec(journey: M5ProtectedJourney) -> JourneySpec {
    use M5JourneyCheckpoint as K;
    match journey {
        // Warm startup holds full checkpoint visibility across all four dimensions.
        M5ProtectedJourney::WarmStartup => full(
            M5LifecycleObjectFamily::Workspace,
            Some(M5CriticalJourney::WorkspaceRestore),
            "Warm startup replaces the blank window with named milestones — skeleton shell, command \
             system ready, session restore note, first interactive editor — instead of one \
             anonymous boot spinner",
            vec![K::Preparing, K::Warming, K::Restoring, K::Ready],
        ),
        // Large-repo open shows a disclosed coarse partial-truth label while the tree is partial and
        // indexing continues — labeled and attributed, just at a coarse grain (yellow).
        M5ProtectedJourney::LargeRepoOpen => JourneySpec {
            partial_truth_labeling: PartialTruthLabelingState::DisclosedCoarsePartialLabel,
            narrowing_reason: Some(
                "While a large repository opens, the journey shows a disclosed coarse partial-truth \
                 label — the partial tree and warm search fallback are labeled at the container \
                 grain rather than per-file while indexing progresses — while still naming each \
                 milestone and attributing the partial state to indexing, so the large-repo-open \
                 journey is narrowed and disclosed rather than leaving the partial state unlabeled.",
            ),
            ..full(
                M5LifecycleObjectFamily::Workspace,
                None,
                "Large-repo open replaces the frozen tree with named milestones — partial tree, \
                 warm search fallback, indexing progress, first jump confidence note — instead of \
                 one anonymous indexing spinner",
                vec![K::Preparing, K::Warming, K::Building, K::Ready],
            )
        },
        // AI multi-file apply holds full checkpoint visibility across all four dimensions, always
        // exposing the reviewable patch, verification result, and rollback handle as named
        // milestones.
        M5ProtectedJourney::AiMultiFileApply => full(
            M5LifecycleObjectFamily::AiAction,
            Some(M5CriticalJourney::AiActionRun),
            "AI multi-file apply replaces the opaque apply spinner with named milestones — context \
             resolving, approval requirement, reviewable patch, verification result, rollback \
             handle — so a maybe-applied change never hides behind one anonymous spinner",
            vec![K::Preparing, K::Authorizing, K::Building, K::Verifying, K::Ready],
        ),
        // Remote attach-and-run compacts its milestones into a disclosed compact form on the remote
        // status strip while still naming each one (yellow).
        M5ProtectedJourney::RemoteAttachRun => JourneySpec {
            checkpoint_visibility: CheckpointVisibilityState::DisclosedCompactedMilestones,
            narrowing_reason: Some(
                "On a compact remote status strip the attach-and-run journey presents its \
                 auth/policy, environment-probe, sync-warming, and task-stream milestones in a \
                 disclosed compacted form while still naming each milestone individually, so the \
                 remote journey is narrowed and disclosed rather than collapsing its milestones \
                 into an anonymous spinner.",
            ),
            ..full(
                M5LifecycleObjectFamily::RemoteSession,
                Some(M5CriticalJourney::RemoteReconnect),
                "Remote attach-and-run replaces the opaque connect spinner with named milestones — \
                 auth/policy stage, environment probe, sync warming, structured task stream — even \
                 on a compact remote status strip",
                vec![K::Authorizing, K::Connecting, K::Warming, K::Ready],
            )
        },
        // Collaboration join-follow keeps the user's place with a disclosed, waivered reduced
        // next-safe-action when the shared connection drops mid-follow (yellow).
        M5ProtectedJourney::CollaborationJoinFollow => JourneySpec {
            place_continuity: PlaceContinuityState::DisclosedReducedNextAction,
            waiver: Some(collaboration_reduced_next_action_waiver()),
            narrowing_reason: Some(
                "When a collaboration join-follow session loses its shared connection mid-follow, \
                 the journey keeps the user's place in the checkpoint sequence and a disclosed, \
                 waivered reduced next-safe-action — the rejoin affordance is offered immediately \
                 while control transfer is deferred until reconnect — so the collaboration journey \
                 is narrowed and disclosed rather than dropping the user onto a generic shell.",
            ),
            ..full(
                M5LifecycleObjectFamily::CollaborationSession,
                Some(M5CriticalJourney::CollaborationJoin),
                "Collaboration join-follow replaces the opaque join spinner with named milestones — \
                 publish/join, role assignment, follow state, control transfer visibility, archived \
                 outcome — and keeps the user's place when the shared connection drops",
                vec![K::Queued, K::Authorizing, K::Warming, K::Verifying, K::Ready],
            )
        },
    }
}

/// Builds the certification rows for the canonical seed, one per protected journey.
fn seeded_rows() -> Vec<CriticalJourneyRow> {
    M5ProtectedJourney::ALL
        .iter()
        .map(|&journey| row_from_journey(journey, journey_spec(journey)))
        .collect()
}

/// Builds a variant where one journey's spec is mutated after the canonical spec is resolved, used
/// by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ProtectedJourney, mutate: F) -> Vec<CriticalJourneyRow>
where
    F: Fn(&mut JourneySpec),
{
    M5ProtectedJourney::ALL
        .iter()
        .map(|&journey| {
            let mut spec = journey_spec(journey);
            if journey == target {
                mutate(&mut spec);
            }
            row_from_journey(journey, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<CriticalJourneyRow>) -> CriticalJourneyPacket {
    build_m5_critical_journey_checkpoints_packet(CriticalJourneyInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 critical-journey checkpoint packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Warm startup and AI multi-file apply keep full checkpoint visibility (green). The
/// large-repo-open journey auto-narrows to yellow disclosing a coarse partial-truth label, the
/// remote attach-and-run journey auto-narrows to yellow disclosing compacted milestones, and the
/// collaboration join-follow journey auto-narrows to yellow with a waivered reduced next-safe-action
/// — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_critical_journey_checkpoints_packet() -> CriticalJourneyPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the warm-startup journey falls back to one anonymous monolithic spinner,
/// proving an anonymous spinner on a protected journey blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_critical_journey_checkpoints_packet_warm_startup_anonymous_spinner_blocked(
) -> CriticalJourneyPacket {
    let rows = seeded_rows_with(M5ProtectedJourney::WarmStartup, |spec| {
        spec.checkpoint_visibility = CheckpointVisibilityState::AnonymousSpinnerShown;
        spec.narrowing_reason = Some(
            "Warm startup fell back to one anonymous boot spinner instead of its skeleton-shell, \
             command-ready, session-restore, and first-editor milestones, so a half-restored \
             workspace hid behind a single opaque progress indicator, and the journey blocks \
             before keeping a checkpoint claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the large-repo-open journey leaves its partial state unlabeled and
/// unattributed, proving an unlabeled partial state blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_critical_journey_checkpoints_packet_large_repo_partial_unlabeled_blocked(
) -> CriticalJourneyPacket {
    let rows = seeded_rows_with(M5ProtectedJourney::LargeRepoOpen, |spec| {
        spec.partial_truth_labeling =
            PartialTruthLabelingState::PartialStateUnlabeledOrUnattributed;
        spec.narrowing_reason = Some(
            "While a large repository indexed, the partial tree and warm search fallback went \
             unlabeled and unattributed — the user could not tell which results were complete or \
             that indexing was still running — so the partial state hid without a controlled label, \
             and the journey blocks before keeping a checkpoint claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the collaboration join-follow journey loses the user's place and its
/// recovery affordance, proving a lost place blocks promotion (red) before the row can keep its
/// checkpoint claim.
pub fn seeded_m5_critical_journey_checkpoints_packet_collaboration_place_lost_blocked(
) -> CriticalJourneyPacket {
    let rows = seeded_rows_with(M5ProtectedJourney::CollaborationJoinFollow, |spec| {
        spec.place_continuity = PlaceContinuityState::PlaceOrRecoveryLost;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "When a collaboration join-follow session dropped its shared connection, the journey \
             cleared the checkpoint sequence and dropped the user onto a generic shell with no \
             rejoin affordance, losing both the user's place and the named recovery action, so the \
             journey blocks before keeping a checkpoint claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the AI multi-file apply journey's checkpoints do not survive capture,
/// proving checkpoints absent from export/screenshot/support capture block promotion (red) rather
/// than staying green.
pub fn seeded_m5_critical_journey_checkpoints_packet_ai_apply_capture_absent_blocked(
) -> CriticalJourneyPacket {
    let rows = seeded_rows_with(M5ProtectedJourney::AiMultiFileApply, |spec| {
        spec.capture_parity = CaptureParityState::CheckpointsAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The AI multi-file apply journey rendered its reviewable-patch and verification \
             milestones only in a transient overlay that a screenshot, support packet, and export \
             all dropped, so support could not reproduce the checkpoint truth the user saw live, \
             and the journey blocks before keeping a checkpoint claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the remote attach-and-run journey loses the shared state-truth vocabulary
/// in a headless execution, proving a headless/companion-adjacent parity loss blocks promotion (red)
/// rather than staying a disclosed yellow.
pub fn seeded_m5_critical_journey_checkpoints_packet_remote_headless_parity_lost_blocked(
) -> CriticalJourneyPacket {
    let rows = seeded_rows_with(M5ProtectedJourney::RemoteAttachRun, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the remote attach-and-run journey reported a private \
             checkpoint vocabulary that diverged from the controlled milestones shown in-product, \
             so the same journey described a different checkpoint and state language depending on \
             how it ran, and the journey blocks before keeping a checkpoint claim.",
        );
    });
    packet_from_rows(rows)
}
