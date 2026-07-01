//! Canonical seed builders for the M5 owning-window routing proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed fixtures. The headless emitter
//! and the inline tests both call them so the in-code routing proof, the artifacts, and
//! the fixtures never drift. The certified rows are pulled straight from the frozen
//! shell-zone matrix's seeded packet, so the routing proof cannot certify a family the
//! matrix does not freeze, and each family's declared window classes and owning-window
//! routing expectations are derived from the matrix rather than restated by hand.

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

/// The owning-window routing posture seeded for one governed family.
struct RoutingSpec {
    /// When set, the declared owning-window-routing set used instead of the matrix's set
    /// (blocked fixtures use this to prove a set missing a required expectation blocks).
    routing_override: Option<Vec<M5OwningWindowRouting>>,
    dialog_binding: DialogBindingState,
    reopen_continuity: ReopenContinuityState,
    focus_retention: FocusRetentionState,
    os_notification_privacy: OsNotificationPrivacyState,
    waiver: Option<RoutingContinuityWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl RoutingSpec {
    /// A full-routing posture: bound to the owning object, exact reopen, no focus theft,
    /// privacy-safe OS summaries.
    fn stable() -> Self {
        Self {
            routing_override: None,
            dialog_binding: DialogBindingState::BoundToOwningWindowObject,
            reopen_continuity: ReopenContinuityState::ReopensExactObjectOrTruthfulPlaceholder,
            focus_retention: FocusRetentionState::NoFocusStealOnTyping,
            os_notification_privacy: OsNotificationPrivacyState::PrivacySafeSummaryPreservesReopen,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the companion dialog-binding-relocation waiver carried by the seed.
fn companion_binding_relocation_waiver() -> RoutingContinuityWaiver {
    RoutingContinuityWaiver {
        waiver_id: "waiver:companion-binding-relocation:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "When the companion's owning window is not present, an approval dialog relocates \
                 from the owning window to a disclosed, still-visible re-notification affordance in \
                 the primary workspace window rather than stealing focus or orphaning; the \
                 owning-window binding is re-established the moment that window returns, and the \
                 relocation is disclosed, never silent, while the shared attention-route contract \
                 is unified in the next sync."
            .to_owned(),
        owner_role: "Companion surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Builds the per-window routed-action plans for a family from its declared window classes.
///
/// Every window the family may live in binds a routed dialog, notification, or approval to
/// the owning window and authoritative object, preserves focus on protected typing
/// surfaces, and keeps a single exact reopen path.
fn window_plans(window_classes: &[M5WindowClass]) -> Vec<RoutedActionWindowPlan> {
    window_classes
        .iter()
        .map(|&window_class| RoutedActionWindowPlan {
            window_class,
            binds_to_owning_object: true,
            preserves_typing_focus: true,
            keeps_single_reopen_path: true,
        })
        .collect()
}

/// Builds one routing row from a frozen matrix row and a routing posture.
fn row_from_matrix(matrix_row: &M5ShellSurfaceRow, spec: RoutingSpec) -> RoutingContinuityRow {
    let declared_routing = spec
        .routing_override
        .unwrap_or_else(|| matrix_row.owning_window_routing.clone());
    let window_plans = window_plans(&matrix_row.window_classes);
    let mut row = RoutingContinuityRow {
        family: matrix_row.family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        surface_label: surface_label(matrix_row.family).to_owned(),
        canonical_slot: matrix_row.canonical_slot,
        fallback_slot: matrix_row.fallback_slot,
        declared_window_classes: matrix_row.window_classes.clone(),
        declared_owning_window_routing: declared_routing,
        window_plans,
        dialog_binding: spec.dialog_binding,
        reopen_continuity: spec.reopen_continuity,
        focus_retention: spec.focus_retention,
        os_notification_privacy: spec.os_notification_privacy,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: RoutingContinuityStatus::Green,
        routing_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.routing_causes = row.recompute_causes();
    row
}

/// Returns the seeded owning-window routing posture for one governed family.
fn routing_spec(family: M5ShellSurfaceFamily) -> RoutingSpec {
    match family {
        M5ShellSurfaceFamily::Docs => RoutingSpec {
            // A Stable family narrowed only by focus behavior: a docs update notification
            // defers to a disclosed badge or activity-center row rather than stealing focus
            // while a protected typing path is active.
            focus_retention: FocusRetentionState::DisclosedDeferralToBadgeOrCenter,
            narrowing_reason: Some(
                "A docs update notification defers to a disclosed badge and activity-center row \
                 rather than stealing focus from an active typing surface; the row is narrowed \
                 below green while every routed action still binds to the owning window and object.",
            ),
            ..RoutingSpec::stable()
        },
        M5ShellSurfaceFamily::Incident => RoutingSpec {
            // Beta-qualified and reopens into a disclosed truthful placeholder when the live
            // war-room sub-state cannot be restored.
            reopen_continuity: ReopenContinuityState::DisclosedPlaceholderNarrowing,
            narrowing_reason: Some(
                "The incident surface is qualified at Beta; a durable incident notification reopens \
                 onto a truthful placeholder that discloses the live war-room sub-state could not be \
                 restored while preserving the incident identity and the single reopen path, so the \
                 claim is narrowed and disclosed.",
            ),
            ..RoutingSpec::stable()
        },
        M5ShellSurfaceFamily::Companion => RoutingSpec {
            // Beta-qualified and relocates a routed approval dialog to a disclosed, waivered
            // still-visible affordance when its owning window is not present.
            dialog_binding: DialogBindingState::DisclosedBindingRelocation,
            waiver: Some(companion_binding_relocation_waiver()),
            narrowing_reason: Some(
                "The companion surface is qualified at Beta; when its owning window is not present \
                 an approval dialog relocates to a disclosed, waivered still-visible \
                 re-notification affordance in the primary workspace window rather than stealing \
                 focus or orphaning, and re-establishes the owning-window binding when that window \
                 returns.",
            ),
            ..RoutingSpec::stable()
        },
        M5ShellSurfaceFamily::Operator => RoutingSpec {
            // Beta-qualified and discloses a narrowed minimal OS-notification summary while
            // still preserving the single exact in-app reopen path.
            os_notification_privacy: OsNotificationPrivacyState::DisclosedMinimalSummary,
            narrowing_reason: Some(
                "The operator surface is qualified at Beta; its OS-notification summary discloses a \
                 narrowed minimal projection (an even more redacted control-plane summary) while \
                 still routing to the single exact in-app reopen path without bypassing in-app \
                 review, so the claim is narrowed and disclosed.",
            ),
            ..RoutingSpec::stable()
        },
        _ => RoutingSpec::stable(),
    }
}

/// Builds the routing rows for the canonical seed, one per matrix family.
fn seeded_rows() -> Vec<RoutingContinuityRow> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, routing_spec(matrix_row.family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellSurfaceFamily, mutate: F) -> Vec<RoutingContinuityRow>
where
    F: Fn(&mut RoutingSpec),
{
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = routing_spec(matrix_row.family);
            if matrix_row.family == target {
                mutate(&mut spec);
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<RoutingContinuityRow>) -> RoutingContinuityPacket {
    build_m5_owning_window_routing_packet(RoutingContinuityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 owning-window routing packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and
/// CSV artifacts. Six families preserve full owning-window routing (green); the docs
/// surface auto-narrows to yellow disclosing a badge/activity-center focus deferral, the
/// incident surface auto-narrows to yellow disclosing a truthful reopen placeholder, the
/// companion surface auto-narrows to yellow with a waivered dialog-binding relocation, and
/// the operator surface auto-narrows to yellow disclosing a narrowed minimal
/// OS-notification summary — and no row is blocked, so the packet is clean and every row
/// is publishable.
pub fn seeded_m5_owning_window_routing_packet() -> RoutingContinuityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook's routed dialog binding is lost to focus theft or
/// orphaning, proving a lost owning-window binding blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_owning_window_routing_packet_notebook_dialog_binding_lost_blocked(
) -> RoutingContinuityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Notebook, |spec| {
        spec.dialog_binding = DialogBindingState::BindingLostOrOrphaned;
        spec.narrowing_reason = Some(
            "A notebook destructive-confirmation dialog stole focus from an unrelated window and \
             was orphaned when its owning editor detached, losing the owning-window binding, so the \
             row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the preview surface steals focus from a protected typing
/// surface, proving a focus-steal regression blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_owning_window_routing_packet_preview_focus_stolen_blocked(
) -> RoutingContinuityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Preview, |spec| {
        spec.focus_retention = FocusRetentionState::FocusStolenFromTyping;
        spec.narrowing_reason = Some(
            "A preview render-complete notification pulled focus away from an active typing surface \
             instead of deferring to a badge or activity-center row, so the row blocks before \
             keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data grid's durable reopen lands on a generic shell, proving
/// a generic-shell reopen blocks promotion (red) before the row can keep its
/// reopen-continuity claim.
pub fn seeded_m5_owning_window_routing_packet_datagrid_reopen_generic_shell_blocked(
) -> RoutingContinuityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::DataGrid, |spec| {
        spec.reopen_continuity = ReopenContinuityState::LandsOnGenericShell;
        spec.narrowing_reason = Some(
            "A durable data-grid notification reopened onto a generic home screen instead of the \
             exact grid object or a truthful placeholder, losing the object identity and its reopen \
             path, so the row blocks before keeping its reopen-continuity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the review surface's OS notification leaks content or bypasses
/// in-app review, proving a privacy/boundary violation blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_owning_window_routing_packet_review_os_notification_leak_blocked(
) -> RoutingContinuityPacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Review, |spec| {
        spec.os_notification_privacy = OsNotificationPrivacyState::LeaksContentOrBypassesReview;
        spec.narrowing_reason = Some(
            "A review OS notification embedded the change-request diff text in its summary and let \
             the approval be actioned from the OS shell without in-app review, leaking content and \
             bypassing the in-app review boundary, so the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}
