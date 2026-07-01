//! Canonical seed builders for the M5 lifecycle transition-safety proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export,
//! and CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both
//! call them so the in-code certification proof, the artifacts, and the fixtures never drift.
//! Every object attribute each row certifies over — the explicit state machine (admitted states),
//! the named recovery affordance the local fallback anchors on, the declared consumer surfaces, and
//! the applicable downgrade triggers — is pulled straight from the frozen lifecycle matrix's seeded
//! packet, so the certification cannot audit a family the matrix does not freeze, and the bindings
//! are derived from the matrix rather than restated by hand.

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

/// The transition-safety posture seeded for one object family.
struct ObjectSpec {
    /// When set, the evaluated-surface set used instead of the matrix's declared set (blocked
    /// fixtures use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    safe_transition: SafeTransitionState,
    transition_attribution: TransitionAttributionState,
    checkpoint_sequencing: CheckpointSequencingState,
    local_fallback: LocalFallbackState,
    headless_parity_preserved: bool,
    waiver: Option<TransitionSafetyWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ObjectSpec {
    /// A full-safety posture: all four transition dimensions hold and headless parity is preserved.
    fn stable() -> Self {
        Self {
            evaluated_surfaces_override: None,
            safe_transition: SafeTransitionState::SafeRetryCancelRollbackRules,
            transition_attribution: TransitionAttributionState::ActorSubsystemAttributed,
            checkpoint_sequencing: CheckpointSequencingState::RequiredCheckpointsEnforced,
            local_fallback: LocalFallbackState::LocalEditingProtectedFallback,
            headless_parity_preserved: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Short reviewer-facing label for an object family.
fn object_label(object_family: M5LifecycleObjectFamily) -> &'static str {
    match object_family {
        M5LifecycleObjectFamily::Workspace => "Workspace / window session",
        M5LifecycleObjectFamily::Extension => "Installed extension / capability",
        M5LifecycleObjectFamily::RemoteSession => "Remote / tunnel session",
        M5LifecycleObjectFamily::CollaborationSession => "Live collaboration session",
        M5LifecycleObjectFamily::AiAction => "AI assistant action",
        M5LifecycleObjectFamily::UpdateRollback => "Update / rollback lifecycle",
        M5LifecycleObjectFamily::NotebookRuntime => "Notebook kernel runtime",
        M5LifecycleObjectFamily::RequestApiRun => "Request / API run",
        M5LifecycleObjectFamily::PreviewSession => "Preview / live-server session",
        M5LifecycleObjectFamily::PipelineRun => "Pipeline / task run",
        M5LifecycleObjectFamily::DataSession => "Data / database session",
        M5LifecycleObjectFamily::ProfilerCapture => "Profiler / trace capture",
        M5LifecycleObjectFamily::CompanionSession => "Companion / paired device session",
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

/// Builds one certification row from an object family and a transition-safety posture. Every
/// binding — the explicit state machine (admitted states), the named recovery affordance, the
/// declared consumer surfaces, and the downgrade triggers — is pulled from the frozen matrix row.
fn row_from_object(
    object_family: M5LifecycleObjectFamily,
    spec: ObjectSpec,
) -> TransitionSafetyRow {
    let source = matrix_row(object_family);
    let required_consumer_surfaces = source.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| source.consumer_surfaces.clone());
    let mut row = TransitionSafetyRow {
        object_family,
        object_label: object_label(object_family).to_owned(),
        qualification: source.qualification,
        owner_role: source.owner_role.clone(),
        scope_summary: source.scope_summary.clone(),
        admitted_states: source.admitted_states.clone(),
        recovery_affordance: source.recovery_affordance,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        safe_transition: spec.safe_transition,
        transition_attribution: spec.transition_attribution,
        checkpoint_sequencing: spec.checkpoint_sequencing,
        local_fallback: spec.local_fallback,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: source.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: TransitionSafetyStatus::Green,
        transition_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.transition_causes = row.recompute_causes();
    row
}

/// Builds the collaboration reduced-local-fallback waiver carried by the seed.
fn collaboration_reduced_fallback_waiver() -> TransitionSafetyWaiver {
    TransitionSafetyWaiver {
        waiver_id: "waiver:collaboration-reduced-fallback:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CollaborationSession,
        reason:
            "When a live collaboration session loses its shared connection, the object keeps a \
                 disclosed, still-safe local-editing fallback: local edits continue read-only \
                 against the last synced snapshot until the session rejoins, rather than blocking \
                 editing outright. The reduced fallback is disclosed, never silent, and the full \
                 read-write local fallback is restored the moment the collaboration lane rejoins."
                .to_owned(),
        owner_role: "Collaboration owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded transition-safety posture for one object family.
fn object_spec(object_family: M5LifecycleObjectFamily) -> ObjectSpec {
    match object_family {
        M5LifecycleObjectFamily::RemoteSession => ObjectSpec {
            // A reconnecting remote session exposes a disclosed reduced transition set — cancel is
            // deferred until the reconnect resolves — while retry, rollback, and compensation stay
            // safe.
            safe_transition: SafeTransitionState::DisclosedReducedTransitionSet,
            narrowing_reason: Some(
                "While a remote session is reconnecting, it exposes a disclosed reduced transition \
                 set — a pending cancel is deferred until the reconnect resolves so the tunnel is \
                 never left half-torn-down — while retry, rollback, and compensation stay safe, so \
                 the remote object is narrowed and disclosed rather than allowing an unsafe cancel.",
            ),
            ..ObjectSpec::stable()
        },
        M5LifecycleObjectFamily::PipelineRun => ObjectSpec {
            // A fan-out pipeline attributes an in-flight transition to a disclosed coarse stage
            // group until the exact task actor is resolved.
            transition_attribution: TransitionAttributionState::DisclosedCoarseAttribution,
            narrowing_reason: Some(
                "A fan-out pipeline run attributes an in-flight transition to a disclosed coarse \
                 stage group rather than the exact task actor until the specific task that drove the \
                 transition is resolved, while still naming a controlled subsystem, so the pipeline \
                 object is narrowed and disclosed rather than dropping attribution.",
            ),
            ..ObjectSpec::stable()
        },
        M5LifecycleObjectFamily::NotebookRuntime => ObjectSpec {
            // A fast notebook cell run compacts its named checkpoints into a disclosed compact
            // progress while still naming each milestone.
            checkpoint_sequencing: CheckpointSequencingState::DisclosedCompactedCheckpoints,
            narrowing_reason: Some(
                "A fast notebook cell run presents its required queue / execute / render checkpoints \
                 in a disclosed compacted progress on the inline cell surface while still naming each \
                 milestone individually, so the notebook object is narrowed and disclosed rather \
                 than collapsing its checkpoints into an anonymous spinner.",
            ),
            ..ObjectSpec::stable()
        },
        M5LifecycleObjectFamily::CollaborationSession => ObjectSpec {
            // When the collaboration lane degrades, local editing continues as a disclosed,
            // waivered read-only-until-rejoin fallback rather than blocking editing outright.
            local_fallback: LocalFallbackState::DisclosedReducedFallback,
            waiver: Some(collaboration_reduced_fallback_waiver()),
            narrowing_reason: Some(
                "When a live collaboration session loses its shared connection, the object keeps a \
                 disclosed, waivered reduced local-editing fallback — local edits continue read-only \
                 against the last synced snapshot until the session rejoins — while still keeping a \
                 safe local path, so the collaboration object is narrowed and disclosed rather than \
                 losing local editing.",
            ),
            ..ObjectSpec::stable()
        },
        // Every other object family holds full transition safety across all four dimensions.
        _ => ObjectSpec::stable(),
    }
}

/// Builds the certification rows for the canonical seed, one per governed object family.
fn seeded_rows() -> Vec<TransitionSafetyRow> {
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&object_family| row_from_object(object_family, object_spec(object_family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used
/// by the blocked fixtures.
fn seeded_rows_with<F>(target: M5LifecycleObjectFamily, mutate: F) -> Vec<TransitionSafetyRow>
where
    F: Fn(&mut ObjectSpec),
{
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&object_family| {
            let mut spec = object_spec(object_family);
            if object_family == target {
                mutate(&mut spec);
            }
            row_from_object(object_family, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<TransitionSafetyRow>) -> TransitionSafetyPacket {
    build_m5_lifecycle_transition_safety_packet(TransitionSafetyInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 lifecycle transition-safety packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Nine object families keep full transition safety (green). The remote session
/// auto-narrows to yellow disclosing a reduced transition set, the pipeline run auto-narrows to
/// yellow disclosing a coarse transition attribution, the notebook runtime auto-narrows to yellow
/// disclosing compacted checkpoints, and the collaboration session auto-narrows to yellow with a
/// waivered reduced local-editing fallback — and no row is blocked, so the packet is clean and
/// every row is publishable.
pub fn seeded_m5_lifecycle_transition_safety_packet() -> TransitionSafetyPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the AI action allows an unsafe or missing transition, proving an unsafe
/// retry/cancel that could double-apply without rollback blocks promotion (red) rather than staying
/// a disclosed yellow.
pub fn seeded_m5_lifecycle_transition_safety_packet_ai_action_unsafe_transition_blocked(
) -> TransitionSafetyPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::AiAction, |spec| {
        spec.safe_transition = SafeTransitionState::UnsafeOrMissingTransitionRules;
        spec.narrowing_reason = Some(
            "The AI action allowed an unsafe retry that could re-apply an already-applied edit \
             without a rollback or compensation path, so a maybe-applied change can no longer be \
             safely restarted or compensated, and the object blocks before keeping a \
             transition-safety claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the request/API run stops attributing a transition to any actor or
/// subsystem, proving a missing transition attribution blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_lifecycle_transition_safety_packet_request_attribution_missing_blocked(
) -> TransitionSafetyPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::RequestApiRun, |spec| {
        spec.transition_attribution = TransitionAttributionState::AttributionMissingOnTransition;
        spec.narrowing_reason = Some(
            "The request/API run stopped attributing its retry transition to a controlled actor or \
             subsystem, so support can no longer tell whether a user, an automation, or a retry \
             policy drove the re-send, and the object blocks before keeping a transition-safety \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the update/rollback skips a required review/rollback checkpoint, proving
/// a protected flow that skips a required checkpoint blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_lifecycle_transition_safety_packet_update_checkpoint_skipped_blocked(
) -> TransitionSafetyPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::UpdateRollback, |spec| {
        spec.checkpoint_sequencing = CheckpointSequencingState::RequiredCheckpointSkipped;
        spec.narrowing_reason = Some(
            "The update/rollback flow jumped straight to an \"applied\" success banner, skipping its \
             required staged-verify and rollback-available checkpoints behind one generic spinner, \
             so a maybe-applied update hid behind an anonymous progress indicator, and the object \
             blocks before keeping a transition-safety claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data session loses its protected local-editing fallback, proving a
/// lost local fallback blocks promotion (red) before the row can keep its fallback claim.
pub fn seeded_m5_lifecycle_transition_safety_packet_data_local_fallback_lost_blocked(
) -> TransitionSafetyPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::DataSession, |spec| {
        spec.local_fallback = LocalFallbackState::LocalFallbackLost;
        spec.narrowing_reason = Some(
            "When the managed data session lost its connection, the editor also blocked all local \
             editing rather than preserving the protected local-editing fallback, leaving the user \
             with no safe local path forward while the lane was unavailable, so the object blocks \
             before keeping its fallback claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the extension loses the shared state-truth vocabulary in a headless
/// execution, proving a headless/companion-adjacent parity loss blocks promotion (red) rather than
/// staying green.
pub fn seeded_m5_lifecycle_transition_safety_packet_extension_headless_parity_lost_blocked(
) -> TransitionSafetyPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::Extension, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the extension reported a private transition vocabulary that \
             diverged from the controlled lifecycle transitions shown in-product, so the same object \
             described a different transition and state language depending on how it ran, and the \
             object blocks before keeping a transition-safety claim.",
        );
    });
    packet_from_rows(rows)
}
