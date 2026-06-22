use super::*;

const F_PROVIDER: &str = "family:provider-connect:0001";
const F_REQUEST: &str = "family:request-workspace:0001";
const F_SETTINGS: &str = "family:settings-config-editor:0001";

fn seeded() -> M5FormFamilyCertificationSetPacket {
    seeded_m5_form_family_certification_set()
}

fn family<'a>(packet: &'a M5FormFamilyCertificationSetPacket, id: &str) -> &'a FamilyRecord {
    packet
        .families
        .iter()
        .find(|f| f.family_id == id)
        .unwrap_or_else(|| panic!("missing family {id}"))
}

fn cloned(packet: &M5FormFamilyCertificationSetPacket, id: &str) -> FamilyRecord {
    family(packet, id).clone()
}

/// A faithful consumer renders the effective tier, so a narrowing test lowers the rendered
/// tier to match — otherwise the family itself overclaims and floors to withdrawn.
fn render_all(f: &mut FamilyRecord, tier: QualificationTier) {
    f.renderings.iter_mut().for_each(|r| r.rendered_tier = tier);
}

fn set_state(
    f: &mut FamilyRecord,
    dimension: ProofDimension,
    lane: ProofLane,
    state: EvidenceState,
) {
    let cell = f
        .evidence
        .iter_mut()
        .find(|c| c.dimension == dimension && c.source_lane == lane)
        .expect("cell present");
    cell.state = state;
    if state.has_capture() {
        cell.captured_at = Some(SEED_STALE_CAPTURED_AT.to_owned());
        cell.proof_ref = Some(lane.support_export_ref().to_owned());
    } else {
        cell.captured_at = None;
        cell.proof_ref = None;
    }
}

fn decide(f: &FamilyRecord) -> FamilyDecision {
    f.narrow(false)
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded();
    assert_eq!(packet.record_kind, M5_FORM_FAMILY_CERTIFICATION_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_FORM_FAMILY_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.taxonomy_version,
        M5_FORM_FAMILY_CERTIFICATION_TAXONOMY_VERSION
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.families.len(), 7);
}

#[test]
fn checked_in_export_matches_seed() {
    let canonical = current_m5_form_family_certification_set()
        .expect("canonical form-family certification set loads and validates");
    assert_eq!(
        canonical,
        seeded(),
        "checked-in support export drifted from the in-crate builder; regenerate it"
    );
}

#[test]
fn seed_covers_every_taxonomy() {
    let packet = seeded();
    for x in FormFamily::ALL {
        assert!(
            packet.represented_families().contains(&x),
            "missing family {x:?}"
        );
    }
    for d in ProofDimension::ALL {
        assert!(
            packet.represented_dimensions().contains(&d),
            "missing dimension {d:?}"
        );
    }
    for l in ProofLane::ALL {
        assert!(
            packet.represented_lanes().contains(&l),
            "missing lane {l:?}"
        );
    }
    for c in ConsumerSurface::ALL {
        assert!(
            packet.represented_consumer_surfaces().contains(&c),
            "missing consumer surface {c:?}"
        );
    }
}

#[test]
fn every_family_proves_all_required_pairs() {
    let packet = seeded();
    for f in &packet.families {
        for (dimension, lane) in REQUIRED_PROOF_PAIRS {
            assert!(
                f.evidence
                    .iter()
                    .any(|c| c.dimension == dimension && c.source_lane == lane),
                "{} missing pair {:?}/{:?}",
                f.family_id,
                dimension,
                lane
            );
        }
    }
}

#[test]
fn request_workspace_narrows_at_baseline() {
    let packet = seeded();
    let decision = decide(family(&packet, F_REQUEST));
    assert_eq!(decision.effective_tier, QualificationTier::Beta);
    assert_eq!(decision.verdict, CertificationVerdict::Narrowed);
    assert!(decision.narrowed);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::ParameterProvenanceUncertified));
    assert!(decision
        .stale_or_missing_dimensions
        .contains(&ProofDimension::ParameterProvenance));
}

#[test]
fn overall_decision_is_narrowed_at_baseline() {
    let packet = seeded();
    let dist = packet.verdict_distribution();
    assert_eq!(dist.certified, 6);
    assert_eq!(dist.narrowed, 1);
    assert_eq!(dist.withdrawn, 0);
    assert_eq!(dist.overall_decision_token(), "narrowed");
}

// --------------------------------------------------------------------------- //
// Narrowing engine.
// --------------------------------------------------------------------------- //

#[test]
fn clean_family_certifies() {
    let packet = seeded();
    let decision = decide(family(&packet, F_PROVIDER));
    assert_eq!(decision.effective_tier, QualificationTier::Stable);
    assert_eq!(decision.verdict, CertificationVerdict::Certified);
    assert!(decision.certified);
    assert!(decision.reasons.is_empty());
}

#[test]
fn partial_field_validation_narrows_to_beta() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    set_state(
        &mut f,
        ProofDimension::FieldFormValidation,
        ProofLane::FormValidationAndBlockedSubmit,
        EvidenceState::Partial,
    );
    render_all(&mut f, QualificationTier::Beta);
    let decision = decide(&f);
    assert_eq!(decision.effective_tier, QualificationTier::Beta);
    assert_eq!(decision.verdict, CertificationVerdict::Narrowed);
    assert_eq!(
        decision.reasons,
        vec![NarrowingReason::FieldFormValidationUncertified]
    );
}

#[test]
fn missing_proof_narrows_to_preview() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    set_state(
        &mut f,
        ProofDimension::DraftVersusApplied,
        ProofLane::DraftStateAndAutosave,
        EvidenceState::Missing,
    );
    render_all(&mut f, QualificationTier::Preview);
    let decision = decide(&f);
    assert_eq!(decision.effective_tier, QualificationTier::Preview);
    assert_eq!(decision.verdict, CertificationVerdict::Narrowed);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::DraftRecoveryUncertified));
}

#[test]
fn failing_proof_withdraws() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    set_state(
        &mut f,
        ProofDimension::StagedReviewBeforeCommit,
        ProofLane::StagedReviewSheets,
        EvidenceState::Failing,
    );
    render_all(&mut f, QualificationTier::Withdrawn);
    let decision = decide(&f);
    assert_eq!(decision.effective_tier, QualificationTier::Withdrawn);
    assert_eq!(decision.verdict, CertificationVerdict::Withdrawn);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::StagedReviewUncertified));
}

#[test]
fn dropped_required_pair_narrows_to_preview() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    f.evidence
        .retain(|c| c.dimension != ProofDimension::ParameterProvenance);
    render_all(&mut f, QualificationTier::Preview);
    let decision = decide(&f);
    assert_eq!(decision.effective_tier, QualificationTier::Preview);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::RequiredProofMissing));
}

#[test]
fn rendering_overclaim_withdraws() {
    let packet = seeded();
    // Request narrows to beta, but every rendering still shows stable: an overclaim withdraws
    // the family rather than letting a consumer surface read wider than the evidence.
    let mut f = cloned(&packet, F_REQUEST);
    render_all(&mut f, QualificationTier::Stable);
    let decision = decide(&f);
    assert_eq!(decision.effective_tier, QualificationTier::Withdrawn);
    assert_eq!(decision.verdict, CertificationVerdict::Withdrawn);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::VerdictOverclaim));
}

#[test]
fn surface_reuse_incomplete_narrows_to_beta() {
    let packet = seeded();
    let mut f = cloned(&packet, F_SETTINGS);
    // A faithful consumer renders the narrowed tier; the gap is the missing surface.
    render_all(&mut f, QualificationTier::Beta);
    f.renderings
        .retain(|r| r.surface != ConsumerSurface::Compatibility);
    let decision = decide(&f);
    assert_eq!(decision.effective_tier, QualificationTier::Beta);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::SurfaceReuseIncomplete));
}

#[test]
fn elapsed_window_ages_certified_to_beta() {
    let packet = seeded();
    // Under an elapsed window a faithful consumer re-renders the narrowed tier.
    let mut f = cloned(&packet, F_PROVIDER);
    render_all(&mut f, QualificationTier::Beta);
    let decision = f.narrow(true);
    assert_eq!(decision.effective_tier, QualificationTier::Beta);
    assert!(decision
        .reasons
        .contains(&NarrowingReason::CertificationProofStale));
}

#[test]
fn stale_window_detected_past_slo() {
    let packet = seeded();
    assert!(!packet.stale_window());
    assert!(packet.freshness_stale_at("2026-07-01T00:00:00Z"));
}

// --------------------------------------------------------------------------- //
// Structural violations.
// --------------------------------------------------------------------------- //

#[test]
fn narrowed_family_without_rerun_loses_fallback() {
    let packet = seeded();
    let mut f = cloned(&packet, F_REQUEST);
    f.lineage.rerun_ref = None;
    let decision = decide(&f);
    assert!(decision.narrowed);
    assert!(!f.floored_keeps_fallback(decision.effective_tier));
}

#[test]
fn incoherent_evidence_ref_is_caught() {
    let mut input_packet = seeded();
    // Make a current cell drop its proof ref: incoherent with a `current` state.
    let cell = input_packet.families[0]
        .evidence
        .iter_mut()
        .find(|c| c.state == EvidenceState::Current)
        .expect("a current cell");
    cell.proof_ref = None;
    let violations = input_packet.validate();
    assert!(violations.contains(&M5FormFamilyCertificationViolation::EvidenceRefIncoherent));
}

#[test]
fn current_cell_past_window_is_a_freshness_overclaim() {
    let mut packet = seeded();
    let cell = packet.families[0]
        .evidence
        .iter_mut()
        .find(|c| c.state == EvidenceState::Current)
        .expect("a current cell");
    cell.captured_at = Some(SEED_STALE_CAPTURED_AT.to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5FormFamilyCertificationViolation::EvidenceFreshnessOverclaim));
}

#[test]
fn report_renders_and_names_the_narrowed_family() {
    let packet = seeded();
    let report = packet.render_markdown_report();
    assert!(report.contains("Form Family Certification"));
    assert!(report.contains("request_workspace narrowed to beta"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded();
    assert!(!packet
        .validate()
        .contains(&M5FormFamilyCertificationViolation::RawBoundaryMaterialInExport));
}
