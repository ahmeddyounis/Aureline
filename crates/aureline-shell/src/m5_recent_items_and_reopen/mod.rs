//! Recent-item, dock/taskbar, and jump-list reopen fidelity with exact object
//! identity, channel/build ownership, and no-hidden-mutation safeguards.
//!
//! Recent items and system quick actions are launch-bearing desktop truth: the
//! OS shows the user a list of objects in an "Open Recent" menu, a dock recent
//! list, a taskbar jump list, or a pinned jump-list entry, and a single click
//! is expected to land on the *exact* object the user last worked on. This
//! module makes that contract explicit and reviewable. Every object registered
//! in a system-level reopen surface is projected as one typed
//! [`ReopenTargetDescriptor`] that:
//!
//! - preserves the **exact object identity** — the export-safe captured literal
//!   the OS shortcut holds ([`ReopenTargetDescriptor::literal_target_ref`]) and
//!   the canonical identity it was registered against
//!   ([`ReopenTargetDescriptor::canonical_object_ref`]) — so a moved file, a
//!   missing root, or a registration that now points at a different object can
//!   never masquerade as the thing the user expected;
//! - names the **originating channel/build owner** and how it owns the
//!   registration, so a reopen routed through a side-by-side or portable install
//!   never loses ownership provenance;
//! - records the **target freshness** and binds the reopen result to the *same*
//!   restore vocabulary the in-product shell uses
//!   ([`aureline_workspace::RestoreAvailability`],
//!   [`aureline_workspace::TrustState`],
//!   [`aureline_workspace::PortabilityClass`], and
//!   [`aureline_workspace::SafeRecoveryAction`]) so external re-entry never looks
//!   more certain than internal restore; and
//! - declares the **action class** the surface exposes — a plain reopen, a
//!   reveal, or anything privileged/mutating — and requires anything beyond a
//!   summary-safe reopen/reveal to return through a reviewed in-product surface
//!   rather than firing silently from a shortcut.
//!
//! The resulting [`ReopenTargetReport`] is the canonical truth object for the
//! reopen-fidelity lane. It is consumed by:
//!
//! - the live Start Center recent-work rows and the system reopen surfaces,
//!   which render the same identity / freshness / placeholder disclosure the
//!   CLI prints;
//! - the headless inspector (`aureline_shell_m5_reopen_target`), the only
//!   mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/platform/m5-reopen-targets/`;
//! - the support-export wrapper and per-incident case exports, so a reviewer can
//!   reproduce a moved-file, missing-root, changed-channel, stale-provider, or
//!   wrong-target reopen from typed diagnostics instead of screenshots; and
//! - the markdown artifact under
//!   `artifacts/platform/m5-recent-item-and-reopen.md` and the companion doc
//!   under `docs/m5/recent-items-dock-taskbar-jump-list.md`.
//!
//! This lane rides on top of the [native-desktop matrix](crate::m5_native_desktop)
//! (which governs which reopen surfaces a platform exposes) and the
//! [system-entry intake](crate::m5_system_entry) (which governs what happens once
//! a surface delivers a target). This module governs the *fidelity of the object
//! the surface offers to reopen*. The report cross-links the native-desktop
//! matrix, the system-entry intake, the install-topology packet, the restore
//! provenance contract, the Start Center recent-work surface, and the entry
//! interstitials so identity and ownership cannot drift independently.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every required reopen surface is present — recent-item, dock, taskbar, and
//!    jump-list — and each descriptor carries a literal target, a canonical
//!    object, an active-profile owner, an originating channel/build owner, a
//!    trust checkpoint, the canonical in-product command, a continuity note, a
//!    non-empty degraded-state vocabulary, at least one platform, a downgrade
//!    rule, a restore-provenance ref, and `registered_on_reopen_harness = true`.
//! 2. A reopen lands on the exact object or a clearly labeled placeholder: a
//!    non-exact availability MUST carry a placeholder label and at least one
//!    recovery action, and each unavailable class stays a distinct failure. A
//!    detected wrong-target reopen with no recovery is a [`WrongTargetReopen`]
//!    blocker; a moved/missing/changed-channel/stale-provider target with no
//!    recovery is an [`UnavailableTargetSilentLoss`] blocker — the two never
//!    collapse into a single finding.
//! 3. External re-entry never looks more certain than internal restore: a
//!    degraded target (or a stale-freshness target) that still claims
//!    [`RestoreAvailability::Exact`] is a [`StaleCertaintyOverclaim`] blocker.
//! 4. Dock/taskbar/jump-list shortcuts encode no hidden mutation: a
//!    privileged/mutating action that does not return through a reviewed
//!    in-product surface is a [`SilentMutatingAction`] blocker.
//! 5. Channel/build ownership stays visible wherever a side-by-side or portable
//!    install could plausibly own the registration; a missing owner is a
//!    [`HiddenChannelOwnership`] blocker.
//! 6. Stale evidence on a marketed reopen target is a blocker so release tooling
//!    can narrow the surface instead of shipping it as implicitly stable.
//!
//! All identifiers, refs, and label strings are deterministic so the checked-in
//! fixtures under `fixtures/platform/m5-reopen-targets/` are bit-for-bit equal
//! to the seeded report returned by [`seeded_reopen_target_report`].
//!
//! [`WrongTargetReopen`]: ReopenFailureMode::WrongTargetReopen
//! [`UnavailableTargetSilentLoss`]: ReopenFailureMode::UnavailableTargetSilentLoss
//! [`StaleCertaintyOverclaim`]: ReopenFailureMode::StaleCertaintyOverclaim
//! [`SilentMutatingAction`]: ReopenFailureMode::SilentMutatingAction
//! [`HiddenChannelOwnership`]: ReopenFailureMode::HiddenChannelOwnership

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    PortabilityClass, RestoreAvailability, SafeRecoveryAction, TargetKind, TrustState,
};

#[cfg(test)]
mod tests;

/// Schema version exported with every reopen-target record.
pub const REOPEN_TARGET_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every reopen-target surface.
pub const REOPEN_TARGET_SHARED_CONTRACT_REF: &str = "shell:m5_recent_items_and_reopen:v1";

/// Stable record kind for [`ReopenTargetReport`] payloads.
pub const REOPEN_TARGET_REPORT_RECORD_KIND: &str = "shell_m5_reopen_target_report_record";

/// Stable record kind for [`ReopenTargetRow`] payloads.
pub const REOPEN_TARGET_ROW_RECORD_KIND: &str = "shell_m5_reopen_target_row_record";

/// Stable record kind for [`ReopenTargetSupportExport`] payloads.
pub const REOPEN_TARGET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_reopen_target_support_export_record";

/// Stable record kind for [`ReopenTargetCaseExport`] payloads.
pub const REOPEN_TARGET_CASE_EXPORT_RECORD_KIND: &str = "shell_m5_reopen_target_case_export_record";

/// Stable report id quoted across surfaces.
pub const REOPEN_TARGET_REPORT_ID: &str = "shell:m5_recent_items_and_reopen:report:v1";

/// Stable support-export id quoted in the published wrapper.
pub const REOPEN_TARGET_SUPPORT_EXPORT_ID: &str = "support-export:m5-reopen-targets:001";

/// Source schema ref for the canonical reopen-target contract.
pub const REOPEN_TARGET_SOURCE_SCHEMA_REF: &str = "schemas/platform/m5-reopen-target.schema.json";

/// Path of the published markdown artifact.
pub const REOPEN_TARGET_PUBLISHED_REPORT_REF: &str =
    "artifacts/platform/m5-recent-item-and-reopen.md";

/// Path of the published companion doc.
pub const REOPEN_TARGET_PUBLISHED_DOC_REF: &str = "docs/m5/recent-items-dock-taskbar-jump-list.md";

/// Shared restore-provenance contract every reopen row binds its certainty to.
///
/// Reopen rows reuse the shell's restore vocabulary
/// ([`aureline_workspace::RestoreAvailability`]) so external re-entry never
/// looks more certain than internal restore; this ref names the contract that
/// owns that vocabulary.
pub const REOPEN_TARGET_RESTORE_PROVENANCE_REF: &str =
    "shell:restore:provenance_and_placeholders:v1";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// One system-level reopen surface the lane governs.
///
/// These are the surfaces the spec requires the report to cover. The surface
/// says *where* the object is offered for reopen; the platform claim says
/// *which* OS exposes it (a dock surface is macOS, a taskbar jump list is
/// Windows, and a plain recent-item menu spans all three).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenSurfaceKind {
    /// An in-app or OS "Open Recent" recent-item menu.
    RecentItem,
    /// A macOS dock recent-documents menu.
    Dock,
    /// A Windows taskbar recent / jump-list recent section.
    Taskbar,
    /// A Windows jump-list tasks or pinned entry.
    JumpList,
}

impl ReopenSurfaceKind {
    /// Returns the stable schema token for this reopen surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentItem => "recent_item",
            Self::Dock => "dock",
            Self::Taskbar => "taskbar",
            Self::JumpList => "jump_list",
        }
    }

    /// Returns the reviewer-facing label for this reopen surface.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::RecentItem => "Recent item",
            Self::Dock => "Dock",
            Self::Taskbar => "Taskbar",
            Self::JumpList => "Jump list",
        }
    }

    /// Returns the four required reopen surfaces in canonical order.
    pub const fn required_surfaces() -> [Self; 4] {
        [Self::RecentItem, Self::Dock, Self::Taskbar, Self::JumpList]
    }
}

/// A desktop platform a reopen surface is claimed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenPlatform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl ReopenPlatform {
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

/// How the channel/build owns the OS-level reopen registration for a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenOwnershipKind {
    /// Each channel owns its own recent-item registration; side-by-side installs
    /// do not collide.
    ChannelScopedOwner,
    /// A shared default recent list is arbitrated by explicit user or admin
    /// choice.
    SharedDefaultArbitrated,
    /// A managed fleet deployment owns the recent-item registration centrally.
    ManagedFleetOwned,
    /// A portable build keeps its recent list local and registers no OS-level
    /// handler.
    PortableNonRegistering,
}

impl ReopenOwnershipKind {
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

/// Freshness of a captured snapshot — the reopen target or its conformance
/// evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenFreshness {
    /// The snapshot is current.
    Fresh,
    /// The snapshot is stale. A blocker on a marketed target's evidence, and a
    /// signal that a row may not reopen the exact object.
    Stale,
}

impl ReopenFreshness {
    /// Returns the stable schema token for this freshness.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// Resolved availability of a reopen target's object at reopen time.
///
/// Acceptance requires that a reopen lands on the exact object or a clearly
/// labeled placeholder; every class other than [`ExactObject`](Self::ExactObject)
/// is a degraded class that requires a placeholder and a recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenAvailability {
    /// The exact registered object is available; the fast reopen path.
    ExactObject,
    /// The object's file moved or its alias changed since it was registered.
    MovedTarget,
    /// The object's root volume or share is missing or unmounted.
    MissingRoot,
    /// Another channel (a side-by-side or portable install) now owns the
    /// registration.
    ChangedChannel,
    /// A provider-linked object whose backing authority or state has gone stale.
    StaleProviderLinked,
    /// The literal now resolves to a different object than the one registered;
    /// reopening it directly would land on the wrong object.
    WrongTargetDetected,
}

impl ReopenAvailability {
    /// Returns the stable schema token for this availability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactObject => "exact_object",
            Self::MovedTarget => "moved_target",
            Self::MissingRoot => "missing_root",
            Self::ChangedChannel => "changed_channel",
            Self::StaleProviderLinked => "stale_provider_linked",
            Self::WrongTargetDetected => "wrong_target_detected",
        }
    }

    /// Returns the reviewer-facing label for this availability.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactObject => "Exact object",
            Self::MovedTarget => "Moved target",
            Self::MissingRoot => "Missing root",
            Self::ChangedChannel => "Changed channel",
            Self::StaleProviderLinked => "Stale provider-linked",
            Self::WrongTargetDetected => "Wrong target detected",
        }
    }

    /// `true` when the object is not exactly available and therefore requires a
    /// labeled placeholder plus at least one recovery action.
    pub const fn requires_recovery(self) -> bool {
        !matches!(self, Self::ExactObject)
    }

    /// The five non-exact availability classes in canonical order.
    pub const fn degraded_classes() -> [Self; 5] {
        [
            Self::MovedTarget,
            Self::MissingRoot,
            Self::ChangedChannel,
            Self::StaleProviderLinked,
            Self::WrongTargetDetected,
        ]
    }

    /// The distinct failure mode a missing recovery action raises for this
    /// availability. A detected wrong-target reopen is always its own failure
    /// class, never folded into the generic unavailable-path class.
    pub const fn missing_recovery_failure_mode(self) -> Option<ReopenFailureMode> {
        match self {
            Self::ExactObject => None,
            Self::WrongTargetDetected => Some(ReopenFailureMode::WrongTargetReopen),
            Self::MovedTarget
            | Self::MissingRoot
            | Self::ChangedChannel
            | Self::StaleProviderLinked => Some(ReopenFailureMode::UnavailableTargetSilentLoss),
        }
    }
}

/// The action a reopen surface exposes for a target.
///
/// The track guardrail is that dock/taskbar/jump-list shortcuts encode no
/// hidden mutation. A reopen or reveal is summary-safe and may fire directly; a
/// privileged or mutating action MUST return through a reviewed in-product
/// surface instead of firing from the shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenActionClass {
    /// Reopen the exact object in place (summary-safe).
    ReopenObject,
    /// Reveal the object in the OS file manager (summary-safe).
    RevealObject,
    /// An action that would mutate provider/workspace state or widen authority;
    /// MUST route through a reviewed in-product surface.
    PrivilegedOrMutating,
}

impl ReopenActionClass {
    /// Returns the stable schema token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenObject => "reopen_object",
            Self::RevealObject => "reveal_object",
            Self::PrivilegedOrMutating => "privileged_or_mutating",
        }
    }

    /// Returns the reviewer-facing label for this action class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ReopenObject => "Reopen object",
            Self::RevealObject => "Reveal object",
            Self::PrivilegedOrMutating => "Privileged / mutating",
        }
    }

    /// `true` when committing this action MUST route through a reviewed
    /// in-product surface rather than firing directly from the shortcut.
    pub const fn requires_reviewed_return(self) -> bool {
        matches!(self, Self::PrivilegedOrMutating)
    }
}

/// A distinct reopen-fidelity failure class.
///
/// Each class names a materially different way a system-level reopen can betray
/// the user's intent. They are never collapsed: a wrong-target reopen, a silent
/// loss on an unavailable path, a stale-certainty overclaim, a hidden mutating
/// action, a hidden channel owner, an unpreserved identity, and a bypassed
/// trust evaluation are separate findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenFailureMode {
    /// A detected wrong-target reopen offered no placeholder or recovery and
    /// would have landed on the wrong object.
    WrongTargetReopen,
    /// A moved/missing/changed-channel/stale-provider target offered no recovery
    /// and silently lost user context.
    UnavailableTargetSilentLoss,
    /// A degraded or stale target presented itself as more certain than internal
    /// restore allows.
    StaleCertaintyOverclaim,
    /// A privileged/mutating shortcut fired without returning through a reviewed
    /// in-product surface.
    SilentMutatingAction,
    /// The reopen target carried no inspectable channel/build owner.
    HiddenChannelOwnership,
    /// The reopen target preserved no exact object identity.
    IdentityNotPreserved,
    /// The reopen bypassed trust / profile / tenant / policy evaluation.
    TrustEvaluationBypassed,
}

impl ReopenFailureMode {
    /// Returns the stable schema token for this failure mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongTargetReopen => "wrong_target_reopen",
            Self::UnavailableTargetSilentLoss => "unavailable_target_silent_loss",
            Self::StaleCertaintyOverclaim => "stale_certainty_overclaim",
            Self::SilentMutatingAction => "silent_mutating_action",
            Self::HiddenChannelOwnership => "hidden_channel_ownership",
            Self::IdentityNotPreserved => "identity_not_preserved",
            Self::TrustEvaluationBypassed => "trust_evaluation_bypassed",
        }
    }
}

/// Cross-links to the canonical upstream packets the reopen lane depends on so
/// identity, ownership, and routing cannot drift independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTargetCrossLinks {
    /// Native-desktop handler-ownership and reopen-surface matrix.
    pub native_desktop_matrix_ref: String,
    /// System-open / file-association intake packet that resolves a delivered
    /// target.
    pub system_entry_intake_ref: String,
    /// Install-topology / portability governance packet.
    pub install_topology_ref: String,
    /// Restore-provenance and placeholder contract reopen rows bind certainty
    /// to.
    pub restore_provenance_ref: String,
    /// Start Center recent-work surface that renders the same rows in-product.
    pub start_center_ref: String,
    /// Entry-interstitial gate any reviewed-return path routes through.
    pub entry_interstitial_ref: String,
}

impl ReopenTargetCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 6] {
        [
            ("native_desktop_matrix_ref", &self.native_desktop_matrix_ref),
            ("system_entry_intake_ref", &self.system_entry_intake_ref),
            ("install_topology_ref", &self.install_topology_ref),
            ("restore_provenance_ref", &self.restore_provenance_ref),
            ("start_center_ref", &self.start_center_ref),
            ("entry_interstitial_ref", &self.entry_interstitial_ref),
        ]
    }

    /// The canonical cross-link set every report carries.
    pub fn canonical() -> Self {
        Self {
            native_desktop_matrix_ref: "artifacts/platform/m5-native-desktop-matrix.md".to_owned(),
            system_entry_intake_ref: "artifacts/platform/m5-system-open-and-file-association.md"
                .to_owned(),
            install_topology_ref: "artifacts/install/m5/m5-install-and-portability-governance.md"
                .to_owned(),
            restore_provenance_ref: REOPEN_TARGET_RESTORE_PROVENANCE_REF.to_owned(),
            start_center_ref: "shell:m5_start_center_and_switcher:v1".to_owned(),
            entry_interstitial_ref: "shell:entry_interstitials:v1".to_owned(),
        }
    }
}

/// Canonical descriptor for one system-level reopen target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTargetDescriptor {
    /// Stable reopen-target id (e.g. `reopen:recent_item.exact`).
    pub reopen_target_id: String,
    /// Reopen surface the object is offered on.
    pub surface_kind: ReopenSurfaceKind,
    /// Descriptor revision the report was produced against. MUST be non-empty.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Export-safe captured ref for the literal target the OS shortcut holds.
    /// MUST be non-empty. Never a raw path or secret body.
    pub literal_target_ref: String,
    /// Canonical object identity the literal was registered against. MUST be
    /// non-empty.
    pub canonical_object_ref: String,
    /// For a detected wrong-target reopen, the different object the literal now
    /// resolves to, so the wrong-target incident is concrete and exportable.
    pub conflicting_object_ref: Option<String>,
    /// Object kind, in the shared workspace target-kind vocabulary.
    pub target_kind: TargetKind,
    /// Originating channel/build owner of the OS-level registration. MUST be
    /// non-empty.
    pub originating_channel_build_owner_ref: String,
    /// How the channel/build owns the registration.
    pub ownership_kind: ReopenOwnershipKind,
    /// `true` when a side-by-side or portable install could plausibly own the
    /// registration, so the channel/build owner MUST stay visible.
    pub side_by_side_or_portable_plausible: bool,
    /// Active profile owner the reopen routes through. MUST be non-empty.
    pub active_profile_owner_ref: String,
    /// Trust / profile / tenant / policy checkpoint the reopen routes through.
    /// MUST be non-empty.
    pub trust_checkpoint_ref: String,
    /// Freshness of the captured reopen-target snapshot.
    pub target_freshness: ReopenFreshness,
    /// Timestamp the target snapshot was captured.
    pub captured_at: String,
    /// Resolved availability of the object at reopen time.
    pub availability: ReopenAvailability,
    /// Restore availability the reopen advertises, in the shared restore
    /// vocabulary. A degraded availability MUST NOT claim
    /// [`RestoreAvailability::Exact`].
    pub restore_availability: RestoreAvailability,
    /// Workspace trust posture, in the shared restore vocabulary.
    pub trust_state: TrustState,
    /// Portability posture, in the shared restore vocabulary.
    pub portability_class: PortabilityClass,
    /// The action the surface exposes for this target.
    pub action_class: ReopenActionClass,
    /// `true` when the surface action stays a summary-safe reopen/reveal.
    pub stays_summary_only: bool,
    /// Reviewed in-product surface a privileged/mutating action returns through
    /// (required when [`ReopenActionClass::requires_reviewed_return`]).
    pub reviewed_return_surface_ref: Option<String>,
    /// Canonical in-product command the reopen reuses. MUST be non-empty.
    pub canonical_command_ref: String,
    /// Recovery actions offered when the object is not exactly available, in the
    /// shared restore vocabulary.
    pub recovery_actions: Vec<SafeRecoveryAction>,
    /// Placeholder label ref shown instead of a stale/unavailable object
    /// (required for non-exact availability).
    pub placeholder_label_ref: Option<String>,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Exact degraded-state vocabulary user-visible surfaces MUST use. MUST be
    /// non-empty.
    pub degraded_state_vocabulary: Vec<String>,
    /// Restore-provenance contract this row binds its certainty to. MUST be
    /// non-empty.
    pub restore_provenance_ref: String,
    /// Claimed platforms. MUST be non-empty.
    pub claimed_platforms: Vec<ReopenPlatform>,
    /// Freshness of the captured conformance evidence.
    pub evidence_freshness: ReopenFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule user-visible surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` when the target is marketed and must pass the report or narrow.
    pub marketed: bool,
    /// `true` once the target rides the governed reopen harness. MUST be `true`.
    pub registered_on_reopen_harness: bool,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ReopenBlockingFinding {
    /// A detected wrong-target reopen offered no recovery.
    WrongTargetReopen {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// A moved/missing/changed-channel/stale-provider target offered no
    /// recovery.
    UnavailableTargetSilentLoss {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
        /// Availability that required recovery.
        availability: ReopenAvailability,
    },
    /// A degraded or stale target overclaimed restore certainty.
    StaleCertaintyOverclaim {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
        /// Availability that contradicts the claimed restore certainty.
        availability: ReopenAvailability,
        /// Restore availability the target overclaimed.
        restore_availability: RestoreAvailability,
    },
    /// A privileged/mutating shortcut fired without a reviewed return surface.
    SilentMutatingAction {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no inspectable channel/build owner.
    HiddenChannelOwnership {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no literal target identity.
    MissingLiteralTarget {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no canonical object identity.
    MissingCanonicalObject {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen bypassed trust / policy evaluation (no trust checkpoint).
    TrustEvaluationBypassed {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no active-profile owner.
    MissingActiveProfileOwner {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target reused no canonical in-product command.
    MissingCanonicalCommand {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// A non-exact target named no labeled placeholder.
    MissingPlaceholderLabel {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no continuity note.
    MissingContinuityNote {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no degraded-state vocabulary.
    MissingDegradedStateVocabulary {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no restore-provenance ref.
    MissingRestoreProvenance {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target claimed no platform.
    MissingClaimedPlatforms {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target carried no downgrade rule.
    MissingDowngradeRule {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// A marketed reopen target carries stale evidence.
    StaleEvidenceOnMarketedTarget {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
    /// The reopen target drives its own reopen path off the governed harness.
    TargetNotOnHarness {
        /// Reopen target that exposes the gap.
        reopen_target_id: String,
    },
}

impl ReopenBlockingFinding {
    /// Returns the stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::WrongTargetReopen { .. } => "wrong_target_reopen",
            Self::UnavailableTargetSilentLoss { .. } => "unavailable_target_silent_loss",
            Self::StaleCertaintyOverclaim { .. } => "stale_certainty_overclaim",
            Self::SilentMutatingAction { .. } => "silent_mutating_action",
            Self::HiddenChannelOwnership { .. } => "hidden_channel_ownership",
            Self::MissingLiteralTarget { .. } => "missing_literal_target",
            Self::MissingCanonicalObject { .. } => "missing_canonical_object",
            Self::TrustEvaluationBypassed { .. } => "trust_evaluation_bypassed",
            Self::MissingActiveProfileOwner { .. } => "missing_active_profile_owner",
            Self::MissingCanonicalCommand { .. } => "missing_canonical_command",
            Self::MissingPlaceholderLabel { .. } => "missing_placeholder_label",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDegradedStateVocabulary { .. } => "missing_degraded_state_vocabulary",
            Self::MissingRestoreProvenance { .. } => "missing_restore_provenance",
            Self::MissingClaimedPlatforms { .. } => "missing_claimed_platforms",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::StaleEvidenceOnMarketedTarget { .. } => "stale_evidence_on_marketed_target",
            Self::TargetNotOnHarness { .. } => "target_not_on_harness",
        }
    }

    /// Returns the reopen-target id this finding is attached to.
    pub fn reopen_target_id(&self) -> &str {
        match self {
            Self::WrongTargetReopen { reopen_target_id }
            | Self::UnavailableTargetSilentLoss {
                reopen_target_id, ..
            }
            | Self::StaleCertaintyOverclaim {
                reopen_target_id, ..
            }
            | Self::SilentMutatingAction { reopen_target_id }
            | Self::HiddenChannelOwnership { reopen_target_id }
            | Self::MissingLiteralTarget { reopen_target_id }
            | Self::MissingCanonicalObject { reopen_target_id }
            | Self::TrustEvaluationBypassed { reopen_target_id }
            | Self::MissingActiveProfileOwner { reopen_target_id }
            | Self::MissingCanonicalCommand { reopen_target_id }
            | Self::MissingPlaceholderLabel { reopen_target_id }
            | Self::MissingContinuityNote { reopen_target_id }
            | Self::MissingDegradedStateVocabulary { reopen_target_id }
            | Self::MissingRestoreProvenance { reopen_target_id }
            | Self::MissingClaimedPlatforms { reopen_target_id }
            | Self::MissingDowngradeRule { reopen_target_id }
            | Self::StaleEvidenceOnMarketedTarget { reopen_target_id }
            | Self::TargetNotOnHarness { reopen_target_id } => reopen_target_id,
        }
    }

    /// Returns the distinct failure mode this finding maps to, when it maps to a
    /// contract-honesty failure class (rather than a missing-field gap).
    pub fn failure_mode(&self) -> Option<ReopenFailureMode> {
        match self {
            Self::WrongTargetReopen { .. } => Some(ReopenFailureMode::WrongTargetReopen),
            Self::UnavailableTargetSilentLoss { .. } => {
                Some(ReopenFailureMode::UnavailableTargetSilentLoss)
            }
            Self::StaleCertaintyOverclaim { .. } => {
                Some(ReopenFailureMode::StaleCertaintyOverclaim)
            }
            Self::SilentMutatingAction { .. } => Some(ReopenFailureMode::SilentMutatingAction),
            Self::HiddenChannelOwnership { .. } => Some(ReopenFailureMode::HiddenChannelOwnership),
            Self::MissingLiteralTarget { .. } | Self::MissingCanonicalObject { .. } => {
                Some(ReopenFailureMode::IdentityNotPreserved)
            }
            Self::TrustEvaluationBypassed { .. } => {
                Some(ReopenFailureMode::TrustEvaluationBypassed)
            }
            _ => None,
        }
    }
}

/// One per-target reopen row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTargetRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the target.
    pub descriptor: ReopenTargetDescriptor,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<ReopenBlockingFinding>,
    /// `true` when the target is marketed.
    pub marketed: bool,
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in this class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenFindingSummary {
    /// Total blocking findings across the report.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<ReopenFindingCount>,
}

/// Per-surface presence summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenSurfaceCoverage {
    /// Reopen surface this summary covers.
    pub surface_kind: ReopenSurfaceKind,
    /// Number of registered targets on this surface.
    pub target_count: usize,
}

/// Per-availability coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenAvailabilityCoverage {
    /// Availability this summary covers.
    pub availability: ReopenAvailability,
    /// Number of targets that resolve to this availability.
    pub target_count: usize,
    /// Number of those targets that offer at least one recovery action.
    pub with_recovery: usize,
}

/// Per-platform coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenPlatformCoverage {
    /// Platform this summary covers.
    pub platform: ReopenPlatform,
    /// Number of targets claimed on this platform.
    pub target_count: usize,
}

/// A single reopen-identity index entry so platform QA, docs, and support
/// surfaces can quote what each target reopens to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenIdentityEntry {
    /// Reopen target id the entry covers.
    pub reopen_target_id: String,
    /// Reopen surface the entry covers.
    pub surface_kind: ReopenSurfaceKind,
    /// Canonical object identity the literal was registered against.
    pub canonical_object_ref: String,
    /// Object kind in the shared vocabulary.
    pub target_kind: TargetKind,
    /// Availability the reopen resolves to.
    pub availability: ReopenAvailability,
    /// Restore availability the reopen advertises.
    pub restore_availability: RestoreAvailability,
    /// Action class the surface exposes.
    pub action_class: ReopenActionClass,
}

/// One marketed reopen target release tooling should narrow because a control
/// failed or its evidence is stale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenNarrowableEntry {
    /// Reopen target id that must narrow.
    pub reopen_target_id: String,
    /// Failure mode that drives the narrowing, when control-scoped.
    pub failure_mode: Option<ReopenFailureMode>,
    /// Stable reason the target is narrowable.
    pub reason: String,
}

/// Recent-item, dock/taskbar, and jump-list reopen-fidelity report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTargetReport {
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
    /// Required reopen surfaces, in canonical order.
    pub required_surfaces: Vec<ReopenSurfaceKind>,
    /// Union of claimed platforms across all targets, sorted.
    pub claimed_platforms: Vec<ReopenPlatform>,
    /// Cross-links to upstream packets.
    pub cross_links: ReopenTargetCrossLinks,
    /// Per-target rows, sorted by `descriptor.reopen_target_id`.
    pub entries: Vec<ReopenTargetRow>,
    /// Per-surface presence summary, in canonical surface order.
    pub surface_coverage: Vec<ReopenSurfaceCoverage>,
    /// Per-availability coverage summary, in canonical availability order.
    pub availability_coverage: Vec<ReopenAvailabilityCoverage>,
    /// Per-platform coverage summary, in canonical platform order.
    pub platform_coverage: Vec<ReopenPlatformCoverage>,
    /// Per-class blocking-finding summary.
    pub findings_summary: ReopenFindingSummary,
    /// Canonical reopen-identity index, sorted by reopen target id.
    pub identity_index: Vec<ReopenIdentityEntry>,
    /// Number of registered reopen targets present.
    pub registered_target_count: usize,
    /// Number of targets marketed.
    pub marketed_target_count: usize,
    /// Number of targets that land on the exact object.
    pub exact_object_count: usize,
    /// Marketed targets release tooling should narrow.
    pub narrowable_marketed_entries: Vec<ReopenNarrowableEntry>,
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

impl ReopenTargetReport {
    /// Returns `true` when every required reopen surface has at least one
    /// registered target.
    pub fn every_surface_present(&self) -> bool {
        ReopenSurfaceKind::required_surfaces()
            .into_iter()
            .all(|surface| {
                self.entries
                    .iter()
                    .any(|entry| entry.descriptor.surface_kind == surface)
            })
    }

    /// Returns `true` when every degraded availability class is represented, so
    /// the moved-file, missing-root, changed-channel, stale-provider, and
    /// wrong-target reopen failures are all tested and exportable.
    pub fn every_degraded_class_present(&self) -> bool {
        ReopenAvailability::degraded_classes()
            .into_iter()
            .all(|availability| {
                self.entries
                    .iter()
                    .any(|entry| entry.descriptor.availability == availability)
            })
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: targets={}, marketed={}, exact={}, blocking={}, clean={}",
            self.registered_target_count,
            self.marketed_target_count,
            self.exact_object_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for entry in &self.entries {
            lines.push(format!(
                "{}: surface={}, kind={}, avail={}, restore={}, action={}, summary_only={}",
                entry.descriptor.reopen_target_id,
                entry.descriptor.surface_kind.as_str(),
                entry.descriptor.target_kind.as_str(),
                entry.descriptor.availability.as_str(),
                entry.descriptor.restore_availability.as_str(),
                entry.descriptor.action_class.as_str(),
                entry.descriptor.stays_summary_only,
            ));
        }
        for entry in &self.entries {
            for finding in &entry.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.reopen_target_id(),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_entries {
            lines.push(format!(
                "narrowable: {} -- {}",
                narrowable.reopen_target_id, narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 recent-item, dock/taskbar, and jump-list reopen fidelity\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::m5_recent_items_and_reopen`](../../crates/aureline-shell/src/m5_recent_items_and_reopen/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- report-md > \\\n  artifacts/platform/m5-recent-item-and-reopen.md\n",
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
            "- Registered reopen targets: `{}`\n",
            self.registered_target_count
        ));
        out.push_str(&format!(
            "- Marketed reopen targets: `{}`\n",
            self.marketed_target_count
        ));
        out.push_str(&format!(
            "- Exact-object reopen targets: `{}`\n",
            self.exact_object_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed targets: `{}`\n",
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

        out.push_str("## Per-surface coverage\n\n");
        out.push_str(
            "| Reopen surface | Registered targets |\n| -------------- | -----------------: |\n",
        );
        for coverage in &self.surface_coverage {
            out.push_str(&format!(
                "| {} | {} |\n",
                coverage.surface_kind.display_label(),
                coverage.target_count,
            ));
        }
        out.push('\n');

        out.push_str("## Per-availability coverage\n\n");
        out.push_str(
            "| Availability | Targets | With recovery |\n\
             | ------------ | ------: | ------------: |\n",
        );
        for coverage in &self.availability_coverage {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                coverage.availability.display_label(),
                coverage.target_count,
                coverage.with_recovery,
            ));
        }
        out.push('\n');

        out.push_str("## Per-platform coverage\n\n");
        out.push_str("| Platform | Claimed targets |\n| -------- | --------------: |\n");
        for coverage in &self.platform_coverage {
            out.push_str(&format!(
                "| `{}` | {} |\n",
                coverage.platform.as_str(),
                coverage.target_count,
            ));
        }
        out.push('\n');

        out.push_str("## Reopen-identity index\n\n");
        out.push_str(
            "| Reopen target | Surface | Kind | Availability | Restore | Action |\n\
             | ------------- | ------- | ---- | ------------ | ------- | ------ |\n",
        );
        for entry in &self.identity_index {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                entry.reopen_target_id,
                entry.surface_kind.display_label(),
                entry.target_kind.as_str(),
                entry.availability.as_str(),
                entry.restore_availability.as_str(),
                entry.action_class.as_str(),
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

        out.push_str("## Per-target rows\n\n");
        for entry in &self.entries {
            let d = &entry.descriptor;
            out.push_str(&format!(
                "### `{}` ({} on {})\n\n",
                d.reopen_target_id,
                d.target_kind.as_str(),
                d.surface_kind.as_str(),
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                d.descriptor_revision_ref
            ));
            out.push_str(&format!("- Literal target: `{}`\n", d.literal_target_ref));
            out.push_str(&format!(
                "- Canonical object: `{}`\n",
                d.canonical_object_ref
            ));
            if let Some(conflicting) = &d.conflicting_object_ref {
                out.push_str(&format!("- Conflicting object: `{conflicting}`\n"));
            }
            out.push_str(&format!(
                "- Originating channel/build owner: `{}` (`{}`)\n",
                d.originating_channel_build_owner_ref,
                d.ownership_kind.as_str(),
            ));
            out.push_str(&format!(
                "- Side-by-side / portable plausible: `{}`\n",
                d.side_by_side_or_portable_plausible
            ));
            out.push_str(&format!(
                "- Active profile owner: `{}`\n",
                d.active_profile_owner_ref
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                d.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Target freshness: `{}` (captured `{}`)\n",
                d.target_freshness.as_str(),
                d.captured_at,
            ));
            out.push_str(&format!("- Availability: `{}`\n", d.availability.as_str()));
            out.push_str(&format!(
                "- Restore availability: `{}` (trust `{}`, portability `{}`)\n",
                d.restore_availability.as_str(),
                d.trust_state.as_str(),
                d.portability_class_as_str(),
            ));
            out.push_str(&format!(
                "- Action: `{}` (summary-only: `{}`)\n",
                d.action_class.as_str(),
                d.stays_summary_only,
            ));
            if let Some(reviewed) = &d.reviewed_return_surface_ref {
                out.push_str(&format!("- Reviewed return surface: `{reviewed}`\n"));
            }
            out.push_str(&format!(
                "- Canonical command: `{}`\n",
                d.canonical_command_ref
            ));
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
            if let Some(placeholder) = &d.placeholder_label_ref {
                out.push_str(&format!("- Placeholder label: `{placeholder}`\n"));
            }
            out.push_str(&format!(
                "- Restore provenance: `{}`\n",
                d.restore_provenance_ref
            ));
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_reopen_target_fixtures\n");
        out.push_str("python3 tools/ci/m5/reopen_target_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Internal helper so the markdown renderer can name the portability class
/// without depending on a non-`pub` workspace accessor.
trait PortabilityClassLabel {
    /// Returns the stable schema token for a portability class.
    fn portability_class_as_str(&self) -> &'static str;
}

impl PortabilityClassLabel for ReopenTargetDescriptor {
    fn portability_class_as_str(&self) -> &'static str {
        match self.portability_class {
            PortabilityClass::LocalOnly => "local_only",
            PortabilityClass::Synced => "synced",
            PortabilityClass::Imported => "imported",
            PortabilityClass::ProviderLinked => "provider_linked",
            PortabilityClass::Stale => "stale",
        }
    }
}

/// Support-export wrapper for the full reopen-target report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTargetSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: ReopenTargetReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ReopenTargetSupportExport {
    /// Builds the support-export wrapper for a report.
    pub fn from_report(support_export_id: impl Into<String>, report: ReopenTargetReport) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for entry in &report.entries {
            case_ids.push(entry.descriptor.reopen_target_id.clone());
            case_ids.push(entry.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: REOPEN_TARGET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REOPEN_TARGET_SCHEMA_VERSION,
            shared_contract_ref: REOPEN_TARGET_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Per-incident support-export packet for a single degraded reopen target.
///
/// This is the export a reviewer reproduces a moved-file, missing-root,
/// changed-channel, stale-provider, or wrong-target reopen from — the typed
/// diagnostic that replaces a screenshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenTargetCaseExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable case-export id.
    pub case_export_id: String,
    /// Stable case label (e.g. `moved_target`).
    pub case_label: String,
    /// Availability that defines the incident class.
    pub availability: ReopenAvailability,
    /// The reopen-target row in full.
    pub target: ReopenTargetRow,
    /// Recovery actions the incident offers.
    pub recovery_actions: Vec<SafeRecoveryAction>,
    /// Stable reproduction note for support.
    pub reproduction_note: String,
}

impl ReopenTargetCaseExport {
    /// Builds a per-incident case export from a degraded reopen-target row.
    pub fn from_row(
        case_export_id: impl Into<String>,
        case_label: impl Into<String>,
        reproduction_note: impl Into<String>,
        row: ReopenTargetRow,
    ) -> Self {
        let availability = row.descriptor.availability;
        let recovery_actions = row.descriptor.recovery_actions.clone();
        Self {
            record_kind: REOPEN_TARGET_CASE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REOPEN_TARGET_SCHEMA_VERSION,
            shared_contract_ref: REOPEN_TARGET_SHARED_CONTRACT_REF.to_owned(),
            case_export_id: case_export_id.into(),
            case_label: case_label.into(),
            availability,
            target: row,
            recovery_actions,
            reproduction_note: reproduction_note.into(),
        }
    }
}

/// Computes the per-target blocking findings from a descriptor.
fn compute_target_findings(descriptor: &ReopenTargetDescriptor) -> Vec<ReopenBlockingFinding> {
    let mut findings = Vec::new();
    let id = descriptor.reopen_target_id.clone();

    // Exact object identity.
    if descriptor.literal_target_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingLiteralTarget {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.canonical_object_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingCanonicalObject {
            reopen_target_id: id.clone(),
        });
    }

    // Channel/build ownership stays visible.
    if descriptor
        .originating_channel_build_owner_ref
        .trim()
        .is_empty()
    {
        findings.push(ReopenBlockingFinding::HiddenChannelOwnership {
            reopen_target_id: id.clone(),
        });
    }

    // Trust / profile integrity.
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::TrustEvaluationBypassed {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.active_profile_owner_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingActiveProfileOwner {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.canonical_command_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingCanonicalCommand {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingContinuityNote {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor
        .degraded_state_vocabulary
        .iter()
        .all(|phrase| phrase.trim().is_empty())
    {
        findings.push(ReopenBlockingFinding::MissingDegradedStateVocabulary {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.restore_provenance_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingRestoreProvenance {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.claimed_platforms.is_empty() {
        findings.push(ReopenBlockingFinding::MissingClaimedPlatforms {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(ReopenBlockingFinding::MissingDowngradeRule {
            reopen_target_id: id.clone(),
        });
    }
    if !descriptor.registered_on_reopen_harness {
        findings.push(ReopenBlockingFinding::TargetNotOnHarness {
            reopen_target_id: id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == ReopenFreshness::Stale {
        findings.push(ReopenBlockingFinding::StaleEvidenceOnMarketedTarget {
            reopen_target_id: id.clone(),
        });
    }

    // No hidden mutation: a privileged/mutating shortcut must return through a
    // reviewed in-product surface.
    if descriptor.action_class.requires_reviewed_return() {
        let routed = !descriptor.stays_summary_only
            && descriptor
                .reviewed_return_surface_ref
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                == Some(false);
        if !routed {
            findings.push(ReopenBlockingFinding::SilentMutatingAction {
                reopen_target_id: id.clone(),
            });
        }
    }

    // Restore-certainty binding: external re-entry never looks more certain than
    // internal restore. A degraded or stale target may not claim exact restore.
    let degraded = descriptor.availability.requires_recovery();
    let stale_freshness = descriptor.target_freshness == ReopenFreshness::Stale;
    if (degraded || stale_freshness)
        && descriptor.restore_availability == RestoreAvailability::Exact
    {
        findings.push(ReopenBlockingFinding::StaleCertaintyOverclaim {
            reopen_target_id: id.clone(),
            availability: descriptor.availability,
            restore_availability: descriptor.restore_availability,
        });
    }

    // Recovery + placeholder: a non-exact target lands on a labeled placeholder
    // with at least one recovery action, and each unavailable class stays a
    // distinct failure.
    if degraded {
        if descriptor.recovery_actions.is_empty() {
            if let Some(mode) = descriptor.availability.missing_recovery_failure_mode() {
                let finding = match mode {
                    ReopenFailureMode::WrongTargetReopen => {
                        ReopenBlockingFinding::WrongTargetReopen {
                            reopen_target_id: id.clone(),
                        }
                    }
                    _ => ReopenBlockingFinding::UnavailableTargetSilentLoss {
                        reopen_target_id: id.clone(),
                        availability: descriptor.availability,
                    },
                };
                findings.push(finding);
            }
        }
        if descriptor
            .placeholder_label_ref
            .as_deref()
            .map(str::trim)
            .map(str::is_empty)
            != Some(false)
        {
            findings.push(ReopenBlockingFinding::MissingPlaceholderLabel {
                reopen_target_id: id.clone(),
            });
        }
    }

    findings
}

/// Builds a [`ReopenTargetRow`] from a descriptor, computing the per-target
/// blocking findings.
pub fn build_reopen_target_row(descriptor: ReopenTargetDescriptor) -> ReopenTargetRow {
    let marketed = descriptor.marketed;
    let blocking_findings = compute_target_findings(&descriptor);
    ReopenTargetRow {
        record_kind: REOPEN_TARGET_ROW_RECORD_KIND.to_owned(),
        schema_version: REOPEN_TARGET_SCHEMA_VERSION,
        shared_contract_ref: REOPEN_TARGET_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        blocking_findings,
        marketed,
    }
}

/// Computes the per-surface, per-availability, per-platform, and per-class
/// summaries from finished rows.
fn summarize_report(
    entries: &[ReopenTargetRow],
) -> (
    Vec<ReopenSurfaceCoverage>,
    Vec<ReopenAvailabilityCoverage>,
    Vec<ReopenPlatformCoverage>,
    ReopenFindingSummary,
) {
    let mut surface_coverage: Vec<ReopenSurfaceCoverage> = ReopenSurfaceKind::required_surfaces()
        .into_iter()
        .map(|surface_kind| ReopenSurfaceCoverage {
            surface_kind,
            target_count: 0,
        })
        .collect();

    let availability_order = [
        ReopenAvailability::ExactObject,
        ReopenAvailability::MovedTarget,
        ReopenAvailability::MissingRoot,
        ReopenAvailability::ChangedChannel,
        ReopenAvailability::StaleProviderLinked,
        ReopenAvailability::WrongTargetDetected,
    ];
    let mut availability_coverage: Vec<ReopenAvailabilityCoverage> = availability_order
        .into_iter()
        .map(|availability| ReopenAvailabilityCoverage {
            availability,
            target_count: 0,
            with_recovery: 0,
        })
        .collect();

    let mut platform_coverage: Vec<ReopenPlatformCoverage> = ReopenPlatform::all()
        .into_iter()
        .map(|platform| ReopenPlatformCoverage {
            platform,
            target_count: 0,
        })
        .collect();

    let mut class_counts: Vec<ReopenFindingCount> = Vec::new();
    let mut total = 0usize;

    for entry in entries {
        if let Some(surface_row) = surface_coverage
            .iter_mut()
            .find(|row| row.surface_kind == entry.descriptor.surface_kind)
        {
            surface_row.target_count += 1;
        }
        if let Some(avail_row) = availability_coverage
            .iter_mut()
            .find(|row| row.availability == entry.descriptor.availability)
        {
            avail_row.target_count += 1;
            if !entry.descriptor.recovery_actions.is_empty() {
                avail_row.with_recovery += 1;
            }
        }
        for platform in &entry.descriptor.claimed_platforms {
            if let Some(platform_row) = platform_coverage
                .iter_mut()
                .find(|row| row.platform == *platform)
            {
                platform_row.target_count += 1;
            }
        }
        for finding in &entry.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(ReopenFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        surface_coverage,
        availability_coverage,
        platform_coverage,
        ReopenFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed targets release tooling should narrow because a
/// control failed or their evidence is stale.
fn compute_narrowable_entries(entries: &[ReopenTargetRow]) -> Vec<ReopenNarrowableEntry> {
    let mut narrowable = Vec::new();
    for entry in entries {
        if !entry.marketed {
            continue;
        }
        for finding in &entry.blocking_findings {
            narrowable.push(ReopenNarrowableEntry {
                reopen_target_id: entry.descriptor.reopen_target_id.clone(),
                failure_mode: finding.failure_mode(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a full [`ReopenTargetReport`] from per-target rows.
pub fn build_reopen_target_report(entries: Vec<ReopenTargetRow>) -> ReopenTargetReport {
    let mut entries = entries;
    entries.sort_by(|left, right| {
        left.descriptor
            .reopen_target_id
            .cmp(&right.descriptor.reopen_target_id)
    });

    let registered_target_count = entries.len();
    let marketed_target_count = entries.iter().filter(|entry| entry.marketed).count();
    let exact_object_count = entries
        .iter()
        .filter(|entry| entry.descriptor.availability == ReopenAvailability::ExactObject)
        .count();

    let (surface_coverage, availability_coverage, platform_coverage, findings_summary) =
        summarize_report(&entries);
    let narrowable_marketed_entries = compute_narrowable_entries(&entries);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut platform_set: Vec<ReopenPlatform> = Vec::new();
    for entry in &entries {
        for platform in &entry.descriptor.claimed_platforms {
            if !platform_set.contains(platform) {
                platform_set.push(*platform);
            }
        }
    }
    platform_set.sort();

    let mut identity_index: Vec<ReopenIdentityEntry> = entries
        .iter()
        .map(|entry| ReopenIdentityEntry {
            reopen_target_id: entry.descriptor.reopen_target_id.clone(),
            surface_kind: entry.descriptor.surface_kind,
            canonical_object_ref: entry.descriptor.canonical_object_ref.clone(),
            target_kind: entry.descriptor.target_kind,
            availability: entry.descriptor.availability,
            restore_availability: entry.descriptor.restore_availability,
            action_class: entry.descriptor.action_class,
        })
        .collect();
    identity_index.sort_by(|left, right| left.reopen_target_id.cmp(&right.reopen_target_id));

    ReopenTargetReport {
        record_kind: REOPEN_TARGET_REPORT_RECORD_KIND.to_owned(),
        schema_version: REOPEN_TARGET_SCHEMA_VERSION,
        shared_contract_ref: REOPEN_TARGET_SHARED_CONTRACT_REF.to_owned(),
        report_id: REOPEN_TARGET_REPORT_ID.to_owned(),
        source_schema_ref: REOPEN_TARGET_SOURCE_SCHEMA_REF.to_owned(),
        required_surfaces: ReopenSurfaceKind::required_surfaces().to_vec(),
        claimed_platforms: platform_set,
        cross_links: ReopenTargetCrossLinks::canonical(),
        entries,
        surface_coverage,
        availability_coverage,
        platform_coverage,
        findings_summary,
        identity_index,
        registered_target_count,
        marketed_target_count,
        exact_object_count,
        narrowable_marketed_entries,
        report_clean,
        published_report_ref: REOPEN_TARGET_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: REOPEN_TARGET_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            REOPEN_TARGET_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/recent_items_and_reopen.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-reopen-targets".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_reopen_target_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ReopenTargetValidationError {
    /// The report has no registered targets.
    NoRegisteredTargets,
    /// A required reopen surface has no registered target.
    RequiredSurfaceMissing { surface_kind: String },
    /// A required degraded availability class has no registered target.
    RequiredDegradedClassMissing { availability: String },
    /// A blocking finding remains on a target.
    BlockingFindingPresent {
        reopen_target_id: String,
        class: String,
    },
    /// A cross-link ref is empty.
    CrossLinkMissing { field: String },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A target's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { reopen_target_id: String },
}

/// Validates a report against the reopen-target acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_reopen_target_report(
    report: &ReopenTargetReport,
) -> Result<(), Vec<ReopenTargetValidationError>> {
    let mut errors = Vec::new();

    if report.entries.is_empty() {
        errors.push(ReopenTargetValidationError::NoRegisteredTargets);
    }

    for surface in ReopenSurfaceKind::required_surfaces() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.surface_kind == surface);
        if !present {
            errors.push(ReopenTargetValidationError::RequiredSurfaceMissing {
                surface_kind: surface.as_str().to_owned(),
            });
        }
    }

    for availability in ReopenAvailability::degraded_classes() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.availability == availability);
        if !present {
            errors.push(ReopenTargetValidationError::RequiredDegradedClassMissing {
                availability: availability.as_str().to_owned(),
            });
        }
    }

    for entry in &report.entries {
        if entry.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(ReopenTargetValidationError::MissingDescriptorRevisionRef {
                reopen_target_id: entry.descriptor.reopen_target_id.clone(),
            });
        }
        for finding in &entry.blocking_findings {
            errors.push(ReopenTargetValidationError::BlockingFindingPresent {
                reopen_target_id: finding.reopen_target_id().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(ReopenTargetValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(ReopenTargetValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(ReopenTargetValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_reopen_target_report`].
struct ReopenSeed {
    reopen_target_id: &'static str,
    surface_kind: ReopenSurfaceKind,
    literal_target_ref: &'static str,
    canonical_object_ref: &'static str,
    conflicting_object_ref: Option<&'static str>,
    target_kind: TargetKind,
    ownership_kind: ReopenOwnershipKind,
    side_by_side_or_portable_plausible: bool,
    target_freshness: ReopenFreshness,
    availability: ReopenAvailability,
    restore_availability: RestoreAvailability,
    trust_state: TrustState,
    portability_class: PortabilityClass,
    action_class: ReopenActionClass,
    reviewed_return_surface_ref: Option<&'static str>,
    canonical_command_ref: &'static str,
    recovery_actions: &'static [SafeRecoveryAction],
    placeholder_label_ref: Option<&'static str>,
    continuity_note: &'static str,
    degraded_state_vocabulary: &'static [&'static str],
    claimed_platforms: &'static [ReopenPlatform],
}

fn build_target_from_seed(seed: &ReopenSeed) -> ReopenTargetRow {
    let stays_summary_only = !seed.action_class.requires_reviewed_return();
    let descriptor = ReopenTargetDescriptor {
        reopen_target_id: seed.reopen_target_id.to_owned(),
        surface_kind: seed.surface_kind,
        descriptor_revision_ref: format!("{}:rev:2026.06.01-01", seed.reopen_target_id),
        primary_label_ref: format!("label:{}:primary", seed.reopen_target_id),
        literal_target_ref: seed.literal_target_ref.to_owned(),
        canonical_object_ref: seed.canonical_object_ref.to_owned(),
        conflicting_object_ref: seed.conflicting_object_ref.map(str::to_owned),
        target_kind: seed.target_kind,
        originating_channel_build_owner_ref: format!("channel-owner:{}", seed.reopen_target_id),
        ownership_kind: seed.ownership_kind,
        side_by_side_or_portable_plausible: seed.side_by_side_or_portable_plausible,
        active_profile_owner_ref: format!("profile-owner:{}", seed.reopen_target_id),
        trust_checkpoint_ref: format!("trust:{}:profile_tenant_policy", seed.reopen_target_id),
        target_freshness: seed.target_freshness,
        captured_at: GENERATED_AT.to_owned(),
        availability: seed.availability,
        restore_availability: seed.restore_availability,
        trust_state: seed.trust_state,
        portability_class: seed.portability_class,
        action_class: seed.action_class,
        stays_summary_only,
        reviewed_return_surface_ref: seed.reviewed_return_surface_ref.map(str::to_owned),
        canonical_command_ref: seed.canonical_command_ref.to_owned(),
        recovery_actions: seed.recovery_actions.to_vec(),
        placeholder_label_ref: seed.placeholder_label_ref.map(str::to_owned),
        continuity_note: seed.continuity_note.to_owned(),
        degraded_state_vocabulary: seed
            .degraded_state_vocabulary
            .iter()
            .map(|phrase| (*phrase).to_owned())
            .collect(),
        restore_provenance_ref: REOPEN_TARGET_RESTORE_PROVENANCE_REF.to_owned(),
        claimed_platforms: seed.claimed_platforms.to_vec(),
        evidence_freshness: ReopenFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:reopen_target:narrow_on_stale_evidence".to_owned(),
        marketed: true,
        registered_on_reopen_harness: true,
    };
    build_reopen_target_row(descriptor)
}

const ALL_PLATFORMS: &[ReopenPlatform] = &[
    ReopenPlatform::Macos,
    ReopenPlatform::Windows,
    ReopenPlatform::Linux,
];
const MACOS_ONLY: &[ReopenPlatform] = &[ReopenPlatform::Macos];
const WINDOWS_ONLY: &[ReopenPlatform] = &[ReopenPlatform::Windows];

const REOPEN_SEEDS: &[ReopenSeed] = &[
    // ---- Clean exact rows: one per required reopen surface. ----
    // Recent-item reopen of a single file: the fast exact-object path.
    ReopenSeed {
        reopen_target_id: "reopen:recent_item.exact",
        surface_kind: ReopenSurfaceKind::RecentItem,
        literal_target_ref: "literal:recent_item.exact:captured",
        canonical_object_ref: "canonical:recent_item.exact:single_file",
        conflicting_object_ref: None,
        target_kind: TargetKind::LocalFile,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Fresh,
        availability: ReopenAvailability::ExactObject,
        restore_availability: RestoreAvailability::Exact,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[],
        placeholder_label_ref: None,
        continuity_note: "A recent-item reopen of a still-present file lands on the exact object in the active profile with the originating channel owner shown next to it.",
        degraded_state_vocabulary: &[
            "Reopen this file",
            "This file is no longer available",
            "Locate the file",
        ],
        claimed_platforms: ALL_PLATFORMS,
    },
    // Dock reopen of a repository: exact object, channel owner visible.
    ReopenSeed {
        reopen_target_id: "reopen:dock.exact",
        surface_kind: ReopenSurfaceKind::Dock,
        literal_target_ref: "literal:dock.exact:captured",
        canonical_object_ref: "canonical:dock.exact:repo_root",
        conflicting_object_ref: None,
        target_kind: TargetKind::LocalRepoRoot,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Fresh,
        availability: ReopenAvailability::ExactObject,
        restore_availability: RestoreAvailability::Exact,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[],
        placeholder_label_ref: None,
        continuity_note: "A dock recent-documents reopen of a repository lands on the exact root in the active profile and names the channel that owns the dock registration.",
        degraded_state_vocabulary: &[
            "Reopen this repository",
            "This repository moved or was removed",
            "Locate the repository",
        ],
        claimed_platforms: MACOS_ONLY,
    },
    // Taskbar reveal of a folder: exact object, reveal action stays summary-safe.
    ReopenSeed {
        reopen_target_id: "reopen:taskbar.exact",
        surface_kind: ReopenSurfaceKind::Taskbar,
        literal_target_ref: "literal:taskbar.exact:captured",
        canonical_object_ref: "canonical:taskbar.exact:folder_root",
        conflicting_object_ref: None,
        target_kind: TargetKind::LocalFolder,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Fresh,
        availability: ReopenAvailability::ExactObject,
        restore_availability: RestoreAvailability::Exact,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::RevealObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.reveal.target",
        recovery_actions: &[],
        placeholder_label_ref: None,
        continuity_note: "A taskbar recent reveal of a folder opens the OS file manager at the exact location and never silently opens or mutates the workspace.",
        degraded_state_vocabulary: &[
            "Reveal this folder",
            "This folder moved or was removed",
            "Locate the folder",
        ],
        claimed_platforms: WINDOWS_ONLY,
    },
    // Jump-list reopen of a workspace manifest: exact object, compatible restore.
    ReopenSeed {
        reopen_target_id: "reopen:jump_list.exact",
        surface_kind: ReopenSurfaceKind::JumpList,
        literal_target_ref: "literal:jump_list.exact:captured",
        canonical_object_ref: "canonical:jump_list.exact:workspace_manifest",
        conflicting_object_ref: None,
        target_kind: TargetKind::WorkspaceManifest,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Fresh,
        availability: ReopenAvailability::ExactObject,
        restore_availability: RestoreAvailability::Compatible,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[],
        placeholder_label_ref: None,
        continuity_note: "A pinned jump-list reopen of a workspace lands on the exact manifest with a compatible restore, never silently widening into an unrelated workspace.",
        degraded_state_vocabulary: &[
            "Reopen this workspace",
            "This workspace moved or was removed",
            "Locate the workspace",
        ],
        claimed_platforms: WINDOWS_ONLY,
    },
    // ---- Degraded corpus rows: the five required reopen incidents. ----
    // Moved files: a recent-item reopen whose file moved.
    ReopenSeed {
        reopen_target_id: "reopen:case.moved_target",
        surface_kind: ReopenSurfaceKind::RecentItem,
        literal_target_ref: "literal:case.moved_target:captured",
        canonical_object_ref: "canonical:case.moved_target:single_file",
        conflicting_object_ref: None,
        target_kind: TargetKind::LocalFile,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Stale,
        availability: ReopenAvailability::MovedTarget,
        restore_availability: RestoreAvailability::LayoutOnly,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        placeholder_label_ref: Some("placeholder:case.moved_target:moved_target"),
        continuity_note: "A recent-item reopen whose file moved shows a truthful moved-target placeholder with a locate action and preserves the cached identity until a new location is selected.",
        degraded_state_vocabulary: &[
            "This item moved or was removed",
            "Locate the file",
            "Remove from list",
        ],
        claimed_platforms: ALL_PLATFORMS,
    },
    // Missing roots: a jump-list reopen whose workspace root is unmounted.
    ReopenSeed {
        reopen_target_id: "reopen:case.missing_root",
        surface_kind: ReopenSurfaceKind::JumpList,
        literal_target_ref: "literal:case.missing_root:captured",
        canonical_object_ref: "canonical:case.missing_root:workspace_manifest",
        conflicting_object_ref: None,
        target_kind: TargetKind::WorkspaceManifest,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Stale,
        availability: ReopenAvailability::MissingRoot,
        restore_availability: RestoreAvailability::LayoutOnly,
        trust_state: TrustState::Trusted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        placeholder_label_ref: Some("placeholder:case.missing_root:missing_root"),
        continuity_note: "A pinned jump-list reopen whose root volume or share is unmounted shows a missing-root placeholder with locate and open-without-restore actions rather than opening an empty shell.",
        degraded_state_vocabulary: &[
            "This workspace root is missing or unmounted",
            "Locate the workspace",
            "Open anyway without restore",
        ],
        claimed_platforms: WINDOWS_ONLY,
    },
    // Changed channels: a dock reopen now owned by a side-by-side install.
    ReopenSeed {
        reopen_target_id: "reopen:case.changed_channel",
        surface_kind: ReopenSurfaceKind::Dock,
        literal_target_ref: "literal:case.changed_channel:captured",
        canonical_object_ref: "canonical:case.changed_channel:repo_root",
        conflicting_object_ref: None,
        target_kind: TargetKind::LocalRepoRoot,
        ownership_kind: ReopenOwnershipKind::SharedDefaultArbitrated,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Stale,
        availability: ReopenAvailability::ChangedChannel,
        restore_availability: RestoreAvailability::LayoutOnly,
        trust_state: TrustState::PendingEvaluation,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        placeholder_label_ref: Some("placeholder:case.changed_channel:changed_channel"),
        continuity_note: "A dock reopen whose registration is now owned by a side-by-side or portable channel shows a changed-channel placeholder naming the owning channel rather than silently reopening under the wrong build.",
        degraded_state_vocabulary: &[
            "Another channel now owns this recent item",
            "Reopen in the channel that owns it",
            "Remove from list",
        ],
        claimed_platforms: MACOS_ONLY,
    },
    // Stale provider-linked objects: a taskbar resume routed through review.
    ReopenSeed {
        reopen_target_id: "reopen:case.stale_provider_linked",
        surface_kind: ReopenSurfaceKind::Taskbar,
        literal_target_ref: "literal:case.stale_provider_linked:captured",
        canonical_object_ref: "canonical:case.stale_provider_linked:cloud_workspace",
        conflicting_object_ref: None,
        target_kind: TargetKind::ManagedCloudWorkspace,
        ownership_kind: ReopenOwnershipKind::ManagedFleetOwned,
        side_by_side_or_portable_plausible: false,
        target_freshness: ReopenFreshness::Stale,
        availability: ReopenAvailability::StaleProviderLinked,
        restore_availability: RestoreAvailability::EvidenceOnly,
        trust_state: TrustState::PendingEvaluation,
        portability_class: PortabilityClass::ProviderLinked,
        action_class: ReopenActionClass::PrivilegedOrMutating,
        reviewed_return_surface_ref: Some("artifacts/auth/m5_auth_and_recovery.md"),
        canonical_command_ref: "cmd:auth.resume_pending_sign_in",
        recovery_actions: &[
            SafeRecoveryAction::Reauth,
            SafeRecoveryAction::Reconnect,
            SafeRecoveryAction::OpenReadOnlyCachedView,
        ],
        placeholder_label_ref: Some("placeholder:case.stale_provider_linked:stale_provider_linked"),
        continuity_note: "A taskbar reopen of a provider-linked cloud workspace whose authority went stale is privileged, so it routes through the reviewed auth-recovery surface to reauthorize instead of mutating provider state directly from the shortcut.",
        degraded_state_vocabulary: &[
            "This cloud workspace needs reauthorization",
            "Reauthorize to reopen",
            "Open a read-only cached view",
        ],
        claimed_platforms: WINDOWS_ONLY,
    },
    // Wrong-target detected: a recent-item literal now points at another object.
    ReopenSeed {
        reopen_target_id: "reopen:case.wrong_target",
        surface_kind: ReopenSurfaceKind::RecentItem,
        literal_target_ref: "literal:case.wrong_target:captured",
        canonical_object_ref: "canonical:case.wrong_target:single_file",
        conflicting_object_ref: Some("canonical:case.wrong_target:conflicting_file"),
        target_kind: TargetKind::LocalFile,
        ownership_kind: ReopenOwnershipKind::ChannelScopedOwner,
        side_by_side_or_portable_plausible: true,
        target_freshness: ReopenFreshness::Stale,
        availability: ReopenAvailability::WrongTargetDetected,
        restore_availability: RestoreAvailability::None,
        trust_state: TrustState::Restricted,
        portability_class: PortabilityClass::LocalOnly,
        action_class: ReopenActionClass::ReopenObject,
        reviewed_return_surface_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        recovery_actions: &[
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        placeholder_label_ref: Some("placeholder:case.wrong_target:wrong_target_detected"),
        continuity_note: "A recent-item reopen whose captured literal now resolves to a different object than the one registered is detected and shown as a wrong-target placeholder naming the conflicting object, never silently reopened.",
        degraded_state_vocabulary: &[
            "This item now points at a different object",
            "Locate the original object",
            "Remove from list",
        ],
        claimed_platforms: ALL_PLATFORMS,
    },
];

/// Seeded report builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5-reopen-targets/`.
pub fn seeded_reopen_target_report() -> ReopenTargetReport {
    let entries = REOPEN_SEEDS.iter().map(build_target_from_seed).collect();
    build_reopen_target_report(entries)
}

/// Stable case-id labels for the five required incident fixtures, in canonical
/// order: moved files, missing roots, changed channels, stale provider-linked
/// objects, and a detected wrong-target reopen.
pub const REOPEN_TARGET_CASE_LABELS: [(&str, &str); 5] = [
    ("reopen:case.moved_target", "moved_target"),
    ("reopen:case.missing_root", "missing_root"),
    ("reopen:case.changed_channel", "changed_channel"),
    ("reopen:case.stale_provider_linked", "stale_provider_linked"),
    ("reopen:case.wrong_target", "wrong_target"),
];

/// Builds the five per-incident case exports from the seeded report, in
/// canonical order.
pub fn seeded_reopen_target_case_exports() -> Vec<ReopenTargetCaseExport> {
    let report = seeded_reopen_target_report();
    REOPEN_TARGET_CASE_LABELS
        .iter()
        .filter_map(|(reopen_target_id, label)| {
            let row = report
                .entries
                .iter()
                .find(|entry| entry.descriptor.reopen_target_id == *reopen_target_id)?
                .clone();
            Some(ReopenTargetCaseExport::from_row(
                format!("support-export:m5-reopen-targets:case:{label}"),
                *label,
                format!(
                    "Reproduce the {label} reopen from this typed target: the literal the shortcut held, the canonical object it was registered against, the resolved availability, and the offered recovery actions.",
                ),
                row,
            ))
        })
        .collect()
}
