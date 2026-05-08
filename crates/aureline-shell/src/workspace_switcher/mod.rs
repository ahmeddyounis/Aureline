//! Workspace switcher projections.
//!
//! The workspace switcher is a keyboard-first launcher over open windows,
//! pinned entries, and recent-work entries. It does not invent its own target
//! vocabulary: rows project from the canonical recent-work registry so target
//! kind, trust state, restore availability, and unavailable-target posture
//! remain consistent across entry surfaces.

use aureline_workspace::{
    RecentWorkEntryRecord, RecentWorkRegistry, RecentWorkTargetState, RestoreAvailability,
    SafeRecoveryAction, TargetKind, TrustState,
};

/// Presentation label rendered for workspace switcher surfaces.
pub const WORKSPACE_SWITCHER_PRESENTATION_LABEL: &str = "Switch Project";

fn normalize_query(query: &str) -> String {
    query.trim().to_ascii_lowercase()
}

fn query_matches(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack.to_ascii_lowercase().contains(needle)
}

/// One projected workspace switcher row backed by a recent-work entry.
#[derive(Debug, Clone)]
pub struct WorkspaceSwitcherRow {
    pub recent_work_id: String,
    pub primary_label: String,
    pub location_or_target_subtitle: Option<String>,
    pub target_kind: TargetKind,
    pub target_state: RecentWorkTargetState,
    pub trust_state: TrustState,
    pub restore_availability: RestoreAvailability,
    pub last_opened_at: String,
    pub pinned: bool,
    pub safe_recovery_actions: Vec<SafeRecoveryAction>,
    pub searchable_terms: Vec<String>,
}

impl WorkspaceSwitcherRow {
    fn from_entry(entry: &RecentWorkEntryRecord) -> Self {
        let mut searchable_terms = Vec::new();
        searchable_terms.push(entry.presentation_label.to_ascii_lowercase());
        if let Some(subtitle) = entry.presentation_subtitle.as_deref() {
            searchable_terms.push(subtitle.to_ascii_lowercase());
        }
        searchable_terms.push(entry.target_kind.as_str().to_string());
        searchable_terms.push(entry.trust_state.as_str().to_string());
        searchable_terms.push(entry.target_state.as_str().to_string());

        Self {
            recent_work_id: entry.recent_work_id.clone(),
            primary_label: entry.presentation_label.clone(),
            location_or_target_subtitle: entry.presentation_subtitle.clone(),
            target_kind: entry.target_kind,
            target_state: entry.target_state,
            trust_state: entry.trust_state,
            restore_availability: entry.restore_availability,
            last_opened_at: entry.last_opened_at.clone(),
            pinned: entry.pinned,
            safe_recovery_actions: entry.safe_recovery_actions.clone(),
            searchable_terms,
        }
    }

    fn matches_query(&self, query: &str) -> bool {
        let query = normalize_query(query);
        if query.is_empty() {
            return true;
        }
        self.searchable_terms
            .iter()
            .any(|term| query_matches(term, &query))
    }
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
    let mut rows: Vec<_> = registry
        .entries
        .iter()
        .map(WorkspaceSwitcherRow::from_entry)
        .filter(|row| row.matches_query(query))
        .collect();

    rows.sort_by_key(|row| (!row.pinned, row.primary_label.to_ascii_lowercase()));
    rows
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
}
