//! Canonical stable truth model for Start Center, recent-work, and workspace
//! switcher target-kind disclosure.
//!
//! ## Why one disclosure record per entry target
//!
//! The no-workspace Start Center, the in-workspace switcher, and the recent-work
//! list are three surfaces that all answer the same question before the user
//! commits: *what will open, can I trust it, how much will restore, and what can
//! I do if it is not reachable?* When each surface answers that question with its
//! own bespoke status text, they drift — a switcher row claims a remote is
//! reachable while the Start Center row already shows it as disconnected, a moved
//! folder renders as an ordinary local open, or a managed workspace implies a
//! restore fidelity the product cannot prove.
//!
//! This module mints one governed [`EntryTargetDisclosureRecord`] per entry
//! target. The record binds a single canonical recent-work identity to the
//! target-kind disclosure, trust posture, restore availability, recovery routes,
//! cross-surface parity, command-palette / menu route parity, and the
//! accessibility narration that the Start Center, recent-work list, workspace
//! switcher, command palette, menus, diagnostics, support exports, Help/About,
//! and docs all read verbatim instead of cloning status text.
//!
//! The target-kind vocabulary, trust posture, restore availability, failure
//! taxonomy, and recovery actions are **not** reinvented here: they are the
//! canonical [`aureline_workspace`] recent-work types, so there is no parallel
//! model.
//!
//! The record is the canonical truth source for this lane (suggested-output stem
//! `stabilize-the-start-center-recent-work-list-workspace`); its boundary schema
//! is `schemas/ux/stabilize-the-start-center-recent-work-list-workspace.schema.json`
//! and its contract narrative is
//! `docs/ux/m4/stabilize-the-start-center-recent-work-list-workspace.md`.
//!
//! ## The honesty invariants
//!
//! The builder refuses to mint a record that would lie. Each is a [`BuildError`],
//! not a warning, so a dishonest projection fails the row instead of shipping:
//!
//! - **No claim the product cannot prove.** A claim ceiling may not assert a live
//!   open for an unavailable target, remote availability for a disconnected
//!   remote, full restore fidelity beyond what restore says, or trust for a row
//!   that is not trusted.
//! - **Recovery before commit.** A non-ready target must expose the recovery
//!   routes its failure state requires (Locate, Reconnect / Reauthorize, an
//!   open-minimal route where safe, and Remove from list).
//! - **Never silently discard a stale entry.** A failed open keeps the row and
//!   its recovery routes; `Remove from list` is scoped to recent-work metadata
//!   only and preserves unrelated durable state.
//! - **One model across surfaces.** The Start Center and switcher projections of
//!   a target share its identity and recovery behaviors, and the switcher keeps a
//!   cancel / reopen-previous return path.
//! - **Same routes everywhere.** The same target is reachable from the Start
//!   Center, the switcher, the command palette, and a menu command, each
//!   keyboard-reachable and pointing at the same target.
//! - **Accessible in every layout.** Tab order, row narration, action labels, and
//!   recovery affordances are present and reachable in normal, high-contrast, and
//!   zoomed layouts, and the narration discloses the target kind.
//! - **No local-open path buried behind account or managed services.** Every row
//!   stays available without an account and without managed services; absent
//!   identity or services degrade a row's state, they never hide it.

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    is_remote_backed_target, RecentWorkFailureState, RecentWorkListSection, RestoreAvailability,
    SafeRecoveryAction, TargetKind, TrustState,
};

use crate::restore::placeholders::WorkspaceSwitchRecoveryAction;

/// Stable record-kind tag carried in serialized disclosure records.
pub const ENTRY_TARGET_DISCLOSURE_RECORD_KIND: &str = "entry_target_disclosure_record";

/// Schema version for the [`EntryTargetDisclosureRecord`] payload shape.
pub const ENTRY_TARGET_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const ENTRY_TARGET_DISCLOSURE_SHARED_CONTRACT_REF: &str =
    "shell:entry_target_disclosure_stable:v1";

/// Reviewer-facing notice rendered on every disclosure surface.
pub const ENTRY_TARGET_DISCLOSURE_NOTICE: &str =
    "Entry-target disclosure truth: the Start Center, recent-work list, and workspace switcher \
     show the same canonical target kind, trust posture, restore availability, and recovery \
     routes before the user commits; an unavailable target keeps its row and its Locate, \
     Reconnect, open-minimal, and Remove-from-list routes instead of being silently discarded; \
     no row claims trust, restore fidelity, or remote availability the product cannot prove; the \
     same target opens from the Start Center, switcher, command palette, and menus; and every row \
     stays available without an account or managed services. Shell, diagnostics, support exports, \
     Help/About, and docs read this record verbatim.";

/// Canonical durable-object URI scheme. Every recent-work, route, diagnostics,
/// support-export, evidence, and narrative ref must be one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one is rejected so chrome cannot
/// wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home", "dashboard", "landing", "index", "overview", "start", "root",
];

/// Returns true when `reference` is a canonical durable-object ref of the form
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

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

/// Coarse target class used to keep the claimed stable matrix honest across
/// local, remote-backed, and managed examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetClass {
    /// A local file, folder, repository, workspace, workset, or imported package.
    Local,
    /// A remote-backed target (remote repository, SSH, container, dev container).
    RemoteBacked,
    /// A managed cloud workspace gated behind managed services.
    Managed,
}

impl TargetClass {
    /// Returns the stable string vocabulary for this target class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::RemoteBacked => "remote_backed",
            Self::Managed => "managed",
        }
    }

    /// Classifies a canonical target kind into its matrix class.
    pub const fn from_target_kind(target_kind: TargetKind) -> Self {
        match target_kind {
            TargetKind::ManagedCloudWorkspace => Self::Managed,
            _ if is_remote_backed_target(target_kind) => Self::RemoteBacked,
            _ => Self::Local,
        }
    }
}

/// What a recent-work subtitle discloses about the target's location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubtitleKind {
    /// A local filesystem path or mount.
    Path,
    /// A remote host or endpoint.
    Host,
    /// A managed provider or account scope.
    Provider,
    /// A target-kind-only subtitle with no location detail.
    TargetOnly,
    /// No subtitle is shown.
    None,
}

impl SubtitleKind {
    /// Returns the stable string vocabulary for this subtitle kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Host => "host",
            Self::Provider => "provider",
            Self::TargetOnly => "target_only",
            Self::None => "none",
        }
    }
}

/// Surface a target can be reached from. The same target must be reachable from
/// all four so no-workspace entry and in-workspace switching stay consistent for
/// keyboard-only and assistive-technology users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryRouteSurface {
    /// First-run / no-workspace Start Center.
    StartCenter,
    /// In-workspace workspace switcher.
    WorkspaceSwitcher,
    /// Command palette.
    CommandPalette,
    /// Application menu command.
    MenuCommand,
}

impl EntryRouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::WorkspaceSwitcher => "workspace_switcher",
            Self::CommandPalette => "command_palette",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a target.
    pub const REQUIRED: [Self; 4] = [
        Self::StartCenter,
        Self::WorkspaceSwitcher,
        Self::CommandPalette,
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
    /// Activates or opens the row.
    Primary,
    /// Repairs or revalidates an unavailable target.
    Recovery,
    /// Non-destructive row management.
    Secondary,
    /// Removes only recent-work metadata after confirmation.
    DestructiveMetadataOnly,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
            Self::DestructiveMetadataOnly => "destructive_metadata_only",
        }
    }

    /// Returns the role for a canonical safe recovery action.
    pub const fn from_action(action: SafeRecoveryAction) -> Self {
        match action {
            SafeRecoveryAction::Open
            | SafeRecoveryAction::OpenInNewWindow
            | SafeRecoveryAction::OpenRestricted
            | SafeRecoveryAction::OpenReadOnlyCachedView
            | SafeRecoveryAction::OpenWithoutRestore => Self::Primary,
            SafeRecoveryAction::LocateMissingTarget
            | SafeRecoveryAction::Reconnect
            | SafeRecoveryAction::Reauth
            | SafeRecoveryAction::RetryLater
            | SafeRecoveryAction::CompareBeforeRestore => Self::Recovery,
            SafeRecoveryAction::RemoveFromRecents => Self::DestructiveMetadataOnly,
            SafeRecoveryAction::Unpin
            | SafeRecoveryAction::Pin
            | SafeRecoveryAction::RevealInExplorer => Self::Secondary,
        }
    }
}

/// One recovery route exposed on a row before the user commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the user must confirm before the action mutates metadata.
    pub requires_confirmation: bool,
    /// Whether the action is guaranteed to affect recent-work metadata only.
    pub metadata_only_cleanup: bool,
    /// Whether the action leaves unrelated durable state intact.
    pub preserves_unrelated_state: bool,
}

impl RecoveryRouteRecord {
    /// Builds a route record from a canonical safe recovery action.
    pub fn from_action(action: SafeRecoveryAction) -> Self {
        let is_remove = action == SafeRecoveryAction::RemoveFromRecents;
        Self {
            action_id: action.as_str().to_string(),
            action_label: action.surface_label().to_string(),
            action_role: RecoveryActionRole::from_action(action),
            requires_confirmation: is_remove,
            metadata_only_cleanup: is_remove,
            preserves_unrelated_state: true,
        }
    }
}

/// One route to the same target from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: EntryRouteSurface,
    /// Canonical route ref pointing at the target on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same canonical target identity.
    pub activates_same_target: bool,
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

/// Accessibility disclosure for one row across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the row in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the row and its actions expose.
    pub tab_stop_count: u32,
    /// Row narration read by assistive technology; discloses the target kind.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

/// The public claim ceiling: what a row is allowed to assert. Each field must be
/// provable from the row's real state; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PublicClaimCeiling {
    /// Whether the row may claim the target opens normally now.
    pub asserts_live_open: bool,
    /// Whether the row may claim the remote / managed target is reachable now.
    pub asserts_remote_available: bool,
    /// Whether the row may claim full (exact) restore fidelity.
    pub asserts_full_restore_fidelity: bool,
    /// Whether the row may claim the target is trusted without re-evaluation.
    pub asserts_trusted_without_evaluation: bool,
}

/// Which facts a row discloses before the user commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureFacts {
    /// Discloses the canonical target kind.
    pub discloses_target_kind: bool,
    /// Discloses the path / host / provider subtitle (or its absence).
    pub discloses_subtitle: bool,
    /// Discloses the last-opened time.
    pub discloses_last_opened: bool,
    /// Discloses the trust posture.
    pub discloses_trust_state: bool,
    /// Discloses the restore availability.
    pub discloses_restore_availability: bool,
    /// Discloses the recovery routes available before activation.
    pub discloses_recovery_routes: bool,
}

/// Cross-surface parity between the Start Center and switcher projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParity {
    /// Start Center row id for this target.
    pub start_center_row_id: String,
    /// Workspace switcher row id for this target.
    pub workspace_switcher_row_id: String,
    /// Switcher entry classes (local / remote / managed / pinned / recent / ...).
    pub switcher_entry_classes: Vec<String>,
    /// Switch-failure return-path tokens preserved by the switcher.
    pub switch_failure_actions: Vec<String>,
    /// Recovery action ids shared by both surfaces.
    pub recovery_action_ids: Vec<String>,
    /// Whether the two projections agree on identity and recovery behavior.
    pub parity_holds: bool,
}

/// Validated input used to mint an [`EntryTargetDisclosureRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryTargetDisclosureInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Canonical recent-work entry ref.
    pub recent_work_ref: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Canonical target kind.
    pub target_kind: TargetKind,
    /// Raw target state.
    pub target_state: aureline_workspace::RecentWorkTargetState,
    /// Shared failure-state taxonomy.
    pub failure_state: RecentWorkFailureState,
    /// Workspace trust posture.
    pub trust_state: TrustState,
    /// Restore availability before activation.
    pub restore_availability: RestoreAvailability,
    /// Pinned or recent section.
    pub list_section: RecentWorkListSection,
    /// Path / host / provider subtitle, when shown.
    pub location_subtitle: Option<String>,
    /// What the subtitle discloses.
    pub subtitle_kind: SubtitleKind,
    /// Last-opened timestamp.
    pub last_opened_at: String,
    /// Whether the row is pinned.
    pub pinned: bool,
    /// Public claim ceiling for this row.
    pub claim_ceiling: PublicClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Whether a failed open discards the stale entry (must be false).
    pub discards_stale_entry_on_failure: bool,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same target.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the row stays available without an account.
    pub available_without_account: bool,
    /// Whether the row stays available without managed services.
    pub available_without_managed_services: bool,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed entry-target disclosure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryTargetDisclosureRecord {
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
    /// Canonical recent-work entry ref.
    pub recent_work_ref: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Canonical target kind.
    pub target_kind: TargetKind,
    /// Compact target-kind label (the vocabulary docs / Help/About ingest).
    pub target_kind_label: String,
    /// Coarse target class for the claimed stable matrix.
    pub target_class: TargetClass,
    /// Raw target state.
    pub target_state: aureline_workspace::RecentWorkTargetState,
    /// Shared failure-state taxonomy.
    pub failure_state: RecentWorkFailureState,
    /// Workspace trust posture.
    pub trust_state: TrustState,
    /// Restore availability before activation.
    pub restore_availability: RestoreAvailability,
    /// Path / host / provider subtitle, when shown.
    pub location_subtitle: Option<String>,
    /// What the subtitle discloses.
    pub subtitle_kind: SubtitleKind,
    /// Last-opened timestamp.
    pub last_opened_at: String,
    /// Whether the row is pinned.
    pub pinned: bool,
    /// Pinned or recent section.
    pub list_section: RecentWorkListSection,
    /// Which facts the row discloses before commit.
    pub disclosure: DisclosureFacts,
    /// Public claim ceiling.
    pub claim_ceiling: PublicClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Whether a failed open discards the stale entry (always false on a record).
    pub discards_stale_entry_on_failure: bool,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same target.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the row stays available without an account.
    pub available_without_account: bool,
    /// Whether the row stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed to disclose (non-ready failure,
    /// non-trusted posture, or less-than-exact restore).
    pub honesty_marker_present: bool,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons an [`EntryTargetDisclosureRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// The claim ceiling asserted a live open for an unavailable target.
    OverclaimsLiveOpen,
    /// The claim ceiling asserted remote availability for a disconnected remote.
    OverclaimsRemoteAvailable,
    /// The claim ceiling asserted a restore fidelity the row cannot prove.
    OverclaimsRestoreFidelity,
    /// The claim ceiling asserted trust the row cannot prove.
    OverclaimsTrust,
    /// A required recovery route for the failure state was missing.
    MissingRecoveryRoute {
        failure_state: RecentWorkFailureState,
        action: SafeRecoveryAction,
    },
    /// A failed open would silently discard the stale entry.
    SilentlyDiscardsStaleEntry,
    /// A removal route was not scoped to recent-work metadata only.
    RemovalNotMetadataOnly { action_id: String },
    /// A removal or recovery route dropped unrelated durable state.
    RecoveryDropsUnrelatedState { action_id: String },
    /// The two surface projections disagreed on identity or recovery behavior.
    SurfaceParityBroken,
    /// The switcher dropped a cancel / reopen-previous return path.
    SwitcherMissingReturnPath { action: WorkspaceSwitchRecoveryAction },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: EntryRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: EntryRouteSurface },
    /// An entry route did not activate the same canonical target.
    RouteTargetsDifferentTarget { surface: EntryRouteSurface },
    /// An entry-route ref was duplicated across surfaces.
    DuplicateRouteSurface { surface: EntryRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The row narration did not disclose the target kind.
    NarrationOmitsTargetKind,
    /// A row was hidden when no account was present.
    HiddenWithoutAccount,
    /// A row was hidden when managed services were absent.
    HiddenWithoutManagedServices,
    /// The subtitle kind was inconsistent with the presence of a subtitle.
    SubtitleKindMismatch,
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
            Self::OverclaimsLiveOpen => write!(
                f,
                "claim ceiling may not assert a live open for an unavailable target"
            ),
            Self::OverclaimsRemoteAvailable => write!(
                f,
                "claim ceiling may not assert remote availability for a disconnected target"
            ),
            Self::OverclaimsRestoreFidelity => write!(
                f,
                "claim ceiling may not assert full restore fidelity beyond what restore proves"
            ),
            Self::OverclaimsTrust => write!(
                f,
                "claim ceiling may not assert trust for a row that is not trusted"
            ),
            Self::MissingRecoveryRoute {
                failure_state,
                action,
            } => write!(
                f,
                "failure state `{}` must expose recovery route `{}`",
                failure_state.as_str(),
                action.as_str()
            ),
            Self::SilentlyDiscardsStaleEntry => {
                write!(f, "a failed open must keep the stale entry, not discard it")
            }
            Self::RemovalNotMetadataOnly { action_id } => write!(
                f,
                "removal route `{action_id}` must be scoped to recent-work metadata only"
            ),
            Self::RecoveryDropsUnrelatedState { action_id } => write!(
                f,
                "recovery route `{action_id}` must preserve unrelated durable state"
            ),
            Self::SurfaceParityBroken => write!(
                f,
                "Start Center and switcher projections must share identity and recovery behavior"
            ),
            Self::SwitcherMissingReturnPath { action } => write!(
                f,
                "switcher must preserve return path `{}`",
                action.as_str()
            ),
            Self::RouteSurfaceMissing { surface } => write!(
                f,
                "entry route surface `{}` is missing",
                surface.as_str()
            ),
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentTarget { surface } => write!(
                f,
                "entry route surface `{}` must activate the same target",
                surface.as_str()
            ),
            Self::DuplicateRouteSurface { surface } => write!(
                f,
                "entry route surface `{}` is duplicated",
                surface.as_str()
            ),
            Self::AccessibilityLayoutModeMissing { mode } => write!(
                f,
                "accessibility layout mode `{}` is missing",
                mode.as_str()
            ),
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::NarrationOmitsTargetKind => {
                write!(f, "row narration must disclose the target kind")
            }
            Self::HiddenWithoutAccount => {
                write!(f, "a recent-work row must stay available without an account")
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a recent-work row must stay available without managed services"
            ),
            Self::SubtitleKindMismatch => write!(
                f,
                "subtitle kind must be consistent with the presence of a subtitle"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl EntryTargetDisclosureRecord {
    /// Builds a governed disclosure record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about availability, trust, restore fidelity, recovery, cross-surface
    /// parity, route reachability, or accessibility.
    pub fn build(input: EntryTargetDisclosureInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        require_ref("recent_work_ref", &input.recent_work_ref)?;
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_ref("narrative_refs", narrative)?;
        }

        let target_kind_label = input.target_kind.surface_label().to_string();
        let target_class = TargetClass::from_target_kind(input.target_kind);
        let is_ready = input.failure_state == RecentWorkFailureState::Ready;

        // --- claim ceiling: never claim what the product cannot prove ---------
        if input.claim_ceiling.asserts_live_open && !is_ready {
            return Err(BuildError::OverclaimsLiveOpen);
        }
        if input.claim_ceiling.asserts_remote_available
            && (input.failure_state == RecentWorkFailureState::ReconnectRequired
                || !is_remote_backed_target(input.target_kind))
        {
            return Err(BuildError::OverclaimsRemoteAvailable);
        }
        if input.claim_ceiling.asserts_full_restore_fidelity
            && input.restore_availability != RestoreAvailability::Exact
        {
            return Err(BuildError::OverclaimsRestoreFidelity);
        }
        if input.claim_ceiling.asserts_trusted_without_evaluation
            && input.trust_state != TrustState::Trusted
        {
            return Err(BuildError::OverclaimsTrust);
        }

        // --- subtitle consistency --------------------------------------------
        let subtitle_present = input.location_subtitle.is_some();
        let subtitle_kind_none = input.subtitle_kind == SubtitleKind::None;
        if subtitle_present == subtitle_kind_none {
            return Err(BuildError::SubtitleKindMismatch);
        }

        // --- never silently discard a stale entry ----------------------------
        if input.discards_stale_entry_on_failure {
            return Err(BuildError::SilentlyDiscardsStaleEntry);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(input.failure_state) {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute {
                    failure_state: input.failure_state,
                    action: required,
                });
            }
        }
        for route in &input.recovery_routes {
            if route.action_id == SafeRecoveryAction::RemoveFromRecents.as_str()
                && !route.metadata_only_cleanup
            {
                return Err(BuildError::RemovalNotMetadataOnly {
                    action_id: route.action_id.clone(),
                });
            }
            if !route.preserves_unrelated_state {
                return Err(BuildError::RecoveryDropsUnrelatedState {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- cross-surface parity --------------------------------------------
        if !input.surfaces.parity_holds {
            return Err(BuildError::SurfaceParityBroken);
        }
        let parity_ids: Vec<&str> = input
            .surfaces
            .recovery_action_ids
            .iter()
            .map(String::as_str)
            .collect();
        if parity_ids != route_ids {
            return Err(BuildError::SurfaceParityBroken);
        }
        for required in [
            WorkspaceSwitchRecoveryAction::CancelSwitch,
            WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace,
        ] {
            if !input
                .surfaces
                .switch_failure_actions
                .iter()
                .any(|action| action == required.as_str())
            {
                return Err(BuildError::SwitcherMissingReturnPath { action: required });
            }
        }

        // --- route parity across surfaces ------------------------------------
        let mut seen_surfaces = Vec::new();
        for route in &input.routes {
            if seen_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_surfaces.push(route.surface);
            require_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_target {
                return Err(BuildError::RouteTargetsDifferentTarget {
                    surface: route.surface,
                });
            }
        }
        for required in EntryRouteSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
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
        if !input
            .accessibility
            .row_narration
            .contains(&target_kind_label)
        {
            return Err(BuildError::NarrationOmitsTargetKind);
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

        // --- availability: never bury a row behind account or services -------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        let disclosure = DisclosureFacts {
            discloses_target_kind: true,
            discloses_subtitle: true,
            discloses_last_opened: !input.last_opened_at.trim().is_empty(),
            discloses_trust_state: true,
            discloses_restore_availability: true,
            discloses_recovery_routes: !input.recovery_routes.is_empty(),
        };
        if !disclosure.discloses_last_opened {
            return Err(BuildError::InvalidSentence {
                field: "last_opened_at",
            });
        }

        let honesty_marker_present = !is_ready
            || input.trust_state != TrustState::Trusted
            || input.restore_availability != RestoreAvailability::Exact;

        Ok(Self {
            record_kind: ENTRY_TARGET_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: ENTRY_TARGET_DISCLOSURE_SCHEMA_VERSION,
            notice: ENTRY_TARGET_DISCLOSURE_NOTICE.to_string(),
            shared_contract_ref: ENTRY_TARGET_DISCLOSURE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            recent_work_ref: input.recent_work_ref,
            title: input.title,
            summary: input.summary,
            target_kind: input.target_kind,
            target_kind_label,
            target_class,
            target_state: input.target_state,
            failure_state: input.failure_state,
            trust_state: input.trust_state,
            restore_availability: input.restore_availability,
            location_subtitle: input.location_subtitle,
            subtitle_kind: input.subtitle_kind,
            last_opened_at: input.last_opened_at,
            pinned: input.pinned,
            list_section: input.list_section,
            disclosure,
            claim_ceiling: input.claim_ceiling,
            recovery_routes: input.recovery_routes,
            discards_stale_entry_on_failure: false,
            surfaces: input.surfaces,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("entry_target_disclosure: {}", self.record_id),
            format!("recent_work_ref: {}", self.recent_work_ref),
            format!("as_of: {}", self.as_of),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "target: {} ({}) class={}",
                self.target_kind.as_str(),
                self.target_kind_label,
                self.target_class.as_str()
            ),
            format!(
                "state: target_state={} failure_state={} trust={} restore={}",
                self.target_state.as_str(),
                self.failure_state.as_str(),
                self.trust_state.as_str(),
                self.restore_availability.as_str()
            ),
            format!(
                "subtitle: {} [{}]",
                self.location_subtitle.as_deref().unwrap_or("(none)"),
                self.subtitle_kind.as_str()
            ),
            format!(
                "claim_ceiling: live_open={} remote_available={} full_restore={} trusted={}",
                self.claim_ceiling.asserts_live_open,
                self.claim_ceiling.asserts_remote_available,
                self.claim_ceiling.asserts_full_restore_fidelity,
                self.claim_ceiling.asserts_trusted_without_evaluation
            ),
        ];
        lines.push("recovery_routes:".to_string());
        for route in &self.recovery_routes {
            lines.push(format!(
                "  - {} ({}) role={} metadata_only={} preserves_unrelated={}",
                route.action_id,
                route.action_label,
                route.action_role.as_str(),
                route.metadata_only_cleanup,
                route.preserves_unrelated_state
            ));
        }
        lines.push(format!(
            "discards_stale_entry_on_failure: {}",
            self.discards_stale_entry_on_failure
        ));
        lines.push(format!(
            "surfaces: start_center={} switcher={} parity_holds={} classes=[{}] return_paths=[{}]",
            self.surfaces.start_center_row_id,
            self.surfaces.workspace_switcher_row_id,
            self.surfaces.parity_holds,
            self.surfaces.switcher_entry_classes.join(", "),
            self.surfaces.switch_failure_actions.join(", ")
        ));
        lines.push("routes:".to_string());
        for route in &self.routes {
            lines.push(format!(
                "  - {} -> {} keyboard={} same_target={}",
                route.surface.as_str(),
                route.route_ref,
                route.keyboard_reachable,
                route.activates_same_target
            ));
        }
        lines.push(format!(
            "accessibility: tab_order={} tab_stops={} narration={:?}",
            self.accessibility.focus_order_index,
            self.accessibility.tab_stop_count,
            self.accessibility.row_narration
        ));
        for mode in &self.accessibility.layout_modes {
            lines.push(format!(
                "  layout {} narration={} affordances_reachable={}",
                mode.mode.as_str(),
                mode.row_narration_available,
                mode.recovery_affordances_reachable
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!("diagnostics_export_ref: {}", self.diagnostics_export_ref));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

/// Returns the recovery actions a failure state must expose before commit.
pub fn required_recovery_actions(failure_state: RecentWorkFailureState) -> Vec<SafeRecoveryAction> {
    match failure_state {
        RecentWorkFailureState::Ready => vec![SafeRecoveryAction::Open],
        RecentWorkFailureState::MissingPath | RecentWorkFailureState::MovedRoot => vec![
            SafeRecoveryAction::LocateMissingTarget,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        RecentWorkFailureState::ReconnectRequired => vec![
            SafeRecoveryAction::RetryLater,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        RecentWorkFailureState::InspectOnly => vec![
            SafeRecoveryAction::OpenReadOnlyCachedView,
            SafeRecoveryAction::RemoveFromRecents,
        ],
        RecentWorkFailureState::Blocked | RecentWorkFailureState::Unknown => vec![
            SafeRecoveryAction::RetryLater,
            SafeRecoveryAction::RemoveFromRecents,
        ],
    }
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::RecentWorkTargetState;

    fn ready_routes() -> Vec<RecoveryRouteRecord> {
        [
            SafeRecoveryAction::Open,
            SafeRecoveryAction::OpenInNewWindow,
            SafeRecoveryAction::RevealInExplorer,
            SafeRecoveryAction::RemoveFromRecents,
        ]
        .into_iter()
        .map(RecoveryRouteRecord::from_action)
        .collect()
    }

    fn routes_all_surfaces() -> Vec<EntryRouteRecord> {
        EntryRouteSurface::REQUIRED
            .into_iter()
            .map(|surface| EntryRouteRecord {
                surface,
                route_ref: format!("aureline://entry-route/{}/recent:test", surface.as_str()),
                keyboard_reachable: true,
                activates_same_target: true,
            })
            .collect()
    }

    fn accessibility(routes: &[RecoveryRouteRecord], narration: String) -> AccessibilityDisclosure {
        AccessibilityDisclosure {
            focus_order_index: 0,
            tab_stop_count: 1 + routes.len() as u32,
            row_narration: narration,
            action_labels: routes.iter().map(|r| r.action_label.clone()).collect(),
            layout_modes: LayoutMode::REQUIRED
                .into_iter()
                .map(|mode| LayoutModeDisclosure {
                    mode,
                    row_narration_available: true,
                    recovery_affordances_reachable: true,
                })
                .collect(),
        }
    }

    fn honest_input() -> EntryTargetDisclosureInput {
        let routes = ready_routes();
        let recovery_ids: Vec<String> = routes.iter().map(|r| r.action_id.clone()).collect();
        EntryTargetDisclosureInput {
            record_id: "entry-target-disclosure:test".to_string(),
            as_of: "2026-05-25T12:00:00Z".to_string(),
            recent_work_ref: "aureline://recent-work/recent:test".to_string(),
            title: "Docs: Folder".to_string(),
            summary: "Folder disclosed as Folder (local); trust trusted, restore exact."
                .to_string(),
            target_kind: TargetKind::LocalFolder,
            target_state: RecentWorkTargetState::Reachable,
            failure_state: RecentWorkFailureState::Ready,
            trust_state: TrustState::Trusted,
            restore_availability: RestoreAvailability::Exact,
            list_section: RecentWorkListSection::Pinned,
            location_subtitle: Some("~/Code/aureline-docs".to_string()),
            subtitle_kind: SubtitleKind::Path,
            last_opened_at: "mono:test".to_string(),
            pinned: true,
            claim_ceiling: PublicClaimCeiling {
                asserts_live_open: true,
                asserts_remote_available: false,
                asserts_full_restore_fidelity: true,
                asserts_trusted_without_evaluation: true,
            },
            recovery_routes: routes.clone(),
            discards_stale_entry_on_failure: false,
            surfaces: SurfaceParity {
                start_center_row_id: "start-center:recent:test".to_string(),
                workspace_switcher_row_id: "workspace-switcher:recent:test".to_string(),
                switcher_entry_classes: vec!["local".to_string(), "pinned".to_string()],
                switch_failure_actions: vec![
                    WorkspaceSwitchRecoveryAction::CancelSwitch.as_str().to_string(),
                    WorkspaceSwitchRecoveryAction::ReopenPreviousWorkspace
                        .as_str()
                        .to_string(),
                ],
                recovery_action_ids: recovery_ids,
                parity_holds: true,
            },
            routes: routes_all_surfaces(),
            accessibility: accessibility(
                &routes,
                "Folder, Aureline docs — ready — recovery: Open, Open in new window."
                    .to_string(),
            ),
            available_without_account: true,
            available_without_managed_services: true,
            diagnostics_export_ref: "aureline://diagnostics/entry-target-disclosure".to_string(),
            support_export_ref: "aureline://support-export/entry-target-disclosure".to_string(),
            evidence_refs: vec!["aureline://artifact/ux-m4-stabilize-start-center".to_string()],
            narrative_refs: vec!["aureline://doc/ux-m4-stabilize-start-center".to_string()],
        }
    }

    #[test]
    fn honest_input_builds() {
        let record = EntryTargetDisclosureRecord::build(honest_input()).expect("builds");
        assert_eq!(record.record_kind, ENTRY_TARGET_DISCLOSURE_RECORD_KIND);
        assert_eq!(record.target_class, TargetClass::Local);
        assert!(!record.honesty_marker_present);
        assert!(!record.discards_stale_entry_on_failure);
        assert!(record.disclosure.discloses_recovery_routes);
    }

    #[test]
    fn rejects_live_open_claim_for_unavailable_target() {
        let mut input = honest_input();
        input.failure_state = RecentWorkFailureState::MissingPath;
        input.target_state = RecentWorkTargetState::MissingTarget;
        input.restore_availability = RestoreAvailability::LayoutOnly;
        input.claim_ceiling.asserts_full_restore_fidelity = false;
        // keep asserts_live_open = true -> must fail
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(err, BuildError::OverclaimsLiveOpen);
    }

    #[test]
    fn rejects_remote_available_claim_when_disconnected() {
        let mut input = honest_input();
        input.target_kind = TargetKind::SshWorkspace;
        input.target_state = RecentWorkTargetState::RemoteUnreachable;
        input.failure_state = RecentWorkFailureState::ReconnectRequired;
        input.restore_availability = RestoreAvailability::EvidenceOnly;
        input.trust_state = TrustState::PendingEvaluation;
        input.claim_ceiling = PublicClaimCeiling {
            asserts_live_open: false,
            asserts_remote_available: true,
            asserts_full_restore_fidelity: false,
            asserts_trusted_without_evaluation: false,
        };
        input.recovery_routes = [
            SafeRecoveryAction::Reconnect,
            SafeRecoveryAction::RetryLater,
            SafeRecoveryAction::OpenWithoutRestore,
            SafeRecoveryAction::RemoveFromRecents,
        ]
        .into_iter()
        .map(RecoveryRouteRecord::from_action)
        .collect();
        input.surfaces.recovery_action_ids = input
            .recovery_routes
            .iter()
            .map(|r| r.action_id.clone())
            .collect();
        input.accessibility = accessibility(
            &input.recovery_routes,
            "SSH, host — reconnect required — recovery.".to_string(),
        );
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(err, BuildError::OverclaimsRemoteAvailable);
    }

    #[test]
    fn rejects_trust_overclaim() {
        let mut input = honest_input();
        input.trust_state = TrustState::Restricted;
        // asserts_trusted_without_evaluation stays true
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(err, BuildError::OverclaimsTrust);
    }

    #[test]
    fn rejects_silent_discard() {
        let mut input = honest_input();
        input.discards_stale_entry_on_failure = true;
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(err, BuildError::SilentlyDiscardsStaleEntry);
    }

    #[test]
    fn rejects_missing_recovery_route() {
        let mut input = honest_input();
        input.failure_state = RecentWorkFailureState::MissingPath;
        input.target_state = RecentWorkTargetState::MissingTarget;
        input.restore_availability = RestoreAvailability::LayoutOnly;
        input.claim_ceiling = PublicClaimCeiling::default();
        // recovery routes still the "ready" set -> missing Locate
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert!(matches!(err, BuildError::MissingRecoveryRoute { .. }));
    }

    #[test]
    fn rejects_route_surface_gap() {
        let mut input = honest_input();
        input.routes.retain(|route| route.surface != EntryRouteSurface::CommandPalette);
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(
            err,
            BuildError::RouteSurfaceMissing {
                surface: EntryRouteSurface::CommandPalette
            }
        );
    }

    #[test]
    fn rejects_unreachable_layout_mode() {
        let mut input = honest_input();
        input.accessibility.layout_modes[1].recovery_affordances_reachable = false;
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert!(matches!(
            err,
            BuildError::AccessibilityLayoutModeUnreachable { .. }
        ));
    }

    #[test]
    fn rejects_narration_without_target_kind() {
        let mut input = honest_input();
        input.accessibility.row_narration = "a row with no kind disclosed".to_string();
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(err, BuildError::NarrationOmitsTargetKind);
    }

    #[test]
    fn rejects_hidden_without_account() {
        let mut input = honest_input();
        input.available_without_account = false;
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert_eq!(err, BuildError::HiddenWithoutAccount);
    }

    #[test]
    fn rejects_non_canonical_recent_work_ref() {
        let mut input = honest_input();
        input.recent_work_ref = "https://example.com/recent".to_string();
        let err = EntryTargetDisclosureRecord::build(input).expect_err("must reject");
        assert!(matches!(err, BuildError::NonCanonicalRef { .. }));
    }
}
