//! Protected tests binding the typed M04-182 register to the checked-in artifact,
//! the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the typed
//! model and the Python gate agree on the publication verdict, the
//! known-limit/support-window/ownership coverage counts, and the packet-freshness counts;
//! the negative cases mutate a parsed copy and the checked-in fixtures to prove that an
//! entry which fails to narrow, a stale packet held, an entry carried wider than its
//! public claim's ceiling, and a publication verdict that disagrees with the firing rules
//! all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication::{
    current_stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication,
    StabilizeKind, StabilizeState,
    StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication,
    StabilizeViolation, STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_RECORD_KIND,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication_validation_capture.json"
));

fn register() -> StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication
{
    current_stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication()
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
    let reg = register();
    assert_eq!(
        reg.schema_version,
        STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_SCHEMA_VERSION
    );
    assert_eq!(
        reg.record_kind,
        STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_RECORD_KIND
    );
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_kind() {
    let reg = register();
    for kind in StabilizeKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let reg = register();
    assert!(!reg.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = reg
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &reg.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(reg.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_published_stable"].as_u64().unwrap() as usize,
        reg.rows_published_stable().len(),
        "capture published count must match the model"
    );
    for (key, kind) in [
        ("known_limit_entries", StabilizeKind::KnownLimit),
        ("support_window_entries", StabilizeKind::PublicSupportWindow),
        ("ownership_entries", StabilizeKind::StableLineOwnership),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            reg.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        reg.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        reg.publication.decision,
        reg.computed_publication_decision()
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
fn register_narrows_an_entry_under_a_still_stable_claim() {
    let reg = register();
    let narrowed = reg.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.publishes_stable()
            && row.publication_state != StabilizeState::NarrowedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking entry under a still-stable claim"
    );
}

#[test]
fn register_narrows_a_support_window_for_expiry() {
    let reg = register();
    let expired = reg
        .rows
        .iter()
        .find(|row| row.publication_state == StabilizeState::NarrowedSupportExpired)
        .expect("the register must show a support-window expiry narrowing");
    assert_eq!(expired.kind, StabilizeKind::PublicSupportWindow);
    assert!(!expired.publishes_stable());
}

#[test]
fn register_narrows_an_ownership_for_missing_record() {
    let reg = register();
    let missing = reg
        .rows
        .iter()
        .find(|row| row.publication_state == StabilizeState::NarrowedOwnershipMissing)
        .expect("the register must show an ownership-missing narrowing");
    assert_eq!(missing.kind, StabilizeKind::StableLineOwnership);
    assert!(!missing.publishes_stable());
}

#[test]
fn narrowing_entry_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.publication_state == StabilizeState::NarrowedStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a narrowed-stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    let violations = reg.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, StabilizeViolation::PublishedLabelNotNarrowed { .. })),
        "a narrowing row that does not narrow must fail validation: {violations:#?}"
    );
}

#[test]
fn held_entry_on_breached_packet_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.publication_state == StabilizeState::Stabilized
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a stabilized row");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    reg.summary = reg.computed_summary();
    let violations = reg.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, StabilizeViolation::HeldOnStalePacket { .. })),
        "a held row on a breached packet must fail validation: {violations:#?}"
    );
}

#[test]
fn published_wider_than_claim_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.claim_label == StableClaimLevel::Beta)
        .expect("register has a beta-claim row");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    let violations = reg.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, StabilizeViolation::PublishedWiderThanClaim { .. })),
        "a row published wider than its claim must fail validation: {violations:#?}"
    );
}

#[test]
fn publication_verdict_mismatch_fails() {
    let mut reg = register();
    let original = reg.publication.decision;
    reg.publication.decision = match original {
        PromotionDecision::Proceed => PromotionDecision::Hold,
        PromotionDecision::Hold => PromotionDecision::Proceed,
    };
    let violations = reg.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            StabilizeViolation::PublicationDecisionInconsistent { .. }
        )),
        "a mismatched publication verdict must fail validation: {violations:#?}"
    );
}

#[test]
fn checked_in_artifact_exists_on_disk() {
    let root = repo_root();
    let path = root.join("artifacts/release/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication.json");
    assert!(path.exists(), "checked-in artifact must exist on disk");
}

#[test]
fn schema_exists_on_disk() {
    let root = repo_root();
    let path = root.join("schemas/release/stabilize-the-known-limits-matrix-public-support-windows-and-stable-line-ownership-publication.schema.json");
    assert!(path.exists(), "schema must exist on disk");
}

#[test]
fn proof_packet_exists_on_disk() {
    let root = repo_root();
    let path = root.join("artifacts/release/m4/stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication_proof_packet.md");
    assert!(path.exists(), "proof packet must exist on disk");
}

#[test]
fn docs_exist_on_disk() {
    let root = repo_root();
    let path = root.join("docs/m4/stabilize-the-known-limits-matrix-public-support-windows-and-stable-line-ownership-publication.md");
    assert!(path.exists(), "docs must exist on disk");
}
