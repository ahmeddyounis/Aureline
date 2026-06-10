//! AI refactor planner with impact sets, candidate previews, and multi-file
//! safety classes.
//!
//! This module ships the canonical M5 packet for an AI refactor planner. It
//! plans a refactor, computes the set of impacted sites with multi-file safety
//! classification, and presents preview candidates that each bind to the
//! evidence-rich patch review lane before any apply. The planner is
//! preview-only: it never applies a change itself. It carries three bound
//! blocks:
//!
//! - A [`RefactorPlanBlock`] binds the refactor to one kind, intent, and scope,
//!   records the plan state, and asserts that a preview is required before any
//!   apply.
//! - An [`ImpactSetBlock`] enumerates every impacted site with its site class,
//!   resolution confidence, and multi-file safety class, and discloses the
//!   worst-case safety class plus whether the analysis is complete. When the
//!   impact set is partial — for example dynamic or reflective references that
//!   cannot be statically resolved — the reason stays visible rather than being
//!   silently dropped.
//! - A [`CandidatePreviewBlock`] presents preview candidates that each reference
//!   a diff packet, validation receipt, and rollback handle into the
//!   evidence-rich patch review lane by id, carry their multi-file safety class,
//!   require human review before apply, and block auto-apply for any unsafe
//!   safety class.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding their
//! content: it cites the
//! [`crate::ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows`]
//! evidence lane for diff/validation/rollback refs and the
//! [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix, and it projects against the frozen context-assembly contract
//! for impact-set and omitted-context truth.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw symbol names, raw file paths, raw diff
//! bodies, raw patch text, raw source bodies, raw prompt bodies, provider
//! payloads, endpoint URLs, credentials, raw token counts, exact prices, and
//! billing-account ids stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/add-the-ai-refactor-planner-with-impact-sets-candidate-previews-and-multi-file-safety-classes.schema.json`](../../../../schemas/ai/add-the-ai-refactor-planner-with-impact-sets-candidate-previews-and-multi-file-safety-classes.schema.json).
//! The contract doc is
//! [`docs/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes.md`](../../../../docs/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RefactorPlannerPacket`].
pub const REFACTOR_PLANNER_RECORD_KIND: &str = "ai_refactor_planner_implementation";

/// Schema version for AI refactor planner records.
pub const REFACTOR_PLANNER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REFACTOR_PLANNER_SCHEMA_REF: &str =
    "schemas/ai/add-the-ai-refactor-planner-with-impact-sets-candidate-previews-and-multi-file-safety-classes.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const REFACTOR_PLANNER_DOC_REF: &str =
    "docs/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes.md";

/// Repo-relative path of the frozen context-assembly contract.
pub const REFACTOR_PLANNER_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen evidence-rich patch review contract.
pub const REFACTOR_PLANNER_EVIDENCE_CONTRACT_REF: &str =
    "docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const REFACTOR_PLANNER_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const REFACTOR_PLANNER_FIXTURE_DIR: &str =
    "fixtures/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes";

/// Repo-relative path of the checked support-export artifact.
pub const REFACTOR_PLANNER_ARTIFACT_REF: &str =
    "artifacts/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REFACTOR_PLANNER_SUMMARY_REF: &str =
    "artifacts/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes.md";

/// Kind of refactor the planner is planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorKind {
    /// Rename a symbol across its definition and references.
    RenameSymbol,
    /// Extract a region into a new function.
    ExtractFunction,
    /// Extract a region into a new variable.
    ExtractVariable,
    /// Inline a symbol at its use sites.
    InlineSymbol,
    /// Move an item between modules or files.
    MoveItem,
    /// Change a function or method signature.
    ChangeSignature,
    /// Introduce a parameter threaded through call sites.
    IntroduceParameter,
    /// Replace a repeated structural pattern.
    ReplacePattern,
    /// Reorganize imports or use declarations.
    OrganizeImports,
}

impl RefactorKind {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenameSymbol => "rename_symbol",
            Self::ExtractFunction => "extract_function",
            Self::ExtractVariable => "extract_variable",
            Self::InlineSymbol => "inline_symbol",
            Self::MoveItem => "move_item",
            Self::ChangeSignature => "change_signature",
            Self::IntroduceParameter => "introduce_parameter",
            Self::ReplacePattern => "replace_pattern",
            Self::OrganizeImports => "organize_imports",
        }
    }
}

/// State of the refactor plan lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorPlanState {
    /// The plan was drafted but the impact set has not been computed.
    Drafted,
    /// The impact set has been analyzed.
    ImpactAnalyzed,
    /// Preview candidates are available.
    CandidatesPreviewed,
    /// The plan is stopped at a human approval gate.
    AwaitingApproval,
    /// A candidate was handed to the evidence-rich patch review apply flow.
    HandedToApply,
    /// The plan is blocked by policy, trust, or an incomplete impact set.
    Blocked,
    /// The plan was discarded while preserving its evidence refs.
    Discarded,
}

impl RefactorPlanState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Drafted => "drafted",
            Self::ImpactAnalyzed => "impact_analyzed",
            Self::CandidatesPreviewed => "candidates_previewed",
            Self::AwaitingApproval => "awaiting_approval",
            Self::HandedToApply => "handed_to_apply",
            Self::Blocked => "blocked",
            Self::Discarded => "discarded",
        }
    }
}

/// Class of an impacted site inside a refactor's impact set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactSiteClass {
    /// The defining occurrence of the refactored item.
    DefinitionSite,
    /// A static reference to the refactored item.
    ReferenceSite,
    /// A test that exercises the refactored item.
    TestSite,
    /// A doc, comment, or example referencing the item.
    DocSite,
    /// A generated or derived site that must be regenerated.
    GeneratedSite,
    /// A public API boundary the refactor crosses.
    PublicApiBoundary,
    /// A cross-crate boundary the refactor crosses.
    CrossCrateBoundary,
    /// A dynamic, reflective, or string-keyed reference that cannot be
    /// statically resolved.
    DynamicOrReflective,
}

impl ImpactSiteClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefinitionSite => "definition_site",
            Self::ReferenceSite => "reference_site",
            Self::TestSite => "test_site",
            Self::DocSite => "doc_site",
            Self::GeneratedSite => "generated_site",
            Self::PublicApiBoundary => "public_api_boundary",
            Self::CrossCrateBoundary => "cross_crate_boundary",
            Self::DynamicOrReflective => "dynamic_or_reflective",
        }
    }
}

/// Confidence with which an impacted site was resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpactConfidenceClass {
    /// Statically resolved with high confidence.
    Resolved,
    /// Matched heuristically and should be reviewed.
    Heuristic,
    /// Ambiguous; a human must confirm whether the site is in scope.
    Ambiguous,
}

impl ImpactConfidenceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Heuristic => "heuristic",
            Self::Ambiguous => "ambiguous",
        }
    }
}

/// Multi-file safety class for a refactor candidate or impacted site.
///
/// The variants are ordered from least to most risky. The planner discloses the
/// worst-case class across the impact set and never lets an unsafe class be
/// auto-applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultiFileSafetyClass {
    /// Purely mechanical edit confined to a single file.
    MechanicalSingleFile,
    /// Mechanical edit across multiple files that type-checks.
    MechanicalMultiFile,
    /// Semantic change confined to one module or local scope.
    SemanticLocal,
    /// Semantic change crossing a module, crate, or API boundary.
    SemanticCrossBoundary,
    /// Change that may alter observable behavior.
    BehaviorAffecting,
    /// Ambiguous or unresolved impact; not safe to apply without a human.
    AmbiguousUnsafe,
}

impl MultiFileSafetyClass {
    /// Every safety class, ordered from least to most risky.
    pub const ALL: [Self; 6] = [
        Self::MechanicalSingleFile,
        Self::MechanicalMultiFile,
        Self::SemanticLocal,
        Self::SemanticCrossBoundary,
        Self::BehaviorAffecting,
        Self::AmbiguousUnsafe,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MechanicalSingleFile => "mechanical_single_file",
            Self::MechanicalMultiFile => "mechanical_multi_file",
            Self::SemanticLocal => "semantic_local",
            Self::SemanticCrossBoundary => "semantic_cross_boundary",
            Self::BehaviorAffecting => "behavior_affecting",
            Self::AmbiguousUnsafe => "ambiguous_unsafe",
        }
    }

    /// Ascending risk rank, used to compute the worst-case across a set.
    pub const fn risk_rank(self) -> u8 {
        match self {
            Self::MechanicalSingleFile => 0,
            Self::MechanicalMultiFile => 1,
            Self::SemanticLocal => 2,
            Self::SemanticCrossBoundary => 3,
            Self::BehaviorAffecting => 4,
            Self::AmbiguousUnsafe => 5,
        }
    }

    /// Whether a candidate of this safety class may ever be auto-applied without
    /// a human review boundary. Only mechanical classes qualify.
    pub const fn is_auto_applicable(self) -> bool {
        matches!(self, Self::MechanicalSingleFile | Self::MechanicalMultiFile)
    }
}

/// State of a preview candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateState {
    /// Proposed but not yet previewed.
    Proposed,
    /// A preview is available.
    Previewed,
    /// Selected for handoff to apply.
    Selected,
    /// Handed to the evidence-rich patch review apply flow.
    HandedToApply,
    /// Rejected and retained for the record.
    Rejected,
}

impl CandidateState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Previewed => "previewed",
            Self::Selected => "selected",
            Self::HandedToApply => "handed_to_apply",
            Self::Rejected => "rejected",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorConsumerSurface {
    /// Desktop refactor planner panel.
    DesktopRefactorPanel,
    /// Desktop review workspace.
    DesktopReviewWorkspace,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Browser or companion follow-up.
    BrowserCompanion,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl RefactorConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopRefactorPanel,
        Self::DesktopReviewWorkspace,
        Self::CliHeadless,
        Self::BrowserCompanion,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopRefactorPanel => "desktop_refactor_panel",
            Self::DesktopReviewWorkspace => "desktop_review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::BrowserCompanion => "browser_companion",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Qualification class for a consumer surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorSurfaceQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental.
    Experimental,
    /// Surface is unavailable on this row.
    Unavailable,
}

impl RefactorSurfaceQualificationClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
        }
    }

    const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Required provider or model is unavailable.
    ProviderUnavailable,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
    /// The impact set is incomplete and the gap was not disclosed.
    ImpactSetIncomplete,
    /// A candidate is missing its required preview/evidence refs.
    CandidatePreviewMissing,
    /// An unsafe safety class was permitted to auto-apply.
    UnsafeClassAutoApplied,
}

impl RefactorDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::ImpactSetIncomplete,
        Self::CandidatePreviewMissing,
        Self::UnsafeClassAutoApplied,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
            Self::ImpactSetIncomplete => "impact_set_incomplete",
            Self::CandidatePreviewMissing => "candidate_preview_missing",
            Self::UnsafeClassAutoApplied => "unsafe_class_auto_applied",
        }
    }
}

/// Refactor plan block binding the refactor to one kind, intent, and scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPlanBlock {
    /// Kind of refactor being planned.
    pub refactor_kind: RefactorKind,
    /// Current plan state.
    pub state: RefactorPlanState,
    /// Opaque ref to the disclosed intent summary. Never raw prompt text.
    pub intent_summary_ref: String,
    /// Opaque ref to the target symbol. Never a raw symbol name.
    pub target_symbol_ref: String,
    /// Opaque ref to the declared refactor scope.
    pub scope_ref: String,
    /// True when the impact set is believed complete. Must match
    /// [`ImpactSetBlock::analysis_complete`].
    pub impact_set_complete: bool,
    /// True when a preview is required before any apply. Must be true.
    pub preview_required_before_apply: bool,
    /// Opaque refs to the context inputs the plan consumed.
    pub context_input_refs: Vec<String>,
}

/// One impacted site row inside the refactor's impact set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactSiteRow {
    /// Opaque site id.
    pub site_id: String,
    /// Opaque file ref the site lives in. Never a raw file path.
    pub file_ref: String,
    /// Class of impacted site.
    pub site_class: ImpactSiteClass,
    /// Confidence with which the site was resolved.
    pub confidence: ImpactConfidenceClass,
    /// Multi-file safety class for this site.
    pub safety_class: MultiFileSafetyClass,
    /// True when the site is included in a candidate's edit.
    pub included_in_candidate: bool,
    /// True when the site is disclosed in the impact set. Must be true.
    pub disclosed: bool,
}

/// Impact-set block enumerating every impacted site and the worst-case safety
/// class across them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactSetBlock {
    /// Stable impact-set id.
    pub impact_set_id: String,
    /// Worst-case multi-file safety class across the impact set. Must be at least
    /// as risky as every site row.
    pub highest_safety_class: MultiFileSafetyClass,
    /// True when the static analysis covered every reachable site.
    pub analysis_complete: bool,
    /// True when, on a partial analysis, the reason for the gap is disclosed.
    pub partial_reason_disclosed: bool,
    /// True when the impact set crosses a module, crate, or API boundary.
    pub cross_boundary_present: bool,
    /// Impacted site rows.
    pub site_rows: Vec<ImpactSiteRow>,
}

/// One preview candidate row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateRow {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Multi-file safety class for this candidate.
    pub safety_class: MultiFileSafetyClass,
    /// Candidate state.
    pub state: CandidateState,
    /// Number of files the candidate edits.
    pub file_count: u32,
    /// Diff packet ref into the evidence-rich patch review lane. Never a raw
    /// diff body.
    pub diff_packet_ref: String,
    /// Validation receipt ref into the evidence-rich patch review lane.
    pub validation_receipt_ref: String,
    /// Rollback handle ref into the evidence-rich patch review lane.
    pub rollback_handle_ref: String,
    /// True when human review is required before apply. Must be true.
    pub review_required_before_apply: bool,
    /// True when auto-apply is blocked because the safety class is unsafe. Must
    /// be true whenever the safety class is not auto-applicable.
    pub auto_apply_blocked_for_unsafe_class: bool,
}

/// Candidate-preview block presenting preview candidates produced before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidatePreviewBlock {
    /// Stable preview id.
    pub preview_id: String,
    /// Candidate id currently selected, if any. Must appear in `candidate_rows`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_candidate_id: Option<String>,
    /// Preview candidate rows.
    pub candidate_rows: Vec<CandidateRow>,
    /// True when a compare-to-base action is available on the preview.
    pub compare_to_base_available: bool,
    /// True when the candidates were produced before any apply. Must be true.
    pub produced_before_apply: bool,
}

/// One cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorSurfaceParityRow {
    /// Consumer surface this row covers.
    pub surface: RefactorConsumerSurface,
    /// True when this surface shows the refactor plan.
    pub shows_plan: bool,
    /// True when this surface shows the impact set.
    pub shows_impact_set: bool,
    /// True when this surface shows the preview candidates.
    pub shows_candidates: bool,
    /// True when this surface is reachable for this packet.
    pub reachable: bool,
    /// Qualification class for this surface projection.
    pub qualification: RefactorSurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

/// Constructor input for [`RefactorPlannerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefactorPlannerPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical refactor plan id shared across surfaces and evidence.
    pub plan_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Refactor plan block.
    pub plan: RefactorPlanBlock,
    /// Impact-set block.
    pub impact_set: ImpactSetBlock,
    /// Candidate-preview block.
    pub candidate_preview: CandidatePreviewBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<RefactorSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<RefactorDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI refactor planner record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPlannerPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical refactor plan id shared across surfaces and evidence.
    pub plan_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Refactor plan block.
    pub plan: RefactorPlanBlock,
    /// Impact-set block.
    pub impact_set: ImpactSetBlock,
    /// Candidate-preview block.
    pub candidate_preview: CandidatePreviewBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<RefactorSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<RefactorDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RefactorPlannerPacket {
    /// Builds an AI refactor planner packet from the stable-lane input.
    pub fn new(input: RefactorPlannerPacketInput) -> Self {
        Self {
            record_kind: REFACTOR_PLANNER_RECORD_KIND.to_owned(),
            schema_version: REFACTOR_PLANNER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            plan_id: input.plan_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            plan: input.plan,
            impact_set: input.impact_set,
            candidate_preview: input.candidate_preview,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the refactor planner packet's stable-line invariants.
    pub fn validate(&self) -> Vec<RefactorPlannerViolation> {
        let mut violations = Vec::new();
        if self.record_kind != REFACTOR_PLANNER_RECORD_KIND {
            violations.push(RefactorPlannerViolation::WrongRecordKind);
        }
        if self.schema_version != REFACTOR_PLANNER_SCHEMA_VERSION {
            violations.push(RefactorPlannerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.plan_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RefactorPlannerViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_plan(self, &mut violations);
        validate_impact_set(self, &mut violations);
        validate_candidate_preview(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("refactor planner packet serializes"),
        ) {
            violations.push(RefactorPlannerViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("refactor planner packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let ambiguous_sites = self
            .impact_set
            .site_rows
            .iter()
            .filter(|row| row.confidence == ImpactConfidenceClass::Ambiguous)
            .count();
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# AI Refactor Planner\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Plan: `{}`\n", self.plan_id));
        out.push_str(&format!(
            "- Refactor: `{}` (state: `{}`)\n",
            self.plan.refactor_kind.as_str(),
            self.plan.state.as_str()
        ));
        out.push_str(&format!(
            "- Impact set: `{}` ({} sites, {} ambiguous, worst-case: `{}`, complete: {})\n",
            self.impact_set.impact_set_id,
            self.impact_set.site_rows.len(),
            ambiguous_sites,
            self.impact_set.highest_safety_class.as_str(),
            self.impact_set.analysis_complete
        ));
        out.push_str(&format!(
            "- Candidates: `{}` ({} candidates, produced before apply: {})\n",
            self.candidate_preview.preview_id,
            self.candidate_preview.candidate_rows.len(),
            self.candidate_preview.produced_before_apply
        ));
        out.push_str(&format!(
            "- Surface parity: {} surfaces ({} stable)\n",
            self.consumer_surface_parity.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Downgrade triggers: {}\n",
            self.downgrade_triggers.len()
        ));
        out
    }
}

/// Validation failures emitted by [`RefactorPlannerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefactorPlannerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Plan block is incomplete.
    PlanIncomplete,
    /// The plan does not require a preview before apply.
    PreviewNotRequiredBeforeApply,
    /// The plan's completeness flag disagrees with the impact set's.
    ImpactCompletenessMismatch,
    /// The impact set is incomplete without disclosing the reason.
    ImpactSetGapUndisclosed,
    /// An impacted site reached the set without being disclosed.
    HiddenImpactSite,
    /// The disclosed worst-case safety class understates a site's class.
    WorstCaseSafetyUnderstated,
    /// A candidate is missing required preview/evidence refs.
    CandidateIncomplete,
    /// A candidate does not require human review before apply.
    CandidateReviewNotRequired,
    /// An unsafe-class candidate is not blocked from auto-apply.
    UnsafeCandidateNotBlocked,
    /// Candidates were not produced before apply.
    CandidatesNotProducedBeforeApply,
    /// The selected candidate id is not present in the candidate rows.
    SelectedCandidateMissing,
    /// A consumer surface is not covered by the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RefactorPlannerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PlanIncomplete => "plan_incomplete",
            Self::PreviewNotRequiredBeforeApply => "preview_not_required_before_apply",
            Self::ImpactCompletenessMismatch => "impact_completeness_mismatch",
            Self::ImpactSetGapUndisclosed => "impact_set_gap_undisclosed",
            Self::HiddenImpactSite => "hidden_impact_site",
            Self::WorstCaseSafetyUnderstated => "worst_case_safety_understated",
            Self::CandidateIncomplete => "candidate_incomplete",
            Self::CandidateReviewNotRequired => "candidate_review_not_required",
            Self::UnsafeCandidateNotBlocked => "unsafe_candidate_not_blocked",
            Self::CandidatesNotProducedBeforeApply => "candidates_not_produced_before_apply",
            Self::SelectedCandidateMissing => "selected_candidate_missing",
            Self::ConsumerSurfaceCoverageMissing => "consumer_surface_coverage_missing",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

impl fmt::Display for RefactorPlannerViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Error for RefactorPlannerViolation {}

/// Errors emitted when reading the checked-in refactor planner export.
#[derive(Debug)]
pub enum RefactorPlannerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RefactorPlannerViolation>),
}

impl fmt::Display for RefactorPlannerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "refactor planner export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "refactor planner export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RefactorPlannerArtifactError {}

/// Returns the checked-in AI refactor planner export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_refactor_planner_export(
) -> Result<RefactorPlannerPacket, RefactorPlannerArtifactError> {
    let packet: RefactorPlannerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/add_the_ai_refactor_planner_with_impact_sets_candidate_previews_and_multi_file_safety_classes/support_export.json"
    )))
    .map_err(RefactorPlannerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RefactorPlannerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RefactorPlannerPacket,
    violations: &mut Vec<RefactorPlannerViolation>,
) {
    for required in [
        REFACTOR_PLANNER_DOC_REF,
        REFACTOR_PLANNER_SCHEMA_REF,
        REFACTOR_PLANNER_CONTEXT_ASSEMBLY_CONTRACT_REF,
        REFACTOR_PLANNER_EVIDENCE_CONTRACT_REF,
        REFACTOR_PLANNER_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(RefactorPlannerViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_plan(packet: &RefactorPlannerPacket, violations: &mut Vec<RefactorPlannerViolation>) {
    let plan = &packet.plan;
    if plan.intent_summary_ref.trim().is_empty()
        || plan.target_symbol_ref.trim().is_empty()
        || plan.scope_ref.trim().is_empty()
        || plan.context_input_refs.is_empty()
    {
        violations.push(RefactorPlannerViolation::PlanIncomplete);
    }
    if !plan.preview_required_before_apply {
        violations.push(RefactorPlannerViolation::PreviewNotRequiredBeforeApply);
    }
    if plan.impact_set_complete != packet.impact_set.analysis_complete {
        violations.push(RefactorPlannerViolation::ImpactCompletenessMismatch);
    }
}

fn validate_impact_set(
    packet: &RefactorPlannerPacket,
    violations: &mut Vec<RefactorPlannerViolation>,
) {
    let impact_set = &packet.impact_set;
    if impact_set.impact_set_id.trim().is_empty() || impact_set.site_rows.is_empty() {
        violations.push(RefactorPlannerViolation::PlanIncomplete);
        return;
    }
    if !impact_set.analysis_complete && !impact_set.partial_reason_disclosed {
        violations.push(RefactorPlannerViolation::ImpactSetGapUndisclosed);
    }
    let mut worst_site_rank = 0u8;
    for site in &impact_set.site_rows {
        if site.site_id.trim().is_empty() || site.file_ref.trim().is_empty() {
            violations.push(RefactorPlannerViolation::PlanIncomplete);
        }
        if !site.disclosed {
            violations.push(RefactorPlannerViolation::HiddenImpactSite);
        }
        worst_site_rank = worst_site_rank.max(site.safety_class.risk_rank());
    }
    if impact_set.highest_safety_class.risk_rank() < worst_site_rank {
        violations.push(RefactorPlannerViolation::WorstCaseSafetyUnderstated);
    }
}

fn validate_candidate_preview(
    packet: &RefactorPlannerPacket,
    violations: &mut Vec<RefactorPlannerViolation>,
) {
    let preview = &packet.candidate_preview;
    if preview.preview_id.trim().is_empty() || preview.candidate_rows.is_empty() {
        violations.push(RefactorPlannerViolation::CandidateIncomplete);
        return;
    }
    if !preview.produced_before_apply {
        violations.push(RefactorPlannerViolation::CandidatesNotProducedBeforeApply);
    }
    for candidate in &preview.candidate_rows {
        if candidate.candidate_id.trim().is_empty()
            || candidate.diff_packet_ref.trim().is_empty()
            || candidate.validation_receipt_ref.trim().is_empty()
            || candidate.rollback_handle_ref.trim().is_empty()
        {
            violations.push(RefactorPlannerViolation::CandidateIncomplete);
        }
        if !candidate.review_required_before_apply {
            violations.push(RefactorPlannerViolation::CandidateReviewNotRequired);
        }
        if !candidate.safety_class.is_auto_applicable()
            && !candidate.auto_apply_blocked_for_unsafe_class
        {
            violations.push(RefactorPlannerViolation::UnsafeCandidateNotBlocked);
        }
    }
    if let Some(selected) = &preview.selected_candidate_id {
        if !preview
            .candidate_rows
            .iter()
            .any(|candidate| &candidate.candidate_id == selected)
        {
            violations.push(RefactorPlannerViolation::SelectedCandidateMissing);
        }
    }
}

fn validate_consumer_surface_parity(
    packet: &RefactorPlannerPacket,
    violations: &mut Vec<RefactorPlannerViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(RefactorPlannerViolation::StableClaimNotQualified);
        }
    }
    for required in RefactorConsumerSurface::ALL {
        if !seen.contains(&required) {
            violations.push(RefactorPlannerViolation::ConsumerSurfaceCoverageMissing);
            break;
        }
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}
