//! Canonical owning-window dialog/approval/notification routing certification for every
//! claimed M5 surface family.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface family —
//! notebook, data grid, profiler, pipeline, docs, preview, review, incident, companion,
//! and operator — to the window classes it may live in and the owning-window routing
//! expectations it must honor (route dialogs, notifications, and approvals back to the
//! owning window and object, preserve the exact object anchor on return, never steal
//! focus from an unrelated window, and never orphan a routed action on detach or close).
//! This lane is the routing-continuity capstone on top of that matrix: for every governed
//! family it certifies that **permission sheets, trust prompts, destructive
//! confirmations, publish/approval dialogs, and durable notification reopen paths bind to
//! the window that owns the authoritative object**, that a durable reopen **lands on the
//! exact object or a truthful placeholder rather than a generic home screen**, that a
//! routed action **never steals focus from a protected typing surface** (degrading to a
//! badge or activity-center row instead), and that **privacy-safe OS-notification
//! summaries preserve one exact reopen path without bypassing in-app review**.
//!
//! Three records carry the truth:
//!
//! - the per-family **routing row** ([`RoutingContinuityRow`]): one row per
//!   [`M5ShellSurfaceFamily`] naming the window classes and owning-window routing
//!   expectations it declares (pulled from the matrix), its per-window routed-action plan
//!   ([`RoutedActionWindowPlan`]) naming whether a routed action originating in each
//!   window binds to the owning object, preserves typing focus, and keeps a single reopen
//!   path, its dialog-binding / reopen-continuity / focus-retention / OS-notification
//!   privacy posture, any active waiver, and a derived green/yellow/red
//!   [`RoutingContinuityStatus`].
//! - the release **routing packet** ([`RoutingContinuityPacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers,
//!   the exact routing causes ([`RoutingContinuityCause`]), and the blocking findings the
//!   lane refuses to ship with.
//! - the **routing dashboard** ([`RoutingContinuityDashboard`]): a light projection the
//!   shell / windowing / notification / release automation reads to auto-narrow a claimed
//!   surface when its routing proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow`
//! the moment its frozen qualification is below Stable, a routed dialog or approval
//! relocates to a disclosed, waivered still-visible re-notification affordance when its
//! owning window is not present, a durable reopen lands on a disclosed truthful
//! placeholder that narrows context while preserving identity and the single reopen path,
//! a routed action defers to a disclosed badge or activity-center row instead of stealing
//! focus, or an OS-notification summary discloses a narrowed minimal projection; it drops
//! to `red` if a routed dialog or approval is lost to focus theft or orphaning, a durable
//! reopen lands on a generic shell losing the object identity and reopen path, a routed
//! action steals focus from a protected typing surface, an OS-notification leaks content
//! or bypasses in-app review, the row fails to declare all four owning-window routing
//! expectations, or a per-window plan drops owning-object binding, focus preservation, or
//! the single reopen path. That derivation is the auto-narrowing the acceptance criteria
//! require, and the routing-completeness and per-window plan checks are the lint that
//! prevents a later routing regression from shipping as stable.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, or credentials — only stable
//! ids, closed vocabulary, counts, refs, and short labels. The surface family, window
//! class, owning-window routing, qualification, downgrade-trigger, and consumer-surface
//! vocabulary is re-exported by reference from the already frozen [matrix]; the certified
//! rows are pulled straight from that matrix's seeded packet, so this lane mints no
//! parallel shell vocabulary and cannot certify a family the matrix does not freeze. Only
//! the routing-specific vocabulary ([`RoutingContinuityStatus`], [`DialogBindingState`],
//! [`ReopenContinuityState`], [`FocusRetentionState`], [`OsNotificationPrivacyState`],
//! [`RoutedActionWindowPlan`], [`RoutingContinuityWaiver`], [`RoutingContinuityCause`],
//! [`RoutingContinuityFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix as matrix;

pub use matrix::{
    M5ContinuityTruth, M5OwningWindowRouting, M5ShellConsumerSurface, M5ShellDowngradeTrigger,
    M5ShellQualificationClass, M5ShellSurfaceFamily, M5ShellZoneSlot, M5WindowClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_owning_window_routing_packet,
    seeded_m5_owning_window_routing_packet_datagrid_reopen_generic_shell_blocked,
    seeded_m5_owning_window_routing_packet_notebook_dialog_binding_lost_blocked,
    seeded_m5_owning_window_routing_packet_preview_focus_stolen_blocked,
    seeded_m5_owning_window_routing_packet_review_os_notification_leak_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_OWNING_WINDOW_ROUTING_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_OWNING_WINDOW_ROUTING_SHARED_CONTRACT_REF: &str = "shell:m5_owning_window_routing:v1";

/// Stable record kind for [`RoutingContinuityPacket`] payloads.
pub const M5_OWNING_WINDOW_ROUTING_PACKET_RECORD_KIND: &str =
    "shell_m5_owning_window_routing_packet_record";

/// Stable record kind for [`RoutingContinuityDashboard`] payloads.
pub const M5_OWNING_WINDOW_ROUTING_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_owning_window_routing_dashboard_record";

/// Stable record kind for [`RoutingContinuitySupportExport`] payloads.
pub const M5_OWNING_WINDOW_ROUTING_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_owning_window_routing_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_OWNING_WINDOW_ROUTING_PACKET_ID: &str = "m5-owning-window-routing:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_OWNING_WINDOW_ROUTING_DASHBOARD_ID: &str =
    "m5-owning-window-routing-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_OWNING_WINDOW_ROUTING_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-owning-window-routing:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_OWNING_WINDOW_ROUTING_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-owning-window-routing.schema.json";

/// Published markdown report ref reviewers reopen the routing proof from.
pub const M5_OWNING_WINDOW_ROUTING_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-owning-window-routing.md";

/// Published routing-packet artifact ref.
pub const M5_OWNING_WINDOW_ROUTING_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-owning-window-routing-proof/packet.json";

/// Published routing-dashboard artifact ref.
pub const M5_OWNING_WINDOW_ROUTING_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-owning-window-routing-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_OWNING_WINDOW_ROUTING_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-owning-window-routing-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_OWNING_WINDOW_ROUTING_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-owning-window-routing-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_OWNING_WINDOW_ROUTING_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_owning_window_routing_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_OWNING_WINDOW_ROUTING_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Window-topology contract this proof mirrors for detach/reopen routing.
pub const M5_OWNING_WINDOW_ROUTING_WINDOW_TOPOLOGY_CONTRACT_REF: &str =
    matrix::M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF;

/// Session-restore fidelity contract this proof mirrors for reopen continuity.
pub const M5_OWNING_WINDOW_ROUTING_SESSION_RESTORE_CONTRACT_REF: &str =
    matrix::M5_SHELL_SESSION_RESTORE_CONTRACT_REF;

/// Every governed surface family the routing proof must cover, in canonical order.
/// These are exactly the families the frozen shell-zone matrix freezes; the lane
/// certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// Every owning-window routing expectation a row must declare, in canonical order.
pub const REQUIRED_ROUTING_EXPECTATIONS: [M5OwningWindowRouting; 4] = M5OwningWindowRouting::ALL;

/// The derived owning-window routing-continuity light a governed surface family carries.
///
/// `green` means every dialog, approval, and durable notification binds to the window
/// that owns the authoritative object, a reopen lands on the exact object or a truthful
/// identity-preserving placeholder, a routed action never steals focus from a protected
/// typing surface, and OS-notification summaries stay privacy-safe while preserving one
/// exact reopen path. `yellow` is a disclosed narrowing (the family is honestly narrowed
/// below Stable, relocates a routed dialog/approval to a disclosed, waivered still-visible
/// affordance, reopens into a disclosed truthful placeholder, defers a routed action to a
/// disclosed badge or activity-center row, or discloses a narrowed minimal OS-notification
/// summary). `red` is blocked: a routed dialog/approval is lost to focus theft or
/// orphaning, a reopen lands on a generic shell, a routed action steals focus from a
/// protected typing surface, an OS notification leaks content or bypasses in-app review, a
/// routing expectation is undeclared, or a per-window plan drops owning-object binding,
/// focus preservation, or the single reopen path — and it may not keep a shell-maturity
/// claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutingContinuityStatus {
    /// Full standing: bound to owning object, exact reopen, no focus theft, privacy-safe.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl RoutingContinuityStatus {
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

/// How dialogs, approvals, trust prompts, and destructive confirmations bind to the
/// window that owns the authoritative object.
///
/// `bound_to_owning_window_object` means every routed dialog or approval returns to the
/// window that owns the authoritative object and preserves that object's anchor.
/// `disclosed_binding_relocation` means a routed dialog or approval relocates to a
/// disclosed, waivered still-visible re-notification affordance when its owning window is
/// not present — a yellow narrowing. `binding_lost_or_orphaned` means a routed dialog or
/// approval stole focus from an unrelated window or was orphaned on detach or close —
/// always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogBindingState {
    /// Routed dialogs/approvals bind to the owning window and authoritative object.
    BoundToOwningWindowObject,
    /// A routed dialog/approval relocates to a disclosed, waivered still-visible affordance.
    DisclosedBindingRelocation,
    /// A routed dialog/approval stole focus or was orphaned — a blocker.
    BindingLostOrOrphaned,
}

impl DialogBindingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundToOwningWindowObject => "bound_to_owning_window_object",
            Self::DisclosedBindingRelocation => "disclosed_binding_relocation",
            Self::BindingLostOrOrphaned => "binding_lost_or_orphaned",
        }
    }

    /// `true` when routed dialogs/approvals bind to the owning window/object at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::BoundToOwningWindowObject)
    }

    /// `true` when a routed dialog/approval took a disclosed relocation.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedBindingRelocation)
    }
}

/// How a durable notification reopen lands relative to the authoritative object.
///
/// `reopens_exact_object_or_truthful_placeholder` means a durable reopen always lands on
/// the exact owning object, or on a truthful identity-preserving placeholder when the
/// object is legitimately gone. `disclosed_placeholder_narrowing` means the reopen lands
/// on a truthful placeholder that discloses a narrowed context (a live sub-state that
/// could not be restored) while preserving the object identity and the single reopen path
/// — a yellow narrowing. `lands_on_generic_shell` means the reopen dropped to a generic
/// home/shell, losing the object identity and its reopen path — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenContinuityState {
    /// Reopen lands on the exact object or a truthful identity-preserving placeholder.
    ReopensExactObjectOrTruthfulPlaceholder,
    /// Reopen lands on a disclosed truthful placeholder that narrows context.
    DisclosedPlaceholderNarrowing,
    /// Reopen dropped to a generic shell, losing object identity and reopen path — a blocker.
    LandsOnGenericShell,
}

impl ReopenContinuityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopensExactObjectOrTruthfulPlaceholder => {
                "reopens_exact_object_or_truthful_placeholder"
            }
            Self::DisclosedPlaceholderNarrowing => "disclosed_placeholder_narrowing",
            Self::LandsOnGenericShell => "lands_on_generic_shell",
        }
    }

    /// `true` when a reopen always lands on the exact object or truthful placeholder.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ReopensExactObjectOrTruthfulPlaceholder)
    }

    /// `true` when a reopen took a disclosed truthful-placeholder narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPlaceholderNarrowing)
    }
}

/// How a routed action preserves focus on protected typing surfaces.
///
/// `no_focus_steal_on_typing` means a routed action never pulls focus away from an active
/// typing surface. `disclosed_deferral_to_badge_or_center` means a routed action defers to
/// a disclosed badge or activity-center row rather than stealing focus while a protected
/// typing path is active — a yellow narrowing. `focus_stolen_from_typing` means a routed
/// action pulled focus away from a protected typing surface — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusRetentionState {
    /// A routed action never steals focus from a protected typing surface.
    NoFocusStealOnTyping,
    /// A routed action defers to a disclosed badge or activity-center row.
    DisclosedDeferralToBadgeOrCenter,
    /// A routed action stole focus from a protected typing surface — a blocker.
    FocusStolenFromTyping,
}

impl FocusRetentionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFocusStealOnTyping => "no_focus_steal_on_typing",
            Self::DisclosedDeferralToBadgeOrCenter => "disclosed_deferral_to_badge_or_center",
            Self::FocusStolenFromTyping => "focus_stolen_from_typing",
        }
    }

    /// `true` when a routed action never steals focus at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::NoFocusStealOnTyping)
    }

    /// `true` when a routed action took a disclosed badge/center deferral.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedDeferralToBadgeOrCenter)
    }
}

/// How a privacy-safe OS notification summary preserves the in-app reopen path.
///
/// `privacy_safe_summary_preserves_reopen` means the OS-notification summary is
/// privacy-safe (no sensitive content) and still routes to one exact in-app reopen path
/// without bypassing in-app review. `disclosed_minimal_summary` means the OS notification
/// discloses a narrowed minimal summary (an even more redacted projection) while still
/// preserving the single reopen path — a yellow narrowing. `leaks_content_or_bypasses_review`
/// means the OS notification leaked sensitive content or bypassed in-app review — always a
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsNotificationPrivacyState {
    /// The OS-notification summary is privacy-safe and preserves one exact reopen path.
    PrivacySafeSummaryPreservesReopen,
    /// The OS notification discloses a narrowed minimal summary.
    DisclosedMinimalSummary,
    /// The OS notification leaked content or bypassed in-app review — a blocker.
    LeaksContentOrBypassesReview,
}

impl OsNotificationPrivacyState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrivacySafeSummaryPreservesReopen => "privacy_safe_summary_preserves_reopen",
            Self::DisclosedMinimalSummary => "disclosed_minimal_summary",
            Self::LeaksContentOrBypassesReview => "leaks_content_or_bypasses_review",
        }
    }

    /// `true` when the OS-notification summary is privacy-safe and preserves reopen.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::PrivacySafeSummaryPreservesReopen)
    }

    /// `true` when the OS notification took a disclosed minimal-summary narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedMinimalSummary)
    }
}

/// The routed-action plan a family lands in for one window class.
///
/// The plan must bind a routed action originating in this window to the owning object,
/// preserve typing focus, and keep the single reopen path; a lost binding, a lost focus
/// preservation, or a lost single reopen path is a blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutedActionWindowPlan {
    /// The window class this plan describes.
    pub window_class: M5WindowClass,
    /// `true` when a routed action here binds to the owning window and authoritative object.
    pub binds_to_owning_object: bool,
    /// `true` when a routed action here never steals focus from a protected typing surface.
    pub preserves_typing_focus: bool,
    /// `true` when a routed action here keeps a single exact reopen path.
    pub keeps_single_reopen_path: bool,
}

impl RoutedActionWindowPlan {
    /// `true` when the plan binds to the owning object, preserves focus, and keeps reopen.
    pub const fn is_fully_routed(&self) -> bool {
        self.binds_to_owning_object && self.preserves_typing_focus && self.keeps_single_reopen_path
    }
}

/// Short, reviewer-facing label for a governed family's guarded surface.
pub const fn surface_label(family: M5ShellSurfaceFamily) -> &'static str {
    match family {
        M5ShellSurfaceFamily::Notebook => "Notebook editor / cell surface",
        M5ShellSurfaceFamily::DataGrid => "Tabular data grid surface",
        M5ShellSurfaceFamily::Profiler => "Profiler / performance surface",
        M5ShellSurfaceFamily::Pipeline => "Pipeline / workflow graph surface",
        M5ShellSurfaceFamily::Docs => "Documentation reader surface",
        M5ShellSurfaceFamily::Preview => "Preview surface (render, diff, media)",
        M5ShellSurfaceFamily::Review => "Review / change-request surface",
        M5ShellSurfaceFamily::Incident => "Incident / operations-response surface",
        M5ShellSurfaceFamily::Companion => "Companion assistant surface",
        M5ShellSurfaceFamily::Operator => "Operator / control-plane surface",
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed
/// (yellow) rather than blocked — never lets a lost binding, generic-shell reopen, focus
/// theft, or content leak hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed family the waiver applies to.
    pub family: M5ShellSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl RoutingContinuityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's routing claim.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuityCause {
    /// The governed family the cause applies to.
    pub family: M5ShellSurfaceFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl RoutingContinuityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed surface family, certified across its dialog-binding, reopen-continuity,
/// focus-retention, and OS-notification-privacy posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuityRow {
    /// The governed family being certified.
    pub family: M5ShellSurfaceFamily,
    /// The family's frozen qualification class from the shell-zone matrix.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Short guarded-surface label.
    pub surface_label: String,
    /// Canonical shell slot from the matrix.
    pub canonical_slot: M5ShellZoneSlot,
    /// Declared fallback slot from the matrix.
    pub fallback_slot: M5ShellZoneSlot,
    /// Window classes this family may live in. Pulled from the matrix.
    pub declared_window_classes: Vec<M5WindowClass>,
    /// Owning-window routing expectations this family honors. Pulled from the matrix.
    pub declared_owning_window_routing: Vec<M5OwningWindowRouting>,
    /// Per-window routed-action plan, one per declared window class.
    pub window_plans: Vec<RoutedActionWindowPlan>,
    /// Dialog/approval binding posture.
    pub dialog_binding: DialogBindingState,
    /// Durable-reopen continuity posture.
    pub reopen_continuity: ReopenContinuityState,
    /// Focus-retention posture on protected typing paths.
    pub focus_retention: FocusRetentionState,
    /// OS-notification-privacy posture.
    pub os_notification_privacy: OsNotificationPrivacyState,
    /// Consumer surfaces this family must stay aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed dialog-binding relocation narrowing is in force.
    pub active_waiver: Option<RoutingContinuityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: RoutingContinuityStatus,
    /// The exact routing causes that narrowed or blocked this row.
    pub routing_causes: Vec<RoutingContinuityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl RoutingContinuityRow {
    /// `true` when this family's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row declares all four owning-window routing expectations.
    pub fn routing_expectations_complete(&self) -> bool {
        let declared: BTreeSet<M5OwningWindowRouting> = self
            .declared_owning_window_routing
            .iter()
            .copied()
            .collect();
        let required: BTreeSet<M5OwningWindowRouting> =
            REQUIRED_ROUTING_EXPECTATIONS.iter().copied().collect();
        declared == required && declared.len() == self.declared_owning_window_routing.len()
    }

    /// `true` when the per-window plans cover exactly the declared window classes — no
    /// window class the family may live in is left uncertified and none is invented.
    pub fn plans_cover_declared_window_classes(&self) -> bool {
        let declared: BTreeSet<M5WindowClass> =
            self.declared_window_classes.iter().copied().collect();
        let present: BTreeSet<M5WindowClass> = self
            .window_plans
            .iter()
            .map(|plan| plan.window_class)
            .collect();
        declared == present && present.len() == self.window_plans.len()
    }

    /// `true` when every per-window plan binds a routed action to the owning object.
    pub fn plans_bind_to_owning(&self) -> bool {
        self.window_plans
            .iter()
            .all(|plan| plan.binds_to_owning_object)
    }

    /// `true` when every per-window plan preserves typing focus.
    pub fn plans_preserve_focus(&self) -> bool {
        self.window_plans
            .iter()
            .all(|plan| plan.preserves_typing_focus)
    }

    /// `true` when every per-window plan keeps a single exact reopen path.
    pub fn plans_keep_single_reopen(&self) -> bool {
        self.window_plans
            .iter()
            .all(|plan| plan.keeps_single_reopen_path)
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.routing_expectations_complete() {
            return true;
        }
        if matches!(
            self.dialog_binding,
            DialogBindingState::BindingLostOrOrphaned
        ) {
            return true;
        }
        if matches!(
            self.reopen_continuity,
            ReopenContinuityState::LandsOnGenericShell
        ) {
            return true;
        }
        if matches!(
            self.focus_retention,
            FocusRetentionState::FocusStolenFromTyping
        ) {
            return true;
        }
        if matches!(
            self.os_notification_privacy,
            OsNotificationPrivacyState::LeaksContentOrBypassesReview
        ) {
            return true;
        }
        if !self.plans_cover_declared_window_classes()
            || !self.plans_bind_to_owning()
            || !self.plans_preserve_focus()
            || !self.plans_keep_single_reopen()
        {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || self.dialog_binding.is_disclosed_narrowing()
            || self.reopen_continuity.is_disclosed_narrowing()
            || self.focus_retention.is_disclosed_narrowing()
            || self.os_notification_privacy.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the routing posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> RoutingContinuityStatus {
        if self.has_hard_blocker() {
            RoutingContinuityStatus::Red
        } else if self.has_narrowing() {
            RoutingContinuityStatus::Yellow
        } else {
            RoutingContinuityStatus::Green
        }
    }

    /// Recomputes the exact routing causes for the row, in deterministic order
    /// (qualification, routing completeness, dialog binding, reopen, focus, OS notification).
    pub fn recompute_causes(&self) -> Vec<RoutingContinuityCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(RoutingContinuityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen shell-zone matrix qualifies this family at `{}`, below a Stable shell claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        if !self.routing_expectations_complete() {
            causes.push(RoutingContinuityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                disclosed: false,
                detail: "The row does not declare all four owning-window routing expectations \
                         (route-to-owning, preserve-anchor, no-focus-theft, no-orphan)."
                    .to_owned(),
            });
        }
        match self.dialog_binding {
            DialogBindingState::BoundToOwningWindowObject => {}
            DialogBindingState::DisclosedBindingRelocation => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A routed dialog or approval relocates to a disclosed, waivered \
                             still-visible re-notification affordance when its owning window is not \
                             present, and re-establishes the owning-window route when it returns."
                        .to_owned(),
                });
            }
            DialogBindingState::BindingLostOrOrphaned => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                    disclosed: false,
                    detail: "A routed dialog or approval stole focus from an unrelated window or \
                             was orphaned on detach or close instead of binding to the owning \
                             window and object."
                        .to_owned(),
                });
            }
        }
        match self.reopen_continuity {
            ReopenContinuityState::ReopensExactObjectOrTruthfulPlaceholder => {}
            ReopenContinuityState::DisclosedPlaceholderNarrowing => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A durable reopen lands on a truthful placeholder that discloses a \
                             narrowed context (a live sub-state that could not be restored) while \
                             preserving the object identity and the single reopen path."
                        .to_owned(),
                });
            }
            ReopenContinuityState::LandsOnGenericShell => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
                    disclosed: false,
                    detail: "A durable reopen dropped to a generic home or shell, losing the \
                             owning-object identity and its exact reopen path."
                        .to_owned(),
                });
            }
        }
        match self.focus_retention {
            FocusRetentionState::NoFocusStealOnTyping => {}
            FocusRetentionState::DisclosedDeferralToBadgeOrCenter => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A routed action defers to a disclosed badge or activity-center row \
                             rather than stealing focus while a protected typing path is active."
                        .to_owned(),
                });
            }
            FocusRetentionState::FocusStolenFromTyping => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                    disclosed: false,
                    detail: "A routed action pulled focus away from a protected typing surface \
                             instead of deferring to a badge or activity-center row."
                        .to_owned(),
                });
            }
        }
        match self.os_notification_privacy {
            OsNotificationPrivacyState::PrivacySafeSummaryPreservesReopen => {}
            OsNotificationPrivacyState::DisclosedMinimalSummary => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The OS-notification summary discloses a narrowed minimal projection \
                             (an even more redacted summary) while still preserving the single \
                             exact in-app reopen path."
                        .to_owned(),
                });
            }
            OsNotificationPrivacyState::LeaksContentOrBypassesReview => {
                causes.push(RoutingContinuityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::PolicyBlocked,
                    disclosed: false,
                    detail: "The OS notification leaked sensitive content into its summary or \
                             bypassed in-app review of the routed action."
                        .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed dialog-binding relocation may only stay yellow (rather than red) when a
    /// waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.dialog_binding,
            DialogBindingState::DisclosedBindingRelocation
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<RoutingContinuityFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if !self.routing_expectations_complete() {
            findings.push(RoutingContinuityFinding::RoutingExpectationsIncomplete {
                family: family.clone(),
            });
        }
        if matches!(
            self.dialog_binding,
            DialogBindingState::BindingLostOrOrphaned
        ) {
            findings.push(RoutingContinuityFinding::DialogBindingLostOrOrphaned {
                family: family.clone(),
            });
        }
        if matches!(
            self.reopen_continuity,
            ReopenContinuityState::LandsOnGenericShell
        ) {
            findings.push(RoutingContinuityFinding::ReopenLandsOnGenericShell {
                family: family.clone(),
            });
        }
        if matches!(
            self.focus_retention,
            FocusRetentionState::FocusStolenFromTyping
        ) {
            findings.push(RoutingContinuityFinding::FocusStolenFromTyping {
                family: family.clone(),
            });
        }
        if matches!(
            self.os_notification_privacy,
            OsNotificationPrivacyState::LeaksContentOrBypassesReview
        ) {
            findings.push(
                RoutingContinuityFinding::OsNotificationLeaksOrBypassesReview {
                    family: family.clone(),
                },
            );
        }
        if !self.plans_cover_declared_window_classes() {
            findings.push(RoutingContinuityFinding::PlanWindowCoverageMismatch {
                family: family.clone(),
            });
        }
        if !self.plans_bind_to_owning() {
            findings.push(RoutingContinuityFinding::PlanBindingNotPreserved {
                family: family.clone(),
            });
        }
        if !self.plans_preserve_focus() {
            findings.push(RoutingContinuityFinding::PlanFocusNotPreserved {
                family: family.clone(),
            });
        }
        if !self.plans_keep_single_reopen() {
            findings.push(RoutingContinuityFinding::PlanReopenPathNotPreserved {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, RoutingContinuityStatus::Green) && !self.has_reason() {
            findings.push(RoutingContinuityFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(RoutingContinuityFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(RoutingContinuityFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(RoutingContinuityFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(RoutingContinuityFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.routing_causes != self.recompute_causes() {
            findings.push(RoutingContinuityFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} dialog={} reopen={} focus={} os_notify={} windows={} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.dialog_binding.as_str(),
            self.reopen_continuity.as_str(),
            self.focus_retention.as_str(),
            self.os_notification_privacy.as_str(),
            self.window_plans.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the owning-window routing proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum RoutingContinuityFinding {
    /// A governed surface family has no routing row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row does not declare all four owning-window routing expectations.
    RoutingExpectationsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row lost a routed dialog or approval to focus theft or orphaning.
    DialogBindingLostOrOrphaned {
        /// The family token.
        family: String,
    },
    /// A row's durable reopen landed on a generic shell.
    ReopenLandsOnGenericShell {
        /// The family token.
        family: String,
    },
    /// A row's routed action stole focus from a protected typing surface.
    FocusStolenFromTyping {
        /// The family token.
        family: String,
    },
    /// A row's OS notification leaked content or bypassed in-app review.
    OsNotificationLeaksOrBypassesReview {
        /// The family token.
        family: String,
    },
    /// A row's per-window plans do not cover exactly the declared window classes.
    PlanWindowCoverageMismatch {
        /// The family token.
        family: String,
    },
    /// A row's per-window plan loses owning-object binding.
    PlanBindingNotPreserved {
        /// The family token.
        family: String,
    },
    /// A row's per-window plan loses focus preservation.
    PlanFocusNotPreserved {
        /// The family token.
        family: String,
    },
    /// A row's per-window plan loses the single reopen path.
    PlanReopenPathNotPreserved {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared routing causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl RoutingContinuityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::RoutingExpectationsIncomplete { .. } => "routing_expectations_incomplete",
            Self::DialogBindingLostOrOrphaned { .. } => "dialog_binding_lost_or_orphaned",
            Self::ReopenLandsOnGenericShell { .. } => "reopen_lands_on_generic_shell",
            Self::FocusStolenFromTyping { .. } => "focus_stolen_from_typing",
            Self::OsNotificationLeaksOrBypassesReview { .. } => {
                "os_notification_leaks_or_bypasses_review"
            }
            Self::PlanWindowCoverageMismatch { .. } => "plan_window_coverage_mismatch",
            Self::PlanBindingNotPreserved { .. } => "plan_binding_not_preserved",
            Self::PlanFocusNotPreserved { .. } => "plan_focus_not_preserved",
            Self::PlanReopenPathNotPreserved { .. } => "plan_reopen_path_not_preserved",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
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
            Self::FamilyMissing { family }
            | Self::RoutingExpectationsIncomplete { family }
            | Self::DialogBindingLostOrOrphaned { family }
            | Self::ReopenLandsOnGenericShell { family }
            | Self::FocusStolenFromTyping { family }
            | Self::OsNotificationLeaksOrBypassesReview { family }
            | Self::PlanWindowCoverageMismatch { family }
            | Self::PlanBindingNotPreserved { family }
            | Self::PlanFocusNotPreserved { family }
            | Self::PlanReopenPathNotPreserved { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release owning-window routing packet shared by the shell / windowing /
/// notification / release automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuityPacket {
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
    /// Window-topology contract this proof mirrors.
    pub window_topology_contract_ref: String,
    /// Session-restore fidelity contract this proof mirrors.
    pub session_restore_contract_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four owning-window routing expectations every row must declare.
    pub required_routing_expectations: Vec<String>,
    /// Per-family routing rows, in canonical order.
    pub rows: Vec<RoutingContinuityRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-routing) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<RoutingContinuityWaiver>,
    /// Every exact routing cause, in row then cause order.
    pub routing_causes: Vec<RoutingContinuityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<RoutingContinuityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow
    /// claimed surfaces.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published routing-packet ref.
    pub published_packet_ref: String,
    /// Published routing-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl RoutingContinuityPacket {
    /// Returns the routing row for `family`, if present.
    pub fn row(&self, family: M5ShellSurfaceFamily) -> Option<&RoutingContinuityRow> {
        self.rows.iter().find(|row| row.family == family)
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
                waiver.family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.routing_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.family.as_str(),
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

    /// Projects the light routing dashboard the shell automation consumes.
    pub fn dashboard(&self) -> RoutingContinuityDashboard {
        RoutingContinuityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 owning-window-routing packet serializes")
    }

    /// Deterministic, machine-readable routing CSV: one row per family naming its status,
    /// qualification, dialog/reopen/focus/os-notification posture, window classes, and
    /// waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,canonical_slot,fallback_slot,dialog_binding,reopen_continuity,focus_retention,os_notification_privacy,window_classes,waiver\n",
        );
        for row in &self.rows {
            let window_classes = row
                .declared_window_classes
                .iter()
                .map(|w| w.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.canonical_slot.as_str(),
                row.fallback_slot.as_str(),
                row.dialog_binding.as_str(),
                row.reopen_continuity.as_str(),
                row.focus_retention.as_str(),
                row.os_notification_privacy.as_str(),
                window_classes,
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
            "# M5 owning-window routing: dialogs, approvals, and notifications bound to the owning window and object\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_owning_window_routing`](../../crates/aureline-shell/src/m5_owning_window_routing/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- markdown > \\\n  artifacts/shell/m5-owning-window-routing.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required routing expectations: {}\n",
            self.required_routing_expectations
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!(
            "- Green (full routing): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
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
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Routing rows\n\n");
        out.push_str(
            "| Surface | Status | Qualification | Dialog binding | Reopen | Focus | OS notification | Waiver |\n\
             | ------- | ------ | ------------- | -------------- | ------ | ----- | --------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.dialog_binding.as_str(),
                row.reopen_continuity.as_str(),
                row.focus_retention.as_str(),
                row.os_notification_privacy.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Per-window routed-action plan\n\n");
        out.push_str(
            "| Surface | Window class | Binds to owner | Preserves focus | Single reopen |\n\
             | ------- | ------------ | -------------- | --------------- | ------------- |\n",
        );
        for row in &self.rows {
            for plan in &row.window_plans {
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` |\n",
                    row.surface_label,
                    plan.window_class.as_str(),
                    plan.binds_to_owning_object,
                    plan.preserves_typing_focus,
                    plan.keeps_single_reopen_path,
                ));
            }
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&RoutingContinuityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, RoutingContinuityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed family binds routed dialogs, approvals, and notifications to the owning window and object.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact routing causes\n\n");
        if self.routing_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.routing_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.family.as_str(),
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
                    waiver.family.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_owning_window_routing_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light routing dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuityDashboardRow {
    /// The governed family.
    pub family: M5ShellSurfaceFamily,
    /// Short guarded-surface label.
    pub surface_label: String,
    /// Derived green/yellow/red status.
    pub status: RoutingContinuityStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Number of window classes this family may live in.
    pub window_class_count: usize,
    /// Dialog/approval binding posture.
    pub dialog_binding: DialogBindingState,
    /// Durable-reopen continuity posture.
    pub reopen_continuity: ReopenContinuityState,
    /// Focus-retention posture.
    pub focus_retention: FocusRetentionState,
    /// OS-notification-privacy posture.
    pub os_notification_privacy: OsNotificationPrivacyState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light routing dashboard the shell / windowing / notification / release automation
/// reads to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuityDashboard {
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
    pub rows: Vec<RoutingContinuityDashboardRow>,
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

impl RoutingContinuityDashboard {
    /// Projects the dashboard from a routing packet.
    pub fn from_packet(packet: &RoutingContinuityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| RoutingContinuityDashboardRow {
                family: row.family,
                surface_label: row.surface_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                window_class_count: row.declared_window_classes.len(),
                dialog_binding: row.dialog_binding,
                reopen_continuity: row.reopen_continuity,
                focus_retention: row.focus_retention,
                os_notification_privacy: row.os_notification_privacy,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .routing_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_OWNING_WINDOW_ROUTING_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_OWNING_WINDOW_ROUTING_SCHEMA_VERSION,
            dashboard_id: M5_OWNING_WINDOW_ROUTING_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 owning-window-routing dashboard serializes")
    }
}

/// Support-export wrapper for the owning-window routing packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContinuitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: RoutingContinuityPacket,
    /// Dashboard quoted in full.
    pub dashboard: RoutingContinuityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl RoutingContinuitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: RoutingContinuityPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_OWNING_WINDOW_ROUTING_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_OWNING_WINDOW_ROUTING_SCHEMA_VERSION,
            shared_contract_ref: M5_OWNING_WINDOW_ROUTING_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_owning_window_routing_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingContinuityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family routing rows.
    pub rows: Vec<RoutingContinuityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The routing packet carries only closed vocabulary, refs, and short labels, so raw
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

/// Builds a [`RoutingContinuityPacket`] from the exact build identity, the frozen matrix
/// ref, and the per-family routing rows.
///
/// Each row's derived status and routing causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_owning_window_routing_packet(
    input: RoutingContinuityInput,
) -> RoutingContinuityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent
    // and the auto-narrowing is the single source of truth.
    let rows: Vec<RoutingContinuityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.routing_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<RoutingContinuityFinding> = Vec::new();

    // Every governed family must carry a routing row.
    let present: BTreeSet<M5ShellSurfaceFamily> = rows.iter().map(|row| row.family).collect();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(RoutingContinuityFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, RoutingContinuityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, RoutingContinuityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, RoutingContinuityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(RoutingContinuityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<RoutingContinuityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let routing_causes: Vec<RoutingContinuityCause> = rows
        .iter()
        .flat_map(|row| row.routing_causes.clone())
        .collect();

    let required_routing_expectations: Vec<String> = REQUIRED_ROUTING_EXPECTATIONS
        .iter()
        .map(|expectation| expectation.as_str().to_owned())
        .collect();

    let mut packet = RoutingContinuityPacket {
        record_kind: M5_OWNING_WINDOW_ROUTING_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_OWNING_WINDOW_ROUTING_SCHEMA_VERSION,
        shared_contract_ref: M5_OWNING_WINDOW_ROUTING_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_OWNING_WINDOW_ROUTING_PACKET_ID.to_owned(),
        source_schema_ref: M5_OWNING_WINDOW_ROUTING_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Owning-window dialog, approval, and notification routing for every claimed M5 \
                   surface family: notebook, data grid, profiler, pipeline, docs, preview, review, \
                   incident, companion, and operator each certified so permission sheets, trust \
                   prompts, destructive confirmations, and publish/approval dialogs bind to the \
                   window that owns the authoritative object, durable notification reopen paths \
                   land on the exact object or a truthful placeholder rather than a generic shell, \
                   routed actions never steal focus from a protected typing surface, and \
                   privacy-safe OS-notification summaries preserve one exact reopen path without \
                   bypassing in-app review, with each row's green/yellow/red claim auto-narrowed \
                   from its dialog-binding, reopen-continuity, focus-retention, and \
                   OS-notification-privacy posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_OWNING_WINDOW_ROUTING_MATRIX_SCHEMA_REF.to_owned(),
        window_topology_contract_ref: M5_OWNING_WINDOW_ROUTING_WINDOW_TOPOLOGY_CONTRACT_REF
            .to_owned(),
        session_restore_contract_ref: M5_OWNING_WINDOW_ROUTING_SESSION_RESTORE_CONTRACT_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_routing_expectations,
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        routing_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.owning_window_routing_registry".to_owned(),
            "release_automation.auto_narrow.owning_window_routing_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.owning_window_routing".to_owned(),
            M5_OWNING_WINDOW_ROUTING_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_OWNING_WINDOW_ROUTING_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-owning-window-routing".to_owned()],
        published_report_ref: M5_OWNING_WINDOW_ROUTING_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_OWNING_WINDOW_ROUTING_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_OWNING_WINDOW_ROUTING_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_OWNING_WINDOW_ROUTING_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("routing packet serializes"),
    ) {
        blocking_findings.push(RoutingContinuityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_owning_window_routing_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum RoutingContinuityValidationError {
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
    /// The declared required routing expectations do not match the lane constants.
    RequiredRoutingExpectationsStale,
    /// The rows do not cover all ten governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared routing causes do not match the recomputed causes.
    RoutingCausesStale,
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

/// Validates a packet against the owning-window routing invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed family
/// carries a current routing row; each row's status is the derived auto-narrowed value,
/// never asserted; a green row cannot keep a claim while a routed dialog/approval is lost
/// to focus theft or orphaning, a reopen lands on a generic shell, a routed action steals
/// focus from a protected typing surface, an OS notification leaks content or bypasses
/// in-app review, a routing expectation is undeclared, or a per-window plan drops
/// owning-object binding, focus preservation, or the single reopen path; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_owning_window_routing_packet(
    packet: &RoutingContinuityPacket,
) -> Result<(), Vec<RoutingContinuityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(RoutingContinuityValidationError::NoRows);
    }
    if packet.record_kind != M5_OWNING_WINDOW_ROUTING_PACKET_RECORD_KIND {
        errors.push(RoutingContinuityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_OWNING_WINDOW_ROUTING_SCHEMA_VERSION {
        errors.push(RoutingContinuityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(RoutingContinuityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(RoutingContinuityValidationError::MatrixPacketRefMissing);
    }
    let expected_expectations: Vec<String> = REQUIRED_ROUTING_EXPECTATIONS
        .iter()
        .map(|expectation| expectation.as_str().to_owned())
        .collect();
    if packet.required_routing_expectations != expected_expectations {
        errors.push(RoutingContinuityValidationError::RequiredRoutingExpectationsStale);
    }

    let present: BTreeSet<M5ShellSurfaceFamily> =
        packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = REQUIRED_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_FAMILIES.len() {
        errors.push(RoutingContinuityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_families {
        errors.push(RoutingContinuityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), RoutingContinuityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), RoutingContinuityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), RoutingContinuityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(RoutingContinuityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<RoutingContinuityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(RoutingContinuityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<RoutingContinuityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.routing_causes {
        errors.push(RoutingContinuityValidationError::RoutingCausesStale);
    }

    let mut recomputed: Vec<RoutingContinuityFinding> = Vec::new();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(RoutingContinuityFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(RoutingContinuityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("routing packet serializes"),
    ) {
        recomputed.push(RoutingContinuityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(RoutingContinuityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(RoutingContinuityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(RoutingContinuityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(RoutingContinuityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(RoutingContinuityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(RoutingContinuityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
