//! Framework-object certainty record model: route / component / service /
//! entity rows, convention-diagnostic rows, and generator / codemod /
//! scaffold previews.

use serde::{Deserialize, Serialize};

use super::{Finding, SurfaceClass};
use super::support_strip::{PackOrBridgeSourceBlock, PackSourceClass, SupportClass};

/// Stable record-kind tag for serialized [`FrameworkObjectCertainty`]
/// payloads.
pub const FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND: &str = "framework_object_certainty_record";

/// Schema version for the [`FrameworkObjectCertainty`] payload shape.
pub const FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION: u32 = 1;

/// Discriminator vocabulary for framework-object certainty records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkObjectKind {
    FrameworkObjectRow,
    ConventionDiagnosticRow,
    GeneratorPreviewRow,
}

impl FrameworkObjectKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkObjectRow => "framework_object_row",
            Self::ConventionDiagnosticRow => "convention_diagnostic_row",
            Self::GeneratorPreviewRow => "generator_preview_row",
        }
    }
}

/// Closed framework-object row vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkObjectRowKind {
    RouteRow,
    ComponentRow,
    ServiceRow,
    EntityRow,
    ConfigRow,
    MiddlewareRow,
    DataSourceRow,
}

impl FrameworkObjectRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteRow => "route_row",
            Self::ComponentRow => "component_row",
            Self::ServiceRow => "service_row",
            Self::EntityRow => "entity_row",
            Self::ConfigRow => "config_row",
            Self::MiddlewareRow => "middleware_row",
            Self::DataSourceRow => "data_source_row",
        }
    }
}

/// Closed certainty-label vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertaintyLabelClass {
    ExactPackBacked,
    ExactRuntimeConfirmed,
    DerivedByConvention,
    Imported,
    PartialEvidence,
    HeuristicSuspicion,
    StaleAgainstSource,
    NoAdmissibleEvidence,
}

impl CertaintyLabelClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactPackBacked => "exact_pack_backed",
            Self::ExactRuntimeConfirmed => "exact_runtime_confirmed",
            Self::DerivedByConvention => "derived_by_convention",
            Self::Imported => "imported",
            Self::PartialEvidence => "partial_evidence",
            Self::HeuristicSuspicion => "heuristic_suspicion",
            Self::StaleAgainstSource => "stale_against_source",
            Self::NoAdmissibleEvidence => "no_admissible_evidence",
        }
    }

    /// Whether this certainty label requires the row to keep a visible
    /// partial / derived note instead of hiding the caveat in a secondary
    /// panel.
    pub const fn requires_partial_or_derived_note(self) -> bool {
        matches!(
            self,
            Self::DerivedByConvention
                | Self::PartialEvidence
                | Self::HeuristicSuspicion
                | Self::StaleAgainstSource
        )
    }

    /// Whether this certainty label is one of the two "exact" labels.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::ExactPackBacked | Self::ExactRuntimeConfirmed)
    }
}

/// Closed authored-vs-generated vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoredOriginClass {
    AuthoredByUser,
    GeneratedByFramework,
    GeneratedByPackTemplate,
    GeneratedByCodemod,
    ManagedByPack,
    ImportedFromSnapshot,
    OriginUnknown,
}

impl AuthoredOriginClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredByUser => "authored_by_user",
            Self::GeneratedByFramework => "generated_by_framework",
            Self::GeneratedByPackTemplate => "generated_by_pack_template",
            Self::GeneratedByCodemod => "generated_by_codemod",
            Self::ManagedByPack => "managed_by_pack",
            Self::ImportedFromSnapshot => "imported_from_snapshot",
            Self::OriginUnknown => "origin_unknown",
        }
    }
}

/// Closed evidence-anchor-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceAnchorKindClass {
    SourceFileAnchor,
    SourceSymbolAnchor,
    PackProvingArtifactAnchor,
    BuildAdapterAnchor,
    RuntimeCaptureAnchor,
    ImportedSnapshotAnchor,
    ConventionPatternAnchor,
}

impl EvidenceAnchorKindClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFileAnchor => "source_file_anchor",
            Self::SourceSymbolAnchor => "source_symbol_anchor",
            Self::PackProvingArtifactAnchor => "pack_proving_artifact_anchor",
            Self::BuildAdapterAnchor => "build_adapter_anchor",
            Self::RuntimeCaptureAnchor => "runtime_capture_anchor",
            Self::ImportedSnapshotAnchor => "imported_snapshot_anchor",
            Self::ConventionPatternAnchor => "convention_pattern_anchor",
        }
    }

    /// Whether this anchor counts as a source round-trip back to a
    /// canonical file or symbol.
    pub const fn is_source_round_trip(self) -> bool {
        matches!(self, Self::SourceFileAnchor | Self::SourceSymbolAnchor)
    }
}

/// One evidence anchor entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceAnchor {
    pub evidence_anchor_kind_class: EvidenceAnchorKindClass,
    pub anchor_ref: String,
    pub anchor_label: String,
}

/// Framework-object row block (route / component / service / entity / etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkObjectRowBlock {
    pub framework_object_row_kind: FrameworkObjectRowKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_ref: Option<String>,
    pub object_label: String,
    pub authored_origin_class: AuthoredOriginClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_object_ref: Option<String>,
    pub evidence_anchors: Vec<EvidenceAnchor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_or_derived_note: Option<String>,
}

/// Closed convention-diagnostic vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConventionDiagnosticClass {
    HardContractViolation,
    FrameworkVersionMismatch,
    PackCapabilityLimitation,
    HeuristicSuspicion,
    AmbiguousConvention,
    MissingRegistration,
    UnreachableRoute,
    GeneratedArtifactDrift,
    NotAvailableInThisMode,
}

impl ConventionDiagnosticClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardContractViolation => "hard_contract_violation",
            Self::FrameworkVersionMismatch => "framework_version_mismatch",
            Self::PackCapabilityLimitation => "pack_capability_limitation",
            Self::HeuristicSuspicion => "heuristic_suspicion",
            Self::AmbiguousConvention => "ambiguous_convention",
            Self::MissingRegistration => "missing_registration",
            Self::UnreachableRoute => "unreachable_route",
            Self::GeneratedArtifactDrift => "generated_artifact_drift",
            Self::NotAvailableInThisMode => "not_available_in_this_mode",
        }
    }
}

/// Closed convention-certainty vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConventionCertaintyClass {
    CertaintyProvenViolation,
    CertaintyHighConfidence,
    CertaintyMediumConfidence,
    CertaintyHeuristicOnly,
    CertaintyUnverified,
}

impl ConventionCertaintyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertaintyProvenViolation => "certainty_proven_violation",
            Self::CertaintyHighConfidence => "certainty_high_confidence",
            Self::CertaintyMediumConfidence => "certainty_medium_confidence",
            Self::CertaintyHeuristicOnly => "certainty_heuristic_only",
            Self::CertaintyUnverified => "certainty_unverified",
        }
    }
}

/// Closed convention-fix-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConventionFixActionClass {
    OpenDocs,
    OpenPackStatus,
    OpenMigrationPath,
    OpenCanonicalSource,
    OpenGeneratorPreview,
    OpenRuntimeInspector,
    RequestPolicyReview,
    SuppressWithJustification,
    NoFixAvailable,
}

impl ConventionFixActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDocs => "open_docs",
            Self::OpenPackStatus => "open_pack_status",
            Self::OpenMigrationPath => "open_migration_path",
            Self::OpenCanonicalSource => "open_canonical_source",
            Self::OpenGeneratorPreview => "open_generator_preview",
            Self::OpenRuntimeInspector => "open_runtime_inspector",
            Self::RequestPolicyReview => "request_policy_review",
            Self::SuppressWithJustification => "suppress_with_justification",
            Self::NoFixAvailable => "no_fix_available",
        }
    }
}

/// Convention-diagnostic block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticBlock {
    pub convention_diagnostic_class: ConventionDiagnosticClass,
    pub convention_certainty_class: ConventionCertaintyClass,
    pub affected_object_ref: String,
    pub affected_object_label: String,
    #[serde(default)]
    pub evidence_anchors: Vec<EvidenceAnchor>,
    pub fix_actions: Vec<ConventionFixActionClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_preview_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppressibility_note: Option<String>,
}

/// Closed generator-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorKindClass {
    FrameworkPackScaffold,
    FrameworkPackCodemod,
    FrameworkNativeGenerator,
    BridgeCompatibilityScaffold,
    HeuristicPatternScaffold,
}

impl GeneratorKindClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackScaffold => "framework_pack_scaffold",
            Self::FrameworkPackCodemod => "framework_pack_codemod",
            Self::FrameworkNativeGenerator => "framework_native_generator",
            Self::BridgeCompatibilityScaffold => "bridge_compatibility_scaffold",
            Self::HeuristicPatternScaffold => "heuristic_pattern_scaffold",
        }
    }
}

/// Closed file-effect vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileEffectClass {
    CreateFile,
    ModifyFile,
    DeleteFile,
    RenameFile,
}

impl FileEffectClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateFile => "create_file",
            Self::ModifyFile => "modify_file",
            Self::DeleteFile => "delete_file",
            Self::RenameFile => "rename_file",
        }
    }
}

/// Closed file-ownership vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileOwnershipClass {
    UserOwnedAuthored,
    ManagedByPackOverwritable,
    ManagedByPackUserExtensionZone,
    FrameworkGeneratedOverwritable,
    SharedUserAndGeneratedBlocks,
    OwnershipUnknownRequiresReview,
}

impl FileOwnershipClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserOwnedAuthored => "user_owned_authored",
            Self::ManagedByPackOverwritable => "managed_by_pack_overwritable",
            Self::ManagedByPackUserExtensionZone => "managed_by_pack_user_extension_zone",
            Self::FrameworkGeneratedOverwritable => "framework_generated_overwritable",
            Self::SharedUserAndGeneratedBlocks => "shared_user_and_generated_blocks",
            Self::OwnershipUnknownRequiresReview => "ownership_unknown_requires_review",
        }
    }
}

/// One file-effect row in a generator preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorFileEffectRow {
    pub file_effect_class: FileEffectClass,
    pub file_ownership_class: FileOwnershipClass,
    pub file_path_handle_ref: String,
    pub file_label: String,
    #[serde(default)]
    pub requires_user_confirmation: bool,
}

/// Closed dependency-impact vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyImpactClass {
    NoDependencyChange,
    AddsWorkspaceDependency,
    UpdatesWorkspaceDependency,
    RemovesWorkspaceDependency,
    AddsFrameworkPackDependency,
    ConfigOnlyChange,
    DependencyImpactUnknownRequiresReview,
}

impl DependencyImpactClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDependencyChange => "no_dependency_change",
            Self::AddsWorkspaceDependency => "adds_workspace_dependency",
            Self::UpdatesWorkspaceDependency => "updates_workspace_dependency",
            Self::RemovesWorkspaceDependency => "removes_workspace_dependency",
            Self::AddsFrameworkPackDependency => "adds_framework_pack_dependency",
            Self::ConfigOnlyChange => "config_only_change",
            Self::DependencyImpactUnknownRequiresReview => {
                "dependency_impact_unknown_requires_review"
            }
        }
    }
}

/// Closed rollback vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackClass {
    RollbackViaCheckpoint,
    RollbackViaVcsRevert,
    RollbackViaExplicitUndo,
    RollbackNotAvailable,
    RollbackUnknownRequiresReview,
}

impl RollbackClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RollbackViaCheckpoint => "rollback_via_checkpoint",
            Self::RollbackViaVcsRevert => "rollback_via_vcs_revert",
            Self::RollbackViaExplicitUndo => "rollback_via_explicit_undo",
            Self::RollbackNotAvailable => "rollback_not_available",
            Self::RollbackUnknownRequiresReview => "rollback_unknown_requires_review",
        }
    }
}

/// Generator preview block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorPreviewBlock {
    pub generator_kind_class: GeneratorKindClass,
    pub generator_id_ref: String,
    pub generator_label: String,
    pub generator_version_label: String,
    pub input_summary: String,
    pub file_effect_rows: Vec<GeneratorFileEffectRow>,
    pub dependency_impact_class: DependencyImpactClass,
    pub rollback_class: RollbackClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    #[serde(default)]
    pub regenerate_path_available: bool,
}

/// Closed row-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowActionClass {
    OpenCanonicalSource,
    OpenPackDocs,
    OpenCompatibilityDetails,
    OpenRuntimeInspector,
    RevealInRouteExplorer,
    RevealInComponentTree,
    OpenGeneratorPreview,
    OpenConventionDiagnostic,
    RequestPolicyReview,
    NoActionAvailable,
}

impl RowActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCanonicalSource => "open_canonical_source",
            Self::OpenPackDocs => "open_pack_docs",
            Self::OpenCompatibilityDetails => "open_compatibility_details",
            Self::OpenRuntimeInspector => "open_runtime_inspector",
            Self::RevealInRouteExplorer => "reveal_in_route_explorer",
            Self::RevealInComponentTree => "reveal_in_component_tree",
            Self::OpenGeneratorPreview => "open_generator_preview",
            Self::OpenConventionDiagnostic => "open_convention_diagnostic",
            Self::RequestPolicyReview => "request_policy_review",
            Self::NoActionAvailable => "no_action_available",
        }
    }
}

/// Canonical framework-object certainty record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkObjectCertainty {
    pub record_kind: String,
    pub framework_object_certainty_schema_version: u32,
    pub framework_object_certainty_id: String,
    pub captured_at: String,
    pub surface_class: SurfaceClass,
    pub framework_object_kind: FrameworkObjectKind,
    pub certainty_label_class: CertaintyLabelClass,
    pub support_class: SupportClass,
    pub pack_or_bridge_source_block: PackOrBridgeSourceBlock,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_object_row_block: Option<FrameworkObjectRowBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convention_diagnostic_block: Option<ConventionDiagnosticBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_preview_block: Option<GeneratorPreviewBlock>,
    pub actions: Vec<RowActionClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_strip_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_certainty_row_record_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_sync_chip_record_ref: Option<String>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl FrameworkObjectCertainty {
    /// Returns typed truth-rule findings; an empty vector means the record
    /// is internally consistent with the schema's allOf rules.
    pub fn validate(&self) -> Vec<Finding> {
        let mut findings = Vec::new();
        let subject = self.framework_object_certainty_id.as_str();

        if self.record_kind != FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND {
            findings.push(Finding::new(
                "framework_object_certainty.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.framework_object_certainty_schema_version
            != FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION
        {
            findings.push(Finding::new(
                "framework_object_certainty.schema_version",
                subject,
                format!(
                    "framework_object_certainty_schema_version must be {}, found {}",
                    FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION,
                    self.framework_object_certainty_schema_version
                ),
            ));
        }
        if self.actions.is_empty() {
            findings.push(Finding::new(
                "framework_object_certainty.actions_not_empty",
                subject,
                "every framework-object certainty record must declare at least one action",
            ));
        }

        // Subject-block discriminator.
        match self.framework_object_kind {
            FrameworkObjectKind::FrameworkObjectRow => {
                if self.framework_object_row_block.is_none() {
                    findings.push(Finding::new(
                        "framework_object_certainty.row_block_required",
                        subject,
                        "framework_object_row requires a non-null framework_object_row_block",
                    ));
                }
                if self.convention_diagnostic_block.is_some()
                    || self.generator_preview_block.is_some()
                {
                    findings.push(Finding::new(
                        "framework_object_certainty.row_block_exclusive",
                        subject,
                        "framework_object_row forbids convention_diagnostic_block or generator_preview_block",
                    ));
                }
            }
            FrameworkObjectKind::ConventionDiagnosticRow => {
                if self.convention_diagnostic_block.is_none() {
                    findings.push(Finding::new(
                        "framework_object_certainty.diagnostic_block_required",
                        subject,
                        "convention_diagnostic_row requires a non-null convention_diagnostic_block",
                    ));
                }
                if self.framework_object_row_block.is_some()
                    || self.generator_preview_block.is_some()
                {
                    findings.push(Finding::new(
                        "framework_object_certainty.diagnostic_block_exclusive",
                        subject,
                        "convention_diagnostic_row forbids framework_object_row_block or generator_preview_block",
                    ));
                }
            }
            FrameworkObjectKind::GeneratorPreviewRow => {
                if self.generator_preview_block.is_none() {
                    findings.push(Finding::new(
                        "framework_object_certainty.generator_block_required",
                        subject,
                        "generator_preview_row requires a non-null generator_preview_block",
                    ));
                }
                if self.framework_object_row_block.is_some()
                    || self.convention_diagnostic_block.is_some()
                {
                    findings.push(Finding::new(
                        "framework_object_certainty.generator_block_exclusive",
                        subject,
                        "generator_preview_row forbids framework_object_row_block or convention_diagnostic_block",
                    ));
                }
            }
        }

        // Certainty / support pairing.
        if self.certainty_label_class.is_exact() && !self.support_class.admits_exact_certainty() {
            findings.push(Finding::new(
                "framework_object_certainty.exact_requires_pack_or_native",
                subject,
                "exact_pack_backed / exact_runtime_confirmed require support_class in {core_native, framework_pack}",
            ));
        }
        if matches!(
            self.certainty_label_class,
            CertaintyLabelClass::HeuristicSuspicion | CertaintyLabelClass::NoAdmissibleEvidence
        ) && self.support_class.admits_exact_certainty()
        {
            findings.push(Finding::new(
                "framework_object_certainty.heuristic_with_exact_support",
                subject,
                "heuristic_suspicion / no_admissible_evidence forbid support_class in {core_native, framework_pack}",
            ));
        }
        if self.certainty_label_class == CertaintyLabelClass::ExactPackBacked
            && !matches!(
                self.pack_or_bridge_source_block.pack_source_class,
                PackSourceClass::FirstPartyNative
                    | PackSourceClass::GovernedFrameworkPack
                    | PackSourceClass::CommunityFrameworkPack
            )
        {
            findings.push(Finding::new(
                "framework_object_certainty.exact_pack_backed_source",
                subject,
                "exact_pack_backed requires a first_party_native, governed_framework_pack, or community_framework_pack source",
            ));
        }
        if self.support_class == SupportClass::UnsupportedOrUnclaimed
            && self.certainty_label_class != CertaintyLabelClass::NoAdmissibleEvidence
        {
            findings.push(Finding::new(
                "framework_object_certainty.unsupported_requires_no_admissible_evidence",
                subject,
                "support_class = unsupported_or_unclaimed forces certainty_label_class = no_admissible_evidence",
            ));
        }

        // Object-row-specific rules.
        if let Some(row) = &self.framework_object_row_block {
            if row.evidence_anchors.is_empty() {
                findings.push(Finding::new(
                    "framework_object_certainty.row_evidence_anchors_required",
                    subject,
                    "framework_object_row requires at least one evidence anchor",
                ));
            }
            if self.certainty_label_class.requires_partial_or_derived_note()
                && row.partial_or_derived_note.is_none()
            {
                findings.push(Finding::new(
                    "framework_object_certainty.partial_or_derived_note_required",
                    subject,
                    "derived/partial/heuristic/stale rows must keep a visible partial_or_derived_note",
                ));
            }
            if matches!(
                row.framework_object_row_kind,
                FrameworkObjectRowKind::RouteRow
                    | FrameworkObjectRowKind::ComponentRow
                    | FrameworkObjectRowKind::EntityRow
                    | FrameworkObjectRowKind::ServiceRow
            ) && !row
                .evidence_anchors
                .iter()
                .any(|anchor| anchor.evidence_anchor_kind_class.is_source_round_trip())
                && row.authored_origin_class == AuthoredOriginClass::AuthoredByUser
            {
                findings.push(Finding::new(
                    "framework_object_certainty.source_round_trip_required",
                    subject,
                    "user-authored route/component/service/entity rows must preserve a source-file or source-symbol anchor",
                ));
            }
        }

        // Convention-diagnostic rules.
        if let Some(diag) = &self.convention_diagnostic_block {
            if diag.fix_actions.is_empty() {
                findings.push(Finding::new(
                    "framework_object_certainty.diagnostic_fix_actions_required",
                    subject,
                    "every convention diagnostic must offer at least one fix action",
                ));
            }
            if diag.convention_certainty_class
                == ConventionCertaintyClass::CertaintyProvenViolation
                && !matches!(
                    diag.convention_diagnostic_class,
                    ConventionDiagnosticClass::HardContractViolation
                        | ConventionDiagnosticClass::FrameworkVersionMismatch
                        | ConventionDiagnosticClass::MissingRegistration
                        | ConventionDiagnosticClass::GeneratedArtifactDrift
                )
            {
                findings.push(Finding::new(
                    "framework_object_certainty.proven_violation_class",
                    subject,
                    "certainty_proven_violation requires a hard / version / missing-registration / generated-drift diagnostic",
                ));
            }
            if diag.convention_diagnostic_class == ConventionDiagnosticClass::HardContractViolation
                && !matches!(
                    self.support_class,
                    SupportClass::CoreNative
                        | SupportClass::FrameworkPack
                        | SupportClass::BridgeCompatibilityLayer
                )
            {
                findings.push(Finding::new(
                    "framework_object_certainty.hard_contract_requires_pack_or_bridge",
                    subject,
                    "hard_contract_violation requires support_class in {core_native, framework_pack, bridge_compatibility_layer}",
                ));
            }
            if diag.convention_diagnostic_class == ConventionDiagnosticClass::HeuristicSuspicion
                && !matches!(
                    self.support_class,
                    SupportClass::HeuristicConventionMode | SupportClass::BridgeCompatibilityLayer
                )
            {
                findings.push(Finding::new(
                    "framework_object_certainty.heuristic_suspicion_support_class",
                    subject,
                    "heuristic_suspicion requires support_class in {heuristic_convention_mode, bridge_compatibility_layer}",
                ));
            }
            if diag
                .fix_actions
                .contains(&ConventionFixActionClass::OpenGeneratorPreview)
                && diag.generator_preview_ref.is_none()
            {
                findings.push(Finding::new(
                    "framework_object_certainty.generator_preview_ref_required",
                    subject,
                    "open_generator_preview must be paired with a generator_preview_ref",
                ));
            }
        }

        // Generator preview rules.
        if let Some(gen) = &self.generator_preview_block {
            if gen.file_effect_rows.is_empty() {
                findings.push(Finding::new(
                    "framework_object_certainty.generator_file_effects_required",
                    subject,
                    "every generator preview must declare at least one file-effect row",
                ));
            }
            for (idx, row) in gen.file_effect_rows.iter().enumerate() {
                if row.file_effect_class == FileEffectClass::DeleteFile
                    && row.file_ownership_class == FileOwnershipClass::UserOwnedAuthored
                    && !row.requires_user_confirmation
                {
                    findings.push(Finding::new(
                        "framework_object_certainty.delete_user_owned_requires_confirmation",
                        subject,
                        format!(
                            "file-effect row {idx} deletes a user-owned file; requires_user_confirmation must be true"
                        ),
                    ));
                }
            }
            match gen.generator_kind_class {
                GeneratorKindClass::FrameworkPackScaffold
                | GeneratorKindClass::FrameworkPackCodemod
                | GeneratorKindClass::FrameworkNativeGenerator => {
                    if !self.support_class.admits_exact_certainty() {
                        findings.push(Finding::new(
                            "framework_object_certainty.pack_generator_support_class",
                            subject,
                            "framework_pack_* and framework_native_generator require support_class in {core_native, framework_pack}",
                        ));
                    }
                }
                GeneratorKindClass::BridgeCompatibilityScaffold => {
                    if self.support_class != SupportClass::BridgeCompatibilityLayer {
                        findings.push(Finding::new(
                            "framework_object_certainty.bridge_generator_support_class",
                            subject,
                            "bridge_compatibility_scaffold requires support_class = bridge_compatibility_layer",
                        ));
                    }
                }
                GeneratorKindClass::HeuristicPatternScaffold => {
                    if self.support_class != SupportClass::HeuristicConventionMode {
                        findings.push(Finding::new(
                            "framework_object_certainty.heuristic_generator_support_class",
                            subject,
                            "heuristic_pattern_scaffold requires support_class = heuristic_convention_mode",
                        ));
                    }
                }
            }
            if gen.rollback_class == RollbackClass::RollbackViaCheckpoint
                && gen.checkpoint_ref.is_none()
            {
                findings.push(Finding::new(
                    "framework_object_certainty.checkpoint_ref_required",
                    subject,
                    "rollback_via_checkpoint requires a non-null checkpoint_ref",
                ));
            }
        }

        findings
    }
}
