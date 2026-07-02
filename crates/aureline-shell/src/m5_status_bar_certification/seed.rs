//! Canonical seed builders for the M5 status-bar certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed and blocked fixtures. The
//! headless emitter and the inline tests both call them so the in-code certification
//! proof, the artifacts, and the fixtures never drift. The ambient bindings —
//! status-item classes, overflow behaviors, freshness labels, accessibility routes,
//! required labels, consumer surfaces, downgrade triggers, qualification, owner, and
//! shell zone — are pulled straight from the frozen shell-primitives matrix's seeded
//! status-bar-item row, so this proof cannot certify an ambient posture the matrix
//! does not freeze.

use super::*;
use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix::{
    seeded_m5_shell_primitives_matrix, M5ShellPrimitiveRow, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value
/// so the checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The certification posture seeded for one governed status context.
struct CertificationSpec {
    placement_stability: PlacementStabilityState,
    overflow_discoverability: OverflowDiscoverabilityState,
    inspector_backlink: InspectorBacklinkState,
    support_export_parity: SupportExportParityState,
    keyboard_reachable_without_hover: bool,
    waiver: Option<StatusBarCertificationWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: stable priority slots, keyboard/menu/palette-
    /// reachable overflow, narrowest inspector back-links, reconstructable export.
    fn certified() -> Self {
        Self {
            placement_stability: PlacementStabilityState::StablePrioritySlotsNoJitter,
            overflow_discoverability: OverflowDiscoverabilityState::KeyboardMenuPaletteReachable,
            inspector_backlink: InspectorBacklinkState::EveryItemBacklinksToNarrowestInspector,
            support_export_parity:
                SupportExportParityState::VisibleAndOverflowedItemsReconstructable,
            keyboard_reachable_without_hover: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the incident compact-priority-compaction waiver carried by the seed.
fn incident_compact_priority_waiver() -> StatusBarCertificationWaiver {
    StatusBarCertificationWaiver {
        waiver_id: "waiver:incident-compact-priority-compaction:0001".to_owned(),
        context: M5StatusContext::IncidentLane,
        reason: "Under the seeded compact incident-response width, ambient-metadata status items \
                 compact into a disclosed summary chip while the recovery-critical incident state \
                 and the execution-context target stay pinned in their stable slots. The \
                 compaction is disclosed, never hidden, and every compacted item stays reachable \
                 through keyboard search, the status menu, and the palette."
            .to_owned(),
        owner_role: "Incident surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed status context.
fn certification_spec(context: M5StatusContext) -> CertificationSpec {
    match context {
        M5StatusContext::RemoteLane => CertificationSpec {
            overflow_discoverability: OverflowDiscoverabilityState::DisclosedReducedOverflowRoute,
            narrowing_reason: Some(
                "The remote lane's status-menu overflow route is temporarily reduced while the \
                 remote-target registry re-syncs; keyboard search and the palette route still \
                 resolve every visible and overflowed item, and the reduction is disclosed. The \
                 row is narrowed below green while the route is reduced.",
            ),
            ..CertificationSpec::certified()
        },
        M5StatusContext::PreviewLane => CertificationSpec {
            inspector_backlink: InspectorBacklinkState::DisclosedGroupedBacklink,
            narrowing_reason: Some(
                "The preview lane's provider-freshness status items share one disclosed grouped \
                 inspector back-link into the preview provenance panel rather than an individual \
                 narrowest target; the grouping is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5StatusContext::ProfilerLane => CertificationSpec {
            support_export_parity: SupportExportParityState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "The profiler lane's support export reconstructs the visible status items and \
                 discloses a partial capture of the sampled capacity-meter overflow set while the \
                 sampler warms up; the partial capture is disclosed and the row is narrowed below \
                 green.",
            ),
            ..CertificationSpec::certified()
        },
        M5StatusContext::IncidentLane => CertificationSpec {
            placement_stability: PlacementStabilityState::DisclosedCompactPriorityCompaction,
            waiver: Some(incident_compact_priority_waiver()),
            narrowing_reason: Some(
                "Under the seeded compact incident width the status bar performs a disclosed, \
                 waivered priority compaction that drops only ambient-metadata items; \
                 recovery-critical and execution-context items stay pinned, and the row is \
                 narrowed below green while the compaction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The ambient status-bar-item row frozen by the shell-primitives matrix. Every
/// certification row pulls its ambient bindings from this one matrix row so the
/// proof mints no parallel status vocabulary.
fn matrix_status_bar_item_row() -> M5ShellPrimitiveRow {
    seeded_m5_shell_primitives_matrix()
        .primitive_rows
        .into_iter()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::StatusBarItem)
        .expect("frozen matrix declares a status-bar-item row")
}

/// Builds one certification row from the frozen ambient matrix row and a posture.
fn row_from_context(
    context: M5StatusContext,
    matrix_row: &M5ShellPrimitiveRow,
    spec: CertificationSpec,
) -> StatusBarCertificationRow {
    let mut row = StatusBarCertificationRow {
        context,
        driven_primitive_families: vec![
            M5ShellPrimitiveFamily::StatusBarItem,
            M5ShellPrimitiveFamily::StatusOverflowMenu,
        ],
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        context_label: context.label().to_owned(),
        shell_zone_slot: matrix_row.shell_zone_slot,
        certified_priority_classes: M5StatusPriorityClass::ALL.to_vec(),
        certified_reach_routes: M5StatusReachRoute::ALL.to_vec(),
        certified_status_item_classes: matrix_row.status_item_classes.clone(),
        overflow_behaviors: matrix_row.overflow_behaviors.clone(),
        source_freshness_labels: matrix_row.source_freshness_labels.clone(),
        accessibility_routes: matrix_row.accessibility_routes.clone(),
        required_labels: matrix_row.required_labels.clone(),
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        placement_stability: spec.placement_stability,
        overflow_discoverability: spec.overflow_discoverability,
        inspector_backlink: spec.inspector_backlink,
        support_export_parity: spec.support_export_parity,
        keyboard_reachable_without_hover: spec.keyboard_reachable_without_hover,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: StatusBarCertificationStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per status context.
fn seeded_rows() -> Vec<StatusBarCertificationRow> {
    let matrix_row = matrix_status_bar_item_row();
    M5StatusContext::ALL
        .iter()
        .map(|&context| row_from_context(context, &matrix_row, certification_spec(context)))
        .collect()
}

/// Builds a variant where one context's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5StatusContext, mutate: F) -> Vec<StatusBarCertificationRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_row = matrix_status_bar_item_row();
    M5StatusContext::ALL
        .iter()
        .map(|&context| {
            let mut spec = certification_spec(context);
            if context == target {
                mutate(&mut spec);
            }
            row_from_context(context, &matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<StatusBarCertificationRow>) -> StatusBarCertificationPacket {
    build_m5_status_bar_certification_packet(StatusBarCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 status-bar certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export,
/// and CSV artifacts. Four contexts are certified at full standing (green); the
/// remote lane auto-narrows to yellow behind a disclosed reduced overflow route, the
/// preview lane auto-narrows to yellow behind a disclosed grouped inspector
/// back-link, the profiler lane auto-narrows to yellow behind a disclosed partial
/// support-export capture, and the incident lane auto-narrows to yellow behind a
/// waivered compact priority compaction — and no row is blocked, so the packet is
/// clean and every row is publishable.
pub fn seeded_m5_status_bar_certification_packet() -> StatusBarCertificationPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook lane's priority slots jitter and reflow
/// around a vanity item, proving unstable placement blocks promotion (red) rather
/// than passing on behavior alone.
pub fn seeded_m5_status_bar_certification_packet_notebook_vanity_reflow_blocked(
) -> StatusBarCertificationPacket {
    let rows = seeded_rows_with(M5StatusContext::NotebookLane, |spec| {
        spec.placement_stability = PlacementStabilityState::UnstableSlotsOrVanityReflow;
        spec.narrowing_reason = Some(
            "The notebook lane's status bar reflowed around a run-spinner vanity item, jittering \
             the recovery-critical kernel-state slot, so the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data/API lane's overflow is reachable only through
/// hover, proving a hover-only overflow blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_status_bar_certification_packet_data_api_overflow_hover_only_blocked(
) -> StatusBarCertificationPacket {
    let rows = seeded_rows_with(M5StatusContext::DataApiLane, |spec| {
        spec.overflow_discoverability = OverflowDiscoverabilityState::OverflowHoverOrPointerOnly;
        spec.narrowing_reason = Some(
            "The data/API lane's status overflow is reachable only through pointer hover, with no \
             keyboard-search, status-menu, or palette route, so the row blocks before keeping its \
             overflow-discoverability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the review lane has an item whose inspector back-link is
/// missing / a generic settings detour, proving a missing back-link blocks promotion
/// (red) before the row can keep its inspector-back-link claim.
pub fn seeded_m5_status_bar_certification_packet_review_backlink_missing_blocked(
) -> StatusBarCertificationPacket {
    let rows = seeded_rows_with(M5StatusContext::ReviewLane, |spec| {
        spec.inspector_backlink = InspectorBacklinkState::BacklinkMissingOrGenericDetour;
        spec.narrowing_reason = Some(
            "The review lane's problem-count status item dumps into a generic settings detour \
             rather than the narrowest review inspector, so the row blocks before keeping its \
             inspector-back-link claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the preview lane's critical-state displacement is absent
/// from the support-export capture, proving an unrecoverable export blocks promotion
/// (red) before the row can keep its support-export claim.
pub fn seeded_m5_status_bar_certification_packet_preview_capture_absent_blocked(
) -> StatusBarCertificationPacket {
    let rows = seeded_rows_with(M5StatusContext::PreviewLane, |spec| {
        spec.support_export_parity =
            SupportExportParityState::CriticalDisplacementAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The preview lane's stale-preview critical-state displacement is absent from the \
             support-export capture, so the ambient shell state cannot be reconstructed without a \
             screenshot and the row blocks before keeping its support-export claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the desktop base lane keeps a status item reachable only
/// through hover, proving a hover-only critical truth blocks promotion (red) before
/// the row can keep its keyboard-reachability claim.
pub fn seeded_m5_status_bar_certification_packet_desktop_base_hover_only_blocked(
) -> StatusBarCertificationPacket {
    let rows = seeded_rows_with(M5StatusContext::DesktopBaseLane, |spec| {
        spec.keyboard_reachable_without_hover = false;
        spec.narrowing_reason = Some(
            "The desktop base lane's sync-freshness status item keeps its critical truth \
             reachable only through pointer hover, so the row blocks before keeping its \
             keyboard-reachability claim.",
        );
    });
    packet_from_rows(rows)
}
