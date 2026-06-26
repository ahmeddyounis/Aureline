//! Protected fixture checks for the M5 content-design certification capstone.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/release/m5-content-design-certification/` through the Rust types and
//! asserts the contract invariants. The packet, dashboard, and support-export
//! fixtures are also asserted bit-for-bit equal to the records minted by the
//! seed, and the published markdown report under
//! `artifacts/release/m5-content-design-certification/m5_content_design_certification.md`
//! and the published dashboard under `artifacts/content/m5-content-truth-dashboard.json`
//! are asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::content_design_certification::{
    seeded_content_design_certification_packet, validate_content_design_certification_packet,
    ContentDesignCertificationPacket, ContentDesignCertificationSupportExport, ContentRowStatus,
    ContentTruthDashboard, M5_CONTENT_CERT_PACKET_RECORD_KIND,
    M5_CONTENT_CERT_PUBLISHED_REPORT_REF, M5_CONTENT_CERT_SHARED_CONTRACT_REF,
    REQUIRED_OBJECT_KINDS,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/release/m5-content-design-certification")
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
    let on_disk: ContentDesignCertificationPacket = load_json("packet.json");
    let seeded = seeded_content_design_certification_packet();
    assert_eq!(
        on_disk, seeded,
        "fixture packet diverged from seeded packet"
    );
    assert_eq!(seeded.record_kind, M5_CONTENT_CERT_PACKET_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_CONTENT_CERT_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_CONTENT_CERT_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_packet_passes_validation_and_is_clean() {
    let packet: ContentDesignCertificationPacket = load_json("packet.json");
    validate_content_design_certification_packet(&packet).expect("fixture packet must validate");
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn fixture_packet_covers_every_governed_object() {
    let packet: ContentDesignCertificationPacket = load_json("packet.json");
    assert_eq!(packet.rows.len(), REQUIRED_OBJECT_KINDS.len());
    for kind in REQUIRED_OBJECT_KINDS {
        assert!(packet.row(kind).is_some(), "packet omits {}", kind.as_str());
    }
}

#[test]
fn fixture_status_is_the_derived_auto_narrowed_value() {
    let packet: ContentDesignCertificationPacket = load_json("packet.json");
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_kind.as_str()
        );
        assert_eq!(row.stale_proof_causes, row.recompute_causes());
        if !matches!(row.derived_status, ContentRowStatus::Green) {
            assert!(
                !row.narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed row {} hides its reason",
                row.object_kind.as_str()
            );
        }
    }
}

#[test]
fn fixture_dashboard_matches_packet_projection() {
    let packet: ContentDesignCertificationPacket = load_json("packet.json");
    let on_disk: ContentTruthDashboard = load_json("dashboard.json");
    assert_eq!(
        on_disk,
        packet.dashboard(),
        "dashboard diverged from projection"
    );
}

#[test]
fn fixture_support_export_quotes_packet_and_waiver_refs() {
    let packet: ContentDesignCertificationPacket = load_json("packet.json");
    let export: ContentDesignCertificationSupportExport = load_json("support_export.json");
    let expected = ContentDesignCertificationSupportExport::from_packet(
        export.support_export_id.clone(),
        packet.clone(),
    );
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
    let seeded = seeded_content_design_certification_packet();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- compact`",
    );
}

#[test]
fn published_packet_md_matches_seeded_rendering() {
    let packet = seeded_content_design_certification_packet();
    let rendered = packet.render_markdown();
    let on_disk = std::fs::read_to_string(repo_root().join(
        "artifacts/release/m5-content-design-certification/m5_content_design_certification.md",
    ))
    .expect("published m5_content_design_certification.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published markdown diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- markdown`",
    );
}

#[test]
fn published_dashboard_json_matches_seeded_projection() {
    let packet = seeded_content_design_certification_packet();
    let rendered = packet.dashboard().export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/content/m5-content-truth-dashboard.json"),
    )
    .expect("published m5-content-truth-dashboard.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published dashboard diverged from seeded projection -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- dashboard`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_objects() {
    let body = std::fs::read_to_string(
        repo_root().join("docs/release/m5-content-design-certification.md"),
    )
    .expect("published content-design-certification doc must exist");
    assert!(body.contains(
        "artifacts/release/m5-content-design-certification/m5_content_design_certification.md"
    ));
    assert!(body.contains("artifacts/content/m5-content-truth-dashboard.json"));
    assert!(body.contains("fixtures/release/m5-content-design-certification/packet.json"));
    assert!(body.contains("schemas/release/m5-content-design-certification.schema.json"));
    assert!(body.contains("tools/ci/m5/content_design_certification_check.py"));
    for kind in REQUIRED_OBJECT_KINDS {
        assert!(
            body.contains(kind.as_str()),
            "doc must name object {}",
            kind.as_str()
        );
    }
}
