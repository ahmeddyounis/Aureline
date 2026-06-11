//! Protected fixture checks for the M5 command-parity audit.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/ux/m5/command-parity/` through the Rust types and asserts
//! the contract invariants. The report fixture is also asserted
//! bit-for-bit equal to the audit minted by
//! `seeded_m5_command_parity_audit`, and the markdown artifact under
//! `artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md` is
//! asserted bit-for-bit equal to the rendering, so the headless
//! inspector remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_command_registry::{
    seeded_m5_command_parity_audit, validate_m5_command_parity_audit, M5CommandParityAuditReport,
    M5CommandParitySupportExport, M5CoverageStatus, M5DiscoveryChannel,
    M5_COMMAND_PARITY_PUBLISHED_REPORT_REF, M5_COMMAND_PARITY_REPORT_RECORD_KIND,
    M5_COMMAND_PARITY_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/command-parity")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/discoverability-audits")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_report_is_bit_for_bit_equal_to_seed() {
    let on_disk: M5CommandParityAuditReport = load_json("report.json");
    let seeded = seeded_m5_command_parity_audit();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded audit");
    assert_eq!(seeded.record_kind, M5_COMMAND_PARITY_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_COMMAND_PARITY_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_COMMAND_PARITY_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: M5CommandParityAuditReport = load_json("report.json");
    validate_m5_command_parity_audit(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_claims_every_required_channel() {
    let report: M5CommandParityAuditReport = load_json("report.json");
    assert!(report.every_required_channel_claimed());
    for required in M5DiscoveryChannel::required_channels() {
        let any_claimed = report.rows.iter().any(|row| {
            row.channels.iter().any(|projection| {
                projection.channel == required
                    && projection.coverage_status == M5CoverageStatus::Claimed
            })
        });
        assert!(
            any_claimed,
            "no claimed row for required channel {}",
            required.as_str()
        );
    }
}

#[test]
fn fixture_help_anchor_index_is_complete_and_non_empty() {
    let report: M5CommandParityAuditReport = load_json("report.json");
    assert_eq!(report.help_anchor_index.len(), report.rows.len());
    for entry in &report.help_anchor_index {
        assert!(
            !entry.help_anchor_ref.trim().is_empty(),
            "help anchor for {} must be non-empty",
            entry.command_id
        );
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: M5CommandParityAuditReport = load_json("report.json");
    let export: M5CommandParitySupportExport = load_json("support_export.json");
    let expected =
        M5CommandParitySupportExport::from_report(export.support_export_id.clone(), report.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(
            export.case_ids.contains(&row.descriptor.command_id),
            "support export must quote command id {}",
            row.descriptor.command_id
        );
        assert!(
            export
                .case_ids
                .contains(&row.descriptor.descriptor_revision_ref),
            "support export must quote descriptor revision {}",
            row.descriptor.descriptor_revision_ref
        );
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_m5_command_parity_audit();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("m5_command_parity_audit.md"))
        .expect("published m5_command_parity_audit.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_command_parity_audit.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- report-md`",
    );
}

#[test]
fn published_doc_links_every_channel_and_artifact() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/ux/m5/command_parity_audit.md");
    let body =
        std::fs::read_to_string(&doc_path).expect("published command_parity_audit doc must exist");
    for required in M5DiscoveryChannel::required_channels() {
        assert!(
            body.contains(required.as_str()),
            "doc must quote required channel {}",
            required.as_str()
        );
    }
    assert!(body.contains("artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md"));
    assert!(body.contains("fixtures/ux/m5/command-parity/report.json"));
    assert!(body.contains("schemas/commands/m5-command-descriptor-diff.schema.json"));
    assert!(body.contains("tools/ci/m5/command_parity_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_m5_command_parity_audit();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- compact`",
    );
}
