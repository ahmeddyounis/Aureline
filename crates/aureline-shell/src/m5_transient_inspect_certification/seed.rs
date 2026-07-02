//! Canonical seed builders for the M5 transient-inspect certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard,
//! support-export, and CSV artifacts plus the narrowed and blocked fixtures. The
//! headless emitter and the inline tests both call them so the in-code certification
//! proof, the artifacts, and the fixtures never drift. The transient-inspect
//! bindings — representation classes, promotion states, freshness labels,
//! accessibility routes, required labels, consumer surfaces, downgrade triggers,
//! qualification, owner, and shell zone — are pulled straight from the frozen
//! shell-primitives matrix's seeded tooltip, hovercard, peek-panel, and
//! pinned-preview rows (the union across the four transient families), so this proof
//! cannot certify a transient-inspect posture the matrix does not freeze.

use super::*;
use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix::{
    seeded_m5_shell_primitives_matrix, M5ShellPrimitiveRow, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID,
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

/// The four transient-inspect primitive families this lane certifies.
const TRANSIENT_FAMILIES: [M5ShellPrimitiveFamily; 4] = [
    M5ShellPrimitiveFamily::Tooltip,
    M5ShellPrimitiveFamily::Hovercard,
    M5ShellPrimitiveFamily::PeekPanel,
    M5ShellPrimitiveFamily::PinnedPreviewPromotion,
];

/// Canonical order of the ten shell consumer surfaces. `M5ShellConsumerSurface`
/// derives no `Ord` and exposes no `ALL`, so the union of consumer surfaces across
/// the four transient rows is ordered against this local list to stay deterministic.
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

/// The certification posture seeded for one governed inspect context.
struct CertificationSpec {
    representation_truth: RepresentationTruthState,
    promotion_continuity: PromotionContinuityState,
    non_hover_reach: NonHoverReachState,
    stale_preview_labeling: StalePreviewLabelingState,
    tooltip_never_sole_critical_instruction: bool,
    waiver: Option<TransientInspectCertificationWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: labeled representation, safe promotion, non-hover
    /// reach, labeled and reconstructable stale content.
    fn certified() -> Self {
        Self {
            representation_truth:
                RepresentationTruthState::IdentitySourceFreshnessRepresentationLabeled,
            promotion_continuity: PromotionContinuityState::PinOpenPathsPreserveIdentityAndState,
            non_hover_reach: NonHoverReachState::KeyboardFocusContextReachable,
            stale_preview_labeling: StalePreviewLabelingState::StaleLabeledAndExportReconstructable,
            tooltip_never_sole_critical_instruction: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the profiler reduced-promotion-path waiver carried by the seed.
fn profiler_reduced_promotion_waiver() -> TransientInspectCertificationWaiver {
    TransientInspectCertificationWaiver {
        waiver_id: "waiver:profiler-reduced-promotion-path:0001".to_owned(),
        context: M5InspectContext::Profiler,
        reason: "Under the seeded profiler capture the flame-graph peek can be pinned and opened \
                 to a full panel — both preserving its target identity, sampled-approximate \
                 freshness, and representation truth — but the detach-to-its-own-window promotion \
                 is deferred while the profiler window host stabilizes. The reduction is disclosed, \
                 never hidden, and the pinned preview stays reconstructable from the support \
                 export."
            .to_owned(),
        owner_role: "Profiler surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed inspect context.
fn certification_spec(context: M5InspectContext) -> CertificationSpec {
    match context {
        M5InspectContext::ReviewChange => CertificationSpec {
            representation_truth: RepresentationTruthState::DisclosedReducedRepresentationDetail,
            narrowing_reason: Some(
                "Under compact review width the change hovercard falls back to a disclosed, \
                 shorter representation of the diff hunk while the target identity, the \
                 provider-attributed source, and the freshness stay labeled; the reduction is \
                 disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5InspectContext::DataGrid => CertificationSpec {
            stale_preview_labeling: StalePreviewLabelingState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "The data-grid cell peek's support export reconstructs the visible preview and \
                 discloses a partial capture of the pinned cached-snapshot set while the API run \
                 re-fetches; the partial capture is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5InspectContext::Profiler => CertificationSpec {
            promotion_continuity: PromotionContinuityState::DisclosedReducedPromotionPath,
            waiver: Some(profiler_reduced_promotion_waiver()),
            narrowing_reason: Some(
                "The profiler flame-graph peek pins and opens while preserving identity, state, \
                 and sampled-approximate freshness, but its detach-to-window promotion path is \
                 disclosedly deferred behind a waiver while the profiler window host stabilizes; \
                 the row is narrowed below green while the reduction is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5InspectContext::Operator => CertificationSpec {
            non_hover_reach: NonHoverReachState::DisclosedReducedReachRoute,
            narrowing_reason: Some(
                "On the seeded compact operator console one non-hover reach route (the hover \
                 info affordance) is temporarily reduced while the console re-lays-out; keyboard \
                 focus and the explicit context action still resolve every tooltip and peek, and \
                 the reduction is disclosed, so the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The four transient-inspect rows frozen by the shell-primitives matrix.
fn transient_matrix_rows() -> Vec<M5ShellPrimitiveRow> {
    let rows: Vec<M5ShellPrimitiveRow> = seeded_m5_shell_primitives_matrix()
        .primitive_rows
        .into_iter()
        .filter(|row| TRANSIENT_FAMILIES.contains(&row.primitive_family))
        .collect();
    assert_eq!(
        rows.len(),
        TRANSIENT_FAMILIES.len(),
        "frozen matrix declares all four transient-inspect rows"
    );
    rows
}

/// The hovercard row is the canonical anchor for the shared bindings (qualification,
/// owner, shell zone) — the richest attributed transient inspect surface.
fn anchor_row(matrix_rows: &[M5ShellPrimitiveRow]) -> &M5ShellPrimitiveRow {
    matrix_rows
        .iter()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::Hovercard)
        .expect("frozen matrix declares a hovercard row")
}

/// The most-narrowed qualification across the four transient rows, so a matrix
/// narrowing of any transient family is recorded on the certification row.
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
        .expect("at least one transient row")
}

/// The union of representation classes across the four transient rows, in canonical
/// declaration order.
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

/// The union of promotion states across the four transient rows, in canonical order.
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

/// The union of source/freshness labels across the four transient rows, in order.
fn union_freshness_labels(matrix_rows: &[M5ShellPrimitiveRow]) -> Vec<M5SourceFreshnessLabel> {
    M5SourceFreshnessLabel::ALL
        .into_iter()
        .filter(|label| {
            matrix_rows
                .iter()
                .any(|row| row.source_freshness_labels.contains(label))
        })
        .collect()
}

/// The union of required labels across the four transient rows, in canonical order.
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

/// The union of accessibility routes across the four transient rows, in order.
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

/// The union of consumer surfaces across the four transient rows, ordered against the
/// local canonical consumer-surface list.
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

/// The union of downgrade triggers across the four transient rows, ordered against
/// the frozen trigger declaration order (`M5ShellPrimitiveDowngradeTrigger` derives no
/// `Ord`).
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

/// Builds one certification row from the frozen transient matrix rows and a posture.
fn row_from_context(
    context: M5InspectContext,
    matrix_rows: &[M5ShellPrimitiveRow],
    spec: CertificationSpec,
) -> TransientInspectCertificationRow {
    let anchor = anchor_row(matrix_rows);
    let mut row = TransientInspectCertificationRow {
        context,
        driven_primitive_families: TRANSIENT_FAMILIES.to_vec(),
        matrix_qualification: worst_qualification(matrix_rows),
        owner_role: anchor.owner_role.clone(),
        context_label: context.label().to_owned(),
        shell_zone_slot: anchor.shell_zone_slot,
        certified_representation_classes: union_representation_classes(matrix_rows),
        certified_promotion_states: union_promotion_states(matrix_rows),
        source_freshness_labels: union_freshness_labels(matrix_rows),
        accessibility_routes: union_accessibility_routes(matrix_rows),
        required_labels: union_required_labels(matrix_rows),
        consumer_surfaces: union_consumer_surfaces(matrix_rows),
        applicable_downgrade_triggers: union_downgrade_triggers(matrix_rows),
        representation_truth: spec.representation_truth,
        promotion_continuity: spec.promotion_continuity,
        non_hover_reach: spec.non_hover_reach,
        stale_preview_labeling: spec.stale_preview_labeling,
        tooltip_never_sole_critical_instruction: spec.tooltip_never_sole_critical_instruction,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: TransientInspectCertificationStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per inspect context.
fn seeded_rows() -> Vec<TransientInspectCertificationRow> {
    let matrix_rows = transient_matrix_rows();
    M5InspectContext::ALL
        .iter()
        .map(|&context| row_from_context(context, &matrix_rows, certification_spec(context)))
        .collect()
}

/// Builds a variant where one context's spec is mutated after the canonical spec is
/// resolved, used by the blocked fixtures.
fn seeded_rows_with<F>(target: M5InspectContext, mutate: F) -> Vec<TransientInspectCertificationRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = transient_matrix_rows();
    M5InspectContext::ALL
        .iter()
        .map(|&context| {
            let mut spec = certification_spec(context);
            if context == target {
                mutate(&mut spec);
            }
            row_from_context(context, &matrix_rows, spec)
        })
        .collect()
}

fn packet_from_rows(
    rows: Vec<TransientInspectCertificationRow>,
) -> TransientInspectCertificationPacket {
    build_m5_transient_inspect_certification_packet(TransientInspectCertificationInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 transient-inspect certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export,
/// and CSV artifacts. Three contexts are certified at full standing (green); the
/// review/change lane auto-narrows to yellow behind a disclosed reduced representation
/// detail, the data-grid lane auto-narrows to yellow behind a disclosed partial
/// support-export capture, the profiler lane auto-narrows to yellow behind a waivered
/// reduced promotion path, and the operator lane auto-narrows to yellow behind a
/// disclosed reduced non-hover reach route — and no row is blocked, so the packet is
/// clean and every row is publishable.
pub fn seeded_m5_transient_inspect_certification_packet() -> TransientInspectCertificationPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the search lane hides its source/provider or freshness
/// truth, proving hidden representation blocks promotion (red) rather than passing on
/// behavior alone.
pub fn seeded_m5_transient_inspect_certification_packet_search_representation_hidden_blocked(
) -> TransientInspectCertificationPacket {
    let rows = seeded_rows_with(M5InspectContext::SearchResults, |spec| {
        spec.representation_truth = RepresentationTruthState::SourceProviderOrFreshnessHidden;
        spec.narrowing_reason = Some(
            "The search-results hovercard hides the provider and freshness of a cached result \
             snippet, so a stale cached snippet reads as a live canonical result and the row \
             blocks before keeping its representation-truth claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the docs lane's peek promotion drops the target identity or
/// representation, proving a dropped promotion blocks promotion (red) rather than
/// staying a disclosed yellow.
pub fn seeded_m5_transient_inspect_certification_packet_docs_promotion_dropped_blocked(
) -> TransientInspectCertificationPacket {
    let rows = seeded_rows_with(M5InspectContext::DocsHelp, |spec| {
        spec.promotion_continuity =
            PromotionContinuityState::PromotionDropsIdentityOrRepresentation;
        spec.narrowing_reason = Some(
            "The docs/help peek panel promotes to a docked panel but drops the target symbol \
             identity and its provenance strip, so the promoted panel no longer names what it \
             previewed and the row blocks before keeping its promotion-continuity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the editor lane keeps information reachable only through
/// hover, proving hover-only information blocks promotion (red) before the row can
/// keep its non-hover-reach claim.
pub fn seeded_m5_transient_inspect_certification_packet_editor_hover_only_blocked(
) -> TransientInspectCertificationPacket {
    let rows = seeded_rows_with(M5InspectContext::Editor, |spec| {
        spec.non_hover_reach = NonHoverReachState::InformationHoverOrPointerOnly;
        spec.narrowing_reason = Some(
            "The editor symbol hovercard's signature detail is reachable only through pointer \
             hover, with no keyboard-focus, context-action, or info-affordance route, so the row \
             blocks before keeping its non-hover-reach claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the data-grid lane's stale preview reads as live / is absent
/// from capture, proving an unlabeled stale preview blocks promotion (red) before the
/// row can keep its stale-preview-labeling claim.
pub fn seeded_m5_transient_inspect_certification_packet_data_stale_reads_live_blocked(
) -> TransientInspectCertificationPacket {
    let rows = seeded_rows_with(M5InspectContext::DataGrid, |spec| {
        spec.stale_preview_labeling =
            StalePreviewLabelingState::StaleReadsAsLiveOrAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The data-grid cell peek shows a cached snapshot without a freshness label after \
             pinning, so the stale preview reads as live canonical data and the row blocks before \
             keeping its stale-preview-labeling claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the operator lane's tooltip carries the sole critical
/// instruction, proving a hover-only critical instruction blocks promotion (red)
/// before the row can keep its tooltip invariant.
pub fn seeded_m5_transient_inspect_certification_packet_operator_tooltip_sole_instruction_blocked(
) -> TransientInspectCertificationPacket {
    let rows = seeded_rows_with(M5InspectContext::Operator, |spec| {
        spec.non_hover_reach = NonHoverReachState::KeyboardFocusContextReachable;
        spec.tooltip_never_sole_critical_instruction = false;
        spec.narrowing_reason = Some(
            "The operator console's confirm-action tooltip carries the sole instruction for an \
             irreversible operation, so the instruction is reachable only through pointer hover \
             and the row blocks before keeping its tooltip invariant.",
        );
    });
    packet_from_rows(rows)
}
