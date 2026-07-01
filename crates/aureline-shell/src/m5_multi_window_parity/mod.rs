//! Canonical multi-window truth-parity certification for every claimed M5 surface
//! family.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface family —
//! notebook, data grid, profiler, pipeline, docs, preview, review, incident, companion,
//! and operator — to the window classes it may live in (primary workspace, secondary
//! detached, floating utility, companion overlay), the workspace-global continuity
//! truths it must preserve (workspace-global trust, remote target, deployment profile,
//! recovery state), and the owning-window routing expectations it must honor. This lane
//! is the multi-window-truth capstone on top of that matrix: for every governed family
//! it certifies that **every window carrying its work preserves the same workspace
//! identity, trust, remote/host, profile, and recovery-critical truth** while keeping
//! layout, density, and focus **local** to that window, that dialogs, notifications, and
//! approvals **route back to the owning window and object** without focus theft or
//! orphaning, and that crash-restore, dependency-loss, and monitor-topology drills stay
//! **predictable and non-destructive**.
//!
//! Three records carry the truth:
//!
//! - the per-family **parity row** ([`MultiWindowParityRow`]): one row per
//!   [`M5ShellSurfaceFamily`] naming the window classes and continuity truths it declares
//!   (pulled from the matrix), its per-window continuity plan ([`WindowContinuityPlan`])
//!   naming which truths each window preserves and whether layout stays local and routing
//!   returns to the owning window, its continuity-parity / layout-locality /
//!   owning-window-routing / recovery-drill posture, any active waiver, and a derived
//!   green/yellow/red [`MultiWindowParityStatus`].
//! - the release **parity packet** ([`MultiWindowParityPacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers,
//!   the exact parity causes ([`MultiWindowParityCause`]), and the blocking findings the
//!   lane refuses to ship with.
//! - the **parity dashboard** ([`MultiWindowParityDashboard`]): a light projection the
//!   shell / windowing / layout / release automation reads to auto-narrow a claimed
//!   surface when its multi-window proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow`
//! the moment its frozen qualification is below Stable, a detached or utility window
//! discloses a narrowed (but still visible) projection of a workspace-global truth, it
//! discloses a purely-local window state, a routed action relocates to a disclosed,
//! waivered still-visible affordance, or a recovery drill discloses a narrowed but
//! non-destructive recovery; it drops to `red` if workspace-global truth diverges across
//! windows, a window hides workspace-global risk or policy state behind a local layout
//! choice, a routed action is lost to focus theft or orphaning, a crash-restore /
//! dependency-loss / monitor-topology drill is destructive or orphaning, the row fails to
//! declare all four workspace-global continuity truths or all four owning-window routing
//! expectations, or a per-window plan drops a required truth, loses layout locality, or
//! loses owning-window routing. That derivation is the auto-narrowing the acceptance
//! criteria require, and the truth-completeness and per-window plan checks are the lint
//! that prevents a later cross-window regression from shipping as stable.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, or credentials — only stable
//! ids, closed vocabulary, counts, refs, and short labels. The surface family, window
//! class, continuity truth, owning-window routing, qualification, downgrade-trigger, and
//! consumer-surface vocabulary is re-exported by reference from the already frozen
//! [matrix]; the certified rows are pulled straight from that matrix's seeded packet, so
//! this lane mints no parallel shell vocabulary and cannot certify a family the matrix
//! does not freeze. Only the parity-specific vocabulary ([`MultiWindowParityStatus`],
//! [`ContinuityParityState`], [`LayoutLocalityState`], [`OwningWindowRoutingState`],
//! [`RecoveryDrillState`], [`WindowContinuityPlan`], [`MultiWindowParityWaiver`],
//! [`MultiWindowParityCause`], [`MultiWindowParityFinding`]) is new.
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
    seeded_m5_multi_window_parity_packet,
    seeded_m5_multi_window_parity_packet_companion_required_truth_missing_blocked,
    seeded_m5_multi_window_parity_packet_datagrid_recovery_destructive_blocked,
    seeded_m5_multi_window_parity_packet_notebook_truth_diverged_blocked,
    seeded_m5_multi_window_parity_packet_preview_routing_lost_blocked, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_MULTI_WINDOW_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_MULTI_WINDOW_PARITY_SHARED_CONTRACT_REF: &str = "shell:m5_multi_window_parity:v1";

/// Stable record kind for [`MultiWindowParityPacket`] payloads.
pub const M5_MULTI_WINDOW_PARITY_PACKET_RECORD_KIND: &str =
    "shell_m5_multi_window_parity_packet_record";

/// Stable record kind for [`MultiWindowParityDashboard`] payloads.
pub const M5_MULTI_WINDOW_PARITY_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_multi_window_parity_dashboard_record";

/// Stable record kind for [`MultiWindowParitySupportExport`] payloads.
pub const M5_MULTI_WINDOW_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_multi_window_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_MULTI_WINDOW_PARITY_PACKET_ID: &str = "m5-multi-window-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_MULTI_WINDOW_PARITY_DASHBOARD_ID: &str =
    "m5-multi-window-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_MULTI_WINDOW_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-multi-window-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_MULTI_WINDOW_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-multi-window-parity.schema.json";

/// Published markdown report ref reviewers reopen the multi-window proof from.
pub const M5_MULTI_WINDOW_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-multi-window-parity.md";

/// Published parity-packet artifact ref.
pub const M5_MULTI_WINDOW_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-multi-window-parity-proof/packet.json";

/// Published parity-dashboard artifact ref.
pub const M5_MULTI_WINDOW_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-multi-window-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_MULTI_WINDOW_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-multi-window-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_MULTI_WINDOW_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-multi-window-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_MULTI_WINDOW_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_multi_window_parity_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_MULTI_WINDOW_PARITY_MATRIX_SCHEMA_REF: &str = matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Window-topology contract this proof mirrors for detach/restore truth.
pub const M5_MULTI_WINDOW_PARITY_WINDOW_TOPOLOGY_CONTRACT_REF: &str =
    matrix::M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF;

/// Session-restore fidelity contract this proof mirrors for recovery truth.
pub const M5_MULTI_WINDOW_PARITY_SESSION_RESTORE_CONTRACT_REF: &str =
    matrix::M5_SHELL_SESSION_RESTORE_CONTRACT_REF;

/// Every governed surface family the multi-window proof must cover, in canonical order.
/// These are exactly the families the frozen shell-zone matrix freezes; the lane
/// certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// Every workspace-global continuity truth a window must preserve, in canonical order.
/// A row must declare all four; a per-window plan must preserve all four.
pub const REQUIRED_CONTINUITY_TRUTHS: [M5ContinuityTruth; 4] = M5ContinuityTruth::ALL;

/// Every owning-window routing expectation a row must declare, in canonical order.
pub const REQUIRED_ROUTING_EXPECTATIONS: [M5OwningWindowRouting; 4] = M5OwningWindowRouting::ALL;

/// The derived multi-window-parity light a governed surface family carries.
///
/// `green` means every window carrying the family's work preserves the same
/// workspace-global trust, remote, profile, and recovery-critical truth while keeping
/// layout local, routes dialogs/notifications/approvals back to the owning window and
/// object, and survives crash-restore / dependency-loss / monitor-topology drills
/// non-destructively. `yellow` is a disclosed narrowing (the family is honestly narrowed
/// below Stable, discloses a narrowed-but-visible projection of a workspace-global truth,
/// discloses a purely-local window state, relocates a routed action to a disclosed,
/// waivered still-visible affordance, or discloses a narrowed but non-destructive
/// recovery). `red` is blocked: workspace-global truth diverges across windows, a window
/// hides workspace-global risk or policy state behind a local layout choice, a routed
/// action is lost to focus theft or orphaning, a recovery drill is destructive or
/// orphaning, a required continuity truth or routing expectation is undeclared, or a
/// per-window plan drops a required truth, layout locality, or owning-window routing —
/// and it may not keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultiWindowParityStatus {
    /// Full standing: workspace-global truth preserved everywhere, layout local, routed.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl MultiWindowParityStatus {
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

/// How every window preserves the same workspace-global trust, remote, profile, and
/// recovery-critical truth.
///
/// `all_truths_preserved_in_every_window` means every window carrying the family's work
/// shows the same workspace-global trust, remote/host, profile, and recovery truth.
/// `disclosed_truth_projection_narrowing` means a detached or utility window shows a
/// narrowed-but-still-visible projection of one of those truths — a yellow narrowing.
/// `workspace_truth_diverged_across_windows` means a window shows different (or missing)
/// workspace-global truth than its peers — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityParityState {
    /// Every window preserves the same workspace-global truth.
    AllTruthsPreservedInEveryWindow,
    /// A window discloses a narrowed-but-still-visible projection of a truth.
    DisclosedTruthProjectionNarrowing,
    /// Workspace-global truth diverged across windows — a blocker.
    WorkspaceTruthDivergedAcrossWindows,
}

impl ContinuityParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllTruthsPreservedInEveryWindow => "all_truths_preserved_in_every_window",
            Self::DisclosedTruthProjectionNarrowing => "disclosed_truth_projection_narrowing",
            Self::WorkspaceTruthDivergedAcrossWindows => "workspace_truth_diverged_across_windows",
        }
    }

    /// `true` when every window preserves the same truth at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::AllTruthsPreservedInEveryWindow)
    }

    /// `true` when a window disclosed a narrowed-but-visible truth projection.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedTruthProjectionNarrowing)
    }
}

/// How layout, density, and focus stay local without hiding workspace-global risk or
/// policy state.
///
/// `layout_density_focus_local_risk_global` means layout, density, and focus can differ
/// per window while workspace-global risk and policy state stay visible everywhere they
/// apply. `disclosed_local_only_state` means a window discloses a purely-local view state
/// (a collapsed panel, a per-window density) that never hides global risk — a yellow
/// narrowing. `workspace_global_risk_hidden_locally` means a local layout choice hid
/// workspace-global risk or policy state in some window — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutLocalityState {
    /// Layout/density/focus are local while workspace-global risk stays global.
    LayoutDensityFocusLocalRiskGlobal,
    /// A window discloses a purely-local view state that never hides global risk.
    DisclosedLocalOnlyState,
    /// A local layout choice hid workspace-global risk or policy state — a blocker.
    WorkspaceGlobalRiskHiddenLocally,
}

impl LayoutLocalityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutDensityFocusLocalRiskGlobal => "layout_density_focus_local_risk_global",
            Self::DisclosedLocalOnlyState => "disclosed_local_only_state",
            Self::WorkspaceGlobalRiskHiddenLocally => "workspace_global_risk_hidden_locally",
        }
    }

    /// `true` when layout is local and workspace-global risk stays visible.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::LayoutDensityFocusLocalRiskGlobal)
    }

    /// `true` when a window disclosed a purely-local view state.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedLocalOnlyState)
    }
}

/// How dialogs, notifications, and approvals route back to the owning window and object.
///
/// `routes_to_owning_window_object` means routed actions return to the owning window and
/// object, never steal focus from an unrelated window, and never orphan on detach or
/// close. `disclosed_routing_relocation` means a routed action relocates to a disclosed,
/// waivered still-visible re-notification affordance when its owning window is not
/// present — a yellow narrowing. `routing_lost_focus_theft_or_orphan` means a routed
/// action stole focus or was orphaned — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwningWindowRoutingState {
    /// Routed actions return to the owning window and object without theft or orphaning.
    RoutesToOwningWindowObject,
    /// A routed action relocates to a disclosed, waivered still-visible affordance.
    DisclosedRoutingRelocation,
    /// A routed action stole focus or was orphaned — a blocker.
    RoutingLostFocusTheftOrOrphan,
}

impl OwningWindowRoutingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutesToOwningWindowObject => "routes_to_owning_window_object",
            Self::DisclosedRoutingRelocation => "disclosed_routing_relocation",
            Self::RoutingLostFocusTheftOrOrphan => "routing_lost_focus_theft_or_orphan",
        }
    }

    /// `true` when routed actions always return to the owning window/object.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::RoutesToOwningWindowObject)
    }

    /// `true` when a routed action took a disclosed relocation.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedRoutingRelocation)
    }
}

/// How crash-restore, dependency-loss, and monitor-topology drills behave.
///
/// `restore_dependency_topology_predictable` means crash restore, dependency loss, and
/// monitor-topology changes all produce predictable, non-destructive multi-window
/// behavior. `disclosed_recovery_narrowing` means a drill discloses a narrowed but
/// non-destructive recovery (a window recenters, a detached window rejoins the primary) —
/// a yellow narrowing. `restore_destructive_or_orphaned` means a drill dropped work,
/// orphaned a window, or diverged truth on restore — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDrillState {
    /// Crash-restore, dependency-loss, and monitor-topology drills are predictable.
    RestoreDependencyTopologyPredictable,
    /// A drill discloses a narrowed but non-destructive recovery.
    DisclosedRecoveryNarrowing,
    /// A drill dropped work, orphaned a window, or diverged truth — a blocker.
    RestoreDestructiveOrOrphaned,
}

impl RecoveryDrillState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreDependencyTopologyPredictable => "restore_dependency_topology_predictable",
            Self::DisclosedRecoveryNarrowing => "disclosed_recovery_narrowing",
            Self::RestoreDestructiveOrOrphaned => "restore_destructive_or_orphaned",
        }
    }

    /// `true` when every drill is predictable and non-destructive.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::RestoreDependencyTopologyPredictable)
    }

    /// `true` when a drill took a disclosed but non-destructive narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedRecoveryNarrowing)
    }
}

/// The continuity plan a family lands in for one window class.
///
/// The plan must preserve exactly the family's declared continuity truths, keep layout
/// local, and route back to the owning window; a dropped truth, a lost layout locality,
/// or a lost owning-window routing is a blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowContinuityPlan {
    /// The window class this plan describes.
    pub window_class: M5WindowClass,
    /// The workspace-global continuity truths this window preserves.
    pub preserved_truths: Vec<M5ContinuityTruth>,
    /// `true` when layout, density, and focus stay local to this window.
    pub layout_is_local: bool,
    /// `true` when routed actions return to the owning window and object.
    pub routes_to_owning_window: bool,
}

impl WindowContinuityPlan {
    /// `true` when the plan keeps layout local and routes to the owning window.
    pub const fn is_locally_stable(&self) -> bool {
        self.layout_is_local && self.routes_to_owning_window
    }

    /// `true` when the plan preserves exactly the given required continuity truths.
    pub fn preserves_truths(&self, required: &BTreeSet<M5ContinuityTruth>) -> bool {
        let present: BTreeSet<M5ContinuityTruth> = self.preserved_truths.iter().copied().collect();
        &present == required && present.len() == self.preserved_truths.len()
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
/// (yellow) rather than blocked — never lets diverged truth, hidden global risk, lost
/// routing, or destructive recovery hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParityWaiver {
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

impl MultiWindowParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's multi-window claim.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParityCause {
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

impl MultiWindowParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed surface family, certified across its continuity-parity, layout-locality,
/// owning-window-routing, and recovery-drill posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParityRow {
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
    /// Workspace-global continuity truths this family preserves. Pulled from the matrix.
    pub declared_continuity_truths: Vec<M5ContinuityTruth>,
    /// Owning-window routing expectations this family honors. Pulled from the matrix.
    pub declared_owning_window_routing: Vec<M5OwningWindowRouting>,
    /// Per-window continuity plan, one per declared window class.
    pub window_plans: Vec<WindowContinuityPlan>,
    /// Continuity-parity posture (same truth in every window).
    pub continuity_parity: ContinuityParityState,
    /// Layout-locality posture (local layout, global risk).
    pub layout_locality: LayoutLocalityState,
    /// Owning-window-routing posture.
    pub owning_window_routing: OwningWindowRoutingState,
    /// Recovery-drill posture (crash restore / dependency loss / monitor topology).
    pub recovery_drill: RecoveryDrillState,
    /// Consumer surfaces this family must stay aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed routing relocation narrowing is in force.
    pub active_waiver: Option<MultiWindowParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: MultiWindowParityStatus,
    /// The exact parity causes that narrowed or blocked this row.
    pub parity_causes: Vec<MultiWindowParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl MultiWindowParityRow {
    /// `true` when this family's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// Required continuity truths as a set.
    fn required_truths() -> BTreeSet<M5ContinuityTruth> {
        REQUIRED_CONTINUITY_TRUTHS.iter().copied().collect()
    }

    /// `true` when the row declares all four workspace-global continuity truths — the
    /// guarantee that every window preserves trust, remote, profile, and recovery truth.
    pub fn declared_truths_complete(&self) -> bool {
        let declared: BTreeSet<M5ContinuityTruth> =
            self.declared_continuity_truths.iter().copied().collect();
        declared == Self::required_truths()
            && declared.len() == self.declared_continuity_truths.len()
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

    /// `true` when every per-window plan preserves exactly the declared continuity truths.
    pub fn plans_preserve_all_truths(&self) -> bool {
        let declared: BTreeSet<M5ContinuityTruth> =
            self.declared_continuity_truths.iter().copied().collect();
        self.window_plans
            .iter()
            .all(|plan| plan.preserves_truths(&declared))
    }

    /// `true` when every per-window plan keeps layout local.
    pub fn plans_layout_local(&self) -> bool {
        self.window_plans.iter().all(|plan| plan.layout_is_local)
    }

    /// `true` when every per-window plan routes back to the owning window.
    pub fn plans_route_to_owning(&self) -> bool {
        self.window_plans
            .iter()
            .all(|plan| plan.routes_to_owning_window)
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.declared_truths_complete() || !self.routing_expectations_complete() {
            return true;
        }
        if matches!(
            self.continuity_parity,
            ContinuityParityState::WorkspaceTruthDivergedAcrossWindows
        ) {
            return true;
        }
        if matches!(
            self.layout_locality,
            LayoutLocalityState::WorkspaceGlobalRiskHiddenLocally
        ) {
            return true;
        }
        if matches!(
            self.owning_window_routing,
            OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan
        ) {
            return true;
        }
        if matches!(
            self.recovery_drill,
            RecoveryDrillState::RestoreDestructiveOrOrphaned
        ) {
            return true;
        }
        if !self.plans_cover_declared_window_classes()
            || !self.plans_preserve_all_truths()
            || !self.plans_layout_local()
            || !self.plans_route_to_owning()
        {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || self.continuity_parity.is_disclosed_narrowing()
            || self.layout_locality.is_disclosed_narrowing()
            || self.owning_window_routing.is_disclosed_narrowing()
            || self.recovery_drill.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the parity posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> MultiWindowParityStatus {
        if self.has_hard_blocker() {
            MultiWindowParityStatus::Red
        } else if self.has_narrowing() {
            MultiWindowParityStatus::Yellow
        } else {
            MultiWindowParityStatus::Green
        }
    }

    /// Recomputes the exact parity causes for the row, in deterministic order
    /// (qualification, truth completeness, routing completeness, continuity, layout,
    /// routing, recovery).
    pub fn recompute_causes(&self) -> Vec<MultiWindowParityCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(MultiWindowParityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen shell-zone matrix qualifies this family at `{}`, below a Stable shell claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        if !self.declared_truths_complete() {
            causes.push(MultiWindowParityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::WorkspaceTruthDivergedAcrossWindows,
                disclosed: false,
                detail: "The row does not declare all four workspace-global continuity truths \
                         (trust, remote, profile, recovery), so some window could omit one."
                    .to_owned(),
            });
        }
        if !self.routing_expectations_complete() {
            causes.push(MultiWindowParityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                disclosed: false,
                detail: "The row does not declare all four owning-window routing expectations \
                         (route-to-owning, preserve-anchor, no-focus-theft, no-orphan)."
                    .to_owned(),
            });
        }
        match self.continuity_parity {
            ContinuityParityState::AllTruthsPreservedInEveryWindow => {}
            ContinuityParityState::DisclosedTruthProjectionNarrowing => {
                causes.push(MultiWindowParityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A detached or utility window shows a narrowed-but-still-visible \
                             projection of a workspace-global truth while a peer window shows it \
                             in full."
                        .to_owned(),
                });
            }
            ContinuityParityState::WorkspaceTruthDivergedAcrossWindows => {
                causes.push(MultiWindowParityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::WorkspaceTruthDivergedAcrossWindows,
                    disclosed: false,
                    detail: "Workspace-global trust, remote, profile, or recovery truth diverged \
                             across windows carrying this family's work."
                        .to_owned(),
                });
            }
        }
        match self.layout_locality {
            LayoutLocalityState::LayoutDensityFocusLocalRiskGlobal => {}
            LayoutLocalityState::DisclosedLocalOnlyState => causes.push(MultiWindowParityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: "A window discloses a purely-local view state (per-window density or a \
                         collapsed panel) that never hides workspace-global risk or policy state."
                    .to_owned(),
            }),
            LayoutLocalityState::WorkspaceGlobalRiskHiddenLocally => {
                causes.push(MultiWindowParityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
                    disclosed: false,
                    detail:
                        "A local layout, density, or focus choice hid workspace-global risk or \
                             policy state in some window."
                            .to_owned(),
                });
            }
        }
        match self.owning_window_routing {
            OwningWindowRoutingState::RoutesToOwningWindowObject => {}
            OwningWindowRoutingState::DisclosedRoutingRelocation => {
                causes.push(MultiWindowParityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A routed dialog, notification, or approval relocates to a disclosed, \
                             waivered still-visible re-notification affordance when its owning \
                             window is not present."
                        .to_owned(),
                });
            }
            OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan => {
                causes.push(MultiWindowParityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                    disclosed: false,
                    detail: "A routed dialog, notification, or approval stole focus from an \
                             unrelated window or was orphaned on detach or close."
                        .to_owned(),
                });
            }
        }
        match self.recovery_drill {
            RecoveryDrillState::RestoreDependencyTopologyPredictable => {}
            RecoveryDrillState::DisclosedRecoveryNarrowing => causes.push(MultiWindowParityCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::SecondaryDisplayTopologyDrift,
                disclosed: true,
                detail: "A crash-restore, dependency-loss, or monitor-topology drill discloses a \
                         narrowed but non-destructive recovery (a window recenters or a detached \
                         window rejoins the primary)."
                    .to_owned(),
            }),
            RecoveryDrillState::RestoreDestructiveOrOrphaned => {
                causes.push(MultiWindowParityCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::SecondaryDisplayTopologyDrift,
                    disclosed: false,
                    detail: "A crash-restore, dependency-loss, or monitor-topology drill dropped \
                             work, orphaned a window, or diverged truth on restore."
                        .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed owning-window routing relocation may only stay yellow (rather than
    /// red) when a waiver discloses it.
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

    fn compute_findings(&self, as_of: &str) -> Vec<MultiWindowParityFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if !self.declared_truths_complete() {
            findings.push(MultiWindowParityFinding::RequiredContinuityTruthMissing {
                family: family.clone(),
            });
        }
        if !self.routing_expectations_complete() {
            findings.push(MultiWindowParityFinding::RoutingExpectationsIncomplete {
                family: family.clone(),
            });
        }
        if matches!(
            self.continuity_parity,
            ContinuityParityState::WorkspaceTruthDivergedAcrossWindows
        ) {
            findings.push(
                MultiWindowParityFinding::WorkspaceTruthDivergedAcrossWindows {
                    family: family.clone(),
                },
            );
        }
        if matches!(
            self.layout_locality,
            LayoutLocalityState::WorkspaceGlobalRiskHiddenLocally
        ) {
            findings.push(MultiWindowParityFinding::WorkspaceGlobalRiskHiddenLocally {
                family: family.clone(),
            });
        }
        if matches!(
            self.owning_window_routing,
            OwningWindowRoutingState::RoutingLostFocusTheftOrOrphan
        ) {
            findings.push(MultiWindowParityFinding::OwningWindowRoutingLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.recovery_drill,
            RecoveryDrillState::RestoreDestructiveOrOrphaned
        ) {
            findings.push(MultiWindowParityFinding::RestoreDestructiveOrOrphaned {
                family: family.clone(),
            });
        }
        if !self.plans_cover_declared_window_classes() {
            findings.push(MultiWindowParityFinding::PlanWindowCoverageMismatch {
                family: family.clone(),
            });
        }
        if !self.plans_preserve_all_truths() {
            findings.push(MultiWindowParityFinding::PlanTruthNotPreserved {
                family: family.clone(),
            });
        }
        if !self.plans_layout_local() {
            findings.push(MultiWindowParityFinding::PlanLayoutNotLocal {
                family: family.clone(),
            });
        }
        if !self.plans_route_to_owning() {
            findings.push(MultiWindowParityFinding::PlanRoutingNotPreserved {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, MultiWindowParityStatus::Green) && !self.has_reason() {
            findings.push(MultiWindowParityFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(MultiWindowParityFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(MultiWindowParityFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(MultiWindowParityFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(MultiWindowParityFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.parity_causes != self.recompute_causes() {
            findings.push(MultiWindowParityFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} continuity={} layout={} routing={} recovery={} windows={} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.continuity_parity.as_str(),
            self.layout_locality.as_str(),
            self.owning_window_routing.as_str(),
            self.recovery_drill.as_str(),
            self.window_plans.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the multi-window-parity proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum MultiWindowParityFinding {
    /// A governed surface family has no parity row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row does not declare all four workspace-global continuity truths.
    RequiredContinuityTruthMissing {
        /// The family token.
        family: String,
    },
    /// A row does not declare all four owning-window routing expectations.
    RoutingExpectationsIncomplete {
        /// The family token.
        family: String,
    },
    /// A row's workspace-global truth diverged across windows.
    WorkspaceTruthDivergedAcrossWindows {
        /// The family token.
        family: String,
    },
    /// A row hid workspace-global risk or policy state behind a local layout choice.
    WorkspaceGlobalRiskHiddenLocally {
        /// The family token.
        family: String,
    },
    /// A row lost a routed action to focus theft or orphaning.
    OwningWindowRoutingLost {
        /// The family token.
        family: String,
    },
    /// A row's recovery drill was destructive or orphaning.
    RestoreDestructiveOrOrphaned {
        /// The family token.
        family: String,
    },
    /// A row's per-window plans do not cover exactly the declared window classes.
    PlanWindowCoverageMismatch {
        /// The family token.
        family: String,
    },
    /// A row's per-window plan drops one of the declared continuity truths.
    PlanTruthNotPreserved {
        /// The family token.
        family: String,
    },
    /// A row's per-window plan loses layout locality.
    PlanLayoutNotLocal {
        /// The family token.
        family: String,
    },
    /// A row's per-window plan loses owning-window routing.
    PlanRoutingNotPreserved {
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
    /// The declared parity causes do not match the recomputed causes.
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

impl MultiWindowParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::RequiredContinuityTruthMissing { .. } => "required_continuity_truth_missing",
            Self::RoutingExpectationsIncomplete { .. } => "routing_expectations_incomplete",
            Self::WorkspaceTruthDivergedAcrossWindows { .. } => {
                "workspace_truth_diverged_across_windows"
            }
            Self::WorkspaceGlobalRiskHiddenLocally { .. } => "workspace_global_risk_hidden_locally",
            Self::OwningWindowRoutingLost { .. } => "owning_window_routing_lost",
            Self::RestoreDestructiveOrOrphaned { .. } => "restore_destructive_or_orphaned",
            Self::PlanWindowCoverageMismatch { .. } => "plan_window_coverage_mismatch",
            Self::PlanTruthNotPreserved { .. } => "plan_truth_not_preserved",
            Self::PlanLayoutNotLocal { .. } => "plan_layout_not_local",
            Self::PlanRoutingNotPreserved { .. } => "plan_routing_not_preserved",
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
            | Self::RequiredContinuityTruthMissing { family }
            | Self::RoutingExpectationsIncomplete { family }
            | Self::WorkspaceTruthDivergedAcrossWindows { family }
            | Self::WorkspaceGlobalRiskHiddenLocally { family }
            | Self::OwningWindowRoutingLost { family }
            | Self::RestoreDestructiveOrOrphaned { family }
            | Self::PlanWindowCoverageMismatch { family }
            | Self::PlanTruthNotPreserved { family }
            | Self::PlanLayoutNotLocal { family }
            | Self::PlanRoutingNotPreserved { family }
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

/// The release multi-window-parity packet shared by the shell / windowing / layout /
/// release automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParityPacket {
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
    /// The four workspace-global continuity truths every window must preserve.
    pub required_continuity_truths: Vec<String>,
    /// Per-family parity rows, in canonical order.
    pub rows: Vec<MultiWindowParityRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-parity) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<MultiWindowParityWaiver>,
    /// Every exact parity cause, in row then cause order.
    pub parity_causes: Vec<MultiWindowParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<MultiWindowParityFinding>,
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
    /// Published parity-packet ref.
    pub published_packet_ref: String,
    /// Published parity-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl MultiWindowParityPacket {
    /// Returns the parity row for `family`, if present.
    pub fn row(&self, family: M5ShellSurfaceFamily) -> Option<&MultiWindowParityRow> {
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
        for cause in &self.parity_causes {
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

    /// Projects the light parity dashboard the shell automation consumes.
    pub fn dashboard(&self) -> MultiWindowParityDashboard {
        MultiWindowParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 multi-window-parity packet serializes")
    }

    /// Deterministic, machine-readable parity CSV: one row per family naming its status,
    /// qualification, continuity/layout/routing/recovery posture, window classes, and
    /// waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,canonical_slot,fallback_slot,continuity_parity,layout_locality,owning_window_routing,recovery_drill,window_classes,waiver\n",
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
                row.continuity_parity.as_str(),
                row.layout_locality.as_str(),
                row.owning_window_routing.as_str(),
                row.recovery_drill.as_str(),
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
            "# M5 multi-window truth parity: same identity, trust, remote, profile, and recovery in every window\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_multi_window_parity`](../../crates/aureline-shell/src/m5_multi_window_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity -- markdown > \\\n  artifacts/shell/m5-multi-window-parity.md\n",
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
            "- Required continuity truths: {}\n",
            self.required_continuity_truths
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!(
            "- Green (full parity): {}\n",
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

        out.push_str("## Parity rows\n\n");
        out.push_str(
            "| Surface | Status | Qualification | Continuity | Layout locality | Routing | Recovery | Waiver |\n\
             | ------- | ------ | ------------- | ---------- | --------------- | ------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.continuity_parity.as_str(),
                row.layout_locality.as_str(),
                row.owning_window_routing.as_str(),
                row.recovery_drill.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Per-window continuity plan\n\n");
        out.push_str(
            "| Surface | Window class | Truths preserved | Layout local | Routes to owner |\n\
             | ------- | ------------ | ---------------- | ------------ | --------------- |\n",
        );
        for row in &self.rows {
            for plan in &row.window_plans {
                out.push_str(&format!(
                    "| {} | `{}` | {} | `{}` | `{}` |\n",
                    row.surface_label,
                    plan.window_class.as_str(),
                    plan.preserved_truths.len(),
                    plan.layout_is_local,
                    plan.routes_to_owning_window,
                ));
            }
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&MultiWindowParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, MultiWindowParityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed family preserves the same workspace-global truth in every window.\n\n",
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

        out.push_str("## Exact parity causes\n\n");
        if self.parity_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.parity_causes {
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_multi_window_parity_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light parity dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParityDashboardRow {
    /// The governed family.
    pub family: M5ShellSurfaceFamily,
    /// Short guarded-surface label.
    pub surface_label: String,
    /// Derived green/yellow/red status.
    pub status: MultiWindowParityStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Number of window classes this family may live in.
    pub window_class_count: usize,
    /// Continuity-parity posture.
    pub continuity_parity: ContinuityParityState,
    /// Layout-locality posture.
    pub layout_locality: LayoutLocalityState,
    /// Owning-window-routing posture.
    pub owning_window_routing: OwningWindowRoutingState,
    /// Recovery-drill posture.
    pub recovery_drill: RecoveryDrillState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light parity dashboard the shell / windowing / layout / release automation reads
/// to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParityDashboard {
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
    pub rows: Vec<MultiWindowParityDashboardRow>,
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

impl MultiWindowParityDashboard {
    /// Projects the dashboard from a parity packet.
    pub fn from_packet(packet: &MultiWindowParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| MultiWindowParityDashboardRow {
                family: row.family,
                surface_label: row.surface_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                window_class_count: row.declared_window_classes.len(),
                continuity_parity: row.continuity_parity,
                layout_locality: row.layout_locality,
                owning_window_routing: row.owning_window_routing,
                recovery_drill: row.recovery_drill,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .parity_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_MULTI_WINDOW_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_MULTI_WINDOW_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_MULTI_WINDOW_PARITY_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 multi-window-parity dashboard serializes")
    }
}

/// Support-export wrapper for the multi-window-parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiWindowParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: MultiWindowParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: MultiWindowParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl MultiWindowParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: MultiWindowParityPacket,
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
            record_kind: M5_MULTI_WINDOW_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_MULTI_WINDOW_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_MULTI_WINDOW_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_multi_window_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiWindowParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family parity rows.
    pub rows: Vec<MultiWindowParityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The parity packet carries only closed vocabulary, refs, and short labels, so raw
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

/// Builds a [`MultiWindowParityPacket`] from the exact build identity, the frozen matrix
/// ref, and the per-family parity rows.
///
/// Each row's derived status and parity causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_multi_window_parity_packet(
    input: MultiWindowParityInput,
) -> MultiWindowParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent
    // and the auto-narrowing is the single source of truth.
    let rows: Vec<MultiWindowParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.parity_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<MultiWindowParityFinding> = Vec::new();

    // Every governed family must carry a parity row.
    let present: BTreeSet<M5ShellSurfaceFamily> = rows.iter().map(|row| row.family).collect();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(MultiWindowParityFinding::FamilyMissing {
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
        .filter(|row| matches!(row.derived_status, MultiWindowParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, MultiWindowParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, MultiWindowParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(MultiWindowParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<MultiWindowParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let parity_causes: Vec<MultiWindowParityCause> = rows
        .iter()
        .flat_map(|row| row.parity_causes.clone())
        .collect();

    let required_continuity_truths: Vec<String> = REQUIRED_CONTINUITY_TRUTHS
        .iter()
        .map(|truth| truth.as_str().to_owned())
        .collect();

    let mut packet = MultiWindowParityPacket {
        record_kind: M5_MULTI_WINDOW_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_MULTI_WINDOW_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_MULTI_WINDOW_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_MULTI_WINDOW_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_MULTI_WINDOW_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Multi-window truth parity for every claimed M5 surface family: notebook, data \
                   grid, profiler, pipeline, docs, preview, review, incident, companion, and \
                   operator each certified so every window carrying its work preserves the same \
                   workspace identity, trust, remote/host, profile, and recovery-critical truth \
                   while keeping layout, density, and focus local, routes dialogs, notifications, \
                   and approvals back to the owning window and object without focus theft or \
                   orphaning, and survives crash-restore, dependency-loss, and monitor-topology \
                   drills non-destructively, with each row's green/yellow/red claim auto-narrowed \
                   from its continuity-parity, layout-locality, owning-window-routing, and \
                   recovery-drill posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_MULTI_WINDOW_PARITY_MATRIX_SCHEMA_REF.to_owned(),
        window_topology_contract_ref: M5_MULTI_WINDOW_PARITY_WINDOW_TOPOLOGY_CONTRACT_REF
            .to_owned(),
        session_restore_contract_ref: M5_MULTI_WINDOW_PARITY_SESSION_RESTORE_CONTRACT_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_continuity_truths,
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        parity_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.multi_window_parity_registry".to_owned(),
            "release_automation.auto_narrow.multi_window_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.multi_window_parity".to_owned(),
            M5_MULTI_WINDOW_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_MULTI_WINDOW_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-multi-window-parity".to_owned()],
        published_report_ref: M5_MULTI_WINDOW_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_MULTI_WINDOW_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_MULTI_WINDOW_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_MULTI_WINDOW_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("parity packet serializes"),
    ) {
        blocking_findings.push(MultiWindowParityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_multi_window_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum MultiWindowParityValidationError {
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
    /// The declared required continuity truths do not match the lane constants.
    RequiredTruthsStale,
    /// The rows do not cover all ten governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared parity causes do not match the recomputed causes.
    ParityCausesStale,
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

/// Validates a packet against the multi-window-parity invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed family
/// carries a current parity row; each row's status is the derived auto-narrowed value,
/// never asserted; a green row cannot keep a claim while its workspace-global truth
/// diverges across windows, a window hides global risk behind a local layout choice, a
/// routed action is lost to focus theft or orphaning, a recovery drill is destructive, a
/// required continuity truth or routing expectation is undeclared, or a per-window plan
/// drops a truth, layout locality, or routing; and a disclosed narrowing is backed by a
/// reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_multi_window_parity_packet(
    packet: &MultiWindowParityPacket,
) -> Result<(), Vec<MultiWindowParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(MultiWindowParityValidationError::NoRows);
    }
    if packet.record_kind != M5_MULTI_WINDOW_PARITY_PACKET_RECORD_KIND {
        errors.push(MultiWindowParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_MULTI_WINDOW_PARITY_SCHEMA_VERSION {
        errors.push(MultiWindowParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(MultiWindowParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(MultiWindowParityValidationError::MatrixPacketRefMissing);
    }
    let expected_truths: Vec<String> = REQUIRED_CONTINUITY_TRUTHS
        .iter()
        .map(|truth| truth.as_str().to_owned())
        .collect();
    if packet.required_continuity_truths != expected_truths {
        errors.push(MultiWindowParityValidationError::RequiredTruthsStale);
    }

    let present: BTreeSet<M5ShellSurfaceFamily> =
        packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = REQUIRED_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_FAMILIES.len() {
        errors.push(MultiWindowParityValidationError::CoverageIncomplete);
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
        errors.push(MultiWindowParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), MultiWindowParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), MultiWindowParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), MultiWindowParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(MultiWindowParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<MultiWindowParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(MultiWindowParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<MultiWindowParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.parity_causes {
        errors.push(MultiWindowParityValidationError::ParityCausesStale);
    }

    let mut recomputed: Vec<MultiWindowParityFinding> = Vec::new();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(MultiWindowParityFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(MultiWindowParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("parity packet serializes"),
    ) {
        recomputed.push(MultiWindowParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(MultiWindowParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(MultiWindowParityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(MultiWindowParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(MultiWindowParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(MultiWindowParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(MultiWindowParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
