//! Canonical seed builders for the M5 lifecycle-object certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export,
//! and CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both
//! call them so the in-code certification proof, the artifacts, and the fixtures never drift.
//! Every object binding each row certifies — the primary status surface, the exportable
//! status-code field, the last-failure-reason field, the named recovery affordance, the declared
//! consumer surfaces, and the applicable downgrade triggers — is pulled straight from the frozen
//! lifecycle matrix's seeded packet, so the certification cannot audit a family the matrix does
//! not freeze, and the bindings are derived from the matrix rather than restated by hand.

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

/// The lifecycle-object certification posture seeded for one object family.
struct ObjectSpec {
    /// When set, the evaluated-surface set used instead of the matrix's declared set (blocked
    /// fixtures use this to prove a partial certification blocks).
    evaluated_surfaces_override: Option<Vec<M5LifecycleConsumerSurface>>,
    status_surface_binding: StatusSurfaceBindingState,
    status_code_export: StatusCodeExportState,
    last_failure_reason: LastFailureReasonState,
    recovery_affordance_binding: RecoveryAffordanceBindingState,
    headless_parity_preserved: bool,
    waiver: Option<LifecycleObjectWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ObjectSpec {
    /// A full-binding posture: all four bindings hold and headless parity is preserved.
    fn stable() -> Self {
        Self {
            evaluated_surfaces_override: None,
            status_surface_binding: StatusSurfaceBindingState::BoundToOnePrimarySurface,
            status_code_export: StatusCodeExportState::StableCodeExportsEverywhere,
            last_failure_reason: LastFailureReasonState::ControlledReasonReported,
            recovery_affordance_binding: RecoveryAffordanceBindingState::NamedRecoveryPresent,
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

/// Builds one certification row from an object family and a certification posture. Every binding —
/// the primary status surface, status-code field, last-failure-reason field, recovery affordance,
/// declared consumer surfaces, and downgrade triggers — is pulled from the frozen matrix row.
fn row_from_object(object_family: M5LifecycleObjectFamily, spec: ObjectSpec) -> LifecycleObjectRow {
    let source = matrix_row(object_family);
    let required_consumer_surfaces = source.consumer_surfaces.clone();
    let evaluated_consumer_surfaces = spec
        .evaluated_surfaces_override
        .unwrap_or_else(|| source.consumer_surfaces.clone());
    let mut row = LifecycleObjectRow {
        object_family,
        object_label: object_label(object_family).to_owned(),
        qualification: source.qualification,
        owner_role: source.owner_role.clone(),
        scope_summary: source.scope_summary.clone(),
        primary_status_surface: source.primary_status_surface,
        status_code_export_field: source.status_code_export_field.clone(),
        last_failure_reason_field: source.last_failure_reason_field.clone(),
        recovery_affordance: source.recovery_affordance,
        required_consumer_surfaces,
        evaluated_consumer_surfaces,
        status_surface_binding: spec.status_surface_binding,
        status_code_export: spec.status_code_export,
        last_failure_reason: spec.last_failure_reason,
        recovery_affordance_binding: spec.recovery_affordance_binding,
        headless_parity_preserved: spec.headless_parity_preserved,
        applicable_downgrade_triggers: source.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: LifecycleObjectStatus::Green,
        object_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.object_causes = row.recompute_causes();
    row
}

/// Builds the companion status-surface-relocation waiver carried by the seed.
fn companion_surface_relocation_waiver() -> LifecycleObjectWaiver {
    LifecycleObjectWaiver {
        waiver_id: "waiver:companion-surface-relocation:0001".to_owned(),
        object_family: M5LifecycleObjectFamily::CompanionSession,
        reason:
            "When a paired companion device drops, the companion presence badge is unavailable, \
                 so the session's lifecycle state is relocated to a disclosed, still-visible \
                 activity-center reconnect prompt in the primary window rather than vanishing; the \
                 relocation is disclosed, never silent, and the single-surface binding is restored \
                 when the device reconnects."
                .to_owned(),
        owner_role: "Companion owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded lifecycle-object certification posture for one object family.
fn object_spec(object_family: M5LifecycleObjectFamily) -> ObjectSpec {
    match object_family {
        M5LifecycleObjectFamily::CompanionSession => ObjectSpec {
            // A dropped companion device relocates the presence badge to a disclosed, waivered
            // still-visible activity-center prompt rather than losing the status surface.
            status_surface_binding: StatusSurfaceBindingState::DisclosedSurfaceRelocation,
            waiver: Some(companion_surface_relocation_waiver()),
            narrowing_reason: Some(
                "When a paired companion device drops, the companion presence badge is unavailable, \
                 so the session's lifecycle state is relocated to a disclosed, waivered \
                 still-visible activity-center reconnect prompt rather than disappearing, so the \
                 companion object is narrowed and disclosed while a named reconnect stays reachable.",
            ),
            ..ObjectSpec::stable()
        },
        M5LifecycleObjectFamily::ProfilerCapture => ObjectSpec {
            // A headless / in-flight profiler capture exports a disclosed coarse status code until
            // the capture is finalized, while still naming the same controlled state.
            status_code_export: StatusCodeExportState::DisclosedPartialExport,
            narrowing_reason: Some(
                "An in-flight or headless profiler capture exports a disclosed coarse status code \
                 on a subset of surfaces until the capture is finalized, while still naming the same \
                 controlled state, so the profiler object is narrowed and disclosed rather than \
                 losing its exportable code.",
            ),
            ..ObjectSpec::stable()
        },
        M5LifecycleObjectFamily::AiAction => ObjectSpec {
            // A policy-blocked AI action discloses a generic (still-controlled) reason class until
            // the specific class is available, rather than dropping the reason.
            last_failure_reason: LastFailureReasonState::DisclosedGenericReason,
            narrowing_reason: Some(
                "When a policy or upstream control blocks an AI action before the specific reason \
                 class is resolved, the action discloses a generic but still-controlled last-failure \
                 reason class rather than raw text or a missing reason, so the AI object is narrowed \
                 and disclosed.",
            ),
            ..ObjectSpec::stable()
        },
        M5LifecycleObjectFamily::PreviewSession => ObjectSpec {
            // A preview whose live-server dependency is unavailable offers a disclosed reduced
            // rebuild affordance that requires the dependency to return first.
            recovery_affordance_binding: RecoveryAffordanceBindingState::DisclosedReducedRecovery,
            narrowing_reason: Some(
                "When a preview session's live-server dependency is unavailable, the object offers a \
                 disclosed reduced rebuild affordance that requires the dependency to return before \
                 the full rebuild is possible, while still naming a path forward, so the preview \
                 object is narrowed and disclosed.",
            ),
            ..ObjectSpec::stable()
        },
        // Every other object family holds full binding across all four bindings.
        _ => ObjectSpec::stable(),
    }
}

/// Builds the certification rows for the canonical seed, one per governed object family.
fn seeded_rows() -> Vec<LifecycleObjectRow> {
    M5LifecycleObjectFamily::ALL
        .iter()
        .map(|&object_family| row_from_object(object_family, object_spec(object_family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used
/// by the blocked fixtures.
fn seeded_rows_with<F>(target: M5LifecycleObjectFamily, mutate: F) -> Vec<LifecycleObjectRow>
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

fn packet_from_rows(rows: Vec<LifecycleObjectRow>) -> LifecycleObjectPacket {
    build_m5_lifecycle_object_certification_packet(LifecycleObjectInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 lifecycle-object certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Nine object families keep full binding (green). The companion session auto-narrows
/// to yellow with a waivered status-surface relocation, the profiler capture auto-narrows to yellow
/// disclosing a partial status-code export, the AI action auto-narrows to yellow disclosing a
/// generic last-failure reason, and the preview session auto-narrows to yellow disclosing a reduced
/// recovery affordance — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_lifecycle_object_certification_packet() -> LifecycleObjectPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook runtime loses its single primary status surface, proving a
/// lost status surface blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_lifecycle_object_certification_packet_notebook_status_surface_missing_blocked(
) -> LifecycleObjectPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::NotebookRuntime, |spec| {
        spec.status_surface_binding = StatusSurfaceBindingState::StatusSurfaceMissingOrSplit;
        spec.narrowing_reason = Some(
            "The notebook runtime's kernel status split across a panel badge and a competing inline \
             chip with no single authoritative surface, so users can no longer read one lifecycle \
             status and the object blocks before keeping a lifecycle claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the request/API run's status code stops exporting, proving an
/// unexportable status code blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_lifecycle_object_certification_packet_request_status_code_unexportable_blocked(
) -> LifecycleObjectPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::RequestApiRun, |spec| {
        spec.status_code_export = StatusCodeExportState::StatusCodeUnexportable;
        spec.narrowing_reason = Some(
            "The request/API run's stable status code stopped exporting on the support and \
             telemetry paths, so diagnostics can no longer read the same code the UI shows, and the \
             object blocks before keeping a lifecycle claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data session drops its controlled last-failure reason, proving a
/// missing/raw reason blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_lifecycle_object_certification_packet_data_last_failure_missing_blocked(
) -> LifecycleObjectPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::DataSession, |spec| {
        spec.last_failure_reason = LastFailureReasonState::LastFailureReasonMissingOrRaw;
        spec.narrowing_reason = Some(
            "The data session reported a raw driver error string instead of a controlled \
             last-failure reason class, so support and diagnostics fall back to surface-specific \
             heuristics, and the object blocks before keeping a lifecycle claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the companion session loses its named recovery affordance, proving a
/// missing recovery affordance blocks promotion (red) before the row can keep its recovery claim.
pub fn seeded_m5_lifecycle_object_certification_packet_companion_recovery_missing_blocked(
) -> LifecycleObjectPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::CompanionSession, |spec| {
        spec.status_surface_binding = StatusSurfaceBindingState::BoundToOnePrimarySurface;
        spec.waiver = None;
        spec.recovery_affordance_binding =
            RecoveryAffordanceBindingState::RecoveryAffordanceMissing;
        spec.narrowing_reason = Some(
            "After a dropped pairing the companion session showed a degraded state with no named \
             reconnect affordance, leaving the user with no action to take, so the object blocks \
             before keeping its recovery claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the extension loses the shared state-truth vocabulary in a headless
/// execution, proving a headless/companion-adjacent parity loss blocks promotion (red) rather than
/// staying green.
pub fn seeded_m5_lifecycle_object_certification_packet_extension_headless_parity_lost_blocked(
) -> LifecycleObjectPacket {
    let rows = seeded_rows_with(M5LifecycleObjectFamily::Extension, |spec| {
        spec.headless_parity_preserved = false;
        spec.narrowing_reason = Some(
            "In headless / CLI execution the extension reported a private capability-state \
             vocabulary that diverged from the controlled lifecycle states shown in-product, so the \
             same object described a different state language depending on how it ran, and the \
             object blocks before keeping a lifecycle claim.",
        );
    });
    packet_from_rows(rows)
}
