//! Canonical seed builders for the M5 window-lifecycle-safety proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed fixtures. The headless emitter and
//! the inline tests both call them so the in-code lifecycle proof, the artifacts, and the
//! fixtures never drift. The certified rows are pulled straight from the frozen shell-zone
//! matrix's seeded packet, so the lifecycle proof cannot certify a family the matrix does
//! not freeze, and each family's declared window classes are derived from the matrix rather
//! than restated by hand.

use super::*;
use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    seeded_m5_shell_zone_matrix, M5ShellSurfaceRow, M5_SHELL_ZONE_MATRIX_PACKET_ID,
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

/// The window-lifecycle-safety posture seeded for one governed family.
struct LifecycleSpec {
    /// When set, the declared protected-resource set used instead of the canonical full set
    /// (blocked fixtures use this to prove a set missing a required resource blocks).
    protected_resources_override: Option<Vec<ProtectedCloseResource>>,
    drag_verb_disclosure: DragVerbDisclosureState,
    close_orphan_guard: CloseOrphanGuardState,
    safe_reopen_fallback: SafeReopenFallbackState,
    waiver: Option<WindowLifecycleWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl LifecycleSpec {
    /// A full-lifecycle posture: verbs advertised with keyboard parity, close guarded, and
    /// reopens into the safest equivalent layout.
    fn stable() -> Self {
        Self {
            protected_resources_override: None,
            drag_verb_disclosure: DragVerbDisclosureState::VerbDisclosedWithKeyboardParity,
            close_orphan_guard: CloseOrphanGuardState::CloseGuardedNoOrphan,
            safe_reopen_fallback: SafeReopenFallbackState::ReopensSafestEquivalentLayout,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the companion close-guard-relocation waiver carried by the seed.
fn companion_close_guard_relocation_waiver() -> WindowLifecycleWaiver {
    WindowLifecycleWaiver {
        waiver_id: "waiver:companion-close-guard-relocation:0001".to_owned(),
        family: M5ShellSurfaceFamily::Companion,
        reason: "When a secondary companion window is closed while it still holds a live approval, \
                 the approval is relocated to a disclosed, still-visible prompt in the primary \
                 workspace window rather than being silently stranded; the relocation is disclosed, \
                 never silent, and the shared close-guard contract is unified in the next sync."
            .to_owned(),
        owner_role: "Companion surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Builds the per-verb cross-window drag plans that advertise every canonical drag verb
/// before the drop completes and keep each keyboard-reachable through a command equivalent.
fn drag_plans() -> Vec<CrossWindowDragPlan> {
    REQUIRED_DRAG_VERBS
        .iter()
        .map(|&verb| CrossWindowDragPlan {
            verb,
            disclosed_before_drop: true,
            keyboard_command_equivalent: true,
        })
        .collect()
}

/// Builds one lifecycle row from a frozen matrix row and a lifecycle posture.
fn row_from_matrix(matrix_row: &M5ShellSurfaceRow, spec: LifecycleSpec) -> WindowLifecycleRow {
    let declared_protected_resources = spec
        .protected_resources_override
        .unwrap_or_else(|| REQUIRED_PROTECTED_RESOURCES.to_vec());
    let mut row = WindowLifecycleRow {
        family: matrix_row.family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        surface_label: surface_label(matrix_row.family).to_owned(),
        canonical_slot: matrix_row.canonical_slot,
        fallback_slot: matrix_row.fallback_slot,
        declared_window_classes: matrix_row.window_classes.clone(),
        declared_protected_resources,
        drag_plans: drag_plans(),
        drag_verb_disclosure: spec.drag_verb_disclosure,
        close_orphan_guard: spec.close_orphan_guard,
        safe_reopen_fallback: spec.safe_reopen_fallback,
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: WindowLifecycleStatus::Green,
        lifecycle_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.lifecycle_causes = row.recompute_causes();
    row
}

/// Returns the seeded window-lifecycle-safety posture for one governed family.
fn lifecycle_spec(family: M5ShellSurfaceFamily) -> LifecycleSpec {
    match family {
        M5ShellSurfaceFamily::Docs => LifecycleSpec {
            // A Stable family narrowed only by drag-verb disclosure: a docs cross-window
            // drag verb is advertised before the drop but reachable via a disclosed
            // command-palette equivalent rather than an inline pre-drop hint.
            drag_verb_disclosure: DragVerbDisclosureState::DisclosedVerbReachNarrowing,
            narrowing_reason: Some(
                "A docs cross-window drag verb is advertised before the drop but is reachable only \
                 through a disclosed command-palette equivalent rather than an inline pre-drop hint; \
                 keyboard parity is preserved, so the row is narrowed below green while every drag \
                 verb still advertises its resulting action.",
            ),
            ..LifecycleSpec::stable()
        },
        M5ShellSurfaceFamily::Incident => LifecycleSpec {
            // Beta-qualified and reopens into a disclosed reduced but still-safe equivalent
            // when the live war-room feature pack cannot be restored.
            safe_reopen_fallback: SafeReopenFallbackState::DisclosedReducedEquivalentFallback,
            narrowing_reason: Some(
                "The incident surface is qualified at Beta; when its live war-room feature pack is \
                 unavailable after a crash or restore it reopens onto a disclosed reduced but \
                 still-safe equivalent layout that preserves the incident identity and reopen path, \
                 so the claim is narrowed and disclosed.",
            ),
            ..LifecycleSpec::stable()
        },
        M5ShellSurfaceFamily::Companion => LifecycleSpec {
            // Beta-qualified and defers a live approval to a disclosed, waivered relocation
            // into the primary window when its secondary companion window is closed.
            close_orphan_guard: CloseOrphanGuardState::DisclosedDeferredGuardRelocation,
            waiver: Some(companion_close_guard_relocation_waiver()),
            narrowing_reason: Some(
                "The companion surface is qualified at Beta; when a secondary companion window is \
                 closed while it still holds a live approval, the approval is deferred to a \
                 disclosed, waivered relocation into the primary workspace window with a \
                 still-visible prompt rather than being silently stranded.",
            ),
            ..LifecycleSpec::stable()
        },
        M5ShellSurfaceFamily::Operator => LifecycleSpec {
            // Beta-qualified and reopens into a disclosed reduced but still-safe equivalent
            // when a control-plane extension or remote target is unavailable.
            safe_reopen_fallback: SafeReopenFallbackState::DisclosedReducedEquivalentFallback,
            narrowing_reason: Some(
                "The operator surface is qualified at Beta; when a control-plane extension or remote \
                 target is unavailable after a crash or restore it reopens onto a disclosed reduced \
                 but still-safe equivalent layout that preserves the control-plane identity and \
                 reopen path, so the claim is narrowed and disclosed.",
            ),
            ..LifecycleSpec::stable()
        },
        _ => LifecycleSpec::stable(),
    }
}

/// Builds the lifecycle rows for the canonical seed, one per matrix family.
fn seeded_rows() -> Vec<WindowLifecycleRow> {
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| row_from_matrix(matrix_row, lifecycle_spec(matrix_row.family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellSurfaceFamily, mutate: F) -> Vec<WindowLifecycleRow>
where
    F: Fn(&mut LifecycleSpec),
{
    seeded_m5_shell_zone_matrix()
        .surface_rows
        .iter()
        .map(|matrix_row| {
            let mut spec = lifecycle_spec(matrix_row.family);
            if matrix_row.family == target {
                mutate(&mut spec);
            }
            row_from_matrix(matrix_row, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<WindowLifecycleRow>) -> WindowLifecyclePacket {
    build_m5_window_lifecycle_safety_packet(WindowLifecycleInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 window-lifecycle-safety packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Six families preserve full lifecycle safety (green); the docs surface
/// auto-narrows to yellow disclosing a command-palette drag-verb reach, the incident surface
/// auto-narrows to yellow disclosing a reduced-but-safe reopen equivalent, the companion
/// surface auto-narrows to yellow with a waivered close-guard relocation, and the operator
/// surface auto-narrows to yellow disclosing a reduced-but-safe reopen equivalent — and no
/// row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_window_lifecycle_safety_packet() -> WindowLifecyclePacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook's secondary-window close silently orphans a protected
/// resource, proving a silent close orphan blocks promotion (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_window_lifecycle_safety_packet_notebook_close_silent_orphan_blocked(
) -> WindowLifecyclePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Notebook, |spec| {
        spec.close_orphan_guard = CloseOrphanGuardState::SilentOrphanOnClose;
        spec.narrowing_reason = Some(
            "Closing a secondary notebook window silently stranded an unsaved buffer instead of \
             blocking or relocating it, so the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the preview surface completes a cross-window drop without
/// advertising the resulting verb, proving a hidden drag verb blocks promotion (red) rather
/// than staying a disclosed yellow.
pub fn seeded_m5_window_lifecycle_safety_packet_preview_drag_verb_hidden_blocked(
) -> WindowLifecyclePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Preview, |spec| {
        spec.drag_verb_disclosure = DragVerbDisclosureState::VerbHiddenOrKeyboardLost;
        spec.narrowing_reason = Some(
            "A preview cross-window drop completed without advertising the resulting verb, so the \
             user could not tell what the drop would do, and the row blocks before keeping a \
             shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data grid's specialized-window reopen orphans the object or
/// lands on the wrong surface, proving an unsafe reopen blocks promotion (red) before the row
/// can keep its reopen-fallback claim.
pub fn seeded_m5_window_lifecycle_safety_packet_datagrid_reopen_wrong_surface_blocked(
) -> WindowLifecyclePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::DataGrid, |spec| {
        spec.safe_reopen_fallback = SafeReopenFallbackState::ReopenOrphanedOrWrongSurface;
        spec.narrowing_reason = Some(
            "A detached data-grid window reopened onto the wrong surface when its column-provider \
             extension was missing, orphaning the grid object and losing its reopen path, so the \
             row blocks before keeping its reopen-fallback claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the review surface does not declare all four protected close
/// resources, proving an incomplete close-guard set blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_window_lifecycle_safety_packet_review_protected_resource_orphan_blocked(
) -> WindowLifecyclePacket {
    let rows = seeded_rows_with(M5ShellSurfaceFamily::Review, |spec| {
        spec.protected_resources_override = Some(vec![
            ProtectedCloseResource::DirtyBuffer,
            ProtectedCloseResource::LiveApproval,
            ProtectedCloseResource::CollaborationControl,
        ]);
        spec.narrowing_reason = Some(
            "The review surface does not declare the long-running evidence review as a protected \
             close resource, so closing a secondary review window could silently orphan an \
             in-flight evidence review, and the row blocks before keeping a shell-maturity claim.",
        );
    });
    packet_from_rows(rows)
}
