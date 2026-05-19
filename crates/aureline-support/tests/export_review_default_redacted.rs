//! Drill tests for the default-redacted support and incident export
//! profile and the reopen manifest.
//!
//! The corpus lives at `fixtures/support/m3/redaction_and_escalation/` and
//! the boundary schema lives at
//! `schemas/support/export_redaction_profile.schema.json`.

use aureline_support::{
    current_escalation_packet_reviews, load_profile_corpus, load_reopen_corpus,
    BroadenEvidenceReviewClass, DestinationClass, EvidenceClass, EvidenceInclusionClass,
    ScenarioFamilyClass, SupportExportRedactionError, SupportExportRedactionProfile,
    SupportExportReopenManifest, REQUIRED_EVIDENCE_CLASSES,
};

#[test]
fn protected_corpus_loads_and_validates() {
    let profiles = load_profile_corpus().expect("profiles load");
    assert_eq!(profiles.len(), 3, "the seed corpus has three profile cases");
    for profile in &profiles {
        profile.validate().expect("profile is default-redacted");
        assert!(profile.is_default_redacted());
        assert!(profile.preserves_local_only_path());
    }
}

#[test]
fn every_profile_preserves_required_evidence_classes() {
    let profiles = load_profile_corpus().expect("profiles load");
    for profile in &profiles {
        for required in REQUIRED_EVIDENCE_CLASSES.iter() {
            assert!(
                profile.default_required_evidence_classes.contains(required),
                "profile {} drops required class {}",
                profile.id(),
                required.as_str(),
            );
            let included = profile
                .evidence_class_rules
                .iter()
                .any(|row| row.evidence_class == required.as_evidence_class() && row.inclusion_class.is_included());
            assert!(
                included,
                "profile {} declares required class {} but no rule includes it",
                profile.id(),
                required.as_str(),
            );
        }
    }
}

#[test]
fn raw_dumps_and_code_adjacent_payloads_never_embed_by_default() {
    let profiles = load_profile_corpus().expect("profiles load");
    for profile in &profiles {
        assert!(!profile.raw_dumps_attached, "{} attaches raw dumps", profile.id());
        assert!(
            !profile.raw_transcripts_attached,
            "{} attaches raw transcripts",
            profile.id(),
        );
        assert!(
            !profile.secret_bearing_attached,
            "{} attaches secret-bearing material",
            profile.id(),
        );
        assert!(!profile.crash_linkage.raw_dump_attached);
        for row in &profile.evidence_class_rules {
            if row.evidence_class.is_always_prohibited() {
                assert_eq!(row.inclusion_class, EvidenceInclusionClass::ExcludedAlways);
                assert_eq!(row.broaden_review_class, BroadenEvidenceReviewClass::Prohibited);
            }
        }
    }
}

#[test]
fn code_adjacent_widening_requires_explicit_broaden_review() {
    let profiles = load_profile_corpus().expect("profiles load");
    let widened = profiles
        .iter()
        .find(|profile| profile.broaden_evidence_review_ref.is_some())
        .expect("at least one profile records a widened evidence class");
    let widened_row = widened
        .evidence_class_rules
        .iter()
        .find(|row| row.evidence_class == EvidenceClass::MutationJournalExcerpt)
        .expect("widened profile has a mutation_journal_excerpt rule");
    assert_eq!(widened_row.inclusion_class, EvidenceInclusionClass::EmbeddedByReference);
    assert!(widened.broaden_evidence_review_ref.is_some());
}

#[test]
fn local_only_path_remains_equal_with_upload_paths() {
    let profiles = load_profile_corpus().expect("profiles load");
    assert!(
        profiles.iter().any(|profile| profile.destination_posture.selected_destination_class
            == DestinationClass::LocalOnlyReview),
        "at least one profile selects local-only review",
    );
    assert!(
        profiles.iter().any(|profile| profile.destination_posture.selected_destination_class
            == DestinationClass::VendorCaseHandoff),
        "at least one profile selects vendor handoff",
    );
    for profile in &profiles {
        assert!(profile.destination_posture.local_only_path_available);
        assert!(profile.destination_posture.local_only_equal_prominence);
    }
}

#[test]
fn crash_manifest_and_symbolication_refs_carry_by_id() {
    let profiles = load_profile_corpus().expect("profiles load");
    let crash_linked = profiles
        .iter()
        .find(|profile| !profile.crash_linkage.crash_manifest_refs.is_empty())
        .expect("at least one profile carries a crash manifest by reference");
    assert!(!crash_linked.crash_linkage.raw_dump_attached);
    assert!(!crash_linked.crash_linkage.symbolication_report_refs.is_empty());
    for manifest_ref in &crash_linked.crash_linkage.crash_manifest_refs {
        assert!(
            !manifest_ref.contains('\n'),
            "manifest refs must be stable single-line ids",
        );
    }
}

#[test]
fn reopen_manifests_round_trip_and_match_profile_ids() {
    let profiles = load_profile_corpus().expect("profiles load");
    let manifests = load_reopen_corpus().expect("manifests load");
    assert_eq!(manifests.len(), 2, "the seed corpus has two reopen cases");
    for manifest in &manifests {
        manifest.validate().expect("manifest is reopen-truth safe");
        assert!(
            profiles.iter().any(|profile| profile.id() == manifest.profile_ref),
            "reopen manifest {} references unknown profile {}",
            manifest.id(),
            manifest.profile_ref,
        );
        let json = serde_json::to_string(manifest).expect("manifest serializes");
        let parsed: SupportExportReopenManifest =
            serde_json::from_str(&json).expect("manifest round-trips");
        assert_eq!(&parsed, manifest);
    }
}

#[test]
fn reopen_manifest_shows_included_and_excluded_classes() {
    let manifests = load_reopen_corpus().expect("manifests load");
    for manifest in &manifests {
        assert!(!manifest.included_evidence_classes.is_empty());
        assert!(manifest.preserves_default_required_classes());
        for excluded in &manifest.excluded_evidence_classes {
            assert!(
                !manifest.included_evidence_classes.contains(excluded),
                "evidence class {} cannot appear in both lists",
                excluded.as_str(),
            );
        }
    }
}

#[test]
fn local_only_manifest_never_records_export_timestamp() {
    let manifests = load_reopen_corpus().expect("manifests load");
    let local_only = manifests
        .iter()
        .find(|manifest| manifest.is_local_only())
        .expect("at least one local-only manifest");
    assert_eq!(local_only.destination_class, DestinationClass::LocalOnlyReview);
    assert!(local_only.exported_at_or_null.is_none());
    assert!(local_only.local_only);

    let exported = manifests
        .iter()
        .find(|manifest| manifest.destination_class != DestinationClass::LocalOnlyReview)
        .expect("at least one exported manifest");
    assert!(exported.exported_at_or_null.is_some());
    assert!(!exported.local_only);
}

#[test]
fn escalation_packet_review_quotes_build_identity_and_refs() {
    let reviews = current_escalation_packet_reviews().expect("reviews compile");
    assert!(!reviews.is_empty());
    for review in &reviews {
        assert!(!review.exact_build_identity_ref.is_empty());
        assert!(!review.raw_dump_attached);
        assert!(!review.raw_transcripts_attached);
        assert!(!review.code_adjacent_attached);
        assert!(!review.secret_bearing_attached);
        assert!(review.local_only_equal_prominence);
        assert!(!review.reopen_manifest_ref.is_empty());
    }
    let extension_review = reviews
        .iter()
        .find(|review| review.scenario_family_class == ScenarioFamilyClass::ExtensionOrHostRegression)
        .expect("extension-regression review present");
    assert!(!extension_review.crash_manifest_refs.is_empty());
    assert!(!extension_review.symbolication_report_refs.is_empty());
}

#[test]
fn profile_refuses_when_local_only_path_is_hidden() {
    let mut profile = load_profile_corpus()
        .expect("profiles load")
        .into_iter()
        .next()
        .expect("at least one profile");
    profile.destination_posture.local_only_equal_prominence = false;
    let err = profile
        .validate()
        .expect_err("profile must refuse when local-only path is no longer equal-prominent");
    assert!(matches!(err, SupportExportRedactionError::LocalOnlyPathHidden));
}

#[test]
fn profile_refuses_when_raw_dump_is_attached() {
    let mut profile = load_profile_corpus()
        .expect("profiles load")
        .into_iter()
        .next()
        .expect("at least one profile");
    profile.crash_linkage.raw_dump_attached = true;
    let err = profile
        .validate()
        .expect_err("profile must refuse when crash linkage attaches a raw dump");
    assert!(matches!(err, SupportExportRedactionError::RawDumpAttached));
}

#[test]
fn profile_refuses_when_a_required_class_is_dropped() {
    let mut profile = load_profile_corpus()
        .expect("profiles load")
        .into_iter()
        .next()
        .expect("at least one profile");
    profile.default_required_evidence_classes.pop();
    let err = profile
        .validate()
        .expect_err("profile must refuse when a default-required class is dropped");
    assert!(matches!(
        err,
        SupportExportRedactionError::MissingRequiredEvidenceClass(_)
    ));
}

#[test]
fn reopen_manifest_refuses_local_only_with_export_timestamp() {
    let mut manifest = load_reopen_corpus()
        .expect("manifests load")
        .into_iter()
        .find(|manifest| manifest.is_local_only())
        .expect("a local-only manifest");
    manifest.exported_at_or_null = Some("2026-05-19T11:00:00Z".to_owned());
    let err = manifest
        .validate()
        .expect_err("reopen manifest must refuse a local-only export with a timestamp");
    assert!(matches!(err, SupportExportRedactionError::LocalOnlyManifestExported));
}

#[test]
fn reopen_manifest_refuses_code_adjacent_without_review() {
    let manifests = load_reopen_corpus().expect("manifests load");
    let mut manifest: SupportExportReopenManifest = manifests
        .into_iter()
        .next()
        .expect("at least one manifest");
    manifest
        .included_evidence_classes
        .push(EvidenceClass::CodeSnippetAttachment);
    manifest.broaden_evidence_review_ref = None;
    let err = manifest
        .validate()
        .expect_err("reopen manifest must refuse widening code-adjacent classes without review");
    assert!(matches!(
        err,
        SupportExportRedactionError::CodeAdjacentWidenedWithoutReview(_)
    ));
}

#[test]
fn profile_round_trips_through_json() {
    let profile = load_profile_corpus()
        .expect("profiles load")
        .into_iter()
        .next()
        .expect("at least one profile");
    let json = serde_json::to_string(&profile).expect("profile serializes");
    let parsed: SupportExportRedactionProfile =
        serde_json::from_str(&json).expect("profile round-trips");
    assert_eq!(parsed, profile);
}
