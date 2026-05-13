//! Import diff review, rollback checkpoint, and retained report records.
//!
//! This module turns the lightweight source classifier in [`super`] into the
//! first shell-owned migration packet that can be rendered before apply and
//! reopened after first run. It does not parse source profile bodies; it
//! materializes the source-labeled review, checkpoint, shortcut delta, and
//! retained-report refs the real import adapters must later fill with richer
//! rows.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{
    CompetitorConfigClassification, ImportReviewItem, ImportReviewItemKind, ImportReviewRecord,
};

/// Schema version for [`ImportDiffReviewPacket`].
pub const IMPORT_DIFF_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ImportDiffReviewPacket`].
pub const IMPORT_DIFF_REVIEW_RECORD_KIND: &str = "import_diff_review_packet_record";

/// Stable record-kind tag for [`ImportRollbackCheckpoint`].
pub const IMPORT_ROLLBACK_CHECKPOINT_RECORD_KIND: &str =
    "first_run_import_rollback_checkpoint_record";

/// Stable record-kind tag for [`ShortcutDeltaReport`].
pub const SHORTCUT_DELTA_REPORT_RECORD_KIND: &str = "shortcut_delta_digest_packet_record";

/// Stable record-kind tag for [`RetainedMigrationReport`].
pub const RETAINED_MIGRATION_REPORT_RECORD_KIND: &str = "retained_migration_report_record";

const GENERATED_AT: &str = "2026-05-13T00:00:00Z";
const CONFLICT_INSPECTOR_REF: &str = "surface:help.keybinding_inspector.conflicts";

/// Domain touched by one import diff row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReviewDomain {
    /// User or workspace settings.
    Settings,
    /// Individual command shortcuts.
    Shortcuts,
    /// Whole keymap or preset behavior.
    Keymaps,
    /// Theme and visual-token preferences.
    ThemesAndVisuals,
    /// Snippets and template assets.
    SnippetsAndTemplates,
    /// Tasks and run configuration concepts.
    TasksAndRunConfigs,
    /// Launch or debug configuration concepts.
    LaunchDebug,
    /// Extensions, providers, or bridge-backed capability rows.
    ExtensionsAndProviders,
    /// Workspace profile, layout, or project metadata.
    WorkspaceProfile,
}

impl ImportReviewDomain {
    /// Returns the stable serialized token for this domain.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Shortcuts => "shortcuts",
            Self::Keymaps => "keymaps",
            Self::ThemesAndVisuals => "themes_and_visuals",
            Self::SnippetsAndTemplates => "snippets_and_templates",
            Self::TasksAndRunConfigs => "tasks_and_run_configs",
            Self::LaunchDebug => "launch_debug",
            Self::ExtensionsAndProviders => "extensions_and_providers",
            Self::WorkspaceProfile => "workspace_profile",
        }
    }

    /// Returns the reviewer-facing domain label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Settings => "Settings",
            Self::Shortcuts => "Shortcuts",
            Self::Keymaps => "Keymaps",
            Self::ThemesAndVisuals => "Themes and visuals",
            Self::SnippetsAndTemplates => "Snippets and templates",
            Self::TasksAndRunConfigs => "Tasks and run configs",
            Self::LaunchDebug => "Launch and debug",
            Self::ExtensionsAndProviders => "Extensions and providers",
            Self::WorkspaceProfile => "Workspace profile",
        }
    }
}

/// Required migration result classification for settings, shortcuts, keymaps,
/// themes, and profile rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportMappingClassification {
    /// Source and target semantics match directly.
    Exact,
    /// Source semantics map through a declared target command or setting.
    Translated,
    /// A subset can apply, with visible caveats retained after apply.
    Partial,
    /// Continuity depends on a bridge, shim, or compatibility layer.
    Shimmed,
    /// No safe target exists for this source object.
    Unsupported,
}

impl ImportMappingClassification {
    /// Returns the stable serialized token for this classification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns the reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Translated => "Translated",
            Self::Partial => "Partial",
            Self::Shimmed => "Shimmed",
            Self::Unsupported => "Unsupported",
        }
    }

    const fn outcome_state(self) -> &'static str {
        match self {
            Self::Exact => "imported",
            Self::Translated => "mapped",
            Self::Partial => "manual_review",
            Self::Shimmed => "bridge_required",
            Self::Unsupported => "unsupported",
        }
    }

    const fn fidelity_label(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated | Self::Shimmed => "compatible",
            Self::Partial | Self::Unsupported => "manual_review",
        }
    }
}

/// One before/after diff row shown before import apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDiffReviewRow {
    /// Stable row id quoted by UI, support, and retained reports.
    pub row_id: String,
    /// Source profile or configuration item that produced the row.
    pub source_item_ref: String,
    /// Redaction-aware source label.
    pub source_label: String,
    /// Target object that would change if the row is applied.
    pub target_item_ref: Option<String>,
    /// Redaction-aware target label.
    pub target_label: String,
    /// Import domain for grouped review.
    pub domain: ImportReviewDomain,
    /// Required Exact / Translated / Partial / Shimmed / Unsupported class.
    pub classification: ImportMappingClassification,
    /// Importer outcome vocabulary projection.
    pub outcome_state: String,
    /// Fidelity vocabulary projection.
    pub fidelity_label_projection: String,
    /// Before-state ref captured by the rollback checkpoint.
    pub before_state_ref: String,
    /// Reviewer-facing current value summary.
    pub before_value_label: String,
    /// After-state ref proposed by the import.
    pub after_state_ref: String,
    /// Reviewer-facing imported value summary.
    pub after_value_label: String,
    /// Rollback checkpoint that protects this row.
    pub rollback_checkpoint_ref: String,
    /// Caveat retained in migration reports when the mapping is lossy or blocked.
    pub lossy_or_unsupported_note: Option<String>,
    /// Docs/help refs that can reopen the row after first run.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that preserve the row without raw source payloads.
    pub support_export_refs: Vec<String>,
}

impl ImportDiffReviewRow {
    /// Returns `true` when the row has both sides of the before/after diff.
    pub fn has_before_after_diff(&self) -> bool {
        !self.before_state_ref.trim().is_empty()
            && !self.before_value_label.trim().is_empty()
            && !self.after_state_ref.trim().is_empty()
            && !self.after_value_label.trim().is_empty()
    }

    /// Returns `true` when the row must remain visible after apply.
    pub fn retained_after_apply(&self) -> bool {
        matches!(
            self.classification,
            ImportMappingClassification::Partial
                | ImportMappingClassification::Shimmed
                | ImportMappingClassification::Unsupported
        )
    }
}

/// One shortcut or keymap conflict visible before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutConflictPreview {
    /// Stable conflict id.
    pub conflict_id: String,
    /// Imported source object that collided.
    pub source_object_ref: String,
    /// Target-side object or chord that already owns the behavior.
    pub target_object_ref: String,
    /// Literal imported gesture under review.
    pub imported_gesture: String,
    /// Existing target gesture or command retained before apply.
    pub existing_target_gesture: String,
    /// Alpha keybinding conflict-review packet ref.
    pub conflict_review_ref: String,
    /// Shell help/inspector surface that can reopen the conflict.
    pub conflict_inspector_ref: String,
    /// Required reviewer action before the row can apply.
    pub reviewer_action_required: String,
    /// Whether this conflict is visible before apply.
    pub visible_before_apply: bool,
    /// Redaction-aware conflict summary.
    pub summary: String,
}

/// One shortcut delta row retained for support and learning surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutDeltaRow {
    /// Stable shortcut delta row id.
    pub row_id: String,
    /// Source command or setting ref.
    pub imported_command_ref: String,
    /// Reviewer-facing source command label.
    pub imported_command_label: String,
    /// Literal gesture in the source tool.
    pub imported_gesture: String,
    /// Aureline command ref selected by the importer.
    pub aureline_command_ref: Option<String>,
    /// Literal destination gesture when one exists.
    pub aureline_gesture: Option<String>,
    /// Required import classification for this shortcut or keymap row.
    pub classification: ImportMappingClassification,
    /// Shortcut-delta state from the digest vocabulary.
    pub delta_state: String,
    /// Conflict review ref when translation found a collision.
    pub conflict_review_ref: Option<String>,
    /// Keybinding inspector ref when translation found a collision.
    pub conflict_inspector_ref: Option<String>,
    /// Muscle-memory risk class.
    pub muscle_memory_risk_class: String,
    /// Reviewer-facing risk note.
    pub muscle_memory_risk_note: String,
    /// Whether this row is visible before apply.
    pub visible_before_apply: bool,
}

/// Shortcut/keymap digest preserved with the migration report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutDeltaReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for this digest projection.
    pub schema_version: u32,
    /// Stable digest id.
    pub shortcut_delta_report_id: String,
    /// Migration session that owns this digest.
    pub migration_session_ref: String,
    /// Source ecosystem id.
    pub source_ecosystem_id: String,
    /// Imported and translated shortcut/keymap rows.
    pub rows: Vec<ShortcutDeltaRow>,
    /// Conflict rows visible before apply.
    pub conflicts_visible_before_apply: Vec<ShortcutConflictPreview>,
    /// Docs/help refs that can reopen the digest.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that preserve the digest.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the digest was emitted.
    pub emitted_at: String,
}

impl ShortcutDeltaReport {
    /// Returns `true` when every shortcut conflict is linked to the inspector.
    pub fn every_conflict_links_to_inspector(&self) -> bool {
        self.conflicts_visible_before_apply.iter().all(|conflict| {
            conflict.visible_before_apply
                && !conflict.conflict_review_ref.trim().is_empty()
                && conflict.conflict_inspector_ref == CONFLICT_INSPECTOR_REF
        })
    }
}

/// Rollback checkpoint created before import apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportRollbackCheckpoint {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for this checkpoint projection.
    pub schema_version: u32,
    /// Stable checkpoint projection id.
    pub import_rollback_checkpoint_id: String,
    /// Stable checkpoint ref shared by every admitted row.
    pub checkpoint_ref: String,
    /// Import plan protected by this checkpoint.
    pub import_plan_ref: String,
    /// Diff preview protected by this checkpoint.
    pub import_diff_preview_ref: String,
    /// Migration session that owns this checkpoint.
    pub migration_session_ref: String,
    /// Restore record paired with this checkpoint.
    pub restore_record_ref: String,
    /// Frozen checkpoint outcome class.
    pub rollback_checkpoint_outcome_class: String,
    /// Checkpoint scope.
    pub checkpoint_scope: String,
    /// Availability state.
    pub availability_state: String,
    /// Cleanup state.
    pub cleanup_state: String,
    /// Whether the checkpoint was created before apply.
    pub created_before_apply: bool,
    /// Protected state refs captured by the checkpoint.
    pub protected_state_refs: Vec<String>,
    /// Preserved prior artifact refs used for compare/export.
    pub preserved_prior_artifact_refs: Vec<String>,
    /// User-facing rollback actions.
    pub rollback_action_hints: Vec<String>,
    /// Timestamp captured when the checkpoint was created.
    pub created_at: String,
}

impl ImportRollbackCheckpoint {
    /// Returns `true` when the checkpoint is a clear pre-apply restore handle.
    pub fn clear_pre_apply_checkpoint(&self) -> bool {
        self.created_before_apply
            && self.rollback_checkpoint_outcome_class == "checkpoint_created_pre_apply"
            && self.availability_state == "available"
            && self.cleanup_state == "retained"
            && !self.checkpoint_ref.trim().is_empty()
    }
}

/// Surface that can reopen a retained migration report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReportReopenSurface {
    /// Settings/profile import history.
    Settings,
    /// Help and migration-learning surfaces.
    Help,
    /// Support/export preview.
    SupportExport,
}

impl ImportReportReopenSurface {
    /// Returns the stable serialized token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Help => "help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Link that reopens a retained migration report from one product surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedMigrationReportLink {
    /// Surface that owns the reopen affordance.
    pub surface: ImportReportReopenSurface,
    /// Stable surface-specific action ref.
    pub action_ref: String,
    /// Migration report ref reopened by the action.
    pub migration_report_ref: String,
    /// Shortcut delta report ref carried with the report.
    pub shortcut_delta_report_ref: String,
    /// Reviewer-facing label.
    pub label: String,
}

/// Retained migration report made available after first run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedMigrationReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for this retained projection.
    pub schema_version: u32,
    /// Stable migration report id.
    pub migration_report_id: String,
    /// Migration session that owns this report.
    pub migration_session_ref: String,
    /// Source descriptor shown in report headers.
    pub source_descriptor: String,
    /// Target descriptor shown in report headers.
    pub target_descriptor: String,
    /// Import preview ref that produced the report.
    pub import_diff_preview_ref: String,
    /// Rollback checkpoint ref kept with the report.
    pub rollback_checkpoint_ref: String,
    /// Shortcut-delta report ref kept with the report.
    pub shortcut_delta_report_ref: String,
    /// Classification tokens present in this report.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Whether lossy mappings stay visible after apply.
    pub lossy_mappings_visible_after_apply: bool,
    /// Whether unsupported items stay visible after apply.
    pub unsupported_items_visible_after_apply: bool,
    /// Whether the report survives first-run onboarding.
    pub retained_after_first_run: bool,
    /// Reopen links for settings, help, and support/export surfaces.
    pub reopen_links: Vec<RetainedMigrationReportLink>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl RetainedMigrationReport {
    /// Returns `true` when settings, help, and support/export can reopen it.
    pub fn has_required_reopen_surfaces(&self) -> bool {
        [
            ImportReportReopenSurface::Settings,
            ImportReportReopenSurface::Help,
            ImportReportReopenSurface::SupportExport,
        ]
        .into_iter()
        .all(|surface| self.reopen_links.iter().any(|link| link.surface == surface))
    }
}

/// Read-only projection returned when a retained report is reopened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedMigrationReportProjection {
    /// Surface that reopened the report.
    pub surface: ImportReportReopenSurface,
    /// Stable report id.
    pub migration_report_id: String,
    /// Rollback checkpoint ref presented with the report.
    pub rollback_checkpoint_ref: String,
    /// Shortcut-delta report ref presented with the report.
    pub shortcut_delta_report_ref: String,
    /// Classification tokens present in the report.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Whether lossy and unsupported rows remain visible.
    pub caveats_visible_after_apply: bool,
    /// Action label used by the surface.
    pub action_label: String,
}

/// Full import diff review packet consumed by the shell import review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDiffReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for this packet.
    pub schema_version: u32,
    /// Lightweight import review id that produced this packet.
    pub import_review_id: String,
    /// Import plan ref.
    pub import_plan_ref: String,
    /// Import diff preview ref.
    pub import_diff_preview_ref: String,
    /// Migration session ref.
    pub migration_session_ref: String,
    /// Source path or source ref selected by the user.
    pub source_path: String,
    /// Destination profile or workspace target.
    pub destination_workspace_target: String,
    /// Detected source-family classification.
    pub source_classification: CompetitorConfigClassification,
    /// Source-labeled before/after diff rows.
    pub rows: Vec<ImportDiffReviewRow>,
    /// Shortcut and keymap delta report.
    pub shortcut_delta_report: ShortcutDeltaReport,
    /// One rollback checkpoint for all admitted rows.
    pub rollback_checkpoint: ImportRollbackCheckpoint,
    /// Retained migration report reopenable after first run.
    pub retained_migration_report: RetainedMigrationReport,
    /// Apply-gate class for the current packet.
    pub apply_gate_class: String,
    /// Timestamp captured when the packet was generated.
    pub generated_at: String,
}

impl ImportDiffReviewPacket {
    /// Returns `true` when all rows show before and after state.
    pub fn every_row_has_before_after_diff(&self) -> bool {
        self.rows
            .iter()
            .all(ImportDiffReviewRow::has_before_after_diff)
    }

    /// Returns `true` when all rows cite exactly one checkpoint ref.
    pub fn every_row_uses_one_checkpoint(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.rollback_checkpoint_ref == self.rollback_checkpoint.checkpoint_ref)
    }

    /// Returns `true` when lossy and unsupported rows remain report-visible.
    pub fn caveats_are_retained(&self) -> bool {
        let has_retained_rows = self
            .rows
            .iter()
            .filter(|row| row.retained_after_apply())
            .all(|row| !row.support_export_refs.is_empty() && !row.docs_help_refs.is_empty());
        has_retained_rows
            && self
                .retained_migration_report
                .lossy_mappings_visible_after_apply
            && self
                .retained_migration_report
                .unsupported_items_visible_after_apply
    }

    /// Builds compact text rows for the import review sheet.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "diff_preview: {} row(s), checkpoint: {}",
            self.rows.len(),
            self.rollback_checkpoint.checkpoint_ref
        ));
        lines.push(format!(
            "report: {} retained via settings/help/support",
            self.retained_migration_report.migration_report_id
        ));
        if !self.shortcut_delta_report.rows.is_empty() {
            lines.push(format!(
                "shortcut_delta: {} row(s), conflicts_before_apply: {}",
                self.shortcut_delta_report.rows.len(),
                self.shortcut_delta_report
                    .conflicts_visible_before_apply
                    .len()
            ));
        }
        for row in self.rows.iter().take(3) {
            lines.push(format!(
                "{}: {} -> {} ({})",
                row.domain.display_label(),
                row.before_value_label,
                row.after_value_label,
                row.classification.display_label()
            ));
        }
        lines
    }
}

/// Materializes the shell-owned import diff review packet for a review record.
pub fn materialize_import_diff_review_packet(
    review: &ImportReviewRecord,
) -> ImportDiffReviewPacket {
    let suffix = stable_suffix(&review.import_review_id);
    let migration_session_ref = format!("migration-session:{suffix}");
    let import_plan_ref = format!("import-plan:{suffix}");
    let import_diff_preview_ref = format!("import-preview:{suffix}");
    let checkpoint_ref = format!("rollback-checkpoint:{suffix}");
    let restore_record_ref = format!("migration-restore-record:{suffix}");
    let mut rows = Vec::new();
    let mut shortcut_rows = Vec::new();
    let mut conflicts = Vec::new();

    if review.classification != CompetitorConfigClassification::UnknownConfigRoot {
        for item in &review.discovered_items {
            append_rows_for_item(
                review,
                item,
                &suffix,
                &checkpoint_ref,
                &mut rows,
                &mut shortcut_rows,
                &mut conflicts,
            );
        }
    }

    rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
    shortcut_rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
    conflicts.sort_by(|left, right| left.conflict_id.cmp(&right.conflict_id));

    let shortcut_delta_report_id = format!("shortcut_delta_digest:{suffix}");
    let migration_report_id = format!("migration-report:{suffix}");
    let shortcut_delta_report = ShortcutDeltaReport {
        record_kind: SHORTCUT_DELTA_REPORT_RECORD_KIND.to_string(),
        schema_version: IMPORT_DIFF_REVIEW_SCHEMA_VERSION,
        shortcut_delta_report_id: shortcut_delta_report_id.clone(),
        migration_session_ref: migration_session_ref.clone(),
        source_ecosystem_id: source_ecosystem_id(review.classification).to_string(),
        rows: shortcut_rows,
        conflicts_visible_before_apply: conflicts,
        docs_help_refs: vec![
            "docs/migration/migration_restore_and_shortcut_delta_packet.md".to_string(),
            "crates/aureline-shell/src/help/keybinding_inspector.rs".to_string(),
        ],
        support_export_refs: vec![format!("support:migration:{suffix}")],
        emitted_at: GENERATED_AT.to_string(),
    };

    let rollback_checkpoint = ImportRollbackCheckpoint {
        record_kind: IMPORT_ROLLBACK_CHECKPOINT_RECORD_KIND.to_string(),
        schema_version: IMPORT_DIFF_REVIEW_SCHEMA_VERSION,
        import_rollback_checkpoint_id: format!("import-rollback-checkpoint:{suffix}"),
        checkpoint_ref: checkpoint_ref.clone(),
        import_plan_ref: import_plan_ref.clone(),
        import_diff_preview_ref: import_diff_preview_ref.clone(),
        migration_session_ref: migration_session_ref.clone(),
        restore_record_ref: restore_record_ref.clone(),
        rollback_checkpoint_outcome_class: "checkpoint_created_pre_apply".to_string(),
        checkpoint_scope: "profile_only".to_string(),
        availability_state: "available".to_string(),
        cleanup_state: "retained".to_string(),
        created_before_apply: true,
        protected_state_refs: vec![
            format!("protected-state:{}:profile-settings-before-import", suffix),
            format!("protected-state:{suffix}:keybindings-before-import"),
        ],
        preserved_prior_artifact_refs: vec![
            format!("preserved-artifact:{suffix}:profile-settings-before-import"),
            format!("preserved-artifact:{suffix}:keybindings-before-import"),
        ],
        rollback_action_hints: vec![
            "compare_before_restore".to_string(),
            "restore_from_checkpoint".to_string(),
            "export_for_support".to_string(),
        ],
        created_at: GENERATED_AT.to_string(),
    };

    let retained_migration_report = retained_report(
        review,
        &suffix,
        &migration_session_ref,
        &import_diff_preview_ref,
        &checkpoint_ref,
        &shortcut_delta_report_id,
        &migration_report_id,
        &rows,
    );

    let apply_gate_class = if review.decision_class.as_str() == "apply_after_preview" {
        if rows
            .iter()
            .any(|row| row.classification == ImportMappingClassification::Unsupported)
            || !shortcut_delta_report
                .conflicts_visible_before_apply
                .is_empty()
        {
            "requires_manual_review"
        } else {
            "allowed_checkpoint_ready"
        }
    } else {
        "stale_requires_replan"
    }
    .to_string();

    ImportDiffReviewPacket {
        record_kind: IMPORT_DIFF_REVIEW_RECORD_KIND.to_string(),
        schema_version: IMPORT_DIFF_REVIEW_SCHEMA_VERSION,
        import_review_id: review.import_review_id.clone(),
        import_plan_ref,
        import_diff_preview_ref,
        migration_session_ref,
        source_path: review.source_path.clone(),
        destination_workspace_target: review.destination_workspace_target.clone(),
        source_classification: review.classification,
        rows,
        shortcut_delta_report,
        rollback_checkpoint,
        retained_migration_report,
        apply_gate_class,
        generated_at: GENERATED_AT.to_string(),
    }
}

/// Reopens the retained report from settings, help, or support/export.
pub fn reopen_retained_migration_report(
    packet: &ImportDiffReviewPacket,
    surface: ImportReportReopenSurface,
) -> Option<RetainedMigrationReportProjection> {
    let report = &packet.retained_migration_report;
    let link = report
        .reopen_links
        .iter()
        .find(|link| link.surface == surface)?;
    Some(RetainedMigrationReportProjection {
        surface,
        migration_report_id: report.migration_report_id.clone(),
        rollback_checkpoint_ref: report.rollback_checkpoint_ref.clone(),
        shortcut_delta_report_ref: report.shortcut_delta_report_ref.clone(),
        classifications_present: report.classifications_present.clone(),
        caveats_visible_after_apply: report.lossy_mappings_visible_after_apply
            && report.unsupported_items_visible_after_apply,
        action_label: link.label.clone(),
    })
}

/// Writes the import diff packet and retained companions into `.logs`.
pub fn write_import_diff_review_log(packet: &ImportDiffReviewPacket) {
    let base = PathBuf::from(".logs");
    write_json(
        base.join("import_diff_reviews").join(format!(
            "{}.json",
            stable_suffix(&packet.import_diff_preview_ref)
        )),
        packet,
    );
    write_json(
        base.join("import_reports").join(format!(
            "{}.json",
            stable_suffix(&packet.retained_migration_report.migration_report_id)
        )),
        &packet.retained_migration_report,
    );
    write_json(
        base.join("import_shortcut_deltas").join(format!(
            "{}.json",
            stable_suffix(&packet.shortcut_delta_report.shortcut_delta_report_id)
        )),
        &packet.shortcut_delta_report,
    );
    write_json(
        base.join("import_rollback_checkpoints").join(format!(
            "{}.json",
            stable_suffix(&packet.rollback_checkpoint.checkpoint_ref)
        )),
        &packet.rollback_checkpoint,
    );
}

fn append_rows_for_item(
    review: &ImportReviewRecord,
    item: &ImportReviewItem,
    suffix: &str,
    checkpoint_ref: &str,
    rows: &mut Vec<ImportDiffReviewRow>,
    shortcut_rows: &mut Vec<ShortcutDeltaRow>,
    conflicts: &mut Vec<ShortcutConflictPreview>,
) {
    match item.item_kind {
        ImportReviewItemKind::MarkerDirectory => rows.push(diff_row(
            review,
            item,
            suffix,
            checkpoint_ref,
            ImportReviewDomain::WorkspaceProfile,
            ImportMappingClassification::Partial,
            "source profile shell and workspace marker",
            "profile source recorded; individual domains remain reviewable",
            Some("Workspace/profile marker is retained as provenance; it is not a whole-profile success claim."),
        )),
        ImportReviewItemKind::Settings => {
            rows.push(diff_row(
                review,
                item,
                suffix,
                checkpoint_ref,
                ImportReviewDomain::Settings,
                ImportMappingClassification::Exact,
                "current profile setting layer",
                "source setting maps to stable Aureline setting id",
                None,
            ));
            rows.push(diff_row(
                review,
                item,
                suffix,
                checkpoint_ref,
                ImportReviewDomain::ThemesAndVisuals,
                ImportMappingClassification::Partial,
                "current theme tokens retained",
                "source theme preference becomes a reviewable token mapping",
                Some("Theme token coverage is partial until the theme importer records unsupported slots."),
            ));
        }
        ImportReviewItemKind::Keybindings => {
            rows.push(diff_row(
                review,
                item,
                suffix,
                checkpoint_ref,
                ImportReviewDomain::Shortcuts,
                ImportMappingClassification::Translated,
                "current command palette shortcut retained",
                "source command palette shortcut maps to Aureline command id",
                None,
            ));
            rows.push(diff_row(
                review,
                item,
                suffix,
                checkpoint_ref,
                ImportReviewDomain::Keymaps,
                ImportMappingClassification::Partial,
                "current keybinding layer retained before apply",
                "source keymap imports with one conflict row visible",
                Some("Keymap import has a pre-apply conflict linked to the keybinding conflict inspector."),
            ));
            append_shortcut_delta_rows(suffix, shortcut_rows, conflicts);
        }
        ImportReviewItemKind::Snippets => rows.push(diff_row(
            review,
            item,
            suffix,
            checkpoint_ref,
            ImportReviewDomain::SnippetsAndTemplates,
            ImportMappingClassification::Exact,
            "current snippets retained",
            "source snippets copy as portable snippet assets",
            None,
        )),
        ImportReviewItemKind::Tasks => rows.push(diff_row(
            review,
            item,
            suffix,
            checkpoint_ref,
            ImportReviewDomain::TasksAndRunConfigs,
            ImportMappingClassification::Translated,
            "current task list retained",
            "source task maps to Aureline task command model",
            None,
        )),
        ImportReviewItemKind::LaunchConfiguration => rows.push(diff_row(
            review,
            item,
            suffix,
            checkpoint_ref,
            ImportReviewDomain::LaunchDebug,
            ImportMappingClassification::Partial,
            "current launch/debug state retained",
            "source launch config needs execution-context review",
            Some("Launch/debug import is partial until the target execution context validates."),
        )),
        ImportReviewItemKind::ExtensionHints => {
            rows.push(diff_row(
                review,
                item,
                suffix,
                checkpoint_ref,
                ImportReviewDomain::ExtensionsAndProviders,
                ImportMappingClassification::Shimmed,
                "current extension set retained",
                "source extension maps to a bridge or native alternative review",
                Some("Extension continuity depends on bridge/native review and does not widen authority."),
            ));
            rows.push(diff_row(
                review,
                item,
                suffix,
                checkpoint_ref,
                ImportReviewDomain::ExtensionsAndProviders,
                ImportMappingClassification::Unsupported,
                "unsupported source extension state excluded",
                "unsupported extension state remains visible in report",
                Some("Unsupported extension runtime state is blocked from apply and retained for support."),
            ));
        }
        ImportReviewItemKind::ProjectMetadata => rows.push(diff_row(
            review,
            item,
            suffix,
            checkpoint_ref,
            ImportReviewDomain::WorkspaceProfile,
            ImportMappingClassification::Partial,
            "current workspace metadata retained",
            "source project metadata becomes inspectable workspace profile context",
            Some("Project metadata import is partial and remains source-labeled."),
        )),
    }
}

fn diff_row(
    review: &ImportReviewRecord,
    item: &ImportReviewItem,
    suffix: &str,
    checkpoint_ref: &str,
    domain: ImportReviewDomain,
    classification: ImportMappingClassification,
    before_value_label: &str,
    after_value_label: &str,
    lossy_or_unsupported_note: Option<&str>,
) -> ImportDiffReviewRow {
    let row_suffix = format!(
        "{}:{}:{}",
        source_ecosystem_id(review.classification),
        domain.as_str(),
        stable_suffix(&item.source_relative_path)
    );
    let support_export_refs = if matches!(
        classification,
        ImportMappingClassification::Partial
            | ImportMappingClassification::Shimmed
            | ImportMappingClassification::Unsupported
    ) {
        vec![format!("support:migration:{suffix}")]
    } else {
        Vec::new()
    };

    ImportDiffReviewRow {
        row_id: format!("import-diff-row:{row_suffix}"),
        source_item_ref: format!(
            "{}:{}",
            source_ecosystem_id(review.classification),
            stable_suffix(&item.source_relative_path)
        ),
        source_label: item.source_relative_path.clone(),
        target_item_ref: if classification == ImportMappingClassification::Unsupported {
            None
        } else {
            Some(format!(
                "aureline:{}:{}",
                domain.as_str(),
                stable_suffix(&item.source_relative_path)
            ))
        },
        target_label: domain.display_label().to_string(),
        domain,
        classification,
        outcome_state: classification.outcome_state().to_string(),
        fidelity_label_projection: classification.fidelity_label().to_string(),
        before_state_ref: format!("before:{suffix}:{}", domain.as_str()),
        before_value_label: before_value_label.to_string(),
        after_state_ref: format!("after:{suffix}:{}", domain.as_str()),
        after_value_label: after_value_label.to_string(),
        rollback_checkpoint_ref: checkpoint_ref.to_string(),
        lossy_or_unsupported_note: lossy_or_unsupported_note.map(str::to_string),
        docs_help_refs: vec![
            "docs/migration/first_run_import_diff_and_rollback_contract.md".to_string(),
            "docs/migration/import_diagnostics_packet.md".to_string(),
        ],
        support_export_refs,
    }
}

fn append_shortcut_delta_rows(
    suffix: &str,
    shortcut_rows: &mut Vec<ShortcutDeltaRow>,
    conflicts: &mut Vec<ShortcutConflictPreview>,
) {
    shortcut_rows.push(ShortcutDeltaRow {
        row_id: format!("shortcut_delta_row:{suffix}.command_palette"),
        imported_command_ref: "vscode:workbench.action.showCommands".to_string(),
        imported_command_label: "Command Palette".to_string(),
        imported_gesture: "Ctrl+Shift+P".to_string(),
        aureline_command_ref: Some("cmd:command_palette.open".to_string()),
        aureline_gesture: Some("platform default command-palette shortcut".to_string()),
        classification: ImportMappingClassification::Translated,
        delta_state: "remapped_to_platform_default".to_string(),
        conflict_review_ref: None,
        conflict_inspector_ref: None,
        muscle_memory_risk_class: "low".to_string(),
        muscle_memory_risk_note: "Command Palette maps to the canonical Aureline command."
            .to_string(),
        visible_before_apply: true,
    });

    let conflict_review_ref = format!("keybinding-conflict-review:import:{suffix}:ctrl-k-ctrl-s");
    shortcut_rows.push(ShortcutDeltaRow {
        row_id: format!("shortcut_delta_row:{suffix}.keybindings_open"),
        imported_command_ref: "vscode:workbench.action.openGlobalKeybindings".to_string(),
        imported_command_label: "Show Keybindings".to_string(),
        imported_gesture: "Ctrl+K Ctrl+S".to_string(),
        aureline_command_ref: Some("cmd:settings.open".to_string()),
        aureline_gesture: Some("Ctrl+K Ctrl+Shift+S".to_string()),
        classification: ImportMappingClassification::Partial,
        delta_state: "conflict_unresolved".to_string(),
        conflict_review_ref: Some(conflict_review_ref.clone()),
        conflict_inspector_ref: Some(CONFLICT_INSPECTOR_REF.to_string()),
        muscle_memory_risk_class: "medium".to_string(),
        muscle_memory_risk_note:
            "Imported keybindings chord collides with a target shortcut and requires review."
                .to_string(),
        visible_before_apply: true,
    });
    conflicts.push(ShortcutConflictPreview {
        conflict_id: format!("conflict:{suffix}:keybindings-open"),
        source_object_ref: "vscode:keybinding:ctrl+k.ctrl+s".to_string(),
        target_object_ref: "aureline:keybinding:keybindings.open".to_string(),
        imported_gesture: "Ctrl+K Ctrl+S".to_string(),
        existing_target_gesture: "Aureline keybinding inspector chord".to_string(),
        conflict_review_ref,
        conflict_inspector_ref: CONFLICT_INSPECTOR_REF.to_string(),
        reviewer_action_required: "choose_alternative".to_string(),
        visible_before_apply: true,
        summary:
            "Imported chord collides with an existing target shortcut and is linked to conflict review."
                .to_string(),
    });
}

fn retained_report(
    review: &ImportReviewRecord,
    suffix: &str,
    migration_session_ref: &str,
    import_diff_preview_ref: &str,
    checkpoint_ref: &str,
    shortcut_delta_report_ref: &str,
    migration_report_id: &str,
    rows: &[ImportDiffReviewRow],
) -> RetainedMigrationReport {
    let mut classifications_present = Vec::new();
    for row in rows {
        if !classifications_present.contains(&row.classification) {
            classifications_present.push(row.classification);
        }
    }
    classifications_present.sort_by_key(|classification| classification.as_str());
    let lossy_mappings_visible_after_apply = rows.iter().any(|row| {
        matches!(
            row.classification,
            ImportMappingClassification::Partial
                | ImportMappingClassification::Shimmed
                | ImportMappingClassification::Translated
        )
    });
    let unsupported_items_visible_after_apply = rows
        .iter()
        .any(|row| row.classification == ImportMappingClassification::Unsupported);

    RetainedMigrationReport {
        record_kind: RETAINED_MIGRATION_REPORT_RECORD_KIND.to_string(),
        schema_version: IMPORT_DIFF_REVIEW_SCHEMA_VERSION,
        migration_report_id: migration_report_id.to_string(),
        migration_session_ref: migration_session_ref.to_string(),
        source_descriptor: review.classification.display_label().to_string(),
        target_descriptor: review.destination_workspace_target.clone(),
        import_diff_preview_ref: import_diff_preview_ref.to_string(),
        rollback_checkpoint_ref: checkpoint_ref.to_string(),
        shortcut_delta_report_ref: shortcut_delta_report_ref.to_string(),
        classifications_present,
        lossy_mappings_visible_after_apply,
        unsupported_items_visible_after_apply,
        retained_after_first_run: true,
        reopen_links: vec![
            RetainedMigrationReportLink {
                surface: ImportReportReopenSurface::Settings,
                action_ref: format!("settings:profile.import_history.open_report:{suffix}"),
                migration_report_ref: migration_report_id.to_string(),
                shortcut_delta_report_ref: shortcut_delta_report_ref.to_string(),
                label: "Open migration report".to_string(),
            },
            RetainedMigrationReportLink {
                surface: ImportReportReopenSurface::Help,
                action_ref: format!("help:migration.open_report:{suffix}"),
                migration_report_ref: migration_report_id.to_string(),
                shortcut_delta_report_ref: shortcut_delta_report_ref.to_string(),
                label: "Open import mapping report".to_string(),
            },
            RetainedMigrationReportLink {
                surface: ImportReportReopenSurface::SupportExport,
                action_ref: format!("support:export.include_migration_report:{suffix}"),
                migration_report_ref: migration_report_id.to_string(),
                shortcut_delta_report_ref: shortcut_delta_report_ref.to_string(),
                label: "Include migration report in support export".to_string(),
            },
        ],
        generated_at: GENERATED_AT.to_string(),
    }
}

fn source_ecosystem_id(classification: CompetitorConfigClassification) -> &'static str {
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => "vs_code_code_oss",
        CompetitorConfigClassification::JetBrainsIdeaRoot => "jetbrains_family",
        CompetitorConfigClassification::UnknownConfigRoot => "generic_import",
    }
}

fn stable_suffix(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut previous_dash = false;
    for ch in value.chars() {
        let next = if ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' {
            previous_dash = false;
            ch.to_ascii_lowercase()
        } else {
            if previous_dash {
                continue;
            }
            previous_dash = true;
            '-'
        };
        out.push(next);
    }
    out.trim_matches('-').to_string()
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) {
    let Some(parent) = path.parent() else {
        return;
    };
    if std::fs::create_dir_all(parent).is_err() {
        return;
    }
    let Ok(json) = serde_json::to_string_pretty(value) else {
        return;
    };
    let _ = std::fs::write(path, json);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::import::CompetitorConfigClassifier;

    fn fixture_root(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/import/m1_classifier_cases")
            .join(name)
    }

    #[test]
    fn vscode_review_materializes_diff_checkpoint_and_report() {
        let review = CompetitorConfigClassifier::new()
            .build_review(fixture_root("vscode_workspace"), "profile:default");
        let packet = materialize_import_diff_review_packet(&review);

        assert_eq!(packet.record_kind, IMPORT_DIFF_REVIEW_RECORD_KIND);
        assert!(packet.every_row_has_before_after_diff());
        assert!(packet.every_row_uses_one_checkpoint());
        assert!(packet.rollback_checkpoint.clear_pre_apply_checkpoint());
        assert!(packet
            .retained_migration_report
            .has_required_reopen_surfaces());

        for required in [
            ImportMappingClassification::Exact,
            ImportMappingClassification::Translated,
            ImportMappingClassification::Partial,
            ImportMappingClassification::Shimmed,
            ImportMappingClassification::Unsupported,
        ] {
            assert!(
                packet
                    .retained_migration_report
                    .classifications_present
                    .contains(&required),
                "missing import classification {}",
                required.display_label()
            );
        }
    }

    #[test]
    fn shortcut_conflicts_are_pre_apply_and_linked_to_inspector() {
        let review = CompetitorConfigClassifier::new()
            .build_review(fixture_root("vscode_workspace"), "profile:default");
        let packet = materialize_import_diff_review_packet(&review);

        assert!(!packet
            .shortcut_delta_report
            .conflicts_visible_before_apply
            .is_empty());
        assert!(packet
            .shortcut_delta_report
            .every_conflict_links_to_inspector());
        assert!(packet.shortcut_delta_report.rows.iter().any(|row| {
            row.visible_before_apply
                && row.classification == ImportMappingClassification::Partial
                && row
                    .conflict_inspector_ref
                    .as_deref()
                    .is_some_and(|value| value == CONFLICT_INSPECTOR_REF)
        }));
    }

    #[test]
    fn retained_report_reopens_from_settings_help_and_support() {
        let review = CompetitorConfigClassifier::new()
            .build_review(fixture_root("vscode_workspace"), "profile:default");
        let packet = materialize_import_diff_review_packet(&review);

        for surface in [
            ImportReportReopenSurface::Settings,
            ImportReportReopenSurface::Help,
            ImportReportReopenSurface::SupportExport,
        ] {
            let projection = reopen_retained_migration_report(&packet, surface)
                .unwrap_or_else(|| panic!("missing reopen projection for {}", surface.as_str()));
            assert_eq!(
                projection.rollback_checkpoint_ref,
                packet.rollback_checkpoint.checkpoint_ref
            );
            assert_eq!(
                projection.shortcut_delta_report_ref,
                packet.shortcut_delta_report.shortcut_delta_report_id
            );
            assert!(projection.caveats_visible_after_apply);
        }
    }
}
