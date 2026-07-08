//! One reusable M5 migration primitive — the migration bridge card — so an imported user can
//! see, from the card alone, exactly how one imported behavior maps onto Aureline: the old
//! path or shortcut it came from, the new command or surface it maps to, whether that mapping
//! is exact, native, a bridge, a shim, partial, or unsupported, what scope the import touched,
//! which edge cases are not covered, and how to review or undo the import — never letting an
//! approximated or partial behavior masquerade as exact parity.
//!
//! Aureline's frozen contextual-teaching / migration-bridge component matrix
//! ([`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`])
//! names the migration bridge card as one governed component family and freezes its controlled
//! vocabulary — the migration mapping classes (`exact`, `native`, `bridge`, `shimmed`,
//! `partial`, `unsupported`) and the imported source tool classes (`legacy_editor`,
//! `rival_ide`, `modal_editor`, `imported_keymap`, `migrated_workflow_config`,
//! `unknown_source`) — plus the surface families, the deployment lines, the consumer surfaces,
//! the accessibility routes, the qualification classes, and the downgrade triggers. This
//! module *implements* that contract as one reusable resolver so a user can tell — from the
//! bridge card alone — where an imported behavior came from, the exact mapping honesty class,
//! what scope the import changed, which edge cases stay unsupported, and whether they can open
//! the native command, review the import checkpoint, or undo the import, without ever masking
//! the mapping state, overstating an approximation as exact parity, dropping the affected
//! scope, or leaving a durable import change with no way to review or roll it back.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_migration_bridge_card`] — takes one imported behavior's mapping class, source
//!    tool, opaque old-path reference, optional opaque new-command reference, affected-scope
//!    summary, unsupported edge cases, whether the import created a durable user-facing change,
//!    the optional opaque import rollback / checkpoint reference, and the opaque stable bridge
//!    identity, and produces one [`M5ResolvedMigrationBridgeCard`] carrying the derived bridge
//!    posture (exact-parity, native-equivalent, bridged-approximation, shimmed-compatibility,
//!    partial-coverage, or unsupported-no-mapping), the bounded view-mapping-details /
//!    open-native-command / undo-import-changes / review-import-checkpoint /
//!    report-unsupported-edge-case actions, and whether the mapping is faithful, approximated,
//!    incomplete, or unsupported. It never masks the mapping state, never overstates an
//!    approximated or partial mapping as exact parity, always preserves the affected scope and
//!    the unsupported edge cases, always preserves the import rollback linkage, and never
//!    leaves a durable import change without an available undo / review action.
//!
//! A single parity matrix — [`M5MigrationBridgeCardPacket`] — binds one row per claimed M5
//! importer / migration consumer (the migration report panel, the import diff row, the
//! first-run switch summary, the keybinding migration notice, and the support migration
//! export) to the shared bridge-card anatomy, the same migration mapping classes, source
//! tools, bridge postures, bounded actions, export fields, and non-visual accessibility
//! routes, so the old-path / new-command / mapping-state / undo-import vocabulary stays
//! identical across desktop, headless/export, and support consumers.
//!
//! The migration mapping class ([`M5MigrationMappingClass`]), imported source tool
//! ([`M5SourceToolClass`]), teaching surface family ([`M5TeachingSurfaceFamily`]), deployment
//! line ([`M5TeachingDeploymentLine`]), teaching consumer surface
//! ([`M5TeachingConsumerSurface`]), accessibility route ([`M5TeachingAccessibilityRoute`]),
//! qualification class ([`M5TeachingQualificationClass`]), and downgrade trigger
//! ([`M5TeachingDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the bridge card itself:
//! its importer / migration consumers, its anatomy parts, its derived bridge posture, its
//! bounded actions, and its export fields. No M5 migration surface invents a second
//! bridge-card grammar.
//!
//! Raw imported config bodies, pasted paths, credentials, and private endpoints stay outside
//! the export boundary; every old-path reference, command reference, rollback reference, and
//! bridge identity is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_migration_bridge_card_keybinding_migration_notice_beta_narrowed,
    seeded_m5_migration_bridge_card_packet,
    seeded_m5_migration_bridge_card_support_migration_export_preview_narrowed,
    M5_MIGRATION_BRIDGE_CARD_PACKET_ID,
};

// The migration mapping class, source tool class, surface family, deployment line, consumer
// surface, accessibility route, qualification class, and downgrade triggers are frozen once,
// in the contextual-teaching / migration-bridge component matrix. This primitive reuses them
// verbatim so it never invents a parallel migration vocabulary.
pub use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5MigrationMappingClass, M5SourceToolClass, M5TeachingAccessibilityRoute,
    M5TeachingConsumerSurface, M5TeachingDeploymentLine, M5TeachingDowngradeTrigger,
    M5TeachingQualificationClass, M5TeachingSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5MigrationBridgeCardPacket`].
pub const M5_MIGRATION_BRIDGE_CARD_RECORD_KIND: &str =
    "implement_m5_migration_bridge_cards_with_old_path_new_command_mapping_native_bridge_shimmed_partial_states_and_undo_import_parity_across_claimed_m5_importer_and_migration_surfaces";

/// Schema version for M5 migration-bridge-card records.
pub const M5_MIGRATION_BRIDGE_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the migration-bridge-card boundary schema.
pub const M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-migration-bridge-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MIGRATION_BRIDGE_CARD_DOC_REF: &str =
    "docs/migration/m5_migration_bridge_card_primitive.md";

/// Repo-relative path of the frozen contextual-teaching / migration-bridge component matrix
/// this primitive narrows from.
pub const M5_MIGRATION_BRIDGE_CARD_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json";

/// Repo-relative path of the importer-outcome contract the bridge card's mapping state binds
/// against.
pub const M5_MIGRATION_BRIDGE_CARD_IMPORTER_OUTCOME_REF: &str =
    "schemas/migration/importer_outcome.schema.json";

/// Repo-relative path of the import-rollback-checkpoint contract the bridge card's undo /
/// review action binds against.
pub const M5_MIGRATION_BRIDGE_CARD_ROLLBACK_CHECKPOINT_REF: &str =
    "schemas/migration/import_rollback_checkpoint.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MIGRATION_BRIDGE_CARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-migration-bridge-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MIGRATION_BRIDGE_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-migration-bridge-card-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_MIGRATION_BRIDGE_CARD_CSV_REF: &str =
    "artifacts/release/m5-migration-bridge-card-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MIGRATION_BRIDGE_CARD_REPORT_REF: &str =
    "artifacts/design/m5-migration-bridge-card-primitive.md";

/// One claimed M5 importer / migration consumer that renders the shared migration bridge card.
/// These are the consumers the acceptance criteria name — the migration report panel, the
/// import diff row, the first-run switch summary, the keybinding migration notice, and the
/// support migration export — so the same bridge-card grammar works across every claimed
/// importer / migration surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationBridgeConsumerSurface {
    /// The migration report panel surface.
    MigrationReportPanel,
    /// The import diff row surface.
    ImportDiffRow,
    /// The first-run switch summary surface.
    FirstRunSwitchSummary,
    /// The keybinding migration notice surface.
    KeybindingMigrationNotice,
    /// The support migration-export surface.
    SupportMigrationExport,
}

impl M5MigrationBridgeConsumerSurface {
    /// Every claimed importer / migration consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MigrationReportPanel,
        Self::ImportDiffRow,
        Self::FirstRunSwitchSummary,
        Self::KeybindingMigrationNotice,
        Self::SupportMigrationExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrationReportPanel => "migration_report_panel",
            Self::ImportDiffRow => "import_diff_row",
            Self::FirstRunSwitchSummary => "first_run_switch_summary",
            Self::KeybindingMigrationNotice => "keybinding_migration_notice",
            Self::SupportMigrationExport => "support_migration_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MigrationReportPanel => "Migration Report Panel",
            Self::ImportDiffRow => "Import Diff Row",
            Self::FirstRunSwitchSummary => "First-Run Switch Summary",
            Self::KeybindingMigrationNotice => "Keybinding Migration Notice",
            Self::SupportMigrationExport => "Support Migration Export",
        }
    }
}

/// The derived bridge posture of a migration bridge card — the resolver's honest verdict about
/// how faithfully an imported behavior maps onto Aureline. Derived one-to-one from the frozen
/// migration mapping class so an approximated, shimmed, partial, or unsupported behavior can
/// never be presented as exact parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationBridgePosture {
    /// An exact one-to-one mapping — the imported behavior is reproduced faithfully.
    ExactParity,
    /// A native Aureline equivalent — the same outcome via a native path.
    NativeEquivalent,
    /// A bridge that approximates the imported behavior — close, not identical.
    BridgedApproximation,
    /// A shimmed compatibility behavior — supported through a compatibility shim.
    ShimmedCompatibility,
    /// A partial mapping — some of the imported behavior is missing.
    PartialCoverage,
    /// An unsupported behavior — no mapping exists.
    UnsupportedNoMapping,
}

impl M5MigrationBridgePosture {
    /// Every bridge posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactParity,
        Self::NativeEquivalent,
        Self::BridgedApproximation,
        Self::ShimmedCompatibility,
        Self::PartialCoverage,
        Self::UnsupportedNoMapping,
    ];

    /// The bridge posture that honestly reflects a migration mapping class — one-to-one, never
    /// upgrading an approximation into exact parity.
    pub const fn from_mapping(mapping: M5MigrationMappingClass) -> Self {
        match mapping {
            M5MigrationMappingClass::Exact => Self::ExactParity,
            M5MigrationMappingClass::Native => Self::NativeEquivalent,
            M5MigrationMappingClass::Bridge => Self::BridgedApproximation,
            M5MigrationMappingClass::Shimmed => Self::ShimmedCompatibility,
            M5MigrationMappingClass::Partial => Self::PartialCoverage,
            M5MigrationMappingClass::Unsupported => Self::UnsupportedNoMapping,
        }
    }

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactParity => "exact_parity",
            Self::NativeEquivalent => "native_equivalent",
            Self::BridgedApproximation => "bridged_approximation",
            Self::ShimmedCompatibility => "shimmed_compatibility",
            Self::PartialCoverage => "partial_coverage",
            Self::UnsupportedNoMapping => "unsupported_no_mapping",
        }
    }

    /// True only for the exact-parity posture — the one posture that may claim identical
    /// behavior. Nothing else ever does.
    pub const fn claims_exact_parity(self) -> bool {
        matches!(self, Self::ExactParity)
    }

    /// True when the mapping reproduces the full imported behavior (exact or via a native
    /// equivalent).
    pub const fn is_faithful(self) -> bool {
        matches!(self, Self::ExactParity | Self::NativeEquivalent)
    }

    /// True when the mapping only approximates the imported behavior (a bridge or a shim).
    pub const fn is_approximated(self) -> bool {
        matches!(
            self,
            Self::BridgedApproximation | Self::ShimmedCompatibility
        )
    }

    /// True when the mapping covers only part of the imported behavior.
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::PartialCoverage)
    }

    /// True when the imported behavior has no mapping at all.
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::UnsupportedNoMapping)
    }
}

/// One bounded action a migration bridge card offers, so an imported user can always inspect
/// the mapping in detail, jump to the native command, review the import checkpoint, undo a
/// durable import change, or report an unsupported edge case — never trapped by an
/// approximated behavior they cannot inspect or reverse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationBridgeAction {
    /// View the full mapping details for this imported behavior.
    ViewMappingDetails,
    /// Open the native command or surface this behavior maps to.
    OpenNativeCommand,
    /// Undo the durable changes this import made.
    UndoImportChanges,
    /// Review the import rollback / checkpoint this import created.
    ReviewImportCheckpoint,
    /// Report an unsupported edge case for this imported behavior.
    ReportUnsupportedEdgeCase,
}

impl M5MigrationBridgeAction {
    /// Every bridge action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ViewMappingDetails,
        Self::OpenNativeCommand,
        Self::UndoImportChanges,
        Self::ReviewImportCheckpoint,
        Self::ReportUnsupportedEdgeCase,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewMappingDetails => "view_mapping_details",
            Self::OpenNativeCommand => "open_native_command",
            Self::UndoImportChanges => "undo_import_changes",
            Self::ReviewImportCheckpoint => "review_import_checkpoint",
            Self::ReportUnsupportedEdgeCase => "report_unsupported_edge_case",
        }
    }
}

/// Controlled migration-bridge-card anatomy part the shared card surfaces. The parts in
/// [`M5MigrationBridgeAnatomyPart::MANDATORY`] are required on every card so the old path, the
/// new command, the mapping state, the affected scope, and the suggested actions are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationBridgeAnatomyPart {
    /// The old path / shortcut cue.
    OldPathCue,
    /// The new command / surface cue.
    NewCommandCue,
    /// The mapping-state cue.
    MappingStateCue,
    /// The affected-scope cue.
    AffectedScopeCue,
    /// The suggested-actions cue.
    SuggestedActionsCue,
    /// The imported source-tool cue.
    SourceToolCue,
    /// The unsupported-edge-cases cue.
    UnsupportedEdgeCasesCue,
    /// The import rollback / checkpoint linkage cue.
    RollbackLinkageCue,
}

impl M5MigrationBridgeAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::OldPathCue,
        Self::NewCommandCue,
        Self::MappingStateCue,
        Self::AffectedScopeCue,
        Self::SuggestedActionsCue,
        Self::SourceToolCue,
        Self::UnsupportedEdgeCasesCue,
        Self::RollbackLinkageCue,
    ];

    /// The anatomy parts every card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::OldPathCue,
        Self::NewCommandCue,
        Self::MappingStateCue,
        Self::AffectedScopeCue,
        Self::SuggestedActionsCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OldPathCue => "old_path_cue",
            Self::NewCommandCue => "new_command_cue",
            Self::MappingStateCue => "mapping_state_cue",
            Self::AffectedScopeCue => "affected_scope_cue",
            Self::SuggestedActionsCue => "suggested_actions_cue",
            Self::SourceToolCue => "source_tool_cue",
            Self::UnsupportedEdgeCasesCue => "unsupported_edge_cases_cue",
            Self::RollbackLinkageCue => "rollback_linkage_cue",
        }
    }
}

/// A field the card export carries so migration-bridge-card truth is reconstructable. The
/// fields in [`M5MigrationBridgeExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationBridgeExportField {
    /// The migration mapping class.
    MappingClass,
    /// The imported source tool.
    SourceTool,
    /// The old-path reference.
    OldPathRef,
    /// The new-command reference.
    NewCommandRef,
    /// The suggested actions.
    SuggestedActions,
    /// The affected scope.
    AffectedScope,
    /// The unsupported edge cases.
    UnsupportedEdgeCases,
    /// The import rollback / checkpoint reference.
    RollbackCheckpointRef,
}

impl M5MigrationBridgeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::MappingClass,
        Self::SourceTool,
        Self::OldPathRef,
        Self::NewCommandRef,
        Self::SuggestedActions,
        Self::AffectedScope,
        Self::UnsupportedEdgeCases,
        Self::RollbackCheckpointRef,
    ];

    /// The export fields every card must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::MappingClass,
        Self::SourceTool,
        Self::OldPathRef,
        Self::NewCommandRef,
        Self::SuggestedActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MappingClass => "mapping_class",
            Self::SourceTool => "source_tool",
            Self::OldPathRef => "old_path_ref",
            Self::NewCommandRef => "new_command_ref",
            Self::SuggestedActions => "suggested_actions",
            Self::AffectedScope => "affected_scope",
            Self::UnsupportedEdgeCases => "unsupported_edge_cases",
            Self::RollbackCheckpointRef => "rollback_checkpoint_ref",
        }
    }
}

// ---- migration-bridge-card resolver -------------------------------------

/// The full input to the migration-bridge-card resolver for one imported behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardResolutionInput {
    /// How the imported behavior maps onto Aureline.
    pub mapping_class: M5MigrationMappingClass,
    /// Where the imported behavior came from.
    pub source_tool: M5SourceToolClass,
    /// The opaque old path or shortcut the behavior came from (must be non-empty).
    pub old_path_ref: String,
    /// The opaque new command or surface the behavior maps to. `None` only for an unsupported
    /// behavior with no mapping; `Some(non-empty)` for every mapped behavior.
    pub new_command_ref: Option<String>,
    /// The opaque affected-scope summary — what the import touched (must be non-empty).
    pub affected_scope: String,
    /// The unsupported edge cases this mapping does not cover. Required to be non-empty for a
    /// partial or unsupported mapping.
    pub unsupported_edge_cases: Vec<String>,
    /// True when the import created a durable user-facing change (settings, keybindings,
    /// snippets, or other durable behavior), so an undo / review path must exist.
    pub import_created_durable_change: bool,
    /// The opaque import rollback / checkpoint reference this import created. Required whenever
    /// the import made a durable change.
    pub rollback_checkpoint_ref: Option<String>,
    /// The opaque stable bridge identity (must be non-empty).
    pub bridge_identity_ref: String,
}

/// The resolved migration-bridge-card truth for one imported behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMigrationBridgeCard {
    /// The migration mapping class.
    pub mapping_class: M5MigrationMappingClass,
    /// The imported source tool.
    pub source_tool: M5SourceToolClass,
    /// The opaque old-path reference, preserved exactly from the input.
    pub old_path_ref: String,
    /// The opaque new-command reference, preserved exactly from the input.
    pub new_command_ref: Option<String>,
    /// The opaque affected-scope summary, preserved exactly from the input.
    pub affected_scope: String,
    /// The unsupported edge cases, preserved exactly from the input.
    pub unsupported_edge_cases: Vec<String>,
    /// The opaque import rollback / checkpoint reference, preserved exactly from the input.
    pub rollback_checkpoint_ref: Option<String>,
    /// The opaque stable bridge identity, preserved exactly from the input.
    pub bridge_identity_ref: String,
    /// The derived bridge posture.
    pub bridge_posture: M5MigrationBridgePosture,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5MigrationBridgeAction>,
    /// True only when the card claims exact parity (mapping class `exact`).
    pub claims_exact_parity: bool,
    /// True when the mapping reproduces the full imported behavior.
    pub is_faithful_mapping: bool,
    /// True when the mapping only approximates the imported behavior.
    pub is_approximated_mapping: bool,
    /// True when the mapping covers only part of the imported behavior.
    pub is_incomplete_mapping: bool,
    /// True when the imported behavior has no mapping at all.
    pub is_unsupported_mapping: bool,
    /// True when the import created a durable user-facing change, preserved from the input.
    pub import_created_durable_change: bool,
    /// True when the card carries an import rollback / checkpoint linkage.
    pub has_rollback_checkpoint: bool,
    /// True when the card offers an open-native-command action.
    pub open_native_command_available: bool,
    /// True when the card offers an undo-import-changes action.
    pub undo_available: bool,
    /// The card always discloses both the old path and the new command. ALWAYS `true`.
    pub discloses_old_path_and_new_command: bool,
    /// The card always discloses its mapping state honestly. ALWAYS `true`.
    pub discloses_mapping_state_honestly: bool,
    /// The card never overstates an approximated or partial mapping as exact parity. ALWAYS
    /// `true`.
    pub never_overstates_as_exact_parity: bool,
    /// The card always preserves the affected scope. ALWAYS `true`.
    pub preserves_affected_scope: bool,
    /// The card always preserves the unsupported edge cases. ALWAYS `true`.
    pub preserves_unsupported_edge_cases: bool,
    /// The card always preserves the import rollback linkage. ALWAYS `true`.
    pub preserves_import_rollback_linkage: bool,
    /// The card always keeps an undo / review action available wherever the import made a
    /// durable change. ALWAYS `true`.
    pub keeps_undo_review_available_for_durable_changes: bool,
}

/// Errors returned by [`resolve_migration_bridge_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MigrationBridgeCardResolutionError {
    /// The old-path reference was empty.
    EmptyOldPath,
    /// The affected-scope summary was empty.
    EmptyAffectedScope,
    /// The bridge identity ref was empty.
    EmptyBridgeIdentity,
    /// A mapped behavior (not unsupported) declared no new-command reference.
    MissingNewCommandForMappedState,
    /// An unsupported behavior wrongly declared a new-command reference.
    NativeCommandOnUnsupportedState,
    /// A partial or unsupported mapping named no unsupported edge cases.
    MissingUnsupportedEdgeCases,
    /// A durable import change declared no rollback / checkpoint reference.
    DurableChangeWithoutRollback,
    /// A card descriptor carried forbidden material.
    ForbiddenBridgeMaterial,
}

impl M5MigrationBridgeCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyOldPath => "empty_old_path",
            Self::EmptyAffectedScope => "empty_affected_scope",
            Self::EmptyBridgeIdentity => "empty_bridge_identity",
            Self::MissingNewCommandForMappedState => "missing_new_command_for_mapped_state",
            Self::NativeCommandOnUnsupportedState => "native_command_on_unsupported_state",
            Self::MissingUnsupportedEdgeCases => "missing_unsupported_edge_cases",
            Self::DurableChangeWithoutRollback => "durable_change_without_rollback",
            Self::ForbiddenBridgeMaterial => "forbidden_bridge_material",
        }
    }
}

impl fmt::Display for M5MigrationBridgeCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "migration bridge card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MigrationBridgeCardResolutionError {}

/// Resolves one migration bridge card from its declared mapping class, source tool, imported
/// references, affected scope, unsupported edge cases, and import rollback linkage.
///
/// The bridge posture is derived one-to-one from the frozen migration mapping class so an
/// approximated (`bridge` / `shimmed`), partial, or unsupported behavior can never be
/// presented as exact parity. The action set always offers view-mapping-details so the mapping
/// can be inspected; it offers open-native-command whenever a native command exists,
/// undo-import-changes whenever the import made a durable change (always backed by a rollback
/// checkpoint), review-import-checkpoint whenever the import created a checkpoint, and
/// report-unsupported-edge-case whenever the mapping is unsupported or names an uncovered edge
/// case. A durable import change with no rollback linkage is rejected outright, so undo /
/// review always stays available wherever an import changed durable user-facing behavior.
pub fn resolve_migration_bridge_card(
    input: &M5MigrationBridgeCardResolutionInput,
) -> Result<M5ResolvedMigrationBridgeCard, M5MigrationBridgeCardResolutionError> {
    if input.old_path_ref.trim().is_empty() {
        return Err(M5MigrationBridgeCardResolutionError::EmptyOldPath);
    }
    if input.affected_scope.trim().is_empty() {
        return Err(M5MigrationBridgeCardResolutionError::EmptyAffectedScope);
    }
    if input.bridge_identity_ref.trim().is_empty() {
        return Err(M5MigrationBridgeCardResolutionError::EmptyBridgeIdentity);
    }
    if bridge_input_has_forbidden_material(input) {
        return Err(M5MigrationBridgeCardResolutionError::ForbiddenBridgeMaterial);
    }

    let is_unsupported = matches!(input.mapping_class, M5MigrationMappingClass::Unsupported);
    let has_new_command = input
        .new_command_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());
    if is_unsupported {
        if has_new_command {
            return Err(M5MigrationBridgeCardResolutionError::NativeCommandOnUnsupportedState);
        }
    } else if !has_new_command {
        return Err(M5MigrationBridgeCardResolutionError::MissingNewCommandForMappedState);
    }

    if matches!(
        input.mapping_class,
        M5MigrationMappingClass::Partial | M5MigrationMappingClass::Unsupported
    ) && input.unsupported_edge_cases.is_empty()
    {
        return Err(M5MigrationBridgeCardResolutionError::MissingUnsupportedEdgeCases);
    }

    let has_rollback_checkpoint = input
        .rollback_checkpoint_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());
    if input.import_created_durable_change && !has_rollback_checkpoint {
        return Err(M5MigrationBridgeCardResolutionError::DurableChangeWithoutRollback);
    }

    let bridge_posture = M5MigrationBridgePosture::from_mapping(input.mapping_class);
    let available_actions = derive_bridge_actions(
        has_new_command,
        input.import_created_durable_change,
        has_rollback_checkpoint,
        is_unsupported,
        &input.unsupported_edge_cases,
    );
    let undo_available = available_actions.contains(&M5MigrationBridgeAction::UndoImportChanges);

    Ok(M5ResolvedMigrationBridgeCard {
        mapping_class: input.mapping_class,
        source_tool: input.source_tool,
        old_path_ref: input.old_path_ref.clone(),
        new_command_ref: input.new_command_ref.clone(),
        affected_scope: input.affected_scope.clone(),
        unsupported_edge_cases: input.unsupported_edge_cases.clone(),
        rollback_checkpoint_ref: input.rollback_checkpoint_ref.clone(),
        bridge_identity_ref: input.bridge_identity_ref.clone(),
        bridge_posture,
        available_actions,
        claims_exact_parity: bridge_posture.claims_exact_parity(),
        is_faithful_mapping: bridge_posture.is_faithful(),
        is_approximated_mapping: bridge_posture.is_approximated(),
        is_incomplete_mapping: bridge_posture.is_incomplete(),
        is_unsupported_mapping: bridge_posture.is_unsupported(),
        import_created_durable_change: input.import_created_durable_change,
        has_rollback_checkpoint,
        open_native_command_available: has_new_command,
        undo_available,
        // The acceptance criteria: bridge cards always disclose the old path and new command,
        // never let an approximated or partial mapping masquerade as exact parity, always
        // preserve the affected scope, the unsupported edge cases, and the import rollback
        // linkage, and always keep an undo / review action available wherever the import made a
        // durable change.
        discloses_old_path_and_new_command: true,
        discloses_mapping_state_honestly: true,
        never_overstates_as_exact_parity: true,
        preserves_affected_scope: true,
        preserves_unsupported_edge_cases: true,
        preserves_import_rollback_linkage: true,
        keeps_undo_review_available_for_durable_changes: true,
    })
}

/// Derives the bounded action set from whether a native command exists, whether the import
/// made a durable change, whether a rollback checkpoint exists, whether the mapping is
/// unsupported, and whether any unsupported edge cases were named.
///
/// Every card offers view-mapping-details so the mapping can always be inspected. A mapped
/// behavior additionally offers open-native-command; a durable import change offers
/// undo-import-changes (always backed by a rollback checkpoint); an import that created a
/// checkpoint offers review-import-checkpoint; and an unsupported mapping or any named edge
/// case offers report-unsupported-edge-case.
fn derive_bridge_actions(
    has_new_command: bool,
    import_created_durable_change: bool,
    has_rollback_checkpoint: bool,
    is_unsupported: bool,
    unsupported_edge_cases: &[String],
) -> Vec<M5MigrationBridgeAction> {
    use M5MigrationBridgeAction as Action;

    let mut actions = vec![Action::ViewMappingDetails];
    if has_new_command {
        actions.push(Action::OpenNativeCommand);
    }
    if import_created_durable_change {
        actions.push(Action::UndoImportChanges);
    }
    if has_rollback_checkpoint {
        actions.push(Action::ReviewImportCheckpoint);
    }
    if is_unsupported || !unsupported_edge_cases.is_empty() {
        actions.push(Action::ReportUnsupportedEdgeCase);
    }
    actions
}

/// True when any opaque descriptor on the input carries obviously forbidden material.
fn bridge_input_has_forbidden_material(input: &M5MigrationBridgeCardResolutionInput) -> bool {
    if value_repr_is_forbidden(&input.old_path_ref)
        || value_repr_is_forbidden(&input.affected_scope)
        || value_repr_is_forbidden(&input.bridge_identity_ref)
    {
        return true;
    }
    if let Some(command) = &input.new_command_ref {
        if value_repr_is_forbidden(command) {
            return true;
        }
    }
    if let Some(rollback) = &input.rollback_checkpoint_ref {
        if value_repr_is_forbidden(rollback) {
            return true;
        }
    }
    input
        .unsupported_edge_cases
        .iter()
        .any(|edge| value_repr_is_forbidden(edge))
}

// ---- worked cases -------------------------------------------------------

/// One worked migration-bridge-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardResolutionCase {
    /// The resolver input.
    pub input: M5MigrationBridgeCardResolutionInput,
    /// The resolved truth. Must equal `resolve_migration_bridge_card(&input)`.
    pub resolved: M5ResolvedMigrationBridgeCard,
}

impl M5MigrationBridgeCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5MigrationBridgeCardResolutionInput) -> Self {
        let resolved = resolve_migration_bridge_card(&input)
            .expect("seed migration bridge card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_migration_bridge_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input bridge identity, old path, new command,
    /// affected scope, unsupported edge cases, and rollback linkage exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.bridge_identity_ref == self.input.bridge_identity_ref
            && self.resolved.old_path_ref == self.input.old_path_ref
            && self.resolved.new_command_ref == self.input.new_command_ref
            && self.resolved.affected_scope == self.input.affected_scope
            && self.resolved.unsupported_edge_cases == self.input.unsupported_edge_cases
            && self.resolved.rollback_checkpoint_ref == self.input.rollback_checkpoint_ref
    }

    /// True when the resolved case discloses the old path / new command / mapping state, never
    /// overstates as exact parity, preserves the affected scope / edge cases / rollback
    /// linkage, and keeps undo / review available for durable changes.
    pub fn preserves_reversibility(&self) -> bool {
        self.resolved.discloses_old_path_and_new_command
            && self.resolved.discloses_mapping_state_honestly
            && self.resolved.never_overstates_as_exact_parity
            && self.resolved.preserves_affected_scope
            && self.resolved.preserves_unsupported_edge_cases
            && self.resolved.preserves_import_rollback_linkage
            && self.resolved.keeps_undo_review_available_for_durable_changes
            // The concrete AC2 guarantee: a durable import change always has an undo action.
            && (!self.resolved.import_created_durable_change || self.resolved.undo_available)
            // The concrete AC1 guarantee: only an exact mapping ever claims exact parity.
            && (!self.resolved.claims_exact_parity
                || self.resolved.mapping_class == M5MigrationMappingClass::Exact)
    }
}

/// One row in the primitive matrix: one importer / migration consumer bound to the shared
/// bridge-card anatomy, migration mapping classes, source tools, bridge postures, bounded
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeConsumerRow {
    /// Importer / migration consumer family.
    pub consumer_surface: M5MigrationBridgeConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TeachingQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 importer / migration surface families that render / consume this card.
    pub surface_families: Vec<M5TeachingSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5TeachingDeploymentLine>,
    /// Anatomy parts this card renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5MigrationBridgeAnatomyPart>,
    /// Migration mapping classes this consumer distinguishes.
    pub mapping_classes: Vec<M5MigrationMappingClass>,
    /// Imported source tools this consumer distinguishes.
    pub source_tools: Vec<M5SourceToolClass>,
    /// Bridge postures this consumer distinguishes.
    pub bridge_postures: Vec<M5MigrationBridgePosture>,
    /// Bounded bridge actions this consumer offers.
    pub bridge_actions: Vec<M5MigrationBridgeAction>,
    /// Export fields this card carries (must include the mandatory fields).
    pub export_fields: Vec<M5MigrationBridgeExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TeachingAccessibilityRoute>,
    /// Teaching subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TeachingDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked bridge-card resolutions proving the resolver on this consumer.
    pub bridge_examples: Vec<M5MigrationBridgeCardResolutionCase>,
    /// Hard invariant: this consumer never masks its migration mapping state. MUST be `false`.
    pub masks_mapping_state: bool,
    /// Hard invariant: this consumer never overstates an approximated or partial mapping as
    /// exact parity. MUST be `false`.
    pub overstates_as_exact_parity: bool,
    /// Hard invariant: this consumer never drops the affected scope or the unsupported edge
    /// cases. MUST be `false`.
    pub drops_affected_scope_or_edge_cases: bool,
    /// Hard invariant: this consumer never severs the import rollback / undo linkage. MUST be
    /// `false`.
    pub severs_import_rollback_linkage: bool,
}

impl M5MigrationBridgeConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5MigrationBridgeAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5MigrationBridgeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5MigrationBridgeExportField> =
            self.export_fields.iter().copied().collect();
        M5MigrationBridgeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_mapping_state
            && !self.overstates_as_exact_parity
            && !self.drops_affected_scope_or_edge_cases
            && !self.severs_import_rollback_linkage
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardVocabularySet {
    /// Importer / migration consumer tokens.
    pub bridge_consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Bridge-posture tokens.
    pub bridge_postures: Vec<String>,
    /// Bridge-action tokens.
    pub bridge_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Migration-mapping-class tokens (reused from the frozen matrix).
    pub mapping_classes: Vec<String>,
    /// Source-tool-class tokens (reused from the frozen matrix).
    pub source_tools: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Teaching-consumer-surface tokens (reused from the frozen matrix).
    pub teaching_consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5MigrationBridgeCardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            bridge_consumer_surfaces: tokens(&M5MigrationBridgeConsumerSurface::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5MigrationBridgeAnatomyPart::ALL, |v| v.as_str()),
            bridge_postures: tokens(&M5MigrationBridgePosture::ALL, |v| v.as_str()),
            bridge_actions: tokens(&M5MigrationBridgeAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5MigrationBridgeExportField::ALL, |v| v.as_str()),
            mapping_classes: tokens(&M5MigrationMappingClass::ALL, |v| v.as_str()),
            source_tools: tokens(&M5SourceToolClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TeachingSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TeachingDeploymentLine::ALL, |v| v.as_str()),
            teaching_consumer_surfaces: tokens(&M5TeachingConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TeachingAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardGovernanceReview {
    /// The bridge card shows the old path or shortcut it came from.
    pub bridge_card_shows_old_path: bool,
    /// The bridge card shows the new command or surface it maps to.
    pub bridge_card_shows_new_command: bool,
    /// The bridge card shows its migration mapping state.
    pub bridge_card_shows_mapping_state: bool,
    /// The bridge card shows the affected scope of the import.
    pub bridge_card_shows_affected_scope: bool,
    /// The bridge card shows any unsupported edge cases.
    pub bridge_card_shows_unsupported_edge_cases: bool,
    /// Imported users never mistake an approximated or partial mapping for exact parity.
    pub imported_users_never_mistake_partial_for_exact: bool,
    /// The bridge card never masks its migration mapping state.
    pub bridge_card_never_masks_mapping_state: bool,
    /// Undo / review actions stay available wherever an import changed durable behavior.
    pub undo_review_available_where_import_changed_durable_behavior: bool,
    /// The bridge card preserves the import rollback / checkpoint linkage.
    pub bridge_card_preserves_import_rollback_linkage: bool,
    /// The bridge card names the imported source tool.
    pub bridge_card_names_imported_source_tool: bool,
    /// Users understand imported behavior without detached docs or tribal knowledge.
    pub users_understand_imported_behavior_without_detached_docs: bool,
    /// Bridge cards keep the same truth across every deployment line.
    pub bridge_cards_stable_across_deployment_lines: bool,
    /// Bridge cards keep the same truth across desktop, headless/export, and support consumers.
    pub bridge_cards_stable_across_consumer_surfaces: bool,
    /// Every bridge card declares a non-visual accessibility route.
    pub every_bridge_card_declares_accessibility_route: bool,
    /// The support / export packet reconstructs bridge-card truth.
    pub support_export_reconstructs_bridge_truth: bool,
    /// Later M5 rows cannot invent parallel bridge-card vocabulary.
    pub later_rows_cannot_invent_parallel_bridge_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardConsumerProjection {
    /// Importer / migration surfaces consume the shared bridge-card vocabulary.
    pub migration_surfaces_consume_bridge_vocabulary: bool,
    /// The bridge-posture resolver reads a single canonical source.
    pub bridge_posture_reads_single_source: bool,
    /// The action-set derivation reads a single canonical source.
    pub action_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop bridge cards read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the migration bridge card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting migration-bridge-card audit.
    pub migration_bridge_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MigrationBridgeCardPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MigrationBridgeCardPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Importer / migration consumer rows.
    pub rows: Vec<M5MigrationBridgeConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MigrationBridgeCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MigrationBridgeCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MigrationBridgeCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MigrationBridgeCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MigrationBridgeCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 migration-bridge-card primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationBridgeCardPacket {
    /// Record kind; must equal [`M5_MIGRATION_BRIDGE_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MIGRATION_BRIDGE_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Importer / migration consumer rows.
    pub rows: Vec<M5MigrationBridgeConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MigrationBridgeCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MigrationBridgeCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MigrationBridgeCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MigrationBridgeCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MigrationBridgeCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MigrationBridgeCardPacket {
    /// Builds an M5 migration-bridge-card-primitive packet from stable-lane input.
    pub fn new(input: M5MigrationBridgeCardPacketInput) -> Self {
        Self {
            record_kind: M5_MIGRATION_BRIDGE_CARD_RECORD_KIND.to_owned(),
            schema_version: M5_MIGRATION_BRIDGE_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 migration-bridge-card-primitive invariants.
    pub fn validate(&self) -> Vec<M5MigrationBridgeCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MIGRATION_BRIDGE_CARD_RECORD_KIND {
            violations.push(M5MigrationBridgeCardViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MIGRATION_BRIDGE_CARD_SCHEMA_VERSION {
            violations.push(M5MigrationBridgeCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MigrationBridgeCardViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_mapping_class_coverage(self, &mut violations);
        validate_posture_coverage(self, &mut violations);
        validate_action_coverage(self, &mut violations);
        validate_undo_parity_coverage(self, &mut violations);
        validate_reversibility(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 migration bridge card primitive packet serializes"),
        ) {
            violations.push(M5MigrationBridgeCardViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 migration bridge card primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per importer / migration consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,mapping_classes,source_tools,bridge_postures,bridge_actions,bridge_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.mapping_classes, |v| v.as_str()),
                join_tokens(&row.source_tools, |v| v.as_str()),
                join_tokens(&row.bridge_postures, |v| v.as_str()),
                join_tokens(&row.bridge_actions, |v| v.as_str()),
                row.bridge_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Migration-Bridge-Card Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Importer / migration consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Bridge postures: {}\n",
            self.vocabulary_set.bridge_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Bridge actions: {}\n",
            self.vocabulary_set.bridge_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Mapping classes: {}\n",
            self.vocabulary_set.mapping_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Importer / migration consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked bridges: {}\n",
                row.bridge_examples.len()
            ));
            for case in &row.bridge_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (exact-parity `{}`, durable `{}`, undo `{}`)\n",
                    case.resolved.bridge_identity_ref,
                    case.resolved.mapping_class.as_str(),
                    case.resolved.source_tool.as_str(),
                    case.resolved.bridge_posture.as_str(),
                    case.resolved.claims_exact_parity,
                    case.resolved.import_created_durable_change,
                    case.resolved.undo_available,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 migration-bridge-card-primitive export.
#[derive(Debug)]
pub enum M5MigrationBridgeCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MigrationBridgeCardViolation>),
}

impl fmt::Display for M5MigrationBridgeCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 migration bridge card primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 migration bridge card primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MigrationBridgeCardArtifactError {}

/// Validation failures emitted by [`M5MigrationBridgeCardPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MigrationBridgeCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required importer / migration consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// An importer / migration consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked bridge resolutions.
    BridgeExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every migration mapping class.
    MappingClassCoverageUnproven,
    /// The worked resolutions do not prove a faithful, an approximated, a partial, and an
    /// unsupported bridge posture.
    PostureCoverageUnproven,
    /// The worked resolutions do not prove the view-mapping-details, open-native-command,
    /// undo-import-changes, review-import-checkpoint, and report-unsupported-edge-case actions.
    ActionCoverageUnproven,
    /// No worked resolution proves undo stays available for a durable import change.
    UndoParityUnproven,
    /// A worked resolution does not preserve honesty, scope, edge cases, or rollback linkage.
    ReversibilityUnproven,
    /// A worked resolution does not preserve its exact bridge identity, old path, new command,
    /// scope, edge cases, or rollback linkage.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5MigrationBridgeCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::BridgeExampleMissing => "bridge_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::MappingClassCoverageUnproven => "mapping_class_coverage_unproven",
            Self::PostureCoverageUnproven => "posture_coverage_unproven",
            Self::ActionCoverageUnproven => "action_coverage_unproven",
            Self::UndoParityUnproven => "undo_parity_unproven",
            Self::ReversibilityUnproven => "reversibility_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 migration-bridge-card-primitive export.
pub fn current_stable_m5_migration_bridge_card_export(
) -> Result<M5MigrationBridgeCardPacket, M5MigrationBridgeCardArtifactError> {
    let packet: M5MigrationBridgeCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-migration-bridge-card-primitive-proof/support_export.json"
    )))
    .map_err(M5MigrationBridgeCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MigrationBridgeCardArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF,
        M5_MIGRATION_BRIDGE_CARD_DOC_REF,
        M5_MIGRATION_BRIDGE_CARD_COMPONENT_MATRIX_REF,
        M5_MIGRATION_BRIDGE_CARD_IMPORTER_OUTCOME_REF,
        M5_MIGRATION_BRIDGE_CARD_ROLLBACK_CHECKPOINT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MigrationBridgeCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5MigrationBridgeCardViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let present: BTreeSet<M5MigrationBridgeConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5MigrationBridgeConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5MigrationBridgeCardViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.mapping_classes.is_empty()
            || row.source_tools.is_empty()
            || row.bridge_postures.is_empty()
            || row.bridge_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5MigrationBridgeCardViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5MigrationBridgeCardViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5MigrationBridgeCardViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5MigrationBridgeCardViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5MigrationBridgeCardViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5MigrationBridgeCardViolation::DowngradeTriggersMissing);
        }
        if row.bridge_examples.is_empty() {
            violations.push(M5MigrationBridgeCardViolation::BridgeExampleMissing);
        }
        if row
            .bridge_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5MigrationBridgeCardViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5MigrationBridgeCardViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5MigrationBridgeCardViolation::RowInvariantViolated);
        }
    }
}

/// Every migration mapping class must be exercised by some worked resolution — the
/// implementation requirement that a bridge card names the exact mapping honesty class across
/// every mapping.
fn validate_mapping_class_coverage(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let exercised: BTreeSet<M5MigrationMappingClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.bridge_examples.iter())
        .map(|case| case.resolved.mapping_class)
        .collect();
    let covered = M5MigrationMappingClass::ALL
        .iter()
        .all(|class| exercised.contains(class));
    if !covered {
        violations.push(M5MigrationBridgeCardViolation::MappingClassCoverageUnproven);
    }
}

/// At least one worked resolution must prove a faithful (exact-parity), an approximated, a
/// partial, and an unsupported bridge posture — the acceptance criterion that imported users
/// never mistake an approximated or partial mapping for exact parity.
fn validate_posture_coverage(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.bridge_examples.iter())
    };
    let has_exact =
        cases().any(|case| case.resolved.bridge_posture == M5MigrationBridgePosture::ExactParity);
    let has_approximated = cases().any(|case| case.resolved.bridge_posture.is_approximated());
    let has_partial = cases().any(|case| case.resolved.bridge_posture.is_incomplete());
    let has_unsupported = cases().any(|case| case.resolved.bridge_posture.is_unsupported());
    if !(has_exact && has_approximated && has_partial && has_unsupported) {
        violations.push(M5MigrationBridgeCardViolation::PostureCoverageUnproven);
    }
}

/// At least one worked resolution must prove each of the view-mapping-details,
/// open-native-command, undo-import-changes, review-import-checkpoint, and
/// report-unsupported-edge-case actions — the implementation requirement that a bridge card
/// offers view-mapping / open-native / undo-import actions.
fn validate_action_coverage(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.bridge_examples.iter())
    };
    let covered = M5MigrationBridgeAction::ALL
        .iter()
        .all(|action| cases().any(|case| case.resolved.available_actions.contains(action)));
    if !covered {
        violations.push(M5MigrationBridgeCardViolation::ActionCoverageUnproven);
    }
}

/// At least one worked resolution must prove that a durable import change keeps its undo action
/// available — the acceptance criterion that undo / review stays available wherever an import
/// changed durable user-facing behavior.
fn validate_undo_parity_coverage(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let proven = packet
        .rows
        .iter()
        .flat_map(|row| row.bridge_examples.iter())
        .any(|case| {
            case.resolved.import_created_durable_change
                && case.resolved.undo_available
                && case.resolved.has_rollback_checkpoint
        });
    if !proven {
        violations.push(M5MigrationBridgeCardViolation::UndoParityUnproven);
    }
}

/// Every worked resolution must disclose the old path / new command / mapping state, never
/// overstate as exact parity, and preserve scope, edge cases, and rollback linkage — the
/// acceptance criteria that imported behavior is explained honestly and stays reversible.
fn validate_reversibility(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.bridge_examples.iter())
        .all(|case| case.preserves_reversibility());
    if !preserved {
        violations.push(M5MigrationBridgeCardViolation::ReversibilityUnproven);
    }
}

/// Every worked resolution must preserve its exact bridge identity, old path, new command,
/// scope, edge cases, and rollback linkage — the invariant that the bridge card never rewrites
/// what it explains.
fn validate_identity_preservation(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.bridge_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5MigrationBridgeCardViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.bridge_card_shows_old_path,
        review.bridge_card_shows_new_command,
        review.bridge_card_shows_mapping_state,
        review.bridge_card_shows_affected_scope,
        review.bridge_card_shows_unsupported_edge_cases,
        review.imported_users_never_mistake_partial_for_exact,
        review.bridge_card_never_masks_mapping_state,
        review.undo_review_available_where_import_changed_durable_behavior,
        review.bridge_card_preserves_import_rollback_linkage,
        review.bridge_card_names_imported_source_tool,
        review.users_understand_imported_behavior_without_detached_docs,
        review.bridge_cards_stable_across_deployment_lines,
        review.bridge_cards_stable_across_consumer_surfaces,
        review.every_bridge_card_declares_accessibility_route,
        review.support_export_reconstructs_bridge_truth,
        review.later_rows_cannot_invent_parallel_bridge_vocabulary,
    ] {
        if !ok {
            violations.push(M5MigrationBridgeCardViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.migration_surfaces_consume_bridge_vocabulary,
        projection.bridge_posture_reads_single_source,
        projection.action_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5MigrationBridgeCardViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MigrationBridgeCardViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MigrationBridgeCardPacket,
    violations: &mut Vec<M5MigrationBridgeCardViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.migration_bridge_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MigrationBridgeCardViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
