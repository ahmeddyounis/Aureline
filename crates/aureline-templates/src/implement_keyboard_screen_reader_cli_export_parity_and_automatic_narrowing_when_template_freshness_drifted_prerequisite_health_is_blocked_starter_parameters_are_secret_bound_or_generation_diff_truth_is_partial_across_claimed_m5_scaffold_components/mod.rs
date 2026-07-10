//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the M5
//! scaffold-template-card / starter-parameter-row / scaffold-preflight-card / template-health-row /
//! generated-project-diff-card / scaffold-handoff-banner components.
//!
//! This module is the M05-1026 accessibility-and-auto-narrowing capstone over the frozen M5
//! scaffold-component matrix
//! ([`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`]).
//! Where the freeze matrix defines the reusable scaffold template card, starter parameter row,
//! scaffold preflight card, template health row, generated-project diff card, and scaffold handoff
//! banner primitives, and the 1021-1025 implementation / boundary / consumer lanes resolve their
//! per-surface truth, this lane certifies — per component family — that scaffold claims stay
//! **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than
//! presenting a drifted template, a blocked prerequisite, a secret-bound parameter, a partial
//! generation diff, or a cached/not-checked validation as still a fully qualified, ready-to-run
//! starter:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same starter source class,
//!   support class, host boundary, side-effect disclosure, parameter source, health freshness, and
//!   generated-versus-user-owned recovery boundary the rich component shows — never a hover-only
//!   chip that strands assistive-tech or headless-CLI users. Hierarchy-heavy families (the
//!   generated-project diff card's nested create / modify / rename / delete file tree) additionally
//!   bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning
//!   from typed tokens and opaque refs **without a raw value** — never a raw secret parameter value
//!   or raw generated file payload — preserving the same stable component identity, source /
//!   support posture, side-effect disclosure, health freshness, recovery boundary, and narrowing
//!   reasons shown in-product so support, docs, and release proof can reconstruct exactly what the
//!   user was actually shown without leaking a blocked secret value.
//! - **Honest auto-narrowing.** When a template's freshness drifts, a prerequisite health check is
//!   blocked, a starter parameter is secret-bound and cannot travel, a generation diff's truth is
//!   partial, or a validation state is cached / not checked, the component's readiness claim
//!   auto-narrows from `QualifiedStarter` to a secret-bound-parameter / blocked-prerequisite /
//!   drifted-template / partial-generation / unchecked-validation projection, discloses the
//!   narrowing with a precise trigger and binding dimension, and preserves the canonical starter
//!   source / support / recovery boundary. The underlying starter source and recovery path is never
//!   dropped opaquely. A component with every dimension intact must NOT carry a spurious narrowing,
//!   and a drifted-template / partial-generation / unchecked-validation state can never keep a
//!   fully-qualified starter claim — incomplete readiness evidence never presents a starter as
//!   ready.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the start center, template
//!   gallery, parameter form, preflight, diff review, workspace, health dashboard, the CLI surface,
//!   and the support export so product, docs, and release publication stay aligned on downgrade
//!   behavior rather than drifting in copy — a qualified-looking surface can never outrun the
//!   source / support / freshness / generation proof it is being viewed away from.
//!
//! Each [`ScaffoldComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::M5ScaffoldComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5ScaffoldRequiredLabel`] and
//! [`M5ScaffoldDowngradeTrigger`] and the shared [`M5ScaffoldConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the matrix
//! and the sibling primitive packets.
//!
//! The packet is metadata-only: raw generated file bodies, credentials, tokens, and secret
//! parameter values never cross this boundary; the packet carries only typed class tokens, opaque
//! scaffold refs, booleans, and controlled labels so support, release, and diagnostics exports can
//! reconstruct exactly what an accessible fallback would have shown without leaking sensitive
//! material or a raw value.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix::{
    M5ScaffoldComponentFamily, M5ScaffoldConsumerSurface, M5ScaffoldDowngradeTrigger,
    M5ScaffoldRequiredLabel, M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1026 scaffold-component accessibility fallback packet.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ScaffoldComponentAccessibilityPacket`].
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_scaffold_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ScaffoldComponentAccessibilityRow`].
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_scaffold_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/templates/m5_scaffold_component_accessibility_fallback.md";

/// Repo-relative path of the frozen scaffold-component matrix this lane certifies.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    M5_SCAFFOLD_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-scaffold-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-scaffold-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-scaffold-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SCAFFOLD_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-scaffold-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the generated-project diff
/// card's nested created / modified / renamed / deleted file tree) and therefore MUST bind their
/// tree to an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5ScaffoldComponentFamily) -> bool {
    matches!(family, M5ScaffoldComponentFamily::GeneratedProjectDiffCard)
}

/// The scaffold dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5ScaffoldComponentFamily,
) -> M5ScaffoldComponentClaimDimension {
    match family {
        M5ScaffoldComponentFamily::ScaffoldTemplateCard => {
            M5ScaffoldComponentClaimDimension::StarterTrustIntegrity
        }
        M5ScaffoldComponentFamily::StarterParameterRow => {
            M5ScaffoldComponentClaimDimension::ParameterPortability
        }
        M5ScaffoldComponentFamily::ScaffoldPreflightCard => {
            M5ScaffoldComponentClaimDimension::PrerequisiteHealth
        }
        M5ScaffoldComponentFamily::TemplateHealthRow => {
            M5ScaffoldComponentClaimDimension::TemplateFreshness
        }
        M5ScaffoldComponentFamily::GeneratedProjectDiffCard => {
            M5ScaffoldComponentClaimDimension::GenerationDiffEvidence
        }
        M5ScaffoldComponentFamily::ScaffoldHandoffBanner => {
            M5ScaffoldComponentClaimDimension::HandoffValidationClarity
        }
    }
}

/// A rendered fallback modality for a scaffold component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentFallbackModality {
    /// A rich, structured (nested created / modified / renamed / deleted file tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5ScaffoldComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentRenderingSurface {
    /// The full-capability desktop scaffold surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5ScaffoldComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl ScaffoldComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without leaking a raw value (a
/// raw secret parameter value or raw generated file body).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw value.
    ReconstructableWithoutRawValue,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw value (red).
    RequiresRawValue,
}

impl ScaffoldComponentExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw value.
    pub const fn never_requires_raw_value(self) -> bool {
        !matches!(self, Self::RequiresRawValue)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawValue => "reconstructable_without_raw_value",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawValue => "requires_raw_value",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ScaffoldComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The readiness claim ceiling a component asserts: how strong a starter-qualification posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a scaffold dimension weakens so a
/// drifted template, a blocked prerequisite, a secret-bound parameter, a partial generation diff,
/// or a cached / not-checked validation can never keep an old `QualifiedStarter` label — incomplete
/// readiness evidence never masquerades as a fully qualified, ready-to-run starter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentClaim {
    /// Qualified starter: a fully-sourced, supported, fresh, side-effect-disclosed, ready-to-run
    /// starter — the strongest claim, a surface Aureline can present as exactly true right now.
    QualifiedStarter,
    /// Secret-bound-parameter projection: a starter parameter is bound to a secret reference that
    /// cannot travel; the surface stays a secret-bound-parameter projection that names the
    /// parameter and its source layer, never exporting or committing a raw value.
    SecretBoundParameterProjection,
    /// Blocked-prerequisite projection: a prerequisite health check is blocked; the surface stays a
    /// blocked-prerequisite projection with its blocked check and recovery path preserved, never a
    /// passed preflight.
    BlockedPrerequisiteProjection,
    /// Drifted-template projection: the template's freshness has drifted; the surface stays a
    /// drifted-template projection with its last-known freshness and source preserved, never a
    /// currently-fresh starter.
    DriftedTemplateProjection,
    /// Partial-generation projection: the generation diff's truth is only partial; the surface
    /// stays a partial-generation projection with its generated-versus-user-owned boundary and
    /// recovery path preserved, never a clean applied change.
    PartialGenerationProjection,
    /// Unchecked-validation projection: a validation state is cached or not checked; the surface
    /// stays an unchecked-validation projection with its last-known validation and recovery path
    /// preserved, never a freshly-verified starter.
    UncheckedValidationProjection,
}

impl M5ScaffoldComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::QualifiedStarter,
        Self::SecretBoundParameterProjection,
        Self::BlockedPrerequisiteProjection,
        Self::DriftedTemplateProjection,
        Self::PartialGenerationProjection,
        Self::UncheckedValidationProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::QualifiedStarter => 5,
            Self::SecretBoundParameterProjection => 4,
            Self::BlockedPrerequisiteProjection => 3,
            Self::DriftedTemplateProjection => 2,
            Self::PartialGenerationProjection => 1,
            Self::UncheckedValidationProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully qualified, ready-to-run starter.
    pub const fn asserts_qualified_starter(self) -> bool {
        matches!(self, Self::QualifiedStarter)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedStarter => "qualified_starter",
            Self::SecretBoundParameterProjection => "secret_bound_parameter_projection",
            Self::BlockedPrerequisiteProjection => "blocked_prerequisite_projection",
            Self::DriftedTemplateProjection => "drifted_template_projection",
            Self::PartialGenerationProjection => "partial_generation_projection",
            Self::UncheckedValidationProjection => "unchecked_validation_projection",
        }
    }
}

/// The scaffold dimension whose state governs how far a component may claim to be a fully qualified
/// starter. The dimensions map 1:1 to the six frozen component families so every family carries an
/// honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentClaimDimension {
    /// Starter-trust integrity: is the starter source / support / validation currently verified?
    StarterTrustIntegrity,
    /// Parameter portability: can the starter parameters travel, or is a value secret-bound?
    ParameterPortability,
    /// Prerequisite health: is the prerequisite preflight clear, or blocked?
    PrerequisiteHealth,
    /// Template freshness: is the template currently fresh, or has its freshness drifted?
    TemplateFreshness,
    /// Generation-diff evidence: is the generation diff truth complete, or only partial?
    GenerationDiffEvidence,
    /// Handoff validation clarity: is the post-bootstrap validation checked, or cached / not-run?
    HandoffValidationClarity,
}

impl M5ScaffoldComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StarterTrustIntegrity,
        Self::ParameterPortability,
        Self::PrerequisiteHealth,
        Self::TemplateFreshness,
        Self::GenerationDiffEvidence,
        Self::HandoffValidationClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StarterTrustIntegrity => "starter_trust_integrity",
            Self::ParameterPortability => "parameter_portability",
            Self::PrerequisiteHealth => "prerequisite_health",
            Self::TemplateFreshness => "template_freshness",
            Self::GenerationDiffEvidence => "generation_diff_evidence",
            Self::HandoffValidationClarity => "handoff_validation_clarity",
        }
    }
}

/// The observed condition of one scaffold dimension. Anything weaker than
/// [`Self::StarterVerifiedReady`] imposes a narrowing ceiling on the component's readiness claim.
/// The three spec axes the lane must auto-narrow on as *incomplete readiness evidence* — a drifted
/// template, a partial generation diff, and a cached / not-checked validation — are the states that
/// [`Self::cannot_be_proven_qualified`] flags. A secret-bound parameter and a blocked prerequisite
/// are honest privacy / operational states, not readiness overstatements, so they are deliberately
/// excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentConditionState {
    /// Fully sourced, supported, fresh, and validated — imposes no ceiling.
    StarterVerifiedReady,
    /// A starter parameter is bound to a secret reference that cannot travel — readiness claim
    /// drops to a secret-bound-parameter projection.
    SecretBoundParameter,
    /// A prerequisite health check is blocked — readiness claim drops to a blocked-prerequisite
    /// projection.
    PrerequisiteBlocked,
    /// The template's freshness has drifted — readiness claim drops to a drifted-template
    /// projection.
    FreshnessDrifted,
    /// The generation diff's truth is only partial — readiness claim drops to a partial-generation
    /// projection.
    GenerationDiffPartial,
    /// A validation state is cached or not checked — readiness claim drops to an
    /// unchecked-validation projection.
    ValidationStale,
}

impl M5ScaffoldComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StarterVerifiedReady,
        Self::SecretBoundParameter,
        Self::PrerequisiteBlocked,
        Self::FreshnessDrifted,
        Self::GenerationDiffPartial,
        Self::ValidationStale,
    ];

    /// Returns true when the dimension is weaker than ready and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::StarterVerifiedReady)
    }

    /// Returns true when the condition reflects incomplete readiness evidence that cannot be proven
    /// a fully qualified, ready-to-run starter and must never be shown as such. A secret-bound
    /// parameter and a blocked prerequisite are honest privacy / operational states, not readiness
    /// overstatements, so they are deliberately excluded here.
    pub const fn cannot_be_proven_qualified(self) -> bool {
        matches!(
            self,
            Self::FreshnessDrifted | Self::GenerationDiffPartial | Self::ValidationStale
        )
    }

    /// The strongest readiness claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ScaffoldComponentClaim {
        match self {
            Self::StarterVerifiedReady => M5ScaffoldComponentClaim::QualifiedStarter,
            Self::SecretBoundParameter => M5ScaffoldComponentClaim::SecretBoundParameterProjection,
            Self::PrerequisiteBlocked => M5ScaffoldComponentClaim::BlockedPrerequisiteProjection,
            Self::FreshnessDrifted => M5ScaffoldComponentClaim::DriftedTemplateProjection,
            Self::GenerationDiffPartial => M5ScaffoldComponentClaim::PartialGenerationProjection,
            Self::ValidationStale => M5ScaffoldComponentClaim::UncheckedValidationProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ScaffoldDowngradeTrigger {
        match self {
            // The ready baseline never narrows; kept for exhaustiveness.
            Self::StarterVerifiedReady => M5ScaffoldDowngradeTrigger::ProofStale,
            Self::SecretBoundParameter => M5ScaffoldDowngradeTrigger::ParameterSourceUnstated,
            Self::PrerequisiteBlocked => M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
            Self::FreshnessDrifted => M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
            Self::GenerationDiffPartial => M5ScaffoldDowngradeTrigger::GeneratedBoundaryBlurred,
            Self::ValidationStale => M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StarterVerifiedReady => "starter_verified_ready",
            Self::SecretBoundParameter => "secret_bound_parameter",
            Self::PrerequisiteBlocked => "prerequisite_blocked",
            Self::FreshnessDrifted => "freshness_drifted",
            Self::GenerationDiffPartial => "generation_diff_partial",
            Self::ValidationStale => "validation_stale",
        }
    }
}

/// One scaffold dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ScaffoldComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ScaffoldComponentConditionState,
}

/// An honest readiness-claim auto-narrow block. When a scaffold dimension weakens, the component's
/// readiness claim lowers to the permitted ceiling, names the binding dimension and frozen trigger,
/// and preserves the canonical starter source / support / recovery boundary rather than silently
/// dropping it — the underlying starter source and recovery path is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentClaimAutoNarrow {
    /// The readiness claim the component is narrowed to.
    pub narrowed_to: M5ScaffoldComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5ScaffoldComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ScaffoldDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical component identity, starter source / support, and recovery boundary are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying starter source / support / recovery boundary is preserved (never dropped)
    /// across the narrowing; must hold so secret-bound-parameter, blocked-prerequisite,
    /// drifted-template, partial-generation, and unchecked-validation states never fail opaquely.
    pub preserves_source_and_recovery: bool,
}

impl ScaffoldComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and the starter
    /// source / recovery boundary and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_source_and_recovery
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw value is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw value is never the only export; must always hold.
    pub raw_value_only_prohibited: bool,
}

impl ScaffoldComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and a raw-value-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_value_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ScaffoldComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ScaffoldComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a scaffold-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw value, over-claims readiness, or drops state silently
    /// (red).
    Stranded,
}

impl ScaffoldComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one scaffold-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentAccessibilityRow {
    /// Record kind; must equal [`SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ScaffoldComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the template / parameter / preflight / diff / handoff object this component
    /// represents; stays visible on every surface, so this is never empty.
    pub scaffold_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ScaffoldComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical starter source, support class, host
    /// boundary, side-effect disclosure, parameter source, health freshness, and recovery boundary
    /// as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ScaffoldComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ScaffoldComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ScaffoldComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ScaffoldComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ScaffoldComponentCopyExportParity,
    /// The full readiness claim this family asserts when every dimension is intact.
    pub full_scaffold_claim: M5ScaffoldComponentClaim,
    /// The observed condition of each modeled scaffold dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ScaffoldComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ScaffoldComponentClaimAutoNarrow>,
    /// Whether the underlying starter source / support / recovery boundary is preserved on this
    /// component regardless of narrowing; must hold so secret-bound-parameter, blocked-prerequisite,
    /// drifted-template, partial-generation, and unchecked-validation states never fail opaquely.
    pub source_and_recovery_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ScaffoldComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ScaffoldComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ScaffoldComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `StarterVerifiedReady` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5ScaffoldComponentClaimDimension,
    ) -> M5ScaffoldComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ScaffoldComponentConditionState::StarterVerifiedReady)
    }

    /// Whether any modeled dimension is weaker than ready.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest readiness claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5ScaffoldComponentClaim {
        let mut permitted = self.full_scaffold_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&ScaffoldComponentClaimConditionEntry> {
        let mut binding: Option<(&ScaffoldComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_scaffold_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5ScaffoldComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The readiness claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ScaffoldComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_scaffold_claim,
        }
    }

    /// AC / auto-narrowing honesty: a secret-bound parameter, a blocked prerequisite, a drifted
    /// template, a partial generation diff, or a cached / not-checked validation can no longer keep
    /// an old `QualifiedStarter` label. The effective claim never exceeds the permitted ceiling;
    /// when a dimension narrows below the full claim, an honest narrow block is present, narrows to
    /// exactly the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity and the source / recovery boundary. When nothing
    /// narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / readiness honesty: a drifted-template / partial-generation / unchecked-validation state
    /// never keeps a fully-qualified starter claim — incomplete readiness evidence never presents a
    /// starter as ready. When such a state is modeled, the effective claim must not assert
    /// `QualifiedStarter`.
    pub fn readiness_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_proven_qualified());
        !(has_unprovable_state && self.effective_claim().asserts_qualified_starter())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth —
    /// no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a non-visual
    /// fallback, and the export reconstructs meaning without a raw value.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.scaffold_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw value.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_value()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: secret-bound-parameter, blocked-prerequisite, drifted-template,
    /// partial-generation, and unchecked-validation states preserve the underlying starter source /
    /// support / recovery boundary. The row must assert `source_and_recovery_preserved`, and any
    /// narrow block must preserve the source / recovery boundary too.
    pub fn preserves_source_and_recovery_continuity(&self) -> bool {
        self.source_and_recovery_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_source_and_recovery)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on
    /// the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ScaffoldRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ScaffoldComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.readiness_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_source_and_recovery_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ScaffoldComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ScaffoldComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            ScaffoldComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.scaffold_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_scaffold_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1026 scaffold-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_readiness_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_source_and_recovery_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ScaffoldComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ScaffoldComponentAccessibilityRow>,
}

/// Checked-in M05-1026 scaffold-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ScaffoldComponentAccessibilityRow>,
    pub summary: ScaffoldComponentAccessibilitySummary,
}

impl ScaffoldComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ScaffoldComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ScaffoldComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_readiness_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_source_and_recovery_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ScaffoldComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ScaffoldComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ScaffoldComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Readiness claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ScaffoldComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ScaffoldConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ScaffoldComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ScaffoldConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&ScaffoldComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ScaffoldComponentAccessibilityStatus::Parity => green += 1,
                ScaffoldComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ScaffoldComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ScaffoldComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ScaffoldComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ScaffoldComponentAccessibilityRow::claim_is_honest),
            all_readiness_honesty_holds: self
                .rows
                .iter()
                .all(ScaffoldComponentAccessibilityRow::readiness_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ScaffoldComponentAccessibilityRow::export_preserves_meaning),
            all_source_and_recovery_preserved: self
                .rows
                .iter()
                .all(ScaffoldComponentAccessibilityRow::preserves_source_and_recovery_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ScaffoldComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ScaffoldComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ScaffoldComponentAccessibilityViolation::SchemaVersion {
                expected: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SCAFFOLD_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ScaffoldComponentAccessibilityViolation::RecordKind {
                expected: SCAFFOLD_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ScaffoldComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ScaffoldComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_proven_qualified())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(ScaffoldComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory scaffold label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ScaffoldComponentFallbackModality::Structured)
            {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a qualified starter for a weakened one.
            if !row.claim_is_honest() {
                violations.push(ScaffoldComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: a drifted-template / partial-generation / unchecked-validation state never keeps
            // a fully-qualified starter claim.
            if !row.readiness_honesty_holds() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::UnprovableStateShownAsQualified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without leaking a raw value.
            if !row.export_preserves_meaning() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::ExportRequiresRawValue {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: secret-bound-parameter, blocked-prerequisite, drifted-template,
            // partial-generation, and unchecked-validation states preserve starter source /
            // recovery boundary.
            if !row.preserves_source_and_recovery_continuity() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::SourceOrRecoveryDropped {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == ScaffoldComponentAccessibilityStatus::Stranded {
                violations.push(ScaffoldComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ScaffoldComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ScaffoldComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the ready baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ScaffoldComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every readiness claim tier appears as an effective claim, so the full narrowing
        // spectrum (qualified-starter → … → unchecked-validation) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ScaffoldComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Readiness honesty must be proven with at least one drifted-template / partial-generation /
        // unchecked-validation row in the packet, so the "cannot-prove never shown as qualified"
        // guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(ScaffoldComponentAccessibilityViolation::ReadinessHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the start-center, template-gallery,
        // parameter-form, preflight, diff-review, workspace, health-dashboard, CLI, and
        // support-export surfaces — so every consumer surface is exercised at least once across the
        // packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ScaffoldConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ScaffoldComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ScaffoldComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("scaffold-component accessibility fallback packet serializes"),
        ) {
            violations.push(ScaffoldComponentAccessibilityViolation::RawStarterMaterialInExport);
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
            .expect("scaffold-component accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_scaffold_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Scaffold-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ScaffoldComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_scaffold_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in scaffold-component accessibility fallback export.
pub fn current_m5_scaffold_component_a11y_fallback_export(
) -> Result<ScaffoldComponentAccessibilityPacket, ScaffoldComponentAccessibilityArtifactError> {
    let packet: ScaffoldComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-scaffold-component-accessibility-fallback/support_export.json"
    )))
    .map_err(ScaffoldComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScaffoldComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in scaffold-component accessibility fallback export.
#[derive(Debug)]
pub enum ScaffoldComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScaffoldComponentAccessibilityViolation>),
}

impl fmt::Display for ScaffoldComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "scaffold-component accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "scaffold-component accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ScaffoldComponentAccessibilityArtifactError {}

/// Validation failure for M05-1026 scaffold-component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldComponentAccessibilityViolation {
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
    MissingPrimaryDimension {
        id: String,
        dimension: M5ScaffoldComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    UnprovableStateShownAsQualified {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawValue {
        id: String,
    },
    SourceOrRecoveryDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5ScaffoldComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5ScaffoldComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ScaffoldComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ScaffoldComponentClaim,
    },
    ReadinessHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ScaffoldConsumerSurface,
    },
    SummaryMismatch,
    RawStarterMaterialInExport,
}

impl ScaffoldComponentAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::HierarchyHeavyMissingStructured { .. } => "hierarchy_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::UnprovableStateShownAsQualified { .. } => "unprovable_state_shown_as_qualified",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawValue { .. } => "export_requires_raw_value",
            Self::SourceOrRecoveryDropped { .. } => "source_or_recovery_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::ReadinessHonestyUnproven => "readiness_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawStarterMaterialInExport => "raw_starter_material_in_export",
        }
    }
}

impl fmt::Display for ScaffoldComponentAccessibilityViolation {
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
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory scaffold label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a qualified starter for a weakened one, or narrows spuriously"
                )
            }
            Self::UnprovableStateShownAsQualified { id } => {
                write!(
                    f,
                    "row {id} shows a drifted-template / partial-generation / unchecked-validation state as a fully qualified starter"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawValue { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw value"
                )
            }
            Self::SourceOrRecoveryDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve the starter source / recovery boundary across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "readiness claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::ReadinessHonestyUnproven => {
                write!(
                    f,
                    "no drifted-template / partial-generation / unchecked-validation row is present to prove the readiness-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawStarterMaterialInExport => {
                write!(f, "export contains raw starter material")
            }
        }
    }
}

impl Error for ScaffoldComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
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
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "drifted"
            | "incomplete"
            | "not fresh"
            | "not qualified"
            | "unqualified"
            | "unchecked"
            | "not checked"
            | "secret bound"
            | "cannot travel"
            | "unverified"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. The governed tokens for
/// this lane legitimately name a `secret_reference` parameter source layer, so the bare word
/// "secret" is not treated as forbidden; a raw credential value (api key, password, PEM block, or
/// bearer token) still is.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in scaffold-component accessibility fallback packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_scaffold_component_a11y_fallback_packet() -> ScaffoldComponentAccessibilityPacket {
    ScaffoldComponentAccessibilityPacket::new(ScaffoldComponentAccessibilityPacketInput {
        packet_id: "m5-scaffold-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:scaffold-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ScaffoldRequiredLabel> {
    M5ScaffoldRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ScaffoldComponentCopyExportParity {
    ScaffoldComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_value_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ScaffoldComponentClaimDimension,
    state: M5ScaffoldComponentConditionState,
) -> ScaffoldComponentClaimConditionEntry {
    ScaffoldComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the CLI
/// surface — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ScaffoldConsumerSurface]) -> Vec<M5ScaffoldConsumerSurface> {
    let mut out = vec![
        M5ScaffoldConsumerSurface::SupportExport,
        M5ScaffoldConsumerSurface::CliSurface,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: ScaffoldComponentNarrowingDisclosureState,
) -> Vec<ScaffoldComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ScaffoldComponentRenderingNarrowingDisclosure {
            rendering_surface: M5ScaffoldComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ScaffoldComponentRenderingNarrowingDisclosure {
            rendering_surface: M5ScaffoldComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ScaffoldComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ScaffoldComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ScaffoldComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ScaffoldComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ScaffoldComponentRenderingSurface> {
    vec![
        M5ScaffoldComponentRenderingSurface::DesktopFull,
        M5ScaffoldComponentRenderingSurface::CliHeadless,
        M5ScaffoldComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5ScaffoldComponentFallbackModality> {
    vec![
        M5ScaffoldComponentFallbackModality::List,
        M5ScaffoldComponentFallbackModality::Textual,
        M5ScaffoldComponentFallbackModality::Cli,
    ]
}

fn seeded_rows() -> Vec<ScaffoldComponentAccessibilityRow> {
    vec![
        // Scaffold template card (verified / ready) — the starter source, support class, and
        // validation are all current, so it is a fully qualified starter and reachable on every
        // surface (green).
        ScaffoldComponentAccessibilityRow {
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:scaffold-template-card-verified".to_owned(),
            component_family: M5ScaffoldComponentFamily::ScaffoldTemplateCard,
            source_family_schema_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            scaffold_context_ref: "scaffold:scaffold-template-card:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ScaffoldComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:scaffold-template-card-verified:a11y".to_owned(),
            copy_export: copy_export(&[
                "template_identity",
                "starter_source_and_support",
                "host_boundary",
                "keyboard_route",
            ]),
            full_scaffold_claim: M5ScaffoldComponentClaim::QualifiedStarter,
            claim_conditions: vec![condition(
                M5ScaffoldComponentClaimDimension::StarterTrustIntegrity,
                M5ScaffoldComponentConditionState::StarterVerifiedReady,
            )],
            claim_narrow: None,
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "template_identity",
                "starter_source_and_support",
                "host_boundary",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ScaffoldConsumerSurface::StartCenterUi,
                M5ScaffoldConsumerSurface::TemplateGalleryUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.16 scaffold template cards".to_owned(),
                SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("scaffold-template-card-verified"),
        },
        // Starter parameter row (secret-bound) — a parameter is bound to a secret reference that
        // cannot travel, so the row auto-narrows to a secret-bound-parameter projection that names
        // the parameter and its source layer, never exporting or committing the raw value
        // (yellow).
        ScaffoldComponentAccessibilityRow {
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:starter-parameter-row-secret-bound".to_owned(),
            component_family: M5ScaffoldComponentFamily::StarterParameterRow,
            source_family_schema_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            scaffold_context_ref: "scaffold:starter-parameter-row:0002".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ScaffoldComponentNonVisualReachState::DisclosedReducedButReachable,
            export_summary: ScaffoldComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:starter-parameter-row-secret-bound:a11y".to_owned(),
            copy_export: copy_export(&[
                "parameter_identity",
                "parameter_source_layer",
                "portability_state",
                "secret_reference_disclosure",
            ]),
            full_scaffold_claim: M5ScaffoldComponentClaim::QualifiedStarter,
            claim_conditions: vec![condition(
                M5ScaffoldComponentClaimDimension::ParameterPortability,
                M5ScaffoldComponentConditionState::SecretBoundParameter,
            )],
            claim_narrow: Some(ScaffoldComponentClaimAutoNarrow {
                narrowed_to: M5ScaffoldComponentClaim::SecretBoundParameterProjection,
                binding_dimension: M5ScaffoldComponentClaimDimension::ParameterPortability,
                trigger: M5ScaffoldDowngradeTrigger::ParameterSourceUnstated,
                narrowed_label:
                    "This parameter is bound to a secret reference that cannot travel — shown as a secret-bound-parameter projection that names the parameter and its source layer, never exporting or committing the raw value"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "parameter_identity",
                "parameter_source_layer",
                "portability_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ScaffoldConsumerSurface::ParameterFormUi,
                M5ScaffoldConsumerSurface::TemplateGalleryUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.16 starter parameter rows".to_owned(),
                SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("starter-parameter-row-secret-bound"),
        },
        // Scaffold preflight card (prerequisite blocked) — a prerequisite health check is blocked,
        // so the card auto-narrows to a blocked-prerequisite projection that keeps its blocked
        // check and create-empty / continue-without-starter recovery visible, never a passed
        // preflight (yellow).
        ScaffoldComponentAccessibilityRow {
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:scaffold-preflight-card-blocked".to_owned(),
            component_family: M5ScaffoldComponentFamily::ScaffoldPreflightCard,
            source_family_schema_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            scaffold_context_ref: "scaffold:scaffold-preflight-card:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ScaffoldComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:scaffold-preflight-card-blocked:a11y".to_owned(),
            copy_export: copy_export(&[
                "preflight_identity",
                "blocked_check",
                "side_effect_disclosure",
                "recovery_path",
            ]),
            full_scaffold_claim: M5ScaffoldComponentClaim::QualifiedStarter,
            claim_conditions: vec![condition(
                M5ScaffoldComponentClaimDimension::PrerequisiteHealth,
                M5ScaffoldComponentConditionState::PrerequisiteBlocked,
            )],
            claim_narrow: Some(ScaffoldComponentClaimAutoNarrow {
                narrowed_to: M5ScaffoldComponentClaim::BlockedPrerequisiteProjection,
                binding_dimension: M5ScaffoldComponentClaimDimension::PrerequisiteHealth,
                trigger: M5ScaffoldDowngradeTrigger::HostBoundaryUnstated,
                narrowed_label:
                    "A prerequisite health check is blocked — shown as a blocked-prerequisite projection that names the blocked check and the create-empty or continue-without-starter recovery path, never as a passed preflight"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "preflight_identity",
                "blocked_check",
                "recovery_path",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ScaffoldConsumerSurface::PreflightUi,
                M5ScaffoldConsumerSurface::WorkspaceUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.16 generation preflights".to_owned(),
                SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("scaffold-preflight-card-blocked"),
        },
        // Template health row (freshness drifted) — the template's freshness has drifted, so the
        // row auto-narrows to a drifted-template projection that keeps its last-known freshness and
        // source visible, never as a currently-fresh starter (yellow). Drift is incomplete
        // readiness evidence, so it can never keep a qualified-starter claim.
        ScaffoldComponentAccessibilityRow {
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:template-health-row-drifted".to_owned(),
            component_family: M5ScaffoldComponentFamily::TemplateHealthRow,
            source_family_schema_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            scaffold_context_ref: "scaffold:template-health-row:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ScaffoldComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:template-health-row-drifted:a11y".to_owned(),
            copy_export: copy_export(&[
                "health_identity",
                "health_signal_class",
                "freshness_state",
                "drift_note",
            ]),
            full_scaffold_claim: M5ScaffoldComponentClaim::QualifiedStarter,
            claim_conditions: vec![condition(
                M5ScaffoldComponentClaimDimension::TemplateFreshness,
                M5ScaffoldComponentConditionState::FreshnessDrifted,
            )],
            claim_narrow: Some(ScaffoldComponentClaimAutoNarrow {
                narrowed_to: M5ScaffoldComponentClaim::DriftedTemplateProjection,
                binding_dimension: M5ScaffoldComponentClaimDimension::TemplateFreshness,
                trigger: M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
                narrowed_label:
                    "This template's freshness has drifted — shown as a drifted-template projection that names the last-known freshness and source, never as a currently-fresh qualified starter"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "health_identity",
                "health_signal_class",
                "freshness_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ScaffoldConsumerSurface::HealthDashboardUi,
                M5ScaffoldConsumerSurface::TemplateGalleryUi,
            ]),
            source_refs: vec![
                "UX Design System §16.45 template-health rows".to_owned(),
                SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("template-health-row-drifted"),
        },
        // Generated-project diff card (generation partial) — hierarchy-heavy (nested created /
        // modified / renamed / deleted file tree); the generation diff's truth is only partial, so
        // the card auto-narrows to a partial-generation projection and binds its nested file tree
        // to a flat list / textual path (yellow). Partial generation is incomplete readiness
        // evidence, so it can never keep a qualified-starter claim.
        ScaffoldComponentAccessibilityRow {
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:generated-project-diff-card-partial".to_owned(),
            component_family: M5ScaffoldComponentFamily::GeneratedProjectDiffCard,
            source_family_schema_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            scaffold_context_ref: "scaffold:generated-project-diff-card:0005".to_owned(),
            fallback_modalities: vec![
                M5ScaffoldComponentFallbackModality::Structured,
                M5ScaffoldComponentFallbackModality::List,
                M5ScaffoldComponentFallbackModality::Textual,
                M5ScaffoldComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ScaffoldComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ScaffoldComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:generated-project-diff-card-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "diff_identity",
                "created_modified_renamed_deleted_counts",
                "generated_versus_user_owned_boundary",
                "recovery_path",
            ]),
            full_scaffold_claim: M5ScaffoldComponentClaim::QualifiedStarter,
            claim_conditions: vec![condition(
                M5ScaffoldComponentClaimDimension::GenerationDiffEvidence,
                M5ScaffoldComponentConditionState::GenerationDiffPartial,
            )],
            claim_narrow: Some(ScaffoldComponentClaimAutoNarrow {
                narrowed_to: M5ScaffoldComponentClaim::PartialGenerationProjection,
                binding_dimension: M5ScaffoldComponentClaimDimension::GenerationDiffEvidence,
                trigger: M5ScaffoldDowngradeTrigger::GeneratedBoundaryBlurred,
                narrowed_label:
                    "This generation diff's truth is only partial — shown as a partial-generation projection that keeps the generated-versus-user-owned boundary and delete-generated recovery path visible, never as a clean applied change"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "diff_identity",
                "created_modified_renamed_deleted_counts",
                "generated_versus_user_owned_boundary",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ScaffoldConsumerSurface::DiffReviewUi,
                M5ScaffoldConsumerSurface::WorkspaceUi,
            ]),
            source_refs: vec![
                "TDD §7.2.15 workspace-bootstrap architecture".to_owned(),
                SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("generated-project-diff-card-partial"),
        },
        // Scaffold handoff banner (validation cached / not checked) — the post-bootstrap validation
        // state is cached or not yet checked, so the banner auto-narrows to an unchecked-validation
        // projection that keeps its last-known validation and delete-generated / reopen-preflight
        // recovery visible, never as a freshly-verified starter (yellow). A cached / not-checked
        // validation is incomplete readiness evidence, so it can never keep a qualified-starter
        // claim.
        ScaffoldComponentAccessibilityRow {
            record_kind: SCAFFOLD_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:scaffold-handoff-banner-unchecked".to_owned(),
            component_family: M5ScaffoldComponentFamily::ScaffoldHandoffBanner,
            source_family_schema_ref: SCAFFOLD_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            scaffold_context_ref: "scaffold:scaffold-handoff-banner:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: ScaffoldComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: ScaffoldComponentExportSummaryState::ReconstructableWithoutRawValue,
            export_summary_ref: "summary:scaffold-handoff-banner-unchecked:a11y".to_owned(),
            copy_export: copy_export(&[
                "handoff_identity",
                "trust_state",
                "validation_state",
                "recovery_path",
            ]),
            full_scaffold_claim: M5ScaffoldComponentClaim::QualifiedStarter,
            claim_conditions: vec![condition(
                M5ScaffoldComponentClaimDimension::HandoffValidationClarity,
                M5ScaffoldComponentConditionState::ValidationStale,
            )],
            claim_narrow: Some(ScaffoldComponentClaimAutoNarrow {
                narrowed_to: M5ScaffoldComponentClaim::UncheckedValidationProjection,
                binding_dimension: M5ScaffoldComponentClaimDimension::HandoffValidationClarity,
                trigger: M5ScaffoldDowngradeTrigger::HealthFreshnessStale,
                narrowed_label:
                    "This workspace's post-bootstrap validation is cached and not re-checked — shown as an unchecked-validation projection that keeps the last-known validation and the delete-generated or reopen-preflight recovery path visible, never as a freshly-verified qualified starter"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_source_and_recovery: true,
            }),
            source_and_recovery_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "handoff_identity",
                "trust_state",
                "validation_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ScaffoldConsumerSurface::WorkspaceUi,
                M5ScaffoldConsumerSurface::StartCenterUi,
            ]),
            source_refs: vec![
                "TDD §7.1.16 start center / workspace handoff".to_owned(),
                SCAFFOLD_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("scaffold-handoff-banner-unchecked"),
        },
    ]
}
