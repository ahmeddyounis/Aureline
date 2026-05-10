//! Target / origin badge and host-boundary cue projection shared across the
//! M1 seed entry points (terminal pane, task seed, debug-prep seed, and the
//! provider / auth entry point).
//!
//! ## Why one badge vocabulary, not four
//!
//! Every seed surface that can launch work or open a provider entry point
//! must answer the same question before the user clicks: *which target is
//! about to run, which origin is asking, and is the local desktop boundary
//! crossed?* Forking a private badge per surface lets one lane drift its
//! vocabulary while another lags — for example, the terminal calling SSH
//! sessions "Remote" while the task runner calls them "Workspace VM". This
//! module mints one [`TargetOriginBadge`] shape, projects it from the
//! canonical [`aureline_runtime::ExecutionContext`] (and, for provider
//! entries, from the canonical
//! [`aureline_auth::BrowserCallbackPacket`]), and gives every entry point a
//! stable record to render verbatim.
//!
//! ## What the badge carries
//!
//! - A target class enum mirrored from
//!   [`aureline_runtime::TargetClass`] so the chrome never re-derives
//!   `LocalDesktop` vs `RemoteHost` from raw strings;
//! - An origin class enum mirrored from
//!   [`aureline_auth::AccountBoundaryClass`] (or `NotApplicable` for
//!   pure-execution entry points) so the same `Managed` token appears on the
//!   provider chip and the terminal/task badge;
//! - A typed [`HostBoundaryCue`] that names *why* the cue is visible
//!   (LocalToRemote, LocalToContainer, LocalToManaged, LocalToProvider,
//!   DegradedTrust, PolicyBlocked, Unknown) rather than a single
//!   `boundary_visible` boolean;
//! - Honest provenance: the [`aureline_runtime::ExecutionContext`] id and
//!   (when applicable) the [`aureline_auth::BrowserCallbackPacket`] id stay
//!   on the row so a support export can correlate the badge back to its
//!   source truth.
//!
//! ## Failure-drill posture
//!
//! When the upstream context flips (trust pending, target unreachable,
//! account boundary unknown) the matching badge surfaces an honesty marker
//! rather than a stale "Local — All clear" label. The fixtures under
//! [`fixtures/runtime/target_origin_cases/*.json`] exercise the protected
//! walk on a trusted local seed, the failure drill on a remote target where
//! every entry point must light the same boundary cue, and the
//! pending-trust honesty drill where the chip never collapses to a stale
//! success state.

use serde::{Deserialize, Serialize};

use aureline_auth::{AccountBoundaryClass, BrowserCallbackPacket, IdentityModeAlias};
use aureline_runtime::{
    DegradedFieldReason, ExecutionContext, ReachabilityState, TargetClass, TrustState,
};

/// Stable record-kind tag carried in serialized [`TargetOriginBadge`] payloads.
pub const TARGET_ORIGIN_BADGE_RECORD_KIND: &str = "target_origin_badge_record";
/// Stable record-kind tag carried in serialized [`TargetOriginBadgeSet`] payloads.
pub const TARGET_ORIGIN_BADGE_SET_RECORD_KIND: &str = "target_origin_badge_set_record";
/// Schema version for [`TargetOriginBadge`] and [`TargetOriginBadgeSet`].
pub const TARGET_ORIGIN_BADGE_SCHEMA_VERSION: u32 = 1;

/// Which seed surface a badge frames.
///
/// The chrome uses this tag to pick the surrounding copy (tab strip, task
/// header, debug-prep card, provider entry chip); the badge fields themselves
/// do not change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeEntryPoint {
    /// Bottom-panel terminal pane.
    Terminal,
    /// Task seed surface.
    TaskSeed,
    /// Debug-prep seed surface.
    DebugPrepSeed,
    /// Provider / auth entry point chip (e.g. sign-in card).
    ProviderAuthEntry,
}

impl BadgeEntryPoint {
    /// Stable string token recorded on the badge.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::TaskSeed => "task_seed",
            Self::DebugPrepSeed => "debug_prep_seed",
            Self::ProviderAuthEntry => "provider_auth_entry",
        }
    }
}

/// Target badge class. Mirrors [`aureline_runtime::TargetClass`] one-for-one
/// for execution entry points, plus a [`Self::ProviderEntryPoint`] variant
/// for provider/auth chips that do not run user code on a host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetBadgeClass {
    LocalDesktop,
    RemoteHost,
    LocalContainer,
    Devcontainer,
    RemoteWorkspaceVm,
    PrebuildRuntime,
    ManagedWorkspace,
    NotebookKernelLocal,
    NotebookKernelRemote,
    AiSandbox,
    /// Provider / auth entry point. The "target" of a sign-in card is the
    /// auth tenant, not an execution host.
    ProviderEntryPoint,
}

impl TargetBadgeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDesktop => "local_desktop",
            Self::RemoteHost => "remote_host",
            Self::LocalContainer => "local_container",
            Self::Devcontainer => "devcontainer",
            Self::RemoteWorkspaceVm => "remote_workspace_vm",
            Self::PrebuildRuntime => "prebuild_runtime",
            Self::ManagedWorkspace => "managed_workspace",
            Self::NotebookKernelLocal => "notebook_kernel_local",
            Self::NotebookKernelRemote => "notebook_kernel_remote",
            Self::AiSandbox => "ai_sandbox",
            Self::ProviderEntryPoint => "provider_entry_point",
        }
    }

    /// Human-readable badge label rendered on the chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDesktop => "Local",
            Self::RemoteHost => "Remote",
            Self::LocalContainer => "Container",
            Self::Devcontainer => "Devcontainer",
            Self::RemoteWorkspaceVm => "Remote workspace",
            Self::PrebuildRuntime => "Prebuild",
            Self::ManagedWorkspace => "Managed",
            Self::NotebookKernelLocal => "Notebook (local)",
            Self::NotebookKernelRemote => "Notebook (remote)",
            Self::AiSandbox => "AI sandbox",
            Self::ProviderEntryPoint => "Provider",
        }
    }

    /// Map a runtime [`TargetClass`] onto the badge class. Pure mirror; the
    /// shell never invents a new target taxonomy.
    pub const fn from_target_class(class: TargetClass) -> Self {
        match class {
            TargetClass::LocalHost => Self::LocalDesktop,
            TargetClass::SshRemote => Self::RemoteHost,
            TargetClass::ContainerLocal => Self::LocalContainer,
            TargetClass::Devcontainer => Self::Devcontainer,
            TargetClass::RemoteWorkspaceVm => Self::RemoteWorkspaceVm,
            TargetClass::PrebuildRuntime => Self::PrebuildRuntime,
            TargetClass::ManagedWorkspace => Self::ManagedWorkspace,
            TargetClass::NotebookKernelLocal => Self::NotebookKernelLocal,
            TargetClass::NotebookKernelRemote => Self::NotebookKernelRemote,
            TargetClass::AiSandbox => Self::AiSandbox,
        }
    }
}

/// Origin badge class. Mirrors [`AccountBoundaryClass`] for provider/auth
/// entries; pure-execution entries reuse the same vocabulary by quoting the
/// resolved [`aureline_runtime::IdentityMode`] (or [`Self::NotApplicable`]
/// when no auth packet is wired).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginBadgeClass {
    AccountFreeLocal,
    SelfHostedOrg,
    Managed,
    RestrictedManagedOnly,
    GraceDegradedManaged,
    UnknownBoundary,
    /// No auth packet is wired and the lane is not a provider entry; the
    /// chrome renders the execution-context identity-mode label.
    NotApplicable,
}

impl OriginBadgeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "account_free_local",
            Self::SelfHostedOrg => "self_hosted_org",
            Self::Managed => "managed",
            Self::RestrictedManagedOnly => "restricted_managed_only",
            Self::GraceDegradedManaged => "grace_degraded_managed",
            Self::UnknownBoundary => "unknown_boundary",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Human-readable origin label rendered on the chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "Local only",
            Self::SelfHostedOrg => "Self-hosted org",
            Self::Managed => "Managed",
            Self::RestrictedManagedOnly => "Managed (restricted)",
            Self::GraceDegradedManaged => "Managed (grace)",
            Self::UnknownBoundary => "Unknown origin",
            Self::NotApplicable => "Workspace identity",
        }
    }

    /// Map an [`AccountBoundaryClass`] onto the badge class. Pure mirror.
    pub const fn from_account_boundary(class: AccountBoundaryClass) -> Self {
        match class {
            AccountBoundaryClass::LocalOnly => Self::AccountFreeLocal,
            AccountBoundaryClass::SelfHosted => Self::SelfHostedOrg,
            AccountBoundaryClass::Managed => Self::Managed,
            AccountBoundaryClass::RestrictedManagedOnly => Self::RestrictedManagedOnly,
            AccountBoundaryClass::GraceDegradedManaged => Self::GraceDegradedManaged,
            AccountBoundaryClass::UnknownBoundary => Self::UnknownBoundary,
        }
    }

    /// Map a runtime [`IdentityModeAlias`] onto a badge class. Used when the
    /// surface has no auth packet but still needs an honest origin label
    /// derived from the resolved execution-context identity mode.
    pub const fn from_identity_mode(mode: IdentityModeAlias) -> Self {
        match mode {
            IdentityModeAlias::AccountFreeLocal => Self::AccountFreeLocal,
            IdentityModeAlias::SelfHostedOrg => Self::SelfHostedOrg,
            IdentityModeAlias::ManagedConvenience => Self::Managed,
        }
    }
}

/// Typed reason the host-boundary cue is visible.
///
/// Surfaces consume this rather than a bare boolean so a cue can never
/// degrade silently — every visible cue carries its own *why*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryCue {
    /// Local desktop, local container with no managed plane attached, and
    /// no degraded trust posture: no boundary cue is required.
    Hidden,
    /// Workspace lives inside a container or devcontainer on this device.
    LocalToContainer,
    /// Workspace runs on a remote host (SSH, remote workspace VM, remote
    /// notebook kernel).
    LocalToRemote,
    /// Workspace runs on a managed plane (managed workspace, prebuild,
    /// AI sandbox).
    LocalToManaged,
    /// Provider / auth entry crosses the local-to-tenant boundary.
    LocalToProvider,
    /// Trust posture is unresolved or degraded; the chrome must surface the
    /// cue even on a local target.
    DegradedTrust,
    /// Target reachability or org policy blocks the lane; the cue is held
    /// visible so the user can see the lane is not green.
    PolicyBlocked,
    /// Boundary cannot be classified; fail-closed cue.
    Unknown,
}

impl HostBoundaryCue {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::LocalToContainer => "local_to_container",
            Self::LocalToRemote => "local_to_remote",
            Self::LocalToManaged => "local_to_managed",
            Self::LocalToProvider => "local_to_provider",
            Self::DegradedTrust => "degraded_trust",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unknown => "unknown",
        }
    }

    /// Human-readable label rendered next to the boundary chip.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hidden => "Local desktop — no boundary",
            Self::LocalToContainer => "Local desktop → container",
            Self::LocalToRemote => "Local desktop → remote host",
            Self::LocalToManaged => "Local desktop → managed plane",
            Self::LocalToProvider => "Local desktop → provider tenant",
            Self::DegradedTrust => "Trust pending — review before action",
            Self::PolicyBlocked => "Policy blocked — action withheld",
            Self::Unknown => "Boundary unknown — review before action",
        }
    }

    /// True when the chrome MUST render the cue visible.
    pub const fn is_visible(self) -> bool {
        !matches!(self, Self::Hidden)
    }
}

/// One target/origin badge.
///
/// The chrome quotes every field verbatim. Stable tokens (e.g.
/// `target_class_token`) accompany every human-readable label so support
/// exports and fixtures can compare values without relying on the rendered
/// string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetOriginBadge {
    pub record_kind: String,
    pub schema_version: u32,
    pub entry_point: BadgeEntryPoint,
    pub target_class: TargetBadgeClass,
    pub target_class_token: String,
    pub target_label: String,
    pub canonical_target_id: String,
    pub origin_class: OriginBadgeClass,
    pub origin_class_token: String,
    pub origin_label: String,
    pub boundary_cue: HostBoundaryCue,
    pub boundary_cue_token: String,
    pub boundary_cue_label: String,
    pub boundary_cue_visible: bool,
    pub trust_state: TrustState,
    pub trust_state_token: String,
    pub execution_context_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_packet_ref: Option<String>,
    /// True when at least one input the badge mirrors carries an honesty
    /// signal — degraded execution-context field, unresolved trust, denied
    /// provider session, or unknown boundary class. The chrome MUST surface a
    /// visible honesty marker when this is true.
    pub honesty_marker_present: bool,
}

impl TargetOriginBadge {
    /// Project a badge for the named entry point from a resolved
    /// [`ExecutionContext`].
    ///
    /// Pure-execution entry points (terminal, task seed, debug-prep seed)
    /// derive their origin label from the resolved
    /// [`aureline_runtime::IdentityMode`]; provider/auth entries should call
    /// [`Self::project_provider_entry`] instead so the badge quotes the
    /// account-boundary class on the auth packet.
    pub fn project(entry_point: BadgeEntryPoint, context: &ExecutionContext) -> Self {
        let target_class =
            TargetBadgeClass::from_target_class(context.target_identity.target_class);
        let origin_class =
            OriginBadgeClass::from_identity_mode(context.policy_and_trust.identity_mode);
        let (boundary_cue, honesty_from_cue) =
            derive_boundary_cue(context, origin_class, entry_point, None);
        let honesty_marker_present = context.has_degraded_field()
            || matches!(
                context.policy_and_trust.trust_state,
                TrustState::PendingEvaluation
            )
            || honesty_from_cue;

        Self {
            record_kind: TARGET_ORIGIN_BADGE_RECORD_KIND.to_owned(),
            schema_version: TARGET_ORIGIN_BADGE_SCHEMA_VERSION,
            entry_point,
            target_class,
            target_class_token: target_class.as_str().to_owned(),
            target_label: target_class.label().to_owned(),
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            origin_class,
            origin_class_token: origin_class.as_str().to_owned(),
            origin_label: origin_class.label().to_owned(),
            boundary_cue,
            boundary_cue_token: boundary_cue.as_str().to_owned(),
            boundary_cue_label: boundary_cue.label().to_owned(),
            boundary_cue_visible: boundary_cue.is_visible(),
            trust_state: context.policy_and_trust.trust_state,
            trust_state_token: trust_token(context.policy_and_trust.trust_state).to_owned(),
            execution_context_ref: context.execution_context_id.clone(),
            auth_packet_ref: None,
            honesty_marker_present,
        }
    }

    /// Project a provider/auth entry-point badge from a resolved
    /// [`ExecutionContext`] joined to an [`aureline_auth::BrowserCallbackPacket`].
    ///
    /// The target class is fixed at [`TargetBadgeClass::ProviderEntryPoint`]
    /// because a sign-in chip does not execute user code on a host. The
    /// origin label comes from the packet's account-boundary class so the
    /// `Local only` / `Managed` / `Restricted` vocabulary stays joined to
    /// the auth lane truth.
    pub fn project_provider_entry(
        context: &ExecutionContext,
        packet: &BrowserCallbackPacket,
    ) -> Self {
        let target_class = TargetBadgeClass::ProviderEntryPoint;
        let origin_class = OriginBadgeClass::from_account_boundary(packet.account_boundary_class);
        let (boundary_cue, honesty_from_cue) = derive_boundary_cue(
            context,
            origin_class,
            BadgeEntryPoint::ProviderAuthEntry,
            Some(packet),
        );
        let auth_honesty = packet.pending_session_denied_reason.is_some()
            || matches!(
                packet.account_boundary_class,
                AccountBoundaryClass::UnknownBoundary
            );
        let honesty_marker_present = context.has_degraded_field()
            || matches!(
                context.policy_and_trust.trust_state,
                TrustState::PendingEvaluation
            )
            || auth_honesty
            || honesty_from_cue;

        Self {
            record_kind: TARGET_ORIGIN_BADGE_RECORD_KIND.to_owned(),
            schema_version: TARGET_ORIGIN_BADGE_SCHEMA_VERSION,
            entry_point: BadgeEntryPoint::ProviderAuthEntry,
            target_class,
            target_class_token: target_class.as_str().to_owned(),
            target_label: target_class.label().to_owned(),
            canonical_target_id: format!("provider:{}", packet.packet_id),
            origin_class,
            origin_class_token: origin_class.as_str().to_owned(),
            origin_label: origin_class.label().to_owned(),
            boundary_cue,
            boundary_cue_token: boundary_cue.as_str().to_owned(),
            boundary_cue_label: boundary_cue.label().to_owned(),
            boundary_cue_visible: boundary_cue.is_visible(),
            trust_state: context.policy_and_trust.trust_state,
            trust_state_token: trust_token(context.policy_and_trust.trust_state).to_owned(),
            execution_context_ref: context.execution_context_id.clone(),
            auth_packet_ref: Some(packet.packet_id.clone()),
            honesty_marker_present,
        }
    }
}

/// One badge set covering every entry point that can appear on a protected
/// row in the M1 seed shell.
///
/// The set is the canonical record a support export quotes: it lets a
/// reviewer compare the four entry points side by side and confirm the host
/// boundary stays consistent across them. The `provider_auth_entry` slot is
/// optional because the provider/auth chip is only present once an auth
/// packet has been wired.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetOriginBadgeSet {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub execution_context_ref: String,
    pub terminal_badge: TargetOriginBadge,
    pub task_seed_badge: TargetOriginBadge,
    pub debug_prep_badge: TargetOriginBadge,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_auth_badge: Option<TargetOriginBadge>,
}

impl TargetOriginBadgeSet {
    /// Project the four-entry set from one execution context.
    ///
    /// The provider/auth slot is left empty; call
    /// [`Self::with_provider_packet`] to attach the provider chip when an
    /// auth packet is available.
    pub fn project(context: &ExecutionContext) -> Self {
        Self {
            record_kind: TARGET_ORIGIN_BADGE_SET_RECORD_KIND.to_owned(),
            schema_version: TARGET_ORIGIN_BADGE_SCHEMA_VERSION,
            workspace_id: context.invocation_subject.workspace_id.clone(),
            execution_context_ref: context.execution_context_id.clone(),
            terminal_badge: TargetOriginBadge::project(BadgeEntryPoint::Terminal, context),
            task_seed_badge: TargetOriginBadge::project(BadgeEntryPoint::TaskSeed, context),
            debug_prep_badge: TargetOriginBadge::project(BadgeEntryPoint::DebugPrepSeed, context),
            provider_auth_badge: None,
        }
    }

    /// Attach the provider/auth entry-point badge derived from a callback
    /// packet. The execution context must be the same one the other three
    /// badges were projected from so support exports can correlate the row.
    pub fn with_provider_packet(
        mut self,
        context: &ExecutionContext,
        packet: &BrowserCallbackPacket,
    ) -> Self {
        self.provider_auth_badge = Some(TargetOriginBadge::project_provider_entry(context, packet));
        self
    }

    /// Iterator over every populated badge in the set, in entry-point order
    /// (terminal, task seed, debug-prep seed, provider/auth entry).
    pub fn badges(&self) -> impl Iterator<Item = &TargetOriginBadge> {
        let provider = self.provider_auth_badge.iter();
        [
            &self.terminal_badge,
            &self.task_seed_badge,
            &self.debug_prep_badge,
        ]
        .into_iter()
        .chain(provider)
    }

    /// True when every populated entry-point badge agrees on the
    /// host-boundary cue. Provider/auth entries are excluded from the
    /// comparison because their boundary class is the auth tenant boundary,
    /// not the execution-host boundary.
    ///
    /// The protected walk MUST observe `true` here on a green seed; the
    /// failure drill exercises the case where the badges remain consistent
    /// even when the boundary becomes visible (e.g. a remote target lights
    /// `LocalToRemote` on every execution-entry badge).
    pub fn execution_entries_consistent(&self) -> bool {
        let cue = self.terminal_badge.boundary_cue;
        cue == self.task_seed_badge.boundary_cue && cue == self.debug_prep_badge.boundary_cue
    }

    /// True when at least one populated badge carries an honesty marker.
    pub fn any_honesty_marker(&self) -> bool {
        self.badges().any(|badge| badge.honesty_marker_present)
    }
}

const fn trust_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

/// Pick the boundary cue and an honesty bit from the joined upstream truth.
///
/// Precedence (highest first): unknown auth boundary > policy block > pending
/// trust > target boundary class > local-to-provider for non-local-only
/// origins on provider entries > hidden.
fn derive_boundary_cue(
    context: &ExecutionContext,
    origin_class: OriginBadgeClass,
    entry_point: BadgeEntryPoint,
    packet: Option<&BrowserCallbackPacket>,
) -> (HostBoundaryCue, bool) {
    if matches!(origin_class, OriginBadgeClass::UnknownBoundary) {
        return (HostBoundaryCue::Unknown, true);
    }

    if matches!(
        context.target_identity.reachability_state,
        ReachabilityState::PolicyBlocked
    ) {
        return (HostBoundaryCue::PolicyBlocked, true);
    }

    if context.degraded_fields.iter().any(|record| {
        matches!(
            record.reason,
            DegradedFieldReason::ActivatorBlockedByPolicy
                | DegradedFieldReason::ActivatorBlockedByTrust
        )
    }) {
        return (HostBoundaryCue::PolicyBlocked, true);
    }

    if matches!(
        context.policy_and_trust.trust_state,
        TrustState::PendingEvaluation
    ) {
        return (HostBoundaryCue::DegradedTrust, true);
    }

    let target_cue = boundary_for_target(context.target_identity.target_class);
    if target_cue.is_visible() {
        return (target_cue, false);
    }

    if matches!(entry_point, BadgeEntryPoint::ProviderAuthEntry)
        && !matches!(origin_class, OriginBadgeClass::AccountFreeLocal)
    {
        // The provider entry crosses the tenant boundary even when the
        // execution target is the local desktop.
        let _ = packet; // packet identity already lives on `auth_packet_ref`.
        return (HostBoundaryCue::LocalToProvider, false);
    }

    (HostBoundaryCue::Hidden, false)
}

const fn boundary_for_target(class: TargetClass) -> HostBoundaryCue {
    match class {
        TargetClass::LocalHost => HostBoundaryCue::Hidden,
        TargetClass::ContainerLocal | TargetClass::Devcontainer => {
            HostBoundaryCue::LocalToContainer
        }
        TargetClass::SshRemote
        | TargetClass::RemoteWorkspaceVm
        | TargetClass::NotebookKernelRemote => HostBoundaryCue::LocalToRemote,
        TargetClass::ManagedWorkspace | TargetClass::PrebuildRuntime | TargetClass::AiSandbox => {
            HostBoundaryCue::LocalToManaged
        }
        TargetClass::NotebookKernelLocal => HostBoundaryCue::Hidden,
    }
}

#[cfg(test)]
mod tests;
