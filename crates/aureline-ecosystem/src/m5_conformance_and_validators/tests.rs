use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::ArtifactFamily;

fn packet() -> M5ConformanceAndValidators {
    current_m5_conformance_and_validators().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_CONFORMANCE_AND_VALIDATORS_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_CONFORMANCE_AND_VALIDATORS_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_scorecard_is_recomputed() {
    let packet = packet();
    assert!(!packet.scorecards.is_empty());
    for s in &packet.scorecards {
        assert_eq!(
            s.certification_signals,
            s.computed_certification_signals(),
            "scorecard {} signals diverge from the recomputed set",
            s.scorecard_id
        );
        assert_eq!(
            s.certification_disposition,
            s.computed_certification_disposition(),
            "scorecard {} disposition diverges from the recomputed value",
            s.scorecard_id
        );
        assert_eq!(
            s.effective_support_class,
            s.computed_effective_support_class(),
            "scorecard {} effective support diverges from the recomputed value",
            s.scorecard_id
        );
    }
}

#[test]
fn covers_every_marketed_package_kind() {
    let packet = packet();
    let kinds: BTreeSet<ArtifactFamily> =
        packet.scorecards.iter().map(|s| s.package_kind).collect();
    assert_eq!(kinds.len(), ArtifactFamily::ALL.len());
    for family in ArtifactFamily::ALL {
        assert!(kinds.contains(&family), "missing scorecard for {family:?}");
    }
}

#[test]
fn covers_every_conformance_label() {
    let packet = packet();
    let labels: BTreeSet<ConformanceLabel> = packet
        .scorecards
        .iter()
        .map(|s| s.conformance_label)
        .collect();
    for label in ConformanceLabel::ALL {
        assert!(labels.contains(&label), "no scorecard exercises {label:?}");
    }
}

#[test]
fn support_claim_requires_evidence_and_certification() {
    // The lane guardrail: any positive support claim must be evidence-backed and not
    // uncertified, so first-party or bridge-backed status never implies support alone.
    let packet = packet();
    for s in &packet.scorecards {
        if s.effective_support_class != SupportClass::Unsupported {
            assert!(
                s.is_evidence_backed(),
                "scorecard {} claims support without evidence linkage",
                s.scorecard_id
            );
            assert_ne!(
                s.certification_disposition,
                CertificationDisposition::Uncertified,
                "scorecard {} claims support while uncertified",
                s.scorecard_id
            );
        }
    }
}

#[test]
fn uncertified_collapses_to_unsupported() {
    let packet = packet();
    for s in &packet.scorecards {
        if s.certification_disposition == CertificationDisposition::Uncertified {
            assert_eq!(
                s.effective_support_class,
                SupportClass::Unsupported,
                "uncertified scorecard {} still claims support",
                s.scorecard_id
            );
        }
    }
}

#[test]
fn every_diagnostic_is_actionable() {
    let packet = packet();
    for s in &packet.scorecards {
        for d in &s.validator_diagnostics {
            assert!(!d.code.trim().is_empty(), "{}: empty code", s.scorecard_id);
            assert!(
                !d.message.trim().is_empty(),
                "{}: empty message",
                s.scorecard_id
            );
            assert!(
                !d.remediation.trim().is_empty(),
                "{} diagnostic {} has no remediation",
                s.scorecard_id,
                d.code
            );
        }
    }
}

#[test]
fn retest_pending_label_blocks_certification() {
    let packet = packet();
    for s in &packet.scorecards {
        if s.conformance_label == ConformanceLabel::RetestPending {
            assert_eq!(
                s.certification_disposition,
                CertificationDisposition::Uncertified
            );
        }
    }
}

#[test]
fn validator_failure_blocks_certification() {
    let packet = packet();
    for s in &packet.scorecards {
        if s.has_severity(ValidatorSeverity::Error) {
            assert_eq!(
                s.certification_disposition,
                CertificationDisposition::Uncertified,
                "scorecard {} certified despite a validator failure",
                s.scorecard_id
            );
        }
    }
}

#[test]
fn export_projection_round_trips_records() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.scorecard_rows.len(), packet.scorecards.len());
    assert_eq!(
        projection.validator_report.len(),
        packet
            .scorecards
            .iter()
            .map(|s| s.validator_diagnostics.len())
            .sum::<usize>()
    );
    assert!(projection.all_scorecards_consistent);
    assert_eq!(
        projection.uncertified_count,
        packet.summary.uncertified_scorecards
    );
}

#[test]
fn detects_a_hand_widened_support_claim() {
    // A scorecard that asserts a stronger effective support class than its facts
    // warrant must fail validation, so the recomputation cannot be bypassed.
    let mut packet = packet();
    let target = packet
        .scorecards
        .iter()
        .position(|s| s.certification_disposition == CertificationDisposition::Uncertified)
        .expect("an uncertified scorecard exists");
    packet.scorecards[target].effective_support_class = SupportClass::FullySupported;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ConformanceAndValidatorsViolation::EffectiveSupportMismatch { .. }
    )));
}

#[test]
fn detects_a_dropped_certification_signal() {
    let mut packet = packet();
    let target = packet
        .scorecards
        .iter()
        .position(|s| !s.certification_signals.is_empty())
        .expect("a scorecard with signals exists");
    packet.scorecards[target].certification_signals.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ConformanceAndValidatorsViolation::CertificationSignalsMismatch { .. }
    )));
}

#[test]
fn closed_vocabularies_are_canonical() {
    let packet = packet();
    assert_eq!(packet.package_kinds, ArtifactFamily::ALL.to_vec());
    assert_eq!(packet.conformance_labels, ConformanceLabel::ALL.to_vec());
    assert_eq!(packet.validator_severities, ValidatorSeverity::ALL.to_vec());
    assert_eq!(packet.validator_domains, ValidatorDomain::ALL.to_vec());
    assert_eq!(
        packet.certification_signals,
        CertificationSignal::ALL.to_vec()
    );
    assert_eq!(
        packet.certification_dispositions,
        CertificationDisposition::ALL.to_vec()
    );
}
