//! Save-target review projection.
//!
//! Materializes a "where will my next save land?" record from a
//! [`crate::save::SaveTargetToken`]. The review is computed before
//! a write reaches the pipeline so the shell can disclose
//! redirects, blocked-mode reasons, and trust posture without
//! waiting for the save participant chain to refuse the write.
//!
//! The record is intentionally narrower than the post-failure
//! save-review sheet (`crates/aureline-shell/src/save_review`):
//! that surface kicks in *after* a save attempt has been refused.
//! This one explains the target up front.

use super::{
    derive_path_truth_chip, AliasKind, IdentityRecord, PathTruthChip, PathTruthClass, TrustState,
};
use crate::capabilities::{AtomicWriteMode, CapabilityFlags};
use crate::save::{GenerationTokenKind, PermissionSnapshot, SaveTargetToken};
use crate::uri_model::VfsUri;

/// Why the save target review forbids or warns about the next save.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveTargetReviewBlocker {
    ReadOnly,
    PolicyConstrained,
    ReviewRequiredBeforeSave,
    ReviewRequiredBeforeRename,
    NotWritablePerSnapshot,
    AtomicWriteModeBlocked,
    DivergentUnknownAlias,
    UntrustedWorkspace,
}

impl SaveTargetReviewBlocker {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PolicyConstrained => "policy_constrained",
            Self::ReviewRequiredBeforeSave => "review_required_before_save",
            Self::ReviewRequiredBeforeRename => "review_required_before_rename",
            Self::NotWritablePerSnapshot => "not_writable_per_snapshot",
            Self::AtomicWriteModeBlocked => "atomic_write_mode_blocked",
            Self::DivergentUnknownAlias => "divergent_unknown_alias",
            Self::UntrustedWorkspace => "untrusted_workspace",
        }
    }
}

/// Permission snapshot summary that surfaces don't have to crack
/// open the full [`PermissionSnapshot`] for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionSummary {
    pub writable: bool,
    pub mode: String,
    pub owner: Option<String>,
    pub group: Option<String>,
}

impl From<&PermissionSnapshot> for PermissionSummary {
    fn from(snapshot: &PermissionSnapshot) -> Self {
        Self {
            writable: snapshot.writable,
            mode: snapshot.mode.clone(),
            owner: snapshot.owner.clone(),
            group: snapshot.group.clone(),
        }
    }
}

/// Pre-write save-target review record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveTargetReviewRecord {
    pub presentation_uri: VfsUri,
    pub canonical_uri: VfsUri,
    pub logical_uri: VfsUri,
    pub display_label: String,
    pub root_badge: String,
    pub trust_state: TrustState,
    pub atomic_write_mode: AtomicWriteMode,
    /// The actual URI bytes will be written to. Always equals the
    /// canonical URI; named explicitly so chip / inspector / review
    /// surfaces never advertise the presentation path as the write
    /// target.
    pub writes_to_canonical_uri: VfsUri,
    pub opens_via_alias_kind: Option<AliasKind>,
    pub path_truth_class: PathTruthClass,
    pub permission_summary: PermissionSummary,
    pub pinned_generation_token_kind: GenerationTokenKind,
    pub pinned_generation_token_value: String,
    pub review_required_before_save: bool,
    pub review_required_before_rename: bool,
    /// True when the next save will land at a canonical URI other
    /// than the presentation URI.
    pub save_redirects_target: bool,
    /// All reasons the save target review marks the next save as
    /// blocked / requiring review, in deterministic order.
    pub blockers: Vec<SaveTargetReviewBlocker>,
    /// Lines the support / save-review surfaces quote verbatim.
    pub explainers: Vec<String>,
    pub detail_target: String,
}

/// Build a review record for a [`SaveTargetToken`].
pub fn review_save_target(token: &SaveTargetToken) -> SaveTargetReviewRecord {
    let chip = derive_path_truth_chip(&token.identity);
    let blockers = collect_blockers(token, &chip);
    let explainers = explainers_for(token, &chip, &blockers);
    review_record_from_parts(token, chip, blockers, explainers)
}

fn review_record_from_parts(
    token: &SaveTargetToken,
    chip: PathTruthChip,
    blockers: Vec<SaveTargetReviewBlocker>,
    explainers: Vec<String>,
) -> SaveTargetReviewRecord {
    let identity: &IdentityRecord = &token.identity;
    let canonical_uri = identity.canonical_filesystem_object.canonical_uri.clone();

    SaveTargetReviewRecord {
        presentation_uri: identity.presentation_path.uri.clone(),
        canonical_uri: canonical_uri.clone(),
        logical_uri: identity.logical_workspace_identity.logical_uri.clone(),
        display_label: identity.presentation_path.display_label.clone(),
        root_badge: identity.presentation_path.root_badge.clone(),
        trust_state: identity.logical_workspace_identity.trust_state,
        atomic_write_mode: token.atomic_write_mode,
        writes_to_canonical_uri: canonical_uri,
        opens_via_alias_kind: chip.opens_via_alias_kind,
        path_truth_class: chip.class,
        permission_summary: (&token.permission_snapshot).into(),
        pinned_generation_token_kind: token.compare_before_write_generation_token.kind,
        pinned_generation_token_value: token.compare_before_write_generation_token.value.clone(),
        review_required_before_save: token.review_required_before_save,
        review_required_before_rename: token.review_required_before_rename,
        save_redirects_target: chip.save_redirects_target,
        blockers,
        explainers,
        detail_target: chip.detail_target.to_owned(),
    }
}

fn collect_blockers(token: &SaveTargetToken, chip: &PathTruthChip) -> Vec<SaveTargetReviewBlocker> {
    let flags: &CapabilityFlags = &token.capability_flags;
    let mut blockers: Vec<SaveTargetReviewBlocker> = Vec::new();

    if flags.read_only {
        blockers.push(SaveTargetReviewBlocker::ReadOnly);
    }
    if flags.policy_constrained {
        blockers.push(SaveTargetReviewBlocker::PolicyConstrained);
    }
    if token.review_required_before_save {
        blockers.push(SaveTargetReviewBlocker::ReviewRequiredBeforeSave);
    }
    if token.review_required_before_rename {
        blockers.push(SaveTargetReviewBlocker::ReviewRequiredBeforeRename);
    }
    if !token.permission_snapshot.writable {
        blockers.push(SaveTargetReviewBlocker::NotWritablePerSnapshot);
    }
    if token.atomic_write_mode == AtomicWriteMode::Blocked {
        blockers.push(SaveTargetReviewBlocker::AtomicWriteModeBlocked);
    }
    if chip.class == PathTruthClass::DivergentUnknown {
        blockers.push(SaveTargetReviewBlocker::DivergentUnknownAlias);
    }
    if chip.trust_state != TrustState::Trusted {
        blockers.push(SaveTargetReviewBlocker::UntrustedWorkspace);
    }

    blockers
}

fn explainers_for(
    token: &SaveTargetToken,
    chip: &PathTruthChip,
    blockers: &[SaveTargetReviewBlocker],
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let presentation = token.identity.presentation_path.uri.as_str();
    let canonical = token
        .identity
        .canonical_filesystem_object
        .canonical_uri
        .as_str();
    let label = &token.identity.presentation_path.display_label;

    if chip.save_redirects_target {
        match chip.opens_via_alias_kind {
            Some(kind) => lines.push(format!(
                "{label}: presentation path {presentation} is a {kind} of canonical {canonical}; bytes will land at canonical.",
                kind = kind.as_str(),
            )),
            None => lines.push(format!(
                "{label}: presentation path {presentation} differs from canonical {canonical} but no alias entry explains the redirect; review before saving.",
            )),
        }
    } else {
        lines.push(format!(
            "{label}: presentation path equals canonical path ({canonical})."
        ));
    }

    lines.push(format!(
        "{label}: write mode = {mode}, pinned generation token = {kind}:{value}.",
        mode = token.atomic_write_mode.as_str(),
        kind = token.compare_before_write_generation_token.kind.as_str(),
        value = token.compare_before_write_generation_token.value,
    ));

    for blocker in blockers {
        lines.push(format!(
            "{label}: blocked because {} (atomic_write_mode={}).",
            blocker.as_str(),
            token.atomic_write_mode.as_str(),
        ));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{
        CapabilityFlags, CaseSensitivity, NormalizationForm, SymlinkEscapePolicy,
    };
    use crate::capabilities::{FallbackIdentityTokenKind, StrongestIdentityTokenKind};
    use crate::identity::{
        Alias, AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityRecord,
        IdentityToken, LogicalWorkspaceIdentity, PresentationPath,
    };
    use crate::save::{CompareBeforeWriteGenerationToken, GenerationTokenKind, PermissionSnapshot};

    fn flags(read_only: bool, policy_constrained: bool, review_required: bool) -> CapabilityFlags {
        CapabilityFlags {
            supports_atomic_replace: true,
            supports_in_place_write: true,
            supports_conditional_remote_write: false,
            case_sensitivity: CaseSensitivity::InsensitivePreserving,
            unicode_normalization: NormalizationForm::MixedObserved,
            supports_case_only_rename: true,
            supports_unicode_normalization_rename: true,
            symlink_escape_policy: SymlinkEscapePolicy::Warn,
            read_only,
            policy_constrained,
            review_required_before_save: review_required,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        }
    }

    fn token(
        presentation: &str,
        canonical: &str,
        aliases: Vec<Alias>,
        atomic_write_mode: AtomicWriteMode,
        capability_flags: CapabilityFlags,
        permission_snapshot: PermissionSnapshot,
        trust_state: TrustState,
    ) -> SaveTargetToken {
        let identity = IdentityRecord {
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
                    value: "dev:1/ino:1/gen:7".to_owned(),
                },
                fallback_identity_tokens: vec![FallbackIdentityToken {
                    kind: FallbackIdentityTokenKind::InodeMtimeSize,
                    value: "1/0/0".to_owned(),
                }],
            },
            alias_set: AliasSet { aliases },
        };
        let review_required_before_save = capability_flags.review_required_before_save;
        let review_required_before_rename = capability_flags.review_required_before_rename;
        SaveTargetToken {
            identity,
            capability_flags,
            atomic_write_mode,
            compare_before_write_generation_token: CompareBeforeWriteGenerationToken {
                kind: GenerationTokenKind::DeviceInodeGeneration,
                value: "dev:1/ino:1/gen:7".to_owned(),
                observed_at: "mono:0".to_owned(),
            },
            permission_snapshot,
            review_required_before_save,
            review_required_before_rename,
        }
    }

    #[test]
    fn direct_writable_open_has_no_blockers() {
        let review = review_save_target(&token(
            "file:///ws/doc",
            "file:///ws/doc",
            vec![],
            AtomicWriteMode::AtomicReplace,
            flags(false, false, false),
            PermissionSnapshot::writable_default(),
            TrustState::Trusted,
        ));
        assert_eq!(review.path_truth_class, PathTruthClass::Direct);
        assert!(!review.save_redirects_target);
        assert_eq!(review.blockers, Vec::<SaveTargetReviewBlocker>::new());
        assert_eq!(review.writes_to_canonical_uri, review.canonical_uri);
    }

    #[test]
    fn symlink_open_records_redirect_explanation() {
        let review = review_save_target(&token(
            "file:///ws/link",
            "file:///ws/doc",
            vec![Alias {
                alias_uri: VfsUri::parse("file:///ws/link".to_owned()).unwrap(),
                alias_kind: AliasKind::Symlink,
                resolution_chain: vec!["-> doc".to_owned()],
            }],
            AtomicWriteMode::AtomicReplace,
            flags(false, false, false),
            PermissionSnapshot::writable_default(),
            TrustState::Trusted,
        ));
        assert!(review.save_redirects_target);
        assert_eq!(review.opens_via_alias_kind, Some(AliasKind::Symlink));
        assert!(review
            .explainers
            .iter()
            .any(|line| line.contains("symlink")));
    }

    #[test]
    fn read_only_root_collects_blockers() {
        let review = review_save_target(&token(
            "file:///ws/doc",
            "file:///ws/doc",
            vec![],
            AtomicWriteMode::Blocked,
            flags(true, false, false),
            PermissionSnapshot::read_only_default(),
            TrustState::Trusted,
        ));
        assert!(review.blockers.contains(&SaveTargetReviewBlocker::ReadOnly));
        assert!(review
            .blockers
            .contains(&SaveTargetReviewBlocker::AtomicWriteModeBlocked));
        assert!(review
            .blockers
            .contains(&SaveTargetReviewBlocker::NotWritablePerSnapshot));
    }

    #[test]
    fn divergent_unknown_alias_marks_review_required() {
        let review = review_save_target(&token(
            "file:///ws/typo",
            "file:///ws/doc",
            vec![],
            AtomicWriteMode::AtomicReplace,
            flags(false, false, false),
            PermissionSnapshot::writable_default(),
            TrustState::Trusted,
        ));
        assert_eq!(review.path_truth_class, PathTruthClass::DivergentUnknown);
        assert!(review
            .blockers
            .contains(&SaveTargetReviewBlocker::DivergentUnknownAlias));
    }

    #[test]
    fn restricted_trust_state_appears_in_blockers() {
        let review = review_save_target(&token(
            "file:///ws/doc",
            "file:///ws/doc",
            vec![],
            AtomicWriteMode::AtomicReplace,
            flags(false, false, false),
            PermissionSnapshot::writable_default(),
            TrustState::Restricted,
        ));
        assert!(review
            .blockers
            .contains(&SaveTargetReviewBlocker::UntrustedWorkspace));
    }
}
