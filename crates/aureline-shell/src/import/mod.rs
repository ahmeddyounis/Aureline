//! Import profile classification and first-pass review records.
//!
//! The importer seed recognizes competitor configuration roots by filesystem
//! markers, then emits a review record the shell can display before any
//! profile or workspace state is written.

pub mod diff_review;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use diff_review::{
    materialize_import_diff_review_packet, reopen_retained_migration_report,
    write_import_diff_review_log, ImportDiffReviewPacket, ImportDiffReviewRow,
    ImportMappingClassification, ImportReportReopenSurface, ImportReviewDomain,
    RetainedMigrationReport, RetainedMigrationReportProjection, ShortcutDeltaReport,
};

/// Classifies a local folder that may contain competitor IDE configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompetitorConfigClassification {
    /// A folder containing a `.vscode/` workspace configuration directory.
    #[serde(rename = "vscode_workspace_root")]
    VSCodeWorkspaceRoot,
    /// A folder containing a JetBrains `.idea/` project configuration directory.
    #[serde(rename = "jetbrains_idea_root")]
    JetBrainsIdeaRoot,
    /// A folder that did not expose a supported competitor configuration marker.
    #[serde(rename = "unknown_config_root")]
    UnknownConfigRoot,
}

impl CompetitorConfigClassification {
    /// Returns the stable serialized token for this classification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VSCodeWorkspaceRoot => "vscode_workspace_root",
            Self::JetBrainsIdeaRoot => "jetbrains_idea_root",
            Self::UnknownConfigRoot => "unknown_config_root",
        }
    }

    /// Returns the Rust variant name shown in the first-pass import sheet.
    pub const fn variant_name(self) -> &'static str {
        match self {
            Self::VSCodeWorkspaceRoot => "VSCodeWorkspaceRoot",
            Self::JetBrainsIdeaRoot => "JetBrainsIdeaRoot",
            Self::UnknownConfigRoot => "UnknownConfigRoot",
        }
    }

    /// Returns a short display label for review surfaces.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::VSCodeWorkspaceRoot => "VS Code workspace root",
            Self::JetBrainsIdeaRoot => "JetBrains .idea root",
            Self::UnknownConfigRoot => "Unknown configuration root",
        }
    }
}

/// Decision class recorded on an import review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReviewDecisionClass {
    /// The import can be applied only after the user has reviewed the preview.
    ApplyAfterPreview,
    /// The import should not be applied.
    Decline,
    /// The import should be paused until a supported source is selected.
    Defer,
}

impl ImportReviewDecisionClass {
    /// Returns the stable serialized token for this decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApplyAfterPreview => "apply_after_preview",
            Self::Decline => "decline",
            Self::Defer => "defer",
        }
    }
}

/// Type of item discovered in a competitor configuration root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReviewItemKind {
    /// Directory marker proving the source family.
    MarkerDirectory,
    /// Settings file or directory.
    Settings,
    /// Keybinding or shortcut file.
    Keybindings,
    /// Snippet directory.
    Snippets,
    /// Task configuration file.
    Tasks,
    /// Launch or run configuration file/directory.
    LaunchConfiguration,
    /// Extension recommendation or enabled-plugin metadata.
    ExtensionHints,
    /// General project metadata.
    ProjectMetadata,
}

impl ImportReviewItemKind {
    /// Returns the stable serialized token for this item kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarkerDirectory => "marker_directory",
            Self::Settings => "settings",
            Self::Keybindings => "keybindings",
            Self::Snippets => "snippets",
            Self::Tasks => "tasks",
            Self::LaunchConfiguration => "launch_configuration",
            Self::ExtensionHints => "extension_hints",
            Self::ProjectMetadata => "project_metadata",
        }
    }
}

/// Per-item state shown in the import review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReviewItemState {
    /// The item was detected by marker/path presence and is ready for review.
    Discovered,
    /// The item is outside the first-pass importer scope.
    Deferred,
}

impl ImportReviewItemState {
    /// Returns the stable serialized token for this item state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Discovered => "discovered",
            Self::Deferred => "deferred",
        }
    }
}

/// One filesystem marker or known configuration item found by the importer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReviewItem {
    /// Path relative to the source root.
    pub source_relative_path: String,
    /// Semantic kind of the discovered item.
    pub item_kind: ImportReviewItemKind,
    /// First-pass review state for the item.
    pub item_state: ImportReviewItemState,
    /// Short review text shown in the sheet.
    pub summary: String,
}

/// Review record emitted before an import apply is allowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReviewRecord {
    /// Stable record kind for serialized review records.
    pub record_kind: String,
    /// Schema version for this lightweight shell-owned record.
    pub schema_version: u32,
    /// Deterministic review id for this source/destination pair.
    pub import_review_id: String,
    /// Source path or source ref selected by the user.
    pub source_path: String,
    /// Destination profile or workspace target selected for the import.
    pub destination_workspace_target: String,
    /// Detected source-family classification.
    pub classification: CompetitorConfigClassification,
    /// Decision class available from the current review state.
    pub decision_class: ImportReviewDecisionClass,
    /// User-visible review status.
    pub status_line: String,
    /// Filesystem markers and known configuration items discovered without
    /// reading file contents.
    pub discovered_items: Vec<ImportReviewItem>,
}

impl ImportReviewRecord {
    /// Builds a deferred review for an unresolved source ref.
    pub fn deferred_unresolved_source(
        source_ref: impl Into<String>,
        destination_workspace_target: impl Into<String>,
    ) -> Self {
        let source_path = source_ref.into();
        let destination_workspace_target = destination_workspace_target.into();
        let import_review_id = review_id_for(
            &source_path,
            &destination_workspace_target,
            CompetitorConfigClassification::UnknownConfigRoot,
        );
        Self {
            record_kind: "import_review_record".to_string(),
            schema_version: 1,
            import_review_id,
            source_path,
            destination_workspace_target,
            classification: CompetitorConfigClassification::UnknownConfigRoot,
            decision_class: ImportReviewDecisionClass::Defer,
            status_line:
                "Choose a readable VS Code .vscode or JetBrains .idea folder before applying."
                    .to_string(),
            discovered_items: Vec::new(),
        }
    }

    /// Returns a compact count string for sheet and activity-center rows.
    pub fn discovered_item_count_label(&self) -> String {
        let count = self.discovered_items.len();
        match count {
            0 => "0 discovered items".to_string(),
            1 => "1 discovered item".to_string(),
            _ => format!("{count} discovered items"),
        }
    }
}

/// Filesystem-marker classifier for competitor IDE configuration roots.
#[derive(Debug, Default, Clone, Copy)]
pub struct CompetitorConfigClassifier;

impl CompetitorConfigClassifier {
    /// Creates a classifier with the default marker set.
    pub const fn new() -> Self {
        Self
    }

    /// Classifies a path using marker directories only.
    pub fn classify(&self, source_path: impl AsRef<Path>) -> CompetitorConfigClassification {
        let source_path = source_path.as_ref();
        if source_path.join(".vscode").is_dir() {
            return CompetitorConfigClassification::VSCodeWorkspaceRoot;
        }
        if source_path.join(".idea").is_dir() {
            return CompetitorConfigClassification::JetBrainsIdeaRoot;
        }
        CompetitorConfigClassification::UnknownConfigRoot
    }

    /// Builds a review record for a source root and destination target.
    pub fn build_review(
        &self,
        source_path: impl AsRef<Path>,
        destination_workspace_target: impl Into<String>,
    ) -> ImportReviewRecord {
        let source_path = source_path.as_ref().to_path_buf();
        let destination_workspace_target = destination_workspace_target.into();
        let classification = self.classify(&source_path);
        let discovered_items = discovered_items_for(&source_path, classification);
        let decision_class = match classification {
            CompetitorConfigClassification::VSCodeWorkspaceRoot
            | CompetitorConfigClassification::JetBrainsIdeaRoot => {
                ImportReviewDecisionClass::ApplyAfterPreview
            }
            CompetitorConfigClassification::UnknownConfigRoot => ImportReviewDecisionClass::Defer,
        };
        let status_line = status_line_for(&source_path, classification, discovered_items.len());
        let source_path_label = source_path.display().to_string();
        let import_review_id = review_id_for(
            &source_path_label,
            &destination_workspace_target,
            classification,
        );
        ImportReviewRecord {
            record_kind: "import_review_record".to_string(),
            schema_version: 1,
            import_review_id,
            source_path: source_path_label,
            destination_workspace_target,
            classification,
            decision_class,
            status_line,
            discovered_items,
        }
    }
}

/// Writes an import review record into `.logs/import_reviews/`.
pub fn write_import_review_log(record: &ImportReviewRecord) {
    let root = PathBuf::from(".logs").join("import_reviews");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }
    let filename = format!(
        "{}.{}.json",
        sanitize_filename(&record.import_review_id),
        sanitize_filename(record.classification.as_str())
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

fn discovered_items_for(
    source_path: &Path,
    classification: CompetitorConfigClassification,
) -> Vec<ImportReviewItem> {
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => {
            let mut items = vec![item(
                ".vscode",
                ImportReviewItemKind::MarkerDirectory,
                "VS Code workspace marker directory discovered.",
            )];
            extend_existing(
                &mut items,
                source_path,
                &[
                    (
                        ".vscode/settings.json",
                        ImportReviewItemKind::Settings,
                        "VS Code workspace settings file discovered.",
                    ),
                    (
                        ".vscode/keybindings.json",
                        ImportReviewItemKind::Keybindings,
                        "VS Code keybindings file discovered.",
                    ),
                    (
                        ".vscode/snippets",
                        ImportReviewItemKind::Snippets,
                        "VS Code snippets directory discovered.",
                    ),
                    (
                        ".vscode/tasks.json",
                        ImportReviewItemKind::Tasks,
                        "VS Code tasks file discovered.",
                    ),
                    (
                        ".vscode/launch.json",
                        ImportReviewItemKind::LaunchConfiguration,
                        "VS Code launch configuration file discovered.",
                    ),
                    (
                        ".vscode/extensions.json",
                        ImportReviewItemKind::ExtensionHints,
                        "VS Code extension recommendations file discovered.",
                    ),
                ],
            );
            items
        }
        CompetitorConfigClassification::JetBrainsIdeaRoot => {
            let mut items = vec![item(
                ".idea",
                ImportReviewItemKind::MarkerDirectory,
                "JetBrains project marker directory discovered.",
            )];
            extend_existing(
                &mut items,
                source_path,
                &[
                    (
                        ".idea/workspace.xml",
                        ImportReviewItemKind::Settings,
                        "JetBrains workspace settings file discovered.",
                    ),
                    (
                        ".idea/misc.xml",
                        ImportReviewItemKind::ProjectMetadata,
                        "JetBrains project metadata file discovered.",
                    ),
                    (
                        ".idea/modules.xml",
                        ImportReviewItemKind::ProjectMetadata,
                        "JetBrains module metadata file discovered.",
                    ),
                    (
                        ".idea/inspectionProfiles",
                        ImportReviewItemKind::Settings,
                        "JetBrains inspection profile directory discovered.",
                    ),
                    (
                        ".idea/runConfigurations",
                        ImportReviewItemKind::LaunchConfiguration,
                        "JetBrains run configuration directory discovered.",
                    ),
                ],
            );
            items
        }
        CompetitorConfigClassification::UnknownConfigRoot => Vec::new(),
    }
}

fn extend_existing(
    items: &mut Vec<ImportReviewItem>,
    source_path: &Path,
    candidates: &[(&str, ImportReviewItemKind, &str)],
) {
    for (relative, kind, summary) in candidates {
        if source_path.join(relative).exists() {
            items.push(item(relative, *kind, summary));
        }
    }
}

fn item(
    source_relative_path: &str,
    item_kind: ImportReviewItemKind,
    summary: &str,
) -> ImportReviewItem {
    ImportReviewItem {
        source_relative_path: source_relative_path.to_string(),
        item_kind,
        item_state: ImportReviewItemState::Discovered,
        summary: summary.to_string(),
    }
}

fn status_line_for(
    source_path: &Path,
    classification: CompetitorConfigClassification,
    discovered_count: usize,
) -> String {
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => format!(
            "VS Code configuration root detected; {discovered_count} item(s) ready for review."
        ),
        CompetitorConfigClassification::JetBrainsIdeaRoot => format!(
            "JetBrains configuration root detected; {discovered_count} item(s) ready for review."
        ),
        CompetitorConfigClassification::UnknownConfigRoot => {
            if source_path.is_dir() {
                "Unknown configuration root: no .vscode or .idea marker directory found."
                    .to_string()
            } else {
                "Unknown configuration root: source path is not a readable directory.".to_string()
            }
        }
    }
}

fn review_id_for(
    source_path: &str,
    destination_workspace_target: &str,
    classification: CompetitorConfigClassification,
) -> String {
    format!(
        "import-review-{:016x}",
        fnv1a_64(&format!(
            "{}\n{}\n{}",
            source_path,
            destination_workspace_target,
            classification.as_str()
        ))
    )
}

fn fnv1a_64(value: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/import/m1_classifier_cases")
            .join(name)
    }

    #[test]
    fn import_classifier_detects_vscode_fixture() {
        let classifier = CompetitorConfigClassifier::new();
        let root = fixture_root("vscode_workspace");
        let review = classifier.build_review(&root, "profile:default");

        assert_eq!(
            review.classification,
            CompetitorConfigClassification::VSCodeWorkspaceRoot
        );
        assert_eq!(
            review.decision_class,
            ImportReviewDecisionClass::ApplyAfterPreview
        );
        assert!(review
            .discovered_items
            .iter()
            .any(|item| item.source_relative_path == ".vscode/settings.json"));
        assert!(review.status_line.contains("VS Code"));
        assert_eq!(
            serde_json::to_string(&review.classification).expect("classification serializes"),
            "\"vscode_workspace_root\""
        );
    }

    #[test]
    fn import_classifier_detects_jetbrains_fixture() {
        let classifier = CompetitorConfigClassifier::new();
        let root = fixture_root("jetbrains_workspace");
        let review = classifier.build_review(&root, "profile:default");

        assert_eq!(
            review.classification,
            CompetitorConfigClassification::JetBrainsIdeaRoot
        );
        assert_eq!(
            review.decision_class,
            ImportReviewDecisionClass::ApplyAfterPreview
        );
        assert!(review
            .discovered_items
            .iter()
            .any(|item| item.source_relative_path == ".idea/workspace.xml"));
        assert!(review.status_line.contains("JetBrains"));
    }

    #[test]
    fn import_classifier_reports_empty_folder_as_unknown() {
        let temp = tempfile::tempdir().expect("tempdir");
        let classifier = CompetitorConfigClassifier::new();
        let review = classifier.build_review(temp.path(), "profile:default");

        assert_eq!(
            review.classification,
            CompetitorConfigClassification::UnknownConfigRoot
        );
        assert_eq!(review.decision_class, ImportReviewDecisionClass::Defer);
        assert!(review.discovered_items.is_empty());
        assert!(review.status_line.contains("no .vscode or .idea"));
    }
}
