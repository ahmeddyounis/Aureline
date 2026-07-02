//! Canonical seed builders for the M5 durable-progress certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed and blocked fixtures. The
//! headless emitter and the inline tests both call them so the in-code certification
//! proof, the artifacts, and the fixtures never drift. The progress bindings — progress
//! states, source/provider/freshness labels, accessibility routes, required labels,
//! consumer surfaces, downgrade triggers, qualification, owner, and shell zone — are
//! pulled straight from the frozen shell-primitives matrix's seeded progress-indicator
//! and durable-job-row rows (the union across the two progress families), so this proof
//! cannot certify a durable-progress posture the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix::{
    seeded_m5_shell_primitives_matrix, M5ShellPrimitiveRow, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so
/// the checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The two durable-progress primitive families this lane certifies.
const PROGRESS_FAMILIES: [M5ShellPrimitiveFamily; 2] = [
    M5ShellPrimitiveFamily::ProgressIndicator,
    M5ShellPrimitiveFamily::DurableJobRow,
];

/// Canonical order of the ten shell consumer surfaces. `M5ShellConsumerSurface` derives
/// no `Ord` and exposes no `ALL`, so the union of consumer surfaces across the two
/// progress rows is ordered against this local list to stay deterministic.
const CONSUMER_SURFACE_ORDER: [M5ShellConsumerSurface; 10] = [
    M5ShellConsumerSurface::ShellFrame,
    M5ShellConsumerSurface::Windowing,
    M5ShellConsumerSurface::Layout,
    M5ShellConsumerSurface::StatusBar,
    M5ShellConsumerSurface::AttentionRouter,
    M5ShellConsumerSurface::NotificationEnvelope,
    M5ShellConsumerSurface::DocsHelp,
    M5ShellConsumerSurface::ReleaseProof,
    M5ShellConsumerSurface::SupportExport,
    M5ShellConsumerSurface::ProductUi,
];

/// The certification posture seeded for one governed job family.
struct CertificationSpec {
    durable_presence: DurablePresenceState,
    progress_attribution: ProgressAttributionState,
    grouped_history: GroupedHistoryState,
    progress_export: ProgressExportState,
    never_spinner_or_toast_only: bool,
    waiver: Option<DurableProgressCertificationWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: durable after focus loss, fully attributed, grouped
    /// history preserved, reconstructable export.
    fn certified() -> Self {
        Self {
            durable_presence: DurablePresenceState::DurableReviewableAfterFocusLoss,
            progress_attribution: ProgressAttributionState::ActorPhaseActionObjectAttributed,
            grouped_history: GroupedHistoryState::GroupedHistoryAndBlockedReasonsPreserved,
            progress_export: ProgressExportState::ProgressAndHistoryReconstructable,
            never_spinner_or_toast_only: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the sync compacted-history waiver carried by the seed.
fn sync_compacted_history_waiver() -> DurableProgressCertificationWaiver {
    DurableProgressCertificationWaiver {
        waiver_id: "waiver:sync-compacted-grouped-history:0001".to_owned(),
        family: M5DurableJobFamily::Sync,
        reason: "Under the seeded sync lane every in-flight job stays reviewable and each \
                 blocked/paused reason stays reconstructable, but older grouped replication \
                 batches roll up into a digest with a reopen path sooner than the standard \
                 retention window rather than staying enumerated per batch. The compaction is \
                 disclosed, never destructive, and the grouped digest keeps its reopen path into \
                 durable history."
            .to_owned(),
        owner_role: "Shell/activity owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed job family.
fn certification_spec(family: M5DurableJobFamily) -> CertificationSpec {
    match family {
        M5DurableJobFamily::Download => CertificationSpec {
            durable_presence: DurablePresenceState::DisclosedReducedHistoryRetention,
            narrowing_reason: Some(
                "Under the seeded download lane older completed download rows compact into a \
                 summary sooner than the standard retention window while every in-flight download \
                 and its recent history stay reviewable after focus loss; the reduction is \
                 disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5DurableJobFamily::ProviderHandoff => CertificationSpec {
            progress_attribution: ProgressAttributionState::DisclosedCoarseAttribution,
            narrowing_reason: Some(
                "Under the seeded provider-handoff lane a grouped batch shows the handoff \
                 subsystem and provider but folds per-job phase into a summary while the actor, \
                 cancel/retry/open-details actions, and authoritative-object link stay present; \
                 the reduction is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5DurableJobFamily::Sync => CertificationSpec {
            grouped_history: GroupedHistoryState::DisclosedCompactedHistory,
            waiver: Some(sync_compacted_history_waiver()),
            narrowing_reason: Some(
                "The sync lane preserves each blocked/paused reason and keeps every in-flight job \
                 reviewable, but older grouped replication batches roll up into a digest with a \
                 reopen path sooner than the standard retention window; the compaction is disclosed \
                 behind a waiver and never destructive, so the row is narrowed below green while \
                 the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5DurableJobFamily::SupportExport => CertificationSpec {
            progress_export: ProgressExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "The support/export lane's own support export reconstructs current progress and \
                 discloses a partial capture of the recent job-history chronology while the \
                 high-volume export log is still being trimmed; the partial capture is disclosed \
                 and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The two progress rows frozen by the shell-primitives matrix.
fn progress_matrix_rows() -> Vec<M5ShellPrimitiveRow> {
    let rows: Vec<M5ShellPrimitiveRow> = seeded_m5_shell_primitives_matrix()
        .primitive_rows
        .into_iter()
        .filter(|row| PROGRESS_FAMILIES.contains(&row.primitive_family))
        .collect();
    assert_eq!(
        rows.len(),
        PROGRESS_FAMILIES.len(),
        "frozen matrix declares both durable-progress rows"
    );
    rows
}

/// The durable-job-row is the canonical anchor for the shared bindings (qualification,
/// owner, shell zone) — the richest progress surface, honouring the full six-label set and
/// the durable activity-center zone.
fn anchor_row(matrix_rows: &[M5ShellPrimitiveRow]) -> &M5ShellPrimitiveRow {
    matrix_rows
        .iter()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::DurableJobRow)
        .expect("frozen matrix declares a durable-job-row row")
}

/// The most-narrowed qualification across the two progress rows, so a matrix narrowing of
/// either progress family is recorded on the certification row.
fn worst_qualification(matrix_rows: &[M5ShellPrimitiveRow]) -> M5PrimitiveQualificationClass {
    fn rank(q: M5PrimitiveQualificationClass) -> u8 {
        match q {
            M5PrimitiveQualificationClass::Stable => 0,
            M5PrimitiveQualificationClass::Beta => 1,
            M5PrimitiveQualificationClass::Preview => 2,
            M5PrimitiveQualificationClass::Experimental => 3,
            M5PrimitiveQualificationClass::Held => 4,
            M5PrimitiveQualificationClass::Unavailable => 5,
        }
    }
    matrix_rows
        .iter()
        .map(|row| row.qualification)
        .max_by_key(|q| rank(*q))
        .expect("at least one progress row")
}

/// The union of progress states across the two progress rows, in canonical order.
fn union_progress_states(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5ProgressState> {
    M5ProgressState::ALL
        .into_iter()
        .filter(|state| {
            matrix_rows
                .iter()
                .any(|row| row.progress_states.contains(state))
        })
        .collect()
}

/// The union of source/freshness labels across the two progress rows, in canonical order.
fn union_source_freshness_labels(
    matrix_rows: &[M5ShellPrimitiveRow],
) -> Vec<M5SourceFreshnessLabel> {
    M5SourceFreshnessLabel::ALL
        .into_iter()
        .filter(|label| {
            matrix_rows
                .iter()
                .any(|row| row.source_freshness_labels.contains(label))
        })
        .collect()
}

/// The union of required labels across the two progress rows, in canonical order.
fn union_required_labels(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5PrimitiveRequiredLabel> {
    M5PrimitiveRequiredLabel::ALL
        .into_iter()
        .filter(|label| {
            matrix_rows
                .iter()
                .any(|row| row.required_labels.contains(label))
        })
        .collect()
}

/// The union of accessibility routes across the two progress rows, in order.
fn union_accessibility_routes(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5AccessibilityRoute> {
    M5AccessibilityRoute::ALL
        .into_iter()
        .filter(|route| {
            matrix_rows
                .iter()
                .any(|row| row.accessibility_routes.contains(route))
        })
        .collect()
}

/// The union of consumer surfaces across the two progress rows, ordered against the local
/// canonical consumer-surface list.
fn union_consumer_surfaces(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5ShellConsumerSurface> {
    CONSUMER_SURFACE_ORDER
        .into_iter()
        .filter(|surface| {
            matrix_rows
                .iter()
                .any(|row| row.consumer_surfaces.contains(surface))
        })
        .collect()
}

/// The union of downgrade triggers across the two progress rows, ordered against the
/// frozen trigger declaration order (`M5ShellPrimitiveDowngradeTrigger` derives no `Ord`).
fn union_downgrade_triggers(
    matrix_rows: &[M5ShellPrimitiveRow],
) -> Vec<M5ShellPrimitiveDowngradeTrigger> {
    M5ShellPrimitiveDowngradeTrigger::ALL
        .into_iter()
        .filter(|trigger| {
            matrix_rows
                .iter()
                .any(|row| row.downgrade_triggers.contains(trigger))
        })
        .collect()
}

/// Builds one certification row from the frozen progress matrix rows and a posture.
fn row_from_family(
    family: M5DurableJobFamily,
    matrix_rows: &[M5ShellPrimitiveRow],
    spec: CertificationSpec,
) -> DurableProgressCertificationRow {
    let anchor = anchor_row(matrix_rows);
    let mut row = DurableProgressCertificationRow {
        family,
        driven_primitive_families: PROGRESS_FAMILIES.to_vec(),
        matrix_qualification: worst_qualification(matrix_rows),
        owner_role: anchor.owner_role.clone(),
        family_label: family.label().to_owned(),
        shell_zone_slot: anchor.shell_zone_slot,
        certified_progress_states: union_progress_states(matrix_rows),
        certified_source_freshness_labels: union_source_freshness_labels(matrix_rows),
        accessibility_routes: union_accessibility_routes(matrix_rows),
        required_labels: union_required_labels(matrix_rows),
        consumer_surfaces: union_consumer_surfaces(matrix_rows),
        applicable_downgrade_triggers: union_downgrade_triggers(matrix_rows),
        durable_presence: spec.durable_presence,
        progress_attribution: spec.progress_attribution,
        grouped_history: spec.grouped_history,
        progress_export: spec.progress_export,
        never_spinner_or_toast_only: spec.never_spinner_or_toast_only,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: DurableProgressCertificationStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per job family.
fn seeded_rows() -> Vec<DurableProgressCertificationRow> {
    let matrix_rows = progress_matrix_rows();
    M5DurableJobFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, &matrix_rows, certification_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(
    target: M5DurableJobFamily,
    mutate: F,
) -> Vec<DurableProgressCertificationRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = progress_matrix_rows();
    M5DurableJobFamily::ALL
        .iter()
        .map(|&family| {
            let mut spec = certification_spec(family);
            if family == target {
                mutate(&mut spec);
            }
            row_from_family(family, &matrix_rows, spec)
        })
        .collect()
}

fn packet_from_rows(
    rows: Vec<DurableProgressCertificationRow>,
) -> DurableProgressCertificationPacket {
    build_m5_durable_progress_certification_packet(DurableProgressCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 durable-progress certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and
/// CSV artifacts. Five families are certified at full standing (green); the download lane
/// auto-narrows to yellow behind a disclosed reduced history retention, the
/// provider-handoff lane auto-narrows to yellow behind a disclosed coarse attribution, the
/// sync lane auto-narrows to yellow behind a waivered compacted grouped history, and the
/// support/export lane auto-narrows to yellow behind a disclosed partial support-export
/// capture — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_durable_progress_certification_packet() -> DurableProgressCertificationPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the indexing lane's durable work is spinner-or-toast-only,
/// proving a non-durable presence blocks the family (red) rather than passing on behavior
/// alone.
pub fn seeded_m5_durable_progress_certification_packet_indexing_transient_spinner_blocked(
) -> DurableProgressCertificationPacket {
    let rows = seeded_rows_with(M5DurableJobFamily::Indexing, |spec| {
        spec.durable_presence = DurablePresenceState::TransientSpinnerOrToastOnly;
        spec.narrowing_reason = Some(
            "The indexing lane shows its scan only through a transient spinner and a toast on \
             completion, with no durable reopenable row, so progress is lost the moment the user \
             looks away and the row blocks before keeping its durable-presence claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the notebook/runtime lane's attribution or authoritative-object
/// link is missing, proving a missing attribution blocks the family (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_durable_progress_certification_packet_notebook_attribution_missing_blocked(
) -> DurableProgressCertificationPacket {
    let rows = seeded_rows_with(M5DurableJobFamily::NotebookRuntime, |spec| {
        spec.progress_attribution = ProgressAttributionState::AttributionOrObjectLinkMissing;
        spec.narrowing_reason = Some(
            "The notebook/runtime lane's grouped execution row hides which cell/kernel is running \
             and drops the link back to the notebook object, so the batch reads as an anonymous \
             spinner and the row blocks before keeping its progress-attribution claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the request/data-load lane's grouped history or blocked reason is
/// lost, proving a lost history blocks the family (red) before the row can keep its
/// grouped-history claim.
pub fn seeded_m5_durable_progress_certification_packet_request_history_lost_blocked(
) -> DurableProgressCertificationPacket {
    let rows = seeded_rows_with(M5DurableJobFamily::RequestDataLoad, |spec| {
        spec.grouped_history = GroupedHistoryState::HistoryOrBlockedReasonLost;
        spec.narrowing_reason = Some(
            "The request/data-load lane drops a failed batch and its failure reason from history \
             once the toast dismisses, and a policy-blocked load gives no blocked reason, so the \
             failure digest is unrecoverable and the row blocks before keeping its grouped-history \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the update lane's progress state is absent from the
/// support-export capture, proving a missing export blocks the family (red) before the row
/// can keep its progress-export claim.
pub fn seeded_m5_durable_progress_certification_packet_update_progress_absent_from_capture_blocked(
) -> DurableProgressCertificationPacket {
    let rows = seeded_rows_with(M5DurableJobFamily::Update, |spec| {
        spec.progress_export = ProgressExportState::ProgressStateAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The update lane's support export omits current update progress and the recent \
             job-history chronology entirely, so a stuck update cannot be explained without a live \
             dashboard, and the row blocks before keeping its progress-export claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the branch-agent lane has a job represented only by a transient
/// spinner or toast, proving the spinner-or-toast-only invariant blocks the family (red)
/// before the row can keep its invariant.
pub fn seeded_m5_durable_progress_certification_packet_branch_agent_spinner_or_toast_only_blocked(
) -> DurableProgressCertificationPacket {
    let rows = seeded_rows_with(M5DurableJobFamily::BranchAgent, |spec| {
        spec.never_spinner_or_toast_only = false;
        spec.narrowing_reason = Some(
            "The branch-agent lane's automation run is surfaced only by a transient spinner and a \
             completion toast, with no durable reopenable job row, so its progress is \
             spinner-or-toast-only and the row blocks before keeping its spinner-only invariant.",
        );
    });
    packet_from_rows(rows)
}
