//! Alias inspector projection.
//!
//! The alias inspector is the surface that explains, alias by
//! alias, why two different paths refer to the same canonical
//! object. It runs purely off the [`IdentityRecord`] so the
//! support / save-review / explorer surfaces all show the same
//! resolution chain.

use super::{Alias, AliasKind, IdentityRecord};
use crate::uri_model::VfsUri;

/// One row in the alias-inspector projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasInspectionEntry {
    pub alias_uri: VfsUri,
    pub alias_kind: AliasKind,
    pub resolution_chain: Vec<String>,
    /// True when this entry's URI equals the canonical URI of the
    /// underlying object (i.e. this is the canonical "spelling").
    pub is_canonical: bool,
    /// True when this entry's URI equals the presentation URI the
    /// user opened in this session.
    pub is_presentation: bool,
}

/// Aggregated alias inspector record for one open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasInspectionRecord {
    pub presentation_uri: VfsUri,
    pub canonical_uri: VfsUri,
    pub logical_uri: VfsUri,
    pub display_label: String,
    pub root_badge: String,
    pub entries: Vec<AliasInspectionEntry>,
    /// All distinct alias kinds present in [`Self::entries`], in
    /// first-occurrence order.
    pub distinct_alias_kinds: Vec<AliasKind>,
    /// True when the presentation URI does not appear in
    /// [`Self::entries`] under its own URI; usually a fixture bug
    /// or a degraded state caller surfaces verbatim.
    pub presentation_alias_missing: bool,
}

/// Build the alias-inspector projection from an [`IdentityRecord`].
pub fn inspect_aliases(record: &IdentityRecord) -> AliasInspectionRecord {
    let presentation_uri = &record.presentation_path.uri;
    let canonical_uri = &record.canonical_filesystem_object.canonical_uri;

    let entries: Vec<AliasInspectionEntry> = record
        .alias_set
        .aliases
        .iter()
        .map(|alias| build_entry(alias, presentation_uri, canonical_uri))
        .collect();

    let mut distinct_alias_kinds: Vec<AliasKind> = Vec::new();
    for entry in &entries {
        if !distinct_alias_kinds.contains(&entry.alias_kind) {
            distinct_alias_kinds.push(entry.alias_kind);
        }
    }

    let presentation_alias_missing = presentation_uri != canonical_uri
        && !entries.iter().any(|entry| entry.is_presentation);

    AliasInspectionRecord {
        presentation_uri: presentation_uri.clone(),
        canonical_uri: canonical_uri.clone(),
        logical_uri: record.logical_workspace_identity.logical_uri.clone(),
        display_label: record.presentation_path.display_label.clone(),
        root_badge: record.presentation_path.root_badge.clone(),
        entries,
        distinct_alias_kinds,
        presentation_alias_missing,
    }
}

fn build_entry(
    alias: &Alias,
    presentation_uri: &VfsUri,
    canonical_uri: &VfsUri,
) -> AliasInspectionEntry {
    AliasInspectionEntry {
        alias_uri: alias.alias_uri.clone(),
        alias_kind: alias.alias_kind,
        resolution_chain: alias.resolution_chain.clone(),
        is_canonical: &alias.alias_uri == canonical_uri,
        is_presentation: &alias.alias_uri == presentation_uri,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{FallbackIdentityTokenKind, NormalizationForm, StrongestIdentityTokenKind};
    use crate::identity::{
        Alias, AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityRecord,
        IdentityToken, LogicalWorkspaceIdentity, PresentationPath, TrustState,
    };

    fn record(presentation: &str, canonical: &str, aliases: Vec<Alias>) -> IdentityRecord {
        IdentityRecord {
            presentation_path: PresentationPath {
                uri: VfsUri::parse(presentation.to_owned()).unwrap(),
                display_label: "doc".to_owned(),
                root_badge: "local".to_owned(),
            },
            logical_workspace_identity: LogicalWorkspaceIdentity {
                workspace_id: "ws".to_owned(),
                root_id: "root".to_owned(),
                logical_uri: VfsUri::parse("aureline-ws://ws/root/doc".to_owned()).unwrap(),
                trust_state: TrustState::Trusted,
                policy_scope: None,
            },
            canonical_filesystem_object: CanonicalFilesystemObject {
                canonical_uri: VfsUri::parse(canonical.to_owned()).unwrap(),
                normalization_form: NormalizationForm::Nfc,
                strongest_identity_token: IdentityToken {
                    kind: StrongestIdentityTokenKind::DeviceInodeGeneration,
                    value: "dev:1/ino:1/gen:1".to_owned(),
                },
                fallback_identity_tokens: vec![FallbackIdentityToken {
                    kind: FallbackIdentityTokenKind::InodeMtimeSize,
                    value: "1/0/0".to_owned(),
                }],
            },
            alias_set: AliasSet { aliases },
        }
    }

    #[test]
    fn marks_presentation_and_canonical_entries() {
        let inspection = inspect_aliases(&record(
            "file:///ws/link",
            "file:///ws/doc",
            vec![
                Alias {
                    alias_uri: VfsUri::parse("file:///ws/link".to_owned()).unwrap(),
                    alias_kind: AliasKind::Symlink,
                    resolution_chain: vec!["-> doc".to_owned()],
                },
                Alias {
                    alias_uri: VfsUri::parse("file:///ws/doc".to_owned()).unwrap(),
                    alias_kind: AliasKind::Symlink,
                    resolution_chain: vec!["-> canonical".to_owned()],
                },
            ],
        ));

        assert_eq!(inspection.entries.len(), 2);
        assert!(inspection.entries[0].is_presentation);
        assert!(!inspection.entries[0].is_canonical);
        assert!(!inspection.entries[1].is_presentation);
        assert!(inspection.entries[1].is_canonical);
        assert_eq!(inspection.distinct_alias_kinds, vec![AliasKind::Symlink]);
        assert!(!inspection.presentation_alias_missing);
    }

    #[test]
    fn flags_missing_presentation_entry_for_divergent_open() {
        let inspection = inspect_aliases(&record(
            "file:///ws/typo",
            "file:///ws/doc",
            vec![Alias {
                alias_uri: VfsUri::parse("file:///ws/doc".to_owned()).unwrap(),
                alias_kind: AliasKind::Symlink,
                resolution_chain: vec!["-> canonical".to_owned()],
            }],
        ));
        assert!(inspection.presentation_alias_missing);
    }
}
