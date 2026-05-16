use std::path::Path;

use super::*;

fn manifest_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/release/m3/claim_manifest.json")
}

fn fixture_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/release/beta_truth_cases")
        .join(name)
}

fn load_generated_manifest() -> M3ClaimManifestSnapshot {
    M3ClaimManifestSnapshot::load_from_path(manifest_path())
        .expect("generated m3 claim manifest must load")
}

fn load_fixture_manifest(name: &str) -> M3ClaimManifestSnapshot {
    M3ClaimManifestSnapshot::load_from_path(fixture_path(name)).expect("fixture must load")
}

#[test]
fn generated_manifest_loads_and_record_kind_matches() {
    let manifest = load_generated_manifest();
    assert_eq!(
        manifest.record_kind,
        M3ClaimManifestSnapshot::EXPECTED_RECORD_KIND
    );
    assert_eq!(manifest.milestone_id, "m3");
    assert!(!manifest.rows.is_empty());
}

#[test]
fn projection_at_manifest_as_of_is_deterministic_and_quotes_envelope() {
    let manifest = load_generated_manifest();
    let surface_one = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let surface_two = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    assert_eq!(surface_one, surface_two);
    assert_eq!(
        surface_one.record_kind,
        SERVICE_HEALTH_BETA_SURFACE_RECORD_KIND
    );
    assert_eq!(
        surface_one.schema_version,
        SERVICE_HEALTH_BETA_SURFACE_SCHEMA_VERSION
    );
    assert_eq!(surface_one.manifest_id, manifest.manifest_id);
    assert_eq!(surface_one.manifest_revision, manifest.manifest_revision);
    assert_eq!(surface_one.milestone_id, "m3");
    assert_eq!(
        surface_one.release_channel_scope_token,
        manifest.release_channel_scope
    );
    assert_eq!(
        surface_one.summary.total_row_count as usize,
        manifest.rows.len()
    );
    assert_eq!(
        surface_one.as_of_for_freshness_evaluation,
        surface_one.as_of
    );
}

#[test]
fn projection_against_generated_manifest_resolves_known_vocabularies() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);

    // No row should fall into the unknown-token sentinel for the closed
    // vocabularies; the manifest schema constrains them already, but the
    // projection should still resolve them cleanly.
    for row in &surface.rows {
        assert_ne!(
            row.row_kind,
            BetaRowKindClass::UnknownRowKind,
            "row_id={}",
            row.row_id
        );
        assert_ne!(
            row.claim_posture.declared,
            ClaimPostureClass::UnknownClaimPosture,
            "row_id={} declared",
            row.row_id,
        );
        assert_ne!(
            row.claim_posture.effective,
            ClaimPostureClass::UnknownClaimPosture,
            "row_id={} effective",
            row.row_id,
        );
        assert_ne!(
            row.support.declared,
            SupportClassClass::UnknownSupportClass,
            "row_id={} support declared",
            row.row_id,
        );
        assert_ne!(
            row.support.effective,
            SupportClassClass::UnknownSupportClass,
            "row_id={} support effective",
            row.row_id,
        );
        assert_ne!(
            row.lifecycle_label,
            LifecycleLabelClass::UnknownLifecycleLabel,
            "row_id={} lifecycle",
            row.row_id,
        );
        assert_ne!(
            row.provenance.label,
            ProvenanceLabelClass::UnknownProvenanceLabel,
            "row_id={} provenance",
            row.row_id,
        );
        assert_ne!(
            row.freshness.badge_class,
            FreshnessBadgeClass::UnknownFreshnessBadge,
            "row_id={} freshness badge",
            row.row_id,
        );
    }
}

#[test]
fn generated_manifest_lights_honesty_marker_via_known_downgraded_rows() {
    // The generated manifest already carries several rows whose declared
    // posture is `claim_bearing` but whose effective posture is `limited`.
    // The projection MUST surface them as downgraded; the surface MUST
    // light its global honesty marker.
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);

    assert!(surface.summary.downgraded_claim_row_count > 0);
    assert!(surface.honesty_marker_present);

    let downgraded: Vec<_> = surface
        .rows
        .iter()
        .filter(|row| row.claim_posture.downgraded)
        .collect();
    assert!(!downgraded.is_empty());
    for row in downgraded {
        assert!(!row.claim_posture.active_downgrade_reasons.is_empty());
        assert!(row.honesty_marker_present);
    }
}

#[test]
fn rows_for_help_about_and_service_health_are_consistent_on_canonical_rows() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);

    let help_about_rows = surface.rows_for_help_about();
    let service_health_rows = surface.rows_for_service_health();

    // Every row that the manifest requires on help_about MUST also be
    // required on service_health for the beta truth lane to stay
    // consistent. Rows that omit one or the other are accepted only when
    // both channels agree on omission; the projection's
    // `required_projection_missing` flag is the chrome's authoritative
    // signal.
    let help_ids: std::collections::BTreeSet<_> =
        help_about_rows.iter().map(|r| r.row_id.clone()).collect();
    let service_ids: std::collections::BTreeSet<_> = service_health_rows
        .iter()
        .map(|r| r.row_id.clone())
        .collect();
    assert_eq!(help_ids, service_ids);
}

#[test]
fn stale_evaluation_against_far_future_date_flips_every_row_to_expired() {
    let manifest = load_generated_manifest();
    // 365 days past the latest evidence date in the manifest is well past
    // every row's review window.
    let surface = ServiceHealthBetaSurface::project(&manifest, "2099-01-01");

    assert_eq!(
        surface.summary.evidence_expired_row_count,
        surface.summary.total_row_count,
    );
    assert!(surface.honesty_marker_present);
    for row in &surface.rows {
        assert_eq!(row.freshness.state, FreshnessStateClass::EvidenceExpired);
        assert!(row.evidence_expired());
    }
}

#[test]
fn freshness_evaluation_at_evidence_date_is_current_on_warm_or_authoritative_rows() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project(&manifest, &manifest.as_of);
    // At the manifest's own as_of, every row whose badge is
    // `authoritative_live` or `warm_cached` must read `current` rather
    // than a stale or expired state.
    for row in &surface.rows {
        if matches!(
            row.freshness.badge_class,
            FreshnessBadgeClass::AuthoritativeLive | FreshnessBadgeClass::WarmCached,
        ) {
            assert_eq!(
                row.freshness.state,
                FreshnessStateClass::Current,
                "row {} should be current at manifest as_of, got {:?}",
                row.row_id,
                row.freshness.state,
            );
        }
    }
}

#[test]
fn plaintext_renderer_includes_envelope_and_per_row_lines() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let text = surface.render_plaintext();
    assert!(text.contains("Service-health beta surface"));
    assert!(text.contains(&format!("Manifest: {}", surface.manifest_id)));
    assert!(text.contains("Release channel:"));
    assert!(text.contains("Honesty marker:"));
    // The plaintext block lists every row exactly once.
    for row in &surface.rows {
        assert!(
            text.contains(&row.row_id),
            "missing row {} from plaintext",
            row.row_id
        );
    }
}

#[test]
fn fixture_protected_walk_does_not_light_honesty_marker() {
    let manifest = load_fixture_manifest("protected_walk_current_manifest_snapshot.json");
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    assert_eq!(surface.summary.downgraded_claim_row_count, 0);
    assert_eq!(surface.summary.downgraded_support_row_count, 0);
    assert_eq!(surface.summary.evidence_stale_row_count, 0);
    assert_eq!(surface.summary.evidence_expired_row_count, 0);
    assert_eq!(surface.summary.required_projection_missing_row_count, 0);
    assert_eq!(surface.summary.copy_field_drift_row_count, 0);
    assert!(!surface.honesty_marker_present);
}

#[test]
fn fixture_stale_evidence_drill_marks_every_row_expired() {
    let manifest = load_fixture_manifest("failure_drill_stale_evidence.json");
    // Use a future as_of so each row's evidence is past its review
    // window. The fixture's evidence dates are far enough in the past
    // that 2030-01-01 trips the expired threshold.
    let surface = ServiceHealthBetaSurface::project(&manifest, "2030-01-01");
    assert!(surface.honesty_marker_present);
    assert!(surface.summary.evidence_expired_row_count >= 1);
    for row in &surface.rows {
        assert!(row.freshness.state.is_honest_warning());
    }
}

#[test]
fn fixture_downgraded_claim_row_lights_posture_and_reasons() {
    let manifest = load_fixture_manifest("failure_drill_downgraded_claim_row.json");
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    assert!(surface.honesty_marker_present);
    assert!(surface.summary.downgraded_claim_row_count >= 1);

    let first = &surface.rows[0];
    assert!(first.claim_posture.downgraded);
    assert!(!first.claim_posture.active_downgrade_reasons.is_empty());
    assert!(first.honesty_marker_present);
}

#[test]
fn day_arithmetic_is_consistent_for_known_iso_dates() {
    // 2026-05-15 to 2026-06-15 is 31 days.
    let a = parse_iso_date("2026-05-15").expect("parse");
    let b = parse_iso_date("2026-06-15").expect("parse");
    assert_eq!(days_between(a, b), 31);
    // 2026-05-15 to 2026-05-15 is zero days.
    let c = parse_iso_date("2026-05-15").expect("parse");
    assert_eq!(days_between(a, c), 0);
    // 2026-05-15 to 2025-05-15 is -365 days.
    let d = parse_iso_date("2025-05-15").expect("parse");
    assert_eq!(days_between(a, d), -365);
}

#[test]
fn missing_required_record_kind_is_rejected() {
    let payload = r#"{
        "schema_version": 1,
        "record_kind": "not_a_manifest",
        "manifest_id": "claim_manifest:x",
        "manifest_revision": 1,
        "milestone_id": "m3",
        "release_channel_scope": "beta",
        "manifest_state": "draft",
        "as_of": "2026-05-15",
        "generated_at": "2026-05-15T00:00:00Z",
        "owner": "@me",
        "backup_owner": null,
        "backup_waiver": null,
        "consuming_surfaces": [],
        "rows": []
    }"#;
    let err = M3ClaimManifestSnapshot::from_bytes(payload.as_bytes()).unwrap_err();
    assert!(matches!(
        err,
        ManifestLoadError::SchemaMismatch {
            expected_record_kind: M3ClaimManifestSnapshot::EXPECTED_RECORD_KIND,
            ..
        }
    ));
}
