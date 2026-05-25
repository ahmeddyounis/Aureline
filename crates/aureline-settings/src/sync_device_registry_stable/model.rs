//! Canonical stable truth model for the **sync / device-registry
//! certification**: device participation truth, field-aware conflict review,
//! snapshot-class provenance, local-authoritative fallback, the secret
//! boundary, REL-SYNC-009 merge precedence, profile-roaming / offboarding
//! truth, and cross-surface explanation parity.
//!
//! ## Why one governed certification record
//!
//! Profile portability is read and acted on from many surfaces — the desktop
//! device-and-sync surface, a CLI / headless inspect, Help/About, a support
//! export, and the admin device-registry view. If each surface re-derives "which
//! devices participate, what synced, what is stale, what would be overwritten,
//! and whether the local profile is still authoritative" from its own private
//! read, the surfaces drift: the UI implies healthy roaming while a support
//! export shows a stale manifest, a paused device silently keeps emitting, or a
//! synced bundle overwrites a local scope with no checkpoint and no diff. The
//! risk this closes: a green "profile portability is replacement-grade" claim
//! that is really an average over surfaces that each describe sync a little
//! differently — masking offline, stale, policy-blocked, or secret-leaking sync
//! as healthy continuity.
//!
//! A [`SyncDeviceRegistryCertification`] mints, for one sync posture:
//!
//! - **Device participation truth.** Every [`DeviceParticipationRow`] exposes a
//!   stable device identity, participation state, profile durability, last
//!   successful sync, selected scope set, current conflict class, retained
//!   rollback checkpoint, and local-authoritative fallback posture — inspectable
//!   without opting into any mutating sync action.
//! - **Field-aware conflict review.** Every [`ConflictReviewRow`] classifies the
//!   outcome as exact-match, translated, partial, stale-remote, policy-locked, or
//!   local-authoritative, names the REL-SYNC-009 merge class, and — when it would
//!   overwrite a local scope — carries a structured change preview and a rollback
//!   checkpoint before any apply.
//! - **Snapshot-class provenance.** Every [`SnapshotRow`] (local rollback
//!   checkpoint, portable profile export, managed sync snapshot, support recovery
//!   manifest) carries its included / excluded state classes, producer Aureline /
//!   schema version, integrity hash, source provenance, and local-authoritative
//!   fallback posture.
//! - **The secret boundary.** [`SecretBoundaryRow`] proves dirty-buffer journals,
//!   raw tokens, passkeys, and private keys never cross the sync or export lane;
//!   only reference-only metadata is allowed.
//! - **Profile-roaming / offboarding truth.** [`ProfileRoamingSummary`] carries
//!   the latest successful sync manifest, the extension-inventory pointer, the
//!   remaining-retention timeline, whether managed sync is available, and proves
//!   local launch / edit authority is retained even when managed sync is
//!   unavailable, with temporary profiles excluded by default.
//! - **Cross-surface parity.** One [`SurfaceParityRow`] per desktop UI, CLI
//!   inspect, Help/About, support export, and admin device-registry surface.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//!
//! Dashboards, docs, Help/About surfaces, and support exports read this record
//! verbatim instead of cloning status text.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub use crate::sync::{DeviceParticipationState, IdentityModeClass};

/// Stable record-kind tag carried in serialized certification records.
pub const SYNC_DEVICE_REGISTRY_RECORD_KIND: &str = "sync_device_registry_certification_record";

/// Schema version for the [`SyncDeviceRegistryCertification`] payload shape.
pub const SYNC_DEVICE_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF: &str =
    "settings:sync_device_registry_stable:v1";

/// Reviewer-facing notice rendered on every certification surface.
pub const SYNC_DEVICE_REGISTRY_NOTICE: &str =
    "Sync / device-registry certification: every participating device exposes its identity, \
     participation state, profile durability, last successful sync, selected scope set, conflict \
     class, retained rollback checkpoint, and local-authoritative fallback posture, inspectable \
     without opting into a mutating sync action; conflict review is field-aware (exact match, \
     translated, partial, stale-remote, policy-locked, local-authoritative) and shows a structured \
     change preview plus a rollback checkpoint before any synced value overwrites a local scope; \
     each snapshot class — local rollback checkpoint, portable profile export, managed sync \
     snapshot, support recovery manifest — carries its included and excluded state classes, \
     producer Aureline and schema version, integrity hash, source provenance, and \
     local-authoritative fallback; dirty-buffer journals, raw tokens, passkeys, and private keys \
     never cross the sync or export lane, only reference-only metadata; profile-roaming and \
     offboarding truth carries the latest successful sync manifest, extension inventory pointer, \
     remaining-retention timeline, and whether managed sync is unavailable without losing local \
     launch or edit authority, with temporary profiles excluded by default; the desktop UI, CLI \
     inspect, Help/About, support export, and admin device-registry view all consume the same \
     record instead of cloning prose; and a posture that cannot prove a pillar, or whose lowest \
     surface marker is below Stable, narrows below Stable with a named reason rather than \
     inheriting an adjacent green row.";

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
    /// Sync / device-registry portability is replacement-grade across the rows.
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

/// Lifecycle marker carried by a surface.
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
    /// The device-and-sync registry surface — the authoritative surface.
    DeviceRegistry,
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
            Self::DeviceRegistry => "device_registry",
            Self::CommandPalette => "command_palette",
            Self::StatusBar => "status_bar",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a record.
    pub const REQUIRED: [Self; 4] = [
        Self::DeviceRegistry,
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
    /// Opens or focuses the authoritative device-registry surface.
    Primary,
    /// Inspects or recovers sync / device-registry state.
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
// Domain vocabularies
// ---------------------------------------------------------------------------

/// Whether a profile is durable, session-only, promoted, or discarded on exit.
///
/// Temporary / troubleshooting profiles are `SessionOnly` or `DiscardedOnExit`
/// and are never synced by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileDurabilityClass {
    /// A durable profile that participates in sync subject to scope selection.
    Durable,
    /// A session-only profile that never syncs by default.
    SessionOnly,
    /// A previously session-only profile a user explicitly promoted to durable.
    Promoted,
    /// A troubleshooting profile that is discarded on exit and never syncs.
    DiscardedOnExit,
}

impl ProfileDurabilityClass {
    /// Returns the stable durability token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Durable => "durable",
            Self::SessionOnly => "session_only",
            Self::Promoted => "promoted",
            Self::DiscardedOnExit => "discarded_on_exit",
        }
    }

    /// True when a profile of this class must never sync by default.
    pub const fn never_syncs_by_default(self) -> bool {
        matches!(self, Self::SessionOnly | Self::DiscardedOnExit)
    }
}

/// Field-aware conflict-review outcome class (the v5 vocabulary).
///
/// Surfaces consume these tokens verbatim and must not invent their own outcome
/// names. The class distinguishes the cases a generic "merge?" prompt hides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictOutcomeClass {
    /// Local and remote values are byte-equal; nothing changes.
    ExactMatch,
    /// The remote value maps onto the local value through a migration alias.
    Translated,
    /// Some fields merge cleanly; others need explicit review.
    Partial,
    /// The remote copy is older than the local lineage; local wins.
    StaleRemote,
    /// A policy ceiling owns the value; neither side may overwrite it freely.
    PolicyLocked,
    /// The local explicit edit is authoritative; the remote does not overwrite.
    LocalAuthoritative,
}

impl ConflictOutcomeClass {
    /// Returns the stable outcome token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactMatch => "exact_match",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::StaleRemote => "stale_remote",
            Self::PolicyLocked => "policy_locked",
            Self::LocalAuthoritative => "local_authoritative",
        }
    }

    /// The outcome classes the review must be able to distinguish, in canonical
    /// order, so a posture proves it is field-aware rather than a generic prompt.
    pub const REQUIRED_COVERAGE: [Self; 6] = [
        Self::ExactMatch,
        Self::Translated,
        Self::Partial,
        Self::StaleRemote,
        Self::PolicyLocked,
        Self::LocalAuthoritative,
    ];
}

/// REL-SYNC-009 merge class for one conflicting setting / asset.
///
/// Scalars merge fieldwise, additive assets merge additively where safe, and
/// keybindings / tasks / launch / workset definitions require explicit conflict
/// review before any overwrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeClass {
    /// Scalar settings: fieldwise merge.
    FieldwiseMerge,
    /// Additive assets: additive merge where safe.
    AdditiveMerge,
    /// Keybindings / tasks / launch / workset definitions: explicit review.
    ExplicitConflictReview,
    /// Local explicit edit wins over a stale remote copy.
    LocalPrecedence,
}

impl MergeClass {
    /// Returns the stable merge-class token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldwiseMerge => "fieldwise_merge",
            Self::AdditiveMerge => "additive_merge",
            Self::ExplicitConflictReview => "explicit_conflict_review",
            Self::LocalPrecedence => "local_precedence",
        }
    }
}

/// Setting / asset category used to check the REL-SYNC-009 merge rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingCategory {
    /// A scalar setting (boolean / integer / enum / string).
    Scalar,
    /// An additive asset list (e.g. trusted folders).
    AdditiveAsset,
    /// A keybinding / task / launch / workset definition.
    StructuredDefinition,
}

impl SettingCategory {
    /// Returns the stable category token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::AdditiveAsset => "additive_asset",
            Self::StructuredDefinition => "structured_definition",
        }
    }

    /// The merge class REL-SYNC-009 requires for this category before overwrite.
    pub const fn required_merge_class(self) -> MergeClass {
        match self {
            Self::Scalar => MergeClass::FieldwiseMerge,
            Self::AdditiveAsset => MergeClass::AdditiveMerge,
            Self::StructuredDefinition => MergeClass::ExplicitConflictReview,
        }
    }
}

/// Snapshot class (the v21 vocabulary) carried by a [`SnapshotRow`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotClass {
    /// A local rollback checkpoint created before a mutating apply.
    LocalRollbackCheckpoint,
    /// A user-initiated portable profile export.
    PortableProfileExport,
    /// A managed sync snapshot pushed or pulled across the sync lane.
    ManagedSyncSnapshot,
    /// A redacted support recovery manifest.
    SupportRecoveryManifest,
}

impl SnapshotClass {
    /// Returns the stable snapshot-class token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRollbackCheckpoint => "local_rollback_checkpoint",
            Self::PortableProfileExport => "portable_profile_export",
            Self::ManagedSyncSnapshot => "managed_sync_snapshot",
            Self::SupportRecoveryManifest => "support_recovery_manifest",
        }
    }

    /// True when a snapshot of this class crosses a sync or export lane, so it
    /// must never include secret or dirty-buffer state classes.
    pub const fn crosses_sync_or_export_lane(self) -> bool {
        matches!(
            self,
            Self::PortableProfileExport | Self::ManagedSyncSnapshot | Self::SupportRecoveryManifest
        )
    }

    /// The snapshot classes a posture must enumerate, in canonical order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalRollbackCheckpoint,
        Self::PortableProfileExport,
        Self::ManagedSyncSnapshot,
        Self::SupportRecoveryManifest,
    ];
}

/// State class included in or excluded from a snapshot or sync / export lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
    /// Scalar settings.
    ScalarSettings,
    /// Keybinding definitions.
    Keybindings,
    /// Task definitions.
    Tasks,
    /// Launch configurations.
    LaunchConfigs,
    /// Workset definitions.
    WorksetDefinitions,
    /// Reference-only extension inventory pointers.
    ExtensionInventoryRefs,
    /// Machine-local topology (never carried by sync).
    MachineLocalTopology,
    /// Dirty-buffer journals (never auto-synced cross-device).
    DirtyBufferJournals,
    /// Secret material: raw tokens, passkeys, private keys.
    SecretMaterial,
    /// Reference-only metadata (the only secret-adjacent class allowed to cross).
    ReferenceOnlyMetadata,
}

impl StateClass {
    /// Returns the stable state-class token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScalarSettings => "scalar_settings",
            Self::Keybindings => "keybindings",
            Self::Tasks => "tasks",
            Self::LaunchConfigs => "launch_configs",
            Self::WorksetDefinitions => "workset_definitions",
            Self::ExtensionInventoryRefs => "extension_inventory_refs",
            Self::MachineLocalTopology => "machine_local_topology",
            Self::DirtyBufferJournals => "dirty_buffer_journals",
            Self::SecretMaterial => "secret_material",
            Self::ReferenceOnlyMetadata => "reference_only_metadata",
        }
    }

    /// True when a state class must never be included in a snapshot that crosses
    /// a sync or export lane (dirty buffers and secret material).
    pub const fn is_secret_or_volatile(self) -> bool {
        matches!(self, Self::DirtyBufferJournals | Self::SecretMaterial)
    }

    /// The forbidden state classes the secret boundary must explicitly exclude.
    pub const FORBIDDEN_ON_LANE: [Self; 2] = [Self::DirtyBufferJournals, Self::SecretMaterial];
}

// ---------------------------------------------------------------------------
// Cross-surface parity vocabulary
// ---------------------------------------------------------------------------

/// A surface that must explain sync / device-registry truth from the shared
/// record, not its own prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The desktop device-and-sync surface.
    DesktopUi,
    /// The CLI / headless inspect command.
    CliInspect,
    /// The Help/About surface.
    HelpAbout,
    /// The support export.
    SupportExport,
    /// The admin device-registry view.
    AdminDeviceRegistry,
}

impl SurfaceClass {
    /// Returns the stable string vocabulary for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::AdminDeviceRegistry => "admin_device_registry",
        }
    }

    /// The closed required surface set in canonical order.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopUi,
        Self::CliInspect,
        Self::HelpAbout,
        Self::SupportExport,
        Self::AdminDeviceRegistry,
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
// Device participation rows
// ---------------------------------------------------------------------------

/// One device record in the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceParticipationRow {
    /// Opaque stable device id (never a hostname, serial, MAC, or IP address).
    pub device_id: String,
    /// User-chosen redaction-safe label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_label: Option<String>,
    /// Coarse device class token.
    pub device_class: String,
    /// Coarse os-family class token.
    pub os_family_class: String,
    /// Identity mode reported at the time the row was produced.
    pub identity_mode: IdentityModeClass,
    /// Participation lifecycle stage.
    pub participation_state: DeviceParticipationState,
    /// Whether this is the local resolving device.
    pub is_local_device: bool,
    /// Durability of the profile this device carries.
    pub profile_durability: ProfileDurabilityClass,
    /// Last successful sync stamp. `None` means never synced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_sync: Option<String>,
    /// Sync-freshness token (`fresh`, `stale`, `never`, `paused`, `blocked`).
    pub sync_freshness: String,
    /// Selected scope set this device participates in, sorted.
    pub selected_scope_set: Vec<String>,
    /// Current unresolved conflict-class token, or `none`.
    pub conflict_class: String,
    /// Retained rollback checkpoint ref (kept even when sync is paused/failed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Whether the local profile stays authoritative if this device's data is
    /// stale, unavailable, undecryptable, or policy-denied.
    pub local_authoritative_fallback: bool,
    /// Whether participation is inspectable without opting into a mutating sync
    /// action.
    pub inspectable_without_mutation: bool,
    /// Optional revocation-reason token for non-active rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    /// Optional bounded waiver ref for a non-conforming row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the row exposes the required participation truth.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Conflict-review rows
// ---------------------------------------------------------------------------

/// One field-aware conflict-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictReviewRow {
    /// Canonical setting id (or asset id).
    pub setting_id: String,
    /// Scope targeted by the arriving synced value.
    pub conflicting_scope: String,
    /// Setting / asset category used to check the merge rule.
    pub setting_category: SettingCategory,
    /// Field-aware outcome class.
    pub outcome_class: ConflictOutcomeClass,
    /// REL-SYNC-009 merge class.
    pub merge_class: MergeClass,
    /// Device id that produced the arriving synced value.
    pub remote_device_id: String,
    /// Producer participation state.
    pub remote_participation_state: DeviceParticipationState,
    /// Recommended resolution-path token.
    pub recommended_path: String,
    /// Whether applying the recommended path would overwrite the local value.
    pub overwrites_local: bool,
    /// Whether the local explicit edit stays authoritative.
    pub local_authoritative: bool,
    /// Structured change-preview ref shown before any overwrite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_preview_ref: Option<String>,
    /// Rollback checkpoint ref created before any overwrite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Whether the user can inspect what changes before apply, without mutating.
    pub inspectable_before_apply: bool,
    /// Whether applying the row would widen trust, egress, permissions, or
    /// managed authority (must be false to conform).
    pub widens_authority: bool,
    /// Redaction class applied to value previews.
    pub redaction_class: String,
    /// Lock-state token from the resolver.
    pub lock_state: String,
    /// Ref back to the projected sync conflict packet, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_packet_ref: Option<String>,
    /// Canonical Diagnostics Center entry point for the conflict.
    pub diagnostics_entry_ref: String,
    /// Derived: the merge class matches the category's REL-SYNC-009 requirement.
    pub merge_rule_satisfied: bool,
    /// Derived: a row that overwrites local carries a change preview and a
    /// rollback checkpoint before apply.
    pub protected_before_overwrite: bool,
    /// Optional bounded waiver ref for a non-conforming row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the row is field-aware, merge-correct, protected, and does not
    /// widen authority.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Snapshot rows
// ---------------------------------------------------------------------------

/// One snapshot-class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRow {
    /// Snapshot class.
    pub snapshot_class: SnapshotClass,
    /// Canonical snapshot ref.
    pub snapshot_ref: String,
    /// Producer Aureline version.
    pub producer_aureline_version: String,
    /// Producer schema version.
    pub producer_schema_version: String,
    /// Integrity hash over the snapshot body.
    pub integrity_hash: String,
    /// Source provenance: originating device / profile revision.
    pub source_provenance: String,
    /// Included state classes, sorted.
    pub included_state_classes: Vec<StateClass>,
    /// Excluded state classes, sorted.
    pub excluded_state_classes: Vec<StateClass>,
    /// Whether the local profile stays authoritative if this snapshot is stale,
    /// unavailable, or undecryptable.
    pub local_authoritative_fallback: bool,
    /// Derived: the snapshot does not include a forbidden state class on a lane
    /// that crosses sync / export.
    pub carries_forbidden_state_class: bool,
    /// Optional bounded waiver ref for a non-conforming row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the snapshot carries full provenance and holds the boundary.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Secret-boundary rows
// ---------------------------------------------------------------------------

/// One secret-boundary exclusion proof for a forbidden state class on a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBoundaryRow {
    /// Forbidden state class this row proves is excluded.
    pub state_class: StateClass,
    /// Lane the exclusion applies to (`sync` or `export`).
    pub lane: String,
    /// Whether the class is excluded from this lane (must be true to conform).
    pub excluded: bool,
    /// Whether reference-only metadata may stand in for the class.
    pub reference_only_allowed: bool,
    /// Reviewer-facing reason.
    pub reason: String,
    /// Derived: the forbidden class is excluded from the lane.
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
    /// Whether the surface consumes the shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface clones manually maintained prose (must be false).
    pub clones_prose: bool,
    /// The maturity of this surface's treatment.
    pub surface_marker: LifecycleMarker,
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
// Profile-roaming / offboarding summary
// ---------------------------------------------------------------------------

/// Profile-roaming and offboarding truth for the posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileRoamingSummary {
    /// Latest successful sync manifest ref. `None` means never synced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_successful_sync_ref: Option<String>,
    /// Reference-only extension-inventory pointer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_inventory_ref: Option<String>,
    /// Remaining-retention timeline in days. `None` means not applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_retention_days: Option<u32>,
    /// Whether managed sync is currently available.
    pub managed_sync_available: bool,
    /// Whether local launch / edit authority is retained even when managed sync
    /// is unavailable.
    pub local_launch_edit_authority_retained: bool,
    /// Whether temporary / troubleshooting profiles are excluded by default.
    pub temporary_profiles_excluded: bool,
    /// Durability of the active profile.
    pub active_profile_durability: ProfileDurabilityClass,
    /// Originating profile revision.
    pub originating_profile_revision: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Derived: roaming truth is coherent and local authority is retained.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Pillars, claim ceiling, qualification, upstream
// ---------------------------------------------------------------------------

/// The derived pillar verdicts (what the posture can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPillars {
    /// Every device exposes the required participation truth.
    pub device_participation_truth: bool,
    /// Conflict review is field-aware across the required outcome classes.
    pub conflict_review_field_aware: bool,
    /// Every snapshot class carries full provenance.
    pub snapshot_provenance_complete: bool,
    /// Local-authoritative fallback is proven and overwrites are protected.
    pub local_fallback_proven: bool,
    /// The secret boundary excludes dirty buffers and secret material.
    pub secret_boundary_held: bool,
    /// REL-SYNC-009 merge rules are enforced per setting category.
    pub merge_rules_enforced: bool,
    /// Profile-roaming / offboarding truth is coherent and keeps local authority.
    pub profile_roaming_truth: bool,
    /// Every surface consumes the shared record and clones no prose.
    pub surfaces_share_one_truth: bool,
}

/// The public claim ceiling: what a posture is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationClaimCeiling {
    /// May claim full device participation truth.
    pub asserts_device_participation_truth: bool,
    /// May claim field-aware conflict review.
    pub asserts_conflict_review_field_aware: bool,
    /// May claim complete snapshot provenance.
    pub asserts_snapshot_provenance_complete: bool,
    /// May claim local-authoritative fallback is proven.
    pub asserts_local_fallback_proven: bool,
    /// May claim the secret boundary is held.
    pub asserts_secret_boundary_held: bool,
    /// May claim REL-SYNC-009 merge rules are enforced.
    pub asserts_merge_rules_enforced: bool,
    /// May claim profile-roaming truth is complete.
    pub asserts_profile_roaming_truth: bool,
    /// May claim every surface shares one truth.
    pub asserts_surfaces_share_one_truth: bool,
}

/// Reason a posture is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowingReason {
    /// A device does not expose the required participation truth.
    DeviceParticipationIncomplete,
    /// Conflict review is not field-aware across the required outcome classes.
    ConflictReviewNotFieldAware,
    /// A snapshot class is missing provenance.
    SnapshotProvenanceMissing,
    /// Local-authoritative fallback is unproven, or an overwrite is unprotected.
    LocalFallbackUnproven,
    /// The secret boundary admits a forbidden state class.
    SecretBoundaryUnproven,
    /// A merge rule does not match the setting category.
    MergeRuleUnenforced,
    /// Profile-roaming truth is incomplete or loses local authority.
    ProfileRoamingIncomplete,
    /// A surface clones prose instead of consuming the shared record.
    SurfaceClonesProse,
    /// The lowest surface marker is below Stable, so the posture must not inherit
    /// Stable by adjacency.
    SurfaceNotYetStable,
}

impl CertificationNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceParticipationIncomplete => "device_participation_incomplete",
            Self::ConflictReviewNotFieldAware => "conflict_review_not_field_aware",
            Self::SnapshotProvenanceMissing => "snapshot_provenance_missing",
            Self::LocalFallbackUnproven => "local_fallback_unproven",
            Self::SecretBoundaryUnproven => "secret_boundary_unproven",
            Self::MergeRuleUnenforced => "merge_rule_unenforced",
            Self::ProfileRoamingIncomplete => "profile_roaming_incomplete",
            Self::SurfaceClonesProse => "surface_clones_prose",
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
    /// Resolver-state ref the conflict rows were resolved from.
    pub resolver_state_ref: String,
    /// Sync projection shared-contract ref the rows project through.
    pub sync_contract_ref: String,
    /// Participating device ids, sorted and deduped.
    pub participating_device_ids: Vec<String>,
    /// Reviewed setting ids, sorted and deduped.
    pub reviewed_setting_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`SyncDeviceRegistryCertification`].
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
    /// Device participation rows.
    pub device_participation: Vec<DeviceParticipationRow>,
    /// Conflict-review rows.
    pub conflict_review: Vec<ConflictReviewRow>,
    /// Snapshot rows.
    pub snapshots: Vec<SnapshotRow>,
    /// Secret-boundary rows.
    pub secret_boundary: Vec<SecretBoundaryRow>,
    /// Cross-surface parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Profile-roaming summary.
    pub profile_roaming: ProfileRoamingSummary,
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

/// The canonical, governed sync / device-registry certification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDeviceRegistryCertification {
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
    /// Device participation rows, sorted by device id.
    pub device_participation: Vec<DeviceParticipationRow>,
    /// Conflict-review rows, sorted by (setting id, scope).
    pub conflict_review: Vec<ConflictReviewRow>,
    /// Snapshot rows, in canonical class order.
    pub snapshots: Vec<SnapshotRow>,
    /// Secret-boundary rows, in canonical (class, lane) order.
    pub secret_boundary: Vec<SecretBoundaryRow>,
    /// Cross-surface parity rows, in canonical surface order.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Profile-roaming summary.
    pub profile_roaming: ProfileRoamingSummary,
    /// Outcome classes exercised across all conflict rows, sorted.
    pub outcome_coverage: Vec<ConflictOutcomeClass>,
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

/// Reasons a [`SyncDeviceRegistryCertification`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// No device participation rows were supplied.
    NoDevices,
    /// No local device was present in the registry.
    NoLocalDevice,
    /// A device id was duplicated.
    DuplicateDevice { device_id: String },
    /// A device row was non-conforming without a bounded waiver.
    DeviceNarrowedWithoutWaiver { device_id: String },
    /// No conflict-review rows were supplied.
    NoConflictRows,
    /// A conflict-review row was non-conforming without a bounded waiver.
    ConflictNarrowedWithoutWaiver { setting_id: String },
    /// A required snapshot class was missing.
    SnapshotClassMissing { class: SnapshotClass },
    /// A snapshot class was duplicated.
    DuplicateSnapshotClass { class: SnapshotClass },
    /// A snapshot row was non-conforming without a bounded waiver.
    SnapshotNarrowedWithoutWaiver { class: SnapshotClass },
    /// A required forbidden state class was missing from the secret boundary.
    SecretBoundaryClassMissing { class: StateClass, lane: &'static str },
    /// A required surface-parity row was missing.
    SurfaceRowMissing { surface: SurfaceClass },
    /// A surface-parity row was duplicated.
    DuplicateSurfaceRow { surface: SurfaceClass },
    /// A surface narrowed below Stable without a bounded waiver.
    SurfaceNarrowedWithoutWaiver { surface: SurfaceClass },
    /// The claim ceiling asserted device participation truth it cannot prove.
    OverclaimsDeviceParticipation,
    /// The claim ceiling asserted field-aware conflict review it cannot prove.
    OverclaimsConflictReview,
    /// The claim ceiling asserted snapshot provenance it cannot prove.
    OverclaimsSnapshotProvenance,
    /// The claim ceiling asserted local fallback it cannot prove.
    OverclaimsLocalFallback,
    /// The claim ceiling asserted the secret boundary it cannot prove.
    OverclaimsSecretBoundary,
    /// The claim ceiling asserted merge enforcement it cannot prove.
    OverclaimsMergeRules,
    /// The claim ceiling asserted profile-roaming truth it cannot prove.
    OverclaimsProfileRoaming,
    /// The claim ceiling asserted shared-truth surfaces it cannot prove.
    OverclaimsSurfaces,
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
            Self::NoDevices => {
                write!(f, "a sync certification must register at least one device")
            }
            Self::NoLocalDevice => write!(
                f,
                "a sync certification must register the local resolving device"
            ),
            Self::DuplicateDevice { device_id } => {
                write!(f, "device row `{device_id}` is duplicated")
            }
            Self::DeviceNarrowedWithoutWaiver { device_id } => write!(
                f,
                "device row `{device_id}` is non-conforming but carries no bounded waiver ref"
            ),
            Self::NoConflictRows => write!(
                f,
                "a sync certification must exercise at least one conflict-review row"
            ),
            Self::ConflictNarrowedWithoutWaiver { setting_id } => write!(
                f,
                "conflict row `{setting_id}` is non-conforming but carries no bounded waiver ref"
            ),
            Self::SnapshotClassMissing { class } => {
                write!(f, "snapshot class `{}` is missing", class.as_str())
            }
            Self::DuplicateSnapshotClass { class } => {
                write!(f, "snapshot class `{}` is duplicated", class.as_str())
            }
            Self::SnapshotNarrowedWithoutWaiver { class } => write!(
                f,
                "snapshot row `{}` is non-conforming but carries no bounded waiver ref",
                class.as_str()
            ),
            Self::SecretBoundaryClassMissing { class, lane } => write!(
                f,
                "secret boundary must exclude forbidden class `{}` on lane `{lane}`",
                class.as_str()
            ),
            Self::SurfaceRowMissing { surface } => {
                write!(f, "surface-parity row `{}` is missing", surface.as_str())
            }
            Self::DuplicateSurfaceRow { surface } => {
                write!(f, "surface-parity row `{}` is duplicated", surface.as_str())
            }
            Self::SurfaceNarrowedWithoutWaiver { surface } => write!(
                f,
                "surface-parity row `{}` is narrowed below Stable but carries no bounded waiver ref",
                surface.as_str()
            ),
            Self::OverclaimsDeviceParticipation => write!(
                f,
                "claim ceiling may not assert device participation truth it cannot prove"
            ),
            Self::OverclaimsConflictReview => write!(
                f,
                "claim ceiling may not assert field-aware conflict review it cannot prove"
            ),
            Self::OverclaimsSnapshotProvenance => write!(
                f,
                "claim ceiling may not assert snapshot provenance it cannot prove"
            ),
            Self::OverclaimsLocalFallback => write!(
                f,
                "claim ceiling may not assert local-authoritative fallback it cannot prove"
            ),
            Self::OverclaimsSecretBoundary => write!(
                f,
                "claim ceiling may not assert the secret boundary it cannot prove"
            ),
            Self::OverclaimsMergeRules => write!(
                f,
                "claim ceiling may not assert merge enforcement it cannot prove"
            ),
            Self::OverclaimsProfileRoaming => write!(
                f,
                "claim ceiling may not assert profile-roaming truth it cannot prove"
            ),
            Self::OverclaimsSurfaces => write!(
                f,
                "claim ceiling may not assert shared-truth surfaces when a surface clones prose"
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
                "a sync certification must stay available without an account"
            ),
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a sync certification must stay available without managed services"
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

impl SyncDeviceRegistryCertification {
    /// Builds a governed certification record from validated input.
    ///
    /// The pillar verdicts are *derived* from the device participation rows, the
    /// conflict-review rows, the snapshot rows, the secret boundary, the surface
    /// parity rows, and the profile-roaming summary, so a record can never
    /// publish a claim wider than its proof. Structural lies (a non-canonical
    /// ref, a missing local device, a missing required snapshot class, a missing
    /// secret-boundary exclusion) are rejected outright; provable-but-imperfect
    /// postures (a stale device with a retained checkpoint, a surface that clones
    /// prose, a below-Stable surface) are minted but narrowed below Stable with a
    /// named reason and a bounded waiver.
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
        require_present_ref("upstream.sync_contract_ref", &input.upstream.sync_contract_ref)?;

        // --- device participation --------------------------------------------
        if input.device_participation.is_empty() {
            return Err(BuildError::NoDevices);
        }
        let mut seen_devices: BTreeSet<String> = BTreeSet::new();
        for row in &input.device_participation {
            if !seen_devices.insert(row.device_id.clone()) {
                return Err(BuildError::DuplicateDevice {
                    device_id: row.device_id.clone(),
                });
            }
            if let Some(checkpoint) = &row.rollback_checkpoint_ref {
                require_canonical_ref("device_participation.rollback_checkpoint_ref", checkpoint)?;
            }
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("device_participation.waiver_ref", waiver)?;
            }
        }
        let mut device_participation: Vec<DeviceParticipationRow> =
            input.device_participation.clone();
        device_participation.sort_by(|a, b| a.device_id.cmp(&b.device_id));
        let local_device_present = device_participation.iter().any(|row| row.is_local_device);
        if !local_device_present {
            return Err(BuildError::NoLocalDevice);
        }
        for row in &mut device_participation {
            row.selected_scope_set.sort();
            row.selected_scope_set.dedup();
            let identity_present = !row.device_id.trim().is_empty();
            let state_disclosed = matches!(
                row.participation_state,
                DeviceParticipationState::Active
            ) || row.revocation_reason.is_some();
            row.conforms = identity_present
                && state_disclosed
                && row.local_authoritative_fallback
                && row.inspectable_without_mutation
                && !row.sync_freshness.trim().is_empty()
                && !row.conflict_class.trim().is_empty();
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::DeviceNarrowedWithoutWaiver {
                    device_id: row.device_id.clone(),
                });
            }
        }
        let device_participation_truth = device_participation.iter().all(|row| row.conforms);

        // --- conflict review -------------------------------------------------
        if input.conflict_review.is_empty() {
            return Err(BuildError::NoConflictRows);
        }
        let mut conflict_review: Vec<ConflictReviewRow> = input.conflict_review.clone();
        for row in &conflict_review {
            require_canonical_ref("conflict_review.diagnostics_entry_ref", &row.diagnostics_entry_ref)?;
            if let Some(preview) = &row.change_preview_ref {
                require_canonical_ref("conflict_review.change_preview_ref", preview)?;
            }
            if let Some(checkpoint) = &row.rollback_checkpoint_ref {
                require_canonical_ref("conflict_review.rollback_checkpoint_ref", checkpoint)?;
            }
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("conflict_review.waiver_ref", waiver)?;
            }
        }
        conflict_review.sort_by(|a, b| {
            a.setting_id
                .cmp(&b.setting_id)
                .then(a.conflicting_scope.cmp(&b.conflicting_scope))
        });
        let mut outcome_set: BTreeSet<ConflictOutcomeClass> = BTreeSet::new();
        for row in &mut conflict_review {
            outcome_set.insert(row.outcome_class);
            row.merge_rule_satisfied =
                row.merge_class == row.setting_category.required_merge_class()
                    || (row.outcome_class == ConflictOutcomeClass::StaleRemote
                        && row.merge_class == MergeClass::LocalPrecedence);
            row.protected_before_overwrite = !row.overwrites_local
                || (row.change_preview_ref.is_some() && row.rollback_checkpoint_ref.is_some());
            row.conforms = row.merge_rule_satisfied
                && row.protected_before_overwrite
                && row.inspectable_before_apply
                && !row.widens_authority;
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::ConflictNarrowedWithoutWaiver {
                    setting_id: row.setting_id.clone(),
                });
            }
        }
        let outcome_coverage: Vec<ConflictOutcomeClass> = outcome_set.iter().copied().collect();
        let conflict_review_field_aware = ConflictOutcomeClass::REQUIRED_COVERAGE
            .iter()
            .all(|required| outcome_set.contains(required));
        let local_fallback_proven = conflict_review
            .iter()
            .all(|row| row.protected_before_overwrite && (row.overwrites_local || row.local_authoritative))
            && conflict_review.iter().all(|row| !row.widens_authority);
        let merge_rules_enforced = conflict_review.iter().all(|row| row.merge_rule_satisfied);

        // --- snapshots -------------------------------------------------------
        let mut seen_snapshots: BTreeSet<SnapshotClass> = BTreeSet::new();
        for row in &input.snapshots {
            if !seen_snapshots.insert(row.snapshot_class) {
                return Err(BuildError::DuplicateSnapshotClass {
                    class: row.snapshot_class,
                });
            }
            require_canonical_ref("snapshots.snapshot_ref", &row.snapshot_ref)?;
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("snapshots.waiver_ref", waiver)?;
            }
        }
        for required in SnapshotClass::REQUIRED {
            if !seen_snapshots.contains(&required) {
                return Err(BuildError::SnapshotClassMissing { class: required });
            }
        }
        let mut snapshots: Vec<SnapshotRow> = input.snapshots.clone();
        snapshots.sort_by_key(|row| snapshot_order(row.snapshot_class));
        for row in &mut snapshots {
            row.included_state_classes.sort();
            row.included_state_classes.dedup();
            row.excluded_state_classes.sort();
            row.excluded_state_classes.dedup();
            row.carries_forbidden_state_class = row.snapshot_class.crosses_sync_or_export_lane()
                && row
                    .included_state_classes
                    .iter()
                    .any(|class| class.is_secret_or_volatile());
            let provenance_complete = !row.producer_aureline_version.trim().is_empty()
                && !row.producer_schema_version.trim().is_empty()
                && !row.integrity_hash.trim().is_empty()
                && !row.source_provenance.trim().is_empty();
            row.conforms = provenance_complete
                && !row.carries_forbidden_state_class
                && row.local_authoritative_fallback;
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::SnapshotNarrowedWithoutWaiver {
                    class: row.snapshot_class,
                });
            }
        }
        let snapshot_provenance_complete = snapshots.iter().all(|row| {
            !row.producer_aureline_version.trim().is_empty()
                && !row.producer_schema_version.trim().is_empty()
                && !row.integrity_hash.trim().is_empty()
                && !row.source_provenance.trim().is_empty()
        });

        // --- secret boundary -------------------------------------------------
        let mut secret_boundary: Vec<SecretBoundaryRow> = input.secret_boundary.clone();
        secret_boundary.sort_by(|a, b| {
            state_class_order(a.state_class)
                .cmp(&state_class_order(b.state_class))
                .then(a.lane.cmp(&b.lane))
        });
        for row in &mut secret_boundary {
            row.conforms = row.excluded && row.state_class.is_secret_or_volatile();
        }
        // Every forbidden class must be explicitly excluded on both lanes.
        for class in StateClass::FORBIDDEN_ON_LANE {
            for lane in ["sync", "export"] {
                let present = secret_boundary
                    .iter()
                    .any(|row| row.state_class == class && row.lane == lane && row.excluded);
                if !present {
                    return Err(BuildError::SecretBoundaryClassMissing { class, lane });
                }
            }
        }
        let snapshots_hold_boundary = snapshots.iter().all(|row| !row.carries_forbidden_state_class);
        let secret_boundary_held =
            snapshots_hold_boundary && secret_boundary.iter().all(|row| row.conforms);

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

        // --- profile roaming -------------------------------------------------
        let mut profile_roaming = input.profile_roaming.clone();
        if let Some(manifest) = &profile_roaming.latest_successful_sync_ref {
            require_canonical_ref("profile_roaming.latest_successful_sync_ref", manifest)?;
        }
        if let Some(inventory) = &profile_roaming.extension_inventory_ref {
            require_canonical_ref("profile_roaming.extension_inventory_ref", inventory)?;
        }
        if !is_reviewable_sentence(&profile_roaming.summary) {
            return Err(BuildError::InvalidSentence {
                field: "profile_roaming.summary",
            });
        }
        require_present_ref(
            "profile_roaming.originating_profile_revision",
            &profile_roaming.originating_profile_revision,
        )?;
        let profile_roaming_truth = profile_roaming.local_launch_edit_authority_retained
            && profile_roaming.temporary_profiles_excluded
            && !profile_roaming.summary.trim().is_empty()
            && !profile_roaming.originating_profile_revision.trim().is_empty();
        profile_roaming.conforms = profile_roaming_truth;

        // --- derive pillars --------------------------------------------------
        let pillars = CertificationPillars {
            device_participation_truth,
            conflict_review_field_aware,
            snapshot_provenance_complete,
            local_fallback_proven,
            secret_boundary_held,
            merge_rules_enforced,
            profile_roaming_truth,
            surfaces_share_one_truth,
        };

        // --- claim ceiling: never claim what cannot be proven ----------------
        if input.claim_ceiling.asserts_device_participation_truth && !device_participation_truth {
            return Err(BuildError::OverclaimsDeviceParticipation);
        }
        if input.claim_ceiling.asserts_conflict_review_field_aware && !conflict_review_field_aware {
            return Err(BuildError::OverclaimsConflictReview);
        }
        if input.claim_ceiling.asserts_snapshot_provenance_complete && !snapshot_provenance_complete
        {
            return Err(BuildError::OverclaimsSnapshotProvenance);
        }
        if input.claim_ceiling.asserts_local_fallback_proven && !local_fallback_proven {
            return Err(BuildError::OverclaimsLocalFallback);
        }
        if input.claim_ceiling.asserts_secret_boundary_held && !secret_boundary_held {
            return Err(BuildError::OverclaimsSecretBoundary);
        }
        if input.claim_ceiling.asserts_merge_rules_enforced && !merge_rules_enforced {
            return Err(BuildError::OverclaimsMergeRules);
        }
        if input.claim_ceiling.asserts_profile_roaming_truth && !profile_roaming_truth {
            return Err(BuildError::OverclaimsProfileRoaming);
        }
        if input.claim_ceiling.asserts_surfaces_share_one_truth && !surfaces_share_one_truth {
            return Err(BuildError::OverclaimsSurfaces);
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

        // --- surface marker = lowest among surface markers -------------------
        let mut surface_markers: Vec<LifecycleMarker> = Vec::new();
        for row in &surface_parity {
            surface_markers.push(row.surface_marker);
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::SurfaceNarrowedWithoutWaiver {
                    surface: row.surface_class,
                });
            }
        }
        let surface_lifecycle_marker = surface_markers
            .into_iter()
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !device_participation_truth {
            narrowing_reasons.push(CertificationNarrowingReason::DeviceParticipationIncomplete);
        }
        if !conflict_review_field_aware {
            narrowing_reasons.push(CertificationNarrowingReason::ConflictReviewNotFieldAware);
        }
        if !snapshot_provenance_complete {
            narrowing_reasons.push(CertificationNarrowingReason::SnapshotProvenanceMissing);
        }
        if !local_fallback_proven {
            narrowing_reasons.push(CertificationNarrowingReason::LocalFallbackUnproven);
        }
        if !secret_boundary_held {
            narrowing_reasons.push(CertificationNarrowingReason::SecretBoundaryUnproven);
        }
        if !merge_rules_enforced {
            narrowing_reasons.push(CertificationNarrowingReason::MergeRuleUnenforced);
        }
        if !profile_roaming_truth {
            narrowing_reasons.push(CertificationNarrowingReason::ProfileRoamingIncomplete);
        }
        if !surfaces_share_one_truth {
            narrowing_reasons.push(CertificationNarrowingReason::SurfaceClonesProse);
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
        let mut participating_device_ids: Vec<String> = device_participation
            .iter()
            .map(|row| row.device_id.clone())
            .collect();
        participating_device_ids.sort();
        participating_device_ids.dedup();
        let mut reviewed_setting_ids: Vec<String> = conflict_review
            .iter()
            .map(|row| row.setting_id.clone())
            .collect();
        reviewed_setting_ids.sort();
        reviewed_setting_ids.dedup();

        Ok(Self {
            record_kind: SYNC_DEVICE_REGISTRY_RECORD_KIND.to_string(),
            schema_version: SYNC_DEVICE_REGISTRY_SCHEMA_VERSION,
            notice: SYNC_DEVICE_REGISTRY_NOTICE.to_string(),
            shared_contract_ref: SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            surface_lifecycle_marker,
            device_participation,
            conflict_review,
            snapshots,
            secret_boundary,
            surface_parity,
            profile_roaming,
            outcome_coverage,
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
                sync_contract_ref: input.upstream.sync_contract_ref,
                participating_device_ids,
                reviewed_setting_ids,
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
            format!("sync_device_registry_certification: {}", self.record_id),
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
                "pillars: device_participation={} conflict_field_aware={} snapshot_provenance={} \
                 local_fallback={} secret_boundary={} merge_rules={} profile_roaming={} \
                 surfaces_one_truth={}",
                self.pillars.device_participation_truth,
                self.pillars.conflict_review_field_aware,
                self.pillars.snapshot_provenance_complete,
                self.pillars.local_fallback_proven,
                self.pillars.secret_boundary_held,
                self.pillars.merge_rules_enforced,
                self.pillars.profile_roaming_truth,
                self.pillars.surfaces_share_one_truth
            ),
            format!(
                "outcome_coverage: [{}]",
                self.outcome_coverage
                    .iter()
                    .map(|class| class.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ];
        lines.push("device_participation:".to_string());
        for row in &self.device_participation {
            lines.push(format!(
                "  - {} local={} state={} durability={} freshness={} conflict={} \
                 local_fallback={} inspectable={} last_sync={} conforms={}",
                row.device_id,
                row.is_local_device,
                row.participation_state.as_str(),
                row.profile_durability.as_str(),
                row.sync_freshness,
                row.conflict_class,
                row.local_authoritative_fallback,
                row.inspectable_without_mutation,
                row.last_successful_sync.as_deref().unwrap_or("never"),
                row.conforms
            ));
        }
        lines.push("conflict_review:".to_string());
        for row in &self.conflict_review {
            lines.push(format!(
                "  - {} scope={} outcome={} merge={} category={} overwrites_local={} \
                 protected={} local_authoritative={} widens_authority={} conforms={}",
                row.setting_id,
                row.conflicting_scope,
                row.outcome_class.as_str(),
                row.merge_class.as_str(),
                row.setting_category.as_str(),
                row.overwrites_local,
                row.protected_before_overwrite,
                row.local_authoritative,
                row.widens_authority,
                row.conforms
            ));
        }
        lines.push("snapshots:".to_string());
        for row in &self.snapshots {
            lines.push(format!(
                "  - {} producer={}/{} hash={} provenance={} included=[{}] excluded=[{}] \
                 forbidden={} conforms={}",
                row.snapshot_class.as_str(),
                row.producer_aureline_version,
                row.producer_schema_version,
                row.integrity_hash,
                row.source_provenance,
                row.included_state_classes
                    .iter()
                    .map(|class| class.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                row.excluded_state_classes
                    .iter()
                    .map(|class| class.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                row.carries_forbidden_state_class,
                row.conforms
            ));
        }
        lines.push("secret_boundary:".to_string());
        for row in &self.secret_boundary {
            lines.push(format!(
                "  - {} lane={} excluded={} reference_only_allowed={} conforms={}",
                row.state_class.as_str(),
                row.lane,
                row.excluded,
                row.reference_only_allowed,
                row.conforms
            ));
        }
        lines.push("surface_parity:".to_string());
        for row in &self.surface_parity {
            lines.push(format!(
                "  - {} consumes_shared_record={} clones_prose={} marker={} conforms={}",
                row.surface_class.as_str(),
                row.consumes_shared_record,
                row.clones_prose,
                row.surface_marker.as_str(),
                row.conforms
            ));
        }
        lines.push("profile_roaming:".to_string());
        lines.push(format!(
            "  managed_sync_available={} local_authority_retained={} temporary_excluded={} \
             durability={} retention_days={} originating_revision={} conforms={}",
            self.profile_roaming.managed_sync_available,
            self.profile_roaming.local_launch_edit_authority_retained,
            self.profile_roaming.temporary_profiles_excluded,
            self.profile_roaming.active_profile_durability.as_str(),
            self.profile_roaming
                .remaining_retention_days
                .map(|days| days.to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            self.profile_roaming.originating_profile_revision,
            self.profile_roaming.conforms
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

fn snapshot_order(class: SnapshotClass) -> usize {
    SnapshotClass::REQUIRED
        .iter()
        .position(|candidate| *candidate == class)
        .unwrap_or(usize::MAX)
}

fn state_class_order(class: StateClass) -> usize {
    const ORDER: [StateClass; 10] = [
        StateClass::ScalarSettings,
        StateClass::Keybindings,
        StateClass::Tasks,
        StateClass::LaunchConfigs,
        StateClass::WorksetDefinitions,
        StateClass::ExtensionInventoryRefs,
        StateClass::MachineLocalTopology,
        StateClass::DirtyBufferJournals,
        StateClass::SecretMaterial,
        StateClass::ReferenceOnlyMetadata,
    ];
    ORDER
        .iter()
        .position(|candidate| *candidate == class)
        .unwrap_or(usize::MAX)
}

// ---------------------------------------------------------------------------
// Recovery vocabulary
// ---------------------------------------------------------------------------

/// Closed recovery-action vocabulary exposed on a certification record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationRecoveryAction {
    /// Open the device-and-sync registry — the authoritative surface.
    OpenDeviceRegistry,
    /// Inspect a field-aware conflict review before apply.
    InspectConflictReview,
    /// Inspect device participation without opting into a mutating sync action.
    InspectDeviceParticipation,
    /// Export a redacted sync / device-registry support packet.
    ExportSyncSupport,
}

impl CertificationRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDeviceRegistry => "open_device_registry",
            Self::InspectConflictReview => "inspect_conflict_review",
            Self::InspectDeviceParticipation => "inspect_device_participation",
            Self::ExportSyncSupport => "export_sync_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenDeviceRegistry => "Open device & sync registry",
            Self::InspectConflictReview => "Inspect conflict review",
            Self::InspectDeviceParticipation => "Inspect device participation",
            Self::ExportSyncSupport => "Export sync support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenDeviceRegistry => RecoveryActionRole::Primary,
            Self::InspectConflictReview | Self::InspectDeviceParticipation => {
                RecoveryActionRole::Recovery
            }
            Self::ExportSyncSupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every record must expose, in rendered order.
    pub const REQUIRED: [Self; 4] = [
        Self::OpenDeviceRegistry,
        Self::InspectConflictReview,
        Self::InspectDeviceParticipation,
        Self::ExportSyncSupport,
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
