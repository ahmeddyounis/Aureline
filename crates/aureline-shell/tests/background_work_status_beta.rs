//! Fixture replay for shell background-work status projections.

use std::path::{Path, PathBuf};

use aureline_runtime::seeded_resource_governor_snapshot;
use aureline_shell::background_work_status::{
    BackgroundWorkStatusBundle, BACKGROUND_WORK_STATUS_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("resource_governor_and_queue_truth")
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    shell_support_export_record_kind: String,
    raw_private_material_excluded: bool,
    required_shell_banner_work: Vec<String>,
    required_pressure_card_dimensions: Vec<String>,
}

fn read_case(name: &str) -> CaseFixture {
    let path = fixture_root().join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn shell_projection_preserves_runtime_queue_and_pressure_truth() {
    let fixture = read_case("support_export_parity.json");
    assert_eq!(fixture.record_kind, "resource_governor_truth_fixture");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.case_id, "support_export_parity");

    let snapshot = seeded_resource_governor_snapshot(
        "resource-governor:snapshot:shell-fixture",
        "workspace.resource-governor.shell-fixture",
        "profile.standard",
        "2026-05-18T20:05:00Z",
    );
    let bundle = BackgroundWorkStatusBundle::from_snapshot(&snapshot);
    let validation = bundle.validate(&snapshot);
    assert!(
        validation.is_ok(),
        "violations: {:?}",
        validation.violations
    );

    assert_eq!(
        bundle.support_export.record_kind,
        BACKGROUND_WORK_STATUS_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(
        bundle.support_export.record_kind,
        fixture.expect.shell_support_export_record_kind
    );
    assert_eq!(
        bundle.support_export.raw_private_material_excluded,
        fixture.expect.raw_private_material_excluded
    );

    for work in fixture.expect.required_shell_banner_work {
        assert!(
            bundle.deferred_work_banners.iter().any(|banner| banner
                .paused_or_denied_work
                .iter()
                .any(|label| label == &work)),
            "missing banner work {work}"
        );
    }
    for dimension in fixture.expect.required_pressure_card_dimensions {
        assert!(
            bundle
                .pressure_cards
                .iter()
                .any(|card| card.pressure_dimension == dimension),
            "missing pressure card {dimension}"
        );
    }
    assert_eq!(bundle.support_export.lane_rows, bundle.lane_rows);
    assert_eq!(bundle.support_export.pressure_cards, bundle.pressure_cards);
}
