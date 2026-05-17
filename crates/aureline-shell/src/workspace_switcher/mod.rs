//! Workspace switcher projections.
//!
//! The workspace switcher is a keyboard-first launcher over open windows,
//! pinned entries, and recent-work entries. It does not invent its own target
//! vocabulary: rows project from the canonical recent-work registry so target
//! kind, trust state, restore availability, and unavailable-target posture
//! remain consistent across entry surfaces.

use aureline_workspace::{
    is_remote_backed_target, normalized_recent_work_recovery_actions,
    project_searchable_recent_work_lists, RecentWorkEntryRecord, RecentWorkFailureState,
    RecentWorkListRow, RecentWorkRegistry, RecentWorkTargetState, RestoreAvailability,
    SafeRecoveryAction, TargetKind, TrustState,
};

use crate::restore::placeholders::{
    recent_work_placeholder_card, PlaceholderSurfaceClass, RecentWorkPlaceholderCard,
    WorkspaceSwitchRecoveryAction,
};

/// Presentation label rendered for workspace switcher surfaces.
pub const WORKSPACE_SWITCHER_PRESENTATION_LABEL: &str = "Switch Project";

/// One projected workspace switcher row backed by a recent-work entry.
#[derive(Debug, Clone)]
pub struct WorkspaceSwitcherRow {
    pub recent_work_id: String,
    pub primary_label: String,
    pub location_or_target_subtitle: Option<String>,
    pub entry_classes: Vec<WorkspaceSwitcherEntryClass>,
    pub target_kind: TargetKind,
    pub target_kind_label: &'static str,
    pub target_state: RecentWorkTargetState,
    pub failure_state: RecentWorkFailureState,
    pub trust_state: TrustState,
    pub restore_availability: RestoreAvailability,
    pub last_opened_at: String,
    pub pinned: bool,
    pub safe_recovery_actions: Vec<SafeRecoveryAction>,
    pub switch_failure_actions: Vec<WorkspaceSwitchRecoveryAction>,
    pub placeholder_card: Option<RecentWorkPlaceholderCard>,
    pub searchable_terms: Vec<String>,
}

/// Row classes shown and searched by the workspace switcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceSwitcherEntryClass {
    /// Local file, folder, repository, workspace, or workset.
    Local,
    /// Remote-backed target.
    Remote,
    /// Managed cloud workspace target.
    Managed,
    /// Pinned entry.
    Pinned,
    /// Recent entry.
    Recent,
    /// Entry with restorable state.
    RecentlyRestored,
    /// Template or prebuild snapshot.
    Template,
    /// Imported, handoff, or portable-state target.
    Imported,
}

impl WorkspaceSwitcherEntryClass {
    /// Returns the stable string vocabulary for the class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Managed => "managed",
            Self::Pinned => "pinned",
            Self::Recent => "recent",
            Self::RecentlyRestored => "recently_restored",
            Self::Template => "template",
            Self::Imported => "imported",
        }
    }
}

impl WorkspaceSwitcherRow {
    fn from_list_row(row: &RecentWorkListRow) -> Self {
        let entry = row.to_entry_record();
        let failure_state = row.failure_state;
        let entry_classes = entry_classes_for(&entry);
        let placeholder_card =
            recent_work_placeholder_card(&entry, PlaceholderSurfaceClass::WorkspaceSwitcher);
        let safe_recovery_actions = placeholder_card
            .as_ref()
            .map(|card| card.safe_recovery_actions.clone())
            .unwrap_or_else(|| normalized_recent_work_recovery_actions(&entry));
        let switch_failure_actions = placeholder_card
            .as_ref()
            .map(|card| card.switch_recovery_actions.clone())
            .unwrap_or_else(|| {
                vec![
                    WorkspaceSwitchRecoveryAction::CancelSwitch,
                    WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace,
                ]
            });
        let mut searchable_terms = row.searchable_terms.clone();
        for class in &entry_classes {
            searchable_terms.push(class.as_str().to_string());
        }

        Self {
            recent_work_id: entry.recent_work_id.clone(),
            primary_label: entry.presentation_label.clone(),
            location_or_target_subtitle: entry.presentation_subtitle.clone(),
            entry_classes,
            target_kind: entry.target_kind,
            target_kind_label: entry.target_kind.surface_label(),
            target_state: entry.target_state,
            failure_state,
            trust_state: entry.trust_state,
            restore_availability: entry.restore_availability,
            last_opened_at: entry.last_opened_at.clone(),
            pinned: entry.pinned,
            safe_recovery_actions,
            switch_failure_actions,
            placeholder_card,
            searchable_terms,
        }
    }
}

fn entry_classes_for(entry: &RecentWorkEntryRecord) -> Vec<WorkspaceSwitcherEntryClass> {
    let mut classes = Vec::new();
    if is_remote_backed_target(entry.target_kind) {
        classes.push(WorkspaceSwitcherEntryClass::Remote);
    } else {
        classes.push(WorkspaceSwitcherEntryClass::Local);
    }
    if entry.target_kind == TargetKind::ManagedCloudWorkspace {
        classes.push(WorkspaceSwitcherEntryClass::Managed);
    }
    if entry.target_kind == TargetKind::TemplateOrPrebuildSnapshot {
        classes.push(WorkspaceSwitcherEntryClass::Template);
    }
    if matches!(
        entry.target_kind,
        TargetKind::PortableStatePackage
            | TargetKind::HandoffPacket
            | TargetKind::CompetitorConfigRoot
    ) {
        classes.push(WorkspaceSwitcherEntryClass::Imported);
    }
    if entry.pinned {
        classes.push(WorkspaceSwitcherEntryClass::Pinned);
    }
    classes.push(WorkspaceSwitcherEntryClass::Recent);
    if entry.restore_availability != RestoreAvailability::None {
        classes.push(WorkspaceSwitcherEntryClass::RecentlyRestored);
    }
    classes
}

/// Mutable interaction state for a workspace switcher list.
#[derive(Debug, Clone)]
pub struct WorkspaceSwitcherState {
    query: String,
    selection: usize,
}

impl WorkspaceSwitcherState {
    /// Creates an empty switcher state.
    pub const fn new() -> Self {
        Self {
            query: String::new(),
            selection: 0,
        }
    }

    /// Returns the current query string.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the currently selected index.
    pub const fn selection(&self) -> usize {
        self.selection
    }

    /// Appends text input to the query, returning true when it changes.
    pub fn push_query_char(&mut self, ch: char) -> bool {
        if ch.is_control() {
            return false;
        }
        self.query.push(ch);
        self.selection = 0;
        true
    }

    /// Removes one character from the query, returning true when it changes.
    pub fn pop_query_char(&mut self) -> bool {
        let changed = self.query.pop().is_some();
        if changed {
            self.selection = 0;
        }
        changed
    }

    /// Advances selection by one within `row_count`, wrapping at the end.
    pub fn select_next(&mut self, row_count: usize) {
        if row_count == 0 {
            self.selection = 0;
            return;
        }
        self.selection = (self.selection + 1) % row_count;
    }

    /// Moves selection up by one within `row_count`, wrapping at the start.
    pub fn select_prev(&mut self, row_count: usize) {
        if row_count == 0 {
            self.selection = 0;
            return;
        }
        self.selection = self.selection.wrapping_add(row_count - 1) % row_count;
    }
}

/// Builds workspace switcher rows by filtering the registry with `query`.
pub fn build_switcher_rows(
    registry: &RecentWorkRegistry,
    query: &str,
) -> Vec<WorkspaceSwitcherRow> {
    project_searchable_recent_work_lists(registry, query)
        .rows()
        .iter()
        .map(WorkspaceSwitcherRow::from_list_row)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_query_terms() {
        let registry = RecentWorkRegistry {
            record_kind: aureline_workspace::RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
            recent_work_registry_schema_version: 1,
            updated_at: "mono:0".to_string(),
            entries: vec![RecentWorkEntryRecord {
                record_kind: aureline_workspace::RecentWorkEntryRecordKind::RecentWorkEntryRecord,
                entry_and_restore_schema_version: 1,
                recent_work_id: "recent:one".to_string(),
                presentation_label: "alpha".to_string(),
                presentation_subtitle: Some("/tmp/alpha".to_string()),
                target_kind: TargetKind::LocalFolder,
                target_state: RecentWorkTargetState::Reachable,
                portability_class: aureline_workspace::PortabilityClass::LocalOnly,
                trust_state: TrustState::Trusted,
                restore_availability: RestoreAvailability::None,
                safe_recovery_actions: vec![aureline_workspace::SafeRecoveryAction::Open],
                pinned: false,
                last_opened_at: "mono:0".to_string(),
                filesystem_identity_ref: None,
                remote_target_descriptor_ref: None,
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: None,
            }],
        };

        let rows = build_switcher_rows(&registry, "alp");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].primary_label, "alpha");

        let rows = build_switcher_rows(&registry, "missing");
        assert!(rows.is_empty());
    }

    #[test]
    fn placeholder_rows_expose_recovery_and_return_paths() {
        let registry = RecentWorkRegistry {
            record_kind: aureline_workspace::RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
            recent_work_registry_schema_version: 1,
            updated_at: "mono:0".to_string(),
            entries: vec![RecentWorkEntryRecord {
                record_kind: aureline_workspace::RecentWorkEntryRecordKind::RecentWorkEntryRecord,
                entry_and_restore_schema_version: 1,
                recent_work_id: "recent:missing".to_string(),
                presentation_label: "payments".to_string(),
                presentation_subtitle: Some("Local repository".to_string()),
                target_kind: TargetKind::LocalRepoRoot,
                target_state: RecentWorkTargetState::MissingTarget,
                portability_class: aureline_workspace::PortabilityClass::LocalOnly,
                trust_state: TrustState::Trusted,
                restore_availability: RestoreAvailability::LayoutOnly,
                safe_recovery_actions: vec![SafeRecoveryAction::LocateMissingTarget],
                pinned: true,
                last_opened_at: "mono:0".to_string(),
                filesystem_identity_ref: Some("fs:payments".to_string()),
                remote_target_descriptor_ref: None,
                artifact_descriptor_ref: None,
                recovery_checkpoint_refs: None,
            }],
        };

        let rows = build_switcher_rows(&registry, "missing_path");
        assert_eq!(rows.len(), 1);
        let row = &rows[0];
        assert_eq!(row.failure_state, RecentWorkFailureState::MissingPath);
        assert!(row
            .entry_classes
            .contains(&WorkspaceSwitcherEntryClass::Pinned));
        assert!(row
            .entry_classes
            .contains(&WorkspaceSwitcherEntryClass::Recent));
        assert_eq!(row.target_kind_label, "Repository");
        assert!(row.placeholder_card.is_some());
        assert!(row
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::OpenWithoutRestore));
        assert!(row
            .switch_failure_actions
            .contains(&WorkspaceSwitchRecoveryAction::CancelSwitch));
        assert!(row
            .switch_failure_actions
            .contains(&WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace));
    }
}
