//! Canonical seed builders for the M5 trust-component release proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed and blocked fixtures. The headless emitter and the inline tests
//! both call them so the in-code certification proof, the artifacts, and the fixtures never drift.
//! The component bindings — settings-row states, source pills, consequence classes, scope states,
//! chronology verbs, provenance badges, detail states, export fields, accessibility routes, required
//! labels, shell zone, responsive classes, window classes, surface families, consumer surfaces,
//! downgrade triggers, owner role, scope summary, and qualification — are pulled straight from the
//! frozen trust-chronology component matrix's seeded per-family component row, so this proof cannot
//! certify a release posture the matrix does not freeze.

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

/// The certification posture seeded for one governed component family.
struct CertificationSpec {
    component_contract_truth: ComponentContractTruthState,
    cross_surface_parity: CrossSurfaceParityState,
    support_export_proof: SupportExportProofState,
    proof_freshness: ProofFreshnessState,
    never_drops_audit_or_support_truth: bool,
    waiver: Option<TrustReleaseProofWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: contract truth, cross-surface parity, reconstructable export, fresh
    /// proof.
    fn certified() -> Self {
        Self {
            component_contract_truth:
                ComponentContractTruthState::ContractTruthCertifiedEverySurface,
            cross_surface_parity: CrossSurfaceParityState::ParityCertifiedAcrossSurfaces,
            support_export_proof: SupportExportProofState::ReconstructableInExportAndScreenshot,
            proof_freshness: ProofFreshnessState::ExportedProofFreshAndCurrent,
            never_drops_audit_or_support_truth: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the timeline-group reduced-surface-projection waiver carried by the seed.
fn timeline_group_surface_projection_waiver() -> TrustReleaseProofWaiver {
    TrustReleaseProofWaiver {
        waiver_id: "waiver:reduced-surface-projection:0001".to_owned(),
        component_family: M5TrustComponentFamily::TimelineGroup,
        reason: "Under the seeded release the timeline group shows a disclosedly summarized \
                 projection on the most compact secondary surface (a collapsed grouping heading in \
                 place of the full grouped detail) while the shared row grammar, the stable verbs and \
                 provenance badges, and the reopen path stay identical to the primary surface. The \
                 narrowing is disclosed, never hides a grouped event, and keeps one row grammar."
            .to_owned(),
        owner_role: "Activity/evidence component owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed component family.
fn certification_spec(family: M5TrustComponentFamily) -> CertificationSpec {
    match family {
        M5TrustComponentFamily::TimelineGroup => CertificationSpec {
            cross_surface_parity: CrossSurfaceParityState::DisclosedReducedSurfaceProjection,
            waiver: Some(timeline_group_surface_projection_waiver()),
            narrowing_reason: Some(
                "The timeline group serves a disclosedly reduced surface projection on the most \
                 compact secondary surface (a collapsed grouping heading in place of the full grouped \
                 detail) while the shared row grammar, stable verbs, provenance badges, and reopen \
                 path stay identical to the primary surface; the reduction is disclosed behind a \
                 waiver, so the row is narrowed below green while it is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5TrustComponentFamily::NarrativeSummaryCard => CertificationSpec {
            support_export_proof: SupportExportProofState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "Under the seeded release the narrative summary card reconstructs its verb / \
                 provenance / reopen truth from the support export and screenshot baselines but \
                 discloses a partial capture of some low-priority prose-summary phrasing while the \
                 export queue is throttled; the partial capture is disclosed and the row is narrowed \
                 below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5TrustComponentFamily::ChronologyExportPreview => CertificationSpec {
            proof_freshness: ProofFreshnessState::DisclosedPartialRefresh,
            narrowing_reason: Some(
                "Under the seeded release the chronology export preview's exported proof is refreshed \
                 for every mandatory export field but discloses a partial refresh of a low-priority \
                 redaction-class annotation that awaits the next scheduled refresh while the current \
                 claim stays backed; the partial refresh is disclosed and the row is narrowed below \
                 green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The six component rows frozen by the trust-chronology component matrix, keyed by family.
fn matrix_component_rows() -> Vec<M5TrustComponentRow> {
    let rows = seeded_m5_trust_chronology_component_matrix().component_rows;
    assert_eq!(
        rows.len(),
        M5TrustComponentFamily::ALL.len(),
        "frozen matrix declares all six trust components"
    );
    rows
}

/// Finds the frozen matrix row for `family`.
fn matrix_row_for(
    matrix_rows: &[M5TrustComponentRow],
    family: M5TrustComponentFamily,
) -> &M5TrustComponentRow {
    matrix_rows
        .iter()
        .find(|row| row.component_family == family)
        .expect("frozen matrix declares every family")
}

/// Builds one certification row from the family's frozen matrix row and a posture.
fn row_from_family(
    family: M5TrustComponentFamily,
    matrix_rows: &[M5TrustComponentRow],
    spec: CertificationSpec,
) -> TrustReleaseProofRow {
    let matrix_row = matrix_row_for(matrix_rows, family);
    let mut row = TrustReleaseProofRow {
        component_family: family,
        matrix_qualification: matrix_row.qualification,
        owner_role: matrix_row.owner_role.clone(),
        family_label: component_family_label(family).to_owned(),
        scope_summary: matrix_row.scope_summary.clone(),
        certified_truth_pillars: applicable_truth_pillars(family),
        shell_zone_slot: matrix_row.shell_zone_slot,
        certified_responsive_classes: matrix_row.responsive_classes.clone(),
        certified_window_classes: matrix_row.window_classes.clone(),
        certified_surface_families: matrix_row.surface_families.clone(),
        certified_settings_row_states: matrix_row.settings_row_states.clone(),
        certified_source_pills: matrix_row.source_pills.clone(),
        certified_consequence_classes: matrix_row.consequence_classes.clone(),
        certified_capability_scope_states: matrix_row.capability_scope_states.clone(),
        certified_chronology_verbs: matrix_row.chronology_verbs.clone(),
        certified_provenance_badges: matrix_row.provenance_badges.clone(),
        certified_chronology_detail_states: matrix_row.chronology_detail_states.clone(),
        certified_chronology_export_fields: matrix_row.chronology_export_fields.clone(),
        accessibility_routes: matrix_row.accessibility_routes.clone(),
        required_labels: matrix_row.required_labels.clone(),
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        component_contract_truth: spec.component_contract_truth,
        cross_surface_parity: spec.cross_surface_parity,
        support_export_proof: spec.support_export_proof,
        proof_freshness: spec.proof_freshness,
        never_drops_audit_or_support_truth: spec.never_drops_audit_or_support_truth,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: TrustReleaseProofStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per component family.
fn seeded_rows() -> Vec<TrustReleaseProofRow> {
    let matrix_rows = matrix_component_rows();
    M5TrustComponentFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, &matrix_rows, certification_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by
/// the blocked fixtures.
fn seeded_rows_with<F>(target: M5TrustComponentFamily, mutate: F) -> Vec<TrustReleaseProofRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = matrix_component_rows();
    M5TrustComponentFamily::ALL
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

fn packet_from_rows(rows: Vec<TrustReleaseProofRow>) -> TrustReleaseProofPacket {
    build_m5_trust_component_release_proof_packet(TrustReleaseProofInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_TRUST_COMPONENTS_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 trust-component release-proof packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Three families are certified at full standing (green); the timeline group auto-narrows
/// to yellow behind a waivered reduced cross-surface projection, the narrative summary card
/// auto-narrows to yellow behind a disclosed partial support-export capture, and the chronology
/// export preview auto-narrows to yellow behind a disclosed partial proof refresh — and no row is
/// blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_trust_component_release_proof_packet() -> TrustReleaseProofPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the settings-row family's contract truth collapses or drifts on a claimed
/// surface, proving a collapsed contract truth blocks the family (red) rather than staying a disclosed
/// yellow.
pub fn seeded_m5_trust_component_release_proof_packet_settings_row_contract_truth_collapsed_blocked(
) -> TrustReleaseProofPacket {
    let rows = seeded_rows_with(M5TrustComponentFamily::SettingsRow, |spec| {
        spec.component_contract_truth =
            ComponentContractTruthState::ContractTruthCollapsedOrDrifted;
        spec.narrowing_reason = Some(
            "On a claimed config surface the settings row conflates its effective and configured \
             value and drops the source pill, so the effective-versus-configured truth collapses into \
             a bare value, and the family blocks before keeping its contract-truth claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the capability-sheet family reinvents a second row grammar off the primary
/// surface, proving a diverged row grammar blocks the family (red) before it can keep its
/// cross-surface-parity claim.
pub fn seeded_m5_trust_component_release_proof_packet_capability_sheet_row_grammar_diverged_blocked(
) -> TrustReleaseProofPacket {
    let rows = seeded_rows_with(M5TrustComponentFamily::CapabilitySheet, |spec| {
        spec.cross_surface_parity = CrossSurfaceParityState::RowGrammarDivergedOffPrimarySurface;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "Off the primary surface a notification-envelope capability prompt reinvents a flat \
             permission list instead of the consequence-grouped sheet, so the same request reads with \
             a second row grammar, and the family blocks before keeping its cross-surface-parity \
             claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the event/history-row family's component truth is absent from the
/// support-export capture, proving a missing export blocks the family (red) before it can keep its
/// support-export-proof claim.
pub fn seeded_m5_trust_component_release_proof_packet_event_history_row_capture_absent_blocked(
) -> TrustReleaseProofPacket {
    let rows = seeded_rows_with(M5TrustComponentFamily::EventHistoryRow, |spec| {
        spec.support_export_proof = SupportExportProofState::ComponentTruthAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The support export omits the event/history row's verb and provenance badge entirely, so \
             an activity regression cannot be explained without a live screenshot, and the family \
             blocks before keeping its support-export-proof claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the timeline-group family's exported proof is stale or divergent, proving a
/// stale proof blocks the family (red) rather than staying a disclosed yellow.
pub fn seeded_m5_trust_component_release_proof_packet_timeline_group_proof_stale_blocked(
) -> TrustReleaseProofPacket {
    let rows = seeded_rows_with(M5TrustComponentFamily::TimelineGroup, |spec| {
        spec.cross_surface_parity = CrossSurfaceParityState::ParityCertifiedAcrossSurfaces;
        spec.waiver = None;
        spec.proof_freshness = ProofFreshnessState::ExportedProofStaleOrDivergent;
        spec.narrowing_reason = Some(
            "The timeline group's exported proof diverged from the current grouping contract after a \
             detail-state change and was never refreshed, so the claim is no longer backed by a \
             current proof, and the family blocks before keeping its proof-freshness claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the narrative-summary-card family drops audit / support truth off the
/// primary surface, proving the invariant blocks the family (red) before it can keep its invariant.
pub fn seeded_m5_trust_component_release_proof_packet_narrative_summary_card_audit_truth_dropped_blocked(
) -> TrustReleaseProofPacket {
    let rows = seeded_rows_with(M5TrustComponentFamily::NarrativeSummaryCard, |spec| {
        spec.support_export_proof = SupportExportProofState::ReconstructableInExportAndScreenshot;
        spec.never_drops_audit_or_support_truth = false;
        spec.narrowing_reason = Some(
            "Off the primary surface the narrative summary card drops its reopen path back into the \
             underlying events, so its chronology truth cannot be reconstructed off the surface that \
             rendered it, and the family blocks before keeping its no-dropped-audit-truth invariant.",
        );
    });
    packet_from_rows(rows)
}
