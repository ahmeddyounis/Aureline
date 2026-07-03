//! Canonical seed builders for the M5 advisory-component release proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the narrowed and blocked fixtures. The headless emitter and the inline tests
//! both call them so the in-code certification proof, the artifacts, and the fixtures never drift.
//! The component bindings — severity classes, advisory anatomy, action and dismissal states,
//! continuity claims, delivery and freshness states, disclosure and export fields, notification
//! behaviors, projection surfaces, accessibility routes, required labels, shell zone, responsive
//! classes, window classes, surface families, consumer surfaces, downgrade triggers, owner role,
//! scope summary, and qualification — are pulled straight from the frozen advisory-component
//! matrix's seeded per-family component row, so this proof cannot certify a release posture the
//! matrix does not freeze.

use super::*;
use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    seeded_m5_advisory_component_matrix, M5AdvisoryComponentRow,
    M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-07-03T00:00:00Z";

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
    advisory_contract_truth: AdvisoryContractTruthState,
    cross_channel_parity: CrossChannelParityState,
    support_export_proof: SupportExportProofState,
    proof_freshness: ProofFreshnessState,
    never_hides_advisory_truth_off_channel: bool,
    waiver: Option<AdvisoryReleaseProofWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl CertificationSpec {
    /// A fully certified posture: advisory truth, cross-channel parity, reconstructable export,
    /// fresh proof.
    fn certified() -> Self {
        Self {
            advisory_contract_truth: AdvisoryContractTruthState::AdvisoryTruthCertifiedEveryChannel,
            cross_channel_parity: CrossChannelParityState::ParityCertifiedAcrossChannels,
            support_export_proof: SupportExportProofState::ReconstructableInExportAndScreenshot,
            proof_freshness: ProofFreshnessState::ExportedProofFreshAndCurrent,
            never_hides_advisory_truth_off_channel: true,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// Builds the affected-install reduced-channel-projection waiver carried by the seed.
fn affected_install_channel_projection_waiver() -> AdvisoryReleaseProofWaiver {
    AdvisoryReleaseProofWaiver {
        waiver_id: "waiver:reduced-channel-projection:0001".to_owned(),
        component_family: M5AdvisoryComponentFamily::AffectedInstallPanel,
        reason: "Under the seeded release the affected-install panel shows a disclosedly summarized \
                 projection on the most compact secondary channel (a collapsed exposure summary in \
                 place of the full per-lane install breakdown) while the shared row grammar, the \
                 severity vocabulary, the mirror-freshness state, and the local-continuity claim stay \
                 identical to the primary channel. The narrowing is disclosed, never hides an \
                 affected lane or the mirror-lag state, and keeps one row grammar."
            .to_owned(),
        owner_role: "Install/update component owner".to_owned(),
        expires_at: "2026-10-31T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded certification posture for one governed component family.
fn certification_spec(family: M5AdvisoryComponentFamily) -> CertificationSpec {
    match family {
        M5AdvisoryComponentFamily::AffectedInstallPanel => CertificationSpec {
            cross_channel_parity: CrossChannelParityState::DisclosedReducedChannelProjection,
            waiver: Some(affected_install_channel_projection_waiver()),
            narrowing_reason: Some(
                "The affected-install panel serves a disclosedly reduced channel projection on the \
                 most compact secondary channel (a collapsed exposure summary in place of the full \
                 per-lane install breakdown) while the shared row grammar, severity vocabulary, \
                 mirror-freshness state, and local-continuity claim stay identical to the primary \
                 channel; the reduction is disclosed behind a waiver, so the row is narrowed below \
                 green while it is in force.",
            ),
            ..CertificationSpec::certified()
        },
        M5AdvisoryComponentFamily::DisclosureBlock => CertificationSpec {
            support_export_proof: SupportExportProofState::DisclosedPartialCapture,
            narrowing_reason: Some(
                "Under the seeded release the disclosure/history block reconstructs its copy-safe \
                 advisory/CVE/GHSA ids, disclosure path, and resolved-versus-active history from the \
                 support export and screenshot baselines but discloses a partial capture of some \
                 low-priority provenance annotation while the export queue is throttled; the partial \
                 capture is disclosed and the row is narrowed below green.",
            ),
            ..CertificationSpec::certified()
        },
        M5AdvisoryComponentFamily::AdvisoryActivityRow => CertificationSpec {
            proof_freshness: ProofFreshnessState::DisclosedPartialRefresh,
            narrowing_reason: Some(
                "Under the seeded release the advisory activity row's exported proof is refreshed for \
                 every mandatory export field but discloses a partial refresh of a low-priority \
                 disclosure-visibility annotation that awaits the next scheduled refresh while the \
                 current claim stays backed; the partial refresh is disclosed and the row is narrowed \
                 below green.",
            ),
            ..CertificationSpec::certified()
        },
        _ => CertificationSpec::certified(),
    }
}

/// The six component rows frozen by the advisory-component matrix, keyed by family.
fn matrix_component_rows() -> Vec<M5AdvisoryComponentRow> {
    let rows = seeded_m5_advisory_component_matrix().component_rows;
    assert_eq!(
        rows.len(),
        M5AdvisoryComponentFamily::ALL.len(),
        "frozen matrix declares all six advisory components"
    );
    rows
}

/// Finds the frozen matrix row for `family`.
fn matrix_row_for(
    matrix_rows: &[M5AdvisoryComponentRow],
    family: M5AdvisoryComponentFamily,
) -> &M5AdvisoryComponentRow {
    matrix_rows
        .iter()
        .find(|row| row.component_family == family)
        .expect("frozen matrix declares every family")
}

/// Builds one certification row from the family's frozen matrix row and a posture.
fn row_from_family(
    family: M5AdvisoryComponentFamily,
    matrix_rows: &[M5AdvisoryComponentRow],
    spec: CertificationSpec,
) -> AdvisoryReleaseProofRow {
    let matrix_row = matrix_row_for(matrix_rows, family);
    let mut row = AdvisoryReleaseProofRow {
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
        certified_severity_classes: matrix_row.severity_classes.clone(),
        certified_projection_surfaces: matrix_row.projection_surfaces.clone(),
        certified_anatomy_fields: matrix_row.anatomy_fields.clone(),
        certified_action_states: matrix_row.action_states.clone(),
        certified_required_actions: matrix_row.required_actions.clone(),
        certified_dismissal_states: matrix_row.dismissal_states.clone(),
        certified_continuity_claims: matrix_row.continuity_claims.clone(),
        certified_delivery_profiles: matrix_row.delivery_profiles.clone(),
        certified_freshness_states: matrix_row.freshness_states.clone(),
        certified_disclosure_fields: matrix_row.disclosure_fields.clone(),
        certified_notification_behaviors: matrix_row.notification_behaviors.clone(),
        certified_export_fields: matrix_row.export_fields.clone(),
        accessibility_routes: matrix_row.accessibility_routes.clone(),
        required_labels: matrix_row.required_labels.clone(),
        consumer_surfaces: matrix_row.consumer_surfaces.clone(),
        applicable_downgrade_triggers: matrix_row.downgrade_triggers.clone(),
        advisory_contract_truth: spec.advisory_contract_truth,
        cross_channel_parity: spec.cross_channel_parity,
        support_export_proof: spec.support_export_proof,
        proof_freshness: spec.proof_freshness,
        never_hides_advisory_truth_off_channel: spec.never_hides_advisory_truth_off_channel,
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: AdvisoryReleaseProofStatus::Green,
        certification_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.certification_causes = row.recompute_causes();
    row
}

/// Builds the certification rows for the canonical seed, one per component family.
fn seeded_rows() -> Vec<AdvisoryReleaseProofRow> {
    let matrix_rows = matrix_component_rows();
    M5AdvisoryComponentFamily::ALL
        .iter()
        .map(|&family| row_from_family(family, &matrix_rows, certification_spec(family)))
        .collect()
}

/// Builds a variant where one family's spec is mutated after the canonical spec is resolved, used by
/// the blocked fixtures.
fn seeded_rows_with<F>(target: M5AdvisoryComponentFamily, mutate: F) -> Vec<AdvisoryReleaseProofRow>
where
    F: Fn(&mut CertificationSpec),
{
    let matrix_rows = matrix_component_rows();
    M5AdvisoryComponentFamily::ALL
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

fn packet_from_rows(rows: Vec<AdvisoryReleaseProofRow>) -> AdvisoryReleaseProofPacket {
    build_m5_advisory_component_release_proof_packet(AdvisoryReleaseProofInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 advisory-component release-proof packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. Three families are certified at full standing (green); the affected-install panel
/// auto-narrows to yellow behind a waivered reduced cross-channel projection, the disclosure/history
/// block auto-narrows to yellow behind a disclosed partial support-export capture, and the advisory
/// activity row auto-narrows to yellow behind a disclosed partial proof refresh — and no row is
/// blocked, so the packet is clean and every row is publishable.
pub fn seeded_m5_advisory_component_release_proof_packet() -> AdvisoryReleaseProofPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the advisory-card family's advisory truth collapses or drifts on a claimed
/// channel, proving a collapsed advisory truth blocks the family (red) rather than staying a
/// disclosed yellow.
pub fn seeded_m5_advisory_component_release_proof_packet_advisory_card_contract_truth_collapsed_blocked(
) -> AdvisoryReleaseProofPacket {
    let rows = seeded_rows_with(M5AdvisoryComponentFamily::AdvisoryCard, |spec| {
        spec.advisory_contract_truth = AdvisoryContractTruthState::AdvisoryTruthCollapsedOrDrifted;
        spec.narrowing_reason = Some(
            "On a claimed update-center channel the advisory card collapses its affected object, \
             severity, and fixed-version anatomy into a generic 'an update is available' banner, so \
             the affected-scope-and-exposure advisory truth collapses, and the family blocks before \
             keeping its advisory-contract-truth claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the affected-install family reinvents a second row grammar off the primary
/// channel, proving a diverged row grammar blocks the family (red) before it can keep its
/// cross-channel-parity claim.
pub fn seeded_m5_advisory_component_release_proof_packet_affected_install_channel_diverged_blocked(
) -> AdvisoryReleaseProofPacket {
    let rows = seeded_rows_with(M5AdvisoryComponentFamily::AffectedInstallPanel, |spec| {
        spec.cross_channel_parity =
            CrossChannelParityState::ChannelGrammarDivergedOffPrimaryChannel;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "Off the primary channel the marketplace surface reinvents a flat 'update recommended' \
             note instead of the affected-install panel's per-lane exposure and mirror-freshness \
             breakdown, so the same advisory reads with a second row grammar, and the family blocks \
             before keeping its cross-channel-parity claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the disclosure-block family's advisory truth is absent from the
/// support-export capture, proving a missing export blocks the family (red) before it can keep its
/// support-export-proof claim.
pub fn seeded_m5_advisory_component_release_proof_packet_disclosure_block_capture_absent_blocked(
) -> AdvisoryReleaseProofPacket {
    let rows = seeded_rows_with(M5AdvisoryComponentFamily::DisclosureBlock, |spec| {
        spec.support_export_proof = SupportExportProofState::AdvisoryTruthAbsentFromCapture;
        spec.narrowing_reason = Some(
            "The support export omits the disclosure/history block's copy-safe advisory/CVE/GHSA ids \
             and disclosure path entirely, so a disclosure regression cannot be explained without a \
             live screenshot, and the family blocks before keeping its support-export-proof claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the advisory-activity-row family's exported proof is stale or divergent,
/// proving a stale proof blocks the family (red) rather than staying a disclosed yellow.
pub fn seeded_m5_advisory_component_release_proof_packet_advisory_activity_row_proof_stale_blocked(
) -> AdvisoryReleaseProofPacket {
    let rows = seeded_rows_with(M5AdvisoryComponentFamily::AdvisoryActivityRow, |spec| {
        spec.proof_freshness = ProofFreshnessState::ExportedProofStaleOrDivergent;
        spec.narrowing_reason = Some(
            "The advisory activity row's exported proof diverged from the current export-field \
             contract after a mitigation-state change and was never refreshed, so the claim is no \
             longer backed by a current proof, and the family blocks before keeping its \
             proof-freshness claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the emergency-notice family hides advisory truth off the primary channel,
/// proving the invariant blocks the family (red) before it can keep its invariant.
pub fn seeded_m5_advisory_component_release_proof_packet_emergency_notice_advisory_truth_dropped_blocked(
) -> AdvisoryReleaseProofPacket {
    let rows = seeded_rows_with(M5AdvisoryComponentFamily::EmergencyNotice, |spec| {
        spec.never_hides_advisory_truth_off_channel = false;
        spec.narrowing_reason = Some(
            "Off the primary channel the emergency notice drops its forced-disable scope and \
             blast-radius statement, so its emergency advisory truth cannot be reconstructed off the \
             channel that rendered it, and the family blocks before keeping its \
             no-hidden-advisory-truth invariant.",
        );
    });
    packet_from_rows(rows)
}
