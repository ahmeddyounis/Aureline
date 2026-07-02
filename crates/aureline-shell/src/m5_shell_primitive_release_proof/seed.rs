//! Canonical seed builders for the M5 shell-primitive release proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export,
//! and CSV artifacts plus the narrowed and blocked fixtures. The headless emitter and the inline
//! tests both call them so the in-code certification proof, the artifacts, and the fixtures never
//! drift. The primitive bindings — status-item classes, overflow behaviors, representation
//! classes, promotion states, pane-resize states, progress states, source/provider/freshness
//! labels, accessibility routes, required labels, shell zone, consumer surfaces, downgrade
//! triggers, owner role, scope summary, and qualification — are pulled straight from the frozen
//! shell-primitives matrix's seeded primitive row for that family, so this proof cannot certify a
//! primitive the matrix does not freeze.

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

/// The certification posture seeded for one governed shell primitive.
struct CertificationSpec {
    primitive_truth: PrimitiveTruthState,
    representation_freshness: RepresentationFreshnessState,
    interaction_reach: InteractionReachState,
    exported_proof_parity: ExportedProofParityState,
    never_hover_spinner_or_pointer_only: bool,
    waiver: Option<ShellPrimitiveReleaseWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: primitive truth current, representation/freshness preserved,
    /// keyboard/touch reach and precise resize, exported proof current.
    fn certified() -> Self {
        Self {
            primitive_truth: PrimitiveTruthState::PrimitiveTruthCertifiedAndCurrent,
            representation_freshness:
                RepresentationFreshnessState::SourceFreshnessRepresentationPreserved,
            interaction_reach: InteractionReachState::KeyboardTouchReachAndResizeCertified,
            exported_proof_parity: ExportedProofParityState::ExportedSurfacesReflectCurrentProof,
            never_hover_spinner_or_pointer_only: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the pane-resize-preset reduced-reach waiver carried by the seed.
fn pane_resize_reduced_reach_waiver() -> ShellPrimitiveReleaseWaiver {
    ShellPrimitiveReleaseWaiver {
        waiver_id: "waiver:pane-resize-reduced-reach:0001".to_owned(),
        primitive_family: M5ShellPrimitiveFamily::PaneResizePreset,
        reason: "Under the seeded release proof the pane-resize preset serves a coarser keyboard \
                 resize step on the compact profile while every preset stays keyboard-invokable, \
                 serializable, and precise on the standard profile, and no resize is pointer-only. \
                 The reduced reach is disclosed and reversible; the narrowing is disclosed, never \
                 hides a state, and keeps the keyboard/touch route."
            .to_owned(),
        owner_role: "Shell/layout owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed shell primitive.
fn certification_spec(family: M5ShellPrimitiveFamily) -> CertificationSpec {
    match family {
        M5ShellPrimitiveFamily::Hovercard => CertificationSpec {
            exported_proof_parity: ExportedProofParityState::DisclosedPartialExportRefresh,
            narrowing_reason: Some(
                "The hovercard's exported proof reflects the current provenance/representation \
                 state and discloses a partial refresh of a low-priority decorative-attribution \
                 detail while the export queue is throttled; the partial refresh is disclosed and \
                 the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5ShellPrimitiveFamily::PinnedPreviewPromotion => CertificationSpec {
            representation_freshness: RepresentationFreshnessState::DisclosedPartialRepresentation,
            narrowing_reason: Some(
                "Across a promotion the pinned-preview promotion trims a low-priority provenance \
                 strip detail to a shorter form while the source, freshness, and representation \
                 truth stay preserved and no stale preview reads as live; the reduction is \
                 disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5ShellPrimitiveFamily::PaneResizePreset => CertificationSpec {
            interaction_reach: InteractionReachState::DisclosedReducedReachOrResize,
            waiver: Some(pane_resize_reduced_reach_waiver()),
            narrowing_reason: Some(
                "The pane-resize preset serves a coarser keyboard resize step on the compact \
                 profile while every preset stays keyboard-invokable, serializable, and precise and \
                 no resize is pointer-only; the reduced reach is disclosed behind a waiver, so the \
                 row is narrowed below green while the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5ShellPrimitiveFamily::ProgressIndicator => CertificationSpec {
            primitive_truth: PrimitiveTruthState::DisclosedReducedTruthScope,
            narrowing_reason: Some(
                "The progress indicator presents a grouped batch summary in place of per-item \
                 progress for a small set of high-frequency jobs while every job's primary state \
                 stays current, named, and reopenable into durable history; the reduced scope is \
                 disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The primitive rows frozen by the shell-primitives matrix.
fn matrix_primitive_rows() -> Vec<M5ShellPrimitiveRow> {
    let rows = seeded_m5_shell_primitives_matrix().primitive_rows;
    assert_eq!(
        rows.len(),
        M5ShellPrimitiveFamily::ALL.len(),
        "frozen matrix declares all ten shell primitives"
    );
    rows
}

/// Returns the frozen matrix row for `family`.
fn matrix_row(
    matrix_rows: &[M5ShellPrimitiveRow],
    family: M5ShellPrimitiveFamily,
) -> &M5ShellPrimitiveRow {
    matrix_rows
        .iter()
        .find(|row| row.primitive_family == family)
        .unwrap_or_else(|| panic!("frozen matrix declares the {} primitive", family.as_str()))
}

/// Short reviewer-facing label for a governed primitive family.
fn family_label(family: M5ShellPrimitiveFamily) -> &'static str {
    match family {
        M5ShellPrimitiveFamily::StatusBarItem => "Status-bar item",
        M5ShellPrimitiveFamily::StatusOverflowMenu => "Status overflow menu",
        M5ShellPrimitiveFamily::Tooltip => "Tooltip",
        M5ShellPrimitiveFamily::Hovercard => "Hovercard",
        M5ShellPrimitiveFamily::PeekPanel => "Peek panel",
        M5ShellPrimitiveFamily::PinnedPreviewPromotion => "Pinned-preview promotion",
        M5ShellPrimitiveFamily::SplitterHandle => "Splitter handle",
        M5ShellPrimitiveFamily::PaneResizePreset => "Pane-resize preset",
        M5ShellPrimitiveFamily::ProgressIndicator => "Progress indicator",
        M5ShellPrimitiveFamily::DurableJobRow => "Durable job row",
    }
}

/// Builds one certification row from the frozen matrix row and a posture.
fn row_from_family(
    family: M5ShellPrimitiveFamily,
    matrix_rows: &[M5ShellPrimitiveRow],
    spec: CertificationSpec,
) -> ShellPrimitiveReleaseRow {
    let source = matrix_row(matrix_rows, family);
    let mut row = ShellPrimitiveReleaseRow {
        primitive_family: family,
        truth_pillar: M5ShellPrimitiveTruthPillar::from_family(family),
        matrix_qualification: source.qualification,
        owner_role: source.owner_role.clone(),
        primitive_label: family_label(family).to_owned(),
        scope_summary: source.scope_summary.clone(),
        shell_zone_slot: source.shell_zone_slot,
        responsive_classes: source.responsive_classes.clone(),
        window_classes: source.window_classes.clone(),
        surface_families: source.surface_families.clone(),
        certified_profiles: M5ShellReleaseProfile::ALL.to_vec(),
        certified_status_item_classes: source.status_item_classes.clone(),
        certified_overflow_behaviors: source.overflow_behaviors.clone(),
        certified_representation_classes: source.representation_classes.clone(),
        certified_promotion_states: source.promotion_states.clone(),
        certified_pane_resize_states: source.pane_resize_states.clone(),
        certified_progress_states: source.progress_states.clone(),
        certified_source_freshness_labels: source.source_freshness_labels.clone(),
        accessibility_routes: source.accessibility_routes.clone(),
        required_labels: source.required_labels.clone(),
        consumer_surfaces: source.consumer_surfaces.clone(),
        applicable_downgrade_triggers: source.downgrade_triggers.clone(),
        primitive_truth: spec.primitive_truth,
        representation_freshness: spec.representation_freshness,
        interaction_reach: spec.interaction_reach,
        exported_proof_parity: spec.exported_proof_parity,
        never_hover_spinner_or_pointer_only: spec.never_hover_spinner_or_pointer_only,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: ShellPrimitiveReleaseStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per primitive family.
fn seeded_rows() -> Vec<ShellPrimitiveReleaseRow> {
    let matrix_rows = matrix_primitive_rows();
    M5ShellPrimitiveFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, &matrix_rows, certification_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used
/// by the blocked fixtures.
fn seeded_rows_with<F>(target: M5ShellPrimitiveFamily, mutate: F) -> Vec<ShellPrimitiveReleaseRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = matrix_primitive_rows();
    M5ShellPrimitiveFamily::ALL
        .iter()
        .map(|&family| {
            let mut spec = certification_spec(family);
            if family == target {
                mutate(&mut spec);
            }
            row_from_family(family, &matrix_rows, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<ShellPrimitiveReleaseRow>) -> ShellPrimitiveReleasePacket {
    build_m5_shell_primitive_release_proof_packet(ShellPrimitiveReleaseInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 shell-primitive release-proof packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Six primitives are certified at full standing (green); the hovercard auto-narrows to
/// yellow behind a disclosed partial export refresh, the pinned-preview promotion auto-narrows to
/// yellow behind a disclosed partial representation, the pane-resize preset auto-narrows to yellow
/// behind a waivered reduced interaction reach, and the progress indicator auto-narrows to yellow
/// behind a disclosed reduced truth scope — and no row is blocked, so the packet is clean and every
/// row is publishable.
pub fn seeded_m5_shell_primitive_release_proof_packet() -> ShellPrimitiveReleasePacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the status-bar item's typed state truth collapses into a spinner,
/// proving a collapsed primitive truth blocks the primitive (red) rather than passing on behavior
/// alone.
pub fn seeded_m5_shell_primitive_release_proof_packet_status_bar_truth_collapsed_blocked(
) -> ShellPrimitiveReleasePacket {
    let rows = seeded_rows_with(M5ShellPrimitiveFamily::StatusBarItem, |spec| {
        spec.primitive_truth = PrimitiveTruthState::PrimitiveTruthCollapsedOrLost;
        spec.narrowing_reason = Some(
            "The status-bar item's background-work and sync-freshness truth collapses into a bare \
             spinner with no named state under load, so a truth-bearing item is lost, and the row \
             blocks before keeping its primitive-truth claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the hovercard hides its source/freshness so a stale preview reads as
/// live, proving a hidden source/freshness blocks the primitive (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_shell_primitive_release_proof_packet_hovercard_source_freshness_hidden_blocked(
) -> ShellPrimitiveReleasePacket {
    let rows = seeded_rows_with(M5ShellPrimitiveFamily::Hovercard, |spec| {
        spec.representation_freshness =
            RepresentationFreshnessState::SourceOrFreshnessHiddenOrStale;
        spec.narrowing_reason = Some(
            "The hovercard drops its provenance strip after a refresh so a cached snapshot reads as \
             live canonical content with no source or freshness label, and the row blocks before \
             keeping its representation/freshness claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the splitter handle can only be resized by pointer, proving a
/// pointer-only resize blocks the primitive (red) before the row can keep its interaction-reach
/// claim.
pub fn seeded_m5_shell_primitive_release_proof_packet_splitter_pointer_only_resize_blocked(
) -> ShellPrimitiveReleasePacket {
    let rows = seeded_rows_with(M5ShellPrimitiveFamily::SplitterHandle, |spec| {
        spec.interaction_reach = InteractionReachState::PointerOrHoverOnlyOrBrittleResize;
        spec.narrowing_reason = Some(
            "The splitter handle exposes no keyboard step and no serialized ratio, so a pane can \
             only be resized by pointer drag and the resize is not restorable, and the row blocks \
             before keeping its interaction-reach claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the durable job row's exported proof is stale/divergent, proving a stale
/// export blocks the primitive (red) before the row can keep its exported-proof-parity claim.
pub fn seeded_m5_shell_primitive_release_proof_packet_job_row_exported_proof_stale_blocked(
) -> ShellPrimitiveReleasePacket {
    let rows = seeded_rows_with(M5ShellPrimitiveFamily::DurableJobRow, |spec| {
        spec.exported_proof_parity = ExportedProofParityState::ExportedProofStaleOrDivergent;
        spec.narrowing_reason = Some(
            "The durable job row's exported release proof still shows a running state for jobs that \
             have since failed, so the export is divergent from the current state and a regression \
             cannot be explained without a live screenshot, and the row blocks before keeping its \
             exported-proof-parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the progress indicator keeps a critical progress hover-/spinner-only,
/// proving the hover/spinner/pointer-only invariant blocks the primitive (red) before the row can
/// keep its invariant.
pub fn seeded_m5_shell_primitive_release_proof_packet_progress_hover_spinner_only_blocked(
) -> ShellPrimitiveReleasePacket {
    let rows = seeded_rows_with(M5ShellPrimitiveFamily::ProgressIndicator, |spec| {
        spec.never_hover_spinner_or_pointer_only = false;
        spec.narrowing_reason = Some(
            "The progress indicator keeps a sync job's critical progress visible only through a \
             transient spinner with no durable text or reopen path, so a critical truth is kept \
             spinner-only, and the row blocks before keeping its hover/spinner/pointer-only \
             invariant.",
        );
    });
    packet_from_rows(rows)
}
