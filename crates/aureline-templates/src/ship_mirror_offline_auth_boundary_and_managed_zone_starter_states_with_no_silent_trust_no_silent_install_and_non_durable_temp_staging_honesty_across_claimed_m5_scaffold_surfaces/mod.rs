//! Starter boundary states for the reusable M5 scaffold / project-entry components, so a user can
//! tell — *before* any silent trust or install step — whether a starter depends on public registry
//! access, a mirror, an offline cache, browser / device sign-in, a remote-image or managed
//! workspace, or non-durable temporary staging, and always keeps an explicit recovery path once a
//! starter partially materializes output.
//!
//! Aureline's frozen scaffold-component matrix
//! ([`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`])
//! locks the six governed scaffold / project-entry components — the scaffold template card, the
//! starter parameter row, the scaffold preflight card, the template health row, the generated-project
//! diff card, and the scaffold handoff banner — and freezes their shared controlled vocabulary (the
//! dispositions, surface families, deployment lines, consumer surfaces, accessibility routes,
//! required labels, and downgrade triggers). This lane *implements* the cross-cutting **starter
//! boundary state** that any of those six components can carry when a scaffold's source, availability,
//! trust, or durability is not the plain public-registry default — so the start-center, template
//! gallery, and scaffold preflight can name a mirror-only, offline-cache-only, sign-in-required,
//! remote / managed-workspace, or non-durable temporary-staging dependency instead of routing a user
//! through a silent trust prompt or install step.
//!
//! The module has one derived resolver:
//!
//! [`resolve_starter_disclosure`] — takes a boundary state's frozen boundary kind and availability
//! state and derives its **access posture** (direct public access, mirror-mediated, offline-cache
//! backed, auth-gated, managed-remote, or non-durable staging) and its **availability posture**
//! (reachable now, reachable via a mirror, reachable from an offline cache, blocked pending sign-in,
//! blocked pending provisioning, or not reachable), so a sign-in-gated, managed-remote, or
//! non-durable starter can never read as a plain public-registry create and an unavailable or blocked
//! starter can never read as ready.
//!
//! A single controls packet — [`StarterBoundaryStateControlsPacket`] — binds one vector of boundary
//! states to the same source / owner / freshness / availability, trust-and-install disclosure,
//! recovery-verb, deep-link, and non-visual accessibility vocabulary, so what a starter depends on
//! and how to recover stay explicit across desktop, headless / export, and support consumers.
//!
//! The disposition ([`M5ScaffoldDisposition`]), starter source class ([`M5StarterSourceClass`]),
//! component family ([`M5ScaffoldComponentFamily`]), surface family ([`M5ScaffoldSurfaceFamily`]),
//! deployment line ([`M5ScaffoldDeploymentLine`]), consumer surface ([`M5ScaffoldConsumerSurface`]),
//! accessibility route ([`M5ScaffoldAccessibilityRoute`]), required label ([`M5ScaffoldRequiredLabel`]),
//! and downgrade trigger ([`M5ScaffoldDowngradeTrigger`]) are reused verbatim from the frozen matrix.
//! This module mints new vocabulary only for what that matrix left implicit about the boundary state
//! itself: the acceptance-criteria boundary kinds, the availability state, the owner and freshness
//! cues, the derived access and availability postures, the bounded boundary-state actions, and the
//! recovery verbs. No M5 scaffold surface invents a second boundary-state grammar.
//!
//! Raw file bodies, raw secret values, pasted local paths, repository URLs, credentials, and secrets
//! stay outside the export boundary; every note, deep-link reference, and component identity is
//! carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The disposition vocabulary, the starter source class, the component families, and the surface /
// deployment / consumer / accessibility / label / downgrade vocabularies are frozen once, in the
// scaffold-component matrix. This lane reuses them verbatim so it never invents a parallel
// boundary-state vocabulary.
pub use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5ScaffoldAccessibilityRoute, M5ScaffoldComponentFamily, M5ScaffoldConsumerSurface,
    M5ScaffoldDeploymentLine, M5ScaffoldDisposition, M5ScaffoldDowngradeTrigger,
    M5ScaffoldRequiredLabel, M5ScaffoldSurfaceFamily, M5StarterSourceClass,
    M5_SCAFFOLD_COMPONENT_DOC_REF, M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`StarterBoundaryStateControlsPacket`].
pub const STARTER_BOUNDARY_STATE_RECORD_KIND: &str =
    "ship_mirror_offline_auth_boundary_and_managed_zone_starter_states_with_no_silent_trust_no_silent_install_and_non_durable_temp_staging_honesty_across_claimed_m5_scaffold_surfaces";

/// Schema version for M5 starter-boundary-state control records.
pub const STARTER_BOUNDARY_STATE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const STARTER_BOUNDARY_STATE_SCHEMA_REF: &str =
    "schemas/ui/m5-starter-boundary-state-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const STARTER_BOUNDARY_STATE_DOC_REF: &str =
    "docs/templates/m5_starter_boundary_state_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const STARTER_BOUNDARY_STATE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-starter-boundary-state-controls";

/// Repo-relative path of the checked support-export artifact.
pub const STARTER_BOUNDARY_STATE_ARTIFACT_REF: &str =
    "artifacts/release/m5-starter-boundary-state-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const STARTER_BOUNDARY_STATE_CSV_REF: &str =
    "artifacts/release/m5-starter-boundary-state-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const STARTER_BOUNDARY_STATE_REPORT_REF: &str = "artifacts/design/m5-starter-boundary-state.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a starter boundary state binds its next step against, so a boundary
/// state never routes through an ephemeral overlay — every next step is a stable template manifest,
/// starter-registry entry, docs, or policy reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable template-manifest reference.
    TemplateManifest,
    /// A stable starter-registry entry reference.
    StarterRegistryEntry,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable policy reference.
    PolicyReference,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TemplateManifest,
        Self::StarterRegistryEntry,
        Self::DocsAnchor,
        Self::PolicyReference,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateManifest => "template_manifest",
            Self::StarterRegistryEntry => "starter_registry_entry",
            Self::DocsAnchor => "docs_anchor",
            Self::PolicyReference => "policy_reference",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- starter-boundary-state vocabulary ----------------------------------

/// The boundary kind a starter depends on. These are the exact acceptance-criteria variants, so a
/// user can tell whether a starter depends on public registry access, a mirror, an offline cache,
/// browser / device sign-in, a remote-image or managed workspace, or non-durable temporary staging
/// *before* any silent trust or install step occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterBoundaryKind {
    /// Depends on public registry access (the plain default).
    PublicRegistry,
    /// Served only through a mirror.
    MirrorOnly,
    /// Served only from an offline cache.
    OfflineCacheOnly,
    /// Requires a browser / device sign-in before it can be fetched.
    SignInRequired,
    /// Depends on a remote image or a managed workspace.
    RemoteOrManagedWorkspace,
    /// Materializes into non-durable temporary staging.
    NonDurableTempStaging,
}

impl StarterBoundaryKind {
    /// Every boundary kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublicRegistry,
        Self::MirrorOnly,
        Self::OfflineCacheOnly,
        Self::SignInRequired,
        Self::RemoteOrManagedWorkspace,
        Self::NonDurableTempStaging,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicRegistry => "public_registry",
            Self::MirrorOnly => "mirror_only",
            Self::OfflineCacheOnly => "offline_cache_only",
            Self::SignInRequired => "sign_in_required",
            Self::RemoteOrManagedWorkspace => "remote_or_managed_workspace",
            Self::NonDurableTempStaging => "non_durable_temp_staging",
        }
    }
}

/// The availability state a starter reports, so a boundary state never leaves whether the starter is
/// reachable — and by what route — implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterAvailabilityState {
    /// Reachable directly right now.
    Available,
    /// Reachable only through a mirror.
    MirrorReachableOnly,
    /// Reachable only from an offline cache.
    CacheOnlyOffline,
    /// Blocked pending a browser / device sign-in.
    SignInPending,
    /// Blocked pending remote provisioning.
    ProvisioningPending,
    /// Not reachable.
    Unavailable,
}

impl StarterAvailabilityState {
    /// Every availability state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Available,
        Self::MirrorReachableOnly,
        Self::CacheOnlyOffline,
        Self::SignInPending,
        Self::ProvisioningPending,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::MirrorReachableOnly => "mirror_reachable_only",
            Self::CacheOnlyOffline => "cache_only_offline",
            Self::SignInPending => "sign_in_pending",
            Self::ProvisioningPending => "provisioning_pending",
            Self::Unavailable => "unavailable",
        }
    }
}

/// The owner a starter's source belongs to, so a boundary state never leaves who serves the starter
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterOwnerClass {
    /// A first-party registry.
    FirstPartyRegistry,
    /// A team-operated mirror.
    TeamMirror,
    /// A local offline cache.
    LocalCache,
    /// A managed service.
    ManagedService,
    /// The owner is unknown.
    UnknownOwner,
}

impl StarterOwnerClass {
    /// Every owner class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstPartyRegistry,
        Self::TeamMirror,
        Self::LocalCache,
        Self::ManagedService,
        Self::UnknownOwner,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyRegistry => "first_party_registry",
            Self::TeamMirror => "team_mirror",
            Self::LocalCache => "local_cache",
            Self::ManagedService => "managed_service",
            Self::UnknownOwner => "unknown_owner",
        }
    }
}

/// The freshness a starter's source carries, so a boundary state never leaves whether its content is
/// live, mirror-synced, stale, ephemeral, or unknown implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterFreshnessState {
    /// Live from the source.
    Live,
    /// Synced from a mirror.
    MirrorSynced,
    /// Served from a stale cache.
    CacheStale,
    /// Non-durable / ephemeral (temporary staging).
    Ephemeral,
    /// Freshness is unknown.
    FreshnessUnknown,
}

impl StarterFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::MirrorSynced,
        Self::CacheStale,
        Self::Ephemeral,
        Self::FreshnessUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::MirrorSynced => "mirror_synced",
            Self::CacheStale => "cache_stale",
            Self::Ephemeral => "ephemeral",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }
}

/// Derived access posture a starter boundary state may present.
///
/// This is the source honesty axis: the posture is derived from the frozen boundary kind, never
/// asserted, so a sign-in-gated, managed-remote, or non-durable starter can never present as a plain
/// public-registry create.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterAccessPosture {
    /// Direct public-registry access.
    DirectPublicAccess,
    /// Mediated through a mirror.
    MirrorMediated,
    /// Backed by an offline cache.
    OfflineCacheBacked,
    /// Gated behind a browser / device sign-in.
    AuthGated,
    /// Dependent on a remote image or a managed workspace.
    ManagedRemote,
    /// Materialized into non-durable temporary staging.
    NonDurableStaging,
}

impl StarterAccessPosture {
    /// Every access posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DirectPublicAccess,
        Self::MirrorMediated,
        Self::OfflineCacheBacked,
        Self::AuthGated,
        Self::ManagedRemote,
        Self::NonDurableStaging,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectPublicAccess => "direct_public_access",
            Self::MirrorMediated => "mirror_mediated",
            Self::OfflineCacheBacked => "offline_cache_backed",
            Self::AuthGated => "auth_gated",
            Self::ManagedRemote => "managed_remote",
            Self::NonDurableStaging => "non_durable_staging",
        }
    }

    /// True when the starter requires a browser / device sign-in.
    pub const fn requires_sign_in(self) -> bool {
        matches!(self, Self::AuthGated)
    }

    /// True when the starter depends on a remote image or a managed workspace.
    pub const fn is_managed_remote(self) -> bool {
        matches!(self, Self::ManagedRemote)
    }

    /// True when the starter materializes into non-durable temporary staging.
    pub const fn is_non_durable(self) -> bool {
        matches!(self, Self::NonDurableStaging)
    }

    /// True when the starter depends on a mirror or an offline cache.
    pub const fn is_mirror_or_cache(self) -> bool {
        matches!(self, Self::MirrorMediated | Self::OfflineCacheBacked)
    }
}

/// Derived availability posture a starter boundary state may present.
///
/// This is the availability honesty axis: the posture is derived from the frozen availability state,
/// never asserted, so an unavailable or blocked starter can never present as ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterAvailabilityPosture {
    /// Reachable now.
    ReachableNow,
    /// Reachable via a mirror.
    ReachableViaMirror,
    /// Reachable from an offline cache.
    ReachableFromCache,
    /// Blocked pending a sign-in.
    BlockedPendingSignIn,
    /// Blocked pending remote provisioning.
    BlockedPendingProvisioning,
    /// Not reachable.
    NotReachable,
}

impl StarterAvailabilityPosture {
    /// Every availability posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReachableNow,
        Self::ReachableViaMirror,
        Self::ReachableFromCache,
        Self::BlockedPendingSignIn,
        Self::BlockedPendingProvisioning,
        Self::NotReachable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableNow => "reachable_now",
            Self::ReachableViaMirror => "reachable_via_mirror",
            Self::ReachableFromCache => "reachable_from_cache",
            Self::BlockedPendingSignIn => "blocked_pending_sign_in",
            Self::BlockedPendingProvisioning => "blocked_pending_provisioning",
            Self::NotReachable => "not_reachable",
        }
    }

    /// True when the starter can still be reached by some route.
    pub const fn is_reachable(self) -> bool {
        matches!(
            self,
            Self::ReachableNow | Self::ReachableViaMirror | Self::ReachableFromCache
        )
    }

    /// True only when the starter is not reachable at all.
    pub const fn is_not_reachable(self) -> bool {
        matches!(self, Self::NotReachable)
    }
}

/// One recovery verb a starter boundary state preserves when a starter partially materializes output
/// or cannot complete the full bootstrap path, so the user always has an explicit way out rather than
/// an ambiguous half-generated tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterRecoveryVerb {
    /// Delete the generated output.
    DeleteGenerated,
    /// Reuse the existing output.
    ReuseExisting,
    /// Clone the starter elsewhere.
    CloneElsewhere,
    /// Continue without a starter.
    ContinueWithoutStarter,
    /// Retry when the starter becomes available.
    RetryWhenAvailable,
}

impl StarterRecoveryVerb {
    /// Every recovery verb, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DeleteGenerated,
        Self::ReuseExisting,
        Self::CloneElsewhere,
        Self::ContinueWithoutStarter,
        Self::RetryWhenAvailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeleteGenerated => "delete_generated",
            Self::ReuseExisting => "reuse_existing",
            Self::CloneElsewhere => "clone_elsewhere",
            Self::ContinueWithoutStarter => "continue_without_starter",
            Self::RetryWhenAvailable => "retry_when_available",
        }
    }
}

/// True when a recovery-verb set offers a real recovery route (delete-generated, reuse-existing,
/// clone-elsewhere, or continue-without-starter) rather than only retry-when-available.
fn offers_real_recovery(verbs: &[StarterRecoveryVerb]) -> bool {
    verbs.iter().any(|verb| {
        matches!(
            verb,
            StarterRecoveryVerb::DeleteGenerated
                | StarterRecoveryVerb::ReuseExisting
                | StarterRecoveryVerb::CloneElsewhere
                | StarterRecoveryVerb::ContinueWithoutStarter
        )
    })
}

/// One keyboard-complete default action a starter boundary state offers, so a boundary state never
/// hides its source, availability, or trust / install affordance behind a pointer-only gesture and
/// always keeps a recovery choice. `ReviewSourceAndOwner`, `ReviewAvailabilityAndFreshness`, and
/// `ReviewTrustAndInstallSteps` are always offered so no silent trust or install step can occur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StarterStateAction {
    /// Review the starter's source and owner (always available).
    ReviewSourceAndOwner,
    /// Review the starter's availability and freshness (always available).
    ReviewAvailabilityAndFreshness,
    /// Review the trust and install steps before they run (always available).
    ReviewTrustAndInstallSteps,
    /// Proceed only after the boundary is disclosed.
    ProceedWithDisclosure,
    /// Choose a recovery verb.
    ChooseRecovery,
    /// Open the stable manifest / registry / docs / policy deep link.
    OpenDeepLink,
}

impl StarterStateAction {
    /// Every boundary-state action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewSourceAndOwner,
        Self::ReviewAvailabilityAndFreshness,
        Self::ReviewTrustAndInstallSteps,
        Self::ProceedWithDisclosure,
        Self::ChooseRecovery,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete boundary state must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::ReviewSourceAndOwner,
        Self::ReviewAvailabilityAndFreshness,
        Self::ReviewTrustAndInstallSteps,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewSourceAndOwner => "review_source_and_owner",
            Self::ReviewAvailabilityAndFreshness => "review_availability_and_freshness",
            Self::ReviewTrustAndInstallSteps => "review_trust_and_install_steps",
            Self::ProceedWithDisclosure => "proceed_with_disclosure",
            Self::ChooseRecovery => "choose_recovery",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures a starter boundary state must carry, derived from the frozen boundary kind and
/// availability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarterBoundaryDisclosure {
    /// The derived access posture this boundary state may present.
    pub access_posture: StarterAccessPosture,
    /// The derived availability posture this boundary state may present.
    pub availability_posture: StarterAvailabilityPosture,
    /// Whether the starter requires a browser / device sign-in.
    pub requires_sign_in: bool,
    /// Whether the starter depends on a remote image or a managed workspace.
    pub requires_managed_provisioning: bool,
    /// Whether the starter materializes into non-durable temporary staging.
    pub is_non_durable: bool,
    /// Whether the starter depends on a mirror or an offline cache.
    pub depends_on_mirror_or_cache: bool,
    /// Whether the starter can still be reached by some route.
    pub is_reachable: bool,
    /// Whether the boundary state must carry an explicit sign-in note.
    pub needs_sign_in_note: bool,
    /// Whether the boundary state must carry an explicit managed / remote note.
    pub needs_managed_note: bool,
    /// Whether the boundary state must carry an explicit non-durable note.
    pub needs_non_durable_note: bool,
    /// Whether the boundary state must carry an explicit mirror / offline-cache note.
    pub needs_mirror_or_cache_note: bool,
    /// Whether the boundary state must carry an explicit unavailable note.
    pub needs_unavailable_note: bool,
}

/// Resolves the source and availability truth a starter boundary state may present.
///
/// A `public_registry` starter is direct public access, a `mirror_only` starter is mirror-mediated,
/// an `offline_cache_only` starter is offline-cache backed, a `sign_in_required` starter is
/// auth-gated, a `remote_or_managed_workspace` starter is managed-remote, and a
/// `non_durable_temp_staging` starter materializes into non-durable staging — so a sign-in-gated,
/// managed-remote, or non-durable starter can never read as a plain public-registry create.
/// Independently, the availability state derives the availability posture so an unavailable or
/// blocked starter can never read as ready.
pub fn resolve_starter_disclosure(
    boundary_kind: StarterBoundaryKind,
    availability_state: StarterAvailabilityState,
) -> StarterBoundaryDisclosure {
    use StarterAccessPosture as Access;
    use StarterAvailabilityPosture as Avail;
    use StarterAvailabilityState as State;
    use StarterBoundaryKind as Kind;

    let access_posture = match boundary_kind {
        Kind::PublicRegistry => Access::DirectPublicAccess,
        Kind::MirrorOnly => Access::MirrorMediated,
        Kind::OfflineCacheOnly => Access::OfflineCacheBacked,
        Kind::SignInRequired => Access::AuthGated,
        Kind::RemoteOrManagedWorkspace => Access::ManagedRemote,
        Kind::NonDurableTempStaging => Access::NonDurableStaging,
    };

    let availability_posture = match availability_state {
        State::Available => Avail::ReachableNow,
        State::MirrorReachableOnly => Avail::ReachableViaMirror,
        State::CacheOnlyOffline => Avail::ReachableFromCache,
        State::SignInPending => Avail::BlockedPendingSignIn,
        State::ProvisioningPending => Avail::BlockedPendingProvisioning,
        State::Unavailable => Avail::NotReachable,
    };

    StarterBoundaryDisclosure {
        access_posture,
        availability_posture,
        requires_sign_in: access_posture.requires_sign_in(),
        requires_managed_provisioning: access_posture.is_managed_remote(),
        is_non_durable: access_posture.is_non_durable(),
        depends_on_mirror_or_cache: access_posture.is_mirror_or_cache(),
        is_reachable: availability_posture.is_reachable(),
        needs_sign_in_note: access_posture.requires_sign_in(),
        needs_managed_note: access_posture.is_managed_remote(),
        needs_non_durable_note: access_posture.is_non_durable(),
        needs_mirror_or_cache_note: access_posture.is_mirror_or_cache(),
        needs_unavailable_note: availability_posture.is_not_reachable(),
    }
}

/// A starter boundary state naming its boundary kind, availability state, owner, and freshness, its
/// source / owner / availability / freshness cues, the trust and install steps it discloses before
/// they run, a recovery route, its derived access and availability postures, bounded review /
/// recovery actions, and a stable manifest / registry / docs / policy deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterBoundaryStateCard {
    /// Stable boundary-state id.
    pub state_id: String,
    /// Human-readable boundary-state name; required and non-empty.
    pub state_name: String,
    /// Boundary kind this state names (the exact acceptance-criteria vocabulary).
    pub boundary_kind: StarterBoundaryKind,
    /// Availability state, reported for the starter.
    pub availability_state: StarterAvailabilityState,
    /// Owner class of the starter's source.
    pub owner_class: StarterOwnerClass,
    /// Freshness state of the starter's source.
    pub freshness_state: StarterFreshnessState,
    /// Starter source class, reused from the frozen matrix.
    pub source_class: M5StarterSourceClass,
    /// Source label; always required so where the starter comes from stays explicit.
    pub source_label: String,
    /// Owner label; always required so who serves the starter stays explicit.
    pub owner_label: String,
    /// Availability note; always required so whether — and by what route — the starter is reachable
    /// stays explicit.
    pub availability_note: String,
    /// Freshness note; always required so whether the content is live, synced, stale, or ephemeral
    /// stays explicit.
    pub freshness_note: String,
    /// Derived access posture (must equal the resolved posture).
    pub derived_access_posture: StarterAccessPosture,
    /// Derived availability posture (must equal the resolved posture).
    pub derived_availability_posture: StarterAvailabilityPosture,
    /// Whether the state claims the starter requires a sign-in (must equal derived truth).
    pub claims_requires_sign_in: bool,
    /// Whether the state claims the starter requires managed provisioning (must equal derived truth).
    pub claims_requires_managed_provisioning: bool,
    /// Whether the state claims the starter is non-durable (must equal derived truth).
    pub claims_is_non_durable: bool,
    /// Whether the state claims the starter depends on a mirror or cache (must equal derived truth).
    pub claims_depends_on_mirror_or_cache: bool,
    /// Whether the state claims the starter is reachable (must equal derived truth).
    pub claims_reachable: bool,
    /// Sign-in note; required when the starter requires a browser / device sign-in.
    pub sign_in_note: String,
    /// Managed / remote note; required when the starter depends on a managed workspace or remote
    /// image.
    pub managed_note: String,
    /// Non-durable note; required when the starter materializes into non-durable temporary staging.
    pub non_durable_note: String,
    /// Mirror / offline-cache note; required when the starter depends on a mirror or an offline
    /// cache.
    pub mirror_or_cache_note: String,
    /// Unavailable note; required when the starter is not reachable.
    pub unavailable_note: String,
    /// Trust-disclosure note; always required so no trust step happens silently.
    pub trust_disclosure_note: String,
    /// Install-disclosure note; always required so no install or network step happens silently.
    pub install_disclosure_note: String,
    /// Recovery note; always required (a delete-generated / reuse-existing / clone-elsewhere /
    /// continue-without-starter route).
    pub recovery_note: String,
    /// Context note; always required so the state names what to check before proceeding.
    pub context_note: String,
    /// Kind of stable deep link this state binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include review-source / review-availability /
    /// review-trust-and-install).
    pub state_actions: Vec<StarterStateAction>,
    /// Recovery verbs this state preserves (must offer a real recovery route including
    /// continue-without-starter).
    pub recovery_verbs: Vec<StarterRecoveryVerb>,
    /// Frozen component families this boundary state can apply to (required, non-empty).
    pub applies_to_components: Vec<M5ScaffoldComponentFamily>,
    /// Dispositions this state binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Downgrade triggers this state can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Mandatory labels this state can show (must include the mandatory labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Claimed M5 surface families that render this state.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this state keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Non-visual accessibility routes this state offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this state's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this state.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hides the starter source or owner. MUST be `false`.
    pub hides_starter_source_or_owner: bool,
    /// Hard invariant: never hides a mirror, offline, managed, remote, or non-durable dependency.
    /// MUST be `false`.
    pub hides_mirror_offline_or_managed_dependency: bool,
    /// Hard invariant: never performs a trust or install step silently. MUST be `false`.
    pub performs_silent_trust_or_install: bool,
    /// Hard invariant: never omits a recovery or continue-without-starter path. MUST be `false`.
    pub omits_recovery_or_continue_without_starter_path: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl StarterBoundaryStateCard {
    /// Source and availability disclosures this state must carry, derived from the frozen fields.
    pub fn disclosure(&self) -> StarterBoundaryDisclosure {
        resolve_starter_disclosure(self.boundary_kind, self.availability_state)
    }

    /// Whether the state offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<StarterStateAction> = self.state_actions.iter().copied().collect();
        StarterStateAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the state offers a real recovery route.
    fn offers_real_recovery(&self) -> bool {
        offers_real_recovery(&self.recovery_verbs)
    }

    /// Whether the state preserves the continue-without-starter recovery verb.
    fn offers_continue_without_starter(&self) -> bool {
        self.recovery_verbs
            .contains(&StarterRecoveryVerb::ContinueWithoutStarter)
    }

    /// Whether the state declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ScaffoldRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ScaffoldRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the state offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.state_actions
            .contains(&StarterStateAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance starter-boundary review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterBoundaryReview {
    /// Every boundary state names its source and owner.
    pub state_names_source_and_owner: bool,
    /// Every boundary state names a mirror / offline / managed / remote / staging dependency.
    pub state_names_mirror_offline_managed_or_staging_dependency: bool,
    /// Every boundary state names its availability and freshness.
    pub state_names_availability_and_freshness: bool,
    /// The trust step is disclosed before any trust prompt.
    pub trust_step_disclosed_before_prompt: bool,
    /// The install / network step is disclosed before it runs.
    pub install_step_disclosed_before_run: bool,
    /// Access and availability posture are derived from state, never asserted.
    pub access_and_availability_posture_derived_never_asserted: bool,
    /// An unavailable or blocked starter is never shown as ready.
    pub unavailable_or_blocked_never_shown_as_ready: bool,
    /// A non-durable staging starter is never shown as durable.
    pub non_durable_staging_never_shown_as_durable: bool,
    /// A managed or remote dependency is never hidden behind a plain create.
    pub managed_or_remote_dependency_never_hidden: bool,
    /// The recovery verbs preserve delete-generated / reuse-existing / clone-elsewhere /
    /// continue-without-starter.
    pub recovery_verbs_preserve_delete_reuse_clone_continue: bool,
    /// Continue-without-starter is always available.
    pub continue_without_starter_always_available: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The states keep the same truth across every deployment line.
    pub states_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl StarterBoundaryReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.state_names_source_and_owner
            && self.state_names_mirror_offline_managed_or_staging_dependency
            && self.state_names_availability_and_freshness
            && self.trust_step_disclosed_before_prompt
            && self.install_step_disclosed_before_run
            && self.access_and_availability_posture_derived_never_asserted
            && self.unavailable_or_blocked_never_shown_as_ready
            && self.non_durable_staging_never_shown_as_durable
            && self.managed_or_remote_dependency_never_hidden
            && self.recovery_verbs_preserve_delete_reuse_clone_continue
            && self.continue_without_starter_always_available
            && self.no_surface_invents_alternate_state_label
            && self.states_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterBoundaryConsumerProjection {
    /// The start-center reads a single canonical source.
    pub start_center_reads_single_source: bool,
    /// The template gallery reads a single canonical source.
    pub template_gallery_reads_single_source: bool,
    /// The scaffold preflight reads a single canonical source.
    pub preflight_reads_single_source: bool,
    /// The boundary is visible before any trust or install step.
    pub boundary_visible_before_trust_or_install: bool,
    /// The recovery path is visible on partial or failed bootstrap.
    pub recovery_path_visible_on_failure: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl StarterBoundaryConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.start_center_reads_single_source
            && self.template_gallery_reads_single_source
            && self.preflight_reads_single_source
            && self.boundary_visible_before_trust_or_install
            && self.recovery_path_visible_on_failure
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterBoundaryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`StarterBoundaryStateControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarterBoundaryStateControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Starter boundary states.
    pub boundary_states: Vec<StarterBoundaryStateCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Starter-boundary review block.
    pub boundary_review: StarterBoundaryReview,
    /// Consumer projection block.
    pub consumer_projection: StarterBoundaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StarterBoundaryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe starter-boundary-state controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarterBoundaryStateControlsPacket {
    /// Record kind; must equal [`STARTER_BOUNDARY_STATE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STARTER_BOUNDARY_STATE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Starter boundary states.
    pub boundary_states: Vec<StarterBoundaryStateCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Starter-boundary review block.
    pub boundary_review: StarterBoundaryReview,
    /// Consumer projection block.
    pub consumer_projection: StarterBoundaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StarterBoundaryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl StarterBoundaryStateControlsPacket {
    /// Builds a starter-boundary-state controls packet from stable-lane input.
    pub fn new(input: StarterBoundaryStateControlsPacketInput) -> Self {
        Self {
            record_kind: STARTER_BOUNDARY_STATE_RECORD_KIND.to_owned(),
            schema_version: STARTER_BOUNDARY_STATE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            boundary_states: input.boundary_states,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            boundary_review: input.boundary_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the starter-boundary-state control invariants.
    pub fn validate(&self) -> Vec<StarterBoundaryStateViolation> {
        let mut violations = Vec::new();

        if self.record_kind != STARTER_BOUNDARY_STATE_RECORD_KIND {
            violations.push(StarterBoundaryStateViolation::WrongRecordKind);
        }
        if self.schema_version != STARTER_BOUNDARY_STATE_SCHEMA_VERSION {
            violations.push(StarterBoundaryStateViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(StarterBoundaryStateViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(StarterBoundaryStateViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(StarterBoundaryStateViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_boundary_states(self, &mut violations);

        if !self.boundary_review.all_hold() {
            violations.push(StarterBoundaryStateViolation::BoundaryReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(StarterBoundaryStateViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(StarterBoundaryStateViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("starter boundary state packet serializes"),
        ) {
            violations.push(StarterBoundaryStateViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("starter boundary state packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per boundary state.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "boundary_state,id,boundary_kind,availability_state,access_posture,availability_posture,reachable,deep_link_kind\n",
        );
        for state in &self.boundary_states {
            let disclosure = state.disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "starter_boundary_state",
                csv_field(&state.state_id),
                state.boundary_kind.as_str(),
                state.availability_state.as_str(),
                disclosure.access_posture.as_str(),
                disclosure.availability_posture.as_str(),
                disclosure.is_reachable,
                state.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked_states = self
            .boundary_states
            .iter()
            .filter(|state| !state.disclosure().is_reachable)
            .count();
        let non_durable_states = self
            .boundary_states
            .iter()
            .filter(|state| state.disclosure().is_non_durable)
            .count();

        let mut out = String::new();
        out.push_str("# Starter boundary states\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Starter boundary states: {} ({} blocked, {} non-durable)\n",
            self.boundary_states.len(),
            blocked_states,
            non_durable_states
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Starter boundary states\n\n");
        for state in &self.boundary_states {
            let disclosure = state.disclosure();
            out.push_str(&format!(
                "- **{}** — boundary `{}` → `{}`, availability `{}` → `{}`, owner `{}`, freshness `{}`, source `{}`, deep link `{}`\n",
                state.state_name,
                state.boundary_kind.as_str(),
                disclosure.access_posture.as_str(),
                state.availability_state.as_str(),
                disclosure.availability_posture.as_str(),
                state.owner_class.as_str(),
                state.freshness_state.as_str(),
                state.source_class.as_str(),
                state.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in starter-boundary-state export.
#[derive(Debug)]
pub enum StarterBoundaryStateArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StarterBoundaryStateViolation>),
}

impl fmt::Display for StarterBoundaryStateArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "starter boundary state export parse failed: {error}"
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
                    "starter boundary state export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for StarterBoundaryStateArtifactError {}

/// Validation failures emitted by [`StarterBoundaryStateControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarterBoundaryStateViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No starter boundary states are present.
    BoundaryStatesMissing,
    /// A starter boundary state is incomplete.
    BoundaryStateIncomplete,
    /// A boundary state misrepresents its derived access / availability posture or claims.
    AccessPostureMisrepresented,
    /// A boundary state does not name its source.
    SourceLabelMissing,
    /// A boundary state does not name its owner.
    OwnerLabelMissing,
    /// A boundary state does not name its availability.
    AvailabilityNoteMissing,
    /// A boundary state does not name its freshness.
    FreshnessNoteMissing,
    /// A sign-in-gated boundary state does not name its sign-in step.
    SignInNoteMissing,
    /// A managed / remote boundary state does not name its managed dependency.
    ManagedNoteMissing,
    /// A non-durable boundary state does not name its non-durable staging.
    NonDurableNoteMissing,
    /// A mirror / offline-cache boundary state does not name its mirror or cache dependency.
    MirrorOrCacheNoteMissing,
    /// A not-reachable boundary state does not name its unavailable state.
    UnavailableNoteMissing,
    /// A boundary state does not disclose its trust step.
    TrustDisclosureNoteMissing,
    /// A boundary state does not disclose its install / network step.
    InstallDisclosureNoteMissing,
    /// A boundary state does not name its recovery route.
    RecoveryNoteMissing,
    /// A boundary state omits a mandatory review action.
    StateActionsIncomplete,
    /// A boundary state does not offer a real recovery route.
    RealRecoveryPathMissing,
    /// A boundary state does not preserve the continue-without-starter recovery verb.
    ContinueWithoutStarterMissing,
    /// The boundary states do not cover every boundary kind.
    BoundaryKindCoverageMissing,
    /// The boundary states do not cover every availability state.
    AvailabilityStateCoverageMissing,
    /// The boundary states do not cover every owner class.
    OwnerClassCoverageMissing,
    /// The boundary states do not cover every freshness state.
    FreshnessStateCoverageMissing,
    /// The boundary states do not cover every access posture.
    AccessPostureCoverageMissing,
    /// The boundary states do not cover every availability posture.
    AvailabilityPostureCoverageMissing,
    /// The boundary states do not cover every recovery verb.
    RecoveryVerbCoverageMissing,
    /// A boundary state does not name its context.
    ContextNoteMissing,
    /// A boundary state offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A boundary state names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A boundary state does not bind any disposition.
    DispositionsMissing,
    /// A boundary state does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A boundary state does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A boundary state does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A boundary state does not name any component family it applies to.
    AppliesToComponentsMissing,
    /// A boundary state hides its source or owner.
    StarterSourceOrOwnerHidden,
    /// A boundary state hides a mirror, offline, managed, remote, or non-durable dependency.
    MirrorOfflineOrManagedDependencyHidden,
    /// A boundary state performs a trust or install step silently.
    SilentTrustOrInstallPerformed,
    /// A boundary state omits a recovery or continue-without-starter path.
    RecoveryOrContinueWithoutStarterOmitted,
    /// A boundary state invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Boundary review does not satisfy required invariants.
    BoundaryReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl StarterBoundaryStateViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::BoundaryStatesMissing => "boundary_states_missing",
            Self::BoundaryStateIncomplete => "boundary_state_incomplete",
            Self::AccessPostureMisrepresented => "access_posture_misrepresented",
            Self::SourceLabelMissing => "source_label_missing",
            Self::OwnerLabelMissing => "owner_label_missing",
            Self::AvailabilityNoteMissing => "availability_note_missing",
            Self::FreshnessNoteMissing => "freshness_note_missing",
            Self::SignInNoteMissing => "sign_in_note_missing",
            Self::ManagedNoteMissing => "managed_note_missing",
            Self::NonDurableNoteMissing => "non_durable_note_missing",
            Self::MirrorOrCacheNoteMissing => "mirror_or_cache_note_missing",
            Self::UnavailableNoteMissing => "unavailable_note_missing",
            Self::TrustDisclosureNoteMissing => "trust_disclosure_note_missing",
            Self::InstallDisclosureNoteMissing => "install_disclosure_note_missing",
            Self::RecoveryNoteMissing => "recovery_note_missing",
            Self::StateActionsIncomplete => "state_actions_incomplete",
            Self::RealRecoveryPathMissing => "real_recovery_path_missing",
            Self::ContinueWithoutStarterMissing => "continue_without_starter_missing",
            Self::BoundaryKindCoverageMissing => "boundary_kind_coverage_missing",
            Self::AvailabilityStateCoverageMissing => "availability_state_coverage_missing",
            Self::OwnerClassCoverageMissing => "owner_class_coverage_missing",
            Self::FreshnessStateCoverageMissing => "freshness_state_coverage_missing",
            Self::AccessPostureCoverageMissing => "access_posture_coverage_missing",
            Self::AvailabilityPostureCoverageMissing => "availability_posture_coverage_missing",
            Self::RecoveryVerbCoverageMissing => "recovery_verb_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::AppliesToComponentsMissing => "applies_to_components_missing",
            Self::StarterSourceOrOwnerHidden => "starter_source_or_owner_hidden",
            Self::MirrorOfflineOrManagedDependencyHidden => {
                "mirror_offline_or_managed_dependency_hidden"
            }
            Self::SilentTrustOrInstallPerformed => "silent_trust_or_install_performed",
            Self::RecoveryOrContinueWithoutStarterOmitted => {
                "recovery_or_continue_without_starter_omitted"
            }
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::BoundaryReviewIncomplete => "boundary_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable starter-boundary-state export.
///
/// This is the first real consumer of the starter-boundary-state lane: a start-center, template
/// gallery, scaffold preflight, or support-export surface calls it to ingest the canonical
/// boundary states rather than cloning status text.
///
/// # Errors
///
/// Returns [`StarterBoundaryStateArtifactError`] when the checked-in support export fails to parse or
/// fails validation.
pub fn current_starter_boundary_state_export(
) -> Result<StarterBoundaryStateControlsPacket, StarterBoundaryStateArtifactError> {
    let packet: StarterBoundaryStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-starter-boundary-state-proof/support_export.json"
    )))
    .map_err(StarterBoundaryStateArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StarterBoundaryStateArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &StarterBoundaryStateControlsPacket,
    violations: &mut Vec<StarterBoundaryStateViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        STARTER_BOUNDARY_STATE_SCHEMA_REF,
        STARTER_BOUNDARY_STATE_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(StarterBoundaryStateViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_boundary_states(
    packet: &StarterBoundaryStateControlsPacket,
    violations: &mut Vec<StarterBoundaryStateViolation>,
) {
    if packet.boundary_states.is_empty() {
        violations.push(StarterBoundaryStateViolation::BoundaryStatesMissing);
        return;
    }

    let mut kinds: BTreeSet<StarterBoundaryKind> = BTreeSet::new();
    let mut availabilities: BTreeSet<StarterAvailabilityState> = BTreeSet::new();
    let mut owners: BTreeSet<StarterOwnerClass> = BTreeSet::new();
    let mut freshnesses: BTreeSet<StarterFreshnessState> = BTreeSet::new();
    let mut access_postures: BTreeSet<StarterAccessPosture> = BTreeSet::new();
    let mut availability_postures: BTreeSet<StarterAvailabilityPosture> = BTreeSet::new();
    let mut recovery_verbs: BTreeSet<StarterRecoveryVerb> = BTreeSet::new();

    for state in &packet.boundary_states {
        let disclosure = state.disclosure();
        kinds.insert(state.boundary_kind);
        availabilities.insert(state.availability_state);
        owners.insert(state.owner_class);
        freshnesses.insert(state.freshness_state);
        access_postures.insert(disclosure.access_posture);
        availability_postures.insert(disclosure.availability_posture);
        for verb in &state.recovery_verbs {
            recovery_verbs.insert(*verb);
        }

        if state.state_id.trim().is_empty()
            || state.state_name.trim().is_empty()
            || state.fields_shown.is_empty()
            || state.surface_families.is_empty()
            || state.deployment_lines.is_empty()
            || state.consumer_surfaces.is_empty()
            || state.source_contract_refs.is_empty()
            || state.recovery_verbs.is_empty()
        {
            violations.push(StarterBoundaryStateViolation::BoundaryStateIncomplete);
        }
        if state.derived_access_posture != disclosure.access_posture
            || state.derived_availability_posture != disclosure.availability_posture
            || state.claims_requires_sign_in != disclosure.requires_sign_in
            || state.claims_requires_managed_provisioning
                != disclosure.requires_managed_provisioning
            || state.claims_is_non_durable != disclosure.is_non_durable
            || state.claims_depends_on_mirror_or_cache != disclosure.depends_on_mirror_or_cache
            || state.claims_reachable != disclosure.is_reachable
        {
            violations.push(StarterBoundaryStateViolation::AccessPostureMisrepresented);
        }
        if state.source_label.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::SourceLabelMissing);
        }
        if state.owner_label.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::OwnerLabelMissing);
        }
        if state.availability_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::AvailabilityNoteMissing);
        }
        if state.freshness_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::FreshnessNoteMissing);
        }
        if disclosure.needs_sign_in_note && state.sign_in_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::SignInNoteMissing);
        }
        if disclosure.needs_managed_note && state.managed_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::ManagedNoteMissing);
        }
        if disclosure.needs_non_durable_note && state.non_durable_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::NonDurableNoteMissing);
        }
        if disclosure.needs_mirror_or_cache_note && state.mirror_or_cache_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::MirrorOrCacheNoteMissing);
        }
        if disclosure.needs_unavailable_note && state.unavailable_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::UnavailableNoteMissing);
        }
        if state.trust_disclosure_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::TrustDisclosureNoteMissing);
        }
        if state.install_disclosure_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::InstallDisclosureNoteMissing);
        }
        if state.recovery_note.trim().is_empty() {
            violations.push(StarterBoundaryStateViolation::RecoveryNoteMissing);
        }
        if !state.declares_mandatory_actions() {
            violations.push(StarterBoundaryStateViolation::StateActionsIncomplete);
        }
        if !state.offers_real_recovery() {
            violations.push(StarterBoundaryStateViolation::RealRecoveryPathMissing);
        }
        if !state.offers_continue_without_starter() {
            violations.push(StarterBoundaryStateViolation::ContinueWithoutStarterMissing);
        }
        if state.applies_to_components.is_empty() {
            violations.push(StarterBoundaryStateViolation::AppliesToComponentsMissing);
        }
        validate_deep_link(
            state.offers_deep_link_action(),
            state.deep_link_kind,
            &state.deep_link_ref,
            &state.context_note,
            violations,
        );
        validate_common_control(
            &state.dispositions,
            &state.downgrade_triggers,
            state.declares_mandatory_labels(),
            &state.accessibility_routes,
            ControlInvariants {
                hides_starter_source_or_owner: state.hides_starter_source_or_owner,
                hides_mirror_offline_or_managed_dependency: state
                    .hides_mirror_offline_or_managed_dependency,
                performs_silent_trust_or_install: state.performs_silent_trust_or_install,
                omits_recovery_or_continue_without_starter_path: state
                    .omits_recovery_or_continue_without_starter_path,
                invents_alternate_state_label: state.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in StarterBoundaryKind::ALL {
        if !kinds.contains(&required) {
            violations.push(StarterBoundaryStateViolation::BoundaryKindCoverageMissing);
            break;
        }
    }
    for required in StarterAvailabilityState::ALL {
        if !availabilities.contains(&required) {
            violations.push(StarterBoundaryStateViolation::AvailabilityStateCoverageMissing);
            break;
        }
    }
    for required in StarterOwnerClass::ALL {
        if !owners.contains(&required) {
            violations.push(StarterBoundaryStateViolation::OwnerClassCoverageMissing);
            break;
        }
    }
    for required in StarterFreshnessState::ALL {
        if !freshnesses.contains(&required) {
            violations.push(StarterBoundaryStateViolation::FreshnessStateCoverageMissing);
            break;
        }
    }
    for required in StarterAccessPosture::ALL {
        if !access_postures.contains(&required) {
            violations.push(StarterBoundaryStateViolation::AccessPostureCoverageMissing);
            break;
        }
    }
    for required in StarterAvailabilityPosture::ALL {
        if !availability_postures.contains(&required) {
            violations.push(StarterBoundaryStateViolation::AvailabilityPostureCoverageMissing);
            break;
        }
    }
    for required in StarterRecoveryVerb::ALL {
        if !recovery_verbs.contains(&required) {
            violations.push(StarterBoundaryStateViolation::RecoveryVerbCoverageMissing);
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by every boundary state.
///
/// A boundary state that offers a deep-link action must name a resolvable deep-link kind, a boundary
/// state that names a resolvable kind must carry its stable reference, and every boundary state must
/// name its context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<StarterBoundaryStateViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(StarterBoundaryStateViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(StarterBoundaryStateViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(StarterBoundaryStateViolation::DeepLinkRefMissing);
    }
}

/// The five hard-invariant bools every boundary state must keep `false`.
struct ControlInvariants {
    hides_starter_source_or_owner: bool,
    hides_mirror_offline_or_managed_dependency: bool,
    performs_silent_trust_or_install: bool,
    omits_recovery_or_continue_without_starter_path: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by every boundary state.
fn validate_common_control(
    dispositions: &[M5ScaffoldDisposition],
    downgrade_triggers: &[M5ScaffoldDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ScaffoldAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<StarterBoundaryStateViolation>,
) {
    if dispositions.is_empty() {
        violations.push(StarterBoundaryStateViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(StarterBoundaryStateViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(StarterBoundaryStateViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(StarterBoundaryStateViolation::AccessibilityRouteMissing);
    }
    if invariants.hides_starter_source_or_owner {
        violations.push(StarterBoundaryStateViolation::StarterSourceOrOwnerHidden);
    }
    if invariants.hides_mirror_offline_or_managed_dependency {
        violations.push(StarterBoundaryStateViolation::MirrorOfflineOrManagedDependencyHidden);
    }
    if invariants.performs_silent_trust_or_install {
        violations.push(StarterBoundaryStateViolation::SilentTrustOrInstallPerformed);
    }
    if invariants.omits_recovery_or_continue_without_starter_path {
        violations.push(StarterBoundaryStateViolation::RecoveryOrContinueWithoutStarterOmitted);
    }
    if invariants.invents_alternate_state_label {
        violations.push(StarterBoundaryStateViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical starter-boundary-state controls packet.
pub const STARTER_BOUNDARY_STATE_PACKET_ID: &str = "m5-starter-boundary-state-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn boundary_state_source_refs() -> Vec<String> {
    strings(&[
        STARTER_BOUNDARY_STATE_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
    ])
}

fn boundary_state_downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::StarterSourceUnstated,
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

/// The three mandatory labels plus the starter-source, side-effect, and recovery / ownership labels.
fn label_set() -> Vec<M5ScaffoldRequiredLabel> {
    let mut labels = M5ScaffoldRequiredLabel::MANDATORY.to_vec();
    labels.extend_from_slice(&[
        M5ScaffoldRequiredLabel::StarterSourceAndSupport,
        M5ScaffoldRequiredLabel::SideEffectDisclosure,
        M5ScaffoldRequiredLabel::RecoveryAndOwnershipBoundary,
    ]);
    labels
}

/// Input for [`boundary_state`], grouped so the seed builder stays under the argument limit and reads
/// as one boundary scenario.
struct BoundaryStateSeed<'a> {
    state_id: &'a str,
    state_name: &'a str,
    boundary_kind: StarterBoundaryKind,
    availability_state: StarterAvailabilityState,
    owner_class: StarterOwnerClass,
    freshness_state: StarterFreshnessState,
    source_class: M5StarterSourceClass,
    source_label: &'a str,
    owner_label: &'a str,
    availability_note: &'a str,
    freshness_note: &'a str,
    trust_disclosure_note: &'a str,
    install_disclosure_note: &'a str,
    recovery_note: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    state_actions: Vec<StarterStateAction>,
    recovery_verbs: Vec<StarterRecoveryVerb>,
    dispositions: Vec<M5ScaffoldDisposition>,
}

/// Builds a starter boundary state, deriving the access posture, availability posture, claims, and
/// the required conditional notes from the honest inputs so the seed is always self-consistent with
/// the resolver.
fn boundary_state(seed: BoundaryStateSeed<'_>) -> StarterBoundaryStateCard {
    let disclosure = resolve_starter_disclosure(seed.boundary_kind, seed.availability_state);
    StarterBoundaryStateCard {
        state_id: seed.state_id.to_owned(),
        state_name: seed.state_name.to_owned(),
        boundary_kind: seed.boundary_kind,
        availability_state: seed.availability_state,
        owner_class: seed.owner_class,
        freshness_state: seed.freshness_state,
        source_class: seed.source_class,
        source_label: seed.source_label.to_owned(),
        owner_label: seed.owner_label.to_owned(),
        availability_note: seed.availability_note.to_owned(),
        freshness_note: seed.freshness_note.to_owned(),
        derived_access_posture: disclosure.access_posture,
        derived_availability_posture: disclosure.availability_posture,
        claims_requires_sign_in: disclosure.requires_sign_in,
        claims_requires_managed_provisioning: disclosure.requires_managed_provisioning,
        claims_is_non_durable: disclosure.is_non_durable,
        claims_depends_on_mirror_or_cache: disclosure.depends_on_mirror_or_cache,
        claims_reachable: disclosure.is_reachable,
        sign_in_note: if disclosure.needs_sign_in_note {
            "A browser or device sign-in is required before this starter can be fetched".to_owned()
        } else {
            String::new()
        },
        managed_note: if disclosure.needs_managed_note {
            "This starter depends on a remote image or a managed workspace, provisioned remotely"
                .to_owned()
        } else {
            String::new()
        },
        non_durable_note: if disclosure.needs_non_durable_note {
            "This starter materializes into non-durable temporary staging that is not persisted"
                .to_owned()
        } else {
            String::new()
        },
        mirror_or_cache_note: if disclosure.needs_mirror_or_cache_note {
            "This starter is served through a mirror or an offline cache, not the public registry"
                .to_owned()
        } else {
            String::new()
        },
        unavailable_note: if disclosure.needs_unavailable_note {
            "This starter is not reachable; recover or continue without a starter".to_owned()
        } else {
            String::new()
        },
        trust_disclosure_note: seed.trust_disclosure_note.to_owned(),
        install_disclosure_note: seed.install_disclosure_note.to_owned(),
        recovery_note: seed.recovery_note.to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        state_actions: seed.state_actions,
        recovery_verbs: seed.recovery_verbs,
        applies_to_components: M5ScaffoldComponentFamily::ALL.to_vec(),
        dispositions: seed.dispositions,
        downgrade_triggers: boundary_state_downgrade_triggers(),
        required_labels: label_set(),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "boundary_kind",
            "availability_state",
            "owner_label",
            "freshness_state",
            "source_label",
            "trust_disclosure_note",
            "install_disclosure_note",
            "recovery_note",
        ]),
        source_contract_refs: boundary_state_source_refs(),
        hides_starter_source_or_owner: false,
        hides_mirror_offline_or_managed_dependency: false,
        performs_silent_trust_or_install: false,
        omits_recovery_or_continue_without_starter_path: false,
        invents_alternate_state_label: false,
    }
}

fn boundary_states() -> Vec<StarterBoundaryStateCard> {
    use DeepLinkKind as Link;
    use M5ScaffoldDisposition as Disp;
    use M5StarterSourceClass as Source;
    use StarterAvailabilityState as Avail;
    use StarterBoundaryKind as Kind;
    use StarterFreshnessState as Fresh;
    use StarterOwnerClass as Owner;
    use StarterRecoveryVerb as Recover;
    use StarterStateAction as Action;

    let review_actions = || {
        vec![
            Action::ReviewSourceAndOwner,
            Action::ReviewAvailabilityAndFreshness,
            Action::ReviewTrustAndInstallSteps,
            Action::ChooseRecovery,
            Action::OpenDeepLink,
        ]
    };

    vec![
        // 1. Public registry / available -> direct public access + reachable now.
        boundary_state(BoundaryStateSeed {
            state_id: "boundary-public-registry",
            state_name: "Public registry starter",
            boundary_kind: Kind::PublicRegistry,
            availability_state: Avail::Available,
            owner_class: Owner::FirstPartyRegistry,
            freshness_state: Fresh::Live,
            source_class: Source::FirstPartyStarter,
            source_label: "First-party registry: aureline/react-spa",
            owner_label: "Aureline first-party registry",
            availability_note: "Reachable directly from the public registry right now",
            freshness_note: "Live from the first-party registry",
            trust_disclosure_note:
                "This starter will prompt for trust before it runs any generated code",
            install_disclosure_note:
                "This starter installs 18 dependencies from the public registry; the install step is shown first",
            recovery_note:
                "Delete generated output, reuse existing files, or continue without a starter",
            context_note: "A plain public-registry starter; the install step is still disclosed first",
            deep_link_kind: Link::TemplateManifest,
            deep_link_ref: "manifest:starters/react-spa#boundary",
            state_actions: vec![
                Action::ReviewSourceAndOwner,
                Action::ReviewAvailabilityAndFreshness,
                Action::ReviewTrustAndInstallSteps,
                Action::ProceedWithDisclosure,
                Action::ChooseRecovery,
                Action::OpenDeepLink,
            ],
            recovery_verbs: vec![
                Recover::DeleteGenerated,
                Recover::ReuseExisting,
                Recover::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::FirstParty],
        }),
        // 2. Mirror only / mirror reachable -> mirror-mediated + reachable via mirror.
        boundary_state(BoundaryStateSeed {
            state_id: "boundary-mirror-only",
            state_name: "Mirror-only starter",
            boundary_kind: Kind::MirrorOnly,
            availability_state: Avail::MirrorReachableOnly,
            owner_class: Owner::TeamMirror,
            freshness_state: Fresh::MirrorSynced,
            source_class: Source::MirroredStarter,
            source_label: "Team mirror: mirror.internal/react-spa",
            owner_label: "Team-operated mirror",
            availability_note: "Reachable only through the team mirror, not the public registry",
            freshness_note: "Synced from the mirror on its last refresh",
            trust_disclosure_note:
                "This starter will prompt for trust before it runs any mirrored code",
            install_disclosure_note:
                "This starter installs dependencies from the team mirror; the mirror install step is shown first",
            recovery_note:
                "Delete generated output, retry when available, or continue without a starter",
            context_note: "A mirror-only starter; the public registry is not used",
            deep_link_kind: Link::StarterRegistryEntry,
            deep_link_ref: "registry:team/mirror-starter#boundary",
            state_actions: review_actions(),
            recovery_verbs: vec![
                Recover::DeleteGenerated,
                Recover::RetryWhenAvailable,
                Recover::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::TeamManaged],
        }),
        // 3. Offline cache only / cache offline -> offline-cache backed + reachable from cache.
        boundary_state(BoundaryStateSeed {
            state_id: "boundary-offline-cache",
            state_name: "Offline-cache-only starter",
            boundary_kind: Kind::OfflineCacheOnly,
            availability_state: Avail::CacheOnlyOffline,
            owner_class: Owner::LocalCache,
            freshness_state: Fresh::CacheStale,
            source_class: Source::LocalOnlyStarter,
            source_label: "Local offline cache: ~/.aureline/cache/react-spa",
            owner_label: "Local offline cache",
            availability_note: "Reachable only from the offline cache while the network is down",
            freshness_note: "Served from a stale cache; it may be behind the registry",
            trust_disclosure_note:
                "This starter will prompt for trust before it runs any cached code",
            install_disclosure_note:
                "This starter installs dependencies from the offline cache; nothing is fetched from the network",
            recovery_note:
                "Reuse existing cached files, clone elsewhere, or continue without a starter",
            context_note: "An offline-cache-only starter; content may be stale",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/offline-cache",
            state_actions: review_actions(),
            recovery_verbs: vec![
                Recover::ReuseExisting,
                Recover::CloneElsewhere,
                Recover::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::LocalOnly],
        }),
        // 4. Sign-in required / sign-in pending -> auth-gated + blocked pending sign-in.
        boundary_state(BoundaryStateSeed {
            state_id: "boundary-sign-in-required",
            state_name: "Sign-in-required starter",
            boundary_kind: Kind::SignInRequired,
            availability_state: Avail::SignInPending,
            owner_class: Owner::ManagedService,
            freshness_state: Fresh::FreshnessUnknown,
            source_class: Source::TeamManagedStarter,
            source_label: "Managed service: starters.company.example",
            owner_label: "Managed starter service",
            availability_note: "Blocked until a browser or device sign-in completes",
            freshness_note: "Freshness is unknown until sign-in resolves",
            trust_disclosure_note:
                "This starter will prompt for trust; the sign-in step is shown before any prompt",
            install_disclosure_note:
                "This starter installs dependencies after sign-in; the install step is shown first",
            recovery_note:
                "Delete generated output, clone elsewhere, or continue without a starter",
            context_note: "A sign-in-required starter; nothing is fetched before sign-in",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/sign-in",
            state_actions: review_actions(),
            recovery_verbs: vec![
                Recover::DeleteGenerated,
                Recover::CloneElsewhere,
                Recover::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::Blocked],
        }),
        // 5. Remote / managed workspace / provisioning pending -> managed-remote + blocked pending
        //    provisioning.
        boundary_state(BoundaryStateSeed {
            state_id: "boundary-managed-workspace",
            state_name: "Managed-workspace starter",
            boundary_kind: Kind::RemoteOrManagedWorkspace,
            availability_state: Avail::ProvisioningPending,
            owner_class: Owner::ManagedService,
            freshness_state: Fresh::Live,
            source_class: Source::TeamManagedStarter,
            source_label: "Managed workspace image: managed/devbox-node",
            owner_label: "Managed workspace service",
            availability_note: "Blocked until the remote workspace finishes provisioning",
            freshness_note: "Live image; provisioning is still in progress",
            trust_disclosure_note:
                "This starter will prompt for trust before the managed workspace runs code",
            install_disclosure_note:
                "This starter provisions a remote managed workspace; the provisioning step is shown first",
            recovery_note:
                "Delete generated output, retry when available, or continue without a starter",
            context_note: "A managed-workspace starter; provisioning happens remotely",
            deep_link_kind: Link::PolicyReference,
            deep_link_ref: "policy:workspace/managed-provisioning",
            state_actions: review_actions(),
            recovery_verbs: vec![
                Recover::DeleteGenerated,
                Recover::RetryWhenAvailable,
                Recover::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::TeamManaged],
        }),
        // 6. Non-durable temp staging / unavailable -> non-durable staging + not reachable.
        boundary_state(BoundaryStateSeed {
            state_id: "boundary-non-durable-staging",
            state_name: "Non-durable temporary-staging starter",
            boundary_kind: Kind::NonDurableTempStaging,
            availability_state: Avail::Unavailable,
            owner_class: Owner::UnknownOwner,
            freshness_state: Fresh::Ephemeral,
            source_class: Source::UnknownSourceStarter,
            source_label: "Temporary staging: tmp staging area (non-durable)",
            owner_label: "Owner unknown",
            availability_note: "The staging source is not reachable and cannot be refetched",
            freshness_note: "Ephemeral; the staging area is not persisted across restarts",
            trust_disclosure_note:
                "This starter will prompt for trust before it runs any staged code",
            install_disclosure_note:
                "This starter stages files in non-durable temporary storage; the staging step is shown first",
            recovery_note:
                "Delete generated output, clone elsewhere, reuse existing, or continue without a starter",
            context_note: "A non-durable staging starter; output is not persisted",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:templates/non-durable-staging",
            state_actions: review_actions(),
            recovery_verbs: vec![
                Recover::DeleteGenerated,
                Recover::CloneElsewhere,
                Recover::ReuseExisting,
                Recover::ContinueWithoutStarter,
            ],
            dispositions: vec![Disp::Warning],
        }),
    ]
}

fn downgrade_triggers() -> Vec<M5ScaffoldDowngradeTrigger> {
    vec![
        M5ScaffoldDowngradeTrigger::StarterSourceUnstated,
        M5ScaffoldDowngradeTrigger::SupportClassUnstated,
        M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
        M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
        M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
        M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
        M5ScaffoldDowngradeTrigger::ProofStale,
    ]
}

fn boundary_review() -> StarterBoundaryReview {
    StarterBoundaryReview {
        state_names_source_and_owner: true,
        state_names_mirror_offline_managed_or_staging_dependency: true,
        state_names_availability_and_freshness: true,
        trust_step_disclosed_before_prompt: true,
        install_step_disclosed_before_run: true,
        access_and_availability_posture_derived_never_asserted: true,
        unavailable_or_blocked_never_shown_as_ready: true,
        non_durable_staging_never_shown_as_durable: true,
        managed_or_remote_dependency_never_hidden: true,
        recovery_verbs_preserve_delete_reuse_clone_continue: true,
        continue_without_starter_always_available: true,
        no_surface_invents_alternate_state_label: true,
        states_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> StarterBoundaryConsumerProjection {
    StarterBoundaryConsumerProjection {
        start_center_reads_single_source: true,
        template_gallery_reads_single_source: true,
        preflight_reads_single_source: true,
        boundary_visible_before_trust_or_install: true,
        recovery_path_visible_on_failure: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> StarterBoundaryProofFreshness {
    StarterBoundaryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        STARTER_BOUNDARY_STATE_SCHEMA_REF,
        STARTER_BOUNDARY_STATE_DOC_REF,
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
    ])
}

/// Builds the canonical starter-boundary-state controls packet.
pub fn seeded_starter_boundary_states() -> StarterBoundaryStateControlsPacket {
    StarterBoundaryStateControlsPacket::new(StarterBoundaryStateControlsPacketInput {
        packet_id: STARTER_BOUNDARY_STATE_PACKET_ID.to_owned(),
        surface_label:
            "M5 starter boundary states: mirror-only, offline-cache-only, sign-in-required, remote/managed-workspace, and non-durable temporary-staging honesty with no silent trust or install across claimed scaffold surfaces"
                .to_owned(),
        boundary_states: boundary_states(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        boundary_review: boundary_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights the mirror-only and offline-cache-only boundary states that must
/// name their mirror / offline dependency rather than reading as a plain public-registry create.
/// Every boundary kind, availability state, owner class, freshness state, access posture,
/// availability posture, and recovery verb stays covered so the fixture validates on its own.
pub fn seeded_starter_boundary_states_mirror_only_offline() -> StarterBoundaryStateControlsPacket {
    let mut packet = seeded_starter_boundary_states();
    packet.packet_id = "m5-starter-boundary-state-controls:fixture:mirror-only-offline".to_owned();
    packet.surface_label =
        "M5 starter boundary states: a mirror-only or offline-cache-only starter never reads as a plain public-registry create"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights the sign-in-required boundary state that must disclose its sign-in
/// step before any silent trust or install occurs. Every boundary kind, availability state, owner
/// class, freshness state, access posture, availability posture, and recovery verb stays covered so
/// the fixture validates on its own.
pub fn seeded_starter_boundary_states_sign_in_required() -> StarterBoundaryStateControlsPacket {
    let mut packet = seeded_starter_boundary_states();
    packet.packet_id = "m5-starter-boundary-state-controls:fixture:sign-in-required".to_owned();
    packet.surface_label =
        "M5 starter boundary states: a sign-in-required starter discloses its sign-in step before any trust or install"
            .to_owned();
    packet
}
