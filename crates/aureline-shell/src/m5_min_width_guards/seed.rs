//! Canonical seed builders for the M5 min-width-guard proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed fixtures. The headless emitter
//! and the inline tests both call them so the in-code guard proof, the artifacts, and
//! the fixtures never drift. The certified rows are pulled straight from the frozen
//! shell-zone matrix's seeded packet, so the guard proof cannot certify a family the
//! matrix does not freeze, and each family's declared safe-fallback set is derived from
//! the matrix's occupant transitions rather than restated by hand.

use super::*;
use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    seeded_m5_shell_zone_matrix, M5ShellSurfaceRow, M5_SHELL_ZONE_MATRIX_PACKET_ID,
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

/// The min-width-guard posture seeded for one governed family.
struct GuardSpec {
    /// When set, the declared strategy set used instead of the occupancy-derived set
    /// (blocked fixtures use this to prove a set without a safe terminal blocks).
    strategy_set_override: Option<Vec<M5CompareFallbackStrategy>>,
    /// When set, the primary fallback strategy used instead of the compact plan's.
    compare_fallback_strategy_override: Option<M5CompareFallbackStrategy>,
    min_useful_width_px: u32,
    min_useful_height_px: u32,
    min_size_enforcement: MinSizeEnforcementState,
    compare_fallback: CompareFallbackState,
    status_continuity: StatusContinuityState,
    waiver: Option<MinWidthGuardWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl GuardSpec {
    /// A fully-guarded posture: min size enforced, safe fallback, status preserved.
    fn stable() -> Self {
        Self {
            strategy_set_override: None,
            compare_fallback_strategy_override: None,
            min_useful_width_px: 560,
            min_useful_height_px: 360,
            min_size_enforcement: MinSizeEnforcementState::MinUsefulSizeEnforced,
            compare_fallback: CompareFallbackState::SafeFallbackBeforeUnusableSplit,
            status_continuity: StatusContinuityState::IdentityBreadcrumbsStatusPreserved,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the companion recovery-status relocation waiver carried by the seed.
fn companion_status_relocation_waiver() -> MinWidthGuardWaiver {
    MinWidthGuardWaiver {
        waiver_id: "waiver:companion-status-relocation:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "Under compact width the companion assistant cannot host a side-by-side compare and \
                 falls back to sequential disclosure; its recovery-critical connection status \
                 relocates from the inline header to a disclosed, still-visible status affordance \
                 while the shared status-strip contract is unified in the next sync. Breadcrumbs and \
                 the active object identity stay visible and the relocation is disclosed, never \
                 silent."
            .to_owned(),
        owner_role: "Companion surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Derives the declared safe compare-fallback strategies for a family from its matrix
/// occupant transitions.
///
/// A family that may split side-by-side declares [`M5CompareFallbackStrategy::SideBySideSplit`];
/// one that may tab declares [`M5CompareFallbackStrategy::TabbedCompare`]; one that may
/// sheet declares [`M5CompareFallbackStrategy::StagedPeek`]. Every family additionally
/// declares [`M5CompareFallbackStrategy::SequentialDisclosure`] and
/// [`M5CompareFallbackStrategy::ExplicitUserChoice`] as the universal safe terminals a
/// surface can always fall back to no matter how narrow it becomes, so the set always
/// terminates in a universally-available safe mode. The set is ordered
/// widest-to-narrowest by construction.
fn declared_strategies(occupant: &[M5OccupantPersistence]) -> Vec<M5CompareFallbackStrategy> {
    let mut strategies = Vec::new();
    if occupant.contains(&M5OccupantPersistence::SideBySide) {
        strategies.push(M5CompareFallbackStrategy::SideBySideSplit);
    }
    if occupant.contains(&M5OccupantPersistence::Tabbed) {
        strategies.push(M5CompareFallbackStrategy::TabbedCompare);
    }
    if occupant.contains(&M5OccupantPersistence::Sheeted) {
        strategies.push(M5CompareFallbackStrategy::StagedPeek);
    }
    // Universal safe terminals, always available regardless of dock capability.
    strategies.push(M5CompareFallbackStrategy::SequentialDisclosure);
    strategies.push(M5CompareFallbackStrategy::ExplicitUserChoice);
    strategies
}

/// Builds the per-class compare plans for a family from its declared strategy set.
///
/// At expanded and standard widths the surface uses the widest declared strategy; under
/// compact width it steps one rung down the set (side-by-side→tabbed, tabbed→staged,
/// etc.), so a narrower class always uses a strategy that needs at most as much width
/// and the plans stay monotonic across the three classes.
fn class_plans(strategies: &[M5CompareFallbackStrategy]) -> Vec<MinWidthClassPlan> {
    let widest = strategies
        .first()
        .copied()
        .unwrap_or(M5CompareFallbackStrategy::ExplicitUserChoice);
    let compact = strategies.get(1).copied().unwrap_or(widest);
    M5ResponsiveClass::ALL
        .iter()
        .map(|&responsive_class| {
            let strategy = match responsive_class {
                M5ResponsiveClass::CompactDesktop => compact,
                M5ResponsiveClass::StandardDesktop | M5ResponsiveClass::ExpandedDesktop => widest,
            };
            MinWidthClassPlan {
                responsive_class,
                strategy,
                meets_min_useful_size: true,
                identity_and_status_preserved: true,
            }
        })
        .collect()
}

/// Builds one guard row from a frozen matrix row and a guard posture.
fn row_from_matrix(matrix_row: &M5ShellSurfaceRow, spec: GuardSpec) -> MinWidthGuardRow {
    let declared = spec
        .strategy_set_override
        .unwrap_or_else(|| declared_strategies(&matrix_row.occupant_persistence));
    let class_plans = class_plans(&declared);
    // The primary fallback is the strategy the surface lands in under compact width.
    let compare_fallback_strategy = spec
        .compare_fallback_strategy_override
        .or_else(|| declared.get(1).copied())
        .unwrap_or(M5CompareFallbackStrategy::ExplicitUserChoice);
    let mut row = MinWidthGuardRow {
        family: matrix_row.family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        surface_label: surface_label(matrix_row.family).to_owned(),
        canonical_slot: matrix_row.canonical_slot,
        fallback_slot: matrix_row.fallback_slot,
        declared_responsive_classes: matrix_row.responsive_classes.clone(),
        declared_occupant_persistence: matrix_row.occupant_persistence.clone(),
        declared_strategies: declared,
        compare_fallback_strategy,
        min_useful_width_px: spec.min_useful_width_px,
        min_useful_height_px: spec.min_useful_height_px,
        class_plans,
        min_size_enforcement: spec.min_size_enforcement,
        compare_fallback: spec.compare_fallback,
        status_continuity: spec.status_continuity,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: MinWidthGuardStatus::Green,
        guard_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.guard_causes = row.recompute_causes();
    row
}

/// Returns the seeded min-width-guard posture for one governed family.
fn guard_spec(family: M5ShellSurfaceFamily) -> GuardSpec {
    match family {
        M5ShellSurfaceFamily::Profiler => GuardSpec {
            // A Stable family narrowed only by min-size behavior: under compact width
            // the profiler discloses a reduced-but-still-usable minimum for its capture
            // readout while staying above the absolute floor.
            min_useful_width_px: 360,
            min_useful_height_px: 240,
            min_size_enforcement: MinSizeEnforcementState::DisclosedReducedMinimum,
            narrowing_reason: Some(
                "Under compact width the profiler discloses a reduced-but-still-usable minimum \
                 useful size for its capture readout, staying above the absolute floor; the row is \
                 narrowed below green while the pane stays usable.",
            ),
            ..GuardSpec::stable()
        },
        M5ShellSurfaceFamily::Incident => GuardSpec {
            min_useful_width_px: 520,
            min_useful_height_px: 340,
            narrowing_reason: Some(
                "The incident surface is qualified at Beta in the frozen shell-zone matrix; its \
                 min-size enforcement and compare fallback are fully guarded but the claim is \
                 narrowed below Stable and disclosed.",
            ),
            ..GuardSpec::stable()
        },
        M5ShellSurfaceFamily::Companion => GuardSpec {
            // Beta-qualified and, under compact width, relocates recovery-critical status
            // to a disclosed, waivered still-visible affordance.
            min_useful_width_px: 520,
            min_useful_height_px: 340,
            status_continuity: StatusContinuityState::DisclosedStatusRelocation,
            waiver: Some(companion_status_relocation_waiver()),
            narrowing_reason: Some(
                "The companion surface is qualified at Beta; under compact width it cannot host a \
                 side-by-side compare and relocates its recovery-critical connection status to a \
                 disclosed, waivered still-visible affordance while preserving breadcrumbs and the \
                 active object identity.",
            ),
            ..GuardSpec::stable()
        },
        M5ShellSurfaceFamily::Operator => GuardSpec {
            // Beta-qualified and discloses a narrowed compare fallback while keeping a
            // safe mode before any unusable split.
            min_useful_width_px: 520,
            min_useful_height_px: 340,
            compare_fallback: CompareFallbackState::DisclosedFallbackNarrowing,
            narrowing_reason: Some(
                "The operator surface is qualified at Beta; under compact width its compare fallback \
                 trims a secondary control panel's optional detail in a disclosed way before an \
                 unusable split could occur, so the claim is narrowed and disclosed.",
            ),
            ..GuardSpec::stable()
        },
        _ => GuardSpec::stable(),
    }
}

/// Builds the guard rows for the canonical seed, one per matrix family.
fn seeded_rows() -> Vec<MinWidthGuardRow> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, guard_spec(matrix_row.family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellSurfaceFamily, mutate: F) -> Vec<MinWidthGuardRow>
where
    F: Fn(&mut GuardSpec),
{
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = guard_spec(matrix_row.family);
            if matrix_row.family == target {
                mutate(&mut spec);
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<MinWidthGuardRow>) -> MinWidthGuardPacket {
    build_m5_min_width_guards_packet(MinWidthGuardInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 min-width-guard packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and
/// CSV artifacts. Six families are fully guarded across every class (green); the
/// profiler auto-narrows to yellow disclosing a reduced-but-usable minimum, the incident
/// surface auto-narrows to yellow on its Beta qualification, the companion surface
/// auto-narrows to yellow with a waivered recovery-status relocation, and the operator
/// surface auto-narrows to yellow disclosing a narrowed compare fallback — and no row is
/// blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_min_width_guards_packet() -> MinWidthGuardPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook editor can be forced below a usable minimum,
/// proving an unusable narrow pane blocks promotion (red) rather than passing on
/// behavior alone.
pub fn seeded_m5_min_width_guards_packet_notebook_pane_below_minimum_blocked(
) -> MinWidthGuardPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Notebook, |spec| {
        spec.min_size_enforcement = MinSizeEnforcementState::PaneForcedBelowUsableMinimum;
        spec.min_useful_width_px = 180;
        spec.min_useful_height_px = 120;
        spec.narrowing_reason = Some(
            "The notebook editor could be dragged or split below a usable minimum, producing a \
             silent unusable narrow pane, so the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the preview surface produces a silent unusable narrow split,
/// proving a silent split with no safe fallback blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_min_width_guards_packet_preview_silent_unusable_split_blocked(
) -> MinWidthGuardPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Preview, |spec| {
        spec.compare_fallback = CompareFallbackState::SilentUnusableSplit;
        spec.narrowing_reason = Some(
            "The preview surface produced a silent unusable narrow diff split under compact width \
             instead of falling back to a staged peek, so the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data grid loses recovery-critical status under fallback,
/// proving lost breadcrumbs/identity/status blocks promotion (red) before the row can
/// keep its recovery-truth claim.
pub fn seeded_m5_min_width_guards_packet_datagrid_status_lost_blocked() -> MinWidthGuardPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::DataGrid, |spec| {
        spec.status_continuity = StatusContinuityState::StatusOrIdentityLostUnderFallback;
        spec.narrowing_reason = Some(
            "The data grid dropped its recovery-critical reconnect status while its compare \
             fallback was active, losing recovery truth, so the row blocks before keeping its \
             recovery-truth claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the companion's declared safe-fallback set loses its universal
/// safe terminal, proving a fallback set that can dead-end at a too-wide strategy blocks
/// promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_min_width_guards_packet_companion_strategy_set_no_terminal_blocked(
) -> MinWidthGuardPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Companion, |spec| {
        // Drop the universal safe terminals so the set dead-ends at staged peek, which
        // still needs more width than a usable minimum can guarantee.
        spec.strategy_set_override = Some(vec![
            M5CompareFallbackStrategy::TabbedCompare,
            M5CompareFallbackStrategy::StagedPeek,
        ]);
        spec.compare_fallback_strategy_override = Some(M5CompareFallbackStrategy::StagedPeek);
        spec.narrowing_reason = Some(
            "The companion surface's declared safe-fallback set was trimmed to tabbed-compare then \
             staged-peek with no universally-available safe terminal, so under extreme narrowing it \
             could dead-end at a strategy that needs more width; the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}
