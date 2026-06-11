//! Protected fixture checks for the M5 notification-route audit.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/ux/m5/notification-dedupe/` through the Rust types and asserts the
//! contract invariants. The report fixture is also asserted bit-for-bit equal
//! to the audit minted by `seeded_m5_notification_routes_audit`, and the
//! markdown artifact under
//! `artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_notification_routes::{
    seeded_m5_notification_routes_audit, validate_m5_notification_routes, M5NotificationGuarantee,
    M5NotificationRouteReport, M5NotificationStatus, M5NotificationSupportExport,
    M5_NOTIFICATION_PUBLISHED_REPORT_REF, M5_NOTIFICATION_REPORT_RECORD_KIND,
    M5_NOTIFICATION_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/notification-dedupe")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/quiet-hours-and-privacy")
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
    let on_disk: M5NotificationRouteReport = load_json("report.json");
    let seeded = seeded_m5_notification_routes_audit();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded audit");
    assert_eq!(seeded.record_kind, M5_NOTIFICATION_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_NOTIFICATION_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_NOTIFICATION_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: M5NotificationRouteReport = load_json("report.json");
    validate_m5_notification_routes(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_qualifies_every_required_guarantee() {
    let report: M5NotificationRouteReport = load_json("report.json");
    assert!(report.every_required_guarantee_qualified());
    for required in M5NotificationGuarantee::required_guarantees() {
        let any_qualified = report.rows.iter().any(|source| {
            source.bindings.iter().any(|binding| {
                binding.guarantee == required
                    && binding.qualification_status == M5NotificationStatus::Qualified
            })
        });
        assert!(
            any_qualified,
            "no qualified source for required guarantee {}",
            required.as_str()
        );
    }
}

#[test]
fn fixture_reopen_anchor_index_is_complete_and_non_empty() {
    let report: M5NotificationRouteReport = load_json("report.json");
    assert_eq!(report.reopen_anchor_index.len(), report.rows.len());
    for entry in &report.reopen_anchor_index {
        assert!(
            !entry.reopen_anchor_ref.trim().is_empty(),
            "reopen anchor for {} must be non-empty",
            entry.source_id
        );
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: M5NotificationRouteReport = load_json("report.json");
    let export: M5NotificationSupportExport = load_json("support_export.json");
    let expected =
        M5NotificationSupportExport::from_report(export.support_export_id.clone(), report.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    for source in &report.rows {
        assert!(
            export.case_ids.contains(&source.descriptor.source_id),
            "support export must quote source id {}",
            source.descriptor.source_id
        );
        assert!(
            export
                .case_ids
                .contains(&source.descriptor.descriptor_revision_ref),
            "support export must quote descriptor revision {}",
            source.descriptor.descriptor_revision_ref
        );
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_m5_notification_routes_audit();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("m5_notification_routes_audit.md"))
        .expect("published m5_notification_routes_audit.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_notification_routes_audit.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- report-md`",
    );
}

#[test]
fn published_doc_links_every_guarantee_and_artifact() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/m5/notification-privacy-and-badges.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published notification-privacy-and-badges doc must exist");
    for required in M5NotificationGuarantee::required_guarantees() {
        assert!(
            body.contains(required.as_str()),
            "doc must quote required guarantee {}",
            required.as_str()
        );
    }
    assert!(
        body.contains("artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md")
    );
    assert!(body.contains("fixtures/ux/m5/notification-dedupe/report.json"));
    assert!(body.contains("schemas/ux/m5-notification-envelope-diff.schema.json"));
    assert!(body.contains("tools/ci/m5/notification_routes_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_m5_notification_routes_audit();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- compact`",
    );
}
