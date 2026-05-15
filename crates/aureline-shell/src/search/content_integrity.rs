//! Search result content-integrity projections.
//!
//! Search snippets can carry the same deceptive source bytes that editor and
//! diff panes render. This module consumes the shared content-safety warning
//! record so search rows do not define a separate suspicious-text contract.

use aureline_content_safety::{
    project_content_integrity_warnings, ContentIntegritySurfaceKind, ContentIntegrityWarningRecord,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SearchContentIntegrityProjection`].
pub const SEARCH_CONTENT_INTEGRITY_PROJECTION_RECORD_KIND: &str =
    "search_content_integrity_projection";

/// Schema version for [`SearchContentIntegrityProjection`].
pub const SEARCH_CONTENT_INTEGRITY_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Search-row projection carrying shared content-integrity warnings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchContentIntegrityProjection {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Case, session, or result-set id for this detector run.
    pub case_id: String,
    /// Opaque search result row or snippet ref.
    pub result_ref: String,
    /// Shared warning records emitted by `aureline-content-safety`.
    pub warnings: Vec<ContentIntegrityWarningRecord>,
}

impl SearchContentIntegrityProjection {
    /// Returns true when at least one shared warning is present.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Projects shared content-integrity warnings for one search snippet.
pub fn project_search_content_integrity(
    case_id: &str,
    result_ref: &str,
    snippet: &str,
) -> SearchContentIntegrityProjection {
    SearchContentIntegrityProjection {
        record_kind: SEARCH_CONTENT_INTEGRITY_PROJECTION_RECORD_KIND.to_string(),
        schema_version: SEARCH_CONTENT_INTEGRITY_PROJECTION_SCHEMA_VERSION,
        case_id: case_id.to_string(),
        result_ref: result_ref.to_string(),
        warnings: project_content_integrity_warnings(
            case_id,
            ContentIntegritySurfaceKind::Search,
            result_ref,
            snippet,
        ),
    }
}
