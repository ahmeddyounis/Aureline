use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::ArtifactFamily;
use crate::m5_author_and_publish_preview::current_m5_author_publish_matrix;
use crate::m5_ecosystem_certification::current_m5_ecosystem_certification;

fn packet() -> M5AuthorCertification {
    current_m5_author_certification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_AUTHOR_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_AUTHOR_CERTIFICATION_RECORD_KIND);
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
            e.certification_signals,
            e.computed_certification_signals(),
            "entry {} signals diverge from the recomputed set",
            e.entry_id
        );
        assert_eq!(
            e.certification_disposition,
            e.computed_certification_disposition(),
            "entry {} disposition diverges from the recomputed value",
            e.entry_id
        );
        assert_eq!(
            e.effective_trust_posture,
            e.computed_effective_trust_posture(),
            "entry {} effective trust diverges from the recomputed value",
            e.entry_id
        );
        assert_eq!(
            e.effective_author_support_class,
            e.computed_effective_author_support_class(),
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
fn every_entry_runs_every_author_drill_lane() {
    // The aggregation guardrail: a row cannot be certified by running a subset of the
    // author drills, so each entry must carry one evidence record per lane.
    let packet = packet();
    for e in &packet.entries {
        let lanes: BTreeSet<AuthorCertificationLane> =
            e.lane_evidence.iter().map(|l| l.lane).collect();
        assert_eq!(
            lanes.len(),
            AuthorCertificationLane::ALL.len(),
            "entry {} does not cover every author drill lane",
            e.entry_id
        );
        for lane in AuthorCertificationLane::ALL {
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
fn covers_every_certification_disposition() {
    let packet = packet();
    let dispositions: BTreeSet<DispositionToken> = packet
        .entries
        .iter()
        .map(|e| DispositionToken(e.certification_disposition))
        .collect();
    for disposition in AuthorCertificationDisposition::ALL {
        assert!(
            dispositions.contains(&DispositionToken(disposition)),
            "no entry exercises disposition {disposition:?}"
        );
    }
}

// Wrapper so the local disposition enum can live in a BTreeSet for the coverage check.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct DispositionToken(AuthorCertificationDisposition);

#[test]
fn covers_every_author_lane_state() {
    let packet = packet();
    let states: BTreeSet<LaneStateToken> = packet
        .entries
        .iter()
        .flat_map(|e| e.lane_evidence.iter().map(|l| LaneStateToken(l.state)))
        .collect();
    for state in AuthorLaneState::ALL {
        assert!(
            states.contains(&LaneStateToken(state)),
            "no lane exercises state {state:?}"
        );
    }
}

// Wrapper so the local lane-state enum can live in a BTreeSet for the coverage check.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct LaneStateToken(AuthorLaneState);

#[test]
fn local_or_untrusted_artifacts_never_inherit_a_trusted_badge() {
    // The non-inheritance invariant: an unsigned, side-loaded, revoked, local-dev-origin,
    // or pending-rebind row must render unsigned_local_only regardless of the declared
    // posture, so a local artifact never inherits a verified/enterprise badge.
    let packet = packet();
    for e in &packet.entries {
        let structurally_local = e.signature_state.is_local_or_untrusted()
            || e.workspace_origin.is_local_authored()
            || matches!(
                e.registry_binding,
                BindPublishedIdentity::StayLocal | BindPublishedIdentity::BindReviewRequired
            );
        if structurally_local {
            assert_eq!(
                e.effective_trust_posture,
                TrustPosture::UnsignedLocalOnly,
                "entry {} inherited a trusted badge despite a local/untrusted posture",
                e.entry_id
            );
        }
    }
}

#[test]
fn author_claim_never_exceeds_the_install_claim() {
    // The certification narrows the marketed row; it never widens it.
    let packet = packet();
    for e in &packet.entries {
        assert!(
            e.effective_author_support_class.rank() <= e.end_user_install_support_class.rank(),
            "entry {} author claim exceeds the install claim it guards",
            e.entry_id
        );
    }
}

#[test]
fn weaker_author_lane_auto_narrows_below_the_install_claim() {
    // The core auto-narrowing behaviour: an entry whose author-side ceiling is below the
    // install claim must record the author_claim_below_install_claim signal and apply a
    // downgrade.
    let packet = packet();
    for e in &packet.entries {
        if e.author_side_support_ceiling().rank() < e.end_user_install_support_class.rank() {
            assert!(
                e.certification_signals
                    .contains(&AuthorCertificationSignal::AuthorClaimBelowInstallClaim),
                "entry {} narrows below the install claim but omits the signal",
                e.entry_id
            );
            assert!(
                e.downgrade_path.applied,
                "entry {} narrows below the install claim but applies no downgrade",
                e.entry_id
            );
        }
    }
}

#[test]
fn uncertified_collapses_to_unsupported() {
    let packet = packet();
    for e in &packet.entries {
        if e.certification_disposition == AuthorCertificationDisposition::Uncertified {
            assert_eq!(
                e.effective_author_support_class,
                SupportClass::Unsupported,
                "uncertified entry {} still claims support",
                e.entry_id
            );
        }
    }
}

#[test]
fn support_claim_requires_evidence_and_certification() {
    // The support guardrail: any positive effective support claim must be evidence-backed
    // and not uncertified, so trusted-machine status never implies an author-lane claim.
    let packet = packet();
    for e in &packet.entries {
        if e.effective_author_support_class != SupportClass::Unsupported {
            assert!(
                e.is_evidence_backed(),
                "entry {} claims support without evidence linkage",
                e.entry_id
            );
            assert_ne!(
                e.certification_disposition,
                AuthorCertificationDisposition::Uncertified,
                "entry {} claims support while uncertified",
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
fn entries_resolve_to_author_matrix_and_install_certification_rows() {
    // The certification is an aggregator, not a parallel spreadsheet: each entry must
    // resolve to a real author-matrix family and a real install-certification entry.
    let packet = packet();
    let author = current_m5_author_publish_matrix().expect("author matrix parses");
    let install = current_m5_ecosystem_certification().expect("install certification parses");
    for e in &packet.entries {
        assert!(
            author
                .families
                .iter()
                .any(|f| f.artifact_family == e.package_kind),
            "entry {} resolves to no author-matrix family",
            e.entry_id
        );
        assert!(
            install
                .entries
                .iter()
                .any(|i| i.entry_id == e.install_certification_ref),
            "entry {} install ref {} resolves to no install certification entry",
            e.entry_id,
            e.install_certification_ref
        );
    }
}

#[test]
fn never_renders_a_stronger_badge_than_the_publish_gate() {
    // The board-level cross-check: no entry may render a stronger trust posture than the
    // author-and-publish-preview gate grants the same family, so the author certification
    // and the publish preview project one trust truth.
    let packet = packet();
    let author = current_m5_author_publish_matrix().expect("author matrix parses");
    for e in &packet.entries {
        if let Some(row) = author.family(e.package_kind) {
            assert!(
                e.effective_trust_posture.rank() <= row.published_trust_posture.rank(),
                "entry {} renders a stronger badge than the publish gate grants {}",
                e.entry_id,
                e.package_kind.as_str()
            );
        }
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
    assert_eq!(projection.certified_count, packet.summary.certified_entries);
    assert_eq!(
        projection.downgraded_count,
        packet.summary.downgraded_entries
    );
    assert_eq!(
        projection.uncertified_count,
        packet.summary.uncertified_entries
    );
    assert_eq!(
        projection.downgrade_applied_count,
        packet.summary.entries_with_downgrade_applied
    );
    assert_eq!(
        projection.local_only_count,
        packet.summary.local_only_trust_entries
    );
}

#[test]
fn detects_a_hand_widened_trust_posture() {
    // An entry that asserts a stronger trust posture than its facts warrant must fail
    // validation, so the recomputation cannot be bypassed.
    let mut packet = packet();
    let target = packet
        .entries
        .iter()
        .position(|e| e.effective_trust_posture == TrustPosture::UnsignedLocalOnly)
        .expect("a local-only entry exists");
    packet.entries[target].effective_trust_posture = TrustPosture::EnterpriseApproved;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AuthorCertificationViolation::OverstatedTrustPosture { .. }
    )));
}

#[test]
fn detects_a_hand_widened_support_claim() {
    let mut packet = packet();
    let target = packet
        .entries
        .iter()
        .position(|e| e.certification_disposition == AuthorCertificationDisposition::Uncertified)
        .expect("an uncertified entry exists");
    packet.entries[target].effective_author_support_class = SupportClass::FullySupported;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AuthorCertificationViolation::EffectiveSupportMismatch { .. }
    )));
}

#[test]
fn detects_a_dropped_certification_signal() {
    let mut packet = packet();
    let target = packet
        .entries
        .iter()
        .position(|e| !e.certification_signals.is_empty())
        .expect("an entry with signals exists");
    packet.entries[target].certification_signals.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AuthorCertificationViolation::CertificationSignalsMismatch { .. }
    )));
}

#[test]
fn detects_a_dropped_author_drill_lane() {
    let mut packet = packet();
    packet.entries[0].lane_evidence.remove(0);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AuthorCertificationViolation::MissingLane { .. })));
}

#[test]
fn closed_vocabularies_are_canonical() {
    let packet = packet();
    assert_eq!(packet.package_kinds, ArtifactFamily::ALL.to_vec());
    assert_eq!(packet.source_classes, SourceClass::ALL.to_vec());
    assert_eq!(packet.runtime_classes, RuntimeClass::ALL.to_vec());
    assert_eq!(packet.host_abi_classes, HostAbiClass::ALL.to_vec());
    assert_eq!(packet.signature_states, SignatureState::ALL.to_vec());
    assert_eq!(packet.workspace_origins, WorkspaceOrigin::ALL.to_vec());
    assert_eq!(
        packet.registry_bindings,
        BindPublishedIdentity::ALL.to_vec()
    );
    assert_eq!(packet.trust_postures, TrustPosture::ALL.to_vec());
    assert_eq!(packet.support_classes, SupportClass::ALL.to_vec());
    assert_eq!(
        packet.evidence_freshness_classes,
        EvidenceFreshness::ALL.to_vec()
    );
    assert_eq!(
        packet.publish_readiness_states,
        PublishReadiness::ALL.to_vec()
    );
    assert_eq!(
        packet.author_certification_lanes,
        AuthorCertificationLane::ALL.to_vec()
    );
    assert_eq!(packet.author_lane_states, AuthorLaneState::ALL.to_vec());
    assert_eq!(
        packet.author_certification_signals,
        AuthorCertificationSignal::ALL.to_vec()
    );
    assert_eq!(
        packet.author_certification_dispositions,
        AuthorCertificationDisposition::ALL.to_vec()
    );
}
