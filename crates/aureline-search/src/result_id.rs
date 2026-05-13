//! Stable search-result identifier builders.
//!
//! Search surfaces project many row shapes: lexical file rows, quick-open
//! recents, command rows, and planner-fused symbol rows. This module owns the
//! small set of deterministic ID builders those surfaces share so support
//! exports and deep-link recovery do not reverse-engineer IDs from rendered
//! text.

use serde::{Deserialize, Serialize};

use crate::lexical::SourceClass;
use crate::query_session::SearchSurface;

/// Stable scheme token for lexical result IDs.
pub const LEXICAL_RESULT_ID_SCHEME: &str = "wsearch";

/// Result-kind token used by cross-surface stable result IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableResultKind {
    /// A recently opened file, place, or route target.
    RecentTarget,
    /// A canonical command-registry row.
    Command,
    /// A workspace-relative file path.
    WorkspaceFile,
    /// A symbol, route, type, member, or structural target.
    Symbol,
}

impl StableResultKind {
    /// Stable token used in result IDs and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentTarget => "recent_target",
            Self::Command => "command",
            Self::WorkspaceFile => "workspace_file",
            Self::Symbol => "symbol",
        }
    }
}

/// Normalizes one ID segment without changing case-sensitive path or command
/// semantics.
pub fn normalize_result_id_part(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "unknown".to_string();
    }
    trimmed
        .chars()
        .map(|ch| {
            if ch.is_control() || ch.is_whitespace() {
                '_'
            } else {
                ch
            }
        })
        .collect()
}

/// Builds the canonical lexical-result ID used by [`crate::ResultIdentity`].
///
/// The returned value intentionally matches the existing lexical URN shape:
/// `wsearch:{workspace_id}:{source_class}:{relative_path}`. Keeping this
/// helper separate lets quick open quote the same file-row identity without
/// constructing a full lexical [`crate::ResultIdentity`] packet.
pub fn build_lexical_result_id(
    workspace_id: &str,
    source_class: SourceClass,
    relative_path: &str,
) -> String {
    format!(
        "{}:{}:{}:{}",
        LEXICAL_RESULT_ID_SCHEME,
        normalize_result_id_part(workspace_id),
        source_class.as_str(),
        normalize_result_id_part(relative_path),
    )
}

/// Builds a cross-surface result ID for rows that do not come from the lexical
/// search row contract.
pub fn build_surface_result_id(
    surface: SearchSurface,
    workspace_id: &str,
    kind: StableResultKind,
    canonical_ref: &str,
) -> String {
    format!(
        "search:result:{}:{}:{}:{}",
        surface.as_str(),
        normalize_result_id_part(workspace_id),
        kind.as_str(),
        normalize_result_id_part(canonical_ref),
    )
}

/// Builds the stable result ID for a planner-fused row.
///
/// This preserves the existing planner result shape so fixture and support
/// references remain stable while giving the format a single owning function.
pub fn build_planned_result_id(surface: SearchSurface, canonical_id: &str) -> String {
    format!(
        "search:planned:{}:{}",
        surface.as_str(),
        normalize_result_id_part(canonical_id),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexical::SourceClass;
    use crate::query_session::SearchSurface;

    #[test]
    fn lexical_builder_preserves_existing_urn_shape() {
        assert_eq!(
            build_lexical_result_id("ws-alpha", SourceClass::LexicalFilename, "src/main.rs",),
            "wsearch:ws-alpha:lexical_filename:src/main.rs",
        );
    }

    #[test]
    fn surface_builder_is_stable_for_command_rows() {
        assert_eq!(
            build_surface_result_id(
                SearchSurface::QuickOpen,
                "ws-alpha",
                StableResultKind::Command,
                "cmd:workspace.open_folder",
            ),
            "search:result:quick_open:ws-alpha:command:cmd:workspace.open_folder",
        );
    }

    #[test]
    fn planned_builder_preserves_existing_shape() {
        assert_eq!(
            build_planned_result_id(
                SearchSurface::SymbolSearch,
                "workspace:symbol:open_workspace"
            ),
            "search:planned:symbol_search:workspace:symbol:open_workspace",
        );
    }
}
