//! Search helpers for the command-reference catalog.
//!
//! The search index is the same one consumed by the palette, the
//! docs/help search box, the CLI help renderer, and onboarding so
//! resolving a command by label, id, alias, or literal key sequence
//! produces the same hits.

use super::{CommandReferenceCatalog, CommandReferenceEntry, SearchTokenClass};

/// Match class returned by [`search_entries`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SearchMatchKind {
    /// Exact token equality.
    Exact,
    /// Case-insensitive substring match.
    Substring,
}

/// One search hit returned by [`search_entries`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit<'a> {
    pub entry: &'a CommandReferenceEntry,
    pub token_class: SearchTokenClass,
    pub matched_value: String,
    pub match_kind: SearchMatchKind,
}

fn normalize(token: &str) -> String {
    token.trim().to_ascii_lowercase()
}

/// Returns the deterministic ranked hits for a query against the
/// catalog. Hits are emitted in catalog order, with exact matches
/// preceding substring matches for the same entry, and tokens
/// emitted in the canonical token order on the entry.
pub fn search_entries<'a>(catalog: &'a CommandReferenceCatalog, query: &str) -> Vec<SearchHit<'a>> {
    let normalized = normalize(query);
    if normalized.is_empty() {
        return Vec::new();
    }

    let mut exact = Vec::new();
    let mut substring = Vec::new();

    for entry in &catalog.entries {
        for token in &entry.search_index {
            let value = normalize(&token.value);
            if value == normalized {
                exact.push(SearchHit {
                    entry,
                    token_class: token.token_class,
                    matched_value: token.value.clone(),
                    match_kind: SearchMatchKind::Exact,
                });
            } else if value.contains(&normalized) {
                substring.push(SearchHit {
                    entry,
                    token_class: token.token_class,
                    matched_value: token.value.clone(),
                    match_kind: SearchMatchKind::Substring,
                });
            }
        }
    }

    exact.extend(substring);
    exact
}

#[cfg(test)]
mod tests {
    use super::super::seeded_command_reference_catalog;
    use super::*;

    #[test]
    fn label_query_returns_matching_entry() {
        let catalog = seeded_command_reference_catalog();
        let hits = search_entries(&catalog, "Open Folder");
        assert!(!hits.is_empty());
        assert_eq!(hits[0].entry.command_id, "cmd:workspace.open_folder");
        assert_eq!(hits[0].token_class, SearchTokenClass::HumanLabel);
        assert_eq!(hits[0].match_kind, SearchMatchKind::Exact);
    }

    #[test]
    fn command_id_query_returns_matching_entry() {
        let catalog = seeded_command_reference_catalog();
        let hits = search_entries(&catalog, "cmd:workspace.import_profile");
        assert!(!hits.is_empty());
        assert_eq!(hits[0].entry.command_id, "cmd:workspace.import_profile");
        assert_eq!(hits[0].token_class, SearchTokenClass::CommandId);
        assert_eq!(hits[0].match_kind, SearchMatchKind::Exact);
    }

    #[test]
    fn alias_query_returns_canonical_entry() {
        let catalog = seeded_command_reference_catalog();
        let hits = search_entries(
            &catalog,
            "alias:workspace.open_folder:legacy_file_open_folder",
        );
        assert!(!hits.is_empty());
        assert_eq!(hits[0].entry.command_id, "cmd:workspace.open_folder");
        assert_eq!(hits[0].token_class, SearchTokenClass::AliasId);
    }

    #[test]
    fn key_sequence_query_returns_canonical_entry() {
        let catalog = seeded_command_reference_catalog();
        let hits = search_entries(&catalog, "chord:cmd+shift+p");
        assert!(!hits.is_empty());
        assert_eq!(hits[0].entry.command_id, "cmd:command_palette.open");
        assert_eq!(hits[0].token_class, SearchTokenClass::KeySequence);
    }

    #[test]
    fn empty_query_returns_no_hits() {
        let catalog = seeded_command_reference_catalog();
        let hits = search_entries(&catalog, "   ");
        assert!(hits.is_empty());
    }

    #[test]
    fn substring_query_falls_back_to_partial_match() {
        let catalog = seeded_command_reference_catalog();
        let hits = search_entries(&catalog, "restore");
        assert!(!hits.is_empty());
        assert_eq!(
            hits[0].entry.command_id,
            "cmd:workspace.restore_from_checkpoint"
        );
    }
}
