//! Declarative recipe-builder object and its first consumers.
//!
//! The automation contract baseline in
//! [`crate::m5_automation_contract_baseline`] froze *what* a recipe-builder
//! authoring session is ([`RecipeBuilderSession`]) and the reused safety-label
//! vocabulary every surface reads. This module makes the builder concrete: a
//! live, mutable [`RecipeBuilder`] that authors a recipe as **ordered,
//! declarative step objects** citing stable command identities, reorders steps
//! by drag *or* keyboard through one canonical operation, keeps unresolved and
//! blocked steps visible, exposes a copy-CLI string and an open-docs anchor that
//! cite the same command, and round-trips through export/import without losing
//! builder state or step provenance.
//!
//! The builder never holds private form state: [`RecipeBuilder::to_session_record`]
//! projects the live builder back onto the frozen [`RecipeBuilderSession`] so
//! every consumer reads the same command truth (each step quotes its
//! `command_id`, `command_revision_ref`, and `canonical_verb` from a command
//! descriptor) instead of inventing a feature-local wizard. The builder emits
//! declarative recipe manifests only ([`RecipeBuilder::manifest_target_schema_ref`]
//! is the recipe-manifest schema); it never embeds raw shell, paths, URLs,
//! prompt text, or credential material.
//!
//! [`RecipeBuilderFirstConsumersPacket`] binds the first M5 surfaces that author
//! or save recipes — notebook, task/test/debug, request/API, package, incident,
//! and the AI assistant — each to a seeded builder, and
//! [`RecipeBuilderFirstConsumersPacket::validate`] enforces the freeze
//! mechanically: every entrypoint binds the canonical builder, every step keeps
//! its command identity, every builder targets a declarative manifest, a UI-only
//! step stays blocked, copy-CLI and open-docs cite the same command, and every
//! invariant holds. A dropped entrypoint, a non-declarative manifest target, an
//! unblocked UI-only step, broken CLI/docs parity, or a violated invariant
//! *blocks stable*.
//!
//! The reviewer-facing landing page is [`/docs/m5/recipe-builder.md`]; the
//! cross-tool boundary schema is
//! [`/schemas/automation/recipe-builder-first-consumers.schema.json`]; the
//! reused authoring-session schema is
//! [`/schemas/automation/recipe-builder.schema.json`].
//!
//! [`/docs/m5/recipe-builder.md`]: ../../../docs/m5/recipe-builder.md
//! [`/schemas/automation/recipe-builder-first-consumers.schema.json`]: ../../../schemas/automation/recipe-builder-first-consumers.schema.json
//! [`/schemas/automation/recipe-builder.schema.json`]: ../../../schemas/automation/recipe-builder.schema.json

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::m5_automation_contract_baseline::{
    AutomationBaselinePromotionState, AutomationSafetyLabelId, BuilderValidationFinding,
    RecipeBuilderSession, RecipeBuilderStateClass, RecipeBuilderStepDraft,
    RECIPE_BUILDER_SCHEMA_REF, RECIPE_MANIFEST_SCHEMA_REF,
};

/// Stable record-kind tag for [`RecipeBuilderFirstConsumersPacket`].
pub const RECIPE_BUILDER_FIRST_CONSUMERS_RECORD_KIND: &str =
    "m5_recipe_builder_first_consumers_packet";

/// Stable record-kind tag for [`RecipeBuilderFirstConsumersSupportExport`].
pub const RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_recipe_builder_first_consumers_support_export";

/// Stable record-kind tag for [`RecipeBuilderFirstConsumersCliHeadlessView`].
pub const RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_recipe_builder_first_consumers_cli_headless";

/// Stable record-kind tag for [`RecipeBuilderExport`].
pub const RECIPE_BUILDER_EXPORT_RECORD_KIND: &str = "recipe_builder_export_record";

/// Stable record-kind tag for the authoring-session record the builder emits.
pub const RECIPE_BUILDER_SESSION_RECORD_KIND: &str = "recipe_builder_session_record";

/// Integer schema version for the first-consumers packet family.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the first-consumers boundary schema.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_REF: &str =
    "schemas/automation/recipe-builder-first-consumers.schema.json";

/// Repo-relative path of the reviewer contract doc for the builder object.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_DOC_REF: &str = "docs/m5/recipe-builder.md";

/// Repo-relative path of the checked-in first-consumers packet artifact.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/automation/recipe-builder-first-consumers/packet.json";

/// Repo-relative root the worked-example recipe-builder fixtures live under.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/automation/m5/recipe-builder";

/// Stable packet id minted by the seed.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_ID: &str =
    "automation:m5:recipe-builder-first-consumers:v1";

/// Stable support-export id minted by the seed inspector.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID: &str =
    "support-export:automation:m5:recipe-builder-first-consumers";

/// Stable CLI/headless view id minted by the seed inspector.
pub const RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_ID: &str =
    "cli-headless:automation:m5:recipe-builder-first-consumers";

/// Headless CLI binary name the copy-CLI action cites.
pub const RECIPE_BUILDER_CLI_BINARY: &str = "aureline";

/// Repo-relative command-reference doc the open-docs anchor projects into.
pub const RECIPE_BUILDER_COMMAND_DOCS_BASE: &str = "docs/commands/command_reference.md";

/// Default declarative authoring language the builder emits.
pub const RECIPE_BUILDER_DEFAULT_AUTHORING_LANGUAGE: &str = "declarative_yaml_recipe";

// ---------------------------------------------------------------------------
// Copy-CLI / open-docs parity helpers
// ---------------------------------------------------------------------------

/// Slugifies a dotted canonical verb into a docs-anchor fragment.
///
/// Dots and underscores become hyphens so `editor.format_changed` projects to
/// `editor-format-changed`. The open-docs anchor and copy-CLI string both derive
/// from the same `canonical_verb`, which is how copy-CLI/open-docs parity stays
/// mechanically checkable.
pub fn slugify_canonical_verb(canonical_verb: &str) -> String {
    canonical_verb
        .chars()
        .map(|ch| if ch == '.' || ch == '_' { '-' } else { ch })
        .collect()
}

/// Builds the reviewable copy-CLI invocation for one canonical verb.
///
/// The string cites the canonical verb only; it never embeds raw argv, paths,
/// URLs, or secrets.
pub fn copy_cli_for_verb(canonical_verb: &str) -> String {
    format!("{RECIPE_BUILDER_CLI_BINARY} command run {canonical_verb}")
}

/// Builds the open-docs anchor for one canonical verb.
pub fn open_docs_for_verb(canonical_verb: &str) -> String {
    format!(
        "{RECIPE_BUILDER_COMMAND_DOCS_BASE}#{}",
        slugify_canonical_verb(canonical_verb)
    )
}

/// Whether a step's copy-CLI string and open-docs anchor cite its canonical verb.
///
/// Parity holds when the copy-CLI string contains the verb and the open-docs
/// anchor ends with the slugified verb fragment. Both projecting from the same
/// verb is the guarantee that the CLI and the docs point at the same command.
pub fn step_parity_holds(canonical_verb: &str, copy_cli: &str, open_docs: &str) -> bool {
    let fragment = format!("#{}", slugify_canonical_verb(canonical_verb));
    copy_cli.contains(canonical_verb) && open_docs.ends_with(&fragment)
}

// ---------------------------------------------------------------------------
// Builder step
// ---------------------------------------------------------------------------

/// Why a builder step cannot be admitted to a declarative recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepBlockReason {
    /// The cited command is interactive only and is not admissible to a recipe.
    UiOnlyCommandNotRecipeSafe,
    /// The cited command is denied by admin policy.
    DeniedByPolicy,
    /// The cited command is denied at a trust gate.
    TrustGateDenied,
}

impl StepBlockReason {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            StepBlockReason::UiOnlyCommandNotRecipeSafe => "ui_only_command_not_recipe_safe",
            StepBlockReason::DeniedByPolicy => "denied_by_policy",
            StepBlockReason::TrustGateDenied => "trust_gate_denied",
        }
    }

    /// Reviewable summary of why the step is blocked.
    pub fn summary(self, step_id: &str) -> String {
        match self {
            StepBlockReason::UiOnlyCommandNotRecipeSafe => format!(
                "step {step_id} cites a UI-only command that is not admissible to a declarative recipe"
            ),
            StepBlockReason::DeniedByPolicy => {
                format!("step {step_id} cites a command denied by admin policy")
            }
            StepBlockReason::TrustGateDenied => {
                format!("step {step_id} cites a command denied at a trust gate")
            }
        }
    }
}

/// One ordered step in a live recipe builder.
///
/// The step wraps a [`RecipeBuilderStepDraft`] (the reused command truth) and
/// adds the authoring affordances the builder owns: unresolved argument slots,
/// an optional block reason, and the copy-CLI / open-docs actions. The draft is
/// never rewritten; reorders move the whole step so its command identity is
/// preserved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderStep {
    /// The reused command-truth draft this step cites.
    pub draft: RecipeBuilderStepDraft,
    /// Reviewable names of required argument slots still needing input.
    pub unresolved_argument_slots: Vec<String>,
    /// Why the step is blocked from a declarative recipe, if it is.
    pub block_reason: Option<StepBlockReason>,
    /// Reviewable headless invocation citing the step's canonical verb.
    pub copy_cli: String,
    /// Docs anchor resolving to the step's canonical verb.
    pub open_docs: String,
}

impl RecipeBuilderStep {
    /// Authors a step from a command-truth draft and its unresolved slots.
    ///
    /// A draft that projects the `ui_only` safety label is blocked on authoring:
    /// the builder refuses to silently produce an inadmissible recipe.
    pub fn author(draft: RecipeBuilderStepDraft, unresolved_argument_slots: Vec<String>) -> Self {
        let block_reason = if draft
            .projected_safety_labels
            .contains(&AutomationSafetyLabelId::UiOnly)
        {
            Some(StepBlockReason::UiOnlyCommandNotRecipeSafe)
        } else {
            None
        };
        let copy_cli = copy_cli_for_verb(&draft.canonical_verb);
        let open_docs = open_docs_for_verb(&draft.canonical_verb);
        RecipeBuilderStep {
            draft,
            unresolved_argument_slots,
            block_reason,
            copy_cli,
            open_docs,
        }
    }

    /// The opaque step id (delegates to the draft).
    pub fn step_id(&self) -> &str {
        &self.draft.step_id
    }

    /// The canonical verb the step cites (delegates to the draft).
    pub fn canonical_verb(&self) -> &str {
        &self.draft.canonical_verb
    }

    /// Whether the step still needs argument input.
    pub fn is_unresolved(&self) -> bool {
        !self.unresolved_argument_slots.is_empty()
    }

    /// Whether copy-CLI and open-docs cite this step's canonical verb.
    pub fn parity_holds(&self) -> bool {
        step_parity_holds(self.canonical_verb(), &self.copy_cli, &self.open_docs)
    }
}

// ---------------------------------------------------------------------------
// Reorder
// ---------------------------------------------------------------------------

/// A reorder gesture, as produced by a drag handle or a keyboard shortcut.
///
/// Drag and keyboard both resolve to one canonical target index, so a recipe
/// reordered by drag and the same recipe reordered by keyboard converge on the
/// identical step order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReorderGesture {
    /// A drag handle dropped the step at the given target index.
    DragToIndex(usize),
    /// A keyboard shortcut moved the step one position earlier.
    KeyboardMoveUp,
    /// A keyboard shortcut moved the step one position later.
    KeyboardMoveDown,
}

impl ReorderGesture {
    /// Resolves the gesture to a canonical target index, clamped to the recipe.
    pub fn resolve_target(self, from_index: usize, step_count: usize) -> usize {
        let last = step_count.saturating_sub(1);
        match self {
            ReorderGesture::DragToIndex(index) => index.min(last),
            ReorderGesture::KeyboardMoveUp => from_index.saturating_sub(1),
            ReorderGesture::KeyboardMoveDown => (from_index + 1).min(last),
        }
    }

    /// The recorded gesture kind for the reorder log.
    pub fn kind(self) -> ReorderGestureKind {
        match self {
            ReorderGesture::DragToIndex(_) => ReorderGestureKind::DragToIndex,
            ReorderGesture::KeyboardMoveUp => ReorderGestureKind::KeyboardMoveUp,
            ReorderGesture::KeyboardMoveDown => ReorderGestureKind::KeyboardMoveDown,
        }
    }
}

/// The serializable kind of a recorded reorder gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReorderGestureKind {
    /// A drag handle dropped the step at a target index.
    DragToIndex,
    /// A keyboard shortcut moved the step one position earlier.
    KeyboardMoveUp,
    /// A keyboard shortcut moved the step one position later.
    KeyboardMoveDown,
}

impl ReorderGestureKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            ReorderGestureKind::DragToIndex => "drag_to_index",
            ReorderGestureKind::KeyboardMoveUp => "keyboard_move_up",
            ReorderGestureKind::KeyboardMoveDown => "keyboard_move_down",
        }
    }
}

/// One recorded reorder, preserved so step provenance survives export/import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReorderEvent {
    /// The step that moved (its identity is preserved by the move).
    pub step_id: String,
    /// The gesture that produced the move.
    pub gesture_kind: ReorderGestureKind,
    /// The step's index before the move.
    pub from_index: u32,
    /// The step's index after the move.
    pub to_index: u32,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// An error raised by a [`RecipeBuilder`] mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecipeBuilderError {
    /// No step with the given id is present.
    StepNotFound(String),
    /// A step with the given id is already present.
    DuplicateStepId(String),
    /// An insert index is past the end of the recipe.
    InsertIndexOutOfRange {
        /// The requested index.
        index: usize,
        /// The current step count.
        len: usize,
    },
}

impl std::fmt::Display for RecipeBuilderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeBuilderError::StepNotFound(step_id) => {
                write!(formatter, "no step with id {step_id} is present")
            }
            RecipeBuilderError::DuplicateStepId(step_id) => {
                write!(formatter, "a step with id {step_id} is already present")
            }
            RecipeBuilderError::InsertIndexOutOfRange { index, len } => {
                write!(formatter, "insert index {index} is past the end ({len})")
            }
        }
    }
}

impl std::error::Error for RecipeBuilderError {}

// ---------------------------------------------------------------------------
// Recipe builder
// ---------------------------------------------------------------------------

/// The live, mutable recipe-builder authoring object.
///
/// The builder owns the ordered list of [`RecipeBuilderStep`]s and the reorder
/// log; it derives its [`RecipeBuilderStateClass`] and validation findings from
/// the steps, projects the reused safety-label union, and emits the frozen
/// [`RecipeBuilderSession`] on demand. It holds no private form state: every
/// projection reads back through the step drafts, so a consumer authoring a
/// recipe reuses command truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilder {
    /// Opaque builder session id.
    pub builder_id: String,
    /// The surface that opened this builder.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Opaque draft recipe revision ref.
    pub draft_recipe_revision_ref: String,
    /// Reviewable title.
    pub title: String,
    /// Reviewable summary sentence.
    pub summary: String,
    /// Re-exported declarative authoring-language class.
    pub authoring_language_class: String,
    /// Ordered authored steps.
    pub steps: Vec<RecipeBuilderStep>,
    /// Append-only log of reorder gestures applied to the recipe.
    pub reorder_log: Vec<ReorderEvent>,
    /// Opaque ref to the parameter-review sheet bound to this builder.
    pub parameter_review_sheet_ref: String,
    /// Opaque ref to the dry-run / explain packet bound to this builder.
    pub dry_run_explain_packet_ref: String,
    /// Schema the builder emits a declarative manifest against on save.
    pub manifest_target_schema_ref: String,
    /// Monotonic mint timestamp.
    pub minted_at: String,
}

impl RecipeBuilder {
    /// Opens an empty builder for one entrypoint.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        entrypoint: RecipeBuilderEntrypoint,
        builder_id: impl Into<String>,
        draft_recipe_revision_ref: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        parameter_review_sheet_ref: impl Into<String>,
        dry_run_explain_packet_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        RecipeBuilder {
            builder_id: builder_id.into(),
            entrypoint,
            draft_recipe_revision_ref: draft_recipe_revision_ref.into(),
            title: title.into(),
            summary: summary.into(),
            authoring_language_class: RECIPE_BUILDER_DEFAULT_AUTHORING_LANGUAGE.to_owned(),
            steps: Vec::new(),
            reorder_log: Vec::new(),
            parameter_review_sheet_ref: parameter_review_sheet_ref.into(),
            dry_run_explain_packet_ref: dry_run_explain_packet_ref.into(),
            manifest_target_schema_ref: RECIPE_MANIFEST_SCHEMA_REF.to_owned(),
            minted_at: minted_at.into(),
        }
    }

    /// Appends an authored step at the end of the recipe.
    pub fn append_step(
        &mut self,
        draft: RecipeBuilderStepDraft,
        unresolved_argument_slots: Vec<String>,
    ) -> Result<(), RecipeBuilderError> {
        self.insert_step(self.steps.len(), draft, unresolved_argument_slots)
    }

    /// Inserts an authored step at the given index.
    pub fn insert_step(
        &mut self,
        index: usize,
        draft: RecipeBuilderStepDraft,
        unresolved_argument_slots: Vec<String>,
    ) -> Result<(), RecipeBuilderError> {
        if index > self.steps.len() {
            return Err(RecipeBuilderError::InsertIndexOutOfRange {
                index,
                len: self.steps.len(),
            });
        }
        if self.step_index(&draft.step_id).is_some() {
            return Err(RecipeBuilderError::DuplicateStepId(draft.step_id.clone()));
        }
        self.steps.insert(
            index,
            RecipeBuilderStep::author(draft, unresolved_argument_slots),
        );
        Ok(())
    }

    /// Removes the step with the given id, returning it.
    pub fn remove_step(&mut self, step_id: &str) -> Result<RecipeBuilderStep, RecipeBuilderError> {
        let index = self
            .step_index(step_id)
            .ok_or_else(|| RecipeBuilderError::StepNotFound(step_id.to_owned()))?;
        Ok(self.steps.remove(index))
    }

    /// The current index of the step with the given id.
    pub fn step_index(&self, step_id: &str) -> Option<usize> {
        self.steps.iter().position(|step| step.step_id() == step_id)
    }

    /// Reorders a step by drag or keyboard gesture, recording the move.
    ///
    /// Both gesture kinds resolve to one canonical target index, so the step's
    /// identity is preserved and a drag and the equivalent keyboard moves
    /// converge on the same order.
    pub fn reorder(
        &mut self,
        step_id: &str,
        gesture: ReorderGesture,
    ) -> Result<ReorderEvent, RecipeBuilderError> {
        let from_index = self
            .step_index(step_id)
            .ok_or_else(|| RecipeBuilderError::StepNotFound(step_id.to_owned()))?;
        let to_index = gesture.resolve_target(from_index, self.steps.len());
        if to_index != from_index {
            let step = self.steps.remove(from_index);
            self.steps.insert(to_index, step);
        }
        let event = ReorderEvent {
            step_id: step_id.to_owned(),
            gesture_kind: gesture.kind(),
            from_index: from_index as u32,
            to_index: to_index as u32,
        };
        self.reorder_log.push(event.clone());
        Ok(event)
    }

    /// Moves a step one position earlier by keyboard.
    pub fn move_step_up(&mut self, step_id: &str) -> Result<ReorderEvent, RecipeBuilderError> {
        self.reorder(step_id, ReorderGesture::KeyboardMoveUp)
    }

    /// Moves a step one position later by keyboard.
    pub fn move_step_down(&mut self, step_id: &str) -> Result<ReorderEvent, RecipeBuilderError> {
        self.reorder(step_id, ReorderGesture::KeyboardMoveDown)
    }

    /// Drags a step to a target index.
    pub fn drag_step_to(
        &mut self,
        step_id: &str,
        to_index: usize,
    ) -> Result<ReorderEvent, RecipeBuilderError> {
        self.reorder(step_id, ReorderGesture::DragToIndex(to_index))
    }

    /// Ordered step ids, the canonical view of the recipe's order.
    pub fn step_order(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| step.step_id().to_owned())
            .collect()
    }

    /// The reused safety-label union over the steps, in canonical order.
    pub fn projected_safety_labels(&self) -> Vec<AutomationSafetyLabelId> {
        AutomationSafetyLabelId::ALL
            .into_iter()
            .filter(|label| {
                self.steps
                    .iter()
                    .any(|step| step.draft.projected_safety_labels.contains(label))
            })
            .collect()
    }

    /// The validation findings derived from the steps.
    ///
    /// A blocked step raises a blocker keyed to its block reason; a step with no
    /// capability declarations raises a blocker that fails validation without
    /// hard-blocking. Findings keep blocked and invalid steps visible before the
    /// user trusts the recipe.
    pub fn validation_findings(&self) -> Vec<BuilderValidationFinding> {
        let mut findings = Vec::new();
        for step in &self.steps {
            if let Some(reason) = step.block_reason {
                findings.push(BuilderValidationFinding {
                    finding_kind: reason.as_str().to_owned(),
                    severity: "blocker".to_owned(),
                    summary: reason.summary(step.step_id()),
                });
            } else if step.draft.capability_declarations.is_empty() {
                findings.push(BuilderValidationFinding {
                    finding_kind: "step_missing_capability_declaration".to_owned(),
                    severity: "blocker".to_owned(),
                    summary: format!(
                        "step {} declares no capability and cannot be validated",
                        step.step_id()
                    ),
                });
            }
        }
        findings
    }

    /// Count of required argument slots still unresolved across all steps.
    pub fn unresolved_required_count(&self) -> u32 {
        self.steps
            .iter()
            .map(|step| step.unresolved_argument_slots.len() as u32)
            .sum()
    }

    /// Whether any step still needs argument input.
    pub fn has_unresolved_steps(&self) -> bool {
        self.steps.iter().any(RecipeBuilderStep::is_unresolved)
    }

    /// Whether any step is blocked from a declarative recipe.
    pub fn has_blocked_steps(&self) -> bool {
        self.steps.iter().any(|step| step.block_reason.is_some())
    }

    /// Derives the builder's authoring state from its steps.
    ///
    /// A step blocked by trust, policy, or a denied label drives `Blocked`; a
    /// non-blocking blocker finding drives `ValidationFailed`; an unresolved
    /// required slot keeps the recipe in `Draft`; an approval-required label
    /// drives `ApprovalRequired`; otherwise the recipe is `PreviewReady`.
    pub fn state_class(&self) -> RecipeBuilderStateClass {
        let findings = self.validation_findings();
        let has_blocker = findings.iter().any(|finding| finding.severity == "blocker");
        if has_blocker {
            if self.has_blocked_steps() {
                RecipeBuilderStateClass::Blocked
            } else {
                RecipeBuilderStateClass::ValidationFailed
            }
        } else if self.unresolved_required_count() > 0 {
            RecipeBuilderStateClass::Draft
        } else if self
            .projected_safety_labels()
            .contains(&AutomationSafetyLabelId::ApprovalRequired)
        {
            RecipeBuilderStateClass::ApprovalRequired
        } else {
            RecipeBuilderStateClass::PreviewReady
        }
    }

    /// Per-step copy-CLI lines, index-aligned with the steps.
    pub fn copy_cli_lines(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| step.copy_cli.clone())
            .collect()
    }

    /// Per-step open-docs anchors, index-aligned with the steps.
    pub fn open_docs_anchors(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| step.open_docs.clone())
            .collect()
    }

    /// The whole-recipe copy-CLI invocation citing the draft recipe revision.
    pub fn recipe_copy_cli(&self) -> String {
        format!(
            "{RECIPE_BUILDER_CLI_BINARY} recipe run {}",
            self.draft_recipe_revision_ref
        )
    }

    /// Whether every step's copy-CLI and open-docs cite its canonical verb.
    pub fn parity_holds(&self) -> bool {
        self.steps.iter().all(RecipeBuilderStep::parity_holds)
    }

    /// Projects the live builder onto the frozen authoring-session record.
    ///
    /// This is the proof the builder reuses command truth: each emitted step
    /// draft quotes the same `command_id`, `command_revision_ref`, and
    /// `canonical_verb` the builder holds.
    pub fn to_session_record(&self) -> RecipeBuilderSession {
        RecipeBuilderSession {
            record_kind: RECIPE_BUILDER_SESSION_RECORD_KIND.to_owned(),
            recipe_builder_schema_version: RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            builder_id: self.builder_id.clone(),
            draft_recipe_revision_ref: self.draft_recipe_revision_ref.clone(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            builder_state_class: self.state_class(),
            authoring_language_class: self.authoring_language_class.clone(),
            step_drafts: self.steps.iter().map(|step| step.draft.clone()).collect(),
            projected_safety_labels: self.projected_safety_labels(),
            validation_findings: self.validation_findings(),
            parameter_review_sheet_ref: self.parameter_review_sheet_ref.clone(),
            dry_run_explain_packet_ref: self.dry_run_explain_packet_ref.clone(),
            manifest_target_schema_ref: self.manifest_target_schema_ref.clone(),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Exports the builder, preserving full state and step provenance.
    pub fn export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RecipeBuilderExport {
        RecipeBuilderExport {
            record_kind: RECIPE_BUILDER_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            builder_state_class: self.state_class(),
            session_record: self.to_session_record(),
            builder: self.clone(),
            export_digest: fnv1a64(&self.digest_tokens()),
        }
    }

    /// Tokens hashed into the export digest, in step order.
    fn digest_tokens(&self) -> Vec<String> {
        let mut tokens = vec![
            self.builder_id.clone(),
            self.draft_recipe_revision_ref.clone(),
        ];
        for step in &self.steps {
            tokens.push(step.draft.command_id.clone());
            tokens.push(step.draft.command_revision_ref.clone());
            tokens.push(step.draft.canonical_verb.clone());
        }
        tokens
    }
}

/// A builder exported for sharing, support bundles, or rerun review.
///
/// The export nests the whole [`RecipeBuilder`] verbatim — so step order,
/// command identity, unresolved and blocked state, the copy-CLI / open-docs
/// actions, and the reorder log all survive — alongside the derived
/// authoring-session projection and an order-stable digest.
/// [`RecipeBuilderExport::import`] reconstructs the identical builder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Monotonic export timestamp.
    pub exported_at: String,
    /// Authoring state of the exported builder.
    pub builder_state_class: RecipeBuilderStateClass,
    /// The frozen authoring-session projection consumers read.
    pub session_record: RecipeBuilderSession,
    /// The builder, preserved verbatim for round-trip import.
    pub builder: RecipeBuilder,
    /// Order-stable digest over the builder's command provenance.
    pub export_digest: String,
}

impl RecipeBuilderExport {
    /// Reconstructs the builder from the export.
    pub fn import(&self) -> RecipeBuilder {
        self.builder.clone()
    }

    /// Whether the export carries inspectable provenance for every step.
    pub fn provenance_preserved(&self) -> bool {
        !self.builder.steps.is_empty()
            && self.builder.steps.iter().all(|step| {
                !step.draft.command_id.is_empty()
                    && !step.draft.command_revision_ref.is_empty()
                    && !step.draft.canonical_verb.is_empty()
            })
            && self.session_record.step_drafts.len() == self.builder.steps.len()
    }
}

// ---------------------------------------------------------------------------
// First-consumer entrypoints
// ---------------------------------------------------------------------------

/// One M5 surface that authors or saves recipes through the canonical builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeBuilderEntrypoint {
    /// The notebook surface (run and export notebook flows).
    Notebook,
    /// The task / test / debug surface.
    TaskTestDebug,
    /// The request / API workspace surface.
    RequestApi,
    /// The package / dependency surface.
    Package,
    /// The incident / runbook surface.
    Incident,
    /// The AI assistant surface (AI-proposed automation).
    AiAssistant,
}

impl RecipeBuilderEntrypoint {
    /// Every entrypoint in canonical (declaration) order.
    pub const ALL: [RecipeBuilderEntrypoint; 6] = [
        RecipeBuilderEntrypoint::Notebook,
        RecipeBuilderEntrypoint::TaskTestDebug,
        RecipeBuilderEntrypoint::RequestApi,
        RecipeBuilderEntrypoint::Package,
        RecipeBuilderEntrypoint::Incident,
        RecipeBuilderEntrypoint::AiAssistant,
    ];

    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            RecipeBuilderEntrypoint::Notebook => "notebook",
            RecipeBuilderEntrypoint::TaskTestDebug => "task_test_debug",
            RecipeBuilderEntrypoint::RequestApi => "request_api",
            RecipeBuilderEntrypoint::Package => "package",
            RecipeBuilderEntrypoint::Incident => "incident",
            RecipeBuilderEntrypoint::AiAssistant => "ai_assistant",
        }
    }

    /// Reviewable title.
    pub fn title(self) -> &'static str {
        match self {
            RecipeBuilderEntrypoint::Notebook => "Notebook automation",
            RecipeBuilderEntrypoint::TaskTestDebug => "Task, test, and debug automation",
            RecipeBuilderEntrypoint::RequestApi => "Request and API automation",
            RecipeBuilderEntrypoint::Package => "Package and dependency automation",
            RecipeBuilderEntrypoint::Incident => "Incident and runbook automation",
            RecipeBuilderEntrypoint::AiAssistant => "AI-assistant automation",
        }
    }
}

/// One entrypoint binding: the seeded builder a first consumer authors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderConsumerBinding {
    /// The entrypoint this binding describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Opaque builder id the consumer authored.
    pub builder_id: String,
    /// Authoring state of the consumer's builder.
    pub builder_state_class: RecipeBuilderStateClass,
    /// The frozen authoring-session record the consumer reuses.
    pub session_record: RecipeBuilderSession,
    /// Step count, carried for compact projections.
    pub step_count: u32,
    /// The reused safety-label union over the recipe.
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
    /// Per-step copy-CLI lines, index-aligned with the session step drafts.
    pub copy_cli_lines: Vec<String>,
    /// Per-step open-docs anchors, index-aligned with the session step drafts.
    pub open_docs_anchors: Vec<String>,
    /// Count of required argument slots still unresolved.
    pub unresolved_required_count: u32,
    /// Reviewable summary of what the consumer authored.
    pub entry_summary: String,
}

impl RecipeBuilderConsumerBinding {
    /// Builds a binding from a consumer's authored builder.
    pub fn from_builder(builder: &RecipeBuilder) -> Self {
        RecipeBuilderConsumerBinding {
            entrypoint: builder.entrypoint,
            title: builder.entrypoint.title().to_owned(),
            builder_id: builder.builder_id.clone(),
            builder_state_class: builder.state_class(),
            session_record: builder.to_session_record(),
            step_count: builder.steps.len() as u32,
            projected_safety_labels: builder.projected_safety_labels(),
            copy_cli_lines: builder.copy_cli_lines(),
            open_docs_anchors: builder.open_docs_anchors(),
            unresolved_required_count: builder.unresolved_required_count(),
            entry_summary: builder.summary.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Invariants and findings
// ---------------------------------------------------------------------------

/// Frozen invariants the first-consumers packet pins as schema-level constants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderInvariantsBlock {
    /// The builder reuses command truth, not private form state.
    pub builder_reuses_command_truth_not_private_form_state: bool,
    /// Every first-consumer entrypoint binds the canonical builder.
    pub every_entrypoint_binds_the_canonical_builder: bool,
    /// Steps are ordered and reorder preserves step identity.
    pub steps_are_ordered_and_reorder_preserves_identity: bool,
    /// Blocked or unresolved steps remain visible before the user trusts it.
    pub blocked_or_unresolved_steps_remain_visible: bool,
    /// Copy-CLI and open-docs cite the same command identity.
    pub copy_cli_and_open_docs_cite_the_same_command: bool,
    /// The builder emits declarative manifests only.
    pub builder_emits_declarative_manifests_only: bool,
    /// Builder state and step provenance survive export and import.
    pub builder_state_survives_export_import: bool,
}

impl RecipeBuilderInvariantsBlock {
    /// The frozen all-true invariants block.
    pub fn frozen() -> Self {
        RecipeBuilderInvariantsBlock {
            builder_reuses_command_truth_not_private_form_state: true,
            every_entrypoint_binds_the_canonical_builder: true,
            steps_are_ordered_and_reorder_preserves_identity: true,
            blocked_or_unresolved_steps_remain_visible: true,
            copy_cli_and_open_docs_cite_the_same_command: true,
            builder_emits_declarative_manifests_only: true,
            builder_state_survives_export_import: true,
        }
    }

    /// Returns the `(name, value)` pairs in declaration order.
    pub fn entries(&self) -> [(&'static str, bool); 7] {
        [
            (
                "builder_reuses_command_truth_not_private_form_state",
                self.builder_reuses_command_truth_not_private_form_state,
            ),
            (
                "every_entrypoint_binds_the_canonical_builder",
                self.every_entrypoint_binds_the_canonical_builder,
            ),
            (
                "steps_are_ordered_and_reorder_preserves_identity",
                self.steps_are_ordered_and_reorder_preserves_identity,
            ),
            (
                "blocked_or_unresolved_steps_remain_visible",
                self.blocked_or_unresolved_steps_remain_visible,
            ),
            (
                "copy_cli_and_open_docs_cite_the_same_command",
                self.copy_cli_and_open_docs_cite_the_same_command,
            ),
            (
                "builder_emits_declarative_manifests_only",
                self.builder_emits_declarative_manifests_only,
            ),
            (
                "builder_state_survives_export_import",
                self.builder_state_survives_export_import,
            ),
        ]
    }
}

/// Severity of a first-consumers validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstConsumersFindingSeverity {
    /// Blocks the packet from stable.
    Blocker,
    /// Narrows the packet below stable.
    Warning,
}

/// Kind of a first-consumers validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstConsumersFindingKind {
    /// A required first-consumer entrypoint is absent.
    MissingEntrypoint,
    /// An entrypoint binds a builder with no steps.
    EntrypointBuilderEmpty,
    /// A step is missing its command identity.
    StepMissingCommandIdentity,
    /// A builder targets a non-declarative manifest schema.
    NonDeclarativeManifestTarget,
    /// A UI-only step is present but the builder is not blocked.
    UiOnlyStepNotBlocked,
    /// A step's copy-CLI and open-docs do not cite the same command.
    CliDocsParityBroken,
    /// A frozen invariant is set false.
    InvariantViolated,
}

impl FirstConsumersFindingKind {
    /// Stable snake_case token.
    pub fn as_str(self) -> &'static str {
        match self {
            FirstConsumersFindingKind::MissingEntrypoint => "missing_entrypoint",
            FirstConsumersFindingKind::EntrypointBuilderEmpty => "entrypoint_builder_empty",
            FirstConsumersFindingKind::StepMissingCommandIdentity => {
                "step_missing_command_identity"
            }
            FirstConsumersFindingKind::NonDeclarativeManifestTarget => {
                "non_declarative_manifest_target"
            }
            FirstConsumersFindingKind::UiOnlyStepNotBlocked => "ui_only_step_not_blocked",
            FirstConsumersFindingKind::CliDocsParityBroken => "cli_docs_parity_broken",
            FirstConsumersFindingKind::InvariantViolated => "invariant_violated",
        }
    }
}

/// One blocking or warning finding raised by the first-consumers gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstConsumersFinding {
    /// The finding kind.
    pub finding_kind: FirstConsumersFindingKind,
    /// Whether the finding blocks stable or narrows below stable.
    pub severity: FirstConsumersFindingSeverity,
    /// Optional subject the finding is about.
    pub subject: Option<String>,
    /// Reviewable summary sentence.
    pub summary: String,
}

impl FirstConsumersFinding {
    fn blocker(
        finding_kind: FirstConsumersFindingKind,
        subject: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        FirstConsumersFinding {
            finding_kind,
            severity: FirstConsumersFindingSeverity::Blocker,
            subject,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// First-consumers packet
// ---------------------------------------------------------------------------

/// Mutable input the seed mints and the materializer freezes into a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderFirstConsumersInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<RecipeBuilderConsumerBinding>,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Frozen invariants block.
    pub invariants: RecipeBuilderInvariantsBlock,
}

/// Canonical M5 recipe-builder first-consumers packet.
///
/// The packet binds every first-consumer entrypoint to a seeded builder and
/// pins the freeze invariants. [`RecipeBuilderFirstConsumersPacket::validate`]
/// recomputes the findings so the fail-closed gate and the typed consumer agree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderFirstConsumersPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Monotonic mint timestamp.
    pub generated_at: String,
    /// Boundary schema ref for this packet.
    pub schema_ref: String,
    /// Reused authoring-session boundary schema ref.
    pub session_schema_ref: String,
    /// Declarative recipe-manifest schema the builder emits against.
    pub recipe_manifest_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Existing contracts this packet reuses instead of re-deciding.
    pub reused_contract_refs: Vec<String>,
    /// Entrypoint bindings.
    pub consumer_bindings: Vec<RecipeBuilderConsumerBinding>,
    /// Frozen invariants block.
    pub invariants: RecipeBuilderInvariantsBlock,
    /// Findings raised against this packet.
    pub validation_findings: Vec<FirstConsumersFinding>,
    /// Promotion state derived from the findings.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Order-invariant digest over entrypoint and command tokens.
    pub packet_digest: String,
}

impl RecipeBuilderFirstConsumersPacket {
    /// Freezes an input into a packet, computing findings, promotion, and digest.
    pub fn materialize(input: RecipeBuilderFirstConsumersInput) -> Self {
        let findings = validate_parts(&input.consumer_bindings, &input.invariants);
        let promotion_state = promotion_state_for_findings(&findings);
        let packet_digest = packet_digest(&input.consumer_bindings);
        RecipeBuilderFirstConsumersPacket {
            record_kind: RECIPE_BUILDER_FIRST_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            schema_ref: RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_REF.to_owned(),
            session_schema_ref: RECIPE_BUILDER_SCHEMA_REF.to_owned(),
            recipe_manifest_schema_ref: RECIPE_MANIFEST_SCHEMA_REF.to_owned(),
            doc_ref: RECIPE_BUILDER_FIRST_CONSUMERS_DOC_REF.to_owned(),
            reused_contract_refs: input.reused_contract_refs,
            consumer_bindings: input.consumer_bindings,
            invariants: input.invariants,
            validation_findings: findings,
            promotion_state,
            packet_digest,
        }
    }

    /// Re-validates the materialized packet.
    pub fn validate(&self) -> Vec<FirstConsumersFinding> {
        validate_parts(&self.consumer_bindings, &self.invariants)
    }

    /// Whether the packet promotes to stable.
    pub fn is_stable(&self) -> bool {
        self.promotion_state == AutomationBaselinePromotionState::Stable
    }

    /// The binding for one entrypoint, if present.
    pub fn binding(
        &self,
        entrypoint: RecipeBuilderEntrypoint,
    ) -> Option<&RecipeBuilderConsumerBinding> {
        self.consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
    }

    /// Entrypoint tokens in the order the packet stores them.
    pub fn entrypoint_tokens(&self) -> Vec<&'static str> {
        self.consumer_bindings
            .iter()
            .map(|binding| binding.entrypoint.as_str())
            .collect()
    }

    /// Builds the redacted support-export projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RecipeBuilderFirstConsumersSupportExport {
        RecipeBuilderFirstConsumersSupportExport {
            record_kind: RECIPE_BUILDER_FIRST_CONSUMERS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            packet_id: self.packet_id.clone(),
            packet_digest: self.packet_digest.clone(),
            promotion_state: self.promotion_state,
            consumer_rows: self
                .consumer_bindings
                .iter()
                .map(|binding| SupportExportConsumerRow {
                    entrypoint: binding.entrypoint,
                    title: binding.title.clone(),
                    builder_id: binding.builder_id.clone(),
                    builder_state_class: binding.builder_state_class,
                    step_count: binding.step_count,
                    unresolved_required_count: binding.unresolved_required_count,
                    projected_safety_labels: binding.projected_safety_labels.clone(),
                })
                .collect(),
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
    ) -> RecipeBuilderFirstConsumersCliHeadlessView {
        RecipeBuilderFirstConsumersCliHeadlessView {
            record_kind: RECIPE_BUILDER_FIRST_CONSUMERS_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: RECIPE_BUILDER_FIRST_CONSUMERS_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            packet_id: self.packet_id.clone(),
            promotion_state: self.promotion_state,
            consumer_lines: self
                .consumer_bindings
                .iter()
                .map(|binding| {
                    format!(
                        "{} builder={} state={} steps={} unresolved={}",
                        binding.entrypoint.as_str(),
                        binding.builder_id,
                        binding.builder_state_class.as_str(),
                        binding.step_count,
                        binding.unresolved_required_count,
                    )
                })
                .collect(),
        }
    }

    /// Compact text projection lines for `compact.txt`.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![format!(
            "packet {} schema_version={} promotion={} consumers={} digest={}",
            self.packet_id,
            self.schema_version,
            self.promotion_state.as_str(),
            self.consumer_bindings.len(),
            self.packet_digest,
        )];
        for binding in &self.consumer_bindings {
            lines.push(format!(
                "consumer {} builder={} state={} steps={} cli={} docs={}",
                binding.entrypoint.as_str(),
                binding.builder_id,
                binding.builder_state_class.as_str(),
                binding.step_count,
                binding.copy_cli_lines.len(),
                binding.open_docs_anchors.len(),
            ));
        }
        lines
    }
}

/// One support-export consumer row (redacted projection).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportConsumerRow {
    /// The entrypoint this row describes.
    pub entrypoint: RecipeBuilderEntrypoint,
    /// Reviewable title.
    pub title: String,
    /// Opaque builder id.
    pub builder_id: String,
    /// Authoring state of the consumer's builder.
    pub builder_state_class: RecipeBuilderStateClass,
    /// Step count.
    pub step_count: u32,
    /// Count of required argument slots still unresolved.
    pub unresolved_required_count: u32,
    /// The reused safety-label union over the recipe.
    pub projected_safety_labels: Vec<AutomationSafetyLabelId>,
}

/// Redacted support-export projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderFirstConsumersSupportExport {
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
    /// Packet digest carried for verification.
    pub packet_digest: String,
    /// Promotion state of the source packet.
    pub promotion_state: AutomationBaselinePromotionState,
    /// Consumer rows.
    pub consumer_rows: Vec<SupportExportConsumerRow>,
    /// Frozen invariants block.
    pub invariants: RecipeBuilderInvariantsBlock,
    /// Finding kinds carried for support review.
    pub finding_kinds: Vec<FirstConsumersFindingKind>,
}

impl RecipeBuilderFirstConsumersSupportExport {
    /// Whether the export is safe to cross a tenant or surface boundary.
    pub fn is_export_safe(&self) -> bool {
        !self.packet_id.is_empty()
            && !self.packet_digest.is_empty()
            && !self.consumer_rows.is_empty()
    }
}

/// Compact CLI / headless projection of the first-consumers packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeBuilderFirstConsumersCliHeadlessView {
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
    /// One line per consumer entrypoint.
    pub consumer_lines: Vec<String>,
}

impl RecipeBuilderFirstConsumersCliHeadlessView {
    /// Whether the view explains every entrypoint.
    pub fn every_entrypoint_explained(&self) -> bool {
        self.consumer_lines.len() == RecipeBuilderEntrypoint::ALL.len()
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_parts(
    consumer_bindings: &[RecipeBuilderConsumerBinding],
    invariants: &RecipeBuilderInvariantsBlock,
) -> Vec<FirstConsumersFinding> {
    let mut findings = Vec::new();

    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let Some(binding) = consumer_bindings
            .iter()
            .find(|binding| binding.entrypoint == entrypoint)
        else {
            findings.push(FirstConsumersFinding::blocker(
                FirstConsumersFindingKind::MissingEntrypoint,
                Some(entrypoint.as_str().to_owned()),
                format!(
                    "the {} entrypoint binds no canonical builder",
                    entrypoint.as_str()
                ),
            ));
            continue;
        };
        validate_binding(binding, &mut findings);
    }

    for (name, value) in invariants.entries() {
        if !value {
            findings.push(FirstConsumersFinding::blocker(
                FirstConsumersFindingKind::InvariantViolated,
                Some(name.to_owned()),
                format!("the invariant {name} is set false"),
            ));
        }
    }

    findings
}

fn validate_binding(
    binding: &RecipeBuilderConsumerBinding,
    findings: &mut Vec<FirstConsumersFinding>,
) {
    let entrypoint = binding.entrypoint.as_str();
    let drafts = &binding.session_record.step_drafts;

    if drafts.is_empty() {
        findings.push(FirstConsumersFinding::blocker(
            FirstConsumersFindingKind::EntrypointBuilderEmpty,
            Some(entrypoint.to_owned()),
            format!("the {entrypoint} entrypoint binds a builder with no steps"),
        ));
        return;
    }

    if binding.session_record.manifest_target_schema_ref != RECIPE_MANIFEST_SCHEMA_REF {
        findings.push(FirstConsumersFinding::blocker(
            FirstConsumersFindingKind::NonDeclarativeManifestTarget,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} builder targets {} instead of the declarative recipe manifest",
                binding.session_record.manifest_target_schema_ref
            ),
        ));
    }

    let has_ui_only_step = drafts.iter().any(|draft| {
        draft
            .projected_safety_labels
            .contains(&AutomationSafetyLabelId::UiOnly)
    });
    if has_ui_only_step && binding.builder_state_class != RecipeBuilderStateClass::Blocked {
        findings.push(FirstConsumersFinding::blocker(
            FirstConsumersFindingKind::UiOnlyStepNotBlocked,
            Some(entrypoint.to_owned()),
            format!(
                "the {entrypoint} builder cites a UI-only command but is {} rather than blocked",
                binding.builder_state_class.as_str()
            ),
        ));
    }

    for (index, draft) in drafts.iter().enumerate() {
        if draft.command_id.is_empty()
            || draft.command_revision_ref.is_empty()
            || draft.canonical_verb.is_empty()
        {
            findings.push(FirstConsumersFinding::blocker(
                FirstConsumersFindingKind::StepMissingCommandIdentity,
                Some(format!("{entrypoint}:{}", draft.step_id)),
                format!(
                    "step {} on {entrypoint} is missing its command identity",
                    draft.step_id
                ),
            ));
        }
        let copy_cli = binding.copy_cli_lines.get(index);
        let open_docs = binding.open_docs_anchors.get(index);
        let parity = match (copy_cli, open_docs) {
            (Some(cli), Some(docs)) => step_parity_holds(&draft.canonical_verb, cli, docs),
            _ => false,
        };
        if !parity {
            findings.push(FirstConsumersFinding::blocker(
                FirstConsumersFindingKind::CliDocsParityBroken,
                Some(format!("{entrypoint}:{}", draft.step_id)),
                format!(
                    "step {} on {entrypoint} has copy-CLI / open-docs that do not cite {}",
                    draft.step_id, draft.canonical_verb
                ),
            ));
        }
    }
}

fn promotion_state_for_findings(
    findings: &[FirstConsumersFinding],
) -> AutomationBaselinePromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FirstConsumersFindingSeverity::Blocker)
    {
        AutomationBaselinePromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FirstConsumersFindingSeverity::Warning)
    {
        AutomationBaselinePromotionState::NarrowedBelowStable
    } else {
        AutomationBaselinePromotionState::Stable
    }
}

fn packet_digest(consumer_bindings: &[RecipeBuilderConsumerBinding]) -> String {
    let mut tokens: Vec<String> = Vec::new();
    for binding in consumer_bindings {
        tokens.push(binding.entrypoint.as_str().to_owned());
        for draft in &binding.session_record.step_drafts {
            tokens.push(draft.command_id.clone());
        }
    }
    tokens.sort_unstable();
    fnv1a64(&tokens)
}

/// Order-stable FNV-1a 64-bit digest of a sequence of strings.
fn fnv1a64(items_in_order: &[String]) -> String {
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
// Seeds
// ---------------------------------------------------------------------------

fn s(value: &str) -> String {
    value.to_owned()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn draft(
    step_id: &str,
    command_id: &str,
    command_revision_ref: &str,
    canonical_verb: &str,
    capability_declarations: &[&str],
    projected_safety_labels: &[AutomationSafetyLabelId],
) -> RecipeBuilderStepDraft {
    RecipeBuilderStepDraft {
        step_id: s(step_id),
        step_kind: s("invoke_descriptor_command"),
        command_id: s(command_id),
        command_revision_ref: s(command_revision_ref),
        canonical_verb: s(canonical_verb),
        capability_declarations: strings(capability_declarations),
        projected_safety_labels: projected_safety_labels.to_vec(),
    }
}

/// Existing contracts the first-consumers packet reuses instead of re-deciding.
pub fn canonical_reused_contract_refs() -> Vec<String> {
    strings(&[
        RECIPE_BUILDER_SCHEMA_REF,
        RECIPE_MANIFEST_SCHEMA_REF,
        "schemas/automation/automation-contract-baseline.schema.json",
        "schemas/automation/automation-manifest.schema.json",
        "schemas/commands/command_descriptor.schema.json",
        "schemas/commands/shareability_metadata.schema.json",
        "docs/m5/recipe-builder-and-macro-contract.md",
        "docs/automation/recipe_and_macro_contract.md",
    ])
}

/// Builds the seeded builder one first consumer authors.
pub fn seeded_consumer_builder(entrypoint: RecipeBuilderEntrypoint) -> RecipeBuilder {
    use AutomationSafetyLabelId::{
        ApprovalRequired, HeadlessSafe, NetworkCall, RecipeSafe, RunsProcess, WritesFiles,
    };
    match entrypoint {
        RecipeBuilderEntrypoint::Notebook => {
            let mut builder = RecipeBuilder::new(
                entrypoint,
                "builder:notebook:run-and-export:v1",
                "recipe-rev:notebook-run-and-export:1",
                "Run notebook and export results",
                "Runs every notebook cell, then exports the rendered results to HTML.",
                "sheet:notebook-run-and-export:v1",
                "dry-run:notebook-run-and-export:v1",
                "2026-06-18T00:00:00Z",
            );
            builder
                .append_step(
                    draft(
                        "step:run-all",
                        "command:notebook.run_all_cells",
                        "command-rev:notebook.run_all_cells:4",
                        "notebook.run_all_cells",
                        &["reversible_workspace_filesystem_mutation"],
                        &[RecipeSafe, HeadlessSafe, RunsProcess],
                    ),
                    Vec::new(),
                )
                .expect("append run-all");
            builder
                .append_step(
                    draft(
                        "step:export-html",
                        "command:notebook.export_html",
                        "command-rev:notebook.export_html:2",
                        "notebook.export_html",
                        &["reversible_workspace_filesystem_mutation"],
                        &[RecipeSafe, HeadlessSafe, WritesFiles],
                    ),
                    Vec::new(),
                )
                .expect("append export-html");
            builder
        }
        RecipeBuilderEntrypoint::TaskTestDebug => {
            let mut builder = RecipeBuilder::new(
                entrypoint,
                "builder:task:test-then-rerun-failed:v1",
                "recipe-rev:task-test-then-rerun-failed:1",
                "Run tests and rerun failures",
                "Runs the test task, then reruns only the failed tests for a quick loop.",
                "sheet:task-test-then-rerun-failed:v1",
                "dry-run:task-test-then-rerun-failed:v1",
                "2026-06-18T00:00:00Z",
            );
            builder
                .append_step(
                    draft(
                        "step:run-tests",
                        "command:task.run_tests",
                        "command-rev:task.run_tests:9",
                        "task.run_tests",
                        &["reversible_process_launch"],
                        &[RecipeSafe, HeadlessSafe, RunsProcess],
                    ),
                    Vec::new(),
                )
                .expect("append run-tests");
            builder
                .append_step(
                    draft(
                        "step:rerun-failed",
                        "command:task.rerun_failed_tests",
                        "command-rev:task.rerun_failed_tests:5",
                        "task.rerun_failed_tests",
                        &["reversible_process_launch"],
                        &[RecipeSafe, HeadlessSafe, RunsProcess],
                    ),
                    Vec::new(),
                )
                .expect("append rerun-failed");
            builder
        }
        RecipeBuilderEntrypoint::RequestApi => {
            let mut builder = RecipeBuilder::new(
                entrypoint,
                "builder:request:send-and-save:v1",
                "recipe-rev:request-send-and-save:1",
                "Send request and save response",
                "Sends the saved request, then stores the response body in the workspace.",
                "sheet:request-send-and-save:v1",
                "dry-run:request-send-and-save:v1",
                "2026-06-18T00:00:00Z",
            );
            builder
                .append_step(
                    draft(
                        "step:send-request",
                        "command:request.send_saved",
                        "command-rev:request.send_saved:6",
                        "request.send_saved",
                        &["outbound_network_request"],
                        &[RecipeSafe, HeadlessSafe, NetworkCall],
                    ),
                    strings(&["environment_profile"]),
                )
                .expect("append send-request");
            builder
                .append_step(
                    draft(
                        "step:save-response",
                        "command:request.save_response_body",
                        "command-rev:request.save_response_body:3",
                        "request.save_response_body",
                        &["reversible_workspace_filesystem_mutation"],
                        &[RecipeSafe, HeadlessSafe, WritesFiles],
                    ),
                    Vec::new(),
                )
                .expect("append save-response");
            builder
        }
        RecipeBuilderEntrypoint::Package => {
            let mut builder = RecipeBuilder::new(
                entrypoint,
                "builder:package:audit-then-update:v1",
                "recipe-rev:package-audit-then-update:1",
                "Audit and update dependencies",
                "Audits the dependency graph, then applies the safe updates under approval.",
                "sheet:package-audit-then-update:v1",
                "dry-run:package-audit-then-update:v1",
                "2026-06-18T00:00:00Z",
            );
            builder
                .append_step(
                    draft(
                        "step:audit",
                        "command:package.audit_dependencies",
                        "command-rev:package.audit_dependencies:7",
                        "package.audit_dependencies",
                        &["read_only_dependency_scan"],
                        &[RecipeSafe, HeadlessSafe],
                    ),
                    Vec::new(),
                )
                .expect("append audit");
            builder
                .append_step(
                    draft(
                        "step:apply-updates",
                        "command:package.apply_safe_updates",
                        "command-rev:package.apply_safe_updates:4",
                        "package.apply_safe_updates",
                        &["reversible_workspace_filesystem_mutation"],
                        &[RecipeSafe, ApprovalRequired, WritesFiles],
                    ),
                    Vec::new(),
                )
                .expect("append apply-updates");
            builder
        }
        RecipeBuilderEntrypoint::Incident => {
            let mut builder = RecipeBuilder::new(
                entrypoint,
                "builder:incident:capture-evidence:v1",
                "recipe-rev:incident-capture-evidence:1",
                "Capture incident evidence bundle",
                "Snapshots the incident timeline, then exports a redacted support bundle.",
                "sheet:incident-capture-evidence:v1",
                "dry-run:incident-capture-evidence:v1",
                "2026-06-18T00:00:00Z",
            );
            builder
                .append_step(
                    draft(
                        "step:snapshot-timeline",
                        "command:incident.snapshot_timeline",
                        "command-rev:incident.snapshot_timeline:3",
                        "incident.snapshot_timeline",
                        &["read_only_timeline_capture"],
                        &[RecipeSafe, HeadlessSafe],
                    ),
                    Vec::new(),
                )
                .expect("append snapshot-timeline");
            builder
                .append_step(
                    draft(
                        "step:export-bundle",
                        "command:support.export_incident_bundle",
                        "command-rev:support.export_incident_bundle:5",
                        "support.export_incident_bundle",
                        &["reversible_workspace_filesystem_mutation"],
                        &[RecipeSafe, HeadlessSafe, WritesFiles],
                    ),
                    Vec::new(),
                )
                .expect("append export-bundle");
            builder
        }
        RecipeBuilderEntrypoint::AiAssistant => {
            let mut builder = RecipeBuilder::new(
                entrypoint,
                "builder:ai:apply-proposed-fix:v1",
                "recipe-rev:ai-apply-proposed-fix:1",
                "Apply AI-proposed fix under review",
                "Applies the AI-proposed edit under approval, then runs the test task to confirm.",
                "sheet:ai-apply-proposed-fix:v1",
                "dry-run:ai-apply-proposed-fix:v1",
                "2026-06-18T00:00:00Z",
            );
            builder
                .append_step(
                    draft(
                        "step:apply-edit",
                        "command:ai.apply_proposed_edit",
                        "command-rev:ai.apply_proposed_edit:8",
                        "ai.apply_proposed_edit",
                        &["reversible_workspace_filesystem_mutation"],
                        &[RecipeSafe, ApprovalRequired, WritesFiles],
                    ),
                    strings(&["proposal_id"]),
                )
                .expect("append apply-edit");
            builder
                .append_step(
                    draft(
                        "step:confirm-tests",
                        "command:task.run_tests",
                        "command-rev:task.run_tests:9",
                        "task.run_tests",
                        &["reversible_process_launch"],
                        &[RecipeSafe, HeadlessSafe, RunsProcess],
                    ),
                    Vec::new(),
                )
                .expect("append confirm-tests");
            builder
        }
    }
}

/// Builds the canonical stable first-consumers input.
pub fn current_recipe_builder_first_consumers_input() -> RecipeBuilderFirstConsumersInput {
    let consumer_bindings = RecipeBuilderEntrypoint::ALL
        .into_iter()
        .map(|entrypoint| {
            RecipeBuilderConsumerBinding::from_builder(&seeded_consumer_builder(entrypoint))
        })
        .collect();
    RecipeBuilderFirstConsumersInput {
        packet_id: RECIPE_BUILDER_FIRST_CONSUMERS_ID.to_owned(),
        generated_at: "2026-06-18T00:00:00Z".to_owned(),
        consumer_bindings,
        reused_contract_refs: canonical_reused_contract_refs(),
        invariants: RecipeBuilderInvariantsBlock::frozen(),
    }
}

/// Materializes the canonical stable first-consumers packet.
pub fn seeded_recipe_builder_first_consumers_packet() -> RecipeBuilderFirstConsumersPacket {
    RecipeBuilderFirstConsumersPacket::materialize(current_recipe_builder_first_consumers_input())
}

/// Validates a packet, returning `Ok(())` or the findings.
pub fn validate_recipe_builder_first_consumers_packet(
    packet: &RecipeBuilderFirstConsumersPacket,
) -> Result<(), Vec<FirstConsumersFinding>> {
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Worked example: the notebook builder exported for round-trip review.
pub fn seeded_recipe_builder_export_roundtrip() -> RecipeBuilderExport {
    seeded_consumer_builder(RecipeBuilderEntrypoint::Notebook)
        .export("export:notebook-run-and-export:v1", "2026-06-18T00:01:00Z")
}

/// Worked example: a blocked builder session that cites a UI-only command.
///
/// The builder refuses to emit a declarative recipe; the blocked step stays
/// visible with a blocker finding.
pub fn seeded_blocked_recipe_builder() -> RecipeBuilder {
    use AutomationSafetyLabelId::{RecipeSafe, RunsProcess, UiOnly};
    let mut builder = RecipeBuilder::new(
        RecipeBuilderEntrypoint::TaskTestDebug,
        "builder:task:debug-attach-blocked:v1",
        "recipe-rev:task-debug-attach-blocked:1",
        "Run tests then attach interactive debugger",
        "Attempts to add an interactive-only debugger-attach command; the builder blocks it.",
        "sheet:task-debug-attach-blocked:v1",
        "dry-run:task-debug-attach-blocked:v1",
        "2026-06-18T00:00:00Z",
    );
    builder
        .append_step(
            draft(
                "step:run-tests",
                "command:task.run_tests",
                "command-rev:task.run_tests:9",
                "task.run_tests",
                &["reversible_process_launch"],
                &[RecipeSafe, RunsProcess],
            ),
            Vec::new(),
        )
        .expect("append run-tests");
    builder
        .append_step(
            draft(
                "step:attach-debugger",
                "command:debug.attach_interactive",
                "command-rev:debug.attach_interactive:2",
                "debug.attach_interactive",
                &["reversible_local_editor_mutation"],
                &[UiOnly],
            ),
            Vec::new(),
        )
        .expect("append attach-debugger");
    builder
}
