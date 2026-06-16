//! Protected tests binding the typed versioned-boundary-manifest register to the
//! checked-in artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every
//! family has exactly one versioned manifest and every narrowing reason is wired; the
//! capture cross-check proves the typed model and the CI gate agree on the publication
//! verdict, the release-link parity, and the published/narrowed counts; the narrowing
//! check proves a manifest-layer failure on a still-stable family holds promotion while
//! inherited and waived narrowings stay gated upstream; the negative cases mutate a
//! parsed copy and read the checked-in fixtures to prove that an over-claim past the
//! release evidence, an undisclosed residual dependency, a narrowed manifest that stays
//! above the cutline, and a proceed verdict while a rule fires all fail validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::LifecycleLabel;
use aureline_governance::m5_versioned_boundary_manifests::{
    current_m5_versioned_boundary_manifests, BoundaryManifestRegister, GuardrailKind, M5Family,
    ManifestReason, ManifestState, PublicationDecision, RegisterViolation, ReleaseLinkState,
    M5_VERSIONED_BOUNDARY_MANIFESTS_RECORD_KIND, M5_VERSIONED_BOUNDARY_MANIFESTS_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-versioned-boundary-manifests_validation_capture.json"
));

fn register() -> BoundaryManifestRegister {
    current_m5_versioned_boundary_manifests().expect("checked-in register parses into the model")
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
        M5_VERSIONED_BOUNDARY_MANIFESTS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_VERSIONED_BOUNDARY_MANIFESTS_RECORD_KIND);
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
        let m = r
            .manifest_for_family(family)
            .unwrap_or_else(|| panic!("family {} must have a manifest", family.as_str()));
        assert!(
            !m.manifest_version.trim().is_empty(),
            "family {} manifest must be versioned",
            family.as_str()
        );
        for kind in GuardrailKind::ALL {
            assert!(
                m.guardrails.iter().any(|g| g.kind == kind),
                "manifest {} must declare guardrail {}",
                m.manifest_id,
                kind.as_str()
            );
        }
    }
    for reason in ManifestReason::ALL {
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
    let states: std::collections::BTreeSet<ManifestState> =
        r.manifests.iter().map(|m| m.manifest_state).collect();
    assert!(states.contains(&ManifestState::Published));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&ManifestState::NarrowedParity));
    assert!(states.contains(&ManifestState::NarrowedReleaseLink));
    assert!(states.contains(&ManifestState::NarrowedDisclosure));
    assert!(states.contains(&ManifestState::NarrowedGuardrail));
    assert!(states.contains(&ManifestState::NarrowedStale));

    let reasons: std::collections::BTreeSet<ManifestReason> = r
        .manifests
        .iter()
        .flat_map(|m| m.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ManifestReason::ReleaseParityBroken));
    assert!(reasons.contains(&ManifestReason::UndisclosedResidualDependency));
    assert!(reasons.contains(&ManifestReason::GuardrailUnsatisfied));
}

#[test]
fn residual_proprietary_dependencies_are_disclosed_or_narrow() {
    let r = register();
    // The register actually carries residual dependencies (not an empty axis), and
    // any undisclosed dependency narrows its manifest on the disclosure axis.
    assert!(
        r.summary.total_residual_dependencies > 0,
        "the register must record residual proprietary/hosted dependencies"
    );
    for m in &r.manifests {
        if m.has_undisclosed_dependency() {
            assert!(
                m.has_active_reason(ManifestReason::UndisclosedResidualDependency),
                "manifest {} hides a residual dependency without narrowing",
                m.manifest_id
            );
        }
        for dep in &m.residual_dependencies {
            if dep.disclosed {
                assert!(
                    !dep.disclosure_surface_refs.is_empty(),
                    "disclosed dependency {} must name a disclosure surface",
                    dep.dependency_id
                );
            }
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
    assert_eq!(u(&summary["total_manifests"]), computed.total_manifests);
    assert_eq!(
        u(&summary["manifests_published"]),
        computed.manifests_published
    );
    assert_eq!(
        u(&summary["manifests_narrowed"]),
        computed.manifests_narrowed
    );
    assert_eq!(u(&summary["state_published"]), computed.state_published);
    assert_eq!(
        u(&summary["state_narrowed_release_link"]),
        computed.state_narrowed_release_link
    );
    assert_eq!(
        u(&summary["state_narrowed_parity"]),
        computed.state_narrowed_parity
    );
    assert_eq!(
        u(&summary["state_narrowed_disclosure"]),
        computed.state_narrowed_disclosure
    );
    assert_eq!(
        u(&summary["state_narrowed_guardrail"]),
        computed.state_narrowed_guardrail
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
        u(&summary["manifests_on_active_waiver"]),
        computed.manifests_on_active_waiver
    );
    assert_eq!(
        u(&summary["total_residual_dependencies"]),
        computed.total_residual_dependencies
    );
    assert_eq!(
        u(&summary["residual_dependencies_undisclosed"]),
        computed.residual_dependencies_undisclosed
    );
    assert_eq!(u(&summary["total_guardrails"]), computed.total_guardrails);
    assert_eq!(
        u(&summary["guardrails_unsatisfied"]),
        computed.guardrails_unsatisfied
    );
    assert_eq!(u(&summary["manifests_linked"]), computed.manifests_linked);
    assert_eq!(
        u(&summary["total_active_reasons"]),
        computed.total_active_reasons
    );
    assert_eq!(u(&summary["rules_firing"]), computed.rules_firing);

    let parity = &capture["release_link_parity"];
    let computed_parity = r.computed_release_link_parity();
    assert_eq!(
        u(&parity["families_in_parity"]),
        computed_parity.families_in_parity
    );
    assert_eq!(
        u(&parity["families_link_broken"]),
        computed_parity.families_link_broken
    );
    assert_eq!(
        u(&parity["families_parity_broken"]),
        computed_parity.families_parity_broken
    );
    assert_eq!(
        parity["all_families_linked"].as_bool(),
        Some(computed_parity.all_families_linked)
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

    let captured_manifests: Vec<&str> = capture["publication"]["blocking_manifest_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_manifests, r.computed_blocking_manifest_ids());

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
fn manifest_layer_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_manifest_ids();
    assert!(
        !blocking.is_empty(),
        "a manifest-layer failure on a still-stable family must hold promotion"
    );
    for id in &blocking {
        let m = r.manifest(id).expect("blocking manifest exists");
        assert!(m.release_blocking);
        assert!(m.declares_at_or_above_cutline());
        assert!(!m.is_waived());
    }
    // The over-claiming notebook family holds promotion on the parity axis.
    let notebook = r
        .manifest_for_family(M5Family::Notebook)
        .expect("notebook manifest exists");
    assert_eq!(notebook.manifest_state, ManifestState::NarrowedParity);
    assert_eq!(
        notebook.release_link.link_state,
        ReleaseLinkState::ParityBroken
    );
    assert!(blocking.contains(&notebook.manifest_id));
    // The undisclosed-dependency AI-adjacent family holds promotion on the disclosure axis.
    let ai = r
        .manifest_for_family(M5Family::AiAdjacent)
        .expect("ai-adjacent manifest exists");
    assert_eq!(ai.manifest_state, ManifestState::NarrowedDisclosure);
    assert!(blocking.contains(&ai.manifest_id));
    // The waived review family is narrowed and visible, but gated upstream.
    let review = r
        .manifest_for_family(M5Family::Review)
        .expect("review manifest exists");
    assert!(review.manifest_state.is_narrowed());
    assert!(review.is_waived());
    assert!(!blocking.contains(&review.manifest_id));
    // The data-rich family already sits below the cutline (Beta): inherited.
    let data_rich = r
        .manifest_for_family(M5Family::DataRich)
        .expect("data-rich manifest exists");
    assert!(data_rich.manifest_state.is_narrowed());
    assert!(!data_rich.declares_at_or_above_cutline());
    assert!(!blocking.contains(&data_rich.manifest_id));
}

#[test]
fn published_over_claim_fails() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.is_published() && m.declared_label == LifecycleLabel::Stable)
        .expect("a published stable manifest exists");
    m.release_link.train_label = LifecycleLabel::Beta;
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::PublishedOverClaimsReleaseEvidence { .. }
        )),
        "a published manifest greener than its release evidence must fail validation"
    );
}

#[test]
fn undisclosed_dependency_without_a_reason_fails() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.is_published() && !m.residual_dependencies.is_empty())
        .expect("a published manifest with a disclosed dependency exists");
    m.residual_dependencies[0].disclosed = false;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::GapWithoutReason {
            reason: ManifestReason::UndisclosedResidualDependency,
            ..
        }
    )));
}

#[test]
fn narrowed_manifest_above_the_cutline_fails() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.manifest_state.is_narrowed())
        .expect("a narrowed manifest exists");
    m.effective_label = LifecycleLabel::Stable;
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
    let fixtures_dir = repo_root().join("fixtures/governance/m5-versioned-boundary-manifests");
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
        let candidate: BoundaryManifestRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
