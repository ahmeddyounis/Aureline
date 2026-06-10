use super::*;

const PACKET_ID: &str = "convention-diagnostic:stable:0001";
const PACKET_LABEL: &str =
    "Convention Diagnostics with Confidence Labels, Suppressibility, and Proving-File Disclosure";

const EXACT_FILE_LOCATION: &str =
    "convention-diagnostic-row:file_location.controllers.exact:2026.06";
const HIGH_NAMING: &str = "convention-diagnostic-row:naming.model.high:2026.06";
const HEURISTIC_API: &str = "convention-diagnostic-row:api_usage.legacy.heuristic:2026.05";
const SUPPRESSED_ROUTE: &str =
    "convention-diagnostic-row:required_registration.route.suppressed:2026.06";
const PROVING_UNAVAILABLE: &str =
    "convention-diagnostic-row:config_convention.proving_unavailable:2026.04";
const BRIDGE_NAMING: &str = "convention-diagnostic-row:naming.bridge.external_linter.low:2026.06";

fn proof_freshness() -> ConventionDiagnosticProofFreshness {
    ConventionDiagnosticProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ConventionDiagnosticPacket {
    canonical_convention_diagnostics(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-08T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a ConventionDiagnosticPacket, row_id: &str) -> &'a ConventionDiagnosticRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn convention_diagnostic_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_confidence_spectrum() {
    let packet = packet();
    let confidence: Vec<ConfidenceLabel> =
        packet.rows.iter().map(|row| row.confidence_label).collect();
    for required in [
        ConfidenceLabel::Exact,
        ConfidenceLabel::High,
        ConfidenceLabel::Heuristic,
        ConfidenceLabel::Low,
        ConfidenceLabel::ConfidenceUnknown,
    ] {
        assert!(
            confidence.contains(&required),
            "missing confidence {}",
            required.as_str()
        );
    }
}

#[test]
fn canonical_rows_cover_suppression_and_proving_spectrum() {
    let packet = packet();
    let suppression: Vec<SuppressionClass> = packet
        .rows
        .iter()
        .map(|row| row.suppression_class)
        .collect();
    assert!(suppression.contains(&SuppressionClass::NotSuppressible));
    assert!(suppression.contains(&SuppressionClass::Suppressible));
    assert!(suppression.contains(&SuppressionClass::SuppressedByUser));

    let proving: Vec<ProvingDisclosureClass> = packet
        .rows
        .iter()
        .map(|row| row.proving_disclosure_class)
        .collect();
    assert!(proving.contains(&ProvingDisclosureClass::ProvingManifestDisclosed));
    assert!(proving.contains(&ProvingDisclosureClass::ProvingFileDisclosed));
    assert!(proving.contains(&ProvingDisclosureClass::ProvingFileUnavailable));
}

#[test]
fn exact_diagnostic_is_grounded_and_active() {
    let packet = packet();
    let exact = row(&packet, EXACT_FILE_LOCATION);
    assert_eq!(exact.confidence_label, ConfidenceLabel::Exact);
    assert!(exact.confidence_label.is_confident());
    assert!(exact.proving_disclosure_class.is_grounded());
    assert!(!exact.proving_file_refs.is_empty());
    assert_eq!(exact.suppression_class, SuppressionClass::NotSuppressible);
    assert!(exact.admitted_for_display);
    assert!(!exact.is_blocked());
}

#[test]
fn high_confidence_diagnostic_discloses_proving_file() {
    let packet = packet();
    let high = row(&packet, HIGH_NAMING);
    assert_eq!(high.confidence_label, ConfidenceLabel::High);
    assert_eq!(
        high.proving_disclosure_class,
        ProvingDisclosureClass::ProvingFileDisclosed
    );
    assert!(high.admitted_for_display);
}

#[test]
fn heuristic_diagnostic_discloses_banner_and_is_held() {
    let packet = packet();
    let heuristic = row(&packet, HEURISTIC_API);
    assert!(heuristic.confidence_label.requires_banner());
    assert!(heuristic.support_class.requires_disclosure());
    assert!(!heuristic.known_issue_refs.is_empty());
    assert!(heuristic.downgrade_banner_class.is_present());
    assert!(heuristic
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::HeuristicConfidenceDisclosed));
    assert!(!heuristic.admitted_for_display);
}

#[test]
fn suppressed_diagnostic_is_labeled_not_hidden() {
    let packet = packet();
    let suppressed = row(&packet, SUPPRESSED_ROUTE);
    assert!(suppressed.suppression_class.is_suppressed());
    assert!(!suppressed.admitted_for_display);
    assert!(suppressed
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::SuppressionApplied));
    // The suppressed diagnostic is still present in the packet, not removed.
    assert!(packet.rows.iter().any(|row| row.row_id == SUPPRESSED_ROUTE));
}

#[test]
fn proving_file_unavailable_diagnostic_is_blocked() {
    let packet = packet();
    let blocked = row(&packet, PROVING_UNAVAILABLE);
    assert!(blocked.proving_disclosure_class.is_unavailable());
    assert!(blocked.confidence_label.is_unknown());
    assert_eq!(
        blocked.downgrade_banner_class,
        DiagnosticDowngradeBannerClass::ProvingFileUnavailableBanner
    );
    assert!(blocked.is_blocked());
    assert!(!blocked.admitted_for_display);
}

#[test]
fn bridged_diagnostic_discloses_known_issue_and_is_held() {
    let packet = packet();
    let bridge = row(&packet, BRIDGE_NAMING);
    assert_eq!(bridge.support_class, DiagnosticSupportClass::BridgeBehavior);
    assert!(!bridge.known_issue_refs.is_empty());
    assert!(bridge
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::BridgeBehaviorDisclosed));
    assert!(!bridge.admitted_for_display);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::RowsEmpty));
}

#[test]
fn confident_claim_without_proving_file_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == HIGH_NAMING)
        .unwrap()
        .proving_disclosure_class = ProvingDisclosureClass::NoProvingFileNeeded;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ProvingFileUndisclosedForConfidentClaim));
}

#[test]
fn non_confident_diagnostic_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == HEURISTIC_API)
        .unwrap()
        .downgrade_banner_class = DiagnosticDowngradeBannerClass::NoBanner;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ConfidenceBannerMissing));
}

#[test]
fn bridge_diagnostic_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == BRIDGE_NAMING)
        .unwrap()
        .known_issue_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::SupportClassUndisclosed));
}

#[test]
fn suppressed_diagnostic_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == SUPPRESSED_ROUTE)
        .unwrap()
        .admitted_for_display = true;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::SuppressionUndisclosed));
}

#[test]
fn proving_unavailable_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == PROVING_UNAVAILABLE)
        .unwrap()
        .downgrade_banner_class = DiagnosticDowngradeBannerClass::FreshnessBanner;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ProvingFileUnavailableBannerMissing));
}

#[test]
fn blocked_diagnostic_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == PROVING_UNAVAILABLE)
        .unwrap()
        .admitted_for_display = true;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::BlockedDisplayAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.suppressed_diagnostic_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .blocked_diagnostics_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ConventionDiagnosticViolation::ProofFreshnessIncomplete));
}

#[test]
fn unavailable_proving_file_blocks_a_diagnostic() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
        row_id: HIGH_NAMING.to_owned(),
        proving_file_available: false,
        confidence_verified: true,
        analysis_fresh: true,
        suppression_active: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let high = row(&packet, HIGH_NAMING);
    assert_eq!(
        high.proving_disclosure_class,
        ProvingDisclosureClass::ProvingFileUnavailable
    );
    assert_eq!(high.confidence_label, ConfidenceLabel::ConfidenceUnknown);
    assert!(high.proving_file_refs.is_empty());
    assert_eq!(
        high.downgrade_banner_class,
        DiagnosticDowngradeBannerClass::ProvingFileUnavailableBanner
    );
    assert!(!high.admitted_for_display);
    assert!(high
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::ProvingFileUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unverified_confidence_narrows_and_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
        row_id: EXACT_FILE_LOCATION.to_owned(),
        proving_file_available: true,
        confidence_verified: false,
        analysis_fresh: true,
        suppression_active: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let exact = row(&packet, EXACT_FILE_LOCATION);
    assert_eq!(exact.confidence_label, ConfidenceLabel::ConfidenceUnknown);
    assert!(exact.downgrade_banner_class.is_present());
    assert!(!exact.admitted_for_display);
    assert!(exact
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::ConfidenceDegraded));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_analysis_raises_banner_and_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
        row_id: HIGH_NAMING.to_owned(),
        proving_file_available: true,
        confidence_verified: true,
        analysis_fresh: false,
        suppression_active: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let high = row(&packet, HIGH_NAMING);
    assert_eq!(high.freshness_class, DiagnosticFreshnessClass::Stale);
    assert!(high.downgrade_banner_class.is_present());
    assert!(!high.admitted_for_display);
    assert!(high
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::AnalysisStale));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn newly_suppressed_diagnostic_is_labeled_and_withdrawn() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
        row_id: EXACT_FILE_LOCATION.to_owned(),
        proving_file_available: true,
        confidence_verified: true,
        analysis_fresh: true,
        suppression_active: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let exact = row(&packet, EXACT_FILE_LOCATION);
    assert_eq!(exact.suppression_class, SuppressionClass::SuppressedByScope);
    assert!(!exact.admitted_for_display);
    assert!(exact
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::SuppressionApplied));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ConventionDiagnosticRowObservation {
        row_id: EXACT_FILE_LOCATION.to_owned(),
        proving_file_available: true,
        confidence_verified: true,
        analysis_fresh: true,
        suppression_active: false,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let exact = row(&packet, EXACT_FILE_LOCATION);
    assert!(!exact.admitted_for_display);
    assert!(exact
        .downgrade_triggers
        .contains(&DiagnosticDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_diagnostic() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.diagnostic_label),
            "summary missing diagnostic {}",
            row.diagnostic_label
        );
    }
    assert!(summary.contains("proving_file_unavailable_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_convention_diagnostic_export()
        .expect("checked convention-diagnostic export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_convention_diagnostic_export()
        .expect("checked convention-diagnostic export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/proving_file_unavailable_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/confidence_unverified_withheld.json"
        )),
    ] {
        let packet: ConventionDiagnosticPacket =
            serde_json::from_str(raw).expect("fixture parses as convention-diagnostic packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
