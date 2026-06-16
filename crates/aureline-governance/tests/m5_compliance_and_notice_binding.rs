//! Protected tests binding the typed repository-compliance and notice-binding register
//! to the checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every family
//! has an artifact-family record and every narrowing reason is wired; the capture
//! cross-check proves the typed model and the CI gate agree on the promotion verdict, the
//! scan/surface parity, and the cleared/narrowed counts; the no-mask check proves a green,
//! bound SBOM still narrows on a notice/licensing gap and that scan and surface agree on
//! every record; the narrowing check proves a compliance-layer failure on a still-stable
//! subject holds promotion while inherited and waived narrowings stay gated upstream; the
//! negative cases mutate a parsed copy and read the checked-in fixtures to prove that a
//! hidden licensing gap, a clean surface over a gapped scan, a narrowed record that stays
//! above the cutline, and a proceed verdict while a rule fires all fail validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{FreshnessSloState, LifecycleLabel};
use aureline_governance::m5_compliance_and_notice_binding::{
    current_m5_compliance_and_notice_binding, CompliancePosture, ComplianceReason,
    ComplianceRegister, ComplianceState, ControlDimension, PublicationDecision, RegisterViolation,
    SbomBindingState, ScopeKind, M5_COMPLIANCE_AND_NOTICE_BINDING_RECORD_KIND,
    M5_COMPLIANCE_AND_NOTICE_BINDING_SCHEMA_VERSION,
};
use aureline_governance::m5_versioned_boundary_manifests::M5Family;

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-compliance-and-notice-binding_validation_capture.json"
));

fn register() -> ComplianceRegister {
    current_m5_compliance_and_notice_binding().expect("checked-in register parses into the model")
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
        M5_COMPLIANCE_AND_NOTICE_BINDING_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_COMPLIANCE_AND_NOTICE_BINDING_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_family_and_every_reason_has_a_rule() {
    let r = register();
    for family in M5Family::ALL {
        let rec = r.artifact_family_record(family).unwrap_or_else(|| {
            panic!(
                "family {} must have an artifact-family record",
                family.as_str()
            )
        });
        assert_eq!(rec.scope_kind, ScopeKind::ArtifactFamily);
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
    for reason in ComplianceReason::ALL {
        assert!(
            r.rules.iter().any(|rule| rule.trigger_reason == reason),
            "reason {} must be watched by a rule",
            reason.as_str()
        );
    }
    // Docs packs and mirrored outputs are covered alongside artifact families.
    assert!(r
        .records
        .iter()
        .any(|x| x.scope_kind == ScopeKind::DocsPack));
    assert!(r
        .records
        .iter()
        .any(|x| x.scope_kind == ScopeKind::MirroredOutput));
}

#[test]
fn keeps_per_axis_state_not_one_global_flag() {
    let r = register();
    let states: std::collections::BTreeSet<ComplianceState> =
        r.records.iter().map(|x| x.compliance_state).collect();
    assert!(states.contains(&ComplianceState::Cleared));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&ComplianceState::NarrowedProvenance));
    assert!(states.contains(&ComplianceState::NarrowedLicensing));
    assert!(states.contains(&ComplianceState::NarrowedNotice));
    assert!(states.contains(&ComplianceState::NarrowedSbom));
    assert!(states.contains(&ComplianceState::NarrowedMirror));
    assert!(states.contains(&ComplianceState::NarrowedStale));

    let reasons: std::collections::BTreeSet<ComplianceReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ComplianceReason::DcoSignoffMissing));
    assert!(reasons.contains(&ComplianceReason::LicensingCoverageIncomplete));
    assert!(reasons.contains(&ComplianceReason::NoticeInventoryPartial));
}

#[test]
fn green_sbom_never_masks_a_notice_or_licensing_gap() {
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
    // A record carries a present, bound SBOM yet still narrows on a notice gap.
    let masked = r.records.iter().find(|rec| {
        rec.sbom.spdx_primary_present
            && rec.sbom.binding_state == SbomBindingState::Bound
            && rec.has_active_reason(ComplianceReason::NoticeInventoryPartial)
    });
    let masked =
        masked.expect("a record with a green, bound SBOM that still narrows on a notice gap");
    assert_eq!(masked.compliance_state, ComplianceState::NarrowedNotice);
    assert_eq!(masked.surface_posture, CompliancePosture::GapsFound);
}

#[test]
fn dco_cla_and_reuse_spdx_truth_is_recorded() {
    let r = register();
    // The provenance and licensing axes actually carry gaps (not empty axes).
    assert!(
        r.summary.provenance_gaps > 0,
        "the register must record a DCO/CLA gap"
    );
    assert!(
        r.summary.licensing_gaps > 0,
        "the register must record a REUSE/SPDX gap"
    );
    // SPDX is the primary output and CycloneDX export is visible on every subject.
    assert_eq!(r.summary.spdx_primary_present, r.records.len());
    assert!(r.summary.cyclonedx_export_available > 0);
    for rec in &r.records {
        if rec.provenance.dco_gap() {
            assert!(rec.has_active_reason(ComplianceReason::DcoSignoffMissing));
            assert!(rec.provenance.unsigned_commit_count > 0);
        }
        if rec.licensing.exception_undocumented() {
            assert!(rec.has_active_reason(ComplianceReason::LicenseExceptionUndocumented));
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
        u(&summary["state_narrowed_licensing"]),
        computed.state_narrowed_licensing
    );
    assert_eq!(
        u(&summary["state_narrowed_notice"]),
        computed.state_narrowed_notice
    );
    assert_eq!(
        u(&summary["state_narrowed_sbom"]),
        computed.state_narrowed_sbom
    );
    assert_eq!(
        u(&summary["state_narrowed_mirror"]),
        computed.state_narrowed_mirror
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
    assert_eq!(u(&summary["licensing_gaps"]), computed.licensing_gaps);
    assert_eq!(u(&summary["notice_gaps"]), computed.notice_gaps);
    assert_eq!(u(&summary["sbom_gaps"]), computed.sbom_gaps);
    assert_eq!(u(&summary["mirror_gaps"]), computed.mirror_gaps);
    assert_eq!(
        u(&summary["spdx_primary_present"]),
        computed.spdx_primary_present
    );
    assert_eq!(
        u(&summary["cyclonedx_export_available"]),
        computed.cyclonedx_export_available
    );
    assert_eq!(u(&summary["notices_complete"]), computed.notices_complete);
    assert_eq!(
        u(&summary["total_active_reasons"]),
        computed.total_active_reasons
    );
    assert_eq!(u(&summary["rules_firing"]), computed.rules_firing);

    let parity = &capture["scan_surface_parity"];
    let computed_parity = r.computed_scan_surface_parity();
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
fn compliance_layer_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "a compliance-layer failure on a still-stable subject must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
    }
    // The DCO-gap notebook family holds promotion on the provenance axis.
    let notebook = r
        .artifact_family_record(M5Family::Notebook)
        .expect("notebook record exists");
    assert_eq!(
        notebook.compliance_state,
        ComplianceState::NarrowedProvenance
    );
    assert!(blocking.contains(&notebook.record_id));
    // The licensing-gap AI-adjacent family holds promotion on the licensing axis.
    let ai = r
        .artifact_family_record(M5Family::AiAdjacent)
        .expect("ai-adjacent record exists");
    assert_eq!(ai.compliance_state, ComplianceState::NarrowedLicensing);
    assert!(blocking.contains(&ai.record_id));
    // The waived review family is narrowed and visible, but gated upstream.
    let review = r
        .artifact_family_record(M5Family::Review)
        .expect("review record exists");
    assert_eq!(review.compliance_state, ComplianceState::NarrowedSbom);
    assert!(review.is_waived());
    assert!(!blocking.contains(&review.record_id));
    // The data-rich family already sits below the cutline (Beta): inherited.
    let data_rich = r
        .artifact_family_record(M5Family::DataRich)
        .expect("data-rich record exists");
    assert!(data_rich.compliance_state.is_narrowed());
    assert!(!data_rich.declares_at_or_above_cutline());
    assert!(!blocking.contains(&data_rich.record_id));
}

#[test]
fn hidden_licensing_gap_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    rec.licensing.files_total += 1;
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::GapWithoutReason {
                reason: ComplianceReason::LicensingCoverageIncomplete,
                ..
            }
        )),
        "a hidden file-level licensing gap must fail validation"
    );
}

#[test]
fn green_surface_over_a_gapped_scan_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.compliance_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.surface_posture = CompliancePosture::Clear;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::ScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn narrowed_record_above_the_cutline_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.compliance_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn stale_proof_narrows_on_the_stale_axis() {
    let r = register();
    let companion = r
        .artifact_family_record(M5Family::Companion)
        .expect("companion record exists");
    assert_eq!(companion.compliance_state, ComplianceState::NarrowedStale);
    assert_eq!(
        companion.proof_packet.slo_state,
        FreshnessSloState::Breached
    );
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
    let fixtures_dir = repo_root().join("fixtures/governance/m5-compliance-and-notice-binding");
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
        let candidate: ComplianceRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
