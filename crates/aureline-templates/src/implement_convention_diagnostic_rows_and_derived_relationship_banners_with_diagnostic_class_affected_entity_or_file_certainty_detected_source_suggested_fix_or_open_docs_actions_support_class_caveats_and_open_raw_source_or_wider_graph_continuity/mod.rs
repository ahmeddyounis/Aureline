//! Two reusable M5 framework-diagnostic components — the convention-diagnostic row and the
//! derived-relationship banner — so a user can read a framework warning without collapsing distinct
//! problems into one generic state and can see, at the point of consumption, exactly where framework
//! truth was inferred rather than read: every convention-diagnostic row names its diagnostic class
//! (a hard contract violation, a pack limitation, a version mismatch, a heuristic suspicion, a
//! deprecation notice, or an unknown diagnostic), its affected entity and file, its confidence and
//! severity, its detected source, its suggested fix or open-docs action, and its support-class
//! caveat, and links back to a canonical proving file; every derived-relationship banner names its
//! source of inference, its last refresh, its exact / partial / heuristic / runtime-confirmed state,
//! and its open-raw-source or open-wider-graph actions, and links back to a canonical proving source.
//! Neither component acts like a hidden parallel model — the exact-versus-heuristic certainty and the
//! distinct-diagnostic-class boundary stay visible at row level rather than only in a buried detail
//! panel, and a banner never hides an approximation in the background.
//!
//! Aureline's frozen framework-component matrix
//! ([`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`])
//! names the convention-diagnostic row and the derived-relationship banner as two governed component
//! families and freezes their controlled vocabulary — the one controlled certainty disposition
//! (`core_native`, `framework_pack`, `bridge`, `heuristic_convention`, `verified`,
//! `derived_by_convention`, `runtime_confirmed`, `partial`); the convention confidence classes
//! (`verified`, `high_confidence`, `heuristic_convention`, `derived_by_convention`, `low_confidence`,
//! `unknown`) and diagnostic severities (`error`, `warning`, `hint`, `info`, `suppressed`, `stale`);
//! the derived-relationship classes (`exact_from_source`, `inferred_from_runtime`, `heuristic_link`,
//! `derived_by_convention`, `partial_link`, `unresolved_link`) and relationship proving states
//! (`proving_source_linked`, `source_linked_partial`, `runtime_evidence_only`, `convention_only`,
//! `no_proving_source`, `unknown_proving`); the surface families; the deployment lines; the consumer
//! surfaces; the accessibility routes; the required labels; and the downgrade triggers. This module
//! *implements* those contracts as two co-equal component vectors — a convention-diagnostic row and a
//! derived-relationship banner — so a claimed M5 convention-diagnostics, editor-gutter, topology-
//! explorer, CLI, or support-export surface can project a row and a banner that keep the same
//! certainty, distinct-diagnostic-class, proving-source, and support-caveat truth.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_convention_diagnostic_posture`] — takes a row's frozen convention confidence class and
//!   derives its certainty posture (exact from source, runtime confirmed, heuristic, or partial /
//!   unresolved), whether the row claims exact-from-source, whether it has a proving source form, and
//!   which notes it must carry — so a heuristic suspicion can never read as an exact contract fact and
//!   an ungrounded diagnostic can never pretend to link to a proving file that does not exist.
//! * [`resolve_derived_relationship_posture`] — takes a banner's frozen derived-relationship class and
//!   relationship proving state and derives its certainty posture, whether it claims exact-from-source,
//!   whether it has a proving source form, and which notes it must carry — so an inferred or derived
//!   link can never read as exact and an unresolved relationship can never pretend to link to a proving
//!   source it does not have.
//!
//! A single controls packet — [`ConventionDiagnosticDerivedRelationshipControlsPacket`] — binds one
//! vector of convention-diagnostic rows and one vector of derived-relationship banners to the same
//! certainty, proving-source, and non-visual accessibility vocabulary, so certainty and evidence stay
//! explicit across the convention-diagnostics, editor-gutter, topology-explorer, CLI, and support
//! consumers.
//!
//! The component family ([`M5FrameworkComponentFamily`]), convention confidence class
//! ([`M5ConventionConfidenceClass`]), diagnostic severity ([`M5ConventionDiagnosticSeverity`]),
//! derived-relationship class ([`M5DerivedRelationshipClass`]), relationship proving state
//! ([`M5RelationshipProvingState`]), certainty disposition ([`M5FrameworkCertaintyDisposition`]),
//! surface family ([`M5FrameworkSurfaceFamily`]), deployment line ([`M5FrameworkDeploymentLine`]),
//! consumer surface ([`M5FrameworkConsumerSurface`]), accessibility route
//! ([`M5FrameworkAccessibilityRoute`]), required label ([`M5FrameworkRequiredLabel`]), and downgrade
//! trigger ([`M5FrameworkDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the two components themselves:
//! the derived certainty posture, the diagnostic class, the detection source, the support-caveat
//! class, the diagnostic fix action, the inference source, the banner refresh state, the proving-
//! source link kind, and the bounded row and banner actions. No M5 diagnostic surface invents a
//! second diagnostic or banner grammar.
//!
//! Raw file bodies, raw source trees, pasted local paths, repository URLs, credentials, and secrets
//! stay outside the export boundary; every note, proving-source reference, and component identity is
//! carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The component family, the convention / relationship vocabularies, the certainty disposition, and
// the surface / deployment / consumer / accessibility / label / downgrade vocabularies are frozen
// once, in the framework-component matrix. This lane reuses them verbatim so it never invents a
// parallel diagnostic or banner vocabulary.
pub use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5ConventionConfidenceClass, M5ConventionDiagnosticSeverity, M5DerivedRelationshipClass,
    M5FrameworkAccessibilityRoute, M5FrameworkCertaintyDisposition, M5FrameworkComponentFamily,
    M5FrameworkConsumerSurface, M5FrameworkDeploymentLine, M5FrameworkDowngradeTrigger,
    M5FrameworkRequiredLabel, M5FrameworkSurfaceFamily, M5RelationshipProvingState,
    M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF, M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF,
    M5_FRAMEWORK_COMPONENT_DOC_REF, M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ConventionDiagnosticDerivedRelationshipControlsPacket`].
pub const CONVENTION_RELATIONSHIP_CONTROLS_RECORD_KIND: &str =
    "implement_convention_diagnostic_rows_and_derived_relationship_banners_with_diagnostic_class_affected_entity_or_file_certainty_detected_source_suggested_fix_or_open_docs_actions_support_class_caveats_and_open_raw_source_or_wider_graph_continuity";

/// Schema version for M5 convention-diagnostic-row / derived-relationship-banner control records.
pub const CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-convention-diagnostic-derived-relationship-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const CONVENTION_RELATIONSHIP_CONTROLS_DOC_REF: &str =
    "docs/frameworks/m5/m5_convention_diagnostic_derived_relationship_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const CONVENTION_RELATIONSHIP_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-convention-diagnostic-derived-relationship-controls";

/// Repo-relative path of the checked support-export artifact.
pub const CONVENTION_RELATIONSHIP_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-convention-diagnostic-derived-relationship-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CONVENTION_RELATIONSHIP_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-convention-diagnostic-derived-relationship-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const CONVENTION_RELATIONSHIP_CONTROLS_REPORT_REF: &str =
    "artifacts/design/m5-convention-diagnostic-derived-relationship.md";

// ---- shared derived vocabulary ------------------------------------------

/// Derived certainty posture a convention-diagnostic row or derived-relationship banner may present.
/// These are the exact acceptance-criteria labels so a user can tell at a glance whether the claim is
/// exact from source, runtime confirmed, a heuristic guess, or only partial / unresolved — a
/// heuristic diagnostic or an inferred relationship can never read as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedCertaintyPosture {
    /// Exact, read directly from source.
    ExactFromSource,
    /// Confirmed by observing the running application.
    RuntimeConfirmed,
    /// A heuristic convention or derived-by-convention guess, not an exact fact.
    Heuristic,
    /// Partial evidence only, or unresolved.
    PartialOrUnresolved,
}

impl DerivedCertaintyPosture {
    /// Every certainty posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactFromSource,
        Self::RuntimeConfirmed,
        Self::Heuristic,
        Self::PartialOrUnresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFromSource => "exact_from_source",
            Self::RuntimeConfirmed => "runtime_confirmed",
            Self::Heuristic => "heuristic",
            Self::PartialOrUnresolved => "partial_or_unresolved",
        }
    }

    /// True only when the claim is exact, read directly from source.
    pub const fn is_exact_from_source(self) -> bool {
        matches!(self, Self::ExactFromSource)
    }

    /// True when the claim is confirmed by observing the running application.
    pub const fn is_runtime_confirmed(self) -> bool {
        matches!(self, Self::RuntimeConfirmed)
    }

    /// True when the posture is heuristic or partial / unresolved and must therefore never read as
    /// exact from source.
    pub const fn must_not_read_as_exact(self) -> bool {
        matches!(self, Self::Heuristic | Self::PartialOrUnresolved)
    }

    /// True when the claim must carry an explicit heuristic note.
    pub const fn needs_heuristic_note(self) -> bool {
        matches!(self, Self::Heuristic)
    }

    /// True when the claim must carry an explicit partial / unresolved note.
    pub const fn needs_partial_note(self) -> bool {
        matches!(self, Self::PartialOrUnresolved)
    }
}

/// The kind of stable proving source a diagnostic row or relationship banner links its next step
/// against, so a component never acts like a hidden parallel model — every next step is a canonical
/// source file, source symbol, runtime trace, or docs reference the user can reopen, or an explicit
/// no-proving-source state when none exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvingSourceLink {
    /// A canonical source-file reference.
    SourceFile,
    /// A canonical source-symbol reference.
    SourceSymbol,
    /// A runtime-trace reference (confirmed by observation, no static source form).
    RuntimeTrace,
    /// A docs / reference anchor.
    DocsAnchor,
    /// No proving source exists (the component names that it links nowhere).
    NoProvingSource,
}

impl ProvingSourceLink {
    /// Every proving-source link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SourceFile,
        Self::SourceSymbol,
        Self::RuntimeTrace,
        Self::DocsAnchor,
        Self::NoProvingSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFile => "source_file",
            Self::SourceSymbol => "source_symbol",
            Self::RuntimeTrace => "runtime_trace",
            Self::DocsAnchor => "docs_anchor",
            Self::NoProvingSource => "no_proving_source",
        }
    }

    /// True when this kind names a resolvable proving-source target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoProvingSource)
    }
}

// ---- convention-diagnostic vocabulary -----------------------------------

/// The distinct class of a convention-diagnostic row, so a framework warning never collapses a hard
/// contract violation, a pack limitation, a version mismatch, and a heuristic suspicion into one
/// generic warning state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticClass {
    /// A hard framework-contract violation.
    HardContractViolation,
    /// A limitation of the active framework pack.
    PackLimitation,
    /// A framework / pack version mismatch.
    VersionMismatch,
    /// A heuristic suspicion, not a confirmed problem.
    HeuristicSuspicion,
    /// A deprecation notice.
    DeprecationNotice,
    /// An unknown / unclassified diagnostic.
    UnknownDiagnostic,
}

impl DiagnosticClass {
    /// Every diagnostic class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HardContractViolation,
        Self::PackLimitation,
        Self::VersionMismatch,
        Self::HeuristicSuspicion,
        Self::DeprecationNotice,
        Self::UnknownDiagnostic,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardContractViolation => "hard_contract_violation",
            Self::PackLimitation => "pack_limitation",
            Self::VersionMismatch => "version_mismatch",
            Self::HeuristicSuspicion => "heuristic_suspicion",
            Self::DeprecationNotice => "deprecation_notice",
            Self::UnknownDiagnostic => "unknown_diagnostic",
        }
    }
}

/// How a convention-diagnostic row was detected, so the detected source stays explicit and a
/// heuristic scan never reads as a verified contract check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    /// Detected by static analysis of the source.
    StaticAnalysis,
    /// Detected against a framework contract.
    FrameworkContract,
    /// Detected from the framework pack manifest.
    PackManifest,
    /// Detected by observing the running application.
    RuntimeProbe,
    /// Detected by a heuristic scan.
    HeuristicScan,
}

impl DetectionSource {
    /// Every detection source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StaticAnalysis,
        Self::FrameworkContract,
        Self::PackManifest,
        Self::RuntimeProbe,
        Self::HeuristicScan,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticAnalysis => "static_analysis",
            Self::FrameworkContract => "framework_contract",
            Self::PackManifest => "pack_manifest",
            Self::RuntimeProbe => "runtime_probe",
            Self::HeuristicScan => "heuristic_scan",
        }
    }
}

/// The support-class caveat a convention-diagnostic row carries, so a pack limitation, a version
/// mismatch, or bridged / heuristic-only behavior never reads as fully-supported first-party truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCaveatClass {
    /// Fully supported, no caveat.
    FullySupported,
    /// Limited by the active framework pack.
    PackLimited,
    /// The diagnostic depends on a mismatched framework / pack version.
    VersionMismatch,
    /// Produced by bridged behavior, not exact first-party support.
    BridgedBehavior,
    /// Produced by a heuristic only.
    HeuristicOnly,
    /// Unsupported on this framework / build.
    Unsupported,
}

impl SupportCaveatClass {
    /// Every support-caveat class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullySupported,
        Self::PackLimited,
        Self::VersionMismatch,
        Self::BridgedBehavior,
        Self::HeuristicOnly,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::PackLimited => "pack_limited",
            Self::VersionMismatch => "version_mismatch",
            Self::BridgedBehavior => "bridged_behavior",
            Self::HeuristicOnly => "heuristic_only",
            Self::Unsupported => "unsupported",
        }
    }

    /// True when the caveat is anything other than fully-supported and must therefore carry an
    /// explicit caveat label.
    pub const fn needs_caveat_label(self) -> bool {
        !matches!(self, Self::FullySupported)
    }
}

/// The suggested-fix affordance a convention-diagnostic row offers, so a diagnostic never implies a
/// one-click fix it does not have and always keeps at least an open-docs path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticFixAction {
    /// An auto-fix is available.
    AutoFixAvailable,
    /// Manual fix guidance is available.
    ManualFixGuidance,
    /// Only open-docs guidance is available.
    OpenDocsOnly,
    /// No fix is available.
    NoFixAvailable,
}

impl DiagnosticFixAction {
    /// Every fix action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AutoFixAvailable,
        Self::ManualFixGuidance,
        Self::OpenDocsOnly,
        Self::NoFixAvailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutoFixAvailable => "auto_fix_available",
            Self::ManualFixGuidance => "manual_fix_guidance",
            Self::OpenDocsOnly => "open_docs_only",
            Self::NoFixAvailable => "no_fix_available",
        }
    }
}

/// One keyboard-complete default action a convention-diagnostic row offers, so a row never hides its
/// proving-file, class / confidence, or fix / open-docs affordance behind a pointer-only gesture.
/// `OpenProvingFile`, `InspectClassAndConfidence`, and `OpenDocsOrApplyFix` are always offered so the
/// proving file, the class / confidence, and the fix / docs path stay inspectable before a user
/// trusts the diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticRowAction {
    /// Open the canonical proving file (always available).
    OpenProvingFile,
    /// Inspect the diagnostic class and confidence (always available).
    InspectClassAndConfidence,
    /// Open the docs or apply the suggested fix (always available).
    OpenDocsOrApplyFix,
    /// Copy the stable diagnostic id.
    CopyDiagnosticId,
    /// Open a docs / reference anchor.
    OpenReference,
}

impl DiagnosticRowAction {
    /// Every diagnostic-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenProvingFile,
        Self::InspectClassAndConfidence,
        Self::OpenDocsOrApplyFix,
        Self::CopyDiagnosticId,
        Self::OpenReference,
    ];

    /// The default actions every keyboard-complete diagnostic row must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenProvingFile,
        Self::InspectClassAndConfidence,
        Self::OpenDocsOrApplyFix,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenProvingFile => "open_proving_file",
            Self::InspectClassAndConfidence => "inspect_class_and_confidence",
            Self::OpenDocsOrApplyFix => "open_docs_or_apply_fix",
            Self::CopyDiagnosticId => "copy_diagnostic_id",
            Self::OpenReference => "open_reference",
        }
    }
}

// ---- derived-relationship vocabulary ------------------------------------

/// The source of inference a derived-relationship banner names, so a banner never leaves how it knows
/// the relationship implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InferenceSource {
    /// Read directly from static source.
    StaticSource,
    /// Observed from the running application.
    RuntimeObservation,
    /// Inferred from a naming convention.
    NamingConvention,
    /// Inferred from the dependency graph.
    DependencyGraph,
    /// Declared in a manifest.
    ManifestDeclaration,
}

impl InferenceSource {
    /// Every inference source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StaticSource,
        Self::RuntimeObservation,
        Self::NamingConvention,
        Self::DependencyGraph,
        Self::ManifestDeclaration,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticSource => "static_source",
            Self::RuntimeObservation => "runtime_observation",
            Self::NamingConvention => "naming_convention",
            Self::DependencyGraph => "dependency_graph",
            Self::ManifestDeclaration => "manifest_declaration",
        }
    }
}

/// The refresh state a derived-relationship banner carries, so a stale or never-refreshed inference
/// never reads as current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefreshState {
    /// The inference is current.
    Current,
    /// The inference was imported from another environment.
    Imported,
    /// The inference is stale.
    Stale,
    /// The relationship has never been refreshed.
    NeverRefreshed,
    /// Refresh state is unknown.
    Unknown,
}

impl RefreshState {
    /// Every refresh state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Imported,
        Self::Stale,
        Self::NeverRefreshed,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::NeverRefreshed => "never_refreshed",
            Self::Unknown => "unknown",
        }
    }

    /// True when the refresh signal must carry an explicit not-current note.
    pub const fn needs_note(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// One keyboard-complete default action a derived-relationship banner offers, so a banner never hides
/// its raw-source, wider-graph, or state / source affordance behind a pointer-only gesture.
/// `OpenRawSource`, `OpenWiderGraph`, and `InspectStateAndSource` are always offered so the raw
/// source, the wider graph, and the state / inference source stay inspectable before a user trusts
/// the banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BannerAction {
    /// Open the raw / proving source (always available).
    OpenRawSource,
    /// Open the wider relationship graph (always available).
    OpenWiderGraph,
    /// Inspect the derived state and inference source (always available).
    InspectStateAndSource,
    /// Copy the stable banner id.
    CopyBannerId,
    /// Open a docs / reference anchor.
    OpenReference,
}

impl BannerAction {
    /// Every banner action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenRawSource,
        Self::OpenWiderGraph,
        Self::InspectStateAndSource,
        Self::CopyBannerId,
        Self::OpenReference,
    ];

    /// The default actions every keyboard-complete banner must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenRawSource,
        Self::OpenWiderGraph,
        Self::InspectStateAndSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRawSource => "open_raw_source",
            Self::OpenWiderGraph => "open_wider_graph",
            Self::InspectStateAndSource => "inspect_state_and_source",
            Self::CopyBannerId => "copy_banner_id",
            Self::OpenReference => "open_reference",
        }
    }
}

// ---- resolvers ----------------------------------------------------------

/// Disclosures a convention-diagnostic row must carry, derived from its convention confidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConventionDiagnosticDisclosure {
    /// The derived certainty posture this row may present.
    pub certainty_posture: DerivedCertaintyPosture,
    /// Whether the row is exact, read directly from source.
    pub is_exact_from_source: bool,
    /// Whether the row is runtime confirmed.
    pub is_runtime_confirmed: bool,
    /// Whether the row must never read as exact from source.
    pub must_not_read_as_exact: bool,
    /// Whether the row has a proving source form to prove.
    pub has_source_form: bool,
    /// Whether the row must carry an explicit heuristic note.
    pub needs_heuristic_note: bool,
    /// Whether the row must carry an explicit partial / unresolved note.
    pub needs_partial_note: bool,
    /// Whether the row must carry an explicit no-source-form note (unknown / ungrounded).
    pub needs_no_source_form_note: bool,
}

/// Resolves the certainty and proving-source truth a convention-diagnostic row may present.
///
/// A `verified` confidence is exact; a `high_confidence`, `heuristic_convention`, or
/// `derived_by_convention` one is heuristic; a `low_confidence` or `unknown` one is partial /
/// unresolved — so a heuristic suspicion can never read as an exact contract fact. An `unknown`
/// confidence has no proving source form, so it can never pretend to link to a proving file it does
/// not have.
pub fn resolve_convention_diagnostic_posture(
    confidence: M5ConventionConfidenceClass,
) -> ConventionDiagnosticDisclosure {
    use DerivedCertaintyPosture as Certainty;
    use M5ConventionConfidenceClass as Confidence;

    let certainty_posture = match confidence {
        Confidence::Verified => Certainty::ExactFromSource,
        Confidence::HighConfidence
        | Confidence::HeuristicConvention
        | Confidence::DerivedByConvention => Certainty::Heuristic,
        Confidence::LowConfidence | Confidence::Unknown => Certainty::PartialOrUnresolved,
    };
    let has_source_form = !matches!(confidence, Confidence::Unknown);

    ConventionDiagnosticDisclosure {
        certainty_posture,
        is_exact_from_source: certainty_posture.is_exact_from_source(),
        is_runtime_confirmed: certainty_posture.is_runtime_confirmed(),
        must_not_read_as_exact: certainty_posture.must_not_read_as_exact(),
        has_source_form,
        needs_heuristic_note: certainty_posture.needs_heuristic_note(),
        needs_partial_note: certainty_posture.needs_partial_note(),
        needs_no_source_form_note: !has_source_form,
    }
}

/// Disclosures a derived-relationship banner must carry, derived from its derived-relationship class
/// and relationship proving state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedRelationshipDisclosure {
    /// The derived certainty posture this banner may present.
    pub certainty_posture: DerivedCertaintyPosture,
    /// Whether the banner is exact, read directly from source.
    pub is_exact_from_source: bool,
    /// Whether the banner is runtime confirmed.
    pub is_runtime_confirmed: bool,
    /// Whether the banner must never read as exact from source.
    pub must_not_read_as_exact: bool,
    /// Whether the banner has a proving source form to prove.
    pub has_source_form: bool,
    /// Whether the banner must carry an explicit heuristic note.
    pub needs_heuristic_note: bool,
    /// Whether the banner must carry an explicit partial / unresolved note.
    pub needs_partial_note: bool,
    /// Whether the banner must carry an explicit no-source-form note (no / unknown proving).
    pub needs_no_source_form_note: bool,
}

/// Resolves the certainty and proving-source truth a derived-relationship banner may present.
///
/// An `exact_from_source` class is exact; an `inferred_from_runtime` one is runtime confirmed; a
/// `heuristic_link` or `derived_by_convention` one is heuristic; a `partial_link` or `unresolved_link`
/// one is partial / unresolved — so an inferred or derived link can never read as an exact one. A
/// `no_proving_source` or `unknown_proving` proving state has no source form, so the banner can never
/// pretend to link to a proving source it does not have.
pub fn resolve_derived_relationship_posture(
    relationship_class: M5DerivedRelationshipClass,
    proving_state: M5RelationshipProvingState,
) -> DerivedRelationshipDisclosure {
    use DerivedCertaintyPosture as Certainty;
    use M5DerivedRelationshipClass as Class;
    use M5RelationshipProvingState as Proving;

    let certainty_posture = match relationship_class {
        Class::ExactFromSource => Certainty::ExactFromSource,
        Class::InferredFromRuntime => Certainty::RuntimeConfirmed,
        Class::HeuristicLink | Class::DerivedByConvention => Certainty::Heuristic,
        Class::PartialLink | Class::UnresolvedLink => Certainty::PartialOrUnresolved,
    };
    let has_source_form = !matches!(
        proving_state,
        Proving::NoProvingSource | Proving::UnknownProving
    );

    DerivedRelationshipDisclosure {
        certainty_posture,
        is_exact_from_source: certainty_posture.is_exact_from_source(),
        is_runtime_confirmed: certainty_posture.is_runtime_confirmed(),
        must_not_read_as_exact: certainty_posture.must_not_read_as_exact(),
        has_source_form,
        needs_heuristic_note: certainty_posture.needs_heuristic_note(),
        needs_partial_note: certainty_posture.needs_partial_note(),
        needs_no_source_form_note: !has_source_form,
    }
}

// ---- component structs --------------------------------------------------

/// A convention-diagnostic row naming its diagnostic class, affected entity / file, confidence /
/// severity, detected source, suggested fix / open-docs action, and support-class caveat, with a
/// derived certainty posture, a canonical proving-source link, and bounded open-proving-file /
/// inspect / fix actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticRow {
    /// Frozen component this control implements; must be `convention_diagnostic_row`.
    pub component: M5FrameworkComponentFamily,
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Diagnostic message label; required and non-empty.
    pub diagnostic_message_label: String,
    /// The distinct class of this diagnostic.
    pub diagnostic_class: DiagnosticClass,
    /// Affected entity label; always required so the affected entity stays explicit.
    pub affected_entity_label: String,
    /// Affected file label; always required so the proving file stays explicit.
    pub affected_file_label: String,
    /// Convention confidence class, reused from the frozen matrix.
    pub convention_confidence_class: M5ConventionConfidenceClass,
    /// Diagnostic severity, reused from the frozen matrix.
    pub diagnostic_severity: M5ConventionDiagnosticSeverity,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Derived certainty posture (must equal the resolved posture).
    pub derived_certainty_posture: DerivedCertaintyPosture,
    /// Whether the row claims exact-from-source (must equal derived truth).
    pub claims_exact_from_source: bool,
    /// Whether the row has a proving source form to prove (must equal derived truth).
    pub has_proving_source_form: bool,
    /// How the diagnostic was detected.
    pub detection_source: DetectionSource,
    /// Detected source label; always required so the detected source stays explicit.
    pub detected_source_label: String,
    /// The support-class caveat this row carries.
    pub support_caveat: SupportCaveatClass,
    /// Support-class caveat label; required when the caveat is anything other than fully supported.
    pub support_caveat_label: String,
    /// The suggested-fix affordance this row offers.
    pub fix_action: DiagnosticFixAction,
    /// Suggested-fix label; always required so the suggested fix / open-docs path stays explicit.
    pub suggested_fix_label: String,
    /// Heuristic note; required when the certainty posture is heuristic.
    pub heuristic_note: String,
    /// Partial note; required when the certainty posture is partial / unresolved.
    pub partial_note: String,
    /// No-source-form note; required when the row has no proving source form.
    pub no_source_form_note: String,
    /// Certainty and confidence note; always required so the row states both at row level.
    pub certainty_and_confidence_note: String,
    /// Kind of canonical proving source this row links its next step against.
    pub proving_source_kind: ProvingSourceLink,
    /// Opaque canonical proving-source reference; required when the kind resolves.
    pub proving_source_ref: String,
    /// Context note; always required so the row names what to check before trusting it.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub row_actions: Vec<DiagnosticRowAction>,
    /// Certainty dispositions this row binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this row can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never lets a heuristic suspicion masquerade as exact. MUST be `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: never collapses distinct diagnostics into one generic warning. MUST be
    /// `false`.
    pub collapses_distinct_diagnostics_into_generic_warning: bool,
    /// Hard invariant: never acts like a hidden parallel model without a proving source. MUST be
    /// `false`.
    pub acts_as_hidden_parallel_model: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ConventionDiagnosticRow {
    /// Certainty / proving-source disclosures this row must carry, derived from the frozen classes.
    pub fn posture_disclosure(&self) -> ConventionDiagnosticDisclosure {
        resolve_convention_diagnostic_posture(self.convention_confidence_class)
    }

    /// Whether the row offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<DiagnosticRowAction> = self.row_actions.iter().copied().collect();
        DiagnosticRowAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }
}

/// A derived-relationship banner naming its source of inference, its last refresh, its exact /
/// partial / heuristic / runtime-confirmed state, and its open-raw-source or open-wider-graph
/// actions, with a canonical proving-source link and a named place of consumption so an approximation
/// never hides in the background.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedRelationshipBanner {
    /// Frozen component this control implements; must be `derived_relationship_banner`.
    pub component: M5FrameworkComponentFamily,
    /// Stable banner id.
    pub banner_id: String,
    /// Relationship label; required and non-empty.
    pub relationship_label: String,
    /// Derived-relationship class, reused from the frozen matrix.
    pub derived_relationship_class: M5DerivedRelationshipClass,
    /// Relationship proving state, reused from the frozen matrix.
    pub relationship_proving_state: M5RelationshipProvingState,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Derived certainty posture (must equal the resolved posture).
    pub derived_certainty_posture: DerivedCertaintyPosture,
    /// Whether the banner claims exact-from-source (must equal derived truth).
    pub claims_exact_from_source: bool,
    /// Whether the banner has a proving source form to prove (must equal derived truth).
    pub has_proving_source_form: bool,
    /// The source of inference this banner names.
    pub inference_source: InferenceSource,
    /// Source-of-inference label; always required so the source of inference stays explicit.
    pub inference_source_label: String,
    /// The refresh state of this inference.
    pub refresh_state: RefreshState,
    /// Last-refresh label; always required so how current the inference is stays explicit.
    pub last_refresh_label: String,
    /// Consumed-context label; always required so the banner names where the inferred truth is
    /// consumed rather than hiding the approximation in the background.
    pub consumed_context_label: String,
    /// Heuristic note; required when the certainty posture is heuristic.
    pub heuristic_note: String,
    /// Partial note; required when the certainty posture is partial / unresolved.
    pub partial_note: String,
    /// No-source-form note; required when the banner has no proving source form.
    pub no_source_form_note: String,
    /// Certainty and state note; always required so the banner states both at banner level.
    pub certainty_and_state_note: String,
    /// Kind of canonical proving source this banner links its next step against.
    pub proving_source_kind: ProvingSourceLink,
    /// Opaque canonical proving-source reference; required when the kind resolves.
    pub proving_source_ref: String,
    /// Context note; always required so the banner names what to check before trusting it.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub banner_actions: Vec<BannerAction>,
    /// Certainty dispositions this banner binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this banner can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this banner can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this banner.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never lets a heuristic / inferred link masquerade as exact. MUST be `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: never hides the approximation in the background. MUST be `false`.
    pub hides_approximation_in_background: bool,
    /// Hard invariant: never acts like a hidden parallel model without a proving source. MUST be
    /// `false`.
    pub acts_as_hidden_parallel_model: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl DerivedRelationshipBanner {
    /// Certainty / proving-source disclosures this banner must carry, derived from the frozen
    /// classes.
    pub fn posture_disclosure(&self) -> DerivedRelationshipDisclosure {
        resolve_derived_relationship_posture(
            self.derived_relationship_class,
            self.relationship_proving_state,
        )
    }

    /// Whether the banner offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<BannerAction> = self.banner_actions.iter().copied().collect();
        BannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the banner declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }
}

/// Whether a required-label list declares all three mandatory labels.
fn declares_mandatory_labels(labels: &[M5FrameworkRequiredLabel]) -> bool {
    let present: BTreeSet<M5FrameworkRequiredLabel> = labels.iter().copied().collect();
    M5FrameworkRequiredLabel::MANDATORY
        .iter()
        .all(|label| present.contains(label))
}

// ---- review blocks ------------------------------------------------------

/// First-glance convention-diagnostic / derived-relationship review block; every flag is a hard
/// invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionRelationshipReview {
    /// The diagnostic row names its class and its affected entity / file.
    pub diagnostic_row_shows_class_and_entity: bool,
    /// The diagnostic row names its confidence and severity.
    pub diagnostic_row_shows_confidence_and_severity: bool,
    /// The diagnostic row offers an open-proving-file action.
    pub diagnostic_row_offers_proving_file: bool,
    /// The banner names its relationship and its source of inference.
    pub banner_shows_relationship_and_source: bool,
    /// The banner names its exact / partial / heuristic / runtime-confirmed state and last refresh.
    pub banner_shows_state_and_refresh: bool,
    /// The banner offers open-raw-source and open-wider-graph actions.
    pub banner_offers_raw_source_and_wider_graph: bool,
    /// Certainty is derived from state, never asserted.
    pub certainty_derived_never_asserted: bool,
    /// A heuristic diagnostic or inferred relationship is never shown as exact.
    pub heuristic_never_shown_as_exact: bool,
    /// Distinct diagnostics are never collapsed into one generic warning.
    pub distinct_diagnostics_never_collapsed: bool,
    /// An approximation is never hidden in the background.
    pub approximation_never_hidden_in_background: bool,
    /// The support-class caveat stays visible on the diagnostic row.
    pub support_class_caveat_always_visible: bool,
    /// Every row and banner links back to a canonical proving source rather than acting as a hidden
    /// parallel model.
    pub every_component_links_to_proving_source: bool,
    /// An ungrounded or unresolved component never pretends to link to a source it does not have.
    pub ungrounded_component_never_fakes_a_source: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ConventionRelationshipReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diagnostic_row_shows_class_and_entity
            && self.diagnostic_row_shows_confidence_and_severity
            && self.diagnostic_row_offers_proving_file
            && self.banner_shows_relationship_and_source
            && self.banner_shows_state_and_refresh
            && self.banner_offers_raw_source_and_wider_graph
            && self.certainty_derived_never_asserted
            && self.heuristic_never_shown_as_exact
            && self.distinct_diagnostics_never_collapsed
            && self.approximation_never_hidden_in_background
            && self.support_class_caveat_always_visible
            && self.every_component_links_to_proving_source
            && self.ungrounded_component_never_fakes_a_source
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionRelationshipConsumerProjection {
    /// The convention diagnostic-center surface reads a single canonical source.
    pub diagnostic_center_reads_single_source: bool,
    /// The editor-gutter surface reads a single canonical source.
    pub editor_gutter_reads_single_source: bool,
    /// The topology-explorer surface reads a single canonical source.
    pub topology_explorer_reads_single_source: bool,
    /// Certainty and support caveat are visible before a user trusts the diagnostic.
    pub certainty_and_caveat_visible_before_trust: bool,
    /// The banner appears exactly where inferred framework truth is consumed.
    pub banner_appears_where_inferred_truth_consumed: bool,
    /// The proving source is reachable before a user trusts the component.
    pub proving_source_reachable_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl ConventionRelationshipConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diagnostic_center_reads_single_source
            && self.editor_gutter_reads_single_source
            && self.topology_explorer_reads_single_source
            && self.certainty_and_caveat_visible_before_trust
            && self.banner_appears_where_inferred_truth_consumed
            && self.proving_source_reachable_before_trust
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionRelationshipProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ConventionDiagnosticDerivedRelationshipControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConventionDiagnosticDerivedRelationshipControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Convention-diagnostic rows.
    pub diagnostic_rows: Vec<ConventionDiagnosticRow>,
    /// Derived-relationship banners.
    pub relationship_banners: Vec<DerivedRelationshipBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Convention / relationship review block.
    pub convention_relationship_review: ConventionRelationshipReview,
    /// Consumer projection block.
    pub consumer_projection: ConventionRelationshipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ConventionRelationshipProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe convention-diagnostic-row / derived-relationship-banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticDerivedRelationshipControlsPacket {
    /// Record kind; must equal [`CONVENTION_RELATIONSHIP_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Convention-diagnostic rows.
    pub diagnostic_rows: Vec<ConventionDiagnosticRow>,
    /// Derived-relationship banners.
    pub relationship_banners: Vec<DerivedRelationshipBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Convention / relationship review block.
    pub convention_relationship_review: ConventionRelationshipReview,
    /// Consumer projection block.
    pub consumer_projection: ConventionRelationshipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ConventionRelationshipProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ConventionDiagnosticDerivedRelationshipControlsPacket {
    /// Builds a convention-diagnostic-row / derived-relationship-banner controls packet from
    /// stable-lane input.
    pub fn new(input: ConventionDiagnosticDerivedRelationshipControlsPacketInput) -> Self {
        Self {
            record_kind: CONVENTION_RELATIONSHIP_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            diagnostic_rows: input.diagnostic_rows,
            relationship_banners: input.relationship_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            convention_relationship_review: input.convention_relationship_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the convention-diagnostic-row / derived-relationship-banner control invariants.
    pub fn validate(&self) -> Vec<ConventionRelationshipControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CONVENTION_RELATIONSHIP_CONTROLS_RECORD_KIND {
            violations.push(ConventionRelationshipControlsViolation::WrongRecordKind);
        }
        if self.schema_version != CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_VERSION {
            violations.push(ConventionRelationshipControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ConventionRelationshipControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ConventionRelationshipControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ConventionRelationshipControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_diagnostic_rows(self, &mut violations);
        validate_relationship_banners(self, &mut violations);

        if !self.convention_relationship_review.all_hold() {
            violations.push(ConventionRelationshipControlsViolation::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ConventionRelationshipControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ConventionRelationshipControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("convention relationship controls packet serializes"),
        ) {
            violations.push(ConventionRelationshipControlsViolation::RawMaterialInExport);
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
            .expect("convention relationship controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,primary_class,secondary,certainty_posture,exact_from_source,proving_source_kind\n",
        );
        for row in &self.diagnostic_rows {
            let disclosure = row.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "convention_diagnostic_row",
                csv_field(&row.diagnostic_id),
                row.diagnostic_class.as_str(),
                row.convention_confidence_class.as_str(),
                disclosure.certainty_posture.as_str(),
                disclosure.is_exact_from_source,
                row.proving_source_kind.as_str(),
            ));
        }
        for banner in &self.relationship_banners {
            let disclosure = banner.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "derived_relationship_banner",
                csv_field(&banner.banner_id),
                banner.derived_relationship_class.as_str(),
                banner.relationship_proving_state.as_str(),
                disclosure.certainty_posture.as_str(),
                disclosure.is_exact_from_source,
                banner.proving_source_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let heuristic_rows = self
            .diagnostic_rows
            .iter()
            .filter(|row| row.posture_disclosure().must_not_read_as_exact)
            .count();
        let heuristic_banners = self
            .relationship_banners
            .iter()
            .filter(|banner| banner.posture_disclosure().must_not_read_as_exact)
            .count();

        let mut out = String::new();
        out.push_str("# Convention-diagnostic rows and derived-relationship banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Convention-diagnostic rows: {} ({} heuristic or partial)\n",
            self.diagnostic_rows.len(),
            heuristic_rows
        ));
        out.push_str(&format!(
            "- Derived-relationship banners: {} ({} heuristic or partial)\n",
            self.relationship_banners.len(),
            heuristic_banners
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Convention-diagnostic rows\n\n");
        for row in &self.diagnostic_rows {
            let disclosure = row.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — class `{}`, confidence `{}`, severity `{}`, certainty `{}`, caveat `{}`, proving source `{}`\n",
                row.diagnostic_message_label,
                row.diagnostic_class.as_str(),
                row.convention_confidence_class.as_str(),
                row.diagnostic_severity.as_str(),
                disclosure.certainty_posture.as_str(),
                row.support_caveat.as_str(),
                row.proving_source_kind.as_str(),
            ));
        }

        out.push_str("\n## Derived-relationship banners\n\n");
        for banner in &self.relationship_banners {
            let disclosure = banner.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — class `{}`, proving `{}`, certainty `{}`, inference `{}`, refresh `{}`, proving source `{}`\n",
                banner.relationship_label,
                banner.derived_relationship_class.as_str(),
                banner.relationship_proving_state.as_str(),
                disclosure.certainty_posture.as_str(),
                banner.inference_source.as_str(),
                banner.refresh_state.as_str(),
                banner.proving_source_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in convention-relationship controls export.
#[derive(Debug)]
pub enum ConventionRelationshipControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ConventionRelationshipControlsViolation>),
}

impl fmt::Display for ConventionRelationshipControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "convention relationship controls export parse failed: {error}"
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
                    "convention relationship controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ConventionRelationshipControlsArtifactError {}

/// Validation failures emitted by
/// [`ConventionDiagnosticDerivedRelationshipControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConventionRelationshipControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No convention-diagnostic rows are present.
    DiagnosticRowsMissing,
    /// A convention-diagnostic row is incomplete.
    DiagnosticRowIncomplete,
    /// A convention-diagnostic row carries the wrong frozen component class.
    DiagnosticRowWrongComponentClass,
    /// A convention-diagnostic row misrepresents its derived certainty posture or claims.
    DiagnosticPostureMisrepresented,
    /// No derived-relationship banners are present.
    BannersMissing,
    /// A derived-relationship banner is incomplete.
    BannerIncomplete,
    /// A derived-relationship banner carries the wrong frozen component class.
    BannerWrongComponentClass,
    /// A derived-relationship banner misrepresents its derived certainty posture or claims.
    RelationshipPostureMisrepresented,
    /// A heuristic or partial component claims exact-from-source.
    HeuristicClaimsExact,
    /// A heuristic component does not name its heuristic basis.
    HeuristicNoteMissing,
    /// A partial / unresolved component does not name its partial basis.
    PartialNoteMissing,
    /// A component with no source form does not name why it has no proving source.
    NoSourceFormNoteMissing,
    /// A component claims a resolvable proving source but has no source form.
    ProvingSourceClaimedWithoutForm,
    /// A component with a source form does not link to a resolvable proving source.
    ProvingSourceUnresolved,
    /// A component names a resolvable proving-source kind but not its reference.
    ProvingSourceRefMissing,
    /// A diagnostic row does not name its affected entity or file.
    AffectedEntityOrFileMissing,
    /// A diagnostic row does not name its detected source.
    DetectedSourceMissing,
    /// A diagnostic row does not name its support-class caveat.
    SupportCaveatMissing,
    /// A diagnostic row does not name its suggested fix.
    SuggestedFixMissing,
    /// A banner does not name its source of inference.
    InferenceSourceMissing,
    /// A banner does not name its last refresh.
    RefreshLabelMissing,
    /// A banner does not name where its inferred truth is consumed.
    ConsumedContextMissing,
    /// A banner does not name its relationship.
    RelationshipLabelMissing,
    /// A component does not name its certainty / confidence / state at row level.
    RowLevelStateNoteMissing,
    /// The diagnostic rows do not cover every convention confidence class.
    ConventionConfidenceCoverageMissing,
    /// The diagnostic rows do not cover every diagnostic severity.
    DiagnosticSeverityCoverageMissing,
    /// The diagnostic rows do not cover every diagnostic class.
    DiagnosticClassCoverageMissing,
    /// The diagnostic rows do not cover every detection source.
    DetectionSourceCoverageMissing,
    /// The diagnostic rows do not cover every support-caveat class.
    SupportCaveatCoverageMissing,
    /// The banners do not cover every derived-relationship class.
    DerivedRelationshipCoverageMissing,
    /// The banners do not cover every relationship proving state.
    RelationshipProvingCoverageMissing,
    /// The banners do not cover every inference source.
    InferenceSourceCoverageMissing,
    /// The banners do not cover every refresh state.
    RefreshStateCoverageMissing,
    /// The components do not cover every derived certainty posture.
    CertaintyPostureCoverageMissing,
    /// The components do not cover every proving-source link kind.
    ProvingSourceLinkCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A diagnostic row omits a mandatory action.
    DiagnosticRowActionsIncomplete,
    /// A banner omits a mandatory action.
    BannerActionsIncomplete,
    /// A component does not bind any certainty disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component lets a heuristic diagnostic or link masquerade as exact.
    HeuristicMasqueradesAsExact,
    /// A diagnostic row collapses distinct diagnostics into one generic warning.
    DistinctDiagnosticsCollapsed,
    /// A banner hides the approximation in the background.
    ApproximationHiddenInBackground,
    /// A component acts like a hidden parallel model without a proving source.
    HiddenParallelModel,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Convention / relationship review does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ConventionRelationshipControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DiagnosticRowsMissing => "diagnostic_rows_missing",
            Self::DiagnosticRowIncomplete => "diagnostic_row_incomplete",
            Self::DiagnosticRowWrongComponentClass => "diagnostic_row_wrong_component_class",
            Self::DiagnosticPostureMisrepresented => "diagnostic_posture_misrepresented",
            Self::BannersMissing => "banners_missing",
            Self::BannerIncomplete => "banner_incomplete",
            Self::BannerWrongComponentClass => "banner_wrong_component_class",
            Self::RelationshipPostureMisrepresented => "relationship_posture_misrepresented",
            Self::HeuristicClaimsExact => "heuristic_claims_exact",
            Self::HeuristicNoteMissing => "heuristic_note_missing",
            Self::PartialNoteMissing => "partial_note_missing",
            Self::NoSourceFormNoteMissing => "no_source_form_note_missing",
            Self::ProvingSourceClaimedWithoutForm => "proving_source_claimed_without_form",
            Self::ProvingSourceUnresolved => "proving_source_unresolved",
            Self::ProvingSourceRefMissing => "proving_source_ref_missing",
            Self::AffectedEntityOrFileMissing => "affected_entity_or_file_missing",
            Self::DetectedSourceMissing => "detected_source_missing",
            Self::SupportCaveatMissing => "support_caveat_missing",
            Self::SuggestedFixMissing => "suggested_fix_missing",
            Self::InferenceSourceMissing => "inference_source_missing",
            Self::RefreshLabelMissing => "refresh_label_missing",
            Self::ConsumedContextMissing => "consumed_context_missing",
            Self::RelationshipLabelMissing => "relationship_label_missing",
            Self::RowLevelStateNoteMissing => "row_level_state_note_missing",
            Self::ConventionConfidenceCoverageMissing => "convention_confidence_coverage_missing",
            Self::DiagnosticSeverityCoverageMissing => "diagnostic_severity_coverage_missing",
            Self::DiagnosticClassCoverageMissing => "diagnostic_class_coverage_missing",
            Self::DetectionSourceCoverageMissing => "detection_source_coverage_missing",
            Self::SupportCaveatCoverageMissing => "support_caveat_coverage_missing",
            Self::DerivedRelationshipCoverageMissing => "derived_relationship_coverage_missing",
            Self::RelationshipProvingCoverageMissing => "relationship_proving_coverage_missing",
            Self::InferenceSourceCoverageMissing => "inference_source_coverage_missing",
            Self::RefreshStateCoverageMissing => "refresh_state_coverage_missing",
            Self::CertaintyPostureCoverageMissing => "certainty_posture_coverage_missing",
            Self::ProvingSourceLinkCoverageMissing => "proving_source_link_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DiagnosticRowActionsIncomplete => "diagnostic_row_actions_incomplete",
            Self::BannerActionsIncomplete => "banner_actions_incomplete",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::HeuristicMasqueradesAsExact => "heuristic_masquerades_as_exact",
            Self::DistinctDiagnosticsCollapsed => "distinct_diagnostics_collapsed",
            Self::ApproximationHiddenInBackground => "approximation_hidden_in_background",
            Self::HiddenParallelModel => "hidden_parallel_model",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable convention-relationship controls export.
///
/// This is the first real consumer of the convention-diagnostic-row / derived-relationship-banner
/// component lane: a convention-diagnostics, editor-gutter, topology-explorer, or support-export
/// surface calls it to ingest the canonical components rather than cloning row text.
///
/// # Errors
///
/// Returns [`ConventionRelationshipControlsArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_convention_relationship_controls_export() -> Result<
    ConventionDiagnosticDerivedRelationshipControlsPacket,
    ConventionRelationshipControlsArtifactError,
> {
    let packet: ConventionDiagnosticDerivedRelationshipControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-convention-diagnostic-derived-relationship-proof/support_export.json"
        )))
        .map_err(ConventionRelationshipControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ConventionRelationshipControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ConventionDiagnosticDerivedRelationshipControlsPacket,
    violations: &mut Vec<ConventionRelationshipControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF,
        CONVENTION_RELATIONSHIP_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF,
        M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ConventionRelationshipControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    lets_heuristic_masquerade_as_exact: bool,
    hides_secondary_axis: bool,
    acts_as_hidden_parallel_model: bool,
    invents_alternate_state_label: bool,
    /// The violation to emit when `hides_secondary_axis` is set — family-specific.
    hidden_secondary_violation: ConventionRelationshipControlsViolation,
}

/// Validates the certainty / exact-claim cross-checks and the proving-source truth shared by both
/// component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_evidence(
    certainty_posture: DerivedCertaintyPosture,
    is_exact_from_source: bool,
    claims_exact_from_source: bool,
    has_source_form_derived: bool,
    has_proving_source_form: bool,
    needs_heuristic_note: bool,
    heuristic_note: &str,
    needs_partial_note: bool,
    partial_note: &str,
    proving_source_kind: ProvingSourceLink,
    proving_source_ref: &str,
    misrepresented_violation: ConventionRelationshipControlsViolation,
    violations: &mut Vec<ConventionRelationshipControlsViolation>,
) {
    if is_exact_from_source != claims_exact_from_source
        || has_source_form_derived != has_proving_source_form
    {
        violations.push(misrepresented_violation);
    }
    if certainty_posture.must_not_read_as_exact() && claims_exact_from_source {
        violations.push(ConventionRelationshipControlsViolation::HeuristicClaimsExact);
    }
    if needs_heuristic_note && heuristic_note.trim().is_empty() {
        violations.push(ConventionRelationshipControlsViolation::HeuristicNoteMissing);
    }
    if needs_partial_note && partial_note.trim().is_empty() {
        violations.push(ConventionRelationshipControlsViolation::PartialNoteMissing);
    }
    // Proving-source truth: a component with a source form must link to a resolvable proving
    // source; a component with no source form must not claim one.
    if has_proving_source_form && !proving_source_kind.is_resolvable() {
        violations.push(ConventionRelationshipControlsViolation::ProvingSourceUnresolved);
    }
    if !has_proving_source_form && proving_source_kind.is_resolvable() {
        violations.push(ConventionRelationshipControlsViolation::ProvingSourceClaimedWithoutForm);
    }
    if proving_source_kind.is_resolvable() && proving_source_ref.trim().is_empty() {
        violations.push(ConventionRelationshipControlsViolation::ProvingSourceRefMissing);
    }
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5FrameworkCertaintyDisposition],
    downgrade_triggers: &[M5FrameworkDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5FrameworkAccessibilityRoute],
    context_note: &str,
    invariants: ControlInvariants,
    violations: &mut Vec<ConventionRelationshipControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ConventionRelationshipControlsViolation::ContextNoteMissing);
    }
    if dispositions.is_empty() {
        violations.push(ConventionRelationshipControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ConventionRelationshipControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(ConventionRelationshipControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(ConventionRelationshipControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.lets_heuristic_masquerade_as_exact {
        violations.push(ConventionRelationshipControlsViolation::HeuristicMasqueradesAsExact);
    }
    if invariants.hides_secondary_axis {
        violations.push(invariants.hidden_secondary_violation);
    }
    if invariants.acts_as_hidden_parallel_model {
        violations.push(ConventionRelationshipControlsViolation::HiddenParallelModel);
    }
    if invariants.invents_alternate_state_label {
        violations.push(ConventionRelationshipControlsViolation::AlternateStateLabelInvented);
    }
}

fn validate_diagnostic_rows(
    packet: &ConventionDiagnosticDerivedRelationshipControlsPacket,
    violations: &mut Vec<ConventionRelationshipControlsViolation>,
) {
    if packet.diagnostic_rows.is_empty() {
        violations.push(ConventionRelationshipControlsViolation::DiagnosticRowsMissing);
        return;
    }

    let mut confidence: BTreeSet<M5ConventionConfidenceClass> = BTreeSet::new();
    let mut severity: BTreeSet<M5ConventionDiagnosticSeverity> = BTreeSet::new();
    let mut classes: BTreeSet<DiagnosticClass> = BTreeSet::new();
    let mut detection: BTreeSet<DetectionSource> = BTreeSet::new();
    let mut caveats: BTreeSet<SupportCaveatClass> = BTreeSet::new();

    for row in &packet.diagnostic_rows {
        let disclosure = row.posture_disclosure();
        confidence.insert(row.convention_confidence_class);
        severity.insert(row.diagnostic_severity);
        classes.insert(row.diagnostic_class);
        detection.insert(row.detection_source);
        caveats.insert(row.support_caveat);

        if row.diagnostic_id.trim().is_empty()
            || row.diagnostic_message_label.trim().is_empty()
            || row.detected_source_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ConventionRelationshipControlsViolation::DiagnosticRowIncomplete);
        }
        if row.component != M5FrameworkComponentFamily::ConventionDiagnosticRow {
            violations
                .push(ConventionRelationshipControlsViolation::DiagnosticRowWrongComponentClass);
        }
        if row.derived_certainty_posture != disclosure.certainty_posture {
            violations
                .push(ConventionRelationshipControlsViolation::DiagnosticPostureMisrepresented);
        }
        validate_shared_evidence(
            disclosure.certainty_posture,
            disclosure.is_exact_from_source,
            row.claims_exact_from_source,
            disclosure.has_source_form,
            row.has_proving_source_form,
            disclosure.needs_heuristic_note,
            &row.heuristic_note,
            disclosure.needs_partial_note,
            &row.partial_note,
            row.proving_source_kind,
            &row.proving_source_ref,
            ConventionRelationshipControlsViolation::DiagnosticPostureMisrepresented,
            violations,
        );
        if disclosure.needs_no_source_form_note && row.no_source_form_note.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::NoSourceFormNoteMissing);
        }
        if row.affected_entity_label.trim().is_empty() || row.affected_file_label.trim().is_empty()
        {
            violations.push(ConventionRelationshipControlsViolation::AffectedEntityOrFileMissing);
        }
        if row.detected_source_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::DetectedSourceMissing);
        }
        if row.support_caveat.needs_caveat_label() && row.support_caveat_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::SupportCaveatMissing);
        }
        if row.suggested_fix_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::SuggestedFixMissing);
        }
        if row.certainty_and_confidence_note.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::RowLevelStateNoteMissing);
        }
        if !row.declares_mandatory_actions() {
            violations
                .push(ConventionRelationshipControlsViolation::DiagnosticRowActionsIncomplete);
        }
        validate_common_control(
            &row.dispositions,
            &row.downgrade_triggers,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            &row.context_note,
            ControlInvariants {
                lets_heuristic_masquerade_as_exact: row.lets_heuristic_masquerade_as_exact,
                hides_secondary_axis: row.collapses_distinct_diagnostics_into_generic_warning,
                acts_as_hidden_parallel_model: row.acts_as_hidden_parallel_model,
                invents_alternate_state_label: row.invents_alternate_state_label,
                hidden_secondary_violation:
                    ConventionRelationshipControlsViolation::DistinctDiagnosticsCollapsed,
            },
            violations,
        );
    }

    for required in M5ConventionConfidenceClass::ALL {
        if !confidence.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::ConventionConfidenceCoverageMissing);
            break;
        }
    }
    for required in M5ConventionDiagnosticSeverity::ALL {
        if !severity.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::DiagnosticSeverityCoverageMissing);
            break;
        }
    }
    for required in DiagnosticClass::ALL {
        if !classes.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::DiagnosticClassCoverageMissing);
            break;
        }
    }
    for required in DetectionSource::ALL {
        if !detection.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::DetectionSourceCoverageMissing);
            break;
        }
    }
    for required in SupportCaveatClass::ALL {
        if !caveats.contains(&required) {
            violations.push(ConventionRelationshipControlsViolation::SupportCaveatCoverageMissing);
            break;
        }
    }

    validate_shared_coverage(packet, violations);
}

fn validate_relationship_banners(
    packet: &ConventionDiagnosticDerivedRelationshipControlsPacket,
    violations: &mut Vec<ConventionRelationshipControlsViolation>,
) {
    if packet.relationship_banners.is_empty() {
        violations.push(ConventionRelationshipControlsViolation::BannersMissing);
        return;
    }

    let mut classes: BTreeSet<M5DerivedRelationshipClass> = BTreeSet::new();
    let mut proving: BTreeSet<M5RelationshipProvingState> = BTreeSet::new();
    let mut inference: BTreeSet<InferenceSource> = BTreeSet::new();
    let mut refresh: BTreeSet<RefreshState> = BTreeSet::new();

    for banner in &packet.relationship_banners {
        let disclosure = banner.posture_disclosure();
        classes.insert(banner.derived_relationship_class);
        proving.insert(banner.relationship_proving_state);
        inference.insert(banner.inference_source);
        refresh.insert(banner.refresh_state);

        if banner.banner_id.trim().is_empty()
            || banner.relationship_label.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations.push(ConventionRelationshipControlsViolation::BannerIncomplete);
        }
        if banner.component != M5FrameworkComponentFamily::DerivedRelationshipBanner {
            violations.push(ConventionRelationshipControlsViolation::BannerWrongComponentClass);
        }
        if banner.derived_certainty_posture != disclosure.certainty_posture {
            violations
                .push(ConventionRelationshipControlsViolation::RelationshipPostureMisrepresented);
        }
        validate_shared_evidence(
            disclosure.certainty_posture,
            disclosure.is_exact_from_source,
            banner.claims_exact_from_source,
            disclosure.has_source_form,
            banner.has_proving_source_form,
            disclosure.needs_heuristic_note,
            &banner.heuristic_note,
            disclosure.needs_partial_note,
            &banner.partial_note,
            banner.proving_source_kind,
            &banner.proving_source_ref,
            ConventionRelationshipControlsViolation::RelationshipPostureMisrepresented,
            violations,
        );
        if disclosure.needs_no_source_form_note && banner.no_source_form_note.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::NoSourceFormNoteMissing);
        }
        if banner.relationship_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::RelationshipLabelMissing);
        }
        if banner.inference_source_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::InferenceSourceMissing);
        }
        if banner.last_refresh_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::RefreshLabelMissing);
        }
        if banner.consumed_context_label.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::ConsumedContextMissing);
        }
        if banner.certainty_and_state_note.trim().is_empty() {
            violations.push(ConventionRelationshipControlsViolation::RowLevelStateNoteMissing);
        }
        if !banner.declares_mandatory_actions() {
            violations.push(ConventionRelationshipControlsViolation::BannerActionsIncomplete);
        }
        validate_common_control(
            &banner.dispositions,
            &banner.downgrade_triggers,
            banner.declares_mandatory_labels(),
            &banner.accessibility_routes,
            &banner.context_note,
            ControlInvariants {
                lets_heuristic_masquerade_as_exact: banner.lets_heuristic_masquerade_as_exact,
                hides_secondary_axis: banner.hides_approximation_in_background,
                acts_as_hidden_parallel_model: banner.acts_as_hidden_parallel_model,
                invents_alternate_state_label: banner.invents_alternate_state_label,
                hidden_secondary_violation:
                    ConventionRelationshipControlsViolation::ApproximationHiddenInBackground,
            },
            violations,
        );
    }

    for required in M5DerivedRelationshipClass::ALL {
        if !classes.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::DerivedRelationshipCoverageMissing);
            break;
        }
    }
    for required in M5RelationshipProvingState::ALL {
        if !proving.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::RelationshipProvingCoverageMissing);
            break;
        }
    }
    for required in InferenceSource::ALL {
        if !inference.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::InferenceSourceCoverageMissing);
            break;
        }
    }
    for required in RefreshState::ALL {
        if !refresh.contains(&required) {
            violations.push(ConventionRelationshipControlsViolation::RefreshStateCoverageMissing);
            break;
        }
    }
}

/// Validates that the union of both component vectors covers every derived certainty posture and
/// proving-source link kind the acceptance criteria pin.
fn validate_shared_coverage(
    packet: &ConventionDiagnosticDerivedRelationshipControlsPacket,
    violations: &mut Vec<ConventionRelationshipControlsViolation>,
) {
    let mut postures: BTreeSet<DerivedCertaintyPosture> = BTreeSet::new();
    let mut links: BTreeSet<ProvingSourceLink> = BTreeSet::new();

    for row in &packet.diagnostic_rows {
        postures.insert(row.posture_disclosure().certainty_posture);
        links.insert(row.proving_source_kind);
    }
    for banner in &packet.relationship_banners {
        postures.insert(banner.posture_disclosure().certainty_posture);
        links.insert(banner.proving_source_kind);
    }

    for required in DerivedCertaintyPosture::ALL {
        if !postures.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::CertaintyPostureCoverageMissing);
            break;
        }
    }
    for required in ProvingSourceLink::ALL {
        if !links.contains(&required) {
            violations
                .push(ConventionRelationshipControlsViolation::ProvingSourceLinkCoverageMissing);
            break;
        }
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the scenario
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// components, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical convention-relationship controls packet.
pub const CONVENTION_RELATIONSHIP_CONTROLS_PACKET_ID: &str =
    "m5-convention-diagnostic-derived-relationship-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn diagnostic_source_refs() -> Vec<String> {
    strings(&[
        M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn banner_source_refs() -> Vec<String> {
    strings(&[
        M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn diagnostic_row_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::ConventionConfidenceOverstated,
        M5FrameworkDowngradeTrigger::SupportClassUnstated,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn banner_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::DerivedStateUnlabeled,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

/// The three mandatory labels plus one extra truth label.
fn label_set(extra: M5FrameworkRequiredLabel) -> Vec<M5FrameworkRequiredLabel> {
    let mut labels = M5FrameworkRequiredLabel::MANDATORY.to_vec();
    labels.push(extra);
    labels
}

/// Returns `text` when `needed`, else an empty string.
fn note_if(needed: bool, text: &str) -> String {
    if needed {
        text.to_owned()
    } else {
        String::new()
    }
}

/// Builds a convention-diagnostic row, deriving the certainty posture, exact claim, source form, and
/// required notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn diagnostic_row(
    diagnostic_id: &str,
    diagnostic_message_label: &str,
    diagnostic_class: DiagnosticClass,
    affected_entity_label: &str,
    affected_file_label: &str,
    convention_confidence_class: M5ConventionConfidenceClass,
    diagnostic_severity: M5ConventionDiagnosticSeverity,
    certainty: M5FrameworkCertaintyDisposition,
    detection_source: DetectionSource,
    detected_source_label: &str,
    support_caveat: SupportCaveatClass,
    support_caveat_label: &str,
    fix_action: DiagnosticFixAction,
    suggested_fix_label: &str,
    context_note: &str,
    proving_source_kind: ProvingSourceLink,
    proving_source_ref: &str,
    row_actions: Vec<DiagnosticRowAction>,
) -> ConventionDiagnosticRow {
    let disclosure = resolve_convention_diagnostic_posture(convention_confidence_class);
    ConventionDiagnosticRow {
        component: M5FrameworkComponentFamily::ConventionDiagnosticRow,
        diagnostic_id: diagnostic_id.to_owned(),
        diagnostic_message_label: diagnostic_message_label.to_owned(),
        diagnostic_class,
        affected_entity_label: affected_entity_label.to_owned(),
        affected_file_label: affected_file_label.to_owned(),
        convention_confidence_class,
        diagnostic_severity,
        certainty,
        derived_certainty_posture: disclosure.certainty_posture,
        claims_exact_from_source: disclosure.is_exact_from_source,
        has_proving_source_form: disclosure.has_source_form,
        detection_source,
        detected_source_label: detected_source_label.to_owned(),
        support_caveat,
        support_caveat_label: support_caveat_label.to_owned(),
        fix_action,
        suggested_fix_label: suggested_fix_label.to_owned(),
        heuristic_note: note_if(
            disclosure.needs_heuristic_note,
            "Diagnostic is a heuristic suspicion; treat it as a guess, not an exact contract fact",
        ),
        partial_note: note_if(
            disclosure.needs_partial_note,
            "Diagnostic confidence is only partial or unresolved; do not treat it as certain",
        ),
        no_source_form_note: note_if(
            disclosure.needs_no_source_form_note,
            "Diagnostic is ungrounded and of unknown confidence, so no proving file exists",
        ),
        certainty_and_confidence_note: format!(
            "Certainty {}; confidence {}",
            disclosure.certainty_posture.as_str(),
            convention_confidence_class.as_str()
        ),
        proving_source_kind,
        proving_source_ref: proving_source_ref.to_owned(),
        context_note: context_note.to_owned(),
        row_actions,
        dispositions: vec![certainty],
        downgrade_triggers: diagnostic_row_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::PackSourceAndCertainty),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "diagnostic_message_label",
            "diagnostic_class",
            "affected_entity_label",
            "affected_file_label",
            "convention_confidence_class",
            "diagnostic_severity",
            "detected_source_label",
            "support_caveat",
            "suggested_fix_label",
            "proving_source_kind",
        ]),
        source_contract_refs: diagnostic_source_refs(),
        lets_heuristic_masquerade_as_exact: false,
        collapses_distinct_diagnostics_into_generic_warning: false,
        acts_as_hidden_parallel_model: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a derived-relationship banner, deriving the certainty posture, exact claim, source form,
/// and required notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn relationship_banner(
    banner_id: &str,
    relationship_label: &str,
    derived_relationship_class: M5DerivedRelationshipClass,
    relationship_proving_state: M5RelationshipProvingState,
    certainty: M5FrameworkCertaintyDisposition,
    inference_source: InferenceSource,
    inference_source_label: &str,
    refresh_state: RefreshState,
    last_refresh_label: &str,
    consumed_context_label: &str,
    context_note: &str,
    proving_source_kind: ProvingSourceLink,
    proving_source_ref: &str,
    banner_actions: Vec<BannerAction>,
) -> DerivedRelationshipBanner {
    let disclosure = resolve_derived_relationship_posture(
        derived_relationship_class,
        relationship_proving_state,
    );
    DerivedRelationshipBanner {
        component: M5FrameworkComponentFamily::DerivedRelationshipBanner,
        banner_id: banner_id.to_owned(),
        relationship_label: relationship_label.to_owned(),
        derived_relationship_class,
        relationship_proving_state,
        certainty,
        derived_certainty_posture: disclosure.certainty_posture,
        claims_exact_from_source: disclosure.is_exact_from_source,
        has_proving_source_form: disclosure.has_source_form,
        inference_source,
        inference_source_label: inference_source_label.to_owned(),
        refresh_state,
        last_refresh_label: last_refresh_label.to_owned(),
        consumed_context_label: consumed_context_label.to_owned(),
        heuristic_note: note_if(
            disclosure.needs_heuristic_note,
            "Relationship is inferred by a heuristic; treat it as a guess, not an exact fact",
        ),
        partial_note: note_if(
            disclosure.needs_partial_note,
            "Relationship evidence is only partial or unresolved; do not treat the link as complete",
        ),
        no_source_form_note: note_if(
            disclosure.needs_no_source_form_note,
            "Relationship has no proving source; it is unresolved or unknown, so none can be opened",
        ),
        certainty_and_state_note: format!(
            "Certainty {}; relationship {}",
            disclosure.certainty_posture.as_str(),
            derived_relationship_class.as_str()
        ),
        proving_source_kind,
        proving_source_ref: proving_source_ref.to_owned(),
        context_note: context_note.to_owned(),
        banner_actions,
        dispositions: vec![certainty],
        downgrade_triggers: banner_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::ProvingSourceAndRecoveryBoundary),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "relationship_label",
            "derived_relationship_class",
            "relationship_proving_state",
            "inference_source_label",
            "last_refresh_label",
            "consumed_context_label",
            "proving_source_kind",
        ]),
        source_contract_refs: banner_source_refs(),
        lets_heuristic_masquerade_as_exact: false,
        hides_approximation_in_background: false,
        acts_as_hidden_parallel_model: false,
        invents_alternate_state_label: false,
    }
}

fn diagnostic_rows() -> Vec<ConventionDiagnosticRow> {
    use DetectionSource as Detect;
    use DiagnosticClass as Class;
    use DiagnosticFixAction as Fix;
    use DiagnosticRowAction as Action;
    use M5ConventionConfidenceClass as Confidence;
    use M5ConventionDiagnosticSeverity as Severity;
    use M5FrameworkCertaintyDisposition as Certainty;
    use ProvingSourceLink as Link;
    use SupportCaveatClass as Caveat;

    vec![
        // 1. Verified / error / hard contract violation → exact, fully supported, source file.
        diagnostic_row(
            "diag-missing-loader",
            "Route file is missing its required loader export",
            Class::HardContractViolation,
            "app/routes/users.tsx",
            "app/routes/users.tsx",
            Confidence::Verified,
            Severity::Error,
            Certainty::Verified,
            Detect::FrameworkContract,
            "Checked against the framework loader contract",
            Caveat::FullySupported,
            "Fully supported first-party contract check",
            Fix::AutoFixAvailable,
            "Add the missing loader export (auto-fix available)",
            "Verified hard contract violation; open the proving file before trusting the fix",
            Link::SourceFile,
            "src:app/routes/users.tsx",
            vec![
                Action::OpenProvingFile,
                Action::InspectClassAndConfidence,
                Action::OpenDocsOrApplyFix,
                Action::CopyDiagnosticId,
            ],
        ),
        // 2. High confidence / warning / version mismatch → heuristic, version-mismatch caveat,
        //    source symbol.
        diagnostic_row(
            "diag-version-mismatch",
            "Component uses an API removed in the pinned framework version",
            Class::VersionMismatch,
            "CartView",
            "app/components/cart_view.tsx",
            Confidence::HighConfidence,
            Severity::Warning,
            Certainty::FrameworkPack,
            Detect::PackManifest,
            "Compared against the pinned framework pack manifest",
            Caveat::VersionMismatch,
            "Depends on a mismatched framework version",
            Fix::ManualFixGuidance,
            "Migrate to the supported API (manual guidance)",
            "High-confidence version mismatch; not a verified contract fact, so do not read as exact",
            Link::SourceSymbol,
            "symbol:CartView",
            vec![
                Action::OpenProvingFile,
                Action::InspectClassAndConfidence,
                Action::OpenDocsOrApplyFix,
                Action::CopyDiagnosticId,
            ],
        ),
        // 3. Heuristic convention / hint / heuristic suspicion → heuristic, heuristic-only caveat,
        //    docs anchor.
        diagnostic_row(
            "diag-naming-suspicion",
            "Handler name does not follow the framework naming convention",
            Class::HeuristicSuspicion,
            "handleSubmit",
            "app/routes/checkout.tsx",
            Confidence::HeuristicConvention,
            Severity::Hint,
            Certainty::HeuristicConvention,
            Detect::HeuristicScan,
            "Inferred from a naming-convention scan",
            Caveat::HeuristicOnly,
            "Produced by a heuristic only",
            Fix::OpenDocsOnly,
            "Read the naming-convention docs (no auto-fix)",
            "Heuristic suspicion from a naming scan; treat it as a guess, not a confirmed problem",
            Link::DocsAnchor,
            "docs:frameworks/naming-conventions",
            vec![
                Action::OpenProvingFile,
                Action::InspectClassAndConfidence,
                Action::OpenDocsOrApplyFix,
                Action::OpenReference,
            ],
        ),
        // 4. Derived by convention / info / pack limitation → heuristic, pack-limited caveat,
        //    source file.
        diagnostic_row(
            "diag-pack-limitation",
            "Pack cannot analyze this dynamic route; a convention is assumed",
            Class::PackLimitation,
            "app/routes/[slug].tsx",
            "app/routes/[slug].tsx",
            Confidence::DerivedByConvention,
            Severity::Info,
            Certainty::DerivedByConvention,
            Detect::StaticAnalysis,
            "Assumed by the routing convention after static analysis",
            Caveat::PackLimited,
            "Limited by the active framework pack",
            Fix::ManualFixGuidance,
            "Confirm the dynamic route manually (pack cannot verify)",
            "Pack limitation with a derived convention; the derived basis is explicit, not exact",
            Link::SourceFile,
            "src:app/routes/[slug].tsx",
            vec![
                Action::OpenProvingFile,
                Action::InspectClassAndConfidence,
                Action::OpenDocsOrApplyFix,
                Action::CopyDiagnosticId,
            ],
        ),
        // 5. Low confidence / suppressed / deprecation notice → partial, bridged caveat, runtime
        //    trace.
        diagnostic_row(
            "diag-deprecation",
            "Bridged adapter reports a possibly deprecated lifecycle hook",
            Class::DeprecationNotice,
            "useLegacyEffect",
            "app/hooks/legacy.tsx",
            Confidence::LowConfidence,
            Severity::Suppressed,
            Certainty::Bridge,
            Detect::RuntimeProbe,
            "Observed by a bridged runtime probe",
            Caveat::BridgedBehavior,
            "Produced by bridged behavior, not exact first-party support",
            Fix::OpenDocsOnly,
            "Read the deprecation notes (bridged, low confidence)",
            "Low-confidence deprecation from a bridge probe; evidence is only partial, do not trust it as complete",
            Link::RuntimeTrace,
            "trace:runtime/legacy-hook",
            vec![
                Action::OpenProvingFile,
                Action::InspectClassAndConfidence,
                Action::OpenDocsOrApplyFix,
                Action::OpenReference,
            ],
        ),
        // 6. Unknown / stale / unknown diagnostic → partial, unsupported caveat, no proving source.
        diagnostic_row(
            "diag-unknown",
            "Unclassified framework diagnostic from a stale scan",
            Class::UnknownDiagnostic,
            "unresolved entity",
            "unknown file",
            Confidence::Unknown,
            Severity::Stale,
            Certainty::Partial,
            Detect::StaticAnalysis,
            "Could not be classified from the workspace",
            Caveat::Unsupported,
            "Unsupported on this framework or build",
            Fix::NoFixAvailable,
            "No fix available; re-run the analysis",
            "Unclassified diagnostic of unknown confidence; there is no proving file to open",
            Link::NoProvingSource,
            "",
            vec![
                Action::OpenProvingFile,
                Action::InspectClassAndConfidence,
                Action::OpenDocsOrApplyFix,
            ],
        ),
    ]
}

fn relationship_banners() -> Vec<DerivedRelationshipBanner> {
    use BannerAction as Action;
    use InferenceSource as Infer;
    use M5DerivedRelationshipClass as Class;
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5RelationshipProvingState as Proving;
    use ProvingSourceLink as Link;
    use RefreshState as Fresh;

    vec![
        // 1. Exact from source / proving-source-linked / current → exact, source file.
        relationship_banner(
            "rel-route-to-loader",
            "GET /users → users loader",
            Class::ExactFromSource,
            Proving::ProvingSourceLinked,
            Certainty::CoreNative,
            Infer::StaticSource,
            "Read directly from the route file",
            Fresh::Current,
            "Refreshed just now",
            "Shown on the route-explorer row for GET /users",
            "Exact source-linked relationship; open the raw source before trusting it",
            Link::SourceFile,
            "src:app/routes/users.tsx",
            vec![
                Action::OpenRawSource,
                Action::OpenWiderGraph,
                Action::InspectStateAndSource,
                Action::CopyBannerId,
            ],
        ),
        // 2. Inferred from runtime / runtime-evidence-only / imported → runtime confirmed, runtime
        //    trace.
        relationship_banner(
            "rel-service-call",
            "Checkout page → Payments service",
            Class::InferredFromRuntime,
            Proving::RuntimeEvidenceOnly,
            Certainty::RuntimeConfirmed,
            Infer::RuntimeObservation,
            "Observed from the running application",
            Fresh::Imported,
            "Imported from a runtime scan",
            "Shown on the topology-explorer edge from Checkout to Payments",
            "Runtime-confirmed relationship from observation; inspect the runtime trace",
            Link::RuntimeTrace,
            "trace:runtime/payments-call",
            vec![
                Action::OpenRawSource,
                Action::OpenWiderGraph,
                Action::InspectStateAndSource,
                Action::OpenReference,
            ],
        ),
        // 3. Heuristic link / source-linked-partial / stale → heuristic, source symbol.
        relationship_banner(
            "rel-heuristic-import",
            "AuthModule → Session store (assumed)",
            Class::HeuristicLink,
            Proving::SourceLinkedPartial,
            Certainty::HeuristicConvention,
            Infer::NamingConvention,
            "Inferred from a naming convention",
            Fresh::Stale,
            "Scan is stale",
            "Shown on the topology-explorer dependency edge for AuthModule",
            "Heuristic link from a naming convention; do not treat the dependency as exact",
            Link::SourceSymbol,
            "symbol:AuthModule",
            vec![
                Action::OpenRawSource,
                Action::OpenWiderGraph,
                Action::InspectStateAndSource,
                Action::CopyBannerId,
            ],
        ),
        // 4. Derived by convention / convention-only / never refreshed → heuristic, docs anchor.
        relationship_banner(
            "rel-di-convention",
            "DI container → provider (by convention)",
            Class::DerivedByConvention,
            Proving::ConventionOnly,
            Certainty::DerivedByConvention,
            Infer::DependencyGraph,
            "Derived from the dependency-graph convention",
            Fresh::NeverRefreshed,
            "Never refreshed",
            "Shown on the dependency-graph banner for the DI container",
            "Convention-derived wiring; the derived basis is explicit, not an exact link",
            Link::DocsAnchor,
            "docs:frameworks/di-conventions",
            vec![
                Action::OpenRawSource,
                Action::OpenWiderGraph,
                Action::InspectStateAndSource,
                Action::OpenReference,
            ],
        ),
        // 5. Partial link / no-proving-source / unknown → partial, no proving source.
        relationship_banner(
            "rel-partial-manifest",
            "External billing → unknown consumer (partial)",
            Class::PartialLink,
            Proving::NoProvingSource,
            Certainty::Partial,
            Infer::ManifestDeclaration,
            "Declared partially in a manifest",
            Fresh::Unknown,
            "Refresh state unknown",
            "Shown on the external-boundary banner for billing",
            "Partial link with no proving source; treat the relationship as incomplete and open nothing that does not exist",
            Link::NoProvingSource,
            "",
            vec![
                Action::OpenRawSource,
                Action::OpenWiderGraph,
                Action::InspectStateAndSource,
            ],
        ),
        // 6. Unresolved link / unknown-proving / unknown → partial, no proving source.
        relationship_banner(
            "rel-unresolved",
            "Unresolved relationship",
            Class::UnresolvedLink,
            Proving::UnknownProving,
            Certainty::Partial,
            Infer::StaticSource,
            "Could not be resolved from the workspace",
            Fresh::Unknown,
            "Refresh state unknown",
            "Shown on the topology-explorer banner for an unresolved edge",
            "Unresolved relationship of unknown proving; there is no source to open",
            Link::NoProvingSource,
            "",
            vec![
                Action::OpenRawSource,
                Action::OpenWiderGraph,
                Action::InspectStateAndSource,
            ],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
        M5FrameworkDowngradeTrigger::ConventionConfidenceOverstated,
        M5FrameworkDowngradeTrigger::SupportClassUnstated,
        M5FrameworkDowngradeTrigger::DerivedStateUnlabeled,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn convention_relationship_review() -> ConventionRelationshipReview {
    ConventionRelationshipReview {
        diagnostic_row_shows_class_and_entity: true,
        diagnostic_row_shows_confidence_and_severity: true,
        diagnostic_row_offers_proving_file: true,
        banner_shows_relationship_and_source: true,
        banner_shows_state_and_refresh: true,
        banner_offers_raw_source_and_wider_graph: true,
        certainty_derived_never_asserted: true,
        heuristic_never_shown_as_exact: true,
        distinct_diagnostics_never_collapsed: true,
        approximation_never_hidden_in_background: true,
        support_class_caveat_always_visible: true,
        every_component_links_to_proving_source: true,
        ungrounded_component_never_fakes_a_source: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ConventionRelationshipConsumerProjection {
    ConventionRelationshipConsumerProjection {
        diagnostic_center_reads_single_source: true,
        editor_gutter_reads_single_source: true,
        topology_explorer_reads_single_source: true,
        certainty_and_caveat_visible_before_trust: true,
        banner_appears_where_inferred_truth_consumed: true,
        proving_source_reachable_before_trust: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> ConventionRelationshipProofFreshness {
    ConventionRelationshipProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF,
        CONVENTION_RELATIONSHIP_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_CONVENTION_DIAGNOSTIC_ROW_SCHEMA_REF,
        M5_DERIVED_RELATIONSHIP_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical convention-diagnostic-row / derived-relationship-banner controls packet.
pub fn seeded_convention_relationship_controls(
) -> ConventionDiagnosticDerivedRelationshipControlsPacket {
    ConventionDiagnosticDerivedRelationshipControlsPacket::new(
        ConventionDiagnosticDerivedRelationshipControlsPacketInput {
            packet_id: CONVENTION_RELATIONSHIP_CONTROLS_PACKET_ID.to_owned(),
            surface_label:
                "M5 convention-diagnostic rows and derived-relationship banners: diagnostic class, affected entity / file, confidence / severity, detected source, suggested fix / open-docs action, support-class caveat, source of inference, last refresh, exact-versus-heuristic-versus-runtime-confirmed state, and canonical proving-source truth across claimed framework-diagnostic surfaces"
                    .to_owned(),
            diagnostic_rows: diagnostic_rows(),
            relationship_banners: relationship_banners(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
            convention_relationship_review: convention_relationship_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a heuristic-suspicion diagnostic that must never read as an exact
/// contract fact and must never collapse into a generic warning. Every convention confidence class,
/// diagnostic severity, diagnostic class, detection source, and support caveat stays covered so the
/// fixture validates on its own.
pub fn seeded_convention_relationship_controls_heuristic_diagnostic(
) -> ConventionDiagnosticDerivedRelationshipControlsPacket {
    let mut packet = seeded_convention_relationship_controls();
    packet.packet_id =
        "m5-convention-diagnostic-derived-relationship-controls:fixture:heuristic-diagnostic"
            .to_owned();
    packet.surface_label =
        "M5 convention-diagnostic rows: a heuristic suspicion never reads as an exact contract fact"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights an inferred / unresolved relationship banner that must keep its
/// approximation visible at the point of consumption and never fake a proving source. Every
/// derived-relationship class, relationship proving state, inference source, and refresh state stays
/// covered so the fixture validates on its own.
pub fn seeded_convention_relationship_controls_inferred_relationship(
) -> ConventionDiagnosticDerivedRelationshipControlsPacket {
    let mut packet = seeded_convention_relationship_controls();
    packet.packet_id =
        "m5-convention-diagnostic-derived-relationship-controls:fixture:inferred-relationship"
            .to_owned();
    packet.surface_label =
        "M5 derived-relationship banners: an inferred or unresolved relationship keeps its approximation visible"
            .to_owned();
    packet
}
