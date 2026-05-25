//! Canonical stable truth model for the **settings-UI certification**: the
//! effective-configuration inspector, the setting-definition registry exposure,
//! the shadow contributor chain, scope-explicit previewable writes, the
//! profile-switch review, and cross-surface explanation parity.
//!
//! ## Why one governed certification record
//!
//! Settings are read and written from many surfaces — the desktop settings UI,
//! a CLI / headless inspect, Help/About, a diagnostics or support export, and
//! migration / import review. If each surface re-derives "what value is active,
//! which scope won, why it is locked, and whether it needs a restart" from its
//! own private read of the filesystem, the surfaces drift: the UI shows one
//! winner, the export shows another, and a policy-locked write fails with a
//! toast-only reason that support cannot reconstruct. The risk this closes: a
//! green "settings are truthful" claim that is really an average over surfaces
//! that each explain a setting a little differently, with a flat effective value
//! that hides the active profile, the temporary profile, the synced artifact,
//! and the policy ceiling behind it.
//!
//! A [`SettingsUiCertification`] mints, for one settings posture:
//!
//! - **One effective-setting record per visible setting.** Every
//!   [`EffectiveSettingRow`] resolves through a single effective-setting record
//!   that can explain what value won, which scope supplied it, why it is locked
//!   or allowed, and whether a restart or live apply applies — projected from
//!   the live [`crate::resolver`] and [`crate::inspector`], never re-derived.
//! - **A shadow contributor chain, never a flat value.** Each row's shadow chain
//!   classifies every contributor — built-in default, channel default, active
//!   profile, temporary profile, machine-local, synced, workspace, folder /
//!   language overrides, and the policy-owned ceiling — so the inspector shows
//!   the layered truth rather than implying one value.
//! - **Scope-explicit previewable writes.** Every [`PreviewableWriteRow`] names
//!   the target scope, the target artifact that would receive the write, the
//!   blocked-write reason and a Diagnostics Center entry point when denied, the
//!   restart impact, and any experiment / lifecycle dependency — before commit.
//! - **Cross-surface parity.** One [`SurfaceParityRow`] per desktop UI, CLI
//!   inspect, Help/About, diagnostics / support export, and migration / import
//!   review surface, each proving it consumes the shared setting-definition
//!   registry and effective-setting record rather than cloning prose.
//! - **A profile-switch review.** A [`ProfileSwitchReview`] summarizes the
//!   immediate changes, the restart-required deltas, the excluded machine-local
//!   state, the narrowing effects, and whether a rollback checkpoint is created
//!   before a profile switch applies.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, and support exports read this record
//! verbatim instead of cloning status text.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried in serialized certification records.
pub const SETTINGS_UI_RECORD_KIND: &str = "settings_ui_certification_record";

/// Schema version for the [`SettingsUiCertification`] payload shape.
pub const SETTINGS_UI_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const SETTINGS_UI_SHARED_CONTRACT_REF: &str = "settings:settings_ui_stable:v1";

/// Reviewer-facing notice rendered on every certification surface.
pub const SETTINGS_UI_NOTICE: &str =
    "Settings-UI certification: every visible setting on a claimed stable surface resolves through \
     one effective-setting record that explains what value won, which scope supplied it, why it is \
     locked or allowed, and whether a restart or live apply applies; the inspector exposes the \
     active profile, temporary profile, machine-local, synced, workspace, and policy-owned \
     contributors in one shadow chain rather than implying a flat value; previewable writes are \
     scope-explicit and show the target artifact, the blocked-write reason with a Diagnostics \
     Center entry point when denied, the restart impact, and any experiment or lifecycle \
     dependency before commit; the desktop UI, CLI inspect, Help/About, diagnostics or support \
     export, and migration or import review all consume the same setting-definition registry and \
     effective-setting record instead of cloning prose; a profile-switch review summarizes the \
     immediate changes, restart-required deltas, excluded machine-local state, narrowing effects, \
     and whether a rollback checkpoint is created before apply; stable setting_ids and their \
     migration aliases stay canonical in exports, CLI inspect, diagnostics, and release packets; \
     and a posture that cannot prove a pillar, or whose lowest surface marker is below Stable, \
     narrows below Stable with a named reason rather than inheriting an adjacent green row.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present ref.
const MAX_REF_CHARS: usize = 200;
/// Canonical durable-object scheme used by minted refs.
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";
/// Ref classes that are generic landing targets, not certification objects.
const GENERIC_LANDING_CLASSES: [&str; 3] = ["home", "landing", "root"];

// ---------------------------------------------------------------------------
// Shared governance vocabulary
// ---------------------------------------------------------------------------

/// Public claim class for the lane, reusing the stable lifecycle cutline.
///
/// `Stable` sits at or above the launch cutline; everything else is narrowed
/// below it. The builder *derives* this from the evidence, so a posture can
/// never publish a claim wider than its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The settings UI is replacement-grade across the claimed rows.
    Stable,
    /// Narrowed to the beta promise.
    Beta,
    /// Narrowed to the preview / limited-availability promise.
    Preview,
    /// No public promise yet.
    NotClaimed,
}

impl StableClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }
}

/// Lifecycle marker carried by a setting row or a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMarker {
    /// Preview / limited-availability.
    Preview,
    /// Beta promise.
    Beta,
    /// Replacement-grade stable.
    Stable,
}

impl LifecycleMarker {
    /// Returns the stable string vocabulary for this marker.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Returns `true` when the marker sits below the stable cutline.
    pub const fn is_below_stable(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Surface a certification can be reached from. The same record must be
/// reachable from all four so keyboard-only and assistive-technology users find
/// it consistently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteSurface {
    /// The settings effective-configuration inspector — the authoritative
    /// surface.
    SettingsInspector,
    /// The command palette.
    CommandPalette,
    /// The status bar / status overflow.
    StatusBar,
    /// An application menu command.
    MenuCommand,
}

impl RouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsInspector => "settings_inspector",
            Self::CommandPalette => "command_palette",
            Self::StatusBar => "status_bar",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a record.
    pub const REQUIRED: [Self; 4] = [
        Self::SettingsInspector,
        Self::CommandPalette,
        Self::StatusBar,
        Self::MenuCommand,
    ];
}

/// Layout mode an accessibility disclosure is checked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    /// Default desktop layout.
    Normal,
    /// High-contrast theme.
    HighContrast,
    /// Zoomed / enlarged layout.
    Zoomed,
}

impl LayoutMode {
    /// Returns the stable string vocabulary for this layout mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::HighContrast => "high_contrast",
            Self::Zoomed => "zoomed",
        }
    }

    /// The three layout modes every disclosure must hold in.
    pub const REQUIRED: [Self; 3] = [Self::Normal, Self::HighContrast, Self::Zoomed];
}

/// Role a recovery action plays, used for placement and confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionRole {
    /// Opens or focuses the authoritative effective-configuration inspector.
    Primary,
    /// Inspects or recovers the resolved settings state.
    Recovery,
    /// Non-mutating inspect / export.
    Secondary,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
        }
    }
}

/// Returns true when `reference` is a canonical object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

// ---------------------------------------------------------------------------
// Contributor classes (the shadow chain vocabulary)
// ---------------------------------------------------------------------------

/// The profile / authority class that supplied a shadow-chain contributor.
///
/// This is the v23 vocabulary the inspector exposes so a setting never implies
/// one flat value: the active profile, a temporary profile, machine-local
/// state, a synced artifact, the workspace, and the policy-owned ceiling are all
/// distinct, named contributors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributorClass {
    /// The embedded built-in default.
    BuiltInDefault,
    /// A release-channel or experiment default.
    ChannelDefault,
    /// The active profile's default layer.
    ActiveProfile,
    /// A temporary profile applied for the session.
    TemporaryProfile,
    /// Machine-local, topology-specific state (never carried by sync).
    MachineLocal,
    /// A synced user-profile artifact.
    SyncedProfile,
    /// The workspace layer.
    Workspace,
    /// A folder or module override.
    FolderOverride,
    /// A language override.
    LanguageOverride,
    /// A non-profile session override.
    SessionOverride,
    /// The policy-owned narrowing ceiling.
    PolicyOwned,
}

impl ContributorClass {
    /// Returns the stable string vocabulary for this contributor class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefault => "built_in_default",
            Self::ChannelDefault => "channel_default",
            Self::ActiveProfile => "active_profile",
            Self::TemporaryProfile => "temporary_profile",
            Self::MachineLocal => "machine_local",
            Self::SyncedProfile => "synced_profile",
            Self::Workspace => "workspace",
            Self::FolderOverride => "folder_override",
            Self::LanguageOverride => "language_override",
            Self::SessionOverride => "session_override",
            Self::PolicyOwned => "policy_owned",
        }
    }

    /// Derives a contributor class from a canonical settings-scope token.
    ///
    /// The mapping is deterministic so the certification can never invent a
    /// contributor the resolver did not actually layer.
    pub fn from_scope_token(scope_token: &str) -> Option<Self> {
        Some(match scope_token {
            "built_in_default" => Self::BuiltInDefault,
            "channel_or_experiment_default" => Self::ChannelDefault,
            "imported_profile_default" => Self::ActiveProfile,
            "user_global" => Self::SyncedProfile,
            "machine_specific" => Self::MachineLocal,
            "workspace" => Self::Workspace,
            "folder_or_module_override" => Self::FolderOverride,
            "language_override" => Self::LanguageOverride,
            "session_override" => Self::TemporaryProfile,
            "admin_policy_narrowing" => Self::PolicyOwned,
            _ => return None,
        })
    }

    /// The contributor classes the shadow chain must be able to expose so it is
    /// never a flat value, in canonical order.
    pub const REQUIRED_COVERAGE: [Self; 6] = [
        Self::ActiveProfile,
        Self::TemporaryProfile,
        Self::MachineLocal,
        Self::SyncedProfile,
        Self::Workspace,
        Self::PolicyOwned,
    ];
}

// ---------------------------------------------------------------------------
// Cross-surface parity vocabulary
// ---------------------------------------------------------------------------

/// A surface that must explain a setting from the shared record, not its own
/// prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The desktop settings UI / effective-configuration inspector.
    DesktopUi,
    /// The CLI / headless inspect command.
    CliInspect,
    /// The Help/About surface.
    HelpAbout,
    /// The diagnostics / support export.
    DiagnosticsSupportExport,
    /// The migration / import review surface.
    MigrationImportReview,
}

impl SurfaceClass {
    /// Returns the stable string vocabulary for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::DiagnosticsSupportExport => "diagnostics_support_export",
            Self::MigrationImportReview => "migration_import_review",
        }
    }

    /// The closed required surface set in canonical order.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopUi,
        Self::CliInspect,
        Self::HelpAbout,
        Self::DiagnosticsSupportExport,
        Self::MigrationImportReview,
    ];

    fn order(self) -> usize {
        Self::REQUIRED
            .iter()
            .position(|candidate| *candidate == self)
            .unwrap_or(usize::MAX)
    }
}

// ---------------------------------------------------------------------------
// Routes, recovery, accessibility
// ---------------------------------------------------------------------------

/// One recovery route exposed on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
}

/// One route to the same record from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: RouteSurface,
    /// Canonical route ref pointing at the record on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same certification record.
    pub activates_same_record: bool,
}

/// Accessibility disclosure for one layout mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutModeDisclosure {
    /// Layout mode this disclosure was checked under.
    pub mode: LayoutMode,
    /// Whether the row narration is available in this mode.
    pub row_narration_available: bool,
    /// Whether the recovery affordances stay reachable in this mode.
    pub recovery_affordances_reachable: bool,
}

/// Accessibility disclosure for the record across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the record in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the record and its actions expose.
    pub tab_stop_count: u32,
    /// Record narration read by assistive tech.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

// ---------------------------------------------------------------------------
// Effective-setting rows + shadow contributor chain
// ---------------------------------------------------------------------------

/// One contributor in a setting's shadow chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShadowContributorRow {
    /// Canonical scope token the contributor was layered at.
    pub scope: String,
    /// Profile / authority class the contributor belongs to.
    pub contributor_class: ContributorClass,
    /// Human-readable source label.
    pub source_label: String,
    /// Redacted value preview at this contributor.
    pub value_preview: String,
    /// Relation to the resolved winner (`winner`, `shadowed`, `capped`,
    /// `policy_ceiling`).
    pub relation: String,
    /// Whether this contributor is the resolved winner.
    pub winner: bool,
}

/// One certified effective-setting row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingRow {
    /// Canonical setting id (stable across label changes).
    pub setting_id: String,
    /// Declared value-type token.
    pub declared_type: String,
    /// Winning value preview or redacted summary.
    pub winning_value_summary: String,
    /// Scope token that supplied the effective value.
    pub winning_scope: String,
    /// Profile / authority class that supplied the effective value.
    pub winning_contributor: ContributorClass,
    /// Human-readable source label for the winning scope.
    pub source_label: String,
    /// The setting's own lifecycle badge (informational).
    pub setting_lifecycle: LifecycleMarker,
    /// The maturity of the settings-UI treatment of this row.
    pub surface_marker: LifecycleMarker,
    /// Allowed write scopes declared by the definition.
    pub allowed_scopes: Vec<String>,
    /// Built-in default preview.
    pub default_value_preview: String,
    /// Migration aliases that redirect to this setting id.
    pub migration_aliases: Vec<String>,
    /// Declared restart posture token.
    pub restart_posture: String,
    /// Whether the value needs a restart or reload to fully apply.
    pub restart_required: bool,
    /// Sensitivity class token.
    pub sensitivity_class: String,
    /// Redaction class token.
    pub redaction_class: String,
    /// Preview / checkpoint / approval posture token.
    pub preview_class: String,
    /// Capability / lifecycle dependency keys declared by the definition.
    pub capability_dependencies: Vec<String>,
    /// Stable help / docs reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_doc_ref: Option<String>,
    /// Lock-state token after policy and capability evaluation.
    pub lock_state: String,
    /// Lock-reason token.
    pub lock_reason: String,
    /// Canonical Diagnostics Center entry point / escalation path.
    pub escalation_path_ref: String,
    /// Ordered shadow contributor chain.
    pub shadow_chain: Vec<ShadowContributorRow>,
    /// Canonical effective-setting record ref this row resolves through.
    pub effective_record_ref: String,
    /// Whether the setting_id is canonical (not a label-only handle).
    pub setting_id_canonical: bool,
    /// Derived: the row resolves through one record that explains value,
    /// winning scope, lock, and restart.
    pub resolves_through_one_record: bool,
    /// Optional bounded waiver ref for a row narrowed below Stable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the row conforms to the stable settings-UI contract.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Previewable writes
// ---------------------------------------------------------------------------

/// One scope-explicit previewable write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewableWriteRow {
    /// Canonical setting id.
    pub setting_id: String,
    /// Scope token the write targets.
    pub target_scope: String,
    /// Profile / authority class the write targets.
    pub target_contributor: ContributorClass,
    /// Canonical artifact ref that would receive the write.
    pub target_artifact_ref: String,
    /// Whether the write stays at the selected scope with no broader fan-out.
    pub scope_explicit: bool,
    /// Write verdict token (`allowed`, `allowed_with_*`, `denied`).
    pub verdict: String,
    /// Whether the write is denied.
    pub denied: bool,
    /// Typed blocked-write reason when denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_write_reason: Option<String>,
    /// Canonical Diagnostics Center entry point for the write.
    pub diagnostics_entry_ref: String,
    /// Declared restart posture token.
    pub restart_posture: String,
    /// Human-readable restart impact.
    pub restart_impact: String,
    /// Whether a change preview must be shown before apply.
    pub preview_required: bool,
    /// Whether a rollback checkpoint is required before apply.
    pub checkpoint_required: bool,
    /// Whether an approval ticket is required before apply.
    pub approval_required: bool,
    /// Experiment / lifecycle / capability dependency that gates the write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_dependency: Option<String>,
    /// Derived: the write is scope-explicit and, when denied, names a reason and
    /// a Diagnostics Center entry point.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Cross-surface parity
// ---------------------------------------------------------------------------

/// One surface's parity with the shared record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes the shared setting-definition registry and
    /// effective-setting record.
    pub consumes_shared_record: bool,
    /// Whether the surface clones manually maintained prose (must be false).
    pub clones_prose: bool,
    /// Shared contract ref the surface ingests.
    pub shared_contract_ref: String,
    /// Canonical record ref the surface points at.
    pub record_ref: String,
    /// Optional bounded waiver ref for a surface narrowed below Stable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the surface consumes the shared record and clones no prose.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Profile-switch review
// ---------------------------------------------------------------------------

/// One setting that changes across a profile switch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchChange {
    /// Canonical setting id.
    pub setting_id: String,
    /// Effective value preview before the switch.
    pub before_value_preview: String,
    /// Effective value preview after the switch.
    pub after_value_preview: String,
    /// Whether the change needs a restart or reload to fully apply.
    pub restart_required: bool,
    /// Whether the post-switch value is narrowed by a policy ceiling.
    pub narrowed: bool,
}

/// A profile-switch review summarizing what changes before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSwitchReview {
    /// Canonical ref for the profile switched from.
    pub from_profile_ref: String,
    /// Canonical ref for the profile switched to.
    pub to_profile_ref: String,
    /// Whether the destination profile is a temporary profile.
    pub to_profile_is_temporary: bool,
    /// Changes that apply immediately (no restart).
    pub immediate_changes: Vec<ProfileSwitchChange>,
    /// Changes that require a restart or reload to fully apply.
    pub restart_required_changes: Vec<ProfileSwitchChange>,
    /// Machine-local setting ids excluded from the switch.
    pub excluded_machine_specific: Vec<String>,
    /// Setting ids narrowed by a policy ceiling after the switch.
    pub narrowing_effects: Vec<String>,
    /// Whether a rollback checkpoint is created before apply.
    pub creates_rollback_checkpoint: bool,
    /// Canonical rollback checkpoint ref, present iff one is created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Derived: the review summarizes immediate, restart, excluded, narrowing,
    /// and rollback posture coherently.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Pillars, claim ceiling, qualification, upstream
// ---------------------------------------------------------------------------

/// The derived pillar verdicts (what the posture can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPillars {
    /// Every visible setting resolves through one explaining record.
    pub every_setting_resolves_one_record: bool,
    /// The shadow chain exposes the required contributor classes.
    pub shadow_chain_exposes_contributors: bool,
    /// Every previewable write is scope-explicit and honest when denied.
    pub writes_scope_explicit: bool,
    /// Every surface consumes the shared record and clones no prose.
    pub surfaces_share_one_truth: bool,
    /// The profile-switch review summarizes the switch coherently.
    pub profile_switch_review_complete: bool,
    /// Stable setting_ids and migration aliases stay canonical in exports.
    pub setting_ids_canonical_in_exports: bool,
}

/// The public claim ceiling: what a posture is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationClaimCeiling {
    /// May claim every visible setting resolves through one record.
    pub asserts_every_setting_resolves_one_record: bool,
    /// May claim the shadow chain exposes the required contributors.
    pub asserts_shadow_chain_exposes_contributors: bool,
    /// May claim every previewable write is scope-explicit.
    pub asserts_writes_scope_explicit: bool,
    /// May claim every surface shares one truth.
    pub asserts_surfaces_share_one_truth: bool,
    /// May claim the profile-switch review is complete.
    pub asserts_profile_switch_review_complete: bool,
    /// May claim setting_ids stay canonical in exports.
    pub asserts_setting_ids_canonical: bool,
}

/// Reason a posture is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowingReason {
    /// A visible setting does not resolve through one explaining record.
    SettingResolutionIncomplete,
    /// The shadow chain does not expose the required contributor classes.
    ShadowChainFlattened,
    /// A previewable write is not scope-explicit or hides a blocked reason.
    WriteNotScopeExplicit,
    /// A surface clones prose instead of consuming the shared record.
    SurfaceClonesProse,
    /// The profile-switch review does not summarize the switch coherently.
    ProfileSwitchReviewIncomplete,
    /// A setting_id is not canonical in exports.
    SettingIdNotCanonical,
    /// The lowest surface marker is below Stable, so the posture must not inherit
    /// Stable by adjacency.
    SurfaceNotYetStable,
}

impl CertificationNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingResolutionIncomplete => "setting_resolution_incomplete",
            Self::ShadowChainFlattened => "shadow_chain_flattened",
            Self::WriteNotScopeExplicit => "write_not_scope_explicit",
            Self::SurfaceClonesProse => "surface_clones_prose",
            Self::ProfileSwitchReviewIncomplete => "profile_switch_review_incomplete",
            Self::SettingIdNotCanonical => "setting_id_not_canonical",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

/// The derived stable-claim verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationQualification {
    /// The derived claim class.
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// Reasons the posture is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<CertificationNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationUpstream {
    /// Setting-definition registry id / schema version.
    pub registry_ref: String,
    /// Resolver-state ref the effective values were resolved from.
    pub resolver_state_ref: String,
    /// Inspector shared-contract ref the records project through.
    pub inspector_contract_ref: String,
    /// Certified setting ids, sorted and deduped.
    pub certified_setting_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`SettingsUiCertification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token (the snapshot scenario).
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Effective-setting rows.
    pub effective_settings: Vec<EffectiveSettingRow>,
    /// Previewable-write rows.
    pub previewable_writes: Vec<PreviewableWriteRow>,
    /// Cross-surface parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Profile-switch review.
    pub profile_switch_review: ProfileSwitchReview,
    /// Public claim ceiling.
    pub claim_ceiling: CertificationClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the record.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the record stays available without an account.
    pub available_without_account: bool,
    /// Whether the record stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: CertificationUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed settings-UI certification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiCertification {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The lowest surface marker — the record's overall surface marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Effective-setting rows, sorted by setting id.
    pub effective_settings: Vec<EffectiveSettingRow>,
    /// Previewable-write rows, sorted by (setting id, target scope).
    pub previewable_writes: Vec<PreviewableWriteRow>,
    /// Cross-surface parity rows, in canonical surface order.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Profile-switch review.
    pub profile_switch_review: ProfileSwitchReview,
    /// Contributor classes exposed across all shadow chains, sorted.
    pub contributor_coverage: Vec<ContributorClass>,
    /// The derived pillar verdicts.
    pub pillars: CertificationPillars,
    /// The public claim ceiling.
    pub claim_ceiling: CertificationClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: CertificationQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the record.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the record stays available without an account.
    pub available_without_account: bool,
    /// Whether the record stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: CertificationUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`SettingsUiCertification`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// No effective-setting rows were supplied.
    NoEffectiveSettings,
    /// An effective-setting row was duplicated.
    DuplicateSetting { setting_id: String },
    /// No previewable-write rows were supplied.
    NoPreviewableWrites,
    /// A required surface-parity row was missing.
    SurfaceRowMissing { surface: SurfaceClass },
    /// A surface-parity row was duplicated.
    DuplicateSurfaceRow { surface: SurfaceClass },
    /// A row narrowed below Stable without a bounded waiver.
    RowNarrowedWithoutWaiver { setting_id: String },
    /// A surface narrowed below Stable without a bounded waiver.
    SurfaceNarrowedWithoutWaiver { surface: SurfaceClass },
    /// The profile-switch review declared a checkpoint without a ref (or a ref
    /// without a checkpoint).
    ProfileSwitchCheckpointMismatch,
    /// The claim ceiling asserted one-record resolution it cannot prove.
    OverclaimsResolution,
    /// The claim ceiling asserted contributor coverage it cannot prove.
    OverclaimsContributors,
    /// The claim ceiling asserted scope-explicit writes it cannot prove.
    OverclaimsWrites,
    /// The claim ceiling asserted shared-truth surfaces it cannot prove.
    OverclaimsSurfaces,
    /// The claim ceiling asserted a complete profile-switch review it cannot
    /// prove.
    OverclaimsProfileSwitch,
    /// The claim ceiling asserted canonical setting_ids it cannot prove.
    OverclaimsSettingIds,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: CertificationRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: RouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: RouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: RouteSurface },
    /// An entry route did not activate the same record.
    RouteTargetsDifferentRecord { surface: RouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The record was hidden when no account was present.
    HiddenWithoutAccount,
    /// The record was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::MissingRef { field } => write!(f, "ref `{field}` must be present"),
            Self::NoEffectiveSettings => {
                write!(f, "a settings-UI certification must certify at least one setting")
            }
            Self::DuplicateSetting { setting_id } => {
                write!(f, "effective-setting row `{setting_id}` is duplicated")
            }
            Self::NoPreviewableWrites => write!(
                f,
                "a settings-UI certification must exercise at least one previewable write"
            ),
            Self::SurfaceRowMissing { surface } => {
                write!(f, "surface-parity row `{}` is missing", surface.as_str())
            }
            Self::DuplicateSurfaceRow { surface } => {
                write!(f, "surface-parity row `{}` is duplicated", surface.as_str())
            }
            Self::RowNarrowedWithoutWaiver { setting_id } => write!(
                f,
                "effective-setting row `{setting_id}` is narrowed below Stable but carries no \
                 bounded waiver ref"
            ),
            Self::SurfaceNarrowedWithoutWaiver { surface } => write!(
                f,
                "surface-parity row `{}` is narrowed below Stable but carries no bounded waiver ref",
                surface.as_str()
            ),
            Self::ProfileSwitchCheckpointMismatch => write!(
                f,
                "profile-switch review must carry a rollback checkpoint ref iff it creates one"
            ),
            Self::OverclaimsResolution => write!(
                f,
                "claim ceiling may not assert one-record resolution when a setting does not resolve"
            ),
            Self::OverclaimsContributors => write!(
                f,
                "claim ceiling may not assert contributor coverage the shadow chains cannot prove"
            ),
            Self::OverclaimsWrites => write!(
                f,
                "claim ceiling may not assert scope-explicit writes when a write is not explicit"
            ),
            Self::OverclaimsSurfaces => write!(
                f,
                "claim ceiling may not assert shared-truth surfaces when a surface clones prose"
            ),
            Self::OverclaimsProfileSwitch => write!(
                f,
                "claim ceiling may not assert a complete profile-switch review it cannot prove"
            ),
            Self::OverclaimsSettingIds => write!(
                f,
                "claim ceiling may not assert canonical setting_ids when a setting_id is not canonical"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(f, "record must expose recovery route `{}`", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route `{action_id}` must be keyboard reachable")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "entry route surface `{}` is duplicated", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentRecord { surface } => write!(
                f,
                "entry route surface `{}` must activate the same certification record",
                surface.as_str()
            ),
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "accessibility layout mode `{}` is missing", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::HiddenWithoutAccount => write!(
                f,
                "a settings-UI certification must stay available without an account"
            ),
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a settings-UI certification must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingRef { field })
    }
}

impl SettingsUiCertification {
    /// Builds a governed certification record from validated input.
    ///
    /// The pillar verdicts are *derived* from the effective-setting rows, the
    /// shadow contributor chains, the previewable writes, the surface-parity
    /// rows, and the profile-switch review, so a record can never publish a
    /// claim wider than its proof. Structural lies (a non-canonical ref, a
    /// missing required surface, a checkpoint without a ref) are rejected
    /// outright; provable-but-imperfect postures (a surface that clones prose, a
    /// below-Stable row) are minted but narrowed below Stable with a named
    /// reason.
    pub fn build(input: CertificationInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        for (field, value) in [
            ("title", &input.title),
            ("summary", &input.summary),
            ("posture_label", &input.posture_label),
        ] {
            if !is_reviewable_sentence(value) {
                return Err(BuildError::InvalidSentence { field });
            }
        }
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_present_ref("upstream.registry_ref", &input.upstream.registry_ref)?;
        require_present_ref(
            "upstream.resolver_state_ref",
            &input.upstream.resolver_state_ref,
        )?;
        require_present_ref(
            "upstream.inspector_contract_ref",
            &input.upstream.inspector_contract_ref,
        )?;

        // --- effective-setting rows ------------------------------------------
        if input.effective_settings.is_empty() {
            return Err(BuildError::NoEffectiveSettings);
        }
        let mut seen_settings: BTreeSet<String> = BTreeSet::new();
        for row in &input.effective_settings {
            if !seen_settings.insert(row.setting_id.clone()) {
                return Err(BuildError::DuplicateSetting {
                    setting_id: row.setting_id.clone(),
                });
            }
            require_canonical_ref(
                "effective_settings.effective_record_ref",
                &row.effective_record_ref,
            )?;
            require_canonical_ref(
                "effective_settings.escalation_path_ref",
                &row.escalation_path_ref,
            )?;
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("effective_settings.waiver_ref", waiver)?;
            }
        }
        let mut effective_settings: Vec<EffectiveSettingRow> = input.effective_settings.clone();
        effective_settings.sort_by(|a, b| a.setting_id.cmp(&b.setting_id));
        let mut contributor_set: BTreeSet<ContributorClass> = BTreeSet::new();
        for row in &mut effective_settings {
            row.allowed_scopes.sort();
            row.allowed_scopes.dedup();
            row.migration_aliases.sort();
            row.migration_aliases.dedup();
            row.capability_dependencies.sort();
            row.capability_dependencies.dedup();
            for contributor in &row.shadow_chain {
                contributor_set.insert(contributor.contributor_class);
            }
            row.resolves_through_one_record = !row.winning_scope.is_empty()
                && !row.lock_state.is_empty()
                && !row.lock_reason.is_empty()
                && !row.restart_posture.is_empty()
                && !row.shadow_chain.is_empty()
                && is_canonical_object_ref(&row.effective_record_ref);
            row.conforms = row.resolves_through_one_record
                && row.setting_id_canonical
                && !row.surface_marker.is_below_stable();
            if row.surface_marker.is_below_stable() && row.waiver_ref.is_none() {
                return Err(BuildError::RowNarrowedWithoutWaiver {
                    setting_id: row.setting_id.clone(),
                });
            }
        }
        let every_setting_resolves_one_record = effective_settings
            .iter()
            .all(|row| row.resolves_through_one_record);
        let setting_ids_canonical_in_exports = effective_settings
            .iter()
            .all(|row| row.setting_id_canonical);
        let contributor_coverage: Vec<ContributorClass> = contributor_set.iter().copied().collect();
        let shadow_chain_exposes_contributors = ContributorClass::REQUIRED_COVERAGE
            .iter()
            .all(|required| contributor_set.contains(required));

        // --- previewable writes ----------------------------------------------
        if input.previewable_writes.is_empty() {
            return Err(BuildError::NoPreviewableWrites);
        }
        let mut previewable_writes: Vec<PreviewableWriteRow> = input.previewable_writes.clone();
        for row in &previewable_writes {
            require_canonical_ref(
                "previewable_writes.target_artifact_ref",
                &row.target_artifact_ref,
            )?;
            require_canonical_ref(
                "previewable_writes.diagnostics_entry_ref",
                &row.diagnostics_entry_ref,
            )?;
        }
        previewable_writes.sort_by(|a, b| {
            a.setting_id
                .cmp(&b.setting_id)
                .then(a.target_scope.cmp(&b.target_scope))
        });
        for row in &mut previewable_writes {
            let denied_is_honest = !row.denied
                || (row.blocked_write_reason.is_some()
                    && is_canonical_object_ref(&row.diagnostics_entry_ref));
            row.conforms = row.scope_explicit && denied_is_honest;
        }
        let writes_scope_explicit = previewable_writes.iter().all(|row| row.conforms);

        // --- surface parity --------------------------------------------------
        let mut seen_surfaces: BTreeSet<SurfaceClass> = BTreeSet::new();
        for row in &input.surface_parity {
            if !seen_surfaces.insert(row.surface_class) {
                return Err(BuildError::DuplicateSurfaceRow {
                    surface: row.surface_class,
                });
            }
            require_canonical_ref("surface_parity.record_ref", &row.record_ref)?;
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("surface_parity.waiver_ref", waiver)?;
            }
        }
        for required in SurfaceClass::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceRowMissing { surface: required });
            }
        }
        let mut surface_parity: Vec<SurfaceParityRow> = input.surface_parity.clone();
        surface_parity.sort_by_key(|row| row.surface_class.order());
        for row in &mut surface_parity {
            row.conforms = row.consumes_shared_record && !row.clones_prose;
        }
        let surfaces_share_one_truth = surface_parity.iter().all(|row| row.conforms);

        // --- profile-switch review -------------------------------------------
        let mut profile_switch_review = input.profile_switch_review.clone();
        require_present_ref(
            "profile_switch_review.from_profile_ref",
            &profile_switch_review.from_profile_ref,
        )?;
        require_present_ref(
            "profile_switch_review.to_profile_ref",
            &profile_switch_review.to_profile_ref,
        )?;
        if !is_reviewable_sentence(&profile_switch_review.summary) {
            return Err(BuildError::InvalidSentence {
                field: "profile_switch_review.summary",
            });
        }
        if profile_switch_review.creates_rollback_checkpoint
            != profile_switch_review.rollback_checkpoint_ref.is_some()
        {
            return Err(BuildError::ProfileSwitchCheckpointMismatch);
        }
        if let Some(checkpoint) = &profile_switch_review.rollback_checkpoint_ref {
            require_canonical_ref("profile_switch_review.rollback_checkpoint_ref", checkpoint)?;
        }
        let profile_switch_review_complete = !profile_switch_review.summary.trim().is_empty()
            && (!profile_switch_review.creates_rollback_checkpoint
                || profile_switch_review.rollback_checkpoint_ref.is_some());
        profile_switch_review.conforms = profile_switch_review_complete;

        // --- derive pillars --------------------------------------------------
        let pillars = CertificationPillars {
            every_setting_resolves_one_record,
            shadow_chain_exposes_contributors,
            writes_scope_explicit,
            surfaces_share_one_truth,
            profile_switch_review_complete,
            setting_ids_canonical_in_exports,
        };

        // --- claim ceiling: never claim what cannot be proven ----------------
        if input
            .claim_ceiling
            .asserts_every_setting_resolves_one_record
            && !every_setting_resolves_one_record
        {
            return Err(BuildError::OverclaimsResolution);
        }
        if input
            .claim_ceiling
            .asserts_shadow_chain_exposes_contributors
            && !shadow_chain_exposes_contributors
        {
            return Err(BuildError::OverclaimsContributors);
        }
        if input.claim_ceiling.asserts_writes_scope_explicit && !writes_scope_explicit {
            return Err(BuildError::OverclaimsWrites);
        }
        if input.claim_ceiling.asserts_surfaces_share_one_truth && !surfaces_share_one_truth {
            return Err(BuildError::OverclaimsSurfaces);
        }
        if input.claim_ceiling.asserts_profile_switch_review_complete
            && !profile_switch_review_complete
        {
            return Err(BuildError::OverclaimsProfileSwitch);
        }
        if input.claim_ceiling.asserts_setting_ids_canonical && !setting_ids_canonical_in_exports {
            return Err(BuildError::OverclaimsSettingIds);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in CertificationRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<RouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_record {
                return Err(BuildError::RouteTargetsDifferentRecord {
                    surface: route.surface,
                });
            }
        }
        for required in RouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- surface marker = lowest among setting + surface markers ---------
        let mut surface_markers: Vec<LifecycleMarker> = effective_settings
            .iter()
            .map(|row| row.surface_marker)
            .collect();
        for row in &surface_parity {
            if !row.conforms {
                if row.waiver_ref.is_none() {
                    return Err(BuildError::SurfaceNarrowedWithoutWaiver {
                        surface: row.surface_class,
                    });
                }
                surface_markers.push(LifecycleMarker::Beta);
            }
        }
        let surface_lifecycle_marker = surface_markers
            .into_iter()
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !every_setting_resolves_one_record {
            narrowing_reasons.push(CertificationNarrowingReason::SettingResolutionIncomplete);
        }
        if !shadow_chain_exposes_contributors {
            narrowing_reasons.push(CertificationNarrowingReason::ShadowChainFlattened);
        }
        if !writes_scope_explicit {
            narrowing_reasons.push(CertificationNarrowingReason::WriteNotScopeExplicit);
        }
        if !surfaces_share_one_truth {
            narrowing_reasons.push(CertificationNarrowingReason::SurfaceClonesProse);
        }
        if !profile_switch_review_complete {
            narrowing_reasons.push(CertificationNarrowingReason::ProfileSwitchReviewIncomplete);
        }
        if !setting_ids_canonical_in_exports {
            narrowing_reasons.push(CertificationNarrowingReason::SettingIdNotCanonical);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(CertificationNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == CertificationNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = CertificationQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present =
            !qualifies_stable || surface_lifecycle_marker.is_below_stable();

        // --- normalise upstream ----------------------------------------------
        let mut certified_setting_ids: Vec<String> = effective_settings
            .iter()
            .map(|row| row.setting_id.clone())
            .collect();
        certified_setting_ids.sort();
        certified_setting_ids.dedup();

        Ok(Self {
            record_kind: SETTINGS_UI_RECORD_KIND.to_string(),
            schema_version: SETTINGS_UI_SCHEMA_VERSION,
            notice: SETTINGS_UI_NOTICE.to_string(),
            shared_contract_ref: SETTINGS_UI_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            surface_lifecycle_marker,
            effective_settings,
            previewable_writes,
            surface_parity,
            profile_switch_review,
            contributor_coverage,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: CertificationUpstream {
                registry_ref: input.upstream.registry_ref,
                resolver_state_ref: input.upstream.resolver_state_ref,
                inspector_contract_ref: input.upstream.inspector_contract_ref,
                certified_setting_ids,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("settings_ui_certification: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: resolves_one_record={} contributors={} writes_explicit={} \
                 surfaces_one_truth={} profile_switch={} setting_ids_canonical={}",
                self.pillars.every_setting_resolves_one_record,
                self.pillars.shadow_chain_exposes_contributors,
                self.pillars.writes_scope_explicit,
                self.pillars.surfaces_share_one_truth,
                self.pillars.profile_switch_review_complete,
                self.pillars.setting_ids_canonical_in_exports
            ),
            format!(
                "contributor_coverage: [{}]",
                self.contributor_coverage
                    .iter()
                    .map(|class| class.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ];
        lines.push("effective_settings:".to_string());
        for row in &self.effective_settings {
            lines.push(format!(
                "  - {} type={} value={} scope={} contributor={} lock={}/{} restart={} \
                 setting_lifecycle={} surface={} conforms={}",
                row.setting_id,
                row.declared_type,
                row.winning_value_summary,
                row.winning_scope,
                row.winning_contributor.as_str(),
                row.lock_state,
                row.lock_reason,
                row.restart_posture,
                row.setting_lifecycle.as_str(),
                row.surface_marker.as_str(),
                row.conforms
            ));
            for contributor in &row.shadow_chain {
                lines.push(format!(
                    "      · {} [{}] {} = {} ({})",
                    contributor.scope,
                    contributor.contributor_class.as_str(),
                    contributor.source_label,
                    contributor.value_preview,
                    contributor.relation
                ));
            }
        }
        lines.push("previewable_writes:".to_string());
        for row in &self.previewable_writes {
            lines.push(format!(
                "  - {} -> {} [{}] verdict={} denied={} blocked={:?} restart={} explicit={} conforms={}",
                row.setting_id,
                row.target_scope,
                row.target_contributor.as_str(),
                row.verdict,
                row.denied,
                row.blocked_write_reason,
                row.restart_posture,
                row.scope_explicit,
                row.conforms
            ));
        }
        lines.push("surface_parity:".to_string());
        for row in &self.surface_parity {
            lines.push(format!(
                "  - {} consumes_shared_record={} clones_prose={} conforms={}",
                row.surface_class.as_str(),
                row.consumes_shared_record,
                row.clones_prose,
                row.conforms
            ));
        }
        lines.push("profile_switch_review:".to_string());
        lines.push(format!(
            "  from={} to={} temporary={} immediate={} restart_required={} excluded_machine={} narrowing={} checkpoint={}",
            self.profile_switch_review.from_profile_ref,
            self.profile_switch_review.to_profile_ref,
            self.profile_switch_review.to_profile_is_temporary,
            self.profile_switch_review.immediate_changes.len(),
            self.profile_switch_review.restart_required_changes.len(),
            self.profile_switch_review.excluded_machine_specific.len(),
            self.profile_switch_review.narrowing_effects.len(),
            self.profile_switch_review.creates_rollback_checkpoint
        ));
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

// ---------------------------------------------------------------------------
// Recovery vocabulary
// ---------------------------------------------------------------------------

/// Closed recovery-action vocabulary exposed on a certification record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationRecoveryAction {
    /// Open the effective-configuration inspector — the authoritative surface.
    OpenEffectiveConfigInspector,
    /// Inspect a setting's shadow contributor chain.
    InspectSettingShadowChain,
    /// Preview a scope-explicit write before commit.
    PreviewScopedWrite,
    /// Export a redacted settings support packet.
    ExportSettingsSupport,
}

impl CertificationRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenEffectiveConfigInspector => "open_effective_config_inspector",
            Self::InspectSettingShadowChain => "inspect_setting_shadow_chain",
            Self::PreviewScopedWrite => "preview_scoped_write",
            Self::ExportSettingsSupport => "export_settings_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenEffectiveConfigInspector => "Open effective-configuration inspector",
            Self::InspectSettingShadowChain => "Inspect shadow chain",
            Self::PreviewScopedWrite => "Preview scoped write",
            Self::ExportSettingsSupport => "Export settings support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenEffectiveConfigInspector => RecoveryActionRole::Primary,
            Self::InspectSettingShadowChain | Self::PreviewScopedWrite => {
                RecoveryActionRole::Recovery
            }
            Self::ExportSettingsSupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every record must expose, in rendered order.
    pub const REQUIRED: [Self; 4] = [
        Self::OpenEffectiveConfigInspector,
        Self::InspectSettingShadowChain,
        Self::PreviewScopedWrite,
        Self::ExportSettingsSupport,
    ];

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery routes every record must expose, in rendered order.
pub fn required_recovery_routes() -> Vec<RecoveryRouteRecord> {
    CertificationRecoveryAction::REQUIRED
        .into_iter()
        .map(CertificationRecoveryAction::route)
        .collect()
}
