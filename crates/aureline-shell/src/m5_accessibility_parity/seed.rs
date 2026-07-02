//! Canonical seed builders for the M5 accessibility parity proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export,
//! and CSV artifacts plus the narrowed and blocked fixtures. The headless emitter and the
//! inline tests both call them so the in-code certification proof, the artifacts, and the
//! fixtures never drift. The primitive bindings — status-item classes, overflow behaviors,
//! representation classes, promotion states, pane-resize states, progress states,
//! source/provider/freshness labels, accessibility routes, required labels, shell zones,
//! consumer surfaces, downgrade triggers, and qualification — are pulled straight from the
//! frozen shell-primitives matrix's seeded ten primitive rows (the union across every family),
//! so this proof cannot certify an accessibility posture the matrix does not freeze.

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

/// Owner role accountable for keeping shell-primitive accessibility parity certified.
const ACCESSIBILITY_OWNER_ROLE: &str = "Shell/accessibility owner";

/// Canonical order of the ten shell consumer surfaces. `M5ShellConsumerSurface` derives no
/// `Ord` and exposes no `ALL`, so the union of consumer surfaces across the ten primitive rows
/// is ordered against this local list to stay deterministic.
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

/// The certification posture seeded for one governed accessibility condition.
struct CertificationSpec {
    non_visual_reach: NonVisualReachState,
    zoom_contrast_stability: ZoomContrastStabilityState,
    motion_touch_alternative: MotionTouchAlternativeState,
    accessibility_export: AccessibilityExportState,
    never_pointer_or_hover_only: bool,
    waiver: Option<AccessibilityParityWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: non-visual reach, zoom/contrast stability, motion/touch
    /// alternatives, reconstructable export.
    fn certified() -> Self {
        Self {
            non_visual_reach: NonVisualReachState::KeyboardFocusAndNarrationReachable,
            zoom_contrast_stability: ZoomContrastStabilityState::LegibleStableUnderZoomAndContrast,
            motion_touch_alternative:
                MotionTouchAlternativeState::DurableTextAndTouchAlternativesPresent,
            accessibility_export:
                AccessibilityExportState::AccessibilityPostureAndStateReconstructable,
            never_pointer_or_hover_only: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the reduced-motion reduced-alternative waiver carried by the seed.
fn reduced_motion_alternative_waiver() -> AccessibilityParityWaiver {
    AccessibilityParityWaiver {
        waiver_id: "waiver:reduced-motion-alternative:0001".to_owned(),
        condition: M5AccessibilityCondition::ReducedMotion,
        reason: "Under the seeded reduced-motion condition the shell serves a summarized durable \
                 text alternative for a small set of high-frequency progress rows (a batched \
                 count in place of per-item motion) and a coarser touch target for the splitter, \
                 while every primitive keeps a durable text and touch path, no truth is motion- \
                 or pointer-only, and the reduced alternative is disclosed and reversible. The \
                 narrowing is disclosed, never hides a state, and keeps the keyboard/focus route."
            .to_owned(),
        owner_role: ACCESSIBILITY_OWNER_ROLE.to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed accessibility condition.
fn certification_spec(condition: M5AccessibilityCondition) -> CertificationSpec {
    match condition {
        M5AccessibilityCondition::ScreenReaderNarration => CertificationSpec {
            non_visual_reach: NonVisualReachState::DisclosedReducedReachDetail,
            narrowing_reason: Some(
                "Under the seeded screen-reader narration condition a small set of long \
                 hovercard/peek narrations is disclosedly summarized (the full detail stays \
                 reachable on focus) while every primitive stays keyboard-focusable and \
                 announced and focus returns after dismiss; the reduction is disclosed and the \
                 row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5AccessibilityCondition::HighZoom => CertificationSpec {
            zoom_contrast_stability: ZoomContrastStabilityState::DisclosedReducedZoomContrastDetail,
            narrowing_reason: Some(
                "Under the seeded high-zoom condition a few status-item labels wrap to a shorter \
                 form and a decorative accent drops to fit the large-text layout, while every \
                 primitive stays legible and keeps its truth-bearing content and reopen path; the \
                 reduction is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5AccessibilityCondition::ReducedMotion => CertificationSpec {
            motion_touch_alternative:
                MotionTouchAlternativeState::DisclosedReducedAlternativeDetail,
            waiver: Some(reduced_motion_alternative_waiver()),
            narrowing_reason: Some(
                "The reduced-motion condition serves a summarized durable text alternative for a \
                 small set of high-frequency progress rows and a coarser splitter touch target to \
                 avoid animating many primitives at once, while every primitive keeps a durable \
                 text and touch path and no truth is motion- or pointer-only; the reduced \
                 alternative is disclosed behind a waiver, so the row is narrowed below green \
                 while the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5AccessibilityCondition::HighContrast => CertificationSpec {
            accessibility_export: AccessibilityExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "Under the seeded high-contrast condition the support export reconstructs the \
                 primitive state and accessibility posture but discloses a partial capture of \
                 some low-priority decorative-contrast detail while the export queue is throttled; \
                 the partial capture is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The ten primitive rows frozen by the shell-primitives matrix.
fn matrix_primitive_rows() -> Vec<M5ShellPrimitiveRow> {
    let rows = seeded_m5_shell_primitives_matrix().primitive_rows;
    assert_eq!(
        rows.len(),
        M5ShellPrimitiveFamily::ALL.len(),
        "frozen matrix declares all ten shell primitives"
    );
    rows
}

/// The most-narrowed qualification across the ten primitive rows, so a matrix narrowing of any
/// family is recorded on the certification row.
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
        .expect("at least one primitive row")
}

/// The union of shell zones across the ten primitive rows, in canonical order.
fn union_shell_zone_slots(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5ShellZoneSlot> {
    M5ShellZoneSlot::ALL
        .into_iter()
        .filter(|slot| matrix_rows.iter().any(|row| row.shell_zone_slot == *slot))
        .collect()
}

/// The union of status-item classes across the ten primitive rows, in canonical order.
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

/// The union of overflow behaviors across the ten primitive rows, in canonical order.
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

/// The union of representation classes across the ten primitive rows, in canonical order.
fn union_representation_classes(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5RepresentationClass> {
    M5RepresentationClass::ALL
        .into_iter()
        .filter(|class| {
            matrix_rows
                .iter()
                .any(|row| row.representation_classes.contains(class))
        })
        .collect()
}

/// The union of promotion states across the ten primitive rows, in canonical order.
fn union_promotion_states(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5PromotionState> {
    M5PromotionState::ALL
        .into_iter()
        .filter(|state| {
            matrix_rows
                .iter()
                .any(|row| row.promotion_states.contains(state))
        })
        .collect()
}

/// The union of pane-resize states across the ten primitive rows, in canonical order.
fn union_pane_resize_states(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5PaneResizeState> {
    M5PaneResizeState::ALL
        .into_iter()
        .filter(|state| {
            matrix_rows
                .iter()
                .any(|row| row.pane_resize_states.contains(state))
        })
        .collect()
}

/// The union of progress states across the ten primitive rows, in canonical order.
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

/// The union of source/freshness labels across the ten primitive rows, in canonical order.
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

/// The union of accessibility routes across the ten primitive rows, in order.
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

/// The union of required labels across the ten primitive rows, in canonical order.
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

/// The union of consumer surfaces across the ten primitive rows, ordered against the local
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

/// The union of downgrade triggers across the ten primitive rows, ordered against the frozen
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

/// Builds one certification row from the frozen matrix rows and a posture.
fn row_from_condition(
    condition: M5AccessibilityCondition,
    matrix_rows: &[M5ShellPrimitiveRow],
    spec: CertificationSpec,
) -> AccessibilityParityRow {
    let mut row = AccessibilityParityRow {
        condition,
        driven_primitive_families: M5ShellPrimitiveFamily::ALL.to_vec(),
        matrix_qualification: worst_qualification(matrix_rows),
        owner_role: ACCESSIBILITY_OWNER_ROLE.to_owned(),
        condition_label: condition.label().to_owned(),
        certified_shell_zone_slots: union_shell_zone_slots(matrix_rows),
        certified_status_item_classes: union_status_item_classes(matrix_rows),
        certified_overflow_behaviors: union_overflow_behaviors(matrix_rows),
        certified_representation_classes: union_representation_classes(matrix_rows),
        certified_promotion_states: union_promotion_states(matrix_rows),
        certified_pane_resize_states: union_pane_resize_states(matrix_rows),
        certified_progress_states: union_progress_states(matrix_rows),
        certified_source_freshness_labels: union_source_freshness_labels(matrix_rows),
        accessibility_routes: union_accessibility_routes(matrix_rows),
        required_labels: union_required_labels(matrix_rows),
        consumer_surfaces: union_consumer_surfaces(matrix_rows),
        applicable_downgrade_triggers: union_downgrade_triggers(matrix_rows),
        non_visual_reach: spec.non_visual_reach,
        zoom_contrast_stability: spec.zoom_contrast_stability,
        motion_touch_alternative: spec.motion_touch_alternative,
        accessibility_export: spec.accessibility_export,
        never_pointer_or_hover_only: spec.never_pointer_or_hover_only,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: AccessibilityParityStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per accessibility condition.
fn seeded_rows() -> Vec<AccessibilityParityRow> {
    let matrix_rows = matrix_primitive_rows();
    M5AccessibilityCondition::ALL
        .iter()
        .map(|&condition| {
            row_from_condition(condition, &matrix_rows, certification_spec(condition))
        })
        .collect()
}

/// Builds a variant where one condition's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5AccessibilityCondition, mutate: F) -> Vec<AccessibilityParityRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = matrix_primitive_rows();
    M5AccessibilityCondition::ALL
        .iter()
        .map(|&condition| {
            let mut spec = certification_spec(condition);
            if condition == target {
                mutate(&mut spec);
            }
            row_from_condition(condition, &matrix_rows, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<AccessibilityParityRow>) -> AccessibilityParityPacket {
    build_m5_accessibility_parity_packet(AccessibilityParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 accessibility parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Three conditions are certified at full standing (green); the screen-reader
/// narration condition auto-narrows to yellow behind a disclosed reduced reach detail, the
/// high-zoom condition auto-narrows to yellow behind a disclosed reduced zoom/contrast detail,
/// the reduced-motion condition auto-narrows to yellow behind a waivered reduced motion/touch
/// alternative, and the high-contrast condition auto-narrows to yellow behind a disclosed
/// partial support-export capture — and no row is blocked, so the packet is clean and every row
/// is publishable.
pub fn seeded_m5_accessibility_parity_packet() -> AccessibilityParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the keyboard-reach condition keeps a primitive's truth reachable only
/// by pointer or hover, proving a pointer/hover-only truth blocks the condition (red) rather
/// than passing on behavior alone.
pub fn seeded_m5_accessibility_parity_packet_keyboard_reach_pointer_or_hover_only_blocked(
) -> AccessibilityParityPacket {
    let rows = seeded_rows_with(M5AccessibilityCondition::KeyboardReach, |spec| {
        spec.non_visual_reach = NonVisualReachState::TruthReachableByPointerOrHoverOnly;
        spec.narrowing_reason = Some(
            "Under the keyboard-reach condition a hovercard's attributed detail and a splitter's \
             resize affordance are reachable only by pointer hover, with no keyboard focus or \
             screen-reader narration, so the row blocks before keeping its non-visual-reach claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the high-zoom condition truncates a primitive until it is unreadable,
/// proving an illegible zoom/contrast surface blocks the condition (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_accessibility_parity_packet_high_zoom_unreadable_blocked(
) -> AccessibilityParityPacket {
    let rows = seeded_rows_with(M5AccessibilityCondition::HighZoom, |spec| {
        spec.zoom_contrast_stability =
            ZoomContrastStabilityState::TruncatedOrUnreadableUnderZoomOrContrast;
        spec.narrowing_reason = Some(
            "Under the high-zoom condition a problem-count status item and a progress row clip \
             their truth-bearing content off-screen with no reopen path, so a truth-bearing item \
             is lost, and the row blocks before keeping its zoom/contrast-stability claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the reduced-motion condition conveys critical progress by motion
/// only, proving a motion-only affordance blocks the condition (red) before the row can keep
/// its motion/touch-alternative claim.
pub fn seeded_m5_accessibility_parity_packet_reduced_motion_motion_only_blocked(
) -> AccessibilityParityPacket {
    let rows = seeded_rows_with(M5AccessibilityCondition::ReducedMotion, |spec| {
        spec.motion_touch_alternative =
            MotionTouchAlternativeState::MotionOnlyOrPointerOnlyAffordance;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "Under the reduced-motion condition a sync job's progress is conveyed by a spinner \
             only, with no durable text alternative, so the state is invisible when motion is \
             suppressed, and the row blocks before keeping its motion/touch-alternative claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the touch / context-action condition's accessibility state is absent
/// from the support-export capture, proving a missing export blocks the condition (red) before
/// the row can keep its accessibility-export claim.
pub fn seeded_m5_accessibility_parity_packet_touch_export_absent_blocked(
) -> AccessibilityParityPacket {
    let rows = seeded_rows_with(M5AccessibilityCondition::TouchContextAction, |spec| {
        spec.accessibility_export = AccessibilityExportState::AccessibilityStateAbsentFromCapture;
        spec.narrowing_reason = Some(
            "Under the touch / context-action condition the support export omits the primitive \
             state and accessibility posture entirely, so a touch-alternative regression cannot \
             be explained without a live screenshot, and the row blocks before keeping its \
             accessibility-export claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the focus-return condition keeps a critical truth pointer- or
/// hover-only, proving the pointer/hover-only invariant blocks the condition (red) before the
/// row can keep its invariant.
pub fn seeded_m5_accessibility_parity_packet_focus_return_pointer_or_hover_only_blocked(
) -> AccessibilityParityPacket {
    let rows = seeded_rows_with(M5AccessibilityCondition::FocusReturn, |spec| {
        spec.never_pointer_or_hover_only = false;
        spec.narrowing_reason = Some(
            "Under the focus-return condition a peek panel's dismiss drops focus into the void \
             and its reopen path is reachable only by pointer hover, so a critical truth is kept \
             pointer-/hover-only, and the row blocks before keeping its pointer/hover-only \
             invariant.",
        );
    });
    packet_from_rows(rows)
}
