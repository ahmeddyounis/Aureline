//! Shared Start Center / `Open recent` / command-palette, system-open /
//! file-association, protocol / deep-link / browser-handoff, CLI / headless, and
//! support / diagnostics / docs consumers for the frozen M5 project-entry
//! components.
//!
//! This module is the M05-842 first-consumer adoption lane over the frozen M5
//! project-entry component matrix
//! ([`crate::m5_project_entry_components`]). Where the freeze matrix defines the
//! ten reusable start-center quick-action, recent-work, workspace-switcher,
//! restore-prompt, entry-chooser, entry-review, destination-collision,
//! post-entry-handoff, admission-checkpoint, and archetype-readiness cards,
//! rows, and sheets, this lane proves those families are reusable *primitives*
//! rather than one Start Center page plus a handful of flow-specific dialogs.
//! It adopts them across five claimed M5 project-entry consumer classes:
//!
//! 1. a Start Center / `Open recent` / command-palette surface,
//! 2. a system-open / file-association intake surface,
//! 3. a protocol / deep-link / browser-mobile handoff surface,
//! 4. a CLI / headless entry surface, and
//! 5. a support / diagnostics + docs/help lane (AC3).
//!
//! Each [`EntryConsumerRow`] points back to exactly one canonical component
//! family (its per-family schema + the shared release-proof packet) instead of
//! cloning surface-local entry vocabulary. Every consumer — even a read-only,
//! inspect-only, export-only, review-required, or policy-blocked one — keeps the
//! identical entry-verb / literal-target / resulting-mode /
//! write-scope-trust-host-auth / restore-or-first-useful-work labels, the
//! identical `command_id` for a given entry verb, and the identical
//! degraded-state vocabulary (missing target, remote-unreachable, policy-blocked,
//! cached-only, partial-restore, authority-expired). A narrower consumer
//! discloses the reduction with a reduced-capability banner (and, when it punts
//! to another surface, a desktop / companion / browser / handoff-packet note)
//! rather than renaming or dropping governed state, so deep-link, system-open,
//! and headless lanes never fork entry vocabulary by client, trigger, or
//! platform handoff origin.
//!
//! The packet is metadata-only: raw file paths, clone URLs, credentials, remote
//! hosts, and device identifiers never cross this boundary; the packet carries
//! only typed class tokens, opaque entry-object / command refs, booleans, and
//! redacted labels. This is what lets support and automation reconstruct *what
//! entry path the user actually took* (AC3) from the same object IDs the user
//! saw, without leaking the literal target.
//!
//! The boundary schema is
//! [`schemas/ui/m5-project-entry-component-consumer.schema.json`](../../../../schemas/ui/m5-project-entry-component-consumer.schema.json).
//! The contract doc is
//! [`docs/opening-projects/m5_project_entry_component_consumer_contract.md`](../../../../docs/opening-projects/m5_project_entry_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on the M05-842 consumer packet.
pub const ENTRY_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EntryConsumerPacket`].
pub const ENTRY_CONSUMER_RECORD_KIND: &str = "m5_project_entry_component_consumer_packet";

/// Stable record-kind tag carried by each [`EntryConsumerRow`].
pub const ENTRY_CONSUMER_ROW_RECORD_KIND: &str = "m5_project_entry_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const ENTRY_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-project-entry-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const ENTRY_CONSUMER_DOC_REF: &str =
    "docs/opening-projects/m5_project_entry_component_consumer_contract.md";

/// Repo-relative path of the frozen project-entry component matrix fixture these
/// consumers adopt.
pub const ENTRY_CONSUMER_MATRIX_REF: &str =
    crate::m5_project_entry_components::M5_PROJECT_ENTRY_COMPONENT_FIXTURE_REF;

/// Repo-relative path of the shared frozen component schema.
pub const ENTRY_CONSUMER_SHARED_SCHEMA_REF: &str =
    "schemas/ui/m5-project-entry-component.schema.json";

/// Repo-relative path of the frozen release-proof packet every consumer points
/// back to as the canonical first-resolved truth.
pub const ENTRY_CONSUMER_CANONICAL_PACKET_REF: &str =
    "artifacts/release/m5-project-entry-component-proof/packet.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const ENTRY_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-project-entry-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const ENTRY_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-project-entry-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const ENTRY_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-project-entry-component-consumer-proof/report.md";

/// The controlled label families a consumer must preserve identically across
/// every surface. These are the track-invariant truth pillars of the
/// project-entry components: the distinct entry verb, the literal target, the
/// resulting mode, the write-scope / trust / host / auth posture, and the
/// restore-fidelity or first-useful-work routing. The union of every row's
/// `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 5] = [
    "entry_verb",
    "literal_target",
    "resulting_mode",
    "write_scope_trust_host_auth",
    "restore_or_first_useful_work",
];

/// The canonical degraded-state vocabulary every consumer keeps visible even
/// when narrowed or export-only. These are the missing-target, policy-blocked,
/// remote-unavailable, and partial-restore contexts the spec requires to keep
/// parity across desktop and exported evidence.
pub const CANONICAL_DEGRADED_STATE_VOCAB: [&str; 6] = [
    "missing_target",
    "remote_unreachable",
    "policy_blocked",
    "cached_only",
    "partial_restore",
    "authority_expired",
];

/// The ten frozen M5 project-entry component families this lane adopts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProjectEntryComponentFamily {
    /// Start Center quick-action card.
    StartCenterQuickActionCard,
    /// Recent-work row.
    RecentWorkRow,
    /// Workspace-switcher entry.
    WorkspaceSwitcherEntry,
    /// Restore-prompt card.
    RestorePromptCard,
    /// Entry-chooser row.
    EntryChooserRow,
    /// Entry-review sheet.
    EntryReviewSheet,
    /// Destination-collision sheet.
    DestinationCollisionSheet,
    /// Post-entry handoff card.
    PostEntryHandoffCard,
    /// Admission-checkpoint card.
    AdmissionCheckpointCard,
    /// Archetype-readiness row.
    ArchetypeReadinessRow,
}

impl M5ProjectEntryComponentFamily {
    /// Every frozen family, in declaration order.
    pub const ALL: [M5ProjectEntryComponentFamily; 10] = [
        M5ProjectEntryComponentFamily::StartCenterQuickActionCard,
        M5ProjectEntryComponentFamily::RecentWorkRow,
        M5ProjectEntryComponentFamily::WorkspaceSwitcherEntry,
        M5ProjectEntryComponentFamily::RestorePromptCard,
        M5ProjectEntryComponentFamily::EntryChooserRow,
        M5ProjectEntryComponentFamily::EntryReviewSheet,
        M5ProjectEntryComponentFamily::DestinationCollisionSheet,
        M5ProjectEntryComponentFamily::PostEntryHandoffCard,
        M5ProjectEntryComponentFamily::AdmissionCheckpointCard,
        M5ProjectEntryComponentFamily::ArchetypeReadinessRow,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterQuickActionCard => "start_center_quick_action_card",
            Self::RecentWorkRow => "recent_work_row",
            Self::WorkspaceSwitcherEntry => "workspace_switcher_entry",
            Self::RestorePromptCard => "restore_prompt_card",
            Self::EntryChooserRow => "entry_chooser_row",
            Self::EntryReviewSheet => "entry_review_sheet",
            Self::DestinationCollisionSheet => "destination_collision_sheet",
            Self::PostEntryHandoffCard => "post_entry_handoff_card",
            Self::AdmissionCheckpointCard => "admission_checkpoint_card",
            Self::ArchetypeReadinessRow => "archetype_readiness_row",
        }
    }
}

/// The canonical per-family schema that defines a family's contract. Consumers
/// must point at this schema instead of inventing a surface-local one.
pub fn canonical_schema_ref_for(family: M5ProjectEntryComponentFamily) -> &'static str {
    use M5ProjectEntryComponentFamily::*;
    match family {
        StartCenterQuickActionCard => "schemas/ui/m5-start-center-quick-action-card.schema.json",
        RecentWorkRow => "schemas/ui/m5-recent-work-row.schema.json",
        WorkspaceSwitcherEntry => "schemas/ui/m5-workspace-switcher-entry.schema.json",
        RestorePromptCard => "schemas/ui/m5-restore-prompt-card.schema.json",
        EntryChooserRow => "schemas/ui/m5-entry-chooser-row.schema.json",
        EntryReviewSheet => "schemas/ui/m5-entry-review-sheet.schema.json",
        DestinationCollisionSheet => "schemas/ui/m5-destination-collision-sheet.schema.json",
        PostEntryHandoffCard => "schemas/ui/m5-post-entry-handoff-card.schema.json",
        AdmissionCheckpointCard => "schemas/ui/m5-admission-checkpoint-card.schema.json",
        ArchetypeReadinessRow => "schemas/ui/m5-archetype-readiness-row.schema.json",
    }
}

/// The canonical release-proof packet that certifies a family's first resolved
/// truth. All ten frozen families share the one project-entry release proof, so
/// consumers point back to it rather than cloning it.
pub const fn canonical_packet_ref_for(_family: M5ProjectEntryComponentFamily) -> &'static str {
    ENTRY_CONSUMER_CANONICAL_PACKET_REF
}

/// A distinct M5 project-entry verb. Each verb owns exactly one canonical
/// `command_id`; a consumer that dispatches the verb must reuse that id rather
/// than minting a client-, trigger-, or platform-specific command. This is the
/// heart of the "entry surfaces no longer fork vocabulary" acceptance criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EntryVerb {
    /// Open a local file, folder, or repository root.
    Open,
    /// Reopen a recent project from the recent-work list.
    OpenRecent,
    /// Clone a remote repository.
    Clone,
    /// Import a portable state package or handoff packet.
    Import,
    /// Restore a prior session or recovery checkpoint.
    Restore,
    /// Resume a live or managed session.
    Resume,
}

impl M5EntryVerb {
    /// Every entry verb, in declaration order.
    pub const ALL: [M5EntryVerb; 6] = [
        M5EntryVerb::Open,
        M5EntryVerb::OpenRecent,
        M5EntryVerb::Clone,
        M5EntryVerb::Import,
        M5EntryVerb::Restore,
        M5EntryVerb::Resume,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::OpenRecent => "open_recent",
            Self::Clone => "clone",
            Self::Import => "import",
            Self::Restore => "restore",
            Self::Resume => "resume",
        }
    }

    /// The single canonical command id every surface must reuse for this verb.
    /// These ids already exist in the shell command registry; the consumer lane
    /// forbids forking them per client or platform handoff origin.
    pub const fn canonical_command_id(self) -> &'static str {
        match self {
            Self::Open => "cmd:workspace.open.target",
            Self::OpenRecent => "cmd:start_center.open_recent",
            Self::Clone => "cmd:workspace.clone_repository",
            Self::Import => "cmd:workspace.import.bundle",
            Self::Restore => "cmd:workspace.restore_from_checkpoint",
            Self::Resume => "cmd:remote.open_session",
        }
    }
}

/// The five claimed M5 project-entry consumer classes that must each adopt at
/// least one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerGroup {
    /// A Start Center / `Open recent` / command-palette surface.
    StartCenterPalette,
    /// A system-open / file-association intake surface.
    SystemOpenIntake,
    /// A protocol / deep-link / browser-mobile handoff surface.
    DeepLinkHandoff,
    /// A CLI / headless entry surface.
    CliHeadless,
    /// A support / diagnostics + docs/help lane (AC3).
    SupportDiagnosticsDocs,
}

impl ConsumerGroup {
    /// Every consumer group that must be present for cross-surface reuse.
    pub const ALL: [ConsumerGroup; 5] = [
        ConsumerGroup::StartCenterPalette,
        ConsumerGroup::SystemOpenIntake,
        ConsumerGroup::DeepLinkHandoff,
        ConsumerGroup::CliHeadless,
        ConsumerGroup::SupportDiagnosticsDocs,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterPalette => "start_center_palette",
            Self::SystemOpenIntake => "system_open_intake",
            Self::DeepLinkHandoff => "deep_link_handoff",
            Self::CliHeadless => "cli_headless",
            Self::SupportDiagnosticsDocs => "support_diagnostics_docs",
        }
    }

    /// True when this group is a platform handoff origin (deep-link or
    /// system-open) whose rows must preserve literal target and resulting mode
    /// without special-case copy.
    pub const fn is_handoff_origin(self) -> bool {
        matches!(self, Self::SystemOpenIntake | Self::DeepLinkHandoff)
    }
}

/// The concrete M5 project-entry surface a component is embedded in. Each surface
/// belongs to exactly one [`ConsumerGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EntryConsumerSurface {
    /// The Start Center home surface.
    StartCenterHome,
    /// The `Open recent` list.
    OpenRecentList,
    /// The command palette.
    CommandPalette,
    /// System-open / file-association intake.
    SystemOpenFileAssociation,
    /// Drag-and-drop intake.
    DragAndDropIntake,
    /// A protocol / deep-link entry surface.
    ProtocolDeepLink,
    /// A browser / mobile handoff surface.
    BrowserMobileHandoff,
    /// The CLI entry surface.
    CliEntry,
    /// A headless automation entry surface.
    HeadlessAutomation,
    /// The support / export replay surface.
    SupportExportReplay,
    /// The admin / diagnostics surface.
    AdminDiagnostics,
    /// The docs / help center.
    HelpCenterDocs,
}

impl M5EntryConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [M5EntryConsumerSurface; 12] = [
        M5EntryConsumerSurface::StartCenterHome,
        M5EntryConsumerSurface::OpenRecentList,
        M5EntryConsumerSurface::CommandPalette,
        M5EntryConsumerSurface::SystemOpenFileAssociation,
        M5EntryConsumerSurface::DragAndDropIntake,
        M5EntryConsumerSurface::ProtocolDeepLink,
        M5EntryConsumerSurface::BrowserMobileHandoff,
        M5EntryConsumerSurface::CliEntry,
        M5EntryConsumerSurface::HeadlessAutomation,
        M5EntryConsumerSurface::SupportExportReplay,
        M5EntryConsumerSurface::AdminDiagnostics,
        M5EntryConsumerSurface::HelpCenterDocs,
    ];

    /// The consumer group this surface belongs to.
    pub const fn consumer_group(self) -> ConsumerGroup {
        match self {
            Self::StartCenterHome | Self::OpenRecentList | Self::CommandPalette => {
                ConsumerGroup::StartCenterPalette
            }
            Self::SystemOpenFileAssociation | Self::DragAndDropIntake => {
                ConsumerGroup::SystemOpenIntake
            }
            Self::ProtocolDeepLink | Self::BrowserMobileHandoff => ConsumerGroup::DeepLinkHandoff,
            Self::CliEntry | Self::HeadlessAutomation => ConsumerGroup::CliHeadless,
            Self::SupportExportReplay | Self::AdminDiagnostics | Self::HelpCenterDocs => {
                ConsumerGroup::SupportDiagnosticsDocs
            }
        }
    }

    /// True when this surface is a docs / help reference surface (AC3).
    pub const fn is_docs_help(self) -> bool {
        matches!(self, Self::HelpCenterDocs)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterHome => "start_center_home",
            Self::OpenRecentList => "open_recent_list",
            Self::CommandPalette => "command_palette",
            Self::SystemOpenFileAssociation => "system_open_file_association",
            Self::DragAndDropIntake => "drag_and_drop_intake",
            Self::ProtocolDeepLink => "protocol_deep_link",
            Self::BrowserMobileHandoff => "browser_mobile_handoff",
            Self::CliEntry => "cli_entry",
            Self::HeadlessAutomation => "headless_automation",
            Self::SupportExportReplay => "support_export_replay",
            Self::AdminDiagnostics => "admin_diagnostics",
            Self::HelpCenterDocs => "help_center_docs",
        }
    }
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, review-required,
/// export-only, policy-blocked) but never rename or drop the governed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (perform the entry verb directly).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Review-required: the verb is staged behind an explicit review before any
    /// write, clone, import, restore, or scope widening.
    ReviewRequired,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::ReviewRequired,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::ReviewRequired => "review_required",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot complete the
/// entry verb locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders and completes the component in-place.
    None,
    /// Punt to the desktop shell to complete the entry.
    DesktopShell,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a read-only browser surface.
    BrowserReadonly,
    /// Punt to a portable handoff / support packet.
    HandoffPacket,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore
    /// must carry a desktop / companion / browser / handoff note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DesktopShell => "desktop_shell",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full entry-verb / target / mode / scope / restore label parity.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// entry-verb / target / mode identity support and automation need to
/// reconstruct the entry path).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// control it drops relative to the full desktop entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical project-entry component family on one M5
/// entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryConsumerRow {
    /// Record kind; must equal [`ENTRY_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ENTRY_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_group: ConsumerGroup,
    /// The concrete entry surface; must belong to `consumer_group`.
    pub consumer_surface: M5EntryConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5ProjectEntryComponentFamily,
    /// The distinct entry verb this consumer's primary action dispatches.
    pub entry_verb: M5EntryVerb,
    /// The command id the consumer dispatches. Must equal
    /// `entry_verb.canonical_command_id()`.
    pub command_id: String,
    /// The canonical per-family schema. Must equal
    /// `canonical_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical release-proof packet(s) this consumer points back to. Must
    /// contain `canonical_packet_ref_for(component_family)`.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local entry prose.
    pub references_canonical_not_local_prose: bool,
    /// An opaque, redaction-safe ref to the entry object the user acted on, so
    /// support and automation can reconstruct the entry path (AC3) without
    /// leaking the literal target.
    pub entry_object_ref: String,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The degraded-state vocabulary the consumer keeps visible even when
    /// narrowed.
    #[serde(default)]
    pub degraded_state_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The desktop / companion / browser / handoff note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl EntryConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared group matches the row's declared group.
    pub fn surface_group_consistent(&self) -> bool {
        self.consumer_surface.consumer_group() == self.consumer_group
    }

    /// AC (no fork): the consumer dispatches the canonical command id for its
    /// entry verb rather than a client-, trigger-, or platform-specific one.
    pub fn command_id_is_canonical(&self) -> bool {
        self.command_id == self.entry_verb.canonical_command_id()
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared schema matches the family, a release-proof packet is
    /// referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_schema_ref_for(self.component_family)
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == canonical_packet_ref_for(self.component_family))
            && self.references_canonical_not_local_prose
    }

    /// AC2 (parity): the consumer preserves the family's controlled label
    /// families and degraded-state vocabulary rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.degraded_state_vocab.is_empty()
            && self
                .degraded_state_vocab
                .iter()
                .all(|v| CANONICAL_DEGRADED_STATE_VOCAB.contains(&v.as_str()))
    }

    /// AC (deep-link / system-open): a platform-handoff-origin consumer
    /// preserves the literal-target and resulting-mode truth so a deep link or
    /// system-open never drops target identity or resulting mode into
    /// special-case copy.
    pub fn preserves_handoff_target_truth(&self) -> bool {
        if !self.consumer_group.is_handoff_origin() {
            return true;
        }
        let has = |f: &str| self.preserved_label_families.iter().any(|v| v == f);
        has("literal_target") && has("resulting_mode")
    }

    /// AC3: the row carries the opaque entry-object ref and canonical command id
    /// support and automation reconstruct the taken entry path from.
    pub fn supports_entry_path_reconstruction(&self) -> bool {
        !self.entry_object_ref.trim().is_empty()
            && self.command_id_is_canonical()
            && self.copy_export.is_complete()
    }

    /// AC2 (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == ENTRY_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == ENTRY_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.command_id.trim().is_empty()
            && !self.entry_object_ref.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_packet_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} group={group} family={family} verb={verb} command={command} \
authority={authority} label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            group = self.consumer_group.as_str(),
            family = self.component_family.as_str(),
            verb = self.entry_verb.as_str(),
            command = self.command_id,
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-842 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryConsumerSummary {
    pub row_count: usize,
    pub consumer_group_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub entry_verb_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_use_canonical_command_id: bool,
    pub all_handoff_rows_preserve_target_truth: bool,
    pub all_rows_reconstructable: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub command_ids_stable_across_surfaces: bool,
    pub start_center_palette_consumer_present: bool,
    pub system_open_intake_consumer_present: bool,
    pub deep_link_handoff_consumer_present: bool,
    pub cli_headless_consumer_present: bool,
    pub support_diagnostics_docs_consumer_present: bool,
    pub docs_help_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub families_reused_across_groups: usize,
}

/// Constructor input for [`EntryConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<EntryConsumerRow>,
}

/// Checked-in M05-842 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<EntryConsumerRow>,
    pub summary: EntryConsumerSummary,
}

impl EntryConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: EntryConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: ENTRY_CONSUMER_SCHEMA_VERSION,
            record_kind: ENTRY_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: EntryConsumerSummary {
                row_count: 0,
                consumer_group_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                entry_verb_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_use_canonical_command_id: false,
                all_handoff_rows_preserve_target_truth: false,
                all_rows_reconstructable: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                command_ids_stable_across_surfaces: false,
                start_center_palette_consumer_present: false,
                system_open_intake_consumer_present: false,
                deep_link_handoff_consumer_present: false,
                cli_headless_consumer_present: false,
                support_diagnostics_docs_consumer_present: false,
                docs_help_reference_present: false,
                label_family_coverage_complete: false,
                families_reused_across_groups: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ProjectEntryComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// groups — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_groups(&self) -> usize {
        M5ProjectEntryComponentFamily::ALL
            .iter()
            .filter(|family| {
                let groups: BTreeSet<ConsumerGroup> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_group)
                    .collect();
                groups.len() >= 2
            })
            .count()
    }

    /// Whether every entry verb maps to exactly one command id across every
    /// surface — no surface forks the command by client, trigger, or platform.
    pub fn command_ids_stable_across_surfaces(&self) -> bool {
        let mut per_verb: BTreeMap<M5EntryVerb, BTreeSet<&str>> = BTreeMap::new();
        for row in &self.rows {
            per_verb
                .entry(row.entry_verb)
                .or_default()
                .insert(row.command_id.as_str());
        }
        per_verb.values().all(|ids| ids.len() <= 1)
    }

    /// Whether some docs / help surface references the canonical families (AC3).
    pub fn has_docs_help_reference(&self) -> bool {
        self.rows
            .iter()
            .any(|r| r.consumer_surface.is_docs_help() && r.references_canonical_not_local_prose)
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EntryConsumerSummary {
        let mut groups = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        let mut verbs = BTreeSet::new();
        for row in &self.rows {
            groups.insert(row.consumer_group);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
            verbs.insert(row.entry_verb);
        }

        let has_group = |g: ConsumerGroup| groups.contains(&g);
        let covered = self.covered_label_families();

        EntryConsumerSummary {
            row_count: self.rows.len(),
            consumer_group_count: groups.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            entry_verb_count: verbs.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(EntryConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(EntryConsumerRow::preserves_labels),
            all_rows_use_canonical_command_id: self
                .rows
                .iter()
                .all(EntryConsumerRow::command_id_is_canonical),
            all_handoff_rows_preserve_target_truth: self
                .rows
                .iter()
                .all(EntryConsumerRow::preserves_handoff_target_truth),
            all_rows_reconstructable: self
                .rows
                .iter()
                .all(EntryConsumerRow::supports_entry_path_reconstruction),
            all_narrowed_rows_disclose: self.rows.iter().all(EntryConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            command_ids_stable_across_surfaces: self.command_ids_stable_across_surfaces(),
            start_center_palette_consumer_present: has_group(ConsumerGroup::StartCenterPalette),
            system_open_intake_consumer_present: has_group(ConsumerGroup::SystemOpenIntake),
            deep_link_handoff_consumer_present: has_group(ConsumerGroup::DeepLinkHandoff),
            cli_headless_consumer_present: has_group(ConsumerGroup::CliHeadless),
            support_diagnostics_docs_consumer_present: has_group(
                ConsumerGroup::SupportDiagnosticsDocs,
            ),
            docs_help_reference_present: self.has_docs_help_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            families_reused_across_groups: self.families_reused_across_groups(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EntryConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != ENTRY_CONSUMER_SCHEMA_VERSION {
            violations.push(EntryConsumerViolation::SchemaVersion {
                expected: ENTRY_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != ENTRY_CONSUMER_RECORD_KIND {
            violations.push(EntryConsumerViolation::RecordKind {
                expected: ENTRY_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EntryConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_groups = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EntryConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_groups.insert(row.consumer_group);

            if !row.is_complete() {
                violations.push(EntryConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer group.
            if !row.surface_group_consistent() {
                violations.push(EntryConsumerViolation::SurfaceGroupMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(EntryConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC (no fork): canonical command id per entry verb.
            if !row.command_id_is_canonical() {
                violations.push(EntryConsumerViolation::NonCanonicalCommandId {
                    id: row.row_id.clone(),
                });
            }

            // AC2: controlled label families / degraded vocab preserved.
            if !row.preserves_labels() {
                violations.push(EntryConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC (deep-link / system-open): literal target + resulting mode kept.
            if !row.preserves_handoff_target_truth() {
                violations.push(EntryConsumerViolation::HandoffDropsTargetTruth {
                    id: row.row_id.clone(),
                });
            }

            // AC3: entry path is reconstructable from opaque object + command id.
            if !row.supports_entry_path_reconstruction() {
                violations.push(EntryConsumerViolation::EntryPathNotReconstructable {
                    id: row.row_id.clone(),
                });
            }

            // AC2: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(EntryConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(EntryConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // Cross-surface reuse spans all five claimed consumer classes.
        for group in ConsumerGroup::ALL {
            if !seen_groups.contains(&group) {
                violations.push(EntryConsumerViolation::MissingConsumerGroup { group });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5ProjectEntryComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(EntryConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer groups
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_groups() == 0 {
            violations.push(EntryConsumerViolation::NoFamilyReusedAcrossGroups);
        }

        // AC (no fork): entry verbs resolve to one stable command id per verb.
        if !self.command_ids_stable_across_surfaces() {
            violations.push(EntryConsumerViolation::CommandIdForkedAcrossSurfaces);
        }

        // AC2: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(EntryConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC3: a docs / help consumer references the canonical components rather
        // than cloning local entry vocabulary.
        if !self.has_docs_help_reference() {
            violations.push(EntryConsumerViolation::MissingDocsHelpReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(EntryConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(EntryConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_group,consumer_surface,component_family,entry_verb,command_id,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{group},{surface},{family},{verb},{command},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                group = row.consumer_group.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                verb = row.entry_verb.as_str(),
                command = row.command_id,
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Project-Entry Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer groups and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_group_count,
            self.represented_families().len(),
            M5ProjectEntryComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Entry verbs with stable command ids: {}\n",
            self.summary.entry_verb_count,
        ));
        out.push_str(&format!(
            "- Families reused across groups: {}\n",
            self.summary.families_reused_across_groups,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_project_entry_component_consumers_export(
) -> Result<EntryConsumerPacket, EntryConsumerArtifactError> {
    let packet: EntryConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-project-entry-component-consumer-proof/support_export.json"
    )))
    .map_err(EntryConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EntryConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum EntryConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EntryConsumerViolation>),
}

impl fmt::Display for EntryConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for EntryConsumerArtifactError {}

/// Validation failure for M05-842 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryConsumerViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    SurfaceGroupMismatch {
        id: String,
    },
    NotCanonicalFamily {
        id: String,
    },
    NonCanonicalCommandId {
        id: String,
    },
    LabelParityBroken {
        id: String,
    },
    HandoffDropsTargetTruth {
        id: String,
    },
    EntryPathNotReconstructable {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    MissingConsumerGroup {
        group: ConsumerGroup,
    },
    MissingFamilyCoverage {
        family: M5ProjectEntryComponentFamily,
    },
    NoFamilyReusedAcrossGroups,
    CommandIdForkedAcrossSurfaces,
    MissingLabelFamily {
        family: String,
    },
    MissingDocsHelpReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for EntryConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceGroupMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer group"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::NonCanonicalCommandId { id } => {
                write!(
                    f,
                    "row {id} forks the entry-verb command id instead of reusing the canonical one"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical entry-verb, literal-target, \
resulting-mode, write-scope/trust/host/auth, or restore/first-useful-work label"
                )
            }
            Self::HandoffDropsTargetTruth { id } => {
                write!(
                    f,
                    "deep-link / system-open row {id} drops literal-target or resulting-mode truth"
                )
            }
            Self::EntryPathNotReconstructable { id } => {
                write!(
                    f,
                    "row {id} cannot be reconstructed from its entry object ref and command id"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::MissingConsumerGroup { group } => {
                write!(f, "consumer group {group:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossGroups => write!(
                f,
                "no component family is adopted across two or more consumer groups"
            ),
            Self::CommandIdForkedAcrossSurfaces => write!(
                f,
                "an entry verb resolves to more than one command id across surfaces"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingDocsHelpReference => write!(
                f,
                "no docs / help consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for EntryConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    if lower.contains("get started") {
        return true;
    }
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "read only"
            | "read-only"
            | "offline"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_project_entry_component_consumers_packet() -> EntryConsumerPacket {
    EntryConsumerPacket::new(EntryConsumerPacketInput {
        packet_id: "m5-project-entry-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: ENTRY_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:project-entry-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn degraded_vocab() -> Vec<String> {
    CANONICAL_DEGRADED_STATE_VOCAB
        .iter()
        .map(|v| (*v).to_owned())
        .collect()
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5EntryConsumerSurface,
    component_family: M5ProjectEntryComponentFamily,
    entry_verb: M5EntryVerb,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> EntryConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    EntryConsumerRow {
        record_kind: ENTRY_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: ENTRY_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_group: consumer_surface.consumer_group(),
        consumer_surface,
        component_family,
        entry_verb,
        command_id: entry_verb.canonical_command_id().to_owned(),
        canonical_family_schema_ref: canonical_schema_ref_for(component_family).to_owned(),
        canonical_packet_refs: vec![canonical_packet_ref_for(component_family).to_owned()],
        references_canonical_not_local_prose: true,
        entry_object_ref: format!("entry-object:{row_id}"),
        authority_mode,
        preserved_label_families: labels(label_families),
        degraded_state_vocab: degraded_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        source_refs: vec![
            ENTRY_CONSUMER_MATRIX_REF.to_owned(),
            ENTRY_CONSUMER_SHARED_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<EntryConsumerRow> {
    use AuthorityMode::*;
    use HandoffTarget as H;
    use M5EntryConsumerSurface::*;
    use M5EntryVerb::*;
    use M5ProjectEntryComponentFamily::*;

    vec![
        // --- Start Center / Open recent / command palette ------------------
        row(
            "consumer:start-center:quick-action-open",
            StartCenterHome,
            StartCenterQuickActionCard,
            Open,
            FullInteractive,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "target_kind", "resulting_mode", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:start-center:recent-work-row",
            StartCenterHome,
            RecentWorkRow,
            OpenRecent,
            FullInteractive,
            &["entry_verb", "literal_target", "restore_or_first_useful_work"],
            &["entry_verb", "target_state", "restore_fidelity", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:open-recent:recent-work-row",
            OpenRecentList,
            RecentWorkRow,
            OpenRecent,
            FullInteractive,
            &["entry_verb", "literal_target", "restore_or_first_useful_work"],
            &["entry_verb", "target_state", "restore_fidelity", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:start-center:restore-prompt",
            StartCenterHome,
            RestorePromptCard,
            Restore,
            FullInteractive,
            &["entry_verb", "resulting_mode", "restore_or_first_useful_work"],
            &["entry_verb", "restore_fidelity", "resulting_mode", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:start-center:workspace-switcher",
            StartCenterHome,
            WorkspaceSwitcherEntry,
            Resume,
            FullInteractive,
            &["entry_verb", "literal_target", "restore_or_first_useful_work"],
            &["entry_verb", "object_identity", "restore_fidelity", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:palette:entry-chooser-open",
            CommandPalette,
            EntryChooserRow,
            Open,
            FullInteractive,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "target_kind", "resulting_mode", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:palette:entry-chooser-clone",
            CommandPalette,
            EntryChooserRow,
            Clone,
            FullInteractive,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "target_kind", "resulting_mode", "command_id"],
            H::None,
            "",
            None,
        ),
        // --- System-open / file-association intake -------------------------
        row(
            "consumer:system-open:entry-chooser-open",
            SystemOpenFileAssociation,
            EntryChooserRow,
            Open,
            FullInteractive,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "target_kind", "resulting_mode", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:system-open:entry-review-open",
            SystemOpenFileAssociation,
            EntryReviewSheet,
            Open,
            ReviewRequired,
            &[
                "entry_verb",
                "literal_target",
                "resulting_mode",
                "write_scope_trust_host_auth",
            ],
            &["entry_verb", "literal_target", "resulting_mode", "write_scope", "command_id"],
            H::DesktopShell,
            "handoff:system-open:entry-review-desktop-shell",
            Some(banner(
                "banner:system-open:entry-review-open",
                "Review-required system open: confirm the literal target, resulting mode, and write scope before Aureline opens what the OS handed off",
                ReviewRequired,
                &["open_without_review", "widen_scope_without_review"],
            )),
        ),
        row(
            "consumer:drag-drop:destination-collision-clone",
            DragAndDropIntake,
            DestinationCollisionSheet,
            Clone,
            ReviewRequired,
            &[
                "entry_verb",
                "literal_target",
                "resulting_mode",
                "write_scope_trust_host_auth",
            ],
            &["entry_verb", "literal_target", "resulting_mode", "collision_source", "command_id"],
            H::DesktopShell,
            "handoff:drag-drop:destination-collision-desktop-shell",
            Some(banner(
                "banner:drag-drop:destination-collision-clone",
                "Review-required drop collision: choose reuse, add-existing, clone-elsewhere, or reveal before any clone target is materialized",
                ReviewRequired,
                &["overwrite_existing", "retry_copy_without_choice"],
            )),
        ),
        // --- Protocol / deep-link / browser-mobile handoff -----------------
        row(
            "consumer:deep-link:entry-review-open",
            ProtocolDeepLink,
            EntryReviewSheet,
            Open,
            ReviewRequired,
            &[
                "entry_verb",
                "literal_target",
                "resulting_mode",
                "write_scope_trust_host_auth",
            ],
            &["entry_verb", "literal_target", "resulting_mode", "write_scope", "command_id"],
            H::DesktopShell,
            "handoff:deep-link:entry-review-desktop-shell",
            Some(banner(
                "banner:deep-link:entry-review-open",
                "Review-required deep link: the protocol target, resulting mode, and recovery path are shown before Aureline opens or writes anything",
                ReviewRequired,
                &["open_without_review", "trust_target_without_review"],
            )),
        ),
        row(
            "consumer:deep-link:review-link-handoff",
            ProtocolDeepLink,
            PostEntryHandoffCard,
            Open,
            ReadOnly,
            &[
                "entry_verb",
                "literal_target",
                "resulting_mode",
                "restore_or_first_useful_work",
            ],
            &["entry_verb", "literal_target", "resulting_mode", "first_useful_work_route", "command_id"],
            H::DesktopShell,
            "handoff:deep-link:review-link-desktop-shell",
            Some(banner(
                "banner:deep-link:review-link-handoff",
                "Read-only deep-link preview: read what a review link will open and where first useful work routes; the desktop shell performs the open",
                ReadOnly,
                &["perform_open", "route_first_useful_work"],
            )),
        ),
        row(
            "consumer:browser-mobile:entry-chooser-clone",
            BrowserMobileHandoff,
            EntryChooserRow,
            Clone,
            InspectOnly,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "target_kind", "resulting_mode", "command_id"],
            H::BrowserReadonly,
            "handoff:browser-mobile:entry-chooser-open-in-desktop",
            Some(banner(
                "banner:browser-mobile:entry-chooser-clone",
                "Inspect-only browser handoff: read the clone target and resulting mode; open the desktop app to perform the clone",
                InspectOnly,
                &["perform_clone", "choose_destination"],
            )),
        ),
        // --- CLI / headless entry ------------------------------------------
        row(
            "consumer:cli:entry-review-import",
            CliEntry,
            EntryReviewSheet,
            Import,
            FullInteractive,
            &[
                "entry_verb",
                "literal_target",
                "resulting_mode",
                "write_scope_trust_host_auth",
            ],
            &["entry_verb", "literal_target", "resulting_mode", "write_scope", "command_id"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:cli:archetype-readiness-open",
            CliEntry,
            ArchetypeReadinessRow,
            Open,
            ExportOnly,
            &["entry_verb", "literal_target", "restore_or_first_useful_work"],
            &["entry_verb", "archetype_class", "readiness_bucket", "confidence_class", "command_id"],
            H::HandoffPacket,
            "handoff:cli:archetype-readiness-support-packet",
            Some(banner(
                "banner:cli:archetype-readiness-open",
                "Export-only CLI readiness: emit the detected archetype, readiness bucket, and confidence for scripts; the desktop admission checkpoint acts on it",
                ExportOnly,
                &["run_setup_task", "install_bundle"],
            )),
        ),
        row(
            "consumer:headless:admission-checkpoint-open",
            HeadlessAutomation,
            AdmissionCheckpointCard,
            Open,
            InspectOnly,
            &["entry_verb", "literal_target", "write_scope_trust_host_auth"],
            &["entry_verb", "root_identity", "trust_class", "readiness_summary", "command_id"],
            H::DesktopShell,
            "handoff:headless:admission-checkpoint-desktop-shell",
            Some(banner(
                "banner:headless:admission-checkpoint-open",
                "Inspect-only headless admission: read root identity, trust class, and blocked-vs-optional readiness without auto-installing packs; continue-without stays available in the shell",
                InspectOnly,
                &["auto_install_bundle", "widen_trust_without_review"],
            )),
        ),
        // --- Support / diagnostics + docs/help (AC3) -----------------------
        row(
            "consumer:support-export:recent-work-row",
            SupportExportReplay,
            RecentWorkRow,
            OpenRecent,
            ExportOnly,
            &["entry_verb", "literal_target", "restore_or_first_useful_work"],
            &["entry_verb", "target_state", "restore_fidelity", "entry_object_ref", "command_id"],
            H::HandoffPacket,
            "handoff:support-export:recent-work-support-packet",
            Some(banner(
                "banner:support-export:recent-work-row",
                "Export-only support replay: reconstruct which recent project the user reopened and its restore fidelity from the support packet",
                ExportOnly,
                &["reopen_project", "clear_recent"],
            )),
        ),
        row(
            "consumer:support-export:post-entry-handoff-import",
            SupportExportReplay,
            PostEntryHandoffCard,
            Import,
            ExportOnly,
            &[
                "entry_verb",
                "literal_target",
                "restore_or_first_useful_work",
            ],
            &["entry_verb", "opened_object_ref", "follow_up_state", "entry_object_ref", "command_id"],
            H::HandoffPacket,
            "handoff:support-export:post-entry-handoff-support-packet",
            Some(banner(
                "banner:support-export:post-entry-handoff-import",
                "Export-only support replay: reconstruct what an import staged, what stayed intentionally not done, and the follow-up state from the support packet",
                ExportOnly,
                &["resume_setup", "open_minimal"],
            )),
        ),
        row(
            "consumer:admin-diagnostics:workspace-switcher",
            AdminDiagnostics,
            WorkspaceSwitcherEntry,
            Resume,
            ReadOnly,
            &["entry_verb", "literal_target", "restore_or_first_useful_work"],
            &["entry_verb", "object_identity", "restore_fidelity", "entry_object_ref", "command_id"],
            H::DesktopShell,
            "handoff:admin-diagnostics:workspace-switcher-desktop-shell",
            Some(banner(
                "banner:admin-diagnostics:workspace-switcher",
                "Read-only diagnostics switcher: read each workspace's identity, restore badges, and cross-window state; resuming stays on the desktop shell",
                ReadOnly,
                &["resume_workspace", "transfer_window"],
            )),
        ),
        row(
            "consumer:admin-diagnostics:destination-collision-clone",
            AdminDiagnostics,
            DestinationCollisionSheet,
            Clone,
            ReadOnly,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "literal_target", "resulting_mode", "collision_source", "command_id"],
            H::DesktopShell,
            "handoff:admin-diagnostics:destination-collision-desktop-shell",
            Some(banner(
                "banner:admin-diagnostics:destination-collision-clone",
                "Read-only diagnostics collision view: read why a clone collided and the safe reuse / add-existing / clone-elsewhere choices offered; acting stays on the desktop shell",
                ReadOnly,
                &["choose_safe_action", "reveal_in_filesystem"],
            )),
        ),
        row(
            "consumer:help-docs:entry-chooser-open",
            HelpCenterDocs,
            EntryChooserRow,
            Open,
            ReadOnly,
            &["entry_verb", "literal_target", "resulting_mode"],
            &["entry_verb", "target_kind", "resulting_mode", "keyboard_equivalent", "command_id"],
            H::None,
            "",
            Some(banner(
                "banner:help-docs:entry-chooser-open",
                "Read-only help reference: explains the distinct open / clone / import / restore chooser rows, their targets, resulting modes, and keyboard equivalents",
                ReadOnly,
                &["perform_entry"],
            )),
        ),
        row(
            "consumer:help-docs:restore-prompt",
            HelpCenterDocs,
            RestorePromptCard,
            Restore,
            ReadOnly,
            &["entry_verb", "resulting_mode", "restore_or_first_useful_work"],
            &["entry_verb", "restore_fidelity", "resulting_mode", "command_id"],
            H::None,
            "",
            Some(banner(
                "banner:help-docs:restore-prompt",
                "Read-only help reference: explains Exact restore, Compatible restore, Layout only, Recovered drafts, Evidence only, and No restore and the safest next action",
                ReadOnly,
                &["perform_restore"],
            )),
        ),
    ]
}
