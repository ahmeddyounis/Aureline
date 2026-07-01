//! Canonical seed builders for the M5 shell-zone occupancy proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed fixtures. The headless
//! emitter and the inline tests both call them so the in-code occupancy proof, the
//! artifacts, and the fixtures never drift. The certified rows are pulled straight
//! from the frozen shell-zone matrix's seeded packet, so the occupancy proof cannot
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

/// The occupancy posture seeded for one governed family.
struct OccupancySpec {
    /// When set, the slot the occupant docks into instead of the canonical slot.
    occupied_slot_override: Option<M5ShellZoneSlot>,
    slot_attachment: SlotAttachmentState,
    occupant_availability: OccupantAvailabilityState,
    route_resolution: RouteResolutionState,
    resolved_route_channels: Vec<RouteChannel>,
    waiver: Option<ShellOccupancyWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl OccupancySpec {
    /// A fully occupied posture: declared canonical slot, available occupant, every
    /// route resolving to that slot and occupant.
    fn occupied() -> Self {
        Self {
            occupied_slot_override: None,
            slot_attachment: SlotAttachmentState::AttachedToDeclaredSlot,
            occupant_availability: OccupantAvailabilityState::OccupantAvailable,
            route_resolution: RouteResolutionState::AllRoutesResolveToSlotOccupant,
            resolved_route_channels: RouteChannel::ALL.to_vec(),
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the companion onboarding-route-sync waiver carried by the seed.
fn companion_onboarding_route_waiver() -> ShellOccupancyWaiver {
    ShellOccupancyWaiver {
        waiver_id: "waiver:companion-onboarding-route-sync:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "The companion onboarding route temporarily resolves to the right-inspector sheet \
                 rather than the companion overlay while the onboarding route registry is unified \
                 in the next sync. The fallback is disclosed, never hidden, and the \
                 command/keyboard/docs routes already resolve to the declared slot and occupant."
            .to_owned(),
        owner_role: "Companion surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded occupancy posture for one governed family.
fn occupancy_spec(family: M5ShellSurfaceFamily) -> OccupancySpec {
    match family {
        M5ShellSurfaceFamily::DataGrid => OccupancySpec {
            occupant_availability: OccupantAvailabilityState::DependencyMissingPlaceholder,
            narrowing_reason: Some(
                "The data grid's remote source is unavailable on this build, so the surface \
                 degrades to a disclosed reconnect placeholder card that keeps its main-workspace \
                 slot occupied; the row is narrowed below green while the placeholder is shown.",
            ),
            ..OccupancySpec::occupied()
        },
        M5ShellSurfaceFamily::Incident => OccupancySpec {
            // Under the seeded compact width the incident surface occupies its
            // declared right-inspector fallback slot rather than the main workspace.
            occupied_slot_override: Some(M5ShellZoneSlot::RightInspector),
            narrowing_reason: Some(
                "The incident surface is qualified at Beta in the frozen shell-zone matrix and, \
                 under the seeded compact width, occupies its declared right-inspector fallback \
                 slot; the claim is narrowed below Stable and disclosed.",
            ),
            ..OccupancySpec::occupied()
        },
        M5ShellSurfaceFamily::Companion => OccupancySpec {
            route_resolution: RouteResolutionState::DisclosedRouteFallback,
            // Command, keyboard, and docs routes resolve to the declared slot; the
            // onboarding route falls back to a disclosed, waivered alternative.
            resolved_route_channels: vec![
                RouteChannel::Command,
                RouteChannel::Keyboard,
                RouteChannel::Docs,
            ],
            waiver: Some(companion_onboarding_route_waiver()),
            narrowing_reason: Some(
                "The companion surface is qualified at Beta; its onboarding route resolves to a \
                 disclosed, waivered fallback slot pending the next route-registry sync, while the \
                 command/keyboard/docs routes resolve to its declared right-inspector slot.",
            ),
            ..OccupancySpec::occupied()
        },
        M5ShellSurfaceFamily::Operator => OccupancySpec {
            occupant_availability: OccupantAvailabilityState::PolicyBlockedPlaceholder,
            narrowing_reason: Some(
                "The operator surface is qualified at Beta and, when the control-plane is \
                 policy-blocked, degrades to a disclosed policy-blocked placeholder card that keeps \
                 its bottom-panel slot occupied; the claim is narrowed and disclosed.",
            ),
            ..OccupancySpec::occupied()
        },
        _ => OccupancySpec::occupied(),
    }
}

/// The registered slot set for a family: its canonical slot plus its declared
/// fallback slot, de-duplicated in declaration order.
fn registered_slots(matrix_row: &M5ShellSurfaceRow) -> Vec<M5ShellZoneSlot> {
    let mut slots = vec![matrix_row.canonical_slot];
    if matrix_row.fallback_slot != matrix_row.canonical_slot {
        slots.push(matrix_row.fallback_slot);
    }
    slots
}

/// Builds one occupancy row from a frozen matrix row and an occupancy posture.
fn row_from_matrix(matrix_row: &M5ShellSurfaceRow, spec: OccupancySpec) -> ShellOccupancyRow {
    let occupied_slot = spec
        .occupied_slot_override
        .unwrap_or(matrix_row.canonical_slot);
    let mut row = ShellOccupancyRow {
        family: matrix_row.family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        occupant_surface: occupant_label(matrix_row.family).to_owned(),
        canonical_slot: matrix_row.canonical_slot,
        fallback_slot: matrix_row.fallback_slot,
        registered_slots: registered_slots(matrix_row),
        occupied_slot,
        placeholder_behavior: matrix_row.placeholder_behavior,
        slot_attachment: spec.slot_attachment,
        occupant_availability: spec.occupant_availability,
        route_resolution: spec.route_resolution,
        resolved_route_channels: spec.resolved_route_channels,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: ShellOccupancyStatus::Green,
        occupancy_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.occupancy_causes = row.recompute_causes();
    row
}

/// Builds the occupancy rows for the canonical seed, one per matrix family.
fn seeded_rows() -> Vec<ShellOccupancyRow> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, occupancy_spec(matrix_row.family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellSurfaceFamily, mutate: F) -> Vec<ShellOccupancyRow>
where
    F: Fn(&mut OccupancySpec),
{
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = occupancy_spec(matrix_row.family);
            if matrix_row.family == target {
                mutate(&mut spec);
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<ShellOccupancyRow>) -> ShellOccupancyPacket {
    build_m5_shell_occupancy_packet(ShellOccupancyInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 shell-zone occupancy packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export,
/// and CSV artifacts. Six families occupy their declared slot at full standing
/// (green); the data grid auto-narrows to yellow while its remote source shows a
/// reconnect placeholder, the incident surface auto-narrows to yellow occupying its
/// declared right-inspector fallback slot, the companion surface auto-narrows to
/// yellow with a waivered onboarding-route fallback, and the operator surface
/// auto-narrows to yellow behind a policy-blocked placeholder — and no row is
/// blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_shell_occupancy_packet() -> ShellOccupancyPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook surface attaches outside any declared slot,
/// proving a private chrome island blocks promotion (red) rather than passing on
/// behavior alone.
pub fn seeded_m5_shell_occupancy_packet_notebook_undeclared_blocked() -> ShellOccupancyPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Notebook, |spec| {
        spec.slot_attachment = SlotAttachmentState::UndeclaredSlotAttachment;
        spec.occupied_slot_override = Some(M5ShellZoneSlot::StatusBar);
        spec.narrowing_reason = Some(
            "The notebook surface attached to a private chrome island in the status bar rather \
             than a declared shell slot, so the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data grid's placeholder collapses the surrounding
/// layout, proving a placeholder that loses spatial continuity blocks promotion
/// (red) rather than staying a disclosed yellow.
pub fn seeded_m5_shell_occupancy_packet_data_grid_placeholder_collapsed_blocked(
) -> ShellOccupancyPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::DataGrid, |spec| {
        spec.occupant_availability = OccupantAvailabilityState::PlaceholderCollapsedLayout;
        spec.narrowing_reason = Some(
            "The data grid's placeholder collapsed the surrounding layout instead of holding its \
             slot, so the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the review surface's approval route resolves to a
/// different slot/occupant, proving a conflicting route blocks promotion (red)
/// before the row can keep its routing claim.
pub fn seeded_m5_shell_occupancy_packet_review_route_conflict_blocked() -> ShellOccupancyPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Review, |spec| {
        spec.route_resolution = RouteResolutionState::ConflictingRouteResolution;
        spec.narrowing_reason = Some(
            "The review surface's approval route resolves to a different slot and occupant than \
             the declared owner, so the row blocks before keeping its owning-window routing claim.",
        );
    });
    packet_from_rows(rows)
}
