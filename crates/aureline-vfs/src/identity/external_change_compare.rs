//! External-change compare projection.
//!
//! Compare-before-write is the final safety check before a staged buffer is
//! allowed to replace durable bytes. This module exposes that check as an
//! inspectable record so editor, Git, restore, and support surfaces can route
//! external changes into compare/reload/merge choices without silently
//! overwriting the target.

use super::{filesystem_identity_reference_set, FilesystemIdentityReferenceSet};
use crate::capabilities::AtomicWriteMode;
use crate::hooks::HookCounters;
use crate::roots::VfsRoot;
use crate::save::{GenerationTokenKind, SaveOutcome, SaveTargetToken};
use crate::uri_model::VfsUri;

/// Result of comparing a pinned save target with the current target state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalChangeCompareOutcome {
    /// The current canonical target still matches the pinned token.
    Unchanged,
    /// The target generation changed outside the editor on a local-like root.
    ExternalChangeDetected,
    /// The target changed on a conditional remote write path.
    SaveConflict,
    /// The presentation path now resolves to a different target or no target.
    WrongTargetPrevented,
    /// The current target bytes could not be read for a reviewable diff.
    CurrentBytesUnavailable,
}

impl ExternalChangeCompareOutcome {
    /// Returns the stable string vocabulary for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::ExternalChangeDetected => "external_change_detected",
            Self::SaveConflict => "save_conflict",
            Self::WrongTargetPrevented => "wrong_target_prevented",
            Self::CurrentBytesUnavailable => "current_bytes_unavailable",
        }
    }
}

/// Diff availability for an external-change compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalChangeDiffAvailability {
    /// No diff is needed because the pinned token still matches.
    NotNeeded,
    /// Text diff metadata and preview lines are available.
    Available,
    /// Current or local bytes are not UTF-8 text.
    BinaryOnly,
    /// The current target could not be read.
    CurrentBytesUnavailable,
}

impl ExternalChangeDiffAvailability {
    /// Returns the stable string vocabulary for this availability state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNeeded => "not_needed",
            Self::Available => "available",
            Self::BinaryOnly => "binary_only",
            Self::CurrentBytesUnavailable => "current_bytes_unavailable",
        }
    }
}

/// Content class for the external-change diff projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalChangeContentKind {
    /// Both local and current target bytes decoded as UTF-8.
    Text,
    /// One side did not decode as UTF-8.
    Binary,
    /// No current bytes were available to classify.
    Unknown,
}

impl ExternalChangeContentKind {
    /// Returns the stable string vocabulary for this content kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Binary => "binary",
            Self::Unknown => "unknown",
        }
    }
}

/// Resolution action a surface may offer after compare-before-write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalChangeResolutionAction {
    /// Continue the write because the token still matches.
    Write,
    /// Open the compare/diff surface before any write.
    Compare,
    /// Reload the current target bytes into the buffer after review.
    ReloadExternal,
    /// Open a merge flow that combines local and external changes.
    Merge,
    /// Save the local buffer to a different target.
    SaveAs,
    /// Retry identity and generation-token resolution.
    Recompare,
    /// Inspect alias/canonical path details before selecting an action.
    OpenAliasDetails,
    /// Cancel without mutating durable bytes.
    Cancel,
}

impl ExternalChangeResolutionAction {
    /// Returns the stable string vocabulary for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Write => "write",
            Self::Compare => "compare",
            Self::ReloadExternal => "reload_external",
            Self::Merge => "merge",
            Self::SaveAs => "save_as",
            Self::Recompare => "recompare",
            Self::OpenAliasDetails => "open_alias_details",
            Self::Cancel => "cancel",
        }
    }
}

/// Diff metadata for a local buffer compared with current target bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalChangeDiff {
    /// Whether text preview lines are available for review.
    pub availability: ExternalChangeDiffAvailability,
    /// Content class observed while preparing the diff.
    pub content_kind: ExternalChangeContentKind,
    /// Number of changed hunks in the compact preview.
    pub changed_hunk_count: u32,
    /// Number of changed lines from the current external target.
    pub external_line_change_count: u32,
    /// Number of changed lines from the local staged buffer.
    pub local_line_change_count: u32,
    /// Human-readable summary for the compare surface.
    pub summary: String,
    /// Compact diff preview lines with `-`, `+`, or context prefixes.
    pub preview_lines: Vec<String>,
}

/// Reviewable compare-before-write record for one pinned save token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalChangeCompareRecord {
    /// Shared identity refs quoted by editor, Git, restore, and mutation flows.
    pub identity_references: FilesystemIdentityReferenceSet,
    /// URI the user opened when the save token was pinned.
    pub presentation_uri: VfsUri,
    /// Canonical URI captured on the pinned save token.
    pub pinned_canonical_uri: VfsUri,
    /// Canonical URI observed after re-resolving the presentation URI.
    pub observed_canonical_uri: Option<VfsUri>,
    /// Generation-token kind captured on the pinned save token.
    pub pinned_generation_token_kind: GenerationTokenKind,
    /// Generation-token value captured on the pinned save token.
    pub pinned_generation_token_value: String,
    /// Generation-token kind observed at compare time.
    pub observed_generation_token_kind: Option<GenerationTokenKind>,
    /// Generation-token value observed at compare time.
    pub observed_generation_token_value: Option<String>,
    /// Result of the external-change compare.
    pub outcome: ExternalChangeCompareOutcome,
    /// Save outcome that must be recorded if the compare blocks the write.
    pub blocking_save_outcome: Option<SaveOutcome>,
    /// True when the user must resolve a review path before writing.
    pub review_required: bool,
    /// True when overwriting without review would violate save safety.
    pub silent_overwrite_forbidden: bool,
    /// Diff metadata for current target bytes versus staged buffer bytes.
    pub diff: ExternalChangeDiff,
    /// Ordered safe actions a surface may offer.
    pub resolution_actions: Vec<ExternalChangeResolutionAction>,
    /// Human-readable explanation lines for support and review surfaces.
    pub explainers: Vec<String>,
}

/// Compares a pinned [`SaveTargetToken`] with current root state.
///
/// The function never writes bytes. It re-resolves the presentation path,
/// re-reads the canonical generation token, and only reads current target bytes
/// to build a review diff after a mismatch.
pub fn compare_external_change(
    root: &dyn VfsRoot,
    token: &SaveTargetToken,
    local_content: &[u8],
    counters: &mut HookCounters,
) -> ExternalChangeCompareRecord {
    counters.vfs_save_compare_before_write += 1;

    let identity_references = filesystem_identity_reference_set(&token.identity);
    let presentation_uri = token.identity.presentation_path.uri.clone();
    let pinned_canonical_uri = token
        .identity
        .canonical_filesystem_object
        .canonical_uri
        .clone();
    let pinned_generation_token_kind = token.compare_before_write_generation_token.kind;
    let pinned_generation_token_value = token.compare_before_write_generation_token.value.clone();

    let observed_identity = match root.identity_record(&presentation_uri) {
        Ok(identity) => identity,
        Err(err) => {
            counters.vfs_save_conflict += 1;
            let explainer = format!(
                "presentation path {presentation_uri} no longer resolves to the pinned target: {err}"
            );
            return blocked_record(BlockedRecordInput {
                identity_references,
                presentation_uri,
                pinned_canonical_uri,
                observed_canonical_uri: None,
                pinned_generation_token_kind,
                pinned_generation_token_value,
                observed_generation_token_kind: None,
                observed_generation_token_value: None,
                outcome: ExternalChangeCompareOutcome::WrongTargetPrevented,
                blocking_save_outcome: SaveOutcome::WrongTargetPrevented,
                diff: unavailable_diff("Current target cannot be resolved for comparison."),
                resolution_actions: vec![
                    ExternalChangeResolutionAction::OpenAliasDetails,
                    ExternalChangeResolutionAction::SaveAs,
                    ExternalChangeResolutionAction::Cancel,
                ],
                explainer,
            });
        }
    };

    let observed_canonical_uri = observed_identity
        .canonical_filesystem_object
        .canonical_uri
        .clone();
    if observed_canonical_uri != pinned_canonical_uri {
        counters.vfs_save_conflict += 1;
        let explainer = format!(
            "presentation path {presentation_uri} was pinned to {pinned_canonical_uri} but now resolves to {observed_canonical_uri}; no write is allowed."
        );
        return blocked_record(BlockedRecordInput {
            identity_references,
            presentation_uri,
            pinned_canonical_uri,
            observed_canonical_uri: Some(observed_canonical_uri.clone()),
            pinned_generation_token_kind,
            pinned_generation_token_value,
            observed_generation_token_kind: None,
            observed_generation_token_value: None,
            outcome: ExternalChangeCompareOutcome::WrongTargetPrevented,
            blocking_save_outcome: SaveOutcome::WrongTargetPrevented,
            diff: unavailable_diff(
                "Presentation path now resolves to a different canonical target.",
            ),
            resolution_actions: vec![
                ExternalChangeResolutionAction::OpenAliasDetails,
                ExternalChangeResolutionAction::SaveAs,
                ExternalChangeResolutionAction::Cancel,
            ],
            explainer,
        });
    }

    let current_generation = match root.read_generation_token(&pinned_canonical_uri) {
        Ok(generation) => generation,
        Err(err) => {
            counters.vfs_save_conflict += 1;
            let explainer = format!(
                "canonical target {pinned_canonical_uri} could not provide a generation token: {err}"
            );
            return blocked_record(BlockedRecordInput {
                identity_references,
                presentation_uri,
                pinned_canonical_uri,
                observed_canonical_uri: Some(observed_canonical_uri),
                pinned_generation_token_kind,
                pinned_generation_token_value,
                observed_generation_token_kind: None,
                observed_generation_token_value: None,
                outcome: ExternalChangeCompareOutcome::WrongTargetPrevented,
                blocking_save_outcome: SaveOutcome::WrongTargetPrevented,
                diff: unavailable_diff("Current generation token cannot be read."),
                resolution_actions: vec![
                    ExternalChangeResolutionAction::Recompare,
                    ExternalChangeResolutionAction::SaveAs,
                    ExternalChangeResolutionAction::Cancel,
                ],
                explainer,
            });
        }
    };

    if current_generation.kind == pinned_generation_token_kind
        && current_generation.value == pinned_generation_token_value
    {
        let explainer = format!(
            "compare-before-write matched {kind}:{value} for {pinned_canonical_uri}.",
            kind = pinned_generation_token_kind.as_str(),
            value = pinned_generation_token_value,
        );
        return ExternalChangeCompareRecord {
            identity_references,
            presentation_uri,
            pinned_canonical_uri,
            observed_canonical_uri: Some(observed_canonical_uri),
            pinned_generation_token_kind,
            pinned_generation_token_value,
            observed_generation_token_kind: Some(current_generation.kind),
            observed_generation_token_value: Some(current_generation.value),
            outcome: ExternalChangeCompareOutcome::Unchanged,
            blocking_save_outcome: None,
            review_required: false,
            silent_overwrite_forbidden: false,
            diff: ExternalChangeDiff {
                availability: ExternalChangeDiffAvailability::NotNeeded,
                content_kind: ExternalChangeContentKind::Unknown,
                changed_hunk_count: 0,
                external_line_change_count: 0,
                local_line_change_count: 0,
                summary:
                    "Pinned generation token still matches; no external-change review is required."
                        .to_owned(),
                preview_lines: vec![],
            },
            resolution_actions: vec![ExternalChangeResolutionAction::Write],
            explainers: vec![explainer],
        };
    }

    counters.vfs_external_change_detected += 1;
    counters.vfs_save_conflict += 1;
    let (outcome, save_outcome) =
        if token.atomic_write_mode == AtomicWriteMode::ConditionalRemoteWrite {
            (
                ExternalChangeCompareOutcome::SaveConflict,
                SaveOutcome::SaveConflict,
            )
        } else {
            (
                ExternalChangeCompareOutcome::ExternalChangeDetected,
                SaveOutcome::ExternalChangeDetected,
            )
        };
    let diff = external_diff(root, &pinned_canonical_uri, local_content);
    let resolution_actions = match diff.availability {
        ExternalChangeDiffAvailability::Available => vec![
            ExternalChangeResolutionAction::Compare,
            ExternalChangeResolutionAction::Merge,
            ExternalChangeResolutionAction::ReloadExternal,
            ExternalChangeResolutionAction::SaveAs,
            ExternalChangeResolutionAction::Cancel,
        ],
        ExternalChangeDiffAvailability::BinaryOnly => vec![
            ExternalChangeResolutionAction::Compare,
            ExternalChangeResolutionAction::ReloadExternal,
            ExternalChangeResolutionAction::SaveAs,
            ExternalChangeResolutionAction::Cancel,
        ],
        ExternalChangeDiffAvailability::CurrentBytesUnavailable => vec![
            ExternalChangeResolutionAction::Recompare,
            ExternalChangeResolutionAction::SaveAs,
            ExternalChangeResolutionAction::Cancel,
        ],
        ExternalChangeDiffAvailability::NotNeeded => vec![ExternalChangeResolutionAction::Cancel],
    };

    ExternalChangeCompareRecord {
        identity_references,
        presentation_uri,
        pinned_canonical_uri: pinned_canonical_uri.clone(),
        observed_canonical_uri: Some(observed_canonical_uri),
        pinned_generation_token_kind,
        pinned_generation_token_value: pinned_generation_token_value.clone(),
        observed_generation_token_kind: Some(current_generation.kind),
        observed_generation_token_value: Some(current_generation.value.clone()),
        outcome,
        blocking_save_outcome: Some(save_outcome),
        review_required: true,
        silent_overwrite_forbidden: true,
        diff,
        resolution_actions,
        explainers: vec![format!(
            "generation token mismatch for {pinned_canonical_uri}: pinned {pinned} observed {observed}; write is blocked until review resolves the external change.",
            pinned = pinned_generation_token_value,
            observed = current_generation.value,
        )],
    }
}

struct BlockedRecordInput {
    identity_references: FilesystemIdentityReferenceSet,
    presentation_uri: VfsUri,
    pinned_canonical_uri: VfsUri,
    observed_canonical_uri: Option<VfsUri>,
    pinned_generation_token_kind: GenerationTokenKind,
    pinned_generation_token_value: String,
    observed_generation_token_kind: Option<GenerationTokenKind>,
    observed_generation_token_value: Option<String>,
    outcome: ExternalChangeCompareOutcome,
    blocking_save_outcome: SaveOutcome,
    diff: ExternalChangeDiff,
    resolution_actions: Vec<ExternalChangeResolutionAction>,
    explainer: String,
}

fn blocked_record(input: BlockedRecordInput) -> ExternalChangeCompareRecord {
    ExternalChangeCompareRecord {
        identity_references: input.identity_references,
        presentation_uri: input.presentation_uri,
        pinned_canonical_uri: input.pinned_canonical_uri,
        observed_canonical_uri: input.observed_canonical_uri,
        pinned_generation_token_kind: input.pinned_generation_token_kind,
        pinned_generation_token_value: input.pinned_generation_token_value,
        observed_generation_token_kind: input.observed_generation_token_kind,
        observed_generation_token_value: input.observed_generation_token_value,
        outcome: input.outcome,
        blocking_save_outcome: Some(input.blocking_save_outcome),
        review_required: true,
        silent_overwrite_forbidden: true,
        diff: input.diff,
        resolution_actions: input.resolution_actions,
        explainers: vec![input.explainer],
    }
}

fn unavailable_diff(summary: impl Into<String>) -> ExternalChangeDiff {
    ExternalChangeDiff {
        availability: ExternalChangeDiffAvailability::CurrentBytesUnavailable,
        content_kind: ExternalChangeContentKind::Unknown,
        changed_hunk_count: 0,
        external_line_change_count: 0,
        local_line_change_count: 0,
        summary: summary.into(),
        preview_lines: vec![],
    }
}

fn external_diff(
    root: &dyn VfsRoot,
    canonical_uri: &VfsUri,
    local_content: &[u8],
) -> ExternalChangeDiff {
    let Ok(external_bytes) = root.read_bytes(canonical_uri) else {
        return unavailable_diff("Current target bytes could not be read for comparison.");
    };
    let Some(external) = std::str::from_utf8(&external_bytes).ok() else {
        return binary_diff();
    };
    let Some(local) = std::str::from_utf8(local_content).ok() else {
        return binary_diff();
    };
    text_diff(external, local, 40)
}

fn binary_diff() -> ExternalChangeDiff {
    ExternalChangeDiff {
        availability: ExternalChangeDiffAvailability::BinaryOnly,
        content_kind: ExternalChangeContentKind::Binary,
        changed_hunk_count: 0,
        external_line_change_count: 0,
        local_line_change_count: 0,
        summary: "Binary or non-UTF8 content; textual diff preview unavailable.".to_owned(),
        preview_lines: vec![],
    }
}

fn text_diff(external: &str, local: &str, max_preview_lines: usize) -> ExternalChangeDiff {
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
    let changed_hunk_count = if external_changed == 0 && local_changed == 0 {
        0
    } else {
        1
    };
    let summary = if changed_hunk_count == 0 {
        "External and local text are identical despite the generation-token mismatch.".to_owned()
    } else {
        format!(
            "Diff preview around line {} (external change lines: {}, local change lines: {}).",
            prefix.saturating_add(1),
            external_changed,
            local_changed
        )
    };

    let mut preview_lines = Vec::new();
    let context = 2usize;
    let start_external = prefix.saturating_sub(context);
    let end_external =
        (external_lines.len().saturating_sub(suffix) + context).min(external_lines.len());
    let end_local = (local_lines.len().saturating_sub(suffix) + context).min(local_lines.len());

    for line in external_lines.iter().take(prefix).skip(start_external) {
        if preview_lines.len() >= max_preview_lines {
            break;
        }
        preview_lines.push(format!("  {line}"));
    }
    for line in external_lines
        .iter()
        .take(external_lines.len().saturating_sub(suffix))
        .skip(prefix)
    {
        if preview_lines.len() >= max_preview_lines {
            break;
        }
        preview_lines.push(format!("- {line}"));
    }
    for line in local_lines
        .iter()
        .take(local_lines.len().saturating_sub(suffix))
        .skip(prefix)
    {
        if preview_lines.len() >= max_preview_lines {
            break;
        }
        preview_lines.push(format!("+ {line}"));
    }
    if preview_lines.len() < max_preview_lines {
        for line in external_lines
            .iter()
            .take(end_external)
            .skip(external_lines.len().saturating_sub(suffix))
        {
            if preview_lines.len() >= max_preview_lines {
                break;
            }
            preview_lines.push(format!("  {line}"));
        }
        for (idx, line) in local_lines
            .iter()
            .enumerate()
            .take(end_local)
            .skip(local_lines.len().saturating_sub(suffix))
        {
            if preview_lines.len() >= max_preview_lines {
                break;
            }
            if external_lines
                .get(idx)
                .is_some_and(|external_line| external_line == line)
            {
                continue;
            }
            preview_lines.push(format!("  {line}"));
        }
    }

    ExternalChangeDiff {
        availability: ExternalChangeDiffAvailability::Available,
        content_kind: ExternalChangeContentKind::Text,
        changed_hunk_count,
        external_line_change_count: external_changed as u32,
        local_line_change_count: local_changed as u32,
        summary,
        preview_lines,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{
        CapabilityFlags, CaseSensitivity, NormalizationForm, RootClass, SymlinkEscapePolicy,
    };
    use crate::save::{open_save_target, PermissionSnapshot};
    use crate::{HookCounters, SyntheticRootBuilder};

    fn flags() -> CapabilityFlags {
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

    fn root() -> crate::SyntheticRoot {
        SyntheticRootBuilder::new("root-1", RootClass::LocalPosixLike, flags())
            .add_canonical_object(
                "file:///ws/main.rs",
                "aureline-ws://ws-alpha/root-1/main.rs",
                NormalizationForm::Nfc,
                "dev:1/ino:2",
                5,
                vec![],
                PermissionSnapshot::writable_default(),
                vec![],
                b"fn main() {}\n".to_vec(),
            )
            .add_presentation(
                "file:///ws/main.rs",
                "main.rs",
                "file:///ws/main.rs",
                None,
                vec!["-> canonical".to_owned()],
            )
            .with_workspace_id("ws-alpha")
            .build()
    }

    #[test]
    fn matching_generation_allows_write_action() {
        let root = root();
        let uri = VfsUri::parse("file:///ws/main.rs".to_owned()).unwrap();
        let mut counters = HookCounters::default();
        let token = open_save_target(&root, &uri, "mono:open", &mut counters).unwrap();

        let compare = compare_external_change(&root, &token, b"fn main() {}\n", &mut counters);

        assert_eq!(compare.outcome, ExternalChangeCompareOutcome::Unchanged);
        assert_eq!(
            compare.resolution_actions,
            vec![ExternalChangeResolutionAction::Write]
        );
        assert!(!compare.review_required);
    }

    #[test]
    fn generation_mismatch_blocks_and_builds_text_diff() {
        let mut root = root();
        let uri = VfsUri::parse("file:///ws/main.rs".to_owned()).unwrap();
        let mut counters = HookCounters::default();
        let token = open_save_target(&root, &uri, "mono:open", &mut counters).unwrap();
        root.apply_commit(
            "file:///ws/main.rs",
            b"fn main() { println!(\"external\"); }\n".to_vec(),
        );

        let compare = compare_external_change(
            &root,
            &token,
            b"fn main() { println!(\"local\"); }\n",
            &mut counters,
        );

        assert_eq!(
            compare.outcome,
            ExternalChangeCompareOutcome::ExternalChangeDetected
        );
        assert_eq!(
            compare.blocking_save_outcome,
            Some(SaveOutcome::ExternalChangeDetected)
        );
        assert!(compare.review_required);
        assert!(compare.silent_overwrite_forbidden);
        assert_eq!(
            compare.diff.availability,
            ExternalChangeDiffAvailability::Available
        );
        assert!(compare
            .diff
            .preview_lines
            .iter()
            .any(|line| line.starts_with("- ")));
        assert!(compare
            .diff
            .preview_lines
            .iter()
            .any(|line| line.starts_with("+ ")));
    }
}
