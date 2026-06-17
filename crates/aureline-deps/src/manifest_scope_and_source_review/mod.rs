//! Manifest-scope selectors, root-versus-member scope diffs, registry-source and
//! mirror-auth trust cues, and exact requested-versus-resolved dependency
//! identity for the M5 package-mutation review and result lane.
//!
//! Where
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! *freezes the vocabulary* and [`crate::package_state_descriptors`] *describes
//! one package's state*, this module makes the **target of a mutation explicit
//! before it leaves review**: which manifest or workspace member is being
//! changed, whether a member-level operation silently broadens to the workspace
//! root, and which registry/mirror/auth/freshness/revocation path will resolve
//! it. A [`ScopeMutationRow`] is the one review/result object the desktop
//! workspace, CLI/headless, review workspace, AI context, and support/export
//! packets all reuse, so monorepo scope selection is product-visible rather than
//! an implementation detail of a per-ecosystem adapter.
//!
//! Three properties hold by construction and are validated against the frozen
//! matrix:
//!
//! 1. **Scope never silently broadens.** Each row carries a durable
//!    [`ManifestScopeSelector`] for the *requested* target and one for the
//!    *resolved* effective scope, and a [`ScopeFidelity`] that classifies the
//!    relationship. A workspace-root operation, a member operation that also
//!    touches the shared root lockfile, and a confirmed whole-workspace
//!    operation are kept distinct; an unconfirmed broadening is never
//!    appliable ([`ScopeMutationRow::can_apply`] is `false`), so a member-level
//!    change can never quietly widen to the wrong manifest.
//! 2. **Requested-versus-resolved identity stays separate.** Each row reuses the
//!    descriptor's [`crate::package_state_descriptors::RequestedIdentity`] and
//!    optional [`crate::package_state_descriptors::ResolvedIdentity`] in distinct
//!    fields, and the package's requested manifest scope must agree with the
//!    row's requested selector, so a requested constraint and a resolved fact —
//!    at both the package and the scope level — can never flatten into one badge.
//! 3. **Registry trust is never overclaimed.** Every row carries a
//!    [`RegistrySourceCue`] that surfaces the registry-source class, mirror
//!    owner, auth mode, freshness, and revocation state. A revoked, stale,
//!    offline, or auth-blocked source blocks trust and must disclose itself
//!    specifically; the cue's message class is always a frozen, specific source
//!    disclosure and never a generic not-found or install-failed message.
//!
//! The packet is checked in at `artifacts/deps/m5/manifest-scope-review.json` and
//! embedded here, so this typed consumer and any CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque,
//! redacted ref. It carries no credential bodies, registry tokens, raw provider
//! payloads, or private registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, ManifestScopeClass, PackageStateLabel,
    PackageStateMessageClass, PackageSurface, RegistrySourceAuthority, SurfaceWriteAuthority,
};
use crate::package_state_descriptors::{RequestedIdentity, ResolvedIdentity};

/// Supported manifest-scope review packet schema version.
pub const MANIFEST_SCOPE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const MANIFEST_SCOPE_REVIEW_RECORD_KIND: &str = "manifest_scope_review";

/// Repo-relative path to the checked-in packet.
pub const MANIFEST_SCOPE_REVIEW_PATH: &str = "artifacts/deps/m5/manifest-scope-review.json";

/// Embedded checked-in packet JSON.
pub const MANIFEST_SCOPE_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/manifest-scope-review.json"
));

/// The role a targeted manifest plays in a workspace.
///
/// This is the root-versus-member distinction made explicit. The frozen
/// [`ManifestScopeClass`] names the *breadth* of an operation (whole workspace,
/// a selected manifest, a workset slice, a member, a path/VCS target); the role
/// names the *identity* of the manifest the user pointed at, so a change to a
/// member is never confused with a change to the workspace root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestRole {
    /// The root manifest that defines the workspace.
    WorkspaceRoot,
    /// A member or module manifest inside a workspace.
    WorkspaceMember,
    /// A single manifest that is not part of a workspace.
    StandaloneManifest,
    /// A filesystem-path or version-control target outside any manifest tree.
    PathOrVcsTarget,
}

impl ManifestRole {
    /// Every manifest role, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::WorkspaceRoot,
        Self::WorkspaceMember,
        Self::StandaloneManifest,
        Self::PathOrVcsTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace_root",
            Self::WorkspaceMember => "workspace_member",
            Self::StandaloneManifest => "standalone_manifest",
            Self::PathOrVcsTarget => "path_or_vcs_target",
        }
    }

    /// Whether this role is the workspace root.
    pub const fn is_root(self) -> bool {
        matches!(self, Self::WorkspaceRoot)
    }

    /// Whether this role is a workspace member or module.
    pub const fn is_member(self) -> bool {
        matches!(self, Self::WorkspaceMember)
    }

    /// Whether a manifest in this role must name a parent (root) manifest.
    ///
    /// Only a member has a parent; a root, a standalone manifest, and a path/VCS
    /// target are top-level and must not name one.
    pub const fn requires_parent(self) -> bool {
        matches!(self, Self::WorkspaceMember)
    }

    /// Whether this role permits a given frozen manifest scope.
    ///
    /// A member may be targeted only as itself or as a selected manifest; a root
    /// may anchor a whole-workspace, workset-slice, or selected-manifest
    /// operation; a standalone manifest is always a selected manifest; a path/VCS
    /// target uses the path/VCS scope. This is how a member-level operation is
    /// stopped from claiming a whole-workspace scope.
    pub const fn permits_scope(self, scope: ManifestScopeClass) -> bool {
        match self {
            Self::WorkspaceRoot => matches!(
                scope,
                ManifestScopeClass::WholeWorkspace
                    | ManifestScopeClass::WorksetSlice
                    | ManifestScopeClass::SelectedManifest
            ),
            Self::WorkspaceMember => matches!(
                scope,
                ManifestScopeClass::WorkspaceMember | ManifestScopeClass::SelectedManifest
            ),
            Self::StandaloneManifest => matches!(scope, ManifestScopeClass::SelectedManifest),
            Self::PathOrVcsTarget => matches!(scope, ManifestScopeClass::PathOrVcsTarget),
        }
    }
}

/// How current the registry or mirror metadata behind a source is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFreshness {
    /// Metadata is current and authoritative.
    Current,
    /// Metadata is known to be stale.
    Stale,
    /// Only an offline snapshot of the metadata is available.
    OfflineSnapshot,
    /// Metadata freshness could not be established.
    Unknown,
}

impl SourceFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::Stale,
        Self::OfflineSnapshot,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the source is current enough to resolve an install without a
    /// freshness disclosure.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Whether the credential or signing material behind a source has been revoked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationState {
    /// The source credential or signing material is active.
    NotRevoked,
    /// The source credential or signing material has been revoked.
    Revoked,
    /// Revocation state could not be established.
    Unknown,
}

impl RevocationState {
    /// Every revocation state, in declaration order.
    pub const ALL: [Self; 3] = [Self::NotRevoked, Self::Revoked, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRevoked => "not_revoked",
            Self::Revoked => "revoked",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this state blocks trust outright.
    ///
    /// A revoked source can never be trusted; an unknown state must be disclosed
    /// but does not block on its own.
    pub const fn blocks_trust(self) -> bool {
        matches!(self, Self::Revoked)
    }

    /// Whether this state must be disclosed rather than read as clean.
    pub const fn must_disclose(self) -> bool {
        !matches!(self, Self::NotRevoked)
    }
}

/// How a resolved effective scope relates to the requested target scope.
///
/// This is the no-silent-broadening axis. A member operation may legitimately
/// touch the shared root lockfile, and a whole-workspace operation is legitimate
/// once confirmed; an *unconfirmed* broadening is the failure mode the lane
/// guards against and is never appliable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeFidelity {
    /// The resolved scope is exactly the requested target; nothing else changes.
    Exact,
    /// A member target whose shared root lockfile is also affected; disclosed.
    DisclosedSharedLockfile,
    /// A whole-workspace operation that was explicitly confirmed.
    ConfirmedWorkspaceWide,
    /// The resolved scope is wider than requested and was not confirmed.
    UnconfirmedBroadening,
}

impl ScopeFidelity {
    /// Every scope fidelity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::DisclosedSharedLockfile,
        Self::ConfirmedWorkspaceWide,
        Self::UnconfirmedBroadening,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::DisclosedSharedLockfile => "disclosed_shared_lockfile",
            Self::ConfirmedWorkspaceWide => "confirmed_workspace_wide",
            Self::UnconfirmedBroadening => "unconfirmed_broadening",
        }
    }

    /// Whether the resolved scope reaches beyond the requested target manifest.
    pub const fn broadens_beyond_target(self) -> bool {
        !matches!(self, Self::Exact)
    }

    /// Whether this fidelity is a silent (unconfirmed) broadening.
    pub const fn is_silent_broadening(self) -> bool {
        matches!(self, Self::UnconfirmedBroadening)
    }

    /// Whether a broadening at this fidelity is acknowledged without an explicit
    /// confirmation flag.
    ///
    /// A disclosed shared-lockfile update keeps the member as the target and is
    /// inherently acknowledged; every other broadening must be confirmed.
    pub const fn broadening_self_acknowledged(self) -> bool {
        matches!(self, Self::DisclosedSharedLockfile)
    }
}

/// A durable selector that names exactly one manifest target.
///
/// The selector carries a stable [`Self::manifest_id`] and a
/// [`Self::continuity_token`] so its identity survives a mutation and a reopen;
/// a member selector names its [`Self::parent_manifest_id`] so the root is always
/// reachable from a member view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestScopeSelector {
    /// Durable manifest identity, stable across apply and reopen.
    pub manifest_id: String,
    /// The role this manifest plays in the workspace.
    pub role: ManifestRole,
    /// The frozen breadth class of the operation against this manifest.
    pub scope_class: ManifestScopeClass,
    /// Human-readable selector label.
    pub display_label: String,
    /// Redacted manifest path; never a raw URL or absolute secret path.
    pub redacted_manifest_path: String,
    /// Parent (root) manifest id; present only for a workspace member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_manifest_id: Option<String>,
    /// Opaque continuity token proving identity survives apply and reopen.
    pub continuity_token: String,
}

impl ManifestScopeSelector {
    /// Whether the selector is internally consistent: the role permits the scope
    /// class, and parent linkage matches the role.
    pub fn is_consistent(&self) -> bool {
        self.role.permits_scope(self.scope_class)
            && self.parent_manifest_id.is_some() == self.role.requires_parent()
    }

    /// Whether this selector targets the workspace root.
    pub fn targets_root(&self) -> bool {
        self.role.is_root()
    }

    /// Whether this selector targets a workspace member or module.
    pub fn targets_member(&self) -> bool {
        self.role.is_member()
    }

    /// Whether this scope must be confirmed explicitly before a bulk mutation.
    pub fn requires_explicit_confirmation(&self) -> bool {
        self.scope_class.requires_explicit_confirmation()
    }

    /// Whether this selector and another name the same durable identity — the
    /// same manifest id, role, and continuity token. This is the post-apply
    /// continuity check.
    pub fn same_identity(&self, other: &ManifestScopeSelector) -> bool {
        self.manifest_id == other.manifest_id
            && self.role == other.role
            && self.continuity_token == other.continuity_token
    }
}

/// The registry, mirror, auth, freshness, and revocation cue behind a resolution.
///
/// Anywhere a package mutation or install review invites trust, this cue is the
/// single object that says where the package will come from and whether that path
/// can be trusted right now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistrySourceCue {
    /// Registry or mirror source class.
    pub source_class: RegistrySourceAuthority,
    /// Auth mode used (or required) to reach the source.
    pub auth_mode: AuthMode,
    /// Freshness of the source metadata.
    pub freshness: SourceFreshness,
    /// Revocation state of the source credential or signing material.
    pub revocation: RevocationState,
    /// Redacted mirror or private-registry owner; present only for a private
    /// registry or enterprise mirror, and never a raw URL or token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_owner: Option<String>,
    /// Redacted source label safe for support exports; never a URL or token.
    pub redacted_source_label: String,
}

impl RegistrySourceCue {
    /// Whether a source class names an owner (a private registry or mirror).
    pub const fn class_has_owner(source: RegistrySourceAuthority) -> bool {
        matches!(
            source,
            RegistrySourceAuthority::PrivateRegistry | RegistrySourceAuthority::EnterpriseMirror
        )
    }

    /// The specific frozen source-disclosure message this cue renders.
    ///
    /// This is always a specific source disclosure (public/private/mirror/cache/
    /// offline) and never a generic collapse message.
    pub const fn message_class(&self) -> PackageStateMessageClass {
        self.source_class.canonical_message_class()
    }

    /// Whether trust in this source is blocked right now.
    ///
    /// A revoked source or an unsatisfied auth requirement blocks trust; freshness
    /// and an unknown revocation state are disclosed but do not block on their own.
    pub const fn trust_blocked(&self) -> bool {
        self.revocation.blocks_trust() || self.auth_mode.blocks_until_satisfied()
    }

    /// Whether the cue must disclose its environment rather than read as a clean,
    /// current public resolution.
    pub fn must_disclose(&self) -> bool {
        self.source_class.requires_specific_disclosure()
            || !self.freshness.is_current()
            || self.revocation.must_disclose()
            || self.auth_mode.blocks_until_satisfied()
    }

    /// Whether the cue is internally consistent: owner presence matches the
    /// source class, and the message stays specific.
    pub fn is_consistent(&self) -> bool {
        self.mirror_owner.is_some() == Self::class_has_owner(self.source_class)
            && self.message_class().is_specific()
    }
}

/// A single manifest-scope mutation review (and, once applied, result) row.
///
/// One row describes one package operation against one manifest target: what
/// manifest the user pointed at, what manifest set will actually change, the
/// requested and resolved dependency identity, and the registry/mirror/auth path
/// that will resolve it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeMutationRow {
    /// Stable row id.
    pub row_id: String,
    /// The manifest the user or automation targeted.
    pub requested_scope: ManifestScopeSelector,
    /// The manifest scope that will actually be modified.
    pub resolved_scope: ManifestScopeSelector,
    /// How the resolved scope relates to the requested target.
    pub fidelity: ScopeFidelity,
    /// Whether a broadening beyond the target was explicitly confirmed.
    pub broadening_confirmed: bool,
    /// Every manifest id this operation affects, including the target.
    #[serde(default)]
    pub affected_manifest_ids: Vec<String>,
    /// The requested dependency identity.
    pub requested: RequestedIdentity,
    /// The resolved dependency identity; absent when the package is unresolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedIdentity>,
    /// The registry/mirror/auth/freshness/revocation cue.
    pub source_cue: RegistrySourceCue,
    /// The selector observed after apply; present only on an applied result row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_apply_scope: Option<ManifestScopeSelector>,
    /// Reviewer-facing note.
    pub note: String,
}

impl ScopeMutationRow {
    /// Whether the operation targets a workspace member.
    pub fn targets_member(&self) -> bool {
        self.requested_scope.targets_member()
    }

    /// Whether the operation targets the workspace root.
    pub fn targets_root(&self) -> bool {
        self.requested_scope.targets_root()
    }

    /// Whether the resolved scope reaches manifests beyond the requested target.
    pub fn broadens_beyond_target(&self) -> bool {
        self.affected_manifest_ids
            .iter()
            .any(|id| id != &self.requested_scope.manifest_id)
    }

    /// Whether the operation is a silent (unconfirmed) broadening.
    pub fn is_silent_broadening(&self) -> bool {
        self.fidelity.is_silent_broadening()
    }

    /// Whether this row has been applied (carries a post-apply selector).
    pub fn is_applied(&self) -> bool {
        self.post_apply_scope.is_some()
    }

    /// Whether durable scope identity was preserved across apply.
    ///
    /// `true` when the row is not yet applied, or when the post-apply selector
    /// names the same durable identity as the resolved scope.
    pub fn continuity_preserved(&self) -> bool {
        match &self.post_apply_scope {
            None => true,
            Some(after) => self.resolved_scope.same_identity(after),
        }
    }

    /// Whether the operation may be applied.
    ///
    /// Blocked by a trust-blocked source, an unacknowledged broadening beyond the
    /// target, an unsatisfied whole-workspace confirmation, or an inconsistent
    /// selector — never silently.
    pub fn can_apply(&self) -> bool {
        if self.source_cue.trust_blocked() {
            return false;
        }
        // A broadening beyond the target must be acknowledged: a disclosed
        // shared-lockfile update keeps the member as the target and needs no
        // confirmation; any other broadening must be explicitly confirmed.
        if self.broadens_beyond_target()
            && !self.broadening_confirmed
            && !self.fidelity.broadening_self_acknowledged()
        {
            return false;
        }
        if self.resolved_scope.requires_explicit_confirmation() && !self.broadening_confirmed {
            return false;
        }
        self.requested_scope.is_consistent() && self.resolved_scope.is_consistent()
    }

    /// The requested-side frozen labels (a policy pin, if any).
    pub fn requested_labels(&self) -> BTreeSet<PackageStateLabel> {
        self.requested.requested_labels().into_iter().collect()
    }

    /// The resolved-side frozen labels (the dependency relation, if resolved).
    pub fn resolved_labels(&self) -> BTreeSet<PackageStateLabel> {
        let mut labels = BTreeSet::new();
        if let Some(resolved) = &self.resolved {
            labels.insert(resolved.relation_label());
        }
        labels
    }

    /// Whether requested and resolved identity stay on disjoint label sets.
    pub fn requested_and_resolved_separate(&self) -> bool {
        self.requested_labels().is_disjoint(&self.resolved_labels())
    }

    /// Whether the package's requested manifest scope agrees with the requested
    /// selector's breadth — the scope-level requested-versus-resolved check.
    pub fn scope_identity_aligned(&self) -> bool {
        self.requested.manifest_scope == self.requested_scope.scope_class
    }

    /// Every frozen package-state label this row surfaces.
    pub fn applicable_labels(&self) -> BTreeSet<PackageStateLabel> {
        let mut labels = self.requested_labels();
        labels.extend(self.resolved_labels());
        labels
    }

    /// Projects the root-versus-member diff between the requested and resolved
    /// scope, reused by the review sheet and result packet.
    pub fn scope_diff(&self) -> ScopeDiffView {
        ScopeDiffView {
            row_id: self.row_id.clone(),
            requested_manifest_id: self.requested_scope.manifest_id.clone(),
            requested_role: self.requested_scope.role.as_str().to_owned(),
            requested_scope_class: self.requested_scope.scope_class.as_str().to_owned(),
            resolved_manifest_id: self.resolved_scope.manifest_id.clone(),
            resolved_role: self.resolved_scope.role.as_str().to_owned(),
            resolved_scope_class: self.resolved_scope.scope_class.as_str().to_owned(),
            parent_manifest_id: self.requested_scope.parent_manifest_id.clone(),
            role_changed: self.requested_scope.role != self.resolved_scope.role,
            scope_class_changed: self.requested_scope.scope_class
                != self.resolved_scope.scope_class,
            broadened: self.broadens_beyond_target(),
            broadening_confirmed: self.broadening_confirmed,
            silent_broadening: self.is_silent_broadening(),
            affected_manifest_ids: self.affected_manifest_ids.clone(),
            fidelity: self.fidelity.as_str().to_owned(),
        }
    }

    /// Projects the canonical per-row view reused by the desktop workspace,
    /// review workspace, and AI inspect surfaces.
    pub fn view(&self) -> ScopeMutationView {
        ScopeMutationView {
            row_id: self.row_id.clone(),
            package_name: self.requested.package_name.clone(),
            ecosystem: self.requested.ecosystem.as_str().to_owned(),
            scope_diff: self.scope_diff(),
            requested_ref: self.requested.requested_ref.clone(),
            resolved_ref: self.resolved.as_ref().map(|r| r.resolved_ref.clone()),
            relation: self
                .resolved
                .as_ref()
                .map(|r| r.relation.as_str().to_owned()),
            requested_resolved_separate: self.requested_and_resolved_separate(),
            scope_identity_aligned: self.scope_identity_aligned(),
            source: self.source_view(),
            can_apply: self.can_apply(),
            is_applied: self.is_applied(),
            continuity_preserved: self.continuity_preserved(),
            applicable_labels: self
                .applicable_labels()
                .iter()
                .map(|l| l.as_str().to_owned())
                .collect(),
        }
    }

    /// Projects the registry-source cue into a redaction-safe view.
    pub fn source_view(&self) -> RegistrySourceCueView {
        RegistrySourceCueView {
            source_class: self.source_cue.source_class.as_str().to_owned(),
            auth_mode: self.source_cue.auth_mode.as_str().to_owned(),
            freshness: self.source_cue.freshness.as_str().to_owned(),
            revocation: self.source_cue.revocation.as_str().to_owned(),
            mirror_owner: self.source_cue.mirror_owner.clone(),
            message_class: self.source_cue.message_class().as_str().to_owned(),
            trust_blocked: self.source_cue.trust_blocked(),
            must_disclose: self.source_cue.must_disclose(),
            redacted_source_label: self.source_cue.redacted_source_label.clone(),
        }
    }

    /// Projects a redaction-safe export row reused by support/export packets and
    /// the CLI inspect surface.
    pub fn export_row(&self) -> ScopeMutationExportRow {
        ScopeMutationExportRow {
            row_id: self.row_id.clone(),
            package_name: self.requested.package_name.clone(),
            ecosystem: self.requested.ecosystem.as_str().to_owned(),
            requested_manifest_id: self.requested_scope.manifest_id.clone(),
            requested_role: self.requested_scope.role.as_str().to_owned(),
            resolved_manifest_id: self.resolved_scope.manifest_id.clone(),
            resolved_role: self.resolved_scope.role.as_str().to_owned(),
            fidelity: self.fidelity.as_str().to_owned(),
            broadened: self.broadens_beyond_target(),
            broadening_confirmed: self.broadening_confirmed,
            silent_broadening: self.is_silent_broadening(),
            requested_ref: self.requested.requested_ref.clone(),
            resolved_ref: self.resolved.as_ref().map(|r| r.resolved_ref.clone()),
            source_class: self.source_cue.source_class.as_str().to_owned(),
            source_message_class: self.source_cue.message_class().as_str().to_owned(),
            mirror_owner: self.source_cue.mirror_owner.clone(),
            freshness: self.source_cue.freshness.as_str().to_owned(),
            revocation: self.source_cue.revocation.as_str().to_owned(),
            trust_blocked: self.source_cue.trust_blocked(),
            can_apply: self.can_apply(),
            continuity_preserved: self.continuity_preserved(),
            redacted_source_label: self.source_cue.redacted_source_label.clone(),
        }
    }

    /// Projects the row onto a specific marketed surface, pinning the write
    /// authority that surface may carry from the frozen matrix.
    pub fn surface_projection(&self, surface: PackageSurface) -> ScopeMutationSurfaceProjection {
        let authority = surface.canonical_write_authority();
        ScopeMutationSurfaceProjection {
            surface: surface.as_str().to_owned(),
            write_authority: authority.as_str().to_owned(),
            // A mutation only proceeds where the surface can mutate AND the row
            // is appliable; review and inspect surfaces never carry apply rights.
            can_apply_here: authority.can_mutate() && self.can_apply(),
            redacted: matches!(authority, SurfaceWriteAuthority::RedactedExport),
            view: self.view(),
        }
    }

    /// Whether the row is internally consistent against the contract.
    pub fn is_consistent(&self) -> bool {
        self.requested_scope.is_consistent()
            && self.resolved_scope.is_consistent()
            && self.source_cue.is_consistent()
            && self.requested_and_resolved_separate()
            && self.scope_identity_aligned()
            && self.continuity_preserved()
            && self.fidelity_matches_scope()
            && self.source_cue.message_class().is_specific()
            // A silent broadening must never be appliable.
            && !(self.is_silent_broadening() && self.can_apply())
    }

    /// Whether the declared fidelity agrees with the affected manifest set and
    /// confirmation state.
    pub fn fidelity_matches_scope(&self) -> bool {
        let broadened = self.broadens_beyond_target();
        match self.fidelity {
            // Exact means nothing beyond the target changes and the resolved
            // selector is the same durable identity as the requested one.
            ScopeFidelity::Exact => {
                !broadened && self.resolved_scope.same_identity(&self.requested_scope)
            }
            // A disclosed shared lockfile broadens to a member's parent root and
            // needs no confirmation; the target stays the requested member.
            ScopeFidelity::DisclosedSharedLockfile => {
                broadened && self.requested_scope.targets_member() && !self.broadening_confirmed
            }
            // A confirmed whole-workspace operation must be confirmed and resolve
            // to the whole-workspace scope.
            ScopeFidelity::ConfirmedWorkspaceWide => {
                broadened
                    && self.broadening_confirmed
                    && self.resolved_scope.scope_class == ManifestScopeClass::WholeWorkspace
            }
            // An unconfirmed broadening broadens without confirmation.
            ScopeFidelity::UnconfirmedBroadening => broadened && !self.broadening_confirmed,
        }
    }
}

/// The root-versus-member diff between a requested target and the resolved scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDiffView {
    /// Row id.
    pub row_id: String,
    /// Requested target manifest id.
    pub requested_manifest_id: String,
    /// Requested manifest role token.
    pub requested_role: String,
    /// Requested scope class token.
    pub requested_scope_class: String,
    /// Resolved effective manifest id.
    pub resolved_manifest_id: String,
    /// Resolved manifest role token.
    pub resolved_role: String,
    /// Resolved scope class token.
    pub resolved_scope_class: String,
    /// Parent (root) manifest id of the requested target, if a member.
    pub parent_manifest_id: Option<String>,
    /// Whether the role changed between requested and resolved.
    pub role_changed: bool,
    /// Whether the scope class changed between requested and resolved.
    pub scope_class_changed: bool,
    /// Whether the resolved scope reaches beyond the requested target.
    pub broadened: bool,
    /// Whether a broadening was explicitly confirmed.
    pub broadening_confirmed: bool,
    /// Whether the row is a silent (unconfirmed) broadening.
    pub silent_broadening: bool,
    /// Every affected manifest id.
    pub affected_manifest_ids: Vec<String>,
    /// Scope fidelity token.
    pub fidelity: String,
}

/// A redaction-safe view of the registry-source cue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrySourceCueView {
    /// Source class token.
    pub source_class: String,
    /// Auth mode token.
    pub auth_mode: String,
    /// Freshness token.
    pub freshness: String,
    /// Revocation token.
    pub revocation: String,
    /// Redacted mirror or private-registry owner, if any.
    pub mirror_owner: Option<String>,
    /// Specific source-disclosure message-class token.
    pub message_class: String,
    /// Whether trust is blocked right now.
    pub trust_blocked: bool,
    /// Whether the source must disclose its environment.
    pub must_disclose: bool,
    /// Redacted source label.
    pub redacted_source_label: String,
}

/// The canonical per-row view reused by desktop, review, and AI inspect surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeMutationView {
    /// Row id.
    pub row_id: String,
    /// Package name.
    pub package_name: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Root-versus-member scope diff.
    pub scope_diff: ScopeDiffView,
    /// Requested range, path, or redacted VCS ref.
    pub requested_ref: String,
    /// Exact resolved ref, if resolved.
    pub resolved_ref: Option<String>,
    /// Dependency relation token, if resolved.
    pub relation: Option<String>,
    /// Whether requested and resolved identity stay separate.
    pub requested_resolved_separate: bool,
    /// Whether the package's requested manifest scope aligns with the selector.
    pub scope_identity_aligned: bool,
    /// Registry-source cue view.
    pub source: RegistrySourceCueView,
    /// Whether the operation may be applied.
    pub can_apply: bool,
    /// Whether the row has been applied.
    pub is_applied: bool,
    /// Whether durable scope identity was preserved across apply.
    pub continuity_preserved: bool,
    /// Every applicable package-state label token.
    pub applicable_labels: Vec<String>,
}

/// A redaction-safe export row reused by support/export packets and CLI inspect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeMutationExportRow {
    /// Row id.
    pub row_id: String,
    /// Package name.
    pub package_name: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Requested target manifest id.
    pub requested_manifest_id: String,
    /// Requested manifest role token.
    pub requested_role: String,
    /// Resolved effective manifest id.
    pub resolved_manifest_id: String,
    /// Resolved manifest role token.
    pub resolved_role: String,
    /// Scope fidelity token.
    pub fidelity: String,
    /// Whether the resolved scope reaches beyond the target.
    pub broadened: bool,
    /// Whether a broadening was explicitly confirmed.
    pub broadening_confirmed: bool,
    /// Whether the row is a silent (unconfirmed) broadening.
    pub silent_broadening: bool,
    /// Requested ref.
    pub requested_ref: String,
    /// Resolved ref, if resolved.
    pub resolved_ref: Option<String>,
    /// Source class token.
    pub source_class: String,
    /// Specific source-disclosure message-class token.
    pub source_message_class: String,
    /// Redacted mirror or private-registry owner, if any.
    pub mirror_owner: Option<String>,
    /// Freshness token.
    pub freshness: String,
    /// Revocation token.
    pub revocation: String,
    /// Whether trust is blocked right now.
    pub trust_blocked: bool,
    /// Whether the operation may be applied.
    pub can_apply: bool,
    /// Whether durable scope identity was preserved across apply.
    pub continuity_preserved: bool,
    /// Redacted source label.
    pub redacted_source_label: String,
}

/// A row projected onto a specific marketed surface with its pinned write
/// authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeMutationSurfaceProjection {
    /// Package surface token.
    pub surface: String,
    /// Write authority token pinned by the frozen matrix.
    pub write_authority: String,
    /// Whether the operation may be applied from this surface.
    pub can_apply_here: bool,
    /// Whether the surface produces a redacted export.
    pub redacted: bool,
    /// The canonical per-row view.
    pub view: ScopeMutationView,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestScopeReviewSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Rows that target the workspace root.
    pub root_targeted_rows: usize,
    /// Rows that target a workspace member.
    pub member_targeted_rows: usize,
    /// Rows whose resolved scope reaches beyond the target.
    pub broadened_rows: usize,
    /// Rows with an explicitly confirmed broadening.
    pub confirmed_broadening_rows: usize,
    /// Rows that are silent (unconfirmed) broadenings.
    pub silent_broadening_rows: usize,
    /// Rows whose source trust is blocked.
    pub trust_blocked_rows: usize,
    /// Rows that may be applied.
    pub appliable_rows: usize,
    /// Rows that have been applied.
    pub applied_rows: usize,
    /// Rows that carry a resolved dependency identity.
    pub resolved_identity_rows: usize,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Matrix id every row binds to.
    pub references_matrix_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected export rows.
    pub rows: Vec<ScopeMutationExportRow>,
    /// Whether every row is consistent with the contract.
    pub all_consistent: bool,
    /// Whether no silent broadening is appliable.
    pub no_appliable_silent_broadening: bool,
    /// Whether requested and resolved identity stay separate everywhere.
    pub requested_resolved_separate: bool,
    /// Whether no source cue renders a generic collapse message.
    pub no_generic_collapse: bool,
    /// Whether every row binds to the frozen matrix.
    pub all_bind_matrix: bool,
}

/// The typed manifest-scope review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestScopeReview {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The frozen matrix packet id every row binds to.
    pub references_matrix_id: String,
    /// Closed manifest-role vocabulary represented by this packet.
    pub manifest_roles: Vec<ManifestRole>,
    /// Closed scope-fidelity vocabulary represented by this packet.
    pub scope_fidelities: Vec<ScopeFidelity>,
    /// Closed source-freshness vocabulary represented by this packet.
    pub source_freshness_states: Vec<SourceFreshness>,
    /// Closed revocation-state vocabulary represented by this packet.
    pub revocation_states: Vec<RevocationState>,
    /// The scope mutation rows.
    #[serde(default)]
    pub rows: Vec<ScopeMutationRow>,
    /// Summary counts.
    pub summary: ManifestScopeReviewSummary,
}

impl ManifestScopeReview {
    /// Returns the row with the given id.
    pub fn row(&self, row_id: &str) -> Option<&ScopeMutationRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Whether every row is consistent with the contract.
    pub fn all_consistent(&self) -> bool {
        self.rows.iter().all(ScopeMutationRow::is_consistent)
    }

    /// Whether no silent broadening is appliable anywhere.
    pub fn no_appliable_silent_broadening(&self) -> bool {
        self.rows
            .iter()
            .all(|r| !(r.is_silent_broadening() && r.can_apply()))
    }

    /// Whether requested and resolved identity stay separate in every row.
    pub fn requested_resolved_separate(&self) -> bool {
        self.rows
            .iter()
            .all(ScopeMutationRow::requested_and_resolved_separate)
    }

    /// Whether no row's source cue renders a generic collapse message.
    pub fn no_generic_collapse(&self) -> bool {
        self.rows
            .iter()
            .all(|r| r.source_cue.message_class().is_specific())
    }

    /// Whether every label every row surfaces resolves to a frozen state row,
    /// proving the packet binds to the shared matrix.
    pub fn all_bind_matrix(&self) -> bool {
        let Ok(matrix) = current_m5_package_state_matrix() else {
            return false;
        };
        if self.references_matrix_id != matrix.packet_id {
            return false;
        }
        self.rows.iter().all(|r| {
            r.applicable_labels()
                .iter()
                .all(|label| matrix.state(*label).is_some())
        })
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> ManifestScopeReviewSummary {
        let count =
            |pred: &dyn Fn(&ScopeMutationRow) -> bool| self.rows.iter().filter(|r| pred(r)).count();
        ManifestScopeReviewSummary {
            total_rows: self.rows.len(),
            root_targeted_rows: count(&ScopeMutationRow::targets_root),
            member_targeted_rows: count(&ScopeMutationRow::targets_member),
            broadened_rows: count(&ScopeMutationRow::broadens_beyond_target),
            confirmed_broadening_rows: count(&|r| {
                r.broadens_beyond_target() && r.broadening_confirmed
            }),
            silent_broadening_rows: count(&ScopeMutationRow::is_silent_broadening),
            trust_blocked_rows: count(&|r| r.source_cue.trust_blocked()),
            appliable_rows: count(&ScopeMutationRow::can_apply),
            applied_rows: count(&ScopeMutationRow::is_applied),
            resolved_identity_rows: count(&|r| r.resolved.is_some()),
        }
    }

    /// Produces a redaction-safe export projection that downstream surfaces —
    /// support exports, the CLI inspect surface, and release/public-truth — render
    /// instead of restating scope and source state by hand.
    pub fn export_projection(&self) -> ManifestScopeReviewExportProjection {
        ManifestScopeReviewExportProjection {
            packet_id: self.packet_id.clone(),
            references_matrix_id: self.references_matrix_id.clone(),
            as_of: self.as_of.clone(),
            rows: self.rows.iter().map(ScopeMutationRow::export_row).collect(),
            all_consistent: self.all_consistent(),
            no_appliable_silent_broadening: self.no_appliable_silent_broadening(),
            requested_resolved_separate: self.requested_resolved_separate(),
            no_generic_collapse: self.no_generic_collapse(),
            all_bind_matrix: self.all_bind_matrix(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ManifestScopeReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rows(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ManifestScopeReviewViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ManifestScopeReviewViolation>) {
        if self.schema_version != MANIFEST_SCOPE_REVIEW_SCHEMA_VERSION {
            violations.push(ManifestScopeReviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != MANIFEST_SCOPE_REVIEW_RECORD_KIND {
            violations.push(ManifestScopeReviewViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("references_matrix_id", &self.references_matrix_id),
        ] {
            if value.trim().is_empty() {
                violations.push(ManifestScopeReviewViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "manifest_roles",
                self.manifest_roles == ManifestRole::ALL.to_vec(),
            ),
            (
                "scope_fidelities",
                self.scope_fidelities == ScopeFidelity::ALL.to_vec(),
            ),
            (
                "source_freshness_states",
                self.source_freshness_states == SourceFreshness::ALL.to_vec(),
            ),
            (
                "revocation_states",
                self.revocation_states == RevocationState::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(ManifestScopeReviewViolation::ClosedVocabularyMismatch { field });
            }
        }
        match current_m5_package_state_matrix() {
            Ok(matrix) => {
                if self.references_matrix_id != matrix.packet_id {
                    violations.push(ManifestScopeReviewViolation::MatrixBindingMismatch {
                        referenced: self.references_matrix_id.clone(),
                        expected: matrix.packet_id,
                    });
                }
            }
            Err(_) => violations.push(ManifestScopeReviewViolation::MatrixUnavailable),
        }
    }

    fn validate_rows(&self, violations: &mut Vec<ManifestScopeReviewViolation>) {
        let matrix = current_m5_package_state_matrix().ok();
        let mut seen = BTreeSet::new();
        for row in &self.rows {
            let id = row.row_id.clone();
            if !seen.insert(id.clone()) {
                violations
                    .push(ManifestScopeReviewViolation::DuplicateRowId { row_id: id.clone() });
            }

            for (field, value) in [
                ("row_id", &row.row_id),
                (
                    "requested_scope.manifest_id",
                    &row.requested_scope.manifest_id,
                ),
                (
                    "requested_scope.display_label",
                    &row.requested_scope.display_label,
                ),
                (
                    "requested_scope.continuity_token",
                    &row.requested_scope.continuity_token,
                ),
                (
                    "resolved_scope.manifest_id",
                    &row.resolved_scope.manifest_id,
                ),
                ("requested.package_name", &row.requested.package_name),
                ("requested.requested_ref", &row.requested.requested_ref),
                (
                    "source_cue.redacted_source_label",
                    &row.source_cue.redacted_source_label,
                ),
                ("note", &row.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(ManifestScopeReviewViolation::EmptyField {
                        id: id.clone(),
                        field_name: field,
                    });
                }
            }

            // Selectors must be internally consistent (role permits scope, parent
            // linkage matches role).
            for (which, selector) in [
                ("requested_scope", &row.requested_scope),
                ("resolved_scope", &row.resolved_scope),
            ] {
                if !selector.is_consistent() {
                    violations.push(ManifestScopeReviewViolation::SelectorInconsistent {
                        row_id: id.clone(),
                        which,
                    });
                }
            }

            // The package's requested manifest scope must agree with the selector.
            if !row.scope_identity_aligned() {
                violations.push(ManifestScopeReviewViolation::ScopeIdentityMismatch {
                    row_id: id.clone(),
                });
            }

            // The declared fidelity must agree with the affected set/confirmation.
            if !row.fidelity_matches_scope() {
                violations.push(ManifestScopeReviewViolation::FidelityMismatch {
                    row_id: id.clone(),
                    fidelity: row.fidelity.as_str(),
                });
            }

            // A silent broadening must never be appliable.
            if row.is_silent_broadening() && row.can_apply() {
                violations.push(ManifestScopeReviewViolation::SilentBroadeningAppliable {
                    row_id: id.clone(),
                });
            }

            // Requested and resolved identity must stay separate.
            if !row.requested_and_resolved_separate() {
                violations.push(ManifestScopeReviewViolation::RequestedResolvedConflated {
                    row_id: id.clone(),
                });
            }

            // The affected set must include the requested target manifest.
            if !row
                .affected_manifest_ids
                .contains(&row.requested_scope.manifest_id)
            {
                violations.push(ManifestScopeReviewViolation::AffectedSetMissingTarget {
                    row_id: id.clone(),
                });
            }

            // The mirror owner must be present exactly for a private/mirror source.
            if !row.source_cue.is_consistent() {
                violations.push(ManifestScopeReviewViolation::SourceCueInconsistent {
                    row_id: id.clone(),
                    source_class: row.source_cue.source_class.as_str(),
                });
            }

            // The source message must never collapse into a generic message.
            if row.source_cue.message_class().is_generic_collapse() {
                violations.push(ManifestScopeReviewViolation::GenericCollapseMessage {
                    row_id: id.clone(),
                    message: row.source_cue.message_class().as_str(),
                });
            }

            // Post-apply continuity must be preserved.
            if !row.continuity_preserved() {
                violations
                    .push(ManifestScopeReviewViolation::ContinuityBroken { row_id: id.clone() });
            }

            // No redacted field may leak a raw URL or scheme.
            for (field, value) in [
                (
                    "requested_scope.redacted_manifest_path",
                    &row.requested_scope.redacted_manifest_path,
                ),
                (
                    "resolved_scope.redacted_manifest_path",
                    &row.resolved_scope.redacted_manifest_path,
                ),
                ("requested.requested_ref", &row.requested.requested_ref),
                (
                    "source_cue.redacted_source_label",
                    &row.source_cue.redacted_source_label,
                ),
            ] {
                if leaks_raw_url(value) {
                    violations.push(ManifestScopeReviewViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: field,
                    });
                }
            }
            if let Some(owner) = &row.source_cue.mirror_owner {
                if leaks_raw_url(owner) {
                    violations.push(ManifestScopeReviewViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: "source_cue.mirror_owner",
                    });
                }
            }
            if let Some(resolved) = &row.resolved {
                if leaks_raw_url(&resolved.resolved_ref) {
                    violations.push(ManifestScopeReviewViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: "resolved.resolved_ref",
                    });
                }
            }

            // Every surfaced label must bind to a frozen state row.
            if let Some(matrix) = &matrix {
                for label in row.applicable_labels() {
                    if matrix.state(label).is_none() {
                        violations.push(ManifestScopeReviewViolation::UnboundLabel {
                            row_id: id.clone(),
                            label: label.as_str(),
                        });
                    }
                }
            }
        }
    }
}

/// Whether a string leaks a raw URL or scheme that must be redacted.
fn leaks_raw_url(value: &str) -> bool {
    value.contains("://")
}

/// A validation violation for the manifest-scope review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestScopeReviewViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate id.
        row_id: String,
    },
    /// The frozen matrix could not be loaded for binding validation.
    MatrixUnavailable,
    /// The packet references a matrix id other than the frozen matrix.
    MatrixBindingMismatch {
        /// Referenced matrix id.
        referenced: String,
        /// Expected (frozen) matrix id.
        expected: String,
    },
    /// A selector's role does not permit its scope or its parent linkage is wrong.
    SelectorInconsistent {
        /// Row id.
        row_id: String,
        /// Which selector (requested or resolved).
        which: &'static str,
    },
    /// The package's requested manifest scope disagrees with the selector.
    ScopeIdentityMismatch {
        /// Row id.
        row_id: String,
    },
    /// The declared fidelity disagrees with the affected set or confirmation.
    FidelityMismatch {
        /// Row id.
        row_id: String,
        /// Fidelity token.
        fidelity: &'static str,
    },
    /// A silent (unconfirmed) broadening is appliable.
    SilentBroadeningAppliable {
        /// Row id.
        row_id: String,
    },
    /// A row flattens requested and resolved identity into one label.
    RequestedResolvedConflated {
        /// Row id.
        row_id: String,
    },
    /// The affected manifest set omits the requested target.
    AffectedSetMissingTarget {
        /// Row id.
        row_id: String,
    },
    /// The mirror owner is present without a private/mirror source, or absent
    /// with one.
    SourceCueInconsistent {
        /// Row id.
        row_id: String,
        /// Source class token.
        source_class: &'static str,
    },
    /// A source cue renders a forbidden generic collapse message.
    GenericCollapseMessage {
        /// Row id.
        row_id: String,
        /// Generic message-class token.
        message: &'static str,
    },
    /// Durable scope identity was not preserved across apply.
    ContinuityBroken {
        /// Row id.
        row_id: String,
    },
    /// A field leaks a raw URL that must be redacted.
    RawUrlLeak {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A surfaced label does not bind to a frozen state row.
    UnboundLabel {
        /// Row id.
        row_id: String,
        /// Label token.
        label: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for ManifestScopeReviewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical vocabulary")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::MatrixUnavailable => {
                write!(f, "the frozen package-state matrix could not be loaded")
            }
            Self::MatrixBindingMismatch {
                referenced,
                expected,
            } => write!(
                f,
                "packet references matrix id {referenced} instead of the frozen {expected}"
            ),
            Self::SelectorInconsistent { row_id, which } => write!(
                f,
                "row {row_id} {which} selector role/scope/parent are inconsistent"
            ),
            Self::ScopeIdentityMismatch { row_id } => write!(
                f,
                "row {row_id} package requested manifest scope disagrees with the selector"
            ),
            Self::FidelityMismatch { row_id, fidelity } => write!(
                f,
                "row {row_id} fidelity {fidelity} disagrees with the affected set or confirmation"
            ),
            Self::SilentBroadeningAppliable { row_id } => write!(
                f,
                "row {row_id} is a silent broadening yet is marked appliable"
            ),
            Self::RequestedResolvedConflated { row_id } => {
                write!(f, "row {row_id} flattens requested and resolved identity")
            }
            Self::AffectedSetMissingTarget { row_id } => write!(
                f,
                "row {row_id} affected manifest set omits the requested target"
            ),
            Self::SourceCueInconsistent {
                row_id,
                source_class,
            } => write!(
                f,
                "row {row_id} source cue for {source_class} has an inconsistent mirror owner"
            ),
            Self::GenericCollapseMessage { row_id, message } => write!(
                f,
                "row {row_id} source cue renders forbidden generic message {message}"
            ),
            Self::ContinuityBroken { row_id } => write!(
                f,
                "row {row_id} did not preserve durable scope identity across apply"
            ),
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::UnboundLabel { row_id, label } => write!(
                f,
                "row {row_id} surfaces label {label} with no frozen state row"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for ManifestScopeReviewViolation {}

/// Loads the embedded manifest-scope review packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ManifestScopeReview`].
pub fn current_manifest_scope_review() -> Result<ManifestScopeReview, serde_json::Error> {
    serde_json::from_str(MANIFEST_SCOPE_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
