//! Manifest-scope switchers and registry-or-mirror rows carrying
//! root/member/module/tool scope, active manifest label, lockfile coupling,
//! auth mode, freshness/reachability, policy pinning, and offline/cache-only
//! continuity.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`] — the
//! `manifest_scope_switcher` and the `registry_or_mirror_row` — into one
//! implemented, export-safe packet with two co-equal control vectors. Together
//! they make the target manifest and the source registry explicit *before* any
//! dependency state changes.
//!
//! A [`ManifestScopeSwitcher`] always names the active manifest label and the
//! scope that owns it (root, member package, module, or tool manifest), and its
//! change-scope truth is derived from the target scope and the lockfile
//! coupling rather than asserted: a member change against a shared root lockfile
//! can never read as an isolated member change, so monorepo and multi-root
//! flows never guess which manifest they are about to touch.
//!
//! A [`RegistryOrMirrorRow`] always names where metadata and artifacts come
//! from (public default, enterprise mirror, self-hosted, offline cache, or a
//! policy-pinned source), the auth mode, the freshness/reachability state, and
//! whether the row is offline/cache-only. Offline-cache-only continuity and
//! policy pinning are derived from the source class and reachability, so an
//! offline or pinned answer never presents as a clean, live upstream read and
//! inherited registry state is never hidden.
//!
//! The registry/resolution degradation vocabulary
//! ([`M5PackageComponentDegradationState`]) and rollback posture
//! ([`M5PackageComponentRollbackPosture`]) are reused directly from the frozen
//! matrix, as are the downgrade triggers
//! ([`M5PackageComponentDowngradeTrigger`]) and consumer surfaces
//! ([`M5PackageComponentConsumerSurface`]).
//!
//! Raw manifest bodies, raw lockfile bodies, registry credentials, private
//! registry URLs, and live registry responses stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-manifest-scope-registry-controls.schema.json`](../../../../schemas/ui/m5-manifest-scope-registry-controls.schema.json).
//! The contract doc is
//! [`docs/deps/m5/implement_manifest_scope_switchers_and_registry_or_mirror_rows.md`](../../../../docs/deps/m5/implement_manifest_scope_switchers_and_registry_or_mirror_rows.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-manifest-scope-registry-controls/`](../../../../fixtures/ui/m5-manifest-scope-registry-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::{
    M5PackageComponent, M5PackageComponentConsumerSurface, M5PackageComponentDegradationState,
    M5PackageComponentDowngradeTrigger, M5PackageComponentRollbackPosture,
    M5_PACKAGE_COMPONENT_MATRIX_DOC_REF, M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`ManifestScopeRegistryControlsPacket`].
pub const MANIFEST_SCOPE_REGISTRY_RECORD_KIND: &str = "manifest_scope_registry_controls";

/// Schema version for manifest-scope / registry-or-mirror control records.
pub const MANIFEST_SCOPE_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MANIFEST_SCOPE_REGISTRY_SCHEMA_REF: &str =
    "schemas/ui/m5-manifest-scope-registry-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const MANIFEST_SCOPE_REGISTRY_DOC_REF: &str =
    "docs/deps/m5/implement_manifest_scope_switchers_and_registry_or_mirror_rows.md";

/// Repo-relative path of the protected fixture directory.
pub const MANIFEST_SCOPE_REGISTRY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-manifest-scope-registry-controls";

/// Repo-relative path of the checked support-export artifact.
pub const MANIFEST_SCOPE_REGISTRY_ARTIFACT_REF: &str =
    "artifacts/release/m5-manifest-scope-registry-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MANIFEST_SCOPE_REGISTRY_SUMMARY_REF: &str =
    "artifacts/release/m5-manifest-scope-registry-proof/summary.md";

/// Manifest scope a switcher targets.
///
/// These scopes must stay visually distinct and copy/export safe: a root
/// manifest governs the whole workspace, a member package owns one workspace
/// member, a module manifest owns a nested module, and a tool manifest governs
/// only a tool/toolchain rather than runtime dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestTargetScope {
    /// The workspace root manifest.
    RootManifest,
    /// A workspace member package manifest.
    MemberPackage,
    /// A nested module manifest below a member.
    ModuleManifest,
    /// A tool / toolchain manifest.
    ToolManifest,
}

impl ManifestTargetScope {
    /// Every scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RootManifest,
        Self::MemberPackage,
        Self::ModuleManifest,
        Self::ToolManifest,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootManifest => "root_manifest",
            Self::MemberPackage => "member_package",
            Self::ModuleManifest => "module_manifest",
            Self::ToolManifest => "tool_manifest",
        }
    }

    /// Whether this scope sits below the workspace root, so the switcher must
    /// name which member/module manifest it targets rather than guess.
    pub const fn is_below_root(self) -> bool {
        matches!(self, Self::MemberPackage | Self::ModuleManifest)
    }
}

/// How the target manifest's changes couple to a lockfile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestLockfileCoupling {
    /// One lockfile at the root governs every member; a member change
    /// regenerates the shared root lockfile.
    SharedRootLockfile,
    /// The member owns its own lockfile; a change stays scoped to it.
    MemberScopedLockfile,
    /// No lockfile applies to this manifest (e.g. a tool manifest).
    NoLockfileCoupling,
}

impl ManifestLockfileCoupling {
    /// Every coupling, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SharedRootLockfile,
        Self::MemberScopedLockfile,
        Self::NoLockfileCoupling,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedRootLockfile => "shared_root_lockfile",
            Self::MemberScopedLockfile => "member_scoped_lockfile",
            Self::NoLockfileCoupling => "no_lockfile_coupling",
        }
    }

    /// Whether the switcher must carry an explicit lockfile-coupling note.
    pub const fn needs_coupling_note(self) -> bool {
        !matches!(self, Self::NoLockfileCoupling)
    }

    /// Whether a change through this coupling regenerates the shared root lockfile.
    pub const fn writes_shared_root_lockfile(self) -> bool {
        matches!(self, Self::SharedRootLockfile)
    }
}

/// Derived change-scope class a manifest-scope switcher may present.
///
/// This is the switcher honesty axis: the change scope is derived from the
/// target scope and the lockfile coupling, never asserted, so a member change
/// that regenerates the shared root lockfile can never present as an isolated
/// member-scoped change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestChangeScopeClass {
    /// A root-scoped change affecting the whole workspace.
    RootWideChange,
    /// A member/module change scoped to its own lockfile.
    MemberScopedChange,
    /// A member/module change that regenerates the shared root lockfile.
    MemberChangeSharedLock,
    /// A tool-manifest change that does not touch runtime dependencies.
    ToolManifestChange,
}

impl ManifestChangeScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootWideChange => "root_wide_change",
            Self::MemberScopedChange => "member_scoped_change",
            Self::MemberChangeSharedLock => "member_change_shared_lock",
            Self::ToolManifestChange => "tool_manifest_change",
        }
    }
}

/// Disclosures a manifest-scope switcher must carry, derived from scope and coupling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestChangeScopeDisclosure {
    /// The derived change-scope class this switcher may present.
    pub change_scope_class: ManifestChangeScopeClass,
    /// Whether the switcher must name which member/module manifest it targets.
    pub needs_member_selection_note: bool,
    /// Whether the switcher must carry an explicit lockfile-coupling note.
    pub needs_lockfile_coupling_note: bool,
    /// Whether the change regenerates the shared root lockfile.
    pub affects_shared_root_lockfile: bool,
}

/// Resolves the change-scope truth a manifest-scope switcher may present.
///
/// A root manifest is a root-wide change. A tool manifest is a tool-only
/// change. A member or module manifest is scoped to its own lockfile unless a
/// shared root lockfile couples it, in which case the change regenerates the
/// root lockfile and must say so.
pub fn resolve_manifest_change_scope(
    target_scope: ManifestTargetScope,
    lockfile_coupling: ManifestLockfileCoupling,
) -> ManifestChangeScopeDisclosure {
    let change_scope_class = match target_scope {
        ManifestTargetScope::RootManifest => ManifestChangeScopeClass::RootWideChange,
        ManifestTargetScope::ToolManifest => ManifestChangeScopeClass::ToolManifestChange,
        ManifestTargetScope::MemberPackage | ManifestTargetScope::ModuleManifest => {
            if lockfile_coupling.writes_shared_root_lockfile() {
                ManifestChangeScopeClass::MemberChangeSharedLock
            } else {
                ManifestChangeScopeClass::MemberScopedChange
            }
        }
    };

    ManifestChangeScopeDisclosure {
        change_scope_class,
        needs_member_selection_note: target_scope.is_below_root(),
        needs_lockfile_coupling_note: lockfile_coupling.needs_coupling_note(),
        affects_shared_root_lockfile: lockfile_coupling.writes_shared_root_lockfile(),
    }
}

/// A manifest-scope switcher naming the active manifest, scope, and change scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeSwitcher {
    /// Frozen component this control implements; must be `manifest_scope_switcher`.
    pub component: M5PackageComponent,
    /// Stable switcher id.
    pub switcher_id: String,
    /// Active manifest label; always required and non-empty (no guessing).
    pub active_manifest_label: String,
    /// Target manifest scope.
    pub target_scope: ManifestTargetScope,
    /// Scope disclosure naming the owning manifest; required and non-empty.
    pub scope_disclosure: String,
    /// Member/module selection note; required when the scope is below root.
    pub member_selection_note: String,
    /// Lockfile coupling class.
    pub lockfile_coupling: ManifestLockfileCoupling,
    /// Lockfile-coupling note; required when a lockfile couples the change.
    pub lockfile_coupling_note: String,
    /// Whether the change regenerates the shared root lockfile.
    pub affects_root_lockfile: bool,
    /// Change-scope review action label; required and non-empty.
    pub change_scope_action_label: String,
    /// Change-scope review note; free text.
    pub change_scope_review_note: String,
    /// Rollback / write-back posture, reused from the frozen matrix.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this switcher.
    pub source_contract_refs: Vec<String>,
}

impl ManifestScopeSwitcher {
    /// Change-scope disclosures this switcher must carry, derived from scope and coupling.
    pub fn change_scope_disclosure(&self) -> ManifestChangeScopeDisclosure {
        resolve_manifest_change_scope(self.target_scope, self.lockfile_coupling)
    }

    /// Whether the rollback posture is consistent with a scope-selection control.
    ///
    /// A switcher selects a scope and stages a change-scope review; it never
    /// writes back directly, so it must be read-only or staged-review.
    pub fn rollback_posture_consistent(&self) -> bool {
        matches!(
            self.rollback_posture,
            M5PackageComponentRollbackPosture::ReadOnlyNoMutation
                | M5PackageComponentRollbackPosture::StagedReviewNoWrite
        )
    }
}

/// Registry / mirror source class that answered for a package.
///
/// These are the five source families a user must be able to tell apart:
/// public default, enterprise mirror, self-hosted, offline cache, and a
/// policy-pinned source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryMirrorSourceClass {
    /// The public default registry for the ecosystem.
    PublicDefault,
    /// An enterprise mirror standing in for the upstream registry.
    EnterpriseMirror,
    /// A self-hosted / private registry.
    SelfHosted,
    /// An offline snapshot or local cache.
    OfflineCache,
    /// A source held by policy pinning.
    PolicyPinnedSource,
}

impl RegistryMirrorSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicDefault,
        Self::EnterpriseMirror,
        Self::SelfHosted,
        Self::OfflineCache,
        Self::PolicyPinnedSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicDefault => "public_default",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::SelfHosted => "self_hosted",
            Self::OfflineCache => "offline_cache",
            Self::PolicyPinnedSource => "policy_pinned_source",
        }
    }

    /// Whether this source class implies the row is policy-pinned.
    pub const fn implies_policy_pin(self) -> bool {
        matches!(self, Self::PolicyPinnedSource)
    }
}

/// Auth mode a registry / mirror row uses.
///
/// Only the mode is recorded; no credential material ever crosses this boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryAuthMode {
    /// Anonymous access to a public source; no auth required.
    AnonymousPublic,
    /// Token-authenticated access.
    TokenAuthenticated,
    /// Single-sign-on session access.
    SsoSession,
    /// Client-certificate (mutual TLS) access.
    ClientCertificate,
}

impl RegistryAuthMode {
    /// Every auth mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AnonymousPublic,
        Self::TokenAuthenticated,
        Self::SsoSession,
        Self::ClientCertificate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnonymousPublic => "anonymous_public",
            Self::TokenAuthenticated => "token_authenticated",
            Self::SsoSession => "sso_session",
            Self::ClientCertificate => "client_certificate",
        }
    }

    /// Whether the row must carry an explicit auth-mode disclosure.
    pub const fn needs_auth_disclosure(self) -> bool {
        !matches!(self, Self::AnonymousPublic)
    }
}

/// Freshness / reachability state of a registry / mirror row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryReachabilityState {
    /// Fresh and reachable from the named source.
    FreshReachable,
    /// Reachable but the answer is a stale cached read.
    StaleCached,
    /// Only an offline cache is available; the source was not reached.
    OfflineCacheOnly,
    /// The source is unreachable.
    Unreachable,
}

impl RegistryReachabilityState {
    /// Every reachability state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FreshReachable,
        Self::StaleCached,
        Self::OfflineCacheOnly,
        Self::Unreachable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshReachable => "fresh_reachable",
            Self::StaleCached => "stale_cached",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::Unreachable => "unreachable",
        }
    }

    /// Whether the row must carry an explicit freshness/reachability note.
    pub const fn needs_reachability_note(self) -> bool {
        !matches!(self, Self::FreshReachable)
    }

    /// Whether this reachability state forces offline/cache-only continuity.
    pub const fn is_offline_cache_only(self) -> bool {
        matches!(self, Self::OfflineCacheOnly | Self::Unreachable)
    }
}

/// Disclosures a registry / mirror row must carry, derived from source and reachability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegistryMirrorDisclosure {
    /// Whether the row is offline/cache-only and must say so.
    pub is_offline_cache_only: bool,
    /// Whether the row must carry an offline/cache-only continuity note.
    pub needs_offline_continuity_note: bool,
    /// Whether the row must carry a freshness/reachability note.
    pub needs_reachability_note: bool,
    /// Whether the row must carry a policy-pin note.
    pub needs_policy_pin_note: bool,
    /// Whether the source class itself implies the row is policy-pinned.
    pub source_implies_policy_pin: bool,
}

/// Resolves the source/continuity truth a registry / mirror row may present.
///
/// A row is offline/cache-only when its source is an offline cache or its
/// source is unreachable / cache-only. Policy pinning is implied by a
/// policy-pinned source class and may also be set explicitly.
pub fn resolve_registry_or_mirror_disclosure(
    source_class: RegistryMirrorSourceClass,
    reachability: RegistryReachabilityState,
    is_policy_pinned: bool,
) -> RegistryMirrorDisclosure {
    let is_offline_cache_only = matches!(source_class, RegistryMirrorSourceClass::OfflineCache)
        || reachability.is_offline_cache_only();
    let source_implies_policy_pin = source_class.implies_policy_pin();

    RegistryMirrorDisclosure {
        is_offline_cache_only,
        needs_offline_continuity_note: is_offline_cache_only,
        needs_reachability_note: reachability.needs_reachability_note(),
        needs_policy_pin_note: is_policy_pinned || source_implies_policy_pin,
        source_implies_policy_pin,
    }
}

/// A registry / mirror row naming source, auth, reachability, and continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryOrMirrorRow {
    /// Frozen component this control implements; must be `registry_or_mirror_row`.
    pub component: M5PackageComponent,
    /// Stable row id.
    pub row_id: String,
    /// Human-readable registry / mirror label; required and non-empty.
    pub registry_label: String,
    /// Registry / mirror source class.
    pub source_class: RegistryMirrorSourceClass,
    /// Source provenance disclosure; required and non-empty.
    pub source_disclosure: String,
    /// Auth mode.
    pub auth_mode: RegistryAuthMode,
    /// Auth-mode disclosure; required when auth is not anonymous public.
    pub auth_disclosure: String,
    /// Freshness / reachability state.
    pub reachability: RegistryReachabilityState,
    /// Freshness/reachability note; required when reachability is not fresh.
    pub reachability_note: String,
    /// Whether the source is pinned by policy.
    pub is_policy_pinned: bool,
    /// Policy-pin note; required when the row is (or is implied to be) pinned.
    pub policy_pin_note: String,
    /// Whether the row is offline/cache-only.
    pub offline_cache_only: bool,
    /// Offline/cache-only continuity note; required when offline/cache-only.
    pub offline_continuity_note: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// Rollback / write-back posture, reused from the frozen matrix.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl RegistryOrMirrorRow {
    /// Source/continuity disclosures this row must carry, derived from source and reachability.
    pub fn registry_disclosure(&self) -> RegistryMirrorDisclosure {
        resolve_registry_or_mirror_disclosure(
            self.source_class,
            self.reachability,
            self.is_policy_pinned,
        )
    }

    /// Whether the rollback posture is consistent with a read-only source descriptor.
    ///
    /// A registry / mirror row describes where an answer came from; it never
    /// mutates state, so it must be read-only.
    pub fn rollback_posture_consistent(&self) -> bool {
        matches!(
            self.rollback_posture,
            M5PackageComponentRollbackPosture::ReadOnlyNoMutation
        )
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeRegistryTrustReview {
    /// The active manifest label is always named.
    pub active_manifest_always_named: bool,
    /// The target manifest scope is always explicit.
    pub target_scope_always_explicit: bool,
    /// The lockfile coupling is always explicit.
    pub lockfile_coupling_always_explicit: bool,
    /// The change scope matches the target scope and lockfile coupling.
    pub change_scope_matches_scope_and_lockfile: bool,
    /// No generic manage-package or one-click language conceals the scope.
    pub no_generic_manage_package_language: bool,
    /// The registry source is always explicit.
    pub registry_source_always_explicit: bool,
    /// The auth mode is always explicit.
    pub auth_mode_always_explicit: bool,
    /// The freshness / reachability state is always explicit.
    pub freshness_reachability_always_explicit: bool,
    /// Policy pinning stays explicit.
    pub policy_pinning_explicit: bool,
    /// Offline / cache-only continuity stays explicit.
    pub offline_cache_continuity_explicit: bool,
    /// Inherited registry state is never hidden.
    pub inherited_registry_state_never_hidden: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ManifestScopeRegistryTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.active_manifest_always_named
            && self.target_scope_always_explicit
            && self.lockfile_coupling_always_explicit
            && self.change_scope_matches_scope_and_lockfile
            && self.no_generic_manage_package_language
            && self.registry_source_always_explicit
            && self.auth_mode_always_explicit
            && self.freshness_reachability_always_explicit
            && self.policy_pinning_explicit
            && self.offline_cache_continuity_explicit
            && self.inherited_registry_state_never_hidden
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeRegistryConsumerProjection {
    /// The switcher shows the active manifest and scope.
    pub switcher_shows_active_manifest_and_scope: bool,
    /// The lockfile coupling is shown inline.
    pub lockfile_coupling_shown_inline: bool,
    /// The change-scope action reflects the derived truth.
    pub change_scope_action_reflects_truth: bool,
    /// The registry row shows source, auth, and reachability.
    pub registry_row_shows_source_auth_and_reachability: bool,
    /// Offline and policy-pin state are shown inline.
    pub offline_and_policy_pin_shown_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl ManifestScopeRegistryConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.switcher_shows_active_manifest_and_scope
            && self.lockfile_coupling_shown_inline
            && self.change_scope_action_reflects_truth
            && self.registry_row_shows_source_auth_and_reachability
            && self.offline_and_policy_pin_shown_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeRegistryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ManifestScopeRegistryControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestScopeRegistryControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Manifest-scope switchers.
    pub manifest_scope_switchers: Vec<ManifestScopeSwitcher>,
    /// Registry / mirror rows.
    pub registry_or_mirror_rows: Vec<RegistryOrMirrorRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: ManifestScopeRegistryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ManifestScopeRegistryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ManifestScopeRegistryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe manifest-scope / registry-or-mirror controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestScopeRegistryControlsPacket {
    /// Record kind; must equal [`MANIFEST_SCOPE_REGISTRY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MANIFEST_SCOPE_REGISTRY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Manifest-scope switchers.
    pub manifest_scope_switchers: Vec<ManifestScopeSwitcher>,
    /// Registry / mirror rows.
    pub registry_or_mirror_rows: Vec<RegistryOrMirrorRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: ManifestScopeRegistryTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ManifestScopeRegistryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ManifestScopeRegistryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ManifestScopeRegistryControlsPacket {
    /// Builds a manifest-scope / registry-or-mirror controls packet from stable-lane input.
    pub fn new(input: ManifestScopeRegistryControlsPacketInput) -> Self {
        Self {
            record_kind: MANIFEST_SCOPE_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_SCOPE_REGISTRY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            manifest_scope_switchers: input.manifest_scope_switchers,
            registry_or_mirror_rows: input.registry_or_mirror_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the manifest-scope / registry-or-mirror control invariants.
    pub fn validate(&self) -> Vec<ManifestScopeRegistryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MANIFEST_SCOPE_REGISTRY_RECORD_KIND {
            violations.push(ManifestScopeRegistryViolation::WrongRecordKind);
        }
        if self.schema_version != MANIFEST_SCOPE_REGISTRY_SCHEMA_VERSION {
            violations.push(ManifestScopeRegistryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ManifestScopeRegistryViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ManifestScopeRegistryViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_switchers(self, &mut violations);
        validate_registry_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ManifestScopeRegistryViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ManifestScopeRegistryViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("manifest scope registry packet serializes"),
        ) {
            violations.push(ManifestScopeRegistryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("manifest scope registry packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let shared_lock = self
            .manifest_scope_switchers
            .iter()
            .filter(|switcher| switcher.affects_root_lockfile)
            .count();
        let offline = self
            .registry_or_mirror_rows
            .iter()
            .filter(|row| row.offline_cache_only)
            .count();

        let mut out = String::new();
        out.push_str("# Manifest-scope switchers and registry/mirror rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Switchers: {} ({} regenerate the shared root lockfile)\n",
            self.manifest_scope_switchers.len(),
            shared_lock
        ));
        out.push_str(&format!(
            "- Registry/mirror rows: {} ({} offline/cache-only)\n",
            self.registry_or_mirror_rows.len(),
            offline
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Manifest-scope switchers\n\n");
        for switcher in &self.manifest_scope_switchers {
            out.push_str(&format!(
                "- **{}** ({}) — change `{}` [{}]\n",
                switcher.active_manifest_label,
                switcher.target_scope.as_str(),
                switcher
                    .change_scope_disclosure()
                    .change_scope_class
                    .as_str(),
                switcher.lockfile_coupling.as_str()
            ));
        }

        out.push_str("\n## Registry / mirror rows\n\n");
        for row in &self.registry_or_mirror_rows {
            out.push_str(&format!(
                "- **{}** [{}] auth `{}`, {} {}\n",
                row.registry_label,
                row.source_class.as_str(),
                row.auth_mode.as_str(),
                row.reachability.as_str(),
                if row.offline_cache_only {
                    "(offline/cache-only)"
                } else {
                    ""
                }
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in manifest-scope / registry export.
#[derive(Debug)]
pub enum ManifestScopeRegistryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ManifestScopeRegistryViolation>),
}

impl fmt::Display for ManifestScopeRegistryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "manifest scope registry export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "manifest scope registry export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ManifestScopeRegistryArtifactError {}

/// Validation failures emitted by [`ManifestScopeRegistryControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestScopeRegistryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No manifest-scope switchers are present.
    SwitchersMissing,
    /// A switcher is incomplete.
    SwitcherIncomplete,
    /// A switcher carries the wrong frozen component class.
    SwitcherWrongComponentClass,
    /// A switcher does not name its active manifest label.
    ActiveManifestLabelMissing,
    /// A switcher does not name its owning scope.
    ScopeDisclosureMissing,
    /// A below-root switcher does not name which member/module it targets.
    MemberSelectionNoteMissing,
    /// A lockfile-coupled switcher does not name its lockfile coupling.
    LockfileCouplingNoteMissing,
    /// A switcher does not name a change-scope action label.
    ChangeScopeActionLabelMissing,
    /// A switcher misrepresents its change scope relative to scope/coupling.
    ChangeScopeMisrepresented,
    /// A switcher rollback posture is inconsistent with a scope-selection control.
    SwitcherRollbackPostureInconsistent,
    /// The switchers do not cover root, member, module, and tool scopes.
    ScopeCoverageMissing,
    /// No registry / mirror rows are present.
    RegistryRowsMissing,
    /// A registry / mirror row is incomplete.
    RegistryRowIncomplete,
    /// A registry / mirror row carries the wrong frozen component class.
    RegistryRowWrongComponentClass,
    /// A registry / mirror row does not name its source provenance.
    RegistrySourceDisclosureMissing,
    /// An authenticated row does not name its auth mode.
    AuthDisclosureMissing,
    /// A non-fresh row does not name its freshness/reachability.
    ReachabilityNoteMissing,
    /// A pinned row does not name why it is pinned.
    PolicyPinNoteMissing,
    /// A policy-pinned source is not marked pinned.
    PolicyPinningMisrepresented,
    /// An offline/cache-only row does not name its continuity.
    OfflineContinuityNoteMissing,
    /// A row misrepresents offline/cache-only continuity.
    OfflineContinuityMisrepresented,
    /// A degraded resolution does not carry a degradation note.
    RegistryDegradationNoteMissing,
    /// A registry / mirror row rollback posture is inconsistent with a read-only descriptor.
    RegistryRowRollbackInconsistent,
    /// The registry rows do not cover the five required source classes.
    SourceCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ManifestScopeRegistryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SwitchersMissing => "switchers_missing",
            Self::SwitcherIncomplete => "switcher_incomplete",
            Self::SwitcherWrongComponentClass => "switcher_wrong_component_class",
            Self::ActiveManifestLabelMissing => "active_manifest_label_missing",
            Self::ScopeDisclosureMissing => "scope_disclosure_missing",
            Self::MemberSelectionNoteMissing => "member_selection_note_missing",
            Self::LockfileCouplingNoteMissing => "lockfile_coupling_note_missing",
            Self::ChangeScopeActionLabelMissing => "change_scope_action_label_missing",
            Self::ChangeScopeMisrepresented => "change_scope_misrepresented",
            Self::SwitcherRollbackPostureInconsistent => "switcher_rollback_posture_inconsistent",
            Self::ScopeCoverageMissing => "scope_coverage_missing",
            Self::RegistryRowsMissing => "registry_rows_missing",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::RegistryRowWrongComponentClass => "registry_row_wrong_component_class",
            Self::RegistrySourceDisclosureMissing => "registry_source_disclosure_missing",
            Self::AuthDisclosureMissing => "auth_disclosure_missing",
            Self::ReachabilityNoteMissing => "reachability_note_missing",
            Self::PolicyPinNoteMissing => "policy_pin_note_missing",
            Self::PolicyPinningMisrepresented => "policy_pinning_misrepresented",
            Self::OfflineContinuityNoteMissing => "offline_continuity_note_missing",
            Self::OfflineContinuityMisrepresented => "offline_continuity_misrepresented",
            Self::RegistryDegradationNoteMissing => "registry_degradation_note_missing",
            Self::RegistryRowRollbackInconsistent => "registry_row_rollback_inconsistent",
            Self::SourceCoverageMissing => "source_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable manifest-scope / registry export.
pub fn current_manifest_scope_registry_export(
) -> Result<ManifestScopeRegistryControlsPacket, ManifestScopeRegistryArtifactError> {
    let packet: ManifestScopeRegistryControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-manifest-scope-registry-proof/support_export.json"
    )))
    .map_err(ManifestScopeRegistryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ManifestScopeRegistryArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ManifestScopeRegistryControlsPacket,
    violations: &mut Vec<ManifestScopeRegistryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MANIFEST_SCOPE_REGISTRY_SCHEMA_REF,
        MANIFEST_SCOPE_REGISTRY_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ManifestScopeRegistryViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_switchers(
    packet: &ManifestScopeRegistryControlsPacket,
    violations: &mut Vec<ManifestScopeRegistryViolation>,
) {
    if packet.manifest_scope_switchers.is_empty() {
        violations.push(ManifestScopeRegistryViolation::SwitchersMissing);
        return;
    }

    let mut scopes: BTreeSet<ManifestTargetScope> = BTreeSet::new();

    for switcher in &packet.manifest_scope_switchers {
        scopes.insert(switcher.target_scope);

        if switcher.switcher_id.trim().is_empty()
            || switcher.fields_shown.is_empty()
            || switcher.source_contract_refs.is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::SwitcherIncomplete);
        }
        if switcher.component != M5PackageComponent::ManifestScopeSwitcher {
            violations.push(ManifestScopeRegistryViolation::SwitcherWrongComponentClass);
        }
        if switcher.active_manifest_label.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::ActiveManifestLabelMissing);
        }
        if switcher.scope_disclosure.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::ScopeDisclosureMissing);
        }
        if switcher.change_scope_action_label.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::ChangeScopeActionLabelMissing);
        }

        let disclosure = switcher.change_scope_disclosure();

        if switcher.affects_root_lockfile != disclosure.affects_shared_root_lockfile {
            violations.push(ManifestScopeRegistryViolation::ChangeScopeMisrepresented);
        }
        if disclosure.needs_member_selection_note
            && switcher.member_selection_note.trim().is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::MemberSelectionNoteMissing);
        }
        if disclosure.needs_lockfile_coupling_note
            && switcher.lockfile_coupling_note.trim().is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::LockfileCouplingNoteMissing);
        }
        if !switcher.rollback_posture_consistent() {
            violations.push(ManifestScopeRegistryViolation::SwitcherRollbackPostureInconsistent);
        }
    }

    for required in ManifestTargetScope::ALL {
        if !scopes.contains(&required) {
            violations.push(ManifestScopeRegistryViolation::ScopeCoverageMissing);
            break;
        }
    }
}

fn validate_registry_rows(
    packet: &ManifestScopeRegistryControlsPacket,
    violations: &mut Vec<ManifestScopeRegistryViolation>,
) {
    if packet.registry_or_mirror_rows.is_empty() {
        violations.push(ManifestScopeRegistryViolation::RegistryRowsMissing);
        return;
    }

    let mut sources: BTreeSet<RegistryMirrorSourceClass> = BTreeSet::new();

    for row in &packet.registry_or_mirror_rows {
        sources.insert(row.source_class);

        if row.row_id.trim().is_empty()
            || row.registry_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::RegistryRowIncomplete);
        }
        if row.component != M5PackageComponent::RegistryOrMirrorRow {
            violations.push(ManifestScopeRegistryViolation::RegistryRowWrongComponentClass);
        }
        if row.source_disclosure.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::RegistrySourceDisclosureMissing);
        }
        if row.auth_mode.needs_auth_disclosure() && row.auth_disclosure.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::AuthDisclosureMissing);
        }
        if !matches!(
            row.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && row.degradation_note.trim().is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::RegistryDegradationNoteMissing);
        }

        let disclosure = row.registry_disclosure();

        if row.offline_cache_only != disclosure.is_offline_cache_only {
            violations.push(ManifestScopeRegistryViolation::OfflineContinuityMisrepresented);
        }
        if disclosure.source_implies_policy_pin && !row.is_policy_pinned {
            violations.push(ManifestScopeRegistryViolation::PolicyPinningMisrepresented);
        }
        if disclosure.needs_offline_continuity_note && row.offline_continuity_note.trim().is_empty()
        {
            violations.push(ManifestScopeRegistryViolation::OfflineContinuityNoteMissing);
        }
        if disclosure.needs_reachability_note && row.reachability_note.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::ReachabilityNoteMissing);
        }
        if disclosure.needs_policy_pin_note && row.policy_pin_note.trim().is_empty() {
            violations.push(ManifestScopeRegistryViolation::PolicyPinNoteMissing);
        }
        if !row.rollback_posture_consistent() {
            violations.push(ManifestScopeRegistryViolation::RegistryRowRollbackInconsistent);
        }
    }

    for required in RegistryMirrorSourceClass::ALL {
        if !sources.contains(&required) {
            violations.push(ManifestScopeRegistryViolation::SourceCoverageMissing);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
