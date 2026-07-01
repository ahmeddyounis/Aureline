//! Canonical seed builders for the M5 desktop-profile certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export,
//! and CSV artifacts plus the narrowed fixtures. The headless emitter and the inline tests both
//! call them so the in-code certification proof, the artifacts, and the fixtures never drift.
//! The claimed surface families each profile evaluates are pulled straight from the frozen
//! shell-zone matrix's seeded packet, so the certification cannot audit a family the matrix does
//! not freeze, and the evaluated-family set is derived from the matrix rather than restated by
//! hand.

use super::*;
use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    seeded_m5_shell_zone_matrix, M5_SHELL_ZONE_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Owner role accountable for every certified desktop profile.
const PROFILE_OWNER_ROLE: &str = "M5 shell / adaptive continuity owner";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so the
/// checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The desktop-profile certification posture seeded for one profile.
struct ProfileSpec {
    /// When set, the evaluated-family set used instead of the canonical full set (blocked
    /// fixtures use this to prove a partial evaluation blocks).
    evaluated_families_override: Option<Vec<M5ShellSurfaceFamily>>,
    shell_zone_integrity: ShellZoneIntegrityState,
    adaptive_layout: AdaptiveLayoutState,
    multi_window_truth: MultiWindowTruthState,
    owning_window_routing: OwningWindowRoutingState,
    waiver: Option<DesktopProfileWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ProfileSpec {
    /// A full-continuity posture: shell-zone integrity, adaptive-layout identity, multi-window
    /// truth, and owning-window routing all hold across every claimed surface.
    fn stable() -> Self {
        Self {
            evaluated_families_override: None,
            shell_zone_integrity: ShellZoneIntegrityState::AllSurfacesInDeclaredSlots,
            adaptive_layout: AdaptiveLayoutState::IdentityStableNoUnusablePane,
            multi_window_truth: MultiWindowTruthState::AllTruthsPreservedLayoutLocal,
            owning_window_routing: OwningWindowRoutingState::RoutesToOwningObjectNoFocusTheft,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// The claimed surface families evaluated by every profile row, pulled from the frozen matrix.
fn evaluated_families() -> Vec<M5ShellSurfaceFamily> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| matrix_row.family)
        .collect()
}

/// The profile-relevant downgrade triggers recorded on every row.
fn profile_downgrade_triggers() -> Vec<M5ShellDowngradeTrigger> {
    vec![
        M5ShellDowngradeTrigger::SlotUndeclared,
        M5ShellDowngradeTrigger::CollapseChangedTaskIdentity,
        M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
        M5ShellDowngradeTrigger::WorkspaceTruthDivergedAcrossWindows,
        M5ShellDowngradeTrigger::OwningWindowRoutingLost,
        M5ShellDowngradeTrigger::SecondaryDisplayTopologyDrift,
        M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
        M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

/// Short scenario summary for one profile.
fn scenario_summary(profile: M5DesktopProfile) -> &'static str {
    match profile {
        M5DesktopProfile::CompactDesktop => {
            "Narrow width, high zoom, or a secondary compact display where the responsive collapse \
             ladder is under maximum pressure."
        }
        M5DesktopProfile::StandardDesktop => {
            "Default working width on the primary display — the reference layout every surface \
             docks into."
        }
        M5DesktopProfile::ExpandedDesktop => {
            "Wide primary display where surfaces expand side-by-side without changing task \
             identity."
        }
        M5DesktopProfile::MixedDpi => {
            "Windows spanning displays with different scale factors, where crisp rendering must \
             not drop identity or routing."
        }
        M5DesktopProfile::MultiMonitor => {
            "Secondary displays and window/display topology change, where detach, monitor loss, \
             and routing must recenter without orphaning."
        }
        M5DesktopProfile::DependencyMissingRestore => {
            "Crash or restart restore where an extension, remote target, or feature pack is \
             unavailable and surfaces must degrade to a safe declared arrangement."
        }
    }
}

/// Builds one certification row from a profile and a certification posture.
fn row_from_profile(profile: M5DesktopProfile, spec: ProfileSpec) -> DesktopProfileRow {
    let families = spec
        .evaluated_families_override
        .unwrap_or_else(evaluated_families);
    let mut row = DesktopProfileRow {
        profile,
        profile_label: profile.label().to_owned(),
        owner_role: PROFILE_OWNER_ROLE.to_owned(),
        scenario_summary: scenario_summary(profile).to_owned(),
        evaluated_families: families,
        shell_zone_integrity: spec.shell_zone_integrity,
        adaptive_layout: spec.adaptive_layout,
        multi_window_truth: spec.multi_window_truth,
        owning_window_routing: spec.owning_window_routing,
        applicable_downgrade_triggers: profile_downgrade_triggers(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: DesktopProfileStatus::Green,
        profile_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.profile_causes = row.recompute_causes();
    row
}

/// Builds the multi-monitor routing-relocation waiver carried by the seed.
fn multi_monitor_routing_relocation_waiver() -> DesktopProfileWaiver {
    DesktopProfileWaiver {
        waiver_id: "waiver:multi-monitor-routing-relocation:0001".to_owned(),
        profile: M5DesktopProfile::MultiMonitor,
        reason: "When a secondary-monitor window is closed or its display is removed while a \
                 routed approval is in flight, the approval is relocated to a disclosed, \
                 still-visible prompt in the primary workspace window rather than being orphaned; \
                 the relocation is disclosed, never silent, and the shared routing contract is \
                 unified in the next attention-routing sync."
            .to_owned(),
        owner_role: "Attention-routing surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded desktop-profile certification posture for one profile.
fn profile_spec(profile: M5DesktopProfile) -> ProfileSpec {
    match profile {
        M5DesktopProfile::CompactDesktop => ProfileSpec {
            // Under compact width the responsive collapse ladder takes a disclosed
            // docked→sheet/overflow narrowing while preserving task identity and reopen path.
            adaptive_layout: AdaptiveLayoutState::DisclosedCollapseNarrowing,
            narrowing_reason: Some(
                "Under compact width and high zoom, several surfaces take a disclosed \
                 docked→sheet/overflow collapse narrowing that preserves the task identity, keeps \
                 critical state reachable through a keyboard overflow, and preserves the reopen \
                 path, so the profile is narrowed below green while identity stays stable.",
            ),
            ..ProfileSpec::stable()
        },
        M5DesktopProfile::MultiMonitor => ProfileSpec {
            // Multi-monitor topology change defers a routed approval to a disclosed, waivered
            // relocation into the primary window when a secondary display is removed.
            owning_window_routing: OwningWindowRoutingState::DisclosedRoutingRelocation,
            waiver: Some(multi_monitor_routing_relocation_waiver()),
            narrowing_reason: Some(
                "When a secondary-monitor window is closed or its display is removed while a routed \
                 approval is in flight, the approval is deferred to a disclosed, waivered \
                 relocation into the primary workspace window with a still-visible prompt rather \
                 than being orphaned, so the multi-monitor profile is narrowed and disclosed.",
            ),
            ..ProfileSpec::stable()
        },
        M5DesktopProfile::DependencyMissingRestore => ProfileSpec {
            // On dependency-missing restore a surface falls back to its declared fallback slot
            // (still a declared shell slot), disclosed, until the dependency is restored.
            shell_zone_integrity: ShellZoneIntegrityState::DisclosedSlotFallbackNarrowing,
            multi_window_truth: MultiWindowTruthState::DisclosedTruthProjectionNarrowing,
            narrowing_reason: Some(
                "On a crash/restart restore where an extension, remote target, or feature pack is \
                 unavailable, affected surfaces fall back to their declared fallback shell slot \
                 and a workspace truth is projected in a disclosed reduced form until the \
                 dependency is restored; every fallback stays a declared slot and every truth \
                 stays visible in every window, so the profile is narrowed and disclosed.",
            ),
            ..ProfileSpec::stable()
        },
        // Standard, expanded, and mixed-DPI hold full continuity across every claimed surface.
        _ => ProfileSpec::stable(),
    }
}

/// Builds the certification rows for the canonical seed, one per claimed desktop profile.
fn seeded_rows() -> Vec<DesktopProfileRow> {
    M5DesktopProfile::ALL
        .iter()
        .map(|&profile| row_from_profile(profile, profile_spec(profile)))
        .collect()
}

/// Builds a variant where one profile's spec is mutated after the canonical spec is resolved,
/// used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5DesktopProfile, mutate: F) -> Vec<DesktopProfileRow>
where
    F: Fn(&mut ProfileSpec),
{
    M5DesktopProfile::ALL
        .iter()
        .map(|&profile| {
            let mut spec = profile_spec(profile);
            if profile == target {
                mutate(&mut spec);
            }
            row_from_profile(profile, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<DesktopProfileRow>) -> DesktopProfilePacket {
    build_m5_desktop_profile_certification_packet(DesktopProfileInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 desktop-profile certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Three profiles keep full continuity (green): standard, expanded, and mixed-DPI.
/// The compact profile auto-narrows to yellow disclosing a docked→sheet/overflow collapse
/// narrowing, the multi-monitor profile auto-narrows to yellow with a waivered routing
/// relocation, and the dependency-missing-restore profile auto-narrows to yellow disclosing a
/// declared slot fallback and a reduced truth projection — and no row is blocked, so the packet
/// is clean and every row is publishable.
pub fn seeded_m5_desktop_profile_certification_packet() -> DesktopProfilePacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where a claimed surface invents a private shell slot under the compact
/// profile, proving slot drift blocks promotion (red) rather than staying a disclosed yellow.
pub fn seeded_m5_desktop_profile_certification_packet_compact_private_slot_drift_blocked(
) -> DesktopProfilePacket {
    let rows = seeded_rows_with(M5DesktopProfile::CompactDesktop, |spec| {
        spec.shell_zone_integrity = ShellZoneIntegrityState::PrivateSlotDriftDetected;
        spec.adaptive_layout = AdaptiveLayoutState::IdentityStableNoUnusablePane;
        spec.narrowing_reason = Some(
            "Under compact width a claimed surface attached outside any declared shell slot, \
             inventing a private slot instead of collapsing through its declared ladder, so the \
             profile blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where responsive collapse forces an unusable narrow pane under the compact
/// profile, proving an unusable pane blocks promotion (red) rather than staying a disclosed
/// yellow.
pub fn seeded_m5_desktop_profile_certification_packet_compact_unusable_pane_blocked(
) -> DesktopProfilePacket {
    let rows = seeded_rows_with(M5DesktopProfile::CompactDesktop, |spec| {
        spec.adaptive_layout = AdaptiveLayoutState::IdentityLostOrUnusablePane;
        spec.narrowing_reason = Some(
            "Under compact width responsive collapse forced an unusable narrow editor pane and \
             changed the task identity instead of overflowing critical state, so the profile \
             blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where workspace-global truth diverges across windows under the multi-monitor
/// profile, proving a truth divergence blocks promotion (red) rather than staying a disclosed
/// yellow.
pub fn seeded_m5_desktop_profile_certification_packet_multi_monitor_truth_diverged_blocked(
) -> DesktopProfilePacket {
    let rows = seeded_rows_with(M5DesktopProfile::MultiMonitor, |spec| {
        spec.multi_window_truth = MultiWindowTruthState::WorkspaceTruthDivergedAcrossWindows;
        spec.owning_window_routing = OwningWindowRoutingState::RoutesToOwningObjectNoFocusTheft;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "On a secondary display the detached window showed a stale remote target and trust \
             class while the primary window advanced, so workspace-global truth diverged across \
             windows and the profile blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where a routed action is lost to a wrong-window reopen under the
/// dependency-missing-restore profile, proving lost routing blocks promotion (red) before the
/// row can keep its routing claim.
pub fn seeded_m5_desktop_profile_certification_packet_dependency_restore_routing_lost_blocked(
) -> DesktopProfilePacket {
    let rows = seeded_rows_with(M5DesktopProfile::DependencyMissingRestore, |spec| {
        spec.owning_window_routing = OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan;
        spec.narrowing_reason = Some(
            "After a restore with a missing extension, a routed approval reopened on a generic \
             shell window rather than the owning object, losing the routed action to a \
             wrong-window reopen, so the profile blocks before keeping its routing claim.",
        );
    });
    packet_from_rows(rows)
}
