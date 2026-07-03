//! Canonical cross-window drag-verb disclosure, close-orphan prevention, and safe
//! specialized-window reopen certification for every claimed M5 surface family.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface family —
//! notebook, data grid, profiler, pipeline, docs, preview, review, incident, companion,
//! and operator — to the window classes it may live in and the owning-window routing
//! expectations it must honor. This lane is the window-lifecycle-safety capstone on top of
//! that matrix: for every governed family it certifies that a **cross-window drag/drop
//! advertises the resulting verb — `Move tab`, `Copy editor`, `Open compare here`,
//! `Create window` — before the drop completes and keeps those verbs keyboard-reachable
//! through command equivalents**, that **closing a secondary window can never silently
//! strand a dirty buffer, a live approval, shared collaboration control, or a long-running
//! evidence review**, and that a **specialized window reopened after crash or restore
//! falls back to the safest equivalent shell arrangement when an extension, remote target,
//! or feature pack is unavailable** rather than orphaning the object or landing on the
//! wrong surface.
//!
//! Three records carry the truth:
//!
//! - the per-family **lifecycle row** ([`WindowLifecycleRow`]): one row per
//!   [`M5ShellSurfaceFamily`] naming the window classes it declares (pulled from the
//!   matrix), the protected close resources it guards, its per-verb cross-window drag plan
//!   ([`CrossWindowDragPlan`]) naming whether each drag verb is disclosed before the drop
//!   and keeps a keyboard command equivalent, its drag-verb-disclosure /
//!   close-orphan-guard / safe-reopen-fallback posture, any active waiver, and a derived
//!   green/yellow/red [`WindowLifecycleStatus`].
//! - the release **lifecycle packet** ([`WindowLifecyclePacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers,
//!   the exact lifecycle causes ([`WindowLifecycleCause`]), and the blocking findings the
//!   lane refuses to ship with.
//! - the **lifecycle dashboard** ([`WindowLifecycleDashboard`]): a light projection the
//!   shell / windowing / layout / status automation reads to auto-narrow a claimed surface
//!   when its lifecycle proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the
//! moment its frozen qualification is below Stable, a drag verb is disclosed but reachable
//! only through a disclosed command-palette equivalent rather than an inline pre-drop hint,
//! a secondary-window close defers a protected resource to a disclosed, waivered relocation
//! into the primary window, or a specialized-window reopen lands on a disclosed reduced but
//! still-safe equivalent layout; it drops to `red` if a cross-window drop completes without
//! disclosing the resulting verb or loses keyboard parity, a secondary-window close
//! silently orphans a dirty buffer, approval, collaboration control, or evidence review, a
//! specialized-window reopen orphans the object or lands on the wrong surface, the row
//! fails to declare all four protected close resources, or a per-verb drag plan drops
//! pre-drop disclosure or its keyboard command equivalent. That derivation is the
//! auto-narrowing the acceptance criteria require, and the protected-resource and drag-plan
//! completeness checks are the lint that prevents a later lifecycle regression from
//! shipping as stable.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, or credentials — only stable
//! ids, closed vocabulary, counts, refs, and short labels. The surface family, window
//! class, qualification, downgrade-trigger, and consumer-surface vocabulary is re-exported
//! by reference from the already frozen [matrix]; the certified rows are pulled straight
//! from that matrix's seeded packet, so this lane mints no parallel shell vocabulary and
//! cannot certify a family the matrix does not freeze. Only the lifecycle-specific
//! vocabulary ([`WindowLifecycleStatus`], [`DragVerb`], [`ProtectedCloseResource`],
//! [`DragVerbDisclosureState`], [`CloseOrphanGuardState`], [`SafeReopenFallbackState`],
//! [`CrossWindowDragPlan`], [`WindowLifecycleWaiver`], [`WindowLifecycleCause`],
//! [`WindowLifecycleFinding`]) is new.
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
    seeded_m5_window_lifecycle_safety_packet,
    seeded_m5_window_lifecycle_safety_packet_datagrid_reopen_wrong_surface_blocked,
    seeded_m5_window_lifecycle_safety_packet_notebook_close_silent_orphan_blocked,
    seeded_m5_window_lifecycle_safety_packet_preview_drag_verb_hidden_blocked,
    seeded_m5_window_lifecycle_safety_packet_review_protected_resource_orphan_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_WINDOW_LIFECYCLE_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_WINDOW_LIFECYCLE_SAFETY_SHARED_CONTRACT_REF: &str =
    "shell:m5_window_lifecycle_safety:v1";

/// Stable record kind for [`WindowLifecyclePacket`] payloads.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PACKET_RECORD_KIND: &str =
    "shell_m5_window_lifecycle_safety_packet_record";

/// Stable record kind for [`WindowLifecycleDashboard`] payloads.
pub const M5_WINDOW_LIFECYCLE_SAFETY_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_window_lifecycle_safety_dashboard_record";

/// Stable record kind for [`WindowLifecycleSupportExport`] payloads.
pub const M5_WINDOW_LIFECYCLE_SAFETY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_window_lifecycle_safety_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PACKET_ID: &str = "m5-window-lifecycle-safety:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_WINDOW_LIFECYCLE_SAFETY_DASHBOARD_ID: &str =
    "m5-window-lifecycle-safety-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_WINDOW_LIFECYCLE_SAFETY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-window-lifecycle-safety:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_WINDOW_LIFECYCLE_SAFETY_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-window-lifecycle-safety.schema.json";

/// Published markdown report ref reviewers reopen the lifecycle proof from.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-window-lifecycle-safety.md";

/// Published lifecycle-packet artifact ref.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-window-lifecycle-safety-proof/packet.json";

/// Published lifecycle-dashboard artifact ref.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-window-lifecycle-safety-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-window-lifecycle-safety-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-window-lifecycle-safety-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_window_lifecycle_safety_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_WINDOW_LIFECYCLE_SAFETY_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Window-topology contract this proof mirrors for detach/close/reopen safety.
pub const M5_WINDOW_LIFECYCLE_SAFETY_WINDOW_TOPOLOGY_CONTRACT_REF: &str =
    matrix::M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF;

/// Session-restore fidelity contract this proof mirrors for reopen fallback.
pub const M5_WINDOW_LIFECYCLE_SAFETY_SESSION_RESTORE_CONTRACT_REF: &str =
    matrix::M5_SHELL_SESSION_RESTORE_CONTRACT_REF;

/// Every governed surface family the lifecycle proof must cover, in canonical order.
/// These are exactly the families the frozen shell-zone matrix freezes; the lane
/// certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// The canonical cross-window drag verbs the drop affordance must advertise before the
/// drop completes, and keep keyboard-reachable through a command equivalent.
pub const REQUIRED_DRAG_VERBS: [DragVerb; 4] = DragVerb::ALL;

/// Every protected close resource a secondary-window close must guard, in canonical order.
pub const REQUIRED_PROTECTED_RESOURCES: [ProtectedCloseResource; 4] = ProtectedCloseResource::ALL;

/// The derived window-lifecycle-safety light a governed surface family carries.
///
/// `green` means every cross-window drag verb is advertised before the drop and stays
/// keyboard-reachable, a secondary-window close never orphans a protected resource, and a
/// specialized-window reopen lands on the safest equivalent layout. `yellow` is a disclosed
/// narrowing (the family is honestly narrowed below Stable, discloses a command-palette
/// drag-verb reach, defers a protected resource to a disclosed waivered relocation on
/// close, or reopens into a disclosed reduced but still-safe equivalent). `red` is blocked:
/// a cross-window drop completes without disclosing the verb or loses keyboard parity, a
/// close silently orphans a protected resource, a reopen orphans the object or lands on the
/// wrong surface, a protected resource is undeclared, or a per-verb drag plan drops
/// pre-drop disclosure or its keyboard command equivalent — and it may not keep a
/// shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowLifecycleStatus {
    /// Full standing: verbs advertised, no close orphan, safest reopen.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl WindowLifecycleStatus {
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

/// A cross-window drag verb the drop affordance advertises before the drop completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DragVerb {
    /// Move a tab into the target window.
    MoveTab,
    /// Copy an editor into the target window.
    CopyEditor,
    /// Open a compare view anchored in the target window.
    OpenCompareHere,
    /// Create a new window from the dropped object.
    CreateWindow,
}

impl DragVerb {
    /// Every drag verb, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MoveTab,
        Self::CopyEditor,
        Self::OpenCompareHere,
        Self::CreateWindow,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MoveTab => "move_tab",
            Self::CopyEditor => "copy_editor",
            Self::OpenCompareHere => "open_compare_here",
            Self::CreateWindow => "create_window",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MoveTab => "Move tab",
            Self::CopyEditor => "Copy editor",
            Self::OpenCompareHere => "Open compare here",
            Self::CreateWindow => "Create window",
        }
    }
}

/// A resource a secondary-window close must guard against silent orphaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedCloseResource {
    /// An unsaved (dirty) editor or notebook buffer.
    DirtyBuffer,
    /// A live approval, trust prompt, or destructive confirmation in flight.
    LiveApproval,
    /// Shared collaboration control (host/driver role) held by the window.
    CollaborationControl,
    /// A long-running evidence review the window owns.
    EvidenceReview,
}

impl ProtectedCloseResource {
    /// Every protected close resource, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DirtyBuffer,
        Self::LiveApproval,
        Self::CollaborationControl,
        Self::EvidenceReview,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirtyBuffer => "dirty_buffer",
            Self::LiveApproval => "live_approval",
            Self::CollaborationControl => "collaboration_control",
            Self::EvidenceReview => "evidence_review",
        }
    }
}

/// How a cross-window drag/drop advertises the resulting verb and keeps it keyboard-reachable.
///
/// `verb_disclosed_with_keyboard_parity` means the drop affordance advertises the exact
/// resulting verb before the drop completes and every drag verb keeps a keyboard command
/// equivalent. `disclosed_verb_reach_narrowing` means the verb is still advertised but a
/// specific verb is reachable only through a disclosed command-palette equivalent rather
/// than an inline pre-drop hint — a yellow narrowing. `verb_hidden_or_keyboard_lost` means
/// a cross-window drop completed without disclosing the resulting verb, or a drag verb lost
/// its keyboard command equivalent — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DragVerbDisclosureState {
    /// The resulting verb is advertised before the drop and keeps keyboard parity.
    VerbDisclosedWithKeyboardParity,
    /// A drag verb is reachable only through a disclosed command-palette equivalent.
    DisclosedVerbReachNarrowing,
    /// A drop completed without disclosing the verb, or lost keyboard parity — a blocker.
    VerbHiddenOrKeyboardLost,
}

impl DragVerbDisclosureState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerbDisclosedWithKeyboardParity => "verb_disclosed_with_keyboard_parity",
            Self::DisclosedVerbReachNarrowing => "disclosed_verb_reach_narrowing",
            Self::VerbHiddenOrKeyboardLost => "verb_hidden_or_keyboard_lost",
        }
    }

    /// `true` when the drag verb is advertised with keyboard parity at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::VerbDisclosedWithKeyboardParity)
    }

    /// `true` when the drag disclosure took a disclosed command-palette-reach narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedVerbReachNarrowing)
    }
}

/// How closing a secondary window guards protected resources against silent orphaning.
///
/// `close_guarded_no_orphan` means closing a secondary window blocks on, or safely
/// resolves, every dirty buffer, live approval, collaboration control, and evidence review
/// so none is silently stranded. `disclosed_deferred_guard_relocation` means the close
/// defers a protected resource to a disclosed, waivered relocation into the primary window
/// (with a still-visible prompt) rather than blocking outright — a yellow narrowing.
/// `silent_orphan_on_close` means closing the window silently stranded a dirty buffer,
/// approval, collaboration control, or evidence review — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseOrphanGuardState {
    /// Closing a secondary window never orphans a protected resource.
    CloseGuardedNoOrphan,
    /// The close defers a protected resource to a disclosed, waivered relocation.
    DisclosedDeferredGuardRelocation,
    /// Closing the window silently orphaned a protected resource — a blocker.
    SilentOrphanOnClose,
}

impl CloseOrphanGuardState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CloseGuardedNoOrphan => "close_guarded_no_orphan",
            Self::DisclosedDeferredGuardRelocation => "disclosed_deferred_guard_relocation",
            Self::SilentOrphanOnClose => "silent_orphan_on_close",
        }
    }

    /// `true` when a close never orphans a protected resource at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CloseGuardedNoOrphan)
    }

    /// `true` when the close took a disclosed deferred-relocation narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedDeferredGuardRelocation)
    }
}

/// How a specialized window reopens when an exact dependency is unavailable.
///
/// `reopens_safest_equivalent_layout` means a specialized window reopened after crash or
/// restore lands on the safest equivalent shell arrangement even when an extension, remote
/// target, or feature pack is missing, preserving the object identity and reopen path.
/// `disclosed_reduced_equivalent_fallback` means the reopen lands on a disclosed reduced
/// but still-safe equivalent layout (a live capability that could not be restored) while
/// preserving identity and the reopen path — a yellow narrowing.
/// `reopen_orphaned_or_wrong_surface` means the reopen orphaned the object or landed on the
/// wrong surface, losing identity and the reopen path — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeReopenFallbackState {
    /// Reopen lands on the safest equivalent layout, preserving identity and reopen path.
    ReopensSafestEquivalentLayout,
    /// Reopen lands on a disclosed reduced but still-safe equivalent layout.
    DisclosedReducedEquivalentFallback,
    /// Reopen orphaned the object or landed on the wrong surface — a blocker.
    ReopenOrphanedOrWrongSurface,
}

impl SafeReopenFallbackState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopensSafestEquivalentLayout => "reopens_safest_equivalent_layout",
            Self::DisclosedReducedEquivalentFallback => "disclosed_reduced_equivalent_fallback",
            Self::ReopenOrphanedOrWrongSurface => "reopen_orphaned_or_wrong_surface",
        }
    }

    /// `true` when a reopen lands on the safest equivalent layout at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ReopensSafestEquivalentLayout)
    }

    /// `true` when the reopen took a disclosed reduced-equivalent fallback.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedEquivalentFallback)
    }
}

/// The cross-window drag plan a family declares for one drag verb.
///
/// The plan must advertise the verb before the drop completes and keep a keyboard command
/// equivalent; a verb that is not disclosed before the drop, or that lost its keyboard
/// command equivalent, is a blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWindowDragPlan {
    /// The drag verb this plan describes.
    pub verb: DragVerb,
    /// `true` when the resulting verb is advertised before the cross-window drop completes.
    pub disclosed_before_drop: bool,
    /// `true` when the verb is reachable through a keyboard command equivalent.
    pub keyboard_command_equivalent: bool,
}

impl CrossWindowDragPlan {
    /// `true` when the plan discloses the verb before the drop and keeps keyboard parity.
    pub const fn is_fully_disclosed(&self) -> bool {
        self.disclosed_before_drop && self.keyboard_command_equivalent
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
/// (yellow) rather than blocked — never lets a silent close orphan, a hidden verb, or a
/// wrong-surface reopen hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecycleWaiver {
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

impl WindowLifecycleWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's lifecycle claim.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecycleCause {
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

impl WindowLifecycleCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed surface family, certified across its cross-window drag-verb disclosure,
/// close-orphan-guard, and safe-reopen-fallback posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecycleRow {
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
    /// Protected close resources a secondary-window close must guard.
    pub declared_protected_resources: Vec<ProtectedCloseResource>,
    /// Per-verb cross-window drag plan, one per canonical drag verb.
    pub drag_plans: Vec<CrossWindowDragPlan>,
    /// Cross-window drag-verb disclosure posture.
    pub drag_verb_disclosure: DragVerbDisclosureState,
    /// Secondary-window close-orphan-guard posture.
    pub close_orphan_guard: CloseOrphanGuardState,
    /// Specialized-window safe-reopen-fallback posture.
    pub safe_reopen_fallback: SafeReopenFallbackState,
    /// Consumer surfaces this family must stay aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed deferred close-guard relocation is in force.
    pub active_waiver: Option<WindowLifecycleWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: WindowLifecycleStatus,
    /// The exact lifecycle causes that narrowed or blocked this row.
    pub lifecycle_causes: Vec<WindowLifecycleCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl WindowLifecycleRow {
    /// `true` when this family's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row declares all four protected close resources.
    pub fn protected_resources_complete(&self) -> bool {
        let declared: BTreeSet<ProtectedCloseResource> =
            self.declared_protected_resources.iter().copied().collect();
        let required: BTreeSet<ProtectedCloseResource> =
            REQUIRED_PROTECTED_RESOURCES.iter().copied().collect();
        declared == required && declared.len() == self.declared_protected_resources.len()
    }

    /// `true` when the per-verb drag plans cover exactly the canonical drag verbs — no verb
    /// the drop affordance must advertise is left uncertified and none is invented.
    pub fn plans_cover_required_verbs(&self) -> bool {
        let declared: BTreeSet<DragVerb> = self.drag_plans.iter().map(|plan| plan.verb).collect();
        let required: BTreeSet<DragVerb> = REQUIRED_DRAG_VERBS.iter().copied().collect();
        declared == required && declared.len() == self.drag_plans.len()
    }

    /// `true` when every per-verb plan advertises its verb before the drop completes.
    pub fn plans_disclose_before_drop(&self) -> bool {
        self.drag_plans
            .iter()
            .all(|plan| plan.disclosed_before_drop)
    }

    /// `true` when every per-verb plan keeps a keyboard command equivalent.
    pub fn plans_keyboard_reachable(&self) -> bool {
        self.drag_plans
            .iter()
            .all(|plan| plan.keyboard_command_equivalent)
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.protected_resources_complete() {
            return true;
        }
        if matches!(
            self.drag_verb_disclosure,
            DragVerbDisclosureState::VerbHiddenOrKeyboardLost
        ) {
            return true;
        }
        if matches!(
            self.close_orphan_guard,
            CloseOrphanGuardState::SilentOrphanOnClose
        ) {
            return true;
        }
        if matches!(
            self.safe_reopen_fallback,
            SafeReopenFallbackState::ReopenOrphanedOrWrongSurface
        ) {
            return true;
        }
        if !self.plans_cover_required_verbs()
            || !self.plans_disclose_before_drop()
            || !self.plans_keyboard_reachable()
        {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || self.drag_verb_disclosure.is_disclosed_narrowing()
            || self.close_orphan_guard.is_disclosed_narrowing()
            || self.safe_reopen_fallback.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the lifecycle posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing
    /// forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> WindowLifecycleStatus {
        if self.has_hard_blocker() {
            WindowLifecycleStatus::Red
        } else if self.has_narrowing() {
            WindowLifecycleStatus::Yellow
        } else {
            WindowLifecycleStatus::Green
        }
    }

    /// Recomputes the exact lifecycle causes for the row, in deterministic order
    /// (qualification, protected-resource completeness, drag disclosure, close guard, reopen).
    pub fn recompute_causes(&self) -> Vec<WindowLifecycleCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(WindowLifecycleCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen shell-zone matrix qualifies this family at `{}`, below a Stable shell claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        if !self.protected_resources_complete() {
            causes.push(WindowLifecycleCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                disclosed: false,
                detail:
                    "The row does not declare all four protected close resources (dirty buffer, \
                         live approval, collaboration control, evidence review), so closing a \
                         secondary window could silently orphan one."
                        .to_owned(),
            });
        }
        match self.drag_verb_disclosure {
            DragVerbDisclosureState::VerbDisclosedWithKeyboardParity => {}
            DragVerbDisclosureState::DisclosedVerbReachNarrowing => {
                causes.push(WindowLifecycleCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A cross-window drag verb is still advertised before the drop but is \
                             reachable only through a disclosed command-palette equivalent rather \
                             than an inline pre-drop hint; keyboard parity is preserved."
                        .to_owned(),
                });
            }
            DragVerbDisclosureState::VerbHiddenOrKeyboardLost => {
                causes.push(WindowLifecycleCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                    disclosed: false,
                    detail: "A cross-window drop completed without advertising the resulting verb, \
                             or a drag verb lost its keyboard command equivalent, so the user could \
                             not tell what a cross-window drop would do."
                        .to_owned(),
                });
            }
        }
        match self.close_orphan_guard {
            CloseOrphanGuardState::CloseGuardedNoOrphan => {}
            CloseOrphanGuardState::DisclosedDeferredGuardRelocation => {
                causes.push(WindowLifecycleCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail:
                        "Closing a secondary window defers a protected resource to a disclosed, \
                             waivered relocation into the primary workspace window with a \
                             still-visible prompt rather than blocking outright, so nothing is \
                             silently orphaned."
                            .to_owned(),
                });
            }
            CloseOrphanGuardState::SilentOrphanOnClose => {
                causes.push(WindowLifecycleCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                    disclosed: false,
                    detail: "Closing a secondary window silently stranded a dirty buffer, live \
                             approval, collaboration control, or long-running evidence review \
                             instead of guarding it."
                        .to_owned(),
                });
            }
        }
        match self.safe_reopen_fallback {
            SafeReopenFallbackState::ReopensSafestEquivalentLayout => {}
            SafeReopenFallbackState::DisclosedReducedEquivalentFallback => {
                causes.push(WindowLifecycleCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A specialized window reopens onto a disclosed reduced but still-safe \
                             equivalent layout because an extension, remote target, or feature pack \
                             is unavailable, while preserving the object identity and reopen path."
                        .to_owned(),
                });
            }
            SafeReopenFallbackState::ReopenOrphanedOrWrongSurface => {
                causes.push(WindowLifecycleCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
                    disclosed: false,
                    detail:
                        "A specialized-window reopen orphaned the object or landed on the wrong \
                             surface when an exact dependency was missing, losing the object \
                             identity and its reopen path."
                            .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed deferred close-guard relocation may only stay yellow (rather than red)
    /// when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.close_orphan_guard,
            CloseOrphanGuardState::DisclosedDeferredGuardRelocation
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<WindowLifecycleFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if !self.protected_resources_complete() {
            findings.push(WindowLifecycleFinding::ProtectedResourcesIncomplete {
                family: family.clone(),
            });
        }
        if matches!(
            self.drag_verb_disclosure,
            DragVerbDisclosureState::VerbHiddenOrKeyboardLost
        ) {
            findings.push(WindowLifecycleFinding::DragVerbHiddenOrKeyboardLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.close_orphan_guard,
            CloseOrphanGuardState::SilentOrphanOnClose
        ) {
            findings.push(WindowLifecycleFinding::CloseSilentOrphan {
                family: family.clone(),
            });
        }
        if matches!(
            self.safe_reopen_fallback,
            SafeReopenFallbackState::ReopenOrphanedOrWrongSurface
        ) {
            findings.push(WindowLifecycleFinding::ReopenOrphanedOrWrongSurface {
                family: family.clone(),
            });
        }
        if !self.plans_cover_required_verbs() {
            findings.push(WindowLifecycleFinding::DragPlanVerbCoverageMismatch {
                family: family.clone(),
            });
        }
        if !self.plans_disclose_before_drop() {
            findings.push(WindowLifecycleFinding::DragPlanNotDisclosedBeforeDrop {
                family: family.clone(),
            });
        }
        if !self.plans_keyboard_reachable() {
            findings.push(WindowLifecycleFinding::DragPlanKeyboardParityLost {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, WindowLifecycleStatus::Green) && !self.has_reason() {
            findings.push(WindowLifecycleFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(WindowLifecycleFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(WindowLifecycleFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(WindowLifecycleFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(WindowLifecycleFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.lifecycle_causes != self.recompute_causes() {
            findings.push(WindowLifecycleFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} drag={} close={} reopen={} verbs={} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.drag_verb_disclosure.as_str(),
            self.close_orphan_guard.as_str(),
            self.safe_reopen_fallback.as_str(),
            self.drag_plans.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the window-lifecycle-safety proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum WindowLifecycleFinding {
    /// A governed surface family has no lifecycle row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row does not declare all four protected close resources.
    ProtectedResourcesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row's cross-window drop hid the resulting verb or lost keyboard parity.
    DragVerbHiddenOrKeyboardLost {
        /// The family token.
        family: String,
    },
    /// A row's secondary-window close silently orphaned a protected resource.
    CloseSilentOrphan {
        /// The family token.
        family: String,
    },
    /// A row's specialized-window reopen orphaned the object or landed on the wrong surface.
    ReopenOrphanedOrWrongSurface {
        /// The family token.
        family: String,
    },
    /// A row's per-verb drag plans do not cover exactly the canonical drag verbs.
    DragPlanVerbCoverageMismatch {
        /// The family token.
        family: String,
    },
    /// A row's per-verb drag plan does not advertise its verb before the drop.
    DragPlanNotDisclosedBeforeDrop {
        /// The family token.
        family: String,
    },
    /// A row's per-verb drag plan lost its keyboard command equivalent.
    DragPlanKeyboardParityLost {
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
    /// The declared lifecycle causes do not match the recomputed causes.
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

impl WindowLifecycleFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::ProtectedResourcesIncomplete { .. } => "protected_resources_incomplete",
            Self::DragVerbHiddenOrKeyboardLost { .. } => "drag_verb_hidden_or_keyboard_lost",
            Self::CloseSilentOrphan { .. } => "close_silent_orphan",
            Self::ReopenOrphanedOrWrongSurface { .. } => "reopen_orphaned_or_wrong_surface",
            Self::DragPlanVerbCoverageMismatch { .. } => "drag_plan_verb_coverage_mismatch",
            Self::DragPlanNotDisclosedBeforeDrop { .. } => "drag_plan_not_disclosed_before_drop",
            Self::DragPlanKeyboardParityLost { .. } => "drag_plan_keyboard_parity_lost",
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
            | Self::ProtectedResourcesIncomplete { family }
            | Self::DragVerbHiddenOrKeyboardLost { family }
            | Self::CloseSilentOrphan { family }
            | Self::ReopenOrphanedOrWrongSurface { family }
            | Self::DragPlanVerbCoverageMismatch { family }
            | Self::DragPlanNotDisclosedBeforeDrop { family }
            | Self::DragPlanKeyboardParityLost { family }
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

/// The release window-lifecycle-safety packet shared by the shell / windowing / layout /
/// status automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecyclePacket {
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
    /// The four canonical cross-window drag verbs every row must advertise.
    pub required_drag_verbs: Vec<String>,
    /// The four protected close resources every row must guard.
    pub required_protected_resources: Vec<String>,
    /// Per-family lifecycle rows, in canonical order.
    pub rows: Vec<WindowLifecycleRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-lifecycle) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<WindowLifecycleWaiver>,
    /// Every exact lifecycle cause, in row then cause order.
    pub lifecycle_causes: Vec<WindowLifecycleCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<WindowLifecycleFinding>,
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
    /// Published lifecycle-packet ref.
    pub published_packet_ref: String,
    /// Published lifecycle-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl WindowLifecyclePacket {
    /// Returns the lifecycle row for `family`, if present.
    pub fn row(&self, family: M5ShellSurfaceFamily) -> Option<&WindowLifecycleRow> {
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
        for cause in &self.lifecycle_causes {
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

    /// Projects the light lifecycle dashboard the shell automation consumes.
    pub fn dashboard(&self) -> WindowLifecycleDashboard {
        WindowLifecycleDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 window-lifecycle-safety packet serializes")
    }

    /// Deterministic, machine-readable lifecycle CSV: one row per family naming its status,
    /// qualification, drag/close/reopen posture, window classes, and waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,canonical_slot,fallback_slot,drag_verb_disclosure,close_orphan_guard,safe_reopen_fallback,window_classes,waiver\n",
        );
        for row in &self.rows {
            let window_classes = row
                .declared_window_classes
                .iter()
                .map(|w| w.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.canonical_slot.as_str(),
                row.fallback_slot.as_str(),
                row.drag_verb_disclosure.as_str(),
                row.close_orphan_guard.as_str(),
                row.safe_reopen_fallback.as_str(),
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
            "# M5 window lifecycle safety: cross-window drag verbs, close-orphan prevention, and safe specialized-window reopen\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_window_lifecycle_safety`](../../crates/aureline-shell/src/m5_window_lifecycle_safety/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- markdown > \\\n  artifacts/shell/m5-window-lifecycle-safety.md\n",
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
            "- Required drag verbs: {}\n",
            self.required_drag_verbs
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Required protected resources: {}\n",
            self.required_protected_resources
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!(
            "- Green (full lifecycle): {}\n",
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

        out.push_str("## Lifecycle rows\n\n");
        out.push_str(
            "| Surface | Status | Qualification | Drag verb disclosure | Close-orphan guard | Safe reopen | Waiver |\n\
             | ------- | ------ | ------------- | -------------------- | ------------------ | ----------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.drag_verb_disclosure.as_str(),
                row.close_orphan_guard.as_str(),
                row.safe_reopen_fallback.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Per-verb cross-window drag plan\n\n");
        out.push_str(
            "| Surface | Drag verb | Disclosed before drop | Keyboard equivalent |\n\
             | ------- | --------- | --------------------- | ------------------- |\n",
        );
        for row in &self.rows {
            for plan in &row.drag_plans {
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` |\n",
                    row.surface_label,
                    plan.verb.as_str(),
                    plan.disclosed_before_drop,
                    plan.keyboard_command_equivalent,
                ));
            }
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&WindowLifecycleRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, WindowLifecycleStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed family advertises cross-window drag verbs, guards protected resources on close, and reopens into the safest equivalent layout.\n\n",
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

        out.push_str("## Exact lifecycle causes\n\n");
        if self.lifecycle_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.lifecycle_causes {
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_window_lifecycle_safety_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light lifecycle dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecycleDashboardRow {
    /// The governed family.
    pub family: M5ShellSurfaceFamily,
    /// Short guarded-surface label.
    pub surface_label: String,
    /// Derived green/yellow/red status.
    pub status: WindowLifecycleStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Number of window classes this family may live in.
    pub window_class_count: usize,
    /// Cross-window drag-verb disclosure posture.
    pub drag_verb_disclosure: DragVerbDisclosureState,
    /// Secondary-window close-orphan-guard posture.
    pub close_orphan_guard: CloseOrphanGuardState,
    /// Specialized-window safe-reopen-fallback posture.
    pub safe_reopen_fallback: SafeReopenFallbackState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light lifecycle dashboard the shell / windowing / layout / status automation reads
/// to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecycleDashboard {
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
    pub rows: Vec<WindowLifecycleDashboardRow>,
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

impl WindowLifecycleDashboard {
    /// Projects the dashboard from a lifecycle packet.
    pub fn from_packet(packet: &WindowLifecyclePacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| WindowLifecycleDashboardRow {
                family: row.family,
                surface_label: row.surface_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                window_class_count: row.declared_window_classes.len(),
                drag_verb_disclosure: row.drag_verb_disclosure,
                close_orphan_guard: row.close_orphan_guard,
                safe_reopen_fallback: row.safe_reopen_fallback,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .lifecycle_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_WINDOW_LIFECYCLE_SAFETY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_WINDOW_LIFECYCLE_SAFETY_SCHEMA_VERSION,
            dashboard_id: M5_WINDOW_LIFECYCLE_SAFETY_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 window-lifecycle-safety dashboard serializes")
    }
}

/// Support-export wrapper for the window-lifecycle-safety packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowLifecycleSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: WindowLifecyclePacket,
    /// Dashboard quoted in full.
    pub dashboard: WindowLifecycleDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl WindowLifecycleSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: WindowLifecyclePacket,
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
            record_kind: M5_WINDOW_LIFECYCLE_SAFETY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_WINDOW_LIFECYCLE_SAFETY_SCHEMA_VERSION,
            shared_contract_ref: M5_WINDOW_LIFECYCLE_SAFETY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_window_lifecycle_safety_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowLifecycleInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family lifecycle rows.
    pub rows: Vec<WindowLifecycleRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The lifecycle packet carries only closed vocabulary, refs, and short labels, so raw
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

/// Builds a [`WindowLifecyclePacket`] from the exact build identity, the frozen matrix ref,
/// and the per-family lifecycle rows.
///
/// Each row's derived status and lifecycle causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_window_lifecycle_safety_packet(
    input: WindowLifecycleInput,
) -> WindowLifecyclePacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and
    // the auto-narrowing is the single source of truth.
    let rows: Vec<WindowLifecycleRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.lifecycle_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<WindowLifecycleFinding> = Vec::new();

    // Every governed family must carry a lifecycle row.
    let present: BTreeSet<M5ShellSurfaceFamily> = rows.iter().map(|row| row.family).collect();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(WindowLifecycleFinding::FamilyMissing {
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
        .filter(|row| matches!(row.derived_status, WindowLifecycleStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, WindowLifecycleStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, WindowLifecycleStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(WindowLifecycleFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<WindowLifecycleWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let lifecycle_causes: Vec<WindowLifecycleCause> = rows
        .iter()
        .flat_map(|row| row.lifecycle_causes.clone())
        .collect();

    let required_drag_verbs: Vec<String> = REQUIRED_DRAG_VERBS
        .iter()
        .map(|verb| verb.as_str().to_owned())
        .collect();
    let required_protected_resources: Vec<String> = REQUIRED_PROTECTED_RESOURCES
        .iter()
        .map(|resource| resource.as_str().to_owned())
        .collect();

    let mut packet = WindowLifecyclePacket {
        record_kind: M5_WINDOW_LIFECYCLE_SAFETY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_WINDOW_LIFECYCLE_SAFETY_SCHEMA_VERSION,
        shared_contract_ref: M5_WINDOW_LIFECYCLE_SAFETY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_WINDOW_LIFECYCLE_SAFETY_PACKET_ID.to_owned(),
        source_schema_ref: M5_WINDOW_LIFECYCLE_SAFETY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Cross-window drag-verb disclosure, secondary-window close-orphan prevention, and \
                   safe specialized-window reopen for every claimed M5 surface family: notebook, \
                   data grid, profiler, pipeline, docs, preview, review, incident, companion, and \
                   operator each certified so a cross-window drag/drop advertises the resulting verb \
                   (Move tab, Copy editor, Open compare here, Create window) before the drop \
                   completes and keeps it keyboard-reachable, closing a secondary window never \
                   silently orphans a dirty buffer, live approval, collaboration control, or \
                   evidence review, and a specialized window reopens into the safest equivalent \
                   layout when an extension, remote target, or feature pack is unavailable, with \
                   each row's green/yellow/red claim auto-narrowed from its drag-verb-disclosure, \
                   close-orphan-guard, and safe-reopen-fallback posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_WINDOW_LIFECYCLE_SAFETY_MATRIX_SCHEMA_REF.to_owned(),
        window_topology_contract_ref: M5_WINDOW_LIFECYCLE_SAFETY_WINDOW_TOPOLOGY_CONTRACT_REF
            .to_owned(),
        session_restore_contract_ref: M5_WINDOW_LIFECYCLE_SAFETY_SESSION_RESTORE_CONTRACT_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_drag_verbs,
        required_protected_resources,
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        lifecycle_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.window_lifecycle_safety_registry".to_owned(),
            "release_automation.auto_narrow.window_lifecycle_safety_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.window_lifecycle_safety".to_owned(),
            M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-window-lifecycle-safety".to_owned()],
        published_report_ref: M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_WINDOW_LIFECYCLE_SAFETY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("lifecycle packet serializes"),
    ) {
        blocking_findings.push(WindowLifecycleFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_window_lifecycle_safety_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum WindowLifecycleValidationError {
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
    /// The declared required drag verbs do not match the lane constants.
    RequiredDragVerbsStale,
    /// The declared required protected resources do not match the lane constants.
    RequiredProtectedResourcesStale,
    /// The rows do not cover all ten governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared lifecycle causes do not match the recomputed causes.
    LifecycleCausesStale,
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

/// Validates a packet against the window-lifecycle-safety invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed family
/// carries a current lifecycle row; each row's status is the derived auto-narrowed value,
/// never asserted; a green row cannot keep a claim while a cross-window drop hides the
/// resulting verb or loses keyboard parity, a secondary-window close silently orphans a
/// protected resource, a specialized-window reopen orphans the object or lands on the wrong
/// surface, a protected resource is undeclared, or a per-verb drag plan drops pre-drop
/// disclosure or its keyboard command equivalent; and a disclosed narrowing is backed by a
/// reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_window_lifecycle_safety_packet(
    packet: &WindowLifecyclePacket,
) -> Result<(), Vec<WindowLifecycleValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(WindowLifecycleValidationError::NoRows);
    }
    if packet.record_kind != M5_WINDOW_LIFECYCLE_SAFETY_PACKET_RECORD_KIND {
        errors.push(WindowLifecycleValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_WINDOW_LIFECYCLE_SAFETY_SCHEMA_VERSION {
        errors.push(WindowLifecycleValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(WindowLifecycleValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(WindowLifecycleValidationError::MatrixPacketRefMissing);
    }
    let expected_verbs: Vec<String> = REQUIRED_DRAG_VERBS
        .iter()
        .map(|verb| verb.as_str().to_owned())
        .collect();
    if packet.required_drag_verbs != expected_verbs {
        errors.push(WindowLifecycleValidationError::RequiredDragVerbsStale);
    }
    let expected_resources: Vec<String> = REQUIRED_PROTECTED_RESOURCES
        .iter()
        .map(|resource| resource.as_str().to_owned())
        .collect();
    if packet.required_protected_resources != expected_resources {
        errors.push(WindowLifecycleValidationError::RequiredProtectedResourcesStale);
    }

    let present: BTreeSet<M5ShellSurfaceFamily> =
        packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = REQUIRED_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_FAMILIES.len() {
        errors.push(WindowLifecycleValidationError::CoverageIncomplete);
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
        errors.push(WindowLifecycleValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), WindowLifecycleStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), WindowLifecycleStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), WindowLifecycleStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(WindowLifecycleValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<WindowLifecycleWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(WindowLifecycleValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<WindowLifecycleCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.lifecycle_causes {
        errors.push(WindowLifecycleValidationError::LifecycleCausesStale);
    }

    let mut recomputed: Vec<WindowLifecycleFinding> = Vec::new();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(WindowLifecycleFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(WindowLifecycleFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("lifecycle packet serializes"),
    ) {
        recomputed.push(WindowLifecycleFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(WindowLifecycleValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(WindowLifecycleValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(WindowLifecycleValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(WindowLifecycleValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(WindowLifecycleValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(WindowLifecycleValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
