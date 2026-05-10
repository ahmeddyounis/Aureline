//! Path-truth chip derivation.
//!
//! The chip is the shortest honest answer to "what does this open
//! actually point at?". It is computed purely from an
//! [`IdentityRecord`] so every surface that has a record can render
//! the same chip class, summary, and detail target without
//! re-walking the filesystem.
//!
//! The derivation never invents alias kinds: when the presentation
//! URI differs from the canonical URI but no matching alias entry
//! exists in the alias set, the chip surfaces
//! [`PathTruthClass::DivergentUnknown`] rather than guessing a kind.

use super::{AliasKind, IdentityRecord, TrustState};
use crate::uri_model::VfsUri;

/// Frozen path-truth chip class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathTruthClass {
    /// Presentation URI equals the canonical URI and no alternate
    /// aliases are known.
    Direct,
    /// Presentation URI equals the canonical URI but additional
    /// alternate aliases are recorded; surfaces should still
    /// disclose the alias set on demand.
    DirectWithKnownAliases,
    ViaSymlink,
    ViaJunction,
    ViaHardlinkSibling,
    ViaCaseOnlyVariant,
    ViaUnicodeNormalizationVariant,
    ViaRemoteAlias,
    ViaBindMountAlias,
    ViaContainerMountAlias,
    ViaArchiveInnerAlias,
    /// Presentation URI differs from canonical but no alias entry
    /// in the set explains the difference. Treated as a
    /// review-required degraded state rather than a silent fallback.
    DivergentUnknown,
}

impl PathTruthClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::DirectWithKnownAliases => "direct_with_known_aliases",
            Self::ViaSymlink => "via_symlink",
            Self::ViaJunction => "via_junction",
            Self::ViaHardlinkSibling => "via_hardlink_sibling",
            Self::ViaCaseOnlyVariant => "via_case_only_variant",
            Self::ViaUnicodeNormalizationVariant => "via_unicode_normalization_variant",
            Self::ViaRemoteAlias => "via_remote_alias",
            Self::ViaBindMountAlias => "via_bind_mount_alias",
            Self::ViaContainerMountAlias => "via_container_mount_alias",
            Self::ViaArchiveInnerAlias => "via_archive_inner_alias",
            Self::DivergentUnknown => "divergent_unknown",
        }
    }

    /// Map an [`AliasKind`] (from the alias entry whose `alias_uri`
    /// matched the presentation URI) onto a chip class. Returns
    /// `None` when no alias kind can be reused, signalling that the
    /// caller should fall back to [`Self::DivergentUnknown`].
    pub const fn from_alias_kind(kind: AliasKind) -> Self {
        match kind {
            AliasKind::Symlink => Self::ViaSymlink,
            AliasKind::Junction => Self::ViaJunction,
            AliasKind::HardlinkSibling => Self::ViaHardlinkSibling,
            AliasKind::CaseOnlyVariant => Self::ViaCaseOnlyVariant,
            AliasKind::UnicodeNormalizationVariant => Self::ViaUnicodeNormalizationVariant,
            AliasKind::RemoteAlias => Self::ViaRemoteAlias,
            AliasKind::BindMountAlias => Self::ViaBindMountAlias,
            AliasKind::ContainerMountAlias => Self::ViaContainerMountAlias,
            AliasKind::ArchiveInnerAlias => Self::ViaArchiveInnerAlias,
        }
    }
}

/// Derived chip the shell renders next to the presentation label.
///
/// All fields are inputs to the rendered chip plus the projections
/// support / save-review surfaces use to explain the chip. Surfaces
/// MUST NOT recompute these by re-walking the alias set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathTruthChip {
    pub class: PathTruthClass,
    pub presentation_uri: VfsUri,
    pub canonical_uri: VfsUri,
    pub logical_uri: VfsUri,
    pub root_badge: String,
    pub display_label: String,
    pub trust_state: TrustState,
    /// `Some(kind)` when the presentation URI opened through a
    /// recorded alias; `None` when the open is direct.
    pub opens_via_alias_kind: Option<AliasKind>,
    pub alias_count: usize,
    /// True when the next save will land at a canonical URI other
    /// than the presentation URI the user typed or clicked.
    pub save_redirects_target: bool,
    /// Stable lookup key the chip uses for its "what does this
    /// mean?" detail surface.
    pub detail_target: &'static str,
    /// Short human-readable summary suitable for chip tooltips.
    pub summary: String,
}

/// Derive the chip class for one open.
pub fn derive_path_truth_chip(record: &IdentityRecord) -> PathTruthChip {
    let presentation_uri = &record.presentation_path.uri;
    let canonical_uri = &record.canonical_filesystem_object.canonical_uri;
    let aliases = &record.alias_set.aliases;

    let presentation_alias_entry = aliases
        .iter()
        .find(|alias| &alias.alias_uri == presentation_uri);
    let presentation_differs = presentation_uri != canonical_uri;

    let opens_via_alias_kind = if presentation_differs {
        presentation_alias_entry.map(|alias| alias.alias_kind)
    } else {
        None
    };

    let class = if !presentation_differs {
        let has_other_aliases = aliases
            .iter()
            .any(|alias| &alias.alias_uri != canonical_uri);
        if has_other_aliases {
            PathTruthClass::DirectWithKnownAliases
        } else {
            PathTruthClass::Direct
        }
    } else if let Some(kind) = opens_via_alias_kind {
        PathTruthClass::from_alias_kind(kind)
    } else {
        PathTruthClass::DivergentUnknown
    };

    let summary = path_truth_summary(
        class,
        &record.presentation_path.display_label,
        record.logical_workspace_identity.trust_state,
        aliases.len(),
    );

    PathTruthChip {
        class,
        presentation_uri: presentation_uri.clone(),
        canonical_uri: canonical_uri.clone(),
        logical_uri: record.logical_workspace_identity.logical_uri.clone(),
        root_badge: record.presentation_path.root_badge.clone(),
        display_label: record.presentation_path.display_label.clone(),
        trust_state: record.logical_workspace_identity.trust_state,
        opens_via_alias_kind,
        alias_count: aliases.len(),
        save_redirects_target: presentation_differs,
        detail_target: "aureline.workspace.showAliasDetails",
        summary,
    }
}

fn path_truth_summary(
    class: PathTruthClass,
    label: &str,
    trust_state: TrustState,
    alias_count: usize,
) -> String {
    let trust_suffix = match trust_state {
        TrustState::Trusted => "",
        TrustState::Restricted => " (restricted workspace)",
        TrustState::PendingEvaluation => " (trust pending)",
    };
    let body = match class {
        PathTruthClass::Direct => format!("{label}: opened at its canonical path"),
        PathTruthClass::DirectWithKnownAliases => format!(
            "{label}: opened at canonical path ({alias_count} alias{plural} known)",
            plural = if alias_count == 1 { "" } else { "es" }
        ),
        PathTruthClass::ViaSymlink => format!("{label}: opened through a symlink alias"),
        PathTruthClass::ViaJunction => format!("{label}: opened through a junction alias"),
        PathTruthClass::ViaHardlinkSibling => {
            format!("{label}: opened through a hardlink-sibling alias")
        }
        PathTruthClass::ViaCaseOnlyVariant => {
            format!("{label}: opened through a case-only variant of the canonical path")
        }
        PathTruthClass::ViaUnicodeNormalizationVariant => format!(
            "{label}: opened through a Unicode-normalization variant of the canonical path"
        ),
        PathTruthClass::ViaRemoteAlias => format!("{label}: opened through a remote alias"),
        PathTruthClass::ViaBindMountAlias => format!("{label}: opened through a bind-mount alias"),
        PathTruthClass::ViaContainerMountAlias => {
            format!("{label}: opened through a container-mount alias")
        }
        PathTruthClass::ViaArchiveInnerAlias => {
            format!("{label}: opened through an archive-inner alias")
        }
        PathTruthClass::DivergentUnknown => format!(
            "{label}: presentation and canonical paths differ but no alias entry explains the redirect"
        ),
    };
    format!("{body}{trust_suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{FallbackIdentityTokenKind, NormalizationForm, StrongestIdentityTokenKind};
    use crate::identity::{
        Alias, AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityRecord,
        IdentityToken, LogicalWorkspaceIdentity, PresentationPath,
    };

    fn record(
        presentation: &str,
        canonical: &str,
        aliases: Vec<Alias>,
        trust_state: TrustState,
    ) -> IdentityRecord {
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
                trust_state,
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
    fn direct_open_with_no_aliases_is_direct() {
        let chip = derive_path_truth_chip(&record(
            "file:///ws/doc",
            "file:///ws/doc",
            vec![],
            TrustState::Trusted,
        ));
        assert_eq!(chip.class, PathTruthClass::Direct);
        assert_eq!(chip.opens_via_alias_kind, None);
        assert!(!chip.save_redirects_target);
        assert_eq!(chip.alias_count, 0);
    }

    #[test]
    fn direct_open_with_extra_aliases_lists_them() {
        let chip = derive_path_truth_chip(&record(
            "file:///ws/doc",
            "file:///ws/doc",
            vec![
                Alias {
                    alias_uri: VfsUri::parse("file:///ws/doc-link".to_owned()).unwrap(),
                    alias_kind: AliasKind::Symlink,
                    resolution_chain: vec![],
                },
                Alias {
                    alias_uri: VfsUri::parse("file:///ws/doc".to_owned()).unwrap(),
                    alias_kind: AliasKind::Symlink,
                    resolution_chain: vec![],
                },
            ],
            TrustState::Trusted,
        ));
        assert_eq!(chip.class, PathTruthClass::DirectWithKnownAliases);
        assert_eq!(chip.alias_count, 2);
        assert!(!chip.save_redirects_target);
    }

    #[test]
    fn opening_through_symlink_classifies_via_symlink() {
        let chip = derive_path_truth_chip(&record(
            "file:///ws/link",
            "file:///ws/doc",
            vec![Alias {
                alias_uri: VfsUri::parse("file:///ws/link".to_owned()).unwrap(),
                alias_kind: AliasKind::Symlink,
                resolution_chain: vec!["-> doc".to_owned()],
            }],
            TrustState::Trusted,
        ));
        assert_eq!(chip.class, PathTruthClass::ViaSymlink);
        assert_eq!(chip.opens_via_alias_kind, Some(AliasKind::Symlink));
        assert!(chip.save_redirects_target);
    }

    #[test]
    fn divergent_open_without_matching_alias_is_unknown() {
        let chip = derive_path_truth_chip(&record(
            "file:///ws/typo",
            "file:///ws/doc",
            vec![],
            TrustState::Trusted,
        ));
        assert_eq!(chip.class, PathTruthClass::DivergentUnknown);
        assert!(chip.save_redirects_target);
    }

    #[test]
    fn restricted_trust_state_appears_in_summary() {
        let chip = derive_path_truth_chip(&record(
            "file:///ws/doc",
            "file:///ws/doc",
            vec![],
            TrustState::Restricted,
        ));
        assert!(chip.summary.contains("restricted workspace"));
    }
}
