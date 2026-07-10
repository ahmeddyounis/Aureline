//! Two reusable M5 generator-review components — the generator-preview sheet and the run-config
//! scaffold card — so a framework-generated write or launch is review-first before it touches user
//! code or dispatches execution: every generator-preview sheet names its generator identity and
//! version, its parameters, its created-versus-modified paths, its managed-versus-user-owned files,
//! its dependency / config impact, and its rollback or regenerate posture, and never implies a safe
//! or no-op write when it changes files, dependencies, or config; every run-config scaffold card
//! names its target kind, its environment / profile, its launch command, its required toolchain, and
//! its local / container / SSH / managed execution boundary, so a user can see where code will run
//! and which toolchain is required before a convenience action dispatches execution. Neither
//! component implies a no-op write without explicit file / dependency / config review, hides the
//! execution boundary or the required toolchain behind framework convenience language, or omits its
//! rollback / regenerate recovery path.
//!
//! Aureline's frozen framework-component matrix
//! ([`crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`])
//! names the generator-preview sheet and the run-config scaffold card as two governed component
//! families and freezes their controlled vocabulary — the one controlled certainty disposition; the
//! generator impact classes (`file_write`, `dependency_change`, `config_change`,
//! `script_or_task_change`, `no_change`, `unknown_impact`) and generator apply postures
//! (`preview_ready`, `review_required`, `apply_ready`, `rollback_available`, `regenerate_available`,
//! `blocked`); the run-config mutation classes (`creates_config_file`, `edits_config_file`,
//! `adds_dependency`, `no_write_preview`, `rollback_available`, `unknown_mutation`) and execution
//! boundary classes (`local_process`, `container`, `ssh_remote`, `managed_workspace`, `cloud_remote`,
//! `unknown_boundary`); the surface families; the deployment lines; the consumer surfaces; the
//! accessibility routes; the required labels; and the downgrade triggers. This module *implements*
//! those contracts as two co-equal component vectors so a claimed M5 generator-review, run-config,
//! editor-gutter, CLI, or support-export surface can project a sheet and a card that keep the same
//! write-effect, impact, recovery, and execution-boundary truth.
//!
//! The module has two derived resolvers:
//!
//! * [`resolve_generator_preview_posture`] — takes a sheet's frozen generator impact class and apply
//!   posture and derives its write-effect posture (no-op preview, review-required write, reversible
//!   applied, or unknown / blocked), whether it has a side effect, whether it has a recovery path, and
//!   which notes it must carry — so a generator can never imply a safe or no-op write when it changes
//!   files, dependencies, or config.
//! * [`resolve_run_config_scaffold_posture`] — takes a card's frozen run-config mutation class and
//!   derives the same write-effect posture, so a run-config scaffold can never imply a no-op write when
//!   it creates or edits config or adds a dependency.
//!
//! A single controls packet — [`GeneratorPreviewRunConfigControlsPacket`] — binds one vector of
//! generator-preview sheets and one vector of run-config scaffold cards to the same write-effect,
//! recovery, and non-visual accessibility vocabulary, so write safety and execution-boundary truth
//! stay explicit across the generator-review, run-config, editor-gutter, CLI, and support consumers.
//!
//! The component family ([`M5FrameworkComponentFamily`]), generator impact class
//! ([`M5GeneratorImpactClass`]), generator apply posture ([`M5GeneratorApplyPosture`]), run-config
//! mutation class ([`M5RunConfigMutationClass`]), execution boundary class
//! ([`M5ExecutionBoundaryClass`]), certainty disposition ([`M5FrameworkCertaintyDisposition`]),
//! surface family ([`M5FrameworkSurfaceFamily`]), deployment line ([`M5FrameworkDeploymentLine`]),
//! consumer surface ([`M5FrameworkConsumerSurface`]), accessibility route
//! ([`M5FrameworkAccessibilityRoute`]), required label ([`M5FrameworkRequiredLabel`]), and downgrade
//! trigger ([`M5FrameworkDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the two components themselves:
//! the write-effect posture, the recovery path, the file-effect class, the file-ownership class, the
//! run-target kind, the launch-profile class, the toolchain readiness, and the bounded sheet and card
//! actions. No M5 generator-review surface invents a second write-effect or recovery grammar.
//!
//! Raw file bodies, raw generated trees, pasted local paths, repository URLs, credentials, and
//! secrets stay outside the export boundary; every note, recovery reference, and component identity is
//! carried only as an opaque, export-safe representation.

#[cfg(test)]
mod tests;

// The component family, the generator / run-config vocabularies, the certainty disposition, and the
// surface / deployment / consumer / accessibility / label / downgrade vocabularies are frozen once, in
// the framework-component matrix. This lane reuses them verbatim so it never invents a parallel
// write-effect or recovery vocabulary.
pub use crate::freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix::{
    M5ExecutionBoundaryClass, M5FrameworkAccessibilityRoute, M5FrameworkCertaintyDisposition,
    M5FrameworkComponentFamily, M5FrameworkConsumerSurface, M5FrameworkDeploymentLine,
    M5FrameworkDowngradeTrigger, M5FrameworkRequiredLabel, M5FrameworkSurfaceFamily,
    M5GeneratorApplyPosture, M5GeneratorImpactClass, M5RunConfigMutationClass,
    M5_FRAMEWORK_COMPONENT_DOC_REF, M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF, M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`GeneratorPreviewRunConfigControlsPacket`].
pub const GENERATOR_RUN_CONFIG_CONTROLS_RECORD_KIND: &str =
    "implement_generator_preview_sheets_and_run_config_scaffold_cards_with_generator_version_file_effect_classes_dependency_config_impact_rollback_or_regenerate_posture_required_toolchains_and_local_container_ssh_managed_target_truth";

/// Schema version for M5 generator-preview-sheet / run-config-scaffold-card control records.
pub const GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-generator-preview-run-config-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const GENERATOR_RUN_CONFIG_CONTROLS_DOC_REF: &str =
    "docs/frameworks/m5/m5_generator_preview_run_config_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const GENERATOR_RUN_CONFIG_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-generator-preview-run-config-controls";

/// Repo-relative path of the checked support-export artifact.
pub const GENERATOR_RUN_CONFIG_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-generator-preview-run-config-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const GENERATOR_RUN_CONFIG_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-generator-preview-run-config-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const GENERATOR_RUN_CONFIG_CONTROLS_REPORT_REF: &str =
    "artifacts/design/m5-generator-preview-run-config.md";

// ---- shared derived vocabulary ------------------------------------------

/// Derived write-effect posture a generator-preview sheet or run-config scaffold card may present.
/// These are the exact acceptance-criteria labels so a user can tell at a glance whether an action is
/// a genuine no-op preview, a review-required write, a reversible applied write, or an unknown /
/// blocked one — a generator or scaffold that changes files, dependencies, or config can never read as
/// a safe or no-op write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteEffectPosture {
    /// A genuine no-op: nothing is written, only a preview is shown.
    NoOpPreview,
    /// A write that changes files, dependencies, or config and must be reviewed before it applies.
    ReviewRequiredWrite,
    /// A write that is applied but reversible via rollback or regenerate.
    ReversibleApplied,
    /// The write impact is unknown, or the action is blocked.
    UnknownOrBlocked,
}

impl WriteEffectPosture {
    /// Every write-effect posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoOpPreview,
        Self::ReviewRequiredWrite,
        Self::ReversibleApplied,
        Self::UnknownOrBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOpPreview => "no_op_preview",
            Self::ReviewRequiredWrite => "review_required_write",
            Self::ReversibleApplied => "reversible_applied",
            Self::UnknownOrBlocked => "unknown_or_blocked",
        }
    }

    /// True only when the action is a genuine no-op preview.
    pub const fn is_no_op(self) -> bool {
        matches!(self, Self::NoOpPreview)
    }

    /// True when the action is a write that must be reviewed before it applies.
    pub const fn must_review_before_write(self) -> bool {
        matches!(self, Self::ReviewRequiredWrite)
    }

    /// True when the action is an applied but reversible write.
    pub const fn is_reversible_applied(self) -> bool {
        matches!(self, Self::ReversibleApplied)
    }
}

/// The kind of recovery a generator-preview sheet or run-config scaffold card keeps explicit, so a
/// component never applies a write without naming how it can be undone — rollback, regenerate, both,
/// a forward-fix-only path when no automatic undo exists, or no recovery needed for a genuine no-op.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPath {
    /// Rollback is available.
    Rollback,
    /// Regenerate is available.
    Regenerate,
    /// Both rollback and regenerate are available.
    RollbackAndRegenerate,
    /// No automatic undo exists; recovery is forward-fix only.
    ForwardFixOnly,
    /// No recovery is needed (a genuine no-op preview writes nothing).
    NoRecoveryNeeded,
}

impl RecoveryPath {
    /// Every recovery path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Rollback,
        Self::Regenerate,
        Self::RollbackAndRegenerate,
        Self::ForwardFixOnly,
        Self::NoRecoveryNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rollback => "rollback",
            Self::Regenerate => "regenerate",
            Self::RollbackAndRegenerate => "rollback_and_regenerate",
            Self::ForwardFixOnly => "forward_fix_only",
            Self::NoRecoveryNeeded => "no_recovery_needed",
        }
    }

    /// True when this kind names a reversible recovery (rollback and / or regenerate).
    pub const fn is_reversible(self) -> bool {
        matches!(
            self,
            Self::Rollback | Self::Regenerate | Self::RollbackAndRegenerate
        )
    }
}

// ---- generator-preview vocabulary ---------------------------------------

/// What a generator-preview sheet does to files, so a sheet never leaves its created-versus-modified
/// path distinction implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileEffectClass {
    /// Only creates new files.
    CreatesFile,
    /// Only modifies existing files.
    ModifiesFile,
    /// Creates new files and modifies existing ones.
    CreatesAndModifies,
    /// Touches no files.
    NoFileChange,
}

impl FileEffectClass {
    /// Every file-effect class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CreatesFile,
        Self::ModifiesFile,
        Self::CreatesAndModifies,
        Self::NoFileChange,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreatesFile => "creates_file",
            Self::ModifiesFile => "modifies_file",
            Self::CreatesAndModifies => "creates_and_modifies",
            Self::NoFileChange => "no_file_change",
        }
    }

    /// Whether this class implies at least one created path.
    pub const fn creates(self) -> bool {
        matches!(self, Self::CreatesFile | Self::CreatesAndModifies)
    }

    /// Whether this class implies at least one modified path.
    pub const fn modifies(self) -> bool {
        matches!(self, Self::ModifiesFile | Self::CreatesAndModifies)
    }
}

/// Who owns the files a generator-preview sheet touches, so a sheet never hides whether it writes
/// managed-generated files or user-owned code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileOwnershipClass {
    /// Managed, generator-owned files.
    ManagedGenerated,
    /// User-owned code.
    UserOwned,
    /// A mix of managed and user-owned files.
    MixedOwnership,
    /// Ownership is unknown.
    UnknownOwnership,
}

impl FileOwnershipClass {
    /// Every file-ownership class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ManagedGenerated,
        Self::UserOwned,
        Self::MixedOwnership,
        Self::UnknownOwnership,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedGenerated => "managed_generated",
            Self::UserOwned => "user_owned",
            Self::MixedOwnership => "mixed_ownership",
            Self::UnknownOwnership => "unknown_ownership",
        }
    }
}

/// One keyboard-complete default action a generator-preview sheet offers, so a sheet never hides its
/// diff-review, impact / ownership, or rollback / regenerate affordance behind a pointer-only
/// gesture. `ReviewCreatedAndModifiedDiff`, `InspectImpactAndOwnership`, and
/// `OpenRollbackOrRegenerate` are always offered so the created / modified diff, the dependency /
/// config impact and ownership, and the recovery path stay inspectable before a user applies the
/// generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorSheetAction {
    /// Review the created and modified diff (always available).
    ReviewCreatedAndModifiedDiff,
    /// Inspect the dependency / config impact and the managed / user ownership (always available).
    InspectImpactAndOwnership,
    /// Open the rollback or regenerate recovery path (always available).
    OpenRollbackOrRegenerate,
    /// Apply the generator after review.
    ApplyAfterReview,
    /// Copy the stable generator id.
    CopyGeneratorId,
}

impl GeneratorSheetAction {
    /// Every generator-sheet action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReviewCreatedAndModifiedDiff,
        Self::InspectImpactAndOwnership,
        Self::OpenRollbackOrRegenerate,
        Self::ApplyAfterReview,
        Self::CopyGeneratorId,
    ];

    /// The default actions every keyboard-complete generator-preview sheet must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::ReviewCreatedAndModifiedDiff,
        Self::InspectImpactAndOwnership,
        Self::OpenRollbackOrRegenerate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewCreatedAndModifiedDiff => "review_created_and_modified_diff",
            Self::InspectImpactAndOwnership => "inspect_impact_and_ownership",
            Self::OpenRollbackOrRegenerate => "open_rollback_or_regenerate",
            Self::ApplyAfterReview => "apply_after_review",
            Self::CopyGeneratorId => "copy_generator_id",
        }
    }
}

// ---- run-config-scaffold vocabulary -------------------------------------

/// The kind of target a run-config scaffold card turns framework metadata into, so a card never
/// leaves what it runs implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunTargetKind {
    /// A web application.
    WebApp,
    /// An API server.
    ApiServer,
    /// A CLI tool.
    CliTool,
    /// A test suite.
    TestSuite,
    /// A background job / worker.
    BackgroundJob,
    /// An unknown target.
    UnknownTarget,
}

impl RunTargetKind {
    /// Every run-target kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WebApp,
        Self::ApiServer,
        Self::CliTool,
        Self::TestSuite,
        Self::BackgroundJob,
        Self::UnknownTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WebApp => "web_app",
            Self::ApiServer => "api_server",
            Self::CliTool => "cli_tool",
            Self::TestSuite => "test_suite",
            Self::BackgroundJob => "background_job",
            Self::UnknownTarget => "unknown_target",
        }
    }
}

/// The environment / profile a run-config scaffold card launches under, so a card never leaves the
/// profile implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchProfileClass {
    /// A development profile.
    Development,
    /// A debug profile.
    Debug,
    /// A production profile.
    Production,
    /// A test profile.
    Test,
    /// A custom profile.
    CustomProfile,
}

impl LaunchProfileClass {
    /// Every launch-profile class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Development,
        Self::Debug,
        Self::Production,
        Self::Test,
        Self::CustomProfile,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Debug => "debug",
            Self::Production => "production",
            Self::Test => "test",
            Self::CustomProfile => "custom_profile",
        }
    }
}

/// Whether the toolchain a run-config scaffold card requires is ready, so a card never dispatches
/// execution while implying an absent or mismatched toolchain is present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolchainReadiness {
    /// The required toolchain is present and ready.
    ToolchainReady,
    /// The required toolchain is missing.
    ToolchainMissing,
    /// The required toolchain is present but a version is mismatched.
    ToolchainVersionMismatch,
    /// Toolchain readiness is unknown.
    ToolchainUnknown,
}

impl ToolchainReadiness {
    /// Every toolchain-readiness state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ToolchainReady,
        Self::ToolchainMissing,
        Self::ToolchainVersionMismatch,
        Self::ToolchainUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolchainReady => "toolchain_ready",
            Self::ToolchainMissing => "toolchain_missing",
            Self::ToolchainVersionMismatch => "toolchain_version_mismatch",
            Self::ToolchainUnknown => "toolchain_unknown",
        }
    }
}

/// One keyboard-complete default action a run-config scaffold card offers, so a card never hides its
/// execution-boundary, required-toolchain, or config-mutation affordance behind a pointer-only
/// gesture. `InspectExecutionBoundary`, `InspectRequiredToolchain`, and `ReviewConfigMutation` are
/// always offered so where the code runs, which toolchain is required, and what config is written stay
/// inspectable before a convenience action dispatches execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunConfigCardAction {
    /// Inspect the local / container / SSH / managed execution boundary (always available).
    InspectExecutionBoundary,
    /// Inspect the required toolchain (always available).
    InspectRequiredToolchain,
    /// Review the config mutation this card writes (always available).
    ReviewConfigMutation,
    /// Run the target after review.
    RunAfterReview,
    /// Copy the launch command.
    CopyLaunchCommand,
}

impl RunConfigCardAction {
    /// Every run-config-card action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InspectExecutionBoundary,
        Self::InspectRequiredToolchain,
        Self::ReviewConfigMutation,
        Self::RunAfterReview,
        Self::CopyLaunchCommand,
    ];

    /// The default actions every keyboard-complete run-config scaffold card must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::InspectExecutionBoundary,
        Self::InspectRequiredToolchain,
        Self::ReviewConfigMutation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectExecutionBoundary => "inspect_execution_boundary",
            Self::InspectRequiredToolchain => "inspect_required_toolchain",
            Self::ReviewConfigMutation => "review_config_mutation",
            Self::RunAfterReview => "run_after_review",
            Self::CopyLaunchCommand => "copy_launch_command",
        }
    }
}

// ---- resolvers ----------------------------------------------------------

/// Disclosures a generator-preview sheet or run-config scaffold card must carry, derived from its
/// frozen write axes. Both resolvers return this shared shape so write safety stays identical across
/// the two component families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteSafetyDisclosure {
    /// The derived write-effect posture this component may present.
    pub write_effect_posture: WriteEffectPosture,
    /// Whether the action is a genuine no-op preview.
    pub is_no_op: bool,
    /// Whether the action changes files, dependencies, or config.
    pub has_side_effect: bool,
    /// Whether the action must be reviewed before it writes.
    pub must_review_before_write: bool,
    /// Whether the action is an applied but reversible write.
    pub is_reversible_applied: bool,
    /// Whether the action has a reversible recovery path.
    pub has_recovery_path: bool,
    /// Whether the component must carry an explicit review-required note.
    pub needs_review_note: bool,
    /// Whether the component must carry an explicit applied-recovery note.
    pub needs_applied_recovery_note: bool,
    /// Whether the component must carry an explicit no-recovery note (a write with no reversible
    /// recovery path).
    pub needs_no_recovery_note: bool,
}

fn write_safety_disclosure(
    write_effect_posture: WriteEffectPosture,
    has_side_effect: bool,
    has_recovery_path: bool,
) -> WriteSafetyDisclosure {
    WriteSafetyDisclosure {
        write_effect_posture,
        is_no_op: write_effect_posture.is_no_op(),
        has_side_effect,
        must_review_before_write: write_effect_posture.must_review_before_write(),
        is_reversible_applied: write_effect_posture.is_reversible_applied(),
        has_recovery_path,
        needs_review_note: write_effect_posture.must_review_before_write(),
        needs_applied_recovery_note: write_effect_posture.is_reversible_applied(),
        needs_no_recovery_note: has_side_effect && !has_recovery_path,
    }
}

/// Resolves the write-effect and recovery truth a generator-preview sheet may present.
///
/// A `no_change` impact is a no-op preview; an `unknown_impact` impact or a `blocked` apply posture
/// is unknown / blocked; any writing impact (`file_write`, `dependency_change`, `config_change`,
/// `script_or_task_change`) with a `rollback_available` or `regenerate_available` apply posture is a
/// reversible applied write, and any other writing impact is a review-required write — so a generator
/// can never imply a safe or no-op write when it changes files, dependencies, or config.
pub fn resolve_generator_preview_posture(
    impact: M5GeneratorImpactClass,
    apply: M5GeneratorApplyPosture,
) -> WriteSafetyDisclosure {
    use M5GeneratorApplyPosture as Apply;
    use M5GeneratorImpactClass as Impact;
    use WriteEffectPosture as Posture;

    let has_side_effect = matches!(
        impact,
        Impact::FileWrite
            | Impact::DependencyChange
            | Impact::ConfigChange
            | Impact::ScriptOrTaskChange
    );
    let write_effect_posture =
        if matches!(impact, Impact::UnknownImpact) || matches!(apply, Apply::Blocked) {
            Posture::UnknownOrBlocked
        } else if !has_side_effect {
            Posture::NoOpPreview
        } else if matches!(apply, Apply::RollbackAvailable | Apply::RegenerateAvailable) {
            Posture::ReversibleApplied
        } else {
            Posture::ReviewRequiredWrite
        };
    let has_recovery_path = has_side_effect
        && matches!(
            apply,
            Apply::PreviewReady
                | Apply::ReviewRequired
                | Apply::ApplyReady
                | Apply::RollbackAvailable
                | Apply::RegenerateAvailable
        );

    write_safety_disclosure(write_effect_posture, has_side_effect, has_recovery_path)
}

/// Resolves the write-effect and recovery truth a run-config scaffold card may present.
///
/// A `no_write_preview` mutation is a no-op preview; an `unknown_mutation` is unknown / blocked; a
/// `rollback_available` mutation is a reversible applied write; and a `creates_config_file`,
/// `edits_config_file`, or `adds_dependency` mutation is a review-required write — so a run-config
/// scaffold can never imply a no-op write when it creates or edits config or adds a dependency.
pub fn resolve_run_config_scaffold_posture(
    mutation: M5RunConfigMutationClass,
) -> WriteSafetyDisclosure {
    use M5RunConfigMutationClass as Mutation;
    use WriteEffectPosture as Posture;

    let has_side_effect = matches!(
        mutation,
        Mutation::CreatesConfigFile | Mutation::EditsConfigFile | Mutation::AddsDependency
    );
    let write_effect_posture = match mutation {
        Mutation::UnknownMutation => Posture::UnknownOrBlocked,
        Mutation::NoWritePreview => Posture::NoOpPreview,
        Mutation::RollbackAvailable => Posture::ReversibleApplied,
        Mutation::CreatesConfigFile | Mutation::EditsConfigFile | Mutation::AddsDependency => {
            Posture::ReviewRequiredWrite
        }
    };
    let has_recovery_path = matches!(
        mutation,
        Mutation::CreatesConfigFile
            | Mutation::EditsConfigFile
            | Mutation::AddsDependency
            | Mutation::RollbackAvailable
    );

    write_safety_disclosure(write_effect_posture, has_side_effect, has_recovery_path)
}

// ---- component structs --------------------------------------------------

/// A generator-preview sheet naming its generator identity / version, parameters, created-versus-
/// modified paths, managed-versus-user-owned files, dependency / config impact, and rollback or
/// regenerate posture, with a derived write-effect posture, a recovery path, and bounded review /
/// inspect / recovery actions — so it never implies a safe or no-op write when it changes files,
/// dependencies, or config.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorPreviewSheet {
    /// Frozen component this control implements; must be `generator_preview_sheet`.
    pub component: M5FrameworkComponentFamily,
    /// Stable generator id.
    pub generator_id: String,
    /// Generator name label; required and non-empty.
    pub generator_name_label: String,
    /// Generator version label; always required so the generator version stays explicit.
    pub generator_version_label: String,
    /// Parameters label; always required so the generator parameters stay explicit.
    pub parameters_label: String,
    /// Generator impact class, reused from the frozen matrix.
    pub generator_impact_class: M5GeneratorImpactClass,
    /// Generator apply posture, reused from the frozen matrix.
    pub generator_apply_posture: M5GeneratorApplyPosture,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Derived write-effect posture (must equal the resolved posture).
    pub write_effect_posture: WriteEffectPosture,
    /// Whether the sheet claims a genuine no-op write (must equal derived truth).
    pub claims_no_op_write: bool,
    /// Whether the sheet has a reversible recovery path (must equal derived truth).
    pub has_recovery_path: bool,
    /// What the sheet does to files.
    pub file_effect_class: FileEffectClass,
    /// Count of created paths.
    pub created_path_count: u32,
    /// Count of modified paths.
    pub modified_path_count: u32,
    /// Who owns the files this sheet touches.
    pub file_ownership_class: FileOwnershipClass,
    /// Ownership label; always required so managed-versus-user ownership stays explicit.
    pub ownership_label: String,
    /// Dependency / config impact label; required when the sheet has a side effect.
    pub dependency_config_impact_label: String,
    /// Kind of recovery this sheet keeps explicit.
    pub recovery_kind: RecoveryPath,
    /// Opaque recovery reference; required when the recovery kind is reversible.
    pub recovery_ref: String,
    /// Review-required note; required when the posture is a review-required write.
    pub review_required_note: String,
    /// Applied-recovery note; required when the posture is a reversible applied write.
    pub applied_recovery_note: String,
    /// No-recovery note; required when the sheet writes but has no reversible recovery path.
    pub no_recovery_note: String,
    /// Write-effect note; always required so the sheet states its posture and impact at sheet level.
    pub write_effect_note: String,
    /// Context note; always required so the sheet names what to review before applying it.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub sheet_actions: Vec<GeneratorSheetAction>,
    /// Certainty dispositions this sheet binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this sheet can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this sheet can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this sheet.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this sheet keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this sheet offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this sheet's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this sheet.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never implies a no-op write without explicit review. MUST be `false`.
    pub implies_no_op_write_without_review: bool,
    /// Hard invariant: never hides its dependency / config impact. MUST be `false`.
    pub hides_dependency_or_config_impact: bool,
    /// Hard invariant: never omits its rollback / regenerate recovery path. MUST be `false`.
    pub omits_rollback_or_regenerate_path: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl GeneratorPreviewSheet {
    /// Write-safety disclosures this sheet must carry, derived from the frozen axes.
    pub fn posture_disclosure(&self) -> WriteSafetyDisclosure {
        resolve_generator_preview_posture(self.generator_impact_class, self.generator_apply_posture)
    }

    /// Whether the sheet offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<GeneratorSheetAction> = self.sheet_actions.iter().copied().collect();
        GeneratorSheetAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the sheet declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        declares_mandatory_labels(&self.required_labels)
    }
}

/// A run-config scaffold card naming its target kind, environment / profile, launch command, required
/// toolchain, and local / container / SSH / managed execution boundary, with a derived write-effect
/// posture and a recovery path — so a user can see where code will run and which toolchain is required
/// before a convenience action dispatches execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunConfigScaffoldCard {
    /// Frozen component this control implements; must be `run_config_scaffold_card`.
    pub component: M5FrameworkComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Config name label; required and non-empty.
    pub config_name_label: String,
    /// The kind of target this card runs.
    pub run_target_kind: RunTargetKind,
    /// Target label; always required so the target kind stays explicit.
    pub target_label: String,
    /// The environment / profile this card launches under.
    pub launch_profile_class: LaunchProfileClass,
    /// Environment / profile label; always required so the profile stays explicit.
    pub environment_profile_label: String,
    /// Launch command label; always required so the launch command stays explicit before dispatch.
    pub launch_command_label: String,
    /// Required toolchain label; always required so the required toolchain stays explicit before
    /// dispatch.
    pub required_toolchain_label: String,
    /// Whether the required toolchain is ready.
    pub toolchain_readiness: ToolchainReadiness,
    /// Execution boundary class, reused from the frozen matrix.
    pub execution_boundary_class: M5ExecutionBoundaryClass,
    /// Whether the target runs as a local process (must equal `execution_boundary_class ==
    /// local_process`).
    pub is_local_execution: bool,
    /// Run-config mutation class, reused from the frozen matrix.
    pub run_config_mutation_class: M5RunConfigMutationClass,
    /// Certainty disposition, reused from the frozen matrix.
    pub certainty: M5FrameworkCertaintyDisposition,
    /// Derived write-effect posture (must equal the resolved posture).
    pub write_effect_posture: WriteEffectPosture,
    /// Whether the card claims a genuine no-op write (must equal derived truth).
    pub claims_no_op_write: bool,
    /// Whether the card has a reversible recovery path (must equal derived truth).
    pub has_recovery_path: bool,
    /// Kind of recovery this card keeps explicit.
    pub recovery_kind: RecoveryPath,
    /// Opaque recovery reference; required when the recovery kind is reversible.
    pub recovery_ref: String,
    /// Review-required note; required when the posture is a review-required write.
    pub review_required_note: String,
    /// Applied-recovery note; required when the posture is a reversible applied write.
    pub applied_recovery_note: String,
    /// No-recovery note; required when the card writes but has no reversible recovery path.
    pub no_recovery_note: String,
    /// Write-effect note; always required so the card states its posture and impact at card level.
    pub write_effect_note: String,
    /// Context note; always required so the card names what to check before dispatching it.
    pub context_note: String,
    /// Keyboard-complete default actions (must include the mandatory actions).
    pub card_actions: Vec<RunConfigCardAction>,
    /// Certainty dispositions this card binds (required, from the one shared vocabulary).
    pub dispositions: Vec<M5FrameworkCertaintyDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5FrameworkRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5FrameworkSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5FrameworkDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5FrameworkAccessibilityRoute>,
    /// Framework subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never implies a no-op write without explicit review. MUST be `false`.
    pub implies_no_op_write_without_review: bool,
    /// Hard invariant: never hides its execution boundary or required toolchain. MUST be `false`.
    pub hides_execution_boundary_or_toolchain: bool,
    /// Hard invariant: never omits its rollback / regenerate recovery path. MUST be `false`.
    pub omits_rollback_or_regenerate_path: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl RunConfigScaffoldCard {
    /// Write-safety disclosures this card must carry, derived from the frozen axes.
    pub fn posture_disclosure(&self) -> WriteSafetyDisclosure {
        resolve_run_config_scaffold_posture(self.run_config_mutation_class)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RunConfigCardAction> = self.card_actions.iter().copied().collect();
        RunConfigCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
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

/// First-glance generator-preview / run-config review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunConfigReview {
    /// The generator sheet names its identity and version.
    pub generator_sheet_shows_identity_and_version: bool,
    /// The generator sheet names its created and modified paths.
    pub generator_sheet_shows_created_and_modified_paths: bool,
    /// The generator sheet names its dependency / config impact and managed / user ownership.
    pub generator_sheet_shows_impact_and_ownership: bool,
    /// The generator sheet offers a rollback or regenerate recovery path.
    pub generator_sheet_offers_rollback_or_regenerate: bool,
    /// The run-config card names its target kind and launch command.
    pub run_config_card_shows_target_and_launch_command: bool,
    /// The run-config card names its required toolchain.
    pub run_config_card_shows_required_toolchain: bool,
    /// The run-config card names its local / container / SSH / managed execution boundary.
    pub run_config_card_shows_execution_boundary: bool,
    /// The write-effect posture is derived from the frozen axes, never asserted.
    pub write_posture_derived_never_asserted: bool,
    /// A no-op write is never claimed when a side effect is present.
    pub no_op_write_never_claimed_with_side_effect: bool,
    /// A file / dependency / config side effect is always disclosed before apply.
    pub side_effect_always_disclosed_before_apply: bool,
    /// The execution boundary is always visible before a convenience action dispatches execution.
    pub execution_boundary_always_visible_before_dispatch: bool,
    /// The required toolchain is always visible before a convenience action dispatches execution.
    pub required_toolchain_always_visible_before_dispatch: bool,
    /// The rollback / regenerate recovery path is reachable before apply.
    pub recovery_path_reachable_before_apply: bool,
    /// A blocked or unknown component never fakes a recovery path it does not have.
    pub blocked_component_never_fakes_recovery: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl GeneratorRunConfigReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.generator_sheet_shows_identity_and_version
            && self.generator_sheet_shows_created_and_modified_paths
            && self.generator_sheet_shows_impact_and_ownership
            && self.generator_sheet_offers_rollback_or_regenerate
            && self.run_config_card_shows_target_and_launch_command
            && self.run_config_card_shows_required_toolchain
            && self.run_config_card_shows_execution_boundary
            && self.write_posture_derived_never_asserted
            && self.no_op_write_never_claimed_with_side_effect
            && self.side_effect_always_disclosed_before_apply
            && self.execution_boundary_always_visible_before_dispatch
            && self.required_toolchain_always_visible_before_dispatch
            && self.recovery_path_reachable_before_apply
            && self.blocked_component_never_fakes_recovery
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunConfigConsumerProjection {
    /// The generator-review surface reads a single canonical source.
    pub generator_review_reads_single_source: bool,
    /// The run-config surface reads a single canonical source.
    pub run_config_reads_single_source: bool,
    /// The editor-gutter surface reads a single canonical source.
    pub editor_gutter_reads_single_source: bool,
    /// Dependency / config impact and ownership are visible before apply.
    pub impact_and_ownership_visible_before_apply: bool,
    /// The execution boundary and required toolchain are visible before dispatch.
    pub execution_boundary_and_toolchain_visible_before_dispatch: bool,
    /// The recovery path is reachable before a user trusts the component.
    pub recovery_path_reachable_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

impl GeneratorRunConfigConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.generator_review_reads_single_source
            && self.run_config_reads_single_source
            && self.editor_gutter_reads_single_source
            && self.impact_and_ownership_visible_before_apply
            && self.execution_boundary_and_toolchain_visible_before_dispatch
            && self.recovery_path_reachable_before_trust
            && self.support_export_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorRunConfigProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`GeneratorPreviewRunConfigControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorPreviewRunConfigControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Generator-preview sheets.
    pub generator_sheets: Vec<GeneratorPreviewSheet>,
    /// Run-config scaffold cards.
    pub run_config_cards: Vec<RunConfigScaffoldCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Generator / run-config review block.
    pub generator_run_config_review: GeneratorRunConfigReview,
    /// Consumer projection block.
    pub consumer_projection: GeneratorRunConfigConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GeneratorRunConfigProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe generator-preview-sheet / run-config-scaffold-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorPreviewRunConfigControlsPacket {
    /// Record kind; must equal [`GENERATOR_RUN_CONFIG_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Generator-preview sheets.
    pub generator_sheets: Vec<GeneratorPreviewSheet>,
    /// Run-config scaffold cards.
    pub run_config_cards: Vec<RunConfigScaffoldCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5FrameworkDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5FrameworkConsumerSurface>,
    /// Generator / run-config review block.
    pub generator_run_config_review: GeneratorRunConfigReview,
    /// Consumer projection block.
    pub consumer_projection: GeneratorRunConfigConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: GeneratorRunConfigProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GeneratorPreviewRunConfigControlsPacket {
    /// Builds a generator-preview-sheet / run-config-scaffold-card controls packet from stable-lane
    /// input.
    pub fn new(input: GeneratorPreviewRunConfigControlsPacketInput) -> Self {
        Self {
            record_kind: GENERATOR_RUN_CONFIG_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generator_sheets: input.generator_sheets,
            run_config_cards: input.run_config_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            generator_run_config_review: input.generator_run_config_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the generator-preview-sheet / run-config-scaffold-card control invariants.
    pub fn validate(&self) -> Vec<GeneratorRunConfigControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != GENERATOR_RUN_CONFIG_CONTROLS_RECORD_KIND {
            violations.push(GeneratorRunConfigControlsViolation::WrongRecordKind);
        }
        if self.schema_version != GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_VERSION {
            violations.push(GeneratorRunConfigControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GeneratorRunConfigControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_generator_sheets(self, &mut violations);
        validate_run_config_cards(self, &mut violations);

        if !self.generator_run_config_review.all_hold() {
            violations.push(GeneratorRunConfigControlsViolation::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(GeneratorRunConfigControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(GeneratorRunConfigControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("generator run config controls packet serializes"),
        ) {
            violations.push(GeneratorRunConfigControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("generator run config controls packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,primary_class,secondary,write_effect_posture,is_no_op,recovery_kind\n",
        );
        for sheet in &self.generator_sheets {
            let disclosure = sheet.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "generator_preview_sheet",
                csv_field(&sheet.generator_id),
                sheet.generator_impact_class.as_str(),
                sheet.generator_apply_posture.as_str(),
                disclosure.write_effect_posture.as_str(),
                disclosure.is_no_op,
                sheet.recovery_kind.as_str(),
            ));
        }
        for card in &self.run_config_cards {
            let disclosure = card.posture_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "run_config_scaffold_card",
                csv_field(&card.card_id),
                card.run_config_mutation_class.as_str(),
                card.execution_boundary_class.as_str(),
                disclosure.write_effect_posture.as_str(),
                disclosure.is_no_op,
                card.recovery_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let writing_sheets = self
            .generator_sheets
            .iter()
            .filter(|sheet| sheet.posture_disclosure().has_side_effect)
            .count();
        let writing_cards = self
            .run_config_cards
            .iter()
            .filter(|card| card.posture_disclosure().has_side_effect)
            .count();

        let mut out = String::new();
        out.push_str("# Generator-preview sheets and run-config scaffold cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Generator-preview sheets: {} ({} write files, dependencies, or config)\n",
            self.generator_sheets.len(),
            writing_sheets
        ));
        out.push_str(&format!(
            "- Run-config scaffold cards: {} ({} write config or dependencies)\n",
            self.run_config_cards.len(),
            writing_cards
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Generator-preview sheets\n\n");
        for sheet in &self.generator_sheets {
            let disclosure = sheet.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — impact `{}`, apply `{}`, write `{}`, files `{}` (+{} / ~{}), ownership `{}`, recovery `{}`\n",
                sheet.generator_name_label,
                sheet.generator_impact_class.as_str(),
                sheet.generator_apply_posture.as_str(),
                disclosure.write_effect_posture.as_str(),
                sheet.file_effect_class.as_str(),
                sheet.created_path_count,
                sheet.modified_path_count,
                sheet.file_ownership_class.as_str(),
                sheet.recovery_kind.as_str(),
            ));
        }

        out.push_str("\n## Run-config scaffold cards\n\n");
        for card in &self.run_config_cards {
            let disclosure = card.posture_disclosure();
            out.push_str(&format!(
                "- **{}** — target `{}`, profile `{}`, boundary `{}`, toolchain `{}`, mutation `{}`, write `{}`, recovery `{}`\n",
                card.config_name_label,
                card.run_target_kind.as_str(),
                card.launch_profile_class.as_str(),
                card.execution_boundary_class.as_str(),
                card.toolchain_readiness.as_str(),
                card.run_config_mutation_class.as_str(),
                disclosure.write_effect_posture.as_str(),
                card.recovery_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in generator-run-config controls export.
#[derive(Debug)]
pub enum GeneratorRunConfigControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GeneratorRunConfigControlsViolation>),
}

impl fmt::Display for GeneratorRunConfigControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "generator run config controls export parse failed: {error}"
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
                    "generator run config controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GeneratorRunConfigControlsArtifactError {}

/// Validation failures emitted by [`GeneratorPreviewRunConfigControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratorRunConfigControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No generator-preview sheets are present.
    GeneratorSheetsMissing,
    /// A generator-preview sheet is incomplete.
    GeneratorSheetIncomplete,
    /// A generator-preview sheet carries the wrong frozen component class.
    GeneratorSheetWrongComponentClass,
    /// A generator-preview sheet misrepresents its derived write-effect posture or claims.
    GeneratorPostureMisrepresented,
    /// No run-config scaffold cards are present.
    RunConfigCardsMissing,
    /// A run-config scaffold card is incomplete.
    RunConfigCardIncomplete,
    /// A run-config scaffold card carries the wrong frozen component class.
    RunConfigCardWrongComponentClass,
    /// A run-config scaffold card misrepresents its derived write-effect posture or claims.
    RunConfigPostureMisrepresented,
    /// A component with a side effect claims a no-op write.
    WriteClaimsNoOp,
    /// A review-required write does not carry its review-required note.
    ReviewNoteMissing,
    /// A reversible applied write does not carry its applied-recovery note.
    AppliedRecoveryNoteMissing,
    /// A write with no reversible recovery path does not name why it has no recovery.
    NoRecoveryNoteMissing,
    /// A component claims a reversible recovery path but its recovery kind is not reversible.
    RecoveryClaimedWithoutPath,
    /// A component with a reversible recovery path names a non-reversible recovery kind.
    RecoveryUnresolved,
    /// A component names a reversible recovery kind but not its reference.
    RecoveryRefMissing,
    /// A generator-preview sheet with a side effect does not name its dependency / config impact.
    ImpactLabelMissing,
    /// A generator-preview sheet does not name its managed / user ownership.
    OwnershipLabelMissing,
    /// A generator-preview sheet's file-effect class disagrees with its created / modified counts.
    FileEffectCountMismatch,
    /// A run-config scaffold card does not name its launch command.
    LaunchCommandMissing,
    /// A run-config scaffold card does not name its required toolchain.
    RequiredToolchainMissing,
    /// A run-config scaffold card does not name its target.
    TargetLabelMissing,
    /// A run-config scaffold card does not name its environment / profile.
    EnvironmentProfileMissing,
    /// A run-config scaffold card's local-execution flag disagrees with its execution boundary.
    ExecutionBoundaryMisrepresented,
    /// A component does not name its write-effect posture and impact at row level.
    WriteEffectNoteMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// The generator sheets do not cover every generator impact class.
    GeneratorImpactCoverageMissing,
    /// The generator sheets do not cover every generator apply posture.
    GeneratorApplyCoverageMissing,
    /// The generator sheets do not cover every file-effect class.
    FileEffectCoverageMissing,
    /// The generator sheets do not cover every file-ownership class.
    FileOwnershipCoverageMissing,
    /// The run-config cards do not cover every run-config mutation class.
    RunConfigMutationCoverageMissing,
    /// The run-config cards do not cover every execution boundary class.
    ExecutionBoundaryCoverageMissing,
    /// The run-config cards do not cover every run-target kind.
    RunTargetCoverageMissing,
    /// The run-config cards do not cover every launch-profile class.
    LaunchProfileCoverageMissing,
    /// The run-config cards do not cover every toolchain-readiness state.
    ToolchainReadinessCoverageMissing,
    /// The components do not cover every write-effect posture.
    WriteEffectPostureCoverageMissing,
    /// The components do not cover every recovery path.
    RecoveryPathCoverageMissing,
    /// A generator-preview sheet omits a mandatory action.
    GeneratorSheetActionsIncomplete,
    /// A run-config scaffold card omits a mandatory action.
    RunConfigCardActionsIncomplete,
    /// A component does not bind any certainty disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component implies a no-op write without explicit review.
    ImpliesNoOpWriteWithoutReview,
    /// A generator-preview sheet hides its dependency / config impact.
    DependencyOrConfigImpactHidden,
    /// A run-config scaffold card hides its execution boundary or required toolchain.
    ExecutionBoundaryOrToolchainHidden,
    /// A component omits its rollback / regenerate recovery path.
    RecoveryPathOmitted,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Generator / run-config review does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl GeneratorRunConfigControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::GeneratorSheetsMissing => "generator_sheets_missing",
            Self::GeneratorSheetIncomplete => "generator_sheet_incomplete",
            Self::GeneratorSheetWrongComponentClass => "generator_sheet_wrong_component_class",
            Self::GeneratorPostureMisrepresented => "generator_posture_misrepresented",
            Self::RunConfigCardsMissing => "run_config_cards_missing",
            Self::RunConfigCardIncomplete => "run_config_card_incomplete",
            Self::RunConfigCardWrongComponentClass => "run_config_card_wrong_component_class",
            Self::RunConfigPostureMisrepresented => "run_config_posture_misrepresented",
            Self::WriteClaimsNoOp => "write_claims_no_op",
            Self::ReviewNoteMissing => "review_note_missing",
            Self::AppliedRecoveryNoteMissing => "applied_recovery_note_missing",
            Self::NoRecoveryNoteMissing => "no_recovery_note_missing",
            Self::RecoveryClaimedWithoutPath => "recovery_claimed_without_path",
            Self::RecoveryUnresolved => "recovery_unresolved",
            Self::RecoveryRefMissing => "recovery_ref_missing",
            Self::ImpactLabelMissing => "impact_label_missing",
            Self::OwnershipLabelMissing => "ownership_label_missing",
            Self::FileEffectCountMismatch => "file_effect_count_mismatch",
            Self::LaunchCommandMissing => "launch_command_missing",
            Self::RequiredToolchainMissing => "required_toolchain_missing",
            Self::TargetLabelMissing => "target_label_missing",
            Self::EnvironmentProfileMissing => "environment_profile_missing",
            Self::ExecutionBoundaryMisrepresented => "execution_boundary_misrepresented",
            Self::WriteEffectNoteMissing => "write_effect_note_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::GeneratorImpactCoverageMissing => "generator_impact_coverage_missing",
            Self::GeneratorApplyCoverageMissing => "generator_apply_coverage_missing",
            Self::FileEffectCoverageMissing => "file_effect_coverage_missing",
            Self::FileOwnershipCoverageMissing => "file_ownership_coverage_missing",
            Self::RunConfigMutationCoverageMissing => "run_config_mutation_coverage_missing",
            Self::ExecutionBoundaryCoverageMissing => "execution_boundary_coverage_missing",
            Self::RunTargetCoverageMissing => "run_target_coverage_missing",
            Self::LaunchProfileCoverageMissing => "launch_profile_coverage_missing",
            Self::ToolchainReadinessCoverageMissing => "toolchain_readiness_coverage_missing",
            Self::WriteEffectPostureCoverageMissing => "write_effect_posture_coverage_missing",
            Self::RecoveryPathCoverageMissing => "recovery_path_coverage_missing",
            Self::GeneratorSheetActionsIncomplete => "generator_sheet_actions_incomplete",
            Self::RunConfigCardActionsIncomplete => "run_config_card_actions_incomplete",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ImpliesNoOpWriteWithoutReview => "implies_no_op_write_without_review",
            Self::DependencyOrConfigImpactHidden => "dependency_or_config_impact_hidden",
            Self::ExecutionBoundaryOrToolchainHidden => "execution_boundary_or_toolchain_hidden",
            Self::RecoveryPathOmitted => "recovery_path_omitted",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable generator-run-config controls export.
///
/// This is the first real consumer of the generator-preview-sheet / run-config-scaffold-card
/// component lane: a generator-review, run-config, editor-gutter, or support-export surface calls it
/// to ingest the canonical components rather than cloning sheet or card text.
///
/// # Errors
///
/// Returns [`GeneratorRunConfigControlsArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_generator_run_config_controls_export(
) -> Result<GeneratorPreviewRunConfigControlsPacket, GeneratorRunConfigControlsArtifactError> {
    let packet: GeneratorPreviewRunConfigControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-generator-preview-run-config-proof/support_export.json"
        )))
        .map_err(GeneratorRunConfigControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GeneratorRunConfigControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &GeneratorPreviewRunConfigControlsPacket,
    violations: &mut Vec<GeneratorRunConfigControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF,
        GENERATOR_RUN_CONFIG_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF,
        M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(GeneratorRunConfigControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    implies_no_op_write_without_review: bool,
    hides_secondary_axis: bool,
    omits_recovery_path: bool,
    invents_alternate_state_label: bool,
    /// The violation to emit when `hides_secondary_axis` is set — family-specific.
    hidden_secondary_violation: GeneratorRunConfigControlsViolation,
}

/// Validates the write-effect / no-op cross-checks and the recovery truth shared by both component
/// vectors.
#[allow(clippy::too_many_arguments)]
fn validate_shared_write_safety(
    disclosure: WriteSafetyDisclosure,
    claims_no_op_write: bool,
    has_recovery_path: bool,
    review_required_note: &str,
    applied_recovery_note: &str,
    no_recovery_note: &str,
    recovery_kind: RecoveryPath,
    recovery_ref: &str,
    misrepresented_violation: GeneratorRunConfigControlsViolation,
    violations: &mut Vec<GeneratorRunConfigControlsViolation>,
) {
    if disclosure.is_no_op != claims_no_op_write
        || disclosure.has_recovery_path != has_recovery_path
    {
        violations.push(misrepresented_violation);
    }
    // A component that changes files, dependencies, or config can never claim a no-op write.
    if disclosure.has_side_effect && claims_no_op_write {
        violations.push(GeneratorRunConfigControlsViolation::WriteClaimsNoOp);
    }
    if disclosure.needs_review_note && review_required_note.trim().is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::ReviewNoteMissing);
    }
    if disclosure.needs_applied_recovery_note && applied_recovery_note.trim().is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::AppliedRecoveryNoteMissing);
    }
    if disclosure.needs_no_recovery_note && no_recovery_note.trim().is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::NoRecoveryNoteMissing);
    }
    // Recovery truth: a component with a reversible recovery path must name a reversible recovery
    // kind and its reference; a component without one must not claim a reversible kind.
    if has_recovery_path && !recovery_kind.is_reversible() {
        violations.push(GeneratorRunConfigControlsViolation::RecoveryUnresolved);
    }
    if !has_recovery_path && recovery_kind.is_reversible() {
        violations.push(GeneratorRunConfigControlsViolation::RecoveryClaimedWithoutPath);
    }
    if recovery_kind.is_reversible() && recovery_ref.trim().is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::RecoveryRefMissing);
    }
}

/// Validates the axes shared by both component vectors.
#[allow(clippy::too_many_arguments)]
fn validate_common_control(
    dispositions: &[M5FrameworkCertaintyDisposition],
    downgrade_triggers: &[M5FrameworkDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5FrameworkAccessibilityRoute],
    context_note: &str,
    write_effect_note: &str,
    invariants: ControlInvariants,
    violations: &mut Vec<GeneratorRunConfigControlsViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::ContextNoteMissing);
    }
    if write_effect_note.trim().is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::WriteEffectNoteMissing);
    }
    if dispositions.is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(GeneratorRunConfigControlsViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(GeneratorRunConfigControlsViolation::AccessibilityRouteMissing);
    }
    if invariants.implies_no_op_write_without_review {
        violations.push(GeneratorRunConfigControlsViolation::ImpliesNoOpWriteWithoutReview);
    }
    if invariants.hides_secondary_axis {
        violations.push(invariants.hidden_secondary_violation);
    }
    if invariants.omits_recovery_path {
        violations.push(GeneratorRunConfigControlsViolation::RecoveryPathOmitted);
    }
    if invariants.invents_alternate_state_label {
        violations.push(GeneratorRunConfigControlsViolation::AlternateStateLabelInvented);
    }
}

fn validate_generator_sheets(
    packet: &GeneratorPreviewRunConfigControlsPacket,
    violations: &mut Vec<GeneratorRunConfigControlsViolation>,
) {
    if packet.generator_sheets.is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::GeneratorSheetsMissing);
        return;
    }

    let mut impacts: BTreeSet<M5GeneratorImpactClass> = BTreeSet::new();
    let mut applies: BTreeSet<M5GeneratorApplyPosture> = BTreeSet::new();
    let mut effects: BTreeSet<FileEffectClass> = BTreeSet::new();
    let mut ownerships: BTreeSet<FileOwnershipClass> = BTreeSet::new();

    for sheet in &packet.generator_sheets {
        let disclosure = sheet.posture_disclosure();
        impacts.insert(sheet.generator_impact_class);
        applies.insert(sheet.generator_apply_posture);
        effects.insert(sheet.file_effect_class);
        ownerships.insert(sheet.file_ownership_class);

        if sheet.generator_id.trim().is_empty()
            || sheet.generator_name_label.trim().is_empty()
            || sheet.generator_version_label.trim().is_empty()
            || sheet.parameters_label.trim().is_empty()
            || sheet.fields_shown.is_empty()
            || sheet.surface_families.is_empty()
            || sheet.deployment_lines.is_empty()
            || sheet.consumer_surfaces.is_empty()
            || sheet.source_contract_refs.is_empty()
        {
            violations.push(GeneratorRunConfigControlsViolation::GeneratorSheetIncomplete);
        }
        if sheet.component != M5FrameworkComponentFamily::GeneratorPreviewSheet {
            violations.push(GeneratorRunConfigControlsViolation::GeneratorSheetWrongComponentClass);
        }
        if sheet.write_effect_posture != disclosure.write_effect_posture {
            violations.push(GeneratorRunConfigControlsViolation::GeneratorPostureMisrepresented);
        }
        validate_shared_write_safety(
            disclosure,
            sheet.claims_no_op_write,
            sheet.has_recovery_path,
            &sheet.review_required_note,
            &sheet.applied_recovery_note,
            &sheet.no_recovery_note,
            sheet.recovery_kind,
            &sheet.recovery_ref,
            GeneratorRunConfigControlsViolation::GeneratorPostureMisrepresented,
            violations,
        );
        if disclosure.has_side_effect && sheet.dependency_config_impact_label.trim().is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::ImpactLabelMissing);
        }
        if sheet.ownership_label.trim().is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::OwnershipLabelMissing);
        }
        if !file_effect_counts_agree(
            sheet.file_effect_class,
            sheet.created_path_count,
            sheet.modified_path_count,
        ) {
            violations.push(GeneratorRunConfigControlsViolation::FileEffectCountMismatch);
        }
        if !sheet.declares_mandatory_actions() {
            violations.push(GeneratorRunConfigControlsViolation::GeneratorSheetActionsIncomplete);
        }
        validate_common_control(
            &sheet.dispositions,
            &sheet.downgrade_triggers,
            sheet.declares_mandatory_labels(),
            &sheet.accessibility_routes,
            &sheet.context_note,
            &sheet.write_effect_note,
            ControlInvariants {
                implies_no_op_write_without_review: sheet.implies_no_op_write_without_review,
                hides_secondary_axis: sheet.hides_dependency_or_config_impact,
                omits_recovery_path: sheet.omits_rollback_or_regenerate_path,
                invents_alternate_state_label: sheet.invents_alternate_state_label,
                hidden_secondary_violation:
                    GeneratorRunConfigControlsViolation::DependencyOrConfigImpactHidden,
            },
            violations,
        );
    }

    for required in M5GeneratorImpactClass::ALL {
        if !impacts.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::GeneratorImpactCoverageMissing);
            break;
        }
    }
    for required in M5GeneratorApplyPosture::ALL {
        if !applies.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::GeneratorApplyCoverageMissing);
            break;
        }
    }
    for required in FileEffectClass::ALL {
        if !effects.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::FileEffectCoverageMissing);
            break;
        }
    }
    for required in FileOwnershipClass::ALL {
        if !ownerships.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::FileOwnershipCoverageMissing);
            break;
        }
    }

    validate_shared_coverage(packet, violations);
}

fn validate_run_config_cards(
    packet: &GeneratorPreviewRunConfigControlsPacket,
    violations: &mut Vec<GeneratorRunConfigControlsViolation>,
) {
    if packet.run_config_cards.is_empty() {
        violations.push(GeneratorRunConfigControlsViolation::RunConfigCardsMissing);
        return;
    }

    let mut mutations: BTreeSet<M5RunConfigMutationClass> = BTreeSet::new();
    let mut boundaries: BTreeSet<M5ExecutionBoundaryClass> = BTreeSet::new();
    let mut targets: BTreeSet<RunTargetKind> = BTreeSet::new();
    let mut profiles: BTreeSet<LaunchProfileClass> = BTreeSet::new();
    let mut toolchains: BTreeSet<ToolchainReadiness> = BTreeSet::new();

    for card in &packet.run_config_cards {
        let disclosure = card.posture_disclosure();
        mutations.insert(card.run_config_mutation_class);
        boundaries.insert(card.execution_boundary_class);
        targets.insert(card.run_target_kind);
        profiles.insert(card.launch_profile_class);
        toolchains.insert(card.toolchain_readiness);

        if card.card_id.trim().is_empty()
            || card.config_name_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(GeneratorRunConfigControlsViolation::RunConfigCardIncomplete);
        }
        if card.component != M5FrameworkComponentFamily::RunConfigScaffoldCard {
            violations.push(GeneratorRunConfigControlsViolation::RunConfigCardWrongComponentClass);
        }
        if card.write_effect_posture != disclosure.write_effect_posture {
            violations.push(GeneratorRunConfigControlsViolation::RunConfigPostureMisrepresented);
        }
        validate_shared_write_safety(
            disclosure,
            card.claims_no_op_write,
            card.has_recovery_path,
            &card.review_required_note,
            &card.applied_recovery_note,
            &card.no_recovery_note,
            card.recovery_kind,
            &card.recovery_ref,
            GeneratorRunConfigControlsViolation::RunConfigPostureMisrepresented,
            violations,
        );
        if card.target_label.trim().is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::TargetLabelMissing);
        }
        if card.environment_profile_label.trim().is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::EnvironmentProfileMissing);
        }
        if card.launch_command_label.trim().is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::LaunchCommandMissing);
        }
        if card.required_toolchain_label.trim().is_empty() {
            violations.push(GeneratorRunConfigControlsViolation::RequiredToolchainMissing);
        }
        if card.is_local_execution
            != matches!(
                card.execution_boundary_class,
                M5ExecutionBoundaryClass::LocalProcess
            )
        {
            violations.push(GeneratorRunConfigControlsViolation::ExecutionBoundaryMisrepresented);
        }
        if !card.declares_mandatory_actions() {
            violations.push(GeneratorRunConfigControlsViolation::RunConfigCardActionsIncomplete);
        }
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            &card.context_note,
            &card.write_effect_note,
            ControlInvariants {
                implies_no_op_write_without_review: card.implies_no_op_write_without_review,
                hides_secondary_axis: card.hides_execution_boundary_or_toolchain,
                omits_recovery_path: card.omits_rollback_or_regenerate_path,
                invents_alternate_state_label: card.invents_alternate_state_label,
                hidden_secondary_violation:
                    GeneratorRunConfigControlsViolation::ExecutionBoundaryOrToolchainHidden,
            },
            violations,
        );
    }

    for required in M5RunConfigMutationClass::ALL {
        if !mutations.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::RunConfigMutationCoverageMissing);
            break;
        }
    }
    for required in M5ExecutionBoundaryClass::ALL {
        if !boundaries.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::ExecutionBoundaryCoverageMissing);
            break;
        }
    }
    for required in RunTargetKind::ALL {
        if !targets.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::RunTargetCoverageMissing);
            break;
        }
    }
    for required in LaunchProfileClass::ALL {
        if !profiles.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::LaunchProfileCoverageMissing);
            break;
        }
    }
    for required in ToolchainReadiness::ALL {
        if !toolchains.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::ToolchainReadinessCoverageMissing);
            break;
        }
    }
}

/// Validates that the union of both component vectors covers every write-effect posture and recovery
/// path the acceptance criteria pin.
fn validate_shared_coverage(
    packet: &GeneratorPreviewRunConfigControlsPacket,
    violations: &mut Vec<GeneratorRunConfigControlsViolation>,
) {
    let mut postures: BTreeSet<WriteEffectPosture> = BTreeSet::new();
    let mut recoveries: BTreeSet<RecoveryPath> = BTreeSet::new();

    for sheet in &packet.generator_sheets {
        postures.insert(sheet.posture_disclosure().write_effect_posture);
        recoveries.insert(sheet.recovery_kind);
    }
    for card in &packet.run_config_cards {
        postures.insert(card.posture_disclosure().write_effect_posture);
        recoveries.insert(card.recovery_kind);
    }

    for required in WriteEffectPosture::ALL {
        if !postures.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::WriteEffectPostureCoverageMissing);
            break;
        }
    }
    for required in RecoveryPath::ALL {
        if !recoveries.contains(&required) {
            violations.push(GeneratorRunConfigControlsViolation::RecoveryPathCoverageMissing);
            break;
        }
    }
}

/// Whether a sheet's file-effect class agrees with its created / modified path counts.
fn file_effect_counts_agree(effect: FileEffectClass, created: u32, modified: u32) -> bool {
    (effect.creates() == (created > 0)) && (effect.modifies() == (modified > 0))
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
// These builders are the single producer of the checked-in support export and the scenario fixtures.
// The headless emitter example and the inline tests both call them so the in-code components, the
// artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical generator-run-config controls packet.
pub const GENERATOR_RUN_CONFIG_CONTROLS_PACKET_ID: &str =
    "m5-generator-preview-run-config-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn generator_source_refs() -> Vec<String> {
    strings(&[
        M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn run_config_source_refs() -> Vec<String> {
    strings(&[
        M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
    ])
}

fn generator_sheet_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ImpactUndisclosed,
        M5FrameworkDowngradeTrigger::RollbackPathOmitted,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn run_config_card_downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExecutionBoundaryUnstated,
        M5FrameworkDowngradeTrigger::ImpactUndisclosed,
        M5FrameworkDowngradeTrigger::RollbackPathOmitted,
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

/// Builds a generator-preview sheet, deriving the write-effect posture, no-op claim, recovery path,
/// and required notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn generator_sheet(
    generator_id: &str,
    generator_name_label: &str,
    generator_version_label: &str,
    parameters_label: &str,
    generator_impact_class: M5GeneratorImpactClass,
    generator_apply_posture: M5GeneratorApplyPosture,
    certainty: M5FrameworkCertaintyDisposition,
    file_effect_class: FileEffectClass,
    created_path_count: u32,
    modified_path_count: u32,
    file_ownership_class: FileOwnershipClass,
    ownership_label: &str,
    dependency_config_impact_label: &str,
    recovery_kind: RecoveryPath,
    recovery_ref: &str,
    context_note: &str,
    sheet_actions: Vec<GeneratorSheetAction>,
) -> GeneratorPreviewSheet {
    let disclosure =
        resolve_generator_preview_posture(generator_impact_class, generator_apply_posture);
    GeneratorPreviewSheet {
        component: M5FrameworkComponentFamily::GeneratorPreviewSheet,
        generator_id: generator_id.to_owned(),
        generator_name_label: generator_name_label.to_owned(),
        generator_version_label: generator_version_label.to_owned(),
        parameters_label: parameters_label.to_owned(),
        generator_impact_class,
        generator_apply_posture,
        certainty,
        write_effect_posture: disclosure.write_effect_posture,
        claims_no_op_write: disclosure.is_no_op,
        has_recovery_path: disclosure.has_recovery_path,
        file_effect_class,
        created_path_count,
        modified_path_count,
        file_ownership_class,
        ownership_label: ownership_label.to_owned(),
        dependency_config_impact_label: dependency_config_impact_label.to_owned(),
        recovery_kind,
        recovery_ref: recovery_ref.to_owned(),
        review_required_note: note_if(
            disclosure.needs_review_note,
            "Generator writes files, dependencies, or config; review the diff before it applies",
        ),
        applied_recovery_note: note_if(
            disclosure.needs_applied_recovery_note,
            "Generator has been applied; rollback or regenerate is available to undo it",
        ),
        no_recovery_note: note_if(
            disclosure.needs_no_recovery_note,
            "Generator write is blocked with no automatic undo; recovery is forward-fix only",
        ),
        write_effect_note: format!(
            "Write {}; impact {}",
            disclosure.write_effect_posture.as_str(),
            generator_impact_class.as_str()
        ),
        context_note: context_note.to_owned(),
        sheet_actions,
        dispositions: vec![certainty],
        downgrade_triggers: generator_sheet_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::ProvingSourceAndRecoveryBoundary),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "generator_name_label",
            "generator_version_label",
            "parameters_label",
            "file_effect_class",
            "created_path_count",
            "modified_path_count",
            "file_ownership_class",
            "dependency_config_impact_label",
            "write_effect_posture",
            "recovery_kind",
        ]),
        source_contract_refs: generator_source_refs(),
        implies_no_op_write_without_review: false,
        hides_dependency_or_config_impact: false,
        omits_rollback_or_regenerate_path: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a run-config scaffold card, deriving the write-effect posture, no-op claim, recovery path,
/// local-execution flag, and required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn run_config_card(
    card_id: &str,
    config_name_label: &str,
    run_target_kind: RunTargetKind,
    target_label: &str,
    launch_profile_class: LaunchProfileClass,
    environment_profile_label: &str,
    launch_command_label: &str,
    required_toolchain_label: &str,
    toolchain_readiness: ToolchainReadiness,
    execution_boundary_class: M5ExecutionBoundaryClass,
    run_config_mutation_class: M5RunConfigMutationClass,
    certainty: M5FrameworkCertaintyDisposition,
    recovery_kind: RecoveryPath,
    recovery_ref: &str,
    context_note: &str,
    card_actions: Vec<RunConfigCardAction>,
) -> RunConfigScaffoldCard {
    let disclosure = resolve_run_config_scaffold_posture(run_config_mutation_class);
    RunConfigScaffoldCard {
        component: M5FrameworkComponentFamily::RunConfigScaffoldCard,
        card_id: card_id.to_owned(),
        config_name_label: config_name_label.to_owned(),
        run_target_kind,
        target_label: target_label.to_owned(),
        launch_profile_class,
        environment_profile_label: environment_profile_label.to_owned(),
        launch_command_label: launch_command_label.to_owned(),
        required_toolchain_label: required_toolchain_label.to_owned(),
        toolchain_readiness,
        execution_boundary_class,
        is_local_execution: matches!(
            execution_boundary_class,
            M5ExecutionBoundaryClass::LocalProcess
        ),
        run_config_mutation_class,
        certainty,
        write_effect_posture: disclosure.write_effect_posture,
        claims_no_op_write: disclosure.is_no_op,
        has_recovery_path: disclosure.has_recovery_path,
        recovery_kind,
        recovery_ref: recovery_ref.to_owned(),
        review_required_note: note_if(
            disclosure.needs_review_note,
            "Scaffold creates or edits config or adds a dependency; review it before it applies",
        ),
        applied_recovery_note: note_if(
            disclosure.needs_applied_recovery_note,
            "Scaffold has been applied; rollback is available to undo it",
        ),
        no_recovery_note: note_if(
            disclosure.needs_no_recovery_note,
            "Scaffold write is blocked with no automatic undo; recovery is forward-fix only",
        ),
        write_effect_note: format!(
            "Write {}; mutation {}",
            disclosure.write_effect_posture.as_str(),
            run_config_mutation_class.as_str()
        ),
        context_note: context_note.to_owned(),
        card_actions,
        dispositions: vec![certainty],
        downgrade_triggers: run_config_card_downgrade_triggers(),
        required_labels: label_set(M5FrameworkRequiredLabel::ExecutionBoundaryAndImpact),
        surface_families: M5FrameworkSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5FrameworkDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5FrameworkAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "config_name_label",
            "run_target_kind",
            "target_label",
            "environment_profile_label",
            "launch_command_label",
            "required_toolchain_label",
            "execution_boundary_class",
            "write_effect_posture",
            "recovery_kind",
        ]),
        source_contract_refs: run_config_source_refs(),
        implies_no_op_write_without_review: false,
        hides_execution_boundary_or_toolchain: false,
        omits_rollback_or_regenerate_path: false,
        invents_alternate_state_label: false,
    }
}

fn generator_sheets() -> Vec<GeneratorPreviewSheet> {
    use FileEffectClass as Effect;
    use FileOwnershipClass as Own;
    use GeneratorSheetAction as Action;
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5GeneratorApplyPosture as Apply;
    use M5GeneratorImpactClass as Impact;
    use RecoveryPath as Recovery;

    vec![
        // 1. Config change + blocked → unknown / blocked write with a side effect: honest
        //    forward-fix-only recovery so a blocked write never reads as safe.
        generator_sheet(
            "gen-blocked-config",
            "Add CI config (blocked)",
            "v2.4.0",
            "params: provider=github, cache=true",
            Impact::ConfigChange,
            Apply::Blocked,
            Certainty::FrameworkPack,
            Effect::ModifiesFile,
            0,
            1,
            Own::UserOwned,
            "Edits user-owned .ci/config.yml",
            "Changes CI config; blocked pending a resolved conflict",
            Recovery::ForwardFixOnly,
            "",
            "Blocked config write; there is no automatic undo, so recovery is forward-fix only",
            vec![
                Action::ReviewCreatedAndModifiedDiff,
                Action::InspectImpactAndOwnership,
                Action::OpenRollbackOrRegenerate,
                Action::CopyGeneratorId,
            ],
        ),
        // 2. Dependency change + preview ready → review-required write, rollback and regenerate.
        generator_sheet(
            "gen-add-orm",
            "Add ORM integration",
            "v3.1.2",
            "params: driver=postgres, migrations=true",
            Impact::DependencyChange,
            Apply::PreviewReady,
            Certainty::FrameworkPack,
            Effect::CreatesFile,
            2,
            0,
            Own::ManagedGenerated,
            "Creates managed db/ scaffolding",
            "Adds two dependencies and creates managed db scaffolding",
            Recovery::RollbackAndRegenerate,
            "diff:generated/db",
            "Review the created files and the two added dependencies before applying",
            vec![
                Action::ReviewCreatedAndModifiedDiff,
                Action::InspectImpactAndOwnership,
                Action::OpenRollbackOrRegenerate,
                Action::ApplyAfterReview,
            ],
        ),
        // 3. File write + review required → review-required write, rollback.
        generator_sheet(
            "gen-new-route",
            "Generate new route module",
            "v3.1.2",
            "params: name=orders, methods=GET,POST",
            Impact::FileWrite,
            Apply::ReviewRequired,
            Certainty::CoreNative,
            Effect::CreatesAndModifies,
            1,
            1,
            Own::MixedOwnership,
            "Creates route file, edits the route registry",
            "Creates one route file and edits the shared route registry",
            Recovery::Rollback,
            "diff:app/routes/orders",
            "Review the new route file and the registry edit before applying",
            vec![
                Action::ReviewCreatedAndModifiedDiff,
                Action::InspectImpactAndOwnership,
                Action::OpenRollbackOrRegenerate,
                Action::ApplyAfterReview,
            ],
        ),
        // 4. Script or task change + rollback available → reversible applied, regenerate.
        generator_sheet(
            "gen-add-task",
            "Add build task",
            "v2.4.0",
            "params: task=lint, stage=ci",
            Impact::ScriptOrTaskChange,
            Apply::RollbackAvailable,
            Certainty::FrameworkPack,
            Effect::ModifiesFile,
            0,
            1,
            Own::ManagedGenerated,
            "Edits the managed task manifest",
            "Adds a lint task to the managed task manifest",
            Recovery::Regenerate,
            "diff:tasks/manifest",
            "Task has been applied; regenerate is available to undo the change",
            vec![
                Action::ReviewCreatedAndModifiedDiff,
                Action::InspectImpactAndOwnership,
                Action::OpenRollbackOrRegenerate,
                Action::CopyGeneratorId,
            ],
        ),
        // 5. No change + regenerate available → no-op preview.
        generator_sheet(
            "gen-preview-only",
            "Preview component scaffold",
            "v3.1.2",
            "params: name=Widget, story=true",
            Impact::NoChange,
            Apply::RegenerateAvailable,
            Certainty::CoreNative,
            Effect::NoFileChange,
            0,
            0,
            Own::ManagedGenerated,
            "No files are written by this preview",
            "",
            Recovery::NoRecoveryNeeded,
            "",
            "Preview only; nothing is written until you choose to generate",
            vec![
                Action::ReviewCreatedAndModifiedDiff,
                Action::InspectImpactAndOwnership,
                Action::OpenRollbackOrRegenerate,
                Action::ApplyAfterReview,
            ],
        ),
        // 6. Unknown impact + apply ready → unknown / blocked, no side effect.
        generator_sheet(
            "gen-unknown",
            "Third-party codemod (unknown impact)",
            "unknown",
            "params: unavailable",
            Impact::UnknownImpact,
            Apply::ApplyReady,
            Certainty::Partial,
            Effect::NoFileChange,
            0,
            0,
            Own::UnknownOwnership,
            "Ownership of touched files is unknown",
            "",
            Recovery::NoRecoveryNeeded,
            "",
            "Impact is unknown; inspect the codemod before trusting or applying it",
            vec![
                Action::ReviewCreatedAndModifiedDiff,
                Action::InspectImpactAndOwnership,
                Action::OpenRollbackOrRegenerate,
            ],
        ),
    ]
}

fn run_config_cards() -> Vec<RunConfigScaffoldCard> {
    use LaunchProfileClass as Profile;
    use M5ExecutionBoundaryClass as Boundary;
    use M5FrameworkCertaintyDisposition as Certainty;
    use M5RunConfigMutationClass as Mutation;
    use RecoveryPath as Recovery;
    use RunConfigCardAction as Action;
    use RunTargetKind as Target;
    use ToolchainReadiness as Toolchain;

    vec![
        // 1. Creates config + local process → review-required write, rollback, local.
        run_config_card(
            "run-web-dev",
            "Run web app (dev)",
            Target::WebApp,
            "Local dev server",
            Profile::Development,
            "profile: development",
            "npm run dev",
            "Node.js 20.x",
            Toolchain::ToolchainReady,
            Boundary::LocalProcess,
            Mutation::CreatesConfigFile,
            Certainty::CoreNative,
            Recovery::Rollback,
            "config:.run/web-dev.json",
            "Creates a local run config; the app runs as a local process",
            vec![
                Action::InspectExecutionBoundary,
                Action::InspectRequiredToolchain,
                Action::ReviewConfigMutation,
                Action::RunAfterReview,
            ],
        ),
        // 2. Edits config + container → review-required write, rollback and regenerate, container.
        run_config_card(
            "run-api-container",
            "Run API (container)",
            Target::ApiServer,
            "Containerized API",
            Profile::Debug,
            "profile: debug",
            "docker compose up api",
            "Docker 24.x + Compose v2",
            Toolchain::ToolchainReady,
            Boundary::Container,
            Mutation::EditsConfigFile,
            Certainty::FrameworkPack,
            Recovery::RollbackAndRegenerate,
            "config:compose.debug.yml",
            "Edits the compose config; the API runs inside a container, not on the host",
            vec![
                Action::InspectExecutionBoundary,
                Action::InspectRequiredToolchain,
                Action::ReviewConfigMutation,
                Action::RunAfterReview,
            ],
        ),
        // 3. Adds dependency + SSH remote → review-required write, rollback, remote.
        run_config_card(
            "run-worker-ssh",
            "Run worker (SSH remote)",
            Target::BackgroundJob,
            "Remote worker over SSH",
            Profile::Production,
            "profile: production",
            "ssh deploy@host 'systemctl start worker'",
            "Remote Python 3.12 (version mismatch)",
            Toolchain::ToolchainVersionMismatch,
            Boundary::SshRemote,
            Mutation::AddsDependency,
            Certainty::Bridge,
            Recovery::Rollback,
            "config:.run/worker-ssh.json",
            "Adds a dependency and dispatches to an SSH remote host, not your machine",
            vec![
                Action::InspectExecutionBoundary,
                Action::InspectRequiredToolchain,
                Action::ReviewConfigMutation,
                Action::RunAfterReview,
            ],
        ),
        // 4. No-write preview + managed workspace → no-op preview, managed.
        run_config_card(
            "run-test-managed",
            "Run tests (managed workspace)",
            Target::TestSuite,
            "Managed workspace test run",
            Profile::Test,
            "profile: test",
            "aureline test --all",
            "Managed workspace toolchain",
            Toolchain::ToolchainReady,
            Boundary::ManagedWorkspace,
            Mutation::NoWritePreview,
            Certainty::CoreNative,
            Recovery::NoRecoveryNeeded,
            "",
            "Preview only; the test run dispatches inside the managed workspace, nothing is written",
            vec![
                Action::InspectExecutionBoundary,
                Action::InspectRequiredToolchain,
                Action::ReviewConfigMutation,
                Action::RunAfterReview,
            ],
        ),
        // 5. Rollback available + cloud remote → reversible applied, rollback, cloud.
        run_config_card(
            "run-cli-cloud",
            "Run CLI (cloud remote)",
            Target::CliTool,
            "Cloud remote CLI",
            Profile::CustomProfile,
            "profile: custom",
            "aureline cloud run cli",
            "Cloud runtime (toolchain missing locally)",
            Toolchain::ToolchainMissing,
            Boundary::CloudRemote,
            Mutation::RollbackAvailable,
            Certainty::RuntimeConfirmed,
            Recovery::Rollback,
            "config:.run/cli-cloud.json",
            "A prior config write is applied; rollback is available. Runs on a cloud remote",
            vec![
                Action::InspectExecutionBoundary,
                Action::InspectRequiredToolchain,
                Action::ReviewConfigMutation,
                Action::CopyLaunchCommand,
            ],
        ),
        // 6. Unknown mutation + unknown boundary → unknown / blocked, no recovery.
        run_config_card(
            "run-unknown",
            "Run imported config (unknown)",
            Target::UnknownTarget,
            "Unknown target",
            Profile::CustomProfile,
            "profile: unknown",
            "unknown command",
            "Unknown toolchain",
            Toolchain::ToolchainUnknown,
            Boundary::UnknownBoundary,
            Mutation::UnknownMutation,
            Certainty::Partial,
            Recovery::ForwardFixOnly,
            "",
            "Imported config of unknown mutation and boundary; inspect it before dispatching",
            vec![
                Action::InspectExecutionBoundary,
                Action::InspectRequiredToolchain,
                Action::ReviewConfigMutation,
            ],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5FrameworkDowngradeTrigger> {
    vec![
        M5FrameworkDowngradeTrigger::ExecutionBoundaryUnstated,
        M5FrameworkDowngradeTrigger::ImpactUndisclosed,
        M5FrameworkDowngradeTrigger::ProvingSourceOmitted,
        M5FrameworkDowngradeTrigger::RollbackPathOmitted,
        M5FrameworkDowngradeTrigger::AlternateStateLabelInvented,
        M5FrameworkDowngradeTrigger::ProofStale,
    ]
}

fn generator_run_config_review() -> GeneratorRunConfigReview {
    GeneratorRunConfigReview {
        generator_sheet_shows_identity_and_version: true,
        generator_sheet_shows_created_and_modified_paths: true,
        generator_sheet_shows_impact_and_ownership: true,
        generator_sheet_offers_rollback_or_regenerate: true,
        run_config_card_shows_target_and_launch_command: true,
        run_config_card_shows_required_toolchain: true,
        run_config_card_shows_execution_boundary: true,
        write_posture_derived_never_asserted: true,
        no_op_write_never_claimed_with_side_effect: true,
        side_effect_always_disclosed_before_apply: true,
        execution_boundary_always_visible_before_dispatch: true,
        required_toolchain_always_visible_before_dispatch: true,
        recovery_path_reachable_before_apply: true,
        blocked_component_never_fakes_recovery: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> GeneratorRunConfigConsumerProjection {
    GeneratorRunConfigConsumerProjection {
        generator_review_reads_single_source: true,
        run_config_reads_single_source: true,
        editor_gutter_reads_single_source: true,
        impact_and_ownership_visible_before_apply: true,
        execution_boundary_and_toolchain_visible_before_dispatch: true,
        recovery_path_reachable_before_trust: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> GeneratorRunConfigProofFreshness {
    GeneratorRunConfigProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF,
        GENERATOR_RUN_CONFIG_CONTROLS_DOC_REF,
        M5_FRAMEWORK_COMPONENT_SCHEMA_REF,
        M5_FRAMEWORK_COMPONENT_DOC_REF,
        M5_GENERATOR_PREVIEW_SHEET_SCHEMA_REF,
        M5_RUN_CONFIG_SCAFFOLD_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical generator-preview-sheet / run-config-scaffold-card controls packet.
pub fn seeded_generator_run_config_controls() -> GeneratorPreviewRunConfigControlsPacket {
    GeneratorPreviewRunConfigControlsPacket::new(GeneratorPreviewRunConfigControlsPacketInput {
        packet_id: GENERATOR_RUN_CONFIG_CONTROLS_PACKET_ID.to_owned(),
        surface_label:
            "M5 generator-preview sheets and run-config scaffold cards: generator identity / version, parameters, created-versus-modified paths, managed-versus-user-owned files, dependency / config impact, rollback / regenerate posture, target kind, environment / profile, launch command, required toolchain, and local / container / SSH / managed execution-boundary truth across claimed framework actions"
                .to_owned(),
        generator_sheets: generator_sheets(),
        run_config_cards: run_config_cards(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5FrameworkConsumerSurface::ALL.to_vec(),
        generator_run_config_review: generator_run_config_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a writing generator that changes dependencies and config and must
/// never imply a safe or no-op write without explicit review. Every generator impact class, apply
/// posture, file-effect class, and file-ownership class stays covered so the fixture validates on its
/// own.
pub fn seeded_generator_run_config_controls_writing_generator(
) -> GeneratorPreviewRunConfigControlsPacket {
    let mut packet = seeded_generator_run_config_controls();
    packet.packet_id =
        "m5-generator-preview-run-config-controls:fixture:writing-generator".to_owned();
    packet.surface_label =
        "M5 generator-preview sheets: a generator that changes files, dependencies, or config never reads as a safe or no-op write"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a run-config scaffold card dispatching to a remote boundary that must
/// show where code runs and which toolchain is required before it dispatches execution. Every
/// run-config mutation class, execution boundary class, run-target kind, launch-profile class, and
/// toolchain-readiness state stays covered so the fixture validates on its own.
pub fn seeded_generator_run_config_controls_remote_run_config(
) -> GeneratorPreviewRunConfigControlsPacket {
    let mut packet = seeded_generator_run_config_controls();
    packet.packet_id =
        "m5-generator-preview-run-config-controls:fixture:remote-run-config".to_owned();
    packet.surface_label =
        "M5 run-config scaffold cards: a convenience action shows where code will run and which toolchain is required before it dispatches"
            .to_owned();
    packet
}
