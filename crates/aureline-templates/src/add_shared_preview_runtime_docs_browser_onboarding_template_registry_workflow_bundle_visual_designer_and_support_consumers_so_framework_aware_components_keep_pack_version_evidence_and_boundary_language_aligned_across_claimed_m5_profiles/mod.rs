//! Shared consumers for the reusable M5 framework-aware / topology-explorer components, so the
//! framework pack header, route / endpoint row, component / service tree node, convention-diagnostic
//! row, generator preview sheet, run-config scaffold card, and derived-relationship banner keep
//! pack-version / support-class, exact-versus-heuristic-versus-runtime-confirmed evidence,
//! proving-source, and local-versus-remote execution-boundary language aligned across every claimed
//! M5 surface that explores or acts on framework structure: the preview runtime, the docs / browser,
//! onboarding, the template registry, workflow bundles, the visual designer, and the safe support /
//! export packet.
//!
//! Aureline's frozen framework-component matrix
//! (`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`)
//! names the seven governed component families, and four sibling implement lanes narrow those
//! families into working primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the framework pack header and framework status strip
//!   (`implement_framework_pack_headers_and_framework_status_strips_...`),
//! * the route / endpoint row and component / service tree node
//!   (`implement_route_endpoint_rows_and_component_service_tree_nodes_...`),
//! * the convention-diagnostic row and derived-relationship banner
//!   (`implement_convention_diagnostic_rows_and_derived_relationship_banners_...`), and
//! * the generator preview sheet and run-config scaffold card
//!   (`implement_generator_preview_sheets_and_run_config_scaffold_cards_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the seven families are
//! reusable components — not one framework-pack page plus a few isolated topology objects — by
//! binding every claimed M5 framework consumer (the preview runtime, the docs / browser, onboarding,
//! the template registry, workflow bundles, the visual designer, and the safe support / export
//! packet) to the same canonical component schemas and the same descriptor vocabulary. Each consumer
//! points at the primitive's canonical schema and support-export artifact rather than re-wording
//! pack-version / support-class, evidence-source / certainty, proving-source, or execution-boundary /
//! impact facts in local prose, and each keeps that vocabulary truthful even when a pack's identity or
//! support is unverified, a route / component / relationship is heuristic rather than exact, a
//! generator write or run-config dispatch carries a non-local execution boundary or write effect, or a
//! generator left output that must stay recoverable through rollback or regenerate.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_framework_component_binding`] — that takes one consumer's adoption of one
//!    component family, the descriptor set it surfaces, the parity-health mode it renders under, and
//!    any export caveats, and produces one [`M5FrameworkComponentResolvedBinding`] carrying the
//!    derived claim-parity state and — whenever parity is weakened — a self-contained
//!    [`M5FrameworkComponentAutoNarrowBanner`] that names the exact reason (an unverified pack /
//!    support, a heuristic-not-exact evidence class, a pending execution-boundary / write-effect
//!    disclosure, or a recovery-required generator write), the descriptors that stay preserved, and
//!    the recovery action, rather than a generic "degraded" note. The resolver never lets a narrowed
//!    context drop a required descriptor and never lets a generator write or non-local run present as a
//!    plain safe action.
//! 2. A parity matrix — [`M5FrameworkComponentConsumerPacket`] — that binds one row per claimed M5
//!    framework consumer to the seven canonical component families, the one shared descriptor
//!    vocabulary, the same parity-health modes, export caveats, parity states, narrowing reasons,
//!    recovery actions, export fields, and non-visual accessibility routes, so pack-version /
//!    support-class, evidence-source, proving-source, and execution-boundary facts stop diverging
//!    between the framework-pack cards, route / topology explorers, generator-review sheets, and
//!    support artifacts.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
//! classes, downgrade triggers, and the seven component families themselves are reused verbatim from
//! the frozen framework-component matrix. This module mints new vocabulary only for what the adoption
//! lane itself needs: its framework consumers, the shared descriptor vocabulary, the parity-health
//! modes, the export caveats, the claim-parity states, the narrowing reasons and recovery actions,
//! the consumer anatomy parts, and the export fields.
//!
//! Raw file bodies, raw diffs, raw local paths, repository URLs, credentials, and secrets stay
//! outside the support boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is `schemas/ui/m5-framework-component-consumer.schema.json` and the contract
//! doc is `docs/frameworks/m5/m5_framework_component_consumers.md`. The protected fixture directory is
//! `fixtures/ui/m5-framework-component-consumers/`.

#[cfg(test)]
mod tests;

// The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
// classes, downgrade triggers, and the seven component families are frozen once, in the
// framework-component matrix. This adoption lane reuses them verbatim so it never invents a parallel
// framework vocabulary.
pub use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5FrameworkAccessibilityRoute, M5FrameworkComponentFamily, M5FrameworkConsumerSurface,
    M5FrameworkDeploymentLine, M5FrameworkDowngradeTrigger, M5FrameworkQualificationClass,
    M5FrameworkSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather than
// re-wording their facts in local prose.
use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5_FRAMEWORK_COMPONENT_DOC_REF, M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_convention_diagnostic_rows_and_derived_relationship_banners_with_diagnostic_class_affected_entity_or_file_certainty_detected_source_suggested_fix_or_open_docs_actions_support_class_caveats_and_open_raw_source_or_wider_graph_continuity::{
    CONVENTION_RELATIONSHIP_CONTROLS_ARTIFACT_REF, CONVENTION_RELATIONSHIP_CONTROLS_DOC_REF,
    CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF,
};
use crate::implement_framework_pack_headers_and_framework_status_strips_with_pack_identity_version_support_range_provider_source_freshness_compatibility_and_local_versus_remote_scope_truth::{
    FRAMEWORK_PACK_HEADER_CONTROLS_ARTIFACT_REF, FRAMEWORK_PACK_HEADER_CONTROLS_DOC_REF,
    FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF,
};
use crate::implement_generator_preview_sheets_and_run_config_scaffold_cards_with_generator_version_file_effect_classes_dependency_config_impact_rollback_or_regenerate_posture_required_toolchains_and_local_container_ssh_managed_target_truth::{
    GENERATOR_RUN_CONFIG_CONTROLS_ARTIFACT_REF, GENERATOR_RUN_CONFIG_CONTROLS_DOC_REF,
    GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF,
};
use crate::implement_route_endpoint_rows_and_component_service_tree_nodes_with_authored_versus_generated_state_proving_source_files_or_symbols_exact_versus_heuristic_labels_and_open_source_or_open_references_continuity::{
    ROUTE_TREE_CONTROLS_ARTIFACT_REF, ROUTE_TREE_CONTROLS_DOC_REF, ROUTE_TREE_CONTROLS_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5FrameworkComponentConsumerPacket`].
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_preview_runtime_docs_browser_onboarding_template_registry_workflow_bundle_visual_designer_and_support_consumers_so_framework_aware_components_keep_pack_version_evidence_and_boundary_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 framework component-consumer records.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the framework component-consumer boundary schema.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-framework-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/frameworks/m5/m5_framework_component_consumers.md";

/// Repo-relative path of the frozen framework-component matrix this lane adopts from.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_FRAMEWORK_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_FRAMEWORK_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-framework-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-framework-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-framework-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-framework-component-consumer-proof/report.md";

/// Stable packet id for the canonical framework component-consumer packet.
pub const M5_FRAMEWORK_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-framework-component-consumer:stable:0001";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer that
/// adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5FrameworkComponentFamily) -> &'static str {
    use M5FrameworkComponentFamily as Family;
    match family {
        Family::FrameworkPackHeader => FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF,
        Family::RouteEndpointRow | Family::ComponentServiceTreeNode => {
            ROUTE_TREE_CONTROLS_SCHEMA_REF
        }
        Family::ConventionDiagnosticRow | Family::DerivedRelationshipBanner => {
            CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF
        }
        Family::GeneratorPreviewSheet | Family::RunConfigScaffoldCard => {
            GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5FrameworkComponentFamily) -> &'static str {
    use M5FrameworkComponentFamily as Family;
    match family {
        Family::FrameworkPackHeader => FRAMEWORK_PACK_HEADER_CONTROLS_DOC_REF,
        Family::RouteEndpointRow | Family::ComponentServiceTreeNode => ROUTE_TREE_CONTROLS_DOC_REF,
        Family::ConventionDiagnosticRow | Family::DerivedRelationshipBanner => {
            CONVENTION_RELATIONSHIP_CONTROLS_DOC_REF
        }
        Family::GeneratorPreviewSheet | Family::RunConfigScaffoldCard => {
            GENERATOR_RUN_CONFIG_CONTROLS_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(family: M5FrameworkComponentFamily) -> &'static str {
    use M5FrameworkComponentFamily as Family;
    match family {
        Family::FrameworkPackHeader => FRAMEWORK_PACK_HEADER_CONTROLS_ARTIFACT_REF,
        Family::RouteEndpointRow | Family::ComponentServiceTreeNode => {
            ROUTE_TREE_CONTROLS_ARTIFACT_REF
        }
        Family::ConventionDiagnosticRow | Family::DerivedRelationshipBanner => {
            CONVENTION_RELATIONSHIP_CONTROLS_ARTIFACT_REF
        }
        Family::GeneratorPreviewSheet | Family::RunConfigScaffoldCard => {
            GENERATOR_RUN_CONFIG_CONTROLS_ARTIFACT_REF
        }
    }
}

/// One claimed M5 framework consumer that adopts the shared components. These are the consumers the
/// spec names — the preview runtime, the docs / browser, onboarding, the template registry, workflow
/// bundles, the visual designer, and the safe support / export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentConsumer {
    /// The preview-runtime surface.
    PreviewRuntime,
    /// The docs / browser surface.
    DocsBrowser,
    /// The onboarding surface.
    Onboarding,
    /// The template-registry surface.
    TemplateRegistry,
    /// The workflow-bundle surface.
    WorkflowBundle,
    /// The visual-designer surface.
    VisualDesigner,
    /// The safe support / export packet.
    SupportExport,
}

impl M5FrameworkComponentConsumer {
    /// Every claimed framework consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PreviewRuntime,
        Self::DocsBrowser,
        Self::Onboarding,
        Self::TemplateRegistry,
        Self::WorkflowBundle,
        Self::VisualDesigner,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRuntime => "preview_runtime",
            Self::DocsBrowser => "docs_browser",
            Self::Onboarding => "onboarding",
            Self::TemplateRegistry => "template_registry",
            Self::WorkflowBundle => "workflow_bundle",
            Self::VisualDesigner => "visual_designer",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PreviewRuntime => "Preview Runtime",
            Self::DocsBrowser => "Docs / Browser",
            Self::Onboarding => "Onboarding",
            Self::TemplateRegistry => "Template Registry",
            Self::WorkflowBundle => "Workflow Bundle",
            Self::VisualDesigner => "Visual Designer",
            Self::SupportExport => "Safe Support / Export Packet",
        }
    }

    /// True when this consumer is the safe support / export packet — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// The one shared descriptor vocabulary every framework component keeps aligned across surfaces, so no
/// consumer invents a new grammar or stale wording. The descriptors in
/// [`M5FrameworkComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that pack-version / support-class, evidence-source / certainty,
/// execution-boundary / impact, and recovery / rollback language stay one truth across framework-pack
/// cards, route / topology explorers, generator-review sheets, and support artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkComponentDescriptor {
    /// The pack-identity-and-support descriptor: framework pack identity, pinned version, and support
    /// class.
    PackIdentityAndSupport,
    /// The evidence-certainty-and-proving-source descriptor: authored-versus-generated status,
    /// exact-versus-heuristic-versus-runtime-confirmed certainty, and the proving source that grounds
    /// it.
    EvidenceCertaintyAndProvingSource,
    /// The execution-boundary-and-impact descriptor: the local / container / SSH / managed execution
    /// boundary and the file / dependency / config impact a component discloses.
    ExecutionBoundaryAndImpact,
    /// The recovery-and-rollback descriptor: the rollback or regenerate posture and the
    /// generated-versus-user-owned boundary.
    RecoveryAndRollbackBoundary,
}

impl M5FrameworkComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PackIdentityAndSupport,
        Self::EvidenceCertaintyAndProvingSource,
        Self::ExecutionBoundaryAndImpact,
        Self::RecoveryAndRollbackBoundary,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackIdentityAndSupport => "pack_identity_and_support",
            Self::EvidenceCertaintyAndProvingSource => "evidence_certainty_and_proving_source",
            Self::ExecutionBoundaryAndImpact => "execution_boundary_and_impact",
            Self::RecoveryAndRollbackBoundary => "recovery_and_rollback_boundary",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the authoritative
/// rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerParityHealth {
    /// Full parity: the authoritative rendering.
    FullParity,
    /// A framework pack's identity, version, or support class is unverified, so it is disclosed as
    /// community / bridged / unknown rather than presented as exact first-party support.
    PackOrSupportUnverifiedNarrowed,
    /// A route, component / service node, convention, or relationship is heuristic or derived rather
    /// than exact from source, so it is disclosed as heuristic rather than presented as an exact fact.
    HeuristicEvidenceNarrowed,
    /// A generator write or run-config dispatch carries a file / dependency / config write effect or a
    /// non-local execution boundary, so it is disclosed before any apply or run rather than routed
    /// silently through a convenience action.
    ExecutionBoundaryOrWriteEffectNarrowed,
    /// A generator applied output, so the generated-versus-user-owned boundary and a rollback or
    /// regenerate recovery path stay explicit.
    RecoveryRequiredNarrowed,
}

impl M5FrameworkConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::PackOrSupportUnverifiedNarrowed,
        Self::HeuristicEvidenceNarrowed,
        Self::ExecutionBoundaryOrWriteEffectNarrowed,
        Self::RecoveryRequiredNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::PackOrSupportUnverifiedNarrowed => "pack_or_support_unverified_narrowed",
            Self::HeuristicEvidenceNarrowed => "heuristic_evidence_narrowed",
            Self::ExecutionBoundaryOrWriteEffectNarrowed => {
                "execution_boundary_or_write_effect_narrowed"
            }
            Self::RecoveryRequiredNarrowed => "recovery_required_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5FrameworkConsumerNarrowingReason> {
        Some(match self {
            Self::PackOrSupportUnverifiedNarrowed => {
                M5FrameworkConsumerNarrowingReason::PackOrSupportUnverified
            }
            Self::HeuristicEvidenceNarrowed => {
                M5FrameworkConsumerNarrowingReason::HeuristicEvidenceNotExact
            }
            Self::ExecutionBoundaryOrWriteEffectNarrowed => {
                M5FrameworkConsumerNarrowingReason::ExecutionBoundaryOrWriteEffectPending
            }
            Self::RecoveryRequiredNarrowed => {
                M5FrameworkConsumerNarrowingReason::RecoveryRequiredAfterGeneratorWrite
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner never
/// reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerNarrowingReason {
    /// A framework pack's identity, version, or support class is unverified.
    PackOrSupportUnverified,
    /// A route, component / service node, convention, or relationship is heuristic or derived rather
    /// than exact from source.
    HeuristicEvidenceNotExact,
    /// A generator write or run-config dispatch carries a write effect or non-local execution boundary
    /// that must be disclosed before any apply or run.
    ExecutionBoundaryOrWriteEffectPending,
    /// A generator applied output that must stay recoverable through rollback or regenerate.
    RecoveryRequiredAfterGeneratorWrite,
}

impl M5FrameworkConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PackOrSupportUnverified,
        Self::HeuristicEvidenceNotExact,
        Self::ExecutionBoundaryOrWriteEffectPending,
        Self::RecoveryRequiredAfterGeneratorWrite,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackOrSupportUnverified => "pack_or_support_unverified",
            Self::HeuristicEvidenceNotExact => "heuristic_evidence_not_exact",
            Self::ExecutionBoundaryOrWriteEffectPending => {
                "execution_boundary_or_write_effect_pending"
            }
            Self::RecoveryRequiredAfterGeneratorWrite => "recovery_required_after_generator_write",
        }
    }

    /// True when the reason reflects a generator write or run-config dispatch that carries a write
    /// effect or non-local execution boundary and must never be routed through a convenience action —
    /// the acceptance-criterion boundary that a generator never implies a no-op write and framework
    /// convenience never hides the local / container / SSH / managed boundary.
    pub const fn is_execution_boundary_or_write_effect(self) -> bool {
        matches!(self, Self::ExecutionBoundaryOrWriteEffectPending)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::PackOrSupportUnverified => {
                "a framework pack's identity, version, or support class is unverified, so it is disclosed as community / bridged / unknown rather than presented as exact first-party support"
            }
            Self::HeuristicEvidenceNotExact => {
                "a route, component / service node, convention, or relationship is heuristic or derived rather than exact from source, so it is disclosed as heuristic rather than presented as an exact fact"
            }
            Self::ExecutionBoundaryOrWriteEffectPending => {
                "a generator write or run-config dispatch carries a file / dependency / config write effect or a non-local execution boundary, so it is disclosed before any apply or run rather than routed silently through a convenience action"
            }
            Self::RecoveryRequiredAfterGeneratorWrite => {
                "a generator applied output, so the generated-versus-user-owned boundary and a rollback or regenerate recovery path stay explicit rather than assumed final"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5FrameworkConsumerRecoveryAction {
        match self {
            Self::PackOrSupportUnverified => {
                M5FrameworkConsumerRecoveryAction::InspectPackVersionAndSupport
            }
            Self::HeuristicEvidenceNotExact => {
                M5FrameworkConsumerRecoveryAction::OpenProvingSourceBeforeTrusting
            }
            Self::ExecutionBoundaryOrWriteEffectPending => {
                M5FrameworkConsumerRecoveryAction::ReviewExecutionBoundaryAndImpactBeforeDispatch
            }
            Self::RecoveryRequiredAfterGeneratorWrite => {
                M5FrameworkConsumerRecoveryAction::RollbackOrRegenerateGeneratedOutput
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable from the
/// banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerRecoveryAction {
    /// Inspect the pack's pinned version and support class before trusting an exact first-party read.
    InspectPackVersionAndSupport,
    /// Open the proving source or wider graph before trusting a heuristic or derived read.
    OpenProvingSourceBeforeTrusting,
    /// Review the disclosed execution boundary and file / dependency / config impact before any apply
    /// or run, rather than routing through a convenience action.
    ReviewExecutionBoundaryAndImpactBeforeDispatch,
    /// Roll back or regenerate the generated output after a generator write.
    RollbackOrRegenerateGeneratedOutput,
}

impl M5FrameworkConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectPackVersionAndSupport,
        Self::OpenProvingSourceBeforeTrusting,
        Self::ReviewExecutionBoundaryAndImpactBeforeDispatch,
        Self::RollbackOrRegenerateGeneratedOutput,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectPackVersionAndSupport => "inspect_pack_version_and_support",
            Self::OpenProvingSourceBeforeTrusting => "open_proving_source_before_trusting",
            Self::ReviewExecutionBoundaryAndImpactBeforeDispatch => {
                "review_execution_boundary_and_impact_before_dispatch"
            }
            Self::RollbackOrRegenerateGeneratedOutput => "rollback_or_regenerate_generated_output",
        }
    }
}

/// An export caveat a consumer preserves when a component renders below full parity (an unverified
/// pack / support, a heuristic-not-exact evidence class, a pending execution-boundary / write-effect
/// disclosure, or a recovery-required generator write).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerExportCaveat {
    /// The pack's identity, version, or support class is unverified.
    PackOrSupportUnverified,
    /// The evidence is heuristic, not exact.
    EvidenceHeuristicNotExact,
    /// The execution boundary or write effect is disclosed, not run silently.
    ExecutionBoundaryOrWriteEffectDisclosedNotSilent,
    /// The generated output is recoverable, not final.
    GeneratedOutputRecoverableNotFinal,
}

impl M5FrameworkConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PackOrSupportUnverified,
        Self::EvidenceHeuristicNotExact,
        Self::ExecutionBoundaryOrWriteEffectDisclosedNotSilent,
        Self::GeneratedOutputRecoverableNotFinal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackOrSupportUnverified => "pack_or_support_unverified",
            Self::EvidenceHeuristicNotExact => "evidence_heuristic_not_exact",
            Self::ExecutionBoundaryOrWriteEffectDisclosedNotSilent => {
                "execution_boundary_or_write_effect_disclosed_not_silent"
            }
            Self::GeneratedOutputRecoverableNotFinal => "generated_output_recoverable_not_final",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is kept
/// aligned as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkClaimParityState {
    /// The descriptor vocabulary is kept aligned at full parity.
    ClaimsAligned,
    /// The descriptor vocabulary is kept aligned, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5FrameworkClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsAligned, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsAligned => "claims_aligned",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5FrameworkConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5FrameworkConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable from the shared
/// model. The fields in [`M5FrameworkConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FrameworkConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5FrameworkConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay preserved, the
/// export caveats, and the recovery action, so a narrowed rendering is understood from the banner
/// alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5FrameworkConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5FrameworkConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5FrameworkComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5FrameworkComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5FrameworkComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5FrameworkConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors, and the
    /// recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the framework component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5FrameworkComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5FrameworkComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so
    /// pack-identity-and-support, evidence-certainty-and-proving-source, execution-boundary-and-impact,
    /// and recovery-and-rollback stay explicit.
    pub descriptor_families: Vec<M5FrameworkComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5FrameworkConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5FrameworkConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5FrameworkComponentConsumer,
    /// The component family.
    pub component_family: M5FrameworkComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5FrameworkComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5FrameworkConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5FrameworkConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5FrameworkClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects a generator write or run-config dispatch carrying a write effect
    /// or non-local execution boundary. Such a binding must always be narrowed and never present as a
    /// plain safe action.
    pub reflects_write_or_boundary_risk: bool,
    /// Hard invariant: whether this binding presents a safe apply / run action without a caveat. Only a
    /// full-parity binding may present it; every narrowed binding — and in particular any
    /// write-or-boundary-bearing one — resolves this to `false` so a generator never implies a no-op
    /// write and a non-local run boundary is never hidden.
    pub presents_safe_action_without_caveat: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5FrameworkComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_framework_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5FrameworkComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5FrameworkComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5FrameworkComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "framework component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5FrameworkComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that
/// pack-identity-and-support, evidence-certainty-and-proving-source, execution-boundary-and-impact,
/// and recovery-and-rollback stay explicit on every surface. The claim-parity state is kept aligned at
/// full parity and auto-narrowed under any weakened parity-health mode, and a weakened mode always
/// produces a self-contained banner naming the exact reason and recovery action while keeping the
/// descriptor vocabulary intact. A binding that carries a generator write or a non-local run boundary
/// always narrows and never presents a plain safe action.
pub fn resolve_framework_component_binding(
    input: &M5FrameworkComponentBindingInput,
) -> Result<M5FrameworkComponentResolvedBinding, M5FrameworkComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5FrameworkComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5FrameworkComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5FrameworkComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5FrameworkComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5FrameworkComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text extension from
        // leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5FrameworkComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_write_or_boundary_risk = narrowing_reason
        .is_some_and(M5FrameworkConsumerNarrowingReason::is_execution_boundary_or_write_effect);
    // Only a full-parity binding may present a safe apply / run action without a caveat. Every
    // narrowed binding — and every write-or-boundary-bearing one in particular — does not.
    let presents_safe_action_without_caveat = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5FrameworkClaimParityState::ClaimsAutoNarrowed
    } else {
        M5FrameworkClaimParityState::ClaimsAligned
    };

    let auto_narrow_banner = narrowing_reason.map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5FrameworkComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5FrameworkComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_write_or_boundary_risk,
        presents_safe_action_without_caveat,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs consumer
/// parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentBindingCase {
    /// The resolver input.
    pub input: M5FrameworkComponentBindingInput,
    /// The resolved truth. Must equal `resolve_framework_component_binding(&input)`.
    pub resolved: M5FrameworkComponentResolvedBinding,
}

impl M5FrameworkComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5FrameworkComponentBindingInput) -> Self {
        let resolved =
            resolve_framework_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_framework_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer points
/// at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5FrameworkComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the family's
    /// canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description of its
    /// facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5FrameworkComponentBindingCase>,
}

impl M5FrameworkComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical family
    /// rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one framework consumer bound to the canonical component families,
/// the shared descriptor vocabulary, the parity-health modes, export caveats, parity states, narrowing
/// reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerRow {
    /// Framework consumer.
    pub consumer: M5FrameworkComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5FrameworkQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 framework surface families that render / consume this projection.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5FrameworkConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5FrameworkComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5FrameworkConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5FrameworkConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5FrameworkClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5FrameworkConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5FrameworkConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5FrameworkConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5FrameworkComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new framework grammar. MUST be `false`.
    pub invents_new_framework_grammar: bool,
    /// Hard invariant: this consumer never drops pack-evidence, proving-source, or execution-boundary
    /// truth when narrowed. MUST be `false`.
    pub drops_pack_evidence_or_boundary_truth_when_narrowed: bool,
    /// Hard invariant: this consumer never lets a heuristic route / component / relationship
    /// masquerade as exact. MUST be `false`.
    pub lets_heuristic_masquerade_as_exact: bool,
    /// Hard invariant: this consumer never lets a generator imply a no-op write or hide the local /
    /// container / SSH / managed execution boundary. MUST be `false`.
    pub implies_no_op_write_or_hides_execution_boundary: bool,
}

impl M5FrameworkComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5FrameworkConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5FrameworkConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5FrameworkConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5FrameworkConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5FrameworkComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5FrameworkComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5FrameworkComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5FrameworkComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_framework_grammar
            && !self.drops_pack_evidence_or_boundary_truth_when_narrowed
            && !self.lets_heuristic_masquerade_as_exact
            && !self.implies_no_op_write_or_hides_execution_boundary
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerVocabularySet {
    /// Framework-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5FrameworkComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5FrameworkComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5FrameworkComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5FrameworkComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5FrameworkConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5FrameworkConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5FrameworkConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5FrameworkConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5FrameworkClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5FrameworkConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5FrameworkConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5FrameworkAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5FrameworkComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new framework grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Pack-identity-and-support, evidence-certainty-and-proving-source,
    /// execution-boundary-and-impact, and recovery-and-rollback stay explicit everywhere.
    pub pack_evidence_boundary_and_recovery_explicit_on_every_surface: bool,
    /// A weakened parity-health mode auto-narrows the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// A heuristic route / component / relationship never masquerades as exact.
    pub heuristic_never_masquerades_as_exact: bool,
    /// A generator never implies a no-op write and framework convenience never hides the local /
    /// container / SSH / managed execution boundary.
    pub generator_never_implies_no_op_write_or_hides_boundary: bool,
    /// The support / export packet presents the same framework truth shown in-product.
    pub support_export_presents_same_framework_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerProjection {
    /// The preview runtime, docs / browser, onboarding, the template registry, workflow bundles, the
    /// visual designer, and the safe support / export packet all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The pack-identity-and-support descriptor reads a single canonical source.
    pub pack_identity_and_support_reads_single_source: bool,
    /// The evidence-certainty-and-proving-source descriptor reads a single canonical source.
    pub evidence_certainty_and_proving_source_reads_single_source: bool,
    /// The execution-boundary-and-impact descriptor reads a single canonical source.
    pub execution_boundary_and_impact_reads_single_source: bool,
    /// The recovery-and-rollback descriptor reads a single canonical source.
    pub recovery_and_rollback_boundary_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting framework-component consumer audit.
    pub framework_component_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5FrameworkComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FrameworkComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5FrameworkComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FrameworkComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FrameworkComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FrameworkComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FrameworkComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FrameworkComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 framework component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FrameworkComponentConsumerPacket {
    /// Record kind; must equal [`M5_FRAMEWORK_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5FrameworkComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FrameworkComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FrameworkComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FrameworkComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FrameworkComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FrameworkComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FrameworkComponentConsumerPacket {
    /// Builds an M5 framework component-consumer packet from stable-lane input.
    pub fn new(input: M5FrameworkComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_FRAMEWORK_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 framework component-consumer invariants.
    pub fn validate(&self) -> Vec<M5FrameworkComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FRAMEWORK_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5FrameworkComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5FrameworkComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FrameworkComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_full_parity_preserved(self, &mut violations);
        validate_write_boundary_honesty(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 framework component consumer packet serializes"),
        ) {
            violations.push(M5FrameworkComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 framework component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Framework Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Framework consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Framework consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` -> `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` -> `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 framework component-consumer export.
#[derive(Debug)]
pub enum M5FrameworkComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FrameworkComponentConsumerViolation>),
}

impl fmt::Display for M5FrameworkComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 framework component consumer export parse failed: {error}"
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
                    "m5 framework component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FrameworkComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5FrameworkComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FrameworkComponentConsumerViolation {
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
    /// A required framework consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer (reuse across
    /// surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no banner.
    FullParityPreservedUnproven,
    /// No worked binding proves that a write-or-boundary-bearing action narrows and never presents a
    /// plain safe action, or a binding does so incorrectly.
    WriteBoundaryHonestyUnproven,
    /// The safe support / export packet consumer does not reference the canonical component schema.
    SupportExportReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5FrameworkComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::FullParityPreservedUnproven => "full_parity_preserved_unproven",
            Self::WriteBoundaryHonestyUnproven => "write_boundary_honesty_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 framework component-consumer export.
pub fn current_stable_m5_framework_component_consumer_export(
) -> Result<M5FrameworkComponentConsumerPacket, M5FrameworkComponentConsumerArtifactError> {
    let packet: M5FrameworkComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-framework-component-consumer-proof/support_export.json"
    )))
    .map_err(M5FrameworkComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FrameworkComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_CONSUMER_DOC_REF,
        M5_FRAMEWORK_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_FRAMEWORK_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF,
        ROUTE_TREE_CONTROLS_SCHEMA_REF,
        CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF,
        GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FrameworkComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5FrameworkComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let present: BTreeSet<M5FrameworkComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5FrameworkComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5FrameworkComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5FrameworkComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5FrameworkComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5FrameworkComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5FrameworkComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5FrameworkComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5FrameworkComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5FrameworkComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5FrameworkComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5FrameworkComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5FrameworkComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5FrameworkComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5FrameworkComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5FrameworkComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one framework-pack
/// page plus a few isolated topology objects.
fn validate_family_reuse(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    for family in M5FrameworkComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5FrameworkComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner carries
/// a specific reason, a recovery action, and a non-empty set of preserved descriptors — the
/// acceptance-criterion example that a consumer which cannot preserve parity is visibly narrowed
/// rather than silently dropping pack, evidence, boundary, or recovery language.
fn validate_narrowing_disclosure(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5FrameworkComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with preserved
/// parity and no banner — the acceptance-criterion example that full-parity consumers keep the
/// descriptor vocabulary without a spurious narrowing note.
fn validate_full_parity_preserved(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5FrameworkClaimParityState::ClaimsAligned
    });
    if !proven {
        violations.push(M5FrameworkComponentConsumerViolation::FullParityPreservedUnproven);
    }
}

/// Every worked binding that reflects a generator write or non-local run boundary must be narrowed and
/// must not present a plain safe action, and at least one such binding must be present — the
/// acceptance-criterion that a generator never implies a no-op write and framework convenience never
/// hides the local / container / SSH / managed boundary on any claimed consumer.
fn validate_write_boundary_honesty(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_write_or_boundary_risk {
            // A write-or-boundary-bearing binding that presents a plain safe action, or fails to
            // narrow, breaks the acceptance criterion.
            if resolved.presents_safe_action_without_caveat
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5FrameworkClaimParityState::ClaimsAutoNarrowed
            {
                violations
                    .push(M5FrameworkComponentConsumerViolation::WriteBoundaryHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5FrameworkComponentConsumerViolation::WriteBoundaryHonestyUnproven);
    }
}

/// The safe support / export packet consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that a support / export lane can never drift from the
/// product truth.
fn validate_support_export_reference(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5FrameworkComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5FrameworkComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.pack_evidence_boundary_and_recovery_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.heuristic_never_masquerades_as_exact,
        review.generator_never_implies_no_op_write_or_hides_boundary,
        review.support_export_presents_same_framework_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5FrameworkComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.pack_identity_and_support_reads_single_source,
        projection.evidence_certainty_and_proving_source_reads_single_source,
        projection.execution_boundary_and_impact_reads_single_source,
        projection.recovery_and_rollback_boundary_reads_single_source,
    ] {
        if !ok {
            violations.push(M5FrameworkComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FrameworkComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FrameworkComponentConsumerPacket,
    violations: &mut Vec<M5FrameworkComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .framework_component_consumer_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5FrameworkComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5FrameworkComponentConsumerPacket,
) -> impl Iterator<Item = &M5FrameworkComponentBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

// ---- canonical seed builders --------------------------------------------------------------------

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5FrameworkComponentConsumer,
    component_family: M5FrameworkComponentFamily,
    parity_health: M5FrameworkConsumerParityHealth,
    export_caveats: &[M5FrameworkConsumerExportCaveat],
    note: &str,
) -> M5FrameworkComponentBindingCase {
    M5FrameworkComponentBindingCase::resolved(M5FrameworkComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5FrameworkComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5FrameworkComponentFamily,
    example_bindings: Vec<M5FrameworkComponentBindingCase>,
) -> M5FrameworkComponentBinding {
    M5FrameworkComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5FrameworkComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5FrameworkComponentBinding>,
) -> M5FrameworkComponentConsumerRow {
    M5FrameworkComponentConsumerRow {
        consumer,
        qualification: M5FrameworkQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5FrameworkConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5FrameworkComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5FrameworkConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5FrameworkConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5FrameworkClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5FrameworkConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5FrameworkConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5FrameworkConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5FrameworkDowngradeTrigger::PackIdentityUnstated,
            M5FrameworkDowngradeTrigger::SupportClassUnstated,
            M5FrameworkDowngradeTrigger::ExactVersusHeuristicUnstated,
            M5FrameworkDowngradeTrigger::AuthorshipUnstated,
            M5FrameworkDowngradeTrigger::ExecutionBoundaryUnstated,
            M5FrameworkDowngradeTrigger::ImpactUndisclosed,
            M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
            M5FrameworkDowngradeTrigger::RollbackPathOmitted,
            M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
            M5FrameworkDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_FRAMEWORK_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_framework_grammar: false,
        drops_pack_evidence_or_boundary_truth_when_narrowed: false,
        lets_heuristic_masquerade_as_exact: false,
        implies_no_op_write_or_hides_execution_boundary: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5FrameworkComponentConsumerRow> {
    use M5FrameworkComponentConsumer as Consumer;
    use M5FrameworkComponentFamily as Family;
    use M5FrameworkConsumerExportCaveat as Caveat;
    use M5FrameworkConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Preview runtime — the framework pack header auto-narrowed because the active pack's identity
    //    or support is unverified, and the run-config scaffold card auto-narrowed because the run
    //    carries a non-local execution boundary disclosed before dispatch. This is the write / boundary
    //    honesty case: a convenience run never hides where code executes.
    rows.push(base_row(
        Consumer::PreviewRuntime,
        "Preview-runtime surface owner",
        "The preview runtime adopts the framework pack header auto-narrowed because the active pack's identity, version, or support class is unverified, and the run-config scaffold card auto-narrowed because a run carries a non-local execution boundary disclosed before dispatch, referencing the canonical component schemas so pack-version / support-class, evidence-source, proving-source, and execution-boundary language appears here as in the docs / browser, onboarding, the template registry, workflow bundles, the visual designer, and the safe support / export packet",
        "evidence:m5-framework-consumer-preview-runtime:001",
        vec![
            binding(
                Family::FrameworkPackHeader,
                vec![case(
                    Consumer::PreviewRuntime,
                    Family::FrameworkPackHeader,
                    Health::PackOrSupportUnverifiedNarrowed,
                    &[Caveat::PackOrSupportUnverified],
                    "preview-runtime framework pack header narrowed by unverified pack or support",
                )],
            ),
            binding(
                Family::RunConfigScaffoldCard,
                vec![case(
                    Consumer::PreviewRuntime,
                    Family::RunConfigScaffoldCard,
                    Health::ExecutionBoundaryOrWriteEffectNarrowed,
                    &[Caveat::ExecutionBoundaryOrWriteEffectDisclosedNotSilent],
                    "preview-runtime run-config scaffold card narrowed by non-local execution boundary",
                )],
            ),
        ],
    ));

    // 2. Docs / browser — the route / endpoint row auto-narrowed because the route is heuristic rather
    //    than exact from source, and the component / service tree node at full parity, so a heuristic
    //    route never reads as an exact fact.
    rows.push(base_row(
        Consumer::DocsBrowser,
        "Docs / browser surface owner",
        "The docs / browser adopts the route / endpoint row auto-narrowed because the route is heuristic rather than exact from source, and the component / service tree node at full parity, referencing the canonical component schemas so pack-version / support-class, evidence-source, proving-source, and execution-boundary language stays one truth and a heuristic route never reads as an exact contract fact",
        "evidence:m5-framework-consumer-docs-browser:001",
        vec![
            binding(
                Family::RouteEndpointRow,
                vec![case(
                    Consumer::DocsBrowser,
                    Family::RouteEndpointRow,
                    Health::HeuristicEvidenceNarrowed,
                    &[Caveat::EvidenceHeuristicNotExact],
                    "docs / browser route / endpoint row narrowed by heuristic evidence",
                )],
            ),
            binding(
                Family::ComponentServiceTreeNode,
                vec![case(
                    Consumer::DocsBrowser,
                    Family::ComponentServiceTreeNode,
                    Health::FullParity,
                    &[],
                    "docs / browser component / service tree node at full parity",
                )],
            ),
        ],
    ));

    // 3. Onboarding — the framework pack header at full parity, and the generator preview sheet
    //    auto-narrowed because the generator write carries a file / dependency / config write effect
    //    disclosed before apply. This is a second write / boundary honesty case: a generator never
    //    implies a no-op write.
    rows.push(base_row(
        Consumer::Onboarding,
        "Onboarding surface owner",
        "Onboarding adopts the framework pack header at full parity and the generator preview sheet auto-narrowed because the generator write carries a file / dependency / config write effect disclosed before apply, keeping pack-version / support-class, evidence-source, proving-source, and execution-boundary language explicit so a generator never implies a safe or no-op write",
        "evidence:m5-framework-consumer-onboarding:001",
        vec![
            binding(
                Family::FrameworkPackHeader,
                vec![case(
                    Consumer::Onboarding,
                    Family::FrameworkPackHeader,
                    Health::FullParity,
                    &[],
                    "onboarding framework pack header at full parity",
                )],
            ),
            binding(
                Family::GeneratorPreviewSheet,
                vec![case(
                    Consumer::Onboarding,
                    Family::GeneratorPreviewSheet,
                    Health::ExecutionBoundaryOrWriteEffectNarrowed,
                    &[Caveat::ExecutionBoundaryOrWriteEffectDisclosedNotSilent],
                    "onboarding generator preview sheet narrowed by pending write-effect disclosure",
                )],
            ),
        ],
    ));

    // 4. Template registry — the route / endpoint row at full parity, plus the convention-diagnostic
    //    row auto-narrowed because the diagnostic is a heuristic suspicion rather than an exact contract
    //    fact, so a heuristic warning never overstates its confidence.
    rows.push(base_row(
        Consumer::TemplateRegistry,
        "Template-registry surface owner",
        "The template registry adopts the route / endpoint row at full parity and the convention-diagnostic row auto-narrowed because the diagnostic is a heuristic suspicion rather than an exact contract fact, referencing the canonical component schemas so pack-version / support-class, evidence-source, proving-source, and execution-boundary language stays one truth and a heuristic suspicion never reads as an exact fact",
        "evidence:m5-framework-consumer-template-registry:001",
        vec![
            binding(
                Family::RouteEndpointRow,
                vec![case(
                    Consumer::TemplateRegistry,
                    Family::RouteEndpointRow,
                    Health::FullParity,
                    &[],
                    "template-registry route / endpoint row at full parity",
                )],
            ),
            binding(
                Family::ConventionDiagnosticRow,
                vec![case(
                    Consumer::TemplateRegistry,
                    Family::ConventionDiagnosticRow,
                    Health::HeuristicEvidenceNarrowed,
                    &[Caveat::EvidenceHeuristicNotExact],
                    "template-registry convention-diagnostic row narrowed by heuristic evidence",
                )],
            ),
        ],
    ));

    // 5. Workflow bundle — the generator preview sheet auto-narrowed because a generator applied output
    //    requiring rollback or regenerate recovery, plus the run-config scaffold card at full parity, so
    //    the generated-versus-user-owned boundary and a rollback / regenerate path stay explicit.
    rows.push(base_row(
        Consumer::WorkflowBundle,
        "Workflow-bundle surface owner",
        "The workflow bundle adopts the generator preview sheet auto-narrowed because a generator applied output requiring rollback or regenerate recovery, and the run-config scaffold card at full parity, keeping pack-version / support-class, evidence-source, proving-source, and execution-boundary language explicit so the generated-versus-user-owned boundary and a rollback or regenerate path are never assumed final",
        "evidence:m5-framework-consumer-workflow-bundle:001",
        vec![
            binding(
                Family::GeneratorPreviewSheet,
                vec![case(
                    Consumer::WorkflowBundle,
                    Family::GeneratorPreviewSheet,
                    Health::RecoveryRequiredNarrowed,
                    &[Caveat::GeneratedOutputRecoverableNotFinal],
                    "workflow-bundle generator preview sheet narrowed by recovery-required generator write",
                )],
            ),
            binding(
                Family::RunConfigScaffoldCard,
                vec![case(
                    Consumer::WorkflowBundle,
                    Family::RunConfigScaffoldCard,
                    Health::FullParity,
                    &[],
                    "workflow-bundle run-config scaffold card at full parity",
                )],
            ),
        ],
    ));

    // 6. Visual designer — the component / service tree node at full parity, plus the
    //    derived-relationship banner auto-narrowed because the inferred relationship is heuristic rather
    //    than exact, so an inferred link never reads as exact where it is consumed.
    rows.push(base_row(
        Consumer::VisualDesigner,
        "Visual-designer surface owner",
        "The visual designer adopts the component / service tree node at full parity and the derived-relationship banner auto-narrowed because the inferred relationship is heuristic rather than exact from source, keeping pack-version / support-class, evidence-source, proving-source, and execution-boundary language explicit so an inferred link never reads as exact and the approximation appears exactly where it is consumed",
        "evidence:m5-framework-consumer-visual-designer:001",
        vec![
            binding(
                Family::ComponentServiceTreeNode,
                vec![case(
                    Consumer::VisualDesigner,
                    Family::ComponentServiceTreeNode,
                    Health::FullParity,
                    &[],
                    "visual-designer component / service tree node at full parity",
                )],
            ),
            binding(
                Family::DerivedRelationshipBanner,
                vec![case(
                    Consumer::VisualDesigner,
                    Family::DerivedRelationshipBanner,
                    Health::HeuristicEvidenceNarrowed,
                    &[Caveat::EvidenceHeuristicNotExact],
                    "visual-designer derived-relationship banner narrowed by heuristic evidence",
                )],
            ),
        ],
    ));

    // 7. Safe support / export packet — all seven families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering every other
    //    surface keeps parity with.
    rows.push(base_row(
        Consumer::SupportExport,
        "Safe support / export-packet surface owner",
        "The safe support / export packet adopts the framework pack header, route / endpoint row, component / service tree node, convention-diagnostic row, generator preview sheet, run-config scaffold card, and derived-relationship banner, referencing the canonical component schemas so its prose can never drift from the product truth and keeping pack-version / support-class, evidence-source, proving-source, and execution-boundary language exact in every exported case",
        "evidence:m5-framework-consumer-support-export:001",
        vec![
            binding(
                Family::FrameworkPackHeader,
                vec![case(
                    Consumer::SupportExport,
                    Family::FrameworkPackHeader,
                    Health::FullParity,
                    &[],
                    "safe support / export framework pack header at full parity",
                )],
            ),
            binding(
                Family::RouteEndpointRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::RouteEndpointRow,
                    Health::FullParity,
                    &[],
                    "safe support / export route / endpoint row at full parity",
                )],
            ),
            binding(
                Family::ComponentServiceTreeNode,
                vec![case(
                    Consumer::SupportExport,
                    Family::ComponentServiceTreeNode,
                    Health::FullParity,
                    &[],
                    "safe support / export component / service tree node at full parity",
                )],
            ),
            binding(
                Family::ConventionDiagnosticRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::ConventionDiagnosticRow,
                    Health::FullParity,
                    &[],
                    "safe support / export convention-diagnostic row at full parity",
                )],
            ),
            binding(
                Family::GeneratorPreviewSheet,
                vec![case(
                    Consumer::SupportExport,
                    Family::GeneratorPreviewSheet,
                    Health::FullParity,
                    &[],
                    "safe support / export generator preview sheet at full parity",
                )],
            ),
            binding(
                Family::RunConfigScaffoldCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::RunConfigScaffoldCard,
                    Health::FullParity,
                    &[],
                    "safe support / export run-config scaffold card at full parity",
                )],
            ),
            binding(
                Family::DerivedRelationshipBanner,
                vec![case(
                    Consumer::SupportExport,
                    Family::DerivedRelationshipBanner,
                    Health::FullParity,
                    &[],
                    "safe support / export derived-relationship banner at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5FrameworkComponentConsumerGovernanceReview {
    M5FrameworkComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        pack_evidence_boundary_and_recovery_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        heuristic_never_masquerades_as_exact: true,
        generator_never_implies_no_op_write_or_hides_boundary: true,
        support_export_presents_same_framework_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5FrameworkComponentConsumerProjection {
    M5FrameworkComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        pack_identity_and_support_reads_single_source: true,
        evidence_certainty_and_proving_source_reads_single_source: true,
        execution_boundary_and_impact_reads_single_source: true,
        recovery_and_rollback_boundary_reads_single_source: true,
    }
}

fn proof_freshness() -> M5FrameworkComponentConsumerProofFreshness {
    M5FrameworkComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5FrameworkComponentConsumerReleasePosture {
    M5FrameworkComponentConsumerReleasePosture {
        release_packet_ref: M5_FRAMEWORK_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        framework_component_consumer_audit_ref: M5_FRAMEWORK_COMPONENT_CONSUMER_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FRAMEWORK_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_CONSUMER_DOC_REF,
        M5_FRAMEWORK_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_FRAMEWORK_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5FrameworkComponentFamily::FrameworkPackHeader),
        family_canonical_schema_ref(M5FrameworkComponentFamily::RouteEndpointRow),
        family_canonical_schema_ref(M5FrameworkComponentFamily::ConventionDiagnosticRow),
        family_canonical_schema_ref(M5FrameworkComponentFamily::GeneratorPreviewSheet),
    ])
}

/// Builds the canonical M5 framework component-consumer packet.
pub fn seeded_m5_framework_component_consumer_packet() -> M5FrameworkComponentConsumerPacket {
    M5FrameworkComponentConsumerPacket::new(M5FrameworkComponentConsumerPacketInput {
        packet_id: M5_FRAMEWORK_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 framework component consumers: the preview runtime, docs / browser, onboarding, the template registry, workflow bundles, the visual designer, and the safe support / export packet keep pack-version / support-class, evidence-source, proving-source, and local-versus-remote execution-boundary parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5FrameworkComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the preview runtime is held at Beta because a slice of execution-boundary
/// evidence is still pending; every consumer stays visible.
pub fn seeded_m5_framework_component_consumer_preview_runtime_beta_narrowed(
) -> M5FrameworkComponentConsumerPacket {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.packet_id = "m5-framework-component-consumer:preview-runtime-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5FrameworkComponentConsumer::PreviewRuntime)
        .expect("preview-runtime row present");
    row.qualification = M5FrameworkQualificationClass::Beta;
    packet
}

/// Narrowed variant: onboarding is held at Preview because a slice of generator write-effect evidence
/// is still pending; every consumer stays visible.
pub fn seeded_m5_framework_component_consumer_onboarding_preview_narrowed(
) -> M5FrameworkComponentConsumerPacket {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.packet_id = "m5-framework-component-consumer:onboarding-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5FrameworkComponentConsumer::Onboarding)
        .expect("onboarding row present");
    row.qualification = M5FrameworkQualificationClass::Preview;
    packet
}
