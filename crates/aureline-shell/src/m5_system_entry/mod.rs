//! System-open and file-association intake, resulting-mode truth, and
//! trust/profile/channel review for OS-initiated entry.
//!
//! Aureline's local-first promise has to survive the moment an open *starts
//! outside the product*: the OS hands the app a file, a folder, a workspace
//! manifest, a review or deep link, a patch bundle, or a browser auth return,
//! and the app must decide what that target really is and what mode will
//! result *before* it commits to anything broader than a plain local read.
//! This module makes that decision explicit and reviewable. Every OS-initiated
//! open is projected as one typed [`SystemEntryIntake`] that:
//!
//! - preserves the **literal target** the OS handed over (as an export-safe
//!   captured ref, never a raw path or secret body) alongside the **canonical
//!   target** Aureline detected it to be, so a wrong association or a moved
//!   target can never masquerade as the thing the user expected;
//! - resolves the **intended verb and resulting mode** through the *same*
//!   project-entry vocabulary the in-product Open/Clone/Import/Restore/Resume
//!   flows use — [`aureline_workspace::EntryVerb`],
//!   [`aureline_workspace::ResultingMode`], and
//!   [`aureline_workspace::TargetKind`] — and proves parity by routing the
//!   file/folder/workspace/patch-bundle kinds back through
//!   [`aureline_workspace::resolve_entry_flow`], so an OS open can never coerce
//!   one verb into another;
//! - names the **active profile owner, the channel/build owner, and the trust
//!   checkpoint** the open routes through, so a system open never bypasses
//!   trust, profile, tenant, or policy evaluation; and
//! - declares the **scope class** the open would reach and, for anything wider
//!   than a plain local read, requires an **explicit interstitial** plus a
//!   bounded set of **recovery actions** for the unavailable, wrong-target, and
//!   policy-blocked cases.
//!
//! The resulting [`SystemEntryIntakeReport`] is the canonical truth object for
//! the OS-entry intake lane. It is consumed by:
//!
//! - the live shell entry interstitials and Start Center, which render the
//!   same literal/canonical/resulting-mode disclosure the CLI prints;
//! - the headless inspector (`aureline_shell_m5_system_entry`), the only
//!   mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/platform/m5-system-entry/`;
//! - the support-export wrapper and per-case exports, so a reviewer can
//!   reproduce a wrong-association, moved-target, mixed-root, or policy-blocked
//!   open from typed diagnostics instead of screenshots; and
//! - the markdown artifact under
//!   `artifacts/platform/m5-system-open-and-file-association.md` and the
//!   companion doc under `docs/m5/system-open-and-file-association.md`.
//!
//! The intake layer rides on top of the
//! [native-desktop matrix](crate::m5_native_desktop): that matrix governs the
//! handler ownership and reopen surfaces; this module governs what happens once
//! a surface delivers a target. The report cross-links the native-desktop
//! matrix, the install-topology packet, the project-entry contract, the entry
//! interstitials, the handoff-review surface, and the auth-recovery packet so
//! ownership and routing cannot drift independently.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every required intake kind is present — file, folder, workspace,
//!    review-link, patch-bundle, and provider-return — and each intake carries
//!    a literal target, a canonical target, an active-profile owner, a
//!    channel/build owner, a trust checkpoint, and the canonical command the
//!    in-product path uses.
//! 2. An intake that resolves through the project-entry path reuses the
//!    canonical resolver output exactly; a divergence is a [`VerbCoercion`]
//!    blocker. An intake routed to the review or auth-recovery surface names
//!    that surface.
//! 3. Any scope class wider than a plain local read requires an explicit
//!    interstitial; an auto-open that widens to workspace scope without one is
//!    a [`SilentScopeWiden`] blocker, and one that widens to a mutating
//!    provider flow without one is a [`SilentProviderMutation`] blocker — the
//!    two never collapse into a single finding.
//! 4. A non-exact target carries at least one recovery action, and each
//!    unavailable class stays a distinct failure: a wrong-association or moved
//!    target with no recovery is a [`WrongTargetNoRecovery`] blocker, a
//!    mixed-root or missing path is an [`UnavailablePathSilentLoss`] blocker,
//!    and a policy-blocked open with no recovery is a [`PolicyBlockUnsafe`]
//!    blocker.
//! 5. Stale evidence on a marketed intake is a blocker so release tooling can
//!    narrow the surface instead of shipping it as implicitly stable.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/platform/m5-system-entry/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_system_entry_report`].
//!
//! [`VerbCoercion`]: SystemEntryFailureMode::VerbCoercion
//! [`SilentScopeWiden`]: SystemEntryFailureMode::SilentScopeWiden
//! [`SilentProviderMutation`]: SystemEntryFailureMode::SilentProviderMutation
//! [`WrongTargetNoRecovery`]: SystemEntryFailureMode::WrongTargetNoRecovery
//! [`UnavailablePathSilentLoss`]: SystemEntryFailureMode::UnavailablePathSilentLoss
//! [`PolicyBlockUnsafe`]: SystemEntryFailureMode::PolicyBlockUnsafe

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    resolve_entry_flow, EntryFlowOutcome, EntryFlowRequest, EntryFlowTarget, EntryVerb,
    OpenFlowSheetClass, ResultingMode, TargetKind,
};

#[cfg(test)]
mod tests;

/// Schema version exported with every system-entry intake record.
pub const SYSTEM_ENTRY_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every system-entry surface.
pub const SYSTEM_ENTRY_SHARED_CONTRACT_REF: &str = "shell:m5_system_entry:v1";

/// Stable record kind for [`SystemEntryIntakeReport`] payloads.
pub const SYSTEM_ENTRY_REPORT_RECORD_KIND: &str = "shell_m5_system_entry_report_record";

/// Stable record kind for [`SystemEntryIntakeRow`] payloads.
pub const SYSTEM_ENTRY_ROW_RECORD_KIND: &str = "shell_m5_system_entry_intake_record";

/// Stable record kind for [`SystemEntrySupportExport`] payloads.
pub const SYSTEM_ENTRY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_system_entry_support_export_record";

/// Stable record kind for [`SystemEntryCaseExport`] payloads.
pub const SYSTEM_ENTRY_CASE_EXPORT_RECORD_KIND: &str = "shell_m5_system_entry_case_export_record";

/// Stable report id quoted across surfaces.
pub const SYSTEM_ENTRY_REPORT_ID: &str = "shell:m5_system_entry:report:v1";

/// Stable support-export id quoted in the published wrapper.
pub const SYSTEM_ENTRY_SUPPORT_EXPORT_ID: &str = "support-export:m5-system-entry:001";

/// Source schema ref for the canonical system-entry contract.
pub const SYSTEM_ENTRY_SOURCE_SCHEMA_REF: &str = "schemas/platform/m5-system-entry.schema.json";

/// Path of the published markdown artifact.
pub const SYSTEM_ENTRY_PUBLISHED_REPORT_REF: &str =
    "artifacts/platform/m5-system-open-and-file-association.md";

/// Path of the published companion doc.
pub const SYSTEM_ENTRY_PUBLISHED_DOC_REF: &str = "docs/m5/system-open-and-file-association.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// One OS-initiated entry kind the intake layer governs.
///
/// These are the six target classes the spec requires the intake object to
/// preserve and review through a single typed path, regardless of which OS
/// affordance delivered them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryIntakeKind {
    /// A single file handed over for a plain local open.
    File,
    /// A folder handed over for a local open or add-root.
    Folder,
    /// A workspace manifest handed over for a multi-root open.
    Workspace,
    /// A review or work-item deep link routed to the review surface.
    ReviewLink,
    /// A patch or portable-state bundle routed to the import flow.
    PatchBundle,
    /// A browser auth callback returning to a pending sign-in.
    ProviderReturn,
}

impl SystemEntryIntakeKind {
    /// Returns the stable schema token for this intake kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Folder => "folder",
            Self::Workspace => "workspace",
            Self::ReviewLink => "review_link",
            Self::PatchBundle => "patch_bundle",
            Self::ProviderReturn => "provider_return",
        }
    }

    /// Returns the reviewer-facing label for this intake kind.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Folder => "Folder",
            Self::Workspace => "Workspace",
            Self::ReviewLink => "Review / deep link",
            Self::PatchBundle => "Patch / state bundle",
            Self::ProviderReturn => "Provider return",
        }
    }

    /// Returns the six required intake kinds in canonical order.
    pub const fn required_kinds() -> [Self; 6] {
        [
            Self::File,
            Self::Folder,
            Self::Workspace,
            Self::ReviewLink,
            Self::PatchBundle,
            Self::ProviderReturn,
        ]
    }
}

/// The OS affordance that delivered an intake.
///
/// The intake kind says *what* was handed over; the source surface says *which*
/// native affordance handed it over. The two are tracked separately so a
/// wrong-association incident on a file-association open is a different
/// diagnostic from the same kind arriving through a recent-item reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntrySourceSurface {
    /// System open / save / reveal dialog.
    SystemOpen,
    /// A registered file-type association.
    FileAssociation,
    /// A protocol / deep-link scheme handler.
    ProtocolHandler,
    /// A browser auth callback returning to the app.
    AuthCallback,
    /// A recent-item reopen.
    RecentItem,
    /// A dock, taskbar, or jump-list reopen.
    DockTaskbarJumplist,
}

impl SystemEntrySourceSurface {
    /// Returns the stable schema token for this source surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemOpen => "system_open",
            Self::FileAssociation => "file_association",
            Self::ProtocolHandler => "protocol_handler",
            Self::AuthCallback => "auth_callback",
            Self::RecentItem => "recent_item",
            Self::DockTaskbarJumplist => "dock_taskbar_jumplist",
        }
    }

    /// Returns the reviewer-facing label for this source surface.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::SystemOpen => "System open",
            Self::FileAssociation => "File association",
            Self::ProtocolHandler => "Protocol handler",
            Self::AuthCallback => "Auth callback",
            Self::RecentItem => "Recent item",
            Self::DockTaskbarJumplist => "Dock / taskbar / jump-list",
        }
    }
}

/// A desktop platform the intake is claimed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryPlatform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl SystemEntryPlatform {
    /// Returns the stable schema token for this platform.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Returns the three platforms in canonical order.
    pub const fn all() -> [Self; 3] {
        [Self::Macos, Self::Windows, Self::Linux]
    }
}

/// How the channel/build owns the OS-level registration for an intake.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryOwnershipKind {
    /// Each channel owns its own registration; side-by-side installs do not
    /// collide.
    ChannelScopedOwner,
    /// A shared default is arbitrated by explicit user or admin choice.
    SharedDefaultArbitrated,
    /// A managed fleet deployment owns the registration centrally.
    ManagedFleetOwned,
    /// A portable build does not register an OS-level handler.
    PortableNonRegistering,
}

impl SystemEntryOwnershipKind {
    /// Returns the stable schema token for this ownership kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelScopedOwner => "channel_scoped_owner",
            Self::SharedDefaultArbitrated => "shared_default_arbitrated",
            Self::ManagedFleetOwned => "managed_fleet_owned",
            Self::PortableNonRegistering => "portable_non_registering",
        }
    }
}

/// Shape hint for the literal target string the OS handed over.
///
/// The literal itself is captured as an opaque, export-safe ref; this class is
/// the only structural hint retained so support can reason about the failure
/// without a raw path crossing the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryLiteralFormat {
    /// A Windows drive path (for example `C:\...`).
    WindowsDrivePath,
    /// A Windows UNC path (for example `\\server\share\...`).
    WindowsUncPath,
    /// A POSIX path.
    PosixPath,
    /// A `file://` URI.
    FileUri,
    /// A deep-link / protocol-scheme URI.
    DeepLinkUri,
    /// A browser auth-callback payload.
    ProviderCallback,
    /// The literal shape could not be classified.
    Unknown,
}

impl SystemEntryLiteralFormat {
    /// Returns the stable schema token for this literal format.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowsDrivePath => "windows_drive_path",
            Self::WindowsUncPath => "windows_unc_path",
            Self::PosixPath => "posix_path",
            Self::FileUri => "file_uri",
            Self::DeepLinkUri => "deep_link_uri",
            Self::ProviderCallback => "provider_callback",
            Self::Unknown => "unknown",
        }
    }
}

/// How an intake's intended verb and resulting mode are kept honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryParityClass {
    /// The intake resolves through [`aureline_workspace::resolve_entry_flow`];
    /// its intended verb and mode MUST equal the canonical resolver output.
    EntryFlowResolved,
    /// The intake is routed to the review surface (inspect-only); it MUST name
    /// the routed surface.
    RoutedToReviewSurface,
    /// The intake is routed to the auth-recovery surface; it MUST name the
    /// routed surface.
    RoutedToAuthRecovery,
}

impl SystemEntryParityClass {
    /// Returns the stable schema token for this parity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryFlowResolved => "entry_flow_resolved",
            Self::RoutedToReviewSurface => "routed_to_review_surface",
            Self::RoutedToAuthRecovery => "routed_to_auth_recovery",
        }
    }

    /// `true` when the intake must reuse the canonical entry-flow resolver.
    pub const fn resolves_through_entry_flow(self) -> bool {
        matches!(self, Self::EntryFlowResolved)
    }

    /// `true` when the intake must name a routed reviewed surface.
    pub const fn requires_routed_surface(self) -> bool {
        matches!(
            self,
            Self::RoutedToReviewSurface | Self::RoutedToAuthRecovery
        )
    }
}

/// The authority an auto-open would reach once it commits.
///
/// The track invariant is that nothing wider than a plain local read may widen
/// scope without an explicit interstitial. Every class other than
/// [`PlainLocalRead`](Self::PlainLocalRead) requires one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryScopeClass {
    /// An exact, local, already-trusted read. The fast path; no interstitial.
    PlainLocalRead,
    /// Promotes a single-file or folder open into workspace / multi-root scope.
    WidensToWorkspaceScope,
    /// Crosses a network, review, or tenant boundary to inspect a remote
    /// target (still read-only).
    CrossesBoundary,
    /// Would trigger a mutating provider-side flow.
    WidensToProviderMutation,
    /// Targets an untrusted root and requires an explicit trust decision.
    RequiresTrustDecision,
}

impl SystemEntryScopeClass {
    /// Returns the stable schema token for this scope class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainLocalRead => "plain_local_read",
            Self::WidensToWorkspaceScope => "widens_to_workspace_scope",
            Self::CrossesBoundary => "crosses_boundary",
            Self::WidensToProviderMutation => "widens_to_provider_mutation",
            Self::RequiresTrustDecision => "requires_trust_decision",
        }
    }

    /// Returns the reviewer-facing label for this scope class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::PlainLocalRead => "Plain local read",
            Self::WidensToWorkspaceScope => "Widens to workspace scope",
            Self::CrossesBoundary => "Crosses boundary",
            Self::WidensToProviderMutation => "Widens to provider mutation",
            Self::RequiresTrustDecision => "Requires trust decision",
        }
    }

    /// `true` when committing this scope MUST be gated behind an explicit
    /// interstitial rather than auto-opened.
    pub const fn requires_explicit_interstitial(self) -> bool {
        !matches!(self, Self::PlainLocalRead)
    }
}

/// Availability of the canonical target at intake time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryAvailability {
    /// The exact canonical target is available.
    ExactAvailable,
    /// The file type is registered to another channel or the wrong app.
    WrongAssociation,
    /// The target moved or its alias changed since it was captured.
    MovedTarget,
    /// The target's roots span mismatched or multiple roots.
    MixedRoot,
    /// The target is blocked by policy.
    BlockedByPolicy,
    /// The target's volume or share is missing or unmounted.
    MissingOrUnmounted,
}

impl SystemEntryAvailability {
    /// Returns the stable schema token for this availability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAvailable => "exact_available",
            Self::WrongAssociation => "wrong_association",
            Self::MovedTarget => "moved_target",
            Self::MixedRoot => "mixed_root",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::MissingOrUnmounted => "missing_or_unmounted",
        }
    }

    /// Returns the reviewer-facing label for this availability.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactAvailable => "Exact / available",
            Self::WrongAssociation => "Wrong association",
            Self::MovedTarget => "Moved target",
            Self::MixedRoot => "Mixed root",
            Self::BlockedByPolicy => "Blocked by policy",
            Self::MissingOrUnmounted => "Missing / unmounted",
        }
    }

    /// `true` when the target is not exactly available and therefore requires
    /// at least one recovery action.
    pub const fn requires_recovery(self) -> bool {
        !matches!(self, Self::ExactAvailable)
    }

    /// The distinct failure mode a missing recovery action raises for this
    /// availability. The three recovery failure classes are never collapsed.
    pub const fn missing_recovery_failure_mode(self) -> Option<SystemEntryFailureMode> {
        match self {
            Self::ExactAvailable => None,
            Self::WrongAssociation | Self::MovedTarget => {
                Some(SystemEntryFailureMode::WrongTargetNoRecovery)
            }
            Self::MixedRoot | Self::MissingOrUnmounted => {
                Some(SystemEntryFailureMode::UnavailablePathSilentLoss)
            }
            Self::BlockedByPolicy => Some(SystemEntryFailureMode::PolicyBlockUnsafe),
        }
    }
}

/// A bounded recovery action a degraded intake offers instead of dead-ending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryRecoveryAction {
    /// Reopen the target in the active profile.
    ReopenInActiveProfile,
    /// Let the user choose a different target.
    ChooseDifferentTarget,
    /// Open the file with the channel that actually owns the association.
    OpenWithCorrectHandler,
    /// Reconnect the removable volume or network share.
    ReconnectVolume,
    /// Keep working from the last saved local copy.
    KeepLastSavedCopy,
    /// Sign in again to reach the target.
    SignInToReopen,
    /// Select the intended root when the workspace spans mixed roots.
    SelectIntendedRoot,
    /// Reroute the activation to a compatible verb.
    RerouteToCompatibleVerb,
    /// Return to the review surface that owns the link.
    ReturnToReview,
    /// Show the policy block detail and the contact path.
    ShowPolicyBlockDetail,
}

impl SystemEntryRecoveryAction {
    /// Returns the stable schema token for this recovery action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenInActiveProfile => "reopen_in_active_profile",
            Self::ChooseDifferentTarget => "choose_different_target",
            Self::OpenWithCorrectHandler => "open_with_correct_handler",
            Self::ReconnectVolume => "reconnect_volume",
            Self::KeepLastSavedCopy => "keep_last_saved_copy",
            Self::SignInToReopen => "sign_in_to_reopen",
            Self::SelectIntendedRoot => "select_intended_root",
            Self::RerouteToCompatibleVerb => "reroute_to_compatible_verb",
            Self::ReturnToReview => "return_to_review",
            Self::ShowPolicyBlockDetail => "show_policy_block_detail",
        }
    }
}

/// A distinct system-entry failure class.
///
/// Each class names a materially different way an OS-initiated open can betray
/// the user's intent. They are never collapsed: a silent scope widen, a silent
/// provider mutation, a coerced verb, a wrong-target dead-end, a silent loss on
/// an unavailable path, an unsafe policy block, a bypassed trust evaluation,
/// and a hidden channel owner are separate findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryFailureMode {
    /// An auto-open widened scope beyond a plain local read with no
    /// interstitial.
    SilentScopeWiden,
    /// An auto-open widened to a mutating provider flow with no interstitial.
    SilentProviderMutation,
    /// The intended verb or resulting mode diverged from the canonical
    /// project-entry resolution.
    VerbCoercion,
    /// A wrong-association or moved target offered no recovery.
    WrongTargetNoRecovery,
    /// A mixed-root or missing path silently lost user context.
    UnavailablePathSilentLoss,
    /// A policy-blocked open behaved unsafely instead of degrading truthfully.
    PolicyBlockUnsafe,
    /// The open bypassed trust / profile / tenant / policy evaluation.
    TrustEvaluationBypassed,
    /// The intake carried no inspectable channel/build owner.
    HiddenChannelOwnership,
}

impl SystemEntryFailureMode {
    /// Returns the stable schema token for this failure mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SilentScopeWiden => "silent_scope_widen",
            Self::SilentProviderMutation => "silent_provider_mutation",
            Self::VerbCoercion => "verb_coercion",
            Self::WrongTargetNoRecovery => "wrong_target_no_recovery",
            Self::UnavailablePathSilentLoss => "unavailable_path_silent_loss",
            Self::PolicyBlockUnsafe => "policy_block_unsafe",
            Self::TrustEvaluationBypassed => "trust_evaluation_bypassed",
            Self::HiddenChannelOwnership => "hidden_channel_ownership",
        }
    }
}

/// Freshness of the captured system-entry evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemEntryEvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed intake.
    Stale,
}

impl SystemEntryEvidenceFreshness {
    /// Returns the stable schema token for this freshness.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// Cross-links to the canonical upstream packets the intake layer depends on so
/// ownership and routing cannot drift independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryCrossLinks {
    /// Native-desktop handler-ownership and reopen matrix.
    pub native_desktop_matrix_ref: String,
    /// Install-topology / portability governance packet.
    pub install_topology_ref: String,
    /// Project-entry contract that owns the verb / resulting-mode vocabulary.
    pub project_entry_contract_ref: String,
    /// Entry-interstitial gate the auto-open paths route through.
    pub entry_interstitial_ref: String,
    /// Handoff / review surface review links route to.
    pub handoff_review_ref: String,
    /// Auth-and-recovery packet provider returns route to.
    pub auth_recovery_ref: String,
}

impl SystemEntryCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical
    /// order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 6] {
        [
            ("native_desktop_matrix_ref", &self.native_desktop_matrix_ref),
            ("install_topology_ref", &self.install_topology_ref),
            (
                "project_entry_contract_ref",
                &self.project_entry_contract_ref,
            ),
            ("entry_interstitial_ref", &self.entry_interstitial_ref),
            ("handoff_review_ref", &self.handoff_review_ref),
            ("auth_recovery_ref", &self.auth_recovery_ref),
        ]
    }

    /// The canonical cross-link set every report carries.
    pub fn canonical() -> Self {
        Self {
            native_desktop_matrix_ref: "artifacts/platform/m5-native-desktop-matrix.md".to_owned(),
            install_topology_ref: "artifacts/install/m5/m5-install-and-portability-governance.md"
                .to_owned(),
            project_entry_contract_ref: "docs/ux/project_entry_contract.md".to_owned(),
            entry_interstitial_ref: "shell:entry_interstitials:v1".to_owned(),
            handoff_review_ref: "docs/public/m3/handoff_and_repro_boundary.md".to_owned(),
            auth_recovery_ref: "artifacts/auth/m5_auth_and_recovery.md".to_owned(),
        }
    }
}

/// Canonical descriptor for one OS-initiated entry intake.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryIntake {
    /// Stable intake id (e.g. `intake:file.system_open`).
    pub intake_id: String,
    /// Intake kind the open belongs to.
    pub intake_kind: SystemEntryIntakeKind,
    /// OS affordance that delivered the intake.
    pub source_surface: SystemEntrySourceSurface,
    /// Descriptor revision the report was produced against. MUST be non-empty.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Export-safe captured ref for the literal target the OS handed over. MUST
    /// be non-empty. Never a raw path or secret body.
    pub literal_target_ref: String,
    /// Shape hint for the literal target.
    pub literal_format: SystemEntryLiteralFormat,
    /// Canonical target identity Aureline detected the literal to be. MUST be
    /// non-empty.
    pub canonical_target_ref: String,
    /// Canonical target kind, in the shared project-entry vocabulary.
    pub detected_target_kind: TargetKind,
    /// Verb the open intends, in the shared project-entry vocabulary.
    pub intended_entry_verb: EntryVerb,
    /// Resulting mode the open intends, in the shared project-entry vocabulary.
    pub intended_resulting_mode: ResultingMode,
    /// Candidate resulting modes a reviewer can switch to without leaving the
    /// verb.
    pub candidate_resulting_modes: Vec<ResultingMode>,
    /// How the intended verb/mode is kept honest.
    pub parity_class: SystemEntryParityClass,
    /// Reviewed surface a routed intake hands off to (required for routed
    /// parity classes).
    pub routed_surface_ref: Option<String>,
    /// Active profile owner the open routes through. MUST be non-empty.
    pub active_profile_owner_ref: String,
    /// Channel/build owner of the OS-level registration. MUST be non-empty.
    pub channel_build_owner_ref: String,
    /// How the channel/build owns the registration.
    pub ownership_kind: SystemEntryOwnershipKind,
    /// Trust / profile / tenant / policy checkpoint the open routes through.
    /// MUST be non-empty.
    pub trust_checkpoint_ref: String,
    /// Canonical in-product command the OS path reuses (project-entry parity).
    /// MUST be non-empty.
    pub canonical_command_ref: String,
    /// Authority the auto-open would reach once it commits.
    pub scope_class: SystemEntryScopeClass,
    /// `true` when the commit must be gated behind an explicit interstitial.
    pub requires_explicit_interstitial: bool,
    /// Interstitial ref (required when [`Self::requires_explicit_interstitial`]
    /// is `true`).
    pub interstitial_ref: Option<String>,
    /// Availability of the canonical target at intake time.
    pub availability: SystemEntryAvailability,
    /// Recovery actions offered when the target is not exactly available.
    pub recovery_actions: Vec<SystemEntryRecoveryAction>,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Exact degraded-state vocabulary user-visible surfaces MUST use. MUST be
    /// non-empty.
    pub degraded_state_vocabulary: Vec<String>,
    /// Claimed platforms. MUST be non-empty.
    pub claimed_platforms: Vec<SystemEntryPlatform>,
    /// Freshness of the captured evidence.
    pub evidence_freshness: SystemEntryEvidenceFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule user-visible surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` when the intake is marketed and must pass the report or narrow.
    pub marketed: bool,
    /// `true` once the intake rides the governed system-entry harness. MUST be
    /// `true`.
    pub registered_on_system_entry_harness: bool,
}

/// Outcome of resolving an intake's intended verb/mode against the canonical
/// project-entry resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryParityOutcome {
    /// Parity class the intake declared.
    pub parity_class: SystemEntryParityClass,
    /// `true` when the OS intake provably reuses the in-product entry path.
    pub reuses_project_entry_path: bool,
    /// Canonical open-flow sheet class, when resolved through the entry-flow
    /// resolver.
    pub resolved_sheet_class: Option<OpenFlowSheetClass>,
    /// Resulting mode the canonical resolver produced, when resolved.
    pub resolved_resulting_mode: Option<ResultingMode>,
    /// Routed reviewed-surface ref, for routed parity classes.
    pub routed_surface_ref: Option<String>,
    /// Stable note explaining a divergence, when not matched.
    pub divergence_note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum SystemEntryBlockingFinding {
    /// An auto-open widened scope beyond a plain local read with no
    /// interstitial.
    SilentScopeWiden {
        /// Intake that exposes the gap.
        intake_id: String,
        /// Scope class the intake would have reached.
        scope_class: SystemEntryScopeClass,
    },
    /// An auto-open widened to a mutating provider flow with no interstitial.
    SilentProviderMutation {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intended verb or mode diverged from the canonical resolution.
    VerbCoercion {
        /// Intake that exposes the gap.
        intake_id: String,
        /// Verb the intake intended.
        intended_entry_verb: EntryVerb,
        /// Resulting mode the intake intended.
        intended_resulting_mode: ResultingMode,
        /// Resulting mode the canonical resolver produced, when any.
        resolved_resulting_mode: Option<ResultingMode>,
    },
    /// A wrong-association or moved target offered no recovery.
    WrongTargetNoRecovery {
        /// Intake that exposes the gap.
        intake_id: String,
        /// Availability that required recovery.
        availability: SystemEntryAvailability,
    },
    /// A mixed-root or missing path silently lost user context.
    UnavailablePathSilentLoss {
        /// Intake that exposes the gap.
        intake_id: String,
        /// Availability that required recovery.
        availability: SystemEntryAvailability,
    },
    /// A policy-blocked open behaved unsafely instead of degrading truthfully.
    PolicyBlockUnsafe {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The open bypassed trust / policy evaluation (no trust checkpoint).
    TrustEvaluationBypassed {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no inspectable channel/build owner.
    HiddenChannelOwnership {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no active-profile owner.
    MissingActiveProfileOwner {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no literal target.
    MissingLiteralTarget {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no canonical target.
    MissingCanonicalTarget {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake reused no canonical in-product command.
    MissingCanonicalCommand {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// A scope that requires an interstitial named none.
    MissingInterstitial {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// A routed intake named no reviewed surface.
    MissingRoutedSurface {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no continuity note.
    MissingContinuityNote {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no degraded-state vocabulary.
    MissingDegradedStateVocabulary {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake claimed no platform.
    MissingClaimedPlatforms {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake carried no downgrade rule.
    MissingDowngradeRule {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// A marketed intake carries stale evidence.
    StaleEvidenceOnMarketedIntake {
        /// Intake that exposes the gap.
        intake_id: String,
    },
    /// The intake drives its own entry path off the governed harness.
    IntakeNotOnHarness {
        /// Intake that exposes the gap.
        intake_id: String,
    },
}

impl SystemEntryBlockingFinding {
    /// Returns the stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::SilentScopeWiden { .. } => "silent_scope_widen",
            Self::SilentProviderMutation { .. } => "silent_provider_mutation",
            Self::VerbCoercion { .. } => "verb_coercion",
            Self::WrongTargetNoRecovery { .. } => "wrong_target_no_recovery",
            Self::UnavailablePathSilentLoss { .. } => "unavailable_path_silent_loss",
            Self::PolicyBlockUnsafe { .. } => "policy_block_unsafe",
            Self::TrustEvaluationBypassed { .. } => "trust_evaluation_bypassed",
            Self::HiddenChannelOwnership { .. } => "hidden_channel_ownership",
            Self::MissingActiveProfileOwner { .. } => "missing_active_profile_owner",
            Self::MissingLiteralTarget { .. } => "missing_literal_target",
            Self::MissingCanonicalTarget { .. } => "missing_canonical_target",
            Self::MissingCanonicalCommand { .. } => "missing_canonical_command",
            Self::MissingInterstitial { .. } => "missing_interstitial",
            Self::MissingRoutedSurface { .. } => "missing_routed_surface",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDegradedStateVocabulary { .. } => "missing_degraded_state_vocabulary",
            Self::MissingClaimedPlatforms { .. } => "missing_claimed_platforms",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::StaleEvidenceOnMarketedIntake { .. } => "stale_evidence_on_marketed_intake",
            Self::IntakeNotOnHarness { .. } => "intake_not_on_harness",
        }
    }

    /// Returns the intake id this finding is attached to.
    pub fn intake_id(&self) -> &str {
        match self {
            Self::SilentScopeWiden { intake_id, .. }
            | Self::SilentProviderMutation { intake_id }
            | Self::VerbCoercion { intake_id, .. }
            | Self::WrongTargetNoRecovery { intake_id, .. }
            | Self::UnavailablePathSilentLoss { intake_id, .. }
            | Self::PolicyBlockUnsafe { intake_id }
            | Self::TrustEvaluationBypassed { intake_id }
            | Self::HiddenChannelOwnership { intake_id }
            | Self::MissingActiveProfileOwner { intake_id }
            | Self::MissingLiteralTarget { intake_id }
            | Self::MissingCanonicalTarget { intake_id }
            | Self::MissingCanonicalCommand { intake_id }
            | Self::MissingInterstitial { intake_id }
            | Self::MissingRoutedSurface { intake_id }
            | Self::MissingContinuityNote { intake_id }
            | Self::MissingDegradedStateVocabulary { intake_id }
            | Self::MissingClaimedPlatforms { intake_id }
            | Self::MissingDowngradeRule { intake_id }
            | Self::StaleEvidenceOnMarketedIntake { intake_id }
            | Self::IntakeNotOnHarness { intake_id } => intake_id,
        }
    }

    /// Returns the distinct failure mode this finding maps to, when it maps to
    /// a contract-honesty failure class (rather than a missing-field gap).
    pub fn failure_mode(&self) -> Option<SystemEntryFailureMode> {
        match self {
            Self::SilentScopeWiden { .. } => Some(SystemEntryFailureMode::SilentScopeWiden),
            Self::SilentProviderMutation { .. } => {
                Some(SystemEntryFailureMode::SilentProviderMutation)
            }
            Self::VerbCoercion { .. } => Some(SystemEntryFailureMode::VerbCoercion),
            Self::WrongTargetNoRecovery { .. } => {
                Some(SystemEntryFailureMode::WrongTargetNoRecovery)
            }
            Self::UnavailablePathSilentLoss { .. } => {
                Some(SystemEntryFailureMode::UnavailablePathSilentLoss)
            }
            Self::PolicyBlockUnsafe { .. } => Some(SystemEntryFailureMode::PolicyBlockUnsafe),
            Self::TrustEvaluationBypassed { .. } => {
                Some(SystemEntryFailureMode::TrustEvaluationBypassed)
            }
            Self::HiddenChannelOwnership { .. } => {
                Some(SystemEntryFailureMode::HiddenChannelOwnership)
            }
            _ => None,
        }
    }
}

/// One per-intake system-entry row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryIntakeRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the intake.
    pub descriptor: SystemEntryIntake,
    /// Parity outcome computed against the canonical resolver.
    pub parity_outcome: SystemEntryParityOutcome,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<SystemEntryBlockingFinding>,
    /// `true` when the intake is marketed.
    pub marketed: bool,
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in this class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryFindingSummary {
    /// Total blocking findings across the report.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<SystemEntryFindingCount>,
}

/// Per-intake-kind presence summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryKindCoverage {
    /// Intake kind this summary covers.
    pub intake_kind: SystemEntryIntakeKind,
    /// Number of registered intakes of this kind.
    pub intake_count: usize,
}

/// Per-scope-class coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryScopeCoverage {
    /// Scope class this summary covers.
    pub scope_class: SystemEntryScopeClass,
    /// Number of intakes that reach this scope.
    pub intake_count: usize,
    /// Number of those intakes gated behind an explicit interstitial.
    pub gated_behind_interstitial: usize,
}

/// A single resulting-mode index entry so platform QA, docs, and support
/// surfaces can quote what each intake resolves to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryResultingModeEntry {
    /// Intake id the entry covers.
    pub intake_id: String,
    /// Intake kind the entry covers.
    pub intake_kind: SystemEntryIntakeKind,
    /// Canonical target identity the literal resolved to.
    pub canonical_target_ref: String,
    /// Intended verb in the shared vocabulary.
    pub intended_entry_verb: EntryVerb,
    /// Intended resulting mode in the shared vocabulary.
    pub intended_resulting_mode: ResultingMode,
    /// Scope class the auto-open would reach.
    pub scope_class: SystemEntryScopeClass,
    /// Availability of the canonical target.
    pub availability: SystemEntryAvailability,
}

/// One marketed intake release tooling should narrow because a control failed
/// or its evidence is stale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryNarrowableEntry {
    /// Intake id that must narrow.
    pub intake_id: String,
    /// Failure mode that drives the narrowing, when control-scoped.
    pub failure_mode: Option<SystemEntryFailureMode>,
    /// Stable reason the intake is narrowable.
    pub reason: String,
}

/// System-open and file-association intake report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryIntakeReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required intake kinds, in canonical order.
    pub required_intake_kinds: Vec<SystemEntryIntakeKind>,
    /// Union of claimed platforms across all intakes, sorted.
    pub claimed_platforms: Vec<SystemEntryPlatform>,
    /// Cross-links to upstream packets.
    pub cross_links: SystemEntryCrossLinks,
    /// Per-intake rows, sorted by `descriptor.intake_id`.
    pub entries: Vec<SystemEntryIntakeRow>,
    /// Per-intake-kind presence summary, in canonical kind order.
    pub intake_kind_coverage: Vec<SystemEntryKindCoverage>,
    /// Per-scope-class coverage summary, in canonical scope order.
    pub scope_class_coverage: Vec<SystemEntryScopeCoverage>,
    /// Per-class blocking-finding summary.
    pub findings_summary: SystemEntryFindingSummary,
    /// Canonical resulting-mode index, sorted by intake id.
    pub resulting_mode_index: Vec<SystemEntryResultingModeEntry>,
    /// Number of registered intakes present.
    pub registered_intake_count: usize,
    /// Number of intakes marketed.
    pub marketed_intake_count: usize,
    /// Number of intakes that provably reuse the project-entry path.
    pub project_entry_parity_count: usize,
    /// Marketed intakes release tooling should narrow.
    pub narrowable_marketed_entries: Vec<SystemEntryNarrowableEntry>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this report is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the report can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the report can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl SystemEntryIntakeReport {
    /// Returns `true` when every required intake kind has at least one
    /// registered intake.
    pub fn every_kind_present(&self) -> bool {
        SystemEntryIntakeKind::required_kinds()
            .into_iter()
            .all(|kind| {
                self.entries
                    .iter()
                    .any(|entry| entry.descriptor.intake_kind == kind)
            })
    }

    /// Returns `true` when at least one intake provably reuses the in-product
    /// project-entry resolution path.
    pub fn has_project_entry_parity(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.parity_outcome.reuses_project_entry_path)
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: intakes={}, marketed={}, parity={}, blocking={}, clean={}",
            self.registered_intake_count,
            self.marketed_intake_count,
            self.project_entry_parity_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for entry in &self.entries {
            lines.push(format!(
                "{}: kind={}, verb={}, mode={}, scope={}, avail={}, parity={}",
                entry.descriptor.intake_id,
                entry.descriptor.intake_kind.as_str(),
                entry.descriptor.intended_entry_verb.as_str(),
                entry.descriptor.intended_resulting_mode.as_str(),
                entry.descriptor.scope_class.as_str(),
                entry.descriptor.availability.as_str(),
                entry.parity_outcome.reuses_project_entry_path,
            ));
        }
        for entry in &self.entries {
            for finding in &entry.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.intake_id(),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_entries {
            lines.push(format!(
                "narrowable: {} -- {}",
                narrowable.intake_id, narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 system-open and file-association intake\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::m5_system_entry`](../../crates/aureline-shell/src/m5_system_entry/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- report-md > \\\n  artifacts/platform/m5-system-open-and-file-association.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed platforms: {}\n",
            self.claimed_platforms
                .iter()
                .map(|platform| format!("`{}`", platform.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered intakes: `{}`\n",
            self.registered_intake_count
        ));
        out.push_str(&format!(
            "- Marketed intakes: `{}`\n",
            self.marketed_intake_count
        ));
        out.push_str(&format!(
            "- Project-entry parity intakes: `{}`\n",
            self.project_entry_parity_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed intakes: `{}`\n",
            self.narrowable_marketed_entries.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Cross-links\n\n");
        out.push_str("| Upstream packet | Ref |\n| --------------- | --- |\n");
        for (label, value) in self.cross_links.as_pairs() {
            out.push_str(&format!("| `{label}` | `{value}` |\n"));
        }
        out.push('\n');

        out.push_str("## Per-intake-kind coverage\n\n");
        out.push_str(
            "| Intake kind | Registered intakes |\n| ----------- | -----------------: |\n",
        );
        for coverage in &self.intake_kind_coverage {
            out.push_str(&format!(
                "| {} | {} |\n",
                coverage.intake_kind.display_label(),
                coverage.intake_count,
            ));
        }
        out.push('\n');

        out.push_str("## Per-scope coverage\n\n");
        out.push_str(
            "| Scope class | Intakes | Gated behind interstitial |\n\
             | ----------- | ------: | ------------------------: |\n",
        );
        for coverage in &self.scope_class_coverage {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                coverage.scope_class.display_label(),
                coverage.intake_count,
                coverage.gated_behind_interstitial,
            ));
        }
        out.push('\n');

        out.push_str("## Resulting-mode index\n\n");
        out.push_str(
            "| Intake | Kind | Verb | Resulting mode | Scope | Availability |\n\
             | ------ | ---- | ---- | -------------- | ----- | ------------ |\n",
        );
        for entry in &self.resulting_mode_index {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                entry.intake_id,
                entry.intake_kind.display_label(),
                entry.intended_entry_verb.as_str(),
                entry.intended_resulting_mode.as_str(),
                entry.scope_class.as_str(),
                entry.availability.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        for tally in &self.findings_summary.by_class {
            out.push_str(&format!("| `{}` | {} |\n", tally.class, tally.count));
        }
        if self.findings_summary.by_class.is_empty() {
            out.push_str("| _(none)_ | 0 |\n");
        }
        out.push('\n');

        out.push_str("## Per-intake rows\n\n");
        for entry in &self.entries {
            let d = &entry.descriptor;
            out.push_str(&format!(
                "### `{}` ({} via {})\n\n",
                d.intake_id,
                d.intake_kind.as_str(),
                d.source_surface.as_str(),
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                d.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Literal target: `{}` (`{}`)\n",
                d.literal_target_ref,
                d.literal_format.as_str(),
            ));
            out.push_str(&format!(
                "- Canonical target: `{}`\n",
                d.canonical_target_ref
            ));
            out.push_str(&format!(
                "- Detected target kind: `{}`\n",
                d.detected_target_kind.as_str()
            ));
            out.push_str(&format!(
                "- Intended verb / resulting mode: `{}` / `{}`\n",
                d.intended_entry_verb.as_str(),
                d.intended_resulting_mode.as_str(),
            ));
            out.push_str(&format!(
                "- Parity: `{}` (reuses in-product path: `{}`)\n",
                d.parity_class.as_str(),
                entry.parity_outcome.reuses_project_entry_path,
            ));
            out.push_str(&format!(
                "- Canonical command: `{}`\n",
                d.canonical_command_ref
            ));
            out.push_str(&format!(
                "- Active profile owner: `{}`\n",
                d.active_profile_owner_ref
            ));
            out.push_str(&format!(
                "- Channel/build owner: `{}` (`{}`)\n",
                d.channel_build_owner_ref,
                d.ownership_kind.as_str(),
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                d.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Scope: `{}` (interstitial required: `{}`)\n",
                d.scope_class.as_str(),
                d.requires_explicit_interstitial,
            ));
            out.push_str(&format!("- Availability: `{}`\n", d.availability.as_str()));
            if d.recovery_actions.is_empty() {
                out.push_str("- Recovery actions: _(none required)_\n");
            } else {
                out.push_str(&format!(
                    "- Recovery actions: {}\n",
                    d.recovery_actions
                        .iter()
                        .map(|action| format!("`{}`", action.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            out.push_str(&format!(
                "- Claimed platforms: {}\n",
                d.claimed_platforms
                    .iter()
                    .map(|platform| format!("`{}`", platform.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Evidence freshness: `{}` (captured `{}`)\n",
                d.evidence_freshness.as_str(),
                d.evidence_captured_at,
            ));
            out.push_str(&format!("- Downgrade rule: `{}`\n", d.downgrade_rule_ref));
            out.push_str(&format!(
                "- Marketed: `{}`\n",
                if entry.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!("- Continuity note: {}\n", d.continuity_note));
            out.push_str("- Degraded-state vocabulary:\n");
            for phrase in &d.degraded_state_vocabulary {
                out.push_str(&format!("  - {phrase}\n"));
            }
            out.push('\n');

            if entry.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &entry.blocking_findings {
                    out.push_str(&format!("- `{}`\n", finding.class_token()));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_system_entry_fixtures\n");
        out.push_str("python3 tools/ci/m5/system_entry_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the full system-entry report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntrySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: SystemEntryIntakeReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl SystemEntrySupportExport {
    /// Builds the support-export wrapper for a report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: SystemEntryIntakeReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for entry in &report.entries {
            case_ids.push(entry.descriptor.intake_id.clone());
            case_ids.push(entry.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: SYSTEM_ENTRY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_ENTRY_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_ENTRY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Per-incident support-export packet for a single degraded intake.
///
/// This is the export a reviewer reproduces a wrong-association, moved-target,
/// mixed-root, or policy-blocked open from — the typed diagnostic that replaces
/// a screenshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemEntryCaseExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable case-export id.
    pub case_export_id: String,
    /// Stable case label (e.g. `wrong_association`).
    pub case_label: String,
    /// Availability that defines the incident class.
    pub availability: SystemEntryAvailability,
    /// The intake row in full.
    pub intake: SystemEntryIntakeRow,
    /// Recovery actions the incident offers.
    pub recovery_actions: Vec<SystemEntryRecoveryAction>,
    /// Stable reproduction note for support.
    pub reproduction_note: String,
}

impl SystemEntryCaseExport {
    /// Builds a per-incident case export from a degraded intake row.
    pub fn from_row(
        case_export_id: impl Into<String>,
        case_label: impl Into<String>,
        reproduction_note: impl Into<String>,
        row: SystemEntryIntakeRow,
    ) -> Self {
        let availability = row.descriptor.availability;
        let recovery_actions = row.descriptor.recovery_actions.clone();
        Self {
            record_kind: SYSTEM_ENTRY_CASE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_ENTRY_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_ENTRY_SHARED_CONTRACT_REF.to_owned(),
            case_export_id: case_export_id.into(),
            case_label: case_label.into(),
            availability,
            intake: row,
            recovery_actions,
            reproduction_note: reproduction_note.into(),
        }
    }
}

/// Resolves an intake's intended verb/mode through the canonical project-entry
/// resolver (for entry-flow intakes) or records its routed surface.
fn compute_parity_outcome(descriptor: &SystemEntryIntake) -> SystemEntryParityOutcome {
    match descriptor.parity_class {
        SystemEntryParityClass::EntryFlowResolved => {
            let outcome = resolve_entry_flow(EntryFlowRequest {
                entry_verb: descriptor.intended_entry_verb,
                target: EntryFlowTarget::ExplicitTargetKind(descriptor.detected_target_kind),
                preferred_resulting_mode: Some(descriptor.intended_resulting_mode),
            });
            match outcome {
                EntryFlowOutcome::Resolved(resolved) => {
                    let matched = resolved.entry_verb == descriptor.intended_entry_verb
                        && resolved.resulting_mode == descriptor.intended_resulting_mode;
                    SystemEntryParityOutcome {
                        parity_class: descriptor.parity_class,
                        reuses_project_entry_path: matched,
                        resolved_sheet_class: Some(resolved.sheet_class),
                        resolved_resulting_mode: Some(resolved.resulting_mode),
                        routed_surface_ref: None,
                        divergence_note: if matched {
                            None
                        } else {
                            Some(
                                "resolved resulting mode diverged from the intended mode"
                                    .to_owned(),
                            )
                        },
                    }
                }
                EntryFlowOutcome::Denied(denied) => SystemEntryParityOutcome {
                    parity_class: descriptor.parity_class,
                    reuses_project_entry_path: false,
                    resolved_sheet_class: None,
                    resolved_resulting_mode: None,
                    routed_surface_ref: None,
                    divergence_note: Some(format!(
                        "entry-flow resolution denied: {}",
                        denied.denial_code.as_str()
                    )),
                },
            }
        }
        SystemEntryParityClass::RoutedToReviewSurface
        | SystemEntryParityClass::RoutedToAuthRecovery => SystemEntryParityOutcome {
            parity_class: descriptor.parity_class,
            reuses_project_entry_path: descriptor.routed_surface_ref.is_some(),
            resolved_sheet_class: None,
            resolved_resulting_mode: None,
            routed_surface_ref: descriptor.routed_surface_ref.clone(),
            divergence_note: None,
        },
    }
}

/// Computes the per-intake blocking findings from a descriptor and its parity
/// outcome.
fn compute_intake_findings(
    descriptor: &SystemEntryIntake,
    parity: &SystemEntryParityOutcome,
) -> Vec<SystemEntryBlockingFinding> {
    let mut findings = Vec::new();
    let intake_id = descriptor.intake_id.clone();

    // Identity and ownership integrity.
    if descriptor.literal_target_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingLiteralTarget {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.canonical_target_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingCanonicalTarget {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.active_profile_owner_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingActiveProfileOwner {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.channel_build_owner_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::HiddenChannelOwnership {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::TrustEvaluationBypassed {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.canonical_command_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingCanonicalCommand {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingContinuityNote {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor
        .degraded_state_vocabulary
        .iter()
        .all(|phrase| phrase.trim().is_empty())
    {
        findings.push(SystemEntryBlockingFinding::MissingDegradedStateVocabulary {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.claimed_platforms.is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingClaimedPlatforms {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(SystemEntryBlockingFinding::MissingDowngradeRule {
            intake_id: intake_id.clone(),
        });
    }
    if !descriptor.registered_on_system_entry_harness {
        findings.push(SystemEntryBlockingFinding::IntakeNotOnHarness {
            intake_id: intake_id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == SystemEntryEvidenceFreshness::Stale {
        findings.push(SystemEntryBlockingFinding::StaleEvidenceOnMarketedIntake {
            intake_id: intake_id.clone(),
        });
    }

    // Scope discipline: anything wider than a plain local read must be gated.
    if descriptor.scope_class.requires_explicit_interstitial() {
        if !descriptor.requires_explicit_interstitial {
            if descriptor.scope_class == SystemEntryScopeClass::WidensToProviderMutation {
                findings.push(SystemEntryBlockingFinding::SilentProviderMutation {
                    intake_id: intake_id.clone(),
                });
            } else {
                findings.push(SystemEntryBlockingFinding::SilentScopeWiden {
                    intake_id: intake_id.clone(),
                    scope_class: descriptor.scope_class,
                });
            }
        } else if descriptor
            .interstitial_ref
            .as_deref()
            .map(str::trim)
            .map(str::is_empty)
            != Some(false)
        {
            findings.push(SystemEntryBlockingFinding::MissingInterstitial {
                intake_id: intake_id.clone(),
            });
        }
    }

    // Parity: entry-flow intakes must match the canonical resolver; routed
    // intakes must name their reviewed surface.
    match descriptor.parity_class {
        SystemEntryParityClass::EntryFlowResolved => {
            if !parity.reuses_project_entry_path {
                findings.push(SystemEntryBlockingFinding::VerbCoercion {
                    intake_id: intake_id.clone(),
                    intended_entry_verb: descriptor.intended_entry_verb,
                    intended_resulting_mode: descriptor.intended_resulting_mode,
                    resolved_resulting_mode: parity.resolved_resulting_mode,
                });
            }
        }
        SystemEntryParityClass::RoutedToReviewSurface
        | SystemEntryParityClass::RoutedToAuthRecovery => {
            if descriptor
                .routed_surface_ref
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                != Some(false)
            {
                findings.push(SystemEntryBlockingFinding::MissingRoutedSurface {
                    intake_id: intake_id.clone(),
                });
            }
        }
    }

    // Recovery: a non-exact target must offer a recovery action, and each
    // unavailable class stays a distinct failure.
    if descriptor.availability.requires_recovery() && descriptor.recovery_actions.is_empty() {
        if let Some(mode) = descriptor.availability.missing_recovery_failure_mode() {
            let finding = match mode {
                SystemEntryFailureMode::WrongTargetNoRecovery => {
                    SystemEntryBlockingFinding::WrongTargetNoRecovery {
                        intake_id: intake_id.clone(),
                        availability: descriptor.availability,
                    }
                }
                SystemEntryFailureMode::UnavailablePathSilentLoss => {
                    SystemEntryBlockingFinding::UnavailablePathSilentLoss {
                        intake_id: intake_id.clone(),
                        availability: descriptor.availability,
                    }
                }
                SystemEntryFailureMode::PolicyBlockUnsafe => {
                    SystemEntryBlockingFinding::PolicyBlockUnsafe {
                        intake_id: intake_id.clone(),
                    }
                }
                _ => SystemEntryBlockingFinding::WrongTargetNoRecovery {
                    intake_id: intake_id.clone(),
                    availability: descriptor.availability,
                },
            };
            findings.push(finding);
        }
    }

    findings
}

/// Builds a [`SystemEntryIntakeRow`] from a descriptor, computing the parity
/// outcome and per-intake blocking findings.
pub fn build_system_entry_row(descriptor: SystemEntryIntake) -> SystemEntryIntakeRow {
    let marketed = descriptor.marketed;
    let parity_outcome = compute_parity_outcome(&descriptor);
    let blocking_findings = compute_intake_findings(&descriptor, &parity_outcome);

    SystemEntryIntakeRow {
        record_kind: SYSTEM_ENTRY_ROW_RECORD_KIND.to_owned(),
        schema_version: SYSTEM_ENTRY_SCHEMA_VERSION,
        shared_contract_ref: SYSTEM_ENTRY_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        parity_outcome,
        blocking_findings,
        marketed,
    }
}

/// Computes the per-kind, per-scope, and per-class summaries from finished rows.
fn summarize_report(
    entries: &[SystemEntryIntakeRow],
) -> (
    Vec<SystemEntryKindCoverage>,
    Vec<SystemEntryScopeCoverage>,
    SystemEntryFindingSummary,
) {
    let mut kind_coverage: Vec<SystemEntryKindCoverage> = SystemEntryIntakeKind::required_kinds()
        .into_iter()
        .map(|intake_kind| SystemEntryKindCoverage {
            intake_kind,
            intake_count: 0,
        })
        .collect();

    let scope_order = [
        SystemEntryScopeClass::PlainLocalRead,
        SystemEntryScopeClass::WidensToWorkspaceScope,
        SystemEntryScopeClass::CrossesBoundary,
        SystemEntryScopeClass::WidensToProviderMutation,
        SystemEntryScopeClass::RequiresTrustDecision,
    ];
    let mut scope_coverage: Vec<SystemEntryScopeCoverage> = scope_order
        .into_iter()
        .map(|scope_class| SystemEntryScopeCoverage {
            scope_class,
            intake_count: 0,
            gated_behind_interstitial: 0,
        })
        .collect();

    let mut class_counts: Vec<SystemEntryFindingCount> = Vec::new();
    let mut total = 0usize;

    for entry in entries {
        if let Some(kind_row) = kind_coverage
            .iter_mut()
            .find(|row| row.intake_kind == entry.descriptor.intake_kind)
        {
            kind_row.intake_count += 1;
        }
        if let Some(scope_row) = scope_coverage
            .iter_mut()
            .find(|row| row.scope_class == entry.descriptor.scope_class)
        {
            scope_row.intake_count += 1;
            if entry.descriptor.requires_explicit_interstitial {
                scope_row.gated_behind_interstitial += 1;
            }
        }
        for finding in &entry.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(SystemEntryFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        kind_coverage,
        scope_coverage,
        SystemEntryFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed intakes release tooling should narrow because a
/// control failed or their evidence is stale.
fn compute_narrowable_entries(entries: &[SystemEntryIntakeRow]) -> Vec<SystemEntryNarrowableEntry> {
    let mut narrowable = Vec::new();
    for entry in entries {
        if !entry.marketed {
            continue;
        }
        for finding in &entry.blocking_findings {
            narrowable.push(SystemEntryNarrowableEntry {
                intake_id: entry.descriptor.intake_id.clone(),
                failure_mode: finding.failure_mode(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a full [`SystemEntryIntakeReport`] from per-intake rows.
pub fn build_system_entry_report(entries: Vec<SystemEntryIntakeRow>) -> SystemEntryIntakeReport {
    let mut entries = entries;
    entries.sort_by(|left, right| left.descriptor.intake_id.cmp(&right.descriptor.intake_id));

    let registered_intake_count = entries.len();
    let marketed_intake_count = entries.iter().filter(|entry| entry.marketed).count();
    let project_entry_parity_count = entries
        .iter()
        .filter(|entry| entry.parity_outcome.reuses_project_entry_path)
        .count();

    let (intake_kind_coverage, scope_class_coverage, findings_summary) = summarize_report(&entries);
    let narrowable_marketed_entries = compute_narrowable_entries(&entries);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut platform_set: Vec<SystemEntryPlatform> = Vec::new();
    for entry in &entries {
        for platform in &entry.descriptor.claimed_platforms {
            if !platform_set.contains(platform) {
                platform_set.push(*platform);
            }
        }
    }
    platform_set.sort();

    let mut resulting_mode_index: Vec<SystemEntryResultingModeEntry> = entries
        .iter()
        .map(|entry| SystemEntryResultingModeEntry {
            intake_id: entry.descriptor.intake_id.clone(),
            intake_kind: entry.descriptor.intake_kind,
            canonical_target_ref: entry.descriptor.canonical_target_ref.clone(),
            intended_entry_verb: entry.descriptor.intended_entry_verb,
            intended_resulting_mode: entry.descriptor.intended_resulting_mode,
            scope_class: entry.descriptor.scope_class,
            availability: entry.descriptor.availability,
        })
        .collect();
    resulting_mode_index.sort_by(|left, right| left.intake_id.cmp(&right.intake_id));

    SystemEntryIntakeReport {
        record_kind: SYSTEM_ENTRY_REPORT_RECORD_KIND.to_owned(),
        schema_version: SYSTEM_ENTRY_SCHEMA_VERSION,
        shared_contract_ref: SYSTEM_ENTRY_SHARED_CONTRACT_REF.to_owned(),
        report_id: SYSTEM_ENTRY_REPORT_ID.to_owned(),
        source_schema_ref: SYSTEM_ENTRY_SOURCE_SCHEMA_REF.to_owned(),
        required_intake_kinds: SystemEntryIntakeKind::required_kinds().to_vec(),
        claimed_platforms: platform_set,
        cross_links: SystemEntryCrossLinks::canonical(),
        entries,
        intake_kind_coverage,
        scope_class_coverage,
        findings_summary,
        resulting_mode_index,
        registered_intake_count,
        marketed_intake_count,
        project_entry_parity_count,
        narrowable_marketed_entries,
        report_clean,
        published_report_ref: SYSTEM_ENTRY_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: SYSTEM_ENTRY_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            SYSTEM_ENTRY_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/system_open_and_file_association.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-system-entry".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_system_entry_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum SystemEntryValidationError {
    /// The report has no registered intakes.
    NoRegisteredIntakes,
    /// A required intake kind has no registered intake.
    RequiredIntakeKindMissing { intake_kind: String },
    /// No intake provably reuses the project-entry resolution path.
    NoProjectEntryParity,
    /// A blocking finding remains on an intake.
    BlockingFindingPresent { intake_id: String, class: String },
    /// A cross-link ref is empty.
    CrossLinkMissing { field: String },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// An intake's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { intake_id: String },
}

/// Validates a report against the system-entry acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_system_entry_report(
    report: &SystemEntryIntakeReport,
) -> Result<(), Vec<SystemEntryValidationError>> {
    let mut errors = Vec::new();

    if report.entries.is_empty() {
        errors.push(SystemEntryValidationError::NoRegisteredIntakes);
    }

    for kind in SystemEntryIntakeKind::required_kinds() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.intake_kind == kind);
        if !present {
            errors.push(SystemEntryValidationError::RequiredIntakeKindMissing {
                intake_kind: kind.as_str().to_owned(),
            });
        }
    }

    if !report.has_project_entry_parity() {
        errors.push(SystemEntryValidationError::NoProjectEntryParity);
    }

    for entry in &report.entries {
        if entry.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(SystemEntryValidationError::MissingDescriptorRevisionRef {
                intake_id: entry.descriptor.intake_id.clone(),
            });
        }
        for finding in &entry.blocking_findings {
            errors.push(SystemEntryValidationError::BlockingFindingPresent {
                intake_id: finding.intake_id().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(SystemEntryValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(SystemEntryValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(SystemEntryValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_system_entry_report`].
struct IntakeSeed {
    intake_id: &'static str,
    intake_kind: SystemEntryIntakeKind,
    source_surface: SystemEntrySourceSurface,
    literal_target_ref: &'static str,
    literal_format: SystemEntryLiteralFormat,
    canonical_target_ref: &'static str,
    detected_target_kind: TargetKind,
    intended_entry_verb: EntryVerb,
    intended_resulting_mode: ResultingMode,
    candidate_resulting_modes: &'static [ResultingMode],
    parity_class: SystemEntryParityClass,
    routed_surface_ref: Option<&'static str>,
    ownership_kind: SystemEntryOwnershipKind,
    canonical_command_ref: &'static str,
    scope_class: SystemEntryScopeClass,
    interstitial_ref: Option<&'static str>,
    availability: SystemEntryAvailability,
    recovery_actions: &'static [SystemEntryRecoveryAction],
    continuity_note: &'static str,
    degraded_state_vocabulary: &'static [&'static str],
}

fn build_intake_from_seed(seed: &IntakeSeed) -> SystemEntryIntakeRow {
    let requires_explicit_interstitial = seed.scope_class.requires_explicit_interstitial();
    let descriptor = SystemEntryIntake {
        intake_id: seed.intake_id.to_owned(),
        intake_kind: seed.intake_kind,
        source_surface: seed.source_surface,
        descriptor_revision_ref: format!("{}:rev:2026.06.01-01", seed.intake_id),
        primary_label_ref: format!("label:{}:primary", seed.intake_id),
        literal_target_ref: seed.literal_target_ref.to_owned(),
        literal_format: seed.literal_format,
        canonical_target_ref: seed.canonical_target_ref.to_owned(),
        detected_target_kind: seed.detected_target_kind,
        intended_entry_verb: seed.intended_entry_verb,
        intended_resulting_mode: seed.intended_resulting_mode,
        candidate_resulting_modes: seed.candidate_resulting_modes.to_vec(),
        parity_class: seed.parity_class,
        routed_surface_ref: seed.routed_surface_ref.map(str::to_owned),
        active_profile_owner_ref: format!("profile-owner:{}", seed.intake_id),
        channel_build_owner_ref: format!("channel-owner:{}", seed.intake_id),
        ownership_kind: seed.ownership_kind,
        trust_checkpoint_ref: format!("trust:{}:profile_tenant_policy", seed.intake_id),
        canonical_command_ref: seed.canonical_command_ref.to_owned(),
        scope_class: seed.scope_class,
        requires_explicit_interstitial,
        interstitial_ref: seed.interstitial_ref.map(str::to_owned),
        availability: seed.availability,
        recovery_actions: seed.recovery_actions.to_vec(),
        continuity_note: seed.continuity_note.to_owned(),
        degraded_state_vocabulary: seed
            .degraded_state_vocabulary
            .iter()
            .map(|phrase| (*phrase).to_owned())
            .collect(),
        claimed_platforms: SystemEntryPlatform::all().to_vec(),
        evidence_freshness: SystemEntryEvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:system_entry:narrow_on_stale_evidence".to_owned(),
        marketed: true,
        registered_on_system_entry_harness: true,
    };
    build_system_entry_row(descriptor)
}

const INTAKE_SEEDS: &[IntakeSeed] = &[
    // ---- Clean intake-kind rows: one per required kind. ----
    // File via system open: the fast plain-local-read path.
    IntakeSeed {
        intake_id: "intake:file.system_open",
        intake_kind: SystemEntryIntakeKind::File,
        source_surface: SystemEntrySourceSurface::SystemOpen,
        literal_target_ref: "literal:file.system_open:captured",
        literal_format: SystemEntryLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:file.system_open:single_file",
        detected_target_kind: TargetKind::LocalFile,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::SingleFile,
        candidate_resulting_modes: &[ResultingMode::SingleFile, ResultingMode::Folder],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::ChannelScopedOwner,
        canonical_command_ref: "cmd:workspace.open.target",
        scope_class: SystemEntryScopeClass::PlainLocalRead,
        interstitial_ref: None,
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A system open of a single file resolves to a plain local read in the active profile and never silently widens into a workspace open.",
        degraded_state_vocabulary: &[
            "Open this file",
            "This file is no longer available",
            "Choose a different file",
        ],
    },
    // Folder via system open: still a plain local read.
    IntakeSeed {
        intake_id: "intake:folder.system_open",
        intake_kind: SystemEntryIntakeKind::Folder,
        source_surface: SystemEntrySourceSurface::SystemOpen,
        literal_target_ref: "literal:folder.system_open:captured",
        literal_format: SystemEntryLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:folder.system_open:folder_root",
        detected_target_kind: TargetKind::LocalFolder,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::Folder,
        candidate_resulting_modes: &[
            ResultingMode::Folder,
            ResultingMode::RepoRoot,
            ResultingMode::WorkspaceCandidate,
        ],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::ChannelScopedOwner,
        canonical_command_ref: "cmd:workspace.open.target",
        scope_class: SystemEntryScopeClass::PlainLocalRead,
        interstitial_ref: None,
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A system open of a folder lands on the folder root in the active profile and offers, but never auto-commits, the wider workspace-candidate mode.",
        degraded_state_vocabulary: &[
            "Open this folder",
            "This folder moved or was removed",
            "Choose a different folder",
        ],
    },
    // Workspace via file association: widens to workspace scope, gated.
    IntakeSeed {
        intake_id: "intake:workspace.file_association",
        intake_kind: SystemEntryIntakeKind::Workspace,
        source_surface: SystemEntrySourceSurface::FileAssociation,
        literal_target_ref: "literal:workspace.file_association:captured",
        literal_format: SystemEntryLiteralFormat::FileUri,
        canonical_target_ref: "canonical:workspace.file_association:workspace_manifest",
        detected_target_kind: TargetKind::WorkspaceManifest,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::WorkspaceWithRoots,
        candidate_resulting_modes: &[
            ResultingMode::WorkspaceWithRoots,
            ResultingMode::WorkspaceCandidate,
        ],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::SharedDefaultArbitrated,
        canonical_command_ref: "cmd:workspace.open.target",
        scope_class: SystemEntryScopeClass::WidensToWorkspaceScope,
        interstitial_ref: Some("interstitial:workspace.file_association:confirm_workspace_scope"),
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "Opening a workspace manifest widens to multi-root workspace scope, so it always shows an explicit interstitial before it commits rather than auto-opening every root.",
        degraded_state_vocabulary: &[
            "Open this workspace with all its roots",
            "This workspace manifest is registered to another channel",
            "Open just the manifest file instead",
        ],
    },
    // Review link via protocol handler: inspect-only, crosses a boundary.
    IntakeSeed {
        intake_id: "intake:review_link.protocol_handler",
        intake_kind: SystemEntryIntakeKind::ReviewLink,
        source_surface: SystemEntrySourceSurface::ProtocolHandler,
        literal_target_ref: "literal:review_link.protocol_handler:captured",
        literal_format: SystemEntryLiteralFormat::DeepLinkUri,
        canonical_target_ref: "canonical:review_link.protocol_handler:review_item",
        detected_target_kind: TargetKind::ReviewOrWorkItemDeepLink,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::InspectOnly,
        candidate_resulting_modes: &[ResultingMode::InspectOnly],
        parity_class: SystemEntryParityClass::RoutedToReviewSurface,
        routed_surface_ref: Some("shell:handoff_review:v1"),
        ownership_kind: SystemEntryOwnershipKind::SharedDefaultArbitrated,
        canonical_command_ref: "cmd:review.open_handoff",
        scope_class: SystemEntryScopeClass::CrossesBoundary,
        interstitial_ref: Some("interstitial:review_link.protocol_handler:confirm_remote_review"),
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A review or work-item deep link opens the review surface inspect-only behind an interstitial and is never coerced into a mutating provider action.",
        degraded_state_vocabulary: &[
            "Review this item without making changes",
            "This review link points to an item you cannot access",
            "This review link has expired",
        ],
    },
    // Patch bundle via file association: import flow, gated.
    IntakeSeed {
        intake_id: "intake:patch_bundle.file_association",
        intake_kind: SystemEntryIntakeKind::PatchBundle,
        source_surface: SystemEntrySourceSurface::FileAssociation,
        literal_target_ref: "literal:patch_bundle.file_association:captured",
        literal_format: SystemEntryLiteralFormat::FileUri,
        canonical_target_ref: "canonical:patch_bundle.file_association:portable_state_package",
        detected_target_kind: TargetKind::PortableStatePackage,
        intended_entry_verb: EntryVerb::Import,
        intended_resulting_mode: ResultingMode::ExtractThenReview,
        candidate_resulting_modes: &[
            ResultingMode::ExtractThenReview,
            ResultingMode::CompareBeforeRestore,
            ResultingMode::ApplyToActiveWorkspace,
        ],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::SharedDefaultArbitrated,
        canonical_command_ref: "cmd:workspace.import.bundle",
        scope_class: SystemEntryScopeClass::WidensToWorkspaceScope,
        interstitial_ref: Some("interstitial:patch_bundle.file_association:confirm_import"),
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A patch or state bundle resolves to an extract-then-review import that previews the change before any write, gated behind an explicit interstitial.",
        degraded_state_vocabulary: &[
            "Extract and review this bundle before applying",
            "This bundle is registered to another channel",
            "Open the bundle file without importing",
        ],
    },
    // Provider return via auth callback: routed to auth recovery, gated.
    IntakeSeed {
        intake_id: "intake:provider_return.auth_callback",
        intake_kind: SystemEntryIntakeKind::ProviderReturn,
        source_surface: SystemEntrySourceSurface::AuthCallback,
        literal_target_ref: "literal:provider_return.auth_callback:captured",
        literal_format: SystemEntryLiteralFormat::ProviderCallback,
        canonical_target_ref: "canonical:provider_return.auth_callback:pending_sign_in",
        detected_target_kind: TargetKind::ManagedCloudWorkspace,
        intended_entry_verb: EntryVerb::Resume,
        intended_resulting_mode: ResultingMode::ResumeLiveSession,
        candidate_resulting_modes: &[ResultingMode::ResumeLiveSession],
        parity_class: SystemEntryParityClass::RoutedToAuthRecovery,
        routed_surface_ref: Some("artifacts/auth/m5_auth_and_recovery.md"),
        ownership_kind: SystemEntryOwnershipKind::ChannelScopedOwner,
        canonical_command_ref: "cmd:auth.resume_pending_sign_in",
        scope_class: SystemEntryScopeClass::CrossesBoundary,
        interstitial_ref: Some("interstitial:provider_return.auth_callback:confirm_return"),
        availability: SystemEntryAvailability::ExactAvailable,
        recovery_actions: &[],
        continuity_note: "A browser auth callback returns to the exact pending sign-in in the originating profile behind an interstitial and never silently mutates provider state.",
        degraded_state_vocabulary: &[
            "Return to Aureline to finish signing in",
            "This sign-in link has expired",
            "Sign-in was blocked by policy",
        ],
    },
    // ---- Degraded case rows: the four required incident fixtures. ----
    // Wrong-association: the file type is registered to another channel.
    IntakeSeed {
        intake_id: "intake:case.wrong_association",
        intake_kind: SystemEntryIntakeKind::File,
        source_surface: SystemEntrySourceSurface::FileAssociation,
        literal_target_ref: "literal:case.wrong_association:captured",
        literal_format: SystemEntryLiteralFormat::WindowsDrivePath,
        canonical_target_ref: "canonical:case.wrong_association:single_file",
        detected_target_kind: TargetKind::LocalFile,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::SingleFile,
        candidate_resulting_modes: &[ResultingMode::SingleFile],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::SharedDefaultArbitrated,
        canonical_command_ref: "cmd:workspace.open.target",
        scope_class: SystemEntryScopeClass::PlainLocalRead,
        interstitial_ref: None,
        availability: SystemEntryAvailability::WrongAssociation,
        recovery_actions: &[
            SystemEntryRecoveryAction::OpenWithCorrectHandler,
            SystemEntryRecoveryAction::ChooseDifferentTarget,
        ],
        continuity_note: "A file delivered through an association owned by another channel is not silently opened; the intake offers the correct handler and a target picker instead.",
        degraded_state_vocabulary: &[
            "This file type is registered to another channel",
            "Open with the channel that owns this type",
            "Choose a different file",
        ],
    },
    // Moved-target: a recent-item reopen whose target moved.
    IntakeSeed {
        intake_id: "intake:case.moved_target",
        intake_kind: SystemEntryIntakeKind::Folder,
        source_surface: SystemEntrySourceSurface::RecentItem,
        literal_target_ref: "literal:case.moved_target:captured",
        literal_format: SystemEntryLiteralFormat::PosixPath,
        canonical_target_ref: "canonical:case.moved_target:folder_root",
        detected_target_kind: TargetKind::LocalFolder,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::Folder,
        candidate_resulting_modes: &[ResultingMode::Folder],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::ChannelScopedOwner,
        canonical_command_ref: "cmd:workspace.open.target",
        scope_class: SystemEntryScopeClass::PlainLocalRead,
        interstitial_ref: None,
        availability: SystemEntryAvailability::MovedTarget,
        recovery_actions: &[
            SystemEntryRecoveryAction::ChooseDifferentTarget,
            SystemEntryRecoveryAction::ReopenInActiveProfile,
        ],
        continuity_note: "A recent-item reopen whose folder moved shows a truthful placeholder with a target picker rather than opening an empty or stale shell.",
        degraded_state_vocabulary: &[
            "This item moved or was removed",
            "Reopen in the original workspace",
            "Choose a different folder",
        ],
    },
    // Mixed-root: a workspace whose roots span mismatched roots.
    IntakeSeed {
        intake_id: "intake:case.mixed_root",
        intake_kind: SystemEntryIntakeKind::Workspace,
        source_surface: SystemEntrySourceSurface::SystemOpen,
        literal_target_ref: "literal:case.mixed_root:captured",
        literal_format: SystemEntryLiteralFormat::FileUri,
        canonical_target_ref: "canonical:case.mixed_root:workspace_manifest",
        detected_target_kind: TargetKind::WorkspaceManifest,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::WorkspaceWithRoots,
        candidate_resulting_modes: &[
            ResultingMode::WorkspaceWithRoots,
            ResultingMode::WorkspaceCandidate,
        ],
        parity_class: SystemEntryParityClass::EntryFlowResolved,
        routed_surface_ref: None,
        ownership_kind: SystemEntryOwnershipKind::ChannelScopedOwner,
        canonical_command_ref: "cmd:workspace.open.target",
        scope_class: SystemEntryScopeClass::WidensToWorkspaceScope,
        interstitial_ref: Some("interstitial:case.mixed_root:select_intended_root"),
        availability: SystemEntryAvailability::MixedRoot,
        recovery_actions: &[
            SystemEntryRecoveryAction::SelectIntendedRoot,
            SystemEntryRecoveryAction::ChooseDifferentTarget,
        ],
        continuity_note: "A workspace whose roots span mismatched or unavailable roots does not silently merge them; the intake asks the user to select the intended root behind an interstitial.",
        degraded_state_vocabulary: &[
            "This workspace spans roots that no longer match",
            "Select the root you meant to open",
            "Open just the manifest instead",
        ],
    },
    // Policy-blocked: a review deep link blocked by policy.
    IntakeSeed {
        intake_id: "intake:case.policy_blocked",
        intake_kind: SystemEntryIntakeKind::ReviewLink,
        source_surface: SystemEntrySourceSurface::ProtocolHandler,
        literal_target_ref: "literal:case.policy_blocked:captured",
        literal_format: SystemEntryLiteralFormat::DeepLinkUri,
        canonical_target_ref: "canonical:case.policy_blocked:review_item",
        detected_target_kind: TargetKind::ReviewOrWorkItemDeepLink,
        intended_entry_verb: EntryVerb::Open,
        intended_resulting_mode: ResultingMode::InspectOnly,
        candidate_resulting_modes: &[ResultingMode::InspectOnly],
        parity_class: SystemEntryParityClass::RoutedToReviewSurface,
        routed_surface_ref: Some("shell:handoff_review:v1"),
        ownership_kind: SystemEntryOwnershipKind::ManagedFleetOwned,
        canonical_command_ref: "cmd:review.open_handoff",
        scope_class: SystemEntryScopeClass::CrossesBoundary,
        interstitial_ref: Some("interstitial:case.policy_blocked:policy_block"),
        availability: SystemEntryAvailability::BlockedByPolicy,
        recovery_actions: &[
            SystemEntryRecoveryAction::ShowPolicyBlockDetail,
            SystemEntryRecoveryAction::ReturnToReview,
        ],
        continuity_note: "A review deep link blocked by managed policy degrades truthfully to a policy-block detail with a return path, never a silent dead-end or an unscoped retry.",
        degraded_state_vocabulary: &[
            "This link was blocked by policy",
            "See why this was blocked",
            "Return to the review surface",
        ],
    },
];

/// Seeded report builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5-system-entry/`.
pub fn seeded_system_entry_report() -> SystemEntryIntakeReport {
    let entries = INTAKE_SEEDS.iter().map(build_intake_from_seed).collect();
    build_system_entry_report(entries)
}

/// Stable case-id label for the four required incident fixtures.
pub const SYSTEM_ENTRY_CASE_LABELS: [(&str, &str); 4] = [
    ("intake:case.wrong_association", "wrong_association"),
    ("intake:case.moved_target", "moved_target"),
    ("intake:case.mixed_root", "mixed_root"),
    ("intake:case.policy_blocked", "policy_blocked"),
];

/// Builds the four per-incident case exports from the seeded report, in
/// canonical order.
pub fn seeded_system_entry_case_exports() -> Vec<SystemEntryCaseExport> {
    let report = seeded_system_entry_report();
    SYSTEM_ENTRY_CASE_LABELS
        .iter()
        .filter_map(|(intake_id, label)| {
            let row = report
                .entries
                .iter()
                .find(|entry| entry.descriptor.intake_id == *intake_id)?
                .clone();
            Some(SystemEntryCaseExport::from_row(
                format!("support-export:m5-system-entry:case:{label}"),
                *label,
                format!(
                    "Reproduce the {label} open from this typed intake: the literal target the OS handed over, the canonical target Aureline detected, the resulting mode, and the offered recovery actions.",
                ),
                row,
            ))
        })
        .collect()
}
