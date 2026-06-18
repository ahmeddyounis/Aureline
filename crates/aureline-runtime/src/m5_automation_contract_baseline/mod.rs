//! Frozen automation builder, parameter-review, dry-run/explain, run-history,
//! macro-recorder, and safety-label contract baseline.
//!
//! The recorded-macro / declarative-recipe manifest and the run-record contracts
//! ([`/schemas/automation/recipe_manifest.schema.json`] and
//! [`/schemas/automation/run_record.schema.json`]) already freeze *what a stored
//! recipe or macro is* and *what evidence a dispatch mints*. This module closes
//! the remaining UI/runtime gap the UX and design sources make explicit: the
//! object model for **authoring, previewing, rerunning, and exporting** automation
//! across M5 surfaces. It freezes six object families into one inspected baseline
//! so later palette, docs/help, export, and support work inherits a single
//! builder/preview/history/macro/safety contract instead of inventing
//! feature-local runners:
//!
//! - [`AutomationObjectFamily::RecipeBuilder`] — the live builder authoring state
//!   ([`RecipeBuilderSession`]) that emits *declarative* manifests only.
//! - [`AutomationObjectFamily::ParameterReview`] — the pre-apply parameter-review
//!   sheet ([`ParameterReviewSheet`]) that resolves every argument's provenance.
//! - [`AutomationObjectFamily::DryRunExplain`] — the dry-run/explain preview
//!   ([`DryRunExplainPacket`]) that explains each step before any apply.
//! - [`AutomationObjectFamily::RunHistory`] — the existing run-history row
//!   contract every dispatch projects through, bound in by reference.
//! - [`AutomationObjectFamily::MacroRecorder`] — the macro-recorder session
//!   ([`MacroSession`]) strictly constrained to UI or editor state.
//! - [`AutomationObjectFamily::SafetyLabels`] — the single reused
//!   [`AutomationSafetyLabel`] vocabulary (`Macro-safe`, `Recipe-safe`,
//!   `Headless-safe`, `UI-only`, `Approval required`, `Writes files`,
//!   `Runs process`, `Network call`, `Remote mutation`) projected from the
//!   already-frozen controlled-automation-label axis, never minted anew.
//!
//! The [`AutomationContractBaselinePacket`] is the canonical M5 source: it binds
//! each family to its boundary schema, its state vocabulary, its evidence hooks,
//! and the consumer surfaces that read it, then [`AutomationContractBaselinePacket::validate`]
//! enforces the freeze mechanically (every family present, the whole safety-label
//! vocabulary present and correctly categorized, and every invariant true). A
//! missing family, an incomplete or miscategorized safety-label set, a dropped
//! reused-contract ref, or a violated invariant *blocks stable*.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/recipe-builder-and-macro-contract.md`]; the machine-readable
//! boundaries live at [`/schemas/automation/recipe-builder.schema.json`],
//! [`/schemas/automation/macro-session.schema.json`], and
//! [`/schemas/automation/automation-contract-baseline.schema.json`].
//!
//! [`/docs/m5/recipe-builder-and-macro-contract.md`]: ../../../docs/m5/recipe-builder-and-macro-contract.md
//! [`/schemas/automation/recipe-builder.schema.json`]: ../../../schemas/automation/recipe-builder.schema.json
//! [`/schemas/automation/macro-session.schema.json`]: ../../../schemas/automation/macro-session.schema.json
//! [`/schemas/automation/automation-contract-baseline.schema.json`]: ../../../schemas/automation/automation-contract-baseline.schema.json
//! [`/schemas/automation/recipe_manifest.schema.json`]: ../../../schemas/automation/recipe_manifest.schema.json
//! [`/schemas/automation/run_record.schema.json`]: ../../../schemas/automation/run_record.schema.json

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`AutomationContractBaselinePacket`].
pub const AUTOMATION_CONTRACT_BASELINE_RECORD_KIND: &str = "m5_automation_contract_baseline_packet";

/// Stable record-kind tag for [`AutomationContractBaselineSupportExport`].
pub const AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_automation_contract_baseline_support_export";

/// Stable record-kind tag for [`AutomationContractBaselineCliHeadlessView`].
pub const AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_automation_contract_baseline_cli_headless";

/// Stable record-kind tag for the safety-label manifest projection.
pub const AUTOMATION_SAFETY_LABEL_MANIFEST_RECORD_KIND: &str =
    "automation_safety_label_manifest_record";

/// Integer schema version for the contract-baseline packet family.
pub const AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the contract-baseline boundary schema.
pub const AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF: &str =
    "schemas/automation/automation-contract-baseline.schema.json";

/// Repo-relative path of the recipe-builder / parameter-review / dry-run-explain
/// boundary schema.
pub const RECIPE_BUILDER_SCHEMA_REF: &str = "schemas/automation/recipe-builder.schema.json";

/// Repo-relative path of the macro-recorder session boundary schema.
pub const MACRO_SESSION_SCHEMA_REF: &str = "schemas/automation/macro-session.schema.json";

/// Repo-relative path of the existing run-history row boundary schema this lane
/// binds in by reference rather than re-inventing.
pub const RUN_HISTORY_ROW_SCHEMA_REF: &str = "schemas/automation/run_history_row.schema.json";

/// Repo-relative path of the existing recipe-manifest boundary schema the builder
/// and macro recorder emit against.
pub const RECIPE_MANIFEST_SCHEMA_REF: &str = "schemas/automation/recipe_manifest.schema.json";

/// Repo-relative path of the existing run-record boundary schema every dispatch
/// mints against.
pub const RUN_RECORD_SCHEMA_REF: &str = "schemas/automation/run_record.schema.json";

/// Repo-relative path of the existing safe-summary export boundary schema.
pub const RUN_SUMMARY_EXPORT_SCHEMA_REF: &str = "schemas/automation/run_summary_export.schema.json";

/// Repo-relative path of the existing controlled-automation-label axis the
/// safety-label vocabulary projects from instead of minting parallel labels.
pub const CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF: &str =
    "schemas/automation/automation-manifest.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const AUTOMATION_CONTRACT_BASELINE_DOC_REF: &str =
    "docs/m5/recipe-builder-and-macro-contract.md";

/// Repo-relative path of the checked-in packet artifact.
pub const AUTOMATION_CONTRACT_BASELINE_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/automation-contract-baseline/packet.json";

/// Stable packet id minted by the seed.
pub const AUTOMATION_CONTRACT_BASELINE_ID: &str = "automation:m5:contract-baseline:v1";

/// Stable support-export id minted by the seed inspector.
pub const AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:contract-baseline";

/// Stable CLI/headless view id minted by the seed inspector.
pub const AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:contract-baseline";

/// Stable safety-label manifest id minted by the seed inspector.
pub const AUTOMATION_SAFETY_LABEL_MANIFEST_ID: &str = "automation:m5:safety-labels:v1";

/// Repo-relative root every checked-in worked-example fixture lives under.
pub const AUTOMATION_CONTRACT_BASELINE_FIXTURE_ROOT: &str = "fixtures/automation/m5";

// ---------------------------------------------------------------------------
// Safety-label reuse vocabulary
// ---------------------------------------------------------------------------

/// One controlled automation safety label.
///
/// The closed set mirrors the `controlled_automation_label` vocabulary frozen in
/// [`/schemas/automation/automation-manifest.schema.json`]; this enum is the
/// reuse surface every M5 builder, preview, history, CLI, AI, and support
/// projection reads. Minting a parallel label is non-conforming.
///
/// [`/schemas/automation/automation-manifest.schema.json`]: ../../../schemas/automation/automation-manifest.schema.json
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationSafetyLabelId {
    /// The command can be captured and replayed against explicit UI or editor
    /// state only.
    MacroSafe,
    /// The command can be cited as a typed, gated step in a declarative recipe.
    RecipeSafe,
    /// The command can be dispatched from a CLI / headless surface without an
    /// interactive UI.
    HeadlessSafe,
    /// The command is interactive only and is not admissible to a recipe, macro,
    /// or headless surface.
    UiOnly,
    /// The command requires an approval ticket before any apply.
    ApprovalRequired,
    /// The command writes files in the workspace or on the device.
    WritesFiles,
    /// The command launches or controls a process.
    RunsProcess,
    /// The command performs a network call.
    NetworkCall,
    /// The command mutates remote state.
    RemoteMutation,
}

impl AutomationSafetyLabelId {
    /// Every safety label in canonical (declaration) order.
    pub const ALL: [AutomationSafetyLabelId; 9] = [
        AutomationSafetyLabelId::MacroSafe,
        AutomationSafetyLabelId::RecipeSafe,
        AutomationSafetyLabelId::HeadlessSafe,
        AutomationSafetyLabelId::UiOnly,
        AutomationSafetyLabelId::ApprovalRequired,
        AutomationSafetyLabelId::WritesFiles,
        AutomationSafetyLabelId::RunsProcess,
        AutomationSafetyLabelId::NetworkCall,
        AutomationSafetyLabelId::RemoteMutation,
    ];

    /// Stable snake_case token, identical to the controlled-automation-label
    /// vocabulary value this label reuses.
    pub fn as_str(self) -> &'static str {
        match self {
            AutomationSafetyLabelId::MacroSafe => "macro_safe",
            AutomationSafetyLabelId::RecipeSafe => "recipe_safe",
            AutomationSafetyLabelId::HeadlessSafe => "headless_safe",
            AutomationSafetyLabelId::UiOnly => "ui_only",
            AutomationSafetyLabelId::ApprovalRequired => "approval_required",
            AutomationSafetyLabelId::WritesFiles => "writes_files",
            AutomationSafetyLabelId::RunsProcess => "runs_process",
            AutomationSafetyLabelId::NetworkCall => "network_call",
            AutomationSafetyLabelId::RemoteMutation => "remote_mutation",
        }
    }

    /// Reviewable display token used by palette chips, CLI `--describe`, and docs.
    pub fn display_token(self) -> &'static str {
        match self {
            AutomationSafetyLabelId::MacroSafe => "Macro-safe",
            AutomationSafetyLabelId::RecipeSafe => "Recipe-safe",
            AutomationSafetyLabelId::HeadlessSafe => "Headless-safe",
            AutomationSafetyLabelId::UiOnly => "UI-only",
            AutomationSafetyLabelId::ApprovalRequired => "Approval required",
            AutomationSafetyLabelId::WritesFiles => "Writes files",
            AutomationSafetyLabelId::RunsProcess => "Runs process",
            AutomationSafetyLabelId::NetworkCall => "Network call",
            AutomationSafetyLabelId::RemoteMutation => "Remote mutation",
        }
    }

    /// Whether the label is an admissibility cue (where the command may run) or an
    /// effect disclosure (what the command does).
    pub fn kind(self) -> SafetyLabelKind {
        match self {
            AutomationSafetyLabelId::MacroSafe
            | AutomationSafetyLabelId::RecipeSafe
            | AutomationSafetyLabelId::HeadlessSafe
            | AutomationSafetyLabelId::UiOnly
            | AutomationSafetyLabelId::ApprovalRequired => SafetyLabelKind::AdmissibilityCue,
            AutomationSafetyLabelId::WritesFiles
            | AutomationSafetyLabelId::RunsProcess
            | AutomationSafetyLabelId::NetworkCall
            | AutomationSafetyLabelId::RemoteMutation => SafetyLabelKind::EffectDisclosure,
        }
    }

    /// Short reviewable meaning sentence.
    pub fn meaning(self) -> &'static str {
        match self {
            AutomationSafetyLabelId::MacroSafe => {
                "Captured and replayed locally against explicit UI or editor state only."
            }
            AutomationSafetyLabelId::RecipeSafe => {
                "Admissible as a typed, gated step in a declarative recipe."
            }
            AutomationSafetyLabelId::HeadlessSafe => {
                "Dispatchable from a CLI or headless surface without an interactive UI."
            }
            AutomationSafetyLabelId::UiOnly => {
                "Interactive only; not admissible to a recipe, macro, or headless surface."
            }
            AutomationSafetyLabelId::ApprovalRequired => {
                "Requires an approval ticket before any apply."
            }
            AutomationSafetyLabelId::WritesFiles => {
                "Writes files in the workspace or on the device."
            }
            AutomationSafetyLabelId::RunsProcess => "Launches or controls a process.",
            AutomationSafetyLabelId::NetworkCall => "Performs a network call.",
            AutomationSafetyLabelId::RemoteMutation => "Mutates remote state.",
        }
    }
}

/// Whether a safety label states where automation may run or what effect it has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafetyLabelKind {
    /// States where the command may run (macro / recipe / headless / ui-only /
    /// approval).
    AdmissibilityCue,
    /// Discloses a material effect (writes files / runs process / network call /
    /// remote mutation).
    EffectDisclosure,
}

impl SafetyLabelKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            SafetyLabelKind::AdmissibilityCue => "admissibility_cue",
            SafetyLabelKind::EffectDisclosure => "effect_disclosure",
        }
    }
}

/// One row of the reused safety-label vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationSafetyLabel {
    /// The label this row describes.
    pub label_id: AutomationSafetyLabelId,
    /// Reviewable display token rendered to users.
    pub display_token: String,
    /// Whether the label is an admissibility cue or an effect disclosure.
    pub label_kind: SafetyLabelKind,
    /// The frozen vocabulary axis this label projects from (never minted anew).
    pub source_axis_ref: String,
    /// Short reviewable meaning sentence.
    pub meaning: String,
}

impl AutomationSafetyLabel {
    /// Materializes the canonical row for one label id.
    pub fn canonical(label_id: AutomationSafetyLabelId) -> Self {
        AutomationSafetyLabel {
            label_id,
            display_token: label_id.display_token().to_owned(),
            label_kind: label_id.kind(),
            source_axis_ref: format!(
                "{CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF}#/$defs/controlled_automation_label"
            ),
            meaning: label_id.meaning().to_owned(),
        }
    }
}

/// The canonical, ordered safety-label vocabulary.
pub fn canonical_safety_labels() -> Vec<AutomationSafetyLabel> {
    AutomationSafetyLabelId::ALL
        .into_iter()
        .map(AutomationSafetyLabel::canonical)
        .collect()
}

// ---------------------------------------------------------------------------
// Object families
// ---------------------------------------------------------------------------

/// One frozen automation object family bound into the baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationObjectFamily {
    /// The live recipe-builder authoring session.
    RecipeBuilder,
    /// The pre-apply parameter-review sheet.
    ParameterReview,
    /// The dry-run / explain preview packet.
    DryRunExplain,
    /// The user-facing run-history row contract.
    RunHistory,
    /// The macro-recorder session.
    MacroRecorder,
    /// The reused safety-label vocabulary.
    SafetyLabels,
}

impl AutomationObjectFamily {
    /// Every family in canonical (declaration) order.
    pub const ALL: [AutomationObjectFamily; 6] = [
        AutomationObjectFamily::RecipeBuilder,
        AutomationObjectFamily::ParameterReview,
        AutomationObjectFamily::DryRunExplain,
        AutomationObjectFamily::RunHistory,
        AutomationObjectFamily::MacroRecorder,
        AutomationObjectFamily::SafetyLabels,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            AutomationObjectFamily::RecipeBuilder => "recipe_builder",
            AutomationObjectFamily::ParameterReview => "parameter_review",
            AutomationObjectFamily::DryRunExplain => "dry_run_explain",
            AutomationObjectFamily::RunHistory => "run_history",
            AutomationObjectFamily::MacroRecorder => "macro_recorder",
            AutomationObjectFamily::SafetyLabels => "safety_labels",
        }
    }
}

/// One family binding: which schema, doc anchor, state vocabulary, evidence
/// hooks, and consumer surfaces a family freezes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectFamilyBinding {
    /// The family this binding describes.
    pub family: AutomationObjectFamily,
    /// Reviewable title.
    pub title: String,
    /// Repo-relative boundary schema ref the family records publish against.
    pub schema_ref: String,
    /// Anchor into the reviewer contract doc for this family.
    pub doc_anchor: String,
    /// Closed state vocabulary the family's records draw from.
    pub state_vocabulary: Vec<String>,
    /// Evidence hooks every record of this family resolves through.
    pub evidence_hook_refs: Vec<String>,
    /// Consumer surfaces that read this family's records.
    pub consumer_surfaces: Vec<String>,
    /// Existing contracts this family reuses instead of re-deciding.
    pub reused_from_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Worked-example object model (the frozen Rust types per family)
// ---------------------------------------------------------------------------

/// Frozen recipe-builder authoring state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeBuilderStateClass {
    /// The recipe is being authored and has not yet validated.
    Draft,
    /// Validation found a blocking problem.
    ValidationFailed,
    /// The recipe validated and a dry-run / explain preview is available.
    PreviewReady,
    /// The recipe is preview-ready but cannot apply without an approval ticket.
    ApprovalRequired,
    /// The recipe is blocked from applying by trust, policy, or a denied label.
    Blocked,
}

impl RecipeBuilderStateClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RecipeBuilderStateClass::Draft => "draft",
            RecipeBuilderStateClass::ValidationFailed => "validation_failed",
            RecipeBuilderStateClass::PreviewReady => "preview_ready",
            RecipeBuilderStateClass::ApprovalRequired => "approval_required",
            RecipeBuilderStateClass::Blocked => "blocked",
        }
    }

    /// Every state in canonical order.
    pub const ALL: [RecipeBuilderStateClass; 5] = [
        RecipeBuilderStateClass::Draft,
        RecipeBuilderStateClass::ValidationFailed,
        RecipeBuilderStateClass::PreviewReady,
        RecipeBuilderStateClass::ApprovalRequired,
        RecipeBuilderStateClass::Blocked,
    ];
}

/// Re-exported argument-inspection kind from the shareability contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArgumentInspectionKind {
    /// A typed argument slot resolved against the descriptor.
    TypedArgumentSlotRef,
    /// A value pinned by admin policy.
    PolicyPinnedArgumentRef,
    /// A credential resolved through an opaque broker handle.
    CredentialHandleArgumentRef,
    /// A value backed by the current selection.
    SelectionBackedArgumentRef,
    /// A value backed by the focused context.
    FocusedContextBackedArgumentRef,
    /// A value defaulted from the descriptor.
    DefaultFromDescriptorArgumentRef,
    /// A value proposed by the AI assistant.
    AiProposedArgumentRef,
    /// A value supplied by the recipe.
    AutomationRecipeSuppliedArgumentRef,
    /// A value supplied by an extension.
    ExtensionSuppliedArgumentRef,
}

impl ArgumentInspectionKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ArgumentInspectionKind::TypedArgumentSlotRef => "typed_argument_slot_ref",
            ArgumentInspectionKind::PolicyPinnedArgumentRef => "policy_pinned_argument_ref",
            ArgumentInspectionKind::CredentialHandleArgumentRef => "credential_handle_argument_ref",
            ArgumentInspectionKind::SelectionBackedArgumentRef => "selection_backed_argument_ref",
            ArgumentInspectionKind::FocusedContextBackedArgumentRef => {
                "focused_context_backed_argument_ref"
            }
            ArgumentInspectionKind::DefaultFromDescriptorArgumentRef => {
                "default_from_descriptor_argument_ref"
            }
            ArgumentInspectionKind::AiProposedArgumentRef => "ai_proposed_argument_ref",
            ArgumentInspectionKind::AutomationRecipeSuppliedArgumentRef => {
                "automation_recipe_supplied_argument_ref"
            }
            ArgumentInspectionKind::ExtensionSuppliedArgumentRef => {
                "extension_supplied_argument_ref"
            }
        }
    }
}

/// One authored step draft in a recipe builder session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderStepDraft {
    /// Opaque step id.
    pub step_id: String,
    /// Re-exported recipe step kind.
    pub step_kind: String,
    /// Opaque command id the step cites.
    pub command_id: String,
    /// Opaque command revision ref the step cites.
    pub command_revision_ref: String,
    /// Dotted canonical verb re-exported from the descriptor.
    pub canonical_verb: String,
    /// Capability declarations this step quotes.
    pub capability_declarations: Vec<String>,
    /// Safety labels projected onto this step.
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
}

/// Frozen recipe-builder authoring session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderSession {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub recipe_builder_schema_version: u32,
    /// Opaque builder session id.
    pub builder_id: String,
    /// Opaque draft recipe revision ref.
    pub draft_recipe_revision_ref: String,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// Authoring state of the builder.
    pub builder_state_class: RecipeBuilderStateClass,
    /// Re-exported declarative authoring language class.
    pub authoring_language_class: String,
    /// Authored step drafts.
    pub step_drafts: Vec<RecipeBuilderStepDraft>,
    /// Safety labels projected onto the whole recipe (the union over steps).
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Validation findings raised against the draft.
    pub validation_findings: Vec<BuilderValidationFinding>,
    /// Opaque ref to the parameter-review sheet bound to this session.
    pub parameter_review_sheet_ref: String,
    /// Opaque ref to the dry-run / explain packet bound to this session.
    pub dry_run_explain_packet_ref: String,
    /// Schema the builder emits a declarative manifest against on save.
    pub manifest_target_schema_ref: String,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

/// One validation finding raised against a builder draft.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuilderValidationFinding {
    /// Stable finding-kind token.
    pub finding_kind: String,
    /// Whether the finding blocks apply or only warns.
    pub severity: String,
    /// Reviewable summary sentence.
    pub summary: String,
}

/// Verdict for one reviewed parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterReviewVerdictClass {
    /// The parameter is resolved and ready to apply.
    Resolved,
    /// The parameter still needs user input before apply.
    NeedsInput,
    /// The parameter is pinned by admin policy and cannot be edited.
    PolicyPinned,
    /// The parameter is sensitive and held behind a broker handle for review.
    SensitiveHeldForReview,
    /// The parameter is blocked from resolving.
    Blocked,
}

impl ParameterReviewVerdictClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ParameterReviewVerdictClass::Resolved => "resolved",
            ParameterReviewVerdictClass::NeedsInput => "needs_input",
            ParameterReviewVerdictClass::PolicyPinned => "policy_pinned",
            ParameterReviewVerdictClass::SensitiveHeldForReview => "sensitive_held_for_review",
            ParameterReviewVerdictClass::Blocked => "blocked",
        }
    }

    /// Every verdict in canonical order.
    pub const ALL: [ParameterReviewVerdictClass; 5] = [
        ParameterReviewVerdictClass::Resolved,
        ParameterReviewVerdictClass::NeedsInput,
        ParameterReviewVerdictClass::PolicyPinned,
        ParameterReviewVerdictClass::SensitiveHeldForReview,
        ParameterReviewVerdictClass::Blocked,
    ];
}

/// One reviewed parameter row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewRow {
    /// Snake_case parameter name.
    pub parameter_name: String,
    /// Re-exported argument-inspection kind.
    pub inspection_kind: ArgumentInspectionKind,
    /// Review verdict for this parameter.
    pub verdict_class: ParameterReviewVerdictClass,
    /// Whether the parameter is required before apply.
    pub required: bool,
    /// Re-exported redaction class governing the value's sensitivity.
    pub sensitivity_class: String,
    /// Reviewable summary sentence (never the raw value).
    pub summary: String,
}

/// Frozen pre-apply parameter-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterReviewSheet {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub recipe_builder_schema_version: u32,
    /// Opaque sheet id.
    pub sheet_id: String,
    /// Opaque builder session id this sheet belongs to.
    pub builder_id: String,
    /// Opaque draft recipe revision ref this sheet reviews.
    pub draft_recipe_revision_ref: String,
    /// Reviewed parameter rows.
    pub rows: Vec<ParameterReviewRow>,
    /// Count of required parameters still unresolved (apply-blocking when > 0).
    pub unresolved_required_count: u32,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

/// Aggregate dry-run outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DryRunOutcomeClass {
    /// The recipe would apply cleanly.
    WouldApply,
    /// The recipe would apply only after an approval ticket is granted.
    WouldApplyUnderApproval,
    /// The recipe would be denied at a trust, policy, or capability gate.
    WouldBeDeniedAtGate,
    /// No safe preview exists; apply requires a superseding approval.
    NoSafePreview,
}

impl DryRunOutcomeClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            DryRunOutcomeClass::WouldApply => "would_apply",
            DryRunOutcomeClass::WouldApplyUnderApproval => "would_apply_under_approval",
            DryRunOutcomeClass::WouldBeDeniedAtGate => "would_be_denied_at_gate",
            DryRunOutcomeClass::NoSafePreview => "no_safe_preview",
        }
    }

    /// Every outcome in canonical order.
    pub const ALL: [DryRunOutcomeClass; 4] = [
        DryRunOutcomeClass::WouldApply,
        DryRunOutcomeClass::WouldApplyUnderApproval,
        DryRunOutcomeClass::WouldBeDeniedAtGate,
        DryRunOutcomeClass::NoSafePreview,
    ];
}

/// One explained step in a dry-run / explain preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunStepExplanation {
    /// Opaque step id.
    pub step_id: String,
    /// Dotted canonical verb.
    pub canonical_verb: String,
    /// Plain-language explanation of what the step would do.
    pub explanation: String,
    /// Capability declarations this step quotes.
    pub capability_declarations: Vec<String>,
    /// Safety labels projected onto this step.
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Whether the step's effect is reversible.
    pub reversible: bool,
    /// Reviewable blast-radius summary.
    pub blast_radius_summary: String,
}

/// Frozen dry-run / explain preview packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DryRunExplainPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub recipe_builder_schema_version: u32,
    /// Opaque packet id.
    pub packet_id: String,
    /// Opaque builder session id this preview belongs to.
    pub builder_id: String,
    /// Opaque draft recipe revision ref this preview explains.
    pub draft_recipe_revision_ref: String,
    /// Aggregate dry-run outcome.
    pub dry_run_outcome_class: DryRunOutcomeClass,
    /// Safety labels projected onto the whole recipe.
    pub aggregate_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Per-step explanations.
    pub step_explanations: Vec<DryRunStepExplanation>,
    /// Re-exported preview posture class.
    pub preview_posture_class: String,
    /// Re-exported approval posture class.
    pub approval_posture_class: String,
    /// Schema every dispatch of this recipe mints a run record against.
    pub run_record_schema_ref: String,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

/// Macro-recorder lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroRecorderStateClass {
    /// The recorder is actively capturing UI or editor state.
    Recording,
    /// The recorder is paused.
    Paused,
    /// The recorder has stopped and the macro is captured.
    Stopped,
    /// The recording was discarded without minting a macro.
    Discarded,
    /// The captured macro has been promoted to a declarative recipe.
    PromotedToRecipe,
}

impl MacroRecorderStateClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroRecorderStateClass::Recording => "recording",
            MacroRecorderStateClass::Paused => "paused",
            MacroRecorderStateClass::Stopped => "stopped",
            MacroRecorderStateClass::Discarded => "discarded",
            MacroRecorderStateClass::PromotedToRecipe => "promoted_to_recipe",
        }
    }

    /// Every state in canonical order.
    pub const ALL: [MacroRecorderStateClass; 5] = [
        MacroRecorderStateClass::Recording,
        MacroRecorderStateClass::Paused,
        MacroRecorderStateClass::Stopped,
        MacroRecorderStateClass::Discarded,
        MacroRecorderStateClass::PromotedToRecipe,
    ];
}

/// Promotion affordance for a captured macro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroPromotionAffordanceClass {
    /// The macro can be re-authored as a declarative recipe.
    PromotableToDeclarativeRecipe,
    /// The macro is UI-only and not promotable.
    NotPromotableUiOnly,
    /// Promotion is blocked by admin policy.
    PromotionBlockedByPolicy,
}

impl MacroPromotionAffordanceClass {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            MacroPromotionAffordanceClass::PromotableToDeclarativeRecipe => {
                "promotable_to_declarative_recipe"
            }
            MacroPromotionAffordanceClass::NotPromotableUiOnly => "not_promotable_ui_only",
            MacroPromotionAffordanceClass::PromotionBlockedByPolicy => {
                "promotion_blocked_by_policy"
            }
        }
    }
}

/// Content-address pair for a captured state digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentAddress {
    /// Digest algorithm.
    pub digest_algorithm: String,
    /// Lowercase hex digest.
    pub digest_hex: String,
    /// Digest size in bytes.
    pub digest_size_bytes: u32,
}

/// One captured step in a macro-recorder session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroCaptureStep {
    /// Opaque step id.
    pub step_id: String,
    /// Re-exported recorded-macro surface class.
    pub surface_class: String,
    /// Content-address of the captured UI / editor state.
    pub state_digest: ContentAddress,
    /// Re-exported macro replay posture class.
    pub replay_posture_class: String,
    /// Monotonic capture timestamp.
    pub captured_at: String,
}

/// Frozen macro-recorder session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroSession {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub macro_session_schema_version: u32,
    /// Opaque session id.
    pub session_id: String,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// Recorder lifecycle state.
    pub recorder_state_class: MacroRecorderStateClass,
    /// Re-exported storage scope class (never the managed-only channel).
    pub storage_scope_class: String,
    /// Safety labels projected onto the session (always UI-only / macro-safe).
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Captured UI / editor state steps.
    pub captured_steps: Vec<MacroCaptureStep>,
    /// Re-exported redaction class.
    pub redaction_class: String,
    /// Promotion affordance for the captured macro.
    pub promotion_affordance_class: MacroPromotionAffordanceClass,
    /// Opaque ref to the resulting macro manifest, or null when discarded.
    pub resulting_macro_manifest_ref: Option<String>,
    /// Schema the recorder mints a recorded-macro manifest against on stop.
    pub manifest_target_schema_ref: String,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

// ---------------------------------------------------------------------------
// Baseline packet
// ---------------------------------------------------------------------------

/// Frozen invariants the baseline pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineInvariantsBlock {
    /// The recipe builder emits declarative manifests only.
    pub recipe_builder_emits_declarative_manifests_only: bool,
    /// The macro recorder is constrained to UI or editor state.
    pub macro_recorder_constrained_to_ui_or_editor_state: bool,
    /// A dry-run / explain preview is required before any irreversible apply.
    pub dry_run_explain_required_before_irreversible_apply: bool,
    /// Parameter review resolves every argument's provenance before apply.
    pub parameter_review_resolves_provenance_before_apply: bool,
    /// One safety-label vocabulary is reused across every surface.
    pub one_safety_label_vocabulary_reused_across_surfaces: bool,
    /// Safety labels project from existing axes; they are never minted anew.
    pub safety_labels_project_from_existing_axes_not_minted: bool,
    /// Run history reuses the canonical run record.
    pub run_history_reuses_the_canonical_run_record: bool,
    /// No hidden UI shortcut widens automation authority.
    pub no_hidden_ui_shortcut_widens_automation_authority: bool,
    /// Reruns re-resolve current context; they never replay stale authority.
    pub reruns_reresolve_current_context_never_replay_stale_authority: bool,
}

impl BaselineInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        BaselineInvariantsBlock {
            recipe_builder_emits_declarative_manifests_only: true,
            macro_recorder_constrained_to_ui_or_editor_state: true,
            dry_run_explain_required_before_irreversible_apply: true,
            parameter_review_resolves_provenance_before_apply: true,
            one_safety_label_vocabulary_reused_across_surfaces: true,
            safety_labels_project_from_existing_axes_not_minted: true,
            run_history_reuses_the_canonical_run_record: true,
            no_hidden_ui_shortcut_widens_automation_authority: true,
            reruns_reresolve_current_context_never_replay_stale_authority: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 9] {
        [
            (
                "recipe_builder_emits_declarative_manifests_only",
                self.recipe_builder_emits_declarative_manifests_only,
            ),
            (
                "macro_recorder_constrained_to_ui_or_editor_state",
                self.macro_recorder_constrained_to_ui_or_editor_state,
            ),
            (
                "dry_run_explain_required_before_irreversible_apply",
                self.dry_run_explain_required_before_irreversible_apply,
            ),
            (
                "parameter_review_resolves_provenance_before_apply",
                self.parameter_review_resolves_provenance_before_apply,
            ),
            (
                "one_safety_label_vocabulary_reused_across_surfaces",
                self.one_safety_label_vocabulary_reused_across_surfaces,
            ),
            (
                "safety_labels_project_from_existing_axes_not_minted",
                self.safety_labels_project_from_existing_axes_not_minted,
            ),
            (
                "run_history_reuses_the_canonical_run_record",
                self.run_history_reuses_the_canonical_run_record,
            ),
            (
                "no_hidden_ui_shortcut_widens_automation_authority",
                self.no_hidden_ui_shortcut_widens_automation_authority,
            ),
            (
                "reruns_reresolve_current_context_never_replay_stale_authority",
                self.reruns_reresolve_current_context_never_replay_stale_authority,
            ),
        ]
    }
}

/// Promotion state of the baseline as a whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationBaselinePromotionState {
    /// Every family present, every label present and categorized, every invariant
    /// true.
    Stable,
    /// A non-blocking warning narrows the baseline below stable.
    NarrowedBelowStable,
    /// A blocking finding blocks the baseline from stable.
    BlocksStable,
}

impl AutomationBaselinePromotionState {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            AutomationBaselinePromotionState::Stable => "stable",
            AutomationBaselinePromotionState::NarrowedBelowStable => "narrowed_below_stable",
            AutomationBaselinePromotionState::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity of a baseline validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineFindingSeverity {
    /// Blocks the baseline from stable.
    Blocker,
    /// Narrows the baseline below stable.
    Warning,
}

/// Kind of a baseline validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineFindingKind {
    /// A required object family is absent from the baseline.
    MissingObjectFamily,
    /// A family binding cites no boundary schema.
    FamilyMissingSchemaRef,
    /// A family binding cites no evidence hook.
    FamilyMissingEvidenceHook,
    /// A family binding names no consumer surface.
    FamilyMissingConsumerSurface,
    /// A family binding declares no state vocabulary.
    FamilyMissingStateVocabulary,
    /// The safety-label vocabulary is missing one or more labels.
    SafetyLabelSetIncomplete,
    /// A safety label is categorized into the wrong kind.
    SafetyLabelMiscategorized,
    /// The baseline cites no reused contract refs.
    ReusedContractRefMissing,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl BaselineFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            BaselineFindingKind::MissingObjectFamily => "missing_object_family",
            BaselineFindingKind::FamilyMissingSchemaRef => "family_missing_schema_ref",
            BaselineFindingKind::FamilyMissingEvidenceHook => "family_missing_evidence_hook",
            BaselineFindingKind::FamilyMissingConsumerSurface => "family_missing_consumer_surface",
            BaselineFindingKind::FamilyMissingStateVocabulary => "family_missing_state_vocabulary",
            BaselineFindingKind::SafetyLabelSetIncomplete => "safety_label_set_incomplete",
            BaselineFindingKind::SafetyLabelMiscategorized => "safety_label_miscategorized",
            BaselineFindingKind::ReusedContractRefMissing => "reused_contract_ref_missing",
            BaselineFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by [`AutomationContractBaselinePacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineValidationFinding {
    /// The finding kind.
    pub finding_kind: BaselineFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: BaselineFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl BaselineValidationFinding {
    fn blocker(
        finding_kind: BaselineFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        BaselineValidationFinding {
            finding_kind,
            severity: BaselineFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractBaselineInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Object-family bindings.
    pub object_families: Vec<ObjectFamilyBinding>,
    /// Reused safety-label vocabulary.
    pub safety_labels: Vec<AutomationSafetyLabel>,
    /// Existing contracts this baseline reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Worked-example fixture refs the baseline anchors.
    pub fixture_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: BaselineInvariantsBlock,
}

/// Canonical M5 automation builder/preview/history/macro/safety contract baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractBaselinePacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Boundary schema ref for this packet.
    pub baseline_schema_ref: String,
    /// Recipe-builder / parameter-review / dry-run-explain boundary schema ref.
    pub recipe_builder_schema_ref: String,
    /// Macro-session boundary schema ref.
    pub macro_session_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this baseline reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Object-family bindings.
    pub object_families: Vec<ObjectFamilyBinding>,
    /// Reused safety-label vocabulary.
    pub safety_labels: Vec<AutomationSafetyLabel>,
    /// Worked-example fixture refs the baseline anchors.
    pub fixture_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: BaselineInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<BaselineValidationFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over the frozen family and label tokens.
    pub baseline_digest: String,
}

impl AutomationContractBaselinePacket {
    /// Freezes an input into a packet, computing findings, promotion state, and
    /// the baseline digest.
    pub fn materialize(input: AutomationContractBaselineInput) -> Self {
        let findings = validate_input(&input);
        let promotion_state = promotion_state_for_findings(&findings);
        let baseline_digest = baseline_digest(&input);
        AutomationContractBaselinePacket {
            record_kind: AUTOMATION_CONTRACT_BASELINE_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            baseline_schema_ref: AUTOMATION_CONTRACT_BASELINE_SCHEMA_REF.to_owned(),
            recipe_builder_schema_ref: RECIPE_BUILDER_SCHEMA_REF.to_owned(),
            macro_session_schema_ref: MACRO_SESSION_SCHEMA_REF.to_owned(),
            doc_ref: AUTOMATION_CONTRACT_BASELINE_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            object_families: input.object_families,
            safety_labels: input.safety_labels,
            fixture_refs: input.fixture_refs,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            baseline_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<BaselineValidationFinding> {
        validate_materialized(self)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// Returns the binding for one family, if present.
    pub fn family(&self, family: AutomationObjectFamily) -> Option<&ObjectFamilyBinding> {
        self.object_families
            .iter()
            .find(|binding| binding.family == family)
    }

    /// Object-family tokens in canonical order present in the packet.
    pub fn family_tokens(&self) -> Vec<&'static str> {
        self.object_families
            .iter()
            .map(|binding| binding.family.as_str())
            .collect()
    }

    /// Safety-label tokens in the order the packet stores them.
    pub fn safety_label_tokens(&self) -> Vec<&'static str> {
        self.safety_labels
            .iter()
            .map(|label| label.label_id.as_str())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> AutomationContractBaselineSupportExport {
        AutomationContractBaselineSupportExport {
            record_kind: AUTOMATION_CONTRACT_BASELINE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            baseline_digest: self.baseline_digest.clone(),
            promotion_state: self.promotion_state,
            family_rows: self
                .object_families
                .iter()
                .map(|binding| SupportExportFamilyRow {
                    family: binding.family,
                    title: binding.title.clone(),
                    schema_ref: binding.schema_ref.clone(),
                    state_vocabulary: binding.state_vocabulary.clone(),
                })
                .collect(),
            safety_labels: self.safety_labels.clone(),
            invariants: self.invariants.clone(),
            finding_kinds: self
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind)
                .collect(),
        }
    }

    /// Builds the compact CLI / headless projection.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AutomationContractBaselineCliHeadlessView {
        AutomationContractBaselineCliHeadlessView {
            record_kind: AUTOMATION_CONTRACT_BASELINE_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            family_lines: self
                .object_families
                .iter()
                .map(|binding| {
                    format!(
                        "{} schema={} states={}",
                        binding.family.as_str(),
                        binding.schema_ref,
                        binding.state_vocabulary.len()
                    )
                })
                .collect(),
            safety_label_lines: self
                .safety_labels
                .iter()
                .map(|label| {
                    format!(
                        "{} kind={} \"{}\"",
                        label.label_id.as_str(),
                        label.label_kind.as_str(),
                        label.display_token
                    )
                })
                .collect(),
        }
    }

    /// Builds the safety-label manifest projection.
    pub fn safety_label_manifest(
        &self,
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AutomationSafetyLabelManifest {
        AutomationSafetyLabelManifest {
            record_kind: AUTOMATION_SAFETY_LABEL_MANIFEST_RECORD_KIND.to_owned(),
            recipe_builder_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            source_axis_ref: format!(
                "{CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF}#/$defs/controlled_automation_label"
            ),
            labels: self.safety_labels.clone(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} families={} labels={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.object_families.len(),
            self.safety_labels.len(),
            self.baseline_digest,
        )];
        for binding in &self.object_families {
            lines.push(format!(
                "family {} schema={} doc={} hooks={} surfaces={}",
                binding.family.as_str(),
                binding.schema_ref,
                binding.doc_anchor,
                binding.evidence_hook_refs.len(),
                binding.consumer_surfaces.len(),
            ));
        }
        for label in &self.safety_labels {
            lines.push(format!(
                "label {} kind={} \"{}\"",
                label.label_id.as_str(),
                label.label_kind.as_str(),
                label.display_token,
            ));
        }
        lines
    }
}

/// One support-export family row (redacted projection).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportFamilyRow {
    /// The family this row describes.
    pub family: AutomationObjectFamily,
    /// Reviewable title.
    pub title: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// State vocabulary.
    pub state_vocabulary: Vec<String>,
}

/// Redacted support-export projection of the baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractBaselineSupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// Packet id this export was minted from.
    pub packet_id: String,
    /// Baseline digest carried for verification.
    pub baseline_digest: String,
    /// Promotion state of the source packet.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Family rows.
    pub family_rows: Vec<SupportExportFamilyRow>,
    /// Reused safety-label vocabulary.
    pub safety_labels: Vec<AutomationSafetyLabel>,
    /// Frozen invariants block.
    pub invariants: BaselineInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<BaselineFindingKind>,
}

impl AutomationContractBaselineSupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary: it
    /// carries only opaque ids, closed vocabulary, and reviewable sentences.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.baseline_digest.is_empty()
            && !self.family_rows.is_empty()
            && !self.safety_labels.is_empty()
    }
}

/// Compact CLI / headless projection of the baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationContractBaselineCliHeadlessView {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// Monotonic generation timestamp.
    pub generated_at: String,
    /// Packet id this view was minted from.
    pub packet_id: String,
    /// Promotion state.
    pub promotion_state: AutomationBaselinePromotionState,
    /// One line per object family.
    pub family_lines: Vec<String>,
    /// One line per safety label.
    pub safety_label_lines: Vec<String>,
}

impl AutomationContractBaselineCliHeadlessView {
    /// Whether the view explains every family and label.
    pub fn every_family_explained(&self) -> bool {
        self.family_lines.len() == AutomationObjectFamily::ALL.len()
            && self.safety_label_lines.len() == AutomationSafetyLabelId::ALL.len()
    }
}

/// Safety-label manifest projection (the checked-in reuse vocabulary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationSafetyLabelManifest {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version (shares the recipe-builder schema family).
    pub recipe_builder_schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Monotonic generation timestamp.
    pub generated_at: String,
    /// The frozen vocabulary axis every label projects from.
    pub source_axis_ref: String,
    /// The reused safety-label vocabulary.
    pub labels: Vec<AutomationSafetyLabel>,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_input(input: &AutomationContractBaselineInput) -> Vec<BaselineValidationFinding> {
    validate_parts(
        &input.object_families,
        &input.safety_labels,
        &input.reused_contract_refs,
        &input.invariants,
    )
}

fn validate_materialized(
    packet: &AutomationContractBaselinePacket,
) -> Vec<BaselineValidationFinding> {
    validate_parts(
        &packet.object_families,
        &packet.safety_labels,
        &packet.reused_contract_refs,
        &packet.invariants,
    )
}

fn validate_parts(
    object_families: &[ObjectFamilyBinding],
    safety_labels: &[AutomationSafetyLabel],
    reused_contract_refs: &[String],
    invariants: &BaselineInvariantsBlock,
) -> Vec<BaselineValidationFinding> {
    let mut findings = Vec::new();

    for family in AutomationObjectFamily::ALL {
        let Some(binding) = object_families.iter().find(|row| row.family == family) else {
            findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::MissingObjectFamily,
                Some(family.as_str().to_owned()),
                format!(
                    "the {} object family is absent from the baseline",
                    family.as_str()
                ),
            ));
            continue;
        };
        if binding.schema_ref.is_empty() {
            findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::FamilyMissingSchemaRef,
                Some(family.as_str().to_owned()),
                format!("the {} family cites no boundary schema", family.as_str()),
            ));
        }
        if binding.evidence_hook_refs.is_empty() {
            findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::FamilyMissingEvidenceHook,
                Some(family.as_str().to_owned()),
                format!("the {} family cites no evidence hook", family.as_str()),
            ));
        }
        if binding.consumer_surfaces.is_empty() {
            findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::FamilyMissingConsumerSurface,
                Some(family.as_str().to_owned()),
                format!("the {} family names no consumer surface", family.as_str()),
            ));
        }
        if binding.state_vocabulary.is_empty() {
            findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::FamilyMissingStateVocabulary,
                Some(family.as_str().to_owned()),
                format!(
                    "the {} family declares no state vocabulary",
                    family.as_str()
                ),
            ));
        }
    }

    for expected in AutomationSafetyLabelId::ALL {
        match safety_labels
            .iter()
            .find(|label| label.label_id == expected)
        {
            None => findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::SafetyLabelSetIncomplete,
                Some(expected.as_str().to_owned()),
                format!(
                    "the {} safety label is missing from the vocabulary",
                    expected.as_str()
                ),
            )),
            Some(label) => {
                if label.label_kind != expected.kind() {
                    findings.push(BaselineValidationFinding::blocker(
                        BaselineFindingKind::SafetyLabelMiscategorized,
                        Some(expected.as_str().to_owned()),
                        format!(
                            "the {} safety label is categorized as {} but must be {}",
                            expected.as_str(),
                            label.label_kind.as_str(),
                            expected.kind().as_str()
                        ),
                    ));
                }
            }
        }
    }

    if reused_contract_refs.is_empty() {
        findings.push(BaselineValidationFinding::blocker(
            BaselineFindingKind::ReusedContractRefMissing,
            None,
            "the baseline cites no reused contract refs",
        ));
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(BaselineValidationFinding::blocker(
                BaselineFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn promotion_state_for_findings(
    findings: &[BaselineValidationFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == BaselineFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == BaselineFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn baseline_digest(input: &AutomationContractBaselineInput) -> String {
    let mut tokens: Vec<String> = input
        .object_families
        .iter()
        .map(|binding| binding.family.as_str().to_owned())
        .collect();
    tokens.extend(
        input
            .safety_labels
            .iter()
            .map(|label| label.label_id.as_str().to_owned()),
    );
    tokens.sort_unstable();
    let refs: Vec<&str> = tokens.iter().map(String::as_str).collect();
    fnv1a64(&refs)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for item in items_in_order {
        for byte in item.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

// ---------------------------------------------------------------------------
// Seed
// ---------------------------------------------------------------------------

/// Builds the canonical stable baseline input.
pub fn current_automation_contract_baseline_input() -> AutomationContractBaselineInput {
    AutomationContractBaselineInput {
        packet_id: AUTOMATION_CONTRACT_BASELINE_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        object_families: canonical_object_families(),
        safety_labels: canonical_safety_labels(),
        reused_contract_refs: canonical_reused_contract_refs(),
        fixture_refs: canonical_fixture_refs(),
        invariants: BaselineInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable baseline packet.
pub fn seeded_automation_contract_baseline_packet() -> AutomationContractBaselinePacket {
    AutomationContractBaselinePacket::materialize(current_automation_contract_baseline_input())
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_automation_contract_baseline_packet(
    packet: &AutomationContractBaselinePacket,
) -> Result<(), Vec<BaselineValidationFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn s(value: &str) -> String {
    value.to_owned()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn canonical_reused_contract_refs() -> Vec<String> {
    strings(&[
        RECIPE_MANIFEST_SCHEMA_REF,
        RUN_RECORD_SCHEMA_REF,
        RUN_HISTORY_ROW_SCHEMA_REF,
        RUN_SUMMARY_EXPORT_SCHEMA_REF,
        CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
        "schemas/commands/command_descriptor.schema.json",
        "schemas/commands/shareability_metadata.schema.json",
        "docs/automation/recipe_and_macro_contract.md",
        "docs/automation/run_history_contract.md",
        "docs/automation/preview-and-lifecycle.md",
    ])
}

fn canonical_fixture_refs() -> Vec<String> {
    strings(&[
        "fixtures/automation/m5/recipe-macro/recipe_builder_session_preview_ready.json",
        "fixtures/automation/m5/recipe-macro/recipe_builder_session_blocked.json",
        "fixtures/automation/m5/recipe-macro/parameter_review_sheet.json",
        "fixtures/automation/m5/recipe-macro/dry_run_explain_packet.json",
        "fixtures/automation/m5/recipe-macro/macro_session_stopped_promotable.json",
        "fixtures/automation/m5/recipe-macro/macro_session_discarded.json",
    ])
}

fn canonical_object_families() -> Vec<ObjectFamilyBinding> {
    vec![
        ObjectFamilyBinding {
            family: AutomationObjectFamily::RecipeBuilder,
            title: s("Recipe builder authoring session"),
            schema_ref: s(RECIPE_BUILDER_SCHEMA_REF),
            doc_anchor: s("docs/m5/recipe-builder-and-macro-contract.md#recipe-builder"),
            state_vocabulary: RecipeBuilderStateClass::ALL
                .iter()
                .map(|state| state.as_str().to_owned())
                .collect(),
            evidence_hook_refs: strings(&[
                RECIPE_MANIFEST_SCHEMA_REF,
                "schemas/commands/command_descriptor.schema.json",
                "fixtures/automation/m5/recipe-macro/recipe_builder_session_preview_ready.json",
            ]),
            consumer_surfaces: strings(&[
                "desktop_recipe_builder",
                "cli_headless",
                "ai_assistant",
                "support_export",
            ]),
            reused_from_refs: strings(&[
                RECIPE_MANIFEST_SCHEMA_REF,
                "schemas/commands/command_descriptor.schema.json",
                "schemas/commands/shareability_metadata.schema.json",
            ]),
        },
        ObjectFamilyBinding {
            family: AutomationObjectFamily::ParameterReview,
            title: s("Pre-apply parameter-review sheet"),
            schema_ref: s(RECIPE_BUILDER_SCHEMA_REF),
            doc_anchor: s("docs/m5/recipe-builder-and-macro-contract.md#parameter-review"),
            state_vocabulary: ParameterReviewVerdictClass::ALL
                .iter()
                .map(|state| state.as_str().to_owned())
                .collect(),
            evidence_hook_refs: strings(&[
                "schemas/commands/command_descriptor.schema.json",
                "fixtures/automation/m5/recipe-macro/parameter_review_sheet.json",
            ]),
            consumer_surfaces: strings(&[
                "desktop_recipe_builder",
                "cli_headless",
                "ai_assistant",
                "support_export",
            ]),
            reused_from_refs: strings(&[
                "schemas/commands/command_descriptor.schema.json",
                "schemas/commands/shareability_metadata.schema.json",
            ]),
        },
        ObjectFamilyBinding {
            family: AutomationObjectFamily::DryRunExplain,
            title: s("Dry-run and explain preview packet"),
            schema_ref: s(RECIPE_BUILDER_SCHEMA_REF),
            doc_anchor: s("docs/m5/recipe-builder-and-macro-contract.md#dry-run-and-explain"),
            state_vocabulary: DryRunOutcomeClass::ALL
                .iter()
                .map(|state| state.as_str().to_owned())
                .collect(),
            evidence_hook_refs: strings(&[
                RUN_RECORD_SCHEMA_REF,
                "docs/automation/preview-and-lifecycle.md",
                "fixtures/automation/m5/recipe-macro/dry_run_explain_packet.json",
            ]),
            consumer_surfaces: strings(&[
                "desktop_recipe_builder",
                "command_palette",
                "cli_headless",
                "ai_assistant",
                "support_export",
            ]),
            reused_from_refs: strings(&[
                RUN_RECORD_SCHEMA_REF,
                "docs/automation/preview-and-lifecycle.md",
            ]),
        },
        ObjectFamilyBinding {
            family: AutomationObjectFamily::RunHistory,
            title: s("Automation run-history row"),
            schema_ref: s(RUN_HISTORY_ROW_SCHEMA_REF),
            doc_anchor: s("docs/m5/recipe-builder-and-macro-contract.md#run-history"),
            state_vocabulary: strings(&[
                "recorded_macro_layer",
                "declarative_recipe_layer",
                "managed_only_template_layer",
                "extension_or_external_automation_layer",
                "headless_safe_run_layer",
            ]),
            evidence_hook_refs: strings(&[RUN_RECORD_SCHEMA_REF, RUN_SUMMARY_EXPORT_SCHEMA_REF]),
            consumer_surfaces: strings(&[
                "desktop_run_history",
                "cli_headless",
                "ai_assistant",
                "support_export",
                "organization_audit",
            ]),
            reused_from_refs: strings(&[
                RUN_RECORD_SCHEMA_REF,
                RUN_HISTORY_ROW_SCHEMA_REF,
                RUN_SUMMARY_EXPORT_SCHEMA_REF,
                "docs/automation/run_history_contract.md",
            ]),
        },
        ObjectFamilyBinding {
            family: AutomationObjectFamily::MacroRecorder,
            title: s("Macro-recorder session"),
            schema_ref: s(MACRO_SESSION_SCHEMA_REF),
            doc_anchor: s("docs/m5/recipe-builder-and-macro-contract.md#macro-recorder"),
            state_vocabulary: MacroRecorderStateClass::ALL
                .iter()
                .map(|state| state.as_str().to_owned())
                .collect(),
            evidence_hook_refs: strings(&[
                RECIPE_MANIFEST_SCHEMA_REF,
                RUN_RECORD_SCHEMA_REF,
                "fixtures/automation/m5/recipe-macro/macro_session_stopped_promotable.json",
            ]),
            consumer_surfaces: strings(&[
                "desktop_macro_recorder",
                "cli_headless",
                "support_export",
            ]),
            reused_from_refs: strings(&[RECIPE_MANIFEST_SCHEMA_REF, RUN_RECORD_SCHEMA_REF]),
        },
        ObjectFamilyBinding {
            family: AutomationObjectFamily::SafetyLabels,
            title: s("Automation safety-label vocabulary"),
            schema_ref: s(RECIPE_BUILDER_SCHEMA_REF),
            doc_anchor: s("docs/m5/recipe-builder-and-macro-contract.md#automation-safety-labels"),
            state_vocabulary: AutomationSafetyLabelId::ALL
                .iter()
                .map(|label| label.as_str().to_owned())
                .collect(),
            evidence_hook_refs: strings(&[
                CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
                "schemas/commands/shareability_metadata.schema.json",
            ]),
            consumer_surfaces: strings(&[
                "command_palette",
                "desktop_recipe_builder",
                "desktop_macro_recorder",
                "cli_headless",
                "ai_assistant",
                "support_export",
            ]),
            reused_from_refs: strings(&[
                CONTROLLED_AUTOMATION_LABEL_SCHEMA_REF,
                "schemas/commands/shareability_metadata.schema.json",
                "docs/automation/preview-and-lifecycle.md",
            ]),
        },
    ]
}

// ---------------------------------------------------------------------------
// Worked-example seeds (the checked-in recipe-macro fixtures)
// ---------------------------------------------------------------------------

/// Canonical preview-ready recipe-builder session worked example.
pub fn seeded_recipe_builder_session_preview_ready() -> RecipeBuilderSession {
    RecipeBuilderSession {
        record_kind: s("recipe_builder_session_record"),
        recipe_builder_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
        builder_id: s("builder:format-and-stage:v1"),
        draft_recipe_revision_ref: s("recipe-rev:format-and-stage:1"),
        title: s("Format changed files and stage them"),
        summary: s("Formats the changed files in the workspace, then stages them for review."),
        builder_state_class: RecipeBuilderStateClass::PreviewReady,
        authoring_language_class: s("declarative_yaml_recipe"),
        step_drafts: vec![
            RecipeBuilderStepDraft {
                step_id: s("step:format"),
                step_kind: s("invoke_descriptor_command"),
                command_id: s("command:editor.format_changed"),
                command_revision_ref: s("command-rev:editor.format_changed:7"),
                canonical_verb: s("editor.format_changed"),
                capability_declarations: strings(&["reversible_workspace_filesystem_mutation"]),
                projected_safety_labels: vec![
                    AutomationSafetyLabelId::RecipeSafe,
                    AutomationSafetyLabelId::HeadlessSafe,
                    AutomationSafetyLabelId::WritesFiles,
                ],
            },
            RecipeBuilderStepDraft {
                step_id: s("step:stage"),
                step_kind: s("invoke_descriptor_command"),
                command_id: s("command:vcs.stage_changed"),
                command_revision_ref: s("command-rev:vcs.stage_changed:3"),
                canonical_verb: s("vcs.stage_changed"),
                capability_declarations: strings(&["reversible_workspace_filesystem_mutation"]),
                projected_safety_labels: vec![
                    AutomationSafetyLabelId::RecipeSafe,
                    AutomationSafetyLabelId::HeadlessSafe,
                    AutomationSafetyLabelId::WritesFiles,
                ],
            },
        ],
        projected_safety_labels: vec![
            AutomationSafetyLabelId::RecipeSafe,
            AutomationSafetyLabelId::HeadlessSafe,
            AutomationSafetyLabelId::WritesFiles,
        ],
        validation_findings: Vec::new(),
        parameter_review_sheet_ref: s("sheet:format-and-stage:v1"),
        dry_run_explain_packet_ref: s("dry-run:format-and-stage:v1"),
        manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
        minted_at: s("2026-06-18T00:00:00Z"),
    }
}

/// Canonical blocked recipe-builder session worked example.
///
/// A step cites a `ui_only_interactive` command, so the builder refuses to emit a
/// recipe manifest: the draft is `blocked` with a blocking validation finding.
pub fn seeded_recipe_builder_session_blocked() -> RecipeBuilderSession {
    RecipeBuilderSession {
        record_kind: s("recipe_builder_session_record"),
        recipe_builder_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
        builder_id: s("builder:interactive-merge:v1"),
        draft_recipe_revision_ref: s("recipe-rev:interactive-merge:1"),
        title: s("Open interactive merge tool"),
        summary: s("Attempts to add an interactive-only command to a recipe; the builder blocks it."),
        builder_state_class: RecipeBuilderStateClass::Blocked,
        authoring_language_class: s("declarative_yaml_recipe"),
        step_drafts: vec![RecipeBuilderStepDraft {
            step_id: s("step:open-merge-tool"),
            step_kind: s("invoke_descriptor_command"),
            command_id: s("command:vcs.open_interactive_merge"),
            command_revision_ref: s("command-rev:vcs.open_interactive_merge:2"),
            canonical_verb: s("vcs.open_interactive_merge"),
            capability_declarations: strings(&["reversible_local_editor_mutation"]),
            projected_safety_labels: vec![AutomationSafetyLabelId::UiOnly],
        }],
        projected_safety_labels: vec![AutomationSafetyLabelId::UiOnly],
        validation_findings: vec![BuilderValidationFinding {
            finding_kind: s("ui_only_command_not_recipe_safe"),
            severity: s("blocker"),
            summary: s(
                "step open-merge-tool cites a UI-only command that is not admissible to a declarative recipe",
            ),
        }],
        parameter_review_sheet_ref: s("sheet:interactive-merge:v1"),
        dry_run_explain_packet_ref: s("dry-run:interactive-merge:v1"),
        manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
        minted_at: s("2026-06-18T00:00:00Z"),
    }
}

/// Canonical parameter-review sheet worked example.
pub fn seeded_parameter_review_sheet() -> ParameterReviewSheet {
    ParameterReviewSheet {
        record_kind: s("parameter_review_sheet_record"),
        recipe_builder_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
        sheet_id: s("sheet:format-and-stage:v1"),
        builder_id: s("builder:format-and-stage:v1"),
        draft_recipe_revision_ref: s("recipe-rev:format-and-stage:1"),
        rows: vec![
            ParameterReviewRow {
                parameter_name: s("scope"),
                inspection_kind: ArgumentInspectionKind::SelectionBackedArgumentRef,
                verdict_class: ParameterReviewVerdictClass::Resolved,
                required: true,
                sensitivity_class: s("metadata_safe_default"),
                summary: s("scope is bound to the changed-file selection"),
            },
            ParameterReviewRow {
                parameter_name: s("formatter_profile"),
                inspection_kind: ArgumentInspectionKind::DefaultFromDescriptorArgumentRef,
                verdict_class: ParameterReviewVerdictClass::Resolved,
                required: false,
                sensitivity_class: s("metadata_safe_default"),
                summary: s("formatter_profile defaults to the workspace formatter profile"),
            },
            ParameterReviewRow {
                parameter_name: s("commit_message"),
                inspection_kind: ArgumentInspectionKind::TypedArgumentSlotRef,
                verdict_class: ParameterReviewVerdictClass::NeedsInput,
                required: false,
                sensitivity_class: s("metadata_safe_default"),
                summary: s("commit_message is optional and still empty"),
            },
        ],
        unresolved_required_count: 0,
        minted_at: s("2026-06-18T00:00:00Z"),
    }
}

/// Canonical dry-run / explain packet worked example.
pub fn seeded_dry_run_explain_packet() -> DryRunExplainPacket {
    DryRunExplainPacket {
        record_kind: s("dry_run_explain_packet_record"),
        recipe_builder_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
        packet_id: s("dry-run:format-and-stage:v1"),
        builder_id: s("builder:format-and-stage:v1"),
        draft_recipe_revision_ref: s("recipe-rev:format-and-stage:1"),
        dry_run_outcome_class: DryRunOutcomeClass::WouldApply,
        aggregate_safety_labels: vec![
            AutomationSafetyLabelId::RecipeSafe,
            AutomationSafetyLabelId::HeadlessSafe,
            AutomationSafetyLabelId::WritesFiles,
        ],
        step_explanations: vec![
            DryRunStepExplanation {
                step_id: s("step:format"),
                canonical_verb: s("editor.format_changed"),
                explanation: s(
                    "Reformats the changed files in place; no files are created or deleted.",
                ),
                capability_declarations: strings(&["reversible_workspace_filesystem_mutation"]),
                projected_safety_labels: vec![AutomationSafetyLabelId::WritesFiles],
                reversible: true,
                blast_radius_summary: s("changed files in the current workspace only"),
            },
            DryRunStepExplanation {
                step_id: s("step:stage"),
                canonical_verb: s("vcs.stage_changed"),
                explanation: s("Stages the changed files; staging is reversible with unstage."),
                capability_declarations: strings(&["reversible_workspace_filesystem_mutation"]),
                projected_safety_labels: vec![AutomationSafetyLabelId::WritesFiles],
                reversible: true,
                blast_radius_summary: s("the workspace VCS index only"),
            },
        ],
        preview_posture_class: s("preview_supported"),
        approval_posture_class: s("no_approval_required"),
        run_record_schema_ref: s(RUN_RECORD_SCHEMA_REF),
        minted_at: s("2026-06-18T00:00:00Z"),
    }
}

/// Canonical stopped, promotable macro-session worked example.
pub fn seeded_macro_session_stopped_promotable() -> MacroSession {
    MacroSession {
        record_kind: s("macro_session_record"),
        macro_session_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
        session_id: s("macro-session:rename-symbol-block:v1"),
        title: s("Rename symbol across the visible block"),
        summary: s("Records multi-cursor edits renaming a symbol across the visible editor block."),
        recorder_state_class: MacroRecorderStateClass::Stopped,
        storage_scope_class: s("workspace_scope_local_only"),
        projected_safety_labels: vec![
            AutomationSafetyLabelId::MacroSafe,
            AutomationSafetyLabelId::UiOnly,
        ],
        captured_steps: vec![
            MacroCaptureStep {
                step_id: s("capture:select"),
                surface_class: s("editor_selection_and_cursor_state"),
                state_digest: ContentAddress {
                    digest_algorithm: s("sha256"),
                    digest_hex: s("11111111111111111111111111111111"),
                    digest_size_bytes: 32,
                },
                replay_posture_class: s("replay_ui_or_editor_state_only"),
                captured_at: s("2026-06-18T00:00:00Z"),
            },
            MacroCaptureStep {
                step_id: s("capture:rename"),
                surface_class: s("editor_multi_cursor_edits"),
                state_digest: ContentAddress {
                    digest_algorithm: s("sha256"),
                    digest_hex: s("22222222222222222222222222222222"),
                    digest_size_bytes: 32,
                },
                replay_posture_class: s("replay_ui_or_editor_state_only"),
                captured_at: s("2026-06-18T00:00:01Z"),
            },
        ],
        redaction_class: s("metadata_safe_default"),
        promotion_affordance_class: MacroPromotionAffordanceClass::PromotableToDeclarativeRecipe,
        resulting_macro_manifest_ref: Some(s("macro:rename-symbol-block:1")),
        manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
        minted_at: s("2026-06-18T00:00:02Z"),
    }
}

/// Canonical discarded macro-session worked example.
pub fn seeded_macro_session_discarded() -> MacroSession {
    MacroSession {
        record_kind: s("macro_session_record"),
        macro_session_schema_version: AUTOMATION_CONTRACT_BASELINE_SCHEMA_VERSION,
        session_id: s("macro-session:scratch:v1"),
        title: s("Scratch recording"),
        summary: s("A recording the user discarded before any macro manifest was minted."),
        recorder_state_class: MacroRecorderStateClass::Discarded,
        storage_scope_class: s("user_scope_local_only"),
        projected_safety_labels: vec![
            AutomationSafetyLabelId::MacroSafe,
            AutomationSafetyLabelId::UiOnly,
        ],
        captured_steps: vec![MacroCaptureStep {
            step_id: s("capture:open-panel"),
            surface_class: s("ui_panel_open_close_state"),
            state_digest: ContentAddress {
                digest_algorithm: s("sha256"),
                digest_hex: s("33333333333333333333333333333333"),
                digest_size_bytes: 32,
            },
            replay_posture_class: s("replay_ui_or_editor_state_only"),
            captured_at: s("2026-06-18T00:00:00Z"),
        }],
        redaction_class: s("metadata_safe_default"),
        promotion_affordance_class: MacroPromotionAffordanceClass::NotPromotableUiOnly,
        resulting_macro_manifest_ref: None,
        manifest_target_schema_ref: s(RECIPE_MANIFEST_SCHEMA_REF),
        minted_at: s("2026-06-18T00:00:00Z"),
    }
}
