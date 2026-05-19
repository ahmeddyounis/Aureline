//! Fixture replay for the deployment-profile continuity corpus.
//!
//! The fixtures live under
//! [`fixtures/deployment/m3/profile_truth/`](../../../fixtures/deployment/m3/profile_truth)
//! and
//! [`fixtures/deployment/m3/control_plane_vs_data_plane/`](../../../fixtures/deployment/m3/control_plane_vs_data_plane).
//! Each per-case and per-drill JSON file is a
//! [`DeploymentProfilePage`] record; every page MUST pass
//! `DeploymentProfilePage::audit()` with an empty defect set. The
//! release-evidence excerpt at
//! `artifacts/release/m3/deployment_profile_conformance_report.md`
//! and the residual-dependency matrix at
//! `artifacts/release/m3/residual_dependency_matrix.json` are rendered
//! from the same seeded packet and MUST match the on-disk artifacts
//! byte-for-byte.

use aureline_shell::deployment_profile::corpus::{
    render_deployment_profile_conformance_report_markdown,
    render_residual_dependency_matrix_json, seeded_deployment_profile_corpus_packet,
    validate_deployment_profile_corpus_packet, DeploymentProfileCorpusCase,
    DeploymentProfileCorpusPacket, DeploymentProfileOutageDrill, OutageDrillClass,
    SurfaceLensClass,
};
use aureline_shell::deployment_profile::{
    ConsumerSurfaceClass, DependencyClass, DeploymentProfileClass, DeploymentProfilePage,
    PostureClass,
};

const PROFILE_TRUTH_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deployment/m3/profile_truth"
);

const DRILLS_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/deployment/m3/control_plane_vs_data_plane/drills"
);

const ARTIFACTS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../artifacts/release/m3");

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn load_packet() -> DeploymentProfileCorpusPacket {
    let body = read(&format!("{PROFILE_TRUTH_DIR}/packet.json"));
    serde_json::from_str(&body).expect("packet.json must parse as DeploymentProfileCorpusPacket")
}

#[test]
fn fixture_packet_matches_seeded_builder() {
    let on_disk = load_packet();
    let live = seeded_deployment_profile_corpus_packet();
    assert_eq!(
        on_disk, live,
        "packet.json must match the seeded corpus packet"
    );
    validate_deployment_profile_corpus_packet(&on_disk)
        .expect("seeded corpus packet must validate");
}

#[test]
fn every_case_fixture_round_trips_and_passes_audit() {
    let packet = seeded_deployment_profile_corpus_packet();
    for case in &packet.corpus_cases {
        let body = read(&format!(
            "{PROFILE_TRUTH_DIR}/cases/{}.json",
            case.case_id
        ));
        let on_disk: DeploymentProfileCorpusCase = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("case {} did not parse: {err}", case.case_id));
        assert_eq!(&on_disk, case, "case {} differs from seeded packet", case.case_id);
        let defects = on_disk.page.audit();
        assert!(
            defects.is_empty(),
            "case {} produced defects: {defects:?}",
            case.case_id
        );
    }
}

#[test]
fn every_drill_fixture_round_trips_and_passes_audit() {
    let packet = seeded_deployment_profile_corpus_packet();
    for drill in &packet.outage_drills {
        let body = read(&format!("{DRILLS_DIR}/{}.json", drill.drill_id));
        let on_disk: DeploymentProfileOutageDrill = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("drill {} did not parse: {err}", drill.drill_id));
        assert_eq!(
            &on_disk, drill,
            "drill {} differs from seeded packet",
            drill.drill_id
        );
        let defects = on_disk.page.audit();
        assert!(
            defects.is_empty(),
            "drill {} produced defects: {defects:?}",
            drill.drill_id
        );
    }
}

#[test]
fn conformance_report_matches_seeded_render() {
    let packet = seeded_deployment_profile_corpus_packet();
    let on_disk = read(&format!(
        "{ARTIFACTS_DIR}/deployment_profile_conformance_report.md"
    ));
    let rendered = render_deployment_profile_conformance_report_markdown(&packet);
    assert_eq!(
        on_disk, rendered,
        "deployment_profile_conformance_report.md is out of sync with the seeded packet; \
         re-run aureline_shell_deployment_profile_corpus to refresh."
    );
}

#[test]
fn residual_dependency_matrix_matches_seeded_render() {
    let packet = seeded_deployment_profile_corpus_packet();
    let on_disk = read(&format!("{ARTIFACTS_DIR}/residual_dependency_matrix.json"));
    let rendered = render_residual_dependency_matrix_json(&packet);
    assert_eq!(
        on_disk, rendered,
        "residual_dependency_matrix.json is out of sync with the seeded packet; \
         re-run aureline_shell_deployment_profile_corpus to refresh."
    );
}

#[test]
fn every_marketed_profile_has_at_least_one_case() {
    let packet = load_packet();
    for profile in [
        DeploymentProfileClass::IndividualLocal,
        DeploymentProfileClass::SelfHosted,
        DeploymentProfileClass::EnterpriseOnline,
        DeploymentProfileClass::AirGapped,
        DeploymentProfileClass::ManagedCloud,
    ] {
        assert!(
            packet
                .corpus_cases
                .iter()
                .any(|c| c.deployment_profile == profile),
            "no corpus case covers profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn every_outage_drill_class_has_at_least_one_drill() {
    let packet = load_packet();
    for class in OutageDrillClass::all() {
        assert!(
            packet.outage_drills.iter().any(|d| d.drill_class == class),
            "no outage drill covers class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_surface_lens_is_exercised_by_at_least_one_case() {
    let packet = load_packet();
    for lens in SurfaceLensClass::all() {
        assert!(
            packet
                .corpus_cases
                .iter()
                .any(|c| c.surface_lens_class == lens),
            "no corpus case covers surface lens {}",
            lens.as_str()
        );
    }
}

#[test]
fn air_gapped_cases_never_route_through_companion_surface_or_omit_mirror_artifact_row() {
    let packet = load_packet();
    for case in &packet.corpus_cases {
        if case.deployment_profile == DeploymentProfileClass::AirGapped {
            assert!(
                !case.page
                    .profile_summary
                    .consumer_surfaces
                    .contains(&ConsumerSurfaceClass::CompanionSurface),
                "case {} routed air-gapped through companion_surface",
                case.case_id
            );
            assert!(
                !case.page.mirror_offline_artifact_rows.is_empty(),
                "case {} air-gapped row omitted mirror/offline artifact rows",
                case.case_id
            );
        }
    }
}

#[test]
fn required_vendor_bound_residual_rows_flag_vendor_dependence() {
    let packet = load_packet();
    let mut all_pages: Vec<&DeploymentProfilePage> = Vec::new();
    for c in &packet.corpus_cases {
        all_pages.push(&c.page);
    }
    for d in &packet.outage_drills {
        all_pages.push(&d.page);
    }
    for page in all_pages {
        for row in &page.residual_dependency_rows {
            if row.posture_class == PostureClass::Required
                && row.dependency_class.is_vendor_bound_when_required()
            {
                assert!(
                    row.vendor_or_public_dependence,
                    "row {} required vendor-bound dep {} missing vendor flag",
                    row.row_id,
                    row.dependency_class.as_str()
                );
            }
        }
    }
}

#[test]
fn every_dependency_class_is_present_in_the_matrix() {
    let packet = load_packet();
    let present: Vec<&'static str> = packet
        .residual_dependency_matrix
        .rows
        .iter()
        .map(|r| r.dependency_class.as_str())
        .collect();
    for dep in [
        DependencyClass::SignIn,
        DependencyClass::PackageRegistry,
        DependencyClass::RemoteMirror,
        DependencyClass::RemoteAgent,
        DependencyClass::SymbolService,
        DependencyClass::AiProvider,
        DependencyClass::PolicyBundle,
        DependencyClass::DocsPack,
        DependencyClass::BrowserHandoff,
        DependencyClass::CompanionNotificationChannel,
        DependencyClass::HostedControlPlaneReachability,
    ] {
        assert!(
            present.contains(&dep.as_str()),
            "matrix missing dependency class {}",
            dep.as_str()
        );
    }
}
