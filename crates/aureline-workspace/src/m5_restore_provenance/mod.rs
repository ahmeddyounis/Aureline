//! M5 restore-provenance cards: the one place every restore, import, crash recovery, and
//! browser/companion re-entry discloses not only *that* something came back, but *how well* it
//! came back — exact, compatible, layout-only, or manual-review.
//!
//! The serialization-and-restore matrix classifies *what* M5 may remember and the achievable
//! restore-fidelity classes; the remembered-state inspector projects *what is remembered* for the
//! current workspace. This packet is the **event-facing** projection: one
//! [`RestoreProvenanceCard`] per re-entry surface that attaches to a restore/import/handoff event
//! its source, producer/build provenance, restored schema version, redaction class, and the
//! resulting restore fidelity. It never invents its own fidelity language: every card reuses the
//! canonical [`RestoreFidelityClass`], [`RememberedArtifactClass`], [`RedactionExclusion`], and the
//! schema/dependency/topology/freshness condition vocabularies from
//! [`crate::m5_serialization_and_restore_matrix`], so restore meaning cannot fork between desktop
//! restore, portable-state import, crash recovery, support-packet replay, and companion/browser
//! re-entry.
//!
//! - [`RestoreSource`] names where the remembered state came from (`auto_checkpoint`,
//!   `manual_export`, `backup`, `sync`, `import`, `browser_companion_handoff`) and the highest
//!   restore fidelity that source can imply. A browser/companion handoff is a **contextual reopen**,
//!   so it can never imply a full (exact) restore ([`RestoreSource::fidelity_ceiling`]).
//! - [`ReentrySurface`] names the re-entry flow the card belongs to. Exactly one card is required
//!   per surface, so the exact/compatible/layout-only/manual-review labels are standardized across
//!   every M5 re-entry flow rather than re-invented surface by surface.
//! - [`ProvenanceActionKind`] models the open-details, compare, and recovery-next-step affordances.
//!   Open-details is always present so provenance is never hidden; compare and recovery-next-step
//!   are preserved wherever the fidelity was narrowed or a dependency was missing.
//! - [`ProvenanceConsumerBinding`] wires the surfaces that must carry the **same** record —
//!   exported diagnostics, support packets, crash-recovery packets, and companion handoff — to this
//!   one packet, so they preserve the provenance and fidelity labels verbatim instead of inventing
//!   weaker summaries.
//!
//! The packet is fail-closed. A card's achieved fidelity is the **weakest ceiling** implied by its
//! declared resulting fidelity, its source, and its schema, dependency, topology, and
//! evidence-freshness conditions ([`RestoreProvenanceCard::achieved_fidelity`]); the published
//! fidelity must equal it, so a contextual handoff, a schema drift, a missing dependency, a changed
//! topology, or stale evidence can never publish an exact restore by inertia. A missing dependency
//! never silently deletes layout, and an exact-restore card must be genuinely clean.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-restore-provenance.json` and embedded
//! here. It is metadata-only: every field is a typed state, a count, an opaque ref, or a
//! plain-language label, and it carries no credential bodies, raw provider payloads, live authority
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

/// Supported M5 restore-provenance packet schema version.
pub const M5_RESTORE_PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_RESTORE_PROVENANCE_RECORD_KIND: &str = "m5_restore_provenance";

/// Repo-relative path to the checked-in packet.
pub const M5_RESTORE_PROVENANCE_PATH: &str = "artifacts/workspace/m5/m5-restore-provenance.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_RESTORE_PROVENANCE_SCHEMA_REF: &str =
    "schemas/workspace/m5-restore-provenance.schema.json";

/// Repo-relative path to the companion document.
pub const M5_RESTORE_PROVENANCE_DOC_REF: &str = "docs/workspace/m5/m5-restore-fidelity.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_RESTORE_PROVENANCE_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-restore-provenance.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_RESTORE_PROVENANCE_FIXTURE_DIR: &str = "fixtures/workspace/m5/m5-restore-provenance";

/// Embedded checked-in packet JSON.
pub const M5_RESTORE_PROVENANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-restore-provenance.json"
));

// --- Source and surface vocabulary ---------------------------------------------------------------

/// Where the remembered state a restore-provenance card describes came from.
///
/// A [`RestoreSource::BrowserCompanionHandoff`] is a contextual reopen, not a value-for-value
/// resume, so [`RestoreSource::fidelity_ceiling`] caps it at [`RestoreFidelityClass::LayoutOnly`].
/// Every other source can, in principle, restore exactly; the observed schema, dependency,
/// topology, and freshness conditions then narrow it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSource {
    /// An automatic workspace checkpoint written during normal operation.
    AutoCheckpoint,
    /// A workspace state package a user exported by hand.
    ManualExport,
    /// A backup snapshot.
    Backup,
    /// State synchronized from another machine or install.
    Sync,
    /// A portable-state package imported from elsewhere.
    Import,
    /// A browser or mobile companion handoff packet.
    BrowserCompanionHandoff,
}

impl RestoreSource {
    /// Every restore source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AutoCheckpoint,
        Self::ManualExport,
        Self::Backup,
        Self::Sync,
        Self::Import,
        Self::BrowserCompanionHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoCheckpoint => "auto_checkpoint",
            Self::ManualExport => "manual_export",
            Self::Backup => "backup",
            Self::Sync => "sync",
            Self::Import => "import",
            Self::BrowserCompanionHandoff => "browser_companion_handoff",
        }
    }

    /// Highest restore fidelity this source can ever imply.
    ///
    /// A browser/companion handoff is a contextual reopen and is capped at
    /// [`RestoreFidelityClass::LayoutOnly`]; every other source is capped at
    /// [`RestoreFidelityClass::ExactRestore`] and narrowed by the observed conditions.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::BrowserCompanionHandoff => RestoreFidelityClass::LayoutOnly,
            _ => RestoreFidelityClass::ExactRestore,
        }
    }

    /// Whether this source can only reopen context and never imply a full restore.
    pub const fn is_contextual_only(self) -> bool {
        matches!(self, Self::BrowserCompanionHandoff)
    }
}

/// The re-entry flow a restore-provenance card belongs to.
///
/// Exactly one card per surface keeps the exact/compatible/layout-only/manual-review labels
/// standardized across every M5 re-entry flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReentrySurface {
    /// Desktop workspace restore on launch.
    DesktopRestore,
    /// Portable-state package import.
    PortableStateImport,
    /// Crash recovery and unsaved-state restore.
    CrashRecovery,
    /// Support-packet replay during diagnosis.
    SupportPacketReplay,
    /// Browser/mobile companion re-entry.
    CompanionBrowserReentry,
}

impl ReentrySurface {
    /// Every re-entry surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DesktopRestore,
        Self::PortableStateImport,
        Self::CrashRecovery,
        Self::SupportPacketReplay,
        Self::CompanionBrowserReentry,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopRestore => "desktop_restore",
            Self::PortableStateImport => "portable_state_import",
            Self::CrashRecovery => "crash_recovery",
            Self::SupportPacketReplay => "support_packet_replay",
            Self::CompanionBrowserReentry => "companion_browser_reentry",
        }
    }
}

/// One of the three actions a restore-provenance card can offer.
///
/// Open-details is read-only and always present so provenance is never hidden. Compare and
/// recovery-next-step are preserved wherever the fidelity was narrowed or a dependency was missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceActionKind {
    /// Open the full provenance details — source, producer, schema, redaction, and conditions.
    OpenDetails,
    /// Compare this restore against another remembered state of the same class before relying on it.
    Compare,
    /// Surface the concrete recovery next step that would restore more of the remembered state.
    RecoveryNextStep,
}

impl ProvenanceActionKind {
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

/// A surface that must carry the same restore-provenance record rather than inventing a weaker
/// summary of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceConsumerSurface {
    /// The exported workspace diagnostics bundle.
    DiagnosticsExport,
    /// The support-export packet.
    SupportPacket,
    /// The crash-recovery packet.
    CrashRecoveryPacket,
    /// The browser/mobile companion handoff packet.
    CompanionHandoff,
}

impl ProvenanceConsumerSurface {
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

// --- Provenance and affordances ------------------------------------------------------------------

/// Producer, version, and build provenance for the component that wrote the remembered state, in
/// opaque refs and a version number.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestoreProducer {
    /// Opaque reference to the component that produced the remembered state.
    pub producer_ref: String,
    /// Monotonic producer version that wrote the state.
    pub producer_version: u32,
    /// Opaque reference to the build that wrote it.
    pub build_ref: String,
}

impl RestoreProducer {
    /// Whether the provenance is complete: both refs present and the version is set.
    pub fn is_complete(&self) -> bool {
        !self.producer_ref.trim().is_empty()
            && !self.build_ref.trim().is_empty()
            && self.producer_version != 0
    }
}

/// A keyboard-complete, screen-reader-safe affordance for one restore-provenance action.
///
/// Every affordance carries a command id, a keyboard shortcut, a deterministic focus order, and a
/// screen-reader label, and stays scoped to the one restore event the card describes — no action
/// reaches beyond it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceAction {
    /// Which action this affordance offers.
    pub action: ProvenanceActionKind,
    /// Opaque command id; the action is reachable from the command palette by this id.
    pub command_id: String,
    /// Keyboard shortcut token; the flow is operable without a pointer.
    pub keyboard_shortcut: String,
    /// Deterministic focus order within the card, so keyboard navigation is unambiguous.
    pub focus_order: u32,
    /// Screen-reader label naming the action and the restore it describes.
    pub accessible_label: String,
    /// Attestation that the action stays scoped to this one restore event. Must be true.
    pub scoped_to_event: bool,
}

impl ProvenanceAction {
    /// Whether the affordance is keyboard-complete and screen-reader-safe.
    pub fn is_accessible(&self) -> bool {
        !self.command_id.trim().is_empty()
            && !self.keyboard_shortcut.trim().is_empty()
            && !self.accessible_label.trim().is_empty()
    }
}

// --- Restore-provenance card ---------------------------------------------------------------------

/// One restore-provenance card: a restore/import/handoff event with its source, producer/build
/// provenance, restored schema version, redaction class, and resulting restore fidelity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestoreProvenanceCard {
    /// Stable card id.
    pub card_id: String,
    /// Re-entry surface this card belongs to.
    pub reentry_surface: ReentrySurface,
    /// Owner accountable for the card.
    pub owner: String,
    /// Remembered-state artifact class that was restored. Reused from the matrix vocabulary.
    pub restored_artifact_class: RememberedArtifactClass,
    /// Where the remembered state came from.
    pub source: RestoreSource,
    /// Producer, version, and build that wrote the remembered state.
    pub producer: RestoreProducer,
    /// Schema version of the restored remembered state.
    pub restored_schema_version: u32,
    /// Redaction class: what the provenance record excludes. Reused from the matrix vocabulary.
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
    /// What the card does when a dependency is missing; never a silent delete.
    pub missing_dependency_behavior: MissingDependencyBehavior,
    /// Restore fidelity actually published after the gate; must equal
    /// [`RestoreProvenanceCard::achieved_fidelity`].
    pub published_fidelity: RestoreFidelityClass,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Recovery next step surfaced when the fidelity is narrowed; must equal the recomputed path.
    pub recovery_path: RecoveryPath,
    /// Bounded, accessible actions offered on the card.
    #[serde(default)]
    pub available_actions: Vec<ProvenanceAction>,
    /// Caveats attached to the published fidelity.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields that narrowed the restore below an exact resume.
    #[serde(default)]
    pub narrowed_fields: Vec<String>,
    /// Ref to the card's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the card answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl RestoreProvenanceCard {
    /// The restore fidelity the gate permits this card to publish.
    ///
    /// The weakest ceiling implied by the declared resulting fidelity, the source, and the schema,
    /// dependency, topology, and evidence-freshness conditions — so a contextual handoff, a schema
    /// drift, a missing dependency, a changed topology, or stale evidence can never publish an exact
    /// restore.
    pub fn achieved_fidelity(&self) -> RestoreFidelityClass {
        self.declared_resulting_fidelity
            .min(self.source.fidelity_ceiling())
            .min(self.schema_condition.fidelity_ceiling())
            .min(self.dependency_condition.fidelity_ceiling())
            .min(self.topology_condition.fidelity_ceiling())
            .min(self.evidence_freshness.fidelity_ceiling())
    }

    /// The headline downgrade reasons recomputed from the card's observed conditions.
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

    /// The recovery next step the gate must record, derived from the card's observed conditions and
    /// its source.
    ///
    /// Ordered by severity: a manual-review restore points at review, a missing dependency points at
    /// relocating it, a migratable schema points at a compatible restore, a changed topology or a
    /// contextual handoff points at reopening as context, stale evidence points at a refresh, and a
    /// clean card needs nothing.
    pub fn computed_recovery_path(&self) -> RecoveryPath {
        if self.achieved_fidelity() == RestoreFidelityClass::ManualReview {
            RecoveryPath::ManualReview
        } else if self.dependency_condition.is_missing() {
            RecoveryPath::RelocateDependency
        } else if self.schema_condition.is_drift() {
            RecoveryPath::RestoreCompatibly
        } else if self.topology_condition.is_changed()
            || (self.source.is_contextual_only()
                && self.achieved_fidelity().rank() < RestoreFidelityClass::ExactRestore.rank())
        {
            // A changed topology or a contextual handoff both point at reopening as context.
            RecoveryPath::ReopenAsContext
        } else if self.evidence_freshness.is_stale() {
            RecoveryPath::RefreshEvidence
        } else {
            RecoveryPath::NoneNeeded
        }
    }

    /// Whether the card achieves a clean exact restore.
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

    /// Whether the card must preserve the compare and recovery-next-step actions: it does wherever
    /// the fidelity was narrowed or a dependency was missing.
    pub fn requires_recovery_actions(&self) -> bool {
        self.is_narrowed() || self.dependency_condition.is_missing()
    }

    /// The affordance for an action, if the card offers it.
    pub fn action(&self, kind: ProvenanceActionKind) -> Option<&ProvenanceAction> {
        self.available_actions.iter().find(|a| a.action == kind)
    }

    /// Whether the card offers an action.
    pub fn has_action(&self, kind: ProvenanceActionKind) -> bool {
        self.action(kind).is_some()
    }

    /// Whether the card guarantees every redaction exclusion a provenance record must carry.
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

    /// A plain-language summary line for the card.
    fn summary_line(&self) -> String {
        format!(
            "{}: restored {} from {}, schema v{}, published {}, recovery {}",
            self.reentry_surface.as_str(),
            self.restored_artifact_class.as_str(),
            self.source.as_str(),
            self.restored_schema_version,
            self.published_fidelity.as_str(),
            self.recovery_path.as_str()
        )
    }
}

// --- Consumer binding ----------------------------------------------------------------------------

/// A binding wiring a parity surface to this restore-provenance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceConsumerBinding {
    /// Surface this binding wires.
    pub consumer_surface: ProvenanceConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Restore-provenance packet id this surface ingests.
    pub record_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface carries this record rather than a parallel summary.
    pub ingests_record: bool,
    /// True when the surface preserves the restore-fidelity labels verbatim.
    pub preserves_fidelity_labels: bool,
    /// True when the surface preserves the source/producer labels verbatim.
    pub preserves_source_labels: bool,
    /// True when the surface narrows automatically as cards are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl ProvenanceConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.record_packet_id_ref == packet_id
            && self.ingests_record
            && self.preserves_fidelity_labels
            && self.preserves_source_labels
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
pub struct M5RestoreProvenanceSummary {
    /// Total cards.
    pub cards: usize,
    /// Cards publishing an exact restore.
    pub exact_restore_cards: usize,
    /// Cards publishing a compatible restore.
    pub compatible_restore_cards: usize,
    /// Cards publishing a layout-only restore.
    pub layout_only_cards: usize,
    /// Cards held for manual review.
    pub manual_review_cards: usize,
    /// Cards the gate narrowed below their declared resulting fidelity.
    pub downgraded_cards: usize,
    /// Cards whose published fidelity is short of an exact restore.
    pub narrowed_cards: usize,
    /// Cards restored through a browser/companion handoff (contextual reopen only).
    pub handoff_cards: usize,
}

/// A plain-language card-view row downstream surfaces render instead of restating each card by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceViewRow {
    /// Re-entry surface token.
    pub reentry_surface: String,
    /// Restore-source token.
    pub source: String,
    /// Restored artifact-class token.
    pub restored_artifact_class: String,
    /// Published restore-fidelity token.
    pub published_fidelity: String,
    /// Whether the card was downgraded below its declared resulting fidelity.
    pub downgraded: bool,
    /// Whether the published fidelity is short of an exact restore.
    pub narrowed: bool,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Recovery-next-step token.
    pub recovery_path: String,
    /// Action tokens offered on the card.
    pub actions: Vec<String>,
    /// Redaction-class tokens.
    pub redaction_class: Vec<String>,
    /// Human-readable summary line.
    pub summary: String,
}

/// The plain-language card view downstream surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceView {
    /// Packet id this view was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<ProvenanceViewRow>,
    /// Cards publishing an exact restore.
    pub exact_count: usize,
    /// Cards short of an exact restore.
    pub narrowed_count: usize,
    /// Cards held for manual review.
    pub manual_review_count: usize,
}

// --- Packet --------------------------------------------------------------------------------------

/// The typed M5 restore-provenance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5RestoreProvenance {
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
    /// Closed restore-source vocabulary.
    pub restore_sources: Vec<RestoreSource>,
    /// Closed re-entry-surface vocabulary.
    pub reentry_surfaces: Vec<ReentrySurface>,
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
    pub action_kinds: Vec<ProvenanceActionKind>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ProvenanceConsumerSurface>,
    /// Restore-provenance cards, one per re-entry surface.
    #[serde(default)]
    pub cards: Vec<RestoreProvenanceCard>,
    /// Consumer bindings, one per required parity surface.
    #[serde(default)]
    pub consumer_bindings: Vec<ProvenanceConsumerBinding>,
    /// Summary counts.
    pub summary: M5RestoreProvenanceSummary,
}

impl M5RestoreProvenance {
    /// Returns the card for a re-entry surface.
    pub fn card(&self, surface: ReentrySurface) -> Option<&RestoreProvenanceCard> {
        self.cards.iter().find(|c| c.reentry_surface == surface)
    }

    /// Whether a consumer binding preserves this packet for the given surface.
    pub fn has_binding_for(&self, surface: ProvenanceConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every card agrees with the recomputed gate.
    pub fn all_cards_gate_consistent(&self) -> bool {
        self.cards.iter().all(|c| c.gate_consistent())
    }

    /// Recomputes the summary block from the cards.
    pub fn computed_summary(&self) -> M5RestoreProvenanceSummary {
        let count_fidelity = |class: RestoreFidelityClass| {
            self.cards
                .iter()
                .filter(|c| c.published_fidelity == class)
                .count()
        };
        M5RestoreProvenanceSummary {
            cards: self.cards.len(),
            exact_restore_cards: count_fidelity(RestoreFidelityClass::ExactRestore),
            compatible_restore_cards: count_fidelity(RestoreFidelityClass::CompatibleRestore),
            layout_only_cards: count_fidelity(RestoreFidelityClass::LayoutOnly),
            manual_review_cards: count_fidelity(RestoreFidelityClass::ManualReview),
            downgraded_cards: self.cards.iter().filter(|c| c.is_downgraded()).count(),
            narrowed_cards: self.cards.iter().filter(|c| c.is_narrowed()).count(),
            handoff_cards: self
                .cards
                .iter()
                .filter(|c| c.source == RestoreSource::BrowserCompanionHandoff)
                .count(),
        }
    }

    /// Produces the plain-language card view downstream surfaces render.
    pub fn card_view(&self) -> ProvenanceView {
        let rows = self
            .cards
            .iter()
            .map(|c| ProvenanceViewRow {
                reentry_surface: c.reentry_surface.as_str().to_owned(),
                source: c.source.as_str().to_owned(),
                restored_artifact_class: c.restored_artifact_class.as_str().to_owned(),
                published_fidelity: c.published_fidelity.as_str().to_owned(),
                downgraded: c.is_downgraded(),
                narrowed: c.is_narrowed(),
                downgrade_reasons: c
                    .downgrade_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                recovery_path: c.recovery_path.as_str().to_owned(),
                actions: c
                    .available_actions
                    .iter()
                    .map(|a| a.action.as_str().to_owned())
                    .collect(),
                redaction_class: c
                    .redaction_class
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                summary: c.summary_line(),
            })
            .collect();
        ProvenanceView {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            exact_count: self.cards.iter().filter(|c| c.is_exact()).count(),
            narrowed_count: self.cards.iter().filter(|c| c.is_narrowed()).count(),
            manual_review_count: self
                .cards
                .iter()
                .filter(|c| c.published_fidelity == RestoreFidelityClass::ManualReview)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact restore-provenance record.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5RestoreProvenanceSupportExport {
        M5RestoreProvenanceSupportExport {
            record_kind: M5_RESTORE_PROVENANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_RESTORE_PROVENANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            record_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            record: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5RestoreProvenanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for card in &self.cards {
            if !seen_ids.insert(card.card_id.clone()) {
                violations.push(M5RestoreProvenanceViolation::DuplicateCardId {
                    card_id: card.card_id.clone(),
                });
            }
            if !seen_surfaces.insert(card.reentry_surface) {
                violations.push(M5RestoreProvenanceViolation::DuplicateSurfaceCard {
                    surface: card.reentry_surface.as_str(),
                });
            }
            self.validate_card(card, &mut violations);
        }
        for &surface in &ReentrySurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5RestoreProvenanceViolation::MissingSurfaceCard {
                    surface: surface.as_str(),
                });
            }
        }

        for surface in ProvenanceConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5RestoreProvenanceViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5RestoreProvenanceViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5RestoreProvenanceViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5RestoreProvenanceViolation>) {
        if self.schema_version != M5_RESTORE_PROVENANCE_SCHEMA_VERSION {
            violations.push(M5RestoreProvenanceViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_RESTORE_PROVENANCE_RECORD_KIND {
            violations.push(M5RestoreProvenanceViolation::UnsupportedRecordKind {
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
                violations.push(M5RestoreProvenanceViolation::EmptyField {
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
                "restore_sources",
                self.restore_sources == RestoreSource::ALL.to_vec(),
            ),
            (
                "reentry_surfaces",
                self.reentry_surfaces == ReentrySurface::ALL.to_vec(),
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
                self.action_kinds == ProvenanceActionKind::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == ProvenanceConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5RestoreProvenanceViolation::ClosedVocabularyDrift {
                    field_name: field,
                });
            }
        }
    }

    fn validate_card(
        &self,
        card: &RestoreProvenanceCard,
        violations: &mut Vec<M5RestoreProvenanceViolation>,
    ) {
        for (field, value) in [
            ("card_id", &card.card_id),
            ("owner", &card.owner),
            ("evidence_ref", &card.evidence_ref),
            ("scope_snapshot_ref", &card.scope_snapshot_ref),
            ("note", &card.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5RestoreProvenanceViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: field,
                });
            }
        }
        if card.restored_schema_version == 0 {
            violations.push(M5RestoreProvenanceViolation::EmptyField {
                id: card.card_id.clone(),
                field_name: "restored_schema_version",
            });
        }
        if !card.producer.is_complete() {
            violations.push(M5RestoreProvenanceViolation::EmptyField {
                id: card.card_id.clone(),
                field_name: "producer",
            });
        }

        // Every provenance record excludes secrets, live authority, machine-local anchors, and raw
        // provider payloads; it is metadata only.
        if !card.has_required_exclusions() {
            violations.push(M5RestoreProvenanceViolation::MissingRedactionExclusion {
                card_id: card.card_id.clone(),
            });
        }

        // A missing dependency never silently deletes layout.
        if !card.missing_dependency_behavior.preserves_slot() {
            violations.push(M5RestoreProvenanceViolation::SilentLayoutDelete {
                card_id: card.card_id.clone(),
            });
        }

        let achieved = card.achieved_fidelity();
        // The published fidelity must equal the gate's recomputed ceiling, so a contextual handoff,
        // a schema drift, a missing dependency, a changed topology, or stale evidence can never
        // publish exact.
        if card.published_fidelity != achieved {
            violations.push(M5RestoreProvenanceViolation::OverstatedFidelity {
                card_id: card.card_id.clone(),
                published: card.published_fidelity.as_str(),
                computed: achieved.as_str(),
            });
        }
        // A browser/companion handoff (or any contextually-capped source) can never publish a
        // fidelity above its source ceiling: it must not imply a full restore.
        if card.published_fidelity.rank() > card.source.fidelity_ceiling().rank() {
            violations.push(
                M5RestoreProvenanceViolation::SourceFidelityCeilingExceeded {
                    card_id: card.card_id.clone(),
                    source: card.source.as_str(),
                    published: card.published_fidelity.as_str(),
                    ceiling: card.source.fidelity_ceiling().as_str(),
                },
            );
        }
        // The published fidelity may never exceed the declared resulting fidelity.
        if card.published_fidelity.rank() > card.declared_resulting_fidelity.rank() {
            violations.push(M5RestoreProvenanceViolation::ExceedsDeclaredFidelity {
                card_id: card.card_id.clone(),
                published: card.published_fidelity.as_str(),
                declared: card.declared_resulting_fidelity.as_str(),
            });
        }

        let computed = card.computed_downgrade_reasons();
        if card.downgrade_reasons != computed {
            violations.push(M5RestoreProvenanceViolation::DowngradeReasonsMismatch {
                card_id: card.card_id.clone(),
            });
        }
        let computed_path = card.computed_recovery_path();
        if card.recovery_path != computed_path {
            violations.push(M5RestoreProvenanceViolation::RecoveryPathMismatch {
                card_id: card.card_id.clone(),
                declared: card.recovery_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // Provenance is never hidden: every card offers the read-only open-details action.
        if !card.has_action(ProvenanceActionKind::OpenDetails) {
            violations.push(M5RestoreProvenanceViolation::MissingOpenDetailsAction {
                card_id: card.card_id.clone(),
            });
        }
        // Compare and recovery-next-step are preserved wherever the fidelity was narrowed or a
        // dependency was missing.
        if card.requires_recovery_actions()
            && (!card.has_action(ProvenanceActionKind::Compare)
                || !card.has_action(ProvenanceActionKind::RecoveryNextStep))
        {
            violations.push(M5RestoreProvenanceViolation::MissingRecoveryActions {
                card_id: card.card_id.clone(),
            });
        }

        let mut seen_actions = BTreeSet::new();
        let mut seen_focus = BTreeSet::new();
        for affordance in &card.available_actions {
            if !seen_actions.insert(affordance.action) {
                violations.push(M5RestoreProvenanceViolation::DuplicateAction {
                    card_id: card.card_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !seen_focus.insert(affordance.focus_order) {
                violations.push(M5RestoreProvenanceViolation::DuplicateFocusOrder {
                    card_id: card.card_id.clone(),
                });
            }
            if !affordance.is_accessible() {
                violations.push(M5RestoreProvenanceViolation::InaccessibleAction {
                    card_id: card.card_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
            if !affordance.scoped_to_event {
                violations.push(M5RestoreProvenanceViolation::UnscopedAction {
                    card_id: card.card_id.clone(),
                    action: affordance.action.as_str(),
                });
            }
        }

        // A narrowed card must offer a real recovery next step, name a caveat, and name what was
        // narrowed.
        if achieved != RestoreFidelityClass::ExactRestore {
            if !card.recovery_path.is_offered() {
                violations.push(M5RestoreProvenanceViolation::MissingRecoveryPath {
                    card_id: card.card_id.clone(),
                });
            }
            if card.caveats.is_empty() {
                violations.push(M5RestoreProvenanceViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: "caveats",
                });
            }
            if card.narrowed_fields.is_empty() {
                violations.push(M5RestoreProvenanceViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: "narrowed_fields",
                });
            }
        }

        // An exact-restore card must be genuinely clean: pristine conditions, a non-contextual
        // source, no downgrade reason, and no recovery path. This is the guardrail against
        // presenting a downgraded or placeholder-heavy restore as if it were exact continuity.
        if achieved == RestoreFidelityClass::ExactRestore
            && (card.schema_condition != SchemaCondition::SchemaMatch
                || card.dependency_condition != DependencyCondition::DependenciesPresent
                || card.topology_condition != TopologyCondition::TopologyIdentical
                || card.evidence_freshness != EvidenceFreshness::Current
                || card.source.is_contextual_only()
                || !card.downgrade_reasons.is_empty()
                || !card.caveats.is_empty()
                || !card.narrowed_fields.is_empty()
                || card.recovery_path.is_offered())
        {
            violations.push(M5RestoreProvenanceViolation::ExactCardNotClean {
                card_id: card.card_id.clone(),
            });
        }
    }
}

/// A validation violation for [`M5RestoreProvenance`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum M5RestoreProvenanceViolation {
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
        /// Card or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A card id appears more than once.
    DuplicateCardId {
        /// Duplicate card id.
        card_id: String,
    },
    /// A re-entry surface carries more than one card.
    DuplicateSurfaceCard {
        /// Surface token.
        surface: &'static str,
    },
    /// A re-entry surface has no card.
    MissingSurfaceCard {
        /// Surface token.
        surface: &'static str,
    },
    /// A card publishes a fidelity beyond what the gate computes.
    OverstatedFidelity {
        /// Card id.
        card_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Computed fidelity token.
        computed: &'static str,
    },
    /// A card publishes a fidelity above what its source can imply (e.g. a handoff implying a full
    /// restore).
    SourceFidelityCeilingExceeded {
        /// Card id.
        card_id: String,
        /// Source token.
        source: &'static str,
        /// Published fidelity token.
        published: &'static str,
        /// Source ceiling token.
        ceiling: &'static str,
    },
    /// A card publishes a fidelity above its declared resulting fidelity.
    ExceedsDeclaredFidelity {
        /// Card id.
        card_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Declared resulting fidelity token.
        declared: &'static str,
    },
    /// A card's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Card id.
        card_id: String,
    },
    /// A card's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Card id.
        card_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A card would silently delete layout when a dependency is missing.
    SilentLayoutDelete {
        /// Card id.
        card_id: String,
    },
    /// A card does not guarantee every required redaction exclusion.
    MissingRedactionExclusion {
        /// Card id.
        card_id: String,
    },
    /// A card omits the read-only open-details action, hiding its provenance.
    MissingOpenDetailsAction {
        /// Card id.
        card_id: String,
    },
    /// A narrowed card omits the compare or recovery-next-step action.
    MissingRecoveryActions {
        /// Card id.
        card_id: String,
    },
    /// A narrowed card offers no recovery path.
    MissingRecoveryPath {
        /// Card id.
        card_id: String,
    },
    /// A card offers the same action twice.
    DuplicateAction {
        /// Card id.
        card_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// Two affordances in a card share a focus order, making keyboard navigation ambiguous.
    DuplicateFocusOrder {
        /// Card id.
        card_id: String,
    },
    /// An affordance lacks a command id, keyboard shortcut, or screen-reader label.
    InaccessibleAction {
        /// Card id.
        card_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// An affordance is not scoped to the one restore event.
    UnscopedAction {
        /// Card id.
        card_id: String,
        /// Offending action token.
        action: &'static str,
    },
    /// An exact-restore card still narrows a condition or carries a downgrade reason.
    ExactCardNotClean {
        /// Card id.
        card_id: String,
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
    /// The summary counts disagree with the cards.
    SummaryMismatch,
}

impl fmt::Display for M5RestoreProvenanceViolation {
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
            Self::DuplicateCardId { card_id } => write!(f, "duplicate card id {card_id}"),
            Self::DuplicateSurfaceCard { surface } => {
                write!(f, "duplicate card for re-entry surface {surface}")
            }
            Self::MissingSurfaceCard { surface } => {
                write!(f, "missing card for re-entry surface {surface}")
            }
            Self::OverstatedFidelity {
                card_id,
                published,
                computed,
            } => write!(
                f,
                "card {card_id} publishes fidelity {published} but the gate computes {computed}"
            ),
            Self::SourceFidelityCeilingExceeded {
                card_id,
                source,
                published,
                ceiling,
            } => write!(
                f,
                "card {card_id} from {source} publishes {published} above its source ceiling {ceiling}"
            ),
            Self::ExceedsDeclaredFidelity {
                card_id,
                published,
                declared,
            } => write!(
                f,
                "card {card_id} publishes fidelity {published} above declared {declared}"
            ),
            Self::DowngradeReasonsMismatch { card_id } => {
                write!(f, "card {card_id} downgrade reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch {
                card_id,
                declared,
                required,
            } => write!(
                f,
                "card {card_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::SilentLayoutDelete { card_id } => write!(
                f,
                "card {card_id} would silently delete layout on a missing dependency"
            ),
            Self::MissingRedactionExclusion { card_id } => write!(
                f,
                "card {card_id} does not guarantee its required redaction exclusions"
            ),
            Self::MissingOpenDetailsAction { card_id } => {
                write!(f, "card {card_id} omits the open-details action")
            }
            Self::MissingRecoveryActions { card_id } => write!(
                f,
                "card {card_id} is narrowed but omits the compare or recovery-next-step action"
            ),
            Self::MissingRecoveryPath { card_id } => {
                write!(f, "card {card_id} is narrowed but offers no recovery path")
            }
            Self::DuplicateAction { card_id, action } => {
                write!(f, "card {card_id} offers action {action} twice")
            }
            Self::DuplicateFocusOrder { card_id } => {
                write!(f, "card {card_id} has affordances sharing a focus order")
            }
            Self::InaccessibleAction { card_id, action } => write!(
                f,
                "card {card_id} action {action} lacks a command id, shortcut, or label"
            ),
            Self::UnscopedAction { card_id, action } => {
                write!(f, "card {card_id} action {action} is not scoped to the event")
            }
            Self::ExactCardNotClean { card_id } => write!(
                f,
                "card {card_id} restores exact but narrows a condition or carries a downgrade reason"
            ),
            Self::MissingConsumerBinding { surface } => {
                write!(f, "parity surface {surface} has no consumer binding")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "consumer binding {binding_ref} does not preserve the record")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the cards"),
        }
    }
}

impl Error for M5RestoreProvenanceViolation {}

/// Stable record-kind tag for [`M5RestoreProvenanceSupportExport`].
pub const M5_RESTORE_PROVENANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_restore_provenance_support_export";

/// Support-export wrapper preserving the restore-provenance record verbatim for support and
/// evidence packets, so an exported diagnostics or support bundle carries the same provenance and
/// fidelity record rather than a weaker summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestoreProvenanceSupportExport {
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
    pub record: M5RestoreProvenance,
}

impl M5RestoreProvenanceSupportExport {
    /// Whether the export preserves the same packet id and a clean record.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_RESTORE_PROVENANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_RESTORE_PROVENANCE_SCHEMA_VERSION
            && self.record_packet_id_ref == self.record.packet_id
            && self.raw_private_material_excluded
            && self.record.validate().is_empty()
    }
}

/// Loads the embedded M5 restore-provenance packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches [`M5RestoreProvenance`].
pub fn current_m5_restore_provenance() -> Result<M5RestoreProvenance, serde_json::Error> {
    serde_json::from_str(M5_RESTORE_PROVENANCE_JSON)
}

#[cfg(test)]
mod tests;
