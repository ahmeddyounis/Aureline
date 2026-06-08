//! Protected tests binding optional-surface qualification to the checked-in
//! register, validation capture, and negative fixtures.
//!
//! The positive case is the checked-in register. The capture cross-check proves
//! the typed model and the Python gate agree on the publication verdict,
//! surface-kind coverage, and packet-freshness counts. The negative cases prove
//! that absent packets, stale packets, claim ceilings, coverage drops, and
//! publication drift all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::optional_surface_qualification::{
    current_optional_surface_qualification, NarrowReason, OptionalSurfaceKind,
    OptionalSurfaceQualification, OptionalSurfaceQualificationViolation, SurfaceState,
    OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND, OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/optional_surface_qualification_validation_capture.json"
));

fn register() -> OptionalSurfaceQualification {
    current_optional_surface_qualification()
        .expect("checked-in optional-surface qualification register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let register = register();
    assert_eq!(
        register.schema_version,
        OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        register.record_kind,
        OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND
    );
    let violations = register.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let register = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(register.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_surfaces"].as_u64().unwrap() as usize,
        register.surfaces.len(),
        "capture surface count must match the model"
    );
    assert_eq!(
        summary["surfaces_qualified_stable"].as_u64().unwrap() as usize,
        register.surfaces_qualified_stable().len(),
        "capture qualified count must match the model"
    );
    assert_eq!(
        summary["surfaces_without_packet"].as_u64().unwrap() as usize,
        register.surfaces_without_packet().len(),
        "capture absent-packet count must match the model"
    );
    for (key, kind) in [
        (
            "opt_in_capability_surfaces",
            OptionalSurfaceKind::OptInCapability,
        ),
        (
            "optional_integration_surfaces",
            OptionalSurfaceKind::OptionalIntegration,
        ),
        (
            "secondary_platform_surfaces",
            OptionalSurfaceKind::SecondaryPlatform,
        ),
        (
            "experimental_preview_surfaces",
            OptionalSurfaceKind::ExperimentalPreview,
        ),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            register.surfaces_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        register.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        register.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        register.publication.decision,
        register.computed_publication_decision()
    );

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
fn register_qualifies_packet_backed_surfaces_without_narrowing() {
    let register = register();
    assert!(
        register.surfaces_narrowed().is_empty(),
        "clean register must not narrow optional surfaces"
    );
    assert_eq!(register.surfaces_without_packet().len(), 0);
}

#[test]
fn no_surface_renders_wider_than_its_claim_ceiling() {
    let register = register();
    for surface in &register.surfaces {
        assert!(
            surface.displayed_label.rank() <= surface.claim_label.rank(),
            "{} renders wider than its ceiling",
            surface.surface_id
        );
    }
}

#[test]
fn every_release_relevant_surface_is_covered() {
    let register = register();
    assert!(!register.release_relevant_surface_refs.is_empty());
    let covered: Vec<&str> = register
        .release_relevant_surfaces()
        .into_iter()
        .map(|surface| surface.surface_ref.as_str())
        .collect();
    for declared in &register.release_relevant_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-relevant row"
        );
    }
}

#[test]
fn absent_packet_rendered_qualified_fails() {
    let mut register = register();
    let surface = register
        .surfaces
        .iter_mut()
        .find(|surface| surface.renders_qualified())
        .expect("a qualified surface exists");
    surface.qualification_packet = None;
    surface.surface_state = SurfaceState::QualifiedStable;
    surface.displayed_label = surface.claim_label;
    surface.active_narrow_reasons.clear();
    register.summary = register.computed_summary();
    register.publication.decision = register.computed_publication_decision();
    register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
    register.publication.blocking_surface_ids = register.computed_blocking_surface_ids();

    assert!(
        register.validate().iter().any(|v| matches!(
            v,
            OptionalSurfaceQualificationViolation::QualifiedWithoutPacket { .. }
        )),
        "a surface with no packet may not render qualified"
    );
}

#[test]
fn qualified_surface_on_breached_packet_fails() {
    let mut register = register();
    let surface = register
        .surfaces
        .iter_mut()
        .find(|surface| surface.renders_qualified() && surface.has_packet())
        .expect("a qualified surface with a packet exists");
    surface
        .qualification_packet
        .as_mut()
        .expect("packet exists")
        .slo_state = FreshnessSloState::Breached;
    register.summary = register.computed_summary();

    assert!(
        register.validate().iter().any(|v| matches!(
            v,
            OptionalSurfaceQualificationViolation::QualifiedOnStalePacket { .. }
        )),
        "a qualified surface may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_decision_mismatch_fails() {
    let mut register = register();
    register.publication.decision = PromotionDecision::Hold;

    assert!(
        register.validate().iter().any(|v| matches!(
            v,
            OptionalSurfaceQualificationViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication decision must agree with computed rules"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/optional_surface_qualification");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let expected = case["expected_check_id"]
            .as_str()
            .expect("case names expected check id");
        if expected.starts_with("ceiling.") {
            continue;
        }
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: OptionalSurfaceQualification =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}

#[test]
fn support_projection_includes_constructed_narrow_reasons() {
    let mut register = register();
    let surface = register
        .surfaces
        .iter_mut()
        .find(|surface| surface.renders_qualified())
        .expect("a qualified surface exists");
    surface.qualification_packet = None;
    surface.surface_state = SurfaceState::NarrowedNoPacket;
    surface.displayed_label = StableClaimLevel::Beta;
    surface.active_narrow_reasons = vec![NarrowReason::QualificationPacketAbsent];
    let projection = register.support_export_projection();
    let narrowed = projection
        .surfaces
        .iter()
        .find(|surface| {
            surface
                .active_narrow_reasons
                .contains(&NarrowReason::QualificationPacketAbsent)
        })
        .expect("projection includes absent-packet narrowing");
    assert_eq!(narrowed.displayed_label, StableClaimLevel::Beta);
    assert!(!narrowed.renders_stable);
    assert!(!narrowed.has_packet);
}
