//! Save-review sheet projection.
//!
//! The save-review sheet is a protected surface shown when a save attempt is
//! refused because the pinned save target has drifted or cannot be proven safe
//! to overwrite. The sheet surfaces a diff preview between the local buffer
//! snapshot and the current on-disk bytes and keeps destructive choices
//! explicit.

use aureline_content_safety::{
    project_content_integrity_warnings, ContentIntegritySurfaceKind, ContentIntegrityWarningRecord,
};
use aureline_vfs::{SaveOutcome, SaveTargetToken, VfsRoot};
use aureline_workspace::save::SourceFidelityRecord;
use serde::{Deserialize, Serialize};

use crate::path_truth::{
    alias_inspector_lines, materialize_alias_inspector_record, materialize_path_truth_chip_record,
    materialize_save_target_review_record, path_truth_chip_lines, save_target_review_lines,
};

/// Machine-readable record for a save-review sheet instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveReviewSheetRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub generated_at: String,

    pub packet_id: String,
    pub outcome: String,

    pub presentation_uri: String,
    pub canonical_uri: String,

    pub pinned_generation_token: String,
    pub observed_generation_token: String,

    pub diff: SaveReviewDiffRecord,
    pub offered_choices: Vec<SaveReviewChoiceOffer>,

    pub selected_choice: Option<String>,
    pub selected_at: Option<String>,
}

/// Diff metadata carried by a [`SaveReviewSheetRecord`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveReviewDiffRecord {
    pub diff_availability: String,
    pub content_kind: String,
    pub summary: Option<SaveReviewDiffSummary>,
    pub preview_lines: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_integrity_warnings: Vec<ContentIntegrityWarningRecord>,
}

/// Summary of the diff between the local buffer and the external bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveReviewDiffSummary {
    pub changed_hunk_count: u32,
    pub external_line_change_count: u32,
    pub local_line_change_count: u32,
    pub metadata_only: bool,
    pub summary_text: String,
}

/// One offered choice row rendered by the save-review sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveReviewChoiceOffer {
    pub choice: String,
    pub offered: bool,
    pub enabled: bool,
    pub forbidden_reason: String,
    pub requires_diff_metadata: bool,
    pub requires_checkpoint: bool,
    pub journal_implication: String,
    pub resulting_authoritative_state_if_selected: String,
}

/// Canonical choice keys used by the save-review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveReviewChoiceKey {
    Compare,
    Overwrite,
    Merge,
    Reload,
    Retry,
    SaveAs,
    Cancel,
}

impl SaveReviewChoiceKey {
    /// Returns the stable string vocabulary for this choice key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compare => "compare",
            Self::Overwrite => "overwrite",
            Self::Merge => "merge",
            Self::Reload => "reload",
            Self::Retry => "retry",
            Self::SaveAs => "save_as",
            Self::Cancel => "cancel",
        }
    }
}

fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

/// Writes a save-review sheet record into `.logs/review_sheets/`.
pub fn write_save_review_sheet_log(record: &SaveReviewSheetRecord) {
    let root = std::path::PathBuf::from(".logs").join("review_sheets");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let filename = format!(
        "{}.{}.save_review_sheet.json",
        sanitize_filename(&record.packet_id),
        sanitize_filename(&record.generated_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

/// Builds the human-readable lines used by the shell to render a save-review sheet.
pub fn save_review_sheet_lines(record: &SaveReviewSheetRecord, selection: usize) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Save review — {}", record.outcome));
    lines.push("Esc: cancel   ↑/↓: select   Enter: apply".to_string());
    lines.push("".to_string());

    lines.push(format!("target_presentation: {}", record.presentation_uri));
    lines.push(format!("target_canonical: {}", record.canonical_uri));
    lines.push(format!(
        "generation: pinned={} observed={}",
        record.pinned_generation_token, record.observed_generation_token
    ));

    if let Some(summary) = &record.diff.summary {
        lines.push("".to_string());
        lines.push(format!(
            "diff: {} (hunks={}, external_lines={}, local_lines={})",
            record.diff.diff_availability,
            summary.changed_hunk_count,
            summary.external_line_change_count,
            summary.local_line_change_count
        ));
        lines.push(summary.summary_text.clone());
    } else {
        lines.push("".to_string());
        lines.push(format!("diff: {}", record.diff.diff_availability));
    }

    if !record.diff.preview_lines.is_empty() {
        lines.push("".to_string());
        lines.push("Preview:".to_string());
        lines.extend(record.diff.preview_lines.iter().cloned());
    }

    if !record.diff.content_integrity_warnings.is_empty() {
        lines.push("".to_string());
        lines.push("Content integrity:".to_string());
        for warning in &record.diff.content_integrity_warnings {
            lines.push(format!(
                "{} — {} at char {}",
                warning.record_kind, warning.warning_label, warning.char_offset
            ));
        }
    }

    lines.push("".to_string());
    lines.push("Choices:".to_string());
    for (idx, choice) in record.offered_choices.iter().enumerate() {
        let marker = if idx == selection { ">" } else { " " };
        let enabled = if choice.enabled {
            "enabled"
        } else {
            "disabled"
        };
        let suffix = if choice.enabled {
            String::new()
        } else {
            format!(" ({})", choice.forbidden_reason)
        };
        let mut hint_flags = Vec::new();
        if choice.requires_diff_metadata {
            hint_flags.push("diff");
        }
        if choice.requires_checkpoint {
            hint_flags.push("checkpoint");
        }
        let hint = if hint_flags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", hint_flags.join(", "))
        };
        lines.push(format!(
            "{marker} {choice} — {enabled}{suffix}{hint} journal={journal} result={result}",
            marker = marker,
            choice = choice.choice,
            enabled = enabled,
            suffix = suffix,
            hint = hint,
            journal = choice.journal_implication.as_str(),
            result = choice.resulting_authoritative_state_if_selected.as_str()
        ));
    }

    lines
}

/// Render the save-review sheet body with the path-truth chip,
/// alias inspector body, and pre-write save-target review section
/// appended for the protected walk
/// `open difficult fixture -> inspect chip -> review save target`.
///
/// Surfaces that already render via [`save_review_sheet_lines`]
/// keep working unchanged; this entry point is the wire to the
/// path-truth surface so the live shell never falls back to silent
/// dedupe-by-path-string.
pub fn save_review_sheet_lines_with_path_truth(
    record: &SaveReviewSheetRecord,
    token: &SaveTargetToken,
    selection: usize,
) -> Vec<String> {
    let mut lines = save_review_sheet_lines(record, selection);

    let chip = materialize_path_truth_chip_record(&token.identity);
    let inspection = materialize_alias_inspector_record(&token.identity);
    let review = materialize_save_target_review_record(token);

    lines.push("".to_string());
    lines.push("Path truth:".to_string());
    lines.extend(path_truth_chip_lines(&chip));

    if !inspection.entries.is_empty() || inspection.presentation_alias_missing {
        lines.push("".to_string());
        lines.extend(alias_inspector_lines(&inspection));
    }

    if review.save_redirects_target || !review.blockers.is_empty() {
        lines.push("".to_string());
        lines.extend(save_target_review_lines(&review));
    }

    lines
}

/// Materializes a save-review sheet record for a refused save attempt.
pub fn materialize_save_review_sheet_record(
    root: &dyn VfsRoot,
    token: &SaveTargetToken,
    source_fidelity: &SourceFidelityRecord,
    packet_id: String,
    outcome: SaveOutcome,
    generated_at: String,
    local_content: &[u8],
    reviewed_external_state: bool,
) -> SaveReviewSheetRecord {
    let canonical_uri = &token.identity.canonical_filesystem_object.canonical_uri;
    let presentation_uri = &token.identity.presentation_path.uri;
    let observed_generation_token = root
        .read_generation_token(canonical_uri)
        .map(|t| t.value)
        .unwrap_or_else(|_| "missing".to_owned());
    let pinned_generation_token = token.compare_before_write_generation_token.value.clone();

    let external_bytes = root.read_bytes(canonical_uri).ok();
    let diff_subject_ref = format!("save-review:{packet_id}:diff");
    let diff = materialize_diff_record(
        source_fidelity,
        local_content,
        external_bytes.as_deref(),
        &packet_id,
        &diff_subject_ref,
    );
    let offered_choices = offered_choices_for_state(outcome, &diff, reviewed_external_state, token);

    SaveReviewSheetRecord {
        record_kind: "save_review_sheet_record".to_string(),
        schema_version: 1,
        generated_at,
        packet_id,
        outcome: outcome.as_str().to_string(),
        presentation_uri: format!("{presentation_uri}"),
        canonical_uri: format!("{canonical_uri}"),
        pinned_generation_token,
        observed_generation_token,
        diff,
        offered_choices,
        selected_choice: None,
        selected_at: None,
    }
}

fn materialize_diff_record(
    source_fidelity: &SourceFidelityRecord,
    local_content: &[u8],
    external_bytes: Option<&[u8]>,
    case_id: &str,
    subject_ref: &str,
) -> SaveReviewDiffRecord {
    let content_integrity_warnings =
        project_save_review_diff_content_integrity(case_id, subject_ref, local_content);

    let Some(external_bytes) = external_bytes else {
        return SaveReviewDiffRecord {
            diff_availability: "summary_only".to_string(),
            content_kind: "unknown".to_string(),
            summary: None,
            preview_lines: Vec::new(),
            content_integrity_warnings,
        };
    };

    let local_text = std::str::from_utf8(local_content).ok();
    let external_text = std::str::from_utf8(external_bytes).ok();
    let encoding_is_binary_like = matches!(
        source_fidelity.detected_encoding,
        aureline_workspace::save::DetectedEncoding::UnknownBinaryLike
    );

    match (local_text, external_text) {
        (Some(local), Some(external)) if !encoding_is_binary_like => {
            let (summary, preview_lines) = line_diff_preview(external, local, 36);
            SaveReviewDiffRecord {
                diff_availability: "available".to_string(),
                content_kind: "text".to_string(),
                summary: Some(summary),
                preview_lines,
                content_integrity_warnings,
            }
        }
        _ => SaveReviewDiffRecord {
            diff_availability: "binary_only".to_string(),
            content_kind: "binary".to_string(),
            summary: Some(SaveReviewDiffSummary {
                changed_hunk_count: 0,
                external_line_change_count: 0,
                local_line_change_count: 0,
                metadata_only: false,
                summary_text: "Binary or non-UTF8 content; diff preview unavailable.".to_string(),
            }),
            preview_lines: Vec::new(),
            content_integrity_warnings,
        },
    }
}

/// Projects shared content-integrity warnings for a save-review diff body.
pub fn project_save_review_diff_content_integrity(
    case_id: &str,
    subject_ref: &str,
    local_content: &[u8],
) -> Vec<ContentIntegrityWarningRecord> {
    std::str::from_utf8(local_content)
        .map(|text| {
            project_content_integrity_warnings(
                case_id,
                ContentIntegritySurfaceKind::Diff,
                subject_ref,
                text,
            )
        })
        .unwrap_or_default()
}

fn offered_choices_for_state(
    outcome: SaveOutcome,
    diff: &SaveReviewDiffRecord,
    reviewed_external_state: bool,
    token: &SaveTargetToken,
) -> Vec<SaveReviewChoiceOffer> {
    let diff_available = diff.diff_availability == "available";
    let compare_enabled = diff_available;

    let base_overwrite_block_reason = match outcome {
        SaveOutcome::WrongTargetPrevented => "alias_ambiguous",
        SaveOutcome::WatcherUncertainty => "watcher_uncertain",
        SaveOutcome::ReadOnlyOrPolicyBlocked => "read_only",
        SaveOutcome::ReviewRequiredBeforeSave | SaveOutcome::ReviewRequiredBeforeRename => {
            "policy_blocked"
        }
        _ => "none",
    };

    let overwrite_enabled = token.permission_snapshot.writable
        && diff_available
        && reviewed_external_state
        && base_overwrite_block_reason == "none";

    let overwrite_forbidden_reason = if !token.permission_snapshot.writable {
        "read_only"
    } else if !diff_available {
        "no_diff_basis"
    } else if !reviewed_external_state {
        "user_not_reviewed_external_state"
    } else {
        base_overwrite_block_reason
    };

    let compare_forbidden_reason = if compare_enabled {
        "none"
    } else {
        "no_diff_basis"
    };

    vec![
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::Compare.as_str().to_string(),
            offered: true,
            enabled: compare_enabled,
            forbidden_reason: compare_forbidden_reason.to_string(),
            requires_diff_metadata: true,
            requires_checkpoint: false,
            journal_implication: "review_checkpoint_only".to_string(),
            resulting_authoritative_state_if_selected: "no_authoritative_state_yet".to_string(),
        },
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::Overwrite.as_str().to_string(),
            offered: true,
            enabled: overwrite_enabled,
            forbidden_reason: overwrite_forbidden_reason.to_string(),
            requires_diff_metadata: true,
            requires_checkpoint: true,
            journal_implication: "restore_via_checkpoint_after_write".to_string(),
            resulting_authoritative_state_if_selected: "authoritative_buffer_written".to_string(),
        },
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::Merge.as_str().to_string(),
            offered: true,
            enabled: false,
            forbidden_reason: if diff.diff_availability == "binary_only" {
                "binary_unmergeable".to_string()
            } else if diff_available {
                "merge_strategy_unavailable".to_string()
            } else {
                "no_diff_basis".to_string()
            },
            requires_diff_metadata: true,
            requires_checkpoint: true,
            journal_implication: "local_and_external_checkpoints".to_string(),
            resulting_authoritative_state_if_selected: "authoritative_merged_result".to_string(),
        },
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::Reload.as_str().to_string(),
            offered: true,
            enabled: false,
            forbidden_reason: "not_reload_safe".to_string(),
            requires_diff_metadata: true,
            requires_checkpoint: true,
            journal_implication: "local_buffer_checkpoint".to_string(),
            resulting_authoritative_state_if_selected: "authoritative_external_loaded".to_string(),
        },
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::Retry.as_str().to_string(),
            offered: true,
            enabled: true,
            forbidden_reason: "none".to_string(),
            requires_diff_metadata: false,
            requires_checkpoint: false,
            journal_implication: "revalidation_attempt".to_string(),
            resulting_authoritative_state_if_selected: "authoritative_remote_revalidated"
                .to_string(),
        },
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::SaveAs.as_str().to_string(),
            offered: true,
            enabled: false,
            forbidden_reason: "target_missing".to_string(),
            requires_diff_metadata: false,
            requires_checkpoint: true,
            journal_implication: "local_buffer_checkpoint".to_string(),
            resulting_authoritative_state_if_selected: "authoritative_exported_to_new_target"
                .to_string(),
        },
        SaveReviewChoiceOffer {
            choice: SaveReviewChoiceKey::Cancel.as_str().to_string(),
            offered: true,
            enabled: true,
            forbidden_reason: "none".to_string(),
            requires_diff_metadata: false,
            requires_checkpoint: false,
            journal_implication: "no_mutation".to_string(),
            resulting_authoritative_state_if_selected: "last_known_good_stale".to_string(),
        },
    ]
}

fn line_diff_preview(
    external: &str,
    local: &str,
    max_preview_lines: usize,
) -> (SaveReviewDiffSummary, Vec<String>) {
    let external_lines: Vec<&str> = external.lines().collect();
    let local_lines: Vec<&str> = local.lines().collect();

    let mut prefix = 0usize;
    let prefix_limit = external_lines.len().min(local_lines.len());
    while prefix < prefix_limit && external_lines[prefix] == local_lines[prefix] {
        prefix += 1;
    }

    let mut suffix = 0usize;
    while suffix < external_lines.len().saturating_sub(prefix)
        && suffix < local_lines.len().saturating_sub(prefix)
        && external_lines[external_lines.len() - 1 - suffix]
            == local_lines[local_lines.len() - 1 - suffix]
    {
        suffix += 1;
    }

    let external_changed = external_lines.len().saturating_sub(prefix + suffix);
    let local_changed = local_lines.len().saturating_sub(prefix + suffix);

    let summary_text = if external_changed == 0 && local_changed == 0 {
        "No textual differences detected.".to_string()
    } else {
        format!(
            "Diff preview around line {} (external change lines: {}, local change lines: {}).",
            prefix.saturating_add(1),
            external_changed,
            local_changed
        )
    };

    let summary = SaveReviewDiffSummary {
        changed_hunk_count: if external_changed == 0 && local_changed == 0 {
            0
        } else {
            1
        },
        external_line_change_count: external_changed as u32,
        local_line_change_count: local_changed as u32,
        metadata_only: false,
        summary_text,
    };

    let mut lines = Vec::new();
    let context = 2usize;
    let start_external = prefix.saturating_sub(context);
    let end_external =
        (external_lines.len().saturating_sub(suffix) + context).min(external_lines.len());
    let end_local = (local_lines.len().saturating_sub(suffix) + context).min(local_lines.len());

    for idx in start_external..prefix {
        if lines.len() >= max_preview_lines {
            break;
        }
        lines.push(format!("  {}", external_lines[idx]));
    }
    for idx in prefix..external_lines.len().saturating_sub(suffix) {
        if lines.len() >= max_preview_lines {
            break;
        }
        lines.push(format!("- {}", external_lines[idx]));
    }
    for idx in prefix..local_lines.len().saturating_sub(suffix) {
        if lines.len() >= max_preview_lines {
            break;
        }
        lines.push(format!("+ {}", local_lines[idx]));
    }
    if lines.len() < max_preview_lines {
        for idx in external_lines.len().saturating_sub(suffix)..end_external {
            if lines.len() >= max_preview_lines {
                break;
            }
            lines.push(format!("  {}", external_lines[idx]));
        }
        // If the common suffix contexts do not align, append the local
        // counterpart so the viewer can still see the stable tail.
        for idx in local_lines.len().saturating_sub(suffix)..end_local {
            if lines.len() >= max_preview_lines {
                break;
            }
            if external_lines
                .get(idx)
                .is_some_and(|line| *line == local_lines[idx])
            {
                continue;
            }
            lines.push(format!("  {}", local_lines[idx]));
        }
    }

    (summary, lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_vfs::save::open_save_target;
    use aureline_vfs::{
        CapabilityFlags, CaseSensitivity, HookCounters, NormalizationForm, PermissionSnapshot,
        RootClass, SymlinkEscapePolicy, SyntheticRootBuilder, VfsUri,
    };
    use aureline_workspace::save::{
        BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent,
        FinalNewlineDetected, NewlineModeDetected,
    };
    use serde::Deserialize;
    use std::path::Path;

    #[test]
    fn line_diff_preview_produces_reasonable_summary_and_preview() {
        let external = "alpha\nbeta\ngamma\n";
        let local = "alpha\nbeta\nDELTA\n";

        let (summary, lines) = line_diff_preview(external, local, 32);
        assert_eq!(summary.changed_hunk_count, 1);
        assert!(summary.external_line_change_count > 0);
        assert!(summary.local_line_change_count > 0);
        assert!(lines.iter().any(|line| line.starts_with("- ")));
        assert!(lines.iter().any(|line| line.starts_with("+ ")));
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct SaveReviewFixtureRecord {
        input: SaveReviewFixtureInput,
        expected: SaveReviewSheetRecord,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct SaveReviewFixtureInput {
        packet_id: String,
        outcome: String,
        generated_at: String,
        reviewed_external_state: bool,
        detected_encoding: String,
        presentation_uri: String,
        canonical_uri: String,
        logical_uri: String,
        display_label: String,
        strongest_token_base: String,
        initial_generation: u64,
        initial_external_content: String,
        external_content_after_change: String,
        local_content: String,
    }

    fn fixture_flags() -> CapabilityFlags {
        CapabilityFlags {
            supports_atomic_replace: true,
            supports_in_place_write: true,
            supports_conditional_remote_write: false,
            case_sensitivity: CaseSensitivity::InsensitivePreserving,
            unicode_normalization: NormalizationForm::MixedObserved,
            supports_case_only_rename: true,
            supports_unicode_normalization_rename: true,
            symlink_escape_policy: SymlinkEscapePolicy::Warn,
            read_only: false,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        }
    }

    fn parse_outcome(value: &str) -> SaveOutcome {
        match value {
            "external_change_detected" => SaveOutcome::ExternalChangeDetected,
            "watcher_uncertainty" => SaveOutcome::WatcherUncertainty,
            "save_conflict" => SaveOutcome::SaveConflict,
            "wrong_target_prevented" => SaveOutcome::WrongTargetPrevented,
            "read_only_or_policy_blocked" => SaveOutcome::ReadOnlyOrPolicyBlocked,
            "review_required_before_save" => SaveOutcome::ReviewRequiredBeforeSave,
            "review_required_before_rename" => SaveOutcome::ReviewRequiredBeforeRename,
            other => panic!("unsupported save outcome in fixture: {other}"),
        }
    }

    fn parse_encoding(value: &str) -> DetectedEncoding {
        match value {
            "utf8" => DetectedEncoding::Utf8,
            "utf8_bom" => DetectedEncoding::Utf8Bom,
            "unknown_binary_like" => DetectedEncoding::UnknownBinaryLike,
            other => panic!("unsupported detected_encoding in fixture: {other}"),
        }
    }

    fn source_fidelity_for_encoding(encoding: DetectedEncoding) -> SourceFidelityRecord {
        SourceFidelityRecord {
            detected_encoding: encoding,
            detection_source: DetectionSource::Utf8Heuristic,
            bom_state_detected: BomStateDetected::Absent,
            newline_mode_detected: NewlineModeDetected::Lf,
            final_newline_detected: FinalNewlineDetected::Present,
            executable_intent: ExecutableIntent::NonExecutable,
        }
    }

    fn load_fixture(path: &Path) -> String {
        std::fs::read_to_string(path).expect("fixture must read")
    }

    #[test]
    fn materializes_save_review_sheet_cases_from_fixtures() {
        let root_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/save/save_review_cases");

        for entry in std::fs::read_dir(&root_dir).expect("fixture directory must exist") {
            let entry = entry.expect("fixture directory entry must read");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            let payload = load_fixture(&path);
            let fixture: SaveReviewFixtureRecord =
                serde_json::from_str(&payload).expect("save review fixture must parse");

            let presentation_uri = VfsUri::parse(fixture.input.presentation_uri.clone())
                .expect("fixture presentation_uri must parse");

            let mut root =
                SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, fixture_flags())
                    .with_workspace_id("ws-save-review")
                    .add_canonical_object(
                        fixture.input.canonical_uri.clone(),
                        fixture.input.logical_uri.clone(),
                        NormalizationForm::Nfc,
                        fixture.input.strongest_token_base.clone(),
                        fixture.input.initial_generation,
                        vec![],
                        PermissionSnapshot::writable_default(),
                        vec![],
                        fixture.input.initial_external_content.clone().into_bytes(),
                    )
                    .add_presentation(
                        fixture.input.presentation_uri.clone(),
                        fixture.input.display_label.clone(),
                        fixture.input.canonical_uri.clone(),
                        None,
                        vec!["presentation -> canonical".to_owned()],
                    )
                    .build();

            let mut counters = HookCounters::default();
            let token =
                open_save_target(&root, &presentation_uri, "mono:fixture:open", &mut counters)
                    .expect("open_save_target must succeed for fixture");

            root.apply_commit(
                &fixture.input.canonical_uri,
                fixture
                    .input
                    .external_content_after_change
                    .clone()
                    .into_bytes(),
            )
            .expect("fixture canonical object must exist for external change simulation");

            let outcome = parse_outcome(&fixture.input.outcome);
            let encoding = parse_encoding(&fixture.input.detected_encoding);
            let source_fidelity = source_fidelity_for_encoding(encoding);

            let record = materialize_save_review_sheet_record(
                &root,
                &token,
                &source_fidelity,
                fixture.input.packet_id.clone(),
                outcome,
                fixture.input.generated_at.clone(),
                fixture.input.local_content.as_bytes(),
                fixture.input.reviewed_external_state,
            );

            assert_eq!(
                record,
                fixture.expected,
                "save review sheet record mismatch for fixture {}",
                path.display()
            );
        }
    }

    #[test]
    fn save_review_lines_with_path_truth_appends_chip_section() {
        let presentation_uri =
            VfsUri::parse("file:///ws/save_review.txt".to_owned()).expect("uri must parse");
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, fixture_flags())
            .with_workspace_id("ws-save-review")
            .add_canonical_object(
                "file:///ws/save_review.txt".to_owned(),
                "aureline-ws://ws-save-review/root-1/save_review.txt".to_owned(),
                NormalizationForm::Nfc,
                "dev:1/ino:200".to_owned(),
                5,
                vec![],
                PermissionSnapshot::writable_default(),
                vec![],
                b"alpha\nbeta\ngamma\n".to_vec(),
            )
            .add_presentation(
                "file:///ws/save_review.txt".to_owned(),
                "save_review.txt".to_owned(),
                "file:///ws/save_review.txt".to_owned(),
                None,
                vec!["presentation -> canonical".to_owned()],
            )
            .build();

        let mut counters = HookCounters::default();
        let token = open_save_target(&root, &presentation_uri, "mono:fixture:open", &mut counters)
            .expect("open_save_target must succeed");

        let record = materialize_save_review_sheet_record(
            &root,
            &token,
            &source_fidelity_for_encoding(DetectedEncoding::Utf8),
            "save_packet:lines:01".to_owned(),
            SaveOutcome::ExternalChangeDetected,
            "2026-05-09T00:00:00Z".to_owned(),
            b"alpha\nbeta\nDELTA\n",
            false,
        );

        let lines = save_review_sheet_lines_with_path_truth(&record, &token, 0);
        assert!(
            lines.iter().any(|line| line == "Path truth:"),
            "expected the path-truth section to be appended",
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("opened at its canonical path")),
            "expected the chip summary line for a direct open",
        );
    }

    #[test]
    fn save_review_lines_with_path_truth_explains_alias_redirect() {
        let presentation_uri =
            VfsUri::parse("file:///ws/docs/current.md".to_owned()).expect("uri must parse");
        let aliases = vec![
            aureline_vfs::Alias {
                alias_uri: VfsUri::parse("file:///ws/docs/current.md".to_owned()).unwrap(),
                alias_kind: aureline_vfs::AliasKind::Symlink,
                resolution_chain: vec!["-> README.md".to_owned()],
            },
            aureline_vfs::Alias {
                alias_uri: VfsUri::parse("file:///ws/README.md".to_owned()).unwrap(),
                alias_kind: aureline_vfs::AliasKind::Symlink,
                resolution_chain: vec!["-> canonical".to_owned()],
            },
        ];
        let root = SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, fixture_flags())
            .with_workspace_id("ws-save-review")
            .add_canonical_object(
                "file:///ws/README.md".to_owned(),
                "aureline-ws://ws-save-review/root-1/README.md".to_owned(),
                NormalizationForm::Nfc,
                "dev:1/ino:300".to_owned(),
                4,
                vec![],
                PermissionSnapshot::writable_default(),
                aliases,
                b"alpha\n".to_vec(),
            )
            .add_presentation(
                "file:///ws/docs/current.md".to_owned(),
                "current.md".to_owned(),
                "file:///ws/README.md".to_owned(),
                Some(aureline_vfs::AliasKind::Symlink),
                vec!["-> README.md".to_owned()],
            )
            .add_presentation(
                "file:///ws/README.md".to_owned(),
                "README.md".to_owned(),
                "file:///ws/README.md".to_owned(),
                None,
                vec!["-> canonical".to_owned()],
            )
            .build();

        let mut counters = HookCounters::default();
        let token = open_save_target(&root, &presentation_uri, "mono:alias:open", &mut counters)
            .expect("open_save_target must succeed");

        let record = materialize_save_review_sheet_record(
            &root,
            &token,
            &source_fidelity_for_encoding(DetectedEncoding::Utf8),
            "save_packet:alias:01".to_owned(),
            SaveOutcome::WrongTargetPrevented,
            "2026-05-09T00:00:00Z".to_owned(),
            b"alpha\nDELTA\n",
            false,
        );

        let lines = save_review_sheet_lines_with_path_truth(&record, &token, 0);
        assert!(lines.iter().any(|line| line.contains("via_symlink")));
        assert!(
            lines
                .iter()
                .any(|line| line.contains("file:///ws/README.md")),
            "alias inspector body must surface the canonical URI",
        );
        assert!(
            lines.iter().any(|line| line.contains("Save target review")),
            "save-target review section must render when the open redirects",
        );
    }
}
