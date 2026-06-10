//! Recorded-macro promotion, recipe insertion, and headless-safe result packets
//! for user automation.
//!
//! This module carries user-authored automation through its full lifecycle in
//! one export-safe truth packet whose unit of truth is a [`UserAutomationRow`]: a
//! single recorded macro bound to how it was captured, the promotion that
//! graduates it into a reusable declarative recipe, how that recipe inserts into
//! a target surface, and the headless-safe result the recipe produces when it
//! runs in a non-interactive context. Each row carries a step disclosure for
//! every effect the automation can produce — the side-effect class, how the
//! effect previews before it applies interactively, how it behaves when run
//! headless, the approval gate it carries, how it is audited, and whether it is
//! reversible. The packet is the canonical user-automation source for shell,
//! docs, support export, and release tooling; consumers project it instead of
//! re-deriving promotion, insertion, or headless posture by hand.
//!
//! The packet refuses to present automation greener than its disclosure posture
//! can back. It builds directly on the signed/shared recipe-pack lane and the
//! recorded-macro / declarative-recipe contract: a recorded macro is promoted to
//! a recipe only behind an explicit, audited promotion gate — there is no silent
//! forward from a macro to a recipe — and an imported capture must name the
//! signed recipe pack it rode. Every capture carries a content-addressed digest
//! so a replay can prove the exact bytes it rode. A mutating step is held to the
//! same preview, policy, and audit bar as a first-party command: every mutating
//! step previews before it applies interactively (preview-first), carries a real
//! approval gate, and is audited. Insertion is preview-first too: a mutating
//! recipe previews before it inserts, and an insertion into a headless target
//! cannot rely on an interactive prompt that can never appear.
//!
//! Headless safety is the load-bearing guarantee. Every step discloses how it
//! behaves with no interactive operator present: an inspect-only step is always
//! headless-safe, a mutating step runs headless only under an explicit
//! pre-authorized policy grant that is itself gated and audited, and any step
//! that needs interactive confirmation is deferred for later interactive review,
//! blocked fail-closed, or denied — it never silently executes. An irreversible
//! external publish can never run unattended headless. The headless result block
//! is content-addressed, reconciles its step counts against the disclosed steps,
//! and its result state must agree with whether any step deferred or blocked.
//!
//! A blocked promotion — policy-blocked, tainted-capture-blocked, or withdrawn —
//! narrows its claim instead of staying behind a Stable, Beta, or Preview label,
//! and a macro still awaiting promotion review may not claim Stable. Every row
//! carries a closed set of downgrade rules — including the proof-stale and
//! provider-unavailable triggers — that narrow the claim instead of hiding the
//! automation, reusing the qualification, downgrade-trigger, and rollback-posture
//! vocabularies frozen by the M5 AI workflow matrix lane, the provider/locality
//! mode vocabulary frozen by the routing-policy lane, the side-effect and
//! approval vocabularies frozen by the tool-gateway baseline, and the
//! replay-preview, audit, and reversibility vocabularies frozen by the
//! signed/shared recipe-pack lane, so no automation row may stay greener than its
//! evidence.
//!
//! Raw shell fragments, raw filesystem paths, raw endpoint URLs, credential
//! bodies, raw API keys, OAuth tokens, and raw captured UI or editor buffer bytes
//! stay outside the support boundary; the packet carries content addresses,
//! classes, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/add-recorded-macro-promotion-recipe-insertion-and-headless-safe-result-packets-for-user-automation.schema.json`](../../../../schemas/ai/add-recorded-macro-promotion-recipe-insertion-and-headless-safe-result-packets-for-user-automation.schema.json).
//! The contract doc is
//! [`docs/automation/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation.md`](../../../../docs/automation/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/`](../../../../fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay::{
    RecipeStepAuditClass, RecipeStepReversibilityClass, ReplayPreviewClass, RECIPE_PACK_SCHEMA_REF,
};
use crate::tool_gateway::{
    ToolApprovalPostureClass, ToolPublisherSourceClass, ToolSideEffectClass,
    TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`UserAutomationPacket`].
pub const USER_AUTOMATION_RECORD_KIND: &str =
    "add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation";

/// Schema version for user-automation records.
pub const USER_AUTOMATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const USER_AUTOMATION_SCHEMA_REF: &str =
    "schemas/ai/add-recorded-macro-promotion-recipe-insertion-and-headless-safe-result-packets-for-user-automation.schema.json";

/// Repo-relative path of the user-automation contract doc.
pub const USER_AUTOMATION_DOC_REF: &str =
    "docs/automation/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation.md";

/// Repo-relative path of the recorded-macro / declarative-recipe contract this
/// lane projects from.
pub const USER_AUTOMATION_RECIPE_MACRO_CONTRACT_REF: &str =
    "docs/automation/recipe_and_macro_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const USER_AUTOMATION_FIXTURE_DIR: &str =
    "fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation";

/// Repo-relative path of the checked support-export artifact.
pub const USER_AUTOMATION_ARTIFACT_REF: &str =
    "artifacts/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const USER_AUTOMATION_SUMMARY_REF: &str =
    "artifacts/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation.md";

/// How a recorded macro was captured.
///
/// A capture imported from a shared recipe pack rides the signing posture the
/// signed/shared recipe-pack lane verified, so it must name that pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroCaptureProvenanceClass {
    /// Recorded directly from a live user session.
    RecordedFromUserSession,
    /// Recorded while replaying an earlier session.
    RecordedFromReplaySession,
    /// Imported from a signed, shared recipe pack.
    ImportedFromSharedRecipePack,
    /// Synthesized from a first-party template.
    SynthesizedFromTemplate,
}

impl MacroCaptureProvenanceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordedFromUserSession => "recorded_from_user_session",
            Self::RecordedFromReplaySession => "recorded_from_replay_session",
            Self::ImportedFromSharedRecipePack => "imported_from_shared_recipe_pack",
            Self::SynthesizedFromTemplate => "synthesized_from_template",
        }
    }

    /// Whether this provenance must name the signed recipe pack it rode.
    pub const fn requires_recipe_pack_ref(self) -> bool {
        matches!(self, Self::ImportedFromSharedRecipePack)
    }
}

/// Promotion state of a recorded macro on its way to a reusable recipe.
///
/// A recorded macro graduates into a declarative recipe only through an explicit,
/// audited promotion gate; there is no silent forward from a macro to a recipe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroPromotionStateClass {
    /// Recorded and awaiting promotion review.
    RecordedPendingReview,
    /// Reviewed but deliberately held without promotion.
    ReviewedHeldNotPromoted,
    /// Promoted into a reusable declarative recipe.
    PromotedToRecipe,
    /// Promotion blocked by policy.
    PromotionBlockedPolicy,
    /// Promotion blocked because the capture carries tainted material.
    PromotionBlockedTaintedCapture,
    /// Promotion withdrawn after the fact.
    PromotionWithdrawn,
}

impl MacroPromotionStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordedPendingReview => "recorded_pending_review",
            Self::ReviewedHeldNotPromoted => "reviewed_held_not_promoted",
            Self::PromotedToRecipe => "promoted_to_recipe",
            Self::PromotionBlockedPolicy => "promotion_blocked_policy",
            Self::PromotionBlockedTaintedCapture => "promotion_blocked_tainted_capture",
            Self::PromotionWithdrawn => "promotion_withdrawn",
        }
    }

    /// Whether the macro has been promoted into a recipe.
    pub const fn is_promoted(self) -> bool {
        matches!(self, Self::PromotedToRecipe)
    }

    /// Whether downstream surfaces must display a typed block reason.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::PromotionBlockedPolicy
                | Self::PromotionBlockedTaintedCapture
                | Self::PromotionWithdrawn
        )
    }
}

/// Where a promoted recipe is inserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeInsertionTargetClass {
    /// Inserted into the prompt composer as a draft step list.
    ComposerPromptInsertion,
    /// Inserted into a workspace document.
    WorkspaceDocumentInsertion,
    /// Inserted as a command-palette entry.
    CommandPaletteInsertion,
    /// Inserted into the non-interactive automation queue.
    AutomationQueueInsertion,
    /// Inserted into a headless job that runs with no operator present.
    HeadlessJobInsertion,
}

impl RecipeInsertionTargetClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerPromptInsertion => "composer_prompt_insertion",
            Self::WorkspaceDocumentInsertion => "workspace_document_insertion",
            Self::CommandPaletteInsertion => "command_palette_insertion",
            Self::AutomationQueueInsertion => "automation_queue_insertion",
            Self::HeadlessJobInsertion => "headless_job_insertion",
        }
    }

    /// Whether this target runs with no interactive operator present.
    pub const fn is_headless_target(self) -> bool {
        matches!(
            self,
            Self::AutomationQueueInsertion | Self::HeadlessJobInsertion
        )
    }
}

/// How a step behaves when the automation runs with no interactive operator.
///
/// This is the headless-safety axis the shell, automation queue, and support
/// surfaces read. A mutating step never silently executes headless: it runs only
/// under a pre-authorized policy grant, or it defers, blocks, or is denied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadlessSafetyClass {
    /// Inspect-only; always safe to run headless.
    HeadlessSafeInspectOnly,
    /// Mutating but pre-authorized by an explicit, gated policy grant.
    HeadlessSafePreauthorizedPolicy,
    /// Deferred for later interactive review; not executed headless.
    HeadlessDeferredToInteractive,
    /// Blocked fail-closed because it needs interactive confirmation.
    HeadlessBlockedFailClosed,
    /// Denied outright when run headless.
    HeadlessDeniedByPolicy,
}

impl HeadlessSafetyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeadlessSafeInspectOnly => "headless_safe_inspect_only",
            Self::HeadlessSafePreauthorizedPolicy => "headless_safe_preauthorized_policy",
            Self::HeadlessDeferredToInteractive => "headless_deferred_to_interactive",
            Self::HeadlessBlockedFailClosed => "headless_blocked_fail_closed",
            Self::HeadlessDeniedByPolicy => "headless_denied_by_policy",
        }
    }

    /// Whether the step actually executes when run headless.
    pub const fn runs_headless(self) -> bool {
        matches!(
            self,
            Self::HeadlessSafeInspectOnly | Self::HeadlessSafePreauthorizedPolicy
        )
    }

    /// Whether this class permits a mutating step to execute headless.
    pub const fn permits_mutation_headless(self) -> bool {
        matches!(self, Self::HeadlessSafePreauthorizedPolicy)
    }

    /// Whether the step is deferred for later interactive review.
    pub const fn is_deferred(self) -> bool {
        matches!(self, Self::HeadlessDeferredToInteractive)
    }

    /// Whether the step is held back fail-closed or denied when run headless.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::HeadlessBlockedFailClosed | Self::HeadlessDeniedByPolicy
        )
    }
}

/// Outcome of a headless automation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadlessResultStateClass {
    /// Every step ran safely with no deferral or block.
    CompletedAllStepsSafe,
    /// Completed, but at least one step deferred to interactive review.
    CompletedWithDeferredSteps,
    /// Halted fail-closed because a step needed interactive confirmation.
    BlockedFailClosed,
    /// Denied by policy before any mutating effect.
    DeniedByPolicy,
    /// Ran some steps, then halted on a blocked step.
    PartialThenHalted,
}

impl HeadlessResultStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompletedAllStepsSafe => "completed_all_steps_safe",
            Self::CompletedWithDeferredSteps => "completed_with_deferred_steps",
            Self::BlockedFailClosed => "blocked_fail_closed",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::PartialThenHalted => "partial_then_halted",
        }
    }

    /// Whether the run reached a completed state.
    pub const fn is_complete(self) -> bool {
        matches!(
            self,
            Self::CompletedAllStepsSafe | Self::CompletedWithDeferredSteps
        )
    }

    /// Whether this state requires at least one blocked step.
    pub const fn requires_blocked_step(self) -> bool {
        matches!(
            self,
            Self::BlockedFailClosed | Self::DeniedByPolicy | Self::PartialThenHalted
        )
    }
}

/// One disclosed effect an automation's steps can produce.
///
/// Each disclosure binds the side-effect class to how it previews before it
/// applies interactively, how it behaves headless, the gate it carries, how it is
/// audited, and how it reverses, so neither an interactive replay nor a headless
/// run can produce an effect the automation did not disclose under first-party
/// command rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationStepDisclosure {
    /// Side-effect class this disclosure covers.
    pub side_effect_class: ToolSideEffectClass,
    /// How the step behaves when run with no interactive operator present.
    pub headless_safety: HeadlessSafetyClass,
    /// How the effect previews before it applies in an interactive run.
    pub interactive_preview: ReplayPreviewClass,
    /// Approval gate required before the effect applies.
    pub approval_posture: ToolApprovalPostureClass,
    /// How the effect is audited.
    pub audit: RecipeStepAuditClass,
    /// Reversibility of the effect.
    pub reversibility: RecipeStepReversibilityClass,
    /// Review-safe disclosure label shown before the effect applies.
    pub disclosure_label: String,
}

impl AutomationStepDisclosure {
    /// Whether this disclosure covers a mutating effect held to the first-party
    /// command preview, policy, and audit bar.
    pub fn is_mutating(&self) -> bool {
        self.side_effect_class.requires_approval_gate()
    }

    /// Whether this disclosure carries a real approval gate.
    pub fn has_approval_gate(&self) -> bool {
        self.approval_posture.requires_approval_gate() || self.approval_posture.denies_dispatch()
    }

    /// Whether this mutating step actually executes when run headless.
    pub fn runs_unattended_headless(&self) -> bool {
        self.is_mutating() && self.headless_safety.permits_mutation_headless()
    }
}

/// Promotion block binding a recorded macro to its graduation into a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacroPromotionBlock {
    /// Promotion state of the macro.
    pub state: MacroPromotionStateClass,
    /// Content address of the recipe the macro promoted into; non-empty exactly
    /// when the macro is promoted.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub promoted_recipe_ref: String,
    /// Opaque ref into the signed/shared recipe-pack lane, when the promotion
    /// rode a shared pack.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub recipe_pack_ref: String,
    /// Approval gate required to promote the macro.
    pub promotion_approval: ToolApprovalPostureClass,
    /// Opaque ref to the reviewer identity that approved the promotion.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reviewer_identity_ref: String,
    /// Whether first-use review is required before any material replay.
    pub first_use_review_required: bool,
}

impl MacroPromotionBlock {
    /// Whether the promotion carries a real gate.
    pub fn has_promotion_gate(&self) -> bool {
        self.promotion_approval.requires_approval_gate()
            || self.promotion_approval.denies_dispatch()
    }
}

/// Insertion block binding a promoted recipe to how it inserts into a target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeInsertionBlock {
    /// Where the recipe is inserted.
    pub target_class: RecipeInsertionTargetClass,
    /// How the insertion previews before it commits.
    pub preview: ReplayPreviewClass,
    /// Approval gate required before the insertion commits.
    pub approval: ToolApprovalPostureClass,
    /// Whether the insertion can be reversed.
    pub insertion_reversible: bool,
    /// Review-safe label shown before the insertion commits.
    pub insertion_label: String,
}

impl RecipeInsertionBlock {
    /// Whether the insertion carries a real approval gate.
    pub fn has_approval_gate(&self) -> bool {
        self.approval.requires_approval_gate() || self.approval.denies_dispatch()
    }

    /// Whether the insertion approval can be honored with no interactive
    /// operator present.
    ///
    /// A recurring per-session or per-invocation interactive prompt can never
    /// appear in a headless context, so it is not a valid gate for a
    /// headless-target insertion. A one-time setup prompt, an admin ticket, an
    /// out-of-band allow, or a policy denial are all honorable headless.
    pub fn approval_is_non_interactive(&self) -> bool {
        !matches!(
            self.approval,
            ToolApprovalPostureClass::AllowedWithPerInvocationPrompt
                | ToolApprovalPostureClass::AllowedWithPerSessionPrompt
        )
    }
}

/// Headless-safe result block produced by running an automation unattended.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadlessResultBlock {
    /// Outcome of the headless run.
    pub state: HeadlessResultStateClass,
    /// Content address of the result packet bytes, proving the exact result a
    /// headless run produced.
    pub result_content_address: String,
    /// Total step count the run covered.
    pub steps_total: u32,
    /// Steps that executed safely.
    pub steps_completed: u32,
    /// Steps deferred to later interactive review.
    pub steps_deferred: u32,
    /// Steps blocked fail-closed or denied.
    pub steps_blocked: u32,
    /// How the result is audited.
    pub audit: RecipeStepAuditClass,
    /// Review-safe label describing the result.
    pub result_label: String,
}

/// One downgrade rule that narrows an automation's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAutomationDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the automation narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One user-automation row binding capture, promotion, insertion, and a
/// headless-safe result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAutomationRow {
    /// Stable macro id.
    pub macro_id: String,
    /// Human-readable macro label.
    pub macro_label: String,
    /// Macro family label.
    pub macro_family_label: String,
    /// Macro version.
    pub macro_version: String,
    /// Content address of the captured macro bytes.
    pub capture_content_address: String,
    /// How the macro was captured.
    pub capture_provenance: MacroCaptureProvenanceClass,
    /// Count of recorded steps in the capture.
    pub recorded_step_count: u32,
    /// Source/publisher class.
    pub publisher_source_class: ToolPublisherSourceClass,
    /// Opaque ref to the signed publisher identity record.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub publisher_identity_ref: String,
    /// Provider/locality mode the automation resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Promotion block.
    pub promotion: MacroPromotionBlock,
    /// Insertion block.
    pub insertion: RecipeInsertionBlock,
    /// Headless-safe result block.
    pub headless_result: HeadlessResultBlock,
    /// Disclosed effects the automation's steps can produce.
    pub steps: Vec<AutomationStepDisclosure>,
    /// Qualification class claimed for this automation.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<UserAutomationDowngradeRule>,
    /// Rollback posture for an automation-policy change.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
    /// Review-safe explanation of the automation posture.
    pub explanation_label: String,
}

impl UserAutomationRow {
    /// Whether this automation carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Whether the automation discloses any mutating effect.
    pub fn has_mutating_step(&self) -> bool {
        self.steps.iter().any(AutomationStepDisclosure::is_mutating)
    }

    /// Whether the automation discloses an irreversible external publish.
    pub fn has_irreversible_publish(&self) -> bool {
        self.steps
            .iter()
            .any(|step| step.side_effect_class == ToolSideEffectClass::ExternalIrreversiblePublish)
    }

    /// Whether every mutating step previews before it applies interactively.
    ///
    /// This is the preview-first guarantee consumers project rather than
    /// re-deriving per surface.
    pub fn is_preview_first(&self) -> bool {
        self.steps
            .iter()
            .filter(|step| step.is_mutating())
            .all(|step| step.interactive_preview.previews_before_replay())
    }

    /// Whether every mutating step is headless-safe.
    ///
    /// A mutating step is headless-safe when it either does not execute headless
    /// (it defers, blocks, or is denied) or executes only under a pre-authorized
    /// policy grant that carries a real gate — and no irreversible external
    /// publish ever runs unattended headless. This is the load-bearing guarantee
    /// consumers project rather than re-deriving per surface.
    pub fn is_headless_safe(&self) -> bool {
        self.steps
            .iter()
            .filter(|step| step.is_mutating())
            .all(|step| {
                if step.headless_safety.permits_mutation_headless() {
                    step.has_approval_gate()
                        && step.side_effect_class
                            != ToolSideEffectClass::ExternalIrreversiblePublish
                } else {
                    step.headless_safety.is_deferred() || step.headless_safety.is_blocked()
                }
            })
    }

    /// The disclosure for `side_effect_class`, if present.
    pub fn step(
        &self,
        side_effect_class: ToolSideEffectClass,
    ) -> Option<&AutomationStepDisclosure> {
        self.steps
            .iter()
            .find(|step| step.side_effect_class == side_effect_class)
    }

    /// Qualification this automation narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this automation.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Automation `{}`\n", self.macro_id));
        out.push_str(&format!("- Macro: `{}`\n", self.macro_label));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!(
            "- Capture: `{}` / provenance `{}`\n",
            self.capture_content_address,
            self.capture_provenance.as_str()
        ));
        out.push_str(&format!(
            "- Promotion: `{}` / mode `{}`\n",
            self.promotion.state.as_str(),
            self.resolved_mode.as_str()
        ));
        out.push_str(&format!(
            "- Insertion: `{}` / preview `{}`\n",
            self.insertion.target_class.as_str(),
            self.insertion.preview.as_str()
        ));
        out.push_str(&format!(
            "- Headless result: `{}` ({} done, {} deferred, {} blocked of {})\n",
            self.headless_result.state.as_str(),
            self.headless_result.steps_completed,
            self.headless_result.steps_deferred,
            self.headless_result.steps_blocked,
            self.headless_result.steps_total
        ));
        out.push_str("- Step disclosures:\n");
        for step in &self.steps {
            out.push_str(&format!(
                "  - `{}` / headless `{}` / preview `{}` / approval `{}` / audit `{}` / reversibility `{}` ({})\n",
                step.side_effect_class.as_str(),
                step.headless_safety.as_str(),
                step.interactive_preview.as_str(),
                step.approval_posture.as_str(),
                step.audit.as_str(),
                step.reversibility.as_str(),
                step.disclosure_label
            ));
        }
        out
    }
}

/// Proof freshness block for the user-automation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAutomationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed automations.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`UserAutomationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAutomationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// User-automation rows.
    pub automations: Vec<UserAutomationRow>,
    /// Proof freshness block.
    pub proof_freshness: UserAutomationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe recorded-macro / recipe-insertion / headless-result packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAutomationPacket {
    /// Record kind; must equal [`USER_AUTOMATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`USER_AUTOMATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// User-automation rows.
    pub automations: Vec<UserAutomationRow>,
    /// Proof freshness block.
    pub proof_freshness: UserAutomationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl UserAutomationPacket {
    /// Builds a user-automation packet from stable-lane input.
    pub fn new(input: UserAutomationPacketInput) -> Self {
        Self {
            record_kind: USER_AUTOMATION_RECORD_KIND.to_owned(),
            schema_version: USER_AUTOMATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            automations: input.automations,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the user-automation invariants.
    pub fn validate(&self) -> Vec<UserAutomationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != USER_AUTOMATION_RECORD_KIND {
            violations.push(UserAutomationViolation::WrongRecordKind);
        }
        if self.schema_version != USER_AUTOMATION_SCHEMA_VERSION {
            violations.push(UserAutomationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(UserAutomationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_automations_present(self, &mut violations);
        for automation in &self.automations {
            validate_automation(automation, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_automation_material(
            &serde_json::to_value(self).expect("user automation packet serializes"),
        ) {
            violations.push(UserAutomationViolation::RawAutomationMaterialInExport);
        }

        violations
    }

    /// Count of automations carrying a publicly claimed qualification.
    pub fn claimed_automation_count(&self) -> usize {
        self.automations.iter().filter(|a| a.is_claimed()).count()
    }

    /// Count of automations with a blocked promotion.
    pub fn blocked_automation_count(&self) -> usize {
        self.automations
            .iter()
            .filter(|a| a.promotion.state.is_blocked())
            .count()
    }

    /// Count of automations promoted into a recipe.
    pub fn promoted_automation_count(&self) -> usize {
        self.automations
            .iter()
            .filter(|a| a.promotion.state.is_promoted())
            .count()
    }

    /// Count of automations disclosing a mutating effect.
    pub fn mutating_automation_count(&self) -> usize {
        self.automations
            .iter()
            .filter(|a| a.has_mutating_step())
            .count()
    }

    /// Returns the automation row for `macro_id`, if present.
    pub fn automation(&self, macro_id: &str) -> Option<&UserAutomationRow> {
        self.automations.iter().find(|a| a.macro_id == macro_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("user automation packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Recorded-Macro Promotion, Recipe Insertion, And Headless-Safe Result Packets\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Automations: {} ({} claimed, {} promoted, {} mutating, {} blocked)\n",
            self.automations.len(),
            self.claimed_automation_count(),
            self.promoted_automation_count(),
            self.mutating_automation_count(),
            self.blocked_automation_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Automation inspectors\n\n");
        for automation in &self.automations {
            out.push_str(&automation.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in user-automation export.
#[derive(Debug)]
pub enum UserAutomationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<UserAutomationViolation>),
}

impl fmt::Display for UserAutomationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "user automation export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "user automation export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for UserAutomationArtifactError {}

/// Validation failures emitted by [`UserAutomationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserAutomationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no automations.
    NoAutomations,
    /// A macro id appears more than once.
    DuplicateAutomation,
    /// An automation row is missing a required identity or label field.
    AutomationRowIncomplete,
    /// An automation is missing its capture content address.
    AutomationMissingCaptureAddress,
    /// A publisher class that requires identity is missing a publisher identity.
    AutomationMissingPublisherIdentity,
    /// An imported capture does not name the signed recipe pack it rode.
    ImportedCaptureMissingRecipePackRef,
    /// An automation discloses no steps.
    AutomationMissingStepDisclosures,
    /// A step disclosure is missing its disclosure label.
    StepDisclosureIncomplete,
    /// A side-effect class is disclosed more than once.
    DuplicateStepDisclosure,
    /// A mutating step does not preview before it applies interactively.
    MutatingStepWithoutPreviewFirst,
    /// A mutating step carries no approval gate.
    MutatingStepWithoutApproval,
    /// A mutating step is not audited.
    MutatingStepWithoutAudit,
    /// A disclosure's reversibility disagrees with its side-effect class.
    StepReversibilityMismatch,
    /// An inspect-only step is not marked headless-safe inspect-only.
    InspectStepNotHeadlessInspectSafe,
    /// A mutating step claims the inspect-only headless-safety class.
    MutatingStepClaimsInspectHeadlessSafety,
    /// An irreversible external publish runs unattended headless.
    IrreversiblePublishRunsHeadless,
    /// A step pre-authorized to run headless carries no approval gate.
    HeadlessPreauthorizedStepWithoutApproval,
    /// A step pre-authorized to run headless is not audited.
    HeadlessPreauthorizedStepWithoutAudit,
    /// A promoted macro is missing its promoted recipe ref.
    PromotedMissingRecipeRef,
    /// A macro that is not promoted carries a promoted recipe ref.
    UnpromotedHasRecipeRef,
    /// A promoted mutating macro carries no promotion approval gate.
    MutatingPromotionWithoutGate,
    /// A blocked promotion still claims a public qualification.
    BlockedPromotionClaimsQualification,
    /// A macro pending promotion review claims Stable.
    PendingPromotionClaimsStable,
    /// An insertion is missing its insertion label.
    InsertionMissingLabel,
    /// A mutating automation's insertion does not preview before it commits.
    MutatingInsertionWithoutPreviewFirst,
    /// A headless-target insertion relies on an interactive approval prompt.
    HeadlessTargetInsertionRequiresInteractiveApproval,
    /// A headless result block is missing its content address or label.
    HeadlessResultIncomplete,
    /// A headless result block's step counts do not reconcile.
    HeadlessResultCountMismatch,
    /// A headless result state disagrees with its deferred/blocked counts.
    HeadlessResultStateMismatch,
    /// A headless run that executed a mutating step is not externally audited.
    HeadlessRunMutatingNotExternallyAudited,
    /// A claimed automation is missing required evidence packet refs.
    ClaimedAutomationMissingEvidence,
    /// A claimed automation's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// An automation has no downgrade rules.
    DowngradeRulesMissing,
    /// An automation's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// An automation's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw automation material.
    RawAutomationMaterialInExport,
}

impl UserAutomationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoAutomations => "no_automations",
            Self::DuplicateAutomation => "duplicate_automation",
            Self::AutomationRowIncomplete => "automation_row_incomplete",
            Self::AutomationMissingCaptureAddress => "automation_missing_capture_address",
            Self::AutomationMissingPublisherIdentity => "automation_missing_publisher_identity",
            Self::ImportedCaptureMissingRecipePackRef => "imported_capture_missing_recipe_pack_ref",
            Self::AutomationMissingStepDisclosures => "automation_missing_step_disclosures",
            Self::StepDisclosureIncomplete => "step_disclosure_incomplete",
            Self::DuplicateStepDisclosure => "duplicate_step_disclosure",
            Self::MutatingStepWithoutPreviewFirst => "mutating_step_without_preview_first",
            Self::MutatingStepWithoutApproval => "mutating_step_without_approval",
            Self::MutatingStepWithoutAudit => "mutating_step_without_audit",
            Self::StepReversibilityMismatch => "step_reversibility_mismatch",
            Self::InspectStepNotHeadlessInspectSafe => "inspect_step_not_headless_inspect_safe",
            Self::MutatingStepClaimsInspectHeadlessSafety => {
                "mutating_step_claims_inspect_headless_safety"
            }
            Self::IrreversiblePublishRunsHeadless => "irreversible_publish_runs_headless",
            Self::HeadlessPreauthorizedStepWithoutApproval => {
                "headless_preauthorized_step_without_approval"
            }
            Self::HeadlessPreauthorizedStepWithoutAudit => {
                "headless_preauthorized_step_without_audit"
            }
            Self::PromotedMissingRecipeRef => "promoted_missing_recipe_ref",
            Self::UnpromotedHasRecipeRef => "unpromoted_has_recipe_ref",
            Self::MutatingPromotionWithoutGate => "mutating_promotion_without_gate",
            Self::BlockedPromotionClaimsQualification => "blocked_promotion_claims_qualification",
            Self::PendingPromotionClaimsStable => "pending_promotion_claims_stable",
            Self::InsertionMissingLabel => "insertion_missing_label",
            Self::MutatingInsertionWithoutPreviewFirst => {
                "mutating_insertion_without_preview_first"
            }
            Self::HeadlessTargetInsertionRequiresInteractiveApproval => {
                "headless_target_insertion_requires_interactive_approval"
            }
            Self::HeadlessResultIncomplete => "headless_result_incomplete",
            Self::HeadlessResultCountMismatch => "headless_result_count_mismatch",
            Self::HeadlessResultStateMismatch => "headless_result_state_mismatch",
            Self::HeadlessRunMutatingNotExternallyAudited => {
                "headless_run_mutating_not_externally_audited"
            }
            Self::ClaimedAutomationMissingEvidence => "claimed_automation_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingProviderUnavailable => {
                "downgrade_rule_missing_provider_unavailable"
            }
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawAutomationMaterialInExport => "raw_automation_material_in_export",
        }
    }
}

/// Reads and validates the checked-in user-automation export.
pub fn current_user_automation_export() -> Result<UserAutomationPacket, UserAutomationArtifactError>
{
    let packet: UserAutomationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/support_export.json"
    )))
    .map_err(UserAutomationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(UserAutomationArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

fn validate_source_contracts(
    packet: &UserAutomationPacket,
    violations: &mut Vec<UserAutomationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        USER_AUTOMATION_SCHEMA_REF,
        USER_AUTOMATION_DOC_REF,
        USER_AUTOMATION_RECIPE_MACRO_CONTRACT_REF,
        RECIPE_PACK_SCHEMA_REF,
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        ROUTING_POLICY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(UserAutomationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_automations_present(
    packet: &UserAutomationPacket,
    violations: &mut Vec<UserAutomationViolation>,
) {
    if packet.automations.is_empty() {
        violations.push(UserAutomationViolation::NoAutomations);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for automation in &packet.automations {
        if !seen.insert(automation.macro_id.as_str()) {
            violations.push(UserAutomationViolation::DuplicateAutomation);
        }
    }
}

fn validate_automation(
    automation: &UserAutomationRow,
    violations: &mut Vec<UserAutomationViolation>,
) {
    if automation.macro_id.trim().is_empty()
        || automation.macro_label.trim().is_empty()
        || automation.macro_family_label.trim().is_empty()
        || automation.macro_version.trim().is_empty()
        || automation.explanation_label.trim().is_empty()
    {
        violations.push(UserAutomationViolation::AutomationRowIncomplete);
    }

    if automation.capture_content_address.trim().is_empty() {
        violations.push(UserAutomationViolation::AutomationMissingCaptureAddress);
    }

    if automation
        .publisher_source_class
        .requires_publisher_identity()
        && automation.publisher_identity_ref.trim().is_empty()
    {
        violations.push(UserAutomationViolation::AutomationMissingPublisherIdentity);
    }

    if automation.capture_provenance.requires_recipe_pack_ref()
        && automation.promotion.recipe_pack_ref.trim().is_empty()
    {
        violations.push(UserAutomationViolation::ImportedCaptureMissingRecipePackRef);
    }

    validate_steps(automation, violations);
    validate_promotion(automation, violations);
    validate_insertion(automation, violations);
    validate_headless_result(automation, violations);
    validate_claim_state(automation, violations);
    validate_downgrade_rules(automation, violations);
}

fn validate_steps(automation: &UserAutomationRow, violations: &mut Vec<UserAutomationViolation>) {
    if automation.steps.is_empty() {
        violations.push(UserAutomationViolation::AutomationMissingStepDisclosures);
        return;
    }

    let mut seen: BTreeSet<ToolSideEffectClass> = BTreeSet::new();
    for step in &automation.steps {
        if !seen.insert(step.side_effect_class) {
            violations.push(UserAutomationViolation::DuplicateStepDisclosure);
        }

        if step.disclosure_label.trim().is_empty() {
            violations.push(UserAutomationViolation::StepDisclosureIncomplete);
        }

        // A mutating step is held to the same preview, policy, and audit bar as a
        // first-party command.
        if step.is_mutating() {
            if !step.interactive_preview.previews_before_replay() {
                violations.push(UserAutomationViolation::MutatingStepWithoutPreviewFirst);
            }
            if !step.has_approval_gate() {
                violations.push(UserAutomationViolation::MutatingStepWithoutApproval);
            }
            if !step.audit.is_audited() {
                violations.push(UserAutomationViolation::MutatingStepWithoutAudit);
            }
        }

        validate_step_headless_safety(step, violations);

        // The disclosed reversibility must agree with the effect class for the
        // two unambiguous cases.
        let reversibility_ok = match step.side_effect_class {
            ToolSideEffectClass::InspectOnly => {
                step.reversibility == RecipeStepReversibilityClass::NoSideEffect
            }
            ToolSideEffectClass::ExternalIrreversiblePublish => {
                step.reversibility == RecipeStepReversibilityClass::IrreversibleExternalPublish
            }
            _ => true,
        };
        if !reversibility_ok {
            violations.push(UserAutomationViolation::StepReversibilityMismatch);
        }
    }
}

fn validate_step_headless_safety(
    step: &AutomationStepDisclosure,
    violations: &mut Vec<UserAutomationViolation>,
) {
    let is_inspect = step.side_effect_class == ToolSideEffectClass::InspectOnly;
    let is_inspect_safe = step.headless_safety == HeadlessSafetyClass::HeadlessSafeInspectOnly;

    // An inspect-only step is always headless-safe inspect-only; a mutating step
    // may never claim the inspect-only headless class.
    if is_inspect && !is_inspect_safe {
        violations.push(UserAutomationViolation::InspectStepNotHeadlessInspectSafe);
    }
    if !is_inspect && is_inspect_safe {
        violations.push(UserAutomationViolation::MutatingStepClaimsInspectHeadlessSafety);
    }

    // An irreversible external publish can never run unattended headless.
    if step.side_effect_class == ToolSideEffectClass::ExternalIrreversiblePublish
        && step.headless_safety.permits_mutation_headless()
    {
        violations.push(UserAutomationViolation::IrreversiblePublishRunsHeadless);
    }

    // A step pre-authorized to run headless still carries a real gate and is
    // audited — pre-authorization is an explicit grant, not an exemption.
    if step.is_mutating() && step.headless_safety.permits_mutation_headless() {
        if !step.has_approval_gate() {
            violations.push(UserAutomationViolation::HeadlessPreauthorizedStepWithoutApproval);
        }
        if !step.audit.is_audited() {
            violations.push(UserAutomationViolation::HeadlessPreauthorizedStepWithoutAudit);
        }
    }
}

fn validate_promotion(
    automation: &UserAutomationRow,
    violations: &mut Vec<UserAutomationViolation>,
) {
    let promotion = &automation.promotion;

    // There is no silent forward: a promoted macro names its recipe, and an
    // unpromoted one names none.
    if promotion.state.is_promoted() && promotion.promoted_recipe_ref.trim().is_empty() {
        violations.push(UserAutomationViolation::PromotedMissingRecipeRef);
    }
    if !promotion.state.is_promoted() && !promotion.promoted_recipe_ref.trim().is_empty() {
        violations.push(UserAutomationViolation::UnpromotedHasRecipeRef);
    }

    // Promoting a mutating macro into a reusable recipe requires a real gate.
    if promotion.state.is_promoted()
        && automation.has_mutating_step()
        && !promotion.has_promotion_gate()
    {
        violations.push(UserAutomationViolation::MutatingPromotionWithoutGate);
    }
}

fn validate_insertion(
    automation: &UserAutomationRow,
    violations: &mut Vec<UserAutomationViolation>,
) {
    let insertion = &automation.insertion;

    if insertion.insertion_label.trim().is_empty() {
        violations.push(UserAutomationViolation::InsertionMissingLabel);
    }

    // Insertion is preview-first: a mutating automation previews before it
    // commits into a target surface.
    if automation.has_mutating_step() && !insertion.preview.previews_before_replay() {
        violations.push(UserAutomationViolation::MutatingInsertionWithoutPreviewFirst);
    }

    // A headless target cannot rely on an interactive approval prompt that can
    // never appear with no operator present.
    if insertion.target_class.is_headless_target()
        && insertion.has_approval_gate()
        && !insertion.approval_is_non_interactive()
    {
        violations
            .push(UserAutomationViolation::HeadlessTargetInsertionRequiresInteractiveApproval);
    }
}

fn validate_headless_result(
    automation: &UserAutomationRow,
    violations: &mut Vec<UserAutomationViolation>,
) {
    let result = &automation.headless_result;

    if result.result_content_address.trim().is_empty() || result.result_label.trim().is_empty() {
        violations.push(UserAutomationViolation::HeadlessResultIncomplete);
    }

    // Step counts reconcile against the disclosed steps.
    let total = automation.steps.len() as u32;
    let completed = automation
        .steps
        .iter()
        .filter(|step| step.headless_safety.runs_headless())
        .count() as u32;
    let deferred = automation
        .steps
        .iter()
        .filter(|step| step.headless_safety.is_deferred())
        .count() as u32;
    let blocked = automation
        .steps
        .iter()
        .filter(|step| step.headless_safety.is_blocked())
        .count() as u32;

    if result.steps_total != total
        || result.steps_completed != completed
        || result.steps_deferred != deferred
        || result.steps_blocked != blocked
        || completed + deferred + blocked != total
    {
        violations.push(UserAutomationViolation::HeadlessResultCountMismatch);
    }

    // The result state agrees with whether any step deferred or blocked.
    let state_ok = match result.state {
        HeadlessResultStateClass::CompletedAllStepsSafe => deferred == 0 && blocked == 0,
        HeadlessResultStateClass::CompletedWithDeferredSteps => deferred >= 1 && blocked == 0,
        HeadlessResultStateClass::BlockedFailClosed
        | HeadlessResultStateClass::DeniedByPolicy
        | HeadlessResultStateClass::PartialThenHalted => blocked >= 1,
    };
    if !state_ok {
        violations.push(UserAutomationViolation::HeadlessResultStateMismatch);
    }

    // A headless run that executed a mutating step must be durably, exportably
    // audited.
    let ran_mutating = automation
        .steps
        .iter()
        .any(AutomationStepDisclosure::runs_unattended_headless);
    if ran_mutating && !result.audit.is_externally_auditable() {
        violations.push(UserAutomationViolation::HeadlessRunMutatingNotExternallyAudited);
    }
}

fn validate_claim_state(
    automation: &UserAutomationRow,
    violations: &mut Vec<UserAutomationViolation>,
) {
    // A blocked promotion narrows its claim instead of staying behind a public
    // qualification.
    if automation.is_claimed() && automation.promotion.state.is_blocked() {
        violations.push(UserAutomationViolation::BlockedPromotionClaimsQualification);
    }

    // A macro still awaiting promotion review may not claim Stable.
    if automation.promotion.state == MacroPromotionStateClass::RecordedPendingReview
        && automation.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(UserAutomationViolation::PendingPromotionClaimsStable);
    }

    if automation.is_claimed() && automation.evidence_packet_refs.is_empty() {
        violations.push(UserAutomationViolation::ClaimedAutomationMissingEvidence);
    }

    // A claimed automation whose policy change can be reversed must have drilled
    // that reversal; a non-applicable posture carries no reversal.
    if automation.is_claimed()
        && automation.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !automation.rollback_verified
    {
        violations.push(UserAutomationViolation::ClaimedRollbackUnverified);
    }
}

fn validate_downgrade_rules(
    automation: &UserAutomationRow,
    violations: &mut Vec<UserAutomationViolation>,
) {
    if automation.downgrade_rules.is_empty() {
        violations.push(UserAutomationViolation::DowngradeRulesMissing);
        return;
    }

    if !automation
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(UserAutomationViolation::DowngradeRuleMissingProofStale);
    }

    // Provider outages and quota exhaustion narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !automation
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(UserAutomationViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(automation.claimed_qualification);
    for rule in &automation.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(UserAutomationViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &UserAutomationPacket,
    violations: &mut Vec<UserAutomationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(UserAutomationViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// User automation is declarative and content-addressed: the support boundary
/// carries content addresses, classes, and review-safe labels only, never raw
/// shell fragments, raw filesystem paths, raw endpoint URLs, raw captured buffer
/// bytes, or credential material.
fn json_contains_forbidden_automation_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_automation_material(text),
        serde_json::Value::Array(values) => values
            .iter()
            .any(json_contains_forbidden_automation_material),
        serde_json::Value::Object(map) => map
            .values()
            .any(json_contains_forbidden_automation_material),
        _ => false,
    }
}

fn contains_forbidden_automation_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key=")
        || lower.contains("api-key=")
        || lower.contains("raw_api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("$(")
        || lower.contains("&& ")
}
