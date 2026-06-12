use super::*;

fn packet() -> M5BundleScorecardsPacket {
    current_m5_bundle_scorecards_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_BUNDLE_SCORECARDS_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_BUNDLE_SCORECARDS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_matches_recomputed() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_wedge_has_a_scorecard() {
    // Every claimed M5 launch wedge has a compatibility scorecard.
    let packet = packet();
    assert!(packet.covers_every_wedge());
    for wedge in M5Wedge::ALL {
        assert!(
            packet.scorecards_for_wedge(wedge).next().is_some(),
            "wedge {} has no scorecard",
            wedge.as_str()
        );
    }
}

#[test]
fn every_claimed_class_is_exercised() {
    let packet = packet();
    for class in BundleScorecardClass::ALL {
        assert!(
            packet.scorecards.iter().any(|s| s.claimed_class == class),
            "claimed class {} is not exercised",
            class.as_str()
        );
    }
}

#[test]
fn every_effective_class_is_exercised() {
    let packet = packet();
    for class in BundleScorecardClass::ALL {
        assert!(
            packet
                .scorecards_with_effective_class(class)
                .next()
                .is_some(),
            "effective class {} is not exercised",
            class.as_str()
        );
    }
}

#[test]
fn every_confidence_level_is_exercised() {
    let packet = packet();
    for confidence in ImportedVsNativeConfidence::ALL {
        assert!(
            packet
                .scorecards
                .iter()
                .any(|s| s.imported_vs_native_confidence == confidence),
            "confidence {} is not exercised",
            confidence.as_str()
        );
    }
}

#[test]
fn every_freshness_level_is_exercised() {
    let packet = packet();
    for freshness in EvidenceFreshness::ALL {
        assert!(
            packet
                .scorecards
                .iter()
                .any(|s| s.evidence_freshness == freshness),
            "freshness {} is not exercised",
            freshness.as_str()
        );
    }
}

#[test]
fn every_platform_is_exercised() {
    let packet = packet();
    for platform in PlatformClass::ALL {
        assert!(
            packet
                .scorecards
                .iter()
                .any(|s| s.supported_platforms.contains(&platform)),
            "platform {} is not exercised",
            platform.as_str()
        );
    }
}

#[test]
fn every_dependency_lifecycle_stage_is_exercised() {
    let packet = packet();
    for stage in LifecycleStage::ALL {
        let exercised = packet
            .scorecards
            .iter()
            .flat_map(|s| s.bundle_dependencies.iter())
            .any(|d| d.lifecycle_stage == stage);
        assert!(
            exercised,
            "lifecycle stage {} is not exercised",
            stage.as_str()
        );
    }
}

#[test]
fn effective_class_never_out_ranks_its_inputs() {
    // The effective class is recomputed and can never out-rank the claimed class, the confidence,
    // or the freshness — this is what stops imported/approximate behavior inheriting certified
    // language by inertia.
    let packet = packet();
    for s in &packet.scorecards {
        assert!(
            s.effective_class_consistent(),
            "scorecard {} effective class diverges from its evidence",
            s.bundle_id
        );
        assert!(s.effective_class.rank() <= s.claimed_class.rank());
        assert!(s.effective_class.rank() <= s.imported_vs_native_confidence.cap_rank());
        assert!(s.effective_class.rank() <= s.evidence_freshness.cap_rank());
    }
}

#[test]
fn approximate_or_unverified_never_presents_as_certified() {
    let packet = packet();
    for s in &packet.scorecards {
        if matches!(
            s.imported_vs_native_confidence,
            ImportedVsNativeConfidence::Approximated | ImportedVsNativeConfidence::Unverified
        ) {
            assert!(
                !s.presents_as_certified,
                "scorecard {} presents as certified despite non-native confidence",
                s.bundle_id
            );
        }
    }
}

#[test]
fn stale_or_missing_evidence_never_presents_as_certified() {
    let packet = packet();
    for s in &packet.scorecards {
        if s.evidence_freshness.is_stale() {
            assert!(
                !s.presents_as_certified,
                "scorecard {} presents as certified on stale evidence",
                s.bundle_id
            );
        }
    }
}

#[test]
fn only_certified_effective_class_presents_as_certified() {
    let packet = packet();
    for s in &packet.scorecards {
        assert_eq!(
            s.presents_as_certified,
            s.effective_class == BundleScorecardClass::Certified,
            "scorecard {} certified presentation diverges from its effective class",
            s.bundle_id
        );
    }
}

#[test]
fn narrowed_scorecards_carry_a_caveat() {
    let packet = packet();
    let mut saw_downgrade = false;
    let mut saw_bounded = false;
    for s in &packet.scorecards {
        if s.caveats_required() {
            assert!(
                s.caveats.iter().any(|c| !c.trim().is_empty()),
                "scorecard {} narrows but carries no caveat",
                s.bundle_id
            );
        }
        saw_downgrade |= s.was_downgraded();
        saw_bounded |= s.platform_bounded;
    }
    // The corpus exercises both narrowing paths.
    assert!(saw_downgrade, "no scorecard exercises a class downgrade");
    assert!(saw_bounded, "no scorecard exercises a platform bound");
}

#[test]
fn platform_bounded_flag_matches_support_set() {
    let packet = packet();
    for s in &packet.scorecards {
        assert!(
            s.platforms_well_formed(),
            "scorecard {} platform set malformed",
            s.bundle_id
        );
        assert_eq!(
            s.platform_bounded,
            s.computed_platform_bounded(),
            "scorecard {} bounded flag diverges from its platform set",
            s.bundle_id
        );
    }
}

#[test]
fn every_scorecard_joins_to_existing_proofs() {
    // The scorecard joins to the manifest, compatibility, archetype, and reference-workspace proofs
    // rather than minting another unlinked format.
    let packet = packet();
    for s in &packet.scorecards {
        assert!(
            s.linkage_complete(),
            "scorecard {} does not join to the existing proofs",
            s.bundle_id
        );
        assert!(
            s.consumers_complete(),
            "scorecard {} missing a consumer ref",
            s.bundle_id
        );
    }
}

#[test]
fn export_projection_reflects_scorecards() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.scorecards.len(), packet.scorecards.len());
    assert!(projection.all_scorecards_consistent);
    assert_eq!(
        projection.presents_as_certified,
        packet.summary.presents_as_certified
    );
    for (row, s) in projection.scorecards.iter().zip(&packet.scorecards) {
        assert_eq!(row.bundle_id, s.bundle_id);
        assert_eq!(row.claimed_class, s.claimed_class);
        assert_eq!(row.effective_class, s.effective_class);
        assert_eq!(row.was_downgraded, s.was_downgraded());
    }
}

#[test]
fn all_scorecards_consistent_holds() {
    assert!(packet().all_scorecards_consistent());
}

#[test]
fn validate_flags_an_inflated_effective_class() {
    let mut packet = packet();
    let s = packet
        .scorecards
        .iter_mut()
        .find(|s| s.was_downgraded())
        .expect("a downgraded scorecard exists");
    s.effective_class = BundleScorecardClass::Certified;
    s.presents_as_certified = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BundleScorecardsViolation::EffectiveClassMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_certified_presentation_on_a_non_certified_class() {
    let mut packet = packet();
    let s = packet
        .scorecards
        .iter_mut()
        .find(|s| s.effective_class != BundleScorecardClass::Certified)
        .expect("a non-certified scorecard exists");
    s.presents_as_certified = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BundleScorecardsViolation::CertifiedPresentationMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_missing_linkage() {
    let mut packet = packet();
    packet.scorecards[0].compatibility_scorecard_ref = String::new();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5BundleScorecardsViolation::MissingLinkage { .. })));
}

#[test]
fn validate_flags_a_divergent_platform_flag() {
    let mut packet = packet();
    let s = packet
        .scorecards
        .iter_mut()
        .find(|s| !s.platform_bounded)
        .expect("an unbounded scorecard exists");
    s.supported_platforms = vec![PlatformClass::Linux];
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5BundleScorecardsViolation::PlatformMismatch { .. })));
}

#[test]
fn validate_flags_missing_wedge_coverage() {
    let mut packet = packet();
    packet
        .scorecards
        .retain(|s| s.wedge != M5Wedge::DocsWorkspace);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BundleScorecardsViolation::MissingWedgeCoverage {
            wedge: M5Wedge::DocsWorkspace
        }
    )));
}

#[test]
fn effective_class_downgrade_formula_is_minimum_of_caps() {
    // Spot-check the central downgrade rule directly.
    let cases = [
        (
            BundleScorecardClass::Certified,
            ImportedVsNativeConfidence::Native,
            EvidenceFreshness::Fresh,
            BundleScorecardClass::Certified,
        ),
        (
            BundleScorecardClass::Certified,
            ImportedVsNativeConfidence::Bridged,
            EvidenceFreshness::Stale,
            BundleScorecardClass::Probable,
        ),
        (
            BundleScorecardClass::Certified,
            ImportedVsNativeConfidence::Approximated,
            EvidenceFreshness::Fresh,
            BundleScorecardClass::Imported,
        ),
        (
            BundleScorecardClass::Certified,
            ImportedVsNativeConfidence::Unverified,
            EvidenceFreshness::Fresh,
            BundleScorecardClass::Preview,
        ),
    ];
    for (claimed, confidence, freshness, expected) in cases {
        let rank = claimed
            .rank()
            .min(confidence.cap_rank())
            .min(freshness.cap_rank());
        assert_eq!(BundleScorecardClass::from_rank(rank), expected);
    }
}

#[test]
fn constants_point_at_checked_in_paths() {
    assert_eq!(
        M5_BUNDLE_SCORECARDS_PATH,
        "artifacts/workspace/m5/m5-bundle-scorecards.json"
    );
    assert_eq!(
        M5_BUNDLE_SCORECARDS_SCHEMA_REF,
        "schemas/workspace/m5-bundle-scorecards.schema.json"
    );
    assert_eq!(
        M5_BUNDLE_SCORECARDS_DOC_REF,
        "docs/workspace/m5/m5-bundle-scorecards.md"
    );
}
