//! Protected fixture checks for the M5 OS-attention parity audit.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/ux/m5_os_notifications_and_badges/` through the Rust types and
//! asserts the contract invariants. The report fixture is asserted bit-for-bit
//! equal to the audit minted by `seeded_m5_os_attention_report`, and the
//! markdown artifact under `artifacts/ux/m5/os-notification-and-reopen.md` is
//! asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_os_notifications_and_badges::{
    seeded_m5_os_attention_report, validate_m5_os_attention_report, M5OsAttentionGuarantee,
    M5OsAttentionReport, M5OsAttentionSupportExport, M5OsQualificationStatus,
    M5_OS_ATTENTION_PUBLISHED_REPORT_REF, M5_OS_ATTENTION_REPORT_RECORD_KIND,
    M5_OS_ATTENTION_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5_os_notifications_and_badges")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5")
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
    let on_disk: M5OsAttentionReport = load_json("report.json");
    let seeded = seeded_m5_os_attention_report();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded audit");
    assert_eq!(seeded.record_kind, M5_OS_ATTENTION_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_OS_ATTENTION_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_OS_ATTENTION_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: M5OsAttentionReport = load_json("report.json");
    validate_m5_os_attention_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_covers_every_guarantee() {
    let report: M5OsAttentionReport = load_json("report.json");
    for guarantee in M5OsAttentionGuarantee::required_guarantees() {
        let any_qualified = report.rows.iter().any(|surface| {
            surface.bindings.iter().any(|binding| {
                binding.guarantee == guarantee
                    && binding.qualification_status == M5OsQualificationStatus::Qualified
            })
        });
        assert!(
            any_qualified,
            "guarantee {} not qualified",
            guarantee.as_str()
        );
    }
}

#[test]
fn fixture_support_export_is_bit_for_bit_equal_to_seed() {
    let on_disk: M5OsAttentionSupportExport = load_json("support_export.json");
    let seeded = M5OsAttentionSupportExport::from_report(
        on_disk.support_export_id.clone(),
        seeded_m5_os_attention_report(),
    );
    assert_eq!(on_disk, seeded, "fixture support export diverged from seed");
}

#[test]
fn markdown_artifact_is_bit_for_bit_equal_to_render() {
    let path = artifacts_root().join("os-notification-and-reopen.md");
    let on_disk = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    let rendered = seeded_m5_os_attention_report().render_markdown();
    assert_eq!(on_disk, rendered, "markdown artifact diverged from render");
}
