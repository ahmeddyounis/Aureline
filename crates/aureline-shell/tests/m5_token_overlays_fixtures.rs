//! Protected fixture checks for the M5 token-overlay round-trip audit.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/ux/m5/token-overlay-sync-import/` through the Rust types and
//! asserts the contract invariants. The report fixture is asserted bit-for-bit
//! equal to the audit minted by `seeded_token_overlay_portability`, and the
//! markdown artifact under
//! `artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::token_overlays::{
    seeded_token_overlay_portability, validate_token_overlay_portability, EntryDisposition,
    TokenOverlayPortabilityReport, TokenOverlaySupportExport, ValueState,
    TOKEN_OVERLAY_PUBLISHED_REPORT_REF, TOKEN_OVERLAY_REPORT_RECORD_KIND,
    TOKEN_OVERLAY_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/token-overlay-sync-import")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/token-overlay-roundtrip")
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
    let on_disk: TokenOverlayPortabilityReport = load_json("report.json");
    let seeded = seeded_token_overlay_portability();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded audit");
    assert_eq!(seeded.record_kind, TOKEN_OVERLAY_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        TOKEN_OVERLAY_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        TOKEN_OVERLAY_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: TokenOverlayPortabilityReport = load_json("report.json");
    validate_token_overlay_portability(&report).expect("fixture report must validate");
    assert!(report.report_clean);
    assert!(report.blocking_findings.is_empty());
}

#[test]
fn fixture_round_trip_is_lossless_and_preserves_unsupported() {
    let report: TokenOverlayPortabilityReport = load_json("report.json");
    assert!(report.round_trip_lossless);
    for stage in &report.round_trip.stages {
        assert_eq!(
            stage.dropped_count, 0,
            "stage {} dropped entries",
            stage.stage_id
        );
        assert_eq!(
            stage.rewritten_count, 0,
            "stage {} rewrote entries",
            stage.stage_id
        );
    }
    // Every unsupported override (the imported chart slot and the deprecated
    // extension alias) survives as a disclosed downgrade rather than being
    // dropped.
    let downgraded_survivors = report
        .round_trip
        .entry_traces
        .iter()
        .filter(|trace| trace.disposition == EntryDisposition::Downgraded && trace.survived)
        .count();
    assert_eq!(
        downgraded_survivors,
        report.round_trip.unsupported_preserved_count
    );
    assert!(report.round_trip.unsupported_preserved_count >= 1);
    for trace in &report.round_trip.entry_traces {
        assert!(trace.survived, "trace {} did not survive", trace.entry_ref);
        assert_eq!(trace.origin_scope, trace.final_scope);
    }
}

#[test]
fn fixture_resolution_is_inspectable() {
    let report: TokenOverlayPortabilityReport = load_json("report.json");
    for resolved in &report.resolved_tokens {
        let max_rank = report
            .all_entries()
            .filter(|entry| entry.token_ref == resolved.token_ref)
            .map(|entry| entry.declared_scope.precedence_rank())
            .max()
            .expect("each resolved token must have a contributing entry");
        assert_eq!(
            resolved.winning_scope.precedence_rank(),
            max_rank,
            "wrong winner for {}",
            resolved.token_ref
        );
        let contributors = report
            .all_entries()
            .filter(|entry| entry.token_ref == resolved.token_ref)
            .count();
        assert_eq!(
            resolved.shadowed.len() + 1,
            contributors,
            "shadowed list incomplete for {}",
            resolved.token_ref
        );
        assert!(!resolved.precedence_explained.trim().is_empty());
    }
}

#[test]
fn fixture_every_unmapped_entry_is_inert_and_disclosed() {
    let report: TokenOverlayPortabilityReport = load_json("report.json");
    for entry in report.all_entries() {
        if entry.value_state == ValueState::Unmapped {
            assert!(entry.downgrade_class.is_downgrade());
            assert!(entry.unmapped_source_slot_ref.is_some());
        }
        if entry.value_state == ValueState::Deprecated {
            assert!(entry.deprecated_replacement_ref.is_some());
        }
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: TokenOverlayPortabilityReport = load_json("report.json");
    let export: TokenOverlaySupportExport = load_json("support_export.json");
    let expected =
        TokenOverlaySupportExport::from_report(export.support_export_id.clone(), report.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.appearance_session_ref));
    for overlay in &report.overlays {
        assert!(
            export.case_ids.contains(&overlay.overlay_id),
            "support export must quote overlay {}",
            overlay.overlay_id
        );
        for entry in &overlay.entries {
            assert!(
                export.case_ids.contains(&entry.entry_id),
                "support export must quote entry {}",
                entry.entry_id
            );
        }
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_token_overlay_portability();
    let rendered = report.render_markdown();
    let on_disk =
        std::fs::read_to_string(artifacts_root().join("m5_token_overlay_roundtrip_audit.md"))
            .expect("published m5_token_overlay_roundtrip_audit.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_token_overlay_roundtrip_audit.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- report-md`",
    );
}

#[test]
fn published_doc_links_artifacts_and_gate() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/m5/token-overlays-and-scope.md");
    let body = std::fs::read_to_string(&doc_path).expect("published token-overlays doc must exist");
    assert!(body
        .contains("artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md"));
    assert!(body.contains("fixtures/ux/m5/token-overlay-sync-import/report.json"));
    assert!(body.contains("schemas/ux/token-overlay.schema.json"));
    assert!(body.contains("schemas/design/token_overlay.schema.json"));
    assert!(body.contains("tools/ci/m5/token_overlay_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_token_overlay_portability();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- compact`",
    );
}
