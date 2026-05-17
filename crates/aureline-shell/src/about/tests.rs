use std::path::Path;

use super::*;
use crate::service_health::{M3ClaimManifestSnapshot, ServiceHealthBetaSurface};

fn load_generated_manifest() -> M3ClaimManifestSnapshot {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/release/m3/claim_manifest.json");
    M3ClaimManifestSnapshot::load_from_path(path).expect("generated manifest must load")
}

fn load_fixture(name: &str) -> M3ClaimManifestSnapshot {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/release/beta_truth_cases")
        .join(name);
    M3ClaimManifestSnapshot::load_from_path(path).expect("fixture must load")
}

#[test]
fn release_truth_card_is_projection_only() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let card = HelpAboutReleaseTruthCard::project(&surface);

    assert_eq!(card.record_kind, HELP_ABOUT_RELEASE_TRUTH_CARD_RECORD_KIND);
    assert_eq!(
        card.schema_version,
        HELP_ABOUT_RELEASE_TRUTH_CARD_SCHEMA_VERSION
    );
    assert_eq!(card.manifest_id, manifest.manifest_id);
    assert_eq!(card.manifest_revision, manifest.manifest_revision);
    assert_eq!(card.milestone_id, "m3");
    assert!(!card.rows.is_empty());
    assert!(card.rows.len() <= surface.rows.len());
    // Card rows are a subset of the upstream surface rows whose help_about
    // channel binds with binding_status=required.
    for card_row in &card.rows {
        let surface_row = surface
            .rows
            .iter()
            .find(|r| r.row_id == card_row.row_id)
            .expect("card row must come from surface");
        let projection = surface_row
            .help_about_projection
            .as_ref()
            .expect("card rows MUST carry a help_about projection");
        assert_eq!(projection.binding_status, "required");
        assert_eq!(card_row.help_about_binding_status, "required");
        assert!(
            !card_row.compatibility_row_refs.is_empty(),
            "{} must carry compatibility refs",
            card_row.row_id
        );
    }
}

#[test]
fn release_truth_card_summary_matches_filtered_row_state() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let card = HelpAboutReleaseTruthCard::project(&surface);

    let computed_downgraded = card
        .rows
        .iter()
        .filter(|r| r.claim_posture_downgraded)
        .count() as u32;
    assert_eq!(card.downgraded_claim_row_count, computed_downgraded);

    let computed_downgraded_support =
        card.rows.iter().filter(|r| r.support_downgraded).count() as u32;
    assert_eq!(
        card.downgraded_support_row_count,
        computed_downgraded_support
    );
}

#[test]
fn release_truth_card_lights_honesty_marker_when_manifest_has_downgrades() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let card = HelpAboutReleaseTruthCard::project(&surface);
    assert!(card.honesty_marker_present);
}

#[test]
fn release_truth_card_lights_no_marker_on_protected_walk_fixture() {
    let manifest = load_fixture("protected_walk_current_manifest_snapshot.json");
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let card = HelpAboutReleaseTruthCard::project(&surface);
    assert!(!card.honesty_marker_present);
    assert_eq!(card.downgraded_claim_row_count, 0);
    assert_eq!(card.downgraded_support_row_count, 0);
    assert_eq!(card.evidence_stale_row_count, 0);
    assert_eq!(card.evidence_expired_row_count, 0);
}

#[test]
fn release_truth_card_flips_expired_on_stale_drill_fixture() {
    let manifest = load_fixture("failure_drill_stale_evidence.json");
    let surface = ServiceHealthBetaSurface::project(&manifest, "2030-01-01");
    let card = HelpAboutReleaseTruthCard::project(&surface);
    assert!(card.honesty_marker_present);
    assert!(card.evidence_expired_row_count >= 1 || card.evidence_stale_row_count >= 1);
}

#[test]
fn plaintext_renders_envelope_and_per_row_lines() {
    let manifest = load_generated_manifest();
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let card = HelpAboutReleaseTruthCard::project(&surface);
    let text = card.render_plaintext();
    assert!(text.contains("Help / About — release truth"));
    assert!(text.contains(&format!("Manifest: {}", card.manifest_id)));
    assert!(text.contains("Release channel:"));
    for row in &card.rows {
        assert!(
            text.contains(&row.row_id),
            "missing row {} from plaintext",
            row.row_id
        );
        for compatibility_ref in &row.compatibility_row_refs {
            assert!(
                text.contains(compatibility_ref),
                "missing compatibility ref {} from plaintext",
                compatibility_ref
            );
        }
    }
}
