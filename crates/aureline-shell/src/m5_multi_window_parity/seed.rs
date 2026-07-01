//! Canonical seed builders for the M5 multi-window truth-parity proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed fixtures. The headless emitter
//! and the inline tests both call them so the in-code parity proof, the artifacts, and
//! the fixtures never drift. The certified rows are pulled straight from the frozen
//! shell-zone matrix's seeded packet, so the parity proof cannot certify a family the
//! matrix does not freeze, and each family's declared window classes, continuity truths,
//! and owning-window routing expectations are derived from the matrix rather than
//! restated by hand.

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

/// The multi-window-parity posture seeded for one governed family.
struct ParitySpec {
    /// When set, the declared continuity-truth set used instead of the matrix's set
    /// (blocked fixtures use this to prove a set missing a required truth blocks).
    continuity_truths_override: Option<Vec<M5ContinuityTruth>>,
    continuity_parity: ContinuityParityState,
    layout_locality: LayoutLocalityState,
    owning_window_routing: OwningWindowRoutingState,
    recovery_drill: RecoveryDrillState,
    waiver: Option<MultiWindowParityWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ParitySpec {
    /// A full-parity posture: same truth in every window, layout local, routed,
    /// predictable recovery.
    fn stable() -> Self {
        Self {
            continuity_truths_override: None,
            continuity_parity: ContinuityParityState::AllTruthsPreservedInEveryWindow,
            layout_locality: LayoutLocalityState::LayoutDensityFocusLocalRiskGlobal,
            owning_window_routing: OwningWindowRoutingState::RoutesToOwningWindowObject,
            recovery_drill: RecoveryDrillState::RestoreDependencyTopologyPredictable,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the companion routing-relocation waiver carried by the seed.
fn companion_routing_relocation_waiver() -> MultiWindowParityWaiver {
    MultiWindowParityWaiver {
        waiver_id: "waiver:companion-routing-relocation:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "When the companion's owning window is not present, an approval prompt relocates \
                 from the owning window to a disclosed, still-visible re-notification affordance in \
                 the primary workspace window rather than stealing focus or orphaning; the \
                 owning-window route is re-established the moment that window returns, and the \
                 relocation is disclosed, never silent, while the shared attention-route contract \
                 is unified in the next sync."
            .to_owned(),
        owner_role: "Companion surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Builds the per-window continuity plans for a family from its declared window classes
/// and continuity truths.
///
/// Every window the family may live in preserves exactly the family's declared
/// workspace-global continuity truths, keeps layout / density / focus local, and routes
/// dialogs, notifications, and approvals back to the owning window and object.
fn window_plans(
    window_classes: &[M5WindowClass],
    truths: &[M5ContinuityTruth],
) -> Vec<WindowContinuityPlan> {
    window_classes
        .iter()
        .map(|&window_class| WindowContinuityPlan {
            window_class,
            preserved_truths: truths.to_vec(),
            layout_is_local: true,
            routes_to_owning_window: true,
        })
        .collect()
}

/// Builds one parity row from a frozen matrix row and a parity posture.
fn row_from_matrix(matrix_row: &M5ShellSurfaceRow, spec: ParitySpec) -> MultiWindowParityRow {
    let declared_truths = spec
        .continuity_truths_override
        .unwrap_or_else(|| matrix_row.continuity_truths.clone());
    let window_plans = window_plans(&matrix_row.window_classes, &declared_truths);
    let mut row = MultiWindowParityRow {
        family: matrix_row.family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        surface_label: surface_label(matrix_row.family).to_owned(),
        canonical_slot: matrix_row.canonical_slot,
        fallback_slot: matrix_row.fallback_slot,
        declared_window_classes: matrix_row.window_classes.clone(),
        declared_continuity_truths: declared_truths,
        declared_owning_window_routing: matrix_row.owning_window_routing.clone(),
        window_plans,
        continuity_parity: spec.continuity_parity,
        layout_locality: spec.layout_locality,
        owning_window_routing: spec.owning_window_routing,
        recovery_drill: spec.recovery_drill,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: MultiWindowParityStatus::Green,
        parity_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.parity_causes = row.recompute_causes();
    row
}

/// Returns the seeded multi-window-parity posture for one governed family.
fn parity_spec(family: M5ShellSurfaceFamily) -> ParitySpec {
    match family {
        M5ShellSurfaceFamily::Docs => ParitySpec {
            // A Stable family narrowed only by layout behavior: a floating reference
            // window discloses a purely-local reading state (its own density and scroll)
            // that never hides workspace-global risk or policy state.
            layout_locality: LayoutLocalityState::DisclosedLocalOnlyState,
            narrowing_reason: Some(
                "A floating docs reference window discloses a purely-local reading state (its own \
                 density and scroll position) while keeping workspace-global trust, remote, \
                 profile, and recovery risk visible; the row is narrowed below green while global \
                 risk stays global.",
            ),
            ..ParitySpec::stable()
        },
        M5ShellSurfaceFamily::Incident => ParitySpec {
            // Beta-qualified and, in the monitor-topology drill, discloses a narrowed but
            // non-destructive recovery (the war-room display recenters).
            recovery_drill: RecoveryDrillState::DisclosedRecoveryNarrowing,
            narrowing_reason: Some(
                "The incident surface is qualified at Beta; in the monitor-topology drill a \
                 detached war-room display recenters onto the primary display and discloses the \
                 narrowed but non-destructive recovery rather than orphaning, so the claim is \
                 narrowed and disclosed.",
            ),
            ..ParitySpec::stable()
        },
        M5ShellSurfaceFamily::Companion => ParitySpec {
            // Beta-qualified and relocates a routed approval to a disclosed, waivered
            // still-visible affordance when its owning window is not present.
            owning_window_routing: OwningWindowRoutingState::DisclosedRoutingRelocation,
            waiver: Some(companion_routing_relocation_waiver()),
            narrowing_reason: Some(
                "The companion surface is qualified at Beta; when its owning window is not present \
                 an approval prompt relocates to a disclosed, waivered still-visible \
                 re-notification affordance in the primary workspace window rather than stealing \
                 focus or orphaning, and re-establishes the owning-window route when that window \
                 returns.",
            ),
            ..ParitySpec::stable()
        },
        M5ShellSurfaceFamily::Operator => ParitySpec {
            // Beta-qualified and discloses a narrowed-but-visible projection of a
            // workspace-global truth in a floating utility window.
            continuity_parity: ContinuityParityState::DisclosedTruthProjectionNarrowing,
            narrowing_reason: Some(
                "The operator surface is qualified at Beta; a floating control utility window shows \
                 a compact, disclosed projection of the active deployment profile and remote host \
                 rather than the full inline identity strip, while keeping all four workspace-global \
                 truths visible, so the claim is narrowed and disclosed.",
            ),
            ..ParitySpec::stable()
        },
        _ => ParitySpec::stable(),
    }
}

/// Builds the parity rows for the canonical seed, one per matrix family.
fn seeded_rows() -> Vec<MultiWindowParityRow> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, parity_spec(matrix_row.family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellSurfaceFamily, mutate: F) -> Vec<MultiWindowParityRow>
where
    F: Fn(&mut ParitySpec),
{
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = parity_spec(matrix_row.family);
            if matrix_row.family == target {
                mutate(&mut spec);
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<MultiWindowParityRow>) -> MultiWindowParityPacket {
    build_m5_multi_window_parity_packet(MultiWindowParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 multi-window-parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and
/// CSV artifacts. Six families preserve full multi-window parity (green); the docs
/// surface auto-narrows to yellow disclosing a purely-local reading state, the incident
/// surface auto-narrows to yellow disclosing a non-destructive monitor-topology recovery,
/// the companion surface auto-narrows to yellow with a waivered routing relocation, and
/// the operator surface auto-narrows to yellow disclosing a narrowed truth projection —
/// and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_multi_window_parity_packet() -> MultiWindowParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook's workspace-global truth diverges across windows,
/// proving a diverged truth blocks promotion (red) rather than passing on behavior alone.
pub fn seeded_m5_multi_window_parity_packet_notebook_truth_diverged_blocked(
) -> MultiWindowParityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Notebook, |spec| {
        spec.continuity_parity = ContinuityParityState::WorkspaceTruthDivergedAcrossWindows;
        spec.narrowing_reason = Some(
            "A detached notebook window showed a stale workspace-global trust class while the \
             primary window showed the current one, diverging workspace-global truth across \
             windows, so the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the preview surface loses a routed action to focus theft or
/// orphaning, proving lost owning-window routing blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_multi_window_parity_packet_preview_routing_lost_blocked() -> MultiWindowParityPacket
{
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Preview, |spec| {
        spec.owning_window_routing = OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan;
        spec.narrowing_reason = Some(
            "A preview approval dialog stole focus from an unrelated window and was orphaned when \
             its owning editor detached, losing owning-window routing, so the row blocks before \
             keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data grid's recovery drill is destructive or orphaning,
/// proving a destructive crash-restore / dependency-loss / monitor-topology drill blocks
/// promotion (red) before the row can keep its recovery-truth claim.
pub fn seeded_m5_multi_window_parity_packet_datagrid_recovery_destructive_blocked(
) -> MultiWindowParityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::DataGrid, |spec| {
        spec.recovery_drill = RecoveryDrillState::RestoreDestructiveOrOrphaned;
        spec.narrowing_reason = Some(
            "The crash-restore drill dropped a detached data-grid window's unsaved filter state \
             and orphaned it off a disconnected display instead of recentering it, so the row \
             blocks before keeping its recovery-truth claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the companion's declared continuity-truth set drops the
/// recovery-state truth, proving a row that fails to declare all four workspace-global
/// truths blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_multi_window_parity_packet_companion_required_truth_missing_blocked(
) -> MultiWindowParityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Companion, |spec| {
        // Drop the recovery-state truth so a companion overlay window could omit
        // recovery-critical status entirely.
        spec.continuity_truths_override = Some(vec![
            M5ContinuityTruth::WorkspaceGlobalTrust,
            M5ContinuityTruth::RemoteTarget,
            M5ContinuityTruth::DeploymentProfile,
        ]);
        spec.narrowing_reason = Some(
            "The companion surface's declared continuity-truth set was trimmed to trust, remote, \
             and profile with recovery-state truth dropped, so a companion overlay window could \
             omit recovery-critical status entirely; the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}
