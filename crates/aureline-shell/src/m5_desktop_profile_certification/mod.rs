//! Canonical desktop-profile certification for every claimed M5 desktop profile.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface family —
//! notebook, data grid, profiler, pipeline, docs, preview, review, incident, companion, and
//! operator — to the shell slots it may attach to, the responsive collapse ladder it must
//! survive, the window classes it may live in, and the owning-window routing it must honor.
//! Six sibling capstones certify those promises one *dimension* at a time (slot occupancy,
//! responsive collapse, min-width guards, multi-window parity, owning-window routing, and
//! window-lifecycle safety). This lane is the **desktop-profile** capstone that certifies all
//! four continuity truths *together* on every claimed M5 desktop profile: for each profile —
//! compact, standard, and expanded widths, mixed-DPI, multi-monitor, and dependency-missing
//! restore — it certifies that across every claimed surface **shell-zone integrity holds** (no
//! surface invents a private slot), **adaptive layout preserves task identity** (responsive
//! collapse never changes identity, hides critical state, or forces an unusable narrow pane),
//! **multi-window truth stays global while layout stays local** (every window preserves
//! workspace trust, remote, profile, and recovery truth), and **owning-window routing returns
//! contextful actions to the owning window and object** (no wrong-window reopen, focus theft,
//! or orphaning).
//!
//! Three records carry the truth:
//!
//! - the per-profile **certification row** ([`DesktopProfileRow`]): one row per
//!   [`M5DesktopProfile`] naming the claimed surface families it evaluated (pulled from the
//!   matrix), its shell-zone-integrity / adaptive-layout / multi-window-truth /
//!   owning-window-routing posture, any active waiver, and a derived green/yellow/red
//!   [`DesktopProfileStatus`].
//! - the release **certification packet** ([`DesktopProfilePacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers, the
//!   exact profile causes ([`DesktopProfileCause`]), and the blocking findings the lane
//!   refuses to ship with.
//! - the **certification dashboard** ([`DesktopProfileDashboard`]): a light projection the
//!   shell / windowing / layout / status automation reads to auto-narrow a claimed surface's
//!   desktop profile when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the
//! moment a surface falls back to a disclosed but still-declared slot, responsive collapse
//! takes a disclosed docked→sheet/overflow narrowing while preserving identity, a workspace
//! truth is projected in a disclosed reduced form until a dependency is restored, or a routed
//! action is deferred to a disclosed, waivered relocation into the primary window; it drops to
//! `red` if a claimed surface invents a private shell slot, responsive collapse changes task
//! identity or forces an unusable narrow pane, workspace-global truth diverges across windows,
//! a routed action is lost to focus theft, orphaning, or a wrong-window reopen, or the profile
//! fails to evaluate every claimed surface family. That derivation is the auto-narrowing the
//! acceptance criteria require, and the evaluated-family completeness check is the lint that
//! prevents a profile audit from silently regressing into a partial, single-surface view.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw
//! local paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The surface family and downgrade-trigger
//! vocabulary is re-exported by reference from the already frozen [matrix]; the evaluated
//! families are pulled straight from that matrix's seeded packet, so this lane mints no
//! parallel shell vocabulary and cannot certify a family the matrix does not freeze. Only the
//! desktop-profile-specific vocabulary ([`M5DesktopProfile`], [`M5DesktopTruthDimension`],
//! [`DesktopProfileStatus`], [`ShellZoneIntegrityState`], [`AdaptiveLayoutState`],
//! [`MultiWindowTruthState`], [`OwningWindowRoutingState`], [`DesktopProfileWaiver`],
//! [`DesktopProfileCause`], [`DesktopProfileFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix as matrix;

pub use matrix::{
    M5ShellConsumerSurface, M5ShellDowngradeTrigger, M5ShellQualificationClass,
    M5ShellSurfaceFamily, M5ShellZoneSlot, M5WindowClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_desktop_profile_certification_packet,
    seeded_m5_desktop_profile_certification_packet_compact_private_slot_drift_blocked,
    seeded_m5_desktop_profile_certification_packet_compact_unusable_pane_blocked,
    seeded_m5_desktop_profile_certification_packet_dependency_restore_routing_lost_blocked,
    seeded_m5_desktop_profile_certification_packet_multi_monitor_truth_diverged_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "shell:m5_desktop_profile_certification:v1";

/// Stable record kind for [`DesktopProfilePacket`] payloads.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "shell_m5_desktop_profile_certification_packet_record";

/// Stable record kind for [`DesktopProfileDashboard`] payloads.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_desktop_profile_certification_dashboard_record";

/// Stable record kind for [`DesktopProfileSupportExport`] payloads.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_desktop_profile_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PACKET_ID: &str =
    "m5-desktop-profile-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_DASHBOARD_ID: &str =
    "m5-desktop-profile-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-desktop-profile-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-desktop-profile-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-desktop-profile-certification.md";

/// Published certification-packet artifact ref.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-desktop-profile-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-desktop-profile-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-desktop-profile-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-desktop-profile-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_desktop_profile_certification_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Shell-zoning contract this proof mirrors for shell-zone integrity.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SHELL_ZONING_CONTRACT_REF: &str =
    matrix::M5_SHELL_ZONING_CONTRACT_REF;

/// Reference-layout contract this proof mirrors for adaptive-layout continuity.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_REFERENCE_LAYOUT_CONTRACT_REF: &str =
    matrix::M5_SHELL_REFERENCE_LAYOUT_CONTRACT_REF;

/// Window-topology contract this proof mirrors for multi-window truth parity.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_WINDOW_TOPOLOGY_CONTRACT_REF: &str =
    matrix::M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF;

/// Attention-routing contract this proof mirrors for owning-window routing.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_ATTENTION_ROUTING_CONTRACT_REF: &str =
    matrix::M5_SHELL_ATTENTION_ROUTING_CONTRACT_REF;

/// Session-restore fidelity contract this proof mirrors for dependency-missing restore.
pub const M5_DESKTOP_PROFILE_CERTIFICATION_SESSION_RESTORE_CONTRACT_REF: &str =
    matrix::M5_SHELL_SESSION_RESTORE_CONTRACT_REF;

/// Every governed surface family a profile row must evaluate, in canonical order.
/// These are exactly the families the frozen shell-zone matrix freezes; a profile that
/// evaluates fewer regresses into a partial view and blocks.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// Every claimed M5 desktop profile the certification must cover, in canonical order.
pub const REQUIRED_PROFILES: [M5DesktopProfile; 6] = M5DesktopProfile::ALL;

/// Every continuity truth dimension each profile row certifies, in canonical order.
pub const REQUIRED_TRUTH_DIMENSIONS: [M5DesktopTruthDimension; 4] = M5DesktopTruthDimension::ALL;

/// A claimed M5 desktop profile the certification covers.
///
/// The six profiles are the desktop conditions Aureline already claims: compact, standard,
/// and expanded widths, mixed-DPI across displays, multi-monitor topology, and a
/// dependency-missing restore after crash or restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopProfile {
    /// Compact desktop: narrow width, zoom, or a secondary compact display.
    CompactDesktop,
    /// Standard desktop: default working width.
    StandardDesktop,
    /// Expanded desktop: wide primary display.
    ExpandedDesktop,
    /// Mixed-DPI: windows spanning displays with different scale factors.
    MixedDpi,
    /// Multi-monitor: secondary displays and window/display topology change.
    MultiMonitor,
    /// Dependency-missing restore: crash/restore where an extension, remote, or feature pack
    /// is unavailable.
    DependencyMissingRestore,
}

impl M5DesktopProfile {
    /// Every desktop profile, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CompactDesktop,
        Self::StandardDesktop,
        Self::ExpandedDesktop,
        Self::MixedDpi,
        Self::MultiMonitor,
        Self::DependencyMissingRestore,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
            Self::MixedDpi => "mixed_dpi",
            Self::MultiMonitor => "multi_monitor",
            Self::DependencyMissingRestore => "dependency_missing_restore",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CompactDesktop => "Compact desktop (narrow width / zoom)",
            Self::StandardDesktop => "Standard desktop (default width)",
            Self::ExpandedDesktop => "Expanded desktop (wide display)",
            Self::MixedDpi => "Mixed-DPI (per-display scale factors)",
            Self::MultiMonitor => "Multi-monitor (secondary displays / topology change)",
            Self::DependencyMissingRestore => "Dependency-missing restore (crash / restart)",
        }
    }
}

/// A continuity truth dimension each profile row certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopTruthDimension {
    /// Shell-zone integrity: every surface attaches only to declared shell slots.
    ShellZoneIntegrity,
    /// Adaptive-layout continuity: responsive collapse preserves task identity.
    AdaptiveLayout,
    /// Multi-window truth: workspace-global truth stays global while layout stays local.
    MultiWindowTruth,
    /// Owning-window routing: routed actions return to the owning window and object.
    OwningWindowRouting,
}

impl M5DesktopTruthDimension {
    /// Every truth dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ShellZoneIntegrity,
        Self::AdaptiveLayout,
        Self::MultiWindowTruth,
        Self::OwningWindowRouting,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellZoneIntegrity => "shell_zone_integrity",
            Self::AdaptiveLayout => "adaptive_layout",
            Self::MultiWindowTruth => "multi_window_truth",
            Self::OwningWindowRouting => "owning_window_routing",
        }
    }
}

/// The derived desktop-profile-certification light a profile carries.
///
/// `green` means every claimed surface stays in a declared shell slot, responsive collapse
/// preserves identity with a usable pane, every window preserves workspace-global truth while
/// layout stays local, and routed actions return to the owning window and object. `yellow` is
/// a disclosed narrowing (a surface falls back to a disclosed but still-declared slot, collapse
/// takes a disclosed identity-preserving narrowing, a truth is projected in a disclosed reduced
/// form, or a routed action is deferred to a disclosed waivered relocation). `red` is blocked:
/// a surface invents a private slot, collapse changes identity or forces an unusable pane,
/// workspace truth diverges across windows, a routed action is lost to focus theft, orphaning,
/// or a wrong-window reopen, or the profile did not evaluate every claimed surface family — and
/// it may not keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopProfileStatus {
    /// Full standing: all four continuity truths hold across every claimed surface.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl DesktopProfileStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the profile keeps every claimed surface attached only to declared shell slots.
///
/// `all_surfaces_in_declared_slots` means every claimed surface stays in its declared
/// canonical shell slot under this profile. `disclosed_slot_fallback_narrowing` means a
/// surface falls back to its declared *fallback* slot (a disclosed, still-declared slot)
/// because a dependency is unavailable — a yellow narrowing. `private_slot_drift_detected`
/// means a claimed surface attached outside any declared shell slot — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellZoneIntegrityState {
    /// Every claimed surface stays in a declared shell slot.
    AllSurfacesInDeclaredSlots,
    /// A surface falls back to its declared fallback slot, disclosed.
    DisclosedSlotFallbackNarrowing,
    /// A claimed surface invented a private shell slot — a blocker.
    PrivateSlotDriftDetected,
}

impl ShellZoneIntegrityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllSurfacesInDeclaredSlots => "all_surfaces_in_declared_slots",
            Self::DisclosedSlotFallbackNarrowing => "disclosed_slot_fallback_narrowing",
            Self::PrivateSlotDriftDetected => "private_slot_drift_detected",
        }
    }

    /// `true` when every surface stays in a declared slot at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::AllSurfacesInDeclaredSlots)
    }

    /// `true` when the profile took a disclosed slot-fallback narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedSlotFallbackNarrowing)
    }
}

/// How the profile keeps responsive collapse from changing task identity or forcing an
/// unusable narrow pane.
///
/// `identity_stable_no_unusable_pane` means responsive collapse preserves task identity and
/// keeps every pane usable under this profile. `disclosed_collapse_narrowing` means collapse
/// takes a disclosed docked→sheet/overflow narrowing while preserving identity and the reopen
/// path — a yellow narrowing. `identity_lost_or_unusable_pane` means collapse changed task
/// identity, hid critical state instead of overflowing it, or forced an unusable narrow pane —
/// always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveLayoutState {
    /// Collapse preserves identity and keeps every pane usable.
    IdentityStableNoUnusablePane,
    /// Collapse takes a disclosed identity-preserving narrowing.
    DisclosedCollapseNarrowing,
    /// Collapse changed identity, hid critical state, or forced an unusable pane — a blocker.
    IdentityLostOrUnusablePane,
}

impl AdaptiveLayoutState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityStableNoUnusablePane => "identity_stable_no_unusable_pane",
            Self::DisclosedCollapseNarrowing => "disclosed_collapse_narrowing",
            Self::IdentityLostOrUnusablePane => "identity_lost_or_unusable_pane",
        }
    }

    /// `true` when collapse preserves identity at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::IdentityStableNoUnusablePane)
    }

    /// `true` when the profile took a disclosed collapse narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedCollapseNarrowing)
    }
}

/// How the profile keeps workspace-global truth global while keeping layout local.
///
/// `all_truths_preserved_layout_local` means every window preserves workspace trust, remote,
/// profile, and recovery truth while density/focus/layout stay local under this profile.
/// `disclosed_truth_projection_narrowing` means a workspace truth is projected in a disclosed
/// reduced form until a dependency is restored, while still visible in every window — a yellow
/// narrowing. `workspace_truth_diverged_across_windows` means a workspace-global truth diverged
/// across windows — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultiWindowTruthState {
    /// Every window preserves all workspace-global truth while layout stays local.
    AllTruthsPreservedLayoutLocal,
    /// A workspace truth is projected in a disclosed reduced form.
    DisclosedTruthProjectionNarrowing,
    /// A workspace-global truth diverged across windows — a blocker.
    WorkspaceTruthDivergedAcrossWindows,
}

impl MultiWindowTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllTruthsPreservedLayoutLocal => "all_truths_preserved_layout_local",
            Self::DisclosedTruthProjectionNarrowing => "disclosed_truth_projection_narrowing",
            Self::WorkspaceTruthDivergedAcrossWindows => "workspace_truth_diverged_across_windows",
        }
    }

    /// `true` when every window preserves all truth at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::AllTruthsPreservedLayoutLocal)
    }

    /// `true` when the profile took a disclosed truth-projection narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedTruthProjectionNarrowing)
    }
}

/// How the profile routes dialogs, notifications, and approvals back to the owning window and
/// object without focus theft, orphaning, or a wrong-window reopen.
///
/// `routes_to_owning_object_no_focus_theft` means routed actions return to the owning window
/// and object, reopen the exact object or a truthful placeholder, and never steal focus or
/// orphan. `disclosed_routing_relocation` means a routed action from a closed window is
/// relocated to a disclosed, waivered still-visible prompt in the primary window rather than
/// blocking outright — a yellow narrowing. `routing_lost_focus_theft_or_orphan` means a routed
/// action was lost to focus theft, orphaning, or a wrong-window reopen — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwningWindowRoutingState {
    /// Routed actions return to the owning window and object with no focus theft or orphan.
    RoutesToOwningObjectNoFocusTheft,
    /// A routed action is deferred to a disclosed, waivered relocation.
    DisclosedRoutingRelocation,
    /// A routed action was lost to focus theft, orphaning, or a wrong-window reopen — blocker.
    RoutingLostFocusTheftOrOrphan,
}

impl OwningWindowRoutingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutesToOwningObjectNoFocusTheft => "routes_to_owning_object_no_focus_theft",
            Self::DisclosedRoutingRelocation => "disclosed_routing_relocation",
            Self::RoutingLostFocusTheftOrOrphan => "routing_lost_focus_theft_or_orphan",
        }
    }

    /// `true` when routing returns to the owning object at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::RoutesToOwningObjectNoFocusTheft)
    }

    /// `true` when the profile took a disclosed routing-relocation narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedRoutingRelocation)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed
/// (yellow) rather than blocked — never lets a private slot, an identity loss, a truth
/// divergence, or a lost routed action hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfileWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The desktop profile the waiver applies to.
    pub profile: M5DesktopProfile,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl DesktopProfileWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a desktop profile's certification.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfileCause {
    /// The desktop profile the cause applies to.
    pub profile: M5DesktopProfile,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed
    /// cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl DesktopProfileCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One claimed desktop profile, certified across its shell-zone-integrity,
/// adaptive-layout, multi-window-truth, and owning-window-routing posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfileRow {
    /// The desktop profile being certified.
    pub profile: M5DesktopProfile,
    /// Short reviewer-facing profile label.
    pub profile_label: String,
    /// Owner role accountable for keeping this profile governed.
    pub owner_role: String,
    /// Short scenario summary describing the profile's desktop condition.
    pub scenario_summary: String,
    /// Claimed surface families evaluated under this profile. Pulled from the matrix.
    pub evaluated_families: Vec<M5ShellSurfaceFamily>,
    /// Shell-zone-integrity posture.
    pub shell_zone_integrity: ShellZoneIntegrityState,
    /// Adaptive-layout continuity posture.
    pub adaptive_layout: AdaptiveLayoutState,
    /// Multi-window truth posture.
    pub multi_window_truth: MultiWindowTruthState,
    /// Owning-window routing posture.
    pub owning_window_routing: OwningWindowRoutingState,
    /// Downgrade triggers that apply to this profile.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed routing relocation is in force.
    pub active_waiver: Option<DesktopProfileWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: DesktopProfileStatus,
    /// The exact profile causes that narrowed or blocked this row.
    pub profile_causes: Vec<DesktopProfileCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl DesktopProfileRow {
    /// `true` when the row evaluated all ten claimed surface families — no claimed surface is
    /// left uncertified under this profile and none is invented.
    pub fn families_complete(&self) -> bool {
        let declared: BTreeSet<M5ShellSurfaceFamily> =
            self.evaluated_families.iter().copied().collect();
        let required: BTreeSet<M5ShellSurfaceFamily> = REQUIRED_FAMILIES.iter().copied().collect();
        declared == required && declared.len() == self.evaluated_families.len()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.families_complete() {
            return true;
        }
        if matches!(
            self.shell_zone_integrity,
            ShellZoneIntegrityState::PrivateSlotDriftDetected
        ) {
            return true;
        }
        if matches!(
            self.adaptive_layout,
            AdaptiveLayoutState::IdentityLostOrUnusablePane
        ) {
            return true;
        }
        if matches!(
            self.multi_window_truth,
            MultiWindowTruthState::WorkspaceTruthDivergedAcrossWindows
        ) {
            return true;
        }
        if matches!(
            self.owning_window_routing,
            OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.shell_zone_integrity.is_disclosed_narrowing()
            || self.adaptive_layout.is_disclosed_narrowing()
            || self.multi_window_truth.is_disclosed_narrowing()
            || self.owning_window_routing.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the profile posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing
    /// forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> DesktopProfileStatus {
        if self.has_hard_blocker() {
            DesktopProfileStatus::Red
        } else if self.has_narrowing() {
            DesktopProfileStatus::Yellow
        } else {
            DesktopProfileStatus::Green
        }
    }

    /// Recomputes the exact profile causes for the row, in deterministic order
    /// (shell-zone, adaptive-layout, multi-window, owning-window).
    pub fn recompute_causes(&self) -> Vec<DesktopProfileCause> {
        let mut causes = Vec::new();
        match self.shell_zone_integrity {
            ShellZoneIntegrityState::AllSurfacesInDeclaredSlots => {}
            ShellZoneIntegrityState::DisclosedSlotFallbackNarrowing => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A claimed surface falls back to its declared fallback shell slot \
                             because a dependency is unavailable under this profile; the fallback \
                             slot is still a declared shell slot and the narrowing is disclosed."
                        .to_owned(),
                });
            }
            ShellZoneIntegrityState::PrivateSlotDriftDetected => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::SlotUndeclared,
                    disclosed: false,
                    detail: "A claimed surface attached outside any declared shell slot under this \
                             profile, inventing a private slot instead of using its declared \
                             canonical or fallback slot."
                        .to_owned(),
                });
            }
        }
        match self.adaptive_layout {
            AdaptiveLayoutState::IdentityStableNoUnusablePane => {}
            AdaptiveLayoutState::DisclosedCollapseNarrowing => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "Responsive collapse takes a disclosed docked→sheet/overflow narrowing \
                             under this profile while preserving the task identity and the reopen \
                             path, so the claim is narrowed and disclosed."
                        .to_owned(),
                });
            }
            AdaptiveLayoutState::IdentityLostOrUnusablePane => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::CollapseChangedTaskIdentity,
                    disclosed: false,
                    detail: "Responsive collapse changed the task identity, hid critical state \
                             instead of overflowing it, or forced an unusable narrow pane under \
                             this profile."
                        .to_owned(),
                });
            }
        }
        match self.multi_window_truth {
            MultiWindowTruthState::AllTruthsPreservedLayoutLocal => {}
            MultiWindowTruthState::DisclosedTruthProjectionNarrowing => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A workspace-global truth is projected in a disclosed reduced form \
                             until a dependency is restored under this profile, while staying \
                             visible in every window, so the claim is narrowed and disclosed."
                        .to_owned(),
                });
            }
            MultiWindowTruthState::WorkspaceTruthDivergedAcrossWindows => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::WorkspaceTruthDivergedAcrossWindows,
                    disclosed: false,
                    detail: "A workspace-global trust, remote, profile, or recovery truth diverged \
                             across windows under this profile instead of staying global while \
                             layout stayed local."
                        .to_owned(),
                });
            }
        }
        match self.owning_window_routing {
            OwningWindowRoutingState::RoutesToOwningObjectNoFocusTheft => {}
            OwningWindowRoutingState::DisclosedRoutingRelocation => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A routed action from a closed window is relocated to a disclosed, \
                             waivered still-visible prompt in the primary workspace window under \
                             this profile rather than blocking outright, so nothing is orphaned."
                        .to_owned(),
                });
            }
            OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan => {
                causes.push(DesktopProfileCause {
                    profile: self.profile,
                    trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                    disclosed: false,
                    detail: "A routed dialog, notification, or approval was lost to focus theft, \
                             orphaning, or a wrong-window reopen under this profile instead of \
                             returning to the owning window and object."
                        .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed routing relocation may only stay yellow (rather than red) when a waiver
    /// discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.owning_window_routing,
            OwningWindowRoutingState::DisclosedRoutingRelocation
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<DesktopProfileFinding> {
        let mut findings = Vec::new();
        let profile = self.profile.as_str().to_owned();

        if !self.families_complete() {
            findings.push(DesktopProfileFinding::EvaluatedFamiliesIncomplete {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.shell_zone_integrity,
            ShellZoneIntegrityState::PrivateSlotDriftDetected
        ) {
            findings.push(DesktopProfileFinding::PrivateSlotDriftDetected {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.adaptive_layout,
            AdaptiveLayoutState::IdentityLostOrUnusablePane
        ) {
            findings.push(DesktopProfileFinding::AdaptiveIdentityLostOrUnusablePane {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.multi_window_truth,
            MultiWindowTruthState::WorkspaceTruthDivergedAcrossWindows
        ) {
            findings.push(DesktopProfileFinding::WorkspaceTruthDivergedAcrossWindows {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.owning_window_routing,
            OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan
        ) {
            findings.push(DesktopProfileFinding::OwningWindowRoutingLost {
                profile: profile.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, DesktopProfileStatus::Green) && !self.has_reason() {
            findings.push(DesktopProfileFinding::NarrowedRowWithoutReason {
                profile: profile.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(DesktopProfileFinding::NarrowedRowWithoutWaiver {
                profile: profile.clone(),
            });
        }
        // An attached waiver must still be active and must point at this profile.
        if let Some(waiver) = &self.active_waiver {
            if waiver.profile != self.profile {
                findings.push(DesktopProfileFinding::WaiverProfileMismatch {
                    profile: profile.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(DesktopProfileFinding::WaiverExpired {
                    profile: profile.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(DesktopProfileFinding::RowStatusStale {
                profile: profile.clone(),
            });
        }
        if self.profile_causes != self.recompute_causes() {
            findings.push(DesktopProfileFinding::RowCausesStale { profile });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} zone={} layout={} multiwindow={} routing={} families={} waiver={}",
            self.profile.as_str(),
            self.derived_status.as_str(),
            self.shell_zone_integrity.as_str(),
            self.adaptive_layout.as_str(),
            self.multi_window_truth.as_str(),
            self.owning_window_routing.as_str(),
            self.evaluated_families.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the desktop-profile certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum DesktopProfileFinding {
    /// A claimed desktop profile has no certification row.
    ProfileMissing {
        /// The missing profile token.
        profile: String,
    },
    /// A row did not evaluate all ten claimed surface families.
    EvaluatedFamiliesIncomplete {
        /// The profile token.
        profile: String,
    },
    /// A claimed surface invented a private shell slot under the profile.
    PrivateSlotDriftDetected {
        /// The profile token.
        profile: String,
    },
    /// Responsive collapse changed identity or forced an unusable pane under the profile.
    AdaptiveIdentityLostOrUnusablePane {
        /// The profile token.
        profile: String,
    },
    /// A workspace-global truth diverged across windows under the profile.
    WorkspaceTruthDivergedAcrossWindows {
        /// The profile token.
        profile: String,
    },
    /// A routed action was lost to focus theft, orphaning, or a wrong-window reopen.
    OwningWindowRoutingLost {
        /// The profile token.
        profile: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The profile token.
        profile: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The profile token.
        profile: String,
    },
    /// An attached waiver does not point at the row's profile.
    WaiverProfileMismatch {
        /// The profile token.
        profile: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The profile token.
        profile: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The profile token.
        profile: String,
    },
    /// The declared profile causes do not match the recomputed causes.
    RowCausesStale {
        /// The profile token.
        profile: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered profiles do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl DesktopProfileFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ProfileMissing { .. } => "profile_missing",
            Self::EvaluatedFamiliesIncomplete { .. } => "evaluated_families_incomplete",
            Self::PrivateSlotDriftDetected { .. } => "private_slot_drift_detected",
            Self::AdaptiveIdentityLostOrUnusablePane { .. } => {
                "adaptive_identity_lost_or_unusable_pane"
            }
            Self::WorkspaceTruthDivergedAcrossWindows { .. } => {
                "workspace_truth_diverged_across_windows"
            }
            Self::OwningWindowRoutingLost { .. } => "owning_window_routing_lost",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverProfileMismatch { .. } => "waiver_profile_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ProfileMissing { profile }
            | Self::EvaluatedFamiliesIncomplete { profile }
            | Self::PrivateSlotDriftDetected { profile }
            | Self::AdaptiveIdentityLostOrUnusablePane { profile }
            | Self::WorkspaceTruthDivergedAcrossWindows { profile }
            | Self::OwningWindowRoutingLost { profile }
            | Self::NarrowedRowWithoutReason { profile }
            | Self::NarrowedRowWithoutWaiver { profile }
            | Self::WaiverProfileMismatch { profile, .. }
            | Self::WaiverExpired { profile, .. }
            | Self::RowStatusStale { profile }
            | Self::RowCausesStale { profile } => profile,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release desktop-profile-certification packet shared by the shell / windowing / layout /
/// status automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfilePacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen shell-zone matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen shell-zone matrix schema.
    pub matrix_schema_ref: String,
    /// Shell-zoning contract this proof mirrors for shell-zone integrity.
    pub shell_zoning_contract_ref: String,
    /// Reference-layout contract this proof mirrors for adaptive-layout continuity.
    pub reference_layout_contract_ref: String,
    /// Window-topology contract this proof mirrors for multi-window truth parity.
    pub window_topology_contract_ref: String,
    /// Attention-routing contract this proof mirrors for owning-window routing.
    pub attention_routing_contract_ref: String,
    /// Session-restore fidelity contract this proof mirrors for dependency-missing restore.
    pub session_restore_contract_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four continuity truth dimensions every profile row certifies.
    pub required_truth_dimensions: Vec<String>,
    /// The six claimed desktop profiles the certification must cover.
    pub required_profiles: Vec<String>,
    /// The ten claimed surface families every profile row must evaluate.
    pub required_families: Vec<String>,
    /// Per-profile certification rows, in canonical order.
    pub rows: Vec<DesktopProfileRow>,
    /// Desktop profiles certified, in canonical (sorted) order.
    pub covered_profiles: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-continuity) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<DesktopProfileWaiver>,
    /// Every exact profile cause, in row then cause order.
    pub profile_causes: Vec<DesktopProfileCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<DesktopProfileFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed
    /// surfaces.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl DesktopProfilePacket {
    /// Returns the certification row for `profile`, if present.
    pub fn row(&self, profile: M5DesktopProfile) -> Option<&DesktopProfileRow> {
        self.rows.iter().find(|row| row.profile == profile)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.profile.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.profile_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.profile.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the shell automation consumes.
    pub fn dashboard(&self) -> DesktopProfileDashboard {
        DesktopProfileDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 desktop-profile-certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per profile naming its
    /// status, the four truth-dimension postures, the evaluated-family count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "profile,status,shell_zone_integrity,adaptive_layout,multi_window_truth,owning_window_routing,evaluated_families,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.profile.as_str(),
                row.derived_status.as_str(),
                row.shell_zone_integrity.as_str(),
                row.adaptive_layout.as_str(),
                row.multi_window_truth.as_str(),
                row.owning_window_routing.as_str(),
                row.evaluated_families.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 desktop-profile certification: shell-zone, adaptive-layout, multi-window, and owning-window routing truth on every claimed desktop profile\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_desktop_profile_certification`](../../crates/aureline-shell/src/m5_desktop_profile_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification -- markdown > \\\n  artifacts/shell/m5-desktop-profile-certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!("- Source schema ref: `{}`\n", self.source_schema_ref));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!("- Release channel: `{}`\n", self.release_channel_class));
        out.push_str(&format!(
            "- Required truth dimensions: {}\n",
            self.required_truth_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Required profiles: {}\n",
            self.required_profiles
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!("- Green (full continuity): {}\n", self.green_row_count));
        out.push_str(&format!("- Yellow (auto-narrowed): {}\n", self.yellow_row_count));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean { "clean" } else { "blocked" }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Profile | Status | Shell-zone integrity | Adaptive layout | Multi-window truth | Owning-window routing | Waiver |\n\
             | ------- | ------ | -------------------- | --------------- | ------------------ | --------------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.profile_label,
                row.derived_status.as_str(),
                row.shell_zone_integrity.as_str(),
                row.adaptive_layout.as_str(),
                row.multi_window_truth.as_str(),
                row.owning_window_routing.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&DesktopProfileRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, DesktopProfileStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed desktop profile keeps shell-zone integrity, adaptive-layout identity, multi-window truth, and owning-window routing across every claimed surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.profile.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact profile causes\n\n");
        if self.profile_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.profile_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.profile.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.profile.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_desktop_profile_certification_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfileDashboardRow {
    /// The desktop profile.
    pub profile: M5DesktopProfile,
    /// Short profile label.
    pub profile_label: String,
    /// Derived green/yellow/red status.
    pub status: DesktopProfileStatus,
    /// Number of claimed surface families evaluated under this profile.
    pub evaluated_family_count: usize,
    /// Shell-zone-integrity posture.
    pub shell_zone_integrity: ShellZoneIntegrityState,
    /// Adaptive-layout continuity posture.
    pub adaptive_layout: AdaptiveLayoutState,
    /// Multi-window truth posture.
    pub multi_window_truth: MultiWindowTruthState,
    /// Owning-window routing posture.
    pub owning_window_routing: OwningWindowRoutingState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / windowing / layout / status automation reads
/// to auto-narrow claimed surfaces on the affected desktop profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfileDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<DesktopProfileDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Shell / release automation refs that consume the dashboard.
    pub shell_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl DesktopProfileDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &DesktopProfilePacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| DesktopProfileDashboardRow {
                profile: row.profile,
                profile_label: row.profile_label.clone(),
                status: row.derived_status,
                evaluated_family_count: row.evaluated_families.len(),
                shell_zone_integrity: row.shell_zone_integrity,
                adaptive_layout: row.adaptive_layout,
                multi_window_truth: row.multi_window_truth,
                owning_window_routing: row.owning_window_routing,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .profile_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_DESKTOP_PROFILE_CERTIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_DESKTOP_PROFILE_CERTIFICATION_SCHEMA_VERSION,
            dashboard_id: M5_DESKTOP_PROFILE_CERTIFICATION_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            shell_automation_refs: packet.shell_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 desktop-profile-certification dashboard serializes")
    }
}

/// Support-export wrapper for the desktop-profile-certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopProfileSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: DesktopProfilePacket,
    /// Dashboard quoted in full.
    pub dashboard: DesktopProfileDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl DesktopProfileSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each profile, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the shell automation — can
    /// name the same profile and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: DesktopProfilePacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.profile.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_DESKTOP_PROFILE_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_DESKTOP_PROFILE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_DESKTOP_PROFILE_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_desktop_profile_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopProfileInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-profile certification rows.
    pub rows: Vec<DesktopProfileRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw
/// URLs, credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`DesktopProfilePacket`] from the exact build identity, the frozen matrix ref, and
/// the per-profile certification rows.
///
/// Each row's derived status and profile causes, the aggregate counts, the active waivers, and
/// the blocking findings are recomputed here so the packet is the single source of truth and
/// the auto-narrowing cannot be asserted.
pub fn build_m5_desktop_profile_certification_packet(
    input: DesktopProfileInput,
) -> DesktopProfilePacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<DesktopProfileRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.profile_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<DesktopProfileFinding> = Vec::new();

    // Every claimed profile must carry a certification row.
    let present: BTreeSet<M5DesktopProfile> = rows.iter().map(|row| row.profile).collect();
    for profile in REQUIRED_PROFILES {
        if !present.contains(&profile) {
            blocking_findings.push(DesktopProfileFinding::ProfileMissing {
                profile: profile.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_profiles: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|profile| profile.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, DesktopProfileStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, DesktopProfileStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, DesktopProfileStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(DesktopProfileFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<DesktopProfileWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let profile_causes: Vec<DesktopProfileCause> = rows
        .iter()
        .flat_map(|row| row.profile_causes.clone())
        .collect();

    let required_truth_dimensions: Vec<String> = REQUIRED_TRUTH_DIMENSIONS
        .iter()
        .map(|dim| dim.as_str().to_owned())
        .collect();
    let required_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    let required_families: Vec<String> = REQUIRED_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = DesktopProfilePacket {
        record_kind: M5_DESKTOP_PROFILE_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_DESKTOP_PROFILE_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_DESKTOP_PROFILE_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_DESKTOP_PROFILE_CERTIFICATION_PACKET_ID.to_owned(),
        source_schema_ref: M5_DESKTOP_PROFILE_CERTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Shell-zone, adaptive-layout, multi-window, and owning-window routing truth on \
                   every claimed M5 desktop profile: compact, standard, and expanded widths, \
                   mixed-DPI, multi-monitor, and dependency-missing restore each certified so \
                   every claimed surface stays in a declared shell slot, responsive collapse never \
                   changes task identity or forces an unusable narrow pane, every window preserves \
                   workspace-global trust, remote, profile, and recovery truth while layout stays \
                   local, and routed dialogs, notifications, and approvals return to the owning \
                   window and object without focus theft, orphaning, or a wrong-window reopen, with \
                   each profile's green/yellow/red claim auto-narrowed from its shell-zone-integrity, \
                   adaptive-layout, multi-window-truth, and owning-window-routing posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_DESKTOP_PROFILE_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        shell_zoning_contract_ref: M5_DESKTOP_PROFILE_CERTIFICATION_SHELL_ZONING_CONTRACT_REF
            .to_owned(),
        reference_layout_contract_ref:
            M5_DESKTOP_PROFILE_CERTIFICATION_REFERENCE_LAYOUT_CONTRACT_REF.to_owned(),
        window_topology_contract_ref:
            M5_DESKTOP_PROFILE_CERTIFICATION_WINDOW_TOPOLOGY_CONTRACT_REF.to_owned(),
        attention_routing_contract_ref:
            M5_DESKTOP_PROFILE_CERTIFICATION_ATTENTION_ROUTING_CONTRACT_REF.to_owned(),
        session_restore_contract_ref:
            M5_DESKTOP_PROFILE_CERTIFICATION_SESSION_RESTORE_CONTRACT_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_truth_dimensions,
        required_profiles,
        required_families,
        rows,
        covered_profiles,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        profile_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.desktop_profile_certification_registry".to_owned(),
            "release_automation.auto_narrow.desktop_profile_certification_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.desktop_profile_certification".to_owned(),
            M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-desktop-profile-certification".to_owned()],
        published_report_ref: M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_DESKTOP_PROFILE_CERTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(DesktopProfileFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_desktop_profile_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum DesktopProfileValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required truth dimensions do not match the lane constants.
    RequiredTruthDimensionsStale,
    /// The declared required profiles do not match the lane constants.
    RequiredProfilesStale,
    /// The declared required families do not match the lane constants.
    RequiredFamiliesStale,
    /// The rows do not cover all six claimed desktop profiles.
    CoverageIncomplete,
    /// The declared covered profiles do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared profile causes do not match the recomputed causes.
    ProfileCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the desktop-profile-certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every claimed desktop
/// profile carries a current certification row; each row's status is the derived auto-narrowed
/// value, never asserted; a green row cannot keep a claim while a claimed surface invents a
/// private shell slot, responsive collapse changes identity or forces an unusable narrow pane,
/// workspace-global truth diverges across windows, a routed action is lost to focus theft,
/// orphaning, or a wrong-window reopen, or the profile fails to evaluate every claimed surface
/// family; and a disclosed narrowing is backed by a reason and, where required, an active
/// waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_desktop_profile_certification_packet(
    packet: &DesktopProfilePacket,
) -> Result<(), Vec<DesktopProfileValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(DesktopProfileValidationError::NoRows);
    }
    if packet.record_kind != M5_DESKTOP_PROFILE_CERTIFICATION_PACKET_RECORD_KIND {
        errors.push(DesktopProfileValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_DESKTOP_PROFILE_CERTIFICATION_SCHEMA_VERSION {
        errors.push(DesktopProfileValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(DesktopProfileValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(DesktopProfileValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_TRUTH_DIMENSIONS
        .iter()
        .map(|dim| dim.as_str().to_owned())
        .collect();
    if packet.required_truth_dimensions != expected_dimensions {
        errors.push(DesktopProfileValidationError::RequiredTruthDimensionsStale);
    }
    let expected_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    if packet.required_profiles != expected_profiles {
        errors.push(DesktopProfileValidationError::RequiredProfilesStale);
    }
    let expected_families: Vec<String> = REQUIRED_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_families != expected_families {
        errors.push(DesktopProfileValidationError::RequiredFamiliesStale);
    }

    let present: BTreeSet<M5DesktopProfile> = packet.rows.iter().map(|row| row.profile).collect();
    let coverage_complete = REQUIRED_PROFILES
        .iter()
        .all(|profile| present.contains(profile));
    if !coverage_complete || packet.rows.len() != REQUIRED_PROFILES.len() {
        errors.push(DesktopProfileValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|profile| profile.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_profiles {
        errors.push(DesktopProfileValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), DesktopProfileStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), DesktopProfileStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), DesktopProfileStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(DesktopProfileValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<DesktopProfileWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(DesktopProfileValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<DesktopProfileCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.profile_causes {
        errors.push(DesktopProfileValidationError::ProfileCausesStale);
    }

    let mut recomputed: Vec<DesktopProfileFinding> = Vec::new();
    for profile in REQUIRED_PROFILES {
        if !present.contains(&profile) {
            recomputed.push(DesktopProfileFinding::ProfileMissing {
                profile: profile.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(DesktopProfileFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(DesktopProfileFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(DesktopProfileValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(DesktopProfileValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(DesktopProfileValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(DesktopProfileValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(DesktopProfileValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(DesktopProfileValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
