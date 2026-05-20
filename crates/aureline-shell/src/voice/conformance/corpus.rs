//! Manifest types and the mint-from-truth seed for the voice/dictation
//! conformance corpus.
//!
//! The corpus is the regression-gated proof lane for the bounded voice
//! preview/beta surface. Every drill is a single canonical
//! [`VoicePreviewRow`] (owned by [`crate::voice`]); the runner replays it
//! through the canonical validation [`crate::voice::build_voice_preview_row`]
//! and never re-implements the ruleset.
//!
//! Positive drills MUST validate cleanly (zero blocking findings), carry no
//! raw transcript/audio leak, and match every pinned `expected_*` token.
//! Negative drills MUST be rejected: a [`VoiceNegativeDetection::Validator`]
//! drill MUST produce a blocking finding whose class token contains the
//! recorded `expected_violation_class`, and a
//! [`VoiceNegativeDetection::RedactionScan`] drill MUST trip the
//! raw-transcript/audio leak scan.
//!
//! The seed is built by cloning the canonical product rows from
//! [`crate::voice::seeded_voice_preview_beta_page`] and (for the negatives and
//! the extra unavailable-reason coverage) mutating exactly one field, so the
//! corpus stays tied to the real claimed surface rather than a parallel model.

use serde::{Deserialize, Serialize};

use crate::voice::{
    seeded_voice_preview_beta_page, BackgroundListeningState, EnablementDecisionClass,
    MicIndicatorClass, VoiceActivationClass, VoiceCommandResolutionClass, VoicePreviewRow,
    VoiceUnavailableReason,
};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/ux/m3/voice_conformance_corpus";

/// Stable corpus identifier quoted by audit/support records.
pub const CORPUS_ID: &str = "ux.voice_conformance_corpus.beta";

/// Manifest schema version.
pub const CORPUS_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing description embedded in the manifest.
pub const CORPUS_DESCRIPTION: &str = "Conformance drill corpus for the bounded voice/dictation \
beta lane. Every drill is one canonical VoicePreviewRow owned by aureline-shell::voice; the runner \
replays it through the canonical validation (build_voice_preview_row) and never re-implements the \
ruleset. Positive drills MUST validate cleanly (zero blocking findings), carry no raw \
transcript/audio leak, and match every pinned expected_* token across command mode, dictation \
mode, the local speech engine, the hosted provider, policy-blocked capture, offline/provider \
unavailable, no-microphone, noisy-environment, transcript correction, and high-risk command \
confirmation. Negative drills MUST be rejected: a validator drill MUST raise a blocking finding \
whose class contains expected_violation_class, and a redaction_scan drill MUST trip the \
raw-transcript/audio leak scan. The corpus fails hidden always-listening behavior, missing \
provider/privacy disclosure, transcript leakage, skipped confirmation on risky commands, and \
broken keyboard fallback rather than tolerating them.";

/// Record family a drill fixture deserializes into. The voice corpus is
/// row-centric today; the enum is kept for forward-compatibility and parity
/// with the other conformance lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceDrillRecordType {
    /// A single [`VoicePreviewRow`].
    VoicePreviewRow,
}

impl VoiceDrillRecordType {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VoicePreviewRow => "voice_preview_row",
        }
    }
}

/// How a negative drill is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceNegativeDetection {
    /// The canonical validator MUST raise a blocking finding.
    Validator,
    /// The raw-transcript/audio leak scan MUST trip on the payload.
    RedactionScan,
}

/// Root manifest document for the voice conformance corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceCorpusManifest {
    /// Stable corpus identifier.
    pub corpus_id: String,
    /// Manifest schema version.
    pub schema_version: u32,
    /// Reviewer-facing description.
    pub description: String,
    /// Positive drill specs.
    pub positive_drills: Vec<VoicePositiveDrill>,
    /// Negative drill specs.
    pub negative_drills: Vec<VoiceNegativeDrill>,
}

/// Single positive drill spec. The fixture MUST parse, validate cleanly, carry
/// no leak, and satisfy every pinned expectation. Unspecified expectations
/// (`None`) are not asserted, so a focused drill only pins the truth it stands
/// for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoicePositiveDrill {
    /// Stable drill id used by audit/support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Record family the fixture deserializes into.
    pub record_type: VoiceDrillRecordType,
    /// Reviewer-facing class for the coverage matrix.
    pub drill_class: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,

    /// Expected `VoiceClaimPosture` token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_claim_posture: Option<String>,
    /// Expected active `VoiceModeClass` token (from the mic pill).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_voice_mode: Option<String>,
    /// Expected `VoiceActivationClass` default-activation token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_default_activation: Option<String>,
    /// Expected `ProcessingLocalityCue` token (from the privacy row).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_processing_locality: Option<String>,
    /// Expected `RetentionMode` token (from the privacy row).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_retention_mode: Option<String>,
    /// Expected `BackgroundListeningState` token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_background_listening: Option<String>,
    /// Expected `VoiceUnavailableReason` token, when the row is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_unavailable_reason: Option<String>,
    /// When `true`, the row MUST advertise a keyboard fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_keyboard_fallback: Option<bool>,
    /// Canonical command id a high-impact resolution on the row MUST bind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_high_impact_command_id: Option<String>,
}

/// Single negative drill spec. The fixture MUST be rejected by the recorded
/// detection mechanism.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceNegativeDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Record family the fixture deserializes into.
    pub record_type: VoiceDrillRecordType,
    /// How the fixture is rejected.
    pub detection: VoiceNegativeDetection,
    /// Substring that must appear in a blocking finding's class token
    /// (required for [`VoiceNegativeDetection::Validator`] drills).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_violation_class: Option<String>,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}

/// The seeded, in-memory conformance corpus: the single mint-from-truth source
/// for the checked-in fixtures and manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceConformanceCorpus {
    /// The manifest as it should be written to disk.
    pub manifest: VoiceCorpusManifest,
    /// Positive drills paired with their fixture rows, in manifest order.
    pub positives: Vec<(VoicePositiveDrill, VoicePreviewRow)>,
    /// Negative drills paired with their fixture rows, in manifest order.
    pub negatives: Vec<(VoiceNegativeDrill, VoicePreviewRow)>,
}

/// Fetches a canonical product row from the seeded voice preview page.
fn seeded_row(row_id: &str) -> VoicePreviewRow {
    seeded_voice_preview_beta_page()
        .rows
        .into_iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("seeded voice preview page is missing row `{row_id}`"))
}

/// Re-ids a cloned row (and its nested ids stay as authored — only `row_id`
/// has to be unique across the corpus for the fixture file name to be honest).
fn with_row_id(mut row: VoicePreviewRow, row_id: &str) -> VoicePreviewRow {
    row.row_id = row_id.to_owned();
    // The input fixture represents what a surface claims; the harness derives
    // the blocking findings, so the stored set is always empty.
    row.blocking_findings = Vec::new();
    row
}

/// Builds the deterministic seeded conformance corpus.
pub fn seeded_voice_conformance_corpus() -> VoiceConformanceCorpus {
    let mut positives: Vec<(VoicePositiveDrill, VoicePreviewRow)> = Vec::new();
    let mut negatives: Vec<(VoiceNegativeDrill, VoicePreviewRow)> = Vec::new();

    // ---- Positive drills: the canonical claimed product rows. -------------
    positives.push((
        VoicePositiveDrill {
            drill_id: "positive.command_mode_local".to_owned(),
            fixture: "positive/command_mode_local.json".to_owned(),
            record_type: VoiceDrillRecordType::VoicePreviewRow,
            drill_class: "command_mode_local_engine".to_owned(),
            covers: vec![
                "command_mode".to_owned(),
                "local_speech_engine".to_owned(),
                "push_to_talk".to_owned(),
                "high_risk_command_confirmation".to_owned(),
                "transcript_correction".to_owned(),
            ],
            expected_claim_posture: Some("claimed_beta".to_owned()),
            expected_voice_mode: Some("command_mode_active".to_owned()),
            expected_default_activation: Some("push_to_talk_held".to_owned()),
            expected_processing_locality: Some("local_on_device".to_owned()),
            expected_retention_mode: Some("no_audio_retained_no_transcript_retained".to_owned()),
            expected_background_listening: Some("off_default".to_owned()),
            expected_unavailable_reason: None,
            expected_keyboard_fallback: Some(true),
            expected_high_impact_command_id: Some(
                "cmd:edit.rename_symbol_across_project".to_owned(),
            ),
        },
        with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:command_mode_local_beta",
        ),
    ));

    positives.push((
        VoicePositiveDrill {
            drill_id: "positive.dictation_local".to_owned(),
            fixture: "positive/dictation_local.json".to_owned(),
            record_type: VoiceDrillRecordType::VoicePreviewRow,
            drill_class: "dictation_mode_local_engine".to_owned(),
            covers: vec![
                "dictation_mode".to_owned(),
                "local_speech_engine".to_owned(),
                "transcript_correction".to_owned(),
                "shared_undo_stack".to_owned(),
            ],
            expected_claim_posture: Some("claimed_beta".to_owned()),
            expected_voice_mode: Some("dictation_mode_active".to_owned()),
            expected_default_activation: Some("push_to_talk_held".to_owned()),
            expected_processing_locality: Some("local_on_device".to_owned()),
            expected_retention_mode: Some(
                "ephemeral_audio_local_only_no_transcript_retained".to_owned(),
            ),
            expected_background_listening: Some("off_default".to_owned()),
            expected_unavailable_reason: None,
            expected_keyboard_fallback: Some(true),
            expected_high_impact_command_id: None,
        },
        with_row_id(
            seeded_row("voice:row:dictation_local_beta"),
            "voice:row:dictation_local_beta",
        ),
    ));

    positives.push((
        VoicePositiveDrill {
            drill_id: "positive.hosted_command_preview".to_owned(),
            fixture: "positive/hosted_command_preview.json".to_owned(),
            record_type: VoiceDrillRecordType::VoicePreviewRow,
            drill_class: "command_mode_hosted_provider".to_owned(),
            covers: vec![
                "command_mode".to_owned(),
                "hosted_provider".to_owned(),
                "high_risk_command_confirmation".to_owned(),
                "approval_required".to_owned(),
            ],
            expected_claim_posture: Some("claimed_preview".to_owned()),
            expected_voice_mode: Some("command_mode_active".to_owned()),
            expected_default_activation: Some("manual_command_activation".to_owned()),
            expected_processing_locality: Some("hosted_remote_disclosed".to_owned()),
            expected_retention_mode: Some("transcript_retained_provider_per_contract".to_owned()),
            expected_background_listening: Some("off_default".to_owned()),
            expected_unavailable_reason: None,
            expected_keyboard_fallback: Some(true),
            expected_high_impact_command_id: Some("cmd:git.push_current_branch".to_owned()),
        },
        with_row_id(
            seeded_row("voice:row:hosted_command_preview"),
            "voice:row:hosted_command_preview",
        ),
    ));

    positives.push((
        VoicePositiveDrill {
            drill_id: "positive.no_microphone_fallback".to_owned(),
            fixture: "positive/no_microphone_fallback.json".to_owned(),
            record_type: VoiceDrillRecordType::VoicePreviewRow,
            drill_class: "no_microphone_unavailable".to_owned(),
            covers: vec![
                "no_microphone".to_owned(),
                "keyboard_fallback".to_owned(),
                "unavailable_banner".to_owned(),
                "disabled_reason_parity".to_owned(),
            ],
            expected_claim_posture: Some("claimed_beta".to_owned()),
            expected_voice_mode: Some("idle_microphone_off".to_owned()),
            expected_default_activation: Some("push_to_talk_held".to_owned()),
            expected_processing_locality: Some("processing_unavailable".to_owned()),
            expected_retention_mode: Some("no_audio_retained_no_transcript_retained".to_owned()),
            expected_background_listening: Some("off_default".to_owned()),
            expected_unavailable_reason: Some("no_microphone".to_owned()),
            expected_keyboard_fallback: Some(true),
            expected_high_impact_command_id: None,
        },
        with_row_id(
            seeded_row("voice:row:provider_unavailable_fallback"),
            "voice:row:provider_unavailable_fallback",
        ),
    ));

    positives.push((
        VoicePositiveDrill {
            drill_id: "positive.labs_policy_blocked".to_owned(),
            fixture: "positive/labs_policy_blocked.json".to_owned(),
            record_type: VoiceDrillRecordType::VoicePreviewRow,
            drill_class: "labs_unadvertised_policy_blocked".to_owned(),
            covers: vec![
                "labs_unadvertised".to_owned(),
                "policy_blocked_capture".to_owned(),
                "suppressed_no_resolutions".to_owned(),
                "keyboard_fallback".to_owned(),
            ],
            expected_claim_posture: Some("labs_unadvertised".to_owned()),
            expected_voice_mode: None,
            expected_default_activation: Some("activation_unavailable_in_envelope".to_owned()),
            expected_processing_locality: Some("processing_unavailable".to_owned()),
            expected_retention_mode: Some("retention_unavailable_in_envelope".to_owned()),
            expected_background_listening: Some("off_default".to_owned()),
            expected_unavailable_reason: Some("policy_locked_or_blocked".to_owned()),
            expected_keyboard_fallback: Some(true),
            expected_high_impact_command_id: None,
        },
        with_row_id(
            seeded_row("voice:row:labs_unadvertised_continuous"),
            "voice:row:labs_unadvertised_continuous",
        ),
    ));

    // Extra unavailable-reason coverage: clone the clean no-microphone row and
    // change only the typed reason so offline / provider-unavailable /
    // noisy-environment all keep a keyboard fallback.
    for (drill_id, fixture, drill_class, reason, reason_token, banner_msg, cover) in [
        (
            "positive.offline_no_local_engine",
            "positive/offline_no_local_engine.json",
            "offline_no_local_engine_unavailable",
            VoiceUnavailableReason::OfflineNoLocalEngine,
            "offline_no_local_engine",
            "label:voice:banner:offline_use_keyboard",
            "offline",
        ),
        (
            "positive.provider_unavailable",
            "positive/provider_unavailable.json",
            "provider_unavailable_unavailable",
            VoiceUnavailableReason::ProviderUnavailable,
            "provider_unavailable",
            "label:voice:banner:provider_unavailable_use_keyboard",
            "provider_unavailable",
        ),
        (
            "positive.noisy_environment",
            "positive/noisy_environment.json",
            "noisy_environment_unavailable",
            VoiceUnavailableReason::NoisyEnvironment,
            "noisy_environment",
            "label:voice:banner:noisy_environment_use_keyboard",
            "noisy_environment",
        ),
    ] {
        let row_id = format!("voice:row:{cover}_fallback");
        let mut row = with_row_id(
            seeded_row("voice:row:provider_unavailable_fallback"),
            &row_id,
        );
        row.provider_privacy_row.row_id = format!("voice:privacy:{cover}_fallback");
        row.provider_privacy_row.unavailable_reason = Some(reason);
        if let Some(banner) = row.unavailable_banner.as_mut() {
            banner.banner_id = format!("voice:banner:{cover}_fallback");
            banner.unavailable_reason = reason;
            banner.message_ref = banner_msg.to_owned();
        }
        if let Some(pill) = row.mic_pill.as_mut() {
            pill.pill_id = format!("voice:pill:{cover}_fallback");
        }
        // The single resolution stays blocked-by-envelope with the shared
        // disabled-reason vocabulary; retarget its id so it stays unique.
        for resolution in row.command_resolutions.iter_mut() {
            resolution.resolution_id = format!("voice:resolution:{cover}_blocked");
        }
        positives.push((
            VoicePositiveDrill {
                drill_id: drill_id.to_owned(),
                fixture: fixture.to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                drill_class: drill_class.to_owned(),
                covers: vec![
                    cover.to_owned(),
                    "keyboard_fallback".to_owned(),
                    "unavailable_banner".to_owned(),
                ],
                expected_claim_posture: Some("claimed_beta".to_owned()),
                expected_voice_mode: Some("idle_microphone_off".to_owned()),
                expected_default_activation: Some("push_to_talk_held".to_owned()),
                expected_processing_locality: Some("processing_unavailable".to_owned()),
                expected_retention_mode: Some(
                    "no_audio_retained_no_transcript_retained".to_owned(),
                ),
                expected_background_listening: Some("off_default".to_owned()),
                expected_unavailable_reason: Some(reason_token.to_owned()),
                expected_keyboard_fallback: Some(true),
                expected_high_impact_command_id: None,
            },
            row,
        ));
    }

    // ---- Negative drills: each clones a clean row and breaks one field. ----

    // 1. Hidden always-listening: a claimed row whose default activation is not
    //    explicit (implicit always-on capture).
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_implicit_always_on",
        );
        row.default_activation_class = VoiceActivationClass::WakePhraseContinuousUserOptedIn;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.implicit_always_on".to_owned(),
                fixture: "negative/implicit_always_on.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("implicit_always_on_capture".to_owned()),
                covers: vec![
                    "always_listening".to_owned(),
                    "explicit_activation".to_owned(),
                ],
            },
            row,
        ));
    }

    // 2. Hidden always-listening: background listening on without a wake-phrase
    //    opt-in.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_background_without_opt_in",
        );
        row.background_listening_state = BackgroundListeningState::OnUserOptedIn;
        row.provider_privacy_row.background_listening_state =
            BackgroundListeningState::OnUserOptedIn;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.background_without_opt_in".to_owned(),
                fixture: "negative/background_without_opt_in.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("background_listening_without_opt_in".to_owned()),
                covers: vec![
                    "always_listening".to_owned(),
                    "background_listening".to_owned(),
                ],
            },
            row,
        ));
    }

    // 3. Missing provider/privacy disclosure.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_provider_privacy_hidden",
        );
        row.provider_privacy_row.provider_or_local_engine_label_ref = String::new();
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.provider_privacy_hidden".to_owned(),
                fixture: "negative/provider_privacy_hidden.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("provider_privacy_state_hidden".to_owned()),
                covers: vec![
                    "provider_disclosure".to_owned(),
                    "privacy_disclosure".to_owned(),
                ],
            },
            row,
        ));
    }

    // 4. Skipped confirmation on a risky command: high-impact resolution drops
    //    its required preview.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_high_impact_preview_bypassed",
        );
        for resolution in row.command_resolutions.iter_mut() {
            if resolution.is_high_impact() {
                resolution.preview_required = false;
            }
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.high_impact_preview_bypassed".to_owned(),
                fixture: "negative/high_impact_preview_bypassed.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("high_impact_preview_bypassed".to_owned()),
                covers: vec![
                    "high_risk_confirmation".to_owned(),
                    "preview_required".to_owned(),
                ],
            },
            row,
        ));
    }

    // 5. Weakened no-bypass guard on a high-impact resolution.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:hosted_command_preview"),
            "voice:row:neg_no_bypass_guard_weakened",
        );
        for resolution in row.command_resolutions.iter_mut() {
            if resolution.is_high_impact() {
                resolution.no_bypass_guards.preview_path_preserved = false;
            }
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.no_bypass_guard_weakened".to_owned(),
                fixture: "negative/no_bypass_guard_weakened.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("no_bypass_guard_weakened".to_owned()),
                covers: vec![
                    "high_risk_confirmation".to_owned(),
                    "no_bypass_guards".to_owned(),
                ],
            },
            row,
        ));
    }

    // 6. Broken keyboard fallback on an unavailable row.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:provider_unavailable_fallback"),
            "voice:row:neg_unavailable_without_fallback",
        );
        row.provider_privacy_row.keyboard_fallback_available = false;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.unavailable_without_fallback".to_owned(),
                fixture: "negative/unavailable_without_fallback.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("unavailable_without_keyboard_fallback".to_owned()),
                covers: vec!["keyboard_fallback".to_owned(), "unavailable".to_owned()],
            },
            row,
        ));
    }

    // 7. Mic indicator hidden during active capture.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_mic_indicator_hidden",
        );
        if let Some(pill) = row.mic_pill.as_mut() {
            pill.mic_indicator_class = MicIndicatorClass::PersistentIndicatorHiddenCaptureDisabled;
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.mic_indicator_hidden".to_owned(),
                fixture: "negative/mic_indicator_hidden.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("mic_indicator_hidden_during_capture".to_owned()),
                covers: vec!["mic_indicator".to_owned(), "active_capture".to_owned()],
            },
            row,
        ));
    }

    // 8. Claimed row that is not keyboard reachable.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:dictation_local_beta"),
            "voice:row:neg_not_keyboard_reachable",
        );
        row.keyboard_reachable = false;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.not_keyboard_reachable".to_owned(),
                fixture: "negative/not_keyboard_reachable.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("not_keyboard_reachable".to_owned()),
                covers: vec!["keyboard_reach".to_owned(), "accessibility".to_owned()],
            },
            row,
        ));
    }

    // 9. Claimed row that is not screen-reader narratable.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:dictation_local_beta"),
            "voice:row:neg_not_screen_reader_narratable",
        );
        row.screen_reader_narratable = false;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.not_screen_reader_narratable".to_owned(),
                fixture: "negative/not_screen_reader_narratable.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("not_screen_reader_narratable".to_owned()),
                covers: vec!["screen_reader".to_owned(), "accessibility".to_owned()],
            },
            row,
        ));
    }

    // 10. Claimed row that hides the command/dictation mode split.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_command_mode_not_explicit",
        );
        row.command_mode_explicit = false;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.command_mode_not_explicit".to_owned(),
                fixture: "negative/command_mode_not_explicit.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("command_mode_not_explicit".to_owned()),
                covers: vec!["mode_separation".to_owned(), "command_mode".to_owned()],
            },
            row,
        ));
    }

    // 11. A mutating resolution that resolves to a canonical command but binds
    //     no command id (a private mutation path).
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_resolution_missing_command_id",
        );
        for resolution in row.command_resolutions.iter_mut() {
            if resolution.resolution_class
                == VoiceCommandResolutionClass::ResolvesToCanonicalCommandId
            {
                resolution.canonical_command_id = None;
            }
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.resolution_missing_command_id".to_owned(),
                fixture: "negative/resolution_missing_command_id.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("resolution_missing_command_id".to_owned()),
                covers: vec![
                    "command_parity".to_owned(),
                    "canonical_command_id".to_owned(),
                ],
            },
            row,
        ));
    }

    // 12. A resolution that resolves to a verb outside the registry.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_resolution_uncanonical_verb",
        );
        if let Some(resolution) = row.command_resolutions.first_mut() {
            resolution.resolution_class =
                VoiceCommandResolutionClass::ResolutionDeniedUncanonicalVerb;
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.resolution_uncanonical_verb".to_owned(),
                fixture: "negative/resolution_uncanonical_verb.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("resolution_uncanonical_verb".to_owned()),
                covers: vec!["command_parity".to_owned(), "uncanonical_verb".to_owned()],
            },
            row,
        ));
    }

    // 13. A disabled resolution that drops its typed disabled reason.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_disabled_resolution_missing_reason",
        );
        if let Some(resolution) = row.command_resolutions.first_mut() {
            resolution.enablement_decision_class = EnablementDecisionClass::DisabledWithReason;
            resolution.disabled_reason_code = None;
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.disabled_resolution_missing_reason".to_owned(),
                fixture: "negative/disabled_resolution_missing_reason.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("disabled_resolution_missing_reason".to_owned()),
                covers: vec!["disabled_reason_parity".to_owned()],
            },
            row,
        ));
    }

    // 14. A Labs/unadvertised row that starts advertising broad support.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:labs_unadvertised_continuous"),
            "voice:row:neg_labs_advertises",
        );
        row.background_listening_state = BackgroundListeningState::OnUserOptedIn;
        row.default_activation_class = VoiceActivationClass::WakePhraseContinuousUserOptedIn;
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.labs_advertises".to_owned(),
                fixture: "negative/labs_advertises.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::Validator,
                expected_violation_class: Some("labs_row_advertises_broad_support".to_owned()),
                covers: vec!["labs_suppression".to_owned(), "overclaim".to_owned()],
            },
            row,
        ));
    }

    // 15. Transcript leakage: a transcript label that carries a raw URL/secret.
    {
        let mut row = with_row_id(
            seeded_row("voice:row:command_mode_local_beta"),
            "voice:row:neg_transcript_leak",
        );
        if let Some(strip) = row.transcript_strip.as_mut() {
            strip.transcript_text_label_ref =
                "https://internal.example/raw-transcript-secret".to_owned();
        }
        negatives.push((
            VoiceNegativeDrill {
                drill_id: "negative.transcript_leak".to_owned(),
                fixture: "negative/transcript_leak.json".to_owned(),
                record_type: VoiceDrillRecordType::VoicePreviewRow,
                detection: VoiceNegativeDetection::RedactionScan,
                expected_violation_class: None,
                covers: vec!["transcript_leakage".to_owned(), "redaction".to_owned()],
            },
            row,
        ));
    }

    let manifest = VoiceCorpusManifest {
        corpus_id: CORPUS_ID.to_owned(),
        schema_version: CORPUS_SCHEMA_VERSION,
        description: CORPUS_DESCRIPTION.to_owned(),
        positive_drills: positives.iter().map(|(spec, _)| spec.clone()).collect(),
        negative_drills: negatives.iter().map(|(spec, _)| spec.clone()).collect(),
    };

    VoiceConformanceCorpus {
        manifest,
        positives,
        negatives,
    }
}
