//! Conformance test for the voice/dictation conformance corpus.
//!
//! Loads `fixtures/ux/m3/voice_conformance_corpus/manifest.json` and replays
//! every drill through the canonical voice validation owned by
//! `aureline-shell::voice`. The suite fails when:
//!
//! - the on-disk corpus diverges from the seeded mint-from-truth corpus,
//! - any positive drill does not validate cleanly, leaks raw transcript/audio,
//!   or misses a pinned expectation,
//! - any negative drill is silently accepted or fails for the wrong reason,
//! - the corpus stops covering a required case (command/dictation mode, local
//!   engine, hosted provider, policy-blocked, offline/provider-unavailable,
//!   no-mic, noisy environment, transcript correction, high-risk confirmation),
//! - the negative set stops catching the spec's mandatory violations,
//! - voice/keyboard command parity drifts, the support export leaks raw bytes,
//! - the qualification packet stops downgrading stale/incomplete rows, or
//! - the published artifacts / audit doc drift from the seeded rendering.

use std::path::{Path, PathBuf};

use aureline_shell::voice::conformance::{
    compute_voice_qualification, fresh_complete_proof, render_command_equivalence_audit,
    render_privacy_and_parity_report, run_corpus_from_repo_root, scan_for_raw_voice_leak,
    seeded_voice_conformance_corpus, seeded_voice_qualification_packet, VoiceNegativeDetection,
    VoiceQualificationVerdict, VOICE_COMMAND_EQUIVALENCE_AUDIT_REF,
    VOICE_PREVIEW_BETA_AUDIT_DOC_REF, VOICE_PRIVACY_PARITY_REPORT_REF,
};
use aureline_shell::voice::{
    seeded_voice_preview_beta_page, NoBypassGuards, VoicePreviewSupportExport,
    VOICE_PREVIEW_SUPPORT_EXPORT_ID,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus_dir() -> PathBuf {
    repo_root().join("fixtures/ux/m3/voice_conformance_corpus")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn fixture_json<T: serde::Serialize>(value: &T) -> String {
    let mut json = serde_json::to_string_pretty(value).expect("serialize");
    json.push('\n');
    json
}

#[test]
fn corpus_is_bit_for_bit_equal_to_seed() {
    let corpus = seeded_voice_conformance_corpus();
    let dir = corpus_dir();

    let on_disk_manifest = read(&dir.join("manifest.json"));
    assert_eq!(
        on_disk_manifest,
        fixture_json(&corpus.manifest),
        "manifest.json diverged from the seed -- regenerate with `cargo run -q -p aureline-shell \
         --bin aureline_shell_voice_conformance -- write-corpus`",
    );

    for (fixture, row) in corpus
        .positives
        .iter()
        .map(|(spec, row)| (&spec.fixture, row))
        .chain(
            corpus
                .negatives
                .iter()
                .map(|(spec, row)| (&spec.fixture, row)),
        )
    {
        let on_disk = read(&dir.join(fixture));
        assert_eq!(
            on_disk,
            fixture_json(row),
            "fixture {fixture} diverged from the seed -- regenerate with `write-corpus`",
        );
    }
}

#[test]
fn corpus_runs_clean() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "voice corpus must publish at least one drill"
    );
    if !report.all_passed() {
        let failures: Vec<String> = report
            .failures()
            .iter()
            .map(|drill| {
                format!(
                    "{} ({}) at {}: {:?}",
                    drill.drill_id,
                    if drill.positive {
                        "positive"
                    } else {
                        "negative"
                    },
                    drill.fixture_path.display(),
                    drill.outcome
                )
            })
            .collect();
        panic!("voice corpus had failures: {}", failures.join("; "));
    }
    assert!(
        report.positive_count() >= 8,
        "expected the positive coverage set"
    );
    assert!(
        report.negative_count() >= 10,
        "expected the negative coverage set"
    );
}

#[test]
fn corpus_covers_every_required_case() {
    let corpus = seeded_voice_conformance_corpus();
    let covers: Vec<String> = corpus
        .manifest
        .positive_drills
        .iter()
        .flat_map(|drill| drill.covers.clone())
        .collect();
    for required in [
        "command_mode",
        "dictation_mode",
        "local_speech_engine",
        "hosted_provider",
        "policy_blocked_capture",
        "offline",
        "provider_unavailable",
        "no_microphone",
        "noisy_environment",
        "transcript_correction",
        "high_risk_command_confirmation",
    ] {
        assert!(
            covers.iter().any(|cover| cover == required),
            "voice corpus is missing positive coverage for `{required}`. Observed: {covers:?}"
        );
    }

    // Both command mode and dictation mode are exercised by an explicit mic-pill
    // voice mode, and both a local engine and a hosted provider are present.
    let modes: Vec<String> = corpus
        .manifest
        .positive_drills
        .iter()
        .filter_map(|drill| drill.expected_voice_mode.clone())
        .collect();
    for required in ["command_mode_active", "dictation_mode_active"] {
        assert!(
            modes.iter().any(|mode| mode == required),
            "voice corpus is missing a positive drill in voice mode `{required}`"
        );
    }
    let localities: Vec<String> = corpus
        .manifest
        .positive_drills
        .iter()
        .filter_map(|drill| drill.expected_processing_locality.clone())
        .collect();
    for required in ["local_on_device", "hosted_remote_disclosed"] {
        assert!(
            localities.iter().any(|locality| locality == required),
            "voice corpus is missing a positive drill with processing locality `{required}`"
        );
    }

    // Coverage includes a claimed and a Labs row.
    let postures: Vec<String> = corpus
        .manifest
        .positive_drills
        .iter()
        .filter_map(|drill| drill.expected_claim_posture.clone())
        .collect();
    assert!(postures
        .iter()
        .any(|p| p == "claimed_beta" || p == "claimed_preview"));
    assert!(postures.iter().any(|p| p == "labs_unadvertised"));
}

#[test]
fn negative_drills_catch_the_required_violations() {
    let corpus = seeded_voice_conformance_corpus();

    // The spec's mandatory rejections, caught by the canonical validator.
    let validator_classes: Vec<String> = corpus
        .manifest
        .negative_drills
        .iter()
        .filter(|drill| drill.detection == VoiceNegativeDetection::Validator)
        .filter_map(|drill| drill.expected_violation_class.clone())
        .collect();
    for required in [
        "implicit_always_on_capture",
        "background_listening_without_opt_in",
        "provider_privacy_state_hidden",
        "high_impact_preview_bypassed",
        "unavailable_without_keyboard_fallback",
        "labs_row_advertises_broad_support",
    ] {
        assert!(
            validator_classes.iter().any(|class| class == required),
            "voice corpus is missing a negative drill asserting `{required}`. Observed: \
             {validator_classes:?}"
        );
    }

    // Transcript leakage is caught by the redaction scan, not the validator.
    assert!(
        corpus
            .manifest
            .negative_drills
            .iter()
            .any(|drill| drill.detection == VoiceNegativeDetection::RedactionScan),
        "voice corpus must keep a redaction-scan negative drill for transcript leakage"
    );
}

#[test]
fn positive_fixtures_never_leak_raw_bytes() {
    let corpus = seeded_voice_conformance_corpus();
    let dir = corpus_dir();
    for (spec, _) in &corpus.positives {
        let payload = read(&dir.join(&spec.fixture));
        assert!(
            scan_for_raw_voice_leak(&payload).is_none(),
            "positive fixture {} leaked raw bytes",
            spec.fixture
        );
    }
}

#[test]
fn voice_keyboard_command_parity_holds() {
    let page = seeded_voice_preview_beta_page();
    for row in &page.rows {
        if !row.claim_posture.is_claimed() {
            continue;
        }
        for resolution in &row.command_resolutions {
            if let Some(command_id) = &resolution.canonical_command_id {
                assert_eq!(
                    resolution.keyboard_equivalent_command_id.as_ref(),
                    Some(command_id),
                    "{}: voice and keyboard must reach the same command id",
                    resolution.resolution_id
                );
                assert_eq!(
                    resolution.result_packet_schema_ref,
                    "schemas/commands/command_result_packet.schema.json",
                    "{}: result-packet schema must match the command graph",
                    resolution.resolution_id
                );
                assert!(
                    !resolution.docs_help_anchor_ref.trim().is_empty(),
                    "{}: must carry a docs/help anchor for CLI/help parity",
                    resolution.resolution_id
                );
            }
            if resolution.is_high_impact() {
                assert!(resolution.preview_required, "{}", resolution.resolution_id);
                assert_eq!(resolution.no_bypass_guards, NoBypassGuards::strict());
                assert!(resolution.canonical_command_id.is_some());
            }
        }
    }
}

#[test]
fn support_export_explains_state_without_raw_bytes() {
    let page = seeded_voice_preview_beta_page();
    let export =
        VoicePreviewSupportExport::from_page(VOICE_PREVIEW_SUPPORT_EXPORT_ID, page.clone());
    assert!(export.raw_audio_or_transcript_bytes_excluded);
    assert!(export.case_ids.contains(&page.page_id));
    for row in &page.rows {
        assert!(
            export.case_ids.contains(&row.row_id),
            "support export must quote row id {}",
            row.row_id
        );
    }
    let json = serde_json::to_string(&export).expect("serialize support export");
    assert!(
        scan_for_raw_voice_leak(&json).is_none(),
        "support export must not carry raw transcript/audio/url bytes"
    );
}

#[test]
fn qualification_downgrades_stale_or_incomplete_rows() {
    let page = seeded_voice_preview_beta_page();

    // Green path: a clean corpus and fresh/complete proof keeps every claimed
    // row Preview/Beta.
    let green = seeded_voice_qualification_packet(&page);
    assert!(green.all_claimed_rows_qualified);
    assert_eq!(green.claimed_rows_downgraded, 0);
    assert_eq!(green.claimed_rows_kept, page.summary.claimed_row_count);

    // Stale or incomplete proof on a claimed row forces it back to Labs.
    let mut proof = fresh_complete_proof(&page);
    assert!(
        proof.len() >= 2,
        "need at least two claimed rows to exercise"
    );
    proof[0].proof_fresh = false;
    proof[1].proof_complete = false;
    let stale =
        compute_voice_qualification(&page, true, &proof, "pkt-stale", "2026-05-20T00:00:00Z");
    assert!(!stale.all_claimed_rows_qualified);
    assert!(stale.claimed_rows_downgraded >= 2);
    assert!(stale.rows.iter().any(|row| {
        row.verdict == VoiceQualificationVerdict::DowngradeToLabs
            && row.downgrade_reasons.iter().any(|r| r == "proof_stale")
    }));
    assert!(stale.rows.iter().any(|row| {
        row.verdict == VoiceQualificationVerdict::DowngradeToLabs
            && row
                .downgrade_reasons
                .iter()
                .any(|r| r == "proof_incomplete")
    }));

    // An unclean corpus downgrades every claimed row.
    let unclean =
        compute_voice_qualification(&page, false, &fresh_complete_proof(&page), "pkt-x", "t");
    assert_eq!(unclean.claimed_rows_kept, 0);
}

#[test]
fn published_artifacts_match_seeded_rendering() {
    let page = seeded_voice_preview_beta_page();
    let packet = seeded_voice_qualification_packet(&page);

    let privacy = read(&repo_root().join(VOICE_PRIVACY_PARITY_REPORT_REF));
    assert_eq!(
        privacy,
        render_privacy_and_parity_report(&page, &packet),
        "{VOICE_PRIVACY_PARITY_REPORT_REF} diverged -- regenerate with `aureline_shell_voice_conformance -- privacy-report`",
    );

    let equivalence = read(&repo_root().join(VOICE_COMMAND_EQUIVALENCE_AUDIT_REF));
    assert_eq!(
        equivalence,
        render_command_equivalence_audit(&page),
        "{VOICE_COMMAND_EQUIVALENCE_AUDIT_REF} diverged -- regenerate with `aureline_shell_voice_conformance -- equivalence-audit`",
    );
}

#[test]
fn audit_doc_links_corpus_artifacts_and_gate() {
    let body = read(&repo_root().join(VOICE_PREVIEW_BETA_AUDIT_DOC_REF));
    for token in [
        "fixtures/ux/m3/voice_conformance_corpus",
        VOICE_PRIVACY_PARITY_REPORT_REF,
        VOICE_COMMAND_EQUIVALENCE_AUDIT_REF,
        "schemas/ux/voice_session_state.schema.json",
        "schemas/ux/voice_command_resolution.schema.json",
        "docs/ux/voice_and_dictation_contract.md",
        "voice_conformance_corpus",
        "aureline_shell_voice_conformance",
    ] {
        assert!(body.contains(token), "audit doc must reference {token}");
    }
}
