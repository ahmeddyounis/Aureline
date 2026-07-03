//! Protected fixture checks for the M5 advisory-claim downgrade certification capstone.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/security/m5-advisory-claim-downgrade-certification/` through the Rust types and
//! asserts the contract invariants. The packet, dashboard, and support-export fixtures are also
//! asserted bit-for-bit equal to the records minted by the seed, and the published markdown report
//! under `artifacts/security/m5-advisory-claim-downgrade-certification.md`, the published packet
//! under `artifacts/release/m5-advisory-claim-downgrade-certification-proof/packet.json`, the
//! published dashboard under
//! `artifacts/release/m5-advisory-claim-downgrade-certification-proof/dashboard.json`, and the
//! published CSV under
//! `artifacts/release/m5-advisory-claim-downgrade-certification-proof/matrix.csv` are asserted
//! bit-for-bit equal to the rendering, so the headless emitter remains the only mint-from-truth
//! path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_advisory_claim_downgrade_certification::{
    seeded_m5_advisory_claim_downgrade_certification_packet,
    validate_m5_advisory_claim_downgrade_certification_packet, AdvisoryClaimDashboard,
    AdvisoryClaimPacket, AdvisoryClaimStatus, AdvisoryClaimSupportExport,
    M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_RECORD_KIND,
    M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_REPORT_REF,
    M5_ADVISORY_CLAIM_DOWNGRADE_SHARED_CONTRACT_REF, REQUIRED_PROFILES,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/m5-advisory-claim-downgrade-certification")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_packet_is_bit_for_bit_equal_to_seed() {
    let on_disk: AdvisoryClaimPacket = load_json("packet.json");
    let seeded = seeded_m5_advisory_claim_downgrade_certification_packet();
    assert_eq!(
        on_disk, seeded,
        "fixture packet diverged from seeded packet"
    );
    assert_eq!(
        seeded.record_kind,
        M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_RECORD_KIND
    );
    assert_eq!(
        seeded.shared_contract_ref,
        M5_ADVISORY_CLAIM_DOWNGRADE_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_packet_passes_validation_and_is_clean() {
    let packet: AdvisoryClaimPacket = load_json("packet.json");
    validate_m5_advisory_claim_downgrade_certification_packet(&packet)
        .expect("fixture packet must validate");
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn fixture_packet_covers_every_profile() {
    let packet: AdvisoryClaimPacket = load_json("packet.json");
    assert_eq!(packet.rows.len(), REQUIRED_PROFILES.len());
    for profile in REQUIRED_PROFILES {
        assert!(
            packet.row(profile).is_some(),
            "packet omits {}",
            profile.as_str()
        );
    }
}

#[test]
fn fixture_status_states_and_causes_are_the_derived_values() {
    let packet: AdvisoryClaimPacket = load_json("packet.json");
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.profile.as_str()
        );
        assert_eq!(row.claim_states, row.recompute_claim_states());
        assert_eq!(row.claim_causes, row.recompute_causes());
        if !matches!(row.derived_status, AdvisoryClaimStatus::Green) {
            assert!(
                !row.narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed row {} hides its reason",
                row.profile.as_str()
            );
            assert!(
                !row.claim_states.is_empty(),
                "narrowed row {} collapsed into a generic degraded state",
                row.profile.as_str()
            );
        }
    }
}

#[test]
fn fixture_every_row_evaluates_all_families_and_channels() {
    let packet: AdvisoryClaimPacket = load_json("packet.json");
    for row in &packet.rows {
        assert!(
            row.families_complete(),
            "row {} does not evaluate all six claimed advisory families",
            row.profile.as_str()
        );
        assert!(
            row.channels_complete(),
            "row {} does not project into all five claim surfaces",
            row.profile.as_str()
        );
    }
}

#[test]
fn fixture_dashboard_matches_packet_projection() {
    let packet: AdvisoryClaimPacket = load_json("packet.json");
    let on_disk: AdvisoryClaimDashboard = load_json("dashboard.json");
    assert_eq!(
        on_disk,
        packet.dashboard(),
        "dashboard diverged from projection"
    );
}

#[test]
fn fixture_support_export_quotes_packet_and_waiver_refs() {
    let packet: AdvisoryClaimPacket = load_json("packet.json");
    let export: AdvisoryClaimSupportExport = load_json("support_export.json");
    let expected =
        AdvisoryClaimSupportExport::from_packet(export.support_export_id.clone(), packet.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_m5_advisory_claim_downgrade_certification_packet();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- compact`",
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let rendered = packet.render_markdown();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/security/m5-advisory-claim-downgrade-certification.md"),
    )
    .expect("published m5-advisory-claim-downgrade-certification.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published markdown diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- markdown`",
    );
}

#[test]
fn published_packet_json_matches_seed() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let rendered = packet.export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root()
            .join("artifacts/release/m5-advisory-claim-downgrade-certification-proof/packet.json"),
    )
    .expect("published packet.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published packet diverged from seed -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- packet`",
    );
}

#[test]
fn published_dashboard_json_matches_seeded_projection() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let rendered = packet.dashboard().export_safe_json();
    let on_disk =
        std::fs::read_to_string(repo_root().join(
            "artifacts/release/m5-advisory-claim-downgrade-certification-proof/dashboard.json",
        ))
        .expect("published dashboard.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published dashboard diverged from seeded projection -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- dashboard`",
    );
}

#[test]
fn published_csv_matches_seeded_rendering() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let rendered = packet.render_matrix_csv();
    let on_disk = std::fs::read_to_string(
        repo_root()
            .join("artifacts/release/m5-advisory-claim-downgrade-certification-proof/matrix.csv"),
    )
    .expect("published matrix.csv must exist");
    assert_eq!(
        on_disk, rendered,
        "published CSV diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- csv`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_profiles() {
    let body = std::fs::read_to_string(
        repo_root().join("docs/security/m5_advisory_claim_downgrade_certification_contract.md"),
    )
    .expect("published advisory-claim-downgrade certification contract must exist");
    assert!(body.contains("artifacts/security/m5-advisory-claim-downgrade-certification.md"));
    assert!(body
        .contains("artifacts/release/m5-advisory-claim-downgrade-certification-proof/packet.json"));
    assert!(body.contains(
        "artifacts/release/m5-advisory-claim-downgrade-certification-proof/dashboard.json"
    ));
    assert!(
        body.contains("fixtures/security/m5-advisory-claim-downgrade-certification/packet.json")
    );
    assert!(body.contains("schemas/security/m5-advisory-claim-downgrade-certification.schema.json"));
    for profile in REQUIRED_PROFILES {
        assert!(
            body.contains(profile.as_str()),
            "doc must name profile {}",
            profile.as_str()
        );
    }
}
