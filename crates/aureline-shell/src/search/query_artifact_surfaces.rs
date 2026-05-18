//! Shell projections for durable search artifact review surfaces.
//!
//! These records are intentionally thin wrappers over `aureline-search`
//! artifacts. They give saved-query rows, query-history rows, deep-link open
//! sheets, and search-export review surfaces the same source, privacy,
//! retention, scope-honesty, and live-versus-captured labels without minting
//! shell-local search truth.

use aureline_search::{
    QueryHistoryEntry, SavedQuery, ScopePackBinding, SearchCollectionSnapshot, SearchDeepLink,
};
use serde::{Deserialize, Serialize};

/// Schema version for search artifact shell projections.
pub const SEARCH_ARTIFACT_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`SavedQueryRowProjection`].
pub const SAVED_QUERY_ROW_PROJECTION_RECORD_KIND: &str = "saved_query_row_projection";

/// Stable record-kind tag for [`QueryHistoryRowProjection`].
pub const QUERY_HISTORY_ROW_PROJECTION_RECORD_KIND: &str = "query_history_row_projection";

/// Stable record-kind tag for [`SearchDeepLinkOpenSheetProjection`].
pub const SEARCH_DEEP_LINK_OPEN_SHEET_PROJECTION_RECORD_KIND: &str =
    "search_deep_link_open_sheet_projection";

/// Stable record-kind tag for [`SearchExportReviewProjection`].
pub const SEARCH_EXPORT_REVIEW_PROJECTION_RECORD_KIND: &str = "search_export_review_projection";

/// Shared badge vocabulary shown on durable search artifact surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchArtifactSurfaceBadges {
    /// Source class token.
    pub source_class: String,
    /// Privacy class token.
    pub privacy_class: String,
    /// Retention mode token.
    pub retention_mode: String,
    /// Sync class token.
    pub sync_class: String,
    /// Redaction profile token.
    pub redaction_profile: String,
    /// Retention widening basis token.
    pub retention_widening_basis: String,
    /// Scope-honesty state token.
    pub scope_honesty_state: String,
    /// Live-versus-captured result semantics token.
    pub result_semantics: String,
}

impl SearchArtifactSurfaceBadges {
    fn from_saved_query(saved_query: &SavedQuery) -> Self {
        Self {
            source_class: saved_query.source_class.as_str().to_string(),
            privacy_class: saved_query.privacy_class.as_str().to_string(),
            retention_mode: saved_query.retention_mode.as_str().to_string(),
            sync_class: saved_query.sync_class.as_str().to_string(),
            redaction_profile: saved_query.redaction_profile.as_str().to_string(),
            retention_widening_basis: saved_query.retention_widening_basis.as_str().to_string(),
            scope_honesty_state: saved_query.scope_honesty_state.as_str().to_string(),
            result_semantics: saved_query.result_semantics.as_str().to_string(),
        }
    }

    fn from_history_entry(history_entry: &QueryHistoryEntry) -> Self {
        Self {
            source_class: history_entry.source_class.as_str().to_string(),
            privacy_class: history_entry.privacy_class.as_str().to_string(),
            retention_mode: history_entry.retention_mode.as_str().to_string(),
            sync_class: history_entry.sync_class.as_str().to_string(),
            redaction_profile: history_entry.redaction_profile.as_str().to_string(),
            retention_widening_basis: history_entry.retention_widening_basis.as_str().to_string(),
            scope_honesty_state: history_entry.scope_honesty_state.as_str().to_string(),
            result_semantics: history_entry.result_semantics.as_str().to_string(),
        }
    }

    fn from_deep_link(deep_link: &SearchDeepLink) -> Self {
        Self {
            source_class: deep_link.source_class.as_str().to_string(),
            privacy_class: deep_link.privacy_class.as_str().to_string(),
            retention_mode: deep_link.retention_mode.as_str().to_string(),
            sync_class: deep_link.sync_class.as_str().to_string(),
            redaction_profile: deep_link.redaction_profile.as_str().to_string(),
            retention_widening_basis: deep_link.retention_widening_basis.as_str().to_string(),
            scope_honesty_state: deep_link.scope_honesty_state.as_str().to_string(),
            result_semantics: deep_link.result_semantics.as_str().to_string(),
        }
    }

    fn from_snapshot(snapshot: &SearchCollectionSnapshot) -> Self {
        Self {
            source_class: snapshot.source_class.as_str().to_string(),
            privacy_class: snapshot.privacy_class.as_str().to_string(),
            retention_mode: snapshot.retention_mode.as_str().to_string(),
            sync_class: snapshot.sync_class.as_str().to_string(),
            redaction_profile: snapshot.redaction_profile.as_str().to_string(),
            retention_widening_basis: snapshot.retention_widening_basis.as_str().to_string(),
            scope_honesty_state: snapshot.scope_honesty_state.as_str().to_string(),
            result_semantics: snapshot.result_semantics.as_str().to_string(),
        }
    }
}

/// Row projection for the saved-query list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryRowProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Saved query shown by this row.
    pub saved_query_id_ref: String,
    /// Redaction-safe display label.
    pub display_name: String,
    /// Default surface token for rerun.
    pub default_surface: String,
    /// Scope binding shown by the row.
    pub scope_binding_id_ref: String,
    /// Captured scope label.
    pub scope_label: String,
    /// Query-text mode token.
    pub query_text_mode: String,
    /// True when the local row has raw query text.
    pub raw_query_text_retained_locally: bool,
    /// Migration state token.
    pub migration_state: String,
    /// Shared badges shown by the row.
    pub badges: SearchArtifactSurfaceBadges,
}

impl SavedQueryRowProjection {
    /// Projects a saved query into a shell row.
    pub fn from_saved_query(saved_query: &SavedQuery) -> Self {
        Self {
            record_kind: SAVED_QUERY_ROW_PROJECTION_RECORD_KIND.to_string(),
            schema_version: SEARCH_ARTIFACT_SURFACE_SCHEMA_VERSION,
            saved_query_id_ref: saved_query.saved_query_id.clone(),
            display_name: saved_query.display_name.clone(),
            default_surface: saved_query.default_surface.as_str().to_string(),
            scope_binding_id_ref: saved_query.scope_binding_id_ref.clone(),
            scope_label: saved_query.scope_label.clone(),
            query_text_mode: saved_query.query_text_mode.as_str().to_string(),
            raw_query_text_retained_locally: saved_query.contains_raw_query_text(),
            migration_state: saved_query.migration_state.as_str().to_string(),
            badges: SearchArtifactSurfaceBadges::from_saved_query(saved_query),
        }
    }
}

/// Row projection for local or governed query history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryHistoryRowProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Query-history row shown by this projection.
    pub history_id_ref: String,
    /// Query-session ref shown by the row.
    pub query_session_id_ref: String,
    /// Saved-query ref, when any.
    pub saved_query_id_ref: Option<String>,
    /// Search surface token.
    pub surface: String,
    /// Stored text mode token.
    pub stored_text_mode: String,
    /// Timestamp shown by the row.
    pub last_used_at: String,
    /// Expiry timestamp shown by the row, when any.
    pub expires_at: Option<String>,
    /// Shared badges shown by the row.
    pub badges: SearchArtifactSurfaceBadges,
}

impl QueryHistoryRowProjection {
    /// Projects a query-history entry into a shell row.
    pub fn from_history_entry(history_entry: &QueryHistoryEntry) -> Self {
        Self {
            record_kind: QUERY_HISTORY_ROW_PROJECTION_RECORD_KIND.to_string(),
            schema_version: SEARCH_ARTIFACT_SURFACE_SCHEMA_VERSION,
            history_id_ref: history_entry.history_id.clone(),
            query_session_id_ref: history_entry.query_session_id_ref.clone(),
            saved_query_id_ref: history_entry.saved_query_id_ref.clone(),
            surface: history_entry.surface.as_str().to_string(),
            stored_text_mode: history_entry.stored_text_mode.as_str().to_string(),
            last_used_at: history_entry.last_used_at.clone(),
            expires_at: history_entry.expires_at.clone(),
            badges: SearchArtifactSurfaceBadges::from_history_entry(history_entry),
        }
    }
}

/// Open-sheet projection for search deep links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchDeepLinkOpenSheetProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Deep link being opened.
    pub deep_link_id_ref: String,
    /// Target surface token.
    pub target_surface: String,
    /// Query-session ref carried by the link.
    pub query_session_id_ref: String,
    /// Scope binding ref carried by the link.
    pub scope_binding_id_ref: String,
    /// True when current results require rerun.
    pub rerun_required: bool,
    /// True when recipient permissions are re-resolved.
    pub recipient_re_resolves_under_current_permissions: bool,
    /// True only for unsafe access-widening links.
    pub access_widening_allowed: bool,
    /// Shared badges shown by the open sheet.
    pub badges: SearchArtifactSurfaceBadges,
}

impl SearchDeepLinkOpenSheetProjection {
    /// Projects a search deep link into an open sheet.
    pub fn from_deep_link(deep_link: &SearchDeepLink) -> Self {
        Self {
            record_kind: SEARCH_DEEP_LINK_OPEN_SHEET_PROJECTION_RECORD_KIND.to_string(),
            schema_version: SEARCH_ARTIFACT_SURFACE_SCHEMA_VERSION,
            deep_link_id_ref: deep_link.deep_link_id.clone(),
            target_surface: deep_link.target_surface.as_str().to_string(),
            query_session_id_ref: deep_link.query_session_id_ref.clone(),
            scope_binding_id_ref: deep_link.scope_binding_id_ref.clone(),
            rerun_required: deep_link.rerun_required,
            recipient_re_resolves_under_current_permissions: deep_link
                .recipient_re_resolves_under_current_permissions,
            access_widening_allowed: deep_link.access_widening_allowed,
            badges: SearchArtifactSurfaceBadges::from_deep_link(deep_link),
        }
    }
}

/// Review projection for search export snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchExportReviewProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Snapshot being reviewed.
    pub snapshot_id_ref: String,
    /// Destination token.
    pub destination: String,
    /// Query-session ref quoted by the snapshot.
    pub query_session_id_ref: String,
    /// Scope binding ref quoted by the snapshot.
    pub scope_binding_id_ref: String,
    /// Selected result count.
    pub selected_result_count: u64,
    /// Included result count.
    pub included_result_count: u64,
    /// Omitted result count.
    pub omitted_result_count: u64,
    /// Rows hidden by current scope.
    pub hidden_by_current_scope_rows: u64,
    /// Rows hidden by policy.
    pub hidden_by_policy_rows: u64,
    /// Partiality or omission reasons shown by review.
    pub partiality_reasons: Vec<String>,
    /// True when literal query text is included.
    pub literal_query_text_included: bool,
    /// True when the default export avoids raw query text.
    pub raw_query_free_by_default: bool,
    /// Shared badges shown by export review.
    pub badges: SearchArtifactSurfaceBadges,
}

impl SearchExportReviewProjection {
    /// Projects a captured search collection snapshot into export review.
    pub fn from_snapshot(snapshot: &SearchCollectionSnapshot) -> Self {
        Self {
            record_kind: SEARCH_EXPORT_REVIEW_PROJECTION_RECORD_KIND.to_string(),
            schema_version: SEARCH_ARTIFACT_SURFACE_SCHEMA_VERSION,
            snapshot_id_ref: snapshot.snapshot_id.clone(),
            destination: snapshot.destination.as_str().to_string(),
            query_session_id_ref: snapshot.query_session_id_ref.clone(),
            scope_binding_id_ref: snapshot.scope_binding_id_ref.clone(),
            selected_result_count: snapshot.count_summary.selected_rows,
            included_result_count: snapshot.count_summary.included_rows,
            omitted_result_count: snapshot.count_summary.omitted_result_count,
            hidden_by_current_scope_rows: snapshot.count_summary.hidden_by_current_scope_rows,
            hidden_by_policy_rows: snapshot.count_summary.hidden_by_policy_rows,
            partiality_reasons: snapshot.partiality_reasons.clone(),
            literal_query_text_included: snapshot.literal_query_text_included,
            raw_query_free_by_default: snapshot.export_avoids_raw_query_by_default(),
            badges: SearchArtifactSurfaceBadges::from_snapshot(snapshot),
        }
    }
}

/// Projection set for one complete durable search artifact workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchArtifactSurfaceProjectionSet {
    /// Saved-query row projection.
    pub saved_query_row: SavedQueryRowProjection,
    /// Query-history row projection.
    pub query_history_row: QueryHistoryRowProjection,
    /// Deep-link open-sheet projection.
    pub deep_link_open_sheet: SearchDeepLinkOpenSheetProjection,
    /// Search-export review projection.
    pub export_review: SearchExportReviewProjection,
}

impl SearchArtifactSurfaceProjectionSet {
    /// Projects all shell surfaces from a complete artifact set.
    pub fn from_artifacts(
        saved_query: &SavedQuery,
        history_entry: &QueryHistoryEntry,
        deep_link: &SearchDeepLink,
        snapshot: &SearchCollectionSnapshot,
        _scope_binding: &ScopePackBinding,
    ) -> Self {
        Self {
            saved_query_row: SavedQueryRowProjection::from_saved_query(saved_query),
            query_history_row: QueryHistoryRowProjection::from_history_entry(history_entry),
            deep_link_open_sheet: SearchDeepLinkOpenSheetProjection::from_deep_link(deep_link),
            export_review: SearchExportReviewProjection::from_snapshot(snapshot),
        }
    }
}

#[cfg(test)]
mod tests {
    use aureline_search::{
        SavedQueryPrivacyClass, SavedQuerySharePolicy, SavedQuerySourceClass,
        SearchArtifactMaterializationInput, SearchArtifactSet, SearchExportDestination,
        SearchPlannerAlpha, SearchPlannerInputs,
    };

    use super::*;

    fn artifact_set() -> SearchArtifactSet {
        let planner_input: SearchPlannerInputs = serde_json::from_value(serde_json::json!({
            "query_session": {
                "record_kind": "search_query_session",
                "schema_version": 1,
                "query_session_id": "search:session:shell-artifact:01",
                "surface": "file_search",
                "query_text_mode": "local_text",
                "query_text": "customer auth retry",
                "scope_class": "selected_workset",
                "stable_scope_id": "scope:workset:auth",
                "scope_mode": "sparse",
                "workset_id": "wks:auth",
                "scope_label": "Selected workset - Auth",
                "planner_version": "search-planner-alpha",
                "readiness_state": "warming",
                "index_epoch": "idx:shell-artifact:01",
                "observed_at": "mono:shell-artifact:01"
            },
            "planner_pass_id": "search:planner:shell-artifact:01",
            "result_set_id": "search:result-set:shell-artifact:01",
            "planner_version": "search-planner-alpha",
            "observed_at": "mono:shell-artifact:02",
            "path_snapshots": [{
                "path_kind": "lexical",
                "snapshot_id": "search:path-snapshot:shell-artifact:lexical",
                "readiness": "partial",
                "freshness": "authoritative_live",
                "index_epoch": "idx:shell-artifact:01",
                "partial_truth_causes": ["indexing_in_progress"],
                "rows": [{
                    "candidate_id": "candidate:shell-artifact:policy",
                    "canonical_id": "workspace:file:services/auth/src/policy.rs",
                    "target_kind": "file",
                    "title": "policy.rs",
                    "relative_path": "services/auth/src/policy.rs",
                    "ranking_reasons": ["lexical_path_match"]
                }]
            }]
        }))
        .expect("unit planner input parses");
        let output = SearchPlannerAlpha::plan(planner_input);

        SearchArtifactSet::materialize(SearchArtifactMaterializationInput {
            saved_query_id: "search:saved:shell-artifact:01".to_string(),
            history_id: "search:history:shell-artifact:01".to_string(),
            scope_binding_id: "search:scope-binding:shell-artifact:01".to_string(),
            deep_link_id: "search:deep-link:shell-artifact:01".to_string(),
            snapshot_id: "search:snapshot:shell-artifact:01".to_string(),
            display_name: "Auth retry investigation".to_string(),
            source_class: SavedQuerySourceClass::SupportCaptured,
            privacy_class: SavedQueryPrivacyClass::SupportExportRedacted,
            share_policy: SavedQuerySharePolicy::SupportExportRedactedOnly,
            destination: SearchExportDestination::SupportBundle,
            query_session: output.query_session,
            result_set: output.result_set,
            selected_result_ids: Vec::new(),
            scope_counts: None,
            retention_mode: None,
            sync_class: None,
            redaction_profile: None,
            retention_widening_basis: None,
            created_at: "2026-05-18T10:00:00Z".to_string(),
            last_used_at: "2026-05-18T10:01:00Z".to_string(),
            exported_at: "2026-05-18T10:02:00Z".to_string(),
            expires_at: None,
        })
        .expect("artifact set materializes")
    }

    #[test]
    fn search_artifact_shell_surfaces_preserve_privacy_and_scope_badges() {
        let artifacts = artifact_set();
        let projections = SearchArtifactSurfaceProjectionSet::from_artifacts(
            &artifacts.saved_query,
            &artifacts.history_entry,
            &artifacts.deep_link,
            &artifacts.collection_snapshot,
            &artifacts.scope_binding,
        );

        assert_eq!(
            projections.saved_query_row.badges.source_class,
            "support_captured"
        );
        assert_eq!(
            projections.saved_query_row.badges.retention_mode,
            "support_export_bounded"
        );
        assert_eq!(projections.query_history_row.stored_text_mode, "hash_only");
        assert!(projections.deep_link_open_sheet.rerun_required);
        assert!(
            projections
                .deep_link_open_sheet
                .recipient_re_resolves_under_current_permissions
        );
        assert!(!projections.deep_link_open_sheet.access_widening_allowed);
        assert!(projections.export_review.raw_query_free_by_default);
        assert_eq!(
            projections.export_review.badges.result_semantics,
            "captured_snapshot"
        );
    }
}
