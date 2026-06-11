use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::ArtifactFamily;
use crate::m5_conformance_and_validators::{
    current_m5_conformance_and_validators, CertificationDisposition,
};

fn packet() -> M5EcosystemCertification {
    current_m5_ecosystem_certification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_ECOSYSTEM_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_ECOSYSTEM_CERTIFICATION_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_entry_is_recomputed() {
    let packet = packet();
    assert!(!packet.entries.is_empty());
    for e in &packet.entries {
        assert_eq!(
            e.qualification_signals,
            e.computed_qualification_signals(),
            "entry {} signals diverge from the recomputed set",
            e.entry_id
        );
        assert_eq!(
            e.qualification_disposition,
            e.computed_qualification_disposition(),
            "entry {} disposition diverges from the recomputed value",
            e.entry_id
        );
        assert_eq!(
            e.effective_support_class,
            e.computed_effective_support_class(),
            "entry {} effective support diverges from the recomputed value",
            e.entry_id
        );
        assert_eq!(
            e.downgrade_path,
            e.computed_downgrade_path(),
            "entry {} downgrade path diverges from the recomputed value",
            e.entry_id
        );
    }
}

#[test]
fn every_entry_runs_every_drill_lane() {
    // The aggregation guardrail: a row cannot be certified by running a subset of the
    // ecosystem drills, so each entry must carry one evidence record per lane.
    let packet = packet();
    for e in &packet.entries {
        let lanes: BTreeSet<CertificationLane> = e.lane_evidence.iter().map(|l| l.lane).collect();
        assert_eq!(
            lanes.len(),
            CertificationLane::ALL.len(),
            "entry {} does not cover every drill lane",
            e.entry_id
        );
        for lane in CertificationLane::ALL {
            assert!(
                lanes.contains(&lane),
                "entry {} is missing lane {lane:?}",
                e.entry_id
            );
        }
    }
}

#[test]
fn covers_every_marketed_package_kind() {
    let packet = packet();
    let kinds: BTreeSet<ArtifactFamily> = packet.entries.iter().map(|e| e.package_kind).collect();
    assert_eq!(kinds.len(), ArtifactFamily::ALL.len());
    for family in ArtifactFamily::ALL {
        assert!(kinds.contains(&family), "missing entry for {family:?}");
    }
}

#[test]
fn covers_every_source_class() {
    let packet = packet();
    let sources: BTreeSet<SourceClass> = packet.entries.iter().map(|e| e.source_class).collect();
    for source in SourceClass::ALL {
        assert!(
            sources.contains(&source),
            "no entry exercises source class {source:?}"
        );
    }
}

#[test]
fn covers_every_qualification_disposition() {
    let packet = packet();
    let dispositions: BTreeSet<QualificationDispositionToken> = packet
        .entries
        .iter()
        .map(|e| QualificationDispositionToken(e.qualification_disposition))
        .collect();
    for disposition in QualificationDisposition::ALL {
        assert!(
            dispositions.contains(&QualificationDispositionToken(disposition)),
            "no entry exercises disposition {disposition:?}"
        );
    }
}

// Wrapper so the local disposition enum can live in a BTreeSet for the coverage check.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct QualificationDispositionToken(QualificationDisposition);

#[test]
fn support_claim_requires_evidence_and_qualification() {
    // The lane guardrail: any positive support claim must be evidence-backed and not
    // disqualified, so first-party or public-registry status never implies support alone.
    let packet = packet();
    for e in &packet.entries {
        if e.effective_support_class != SupportClass::Unsupported {
            assert!(
                e.is_evidence_backed(),
                "entry {} claims support without evidence linkage",
                e.entry_id
            );
            assert_ne!(
                e.qualification_disposition,
                QualificationDisposition::Disqualified,
                "entry {} claims support while disqualified",
                e.entry_id
            );
        }
    }
}

#[test]
fn disqualified_collapses_to_unsupported() {
    let packet = packet();
    for e in &packet.entries {
        if e.qualification_disposition == QualificationDisposition::Disqualified {
            assert_eq!(
                e.effective_support_class,
                SupportClass::Unsupported,
                "disqualified entry {} still claims support",
                e.entry_id
            );
        }
    }
}

#[test]
fn non_public_source_classes_cannot_inherit_full_support() {
    // Mirror, private-registry, bridge-backed, and side-loaded rows must narrow safely
    // and never inherit a broader first-party or public-registry full claim.
    let packet = packet();
    for e in &packet.entries {
        let structurally_capped = matches!(
            e.source_class,
            SourceClass::BridgeBacked
                | SourceClass::SideLoaded
                | SourceClass::MirroredRegistry
                | SourceClass::PrivateRegistry
        );
        if structurally_capped {
            assert_ne!(
                e.effective_support_class,
                SupportClass::FullySupported,
                "entry {} inherited a full claim from a non-public source class",
                e.entry_id
            );
        }
    }
}

#[test]
fn narrowed_entries_name_a_requalification_path() {
    let packet = packet();
    for e in &packet.entries {
        if e.downgrade_path.applied {
            assert!(
                !e.requalification_ref.trim().is_empty(),
                "narrowed entry {} names no requalification path",
                e.entry_id
            );
        }
    }
}

#[test]
fn conformance_flags_match_the_conformance_packet() {
    // The certification packet is an aggregator, not a parallel spreadsheet: each entry's
    // conformance_certified flag must match the linked conformance scorecard's actual
    // disposition.
    let packet = packet();
    let conformance = current_m5_conformance_and_validators().expect("conformance packet parses");
    for e in &packet.entries {
        let scorecard = conformance
            .scorecards
            .iter()
            .find(|s| s.scorecard_id == e.conformance_scorecard_ref)
            .unwrap_or_else(|| {
                panic!(
                    "entry {} links unknown conformance scorecard {}",
                    e.entry_id, e.conformance_scorecard_ref
                )
            });
        let certified =
            scorecard.certification_disposition != CertificationDisposition::Uncertified;
        assert_eq!(
            e.conformance_certified, certified,
            "entry {} conformance_certified disagrees with scorecard {}",
            e.entry_id, e.conformance_scorecard_ref
        );
    }
}

#[test]
fn export_projection_round_trips_records() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.index_rows.len(), packet.entries.len());
    assert_eq!(
        projection.downgrade_report.len(),
        packet
            .entries
            .iter()
            .filter(|e| e.downgrade_path.applied)
            .count()
    );
    assert!(projection.all_entries_consistent);
    assert_eq!(projection.qualified_count, packet.summary.qualified_entries);
    assert_eq!(
        projection.downgraded_count,
        packet.summary.downgraded_entries
    );
    assert_eq!(
        projection.disqualified_count,
        packet.summary.disqualified_entries
    );
    assert_eq!(
        projection.downgrade_applied_count,
        packet.summary.entries_with_downgrade_applied
    );
}

#[test]
fn detects_a_hand_widened_support_claim() {
    // An entry that asserts a stronger effective support class than its facts warrant
    // must fail validation, so the recomputation cannot be bypassed.
    let mut packet = packet();
    let target = packet
        .entries
        .iter()
        .position(|e| e.qualification_disposition == QualificationDisposition::Disqualified)
        .expect("a disqualified entry exists");
    packet.entries[target].effective_support_class = SupportClass::FullySupported;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5EcosystemCertificationViolation::EffectiveSupportMismatch { .. }
    )));
}

#[test]
fn detects_a_dropped_qualification_signal() {
    let mut packet = packet();
    let target = packet
        .entries
        .iter()
        .position(|e| !e.qualification_signals.is_empty())
        .expect("an entry with signals exists");
    packet.entries[target].qualification_signals.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5EcosystemCertificationViolation::QualificationSignalsMismatch { .. }
    )));
}

#[test]
fn detects_a_dropped_drill_lane() {
    let mut packet = packet();
    packet.entries[0].lane_evidence.remove(0);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5EcosystemCertificationViolation::MissingLane { .. })));
}

#[test]
fn closed_vocabularies_are_canonical() {
    let packet = packet();
    assert_eq!(packet.package_kinds, ArtifactFamily::ALL.to_vec());
    assert_eq!(packet.source_classes, SourceClass::ALL.to_vec());
    assert_eq!(packet.certification_lanes, CertificationLane::ALL.to_vec());
    assert_eq!(packet.lane_evidence_states, LaneEvidenceState::ALL.to_vec());
    assert_eq!(
        packet.qualification_signals,
        QualificationSignal::ALL.to_vec()
    );
    assert_eq!(
        packet.qualification_dispositions,
        QualificationDisposition::ALL.to_vec()
    );
}
