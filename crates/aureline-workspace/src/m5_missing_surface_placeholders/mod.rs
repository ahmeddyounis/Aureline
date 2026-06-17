//! M5 missing-surface placeholders: the one place a restored window discloses that a pane could not
//! hydrate while keeping the pane's role, slot, last-known provenance, and recovery actions visible
//! instead of silently deleting or reshaping layout.
//!
//! M5 keeps adding multi-pane, restore-heavy surfaces — preview routes, notebook sessions, query
//! consoles, profiler captures, docs panes, and incident workspaces. When the extension, feature
//! pack, remote target, or backing service a pane depends on is missing on restore, the layout must
//! still read as the workspace the user left. This packet is the **slot-facing** projection: one
//! [`MissingSurfacePlaceholderCard`] per pane that could not hydrate, recording the original
//! [`PaneRole`], the stable pane-tree slot, the [`MissingDependencyClass`], the last-known
//! provenance, and the recovery actions that would restore it.
//!
//! It never invents its own continuity language. The fidelity, dependency, schema, topology,
//! freshness, redaction, downgrade, recovery, and missing-dependency-behavior vocabularies are the
//! canonical ones from [`crate::m5_serialization_and_restore_matrix`], and the re-entry-surface and
//! producer-provenance vocabularies are reused from [`crate::m5_restore_provenance`], so a placeholder
//! means the same thing across desktop restore, import, crash recovery, support replay, and
//! companion/browser re-entry.
//!
//! - [`MissingDependencyClass`] names what is missing — a missing extension, an uninstalled feature
//!   pack, an unreachable remote target, or a down backing service — and the recovery action that
//!   resolves it ([`MissingDependencyClass::primary_recovery_action`]).
//! - [`PaneRole`] is the user-facing role the missing pane occupied. A placeholder **keeps** that
//!   role rather than collapsing to a generic empty tab, so the slot still reads as the surface it
//!   stands in for.
//! - [`PlaceholderActionKind`] models the always-present open-details affordance plus the concrete
//!   recovery affordances (install the dependency, reconnect the remote, retry the service, reopen as
//!   context, export the retained evidence).
//! - [`PlaceholderConsumerBinding`] wires the surfaces that must carry the **same** record —
//!   exported diagnostics, support packets, compare/export summaries, and companion handoff — to this
//!   one packet, so they can name the missing-surface classes and affected pane roles rather than
//!   inventing weaker summaries.
//!
//! The packet is fail-closed. A placeholder describes a genuinely missing dependency
//! ([`DependencyCondition::is_missing`]), so its achieved fidelity is the **weakest ceiling** implied
//! by its declared fidelity and its dependency, schema, topology, and freshness conditions
//! ([`MissingSurfacePlaceholderCard::achieved_fidelity`]); a missing dependency caps the restore at a
//! slot-preserving [`RestoreFidelityClass::LayoutOnly`] (or [`RestoreFidelityClass::ManualReview`]
//! when the dependency root is gone), so a placeholder can never publish an exact restore. The
//! substitution behavior must preserve the slot ([`MissingDependencyBehavior::preserves_slot`]);
//! [`MissingDependencyBehavior::SilentDelete`] is reject-only, so a missing surface never silently
//! deletes a pane, loses a tab, or substitutes a misleading empty state. Every card keeps the
//! read-only open-details action so provenance is never hidden, offers the recovery action its
//! missing-dependency class calls for, and carries screen-reader narration that announces the role,
//! slot, missing reason, and recovery so keyboard focus and narration stay sensible.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-missing-surface-placeholders.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, an opaque ref, or a
//! plain-language label, and it carries no credential bodies, raw provider payloads, live authority
//! handles, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_restore_provenance::{ReentrySurface, RestoreProducer};
use crate::m5_serialization_and_restore_matrix::{
    DependencyCondition, DowngradeReason, EvidenceFreshness, MissingDependencyBehavior,
    RecoveryPath, RedactionExclusion, RememberedArtifactClass, RestoreFidelityClass,
    SchemaCondition, TopologyCondition,
};

/// Supported M5 missing-surface-placeholders packet schema version.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_RECORD_KIND: &str = "m5_missing_surface_placeholders";

/// Repo-relative path to the checked-in packet.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_PATH: &str =
    "artifacts/workspace/m5/m5-missing-surface-placeholders.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_SCHEMA_REF: &str =
    "schemas/workspace/m5-missing-surface-placeholders.schema.json";

/// Repo-relative path to the companion document.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_DOC_REF: &str =
    "docs/workspace/m5/m5-missing-surface-placeholders.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-missing-surface-placeholders.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-missing-surface-placeholders";

/// Embedded checked-in packet JSON.
pub const M5_MISSING_SURFACE_PLACEHOLDERS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-missing-surface-placeholders.json"
));

// --- Missing-dependency and pane-role vocabulary -------------------------------------------------

/// The class of dependency that is missing when a restored pane cannot hydrate.
///
/// Each class binds to the recovery action that resolves it
/// ([`MissingDependencyClass::primary_recovery_action`]), so a placeholder can always name a concrete
/// next step rather than a generic "unavailable".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingDependencyClass {
    /// A required extension is missing or disabled.
    Extension,
    /// A required feature pack is not installed.
    FeaturePack,
    /// A required remote target or host is unreachable.
    RemoteTarget,
    /// A required backing service is unavailable.
    BackingService,
}

impl MissingDependencyClass {
    /// Every missing-dependency class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Extension,
        Self::FeaturePack,
        Self::RemoteTarget,
        Self::BackingService,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::FeaturePack => "feature_pack",
            Self::RemoteTarget => "remote_target",
            Self::BackingService => "backing_service",
        }
    }

    /// The recovery action that resolves a missing dependency of this class.
    ///
    /// An extension or feature pack is installed, an unreachable remote target is reconnected, and a
    /// down backing service is retried.
    pub const fn primary_recovery_action(self) -> PlaceholderActionKind {
        match self {
            Self::Extension | Self::FeaturePack => PlaceholderActionKind::InstallDependency,
            Self::RemoteTarget => PlaceholderActionKind::ReconnectRemote,
            Self::BackingService => PlaceholderActionKind::RetryService,
        }
    }
}

/// The user-facing role a pane occupies. A placeholder keeps its original role rather than collapsing
/// to a generic empty tab, so the restored layout still reads as the surface it stands in for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneRole {
    /// A text editor.
    Editor,
    /// A terminal.
    Terminal,
    /// An explorer pane.
    Explorer,
    /// An AI panel.
    AiPanel,
    /// A preview route or rendered-output pane.
    Preview,
    /// A notebook session.
    Notebook,
    /// A database query console.
    QueryConsole,
    /// A profiler trace capture.
    Profiler,
    /// A documentation pane.
    Docs,
    /// An incident workspace.
    IncidentWorkspace,
}

impl PaneRole {
    /// Every pane role, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Editor,
        Self::Terminal,
        Self::Explorer,
        Self::AiPanel,
        Self::Preview,
        Self::Notebook,
        Self::QueryConsole,
        Self::Profiler,
        Self::Docs,
        Self::IncidentWorkspace,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Terminal => "terminal",
            Self::Explorer => "explorer",
            Self::AiPanel => "ai_panel",
            Self::Preview => "preview",
            Self::Notebook => "notebook",
            Self::QueryConsole => "query_console",
            Self::Profiler => "profiler",
            Self::Docs => "docs",
            Self::IncidentWorkspace => "incident_workspace",
        }
    }
}

/// One of the actions a missing-surface placeholder can offer.
///
/// Open-details is read-only and always present so provenance is never hidden. The remaining actions
/// are the concrete recovery next steps a placeholder offers for its missing-dependency class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderActionKind {
    /// Open the full placeholder details — pane role, slot, missing dependency, and provenance.
    OpenDetails,
    /// Install the missing extension or feature pack.
    InstallDependency,
    /// Reconnect the missing remote target.
    ReconnectRemote,
    /// Retry the missing backing service.
    RetryService,
    /// Reopen the pane's last-known context without the missing dependency, slot preserved.
    ReopenAsContext,
    /// Export the retained evidence for the missing surface.
    ExportEvidence,
}

impl PlaceholderActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDetails,
        Self::InstallDependency,
        Self::ReconnectRemote,
        Self::RetryService,
        Self::ReopenAsContext,
        Self::ExportEvidence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetails => "open_details",
            Self::InstallDependency => "install_dependency",
            Self::ReconnectRemote => "reconnect_remote",
            Self::RetryService => "retry_service",
            Self::ReopenAsContext => "reopen_as_context",
            Self::ExportEvidence => "export_evidence",
        }
    }
}

/// A surface that must carry the same missing-surface record rather than inventing a weaker summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderConsumerSurface {
    /// The exported workspace diagnostics bundle.
    DiagnosticsExport,
    /// The support-export packet.
    SupportPacket,
    /// The compare/export summary projecting a restore diff.
    CompareExportSummary,
    /// The browser/mobile companion handoff packet.
    CompanionHandoff,
}

impl PlaceholderConsumerSurface {
    /// Every consumer surface that must preserve this record, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::DiagnosticsExport,
        Self::SupportPacket,
        Self::CompareExportSummary,
        Self::CompanionHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticsExport => "diagnostics_export",
            Self::SupportPacket => "support_packet",
            Self::CompareExportSummary => "compare_export_summary",
            Self::CompanionHandoff => "companion_handoff",
        }
    }
}

// --- Provenance, narration, and affordances ------------------------------------------------------

/// The last-known provenance of the surface a placeholder stands in for.
///
/// It records the producer/version/build that wrote the pane's remembered state, the schema version
/// it was written at, and an opaque ref to its last successful attach, so the slot keeps a real
/// history rather than reading as a never-populated empty tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceholderProvenance {
    /// Producer, version, and build that last wrote the pane's remembered state.
    pub producer: RestoreProducer,
    /// Schema version the remembered state was written at.
    pub remembered_schema_version: u32,
    /// Opaque ref to the pane's last successful attach.
    pub last_attached_ref: String,
}

impl PlaceholderProvenance {
    /// Whether the provenance is complete: producer present and a non-zero schema version recorded.
    pub fn is_complete(&self) -> bool {
        self.producer.is_complete()
            && self.remembered_schema_version != 0
            && !self.last_attached_ref.trim().is_empty()
    }
}

/// Screen-reader narration and focus state for a placeholder occupying a restored pane slot.
///
/// The narration must announce the role, slot, missing reason, and recovery so a screen-reader user
/// understands what stands in the slot, and the slot must remain reachable by keyboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceholderNarration {
    /// Screen-reader summary spoken when the placeholder gains focus.
    pub accessible_summary: String,
    /// Deterministic focus order for the placeholder within its window.
    pub focus_order: u32,
    /// Whether the narration announces the preserved pane role.
    pub role_announced: bool,
    /// Whether the narration announces the slot the placeholder occupies.
    pub slot_announced: bool,
    /// Whether the narration announces why the surface is missing.
    pub missing_reason_announced: bool,
    /// Whether the narration announces the recovery next step.
    pub recovery_announced: bool,
    /// Whether the placeholder slot is reachable by keyboard.
    pub keyboard_reachable: bool,
}

impl PlaceholderNarration {
    /// Whether the narration is sensible: a non-empty summary that announces role, slot, missing
    /// reason, and recovery, with the slot reachable by keyboard.
    pub fn is_sensible(&self) -> bool {
        !self.accessible_summary.trim().is_empty()
            && self.role_announced
            && self.slot_announced
            && self.missing_reason_announced
            && self.recovery_announced
            && self.keyboard_reachable
    }
}

/// A keyboard-complete, screen-reader-safe affordance for one placeholder action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceholderAffordance {
    /// Which action this affordance offers.
    pub action: PlaceholderActionKind,
    /// Opaque command id; the action is reachable from the command palette by this id.
    pub command_id: String,
    /// Keyboard shortcut token; the action is operable without a pointer.
    pub keyboard_shortcut: String,
    /// Deterministic focus order within the placeholder, so keyboard navigation is unambiguous.
    pub focus_order: u32,
    /// Screen-reader label naming the action and the surface it would restore.
    pub accessible_label: String,
    /// Attestation that the action stays scoped to this one pane slot. Must be true.
    pub scoped_to_slot: bool,
}

impl PlaceholderAffordance {
    /// Whether the affordance is keyboard-complete and screen-reader-safe.
    pub fn is_accessible(&self) -> bool {
        !self.command_id.trim().is_empty()
            && !self.keyboard_shortcut.trim().is_empty()
            && !self.accessible_label.trim().is_empty()
    }
}

// --- Placeholder card ----------------------------------------------------------------------------

/// One missing-surface placeholder: a pane that could not hydrate on restore, with its original role,
/// slot, missing-dependency class, last-known provenance, and recovery actions preserved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissingSurfacePlaceholderCard {
    /// Stable placeholder id.
    pub placeholder_id: String,
    /// Owner accountable for the placeholder.
    pub owner: String,
    /// Re-entry flow that produced this placeholder. Reused from the restore-provenance vocabulary.
    pub reentry_surface: ReentrySurface,
    /// The role the original surface occupied, preserved on the placeholder.
    pub pane_role: PaneRole,
    /// Stable pane-tree id of the slot the placeholder occupies. Survives the substitution.
    pub pane_id: String,
    /// Human/diffable slot path within the restored window topology.
    pub slot_path: String,
    /// Opaque ref to the restored window that hosts the slot.
    pub window_ref: String,
    /// What is missing.
    pub missing_dependency_class: MissingDependencyClass,
    /// Opaque ref to the specific missing dependency.
    pub missing_dependency_ref: String,
    /// Dependency condition observed; must be missing for a placeholder to exist.
    pub dependency_condition: DependencyCondition,
    /// Last-known provenance of the surface the placeholder stands in for.
    pub last_known_provenance: PlaceholderProvenance,
    /// Redaction class: what the record excludes. Reused from the matrix vocabulary.
    #[serde(default)]
    pub redaction_class: Vec<RedactionExclusion>,
    /// How the missing dependency is handled; must preserve the slot, never a silent delete.
    pub substitution_behavior: MissingDependencyBehavior,
    /// Best restore fidelity the surface would have had with the dependency present, before the gate.
    pub declared_resulting_fidelity: RestoreFidelityClass,
    /// Observed schema condition.
    pub schema_condition: SchemaCondition,
    /// Observed topology condition.
    pub topology_condition: TopologyCondition,
    /// How fresh the retained evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// Restore fidelity actually published after the gate; must equal
    /// [`MissingSurfacePlaceholderCard::achieved_fidelity`]. Never an exact restore.
    pub published_fidelity: RestoreFidelityClass,
    /// Headline downgrade reasons; must equal the recomputed set and include a missing dependency.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Recovery next step surfaced for the placeholder; must equal the recomputed path.
    pub recovery_path: RecoveryPath,
    /// Bounded, accessible recovery affordances offered on the placeholder.
    #[serde(default)]
    pub available_actions: Vec<PlaceholderAffordance>,
    /// Screen-reader narration and focus state for the placeholder slot.
    pub narration: PlaceholderNarration,
    /// Caveats attached to the published fidelity.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields that the missing surface narrowed below a full restore.
    #[serde(default)]
    pub narrowed_fields: Vec<String>,
    /// Ref to the placeholder's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the placeholder answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl MissingSurfacePlaceholderCard {
    /// The restore fidelity the gate permits this placeholder to publish.
    ///
    /// The weakest ceiling implied by the declared resulting fidelity and the dependency, schema,
    /// topology, and evidence-freshness conditions. Because a placeholder always describes a missing
    /// dependency, the dependency ceiling caps it at a slot-preserving layout-only restore (or manual
    /// review when the dependency root is gone) — a missing surface can never publish an exact
    /// restore.
    pub fn achieved_fidelity(&self) -> RestoreFidelityClass {
        self.declared_resulting_fidelity
            .min(self.dependency_condition.fidelity_ceiling())
            .min(self.schema_condition.fidelity_ceiling())
            .min(self.topology_condition.fidelity_ceiling())
            .min(self.evidence_freshness.fidelity_ceiling())
    }

    /// The headline downgrade reasons recomputed from the placeholder's observed conditions.
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

    /// The recovery next step the gate must record, derived from the placeholder's conditions.
    ///
    /// A manual-review placeholder (the dependency root is gone) points at review; otherwise a missing
    /// dependency points at relocating it, a migratable schema points at a compatible restore, a
    /// changed topology points at reopening as context, and stale evidence points at a refresh.
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

    /// Whether the gate narrowed the achieved fidelity below the declared maximum.
    pub fn is_downgraded(&self) -> bool {
        self.achieved_fidelity().rank() < self.declared_resulting_fidelity.rank()
    }

    /// Whether the placeholder is held for manual review (its dependency root is gone).
    pub fn is_manual_review(&self) -> bool {
        self.achieved_fidelity() == RestoreFidelityClass::ManualReview
    }

    /// The recovery affordance the placeholder's missing-dependency class calls for.
    pub fn primary_recovery_action(&self) -> PlaceholderActionKind {
        self.missing_dependency_class.primary_recovery_action()
    }

    /// The affordance for an action, if the placeholder offers it.
    pub fn action(&self, kind: PlaceholderActionKind) -> Option<&PlaceholderAffordance> {
        self.available_actions.iter().find(|a| a.action == kind)
    }

    /// Whether the placeholder offers an action.
    pub fn has_action(&self, kind: PlaceholderActionKind) -> bool {
        self.action(kind).is_some()
    }

    /// Whether the placeholder guarantees every redaction exclusion the record must carry.
    pub fn has_required_exclusions(&self) -> bool {
        let present: BTreeSet<RedactionExclusion> = self.redaction_class.iter().copied().collect();
        RedactionExclusion::ALL.iter().all(|e| present.contains(e))
    }

    /// Whether the stored published fidelity, reasons, and path all agree with the recomputed gate.
    pub fn gate_consistent(&self) -> bool {
        self.published_fidelity == self.achieved_fidelity()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.recovery_path == self.computed_recovery_path()
    }

    /// A plain-language summary line for the placeholder.
    fn summary_line(&self) -> String {
        format!(
            "{} pane in slot {} is missing {} ({}); slot preserved, published {}, recovery {}",
            self.pane_role.as_str(),
            self.slot_path,
            self.missing_dependency_class.as_str(),
            self.dependency_condition.as_str(),
            self.published_fidelity.as_str(),
            self.recovery_path.as_str()
        )
    }
}

// --- Consumer binding ----------------------------------------------------------------------------

/// A binding wiring a parity surface to this missing-surface-placeholders packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceholderConsumerBinding {
    /// Surface this binding wires.
    pub consumer_surface: PlaceholderConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Missing-surface packet id this surface ingests.
    pub record_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface carries this record rather than a parallel summary.
    pub ingests_record: bool,
    /// True when the surface preserves the missing-dependency-class labels verbatim.
    pub preserves_missing_class_labels: bool,
    /// True when the surface preserves the affected pane-role labels verbatim.
    pub preserves_pane_role_labels: bool,
    /// True when the surface names the affected-surface counts.
    pub names_affected_counts: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl PlaceholderConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.record_packet_id_ref == packet_id
            && self.ingests_record
            && self.preserves_missing_class_labels
            && self.preserves_pane_role_labels
            && self.names_affected_counts
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

// --- Summary and views ---------------------------------------------------------------------------

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MissingSurfacePlaceholdersSummary {
    /// Total placeholders.
    pub placeholders: usize,
    /// Placeholders for a missing extension.
    pub extension_missing: usize,
    /// Placeholders for an uninstalled feature pack.
    pub feature_pack_missing: usize,
    /// Placeholders for an unreachable remote target.
    pub remote_target_missing: usize,
    /// Placeholders for a down backing service.
    pub backing_service_missing: usize,
    /// Placeholders publishing a layout-only restore.
    pub layout_only_placeholders: usize,
    /// Placeholders held for manual review.
    pub manual_review_placeholders: usize,
    /// Placeholders whose slot is preserved with a placeholder card.
    pub slot_preserved_placeholders: usize,
    /// Placeholders whose surface is reopened as context.
    pub reopened_as_context_placeholders: usize,
    /// Distinct pane roles affected.
    pub affected_pane_roles: usize,
}

/// A per-missing-class affected-surface count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderClassCount {
    /// Missing-dependency-class token.
    pub missing_dependency_class: String,
    /// Number of placeholders for the class.
    pub count: usize,
}

/// A per-pane-role affected-surface count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderRoleCount {
    /// Pane-role token.
    pub pane_role: String,
    /// Number of placeholders affecting the role.
    pub count: usize,
}

/// A plain-language diagnostics row downstream surfaces render instead of restating each placeholder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderDiagnosticsRow {
    /// Pane-role token.
    pub pane_role: String,
    /// Slot path.
    pub slot_path: String,
    /// Re-entry-surface token.
    pub reentry_surface: String,
    /// Missing-dependency-class token.
    pub missing_dependency_class: String,
    /// Dependency-condition token.
    pub dependency_condition: String,
    /// Substitution-behavior token.
    pub substitution_behavior: String,
    /// Published restore-fidelity token.
    pub published_fidelity: String,
    /// Recovery-next-step token.
    pub recovery_path: String,
    /// Action tokens offered on the placeholder.
    pub actions: Vec<String>,
    /// Human-readable summary line.
    pub summary: String,
}

/// The diagnostics view downstream surfaces — diagnostics, support packets, and compare/export
/// summaries — render to name the missing-surface classes and affected pane roles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderDiagnosticsView {
    /// Packet id this view was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows, one per placeholder.
    pub rows: Vec<PlaceholderDiagnosticsRow>,
    /// Affected-surface counts by missing-dependency class.
    pub by_missing_class: Vec<PlaceholderClassCount>,
    /// Affected-surface counts by pane role.
    pub by_pane_role: Vec<PlaceholderRoleCount>,
    /// Total placeholders.
    pub total: usize,
    /// Placeholders publishing a layout-only restore.
    pub layout_only_count: usize,
    /// Placeholders held for manual review.
    pub manual_review_count: usize,
}

// --- Packet --------------------------------------------------------------------------------------

/// The typed M5 missing-surface-placeholders packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MissingSurfacePlaceholders {
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
    /// Remembered-state artifact class this packet governs; always `placeholder_card`.
    pub governs_artifact_class: RememberedArtifactClass,
    /// Closed restore-fidelity-class vocabulary.
    pub restore_fidelity_classes: Vec<RestoreFidelityClass>,
    /// Closed remembered-state artifact-class vocabulary.
    pub artifact_classes: Vec<RememberedArtifactClass>,
    /// Closed missing-dependency-class vocabulary.
    pub missing_dependency_classes: Vec<MissingDependencyClass>,
    /// Closed pane-role vocabulary.
    pub pane_roles: Vec<PaneRole>,
    /// Closed re-entry-surface vocabulary.
    pub reentry_surfaces: Vec<ReentrySurface>,
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
    pub action_kinds: Vec<PlaceholderActionKind>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<PlaceholderConsumerSurface>,
    /// Missing-surface placeholders, one per pane that could not hydrate.
    #[serde(default)]
    pub placeholders: Vec<MissingSurfacePlaceholderCard>,
    /// Consumer bindings, one per required parity surface.
    #[serde(default)]
    pub consumer_bindings: Vec<PlaceholderConsumerBinding>,
    /// Summary counts.
    pub summary: M5MissingSurfacePlaceholdersSummary,
}

impl M5MissingSurfacePlaceholders {
    /// Returns the placeholder with the given id.
    pub fn placeholder(&self, placeholder_id: &str) -> Option<&MissingSurfacePlaceholderCard> {
        self.placeholders
            .iter()
            .find(|p| p.placeholder_id == placeholder_id)
    }

    /// Placeholders affecting a pane role.
    pub fn placeholders_for_role(
        &self,
        role: PaneRole,
    ) -> impl Iterator<Item = &MissingSurfacePlaceholderCard> {
        self.placeholders
            .iter()
            .filter(move |p| p.pane_role == role)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: PlaceholderConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every placeholder agrees with the recomputed gate.
    pub fn all_placeholders_gate_consistent(&self) -> bool {
        self.placeholders.iter().all(|p| p.gate_consistent())
    }

    /// The number of distinct pane roles affected by a placeholder.
    pub fn affected_pane_role_count(&self) -> usize {
        let roles: BTreeSet<PaneRole> = self.placeholders.iter().map(|p| p.pane_role).collect();
        roles.len()
    }

    /// Recomputes the summary block from the placeholders.
    pub fn computed_summary(&self) -> M5MissingSurfacePlaceholdersSummary {
        let count_class = |class: MissingDependencyClass| {
            self.placeholders
                .iter()
                .filter(|p| p.missing_dependency_class == class)
                .count()
        };
        let count_fidelity = |class: RestoreFidelityClass| {
            self.placeholders
                .iter()
                .filter(|p| p.published_fidelity == class)
                .count()
        };
        let count_behavior = |behavior: MissingDependencyBehavior| {
            self.placeholders
                .iter()
                .filter(|p| p.substitution_behavior == behavior)
                .count()
        };
        M5MissingSurfacePlaceholdersSummary {
            placeholders: self.placeholders.len(),
            extension_missing: count_class(MissingDependencyClass::Extension),
            feature_pack_missing: count_class(MissingDependencyClass::FeaturePack),
            remote_target_missing: count_class(MissingDependencyClass::RemoteTarget),
            backing_service_missing: count_class(MissingDependencyClass::BackingService),
            layout_only_placeholders: count_fidelity(RestoreFidelityClass::LayoutOnly),
            manual_review_placeholders: count_fidelity(RestoreFidelityClass::ManualReview),
            slot_preserved_placeholders: count_behavior(
                MissingDependencyBehavior::PlaceholderSlotPreserved,
            ),
            reopened_as_context_placeholders: count_behavior(
                MissingDependencyBehavior::ReopenAsContext,
            ),
            affected_pane_roles: self.affected_pane_role_count(),
        }
    }

    /// Produces the diagnostics view downstream surfaces render to name the missing-surface classes
    /// and affected pane roles.
    pub fn diagnostics_view(&self) -> PlaceholderDiagnosticsView {
        let rows = self
            .placeholders
            .iter()
            .map(|p| PlaceholderDiagnosticsRow {
                pane_role: p.pane_role.as_str().to_owned(),
                slot_path: p.slot_path.clone(),
                reentry_surface: p.reentry_surface.as_str().to_owned(),
                missing_dependency_class: p.missing_dependency_class.as_str().to_owned(),
                dependency_condition: p.dependency_condition.as_str().to_owned(),
                substitution_behavior: p.substitution_behavior.as_str().to_owned(),
                published_fidelity: p.published_fidelity.as_str().to_owned(),
                recovery_path: p.recovery_path.as_str().to_owned(),
                actions: p
                    .available_actions
                    .iter()
                    .map(|a| a.action.as_str().to_owned())
                    .collect(),
                summary: p.summary_line(),
            })
            .collect();
        let by_missing_class = MissingDependencyClass::ALL
            .iter()
            .map(|class| PlaceholderClassCount {
                missing_dependency_class: class.as_str().to_owned(),
                count: self
                    .placeholders
                    .iter()
                    .filter(|p| p.missing_dependency_class == *class)
                    .count(),
            })
            .filter(|row| row.count > 0)
            .collect();
        let by_pane_role = PaneRole::ALL
            .iter()
            .map(|role| PlaceholderRoleCount {
                pane_role: role.as_str().to_owned(),
                count: self
                    .placeholders
                    .iter()
                    .filter(|p| p.pane_role == *role)
                    .count(),
            })
            .filter(|row| row.count > 0)
            .collect();
        PlaceholderDiagnosticsView {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            by_missing_class,
            by_pane_role,
            total: self.placeholders.len(),
            layout_only_count: self
                .placeholders
                .iter()
                .filter(|p| p.published_fidelity == RestoreFidelityClass::LayoutOnly)
                .count(),
            manual_review_count: self
                .placeholders
                .iter()
                .filter(|p| p.published_fidelity == RestoreFidelityClass::ManualReview)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact missing-surface record.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5MissingSurfacePlaceholdersSupportExport {
        M5MissingSurfacePlaceholdersSupportExport {
            record_kind: M5_MISSING_SURFACE_PLACEHOLDERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_MISSING_SURFACE_PLACEHOLDERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            record_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            record: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5MissingSurfacePlaceholdersViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_panes = BTreeSet::new();
        for placeholder in &self.placeholders {
            if !seen_ids.insert(placeholder.placeholder_id.clone()) {
                violations.push(
                    M5MissingSurfacePlaceholdersViolation::DuplicatePlaceholderId {
                        placeholder_id: placeholder.placeholder_id.clone(),
                    },
                );
            }
            if !seen_panes.insert(placeholder.pane_id.clone()) {
                violations.push(M5MissingSurfacePlaceholdersViolation::DuplicatePaneSlot {
                    pane_id: placeholder.pane_id.clone(),
                });
            }
            self.validate_placeholder(placeholder, &mut violations);
        }

        for surface in PlaceholderConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(
                    M5MissingSurfacePlaceholdersViolation::MissingConsumerBinding {
                        surface: surface.as_str(),
                    },
                );
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(
                    M5MissingSurfacePlaceholdersViolation::ConsumerBindingDrift {
                        binding_ref: binding.binding_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5MissingSurfacePlaceholdersViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5MissingSurfacePlaceholdersViolation>) {
        if self.schema_version != M5_MISSING_SURFACE_PLACEHOLDERS_SCHEMA_VERSION {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_MISSING_SURFACE_PLACEHOLDERS_RECORD_KIND {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        if self.governs_artifact_class != RememberedArtifactClass::PlaceholderCard {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::WrongGovernedArtifactClass {
                    actual: self.governs_artifact_class.as_str(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MissingSurfacePlaceholdersViolation::EmptyField {
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
                "artifact_classes",
                self.artifact_classes == RememberedArtifactClass::ALL.to_vec(),
            ),
            (
                "missing_dependency_classes",
                self.missing_dependency_classes == MissingDependencyClass::ALL.to_vec(),
            ),
            ("pane_roles", self.pane_roles == PaneRole::ALL.to_vec()),
            (
                "reentry_surfaces",
                self.reentry_surfaces == ReentrySurface::ALL.to_vec(),
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
                self.action_kinds == PlaceholderActionKind::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == PlaceholderConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    M5MissingSurfacePlaceholdersViolation::ClosedVocabularyDrift {
                        field_name: field,
                    },
                );
            }
        }
    }

    fn validate_placeholder(
        &self,
        placeholder: &MissingSurfacePlaceholderCard,
        violations: &mut Vec<M5MissingSurfacePlaceholdersViolation>,
    ) {
        let id = placeholder.placeholder_id.clone();
        for (field, value) in [
            ("placeholder_id", &placeholder.placeholder_id),
            ("owner", &placeholder.owner),
            ("pane_id", &placeholder.pane_id),
            ("slot_path", &placeholder.slot_path),
            ("window_ref", &placeholder.window_ref),
            (
                "missing_dependency_ref",
                &placeholder.missing_dependency_ref,
            ),
            ("evidence_ref", &placeholder.evidence_ref),
            ("scope_snapshot_ref", &placeholder.scope_snapshot_ref),
            ("note", &placeholder.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MissingSurfacePlaceholdersViolation::EmptyField {
                    id: id.clone(),
                    field_name: field,
                });
            }
        }

        // The last-known provenance must be complete, so a placeholder keeps a real history rather
        // than reading as a never-populated empty tab that erases provenance.
        if !placeholder.last_known_provenance.is_complete() {
            violations
                .push(M5MissingSurfacePlaceholdersViolation::ProvenanceErased { id: id.clone() });
        }

        // Every record excludes secrets, live authority, machine-local anchors, and raw provider
        // payloads; it is metadata only.
        if !placeholder.has_required_exclusions() {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::MissingRedactionExclusion { id: id.clone() },
            );
        }

        // A placeholder exists only because a dependency is missing; a present dependency means the
        // slot should have hydrated, not been replaced.
        if !placeholder.dependency_condition.is_missing() {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::PlaceholderWithoutMissingDependency {
                    id: id.clone(),
                },
            );
        }

        // A missing dependency never silently deletes layout.
        if !placeholder.substitution_behavior.preserves_slot() {
            violations
                .push(M5MissingSurfacePlaceholdersViolation::SilentLayoutDelete { id: id.clone() });
        }

        let achieved = placeholder.achieved_fidelity();
        // A missing surface can never publish an exact restore.
        if placeholder.published_fidelity == RestoreFidelityClass::ExactRestore {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::MissingSurfacePublishedExact {
                    id: id.clone(),
                },
            );
        }
        // The published fidelity must equal the gate's recomputed ceiling.
        if placeholder.published_fidelity != achieved {
            violations.push(M5MissingSurfacePlaceholdersViolation::OverstatedFidelity {
                id: id.clone(),
                published: placeholder.published_fidelity.as_str(),
                computed: achieved.as_str(),
            });
        }
        // The published fidelity may never exceed the declared resulting fidelity.
        if placeholder.published_fidelity.rank() > placeholder.declared_resulting_fidelity.rank() {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::ExceedsDeclaredFidelity {
                    id: id.clone(),
                    published: placeholder.published_fidelity.as_str(),
                    declared: placeholder.declared_resulting_fidelity.as_str(),
                },
            );
        }

        let computed = placeholder.computed_downgrade_reasons();
        if placeholder.downgrade_reasons != computed {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::DowngradeReasonsMismatch { id: id.clone() },
            );
        }
        let computed_path = placeholder.computed_recovery_path();
        if placeholder.recovery_path != computed_path {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::RecoveryPathMismatch {
                    id: id.clone(),
                    declared: placeholder.recovery_path.as_str(),
                    required: computed_path.as_str(),
                },
            );
        }

        // Provenance is never hidden: every placeholder offers the read-only open-details action.
        if !placeholder.has_action(PlaceholderActionKind::OpenDetails) {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::MissingOpenDetailsAction { id: id.clone() },
            );
        }
        // The placeholder offers the recovery action its missing-dependency class calls for.
        let primary = placeholder.primary_recovery_action();
        if !placeholder.has_action(primary) {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::MissingRecoveryAction {
                    id: id.clone(),
                    action: primary.as_str(),
                },
            );
        }
        // When the surface is reopened as context, the reopen-as-context action must be offered.
        if placeholder.substitution_behavior == MissingDependencyBehavior::ReopenAsContext
            && !placeholder.has_action(PlaceholderActionKind::ReopenAsContext)
        {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::MissingReopenAsContextAction {
                    id: id.clone(),
                },
            );
        }

        let mut seen_actions = BTreeSet::new();
        let mut seen_focus = BTreeSet::new();
        for affordance in &placeholder.available_actions {
            if !seen_actions.insert(affordance.action) {
                violations.push(M5MissingSurfacePlaceholdersViolation::DuplicateAction {
                    id: id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !seen_focus.insert(affordance.focus_order) {
                violations.push(M5MissingSurfacePlaceholdersViolation::DuplicateFocusOrder {
                    id: id.clone(),
                });
            }
            if !affordance.is_accessible() {
                violations.push(M5MissingSurfacePlaceholdersViolation::InaccessibleAction {
                    id: id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !affordance.scoped_to_slot {
                violations.push(M5MissingSurfacePlaceholdersViolation::UnscopedAction {
                    id: id.clone(),
                    action: affordance.action.as_str(),
                });
            }
        }

        // The placeholder slot stays keyboard-reachable and its narration announces the role, slot,
        // missing reason, and recovery.
        if !placeholder.narration.is_sensible() {
            violations.push(
                M5MissingSurfacePlaceholdersViolation::InaccessibleNarration { id: id.clone() },
            );
        }

        // A placeholder is always narrowed below a full restore, so it must name a caveat and what
        // was narrowed.
        if placeholder.caveats.is_empty() {
            violations.push(M5MissingSurfacePlaceholdersViolation::EmptyField {
                id: id.clone(),
                field_name: "caveats",
            });
        }
        if placeholder.narrowed_fields.is_empty() {
            violations.push(M5MissingSurfacePlaceholdersViolation::EmptyField {
                id,
                field_name: "narrowed_fields",
            });
        }
    }
}

/// A validation violation for [`M5MissingSurfacePlaceholders`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum M5MissingSurfacePlaceholdersViolation {
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
    /// The packet does not govern the placeholder-card artifact class.
    WrongGovernedArtifactClass {
        /// Artifact-class token found in the packet.
        actual: &'static str,
    },
    /// A closed vocabulary disagrees with this build's canonical list.
    ClosedVocabularyDrift {
        /// Offending field.
        field_name: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Placeholder or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A placeholder id appears more than once.
    DuplicatePlaceholderId {
        /// Duplicate placeholder id.
        placeholder_id: String,
    },
    /// Two placeholders claim the same pane slot.
    DuplicatePaneSlot {
        /// Duplicate pane id.
        pane_id: String,
    },
    /// A placeholder records no missing dependency, so the slot should have hydrated.
    PlaceholderWithoutMissingDependency {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder erases provenance — incomplete producer or no last-known attach.
    ProvenanceErased {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder would silently delete layout when the dependency is missing.
    SilentLayoutDelete {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder publishes an exact restore despite a missing surface.
    MissingSurfacePublishedExact {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder publishes a fidelity beyond what the gate computes.
    OverstatedFidelity {
        /// Placeholder id.
        id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Computed fidelity token.
        computed: &'static str,
    },
    /// A placeholder publishes a fidelity above its declared resulting fidelity.
    ExceedsDeclaredFidelity {
        /// Placeholder id.
        id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Declared resulting fidelity token.
        declared: &'static str,
    },
    /// A placeholder's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Placeholder id.
        id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A placeholder does not guarantee every required redaction exclusion.
    MissingRedactionExclusion {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder omits the read-only open-details action, hiding its provenance.
    MissingOpenDetailsAction {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder omits the recovery action its missing-dependency class calls for.
    MissingRecoveryAction {
        /// Placeholder id.
        id: String,
        /// Required action token.
        action: &'static str,
    },
    /// A reopened-as-context placeholder omits the reopen-as-context action.
    MissingReopenAsContextAction {
        /// Placeholder id.
        id: String,
    },
    /// A placeholder offers the same action twice.
    DuplicateAction {
        /// Placeholder id.
        id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// Two affordances in a placeholder share a focus order, making keyboard navigation ambiguous.
    DuplicateFocusOrder {
        /// Placeholder id.
        id: String,
    },
    /// An affordance lacks a command id, keyboard shortcut, or screen-reader label.
    InaccessibleAction {
        /// Placeholder id.
        id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// An affordance is not scoped to the one pane slot.
    UnscopedAction {
        /// Placeholder id.
        id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// A placeholder's narration does not announce the role, slot, missing reason, and recovery, or
    /// the slot is not keyboard-reachable.
    InaccessibleNarration {
        /// Placeholder id.
        id: String,
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
    /// The summary counts disagree with the placeholders.
    SummaryMismatch,
}

impl fmt::Display for M5MissingSurfacePlaceholdersViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::WrongGovernedArtifactClass { actual } => write!(
                f,
                "packet governs artifact class {actual}, expected placeholder_card"
            ),
            Self::ClosedVocabularyDrift { field_name } => {
                write!(f, "closed vocabulary {field_name} disagrees with this build")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicatePlaceholderId { placeholder_id } => {
                write!(f, "duplicate placeholder id {placeholder_id}")
            }
            Self::DuplicatePaneSlot { pane_id } => {
                write!(f, "two placeholders claim pane slot {pane_id}")
            }
            Self::PlaceholderWithoutMissingDependency { id } => write!(
                f,
                "placeholder {id} records no missing dependency; the slot should have hydrated"
            ),
            Self::ProvenanceErased { id } => {
                write!(f, "placeholder {id} erases its last-known provenance")
            }
            Self::SilentLayoutDelete { id } => write!(
                f,
                "placeholder {id} would silently delete layout on a missing dependency"
            ),
            Self::MissingSurfacePublishedExact { id } => write!(
                f,
                "placeholder {id} publishes an exact restore despite a missing surface"
            ),
            Self::OverstatedFidelity {
                id,
                published,
                computed,
            } => write!(
                f,
                "placeholder {id} publishes fidelity {published} but the gate computes {computed}"
            ),
            Self::ExceedsDeclaredFidelity {
                id,
                published,
                declared,
            } => write!(
                f,
                "placeholder {id} publishes fidelity {published} above declared {declared}"
            ),
            Self::DowngradeReasonsMismatch { id } => {
                write!(f, "placeholder {id} downgrade reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch {
                id,
                declared,
                required,
            } => write!(
                f,
                "placeholder {id} records recovery {declared} but the gate requires {required}"
            ),
            Self::MissingRedactionExclusion { id } => write!(
                f,
                "placeholder {id} does not guarantee its required redaction exclusions"
            ),
            Self::MissingOpenDetailsAction { id } => {
                write!(f, "placeholder {id} omits the open-details action")
            }
            Self::MissingRecoveryAction { id, action } => write!(
                f,
                "placeholder {id} omits the {action} recovery action its missing class requires"
            ),
            Self::MissingReopenAsContextAction { id } => write!(
                f,
                "placeholder {id} reopens as context but omits the reopen-as-context action"
            ),
            Self::DuplicateAction { id, action } => {
                write!(f, "placeholder {id} offers action {action} twice")
            }
            Self::DuplicateFocusOrder { id } => {
                write!(f, "placeholder {id} has affordances sharing a focus order")
            }
            Self::InaccessibleAction { id, action } => write!(
                f,
                "placeholder {id} action {action} lacks a command id, shortcut, or label"
            ),
            Self::UnscopedAction { id, action } => {
                write!(f, "placeholder {id} action {action} is not scoped to the slot")
            }
            Self::InaccessibleNarration { id } => write!(
                f,
                "placeholder {id} narration does not announce role/slot/reason/recovery or is not keyboard-reachable"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "parity surface {surface} has no consumer binding")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "consumer binding {binding_ref} does not preserve the record")
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the placeholders")
            }
        }
    }
}

impl Error for M5MissingSurfacePlaceholdersViolation {}

/// Stable record-kind tag for [`M5MissingSurfacePlaceholdersSupportExport`].
pub const M5_MISSING_SURFACE_PLACEHOLDERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_missing_surface_placeholders_support_export";

/// Support-export wrapper preserving the missing-surface record verbatim for support and evidence
/// packets, so an exported diagnostics or support bundle can name the missing-surface classes and
/// affected pane roles rather than a weaker summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MissingSurfacePlaceholdersSupportExport {
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
    pub record: M5MissingSurfacePlaceholders,
}

impl M5MissingSurfacePlaceholdersSupportExport {
    /// Whether the export preserves the same packet id and a clean record.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_MISSING_SURFACE_PLACEHOLDERS_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_MISSING_SURFACE_PLACEHOLDERS_SCHEMA_VERSION
            && self.record_packet_id_ref == self.record.packet_id
            && self.raw_private_material_excluded
            && self.record.validate().is_empty()
    }
}

/// Loads the embedded M5 missing-surface-placeholders packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5MissingSurfacePlaceholders`].
pub fn current_m5_missing_surface_placeholders(
) -> Result<M5MissingSurfacePlaceholders, serde_json::Error> {
    serde_json::from_str(M5_MISSING_SURFACE_PLACEHOLDERS_JSON)
}

#[cfg(test)]
mod tests;
