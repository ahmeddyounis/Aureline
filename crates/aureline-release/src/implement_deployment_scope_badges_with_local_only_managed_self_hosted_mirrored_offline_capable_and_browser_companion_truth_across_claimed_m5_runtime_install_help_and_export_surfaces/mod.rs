//! One reusable M5 deployment-scope badge primitive: the operating mode a capability
//! runs / is available in (Local only / Managed / Self-hosted / Mirrored /
//! Offline-capable / Browser companion), projected the same way across every claimed M5
//! runtime, install/deployment, Help/About, diagnostics, export, and companion consumer
//! — as one distinct, composable cue that never collapses into support class, lifecycle,
//! or channel status.
//!
//! Aureline's frozen badge-family matrix
//! ([`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`])
//! names the deployment-scope badge as one of the six governed badge families and freezes
//! the shared badge infrastructure — the surface families, the deployment lines, the
//! accessibility routes, the qualification classes, the explanation-drawer fields, the
//! consumer surfaces, and the downgrade triggers. This module *implements* that family as
//! one render-facing badge so a user can tell — from the badge and its explanation
//! drawers alone — exactly which operating mode a capability runs in, *and* what residual
//! dependency and local-safe continuity that mode still carries, without the deployment
//! scope overstating sovereignty, offline readiness, or client authority.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_deployment_scope_badge`] — that takes one capability's
//!    subject label, its declared deployment scope, an optional residual-dependency
//!    disclosure, and its last-evaluated timestamp, and produces one
//!    [`M5ResolvedDeploymentScopeBadge`] carrying the scope as its own typed field, the
//!    derived sovereignty posture (locally sovereign / provider governed / operator
//!    governed / mirror synced / offline resilient / host delegated), and — whenever the
//!    scope makes a local, offline, self-hosted, mirrored, or browser-companion authority
//!    claim — a self-contained [`M5ResidualDependencyNote`] that names the exact residual
//!    dependency, the local-safe continuity guarantee, the next action, and the
//!    *preserved* scope context. The resolver never collapses the scope into support
//!    class, lifecycle, or channel, never derives the lifecycle from the scope, and never
//!    lets a local/offline/self-host/mirror/companion badge overstate sovereignty by
//!    omitting its residual dependency.
//! 2. A parity matrix — [`M5DeploymentScopeBadgePrimitivePacket`] — that binds one row
//!    per claimed M5 badge consumer (the runtime capability row, the install/deployment
//!    card, the Help/About panel, the diagnostics report, the support-export row, and the
//!    companion-mode card) to the shared badge anatomy, the same scope values, sovereignty
//!    postures, residual-dependency classes, local-safe continuities, next actions,
//!    explanation-drawer fields, export fields, and non-visual accessibility routes, so
//!    the deployment-scope vocabulary stays identical across runtime, install, Help/About,
//!    diagnostics, export, and companion surfaces.
//!
//! The badge surface family ([`M5BadgeSurfaceFamily`]), deployment line
//! ([`M5DeploymentLine`]), accessibility route ([`M5BadgeAccessibilityRoute`]),
//! qualification class ([`M5BadgeQualificationClass`]), explanation-drawer field
//! ([`M5BadgeExplanationField`]), consumer surface ([`M5BadgeConsumerSurface`]), and
//! downgrade trigger ([`M5BadgeDowngradeTrigger`]) are reused verbatim from the frozen
//! badge-family matrix. This module mints new vocabulary only for what that matrix left
//! implicit about the rendered deployment-scope badge itself: its render-facing value set,
//! its badge consumers, its anatomy parts, its sovereignty postures, its residual
//! dependencies, its local-safe continuities, its next actions, and its export fields. No
//! M5 badge surface invents a second deployment-scope grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
//! bodies stay outside the support boundary; every subject label, residual-dependency
//! disclosure, and timestamp is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-deployment-scope-badge.schema.json`](../../../../schemas/ui/m5-deployment-scope-badge.schema.json)
//! and the contract doc is
//! [`docs/release/m5_deployment_scope_badge_contract.md`](../../../../docs/release/m5_deployment_scope_badge_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-deployment-scope-badges/`](../../../../fixtures/ui/m5-deployment-scope-badges/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed,
    seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed,
    seeded_m5_deployment_scope_badge_primitive_packet,
    M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_PACKET_ID,
};

// The surface families, deployment lines, accessibility routes, qualification classes,
// explanation-drawer fields, consumer surfaces, and downgrade triggers are frozen once,
// in the badge-family matrix. This primitive reuses them verbatim so it never invents a
// parallel badge grammar for the shared badge infrastructure.
pub use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    M5BadgeAccessibilityRoute, M5BadgeConsumerSurface, M5BadgeDowngradeTrigger,
    M5BadgeExplanationField, M5BadgeQualificationClass, M5BadgeSurfaceFamily, M5DeploymentLine,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DeploymentScopeBadgePrimitivePacket`].
pub const M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_deployment_scope_badges_across_claimed_m5_runtime_install_help_and_export_surfaces";

/// Schema version for M5 deployment-scope badge records.
pub const M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the deployment-scope badge boundary schema.
pub const M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF: &str =
    "schemas/ui/m5-deployment-scope-badge.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DEPLOYMENT_SCOPE_BADGE_DOC_REF: &str =
    "docs/release/m5_deployment_scope_badge_contract.md";

/// Repo-relative path of the frozen badge-family matrix this primitive narrows from.
pub const M5_DEPLOYMENT_SCOPE_BADGE_FAMILY_MATRIX_REF: &str =
    "schemas/ui/m5-badge-family-matrix.schema.json";

/// Repo-relative path of the residual-dependency row this primitive projects
/// residual-dependency truth from.
pub const M5_DEPLOYMENT_SCOPE_BADGE_RESIDUAL_REF: &str =
    "schemas/ui/m5-residual-dependency-row.schema.json";

/// Repo-relative path of the mirror / offline artifact row this primitive projects
/// offline-readiness truth from.
pub const M5_DEPLOYMENT_SCOPE_BADGE_MIRROR_OFFLINE_REF: &str =
    "schemas/ui/m5-mirror-offline-artifact-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DEPLOYMENT_SCOPE_BADGE_FIXTURE_DIR: &str = "fixtures/ui/m5-deployment-scope-badges";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DEPLOYMENT_SCOPE_BADGE_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-scope-badge-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DEPLOYMENT_SCOPE_BADGE_CSV_REF: &str =
    "artifacts/release/m5-deployment-scope-badge-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DEPLOYMENT_SCOPE_BADGE_REPORT_REF: &str =
    "artifacts/components/m5-deployment-scope-badges.md";

/// One claimed M5 badge consumer that renders the shared deployment-scope badge. These
/// are the surfaces the implementation requirements name — runtime, install/deployment,
/// Help/About, diagnostics, export, and companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeConsumerSurface {
    /// A runtime capability row.
    RuntimeCapabilityRow,
    /// An install / deployment summary card.
    InstallDeploymentCard,
    /// The Help / About panel.
    HelpAboutPanel,
    /// A diagnostics report.
    DiagnosticsReport,
    /// A support-export row.
    SupportExportRow,
    /// The companion-mode card.
    CompanionModeCard,
}

impl M5DeploymentScopeConsumerSurface {
    /// Every claimed badge consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RuntimeCapabilityRow,
        Self::InstallDeploymentCard,
        Self::HelpAboutPanel,
        Self::DiagnosticsReport,
        Self::SupportExportRow,
        Self::CompanionModeCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeCapabilityRow => "runtime_capability_row",
            Self::InstallDeploymentCard => "install_deployment_card",
            Self::HelpAboutPanel => "help_about_panel",
            Self::DiagnosticsReport => "diagnostics_report",
            Self::SupportExportRow => "support_export_row",
            Self::CompanionModeCard => "companion_mode_card",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RuntimeCapabilityRow => "Runtime Capability Row",
            Self::InstallDeploymentCard => "Install / Deployment Card",
            Self::HelpAboutPanel => "Help / About Panel",
            Self::DiagnosticsReport => "Diagnostics Report",
            Self::SupportExportRow => "Support Export Row",
            Self::CompanionModeCard => "Companion Mode Card",
        }
    }
}

/// Controlled deployment-scope badge value — which operating mode a capability runs in.
/// This is the render-facing deployment-scope vocabulary the acceptance criteria name:
/// Local only, Managed, Self-hosted, Mirrored, Offline-capable, Browser companion. A
/// deployment-scope badge never leaves its scope implicit and never implies a support
/// level, lifecycle stage, or channel — a Local-only capability is not "experimental" and
/// a Browser-companion capability is not a hidden footnote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeBadgeValue {
    /// Local only: runs entirely on the local machine.
    LocalOnly,
    /// Managed: runs in a provider-managed deployment.
    Managed,
    /// Self-hosted: runs on operator-owned infrastructure.
    SelfHosted,
    /// Mirrored: served from a mirror of an upstream source.
    Mirrored,
    /// Offline-capable: keeps working offline within a cached capability window.
    OfflineCapable,
    /// Browser companion: runs as a companion delegated to the host browser.
    BrowserCompanion,
}

impl M5DeploymentScopeBadgeValue {
    /// Every deployment-scope value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnly,
        Self::Managed,
        Self::SelfHosted,
        Self::Mirrored,
        Self::OfflineCapable,
        Self::BrowserCompanion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Mirrored => "mirrored",
            Self::OfflineCapable => "offline_capable",
            Self::BrowserCompanion => "browser_companion",
        }
    }

    /// Review-safe label for the badge and note.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::Managed => "Managed",
            Self::SelfHosted => "Self-hosted",
            Self::Mirrored => "Mirrored",
            Self::OfflineCapable => "Offline-capable",
            Self::BrowserCompanion => "Browser companion",
        }
    }
}

/// The derived sovereignty posture — the resolver's verdict about how much authority a
/// capability's operating mode actually holds, computed from the deployment scope alone so
/// it never implies or is implied by the support class, lifecycle, or channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentSovereigntyPosture {
    /// Locally sovereign: authoritative on the local machine.
    LocallySovereign,
    /// Provider governed: authority rests with the managing provider.
    ProviderGoverned,
    /// Operator governed: authority rests with the self-hosting operator.
    OperatorGoverned,
    /// Mirror synced: authoritative only up to the last mirror sync.
    MirrorSynced,
    /// Offline resilient: authoritative only within the cached capability window.
    OfflineResilient,
    /// Host delegated: authority is delegated to the host browser runtime.
    HostDelegated,
}

impl M5DeploymentSovereigntyPosture {
    /// Every sovereignty posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocallySovereign,
        Self::ProviderGoverned,
        Self::OperatorGoverned,
        Self::MirrorSynced,
        Self::OfflineResilient,
        Self::HostDelegated,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocallySovereign => "locally_sovereign",
            Self::ProviderGoverned => "provider_governed",
            Self::OperatorGoverned => "operator_governed",
            Self::MirrorSynced => "mirror_synced",
            Self::OfflineResilient => "offline_resilient",
            Self::HostDelegated => "host_delegated",
        }
    }

    /// True when the operating mode is openly provider-governed and therefore makes no
    /// local-authority claim to overstate.
    pub const fn is_provider_governed(self) -> bool {
        matches!(self, Self::ProviderGoverned)
    }

    /// True when the operating mode makes a local, offline, self-host, mirror, or
    /// browser-companion authority claim and must therefore disclose a residual
    /// dependency rather than overstate its sovereignty.
    pub const fn makes_local_authority_claim(self) -> bool {
        !self.is_provider_governed()
    }

    /// True when the operating mode is an offline or mirror mode — a product truth the
    /// badge must state explicitly rather than hide in a footnote.
    pub const fn is_offline_or_mirror(self) -> bool {
        matches!(self, Self::MirrorSynced | Self::OfflineResilient)
    }

    /// True when the operating mode is the browser companion — a product truth the badge
    /// must state explicitly rather than hide in a footnote.
    pub const fn is_browser_companion(self) -> bool {
        matches!(self, Self::HostDelegated)
    }

    /// The residual-dependency class this sovereignty claim carries, if any. Returns
    /// `None` for a provider-governed posture, which makes no local-authority claim.
    pub const fn residual_dependency_class(self) -> Option<M5ResidualDependencyClass> {
        Some(match self {
            Self::LocallySovereign => M5ResidualDependencyClass::SigningAndUpdateChannel,
            Self::OperatorGoverned => M5ResidualDependencyClass::OperatorInfrastructure,
            Self::MirrorSynced => M5ResidualDependencyClass::UpstreamMirrorSync,
            Self::OfflineResilient => M5ResidualDependencyClass::CachedCapabilityWindow,
            Self::HostDelegated => M5ResidualDependencyClass::HostBrowserRuntime,
            Self::ProviderGoverned => return None,
        })
    }
}

/// The exact residual dependency a sovereignty-claiming scope still carries, so a
/// local-safe continuity note never reads like an unqualified "fully sovereign" claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResidualDependencyClass {
    /// Local only, but still relies on the signing and update channel.
    SigningAndUpdateChannel,
    /// Self-hosted, but still relies on operator infrastructure.
    OperatorInfrastructure,
    /// Mirrored, but still relies on upstream mirror sync.
    UpstreamMirrorSync,
    /// Offline-capable, but only within a cached capability window.
    CachedCapabilityWindow,
    /// Browser companion, delegated to the host browser runtime.
    HostBrowserRuntime,
}

impl M5ResidualDependencyClass {
    /// Every residual-dependency class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SigningAndUpdateChannel,
        Self::OperatorInfrastructure,
        Self::UpstreamMirrorSync,
        Self::CachedCapabilityWindow,
        Self::HostBrowserRuntime,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SigningAndUpdateChannel => "signing_and_update_channel",
            Self::OperatorInfrastructure => "operator_infrastructure",
            Self::UpstreamMirrorSync => "upstream_mirror_sync",
            Self::CachedCapabilityWindow => "cached_capability_window",
            Self::HostBrowserRuntime => "host_browser_runtime",
        }
    }

    /// Review-safe phrase naming exactly what the sovereignty claim still depends on, so
    /// the badge never overstates sovereignty, offline readiness, or client authority.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::SigningAndUpdateChannel => {
                "runs locally but still relies on the signing and update channel"
            }
            Self::OperatorInfrastructure => {
                "is self-hosted but still relies on operator infrastructure"
            }
            Self::UpstreamMirrorSync => "serves from a mirror but still relies on upstream sync",
            Self::CachedCapabilityWindow => {
                "works offline but only within its cached capability window"
            }
            Self::HostBrowserRuntime => {
                "is a browser companion delegated to the host browser runtime"
            }
        }
    }

    /// True when this residual dependency is an offline-readiness claim (mirror or cached
    /// window) rather than a plain local dependency.
    pub const fn is_offline_readiness_claim(self) -> bool {
        matches!(
            self,
            Self::UpstreamMirrorSync | Self::CachedCapabilityWindow
        )
    }

    /// True when this residual dependency is the browser-companion host runtime.
    pub const fn is_browser_companion(self) -> bool {
        matches!(self, Self::HostBrowserRuntime)
    }

    /// The local-safe continuity guarantee this residual dependency preserves.
    pub const fn local_safe_continuity(self) -> M5LocalSafeContinuity {
        match self {
            Self::SigningAndUpdateChannel | Self::OperatorInfrastructure => {
                M5LocalSafeContinuity::ContinuesFullyLocal
            }
            Self::UpstreamMirrorSync => M5LocalSafeContinuity::ContinuesWithLastMirroredState,
            Self::CachedCapabilityWindow => M5LocalSafeContinuity::ContinuesWithCachedWindow,
            Self::HostBrowserRuntime => M5LocalSafeContinuity::ContinuesWithinHostSession,
        }
    }

    /// The next action a reviewer should take to confirm this residual dependency.
    pub const fn next_action(self) -> M5DeploymentScopeNextAction {
        match self {
            Self::SigningAndUpdateChannel | Self::OperatorInfrastructure => {
                M5DeploymentScopeNextAction::ReviewResidualDependency
            }
            Self::UpstreamMirrorSync | Self::CachedCapabilityWindow => {
                M5DeploymentScopeNextAction::ConfirmOfflineReadinessWindow
            }
            Self::HostBrowserRuntime => M5DeploymentScopeNextAction::ConfirmHostCompanionScope,
        }
    }
}

/// The local-safe continuity guarantee a sovereignty-claiming scope preserves — what
/// keeps working locally even though the scope carries a residual dependency, so a badge
/// states its continuity honestly instead of overstating offline readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalSafeContinuity {
    /// Continues fully local (local-only / self-hosted authority).
    ContinuesFullyLocal,
    /// Continues with the last mirrored state (mirror mode).
    ContinuesWithLastMirroredState,
    /// Continues within the cached capability window (offline mode).
    ContinuesWithCachedWindow,
    /// Continues within the host browser session (browser companion).
    ContinuesWithinHostSession,
}

impl M5LocalSafeContinuity {
    /// Every local-safe continuity guarantee, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ContinuesFullyLocal,
        Self::ContinuesWithLastMirroredState,
        Self::ContinuesWithCachedWindow,
        Self::ContinuesWithinHostSession,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuesFullyLocal => "continues_fully_local",
            Self::ContinuesWithLastMirroredState => "continues_with_last_mirrored_state",
            Self::ContinuesWithCachedWindow => "continues_with_cached_window",
            Self::ContinuesWithinHostSession => "continues_within_host_session",
        }
    }
}

/// The next action named on a residual-dependency note, so a local/offline/companion
/// badge is actionable from the note itself rather than being an inert claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeNextAction {
    /// Review the residual dependency this scope still carries.
    ReviewResidualDependency,
    /// Confirm the offline / mirror readiness window before relying on it offline.
    ConfirmOfflineReadinessWindow,
    /// Confirm the host companion scope and its delegated authority.
    ConfirmHostCompanionScope,
}

impl M5DeploymentScopeNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReviewResidualDependency,
        Self::ConfirmOfflineReadinessWindow,
        Self::ConfirmHostCompanionScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewResidualDependency => "review_residual_dependency",
            Self::ConfirmOfflineReadinessWindow => "confirm_offline_readiness_window",
            Self::ConfirmHostCompanionScope => "confirm_host_companion_scope",
        }
    }
}

/// One anatomy part the shared deployment-scope badge surfaces. The parts in
/// [`M5DeploymentScopeAnatomyPart::MANDATORY`] are required on every consumer so the scope
/// stays a distinct cue with its own explanation and residual-dependency drawers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeAnatomyPart {
    /// The deployment-scope badge itself.
    ScopeBadge,
    /// The scope explanation drawer.
    ScopeExplanationDrawer,
    /// The residual-dependency / local-safe continuity drawer.
    ResidualDependencyDrawer,
    /// The separately-filterable filter keys for the scope axis.
    FilterKeys,
    /// The derived sovereignty-posture note.
    SovereigntyPostureNote,
    /// The residual-dependency continuity banner (shown when the scope claims local
    /// authority).
    ContinuityBanner,
    /// The offline / mirror readiness note.
    OfflineReadinessNote,
}

impl M5DeploymentScopeAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ScopeBadge,
        Self::ScopeExplanationDrawer,
        Self::ResidualDependencyDrawer,
        Self::FilterKeys,
        Self::SovereigntyPostureNote,
        Self::ContinuityBanner,
        Self::OfflineReadinessNote,
    ];

    /// The anatomy parts every badge consumer must render: the badge, both drawers, and
    /// the sovereignty-posture note.
    pub const MANDATORY: [Self; 4] = [
        Self::ScopeBadge,
        Self::ScopeExplanationDrawer,
        Self::ResidualDependencyDrawer,
        Self::SovereigntyPostureNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeBadge => "scope_badge",
            Self::ScopeExplanationDrawer => "scope_explanation_drawer",
            Self::ResidualDependencyDrawer => "residual_dependency_drawer",
            Self::FilterKeys => "filter_keys",
            Self::SovereigntyPostureNote => "sovereignty_posture_note",
            Self::ContinuityBanner => "continuity_banner",
            Self::OfflineReadinessNote => "offline_readiness_note",
        }
    }
}

/// A field the support / export packet carries so deployment-scope truth is
/// reconstructable from the shared model. The fields in
/// [`M5DeploymentScopeExportField::MANDATORY`] are required, and the scope, the residual
/// dependency, and the local-safe continuity are always carried as *separate* fields so
/// exported evidence never loses badge meaning or drops the residual dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeExportField {
    /// The deployment-scope value.
    Scope,
    /// The derived sovereignty posture.
    SovereigntyPosture,
    /// The residual-dependency class (when the scope makes a local-authority claim).
    ResidualDependencyClass,
    /// The residual-dependency disclosure (when the scope makes a local-authority claim).
    ResidualDependency,
    /// The local-safe continuity guarantee.
    LocalSafeContinuity,
    /// The scope explanation.
    ScopeExplanation,
    /// The opaque last-evaluated timestamp.
    LastEvaluated,
    /// The next action.
    NextAction,
    /// The separately-filterable filter keys.
    FilterKeys,
}

impl M5DeploymentScopeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Scope,
        Self::SovereigntyPosture,
        Self::ResidualDependencyClass,
        Self::ResidualDependency,
        Self::LocalSafeContinuity,
        Self::ScopeExplanation,
        Self::LastEvaluated,
        Self::NextAction,
        Self::FilterKeys,
    ];

    /// The export fields every badge export must carry: the scope axis, the sovereignty
    /// posture, the residual dependency, and the local-safe continuity so an offline or
    /// browser-companion badge keeps its residual dependency in exported evidence.
    pub const MANDATORY: [Self; 4] = [
        Self::Scope,
        Self::SovereigntyPosture,
        Self::ResidualDependency,
        Self::LocalSafeContinuity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scope => "scope",
            Self::SovereigntyPosture => "sovereignty_posture",
            Self::ResidualDependencyClass => "residual_dependency_class",
            Self::ResidualDependency => "residual_dependency",
            Self::LocalSafeContinuity => "local_safe_continuity",
            Self::ScopeExplanation => "scope_explanation",
            Self::LastEvaluated => "last_evaluated",
            Self::NextAction => "next_action",
            Self::FilterKeys => "filter_keys",
        }
    }
}

/// A self-contained residual-dependency note: the exact residual-dependency class, the
/// next action, the residual-dependency disclosure, the local-safe continuity guarantee,
/// and — the implementation-requirement invariant — the *preserved* scope context, so a
/// local/offline/self-host/mirror/companion badge names what it still depends on instead
/// of overstating sovereignty, and the scope it was running in is never dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResidualDependencyNote {
    /// The exact residual-dependency class the scope carries.
    pub residual_dependency_class: M5ResidualDependencyClass,
    /// The next action a reviewer should take.
    pub next_action: M5DeploymentScopeNextAction,
    /// The opaque, export-safe residual-dependency disclosure.
    pub residual_dependency: String,
    /// The local-safe continuity guarantee this scope preserves.
    pub local_safe_continuity: M5LocalSafeContinuity,
    /// The scope the capability was running in, preserved as context. Always equals the
    /// resolved scope.
    pub preserved_scope: M5DeploymentScopeBadgeValue,
    /// True when this residual dependency is an offline / mirror readiness claim.
    pub is_offline_readiness_claim: bool,
    /// True when this residual dependency is the browser-companion host runtime.
    pub is_browser_companion: bool,
    /// A deterministic, self-contained headline naming the residual dependency, the
    /// local-safe continuity, the preserved scope, and the next action — never an
    /// unqualified "fully sovereign" claim and never implying a lifecycle from the scope.
    pub headline: String,
}

/// The full input to the deployment-scope badge resolver for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeBadgeInput {
    /// The opaque, export-safe subject label.
    pub subject_label: String,
    /// The declared deployment scope.
    pub scope: M5DeploymentScopeBadgeValue,
    /// The opaque, export-safe residual-dependency disclosure. Required (non-empty)
    /// whenever the scope makes a local, offline, self-host, mirror, or browser-companion
    /// authority claim.
    pub residual_dependency_repr: Option<String>,
    /// The opaque, export-safe last-evaluated representation.
    pub last_evaluated_repr: String,
}

/// The resolved deployment-scope truth for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDeploymentScopeBadge {
    /// The opaque subject label.
    pub subject_label: String,
    /// The deployment scope — carried as its own field, never merged with support class,
    /// lifecycle, or channel.
    pub scope: M5DeploymentScopeBadgeValue,
    /// The derived sovereignty posture, computed from the scope alone.
    pub sovereignty_posture: M5DeploymentSovereigntyPosture,
    /// True when the scope is openly provider-governed.
    pub is_provider_governed: bool,
    /// True when the scope makes a local-authority claim and must disclose a residual
    /// dependency.
    pub is_locally_sovereign: bool,
    /// True when the scope is an offline or mirror mode.
    pub is_offline_or_mirror: bool,
    /// True when the scope is the browser companion.
    pub is_browser_companion: bool,
    /// The opaque last-evaluated representation.
    pub last_evaluated_repr: String,
    /// The residual-dependency note, present whenever the scope makes a local-authority
    /// claim.
    pub residual_dependency_note: Option<M5ResidualDependencyNote>,
}

/// Errors returned by [`resolve_deployment_scope_badge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DeploymentScopeBadgeError {
    /// The subject label was empty.
    EmptySubjectLabel,
    /// The last-evaluated representation was empty.
    EmptyLastEvaluated,
    /// The scope makes a local, offline, self-host, mirror, or browser-companion authority
    /// claim but no residual-dependency disclosure was supplied — the badge must never
    /// overstate sovereignty by hiding its residual dependency.
    MissingResidualDependencyDisclosure,
    /// A subject label, residual-dependency disclosure, or timestamp carried forbidden
    /// material.
    ForbiddenBadgeMaterial,
}

impl M5DeploymentScopeBadgeError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySubjectLabel => "empty_subject_label",
            Self::EmptyLastEvaluated => "empty_last_evaluated",
            Self::MissingResidualDependencyDisclosure => "missing_residual_dependency_disclosure",
            Self::ForbiddenBadgeMaterial => "forbidden_badge_material",
        }
    }
}

impl fmt::Display for M5DeploymentScopeBadgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "deployment-scope badge resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DeploymentScopeBadgeError {}

/// Resolves one deployment-scope badge from its declared operating mode.
///
/// The deployment scope stays a distinct, composable cue. The derived sovereignty posture
/// is computed from the scope axis alone — a Local-only capability is locally sovereign
/// regardless of its support class, lifecycle, or channel, because the scope is never
/// derived from another axis and never implies one. When the scope makes a local, offline,
/// self-host, mirror, or browser-companion authority claim, the resolver requires a
/// residual-dependency disclosure and produces a self-contained residual-dependency note
/// that *preserves* the scope context and states the local-safe continuity honestly rather
/// than overstating sovereignty — a Browser-companion or Offline-capable badge is always
/// an explicit product truth, never a hidden footnote.
pub fn resolve_deployment_scope_badge(
    input: &M5DeploymentScopeBadgeInput,
) -> Result<M5ResolvedDeploymentScopeBadge, M5DeploymentScopeBadgeError> {
    if input.subject_label.trim().is_empty() {
        return Err(M5DeploymentScopeBadgeError::EmptySubjectLabel);
    }
    if input.last_evaluated_repr.trim().is_empty() {
        return Err(M5DeploymentScopeBadgeError::EmptyLastEvaluated);
    }
    let residual_dependency = input
        .residual_dependency_repr
        .as_deref()
        .map(str::trim)
        .unwrap_or("");
    if value_repr_is_forbidden(&input.subject_label)
        || value_repr_is_forbidden(&input.last_evaluated_repr)
        || value_repr_is_forbidden(residual_dependency)
    {
        return Err(M5DeploymentScopeBadgeError::ForbiddenBadgeMaterial);
    }

    let sovereignty_posture = derive_sovereignty_posture(input.scope);
    let is_provider_governed = sovereignty_posture.is_provider_governed();
    let is_locally_sovereign = sovereignty_posture.makes_local_authority_claim();
    let is_offline_or_mirror = sovereignty_posture.is_offline_or_mirror();
    let is_browser_companion = sovereignty_posture.is_browser_companion();

    let residual_dependency_note = match sovereignty_posture.residual_dependency_class() {
        Some(class) => {
            if residual_dependency.is_empty() {
                return Err(M5DeploymentScopeBadgeError::MissingResidualDependencyDisclosure);
            }
            let next_action = class.next_action();
            let local_safe_continuity = class.local_safe_continuity();
            let headline = format!(
                "Deployment scope '{}': {} — local-safe continuity: {}; residual dependency '{}'; scope '{}' preserved; next: {}",
                input.scope.label(),
                class.phrase(),
                local_safe_continuity.as_str(),
                residual_dependency,
                input.scope.as_str(),
                next_action.as_str()
            );
            Some(M5ResidualDependencyNote {
                residual_dependency_class: class,
                next_action,
                residual_dependency: residual_dependency.to_owned(),
                local_safe_continuity,
                preserved_scope: input.scope,
                is_offline_readiness_claim: class.is_offline_readiness_claim(),
                is_browser_companion: class.is_browser_companion(),
                headline,
            })
        }
        None => None,
    };

    Ok(M5ResolvedDeploymentScopeBadge {
        subject_label: input.subject_label.clone(),
        scope: input.scope,
        sovereignty_posture,
        is_provider_governed,
        is_locally_sovereign,
        is_offline_or_mirror,
        is_browser_companion,
        last_evaluated_repr: input.last_evaluated_repr.clone(),
        residual_dependency_note,
    })
}

/// Derives the sovereignty posture from the deployment scope alone, so the scope is never
/// derived from another badge axis and never implies one.
fn derive_sovereignty_posture(
    scope: M5DeploymentScopeBadgeValue,
) -> M5DeploymentSovereigntyPosture {
    match scope {
        M5DeploymentScopeBadgeValue::LocalOnly => M5DeploymentSovereigntyPosture::LocallySovereign,
        M5DeploymentScopeBadgeValue::Managed => M5DeploymentSovereigntyPosture::ProviderGoverned,
        M5DeploymentScopeBadgeValue::SelfHosted => M5DeploymentSovereigntyPosture::OperatorGoverned,
        M5DeploymentScopeBadgeValue::Mirrored => M5DeploymentSovereigntyPosture::MirrorSynced,
        M5DeploymentScopeBadgeValue::OfflineCapable => {
            M5DeploymentSovereigntyPosture::OfflineResilient
        }
        M5DeploymentScopeBadgeValue::BrowserCompanion => {
            M5DeploymentSovereigntyPosture::HostDelegated
        }
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs deployment-scope truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeResolutionCase {
    /// The resolver input.
    pub input: M5DeploymentScopeBadgeInput,
    /// The resolved truth. Must equal `resolve_deployment_scope_badge(&input)`.
    pub resolved: M5ResolvedDeploymentScopeBadge,
}

impl M5DeploymentScopeResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DeploymentScopeBadgeInput) -> Self {
        let resolved =
            resolve_deployment_scope_badge(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_deployment_scope_badge(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one badge consumer bound to the shared badge anatomy,
/// scope values, sovereignty postures, residual-dependency classes, local-safe
/// continuities, next actions, explanation-drawer fields, export fields, and accessibility
/// routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeRow {
    /// Badge consumer family.
    pub consumer_surface: M5DeploymentScopeConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5BadgeQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 badge surface families that render / consume this badge.
    pub surface_families: Vec<M5BadgeSurfaceFamily>,
    /// Deployment lines this badge keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this consumer renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5DeploymentScopeAnatomyPart>,
    /// Scope values this consumer names.
    pub scope_values: Vec<M5DeploymentScopeBadgeValue>,
    /// Sovereignty postures this consumer distinguishes.
    pub sovereignty_postures: Vec<M5DeploymentSovereigntyPosture>,
    /// Residual-dependency classes this consumer names.
    pub residual_dependency_classes: Vec<M5ResidualDependencyClass>,
    /// Local-safe continuities this consumer distinguishes.
    pub local_safe_continuities: Vec<M5LocalSafeContinuity>,
    /// Next actions this consumer names.
    pub next_actions: Vec<M5DeploymentScopeNextAction>,
    /// Explanation-drawer fields this consumer opens (must include the mandatory
    /// [`M5BadgeExplanationField::MANDATORY`] fields).
    pub explanation_fields: Vec<M5BadgeExplanationField>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5DeploymentScopeExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5BadgeAccessibilityRoute>,
    /// Badge subsystems that consume this badge's projection.
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5BadgeDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5DeploymentScopeResolutionCase>,
    /// Hard invariant: this consumer never collapses the deployment scope into support
    /// class, lifecycle, or channel status. MUST be `false`.
    pub collapses_scope_into_support_lifecycle_or_channel: bool,
    /// Hard invariant: this consumer never implies the lifecycle from the deployment
    /// scope. MUST be `false`.
    pub implies_lifecycle_from_deployment_scope: bool,
    /// Hard invariant: this consumer never drops the residual-dependency disclosure when a
    /// scope makes a local-authority claim. MUST be `false`.
    pub drops_residual_dependency_on_sovereignty_claim: bool,
    /// Hard invariant: this consumer never lets exported evidence lose badge meaning. MUST
    /// be `false`.
    pub drops_badge_meaning_in_export: bool,
}

impl M5DeploymentScopeRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DeploymentScopeAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DeploymentScopeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DeploymentScopeExportField> =
            self.export_fields.iter().copied().collect();
        M5DeploymentScopeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory explanation-drawer field.
    fn declares_mandatory_explanation_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeExplanationField> =
            self.explanation_fields.iter().copied().collect();
        M5BadgeExplanationField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_scope_into_support_lifecycle_or_channel
            && !self.implies_lifecycle_from_deployment_scope
            && !self.drops_residual_dependency_on_sovereignty_claim
            && !self.drops_badge_meaning_in_export
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeVocabularySet {
    /// Badge-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Scope-value tokens.
    pub scope_values: Vec<String>,
    /// Sovereignty-posture tokens.
    pub sovereignty_postures: Vec<String>,
    /// Residual-dependency-class tokens.
    pub residual_dependency_classes: Vec<String>,
    /// Local-safe-continuity tokens.
    pub local_safe_continuities: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Explanation-field tokens (reused from the frozen matrix).
    pub explanation_fields: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Badge-consumer-subsystem tokens (reused from the frozen matrix).
    pub badge_consumer_surfaces: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5DeploymentScopeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DeploymentScopeConsumerSurface::ALL, |v| v.as_str()),
            scope_values: tokens(&M5DeploymentScopeBadgeValue::ALL, |v| v.as_str()),
            sovereignty_postures: tokens(&M5DeploymentSovereigntyPosture::ALL, |v| v.as_str()),
            residual_dependency_classes: tokens(&M5ResidualDependencyClass::ALL, |v| v.as_str()),
            local_safe_continuities: tokens(&M5LocalSafeContinuity::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DeploymentScopeAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DeploymentScopeNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DeploymentScopeExportField::ALL, |v| v.as_str()),
            explanation_fields: tokens(&M5BadgeExplanationField::ALL, |v| v.as_str()),
            surface_families: tokens(&M5BadgeSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BadgeAccessibilityRoute::ALL, |v| v.as_str()),
            badge_consumer_surfaces: tokens(&M5BadgeConsumerSurface::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5BadgeDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeGovernanceReview {
    /// The deployment scope is shown as one distinct, composable cue.
    pub deployment_scope_shown_as_distinct_cue: bool,
    /// The scope is never collapsed into support class, lifecycle, or channel.
    pub scope_never_collapsed_into_support_lifecycle_or_channel: bool,
    /// The deployment scope never implies the lifecycle.
    pub deployment_scope_never_implies_lifecycle: bool,
    /// The deployment scope never implies the support class.
    pub deployment_scope_never_implies_support_class: bool,
    /// A local/offline/self-host/mirror/companion scope automatically discloses its
    /// residual dependency.
    pub sovereignty_claim_auto_discloses_residual_dependency: bool,
    /// The residual-dependency note preserves the underlying scope context.
    pub residual_dependency_note_preserves_scope_context: bool,
    /// Browser companion and offline / mirror modes are explicit product truths, never
    /// hidden footnotes.
    pub browser_companion_and_offline_modes_are_explicit_truths: bool,
    /// The local-safe continuity is stated honestly and never overstates sovereignty.
    pub local_safe_continuity_never_overstated: bool,
    /// Every badge can open its explanation drawer.
    pub every_badge_opens_explanation_drawer: bool,
    /// Every badge is separately filterable.
    pub every_badge_is_separately_filterable: bool,
    /// Exported evidence keeps the scope's meaning.
    pub exported_evidence_keeps_scope_meaning: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeConsumerProjection {
    /// Runtime, install/deployment, and Help/About surfaces consume the shared badge.
    pub runtime_install_help_surfaces_consume_shared_scope_badge: bool,
    /// Diagnostics, export, and companion surfaces consume the shared badge.
    pub diagnostics_export_companion_surfaces_consume_shared_scope_badge: bool,
    /// The scope filter reads a single canonical source.
    pub scope_filter_reads_single_source: bool,
    /// The sovereignty posture reads a single canonical source.
    pub sovereignty_posture_reads_single_source: bool,
    /// Support / export reads a single canonical scope-badge source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the deployment-scope badge primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting badge audit.
    pub badge_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DeploymentScopeBadgePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DeploymentScopeBadgePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5DeploymentScopeRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DeploymentScopeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DeploymentScopeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DeploymentScopeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DeploymentScopeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DeploymentScopeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 deployment-scope badge primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DeploymentScopeBadgePrimitivePacket {
    /// Record kind; must equal [`M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5DeploymentScopeRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DeploymentScopeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DeploymentScopeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DeploymentScopeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DeploymentScopeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DeploymentScopeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DeploymentScopeBadgePrimitivePacket {
    /// Builds an M5 deployment-scope badge primitive packet from stable-lane input.
    pub fn new(input: M5DeploymentScopeBadgePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            badge_rows: input.badge_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 deployment-scope badge primitive invariants.
    pub fn validate(&self) -> Vec<M5DeploymentScopeBadgePrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_RECORD_KIND {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_badge_rows(self, &mut violations);
        validate_scope_axis_independence_coverage(self, &mut violations);
        validate_residual_dependency_preservation_coverage(self, &mut violations);
        validate_offline_mirror_and_browser_companion_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 deployment scope badge primitive packet serializes"),
        ) {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 deployment scope badge primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per badge consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,scope_values,sovereignty_postures,residual_dependency_classes,local_safe_continuities,next_actions,export_fields,example_count\n",
        );
        for row in &self.badge_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.scope_values, |v| v.as_str()),
                join_tokens(&row.sovereignty_postures, |v| v.as_str()),
                join_tokens(&row.residual_dependency_classes, |v| v.as_str()),
                join_tokens(&row.local_safe_continuities, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .badge_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Deployment Scope Badge Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Badge consumers: {} ({} stable)\n",
            self.badge_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Scope values: {}\n",
            self.vocabulary_set.scope_values.join(", ")
        ));
        out.push_str(&format!(
            "- Sovereignty postures: {}\n",
            self.vocabulary_set.sovereignty_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Residual-dependency classes: {}\n",
            self.vocabulary_set.residual_dependency_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Badge consumers\n\n");
        for row in &self.badge_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let note = match &case.resolved.residual_dependency_note {
                    Some(note) => note.residual_dependency_class.as_str(),
                    None => "no_residual_dependency",
                };
                out.push_str(&format!(
                    "    - scope `{}` → posture `{}` (residual `{}`)\n",
                    case.resolved.scope.as_str(),
                    case.resolved.sovereignty_posture.as_str(),
                    note
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 deployment-scope badge primitive export.
#[derive(Debug)]
pub enum M5DeploymentScopeBadgePrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DeploymentScopeBadgePrimitiveViolation>),
}

impl fmt::Display for M5DeploymentScopeBadgePrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 deployment scope badge primitive export parse failed: {error}"
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
                    "m5 deployment scope badge primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DeploymentScopeBadgePrimitiveArtifactError {}

/// Validation failures emitted by [`M5DeploymentScopeBadgePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DeploymentScopeBadgePrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required badge consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A badge row is incomplete.
    BadgeRowIncomplete,
    /// A badge row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A badge row declares no scope values.
    ScopeValueMissing,
    /// A badge row declares no sovereignty postures.
    SovereigntyPostureMissing,
    /// A badge row declares no residual-dependency classes.
    ResidualDependencyClassMissing,
    /// A badge row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A badge row omits one of the mandatory explanation-drawer fields.
    ExplanationDrawerIncomplete,
    /// A badge row declares no accessibility routes (or misses keyboard focus or
    /// non-color encoding).
    AccessibilityRouteMissing,
    /// A badge row declares no badge-consumer subsystems.
    ConsumerSurfacesMissing,
    /// A badge row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A badge row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A badge claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves the deployment scope as an independent axis — a
    /// provider-governed scope and a locally-sovereign scope both present, proving the
    /// scope is not collapsed into a single support/lifecycle/channel rank.
    ScopeAxisIndependenceUnproven,
    /// No worked resolution proves a sovereignty-claiming scope preserving its scope
    /// context and disclosing its residual dependency and local-safe continuity.
    ResidualDependencyPreservationUnproven,
    /// No worked resolution proves the browser-companion mode and an offline/mirror mode
    /// as explicit product truths.
    OfflineMirrorAndBrowserCompanionUnproven,
    /// A badge row violates a hard invariant.
    BadgeInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DeploymentScopeBadgePrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::BadgeRowIncomplete => "badge_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::ScopeValueMissing => "scope_value_missing",
            Self::SovereigntyPostureMissing => "sovereignty_posture_missing",
            Self::ResidualDependencyClassMissing => "residual_dependency_class_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ExplanationDrawerIncomplete => "explanation_drawer_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ScopeAxisIndependenceUnproven => "scope_axis_independence_unproven",
            Self::ResidualDependencyPreservationUnproven => {
                "residual_dependency_preservation_unproven"
            }
            Self::OfflineMirrorAndBrowserCompanionUnproven => {
                "offline_mirror_and_browser_companion_unproven"
            }
            Self::BadgeInvariantViolated => "badge_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 deployment-scope badge primitive export.
pub fn current_stable_m5_deployment_scope_badge_primitive_export(
) -> Result<M5DeploymentScopeBadgePrimitivePacket, M5DeploymentScopeBadgePrimitiveArtifactError> {
    let packet: M5DeploymentScopeBadgePrimitivePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-deployment-scope-badge-proof/support_export.json"
        )))
        .map_err(M5DeploymentScopeBadgePrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DeploymentScopeBadgePrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_DOC_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_FAMILY_MATRIX_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_RESIDUAL_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_MIRROR_OFFLINE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DeploymentScopeBadgePrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_badge_rows(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let present: BTreeSet<M5DeploymentScopeConsumerSurface> = packet
        .badge_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DeploymentScopeConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.badge_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.local_safe_continuities.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::BadgeRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.scope_values.is_empty() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::ScopeValueMissing);
        }
        if row.sovereignty_postures.is_empty() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::SovereigntyPostureMissing);
        }
        if row.residual_dependency_classes.is_empty() {
            violations
                .push(M5DeploymentScopeBadgePrimitiveViolation::ResidualDependencyClassMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::MandatoryExportFieldMissing);
        }
        if !row.declares_mandatory_explanation_fields() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::ExplanationDrawerIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::BadgeInvariantViolated);
        }
    }
}

/// AC1: at least one worked resolution must prove the deployment scope is its own
/// independent axis — a provider-governed scope *and* a locally-sovereign scope both
/// present — proving the scope is never collapsed into a single support/lifecycle/channel
/// rank.
fn validate_scope_axis_independence_coverage(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let has_provider_governed = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_provider_governed)
    });
    let has_locally_sovereign = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_locally_sovereign)
    });
    if !(has_provider_governed && has_locally_sovereign) {
        violations.push(M5DeploymentScopeBadgePrimitiveViolation::ScopeAxisIndependenceUnproven);
    }
}

/// Implementation requirement: at least one worked resolution must prove a
/// sovereignty-claiming scope whose residual-dependency note discloses a non-empty
/// residual dependency, preserves the underlying scope context, and states a local-safe
/// continuity — the badge names what it still depends on rather than overstating
/// sovereignty.
fn validate_residual_dependency_preservation_coverage(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let proven = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_locally_sovereign
                && case
                    .resolved
                    .residual_dependency_note
                    .as_ref()
                    .is_some_and(|note| {
                        !note.residual_dependency.trim().is_empty()
                            && note.preserved_scope == case.resolved.scope
                            && !note.headline.trim().is_empty()
                    })
        })
    });
    if !proven {
        violations
            .push(M5DeploymentScopeBadgePrimitiveViolation::ResidualDependencyPreservationUnproven);
    }
}

/// AC2: Browser companion and offline / mirrored modes must be explicit product truths.
/// At least one worked resolution must prove the browser-companion mode with a residual
/// dependency note, and at least one must prove an offline or mirror mode with one.
fn validate_offline_mirror_and_browser_companion_coverage(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let has_browser_companion = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_browser_companion
                && case
                    .resolved
                    .residual_dependency_note
                    .as_ref()
                    .is_some_and(|note| !note.residual_dependency.trim().is_empty())
        })
    });
    let has_offline_or_mirror = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_offline_or_mirror
                && case
                    .resolved
                    .residual_dependency_note
                    .as_ref()
                    .is_some_and(|note| !note.residual_dependency.trim().is_empty())
        })
    });
    if !(has_browser_companion && has_offline_or_mirror) {
        violations.push(
            M5DeploymentScopeBadgePrimitiveViolation::OfflineMirrorAndBrowserCompanionUnproven,
        );
    }
}

fn validate_governance_review(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.deployment_scope_shown_as_distinct_cue,
        review.scope_never_collapsed_into_support_lifecycle_or_channel,
        review.deployment_scope_never_implies_lifecycle,
        review.deployment_scope_never_implies_support_class,
        review.sovereignty_claim_auto_discloses_residual_dependency,
        review.residual_dependency_note_preserves_scope_context,
        review.browser_companion_and_offline_modes_are_explicit_truths,
        review.local_safe_continuity_never_overstated,
        review.every_badge_opens_explanation_drawer,
        review.every_badge_is_separately_filterable,
        review.exported_evidence_keeps_scope_meaning,
        review.every_row_declares_accessibility_route,
    ] {
        if !ok {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.runtime_install_help_surfaces_consume_shared_scope_badge,
        projection.diagnostics_export_companion_surfaces_consume_shared_scope_badge,
        projection.scope_filter_reads_single_source,
        projection.sovereignty_posture_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DeploymentScopeBadgePrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DeploymentScopeBadgePrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DeploymentScopeBadgePrimitivePacket,
    violations: &mut Vec<M5DeploymentScopeBadgePrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.badge_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DeploymentScopeBadgePrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
