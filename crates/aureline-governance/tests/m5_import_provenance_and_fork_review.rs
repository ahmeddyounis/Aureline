//! Protected tests binding the typed import-provenance and fork-review register to the
//! checked-in artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every import
//! kind is exercised and every narrowing reason is wired; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the manifest/surface
//! parity, and the cleared/narrowed counts; the no-mask check proves a clean import surface
//! still narrows on a buried generator identity or an ownerless import and that scan and
//! surface agree on every record; the narrowing check proves an import-layer failure on a
//! still-stable subject holds promotion while inherited and waived narrowings stay gated
//! upstream; the negative cases mutate a parsed copy and read the checked-in fixtures to
//! prove that a hidden ownership gap, a clean surface over a gapped scan, a narrowed record
//! that stays above the cutline, and a proceed verdict while a rule fires all fail
//! validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{FreshnessSloState, LifecycleLabel};
use aureline_governance::m5_import_provenance_and_fork_review::{
    current_m5_import_provenance_and_fork_review, ControlDimension, ImportKind, ImportReason,
    ImportRegister, ImportState, Posture, PublicationDecision, RegisterViolation,
    M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_RECORD_KIND,
    M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-import-provenance-and-fork-review_validation_capture.json"
));

fn register() -> ImportRegister {
    current_m5_import_provenance_and_fork_review()
        .expect("checked-in register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_SCHEMA_VERSION
    );
    assert_eq!(
        r.record_kind,
        M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_RECORD_KIND
    );
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_import_kind_and_every_reason_has_a_rule() {
    let r = register();
    for kind in ImportKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "import kind {} must have at least one record",
            kind.as_str()
        );
    }
    for rec in &r.records {
        for dimension in ControlDimension::ALL {
            assert_eq!(
                rec.controls
                    .iter()
                    .filter(|c| c.dimension == dimension)
                    .count(),
                1,
                "record {} must declare control {} exactly once",
                rec.record_id,
                dimension.as_str()
            );
        }
    }
    for reason in ImportReason::ALL {
        assert!(
            r.rules.iter().any(|rule| rule.trigger_reason == reason),
            "reason {} must be watched by a rule",
            reason.as_str()
        );
    }
}

#[test]
fn keeps_per_axis_state_not_one_global_flag() {
    let r = register();
    let states: std::collections::BTreeSet<ImportState> =
        r.records.iter().map(|x| x.import_state).collect();
    assert!(states.contains(&ImportState::Cleared));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&ImportState::NarrowedProvenance));
    assert!(states.contains(&ImportState::NarrowedOwnership));
    assert!(states.contains(&ImportState::NarrowedDivergence));
    assert!(states.contains(&ImportState::NarrowedGenerator));
    assert!(states.contains(&ImportState::NarrowedStale));

    let reasons: std::collections::BTreeSet<ImportReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ImportReason::OriginUnattributed));
    assert!(reasons.contains(&ImportReason::UpdateOwnerMissing));
    assert!(reasons.contains(&ImportReason::DecisionRecordMissing));
    assert!(reasons.contains(&ImportReason::GeneratorIdentityMissing));
}

#[test]
fn clean_surface_never_masks_an_ownerless_or_generator_free_import() {
    let r = register();
    // Every record's scan and surface agree, so a clean surface can never sit over a
    // scan that found gaps.
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface must agree",
            rec.record_id
        );
        assert_eq!(rec.surface_posture, rec.computed_posture());
    }
    // An ownerless import still narrows on the ownership axis and reports gaps on its surface.
    let ownerless = r
        .records
        .iter()
        .find(|rec| rec.ownership.owner_missing())
        .expect("an ownerless import exists");
    assert_eq!(ownerless.import_state, ImportState::NarrowedOwnership);
    assert_eq!(ownerless.surface_posture, Posture::GapsFound);
    // A generated artifact with a buried generator identity still narrows.
    let buried = r
        .records
        .iter()
        .find(|rec| {
            rec.import_kind == ImportKind::GeneratedArtifact && rec.generator.identity_missing()
        })
        .expect("a generated artifact with a buried generator identity exists");
    assert_eq!(buried.import_state, ImportState::NarrowedGenerator);
    assert_eq!(buried.surface_posture, Posture::GapsFound);
}

#[test]
fn import_provenance_and_decision_truth_is_recorded() {
    let r = register();
    // The provenance, ownership, divergence, and generator axes actually carry gaps.
    assert!(
        r.summary.provenance_gaps > 0,
        "must record a provenance gap"
    );
    assert!(r.summary.ownership_gaps > 0, "must record an ownership gap");
    assert!(
        r.summary.divergence_gaps > 0,
        "must record a divergence gap"
    );
    assert!(r.summary.generator_gaps > 0, "must record a generator gap");
    // Long-lived imports are tracked and at least one decision is recorded.
    assert!(r.summary.long_lived_imports > 0);
    assert!(r.summary.decisions_recorded > 0);
    for rec in &r.records {
        if rec.provenance.origin_unattributed() {
            assert!(rec.has_active_reason(ImportReason::OriginUnattributed));
        }
        if rec.decision_missing() {
            assert!(rec.has_active_reason(ImportReason::DecisionRecordMissing));
        }
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let r = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(r.as_of.as_str()));

    let summary = &capture["summary"];
    let computed = r.computed_summary();
    let u = |v: &serde_json::Value| v.as_u64().unwrap() as usize;
    assert_eq!(u(&summary["total_records"]), computed.total_records);
    assert_eq!(u(&summary["records_cleared"]), computed.records_cleared);
    assert_eq!(u(&summary["records_narrowed"]), computed.records_narrowed);
    assert_eq!(u(&summary["state_cleared"]), computed.state_cleared);
    assert_eq!(
        u(&summary["state_narrowed_provenance"]),
        computed.state_narrowed_provenance
    );
    assert_eq!(
        u(&summary["state_narrowed_ownership"]),
        computed.state_narrowed_ownership
    );
    assert_eq!(
        u(&summary["state_narrowed_divergence"]),
        computed.state_narrowed_divergence
    );
    assert_eq!(
        u(&summary["state_narrowed_generator"]),
        computed.state_narrowed_generator
    );
    assert_eq!(
        u(&summary["state_narrowed_stale"]),
        computed.state_narrowed_stale
    );
    assert_eq!(
        u(&summary["release_blocking_narrowed"]),
        computed.release_blocking_narrowed
    );
    assert_eq!(
        u(&summary["records_on_active_waiver"]),
        computed.records_on_active_waiver
    );
    assert_eq!(u(&summary["provenance_gaps"]), computed.provenance_gaps);
    assert_eq!(u(&summary["ownership_gaps"]), computed.ownership_gaps);
    assert_eq!(u(&summary["divergence_gaps"]), computed.divergence_gaps);
    assert_eq!(u(&summary["generator_gaps"]), computed.generator_gaps);
    assert_eq!(
        u(&summary["third_party_imports"]),
        computed.third_party_imports
    );
    assert_eq!(
        u(&summary["generated_artifacts"]),
        computed.generated_artifacts
    );
    assert_eq!(
        u(&summary["long_lived_imports"]),
        computed.long_lived_imports
    );
    assert_eq!(
        u(&summary["decisions_recorded"]),
        computed.decisions_recorded
    );
    assert_eq!(
        u(&summary["total_active_reasons"]),
        computed.total_active_reasons
    );
    assert_eq!(u(&summary["rules_firing"]), computed.rules_firing);

    let parity = &capture["manifest_surface_parity"];
    let computed_parity = r.computed_manifest_surface_parity();
    assert_eq!(
        u(&parity["subjects_in_agreement"]),
        computed_parity.subjects_in_agreement
    );
    assert_eq!(
        u(&parity["subjects_in_disagreement"]),
        computed_parity.subjects_in_disagreement
    );
    assert_eq!(
        u(&parity["subjects_with_gaps"]),
        computed_parity.subjects_with_gaps
    );
    assert_eq!(
        parity["all_subjects_agree"].as_bool(),
        Some(computed_parity.all_subjects_agree)
    );

    assert_eq!(
        capture["publication"]["decision"].as_str().unwrap(),
        r.publication.decision.as_str()
    );
    assert_eq!(r.publication.decision, r.computed_decision());

    let captured_rules: Vec<&str> = capture["publication"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_rules, r.computed_blocking_rule_ids());

    let captured_records: Vec<&str> = capture["publication"]["blocking_record_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_records, r.computed_blocking_record_ids());

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn import_layer_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "an import-layer failure on a still-stable subject must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
    }
    // The unattributed notebook import holds promotion on the provenance axis.
    let notebook = r
        .record("import-notebook-grid-engine")
        .expect("notebook record exists");
    assert_eq!(notebook.import_state, ImportState::NarrowedProvenance);
    assert!(blocking.contains(&notebook.record_id));
    // The ownerless data-rich import holds promotion on the ownership axis.
    let ownerless = r
        .record("import-data_rich-arrow-bridge")
        .expect("ownerless record exists");
    assert_eq!(ownerless.import_state, ImportState::NarrowedOwnership);
    assert!(blocking.contains(&ownerless.record_id));
    // The waived generator record is narrowed and visible, but gated upstream.
    let waived = r
        .record("import-review-generated-fixtures")
        .expect("waived record exists");
    assert_eq!(waived.import_state, ImportState::NarrowedGenerator);
    assert!(waived.is_waived());
    assert!(!blocking.contains(&waived.record_id));
    // The codec import already sits below the cutline (Beta): inherited.
    let beta = r
        .record("import-framework-sdk-codec")
        .expect("beta record exists");
    assert!(beta.import_state.is_narrowed());
    assert!(!beta.declares_at_or_above_cutline());
    assert!(!blocking.contains(&beta.record_id));
}

#[test]
fn stale_and_missing_proof_narrow_on_the_stale_axis() {
    let r = register();
    let stale = r
        .record("import-managed_depth-object-store")
        .expect("stale-proof record exists");
    assert_eq!(stale.import_state, ImportState::NarrowedStale);
    assert_eq!(stale.proof_packet.slo_state, FreshnessSloState::Breached);
    let missing = r
        .record("import-managed_depth-telemetry-shim")
        .expect("missing-proof record exists");
    assert_eq!(missing.import_state, ImportState::NarrowedStale);
    assert_eq!(missing.proof_packet.slo_state, FreshnessSloState::Missing);
}

#[test]
fn hidden_ownership_gap_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    rec.ownership.ownership_state =
        aureline_governance::m5_import_provenance_and_fork_review::OwnershipState::Ownerless;
    rec.ownership.update_owner_ref = String::new();
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::GapWithoutReason {
                reason: ImportReason::UpdateOwnerMissing,
                ..
            }
        )),
        "a hidden ownership gap must fail validation"
    );
}

#[test]
fn clean_surface_over_a_gapped_scan_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.import_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.surface_posture = Posture::Clear;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::ManifestScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn narrowed_record_above_the_cutline_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.import_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.publication.decision = PublicationDecision::Proceed;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, RegisterViolation::PublicationDecisionInconsistent)));
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/governance/m5-import-provenance-and-fork-review");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: ImportRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
