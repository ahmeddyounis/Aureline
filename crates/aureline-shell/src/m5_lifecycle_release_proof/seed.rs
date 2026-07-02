//! Canonical seed builders for the M5 lifecycle release proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code certification proof, the artifacts, and the fixtures never drift. Every attribute
//! each family row certifies over — the driving matrix journey, the explicit state machine (admitted
//! states), the one visible primary status surface, the one exportable status-code field, the one
//! last-failure-reason field, the named recovery affordance the recovery-affordance-truth pillar
//! anchors on, the checkpoint lineage the checkpoint-truth pillar is shown over, the declared consumer
//! surfaces, the applicable downgrade triggers, and the controlled last-failure reason classes — is
//! pulled straight from the frozen lifecycle matrix's seeded packet, so the certification cannot audit
//! a family the matrix does not anchor, and the bindings are derived from the matrix rather than
//! restated by hand. Only the claimed desktop profiles certified, the truth pillars kept, the four
//! truth postures, the per-family posture, and the scope summary are authored here.

use super::*;
use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::{
    seeded_m5_lifecycle_matrix, M5JourneyCheckpointRow, M5ObjectStateRow,
    M5_LIFECYCLE_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so the checked-in
/// fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The release-proof posture seeded for one object family.
struct FamilySpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The claimed desktop profiles this row certifies its truth across (defaults to all six).
    certified_profiles: Vec<M5DesktopProfile>,
    /// The truth pillars this row keeps (defaults to all three).
    certified_truth_pillars: Vec<M5LifecycleTruthPillar>,
    /// When set, the evaluated-surface set used instead of the object's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    lifecycle_state_truth: LifecycleStateTruthState,
    checkpoint_truth: CheckpointTruthState,
    recovery_affordance_truth: RecoveryAffordanceTruthState,
    exported_proof_parity: ExportedProofParityState,
    headless_parity_preserved: bool,
    waiver: Option<LifecycleReleaseProofWaiver>,
    narrowing_reason: Option<&'static str>,
}

/// Short reviewer-facing label for an object family.
fn object_label(family: M5LifecycleObjectFamily) -> &'static str {
    match family {
        M5LifecycleObjectFamily::Workspace => "Workspace / window session",
        M5LifecycleObjectFamily::Extension => "Installed extension",
        M5LifecycleObjectFamily::RemoteSession => "Remote / tunnel session",
        M5LifecycleObjectFamily::CollaborationSession => "Collaboration session",
        M5LifecycleObjectFamily::AiAction => "AI assistant action",
        M5LifecycleObjectFamily::UpdateRollback => "Update / rollback",
        M5LifecycleObjectFamily::NotebookRuntime => "Notebook runtime",
        M5LifecycleObjectFamily::RequestApiRun => "Request / API run",
        M5LifecycleObjectFamily::PreviewSession => "Preview / live-server session",
        M5LifecycleObjectFamily::PipelineRun => "Pipeline / task run",
        M5LifecycleObjectFamily::DataSession => "Data / database session",
        M5LifecycleObjectFamily::ProfilerCapture => "Profiler / trace capture",
        M5LifecycleObjectFamily::CompanionSession => "Companion / paired-device session",
    }
}

/// Returns the frozen matrix object-state row for a family.
fn matrix_object_row(object_family: M5LifecycleObjectFamily) -> M5ObjectStateRow {
    seeded_m5_lifecycle_matrix()
        .object_state_rows
        .into_iter()
        .find(|row| row.object_family == object_family)
        .expect("frozen lifecycle matrix declares every governed object family")
}

/// Returns the frozen matrix journey-checkpoint row that drives a family.
fn matrix_journey_row(object_family: M5LifecycleObjectFamily) -> M5JourneyCheckpointRow {
    seeded_m5_lifecycle_matrix()
        .journey_checkpoint_rows
        .into_iter()
        .find(|row| row.object_family == object_family)
        .expect("frozen lifecycle matrix declares a journey for every governed object family")
}

/// Builds one certification row from an object family and a release-proof posture. Every binding — the
/// driving matrix journey, the object's qualification, owner, state machine (admitted states), primary
/// status surface, status-code export field, last-failure-reason field, recovery affordance,
/// last-failure reason classes, checkpoint lineage, declared consumer surfaces, and downgrade
/// triggers — is pulled from the frozen matrix rows for the family.
fn row_from_family(family: M5LifecycleObjectFamily, spec: FamilySpec) -> LifecycleReleaseProofRow {
    let object = matrix_object_row(family);
    let journey = matrix_journey_row(family);
    let required_consumer_surfaces = object.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| object.consumer_surfaces.clone());
    let mut row = LifecycleReleaseProofRow {
        object_family: family,
        object_label: object_label(family).to_owned(),
        matrix_journey: journey.journey,
        qualification: object.qualification,
        owner_role: object.owner_role.clone(),
        scope_summary: spec.scope_summary.to_owned(),
        admitted_states: object.admitted_states.clone(),
        primary_status_surface: object.primary_status_surface,
        status_code_export_field: object.status_code_export_field.clone(),
        last_failure_reason_field: object.last_failure_reason_field.clone(),
        recovery_affordance: object.recovery_affordance,
        last_failure_reason_classes: object.last_failure_reason_classes.clone(),
        checkpoint_lineage: journey.checkpoints.clone(),
        certified_profiles: spec.certified_profiles,
        certified_truth_pillars: spec.certified_truth_pillars,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        lifecycle_state_truth: spec.lifecycle_state_truth,
        checkpoint_truth: spec.checkpoint_truth,
        recovery_affordance_truth: spec.recovery_affordance_truth,
        exported_proof_parity: spec.exported_proof_parity,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: object.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: LifecycleReleaseProofStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the companion reduced-recovery-truth waiver carried by the seed.
fn companion_reduced_recovery_truth_waiver() -> LifecycleReleaseProofWaiver {
    LifecycleReleaseProofWaiver {
        waiver_id: "waiver:companion-reduced-recovery-truth:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CompanionSession,
        reason:
            "On the small companion / paired-device surface a degraded session exposes a disclosed \
             reduced recovery truth — the named recovery affordance is deferred to a linked \
             reattach-on-desktop action while the controlled last-failure reason is still named \
             inline — so the recovery truth is narrowed and disclosed rather than dropped. The full \
             in-place recovery affordance is restored the moment the companion reattaches to a \
             standard-width surface."
                .to_owned(),
        owner_role: "Companion owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four proof dimensions hold, all six claimed profiles and all three
/// truth pillars are certified, and headless parity is preserved.
fn full(scope_summary: &'static str) -> FamilySpec {
    FamilySpec {
        scope_summary,
        certified_profiles: M5DesktopProfile::ALL.to_vec(),
        certified_truth_pillars: M5LifecycleTruthPillar::ALL.to_vec(),
        evaluated_surfaces_override: None,
        lifecycle_state_truth: LifecycleStateTruthState::ExplicitStateTruthCertified,
        checkpoint_truth: CheckpointTruthState::NamedCheckpointTruthCertified,
        recovery_affordance_truth: RecoveryAffordanceTruthState::NamedRecoveryAndReasonCertified,
        exported_proof_parity: ExportedProofParityState::ExportedSurfacesReflectCurrentProof,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded release-proof posture for one object family.
fn family_spec(family: M5LifecycleObjectFamily) -> FamilySpec {
    use M5LifecycleObjectFamily as F;
    match family {
        F::Workspace => full(
            "Workspace lifecycle keeps its explicit state truth, its named restore-checkpoint truth, \
             and its named recovery-affordance and last-failure-reason truth across every claimed \
             desktop profile and every exported truth surface — UI, CLI, docs/help, diagnostics, \
             support exports, telemetry, and claim publication",
        ),
        F::Extension => full(
            "Installed extension keeps its explicit capability-lifecycle truth, its named \
             activation-checkpoint truth, and its named recovery-affordance and reason truth across \
             every claimed profile and exported surface",
        ),
        F::RemoteSession => full(
            "Remote / tunnel session keeps its explicit connection-lifecycle truth, its named \
             reconnect-checkpoint truth, and its named recovery-affordance and reason truth across \
             every claimed profile and exported surface",
        ),
        F::AiAction => full(
            "AI assistant action keeps its explicit action-lifecycle truth, its named apply/review \
             checkpoint truth, and its named recovery-affordance and reason truth across every \
             claimed profile and exported surface",
        ),
        F::UpdateRollback => full(
            "Update / rollback keeps its explicit update-lifecycle truth, its named stage/rollback \
             checkpoint truth, and its named rollback-affordance and reason truth across every \
             claimed profile and exported surface",
        ),
        F::NotebookRuntime => full(
            "Notebook runtime keeps its explicit kernel-lifecycle truth, its named execute/reconnect \
             checkpoint truth, and its named recovery-affordance and reason truth across every \
             claimed profile and exported surface",
        ),
        F::RequestApiRun => full(
            "Request / API run keeps its explicit request-lifecycle truth, its named send/retry \
             checkpoint truth, and its named recovery-affordance and reason truth across every \
             claimed profile and exported surface",
        ),
        F::DataSession => full(
            "Data / database session keeps its explicit connection-lifecycle truth, its named \
             connect/reconnect checkpoint truth, and its named recovery-affordance and reason truth \
             across every claimed profile and exported surface",
        ),
        F::CollaborationSession => full(
            "Collaboration session keeps its explicit session-lifecycle truth, its named \
             join/control-transfer checkpoint truth, and its named recovery-affordance and reason \
             truth across every claimed profile and exported surface",
        ),
        // Profiler discloses a reduced lifecycle-state truth on a constrained build (yellow).
        F::ProfilerCapture => FamilySpec {
            lifecycle_state_truth: LifecycleStateTruthState::DisclosedReducedStateTruth,
            narrowing_reason: Some(
                "On a constrained trace-capture build the profiler exposes a disclosed reduced \
                 lifecycle-state truth — a handful of intermediate capture states are grouped into \
                 one disclosed grouped state while the terminal controlled state (ready, \
                 partial_ready, or recoverable_failure) is still named — so the state truth is \
                 narrowed and disclosed rather than collapsed into a generic loading or error \
                 behavior.",
            ),
            ..full(
                "Profiler capture keeps its named checkpoint, recovery-affordance, and exported-proof \
                 truth across every profile and surface, grouping a few intermediate capture states \
                 into one disclosed grouped state on constrained builds",
            )
        },
        // Pipeline discloses a compacted checkpoint truth on long fan-outs (yellow).
        F::PipelineRun => FamilySpec {
            checkpoint_truth: CheckpointTruthState::DisclosedCompactedCheckpointTruth,
            narrowing_reason: Some(
                "On a long fan-out pipeline the run shows a disclosed compacted checkpoint sequence \
                 — two adjacent stage milestones are folded into one disclosed compacted milestone \
                 while each terminal checkpoint is still named — so the checkpoint truth is narrowed \
                 and disclosed rather than collapsed into an anonymous spinner.",
            ),
            ..full(
                "Pipeline run keeps its explicit state, recovery-affordance, and exported-proof truth \
                 across every profile and surface, folding two adjacent stage milestones into one \
                 disclosed compacted milestone on long fan-outs",
            )
        },
        // Preview discloses a partial export refresh on a legacy diagnostics surface (yellow).
        F::PreviewSession => FamilySpec {
            exported_proof_parity: ExportedProofParityState::DisclosedPartialExportRefresh,
            narrowing_reason: Some(
                "On the legacy preview diagnostics surface one exported truth surface takes a \
                 disclosed partial refresh cadence — the legacy diagnostics export refreshes on a \
                 slower cadence than the live UI while still disclosing the lag and still exporting \
                 the same status code and last-failure reason — so the exported parity is narrowed \
                 and disclosed rather than stale or divergent.",
            ),
            ..full(
                "Preview session keeps its explicit state, named checkpoint, and recovery-affordance \
                 truth across every profile and surface, taking a disclosed slower refresh cadence on \
                 one legacy diagnostics export",
            )
        },
        // Companion carries a disclosed, waivered reduced recovery truth on its small surface (yellow).
        F::CompanionSession => FamilySpec {
            recovery_affordance_truth: RecoveryAffordanceTruthState::DisclosedReducedRecoveryTruth,
            waiver: Some(companion_reduced_recovery_truth_waiver()),
            narrowing_reason: Some(
                "On the small companion / paired-device surface a degraded session exposes a \
                 disclosed, waivered reduced recovery truth — the named recovery affordance is \
                 deferred to a linked reattach-on-desktop action while the controlled last-failure \
                 reason is still named inline — so the recovery truth is narrowed and disclosed \
                 rather than dropped.",
            ),
            ..full(
                "Companion session keeps its explicit state, named checkpoint, and exported-proof \
                 truth across every profile and surface, deferring the in-place recovery affordance \
                 to a linked reattach-on-desktop action on the small paired-device surface",
            )
        },
    }
}

/// Builds the certification rows for the canonical seed, one per object family.
fn seeded_rows() -> Vec<LifecycleReleaseProofRow> {
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by
/// the blocked fixtures.
fn seeded_rows_with<F>(target: M5LifecycleObjectFamily, mutate: F) -> Vec<LifecycleReleaseProofRow>
where
    F: Fn(&mut FamilySpec),
{
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&family| {
            let mut spec = family_spec(family);
            if family == target {
                mutate(&mut spec);
            }
            row_from_family(family, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<LifecycleReleaseProofRow>) -> LifecycleReleaseProofPacket {
    build_m5_lifecycle_release_proof_packet(LifecycleReleaseProofInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 lifecycle release-proof packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Nine families keep full lifecycle, checkpoint, recovery-affordance, and exported-proof
/// truth (green). The profiler capture auto-narrows to yellow disclosing a reduced lifecycle-state
/// truth, the pipeline run auto-narrows to yellow disclosing a compacted checkpoint truth, the preview
/// session auto-narrows to yellow disclosing a partial export refresh, and the companion session
/// auto-narrows to yellow with a waivered reduced recovery truth — and no row is blocked, so the packet
/// is clean and every row is publishable.
pub fn seeded_m5_lifecycle_release_proof_packet() -> LifecycleReleaseProofPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook runtime collapses its lifecycle state into a generic loading or
/// error behavior, proving that a collapsed state truth blocks promotion (red) rather than staying
/// green.
pub fn seeded_m5_lifecycle_release_proof_packet_notebook_state_collapsed_blocked(
) -> LifecycleReleaseProofPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::NotebookRuntime, |spec| {
        spec.lifecycle_state_truth =
            LifecycleStateTruthState::StateCollapsedIntoGenericLoadingOrError;
        spec.narrowing_reason = Some(
            "After a kernel reconnect the notebook runtime hid its controlled lifecycle state behind \
             a generic \"loading…\" spinner and a generic \"something went wrong\" error, so the \
             state was no longer diagnosable from the controlled vocabulary on any claimed profile or \
             exported surface, and the runtime blocks before keeping a release-proof claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the remote session collapses its milestone checkpoints into an anonymous
/// spinner, proving that a collapsed checkpoint truth blocks promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_release_proof_packet_remote_checkpoints_collapsed_blocked(
) -> LifecycleReleaseProofPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::RemoteSession, |spec| {
        spec.checkpoint_truth = CheckpointTruthState::CheckpointsCollapsedToAnonymousSpinner;
        spec.narrowing_reason = Some(
            "After a dropped tunnel the remote session collapsed its ordered reconnect milestones — \
             resolve host, open tunnel, attach workspace — into a single anonymous spinner with no \
             named boundaries, so the journey showed no attributable checkpoint the user or support \
             could name, and the session blocks before keeping a release-proof claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data session drops its recovery affordance and last-failure reason,
/// proving that a missing recovery truth blocks promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_release_proof_packet_data_recovery_truth_missing_blocked(
) -> LifecycleReleaseProofPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::DataSession, |spec| {
        spec.recovery_affordance_truth = RecoveryAffordanceTruthState::RecoveryOrReasonTruthMissing;
        spec.narrowing_reason = Some(
            "When the data session dropped to read_only_degraded it exposed no named recovery \
             affordance and no controlled last-failure reason — only a bare disabled state — so the \
             degraded state could neither be recovered from nor diagnosed from the controlled \
             vocabulary, and the session blocks before keeping a release-proof claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the AI action lets an exported truth surface go stale or divergent, proving
/// that a stale exported proof blocks promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_release_proof_packet_ai_exported_proof_stale_blocked(
) -> LifecycleReleaseProofPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::AiAction, |spec| {
        spec.exported_proof_parity = ExportedProofParityState::ExportedProofStaleOrDivergent;
        spec.narrowing_reason = Some(
            "The claim-publication surface kept advertising the AI action as \"applied\" while the \
             live diagnostics export had already moved it to needs_review, so a published claim and a \
             support export overclaimed relative to the current lifecycle truth, and the action \
             blocks before keeping a release-proof claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the extension loses the shared state-truth vocabulary in a headless
/// execution, proving that a headless/companion-adjacent parity loss blocks promotion (red) rather
/// than staying green.
pub fn seeded_m5_lifecycle_release_proof_packet_extension_headless_parity_lost_blocked(
) -> LifecycleReleaseProofPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::Extension, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the extension emitted a private lifecycle and checkpoint \
             vocabulary that diverged from the controlled state truth shown in-product, so the same \
             capability described its state with a different language depending on how it ran, and \
             the extension blocks before keeping a release-proof claim.",
        );
    });
    packet_from_rows(rows)
}
