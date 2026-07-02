//! Canonical seed builders for the M5 ambient-instrumentation stability proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed and blocked fixtures. The headless
//! emitter and the inline tests both call them so the in-code certification proof, the
//! artifacts, and the fixtures never drift. The ambient bindings — status-item classes,
//! overflow behaviors, source/provider/freshness labels, accessibility routes, required
//! labels, consumer surfaces, downgrade triggers, qualification, owner, and shell zone —
//! are pulled straight from the frozen shell-primitives matrix's seeded status-bar-item,
//! status-overflow-menu, and progress-indicator rows (the union across the three ambient
//! families), so this proof cannot certify an ambient-stability posture the matrix does not
//! freeze.

use super::*;
use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix::{
    seeded_m5_shell_primitives_matrix, M5ShellPrimitiveRow, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID,
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

/// The three ambient-instrumentation primitive families this lane certifies: the status-bar
/// item and overflow menu carry the status-item classes and overflow behaviors, and the
/// progress indicator carries the ambient counters and spinners.
const AMBIENT_FAMILIES: [M5ShellPrimitiveFamily; 3] = [
    M5ShellPrimitiveFamily::StatusBarItem,
    M5ShellPrimitiveFamily::StatusOverflowMenu,
    M5ShellPrimitiveFamily::ProgressIndicator,
];

/// Canonical order of the ten shell consumer surfaces. `M5ShellConsumerSurface` derives no
/// `Ord` and exposes no `ALL`, so the union of consumer surfaces across the three ambient
/// rows is ordered against this local list to stay deterministic.
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

/// The certification posture seeded for one governed rendering profile.
struct CertificationSpec {
    counter_stability: CounterSpinnerStabilityState,
    overflow_searchability: OverflowSearchabilityState,
    grouped_summary: GroupedSummaryState,
    stability_export: StabilityExportState,
    never_reflows_around_vanity_items: bool,
    waiver: Option<AmbientStabilityWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: stable counters, searchable overflow, grouped summaries,
    /// reconstructable export.
    fn certified() -> Self {
        Self {
            counter_stability: CounterSpinnerStabilityState::CounterSpinnerSummaryStableNoReflow,
            overflow_searchability:
                OverflowSearchabilityState::OverflowItemsPaletteSearchableSameLabels,
            grouped_summary: GroupedSummaryState::MultiJobGroupedIntoOneSummary,
            stability_export: StabilityExportState::StabilityFixturesAndExportReconstructable,
            never_reflows_around_vanity_items: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the reduced-motion coarse-grouping waiver carried by the seed.
fn reduced_motion_coarse_grouping_waiver() -> AmbientStabilityWaiver {
    AmbientStabilityWaiver {
        waiver_id: "waiver:reduced-motion-coarse-grouping:0001".to_owned(),
        profile: M5AmbientStabilityProfile::ReducedMotion,
        reason: "Under the seeded reduced-motion profile the shell folds distinct job classes \
                 into one summarized-work chip sooner than the standard threshold to avoid \
                 animating many primitives at once, while the summary stays meaningful, keeps its \
                 count, and each job stays reachable from the activity center. The coarse grouping \
                 is disclosed, never hides a job, and keeps the reopen path into durable history."
            .to_owned(),
        owner_role: "Shell/status-bar owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed rendering profile.
fn certification_spec(profile: M5AmbientStabilityProfile) -> CertificationSpec {
    match profile {
        M5AmbientStabilityProfile::Compact => CertificationSpec {
            counter_stability: CounterSpinnerStabilityState::DisclosedReducedCounterDetail,
            narrowing_reason: Some(
                "Under the seeded compact profile a wide problem/background-work count abbreviates \
                 to a magnitude (for example `99+`) and a spinner label shortens to fit the \
                 reduced-width strip, while every counter keeps its stable placement, identity, and \
                 meaning; the reduction is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5AmbientStabilityProfile::HighZoom => CertificationSpec {
            overflow_searchability:
                OverflowSearchabilityState::DisclosedReducedOverflowSearchDetail,
            narrowing_reason: Some(
                "Under the seeded high-zoom profile the status-menu overflow search shows a shorter \
                 explanation and groups low-priority results to fit the large-text layout, while \
                 every overflowed item stays discoverable from the palette/status search and keeps \
                 its original label; the reduction is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5AmbientStabilityProfile::ReducedMotion => CertificationSpec {
            grouped_summary: GroupedSummaryState::DisclosedCoarseGrouping,
            waiver: Some(reduced_motion_coarse_grouping_waiver()),
            narrowing_reason: Some(
                "The reduced-motion profile folds distinct job classes into one summarized-work chip \
                 sooner than the standard threshold to avoid animating many primitives at once, \
                 while the summary stays meaningful and each job stays reachable; the coarse grouping \
                 is disclosed behind a waiver and never hides a job, so the row is narrowed below \
                 green while the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5AmbientStabilityProfile::DegradedNetwork => CertificationSpec {
            stability_export: StabilityExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "Under the seeded degraded-network profile the support export reconstructs the \
                 status items, counters, and grouped summaries but discloses a partial capture of \
                 the low-priority overflow entries while the export queue is throttled; the partial \
                 capture is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The three ambient rows frozen by the shell-primitives matrix.
fn ambient_matrix_rows() -> Vec<M5ShellPrimitiveRow> {
    let rows: Vec<M5ShellPrimitiveRow> = seeded_m5_shell_primitives_matrix()
        .primitive_rows
        .into_iter()
        .filter(|row| AMBIENT_FAMILIES.contains(&row.primitive_family))
        .collect();
    assert_eq!(
        rows.len(),
        AMBIENT_FAMILIES.len(),
        "frozen matrix declares all three ambient-instrumentation rows"
    );
    rows
}

/// The status-bar item is the canonical anchor for the shared bindings (qualification,
/// owner, shell zone) — the primary ambient truth item in the status-bar zone.
fn anchor_row(matrix_rows: &[M5ShellPrimitiveRow]) -> &M5ShellPrimitiveRow {
    matrix_rows
        .iter()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::StatusBarItem)
        .expect("frozen matrix declares a status-bar-item row")
}

/// The most-narrowed qualification across the three ambient rows, so a matrix narrowing of
/// any ambient family is recorded on the certification row.
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
        .expect("at least one ambient row")
}

/// The union of status-item classes across the three ambient rows, in canonical order.
fn union_status_item_classes(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5StatusItemClass> {
    M5StatusItemClass::ALL
        .into_iter()
        .filter(|class| {
            matrix_rows
                .iter()
                .any(|row| row.status_item_classes.contains(class))
        })
        .collect()
}

/// The union of overflow behaviors across the three ambient rows, in canonical order.
fn union_overflow_behaviors(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5OverflowBehavior> {
    M5OverflowBehavior::ALL
        .into_iter()
        .filter(|behavior| {
            matrix_rows
                .iter()
                .any(|row| row.overflow_behaviors.contains(behavior))
        })
        .collect()
}

/// The union of source/freshness labels across the three ambient rows, in canonical order.
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

/// The union of required labels across the three ambient rows, in canonical order.
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

/// The union of accessibility routes across the three ambient rows, in order.
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

/// The union of consumer surfaces across the three ambient rows, ordered against the local
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

/// The union of downgrade triggers across the three ambient rows, ordered against the frozen
/// trigger declaration order (`M5ShellPrimitiveDowngradeTrigger` derives no `Ord`).
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

/// Builds one certification row from the frozen ambient matrix rows and a posture.
fn row_from_profile(
    profile: M5AmbientStabilityProfile,
    matrix_rows: &[M5ShellPrimitiveRow],
    spec: CertificationSpec,
) -> AmbientStabilityRow {
    let anchor = anchor_row(matrix_rows);
    let mut row = AmbientStabilityRow {
        profile,
        driven_primitive_families: AMBIENT_FAMILIES.to_vec(),
        matrix_qualification: worst_qualification(matrix_rows),
        owner_role: anchor.owner_role.clone(),
        profile_label: profile.label().to_owned(),
        shell_zone_slot: anchor.shell_zone_slot,
        certified_status_item_classes: union_status_item_classes(matrix_rows),
        certified_overflow_behaviors: union_overflow_behaviors(matrix_rows),
        certified_source_freshness_labels: union_source_freshness_labels(matrix_rows),
        accessibility_routes: union_accessibility_routes(matrix_rows),
        required_labels: union_required_labels(matrix_rows),
        consumer_surfaces: union_consumer_surfaces(matrix_rows),
        applicable_downgrade_triggers: union_downgrade_triggers(matrix_rows),
        counter_stability: spec.counter_stability,
        overflow_searchability: spec.overflow_searchability,
        grouped_summary: spec.grouped_summary,
        stability_export: spec.stability_export,
        never_reflows_around_vanity_items: spec.never_reflows_around_vanity_items,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: AmbientStabilityStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per rendering profile.
fn seeded_rows() -> Vec<AmbientStabilityRow> {
    let matrix_rows = ambient_matrix_rows();
    M5AmbientStabilityProfile::ALL
        .iter()
        .map(|&profile| row_from_profile(profile, &matrix_rows, certification_spec(profile)))
        .collect()
}

/// Builds a variant where one profile's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5AmbientStabilityProfile, mutate: F) -> Vec<AmbientStabilityRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = ambient_matrix_rows();
    M5AmbientStabilityProfile::ALL
        .iter()
        .map(|&profile| {
            let mut spec = certification_spec(profile);
            if profile == target {
                mutate(&mut spec);
            }
            row_from_profile(profile, &matrix_rows, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<AmbientStabilityRow>) -> AmbientStabilityPacket {
    build_m5_ambient_instrumentation_stability_packet(AmbientStabilityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 ambient-instrumentation stability packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Four profiles are certified at full standing (green); the compact profile
/// auto-narrows to yellow behind a disclosed reduced counter detail, the high-zoom profile
/// auto-narrows to yellow behind a disclosed reduced overflow-search detail, the
/// reduced-motion profile auto-narrows to yellow behind a waivered coarse grouping, and the
/// degraded-network profile auto-narrows to yellow behind a disclosed partial support-export
/// capture — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_ambient_instrumentation_stability_packet() -> AmbientStabilityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the compact profile's status strip reflows or flickers when
/// counters update, proving an unstable strip blocks the profile (red) rather than passing on
/// behavior alone.
pub fn seeded_m5_ambient_instrumentation_stability_packet_compact_status_reflow_blocked(
) -> AmbientStabilityPacket {
    let rows = seeded_rows_with(M5AmbientStabilityProfile::Compact, |spec| {
        spec.counter_stability = CounterSpinnerStabilityState::StatusReflowsOrFlickersOnUpdate;
        spec.narrowing_reason = Some(
            "Under the compact profile the status strip re-lays-out and flickers every time the \
             background-work counter and sync spinner tick, so ambient state jitters and header \
             layout churns, and the row blocks before keeping its counter-stability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the expanded profile's overflowed item is undiscoverable from the
/// palette/status search, proving a lost overflow item blocks the profile (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_ambient_instrumentation_stability_packet_expanded_overflow_undiscoverable_blocked(
) -> AmbientStabilityPacket {
    let rows = seeded_rows_with(M5AmbientStabilityProfile::Expanded, |spec| {
        spec.overflow_searchability =
            OverflowSearchabilityState::OverflowItemUndiscoverableOrRelabeled;
        spec.narrowing_reason = Some(
            "Under the expanded profile a displaced connection-target item drops out of the \
             command-palette and status-menu search and is relabeled in the overflow menu, so the \
             only way to find it is to hover the collapsed chip, and the row blocks before keeping \
             its overflow-searchability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the multi-window profile shows many flickering primitives instead
/// of one grouped summary, proving a flickering churn blocks the profile (red) before the row
/// can keep its grouped-summary claim.
pub fn seeded_m5_ambient_instrumentation_stability_packet_multi_window_flickering_primitives_blocked(
) -> AmbientStabilityPacket {
    let rows = seeded_rows_with(M5AmbientStabilityProfile::MultiWindow, |spec| {
        spec.grouped_summary = GroupedSummaryState::ManyFlickeringPrimitivesInsteadOfSummary;
        spec.narrowing_reason = Some(
            "Under the multi-window profile each detached window renders one status primitive per \
             active job instead of folding them into a single summary chip, so the strip churns \
             with flickering primitives as jobs update, and the row blocks before keeping its \
             grouped-summary claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the degraded-network profile's ambient state is absent from the
/// support-export capture, proving a missing export blocks the profile (red) before the row
/// can keep its stability-export claim.
pub fn seeded_m5_ambient_instrumentation_stability_packet_degraded_network_export_absent_blocked(
) -> AmbientStabilityPacket {
    let rows = seeded_rows_with(M5AmbientStabilityProfile::DegradedNetwork, |spec| {
        spec.stability_export = StabilityExportState::StabilityStateAbsentFromCapture;
        spec.narrowing_reason = Some(
            "Under the degraded-network profile the support export omits the status items, \
             counters, and overflow entries entirely, so a reflow or a lost overflow item cannot \
             be explained without a live screenshot, and the row blocks before keeping its \
             stability-export claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the high-zoom profile's status bar reflows around a vanity item,
/// proving the vanity-reflow invariant blocks the profile (red) before the row can keep its
/// invariant.
pub fn seeded_m5_ambient_instrumentation_stability_packet_high_zoom_vanity_reflow_blocked(
) -> AmbientStabilityPacket {
    let rows = seeded_rows_with(M5AmbientStabilityProfile::HighZoom, |spec| {
        spec.never_reflows_around_vanity_items = false;
        spec.narrowing_reason = Some(
            "Under the high-zoom profile the status bar re-lays-out around a decorative badge, \
             displacing a truth-bearing problem-count item into the overflow, so the strip is not \
             overflow-safe and the row blocks before keeping its vanity-reflow invariant.",
        );
    });
    packet_from_rows(rows)
}
