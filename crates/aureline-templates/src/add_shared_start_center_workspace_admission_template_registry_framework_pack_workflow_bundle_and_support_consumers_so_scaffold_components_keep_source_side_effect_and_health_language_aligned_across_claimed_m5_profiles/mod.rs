//! Shared consumers for the reusable M5 scaffold / project-entry components, so the scaffold
//! template card, starter parameter row, scaffold preflight card, template health row,
//! generated-project diff card, and scaffold handoff banner keep starter-source / support,
//! side-effect, template-health, and generated-versus-user-owned / recovery language aligned across
//! every claimed M5 surface that introduces or widens starter-based project entry: the start center,
//! workspace admission, the template registry, framework packs, workflow bundles, help / support
//! surfaces, and the safe handoff / export packet.
//!
//! Aureline's frozen scaffold-component matrix
//! (`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`)
//! names the six governed component families, and three sibling implement lanes narrow those
//! families into working primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the scaffold template card and starter parameter row
//!   (`implement_scaffold_template_cards_and_starter_parameter_rows_...`),
//! * the scaffold preflight card and template health row
//!   (`ship_scaffold_preflight_cards_and_template_health_rows_...`), and
//! * the generated-project diff card and scaffold handoff banner
//!   (`implement_generated_project_diff_cards_and_scaffold_handoff_banners_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the six families are
//! reusable components — not one start-center page plus a few isolated bootstrap objects — by
//! binding every claimed M5 scaffold consumer (the start center, workspace admission, the template
//! registry, framework packs, workflow bundles, help / support, and the safe handoff / export
//! packet) to the same canonical component schemas and the same descriptor vocabulary. Each consumer
//! points at the primitive's canonical schema and support-export artifact rather than re-wording
//! starter-source / support, side-effect, health-freshness, or generated-versus-user-owned /
//! recovery facts in local prose, and each keeps that vocabulary truthful even when a starter's
//! source or support is unverified, a starter carries an undisclosed network / dependency /
//! provisioning / trust / managed-workspace side effect, a template-health signal is stale, or a
//! partial or failed bootstrap leaves generated output that must stay recoverable.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_scaffold_component_binding`] — that takes one consumer's adoption of one
//!    component family, the descriptor set it surfaces, the parity-health mode it renders under, and
//!    any export caveats, and produces one [`M5ScaffoldComponentResolvedBinding`] carrying the
//!    derived claim-parity state and — whenever parity is weakened — a self-contained
//!    [`M5ScaffoldComponentAutoNarrowBanner`] that names the exact reason (an unverified source /
//!    support, a pending side-effect disclosure, a stale health signal, or a recovery-required
//!    partial generation), the descriptors that stay preserved, and the recovery action, rather than
//!    a generic "degraded" note. The resolver never lets a narrowed context drop a required
//!    descriptor and never lets a side-effect-bearing starter present as a plain ready create.
//! 2. A parity matrix — [`M5ScaffoldComponentConsumerPacket`] — that binds one row per claimed M5
//!    scaffold consumer to the six canonical component families, the one shared descriptor vocabulary,
//!    the same parity-health modes, export caveats, parity states, narrowing reasons, recovery
//!    actions, export fields, and non-visual accessibility routes, so starter-source / support,
//!    side-effect, health-freshness, and generated-versus-user-owned / recovery facts stop diverging
//!    between the gallery cards, entry review sheets, workflow-bundle surfaces, and support artifacts.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
//! classes, downgrade triggers, and the six component families themselves are reused verbatim from
//! the frozen scaffold-component matrix. This module mints new vocabulary only for what the adoption
//! lane itself needs: its scaffold consumers, the shared descriptor vocabulary, the parity-health
//! modes, the export caveats, the claim-parity states, the narrowing reasons and recovery actions,
//! the consumer anatomy parts, and the export fields.
//!
//! Raw file bodies, raw diffs, raw local paths, repository URLs, credentials, and secrets stay
//! outside the support boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is `schemas/ui/m5-scaffold-component-consumer.schema.json` and the contract
//! doc is `docs/templates/m5_scaffold_component_consumers.md`. The protected fixture directory is
//! `fixtures/ui/m5-scaffold-component-consumers/`.

#[cfg(test)]
mod tests;

// The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
// classes, downgrade triggers, and the six component families are frozen once, in the
// scaffold-component matrix. This adoption lane reuses them verbatim so it never invents a parallel
// scaffold vocabulary.
pub use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5ScaffoldAccessibilityRoute, M5ScaffoldComponentFamily, M5ScaffoldConsumerSurface,
    M5ScaffoldDeploymentLine, M5ScaffoldDowngradeTrigger, M5ScaffoldQualificationClass,
    M5ScaffoldSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather than
// re-wording their facts in local prose.
use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5_SCAFFOLD_COMPONENT_DOC_REF, M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_generated_project_diff_cards_and_scaffold_handoff_banners_with_create_modify_rename_delete_counts_dependency_task_extension_impact_trust_state_and_run_now_later_review_recovery_truth_across_claimed_m5_generation_flows::{
    SCAFFOLD_GENERATION_CONTROLS_ARTIFACT_REF, SCAFFOLD_GENERATION_CONTROLS_DOC_REF,
    SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF,
};
use crate::implement_scaffold_template_cards_and_starter_parameter_rows_with_source_support_host_boundary_and_portability_truth_across_claimed_m5_project_entry_surfaces::{
    SCAFFOLD_ENTRY_CONTROLS_ARTIFACT_REF, SCAFFOLD_ENTRY_CONTROLS_DOC_REF,
    SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF,
};
use crate::ship_scaffold_preflight_cards_and_template_health_rows_with_generated_file_counts_immediate_versus_deferred_actions_blocked_warning_optional_checks_and_create_empty_parity_across_claimed_m5_bootstrap_lanes::{
    SCAFFOLD_READINESS_CONTROLS_ARTIFACT_REF, SCAFFOLD_READINESS_CONTROLS_DOC_REF,
    SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ScaffoldComponentConsumerPacket`].
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_start_center_workspace_admission_template_registry_framework_pack_workflow_bundle_and_support_consumers_so_scaffold_components_keep_source_side_effect_and_health_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 scaffold component-consumer records.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the scaffold component-consumer boundary schema.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/templates/m5_scaffold_component_consumers.md";

/// Repo-relative path of the frozen scaffold-component matrix this lane adopts from.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_SCAFFOLD_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_SCAFFOLD_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-scaffold-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-scaffold-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-scaffold-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-scaffold-component-consumer-proof/report.md";

/// Stable packet id for the canonical scaffold component-consumer packet.
pub const M5_SCAFFOLD_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-scaffold-component-consumer:stable:0001";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer that
/// adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5ScaffoldComponentFamily) -> &'static str {
    use M5ScaffoldComponentFamily as Family;
    match family {
        Family::ScaffoldTemplateCard | Family::StarterParameterRow => {
            SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF
        }
        Family::ScaffoldPreflightCard | Family::TemplateHealthRow => {
            SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF
        }
        Family::GeneratedProjectDiffCard | Family::ScaffoldHandoffBanner => {
            SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5ScaffoldComponentFamily) -> &'static str {
    use M5ScaffoldComponentFamily as Family;
    match family {
        Family::ScaffoldTemplateCard | Family::StarterParameterRow => {
            SCAFFOLD_ENTRY_CONTROLS_DOC_REF
        }
        Family::ScaffoldPreflightCard | Family::TemplateHealthRow => {
            SCAFFOLD_READINESS_CONTROLS_DOC_REF
        }
        Family::GeneratedProjectDiffCard | Family::ScaffoldHandoffBanner => {
            SCAFFOLD_GENERATION_CONTROLS_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(family: M5ScaffoldComponentFamily) -> &'static str {
    use M5ScaffoldComponentFamily as Family;
    match family {
        Family::ScaffoldTemplateCard | Family::StarterParameterRow => {
            SCAFFOLD_ENTRY_CONTROLS_ARTIFACT_REF
        }
        Family::ScaffoldPreflightCard | Family::TemplateHealthRow => {
            SCAFFOLD_READINESS_CONTROLS_ARTIFACT_REF
        }
        Family::GeneratedProjectDiffCard | Family::ScaffoldHandoffBanner => {
            SCAFFOLD_GENERATION_CONTROLS_ARTIFACT_REF
        }
    }
}

/// One claimed M5 scaffold consumer that adopts the shared components. These are the consumers the
/// spec names — the start center, workspace admission, the template registry, framework packs,
/// workflow bundles, help / support surfaces, and the safe handoff / export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentConsumer {
    /// The start-center surface.
    StartCenter,
    /// The workspace-admission surface.
    WorkspaceAdmission,
    /// The template-registry surface.
    TemplateRegistry,
    /// The framework-pack surface.
    FrameworkPack,
    /// The workflow-bundle surface.
    WorkflowBundle,
    /// The help / support surface.
    HelpSupport,
    /// The safe handoff / export packet.
    SafeHandoffExport,
}

impl M5ScaffoldComponentConsumer {
    /// Every claimed scaffold consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::StartCenter,
        Self::WorkspaceAdmission,
        Self::TemplateRegistry,
        Self::FrameworkPack,
        Self::WorkflowBundle,
        Self::HelpSupport,
        Self::SafeHandoffExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::WorkspaceAdmission => "workspace_admission",
            Self::TemplateRegistry => "template_registry",
            Self::FrameworkPack => "framework_pack",
            Self::WorkflowBundle => "workflow_bundle",
            Self::HelpSupport => "help_support",
            Self::SafeHandoffExport => "safe_handoff_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StartCenter => "Start Center",
            Self::WorkspaceAdmission => "Workspace Admission",
            Self::TemplateRegistry => "Template Registry",
            Self::FrameworkPack => "Framework Pack",
            Self::WorkflowBundle => "Workflow Bundle",
            Self::HelpSupport => "Help / Support",
            Self::SafeHandoffExport => "Safe Handoff / Export Packet",
        }
    }

    /// True when this consumer is the safe handoff / export packet — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SafeHandoffExport)
    }
}

/// The one shared descriptor vocabulary every scaffold component keeps aligned across surfaces, so no
/// consumer invents a new grammar or stale wording. The descriptors in
/// [`M5ScaffoldComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that starter-source / support, side-effect, health-freshness, and
/// generated-versus-user-owned / recovery language stay one truth across gallery cards, entry review
/// sheets, workflow-bundle surfaces, and support artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentDescriptor {
    /// The source-and-support descriptor: starter source class, template support class, and host
    /// boundary.
    SourceAndSupport,
    /// The side-effect descriptor: network / dependency-install / remote-provisioning / trust /
    /// managed-workspace side effects, immediate-versus-deferred action timing, and file / dependency
    /// / task / extension impact.
    SideEffectDisclosure,
    /// The health-freshness descriptor: template-health signal and freshness state.
    HealthFreshness,
    /// The recovery-and-ownership descriptor: the generated-versus-user-owned boundary and the
    /// delete-generated / continue-without-starter recovery path.
    RecoveryAndOwnershipBoundary,
}

impl M5ScaffoldComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SourceAndSupport,
        Self::SideEffectDisclosure,
        Self::HealthFreshness,
        Self::RecoveryAndOwnershipBoundary,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceAndSupport => "source_and_support",
            Self::SideEffectDisclosure => "side_effect_disclosure",
            Self::HealthFreshness => "health_freshness",
            Self::RecoveryAndOwnershipBoundary => "recovery_and_ownership_boundary",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the authoritative
/// rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerParityHealth {
    /// Full parity: the authoritative rendering.
    FullParity,
    /// A starter's source or support class is unverified, so it is disclosed as community / mirrored
    /// / unknown rather than presented as governed first-party.
    SourceOrSupportUnverifiedNarrowed,
    /// A starter carries a network / dependency-install / remote-provisioning / trust /
    /// managed-workspace side effect, so it is disclosed before any create rather than routed silently
    /// through a generic Create.
    SideEffectPendingNarrowed,
    /// A template-health signal is stale, expired, or never checked, so it is disclosed as not-fresh
    /// rather than assumed current.
    HealthStaleNarrowed,
    /// A partial or failed bootstrap left generated output, so the generated-versus-user-owned
    /// boundary and a delete-generated / continue-without-starter recovery path stay explicit.
    RecoveryRequiredNarrowed,
}

impl M5ScaffoldConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::SourceOrSupportUnverifiedNarrowed,
        Self::SideEffectPendingNarrowed,
        Self::HealthStaleNarrowed,
        Self::RecoveryRequiredNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::SourceOrSupportUnverifiedNarrowed => "source_or_support_unverified_narrowed",
            Self::SideEffectPendingNarrowed => "side_effect_pending_narrowed",
            Self::HealthStaleNarrowed => "health_stale_narrowed",
            Self::RecoveryRequiredNarrowed => "recovery_required_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5ScaffoldConsumerNarrowingReason> {
        Some(match self {
            Self::SourceOrSupportUnverifiedNarrowed => {
                M5ScaffoldConsumerNarrowingReason::SourceOrSupportUnverified
            }
            Self::SideEffectPendingNarrowed => {
                M5ScaffoldConsumerNarrowingReason::SideEffectDisclosurePending
            }
            Self::HealthStaleNarrowed => M5ScaffoldConsumerNarrowingReason::HealthFreshnessStale,
            Self::RecoveryRequiredNarrowed => {
                M5ScaffoldConsumerNarrowingReason::RecoveryRequiredAfterPartialGeneration
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner never
/// reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerNarrowingReason {
    /// A starter's source or support class is unverified.
    SourceOrSupportUnverified,
    /// A starter carries a network / dependency / provisioning / trust / managed-workspace side
    /// effect that must be disclosed before any create.
    SideEffectDisclosurePending,
    /// A template-health signal is stale, expired, or never checked.
    HealthFreshnessStale,
    /// A partial or failed bootstrap left generated output that must stay recoverable.
    RecoveryRequiredAfterPartialGeneration,
}

impl M5ScaffoldConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SourceOrSupportUnverified,
        Self::SideEffectDisclosurePending,
        Self::HealthFreshnessStale,
        Self::RecoveryRequiredAfterPartialGeneration,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceOrSupportUnverified => "source_or_support_unverified",
            Self::SideEffectDisclosurePending => "side_effect_disclosure_pending",
            Self::HealthFreshnessStale => "health_freshness_stale",
            Self::RecoveryRequiredAfterPartialGeneration => {
                "recovery_required_after_partial_generation"
            }
        }
    }

    /// True when the reason reflects a starter carrying a network / dependency / provisioning / trust
    /// / managed-workspace side effect that must never be routed through a generic Create — the
    /// acceptance-criterion boundary that a generic Create never hides a side effect.
    pub const fn is_undisclosed_side_effect(self) -> bool {
        matches!(self, Self::SideEffectDisclosurePending)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::SourceOrSupportUnverified => {
                "a starter's source or support class is unverified, so it is disclosed as community / mirrored / unknown rather than presented as governed first-party"
            }
            Self::SideEffectDisclosurePending => {
                "a starter carries a network, dependency-install, remote-provisioning, trust, or managed-workspace side effect, so it is disclosed before any create rather than routed silently through a generic Create"
            }
            Self::HealthFreshnessStale => {
                "a template-health signal is stale, expired, or never checked, so it is disclosed as not-fresh rather than assumed current"
            }
            Self::RecoveryRequiredAfterPartialGeneration => {
                "a starter partially or fully materialized output, so the generated-versus-user-owned boundary and a delete-generated or continue-without-starter recovery path stay explicit rather than assumed final"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5ScaffoldConsumerRecoveryAction {
        match self {
            Self::SourceOrSupportUnverified => {
                M5ScaffoldConsumerRecoveryAction::InspectStarterSourceAndSupport
            }
            Self::SideEffectDisclosurePending => {
                M5ScaffoldConsumerRecoveryAction::ReviewSideEffectsBeforeCreate
            }
            Self::HealthFreshnessStale => {
                M5ScaffoldConsumerRecoveryAction::RerunHealthCheckBeforeTrusting
            }
            Self::RecoveryRequiredAfterPartialGeneration => {
                M5ScaffoldConsumerRecoveryAction::DeleteGeneratedOrContinueWithoutStarter
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable from the
/// banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerRecoveryAction {
    /// Inspect the starter's source and support class before trusting a first-party read.
    InspectStarterSourceAndSupport,
    /// Review the disclosed side effects before any create, rather than routing through a generic
    /// Create.
    ReviewSideEffectsBeforeCreate,
    /// Rerun the template-health check before trusting a stale signal.
    RerunHealthCheckBeforeTrusting,
    /// Delete generated output or continue without the starter after a partial or failed bootstrap.
    DeleteGeneratedOrContinueWithoutStarter,
}

impl M5ScaffoldConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectStarterSourceAndSupport,
        Self::ReviewSideEffectsBeforeCreate,
        Self::RerunHealthCheckBeforeTrusting,
        Self::DeleteGeneratedOrContinueWithoutStarter,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectStarterSourceAndSupport => "inspect_starter_source_and_support",
            Self::ReviewSideEffectsBeforeCreate => "review_side_effects_before_create",
            Self::RerunHealthCheckBeforeTrusting => "rerun_health_check_before_trusting",
            Self::DeleteGeneratedOrContinueWithoutStarter => {
                "delete_generated_or_continue_without_starter"
            }
        }
    }
}

/// An export caveat a consumer preserves when a component renders below full parity (an unverified
/// source / support, a pending side-effect disclosure, a stale health signal, or a recovery-required
/// partial generation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerExportCaveat {
    /// The starter's source or support class is unverified.
    SourceOrSupportUnverified,
    /// The starter's side effect is disclosed, not run silently.
    SideEffectDisclosedNotSilent,
    /// The template-health signal is stale, not fresh.
    HealthSignalStaleNotFresh,
    /// The generated output is recoverable, not final.
    GeneratedOutputRecoverableNotFinal,
}

impl M5ScaffoldConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SourceOrSupportUnverified,
        Self::SideEffectDisclosedNotSilent,
        Self::HealthSignalStaleNotFresh,
        Self::GeneratedOutputRecoverableNotFinal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceOrSupportUnverified => "source_or_support_unverified",
            Self::SideEffectDisclosedNotSilent => "side_effect_disclosed_not_silent",
            Self::HealthSignalStaleNotFresh => "health_signal_stale_not_fresh",
            Self::GeneratedOutputRecoverableNotFinal => "generated_output_recoverable_not_final",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is kept
/// aligned as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldClaimParityState {
    /// The descriptor vocabulary is kept aligned at full parity.
    ClaimsAligned,
    /// The descriptor vocabulary is kept aligned, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5ScaffoldClaimParityState {
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
/// [`M5ScaffoldConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerAnatomyPart {
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

impl M5ScaffoldConsumerAnatomyPart {
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
/// model. The fields in [`M5ScaffoldConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerExportField {
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

impl M5ScaffoldConsumerExportField {
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
pub struct M5ScaffoldComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5ScaffoldConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5ScaffoldConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5ScaffoldComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5ScaffoldComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5ScaffoldComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5ScaffoldConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors, and the
    /// recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the scaffold component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5ScaffoldComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5ScaffoldComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so
    /// source-and-support, side-effect, health-freshness, and recovery-and-ownership stay explicit.
    pub descriptor_families: Vec<M5ScaffoldComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5ScaffoldConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5ScaffoldConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5ScaffoldComponentConsumer,
    /// The component family.
    pub component_family: M5ScaffoldComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5ScaffoldComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5ScaffoldConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5ScaffoldConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5ScaffoldClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects a starter carrying a network / dependency / provisioning / trust
    /// / managed-workspace side effect. Such a binding must always be narrowed and never present as a
    /// plain ready create.
    pub reflects_undisclosed_side_effect_risk: bool,
    /// Hard invariant: whether this binding presents a ready-to-create starter without a caveat. Only
    /// a full-parity binding may present it; every narrowed binding — and in particular any
    /// side-effect-bearing one — resolves this to `false` so a generic Create never hides a side
    /// effect.
    pub presents_ready_starter_without_caveat: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5ScaffoldComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_scaffold_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ScaffoldComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5ScaffoldComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5ScaffoldComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "scaffold component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ScaffoldComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that source-and-support,
/// side-effect, health-freshness, and recovery-and-ownership stay explicit on every surface. The
/// claim-parity state is kept aligned at full parity and auto-narrowed under any weakened
/// parity-health mode, and a weakened mode always produces a self-contained banner naming the exact
/// reason and recovery action while keeping the descriptor vocabulary intact. A starter that carries
/// a side effect always narrows and never presents a plain ready create.
pub fn resolve_scaffold_component_binding(
    input: &M5ScaffoldComponentBindingInput,
) -> Result<M5ScaffoldComponentResolvedBinding, M5ScaffoldComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5ScaffoldComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5ScaffoldComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5ScaffoldComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5ScaffoldComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5ScaffoldComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text extension from
        // leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5ScaffoldComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_undisclosed_side_effect_risk =
        narrowing_reason.is_some_and(M5ScaffoldConsumerNarrowingReason::is_undisclosed_side_effect);
    // Only a full-parity binding may present a ready-to-create starter without a caveat. Every
    // narrowed binding — and every side-effect-bearing one in particular — does not.
    let presents_ready_starter_without_caveat = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5ScaffoldClaimParityState::ClaimsAutoNarrowed
    } else {
        M5ScaffoldClaimParityState::ClaimsAligned
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
        M5ScaffoldComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5ScaffoldComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_undisclosed_side_effect_risk,
        presents_ready_starter_without_caveat,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs consumer
/// parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentBindingCase {
    /// The resolver input.
    pub input: M5ScaffoldComponentBindingInput,
    /// The resolved truth. Must equal `resolve_scaffold_component_binding(&input)`.
    pub resolved: M5ScaffoldComponentResolvedBinding,
}

impl M5ScaffoldComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ScaffoldComponentBindingInput) -> Self {
        let resolved =
            resolve_scaffold_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_scaffold_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer points
/// at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5ScaffoldComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the family's
    /// canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description of its
    /// facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5ScaffoldComponentBindingCase>,
}

impl M5ScaffoldComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical family
    /// rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one scaffold consumer bound to the canonical component families,
/// the shared descriptor vocabulary, the parity-health modes, export caveats, parity states, narrowing
/// reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerRow {
    /// Scaffold consumer.
    pub consumer: M5ScaffoldComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5ScaffoldQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 scaffold surface families that render / consume this projection.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ScaffoldConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5ScaffoldComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5ScaffoldConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5ScaffoldConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5ScaffoldClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5ScaffoldConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5ScaffoldConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5ScaffoldConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Scaffold subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5ScaffoldComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new scaffold grammar. MUST be `false`.
    pub invents_new_scaffold_grammar: bool,
    /// Hard invariant: this consumer never drops source-and-support, side-effect, or recovery truth
    /// when narrowed. MUST be `false`.
    pub drops_source_support_side_effect_or_recovery_when_narrowed: bool,
    /// Hard invariant: this consumer never routes a network / dependency / provisioning / trust /
    /// managed-workspace side effect through a generic Create. MUST be `false`.
    pub routes_side_effect_through_generic_create: bool,
    /// Hard invariant: this consumer never blurs the generated-versus-user-owned boundary. MUST be
    /// `false`.
    pub blurs_generated_versus_user_owned_boundary: bool,
}

impl M5ScaffoldComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ScaffoldConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ScaffoldConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ScaffoldConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5ScaffoldConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5ScaffoldComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5ScaffoldComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5ScaffoldComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5ScaffoldComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_scaffold_grammar
            && !self.drops_source_support_side_effect_or_recovery_when_narrowed
            && !self.routes_side_effect_through_generic_create
            && !self.blurs_generated_versus_user_owned_boundary
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerVocabularySet {
    /// Scaffold-consumer tokens.
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

impl M5ScaffoldComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5ScaffoldComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5ScaffoldComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5ScaffoldComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5ScaffoldConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5ScaffoldConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5ScaffoldConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5ScaffoldConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5ScaffoldClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ScaffoldConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ScaffoldConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ScaffoldAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5ScaffoldComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new scaffold grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Source-and-support, side-effect, health-freshness, and recovery-and-ownership stay explicit
    /// everywhere.
    pub source_support_side_effect_health_and_recovery_explicit_on_every_surface: bool,
    /// An unverified source / support, a pending side-effect disclosure, a stale health signal, and a
    /// recovery-required partial generation auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// A network / dependency / provisioning / trust / managed-workspace side effect never routes
    /// through a generic Create.
    pub side_effect_never_routes_through_generic_create: bool,
    /// The support / export packet presents the same scaffold truth shown in-product.
    pub support_export_presents_same_scaffold_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerProjection {
    /// The start center, workspace admission, the template registry, framework packs, workflow
    /// bundles, help / support, and the safe handoff / export packet all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The source-and-support descriptor reads a single canonical source.
    pub source_and_support_reads_single_source: bool,
    /// The side-effect descriptor reads a single canonical source.
    pub side_effect_disclosure_reads_single_source: bool,
    /// The health-freshness descriptor reads a single canonical source.
    pub health_freshness_reads_single_source: bool,
    /// The recovery-and-ownership descriptor reads a single canonical source.
    pub recovery_and_ownership_boundary_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting scaffold-component consumer audit.
    pub scaffold_component_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ScaffoldComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ScaffoldComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ScaffoldComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ScaffoldComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ScaffoldComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ScaffoldComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ScaffoldComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ScaffoldComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 scaffold component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerPacket {
    /// Record kind; must equal [`M5_SCAFFOLD_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ScaffoldComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ScaffoldComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ScaffoldComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ScaffoldComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ScaffoldComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ScaffoldComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ScaffoldComponentConsumerPacket {
    /// Builds an M5 scaffold component-consumer packet from stable-lane input.
    pub fn new(input: M5ScaffoldComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_SCAFFOLD_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_VERSION,
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

    /// Validates the M5 scaffold component-consumer invariants.
    pub fn validate(&self) -> Vec<M5ScaffoldComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SCAFFOLD_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5ScaffoldComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5ScaffoldComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ScaffoldComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_side_effect_honesty(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 scaffold component consumer packet serializes"),
        ) {
            violations.push(M5ScaffoldComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 scaffold component consumer packet serializes")
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
        out.push_str("# M5 Scaffold Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Scaffold consumers: {} ({} stable)\n",
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
        out.push_str("\n## Scaffold consumers\n\n");
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

/// Errors emitted when reading the checked-in M5 scaffold component-consumer export.
#[derive(Debug)]
pub enum M5ScaffoldComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ScaffoldComponentConsumerViolation>),
}

impl fmt::Display for M5ScaffoldComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 scaffold component consumer export parse failed: {error}"
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
                    "m5 scaffold component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ScaffoldComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5ScaffoldComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ScaffoldComponentConsumerViolation {
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
    /// A required scaffold consumer is missing from the matrix.
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
    ScopePreservedUnproven,
    /// No worked binding proves that a side-effect-bearing starter narrows and never presents a plain
    /// ready create, or a binding does so incorrectly.
    SideEffectHonestyUnproven,
    /// The safe handoff / export packet consumer does not reference the canonical component schema.
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

impl M5ScaffoldComponentConsumerViolation {
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
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::SideEffectHonestyUnproven => "side_effect_honesty_unproven",
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

/// Reads and validates the checked-in stable M5 scaffold component-consumer export.
pub fn current_stable_m5_scaffold_component_consumer_export(
) -> Result<M5ScaffoldComponentConsumerPacket, M5ScaffoldComponentConsumerArtifactError> {
    let packet: M5ScaffoldComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-scaffold-component-consumer-proof/support_export.json"
    )))
    .map_err(M5ScaffoldComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ScaffoldComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_CONSUMER_DOC_REF,
        M5_SCAFFOLD_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_SCAFFOLD_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF,
        SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF,
        SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ScaffoldComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ScaffoldComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let present: BTreeSet<M5ScaffoldComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5ScaffoldComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5ScaffoldComponentConsumerViolation::RequiredConsumerMissing);
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
            violations.push(M5ScaffoldComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ScaffoldComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5ScaffoldComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ScaffoldComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ScaffoldComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ScaffoldComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ScaffoldComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5ScaffoldComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5ScaffoldComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5ScaffoldComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5ScaffoldComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ScaffoldComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ScaffoldComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one start-center
/// page plus a few isolated bootstrap objects.
fn validate_family_reuse(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    for family in M5ScaffoldComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5ScaffoldComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner carries
/// a specific reason, a recovery action, and a non-empty set of preserved descriptors — the
/// acceptance-criterion example that a consumer which cannot preserve parity is visibly narrowed
/// rather than silently dropping source, side-effect, health, or recovery language.
fn validate_narrowing_disclosure(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
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
        violations.push(M5ScaffoldComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with preserved
/// parity and no banner — the acceptance-criterion example that full-parity consumers keep the
/// descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5ScaffoldClaimParityState::ClaimsAligned
    });
    if !proven {
        violations.push(M5ScaffoldComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects a side-effect-bearing starter must be narrowed and must not
/// present a plain ready create, and at least one such binding must be present — the
/// acceptance-criterion that a generic Create never hides a network / dependency / provisioning /
/// trust / managed-workspace side effect on any claimed consumer.
fn validate_side_effect_honesty(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_undisclosed_side_effect_risk {
            // A side-effect-bearing binding that presents a plain ready create, or fails to narrow,
            // breaks the acceptance criterion.
            if resolved.presents_ready_starter_without_caveat
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5ScaffoldClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5ScaffoldComponentConsumerViolation::SideEffectHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5ScaffoldComponentConsumerViolation::SideEffectHonestyUnproven);
    }
}

/// The safe handoff / export packet consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that a support / export lane can never drift from the
/// product truth.
fn validate_support_export_reference(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5ScaffoldComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5ScaffoldComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.source_support_side_effect_health_and_recovery_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.side_effect_never_routes_through_generic_create,
        review.support_export_presents_same_scaffold_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ScaffoldComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.source_and_support_reads_single_source,
        projection.side_effect_disclosure_reads_single_source,
        projection.health_freshness_reads_single_source,
        projection.recovery_and_ownership_boundary_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ScaffoldComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ScaffoldComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ScaffoldComponentConsumerPacket,
    violations: &mut Vec<M5ScaffoldComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .scaffold_component_consumer_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ScaffoldComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5ScaffoldComponentConsumerPacket,
) -> impl Iterator<Item = &M5ScaffoldComponentBindingCase> {
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
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5ScaffoldComponentConsumer,
    component_family: M5ScaffoldComponentFamily,
    parity_health: M5ScaffoldConsumerParityHealth,
    export_caveats: &[M5ScaffoldConsumerExportCaveat],
    note: &str,
) -> M5ScaffoldComponentBindingCase {
    M5ScaffoldComponentBindingCase::resolved(M5ScaffoldComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5ScaffoldComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5ScaffoldComponentFamily,
    example_bindings: Vec<M5ScaffoldComponentBindingCase>,
) -> M5ScaffoldComponentBinding {
    M5ScaffoldComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5ScaffoldComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5ScaffoldComponentBinding>,
) -> M5ScaffoldComponentConsumerRow {
    M5ScaffoldComponentConsumerRow {
        consumer,
        qualification: M5ScaffoldQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ScaffoldConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5ScaffoldComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5ScaffoldConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5ScaffoldConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5ScaffoldClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5ScaffoldConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5ScaffoldConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5ScaffoldConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ScaffoldConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ScaffoldDowngradeTrigger::StarterSourceUnstated,
            M5ScaffoldDowngradeTrigger::SupportClassUnstated,
            M5ScaffoldDowngradeTrigger::SideEffectUndisclosed,
            M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
            M5ScaffoldDowngradeTrigger::GeneratedBoundaryBlurred,
            M5ScaffoldDowngradeTrigger::RecoveryPathOmitted,
            M5ScaffoldDowngradeTrigger::AlternateStateLabelInvented,
            M5ScaffoldDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_SCAFFOLD_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_scaffold_grammar: false,
        drops_source_support_side_effect_or_recovery_when_narrowed: false,
        routes_side_effect_through_generic_create: false,
        blurs_generated_versus_user_owned_boundary: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5ScaffoldComponentConsumerRow> {
    use M5ScaffoldComponentConsumer as Consumer;
    use M5ScaffoldComponentFamily as Family;
    use M5ScaffoldConsumerExportCaveat as Caveat;
    use M5ScaffoldConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Start center — the scaffold template card auto-narrowed because the starter's source or
    //    support class is unverified (a community starter), and the starter parameter row at full
    //    parity, so source / support, side-effect, health, and recovery language reads here as in
    //    every other claimed scaffold consumer.
    rows.push(base_row(
        Consumer::StartCenter,
        "Start-center surface owner",
        "The start center adopts the scaffold template card auto-narrowed because the starter's source or support class is unverified, and the starter parameter row at full parity, referencing the canonical component schemas so starter source / support, side-effect, health-freshness, and generated-versus-user-owned / recovery language appears here as in workspace admission, the template registry, framework packs, workflow bundles, help / support, and the safe handoff / export packet",
        "evidence:m5-scaffold-consumer-start-center:001",
        vec![
            binding(
                Family::ScaffoldTemplateCard,
                vec![case(
                    Consumer::StartCenter,
                    Family::ScaffoldTemplateCard,
                    Health::SourceOrSupportUnverifiedNarrowed,
                    &[Caveat::SourceOrSupportUnverified],
                    "start-center scaffold template card narrowed by unverified source or support",
                )],
            ),
            binding(
                Family::StarterParameterRow,
                vec![case(
                    Consumer::StartCenter,
                    Family::StarterParameterRow,
                    Health::FullParity,
                    &[],
                    "start-center starter parameter row at full parity",
                )],
            ),
        ],
    ));

    // 2. Workspace admission — the scaffold preflight card at full parity, and the scaffold handoff
    //    banner auto-narrowed because a partial bootstrap left generated output, so the
    //    generated-versus-user-owned boundary and a delete-generated / continue-without-starter
    //    recovery path stay explicit.
    rows.push(base_row(
        Consumer::WorkspaceAdmission,
        "Workspace-admission surface owner",
        "Workspace admission adopts the scaffold preflight card at full parity and the scaffold handoff banner auto-narrowed because a partial or failed bootstrap left generated output, keeping starter source / support, side-effect, health-freshness, and recovery language explicit so the generated-versus-user-owned boundary and a delete-generated or continue-without-starter path are never assumed final",
        "evidence:m5-scaffold-consumer-workspace-admission:001",
        vec![
            binding(
                Family::ScaffoldPreflightCard,
                vec![case(
                    Consumer::WorkspaceAdmission,
                    Family::ScaffoldPreflightCard,
                    Health::FullParity,
                    &[],
                    "workspace-admission scaffold preflight card at full parity",
                )],
            ),
            binding(
                Family::ScaffoldHandoffBanner,
                vec![case(
                    Consumer::WorkspaceAdmission,
                    Family::ScaffoldHandoffBanner,
                    Health::RecoveryRequiredNarrowed,
                    &[Caveat::GeneratedOutputRecoverableNotFinal],
                    "workspace-admission scaffold handoff banner narrowed by recovery-required partial generation",
                )],
            ),
        ],
    ));

    // 3. Template registry — the scaffold template card at full parity, plus the template health row
    //    auto-narrowed because a health signal is stale, so a stale, expired, or never-checked signal
    //    never reads as fresh.
    rows.push(base_row(
        Consumer::TemplateRegistry,
        "Template-registry surface owner",
        "The template registry adopts the scaffold template card at full parity and the template health row auto-narrowed because a template-health signal is stale, referencing the canonical component schemas so starter source / support, side-effect, health-freshness, and recovery language stay one truth and a stale signal never reads as current",
        "evidence:m5-scaffold-consumer-template-registry:001",
        vec![
            binding(
                Family::ScaffoldTemplateCard,
                vec![case(
                    Consumer::TemplateRegistry,
                    Family::ScaffoldTemplateCard,
                    Health::FullParity,
                    &[],
                    "template-registry scaffold template card at full parity",
                )],
            ),
            binding(
                Family::TemplateHealthRow,
                vec![case(
                    Consumer::TemplateRegistry,
                    Family::TemplateHealthRow,
                    Health::HealthStaleNarrowed,
                    &[Caveat::HealthSignalStaleNotFresh],
                    "template-registry template health row narrowed by stale health signal",
                )],
            ),
        ],
    ));

    // 4. Framework pack — the scaffold preflight card auto-narrowed because the pack starter carries a
    //    dependency-install / provisioning side effect that is disclosed before any create, plus the
    //    template health row at full parity. This is the side-effect honesty case: a side-effect
    //    starter never routes through a plain ready create.
    rows.push(base_row(
        Consumer::FrameworkPack,
        "Framework-pack surface owner",
        "The framework pack adopts the scaffold preflight card auto-narrowed because the pack starter carries a network / dependency-install / remote-provisioning / trust / managed-workspace side effect disclosed before any create, and the template health row at full parity, keeping starter source / support, side-effect, health-freshness, and recovery language explicit so a generic Create never hides a side effect",
        "evidence:m5-scaffold-consumer-framework-pack:001",
        vec![
            binding(
                Family::ScaffoldPreflightCard,
                vec![case(
                    Consumer::FrameworkPack,
                    Family::ScaffoldPreflightCard,
                    Health::SideEffectPendingNarrowed,
                    &[Caveat::SideEffectDisclosedNotSilent],
                    "framework-pack scaffold preflight card narrowed by pending side-effect disclosure",
                )],
            ),
            binding(
                Family::TemplateHealthRow,
                vec![case(
                    Consumer::FrameworkPack,
                    Family::TemplateHealthRow,
                    Health::FullParity,
                    &[],
                    "framework-pack template health row at full parity",
                )],
            ),
        ],
    ));

    // 5. Workflow bundle — the starter parameter row and the generated-project diff card at full
    //    parity: a workflow bundle reads the same parameter-source and generated-versus-user-owned
    //    truth the product renders.
    rows.push(base_row(
        Consumer::WorkflowBundle,
        "Workflow-bundle surface owner",
        "The workflow bundle adopts the starter parameter row and the generated-project diff card at full parity, referencing the canonical component schemas so starter source / support, side-effect, health-freshness, and recovery language stay one truth across desktop entry, workflow-bundle surfaces, and support artifacts rather than being re-worded per surface",
        "evidence:m5-scaffold-consumer-workflow-bundle:001",
        vec![
            binding(
                Family::StarterParameterRow,
                vec![case(
                    Consumer::WorkflowBundle,
                    Family::StarterParameterRow,
                    Health::FullParity,
                    &[],
                    "workflow-bundle starter parameter row at full parity",
                )],
            ),
            binding(
                Family::GeneratedProjectDiffCard,
                vec![case(
                    Consumer::WorkflowBundle,
                    Family::GeneratedProjectDiffCard,
                    Health::FullParity,
                    &[],
                    "workflow-bundle generated-project diff card at full parity",
                )],
            ),
        ],
    ));

    // 6. Help / support — the generated-project diff card auto-narrowed because a partial bootstrap
    //    left generated output requiring recovery, plus the scaffold handoff banner at full parity, so
    //    a support flow keeps the generated-versus-user-owned boundary and recovery path explicit.
    rows.push(base_row(
        Consumer::HelpSupport,
        "Help / support surface owner",
        "Help / support adopts the generated-project diff card auto-narrowed because a partial or failed bootstrap left generated output requiring recovery, and the scaffold handoff banner at full parity, keeping starter source / support, side-effect, health-freshness, and recovery language explicit so a support flow never blurs the generated-versus-user-owned boundary",
        "evidence:m5-scaffold-consumer-help-support:001",
        vec![
            binding(
                Family::GeneratedProjectDiffCard,
                vec![case(
                    Consumer::HelpSupport,
                    Family::GeneratedProjectDiffCard,
                    Health::RecoveryRequiredNarrowed,
                    &[Caveat::GeneratedOutputRecoverableNotFinal],
                    "help / support generated-project diff card narrowed by recovery-required partial generation",
                )],
            ),
            binding(
                Family::ScaffoldHandoffBanner,
                vec![case(
                    Consumer::HelpSupport,
                    Family::ScaffoldHandoffBanner,
                    Health::FullParity,
                    &[],
                    "help / support scaffold handoff banner at full parity",
                )],
            ),
        ],
    ));

    // 7. Safe handoff / export packet — all six families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering every other
    //    surface keeps parity with.
    rows.push(base_row(
        Consumer::SafeHandoffExport,
        "Safe handoff / export-packet surface owner",
        "The safe handoff / export packet adopts the scaffold template card, starter parameter row, scaffold preflight card, template health row, generated-project diff card, and scaffold handoff banner, referencing the canonical component schemas so its prose can never drift from the product truth and keeping starter source / support, side-effect, health-freshness, and generated-versus-user-owned / recovery language exact in every exported case",
        "evidence:m5-scaffold-consumer-safe-handoff-export:001",
        vec![
            binding(
                Family::ScaffoldTemplateCard,
                vec![case(
                    Consumer::SafeHandoffExport,
                    Family::ScaffoldTemplateCard,
                    Health::FullParity,
                    &[],
                    "safe handoff / export scaffold template card at full parity",
                )],
            ),
            binding(
                Family::StarterParameterRow,
                vec![case(
                    Consumer::SafeHandoffExport,
                    Family::StarterParameterRow,
                    Health::FullParity,
                    &[],
                    "safe handoff / export starter parameter row at full parity",
                )],
            ),
            binding(
                Family::ScaffoldPreflightCard,
                vec![case(
                    Consumer::SafeHandoffExport,
                    Family::ScaffoldPreflightCard,
                    Health::FullParity,
                    &[],
                    "safe handoff / export scaffold preflight card at full parity",
                )],
            ),
            binding(
                Family::TemplateHealthRow,
                vec![case(
                    Consumer::SafeHandoffExport,
                    Family::TemplateHealthRow,
                    Health::FullParity,
                    &[],
                    "safe handoff / export template health row at full parity",
                )],
            ),
            binding(
                Family::GeneratedProjectDiffCard,
                vec![case(
                    Consumer::SafeHandoffExport,
                    Family::GeneratedProjectDiffCard,
                    Health::FullParity,
                    &[],
                    "safe handoff / export generated-project diff card at full parity",
                )],
            ),
            binding(
                Family::ScaffoldHandoffBanner,
                vec![case(
                    Consumer::SafeHandoffExport,
                    Family::ScaffoldHandoffBanner,
                    Health::FullParity,
                    &[],
                    "safe handoff / export scaffold handoff banner at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ScaffoldComponentConsumerGovernanceReview {
    M5ScaffoldComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        source_support_side_effect_health_and_recovery_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        side_effect_never_routes_through_generic_create: true,
        support_export_presents_same_scaffold_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ScaffoldComponentConsumerProjection {
    M5ScaffoldComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        source_and_support_reads_single_source: true,
        side_effect_disclosure_reads_single_source: true,
        health_freshness_reads_single_source: true,
        recovery_and_ownership_boundary_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ScaffoldComponentConsumerProofFreshness {
    M5ScaffoldComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ScaffoldComponentConsumerReleasePosture {
    M5ScaffoldComponentConsumerReleasePosture {
        release_packet_ref: M5_SCAFFOLD_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        scaffold_component_consumer_audit_ref: M5_SCAFFOLD_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SCAFFOLD_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_CONSUMER_DOC_REF,
        M5_SCAFFOLD_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_SCAFFOLD_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5ScaffoldComponentFamily::ScaffoldTemplateCard),
        family_canonical_schema_ref(M5ScaffoldComponentFamily::ScaffoldPreflightCard),
        family_canonical_schema_ref(M5ScaffoldComponentFamily::GeneratedProjectDiffCard),
    ])
}

/// Builds the canonical M5 scaffold component-consumer packet.
pub fn seeded_m5_scaffold_component_consumer_packet() -> M5ScaffoldComponentConsumerPacket {
    M5ScaffoldComponentConsumerPacket::new(M5ScaffoldComponentConsumerPacketInput {
        packet_id: M5_SCAFFOLD_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 scaffold component consumers: the start center, workspace admission, the template registry, framework packs, workflow bundles, help / support, and the safe handoff / export packet keep starter source / support, side-effect, health-freshness, and generated-versus-user-owned / recovery parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5ScaffoldComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the framework pack is held at Beta because a slice of side-effect disclosure
/// evidence is still pending; every consumer stays visible.
pub fn seeded_m5_scaffold_component_consumer_framework_pack_beta_narrowed(
) -> M5ScaffoldComponentConsumerPacket {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.packet_id = "m5-scaffold-component-consumer:framework-pack-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ScaffoldComponentConsumer::FrameworkPack)
        .expect("framework-pack row present");
    row.qualification = M5ScaffoldQualificationClass::Beta;
    packet
}

/// Narrowed variant: workspace admission is held at Preview because a slice of bootstrap-recovery
/// evidence is still pending; every consumer stays visible.
pub fn seeded_m5_scaffold_component_consumer_workspace_admission_preview_narrowed(
) -> M5ScaffoldComponentConsumerPacket {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.packet_id = "m5-scaffold-component-consumer:workspace-admission-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ScaffoldComponentConsumer::WorkspaceAdmission)
        .expect("workspace-admission row present");
    row.qualification = M5ScaffoldQualificationClass::Preview;
    packet
}
