//! Canonical seed builders for the M5 trust-component accessibility parity proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed and blocked fixtures. The headless emitter and the inline tests
//! both call them so the in-code certification proof, the artifacts, and the fixtures never drift.
//! The component bindings — settings-row states, source pills, consequence classes, scope states,
//! chronology verbs, provenance badges, detail states, export fields, accessibility routes,
//! required labels, shell zones, responsive classes, window classes, surface families, consumer
//! surfaces, downgrade triggers, and qualification — are pulled straight from the frozen
//! trust-chronology component matrix's seeded six component rows (the union across every family), so
//! this proof cannot certify an accessibility posture the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix::{
    seeded_m5_trust_chronology_component_matrix, M5TrustComponentRow,
    M5_TRUST_COMPONENTS_MATRIX_PACKET_ID,
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

/// Owner role accountable for keeping trust-component accessibility parity certified.
const TRUST_COMPONENT_OWNER_ROLE: &str = "Shell/accessibility owner";

/// Canonical order of the ten shell consumer surfaces. `M5ShellConsumerSurface` derives no `Ord`
/// and exposes no `ALL`, so the union of consumer surfaces across the six component rows is ordered
/// against this local list to stay deterministic.
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
    zoom_contrast_density: ZoomContrastDensityState,
    motion_alternative: MotionAlternativeState,
    support_export_parity: SupportExportParityState,
    never_hover_color_only_or_compaction_lost: bool,
    waiver: Option<TrustComponentParityWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: non-visual reach, zoom/contrast/density stability, motion
    /// alternatives, reconstructable export.
    fn certified() -> Self {
        Self {
            non_visual_reach: NonVisualReachState::KeyboardFocusAndNarrationReachable,
            zoom_contrast_density: ZoomContrastDensityState::LegibleStableUnderZoomContrastDensity,
            motion_alternative: MotionAlternativeState::DurableTextAlternativePresent,
            support_export_parity: SupportExportParityState::ComponentTruthReconstructable,
            never_hover_color_only_or_compaction_lost: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the reduced-motion reduced-alternative waiver carried by the seed.
fn reduced_motion_alternative_waiver() -> TrustComponentParityWaiver {
    TrustComponentParityWaiver {
        waiver_id: "waiver:reduced-motion-alternative:0001".to_owned(),
        condition: M5TrustAccessibilityCondition::ReducedMotion,
        reason: "Under the seeded reduced-motion condition the trust components serve a summarized \
                 durable text alternative for a small set of high-frequency live updates (a batched \
                 count in place of a pulsing chronology badge) while every component keeps a durable \
                 static text path, no truth is motion-only, and the reduced alternative is disclosed \
                 and reversible. The narrowing is disclosed, never hides a state, and keeps the \
                 keyboard/focus route."
            .to_owned(),
        owner_role: TRUST_COMPONENT_OWNER_ROLE.to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed accessibility condition.
fn certification_spec(condition: M5TrustAccessibilityCondition) -> CertificationSpec {
    match condition {
        M5TrustAccessibilityCondition::ScreenReaderNarration => CertificationSpec {
            non_visual_reach: NonVisualReachState::DisclosedReducedReachDetail,
            narrowing_reason: Some(
                "Under the seeded screen-reader narration condition a small set of long \
                 capability-scope and chronology-detail narrations is disclosedly summarized (the \
                 full detail stays reachable on focus) while every component stays keyboard-focusable \
                 and announced and focus returns in order after dismiss; the reduction is disclosed \
                 and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5TrustAccessibilityCondition::HighZoom => CertificationSpec {
            zoom_contrast_density:
                ZoomContrastDensityState::DisclosedReducedZoomContrastDensityDetail,
            narrowing_reason: Some(
                "Under the seeded high-zoom condition a few settings-row source pills and \
                 chronology verb labels wrap to a shorter form and a decorative accent drops to fit \
                 the large-text layout, while every component stays legible, keeps a non-color-only \
                 affordance, and keeps its truth-bearing content and reopen path; the reduction is \
                 disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5TrustAccessibilityCondition::ReducedMotion => CertificationSpec {
            motion_alternative: MotionAlternativeState::DisclosedReducedAlternativeDetail,
            waiver: Some(reduced_motion_alternative_waiver()),
            narrowing_reason: Some(
                "The reduced-motion condition serves a summarized durable text alternative for a \
                 small set of high-frequency chronology live updates to avoid animating many \
                 components at once, while every component keeps a durable static text path and no \
                 truth is motion-only; the reduced alternative is disclosed behind a waiver, so the \
                 row is narrowed below green while the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5TrustAccessibilityCondition::HighContrast => CertificationSpec {
            support_export_parity: SupportExportParityState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "Under the seeded high-contrast condition the support export reconstructs the \
                 component state and accessibility posture but discloses a partial capture of some \
                 low-priority decorative-contrast detail while the export queue is throttled; the \
                 partial capture is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The six component rows frozen by the trust-chronology component matrix.
fn matrix_component_rows() -> Vec<M5TrustComponentRow> {
    let rows = seeded_m5_trust_chronology_component_matrix().component_rows;
    assert_eq!(
        rows.len(),
        M5TrustComponentFamily::ALL.len(),
        "frozen matrix declares all six trust components"
    );
    rows
}

/// The most-narrowed qualification across the six component rows, so a matrix narrowing of any
/// family is recorded on the certification row. `M5TrustQualificationClass` derives no `Ord`, so a
/// local rank orders the union deterministically.
fn worst_qualification(matrix_rows: &[M5TrustComponentRow]) -> M5TrustQualificationClass {
    fn rank(q: M5TrustQualificationClass) -> u8 {
        match q {
            M5TrustQualificationClass::Stable => 0,
            M5TrustQualificationClass::Beta => 1,
            M5TrustQualificationClass::Preview => 2,
            M5TrustQualificationClass::Experimental => 3,
            M5TrustQualificationClass::Unavailable => 4,
            M5TrustQualificationClass::Held => 5,
        }
    }
    matrix_rows
        .iter()
        .map(|row| row.qualification)
        .max_by_key(|q| rank(*q))
        .expect("at least one component row")
}

/// The union of shell zones across the six component rows, in canonical order.
fn union_shell_zone_slots(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5ShellZoneSlot> {
    M5ShellZoneSlot::ALL
        .into_iter()
        .filter(|slot| matrix_rows.iter().any(|row| row.shell_zone_slot == *slot))
        .collect()
}

/// The union of responsive classes across the six component rows, in canonical order.
fn union_responsive_classes(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5ResponsiveClass> {
    M5ResponsiveClass::ALL
        .into_iter()
        .filter(|class| {
            matrix_rows
                .iter()
                .any(|row| row.responsive_classes.contains(class))
        })
        .collect()
}

/// The union of window classes across the six component rows, in canonical order.
fn union_window_classes(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5WindowClass> {
    M5WindowClass::ALL
        .into_iter()
        .filter(|class| {
            matrix_rows
                .iter()
                .any(|row| row.window_classes.contains(class))
        })
        .collect()
}

/// The union of surface families across the six component rows, in canonical order.
fn union_surface_families(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5ShellSurfaceFamily> {
    M5ShellSurfaceFamily::ALL
        .into_iter()
        .filter(|family| {
            matrix_rows
                .iter()
                .any(|row| row.surface_families.contains(family))
        })
        .collect()
}

/// The union of settings-row states across the six component rows, in canonical order.
fn union_settings_row_states(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5SettingsRowState> {
    M5SettingsRowState::ALL
        .into_iter()
        .filter(|state| {
            matrix_rows
                .iter()
                .any(|row| row.settings_row_states.contains(state))
        })
        .collect()
}

/// The union of source pills across the six component rows, in canonical order.
fn union_source_pills(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5SettingSourcePill> {
    M5SettingSourcePill::ALL
        .into_iter()
        .filter(|pill| {
            matrix_rows
                .iter()
                .any(|row| row.source_pills.contains(pill))
        })
        .collect()
}

/// The union of capability consequence classes across the six component rows, in canonical order.
fn union_consequence_classes(
    matrix_rows: &[M5TrustComponentRow],
) -> Vec<M5CapabilityConsequenceClass> {
    M5CapabilityConsequenceClass::ALL
        .into_iter()
        .filter(|class| {
            matrix_rows
                .iter()
                .any(|row| row.consequence_classes.contains(class))
        })
        .collect()
}

/// The union of capability scope states across the six component rows, in canonical order.
fn union_capability_scope_states(
    matrix_rows: &[M5TrustComponentRow],
) -> Vec<M5CapabilityScopeState> {
    M5CapabilityScopeState::ALL
        .into_iter()
        .filter(|state| {
            matrix_rows
                .iter()
                .any(|row| row.capability_scope_states.contains(state))
        })
        .collect()
}

/// The union of chronology verbs across the six component rows, in canonical order.
fn union_chronology_verbs(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5ChronologyVerb> {
    M5ChronologyVerb::ALL
        .into_iter()
        .filter(|verb| {
            matrix_rows
                .iter()
                .any(|row| row.chronology_verbs.contains(verb))
        })
        .collect()
}

/// The union of provenance badges across the six component rows, in canonical order.
fn union_provenance_badges(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5ProvenanceBadge> {
    M5ProvenanceBadge::ALL
        .into_iter()
        .filter(|badge| {
            matrix_rows
                .iter()
                .any(|row| row.provenance_badges.contains(badge))
        })
        .collect()
}

/// The union of chronology detail states across the six component rows, in canonical order.
fn union_chronology_detail_states(
    matrix_rows: &[M5TrustComponentRow],
) -> Vec<M5ChronologyDetailState> {
    M5ChronologyDetailState::ALL
        .into_iter()
        .filter(|state| {
            matrix_rows
                .iter()
                .any(|row| row.chronology_detail_states.contains(state))
        })
        .collect()
}

/// The union of chronology export fields across the six component rows, in canonical order.
fn union_chronology_export_fields(
    matrix_rows: &[M5TrustComponentRow],
) -> Vec<M5ChronologyExportField> {
    M5ChronologyExportField::ALL
        .into_iter()
        .filter(|field| {
            matrix_rows
                .iter()
                .any(|row| row.chronology_export_fields.contains(field))
        })
        .collect()
}

/// The union of accessibility routes across the six component rows, in order.
fn union_accessibility_routes(
    matrix_rows: &[M5TrustComponentRow],
) -> Vec<M5TrustAccessibilityRoute> {
    M5TrustAccessibilityRoute::ALL
        .into_iter()
        .filter(|route| {
            matrix_rows
                .iter()
                .any(|row| row.accessibility_routes.contains(route))
        })
        .collect()
}

/// The union of required labels across the six component rows, in canonical order.
fn union_required_labels(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5TrustRequiredLabel> {
    M5TrustRequiredLabel::ALL
        .into_iter()
        .filter(|label| {
            matrix_rows
                .iter()
                .any(|row| row.required_labels.contains(label))
        })
        .collect()
}

/// The union of consumer surfaces across the six component rows, ordered against the local
/// canonical consumer-surface list.
fn union_consumer_surfaces(matrix_rows: &[M5TrustComponentRow]) -> Vec<M5ShellConsumerSurface> {
    CONSUMER_SURFACE_ORDER
        .into_iter()
        .filter(|surface| {
            matrix_rows
                .iter()
                .any(|row| row.consumer_surfaces.contains(surface))
        })
        .collect()
}

/// The union of downgrade triggers across the six component rows, ordered against the frozen
/// trigger declaration order (`M5TrustComponentDowngradeTrigger` derives no `Ord`).
fn union_downgrade_triggers(
    matrix_rows: &[M5TrustComponentRow],
) -> Vec<M5TrustComponentDowngradeTrigger> {
    M5TrustComponentDowngradeTrigger::ALL
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
    condition: M5TrustAccessibilityCondition,
    matrix_rows: &[M5TrustComponentRow],
    spec: CertificationSpec,
) -> TrustComponentParityRow {
    let mut row = TrustComponentParityRow {
        condition,
        driven_component_families: M5TrustComponentFamily::ALL.to_vec(),
        matrix_qualification: worst_qualification(matrix_rows),
        owner_role: TRUST_COMPONENT_OWNER_ROLE.to_owned(),
        condition_label: condition.label().to_owned(),
        certified_shell_zone_slots: union_shell_zone_slots(matrix_rows),
        certified_responsive_classes: union_responsive_classes(matrix_rows),
        certified_window_classes: union_window_classes(matrix_rows),
        certified_surface_families: union_surface_families(matrix_rows),
        certified_settings_row_states: union_settings_row_states(matrix_rows),
        certified_source_pills: union_source_pills(matrix_rows),
        certified_consequence_classes: union_consequence_classes(matrix_rows),
        certified_capability_scope_states: union_capability_scope_states(matrix_rows),
        certified_chronology_verbs: union_chronology_verbs(matrix_rows),
        certified_provenance_badges: union_provenance_badges(matrix_rows),
        certified_chronology_detail_states: union_chronology_detail_states(matrix_rows),
        certified_chronology_export_fields: union_chronology_export_fields(matrix_rows),
        accessibility_routes: union_accessibility_routes(matrix_rows),
        required_labels: union_required_labels(matrix_rows),
        consumer_surfaces: union_consumer_surfaces(matrix_rows),
        applicable_downgrade_triggers: union_downgrade_triggers(matrix_rows),
        non_visual_reach: spec.non_visual_reach,
        zoom_contrast_density: spec.zoom_contrast_density,
        motion_alternative: spec.motion_alternative,
        support_export_parity: spec.support_export_parity,
        never_hover_color_only_or_compaction_lost: spec.never_hover_color_only_or_compaction_lost,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: TrustComponentParityStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per accessibility condition.
fn seeded_rows() -> Vec<TrustComponentParityRow> {
    let matrix_rows = matrix_component_rows();
    M5TrustAccessibilityCondition::ALL
        .iter()
        .map(|&condition| {
            row_from_condition(condition, &matrix_rows, certification_spec(condition))
        })
        .collect()
}

/// Builds a variant where one condition's spec is mutated after the canonical spec is resolved,
/// used by the blocked fixtures.
fn seeded_rows_with<F>(
    target: M5TrustAccessibilityCondition,
    mutate: F,
) -> Vec<TrustComponentParityRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = matrix_component_rows();
    M5TrustAccessibilityCondition::ALL
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

fn packet_from_rows(rows: Vec<TrustComponentParityRow>) -> TrustComponentParityPacket {
    build_m5_trust_component_accessibility_parity_packet(TrustComponentParityInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_TRUST_COMPONENTS_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 trust-component accessibility parity packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Three conditions are certified at full standing (green); the screen-reader narration
/// condition auto-narrows to yellow behind a disclosed reduced reach detail, the high-zoom condition
/// auto-narrows to yellow behind a disclosed reduced zoom/contrast/density detail, the
/// reduced-motion condition auto-narrows to yellow behind a waivered reduced motion alternative, and
/// the high-contrast condition auto-narrows to yellow behind a disclosed partial support-export
/// capture — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_trust_component_accessibility_parity_packet() -> TrustComponentParityPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the keyboard-reach condition keeps a component's truth reachable only by
/// pointer or hover, proving a pointer/hover-only truth blocks the condition (red) rather than
/// passing on behavior alone.
pub fn seeded_m5_trust_component_accessibility_parity_packet_keyboard_reach_pointer_or_hover_only_blocked(
) -> TrustComponentParityPacket {
    let rows = seeded_rows_with(M5TrustAccessibilityCondition::KeyboardReach, |spec| {
        spec.non_visual_reach = NonVisualReachState::TruthReachableByPointerOrHoverOnly;
        spec.narrowing_reason = Some(
            "Under the keyboard-reach condition a settings-row source pill explainer and a \
             capability-sheet transitive-scope popover are reachable only by pointer hover, with no \
             keyboard focus or screen-reader narration, so the row blocks before keeping its \
             non-visual-reach claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the high-zoom condition truncates a component until it is unreadable,
/// proving an illegible zoom/contrast/density surface blocks the condition (red) rather than staying
/// a disclosed yellow.
pub fn seeded_m5_trust_component_accessibility_parity_packet_high_zoom_unreadable_blocked(
) -> TrustComponentParityPacket {
    let rows = seeded_rows_with(M5TrustAccessibilityCondition::HighZoom, |spec| {
        spec.zoom_contrast_density = ZoomContrastDensityState::TruncatedColorOnlyOrLostOnCompaction;
        spec.narrowing_reason = Some(
            "Under the high-zoom condition a settings-row lock state and a chronology export field \
             clip their truth-bearing content off-screen with no reopen path, so a truth-bearing \
             item is lost, and the row blocks before keeping its zoom/contrast/density-stability \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the reduced-motion condition conveys a critical live update by motion
/// only, proving a motion-only affordance blocks the condition (red) before the row can keep its
/// motion-alternative claim.
pub fn seeded_m5_trust_component_accessibility_parity_packet_reduced_motion_motion_only_blocked(
) -> TrustComponentParityPacket {
    let rows = seeded_rows_with(M5TrustAccessibilityCondition::ReducedMotion, |spec| {
        spec.motion_alternative = MotionAlternativeState::MotionOnlyAffordance;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "Under the reduced-motion condition a chronology row's live-update state is conveyed by \
             a pulsing badge only, with no durable static text alternative, so the state is \
             invisible when motion is suppressed, and the row blocks before keeping its \
             motion-alternative claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the high-contrast condition's component state is absent from the
/// support-export capture, proving a missing export blocks the condition (red) before the row can
/// keep its support-export-parity claim.
pub fn seeded_m5_trust_component_accessibility_parity_packet_high_contrast_export_absent_blocked(
) -> TrustComponentParityPacket {
    let rows = seeded_rows_with(M5TrustAccessibilityCondition::HighContrast, |spec| {
        spec.support_export_parity = SupportExportParityState::ComponentStateAbsentFromCapture;
        spec.narrowing_reason = Some(
            "Under the high-contrast condition the support export omits the component state and \
             accessibility posture entirely, so a high-contrast regression cannot be explained \
             without a live screenshot, and the row blocks before keeping its support-export-parity \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the focus-order condition keeps a critical truth hover-only, color-only,
/// or lost on compaction, proving the invariant blocks the condition (red) before the row can keep
/// its invariant.
pub fn seeded_m5_trust_component_accessibility_parity_packet_focus_order_hover_color_only_blocked(
) -> TrustComponentParityPacket {
    let rows = seeded_rows_with(M5TrustAccessibilityCondition::FocusOrder, |spec| {
        spec.never_hover_color_only_or_compaction_lost = false;
        spec.narrowing_reason = Some(
            "Under the focus-order condition a capability-sheet reduced-mode choice is conveyed by \
             color alone and a chronology detail is dropped when the surface compacts, so a critical \
             truth is kept hover-/color-only or compaction-lost, and the row blocks before keeping \
             its invariant.",
        );
    });
    packet_from_rows(rows)
}
