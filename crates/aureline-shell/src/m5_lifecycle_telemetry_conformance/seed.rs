//! Canonical seed builders for the M5 lifecycle-telemetry-conformance proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code certification proof, the artifacts, and the fixtures never drift. Every attribute
//! each family row certifies over — the driving matrix journey, the explicit state machine (admitted
//! states), the one visible primary status surface, the one exportable status-code field, the one
//! last-failure-reason field, the named recovery affordance the mandatory-field conformance anchors
//! on, the checkpoint lineage the transition events replay, the declared consumer surfaces, the
//! applicable downgrade triggers, and the controlled last-failure reason classes — is pulled straight
//! from the frozen lifecycle matrix's seeded packet, so the certification cannot audit a family the
//! matrix does not anchor, and the bindings are derived from the matrix rather than restated by hand.
//! Only the telemetry sinks emitted, the mandatory fields kept conformant, the per-family posture, and
//! the scope summary are authored here.

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

/// The telemetry-conformance posture seeded for one object family.
struct FamilySpec {
    /// Short conformance scope summary.
    scope_summary: &'static str,
    /// The telemetry sinks this row emits its stable enums into (defaults to all four).
    emitted_telemetry_sinks: Vec<M5LifecycleTelemetrySink>,
    /// The mandatory fields this row keeps conformant (defaults to all three).
    conformant_mandatory_fields: Vec<M5LifecycleMandatoryField>,
    /// When set, the evaluated-surface set used instead of the object's declared set (blocked fixtures
    /// use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    enum_emission: TelemetryEnumEmissionState,
    transition_event: TransitionEventEmissionState,
    ui_export_parity: UiExportParityState,
    shared_contract_consumption: SharedContractConsumptionState,
    headless_parity_preserved: bool,
    waiver: Option<TelemetryConformanceWaiver>,
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

/// Builds one certification row from an object family and a telemetry posture. Every binding — the
/// driving matrix journey, the object's qualification, owner, state machine (admitted states), primary
/// status surface, status-code export field, last-failure-reason field, recovery affordance,
/// last-failure reason classes, checkpoint lineage, declared consumer surfaces, and downgrade
/// triggers — is pulled from the frozen matrix rows for the family.
fn row_from_family(family: M5LifecycleObjectFamily, spec: FamilySpec) -> TelemetryConformanceRow {
    let object = matrix_object_row(family);
    let journey = matrix_journey_row(family);
    let required_consumer_surfaces = object.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| object.consumer_surfaces.clone());
    let mut row = TelemetryConformanceRow {
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
        emitted_telemetry_sinks: spec.emitted_telemetry_sinks,
        conformant_mandatory_fields: spec.conformant_mandatory_fields,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        enum_emission: spec.enum_emission,
        transition_event: spec.transition_event,
        ui_export_parity: spec.ui_export_parity,
        shared_contract_consumption: spec.shared_contract_consumption,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: object.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: TelemetryConformanceStatus::Green,
        conformance_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.conformance_causes = row.recompute_causes();
    row
}

/// Builds the companion export-field-narrowing waiver carried by the seed.
fn companion_export_field_narrowing_waiver() -> TelemetryConformanceWaiver {
    TelemetryConformanceWaiver {
        waiver_id: "waiver:companion-export-field-narrowing:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CompanionSession,
        reason:
            "On the small companion / paired-device export the session carries a disclosed reduced \
             field detail — one intermediate checkpoint boundary is collapsed in the compact export \
             while the terminal status code, the last-failure reason, and the recovery affordance are \
             still exported under the same names the companion UI shows — while still disclosing that \
             the export was narrowed. The narrowing is disclosed, never silent, and the full \
             per-transition field detail is restored the moment the companion reattaches to a \
             standard-width export surface."
                .to_owned(),
        owner_role: "Companion owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// A full-conformance posture: all four telemetry dimensions hold, all four telemetry sinks and all
/// three mandatory fields are present, and headless parity is preserved.
fn full(scope_summary: &'static str) -> FamilySpec {
    FamilySpec {
        scope_summary,
        emitted_telemetry_sinks: M5LifecycleTelemetrySink::ALL.to_vec(),
        conformant_mandatory_fields: M5LifecycleMandatoryField::ALL.to_vec(),
        evaluated_surfaces_override: None,
        enum_emission: TelemetryEnumEmissionState::StableEnumsEmittedToEverySink,
        transition_event: TransitionEventEmissionState::TransitionEventsEmittedWithAttribution,
        ui_export_parity: UiExportParityState::UiAndExportNamingAndFieldsAgree,
        shared_contract_consumption:
            SharedContractConsumptionState::SharedContractConsumedNoLocalProse,
        headless_parity_preserved: true,
        waiver: None,
        narrowing_reason: None,
    }
}

/// Returns the seeded telemetry posture for one object family.
fn family_spec(family: M5LifecycleObjectFamily) -> FamilySpec {
    use M5LifecycleObjectFamily as F;
    match family {
        F::Workspace => full(
            "Workspace lifecycle emits its stable state and checkpoint enums into telemetry, \
             structured logs, dashboards, and support exports, fires attributed transition events for \
             each restore checkpoint, and keeps the status code, last-failure reason, and checkpoint \
             boundary named identically in the UI and the export path",
        ),
        F::Extension => full(
            "Extension activation emits stable capability-lifecycle enums into every sink, attributes \
             each enable/disable transition, and keeps the diagnostics view consuming the shared \
             contract",
        ),
        F::RemoteSession => full(
            "Remote reconnect emits stable connection-lifecycle enums into every sink, attributes \
             each reconnect transition to the tunnel subsystem, and exports the same status code and \
             last-failure reason the presence indicator shows",
        ),
        F::AiAction => full(
            "AI action emits stable action-lifecycle enums into every sink, attributes each apply/\
             review transition, and keeps claim-publication tooling consuming the shared contract \
             rather than local prose",
        ),
        F::UpdateRollback => full(
            "Update / rollback emits stable update-lifecycle enums into every sink, attributes each \
             stage and rollback transition, and keeps the update-center row and the export path in \
             agreement on the rollback affordance",
        ),
        F::NotebookRuntime => full(
            "Notebook runtime emits stable kernel-lifecycle enums into every sink, attributes each \
             execute/reconnect transition, and keeps the last-failure reason and checkpoint boundary \
             identical in the UI and export path",
        ),
        F::RequestApiRun => full(
            "Request / API run emits stable request-lifecycle enums into every sink, attributes each \
             send/retry transition, and keeps diagnostics consuming the shared contract",
        ),
        F::DataSession => full(
            "Data session emits stable connection-lifecycle enums into every sink, attributes each \
             connect/reconnect transition, and keeps the status code and last-failure reason named \
             identically in the UI and export path",
        ),
        F::CollaborationSession => full(
            "Collaboration session emits stable session-lifecycle enums into every sink, attributes \
             each join/control-transfer transition, and keeps Support Center consuming the shared \
             contract",
        ),
        // Profiler emits its stable enums into a disclosed reduced sink set on a constrained build (yellow).
        F::ProfilerCapture => FamilySpec {
            enum_emission: TelemetryEnumEmissionState::DisclosedReducedEnumSinkSet,
            narrowing_reason: Some(
                "On a constrained trace-capture build the profiler emits its stable lifecycle and \
                 checkpoint enums into a disclosed reduced sink set — the structured-log emission is \
                 folded into the telemetry stream while stable enums are still emitted into \
                 telemetry, dashboards, and support exports — so the sink coverage is narrowed and \
                 disclosed rather than dropping the controlled vocabulary.",
            ),
            ..full(
                "Profiler capture emits stable trace-lifecycle enums into telemetry, dashboards, and \
                 support exports, folding the structured-log emission into telemetry on constrained \
                 builds",
            )
        },
        // Pipeline emits disclosed coarse-grained transition events (yellow).
        F::PipelineRun => FamilySpec {
            transition_event: TransitionEventEmissionState::DisclosedCoarseTransitionEvents,
            narrowing_reason: Some(
                "On a long fan-out pipeline the run emits disclosed coarse-grained transition events \
                 — one event per checkpoint boundary rather than per intermediate stage transition — \
                 while still attributing each event to the executing subsystem, so the transition \
                 telemetry is narrowed and disclosed rather than anonymous.",
            ),
            ..full(
                "Pipeline run emits stable run-lifecycle enums into every sink and attributes each \
                 stage transition, coarsening to one event per checkpoint boundary on long fan-outs",
            )
        },
        // Preview takes a disclosed partial shared-contract adoption on a legacy surface (yellow).
        F::PreviewSession => FamilySpec {
            shared_contract_consumption:
                SharedContractConsumptionState::DisclosedPartialContractAdoption,
            narrowing_reason: Some(
                "On the legacy preview diagnostics surface the preview session takes a disclosed \
                 partial adoption of the shared lifecycle contract — the status code is resolved from \
                 the shared contract while one legacy build-detail field still renders a disclosed \
                 local label — so the contract consumption is narrowed and disclosed rather than \
                 replaced by local prose.",
            ),
            ..full(
                "Preview session emits stable build-lifecycle enums into every sink and attributes \
                 each rebuild transition, adopting the shared contract everywhere except one \
                 disclosed legacy diagnostics field",
            )
        },
        // Companion carries a disclosed, waivered export-field narrowing on its small export (yellow).
        F::CompanionSession => FamilySpec {
            ui_export_parity: UiExportParityState::DisclosedExportFieldNarrowing,
            waiver: Some(companion_export_field_narrowing_waiver()),
            narrowing_reason: Some(
                "On the small companion / paired-device export the session carries a disclosed, \
                 waivered reduced field detail — one intermediate checkpoint boundary is collapsed in \
                 the compact export while the terminal status code, last-failure reason, and recovery \
                 affordance are still exported under the same names the companion UI shows — so the \
                 export parity is narrowed and disclosed rather than drifted.",
            ),
            ..full(
                "Companion session emits stable session-lifecycle enums into every sink and \
                 attributes each attach transition, carrying a disclosed reduced field detail in the \
                 small paired-device export",
            )
        },
    }
}

/// Builds the certification rows for the canonical seed, one per object family.
fn seeded_rows() -> Vec<TelemetryConformanceRow> {
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, family_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by
/// the blocked fixtures.
fn seeded_rows_with<F>(target: M5LifecycleObjectFamily, mutate: F) -> Vec<TelemetryConformanceRow>
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

fn packet_from_rows(rows: Vec<TelemetryConformanceRow>) -> TelemetryConformancePacket {
    build_m5_lifecycle_telemetry_conformance_packet(TelemetryConformanceInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 lifecycle-telemetry-conformance packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Nine families keep full telemetry conformance (green). The profiler capture auto-narrows
/// to yellow disclosing a reduced telemetry-sink set, the pipeline run auto-narrows to yellow
/// disclosing coarse transition events, the preview session auto-narrows to yellow disclosing a
/// partial shared-contract adoption, and the companion session auto-narrows to yellow with a waivered
/// export-field narrowing — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_lifecycle_telemetry_conformance_packet() -> TelemetryConformancePacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook runtime drops a sink and emits local prose instead of stable
/// enums, proving that absent enums block promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_telemetry_conformance_packet_notebook_enums_absent_blocked(
) -> TelemetryConformancePacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::NotebookRuntime, |spec| {
        spec.enum_emission = TelemetryEnumEmissionState::EnumsAbsentOrLocalProseEmitted;
        spec.narrowing_reason = Some(
            "After a kernel reconnect the notebook runtime logged a free-text \"kernel came back\" \
             line instead of its stable lifecycle enum and never emitted the state into the \
             dashboard sink, so logs and dashboards could not be pivoted on the controlled state \
             vocabulary, and the runtime blocks before keeping a telemetry-conformance claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the remote session fires missing or anonymous transition events, proving
/// that anonymous transition telemetry blocks promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_telemetry_conformance_packet_remote_transition_events_missing_blocked(
) -> TelemetryConformancePacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::RemoteSession, |spec| {
        spec.transition_event = TransitionEventEmissionState::TransitionEventsMissingOrAnonymous;
        spec.narrowing_reason = Some(
            "After a dropped tunnel the remote session jumped from reconnecting to ready with no \
             transition event and no controlled actor or subsystem attribution, so the state change \
             appeared in the machine paths as an anonymous jump with no attributable checkpoint \
             boundary, and the session blocks before keeping a telemetry-conformance claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data session's UI and export paths drift on lifecycle naming, proving
/// that a UI/export drift blocks promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_telemetry_conformance_packet_data_ui_export_drift_blocked(
) -> TelemetryConformancePacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::DataSession, |spec| {
        spec.ui_export_parity = UiExportParityState::UiExportLifecycleNamingOrFieldsDrifted;
        spec.narrowing_reason = Some(
            "The data session's UI labeled its degraded state read_only_degraded while the export \
             path emitted a private \"partial\" code and dropped the last-failure-reason field \
             entirely, so the same state read differently in the UI than in a log, dashboard, or \
             packet, and the session blocks before keeping a telemetry-conformance claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the AI action's downstream consumers replace the shared contract with local
/// prose, proving that a shared-contract replacement blocks promotion (red) rather than staying green.
pub fn seeded_m5_lifecycle_telemetry_conformance_packet_ai_shared_contract_local_prose_blocked(
) -> TelemetryConformancePacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::AiAction, |spec| {
        spec.shared_contract_consumption =
            SharedContractConsumptionState::LocalProseReplacesSharedContract;
        spec.narrowing_reason = Some(
            "The claim-publication tooling stopped resolving the AI action's state through the shared \
             lifecycle contract and hand-wrote its own \"applied / needs review\" prose, so Shiproom \
             and Support Center could no longer diagnose the action's state truth from one contract, \
             and the action blocks before keeping a telemetry-conformance claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the extension loses the shared state-truth vocabulary in a headless
/// execution, proving that a headless/companion-adjacent parity loss blocks promotion (red) rather
/// than staying green.
pub fn seeded_m5_lifecycle_telemetry_conformance_packet_extension_headless_parity_lost_blocked(
) -> TelemetryConformancePacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::Extension, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the extension emitted a private lifecycle and transition \
             vocabulary that diverged from the controlled enums shown in-product, so the same \
             capability described its state with a different language depending on how it ran, and \
             the extension blocks before keeping a telemetry-conformance claim.",
        );
    });
    packet_from_rows(rows)
}
