//! Search query-session records shared by planner-backed surfaces.
//!
//! A query session captures the stable facts every quick-open, file-search,
//! symbol-search, and docs-search pass needs to replay or export: the surface,
//! query text retention mode, scope binding, planner version, readiness state,
//! and index epochs. The planner consumes this record directly so surfaces do
//! not mint private session vocabularies.

use serde::{Deserialize, Serialize};

use crate::lexical::scope::ScopeClass;
use crate::scope::WorkspaceSearchScopeMetadata;

/// Schema version for the alpha query-session record.
pub const SEARCH_QUERY_SESSION_SCHEMA_VERSION: u32 = 1;

/// Search surface family that owns a query session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSurface {
    /// Fast file, recent-place, and symbol jump surface.
    QuickOpen,
    /// Full workspace file and text-search surface.
    FileSearch,
    /// Symbol and structural-navigation search surface.
    SymbolSearch,
    /// Documentation and help search surface.
    DocsSearch,
}

impl SearchSurface {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuickOpen => "quick_open",
            Self::FileSearch => "file_search",
            Self::SymbolSearch => "symbol_search",
            Self::DocsSearch => "docs_search",
        }
    }
}

/// Retention mode for raw query text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryTextMode {
    /// Query text is retained locally in the session record.
    LocalText,
    /// Only a deterministic query hash is retained.
    HashOnly,
    /// Query text and hash are omitted by policy or trust posture.
    OmittedByPolicy,
}

impl QueryTextMode {
    /// Stable token used in records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalText => "local_text",
            Self::HashOnly => "hash_only",
            Self::OmittedByPolicy => "omitted_by_policy",
        }
    }
}

/// Exportable query-session record for planner-backed search surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchQuerySession {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable session identity used by planner passes and result snapshots.
    pub query_session_id: String,
    /// Surface family that opened the session.
    pub surface: SearchSurface,
    /// Query text retention mode for this session.
    pub query_text_mode: QueryTextMode,
    /// Raw query text when retention policy allows local storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// Deterministic query hash when text is retained or hash-only mode is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hash: Option<String>,
    /// Scope class active when the planner pass began.
    pub scope_class: ScopeClass,
    /// Stable scope identity active when the planner pass began.
    pub stable_scope_id: String,
    /// Sparse/full mode active when the planner pass began.
    pub scope_mode: String,
    /// Workset id when the scope was bound to a saved workset/slice artifact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id: Option<String>,
    /// Human-readable scope chip label captured for replay/export.
    pub scope_label: String,
    /// Planner version expected to answer this session.
    pub planner_version: String,
    /// Readiness token observed for the selected result set.
    pub readiness_state: String,
    /// Index epoch or shard epoch used by the selected result set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_epoch: Option<String>,
    /// Graph epoch used by graph-backed answers, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_epoch: Option<String>,
    /// Monotonic or fixture timestamp for export parity.
    pub observed_at: String,
}

impl SearchQuerySession {
    /// Stable record-kind tag carried in serialized query sessions.
    pub const RECORD_KIND: &'static str = "search_query_session";

    /// Builds a local-text query session with a deterministic query hash.
    // Keep this constructor field-shaped so serialized scope and planner evidence
    // stays explicit at call sites that mint query-session records.
    #[allow(clippy::too_many_arguments)]
    pub fn for_local_text(
        query_session_id: impl Into<String>,
        surface: SearchSurface,
        query_text: impl Into<String>,
        scope_class: ScopeClass,
        scope_label: impl Into<String>,
        planner_version: impl Into<String>,
        readiness_state: impl Into<String>,
        observed_at: impl Into<String>,
    ) -> Self {
        let query_text = query_text.into();
        let scope_label = scope_label.into();
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_SESSION_SCHEMA_VERSION,
            query_session_id: query_session_id.into(),
            surface,
            query_text_mode: QueryTextMode::LocalText,
            query_hash: Some(stable_query_hash(&query_text)),
            query_text: Some(query_text),
            scope_class,
            stable_scope_id: default_stable_scope_id(scope_class, &scope_label),
            scope_mode: default_scope_mode(scope_class).to_string(),
            workset_id: None,
            scope_label,
            planner_version: planner_version.into(),
            readiness_state: readiness_state.into(),
            index_epoch: None,
            graph_epoch: None,
            observed_at: observed_at.into(),
        }
    }

    /// Builds a hash-only query session for higher-trust export boundaries.
    // Keep this constructor field-shaped so callers cannot hide the retained
    // hash, scope, planner, readiness, and timestamp evidence behind defaults.
    #[allow(clippy::too_many_arguments)]
    pub fn for_hash_only(
        query_session_id: impl Into<String>,
        surface: SearchSurface,
        query_hash: impl Into<String>,
        scope_class: ScopeClass,
        scope_label: impl Into<String>,
        planner_version: impl Into<String>,
        readiness_state: impl Into<String>,
        observed_at: impl Into<String>,
    ) -> Self {
        let scope_label = scope_label.into();
        Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: SEARCH_QUERY_SESSION_SCHEMA_VERSION,
            query_session_id: query_session_id.into(),
            surface,
            query_text_mode: QueryTextMode::HashOnly,
            query_text: None,
            query_hash: Some(query_hash.into()),
            scope_class,
            stable_scope_id: default_stable_scope_id(scope_class, &scope_label),
            scope_mode: default_scope_mode(scope_class).to_string(),
            workset_id: None,
            scope_label,
            planner_version: planner_version.into(),
            readiness_state: readiness_state.into(),
            index_epoch: None,
            graph_epoch: None,
            observed_at: observed_at.into(),
        }
    }

    /// Attaches canonical workset/slice metadata projected by the search scope resolver.
    pub fn with_scope_metadata(mut self, metadata: &WorkspaceSearchScopeMetadata) -> Self {
        self.stable_scope_id = metadata.stable_scope_id.clone();
        self.scope_mode = metadata.scope_mode_token.clone();
        self.workset_id = metadata.workset_id.clone();
        self.scope_label = metadata.chip_label.clone();
        self
    }
}

/// Returns a deterministic, non-cryptographic query hash token.
pub fn stable_query_hash(query: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x00000100000001b3;

    let mut hash = FNV_OFFSET;
    for byte in query.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

fn default_stable_scope_id(scope_class: ScopeClass, scope_label: &str) -> String {
    format!(
        "scope:{}:{}",
        scope_class.as_str(),
        stable_query_hash(scope_label)
    )
}

fn default_scope_mode(scope_class: ScopeClass) -> &'static str {
    match scope_class {
        ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => "full",
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView => {
            "sparse"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_text_sessions_keep_hash_and_text() {
        let session = SearchQuerySession::for_local_text(
            "search:session:test",
            SearchSurface::QuickOpen,
            "main",
            ScopeClass::CurrentRepo,
            "Current repo",
            "search-planner-alpha",
            "hot_set_ready",
            "mono:1",
        );
        assert_eq!(session.record_kind, SearchQuerySession::RECORD_KIND);
        assert_eq!(session.query_text.as_deref(), Some("main"));
        assert_eq!(session.scope_mode, "full");
        assert!(session.workset_id.is_none());
        let expected_hash = stable_query_hash("main");
        assert_eq!(session.query_hash.as_deref(), Some(expected_hash.as_str()));
    }

    #[test]
    fn stable_query_hash_is_deterministic() {
        assert_eq!(
            stable_query_hash("workspace"),
            stable_query_hash("workspace")
        );
        assert_ne!(
            stable_query_hash("workspace"),
            stable_query_hash("Workspace")
        );
    }
}
