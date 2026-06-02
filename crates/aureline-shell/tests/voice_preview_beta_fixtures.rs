//! Protected fixture checks for the bounded voice-preview surface.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/ux/m3/voice_preview_and_privacy/` through the Rust types and
//! asserts the contract invariants. The page fixture is asserted
//! bit-for-bit equal to the page minted by
//! `seeded_voice_preview_beta_page`, the support-export fixture is
//! asserted equal to the wrapper, the compact fixture and the published
//! markdown artifact are asserted equal to the seeded rendering, and the
//! companion doc is asserted to back-link the canonical schemas, fixtures,
//! artifact, and CI gate, so the headless inspector remains the only
//! mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::voice::{
    seeded_voice_preview_beta_page, validate_voice_preview_beta_page, NoBypassGuards,
    VoiceClaimPosture, VoicePreviewBetaPage, VoicePreviewSupportExport,
    VOICE_PREVIEW_PAGE_RECORD_KIND, VOICE_PREVIEW_PUBLISHED_REPORT_REF,
    VOICE_PREVIEW_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m3/voice_preview_and_privacy")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m3")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_page_is_bit_for_bit_equal_to_seed() {
    let on_disk: VoicePreviewBetaPage = load_json("page.json");
    let seeded = seeded_voice_preview_beta_page();
    assert_eq!(on_disk, seeded, "fixture page diverged from seeded page");
    assert_eq!(seeded.record_kind, VOICE_PREVIEW_PAGE_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        VOICE_PREVIEW_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        VOICE_PREVIEW_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_page_passes_validation_and_is_clean() {
    let page: VoicePreviewBetaPage = load_json("page.json");
    validate_voice_preview_beta_page(&page).expect("fixture page must validate");
    assert!(page.page_clean);
    assert_eq!(page.summary.total_blocking_findings, 0);
}

#[test]
fn fixture_page_proves_modes_and_postures() {
    let page: VoicePreviewBetaPage = load_json("page.json");
    assert!(page.summary.claimed_row_count >= 1, "needs a claimed row");
    assert!(page.summary.labs_row_count >= 1, "needs a labs row");
    assert!(page
        .summary
        .voice_modes_present
        .iter()
        .any(|mode| mode == "command_mode_active"));
    assert!(page
        .summary
        .voice_modes_present
        .iter()
        .any(|mode| mode == "dictation_mode_active"));
}

#[test]
fn claimed_rows_are_reachable_and_narratable_with_explicit_activation() {
    let page: VoicePreviewBetaPage = load_json("page.json");
    for row in &page.rows {
        if row.claim_posture.is_claimed() {
            assert!(row.command_mode_explicit, "{}", row.row_id);
            assert!(row.dictation_mode_explicit, "{}", row.row_id);
            assert!(row.keyboard_reachable, "{}", row.row_id);
            assert!(row.screen_reader_narratable, "{}", row.row_id);
            assert!(row.mic_pill.is_some(), "{}", row.row_id);
            assert!(
                row.default_activation_class.is_explicit(),
                "{} must default to explicit activation",
                row.row_id
            );
        }
    }
}

#[test]
fn high_impact_resolutions_cannot_bypass_preview_or_guards() {
    let page: VoicePreviewBetaPage = load_json("page.json");
    let mut saw_high_impact = false;
    for row in &page.rows {
        for resolution in &row.command_resolutions {
            if resolution.is_high_impact() {
                saw_high_impact = true;
                assert!(resolution.preview_required, "{}", resolution.resolution_id);
                assert_eq!(
                    resolution.no_bypass_guards,
                    NoBypassGuards::strict(),
                    "{} must keep strict no-bypass guards",
                    resolution.resolution_id
                );
                assert!(resolution.canonical_command_id.is_some());
                assert_eq!(
                    resolution.keyboard_equivalent_command_id, resolution.canonical_command_id,
                    "{} must reach the same keyboard command",
                    resolution.resolution_id
                );
            }
        }
    }
    assert!(
        saw_high_impact,
        "fixture must exercise a high-impact resolution"
    );
}

#[test]
fn labs_rows_stay_suppressed() {
    let page: VoicePreviewBetaPage = load_json("page.json");
    let mut saw_labs = false;
    for row in &page.rows {
        if row.claim_posture == VoiceClaimPosture::LabsUnadvertised {
            saw_labs = true;
            assert!(row.command_resolutions.is_empty(), "{}", row.row_id);
            if let Some(pill) = &row.mic_pill {
                assert!(!pill.capture_active, "{}", row.row_id);
            }
        }
    }
    assert!(saw_labs, "fixture must exercise a Labs row");
}

#[test]
fn fixture_support_export_excludes_raw_bytes_and_quotes_case_ids() {
    let page: VoicePreviewBetaPage = load_json("page.json");
    let export: VoicePreviewSupportExport = load_json("support_export.json");
    let expected =
        VoicePreviewSupportExport::from_page(export.support_export_id.clone(), page.clone());
    assert_eq!(export, expected);
    assert!(export.raw_audio_or_transcript_bytes_excluded);
    assert!(export.case_ids.contains(&page.page_id));
    for row in &page.rows {
        assert!(
            export.case_ids.contains(&row.row_id),
            "support export must quote row id {}",
            row.row_id
        );
    }
}

#[test]
fn published_markdown_matches_seeded_rendering() {
    let page = seeded_voice_preview_beta_page();
    let rendered = page.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("voice_preview_beta.md"))
        .expect("published voice_preview_beta.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published voice_preview_beta.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- report-md`",
    );
}

#[test]
fn fixture_compact_lines_match_seed() {
    let on_disk = std::fs::read_to_string(fixtures_root().join("compact.txt"))
        .expect("compact fixture must exist");
    let page = seeded_voice_preview_beta_page();
    let mut rendered = page.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- compact`",
    );
}

#[test]
fn published_doc_links_schemas_fixtures_artifact_and_gate() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/ux/m3/voice_preview_beta.md");
    let body =
        std::fs::read_to_string(&doc_path).expect("published voice_preview_beta doc must exist");
    for token in [
        "schemas/ux/voice_session_state.schema.json",
        "schemas/ux/voice_command_resolution.schema.json",
        "fixtures/ux/m3/voice_preview_and_privacy/page.json",
        "artifacts/ux/m3/voice_preview_beta.md",
        "docs/ux/voice_and_dictation_contract.md",
        "tools/ci/m3/voice_preview_check.py",
    ] {
        assert!(body.contains(token), "doc must reference {token}");
    }
}
