//! Canonical seed builders for the M5 responsive-collapse proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed fixtures. The headless
//! emitter and the inline tests both call them so the in-code collapse proof, the
//! artifacts, and the fixtures never drift. The certified rows are pulled straight
//! from the frozen shell-zone matrix's seeded packet, so the collapse proof cannot
//! certify a family the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    seeded_m5_shell_zone_matrix, M5ShellSurfaceRow, M5_SHELL_ZONE_MATRIX_PACKET_ID,
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

/// The responsive-collapse posture seeded for one governed family.
struct CollapseSpec {
    /// When set, the collapse ladder used instead of the matrix ladder (blocked
    /// fixtures use this to prove a ladder without a placeholder terminal blocks).
    collapse_ladder_override: Option<Vec<M5FallbackPlacement>>,
    collapse_ladder_state: CollapseLadderState,
    identity_continuity: IdentityContinuityState,
    critical_action_reach: CriticalActionReachState,
    zoom_contrast_parity: ZoomContrastParityState,
    waiver: Option<ResponsiveCollapseWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CollapseSpec {
    /// A fully identity-stable posture: every dimension at full standing.
    fn stable() -> Self {
        Self {
            collapse_ladder_override: None,
            collapse_ladder_state: CollapseLadderState::IdentityStableLadder,
            identity_continuity: IdentityContinuityState::IdentityAndStatePreserved,
            critical_action_reach: CriticalActionReachState::AllCriticalAndActionsReachable,
            zoom_contrast_parity: ZoomContrastParityState::RoutesStableAtZoomAndContrast,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the companion docked-to-sheet state-rehydration waiver carried by the seed.
fn companion_state_rehydration_waiver() -> ResponsiveCollapseWaiver {
    ResponsiveCollapseWaiver {
        waiver_id: "waiver:companion-sheet-rehydration:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "Under compact width the companion surface moves from its right-inspector sheet to \
                 a keyboard-reachable overflow and rehydrates its in-progress prompt state through a \
                 disclosed restore path while the state-serialization contract is unified in the \
                 next sync. The object identity is preserved and the rehydration is disclosed, never \
                 silent."
            .to_owned(),
        owner_role: "Companion surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded responsive-collapse posture for one governed family.
fn collapse_spec(family: M5ShellSurfaceFamily) -> CollapseSpec {
    match family {
        M5ShellSurfaceFamily::Profiler => CollapseSpec {
            // A Stable family narrowed only by responsive behavior: under compact
            // width one low-frequency profiler tool moves to a disclosed
            // keyboard-reachable overflow before the capture readout is starved.
            critical_action_reach: CriticalActionReachState::DisclosedOverflowReach,
            narrowing_reason: Some(
                "Under compact width the profiler moves one low-frequency capture tool to a \
                 disclosed keyboard-reachable overflow before the primary capture readout is \
                 starved; the row is narrowed below green while critical state stays visible.",
            ),
            ..CollapseSpec::stable()
        },
        M5ShellSurfaceFamily::Incident => CollapseSpec {
            narrowing_reason: Some(
                "The incident surface is qualified at Beta in the frozen shell-zone matrix; its \
                 compact/standard/expanded presentation is identity-stable but the claim is \
                 narrowed below Stable and disclosed.",
            ),
            ..CollapseSpec::stable()
        },
        M5ShellSurfaceFamily::Companion => CollapseSpec {
            // Beta-qualified and, under compact width, rehydrates in-progress state
            // across the sheet→overflow transition through a disclosed, waivered path.
            identity_continuity: IdentityContinuityState::DisclosedStateRehydration,
            waiver: Some(companion_state_rehydration_waiver()),
            narrowing_reason: Some(
                "The companion surface is qualified at Beta; under compact width its \
                 docked-to-sheet-to-overflow transition rehydrates in-progress prompt state through \
                 a disclosed, waivered restore path while preserving the object identity.",
            ),
            ..CollapseSpec::stable()
        },
        M5ShellSurfaceFamily::Operator => CollapseSpec {
            // Beta-qualified and discloses a narrowed high-zoom presentation while
            // keeping the same routes and task state.
            zoom_contrast_parity: ZoomContrastParityState::DisclosedZoomNarrowing,
            narrowing_reason: Some(
                "The operator surface is qualified at Beta; at 400% zoom its bottom-panel controls \
                 disclose a stacked, narrowed presentation while exposing the same routes and task \
                 state, so the claim is narrowed and disclosed.",
            ),
            ..CollapseSpec::stable()
        },
        _ => CollapseSpec::stable(),
    }
}

/// Builds the per-class presentations for a family from its collapse ladder.
///
/// At expanded and standard widths the surface lands at the most-docked ladder entry;
/// under compact width it moves one rung down the ladder (docked→sheet, sheet→overflow,
/// etc.), so optional detail always sheds before the primary surface is starved and
/// the presentations stay monotonic across the three classes.
fn class_presentations(ladder: &[M5FallbackPlacement]) -> Vec<ResponsiveClassPresentation> {
    let full = ladder.first().copied().unwrap_or(M5FallbackPlacement::Docked);
    let compact = ladder.get(1).copied().unwrap_or(full);
    M5ResponsiveClass::ALL
        .iter()
        .map(|&responsive_class| {
            let placement = match responsive_class {
                M5ResponsiveClass::CompactDesktop => compact,
                M5ResponsiveClass::StandardDesktop | M5ResponsiveClass::ExpandedDesktop => full,
            };
            ResponsiveClassPresentation {
                responsive_class,
                placement,
                identity_preserved: true,
                essential_actions_reachable: true,
            }
        })
        .collect()
}

/// Builds one collapse row from a frozen matrix row and a collapse posture.
fn row_from_matrix(matrix_row: &M5ShellSurfaceRow, spec: CollapseSpec) -> ResponsiveCollapseRow {
    let collapse_ladder = spec
        .collapse_ladder_override
        .unwrap_or_else(|| matrix_row.fallback_placements.clone());
    let class_presentations = class_presentations(&collapse_ladder);
    let mut row = ResponsiveCollapseRow {
        family: matrix_row.family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        surface_label: surface_label(matrix_row.family).to_owned(),
        canonical_slot: matrix_row.canonical_slot,
        fallback_slot: matrix_row.fallback_slot,
        declared_responsive_classes: matrix_row.responsive_classes.clone(),
        collapse_ladder,
        class_presentations,
        collapse_ladder_state: spec.collapse_ladder_state,
        identity_continuity: spec.identity_continuity,
        critical_action_reach: spec.critical_action_reach,
        zoom_contrast_parity: spec.zoom_contrast_parity,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: ResponsiveCollapseStatus::Green,
        collapse_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.collapse_causes = row.recompute_causes();
    row
}

/// Builds the collapse rows for the canonical seed, one per matrix family.
fn seeded_rows() -> Vec<ResponsiveCollapseRow> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, collapse_spec(matrix_row.family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellSurfaceFamily, mutate: F) -> Vec<ResponsiveCollapseRow>
where
    F: Fn(&mut CollapseSpec),
{
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = collapse_spec(matrix_row.family);
            if matrix_row.family == target {
                mutate(&mut spec);
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<ResponsiveCollapseRow>) -> ResponsiveCollapsePacket {
    build_m5_responsive_collapse_packet(ResponsiveCollapseInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 responsive-collapse packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export,
/// and CSV artifacts. Six families stay identity-stable across every class (green);
/// the profiler auto-narrows to yellow moving a low-frequency tool to a disclosed
/// overflow, the incident surface auto-narrows to yellow on its Beta qualification,
/// the companion surface auto-narrows to yellow with a waivered docked-to-sheet
/// state rehydration, and the operator surface auto-narrows to yellow disclosing a
/// narrowed high-zoom presentation — and no row is blocked, so the packet is clean
/// and every row is publishable.
pub fn seeded_m5_responsive_collapse_packet() -> ResponsiveCollapsePacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook's responsive collapse changes the task
/// identity, proving a collapse that reframes the task blocks promotion (red) rather
/// than passing on behavior alone.
pub fn seeded_m5_responsive_collapse_packet_notebook_collapse_identity_blocked(
) -> ResponsiveCollapsePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Notebook, |spec| {
        spec.collapse_ladder_state = CollapseLadderState::LadderChangesIdentity;
        spec.narrowing_reason = Some(
            "The notebook surface's compact collapse reframed the cell editor as a read-only \
             preview, changing the task identity, so the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the profiler hides critical capture state on collapse
/// instead of overflowing it, proving hidden critical state blocks promotion (red)
/// rather than staying a disclosed yellow.
pub fn seeded_m5_responsive_collapse_packet_profiler_critical_state_hidden_blocked(
) -> ResponsiveCollapsePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Profiler, |spec| {
        spec.critical_action_reach = CriticalActionReachState::CriticalStateHidden;
        spec.narrowing_reason = Some(
            "The profiler hid its live capture readout under compact width instead of moving it to \
             a keyboard-reachable overflow, so the row blocks before keeping a shell-maturity \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the docs surface's high-zoom / high-contrast layout diverges
/// the route semantics, proving a zoom route divergence blocks promotion (red) before
/// the row can keep its accessibility-parity claim.
pub fn seeded_m5_responsive_collapse_packet_docs_zoom_route_divergence_blocked(
) -> ResponsiveCollapsePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Docs, |spec| {
        spec.zoom_contrast_parity = ZoomContrastParityState::RouteSemanticsDivergeAtZoom;
        spec.narrowing_reason = Some(
            "At 400% zoom the docs reader exposed a different navigation route than the standard \
             layout, diverging the route semantics, so the row blocks before keeping its \
             accessibility-parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the companion's collapse ladder loses its placeholder
/// terminal, proving a ladder that can dead-end blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_responsive_collapse_packet_companion_ladder_missing_placeholder_blocked(
) -> ResponsiveCollapsePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Companion, |spec| {
        // Drop the terminal placeholder rung so the ladder can dead-end.
        spec.collapse_ladder_override =
            Some(vec![M5FallbackPlacement::Sheet, M5FallbackPlacement::Overflow]);
        spec.narrowing_reason = Some(
            "The companion surface's collapse ladder was trimmed to sheet→overflow with no \
             identity-preserving placeholder terminal, so a lost dependency could dead-end the \
             surface; the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}
