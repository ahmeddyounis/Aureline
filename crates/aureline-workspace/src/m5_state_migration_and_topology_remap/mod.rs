//! M5 state-migration and display-topology-remap events: the place every schema migration,
//! imported-package restore, and display-topology remap becomes an explicit, reviewable event with
//! downgrade truth, so a restore never quietly changes layout or silently drops a meaningful
//! migration or remap decision.
//!
//! The serialization-and-restore matrix classifies *what* M5 may remember and the achievable
//! restore-fidelity classes; the restore-provenance cards attach the resulting fidelity to each
//! re-entry surface. This packet is the **migration/remap-facing** projection: one
//! [`MigrationRemapEvent`] per scenario in which remembered state became untrustworthy because the
//! schema changed, a package came from another machine or org, or the display topology shifted
//! enough to alter restored placement. It never invents its own fidelity language: every event
//! reuses the canonical [`RestoreFidelityClass`], [`RememberedArtifactClass`], [`RedactionExclusion`],
//! [`SchemaCondition`], [`DependencyCondition`], [`TopologyCondition`], [`EvidenceFreshness`],
//! [`MissingDependencyBehavior`], [`DowngradeReason`], and [`RecoveryPath`] vocabularies from
//! [`crate::m5_serialization_and_restore_matrix`], so migration/remap meaning cannot fork between
//! desktop restore, portable-state import, crash recovery, support-packet replay, and
//! companion/browser re-entry.
//!
//! - [`MigrationEventKind`] names the three event kinds this lane makes explicit: a schema
//!   migration, an imported-package provenance disclosure, and a display-topology remap.
//! - [`SchemaMigrationDetail`] records the migration result class, the schema-version jump, and the
//!   number of forward steps; [`ImportedPackageDetail`] records the package origin, channel match,
//!   path-handling posture, machine-local exclusion, and pre-restore disclosure;
//!   [`DisplayTopologyRemapDetail`] records the remap triggers, whether placement was materially
//!   altered, and how the layout was resolved.
//! - [`PriorArtifactAvailability`] records whether the pre-migration artifact is still reachable;
//!   the gate forbids [`PriorArtifactAvailability::PriorArtifactDiscarded`] so a migration or import
//!   never silently discards the old remembered state.
//! - [`MigrationRemapActionKind`] models the open-details, compare, and recovery-next-step
//!   affordances, exactly as the restore-provenance cards do.
//! - [`MigrationRemapConsumerBinding`] wires the parity surfaces — exported diagnostics, support
//!   packets, crash-recovery packets, and companion handoff — to this one packet so support can
//!   distinguish a deliberate platform remap from a generic restore failure.
//!
//! The packet is fail-closed. An event's achieved fidelity is the **weakest ceiling** implied by its
//! declared resulting fidelity and its schema, dependency, topology, and evidence-freshness
//! conditions ([`MigrationRemapEvent::achieved_fidelity`]); the published fidelity must equal it, so a
//! schema drift, a relocated dependency, a changed topology, or stale evidence can never publish an
//! exact restore by inertia. Every event-kind detail is cross-checked for consistency with those
//! conditions: a forward migration must read as a schema drift, a mixed-channel import must not claim
//! a clean schema match, a remapped path must read as a missing dependency, and a display-topology
//! remap must read as a changed topology — so the migration/remap label and the published fidelity
//! can never disagree. A missing dependency never silently deletes layout, a prior artifact is never
//! discarded, and an exact event must be a genuinely clean no-migration baseline.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-state-migration-and-topology-remap.json`
//! and embedded here. It is metadata-only: every field is a typed state, a count, an opaque ref, or
//! a plain-language label, and it carries no credential bodies, raw provider payloads, live authority
//! handles, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_serialization_and_restore_matrix::{
    DependencyCondition, DowngradeReason, EvidenceFreshness, MissingDependencyBehavior,
    RecoveryPath, RedactionExclusion, RememberedArtifactClass, RestoreFidelityClass,
    SchemaCondition, TopologyCondition,
};

/// Supported M5 state-migration and topology-remap packet schema version.
pub const M5_STATE_MIGRATION_REMAP_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_STATE_MIGRATION_REMAP_RECORD_KIND: &str = "m5_state_migration_and_topology_remap";

/// Repo-relative path to the checked-in packet.
pub const M5_STATE_MIGRATION_REMAP_PATH: &str =
    "artifacts/workspace/m5/m5-state-migration-and-topology-remap.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_STATE_MIGRATION_REMAP_SCHEMA_REF: &str =
    "schemas/workspace/m5-state-migration-and-topology-remap.schema.json";

/// Repo-relative path to the companion document.
pub const M5_STATE_MIGRATION_REMAP_DOC_REF: &str =
    "docs/workspace/m5/m5-state-migration-and-topology-remap.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_STATE_MIGRATION_REMAP_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-state-migration-and-topology-remap.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_STATE_MIGRATION_REMAP_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-state-migration-and-topology-remap";

/// Embedded checked-in packet JSON.
pub const M5_STATE_MIGRATION_REMAP_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-state-migration-and-topology-remap.json"
));

// --- Event kind ----------------------------------------------------------------------------------

/// One of the three kinds of remembered-state migration/remap event this lane makes explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationEventKind {
    /// The stored schema changed and was migrated (or could not be migrated) into the running build.
    SchemaMigration,
    /// A package came from another machine or org and its origin and exclusions are disclosed.
    ImportedPackageProvenance,
    /// The display topology shifted enough to materially alter restored window placement or layout.
    DisplayTopologyRemap,
}

impl MigrationEventKind {
    /// Every event kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SchemaMigration,
        Self::ImportedPackageProvenance,
        Self::DisplayTopologyRemap,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaMigration => "schema_migration",
            Self::ImportedPackageProvenance => "imported_package_provenance",
            Self::DisplayTopologyRemap => "display_topology_remap",
        }
    }
}

// --- Schema-migration detail ---------------------------------------------------------------------

/// The result class of a schema migration.
///
/// Ordered best-to-worst by [`MigrationResultClass::fidelity_ceiling`]: an unchanged schema needs no
/// migration and can restore exactly, a forward-migrated schema is a compatible downgrade, and an
/// unmigratable schema is held for manual review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationResultClass {
    /// The stored schema matched the running build; no migration was applied.
    SchemaUnchanged,
    /// The stored schema was forward-migrated into the running build; semantics preserved.
    ForwardMigrated,
    /// The stored schema cannot be migrated; the restore is held for manual review.
    Unmigratable,
}

impl MigrationResultClass {
    /// Every result class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SchemaUnchanged,
        Self::ForwardMigrated,
        Self::Unmigratable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaUnchanged => "schema_unchanged",
            Self::ForwardMigrated => "forward_migrated",
            Self::Unmigratable => "unmigratable",
        }
    }

    /// Highest restore fidelity this result class permits.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::SchemaUnchanged => RestoreFidelityClass::ExactRestore,
            Self::ForwardMigrated => RestoreFidelityClass::CompatibleRestore,
            Self::Unmigratable => RestoreFidelityClass::ManualReview,
        }
    }

    /// The schema condition this result class must agree with, so the migration label and the
    /// gate's published fidelity can never disagree.
    pub const fn required_schema_condition(self) -> SchemaCondition {
        match self {
            Self::SchemaUnchanged => SchemaCondition::SchemaMatch,
            Self::ForwardMigrated => SchemaCondition::SchemaForwardMigratable,
            Self::Unmigratable => SchemaCondition::SchemaUnmigratable,
        }
    }
}

/// The schema-migration detail for a [`MigrationEventKind::SchemaMigration`] event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaMigrationDetail {
    /// Migration result class.
    pub result_class: MigrationResultClass,
    /// Schema version the remembered state was stored at.
    pub from_schema_version: u32,
    /// Schema version the running build expects.
    pub to_schema_version: u32,
    /// Number of forward migration steps applied; zero when none ran.
    pub migration_steps: u32,
}

impl SchemaMigrationDetail {
    /// Whether the result class, version jump, and step count are mutually consistent.
    pub fn is_consistent(&self) -> bool {
        if self.to_schema_version < self.from_schema_version {
            return false;
        }
        match self.result_class {
            MigrationResultClass::SchemaUnchanged => {
                self.from_schema_version == self.to_schema_version && self.migration_steps == 0
            }
            MigrationResultClass::ForwardMigrated => {
                self.to_schema_version > self.from_schema_version && self.migration_steps >= 1
            }
            // An unmigratable schema could not run any forward step.
            MigrationResultClass::Unmigratable => self.migration_steps == 0,
        }
    }
}

// --- Imported-package detail ---------------------------------------------------------------------

/// Where an imported package came from, relative to this machine and org.
///
/// Anything other than [`PackageOriginClass::SameMachine`] is foreign: the import must exclude
/// machine-local anchors and disclose its origin before restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageOriginClass {
    /// Produced on this machine/install.
    SameMachine,
    /// Produced on another machine within the same org/sharing scope.
    SameOrgDifferentMachine,
    /// Produced on a machine outside this org.
    ForeignMachine,
    /// Produced by another organization entirely.
    ForeignOrg,
}

impl PackageOriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SameMachine,
        Self::SameOrgDifferentMachine,
        Self::ForeignMachine,
        Self::ForeignOrg,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameMachine => "same_machine",
            Self::SameOrgDifferentMachine => "same_org_different_machine",
            Self::ForeignMachine => "foreign_machine",
            Self::ForeignOrg => "foreign_org",
        }
    }

    /// Whether this origin is foreign — from another machine or org.
    pub const fn is_foreign(self) -> bool {
        !matches!(self, Self::SameMachine)
    }
}

/// Whether an imported package was produced on the same release channel as the running build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMatch {
    /// The package was produced on the same release channel.
    SameChannel,
    /// The package was produced on a different release channel; the producing build differs.
    MixedChannel,
}

impl ChannelMatch {
    /// Every channel-match state, in declaration order.
    pub const ALL: [Self; 2] = [Self::SameChannel, Self::MixedChannel];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameChannel => "same_channel",
            Self::MixedChannel => "mixed_channel",
        }
    }

    /// Whether the package crossed release channels.
    pub const fn is_mixed(self) -> bool {
        matches!(self, Self::MixedChannel)
    }
}

/// How machine-local paths in an imported package were handled.
///
/// Ordered best-to-worst by [`PathHandlingPosture::dependency_condition`]: portable relative paths
/// resolve as-is, remapped paths fall back to a slot-preserving relocation, and paths that cannot be
/// resolved are surfaced for review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathHandlingPosture {
    /// Paths were stored portably (workspace-relative) and resolve as-is.
    PathsPortableRelative,
    /// Machine-local paths were remapped onto this machine's roots; the slot is preserved.
    PathsRemappedToLocalRoots,
    /// Paths cannot be resolved on this machine; the import is surfaced for review.
    PathsRequireReview,
}

impl PathHandlingPosture {
    /// Every path-handling posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PathsPortableRelative,
        Self::PathsRemappedToLocalRoots,
        Self::PathsRequireReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PathsPortableRelative => "paths_portable_relative",
            Self::PathsRemappedToLocalRoots => "paths_remapped_to_local_roots",
            Self::PathsRequireReview => "paths_require_review",
        }
    }

    /// The dependency condition this posture must agree with, so a remapped or unresolved path can
    /// never read as a clean restore.
    pub const fn required_dependency_condition(self) -> DependencyCondition {
        match self {
            Self::PathsPortableRelative => DependencyCondition::DependenciesPresent,
            Self::PathsRemappedToLocalRoots => DependencyCondition::DependenciesPartialMissing,
            Self::PathsRequireReview => DependencyCondition::DependencyRootMissing,
        }
    }
}

/// The imported-package detail for a [`MigrationEventKind::ImportedPackageProvenance`] event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedPackageDetail {
    /// Where the package came from, relative to this machine and org.
    pub origin: PackageOriginClass,
    /// Whether the package crossed release channels.
    pub channel_match: ChannelMatch,
    /// How machine-local paths in the package were handled.
    pub path_handling: PathHandlingPosture,
    /// Opaque reference to the producer that wrote the package.
    pub producer_ref: String,
    /// Monotonic producer version that wrote the package.
    pub producer_version: u32,
    /// Opaque reference to the build that wrote the package.
    pub build_ref: String,
    /// True when machine-local anchors were excluded from the imported package.
    pub machine_local_excluded: bool,
    /// True when the origin and exclusions were disclosed before the restore was applied.
    pub disclosed_before_restore: bool,
}

impl ImportedPackageDetail {
    /// Whether the producer provenance refs are present and the version is set.
    pub fn provenance_is_complete(&self) -> bool {
        !self.producer_ref.trim().is_empty()
            && !self.build_ref.trim().is_empty()
            && self.producer_version != 0
    }

    /// Whether a foreign package discloses its origin and excludes machine-local anchors before
    /// restore, as the gate requires.
    pub fn foreign_disclosure_is_complete(&self) -> bool {
        if !self.origin.is_foreign() {
            return true;
        }
        self.machine_local_excluded && self.disclosed_before_restore
    }
}

// --- Display-topology-remap detail ---------------------------------------------------------------

/// A display change that can trigger a topology remap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapTrigger {
    /// Monitor count, resolution, or arrangement changed.
    MonitorGeometry,
    /// DPI or scaling factor changed.
    DpiScale,
    /// Fullscreen, snap, or maximize state changed.
    FullscreenSnapState,
    /// A monitor used by the saved layout was detached.
    MonitorDetached,
    /// A previously detached monitor was reattached.
    MonitorReattached,
}

impl RemapTrigger {
    /// Every remap trigger, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MonitorGeometry,
        Self::DpiScale,
        Self::FullscreenSnapState,
        Self::MonitorDetached,
        Self::MonitorReattached,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonitorGeometry => "monitor_geometry",
            Self::DpiScale => "dpi_scale",
            Self::FullscreenSnapState => "fullscreen_snap_state",
            Self::MonitorDetached => "monitor_detached",
            Self::MonitorReattached => "monitor_reattached",
        }
    }
}

/// How a display-topology remap resolved the restored layout.
///
/// A remap is never a corruption: at worst the layout slots are preserved while their contents
/// reopen as context. The two resolutions mirror the adapted/incompatible topology conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapResolution {
    /// Window placement was adapted onto the available displays; a compatible downgrade.
    PlacementAdaptedToAvailableDisplays,
    /// Only the pane/window layout was preserved; contents reopen as context.
    LayoutPreservedContentsReopened,
}

impl RemapResolution {
    /// Every remap resolution, in declaration order.
    pub const ALL: [Self; 2] = [
        Self::PlacementAdaptedToAvailableDisplays,
        Self::LayoutPreservedContentsReopened,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlacementAdaptedToAvailableDisplays => "placement_adapted_to_available_displays",
            Self::LayoutPreservedContentsReopened => "layout_preserved_contents_reopened",
        }
    }

    /// The topology condition this resolution must agree with, so a remap and the published fidelity
    /// can never disagree.
    pub const fn required_topology_condition(self) -> TopologyCondition {
        match self {
            Self::PlacementAdaptedToAvailableDisplays => TopologyCondition::TopologyAdapted,
            Self::LayoutPreservedContentsReopened => TopologyCondition::TopologyIncompatible,
        }
    }
}

/// The display-topology-remap detail for a [`MigrationEventKind::DisplayTopologyRemap`] event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayTopologyRemapDetail {
    /// Display changes that triggered the remap; at least one.
    #[serde(default)]
    pub triggers: Vec<RemapTrigger>,
    /// True when the change materially altered restored window placement or pane layout.
    pub materially_altered_placement: bool,
    /// How the layout was resolved.
    pub resolution: RemapResolution,
}

impl DisplayTopologyRemapDetail {
    /// Whether the remap names at least one trigger and materially altered placement — a remap is
    /// only recorded when it actually changed restored placement.
    pub fn is_recordable(&self) -> bool {
        !self.triggers.is_empty() && self.materially_altered_placement
    }
}

// --- Prior-artifact availability -----------------------------------------------------------------

/// Whether the pre-migration remembered state is still reachable after a migration, import, or remap.
///
/// The gate forbids [`PriorArtifactAvailability::PriorArtifactDiscarded`]: a migration or import must
/// keep the old artifact reachable — it never silently discards it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriorArtifactAvailability {
    /// The pre-migration artifact is retained and directly reachable.
    PriorArtifactRetained,
    /// The pre-migration artifact is archived and reachable on request.
    PriorArtifactArchived,
    /// The pre-migration artifact was discarded. **Forbidden** — present only so the gate rejects it.
    PriorArtifactDiscarded,
}

impl PriorArtifactAvailability {
    /// Every availability state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PriorArtifactRetained,
        Self::PriorArtifactArchived,
        Self::PriorArtifactDiscarded,
    ];

    /// The states the gate permits; [`Self::PriorArtifactDiscarded`] is never one of them.
    pub const ALLOWED: [Self; 2] = [Self::PriorArtifactRetained, Self::PriorArtifactArchived];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorArtifactRetained => "prior_artifact_retained",
            Self::PriorArtifactArchived => "prior_artifact_archived",
            Self::PriorArtifactDiscarded => "prior_artifact_discarded",
        }
    }

    /// Whether this state preserves the prior artifact rather than discarding it.
    pub const fn preserves_prior(self) -> bool {
        !matches!(self, Self::PriorArtifactDiscarded)
    }
}

// --- Actions -------------------------------------------------------------------------------------

/// One of the three actions a migration/remap event can offer.
///
/// Open-details is read-only and always present so the migration/remap decision is never hidden.
/// Compare and recovery-next-step are preserved wherever the fidelity was narrowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationRemapActionKind {
    /// Open the full migration/remap details — result class, origin, triggers, and conditions.
    OpenDetails,
    /// Compare the migrated/remapped state against the preserved prior artifact before relying on it.
    Compare,
    /// Surface the concrete recovery next step that would restore more of the remembered state.
    RecoveryNextStep,
}

impl MigrationRemapActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::OpenDetails, Self::Compare, Self::RecoveryNextStep];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetails => "open_details",
            Self::Compare => "compare",
            Self::RecoveryNextStep => "recovery_next_step",
        }
    }
}

/// A keyboard-complete, screen-reader-safe affordance for one migration/remap action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationRemapAction {
    /// Which action this affordance offers.
    pub action: MigrationRemapActionKind,
    /// Opaque command id; the action is reachable from the command palette by this id.
    pub command_id: String,
    /// Keyboard shortcut token; the flow is operable without a pointer.
    pub keyboard_shortcut: String,
    /// Deterministic focus order within the event, so keyboard navigation is unambiguous.
    pub focus_order: u32,
    /// Screen-reader label naming the action and the event it describes.
    pub accessible_label: String,
    /// Attestation that the action stays scoped to this one migration/remap event. Must be true.
    pub scoped_to_event: bool,
}

impl MigrationRemapAction {
    /// Whether the affordance is keyboard-complete and screen-reader-safe.
    pub fn is_accessible(&self) -> bool {
        !self.command_id.trim().is_empty()
            && !self.keyboard_shortcut.trim().is_empty()
            && !self.accessible_label.trim().is_empty()
    }
}

// --- Consumer surface ----------------------------------------------------------------------------

/// A surface that must carry the same migration/remap record rather than inventing a weaker summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationRemapConsumerSurface {
    /// The exported workspace diagnostics bundle.
    DiagnosticsExport,
    /// The support-export packet.
    SupportPacket,
    /// The crash-recovery packet.
    CrashRecoveryPacket,
    /// The browser/mobile companion handoff packet.
    CompanionHandoff,
}

impl MigrationRemapConsumerSurface {
    /// Every consumer surface that must preserve this record, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::DiagnosticsExport,
        Self::SupportPacket,
        Self::CrashRecoveryPacket,
        Self::CompanionHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticsExport => "diagnostics_export",
            Self::SupportPacket => "support_packet",
            Self::CrashRecoveryPacket => "crash_recovery_packet",
            Self::CompanionHandoff => "companion_handoff",
        }
    }
}

// --- Migration/remap event -----------------------------------------------------------------------

/// One migration/remap event: a schema migration, imported-package provenance disclosure, or
/// display-topology remap, with its result detail and resulting restore fidelity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationRemapEvent {
    /// Stable event id.
    pub event_id: String,
    /// Stable scenario id naming the situation this event records.
    pub scenario_id: String,
    /// Kind of migration/remap event.
    pub event_kind: MigrationEventKind,
    /// Owner accountable for the event.
    pub owner: String,
    /// Remembered-state artifact class the event affected. Reused from the matrix vocabulary.
    pub affected_artifact_class: RememberedArtifactClass,
    /// Schema-migration detail; present iff [`Self::event_kind`] is
    /// [`MigrationEventKind::SchemaMigration`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_migration: Option<SchemaMigrationDetail>,
    /// Imported-package detail; present iff [`Self::event_kind`] is
    /// [`MigrationEventKind::ImportedPackageProvenance`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_package: Option<ImportedPackageDetail>,
    /// Display-topology-remap detail; present iff [`Self::event_kind`] is
    /// [`MigrationEventKind::DisplayTopologyRemap`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_topology_remap: Option<DisplayTopologyRemapDetail>,
    /// Whether the pre-migration artifact is still reachable; never discarded.
    pub prior_artifact_availability: PriorArtifactAvailability,
    /// Redaction class: what the event record excludes. Reused from the matrix vocabulary.
    #[serde(default)]
    pub redaction_class: Vec<RedactionExclusion>,
    /// Best restore fidelity the event claims, before the gate.
    pub declared_resulting_fidelity: RestoreFidelityClass,
    /// Observed schema condition.
    pub schema_condition: SchemaCondition,
    /// Observed dependency condition.
    pub dependency_condition: DependencyCondition,
    /// Observed topology condition.
    pub topology_condition: TopologyCondition,
    /// How fresh the restored evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// What the event does when a dependency is missing; never a silent delete.
    pub missing_dependency_behavior: MissingDependencyBehavior,
    /// Restore fidelity actually published after the gate; must equal
    /// [`MigrationRemapEvent::achieved_fidelity`].
    pub published_fidelity: RestoreFidelityClass,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Recovery next step surfaced when the fidelity is narrowed; must equal the recomputed path.
    pub recovery_path: RecoveryPath,
    /// Bounded, accessible actions offered on the event.
    #[serde(default)]
    pub available_actions: Vec<MigrationRemapAction>,
    /// Caveats attached to the published fidelity.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields that narrowed the restore below an exact resume.
    #[serde(default)]
    pub narrowed_fields: Vec<String>,
    /// Ref to the event's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the event answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl MigrationRemapEvent {
    /// The restore fidelity the gate permits this event to publish.
    ///
    /// The weakest ceiling implied by the declared resulting fidelity and the schema, dependency,
    /// topology, and evidence-freshness conditions, so a schema drift, a relocated dependency, a
    /// changed topology, or stale evidence can never publish an exact restore.
    pub fn achieved_fidelity(&self) -> RestoreFidelityClass {
        self.declared_resulting_fidelity
            .min(self.schema_condition.fidelity_ceiling())
            .min(self.dependency_condition.fidelity_ceiling())
            .min(self.topology_condition.fidelity_ceiling())
            .min(self.evidence_freshness.fidelity_ceiling())
    }

    /// The headline downgrade reasons recomputed from the event's observed conditions.
    pub fn computed_downgrade_reasons(&self) -> Vec<DowngradeReason> {
        let mut reasons = Vec::new();
        if self.schema_condition.is_drift() {
            reasons.push(DowngradeReason::SchemaDrift);
        }
        if self.dependency_condition.is_missing() {
            reasons.push(DowngradeReason::DependencyMissing);
        }
        if self.topology_condition.is_changed() {
            reasons.push(DowngradeReason::TopologyChanged);
        }
        if self.evidence_freshness.is_stale() {
            reasons.push(DowngradeReason::EvidenceStale);
        }
        reasons
    }

    /// The recovery next step the gate must record, derived from the event's observed conditions.
    ///
    /// Ordered by severity: a manual-review restore points at review, a missing dependency points at
    /// relocating it, a migratable schema points at a compatible restore, a changed topology points
    /// at reopening as context, stale evidence points at a refresh, and a clean event needs nothing.
    pub fn computed_recovery_path(&self) -> RecoveryPath {
        if self.achieved_fidelity() == RestoreFidelityClass::ManualReview {
            RecoveryPath::ManualReview
        } else if self.dependency_condition.is_missing() {
            RecoveryPath::RelocateDependency
        } else if self.schema_condition.is_drift() {
            RecoveryPath::RestoreCompatibly
        } else if self.topology_condition.is_changed() {
            RecoveryPath::ReopenAsContext
        } else if self.evidence_freshness.is_stale() {
            RecoveryPath::RefreshEvidence
        } else {
            RecoveryPath::NoneNeeded
        }
    }

    /// Whether the event achieves a clean exact restore (a no-migration baseline).
    pub fn is_exact(&self) -> bool {
        self.achieved_fidelity() == RestoreFidelityClass::ExactRestore
    }

    /// Whether the gate narrowed the achieved fidelity below the declared maximum.
    pub fn is_downgraded(&self) -> bool {
        self.achieved_fidelity().rank() < self.declared_resulting_fidelity.rank()
    }

    /// Whether the achieved fidelity is anything short of an exact restore.
    pub fn is_narrowed(&self) -> bool {
        self.achieved_fidelity() != RestoreFidelityClass::ExactRestore
    }

    /// Whether the event must preserve the compare and recovery-next-step actions: it does wherever
    /// the fidelity was narrowed.
    pub fn requires_recovery_actions(&self) -> bool {
        self.is_narrowed()
    }

    /// The affordance for an action, if the event offers it.
    pub fn action(&self, kind: MigrationRemapActionKind) -> Option<&MigrationRemapAction> {
        self.available_actions.iter().find(|a| a.action == kind)
    }

    /// Whether the event offers an action.
    pub fn has_action(&self, kind: MigrationRemapActionKind) -> bool {
        self.action(kind).is_some()
    }

    /// Whether the event guarantees every redaction exclusion a metadata record must carry.
    pub fn has_required_exclusions(&self) -> bool {
        let present: BTreeSet<RedactionExclusion> = self.redaction_class.iter().copied().collect();
        RedactionExclusion::ALL.iter().all(|e| present.contains(e))
    }

    /// Whether the detail block present matches the event kind: exactly the matching block is set.
    pub fn detail_matches_kind(&self) -> bool {
        let schema = self.schema_migration.is_some();
        let import = self.imported_package.is_some();
        let remap = self.display_topology_remap.is_some();
        match self.event_kind {
            MigrationEventKind::SchemaMigration => schema && !import && !remap,
            MigrationEventKind::ImportedPackageProvenance => import && !schema && !remap,
            MigrationEventKind::DisplayTopologyRemap => remap && !schema && !import,
        }
    }

    /// Whether the schema-migration detail is internally consistent and agrees with the observed
    /// schema condition.
    pub fn schema_detail_consistent(&self) -> bool {
        match &self.schema_migration {
            Some(detail) => {
                detail.is_consistent()
                    && self.schema_condition == detail.result_class.required_schema_condition()
            }
            None => true,
        }
    }

    /// Whether the imported-package detail agrees with the observed conditions and discloses a
    /// foreign origin: a mixed channel can never read as a clean schema match, a remapped or
    /// unresolved path must read as a missing dependency, and a foreign origin must exclude
    /// machine-local anchors and disclose before restore.
    pub fn imported_detail_consistent(&self) -> bool {
        match &self.imported_package {
            Some(detail) => {
                if !detail.provenance_is_complete() || !detail.foreign_disclosure_is_complete() {
                    return false;
                }
                if detail.channel_match.is_mixed()
                    && self.schema_condition == SchemaCondition::SchemaMatch
                {
                    return false;
                }
                self.dependency_condition == detail.path_handling.required_dependency_condition()
            }
            None => true,
        }
    }

    /// Whether the display-topology-remap detail is recordable and agrees with the observed topology
    /// condition: a remap names at least one trigger, materially altered placement, and reads as a
    /// changed topology — never a clean identical topology.
    pub fn remap_detail_consistent(&self) -> bool {
        match &self.display_topology_remap {
            Some(detail) => {
                detail.is_recordable()
                    && self.topology_condition.is_changed()
                    && self.topology_condition == detail.resolution.required_topology_condition()
            }
            None => true,
        }
    }

    /// Whether the stored published fidelity, reasons, and path all agree with the recomputed gate.
    pub fn gate_consistent(&self) -> bool {
        self.published_fidelity == self.achieved_fidelity()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.recovery_path == self.computed_recovery_path()
    }

    /// A plain-language summary line for the event.
    fn summary_line(&self) -> String {
        format!(
            "{}: {} affecting {}, published {}, recovery {}",
            self.event_kind.as_str(),
            self.scenario_id,
            self.affected_artifact_class.as_str(),
            self.published_fidelity.as_str(),
            self.recovery_path.as_str()
        )
    }
}

// --- Consumer binding ----------------------------------------------------------------------------

/// A binding wiring a parity surface to this migration/remap packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationRemapConsumerBinding {
    /// Surface this binding wires.
    pub consumer_surface: MigrationRemapConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Migration/remap packet id this surface ingests.
    pub record_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface carries this record rather than a parallel summary.
    pub ingests_record: bool,
    /// True when the surface preserves the restore-fidelity labels verbatim.
    pub preserves_fidelity_labels: bool,
    /// True when the surface preserves the migration/remap-kind and origin labels verbatim, so
    /// support can tell a platform remap apart from a generic restore failure.
    pub preserves_remap_labels: bool,
    /// True when the surface narrows automatically as events are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl MigrationRemapConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.record_packet_id_ref == packet_id
            && self.ingests_record
            && self.preserves_fidelity_labels
            && self.preserves_remap_labels
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

// --- Summary and views ---------------------------------------------------------------------------

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5StateMigrationRemapSummary {
    /// Total events.
    pub events: usize,
    /// Events publishing an exact restore.
    pub exact_events: usize,
    /// Events publishing a compatible restore.
    pub compatible_events: usize,
    /// Events publishing a layout-only restore.
    pub layout_only_events: usize,
    /// Events held for manual review.
    pub manual_review_events: usize,
    /// Events the gate narrowed below their declared resulting fidelity.
    pub downgraded_events: usize,
    /// Events whose published fidelity is short of an exact restore.
    pub narrowed_events: usize,
    /// Schema-migration events.
    pub schema_migration_events: usize,
    /// Imported-package-provenance events.
    pub imported_package_events: usize,
    /// Display-topology-remap events.
    pub display_topology_remap_events: usize,
    /// Imported-package events whose origin is foreign (another machine or org).
    pub foreign_origin_events: usize,
}

/// A plain-language event-view row downstream surfaces render instead of restating each event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRemapViewRow {
    /// Event id.
    pub event_id: String,
    /// Scenario id.
    pub scenario_id: String,
    /// Event-kind token.
    pub event_kind: String,
    /// Affected artifact-class token.
    pub affected_artifact_class: String,
    /// Published restore-fidelity token.
    pub published_fidelity: String,
    /// Whether the event was downgraded below its declared resulting fidelity.
    pub downgraded: bool,
    /// Whether the published fidelity is short of an exact restore.
    pub narrowed: bool,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Recovery-next-step token.
    pub recovery_path: String,
    /// Prior-artifact-availability token.
    pub prior_artifact_availability: String,
    /// Action tokens offered on the event.
    pub actions: Vec<String>,
    /// Human-readable summary line.
    pub summary: String,
}

/// The plain-language event view downstream surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRemapView {
    /// Packet id this view was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<MigrationRemapViewRow>,
    /// Events publishing an exact restore.
    pub exact_count: usize,
    /// Events short of an exact restore.
    pub narrowed_count: usize,
    /// Events held for manual review.
    pub manual_review_count: usize,
}

// --- Packet --------------------------------------------------------------------------------------

/// The typed M5 state-migration and topology-remap packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5StateMigrationAndTopologyRemap {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed restore-fidelity-class vocabulary.
    pub restore_fidelity_classes: Vec<RestoreFidelityClass>,
    /// Closed event-kind vocabulary.
    pub event_kinds: Vec<MigrationEventKind>,
    /// Closed migration-result-class vocabulary.
    pub migration_result_classes: Vec<MigrationResultClass>,
    /// Closed package-origin-class vocabulary.
    pub package_origin_classes: Vec<PackageOriginClass>,
    /// Closed channel-match vocabulary.
    pub channel_matches: Vec<ChannelMatch>,
    /// Closed path-handling-posture vocabulary.
    pub path_handling_postures: Vec<PathHandlingPosture>,
    /// Closed remap-trigger vocabulary.
    pub remap_triggers: Vec<RemapTrigger>,
    /// Closed remap-resolution vocabulary.
    pub remap_resolutions: Vec<RemapResolution>,
    /// Closed prior-artifact-availability vocabulary.
    pub prior_artifact_availabilities: Vec<PriorArtifactAvailability>,
    /// Closed remembered-state artifact-class vocabulary.
    pub artifact_classes: Vec<RememberedArtifactClass>,
    /// Closed redaction-exclusion vocabulary.
    pub redaction_exclusions: Vec<RedactionExclusion>,
    /// Closed schema-condition vocabulary.
    pub schema_conditions: Vec<SchemaCondition>,
    /// Closed dependency-condition vocabulary.
    pub dependency_conditions: Vec<DependencyCondition>,
    /// Closed topology-condition vocabulary.
    pub topology_conditions: Vec<TopologyCondition>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed missing-dependency-behavior vocabulary.
    pub missing_dependency_behaviors: Vec<MissingDependencyBehavior>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed recovery-path vocabulary.
    pub recovery_paths: Vec<RecoveryPath>,
    /// Closed action-kind vocabulary.
    pub action_kinds: Vec<MigrationRemapActionKind>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<MigrationRemapConsumerSurface>,
    /// Migration/remap events.
    #[serde(default)]
    pub events: Vec<MigrationRemapEvent>,
    /// Consumer bindings, one per required parity surface.
    #[serde(default)]
    pub consumer_bindings: Vec<MigrationRemapConsumerBinding>,
    /// Summary counts.
    pub summary: M5StateMigrationRemapSummary,
}

impl M5StateMigrationAndTopologyRemap {
    /// Returns the event with the given id.
    pub fn event(&self, event_id: &str) -> Option<&MigrationRemapEvent> {
        self.events.iter().find(|e| e.event_id == event_id)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: MigrationRemapConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every event agrees with the recomputed gate.
    pub fn all_events_gate_consistent(&self) -> bool {
        self.events.iter().all(|e| e.gate_consistent())
    }

    /// Recomputes the summary block from the events.
    pub fn computed_summary(&self) -> M5StateMigrationRemapSummary {
        let count_fidelity = |class: RestoreFidelityClass| {
            self.events
                .iter()
                .filter(|e| e.published_fidelity == class)
                .count()
        };
        let count_kind =
            |kind: MigrationEventKind| self.events.iter().filter(|e| e.event_kind == kind).count();
        M5StateMigrationRemapSummary {
            events: self.events.len(),
            exact_events: count_fidelity(RestoreFidelityClass::ExactRestore),
            compatible_events: count_fidelity(RestoreFidelityClass::CompatibleRestore),
            layout_only_events: count_fidelity(RestoreFidelityClass::LayoutOnly),
            manual_review_events: count_fidelity(RestoreFidelityClass::ManualReview),
            downgraded_events: self.events.iter().filter(|e| e.is_downgraded()).count(),
            narrowed_events: self.events.iter().filter(|e| e.is_narrowed()).count(),
            schema_migration_events: count_kind(MigrationEventKind::SchemaMigration),
            imported_package_events: count_kind(MigrationEventKind::ImportedPackageProvenance),
            display_topology_remap_events: count_kind(MigrationEventKind::DisplayTopologyRemap),
            foreign_origin_events: self
                .events
                .iter()
                .filter(|e| {
                    e.imported_package
                        .as_ref()
                        .is_some_and(|d| d.origin.is_foreign())
                })
                .count(),
        }
    }

    /// Produces the plain-language event view downstream surfaces render.
    pub fn event_view(&self) -> MigrationRemapView {
        let rows = self
            .events
            .iter()
            .map(|e| MigrationRemapViewRow {
                event_id: e.event_id.clone(),
                scenario_id: e.scenario_id.clone(),
                event_kind: e.event_kind.as_str().to_owned(),
                affected_artifact_class: e.affected_artifact_class.as_str().to_owned(),
                published_fidelity: e.published_fidelity.as_str().to_owned(),
                downgraded: e.is_downgraded(),
                narrowed: e.is_narrowed(),
                downgrade_reasons: e
                    .downgrade_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                recovery_path: e.recovery_path.as_str().to_owned(),
                prior_artifact_availability: e.prior_artifact_availability.as_str().to_owned(),
                actions: e
                    .available_actions
                    .iter()
                    .map(|a| a.action.as_str().to_owned())
                    .collect(),
                summary: e.summary_line(),
            })
            .collect();
        MigrationRemapView {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            exact_count: self.events.iter().filter(|e| e.is_exact()).count(),
            narrowed_count: self.events.iter().filter(|e| e.is_narrowed()).count(),
            manual_review_count: self
                .events
                .iter()
                .filter(|e| e.published_fidelity == RestoreFidelityClass::ManualReview)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact migration/remap record.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5StateMigrationRemapSupportExport {
        M5StateMigrationRemapSupportExport {
            record_kind: M5_STATE_MIGRATION_REMAP_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_STATE_MIGRATION_REMAP_SCHEMA_VERSION,
            export_id: export_id.into(),
            record_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            record: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5StateMigrationRemapViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_scenarios = BTreeSet::new();
        let mut seen_kinds = BTreeSet::new();
        for event in &self.events {
            if !seen_ids.insert(event.event_id.clone()) {
                violations.push(M5StateMigrationRemapViolation::DuplicateEventId {
                    event_id: event.event_id.clone(),
                });
            }
            if !seen_scenarios.insert(event.scenario_id.clone()) {
                violations.push(M5StateMigrationRemapViolation::DuplicateScenarioId {
                    scenario_id: event.scenario_id.clone(),
                });
            }
            seen_kinds.insert(event.event_kind);
            self.validate_event(event, &mut violations);
        }
        for &kind in &MigrationEventKind::ALL {
            if !seen_kinds.contains(&kind) {
                violations.push(M5StateMigrationRemapViolation::MissingEventKindCoverage {
                    kind: kind.as_str(),
                });
            }
        }

        for surface in MigrationRemapConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5StateMigrationRemapViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5StateMigrationRemapViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5StateMigrationRemapViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5StateMigrationRemapViolation>) {
        if self.schema_version != M5_STATE_MIGRATION_REMAP_SCHEMA_VERSION {
            violations.push(M5StateMigrationRemapViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_STATE_MIGRATION_REMAP_RECORD_KIND {
            violations.push(M5StateMigrationRemapViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5StateMigrationRemapViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "restore_fidelity_classes",
                self.restore_fidelity_classes == RestoreFidelityClass::ALL.to_vec(),
            ),
            (
                "event_kinds",
                self.event_kinds == MigrationEventKind::ALL.to_vec(),
            ),
            (
                "migration_result_classes",
                self.migration_result_classes == MigrationResultClass::ALL.to_vec(),
            ),
            (
                "package_origin_classes",
                self.package_origin_classes == PackageOriginClass::ALL.to_vec(),
            ),
            (
                "channel_matches",
                self.channel_matches == ChannelMatch::ALL.to_vec(),
            ),
            (
                "path_handling_postures",
                self.path_handling_postures == PathHandlingPosture::ALL.to_vec(),
            ),
            (
                "remap_triggers",
                self.remap_triggers == RemapTrigger::ALL.to_vec(),
            ),
            (
                "remap_resolutions",
                self.remap_resolutions == RemapResolution::ALL.to_vec(),
            ),
            (
                "prior_artifact_availabilities",
                self.prior_artifact_availabilities == PriorArtifactAvailability::ALL.to_vec(),
            ),
            (
                "artifact_classes",
                self.artifact_classes == RememberedArtifactClass::ALL.to_vec(),
            ),
            (
                "redaction_exclusions",
                self.redaction_exclusions == RedactionExclusion::ALL.to_vec(),
            ),
            (
                "schema_conditions",
                self.schema_conditions == SchemaCondition::ALL.to_vec(),
            ),
            (
                "dependency_conditions",
                self.dependency_conditions == DependencyCondition::ALL.to_vec(),
            ),
            (
                "topology_conditions",
                self.topology_conditions == TopologyCondition::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "missing_dependency_behaviors",
                self.missing_dependency_behaviors == MissingDependencyBehavior::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == DowngradeReason::ALL.to_vec(),
            ),
            (
                "recovery_paths",
                self.recovery_paths == RecoveryPath::ALL.to_vec(),
            ),
            (
                "action_kinds",
                self.action_kinds == MigrationRemapActionKind::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == MigrationRemapConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5StateMigrationRemapViolation::ClosedVocabularyDrift {
                    field_name: field,
                });
            }
        }
    }

    fn validate_event(
        &self,
        event: &MigrationRemapEvent,
        violations: &mut Vec<M5StateMigrationRemapViolation>,
    ) {
        for (field, value) in [
            ("event_id", &event.event_id),
            ("scenario_id", &event.scenario_id),
            ("owner", &event.owner),
            ("evidence_ref", &event.evidence_ref),
            ("scope_snapshot_ref", &event.scope_snapshot_ref),
            ("note", &event.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5StateMigrationRemapViolation::EmptyField {
                    id: event.event_id.clone(),
                    field_name: field,
                });
            }
        }

        // Exactly the detail block matching the event kind must be present.
        if !event.detail_matches_kind() {
            violations.push(M5StateMigrationRemapViolation::DetailKindMismatch {
                event_id: event.event_id.clone(),
            });
        }
        // The migration result, version jump, and step count must agree with the schema condition.
        if !event.schema_detail_consistent() {
            violations.push(
                M5StateMigrationRemapViolation::SchemaMigrationInconsistent {
                    event_id: event.event_id.clone(),
                },
            );
        }
        // A mixed channel can never read as a clean schema match; a remapped or unresolved path must
        // read as a missing dependency; a foreign origin must exclude machine-local anchors and
        // disclose before restore.
        if !event.imported_detail_consistent() {
            violations.push(
                M5StateMigrationRemapViolation::ImportedPackageInconsistent {
                    event_id: event.event_id.clone(),
                },
            );
        }
        // A remap names a trigger, materially altered placement, and reads as a changed topology.
        if !event.remap_detail_consistent() {
            violations.push(M5StateMigrationRemapViolation::TopologyRemapInconsistent {
                event_id: event.event_id.clone(),
            });
        }

        // Every record excludes secrets, live authority, machine-local anchors, and raw provider
        // payloads; it is metadata only.
        if !event.has_required_exclusions() {
            violations.push(M5StateMigrationRemapViolation::MissingRedactionExclusion {
                event_id: event.event_id.clone(),
            });
        }

        // A missing dependency never silently deletes layout.
        if !event.missing_dependency_behavior.preserves_slot() {
            violations.push(M5StateMigrationRemapViolation::SilentLayoutDelete {
                event_id: event.event_id.clone(),
            });
        }

        // A migration, import, or remap never silently discards the prior artifact.
        if !event.prior_artifact_availability.preserves_prior() {
            violations.push(M5StateMigrationRemapViolation::DiscardedPriorArtifact {
                event_id: event.event_id.clone(),
            });
        }

        let achieved = event.achieved_fidelity();
        // The published fidelity must equal the gate's recomputed ceiling.
        if event.published_fidelity != achieved {
            violations.push(M5StateMigrationRemapViolation::OverstatedFidelity {
                event_id: event.event_id.clone(),
                published: event.published_fidelity.as_str(),
                computed: achieved.as_str(),
            });
        }
        // The published fidelity may never exceed the declared resulting fidelity.
        if event.published_fidelity.rank() > event.declared_resulting_fidelity.rank() {
            violations.push(M5StateMigrationRemapViolation::ExceedsDeclaredFidelity {
                event_id: event.event_id.clone(),
                published: event.published_fidelity.as_str(),
                declared: event.declared_resulting_fidelity.as_str(),
            });
        }

        let computed = event.computed_downgrade_reasons();
        if event.downgrade_reasons != computed {
            violations.push(M5StateMigrationRemapViolation::DowngradeReasonsMismatch {
                event_id: event.event_id.clone(),
            });
        }
        let computed_path = event.computed_recovery_path();
        if event.recovery_path != computed_path {
            violations.push(M5StateMigrationRemapViolation::RecoveryPathMismatch {
                event_id: event.event_id.clone(),
                declared: event.recovery_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // The migration/remap decision is never hidden: every event offers the open-details action.
        if !event.has_action(MigrationRemapActionKind::OpenDetails) {
            violations.push(M5StateMigrationRemapViolation::MissingOpenDetailsAction {
                event_id: event.event_id.clone(),
            });
        }
        // Compare and recovery-next-step are preserved wherever the fidelity was narrowed.
        if event.requires_recovery_actions()
            && (!event.has_action(MigrationRemapActionKind::Compare)
                || !event.has_action(MigrationRemapActionKind::RecoveryNextStep))
        {
            violations.push(M5StateMigrationRemapViolation::MissingRecoveryActions {
                event_id: event.event_id.clone(),
            });
        }

        let mut seen_actions = BTreeSet::new();
        let mut seen_focus = BTreeSet::new();
        for affordance in &event.available_actions {
            if !seen_actions.insert(affordance.action) {
                violations.push(M5StateMigrationRemapViolation::DuplicateAction {
                    event_id: event.event_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !seen_focus.insert(affordance.focus_order) {
                violations.push(M5StateMigrationRemapViolation::DuplicateFocusOrder {
                    event_id: event.event_id.clone(),
                });
            }
            if !affordance.is_accessible() {
                violations.push(M5StateMigrationRemapViolation::InaccessibleAction {
                    event_id: event.event_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !affordance.scoped_to_event {
                violations.push(M5StateMigrationRemapViolation::UnscopedAction {
                    event_id: event.event_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
        }

        // A narrowed event must offer a real recovery next step, name a caveat, and name what was
        // narrowed.
        if achieved != RestoreFidelityClass::ExactRestore {
            if !event.recovery_path.is_offered() {
                violations.push(M5StateMigrationRemapViolation::MissingRecoveryPath {
                    event_id: event.event_id.clone(),
                });
            }
            if event.caveats.is_empty() {
                violations.push(M5StateMigrationRemapViolation::EmptyField {
                    id: event.event_id.clone(),
                    field_name: "caveats",
                });
            }
            if event.narrowed_fields.is_empty() {
                violations.push(M5StateMigrationRemapViolation::EmptyField {
                    id: event.event_id.clone(),
                    field_name: "narrowed_fields",
                });
            }
        }

        // An exact event must be a genuinely clean no-migration baseline: a schema-migration with no
        // schema change, pristine conditions, no downgrade reason, and no recovery path.
        if achieved == RestoreFidelityClass::ExactRestore
            && (event.event_kind != MigrationEventKind::SchemaMigration
                || event.schema_migration.as_ref().map(|d| d.result_class)
                    != Some(MigrationResultClass::SchemaUnchanged)
                || event.schema_condition != SchemaCondition::SchemaMatch
                || event.dependency_condition != DependencyCondition::DependenciesPresent
                || event.topology_condition != TopologyCondition::TopologyIdentical
                || event.evidence_freshness != EvidenceFreshness::Current
                || !event.downgrade_reasons.is_empty()
                || !event.caveats.is_empty()
                || !event.narrowed_fields.is_empty()
                || event.recovery_path.is_offered())
        {
            violations.push(M5StateMigrationRemapViolation::ExactEventNotClean {
                event_id: event.event_id.clone(),
            });
        }
    }
}

/// A validation violation for [`M5StateMigrationAndTopologyRemap`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum M5StateMigrationRemapViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary disagrees with this build's canonical list.
    ClosedVocabularyDrift {
        /// Offending field.
        field_name: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Event or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An event id appears more than once.
    DuplicateEventId {
        /// Duplicate event id.
        event_id: String,
    },
    /// A scenario id appears more than once.
    DuplicateScenarioId {
        /// Duplicate scenario id.
        scenario_id: String,
    },
    /// An event kind has no event.
    MissingEventKindCoverage {
        /// Event-kind token.
        kind: &'static str,
    },
    /// An event's detail block does not match its event kind.
    DetailKindMismatch {
        /// Event id.
        event_id: String,
    },
    /// A schema-migration detail is internally inconsistent or disagrees with the schema condition.
    SchemaMigrationInconsistent {
        /// Event id.
        event_id: String,
    },
    /// An imported-package detail disagrees with the observed conditions or omits foreign disclosure.
    ImportedPackageInconsistent {
        /// Event id.
        event_id: String,
    },
    /// A display-topology-remap detail is not recordable or disagrees with the topology condition.
    TopologyRemapInconsistent {
        /// Event id.
        event_id: String,
    },
    /// An event publishes a fidelity beyond what the gate computes.
    OverstatedFidelity {
        /// Event id.
        event_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Computed fidelity token.
        computed: &'static str,
    },
    /// An event publishes a fidelity above its declared resulting fidelity.
    ExceedsDeclaredFidelity {
        /// Event id.
        event_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Declared resulting fidelity token.
        declared: &'static str,
    },
    /// An event's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Event id.
        event_id: String,
    },
    /// An event's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Event id.
        event_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// An event would silently delete layout when a dependency is missing.
    SilentLayoutDelete {
        /// Event id.
        event_id: String,
    },
    /// An event would silently discard the prior artifact.
    DiscardedPriorArtifact {
        /// Event id.
        event_id: String,
    },
    /// An event does not guarantee every required redaction exclusion.
    MissingRedactionExclusion {
        /// Event id.
        event_id: String,
    },
    /// An event omits the read-only open-details action, hiding its decision.
    MissingOpenDetailsAction {
        /// Event id.
        event_id: String,
    },
    /// A narrowed event omits the compare or recovery-next-step action.
    MissingRecoveryActions {
        /// Event id.
        event_id: String,
    },
    /// A narrowed event offers no recovery path.
    MissingRecoveryPath {
        /// Event id.
        event_id: String,
    },
    /// An event offers the same action twice.
    DuplicateAction {
        /// Event id.
        event_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// Two affordances in an event share a focus order, making keyboard navigation ambiguous.
    DuplicateFocusOrder {
        /// Event id.
        event_id: String,
    },
    /// An affordance lacks a command id, keyboard shortcut, or screen-reader label.
    InaccessibleAction {
        /// Event id.
        event_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// An affordance is not scoped to the one migration/remap event.
    UnscopedAction {
        /// Event id.
        event_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// An exact event is not a genuinely clean no-migration baseline.
    ExactEventNotClean {
        /// Event id.
        event_id: String,
    },
    /// A required parity surface has no preserving consumer binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints the record's labels.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the events.
    SummaryMismatch,
}

impl fmt::Display for M5StateMigrationRemapViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyDrift { field_name } => {
                write!(f, "closed vocabulary {field_name} disagrees with this build")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateEventId { event_id } => write!(f, "duplicate event id {event_id}"),
            Self::DuplicateScenarioId { scenario_id } => {
                write!(f, "duplicate scenario id {scenario_id}")
            }
            Self::MissingEventKindCoverage { kind } => {
                write!(f, "no event covers event kind {kind}")
            }
            Self::DetailKindMismatch { event_id } => {
                write!(f, "event {event_id} detail block does not match its event kind")
            }
            Self::SchemaMigrationInconsistent { event_id } => write!(
                f,
                "event {event_id} schema-migration detail disagrees with the schema condition"
            ),
            Self::ImportedPackageInconsistent { event_id } => write!(
                f,
                "event {event_id} imported-package detail disagrees with the observed conditions or omits foreign disclosure"
            ),
            Self::TopologyRemapInconsistent { event_id } => write!(
                f,
                "event {event_id} display-topology-remap detail is not recordable or disagrees with the topology condition"
            ),
            Self::OverstatedFidelity {
                event_id,
                published,
                computed,
            } => write!(
                f,
                "event {event_id} publishes fidelity {published} but the gate computes {computed}"
            ),
            Self::ExceedsDeclaredFidelity {
                event_id,
                published,
                declared,
            } => write!(
                f,
                "event {event_id} publishes fidelity {published} above declared {declared}"
            ),
            Self::DowngradeReasonsMismatch { event_id } => {
                write!(f, "event {event_id} downgrade reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch {
                event_id,
                declared,
                required,
            } => write!(
                f,
                "event {event_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::SilentLayoutDelete { event_id } => write!(
                f,
                "event {event_id} would silently delete layout on a missing dependency"
            ),
            Self::DiscardedPriorArtifact { event_id } => write!(
                f,
                "event {event_id} would silently discard the prior artifact"
            ),
            Self::MissingRedactionExclusion { event_id } => write!(
                f,
                "event {event_id} does not guarantee its required redaction exclusions"
            ),
            Self::MissingOpenDetailsAction { event_id } => {
                write!(f, "event {event_id} omits the open-details action")
            }
            Self::MissingRecoveryActions { event_id } => write!(
                f,
                "event {event_id} is narrowed but omits the compare or recovery-next-step action"
            ),
            Self::MissingRecoveryPath { event_id } => {
                write!(f, "event {event_id} is narrowed but offers no recovery path")
            }
            Self::DuplicateAction { event_id, action } => {
                write!(f, "event {event_id} offers action {action} twice")
            }
            Self::DuplicateFocusOrder { event_id } => {
                write!(f, "event {event_id} has affordances sharing a focus order")
            }
            Self::InaccessibleAction { event_id, action } => write!(
                f,
                "event {event_id} action {action} lacks a command id, shortcut, or label"
            ),
            Self::UnscopedAction { event_id, action } => {
                write!(f, "event {event_id} action {action} is not scoped to the event")
            }
            Self::ExactEventNotClean { event_id } => write!(
                f,
                "event {event_id} restores exact but is not a clean no-migration baseline"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "parity surface {surface} has no consumer binding")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "consumer binding {binding_ref} does not preserve the record")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the events"),
        }
    }
}

impl Error for M5StateMigrationRemapViolation {}

/// Stable record-kind tag for [`M5StateMigrationRemapSupportExport`].
pub const M5_STATE_MIGRATION_REMAP_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_state_migration_and_topology_remap_support_export";

/// Support-export wrapper preserving the migration/remap record verbatim for support and evidence
/// packets, so an exported diagnostics or support bundle carries the same migration/remap and
/// fidelity record rather than a weaker summary — and support can tell a platform remap apart from a
/// generic restore failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateMigrationRemapSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub record_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact record preserved by the export.
    pub record: M5StateMigrationAndTopologyRemap,
}

impl M5StateMigrationRemapSupportExport {
    /// Whether the export preserves the same packet id and a clean record.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_STATE_MIGRATION_REMAP_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_STATE_MIGRATION_REMAP_SCHEMA_VERSION
            && self.record_packet_id_ref == self.record.packet_id
            && self.raw_private_material_excluded
            && self.record.validate().is_empty()
    }
}

/// Loads the embedded M5 state-migration and topology-remap packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5StateMigrationAndTopologyRemap`].
pub fn current_m5_state_migration_and_topology_remap(
) -> Result<M5StateMigrationAndTopologyRemap, serde_json::Error> {
    serde_json::from_str(M5_STATE_MIGRATION_REMAP_JSON)
}

#[cfg(test)]
mod tests;
