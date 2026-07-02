//! Canonical seed builders for the M5 pane-control certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed and blocked fixtures. The
//! headless emitter and the inline tests both call them so the in-code certification
//! proof, the artifacts, and the fixtures never drift. The pane-control bindings —
//! pane-resize states, accessibility routes, required labels, consumer surfaces,
//! downgrade triggers, qualification, owner, and shell zone — are pulled straight from
//! the frozen shell-primitives matrix's seeded splitter-handle and pane-resize-preset
//! rows (the union across the two pane families), so this proof cannot certify a
//! pane-control posture the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix::{
    seeded_m5_shell_primitives_matrix, M5ShellPrimitiveRow, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID,
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

/// The two pane-control primitive families this lane certifies.
const PANE_FAMILIES: [M5ShellPrimitiveFamily; 2] = [
    M5ShellPrimitiveFamily::SplitterHandle,
    M5ShellPrimitiveFamily::PaneResizePreset,
];

/// Canonical order of the ten shell consumer surfaces. `M5ShellConsumerSurface` derives
/// no `Ord` and exposes no `ALL`, so the union of consumer surfaces across the two pane
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

/// The certification posture seeded for one governed pane layout.
struct CertificationSpec {
    resize_control_precision: ResizeControlPrecisionState,
    proportion_persistence: ProportionPersistenceState,
    reset_restore: ResetRestoreState,
    resize_export: ResizeExportState,
    pane_never_pointer_only_resizable: bool,
    waiver: Option<PaneControlCertificationWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: precise pointer+keyboard resize, proportion-safe
    /// persistence, lossless reset/restore, reconstructable export.
    fn certified() -> Self {
        Self {
            resize_control_precision: ResizeControlPrecisionState::PrecisePointerAndKeyboardResize,
            proportion_persistence: ProportionPersistenceState::ProportionsOrPresetsPersisted,
            reset_restore: ResetRestoreState::DefaultResetAndTopologyRestore,
            resize_export: ResizeExportState::ProportionsAndActionsReconstructable,
            pane_never_pointer_only_resizable: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the profiler reduced-restore-fidelity waiver carried by the seed.
fn profiler_reduced_restore_waiver() -> PaneControlCertificationWaiver {
    PaneControlCertificationWaiver {
        waiver_id: "waiver:profiler-reduced-restore-fidelity:0001".to_owned(),
        layout: M5PaneLayout::Profiler,
        reason: "Under the seeded profiler capture, resize intent persists as proportions and \
                 resets to a named default, but a detached profiler window that loses its host \
                 monitor restores to a safe default layout rather than its exact prior ratios \
                 while the window host re-attaches. The fallback is disclosed, never destructive, \
                 and the pane proportions stay reconstructable from the support export."
            .to_owned(),
        owner_role: "Profiler surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed pane layout.
fn certification_spec(layout: M5PaneLayout) -> CertificationSpec {
    match layout {
        M5PaneLayout::Docs => CertificationSpec {
            resize_control_precision: ResizeControlPrecisionState::DisclosedReducedHitTargetOrStep,
            narrowing_reason: Some(
                "Under the seeded compact docs sheet the splitter's enlarged hit target shrinks \
                 to a disclosed narrower band and the keyboard step coarsens while both pointer \
                 and keyboard resize still resolve and the double-click default-size restore \
                 stays reachable; the reduction is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5PaneLayout::Review => CertificationSpec {
            proportion_persistence: ProportionPersistenceState::DisclosedReducedPersistenceFidelity,
            narrowing_reason: Some(
                "When the review layout moves from the expanded desktop to a compact sheet its \
                 diff/comment preset snaps to the nearest safe ratio rather than the exact prior \
                 proportion; the intent stays serialized as proportions rather than pixels, the \
                 reduction is disclosed, and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5PaneLayout::Profiler => CertificationSpec {
            reset_restore: ResetRestoreState::DisclosedReducedRestoreFidelity,
            waiver: Some(profiler_reduced_restore_waiver()),
            narrowing_reason: Some(
                "The profiler layout resets to its named default and persists proportions, but a \
                 detached profiler window that loses its host monitor restores to a safe default \
                 layout rather than its exact prior ratios while the window host re-attaches; the \
                 fallback is disclosed behind a waiver and never destructive, so the row is \
                 narrowed below green while the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5PaneLayout::Incident => CertificationSpec {
            resize_export: ResizeExportState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "The incident console's support export reconstructs current pane proportions and \
                 discloses a partial capture of the recent resize-action log while the high-volume \
                 log is still being trimmed; the partial capture is disclosed and the row is \
                 narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The two pane-control rows frozen by the shell-primitives matrix.
fn pane_matrix_rows() -> Vec<M5ShellPrimitiveRow> {
    let rows: Vec<M5ShellPrimitiveRow> = seeded_m5_shell_primitives_matrix()
        .primitive_rows
        .into_iter()
        .filter(|row| PANE_FAMILIES.contains(&row.primitive_family))
        .collect();
    assert_eq!(
        rows.len(),
        PANE_FAMILIES.len(),
        "frozen matrix declares both pane-control rows"
    );
    rows
}

/// The splitter-handle row is the canonical anchor for the shared bindings (qualification,
/// owner, shell zone) — the richest pane control, honouring the full pane-resize state
/// set.
fn anchor_row(matrix_rows: &[M5ShellPrimitiveRow]) -> &M5ShellPrimitiveRow {
    matrix_rows
        .iter()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::SplitterHandle)
        .expect("frozen matrix declares a splitter-handle row")
}

/// The most-narrowed qualification across the two pane rows, so a matrix narrowing of
/// either pane family is recorded on the certification row.
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
        .expect("at least one pane row")
}

/// The union of pane-resize states across the two pane rows, in canonical order.
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

/// The union of required labels across the two pane rows, in canonical order.
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

/// The union of accessibility routes across the two pane rows, in order.
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

/// The union of consumer surfaces across the two pane rows, ordered against the local
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

/// The union of downgrade triggers across the two pane rows, ordered against the frozen
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

/// Builds one certification row from the frozen pane matrix rows and a posture.
fn row_from_layout(
    layout: M5PaneLayout,
    matrix_rows: &[M5ShellPrimitiveRow],
    spec: CertificationSpec,
) -> PaneControlCertificationRow {
    let anchor = anchor_row(matrix_rows);
    let mut row = PaneControlCertificationRow {
        layout,
        driven_primitive_families: PANE_FAMILIES.to_vec(),
        matrix_qualification: worst_qualification(matrix_rows),
        owner_role: anchor.owner_role.clone(),
        layout_label: layout.label().to_owned(),
        shell_zone_slot: anchor.shell_zone_slot,
        certified_pane_resize_states: union_pane_resize_states(matrix_rows),
        accessibility_routes: union_accessibility_routes(matrix_rows),
        required_labels: union_required_labels(matrix_rows),
        consumer_surfaces: union_consumer_surfaces(matrix_rows),
        applicable_downgrade_triggers: union_downgrade_triggers(matrix_rows),
        resize_control_precision: spec.resize_control_precision,
        proportion_persistence: spec.proportion_persistence,
        reset_restore: spec.reset_restore,
        resize_export: spec.resize_export,
        pane_never_pointer_only_resizable: spec.pane_never_pointer_only_resizable,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: PaneControlCertificationStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per pane layout.
fn seeded_rows() -> Vec<PaneControlCertificationRow> {
    let matrix_rows = pane_matrix_rows();
    M5PaneLayout::ALL
        .iter()
        .map(|&layout| row_from_layout(layout, &matrix_rows, certification_spec(layout)))
        .collect()
}

/// Builds a variant where one layout's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5PaneLayout, mutate: F) -> Vec<PaneControlCertificationRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = pane_matrix_rows();
    M5PaneLayout::ALL
        .iter()
        .map(|&layout| {
            let mut spec = certification_spec(layout);
            if layout == target {
                mutate(&mut spec);
            }
            row_from_layout(layout, &matrix_rows, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<PaneControlCertificationRow>) -> PaneControlCertificationPacket {
    build_m5_pane_control_certification_packet(PaneControlCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 pane-control certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and
/// CSV artifacts. Two layouts are certified at full standing (green); the docs lane
/// auto-narrows to yellow behind a disclosed reduced hit target / keyboard step, the
/// review lane auto-narrows to yellow behind a disclosed reduced persistence fidelity,
/// the profiler lane auto-narrows to yellow behind a waivered reduced restore fidelity,
/// and the incident lane auto-narrows to yellow behind a disclosed partial support-export
/// capture — and no row is blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_pane_control_certification_packet() -> PaneControlCertificationPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the notebook lane's pane is pointer-only or brittle, proving a
/// non-precise resize control blocks the layout (red) rather than passing on behavior
/// alone.
pub fn seeded_m5_pane_control_certification_packet_notebook_pointer_only_resize_blocked(
) -> PaneControlCertificationPacket {
    let rows = seeded_rows_with(M5PaneLayout::Notebook, |spec| {
        spec.resize_control_precision = ResizeControlPrecisionState::PointerOnlyOrBrittleHitTarget;
        spec.narrowing_reason = Some(
            "The notebook cell/output splitter has no keyboard step-size route and its hit target \
             is a one-pixel band that cannot be reliably grabbed, so precise resize is \
             unavailable and the row blocks before keeping its resize-control-precision claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data lane persists resize intent only as brittle pixels,
/// proving a pixel-only persistence blocks the layout (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_pane_control_certification_packet_data_pixel_only_persistence_blocked(
) -> PaneControlCertificationPacket {
    let rows = seeded_rows_with(M5PaneLayout::Data, |spec| {
        spec.proportion_persistence = ProportionPersistenceState::BrittlePixelOnlyPersistence;
        spec.narrowing_reason = Some(
            "The data-grid layout serializes its splitter positions as absolute pixel offsets, so \
             moving the window to a compact sheet or a second monitor corrupts the layout, and the \
             row blocks before keeping its proportion-persistence claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the review lane's restore is lost or destructive, proving a
/// destructive restore blocks the layout (red) before the row can keep its reset/restore
/// claim.
pub fn seeded_m5_pane_control_certification_packet_review_restore_destructive_blocked(
) -> PaneControlCertificationPacket {
    let rows = seeded_rows_with(M5PaneLayout::Review, |spec| {
        spec.proportion_persistence = ProportionPersistenceState::ProportionsOrPresetsPersisted;
        spec.reset_restore = ResetRestoreState::RestoreLostOrDestructive;
        spec.narrowing_reason = Some(
            "After a crash the review layout restores with its comment pane collapsed to zero \
             width and no reopen path, so the restored layout is unusable and the row blocks \
             before keeping its reset/restore claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the docs lane's resize state is absent from the support-export
/// capture, proving a missing export blocks the layout (red) before the row can keep its
/// resize-state-export claim.
pub fn seeded_m5_pane_control_certification_packet_docs_resize_absent_from_capture_blocked(
) -> PaneControlCertificationPacket {
    let rows = seeded_rows_with(M5PaneLayout::Docs, |spec| {
        spec.resize_control_precision =
            ResizeControlPrecisionState::PrecisePointerAndKeyboardResize;
        spec.resize_export = ResizeExportState::ResizeStateAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The docs layout's support export omits current pane proportions and the recent \
             resize-action log entirely, so a layout bug cannot be explained without a screenshot, \
             and the row blocks before keeping its resize-state-export claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the incident lane has a pane resizable by pointer only, proving
/// the pointer-only invariant blocks the layout (red) before the row can keep its
/// pointer-only invariant.
pub fn seeded_m5_pane_control_certification_packet_incident_pointer_only_resizable_blocked(
) -> PaneControlCertificationPacket {
    let rows = seeded_rows_with(M5PaneLayout::Incident, |spec| {
        spec.resize_export = ResizeExportState::ProportionsAndActionsReconstructable;
        spec.pane_never_pointer_only_resizable = false;
        spec.narrowing_reason = Some(
            "The incident console's action pane can only be resized by dragging its splitter with \
             the pointer, with no keyboard step-size route, so its resize affordance is \
             pointer-only and the row blocks before keeping its pointer-only invariant.",
        );
    });
    packet_from_rows(rows)
}
