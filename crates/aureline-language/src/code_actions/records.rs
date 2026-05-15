use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use aureline_graph::{WorksetScopeDescriptor, WorksetScopeMode};
use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticFreshness, DiagnosticFreshnessClass, DiagnosticSourceDescriptor,
    DiagnosticSourceFamily,
};
use crate::lsp_router::{
    LocalityClass, ProviderFamily, ProviderKind, RedactionClass, RouterDecisionRecord,
    SupportClass as RouterSupportClass,
};

/// Integer schema version for code-action alpha payloads.
pub type CodeActionAlphaSchemaVersion = u32;

/// Schema version used by code-action alpha records and projections.
pub const CODE_ACTION_ALPHA_SCHEMA_VERSION: CodeActionAlphaSchemaVersion = 1;

/// Error returned when code-action records cannot be built from upstream state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeActionContractError {
    /// The router decision did not contain the selected provider row.
    MissingSelectedProvider {
        /// Selected provider id that could not be resolved.
        selected_provider_id: String,
    },
    /// The diagnostic source family cannot own a code-action mutation.
    UnsupportedSourceFamily {
        /// Source descriptor that could not be projected.
        source_descriptor_id: String,
        /// Unsupported diagnostic source family.
        source_family: DiagnosticSourceFamily,
    },
    /// The selected provider kind cannot own a code-action mutation.
    UnsupportedProviderKind {
        /// Provider id that could not be projected.
        provider_id: String,
        /// Unsupported router provider kind.
        provider_kind: ProviderKind,
    },
}

impl fmt::Display for CodeActionContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSelectedProvider {
                selected_provider_id,
            } => write!(
                f,
                "router decision selected provider {selected_provider_id} but did not include its provider row"
            ),
            Self::UnsupportedSourceFamily {
                source_descriptor_id,
                source_family,
            } => write!(
                f,
                "diagnostic source {source_descriptor_id} with family {source_family:?} cannot own a code action"
            ),
            Self::UnsupportedProviderKind {
                provider_id,
                provider_kind,
            } => write!(
                f,
                "provider {provider_id} with kind {provider_kind:?} cannot own a code action"
            ),
        }
    }
}

impl Error for CodeActionContractError {}

/// Source family that proposed a code action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionSourceKindClass {
    /// Compiler or build tooling proposed or justified the action.
    CompilerOrBuild,
    /// Language-server provider proposed the action.
    LanguageServer,
    /// Linter, formatter, style, or organize-imports provider proposed it.
    LinterFormatterStyle,
    /// Framework pack, schema analyzer, or convention analyzer proposed it.
    FrameworkOrSchemaAnalyzer,
    /// Runtime, test, debug, or notebook evidence proposed it.
    RuntimeTestOrDebug,
    /// Policy, trust, compliance, or security provider proposed it.
    PolicyTrustOrSecurity,
}

impl CodeActionSourceKindClass {
    /// Projects a diagnostic source family into a code-action source family.
    pub fn from_diagnostic_source_family(source_family: DiagnosticSourceFamily) -> Option<Self> {
        match source_family {
            DiagnosticSourceFamily::CompilerOrBuild => Some(Self::CompilerOrBuild),
            DiagnosticSourceFamily::LanguageServer => Some(Self::LanguageServer),
            DiagnosticSourceFamily::LinterFormatterStyle => Some(Self::LinterFormatterStyle),
            DiagnosticSourceFamily::FrameworkOrSchemaAnalyzer => {
                Some(Self::FrameworkOrSchemaAnalyzer)
            }
            DiagnosticSourceFamily::RuntimeTestOrDebug => Some(Self::RuntimeTestOrDebug),
            DiagnosticSourceFamily::PolicyTrustOrSecurity => Some(Self::PolicyTrustOrSecurity),
            DiagnosticSourceFamily::EditorStructural
            | DiagnosticSourceFamily::ScannerImport
            | DiagnosticSourceFamily::ProjectGraph
            | DiagnosticSourceFamily::Heuristic => None,
        }
    }

    /// Projects a router provider kind into a code-action source family.
    pub fn from_provider_kind(provider_kind: ProviderKind) -> Option<Self> {
        match provider_kind {
            ProviderKind::LanguageServer => Some(Self::LanguageServer),
            ProviderKind::FormatterAdapter | ProviderKind::LinterAdapter => {
                Some(Self::LinterFormatterStyle)
            }
            ProviderKind::BuildAdapter => Some(Self::CompilerOrBuild),
            ProviderKind::DebugAdapter | ProviderKind::TestAdapter => {
                Some(Self::RuntimeTestOrDebug)
            }
            ProviderKind::FrameworkPack
            | ProviderKind::NativeAnalyzer
            | ProviderKind::GeneratedSourceBridge => Some(Self::FrameworkOrSchemaAnalyzer),
            ProviderKind::SyntaxParser | ProviderKind::ProjectGraph | ProviderKind::AiAssist => {
                None
            }
        }
    }
}

/// Support posture of the provider that owns the action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionSupportClass {
    /// First-party or directly curated provider with supported behavior.
    FirstPartySupported,
    /// First-party or curated provider with a narrower support claim.
    FirstPartyBestEffort,
    /// External tool adapter owns the proposal.
    ExternalTooling,
    /// Policy-governed provider owns the proposal.
    PolicyGoverned,
}

/// Freshness class admitted for a code-action proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionFreshnessClass {
    /// Provider evidence is current for the admitted epoch and scope.
    Current,
    /// Provider evidence is recent but below exact-current posture.
    Recent,
    /// Provider evidence belongs to an older epoch or target.
    Stale,
    /// Newer evidence exists and this proposal remains only for lineage.
    Superseded,
    /// Proposal came from imported evidence and is read-only until confirmed.
    ImportedSnapshot,
}

impl CodeActionFreshnessClass {
    /// Projects diagnostic freshness into code-action freshness.
    pub const fn from_diagnostic_freshness(freshness_class: DiagnosticFreshnessClass) -> Self {
        match freshness_class {
            DiagnosticFreshnessClass::Current => Self::Current,
            DiagnosticFreshnessClass::Recent | DiagnosticFreshnessClass::WarmCached => Self::Recent,
            DiagnosticFreshnessClass::Stale
            | DiagnosticFreshnessClass::DegradedCached
            | DiagnosticFreshnessClass::Unverified => Self::Stale,
            DiagnosticFreshnessClass::Superseded => Self::Superseded,
            DiagnosticFreshnessClass::ImportedSnapshot => Self::ImportedSnapshot,
        }
    }

    /// Returns true when the action cannot claim current exact truth.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Semantic-layer state the action relies on before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticLayerStateClass {
    /// Semantic layer and anchor are current and exact.
    SemanticCurrentExact,
    /// Semantic layer is current enough to remap the anchor.
    SemanticCurrentRemapped,
    /// Semantic layer is current only for a narrower slice.
    SemanticNarrowedScope,
    /// Semantic layer is cached and reviewable but not current exact truth.
    SemanticCachedRecent,
    /// Semantic epoch is stale or mismatched.
    SemanticStaleEpochMismatch,
    /// Only syntax or text fallback is available.
    SyntacticOrTextOnly,
    /// Runtime evidence exists without a safe semantic edit basis.
    RuntimeObservedNoSemanticBasis,
    /// Imported evidence exists without local semantic revalidation.
    ImportedSnapshotOnly,
    /// Policy asserted the finding without a semantic source claim.
    PolicyAssertedNonSemantic,
}

impl SemanticLayerStateClass {
    /// Returns true when a preview or block is required before mutation.
    pub const fn requires_preview_or_block(self) -> bool {
        !matches!(
            self,
            Self::SemanticCurrentExact | Self::SemanticCurrentRemapped
        )
    }
}

/// Action family exposed to editor, review, CLI, and support consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionClass {
    /// Quick fix for one diagnostic.
    QuickFixSingleDiagnostic,
    /// Quick fix for a cluster of related diagnostics.
    QuickFixCluster,
    /// Fix all diagnostics for a rule or action family.
    FixAllRule,
    /// Source action that organizes imports.
    OrganizeImports,
    /// Formatter or whole-document rewrite action.
    FormatterRewrite,
    /// Generated source synchronization action.
    GeneratedSourceSync,
    /// Read-only validation or recheck action.
    ValidationOnlyRecheck,
}

impl ActionClass {
    /// Returns true when the action may write user or derived state.
    pub const fn is_mutation_bearing(self) -> bool {
        !matches!(self, Self::ValidationOnlyRecheck)
    }

    /// Returns true when the action is a quick fix.
    pub const fn is_quick_fix(self) -> bool {
        matches!(self, Self::QuickFixSingleDiagnostic | Self::QuickFixCluster)
    }
}

/// Side-effect class for applying a code action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionSideEffectClass {
    /// Action does not mutate; it only refreshes or validates evidence.
    NoMutationValidationOnly,
    /// Action mutates the current diagnostic anchor only.
    CurrentAnchorTextEdit,
    /// Action mutates the current file outside the exact anchor.
    CurrentFileTextEdit,
    /// Action rewrites the current document.
    WholeFileRewrite,
    /// Action mutates more than one file or workspace object.
    MultiFileWorkspaceEdit,
    /// Action changes configuration, dependency, policy, or repo truth.
    ConfigurationOrDependencyMutation,
    /// Action touches generated or protected paths.
    GeneratedOrProtectedMutation,
    /// Provider did not supply a stable mutation boundary.
    UnknownProviderMutation,
}

impl CodeActionSideEffectClass {
    /// Returns true when the side-effect class represents a mutation.
    pub const fn is_mutation_bearing(self) -> bool {
        !matches!(self, Self::NoMutationValidationOnly)
    }

    /// Returns true when this side-effect class needs a review surface.
    pub const fn requires_preview(self) -> bool {
        matches!(
            self,
            Self::WholeFileRewrite
                | Self::MultiFileWorkspaceEdit
                | Self::ConfigurationOrDependencyMutation
                | Self::GeneratedOrProtectedMutation
                | Self::UnknownProviderMutation
        )
    }

    /// Returns true when inline apply may be admitted if other fields agree.
    pub const fn allows_silent_apply(self) -> bool {
        matches!(
            self,
            Self::CurrentAnchorTextEdit | Self::CurrentFileTextEdit
        )
    }

    /// Returns true when the action changes configuration or dependency truth.
    pub const fn is_configuration_changing(self) -> bool {
        matches!(self, Self::ConfigurationOrDependencyMutation)
    }
}

/// Safety class assigned to the action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionSafetyClass {
    /// Trivia-only change such as whitespace or punctuation.
    TriviaSafe,
    /// Local syntax-level change with no semantic claim.
    LocalSyntaxSafe,
    /// Current-file semantic mutation.
    SemanticLocal,
    /// Cross-file semantic mutation.
    CrossFileSemantic,
    /// Generated or protected path mutation.
    GeneratedOrProtected,
    /// Unknown, unstable, stale, or provider-ambiguous mutation.
    UnknownOrUnstable,
}

impl CodeActionSafetyClass {
    /// Returns true when the safety class cannot be applied inline.
    pub const fn requires_preview(self) -> bool {
        matches!(
            self,
            Self::CrossFileSemantic | Self::GeneratedOrProtected | Self::UnknownOrUnstable
        )
    }
}

/// Refactor-scope admission after binding candidate edits to workset truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorScopeAdmissionClass {
    /// Every candidate edit remains inside the active named workset.
    WithinNamedWorkset,
    /// Candidate edits outside the named workset were refused.
    OutsideNamedWorksetRefused,
    /// The caller attempted to widen and must complete scope review first.
    BlockedPendingScopeWideningReview,
}

impl RefactorScopeAdmissionClass {
    /// Returns the stable schema token for this admission class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinNamedWorkset => "within_named_workset",
            Self::OutsideNamedWorksetRefused => "outside_named_workset_refused",
            Self::BlockedPendingScopeWideningReview => "blocked_pending_scope_widening_review",
        }
    }

    /// Returns true when the admission requires visible scope disclosure.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::WithinNamedWorkset)
    }
}

/// Surface trigger that requested a scope-widening review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeWideningReviewTriggerClass {
    /// Refactor preview or apply attempted to include outside-scope targets.
    RefactorWiden,
}

impl ScopeWideningReviewTriggerClass {
    /// Returns the stable schema token for this trigger class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefactorWiden => "refactor_widen",
        }
    }
}

/// One candidate target in a refactor scope-admission request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorScopeCandidate {
    /// Stable target ref, such as a semantic result or occurrence id.
    pub target_ref: String,
    /// Scope ref that owns this target.
    pub scope_ref: String,
    /// Workspace-relative path for review surfaces.
    pub workspace_relative_path: String,
    /// Export-safe target summary.
    pub summary: String,
}

impl RefactorScopeCandidate {
    /// Builds a candidate target for workset-scope admission.
    pub fn new(
        target_ref: impl Into<String>,
        scope_ref: impl Into<String>,
        workspace_relative_path: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            target_ref: target_ref.into(),
            scope_ref: scope_ref.into(),
            workspace_relative_path: workspace_relative_path.into(),
            summary: summary.into(),
        }
    }
}

/// Scope decision for one refactor candidate target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorScopeTargetRow {
    /// Stable target ref from the candidate.
    pub target_ref: String,
    /// Scope ref that owns this target.
    pub scope_ref: String,
    /// Workspace-relative path for review surfaces.
    pub workspace_relative_path: String,
    /// True when the target is admitted by the active workset descriptor.
    pub inside_named_workset: bool,
    /// Refused reason token when the target is outside scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refused_reason_ref: Option<String>,
    /// Export-safe target summary.
    pub summary: String,
}

/// Typed review prompt created when a refactor attempts to widen scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionScopeWideningReview {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable review request id.
    pub review_request_id: String,
    /// Trigger that requested the widening review.
    pub trigger_class: ScopeWideningReviewTriggerClass,
    /// Active scope id from the workset descriptor.
    pub active_scope_id: String,
    /// Active scope class token from the workset descriptor.
    pub active_scope_class: String,
    /// Active scope mode reused from graph workset truth.
    pub active_scope_mode: WorksetScopeMode,
    /// Outside scope refs that would be added by the widening.
    pub requested_scope_refs: Vec<String>,
    /// Candidate targets blocked until the review is confirmed.
    pub blocked_target_refs: Vec<String>,
    /// Hidden result count from the active workset descriptor.
    pub hidden_result_count: usize,
    /// True because widening must be confirmed with an explicit review action.
    pub typed_confirmation_required: bool,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe review summary.
    pub export_safe_summary: String,
}

impl CodeActionScopeWideningReview {
    /// Stable record-kind tag for scope-widening reviews.
    pub const RECORD_KIND: &'static str = "code_action_scope_widening_review";
}

/// Refactor-scope binding produced from graph workset truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionRefactorScopeBinding {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Workset descriptor that bounds the refactor.
    pub workset_scope: WorksetScopeDescriptor,
    /// Admission class after candidate targets were checked.
    pub admission_class: RefactorScopeAdmissionClass,
    /// Per-target scope decisions.
    pub target_scope_rows: Vec<RefactorScopeTargetRow>,
    /// Target refs admitted for mutation.
    pub admitted_target_refs: Vec<String>,
    /// Target refs refused because they are outside the active workset.
    pub refused_target_refs: Vec<String>,
    /// Typed scope-widening review when the caller attempted to widen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_widening_review: Option<CodeActionScopeWideningReview>,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe scope caveat.
    pub caveat_summary: String,
}

impl CodeActionRefactorScopeBinding {
    /// Stable record-kind tag for refactor-scope bindings.
    pub const RECORD_KIND: &'static str = "code_action_refactor_scope_binding";

    /// Builds a scope binding by checking candidate targets against a workset descriptor.
    pub fn from_workset_scope(
        workset_scope: WorksetScopeDescriptor,
        candidates: impl IntoIterator<Item = RefactorScopeCandidate>,
        scope_widening_requested: bool,
        captured_at: impl Into<String>,
    ) -> Self {
        let captured_at = captured_at.into();
        let target_scope_rows = candidates
            .into_iter()
            .map(|candidate| {
                let inside_named_workset = candidate_inside_workset(&workset_scope, &candidate);
                RefactorScopeTargetRow {
                    target_ref: candidate.target_ref,
                    scope_ref: candidate.scope_ref,
                    workspace_relative_path: candidate.workspace_relative_path,
                    inside_named_workset,
                    refused_reason_ref: (!inside_named_workset)
                        .then(|| "outside_named_workset".to_owned()),
                    summary: candidate.summary,
                }
            })
            .collect::<Vec<_>>();
        let admitted_target_refs = target_scope_rows
            .iter()
            .filter(|row| row.inside_named_workset)
            .map(|row| row.target_ref.clone())
            .collect::<Vec<_>>();
        let refused_target_refs = target_scope_rows
            .iter()
            .filter(|row| !row.inside_named_workset)
            .map(|row| row.target_ref.clone())
            .collect::<Vec<_>>();
        let admission_class = if refused_target_refs.is_empty() {
            RefactorScopeAdmissionClass::WithinNamedWorkset
        } else if scope_widening_requested {
            RefactorScopeAdmissionClass::BlockedPendingScopeWideningReview
        } else {
            RefactorScopeAdmissionClass::OutsideNamedWorksetRefused
        };
        let scope_widening_review = if scope_widening_requested && !refused_target_refs.is_empty() {
            Some(scope_widening_review(
                &workset_scope,
                &target_scope_rows,
                &refused_target_refs,
                &captured_at,
            ))
        } else {
            None
        };
        let caveat_summary = match admission_class {
            RefactorScopeAdmissionClass::WithinNamedWorkset => format!(
                "Refactor is bound to {} with {} admitted targets.",
                workset_scope.scope_id,
                admitted_target_refs.len()
            ),
            RefactorScopeAdmissionClass::OutsideNamedWorksetRefused => format!(
                "{} targets outside {} were refused; scope was not widened.",
                refused_target_refs.len(),
                workset_scope.scope_id
            ),
            RefactorScopeAdmissionClass::BlockedPendingScopeWideningReview => format!(
                "{} targets outside {} require scope-widening review before mutation.",
                refused_target_refs.len(),
                workset_scope.scope_id
            ),
        };

        Self {
            record_kind: Self::RECORD_KIND.into(),
            workset_scope,
            admission_class,
            target_scope_rows,
            admitted_target_refs,
            refused_target_refs,
            scope_widening_review,
            captured_at,
            caveat_summary,
        }
    }

    /// Returns true when the target ref is admitted for mutation.
    pub fn admits_target_ref(&self, target_ref: &str) -> bool {
        self.admitted_target_refs
            .iter()
            .any(|item| item == target_ref)
    }

    /// Returns true when scope state must be visible before apply.
    pub fn requires_preview_or_review(&self) -> bool {
        self.admission_class.requires_disclosure() || self.scope_widening_review.is_some()
    }

    /// Returns true when a widening review blocks outside-scope mutation.
    pub fn requires_scope_widening_review(&self) -> bool {
        matches!(
            self.admission_class,
            RefactorScopeAdmissionClass::BlockedPendingScopeWideningReview
        )
    }
}

/// Mutation scope claimed by the action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationScopeClass {
    /// Only the current anchor is affected.
    SingleAnchor,
    /// One localized region in one file is affected.
    SingleFileLocalized,
    /// One whole document is affected.
    SingleFileWholeDocument,
    /// Multiple files inside one module are affected.
    MultiFileSameModule,
    /// Multiple files across the workspace are affected.
    MultiFileWorkspace,
    /// Generated family or paired generated output is affected.
    GeneratedFamily,
    /// Protected, policy-scoped, or repo-truth state is affected.
    ProtectedOrPolicyScoped,
}

impl MutationScopeClass {
    /// Returns true when the scope is broader than one local file.
    pub const fn is_multi_file(self) -> bool {
        matches!(self, Self::MultiFileSameModule | Self::MultiFileWorkspace)
    }

    /// Returns true when the scope touches generated or protected truth.
    pub const fn is_generated_or_protected(self) -> bool {
        matches!(self, Self::GeneratedFamily | Self::ProtectedOrPolicyScoped)
    }

    /// Returns true when the scope requires preview on alpha surfaces.
    pub const fn requires_preview(self) -> bool {
        matches!(
            self,
            Self::SingleFileWholeDocument
                | Self::MultiFileSameModule
                | Self::MultiFileWorkspace
                | Self::GeneratedFamily
                | Self::ProtectedOrPolicyScoped
        )
    }
}

/// Preview requirement that must be visible before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRequirementClass {
    /// No preview is required.
    NotRequired,
    /// Inline summary disclosure is enough for this action.
    InlineSummary,
    /// Structured diff preview is required.
    StructuredDiff,
    /// Batch scope preview is required.
    BatchScopePreview,
    /// Generated or protected path preview is required.
    GeneratedOrProtectedPreviewRequired,
    /// Policy, configuration, dependency, or repo mutation preview is required.
    PolicyOrRepoMutationPreviewRequired,
}

impl PreviewRequirementClass {
    /// Returns true when apply must route through a preview or review surface.
    pub const fn requires_review(self) -> bool {
        matches!(
            self,
            Self::StructuredDiff
                | Self::BatchScopePreview
                | Self::GeneratedOrProtectedPreviewRequired
                | Self::PolicyOrRepoMutationPreviewRequired
        )
    }
}

/// Current apply posture for the action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyPostureClass {
    /// Inline apply is allowed if all admission checks pass.
    ApplyInlineAllowed,
    /// A preview or review surface must be opened before apply.
    PreviewBeforeApply,
    /// User review is required before the action can apply.
    BlockedPendingUserReview,
    /// Policy or trust review is required before the action can apply.
    BlockedPendingPolicyOrTrust,
    /// The action is read-only and does not apply edits.
    NotApplicableReadOnly,
}

impl ApplyPostureClass {
    /// Returns true when this posture blocks direct mutation.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedPendingUserReview | Self::BlockedPendingPolicyOrTrust
        )
    }
}

/// Reason the action cannot currently apply inline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockingReasonClass {
    /// Generated paths are included in the proposed mutation.
    GeneratedPathsPresent,
    /// Protected paths are included in the proposed mutation.
    ProtectedPathsPresent,
    /// Provider freshness is below the required floor.
    ProviderFreshnessUnmet,
    /// Semantic epoch is stale or mismatched.
    SemanticEpochMismatch,
    /// The proposal is imported evidence only.
    ImportedSnapshotOnly,
    /// Policy denied the mutation.
    PolicyDenied,
    /// Runtime evidence exists without a safe edit basis.
    RuntimeEvidenceOnlyNoSafeEdit,
    /// Suspicious content or raw/rendered integrity cues require review.
    ContentIntegrityCuePresent,
}

/// Validation hint emitted with a code-action proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationHintClass {
    /// Rerun the diagnostic producer.
    RerunDiagnosticProducer,
    /// Rerun build or test evidence.
    RerunBuildOrTest,
    /// Refresh the semantic snapshot.
    RefreshSemanticSnapshot,
    /// Review generated outputs.
    ReviewGeneratedOutputs,
    /// Inspect protected paths before apply.
    InspectProtectedPaths,
    /// Compare imported baseline delta.
    CompareImportedBaselineDelta,
    /// No automatic validation is available.
    NoAutomaticValidationAvailable,
}

/// Replay hint emitted with a code-action proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayHintClass {
    /// Replay against the same execution context.
    ReplayAgainstSameExecutionContext,
    /// Replay against a newer semantic epoch.
    ReplayAgainstNewSemanticEpoch,
    /// Export a review packet.
    ExportReviewPacket,
    /// Attach support bundle evidence.
    AttachSupportBundleEvidence,
    /// Refresh rule metadata before replay.
    RequiresRuleMetadataRefresh,
}

/// Epoch role carried by a code-action proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionEpochRoleClass {
    /// Workspace-scope epoch.
    WorkspaceScope,
    /// Diagnostic collection epoch.
    DiagnosticCollection,
    /// Language semantic model epoch.
    LanguageSemanticModel,
    /// Build graph epoch.
    BuildGraph,
    /// Execution run epoch.
    ExecutionRun,
    /// Imported scan epoch.
    ImportedScan,
    /// Anchor remap family epoch.
    AnchorRemapFamily,
    /// Policy bundle epoch.
    PolicyBundle,
}

/// Trust state applied to a code-action proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionTrustState {
    /// Workspace trust admits the action.
    Trusted,
    /// Trust policy narrows the action.
    Restricted,
}

/// Reversal class attached to a named undo group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoReversalClass {
    /// Prior durable state can be restored exactly.
    Exact,
    /// Related mutations reverse as one exact grouped change.
    GroupedExact,
    /// A compensating mutation is required.
    Compensate,
    /// A generated artifact must be regenerated from source.
    Regenerate,
    /// A named checkpoint must be restored.
    RestoreCheckpoint,
    /// Manual recovery is required.
    Manual,
    /// No product-state undo exists.
    AuditOnly,
}

/// Surface consuming code-action alpha state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionSurfaceClass {
    /// Editor quick-fix or action picker.
    EditorActionPicker,
    /// Review or preview sheet.
    ReviewPreview,
    /// Support or diagnostics export packet.
    SupportExport,
    /// CLI or headless JSON projection.
    CliJson,
}

impl CodeActionSurfaceClass {
    /// Returns the stable schema token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorActionPicker => "editor_action_picker",
            Self::ReviewPreview => "review_preview",
            Self::SupportExport => "support_export",
            Self::CliJson => "cli_json",
        }
    }
}

/// Opaque epoch binding relevant to an action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeActionEpochBinding {
    /// Epoch role.
    pub epoch_role_class: CodeActionEpochRoleClass,
    /// Epoch reference.
    pub epoch_ref: String,
}

/// Minimal policy context attached to an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionPolicyContext {
    /// Policy epoch applied to the action.
    pub policy_epoch: String,
    /// Trust state applied to the action.
    pub trust_state: CodeActionTrustState,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
}

/// Provider descriptor for the actor proposing the action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionProviderDescriptor {
    /// Provider id or source descriptor id.
    pub provider_id: String,
    /// Source kind for the provider.
    pub source_kind_class: CodeActionSourceKindClass,
    /// Support posture for the provider.
    pub support_class: CodeActionSupportClass,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Opaque tool identity reference.
    pub tool_identity_ref: String,
    /// Opaque tool version reference, when known.
    pub tool_version_ref: Option<String>,
    /// Language provider family, when the provider belongs to one.
    pub language_provider_family: Option<ProviderFamily>,
    /// Freshness class for the provider evidence.
    pub freshness_class: CodeActionFreshnessClass,
    /// Locality where the provider ran or originated.
    pub locality_class: LocalityClass,
    /// Semantic-layer state this action relies on.
    pub semantic_layer_state_class: SemanticLayerStateClass,
    /// Current epoch bindings cited by this provider.
    pub current_epoch_bindings: Vec<CodeActionEpochBinding>,
    /// Export-safe provider summary.
    pub summary: String,
}

impl CodeActionProviderDescriptor {
    /// Builds a provider descriptor from a diagnostic source descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`CodeActionContractError::UnsupportedSourceFamily`] when the
    /// diagnostic source cannot own a code-action proposal.
    pub fn from_diagnostic_source(
        source: &DiagnosticSourceDescriptor,
        freshness: &DiagnosticFreshness,
        provider_display_label: impl Into<String>,
        semantic_layer_state_class: SemanticLayerStateClass,
        current_epoch_bindings: Vec<CodeActionEpochBinding>,
    ) -> Result<Self, CodeActionContractError> {
        let source_kind_class =
            CodeActionSourceKindClass::from_diagnostic_source_family(source.source_family)
                .ok_or_else(|| CodeActionContractError::UnsupportedSourceFamily {
                    source_descriptor_id: source.source_descriptor_id.clone(),
                    source_family: source.source_family,
                })?;
        Ok(Self {
            provider_id: source
                .provider_id
                .clone()
                .unwrap_or_else(|| source.source_descriptor_id.clone()),
            source_kind_class,
            support_class: support_from_diagnostic_source(source),
            provider_display_label: provider_display_label.into(),
            tool_identity_ref: source.producer_ref.clone(),
            tool_version_ref: source.producer_version_ref.clone(),
            language_provider_family: provider_family_from_source(source.source_family),
            freshness_class: CodeActionFreshnessClass::from_diagnostic_freshness(
                freshness.freshness_class,
            ),
            locality_class: source.locality_class,
            semantic_layer_state_class,
            current_epoch_bindings,
            summary: source.summary.clone(),
        })
    }

    /// Builds a provider descriptor from the selected provider in a router decision.
    ///
    /// # Errors
    ///
    /// Returns [`CodeActionContractError::MissingSelectedProvider`] when the
    /// decision references an absent provider row, or
    /// [`CodeActionContractError::UnsupportedProviderKind`] when the selected
    /// provider kind cannot own a code-action proposal.
    pub fn from_router_decision(
        decision: &RouterDecisionRecord,
        tool_identity_ref: impl Into<String>,
        tool_version_ref: Option<String>,
        semantic_layer_state_class: SemanticLayerStateClass,
        current_epoch_bindings: Vec<CodeActionEpochBinding>,
    ) -> Result<Self, CodeActionContractError> {
        let selected_provider_id = &decision.decision_outcome.selected_provider_id;
        let row = decision
            .provider_stack_rows
            .iter()
            .find(|row| &row.provider_id == selected_provider_id)
            .ok_or_else(|| CodeActionContractError::MissingSelectedProvider {
                selected_provider_id: selected_provider_id.clone(),
            })?;
        let source_kind_class = CodeActionSourceKindClass::from_provider_kind(row.provider_kind)
            .ok_or_else(|| CodeActionContractError::UnsupportedProviderKind {
                provider_id: row.provider_id.clone(),
                provider_kind: row.provider_kind,
            })?;

        Ok(Self {
            provider_id: row.provider_id.clone(),
            source_kind_class,
            support_class: support_from_router_provider(row.provider_kind, row.support_class),
            provider_display_label: row.provider_display_label.clone(),
            tool_identity_ref: tool_identity_ref.into(),
            tool_version_ref,
            language_provider_family: provider_family_from_kind(row.provider_kind),
            freshness_class: freshness_from_router(row.freshness_class),
            locality_class: row.locality_class,
            semantic_layer_state_class,
            current_epoch_bindings,
            summary: row.summary.clone(),
        })
    }

    /// Returns true when this provider cannot support inline current apply.
    pub fn requires_preview_or_block(&self) -> bool {
        self.freshness_class.requires_disclosure()
            || self.semantic_layer_state_class.requires_preview_or_block()
            || matches!(
                self.support_class,
                CodeActionSupportClass::FirstPartyBestEffort
                    | CodeActionSupportClass::PolicyGoverned
            )
    }
}

/// Counts that summarize the mutation blast radius.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CodeActionMutationCounts {
    /// Diagnostics affected by the proposal.
    pub affected_diagnostic_count: usize,
    /// Files affected by the proposal.
    pub affected_file_count: usize,
    /// Anchors affected by the proposal.
    pub affected_anchor_count: usize,
    /// Generated paths affected by the proposal.
    pub generated_path_count: usize,
    /// Protected paths affected by the proposal.
    pub protected_path_count: usize,
    /// Paths blocked by policy or write safety.
    pub blocked_path_count: usize,
    /// Configuration objects affected by the proposal.
    pub configuration_mutation_count: usize,
    /// Dependency or lockfile objects affected by the proposal.
    pub dependency_mutation_count: usize,
}

impl CodeActionMutationCounts {
    /// Returns true when counts show generated or protected impact.
    pub const fn has_generated_or_protected_impact(&self) -> bool {
        self.generated_path_count > 0
            || self.protected_path_count > 0
            || self.blocked_path_count > 0
    }

    /// Returns true when counts show configuration or dependency mutation.
    pub const fn has_configuration_or_dependency_impact(&self) -> bool {
        self.configuration_mutation_count > 0 || self.dependency_mutation_count > 0
    }

    /// Returns true when counts require preview before apply.
    pub const fn requires_preview(&self) -> bool {
        self.affected_file_count > 1
            || self.has_generated_or_protected_impact()
            || self.has_configuration_or_dependency_impact()
    }
}

/// Validation and replay plan for an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionValidationPlan {
    /// Validation hints that explain post-apply checks.
    pub validation_hint_classes: Vec<ValidationHintClass>,
    /// Replay hints for review, CLI, support, or automation consumers.
    pub replay_hint_classes: Vec<ReplayHintClass>,
    /// Export-safe validation summary.
    pub validation_summary: String,
}

/// Named undo group attached to a mutating action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionUndoGroup {
    /// Stable undo group id.
    pub undo_group_id: String,
    /// Plain-language undo group label.
    pub undo_group_label: String,
    /// Command id that would apply the action.
    pub command_id_ref: String,
    /// Provider or actor that owns the undo group.
    pub actor_provider_ref: String,
    /// Reversal class for this group.
    pub reversal_class: UndoReversalClass,
    /// Checkpoint reference, when one is created.
    pub checkpoint_ref: Option<String>,
    /// Export-safe undo summary.
    pub summary: String,
}

impl CodeActionUndoGroup {
    /// Returns true when the undo group has a name and actor attribution.
    pub fn is_named_and_attributable(&self) -> bool {
        !self.undo_group_id.trim().is_empty()
            && !self.undo_group_label.trim().is_empty()
            && !self.command_id_ref.trim().is_empty()
            && !self.actor_provider_ref.trim().is_empty()
    }
}

/// Content-integrity review state linked to an action.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CodeActionContentIntegrityReview {
    /// Suspicious-text or raw/rendered finding refs linked to the action.
    pub suspicious_text_finding_refs: Vec<String>,
    /// Whether safe preview is required before apply.
    pub safe_preview_required: bool,
    /// Export-safe content-integrity summary.
    pub summary: String,
}

impl CodeActionContentIntegrityReview {
    /// Returns true when content-integrity cues require review.
    pub fn requires_preview(&self) -> bool {
        self.safe_preview_required || !self.suspicious_text_finding_refs.is_empty()
    }
}

/// Runtime code-action alpha record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub code_action_alpha_schema_version: CodeActionAlphaSchemaVersion,
    /// Stable action id.
    pub code_action_id: String,
    /// Action family.
    pub action_class: ActionClass,
    /// Plain-language action label.
    pub action_label: String,
    /// Provider proposing the action.
    pub acting_provider: CodeActionProviderDescriptor,
    /// Diagnostic refs that triggered the action.
    pub triggering_diagnostic_refs: Vec<String>,
    /// Side-effect class disclosed before apply.
    pub side_effect_class: CodeActionSideEffectClass,
    /// Safety class for the proposal.
    pub safety_class: CodeActionSafetyClass,
    /// Mutation scope for the proposal.
    pub mutation_scope_class: MutationScopeClass,
    /// Preview requirement for the proposal.
    pub preview_requirement_class: PreviewRequirementClass,
    /// Current apply posture.
    pub apply_posture_class: ApplyPostureClass,
    /// Blocking reasons that prevent direct apply.
    pub blocking_reason_classes: Vec<BlockingReasonClass>,
    /// Mutation blast-radius counts.
    pub mutation_counts: CodeActionMutationCounts,
    /// Epoch bindings cited by the proposal.
    pub current_epoch_bindings: Vec<CodeActionEpochBinding>,
    /// Validation and replay plan.
    pub validation_plan: CodeActionValidationPlan,
    /// Named undo group, when the action can mutate state.
    pub undo_group: Option<CodeActionUndoGroup>,
    /// Checkpoint reference, when one exists.
    pub checkpoint_ref: Option<String>,
    /// Review packet reference, when one exists.
    pub review_packet_ref: Option<String>,
    /// Content-integrity review cues.
    pub content_integrity_review: CodeActionContentIntegrityReview,
    /// Refactor-scope binding when the action proposes a refactor mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refactor_scope_binding: Option<CodeActionRefactorScopeBinding>,
    /// Policy context applied to this proposal.
    pub policy_context: CodeActionPolicyContext,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CodeActionRecord {
    /// Stable record-kind tag for code-action alpha records.
    pub const RECORD_KIND: &'static str = "code_action_alpha_record";

    /// Returns true when the action may mutate state.
    pub fn is_mutation_bearing(&self) -> bool {
        self.action_class.is_mutation_bearing() && self.side_effect_class.is_mutation_bearing()
    }

    /// Returns true when the action is multi-file or configuration-changing.
    pub fn is_multi_file_or_configuration_changing(&self) -> bool {
        self.mutation_scope_class.is_multi_file()
            || self.side_effect_class.is_configuration_changing()
            || self
                .mutation_counts
                .has_configuration_or_dependency_impact()
    }

    /// Returns true when generated or protected impact is present.
    pub fn has_generated_or_protected_impact(&self) -> bool {
        self.mutation_scope_class.is_generated_or_protected()
            || self.mutation_counts.has_generated_or_protected_impact()
            || self.safety_class == CodeActionSafetyClass::GeneratedOrProtected
            || self.side_effect_class == CodeActionSideEffectClass::GeneratedOrProtectedMutation
    }

    /// Returns true when a preview or review path is required.
    pub fn preview_required(&self) -> bool {
        self.preview_requirement_class.requires_review()
            || self.side_effect_class.requires_preview()
            || self.safety_class.requires_preview()
            || self.mutation_scope_class.requires_preview()
            || self.mutation_counts.requires_preview()
            || self.acting_provider.requires_preview_or_block()
            || self.content_integrity_review.requires_preview()
            || self
                .refactor_scope_binding
                .as_ref()
                .is_some_and(CodeActionRefactorScopeBinding::requires_preview_or_review)
            || self.apply_posture_class == ApplyPostureClass::PreviewBeforeApply
            || !self.blocking_reason_classes.is_empty()
    }

    /// Returns true when the action has a named and attributable undo group.
    pub fn has_named_undo_group(&self) -> bool {
        self.undo_group
            .as_ref()
            .is_some_and(CodeActionUndoGroup::is_named_and_attributable)
    }

    /// Returns true when a mutating action has the required undo metadata.
    pub fn has_required_undo_group(&self) -> bool {
        !self.is_mutation_bearing() || self.has_named_undo_group()
    }

    /// Returns true when inline apply may proceed without opening preview.
    pub fn silent_apply_allowed(&self) -> bool {
        self.apply_posture_class == ApplyPostureClass::ApplyInlineAllowed
            && !self.preview_required()
            && self.side_effect_class.allows_silent_apply()
            && self.has_required_undo_group()
    }

    /// Returns true when a broad or configuration action refuses silent apply.
    pub fn refuses_silent_apply_for_broad_change(&self) -> bool {
        self.is_multi_file_or_configuration_changing() && !self.silent_apply_allowed()
    }

    /// Builds the admission record consumed by editor, review, CLI, and support surfaces.
    pub fn admission(&self) -> CodeActionAdmissionRecord {
        let silent_apply_allowed = self.silent_apply_allowed();
        let refused_silent_apply_reason_refs = if silent_apply_allowed {
            Vec::new()
        } else {
            self.silent_apply_refusal_reasons()
        };

        CodeActionAdmissionRecord {
            record_kind: CodeActionAdmissionRecord::RECORD_KIND.into(),
            code_action_alpha_schema_version: CODE_ACTION_ALPHA_SCHEMA_VERSION,
            code_action_id: self.code_action_id.clone(),
            action_class: self.action_class,
            side_effect_class: self.side_effect_class,
            preview_required: self.preview_required(),
            silent_apply_allowed,
            apply_posture_class: self.apply_posture_class,
            refactor_scope_admission_class: self
                .refactor_scope_binding
                .as_ref()
                .map(|binding| binding.admission_class),
            scope_widening_review_ref: self
                .refactor_scope_binding
                .as_ref()
                .and_then(|binding| binding.scope_widening_review.as_ref())
                .map(|review| review.review_request_id.clone()),
            refused_silent_apply_reason_refs,
            undo_group_ref: self
                .undo_group
                .as_ref()
                .map(|group| group.undo_group_id.clone()),
            captured_at: self.captured_at.clone(),
            export_safe_summary: if silent_apply_allowed {
                format!(
                    "{} may apply inline with side effect {:?}.",
                    self.action_label, self.side_effect_class
                )
            } else {
                format!(
                    "{} cannot apply silently; review posture is {:?}.",
                    self.action_label, self.apply_posture_class
                )
            },
        }
    }

    fn silent_apply_refusal_reasons(&self) -> Vec<String> {
        let mut reasons = Vec::new();
        if self.apply_posture_class != ApplyPostureClass::ApplyInlineAllowed {
            reasons.push(format!("apply_posture:{:?}", self.apply_posture_class));
        }
        if self.preview_requirement_class.requires_review() {
            reasons.push(format!(
                "preview_requirement:{:?}",
                self.preview_requirement_class
            ));
        }
        if self.side_effect_class.requires_preview() {
            reasons.push(format!("side_effect:{:?}", self.side_effect_class));
        }
        if self.safety_class.requires_preview() {
            reasons.push(format!("safety:{:?}", self.safety_class));
        }
        if self.mutation_scope_class.requires_preview() {
            reasons.push(format!("scope:{:?}", self.mutation_scope_class));
        }
        if self.mutation_counts.requires_preview() {
            reasons.push("mutation_counts:preview_required".into());
        }
        if self.acting_provider.requires_preview_or_block() {
            reasons.push("provider:preview_or_block_required".into());
        }
        if self.content_integrity_review.requires_preview() {
            reasons.push("content_integrity:preview_required".into());
        }
        if let Some(binding) = self.refactor_scope_binding.as_ref() {
            if binding.requires_preview_or_review() {
                reasons.push(format!(
                    "refactor_scope:{}",
                    binding.admission_class.as_str()
                ));
            }
        }
        if !self.has_required_undo_group() {
            reasons.push("undo_group:missing_or_unattributed".into());
        }
        for reason in &self.blocking_reason_classes {
            reasons.push(format!("blocking_reason:{reason:?}"));
        }
        reasons
    }
}

/// Admission record for one action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionAdmissionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub code_action_alpha_schema_version: CodeActionAlphaSchemaVersion,
    /// Stable action id.
    pub code_action_id: String,
    /// Action family.
    pub action_class: ActionClass,
    /// Side-effect class disclosed before apply.
    pub side_effect_class: CodeActionSideEffectClass,
    /// Whether preview is required before mutation.
    pub preview_required: bool,
    /// Whether inline apply may proceed without preview.
    pub silent_apply_allowed: bool,
    /// Current apply posture.
    pub apply_posture_class: ApplyPostureClass,
    /// Refactor-scope admission class, when the action is a refactor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refactor_scope_admission_class: Option<RefactorScopeAdmissionClass>,
    /// Scope-widening review ref, when direct mutation is blocked by widening.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_widening_review_ref: Option<String>,
    /// Reasons direct apply was refused.
    pub refused_silent_apply_reason_refs: Vec<String>,
    /// Undo group reference, when present.
    pub undo_group_ref: Option<String>,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe admission summary.
    pub export_safe_summary: String,
}

impl CodeActionAdmissionRecord {
    /// Stable record-kind tag for admission records.
    pub const RECORD_KIND: &'static str = "code_action_admission_record";
}

/// Snapshot request for code-action alpha records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeActionSnapshotRequest {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Action collection id.
    pub collection_id: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// In-memory catalog for code-action alpha records.
#[derive(Debug, Clone, Default)]
pub struct CodeActionCatalog {
    actions: BTreeMap<String, CodeActionRecord>,
}

impl CodeActionCatalog {
    /// Builds an empty code-action catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes or replaces one code-action record.
    pub fn publish(&mut self, action: CodeActionRecord) {
        self.actions.insert(action.code_action_id.clone(), action);
    }

    /// Returns true when no action records are present.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Builds a deterministic snapshot of all code-action records.
    pub fn snapshot(&self, request: CodeActionSnapshotRequest) -> CodeActionAlphaSnapshot {
        let actions = self.actions.values().cloned().collect::<Vec<_>>();
        let aggregate_counts = CodeActionAlphaAggregateCounts::from_actions(&actions);
        let total_count = aggregate_counts.total_count;
        CodeActionAlphaSnapshot {
            record_kind: CodeActionAlphaSnapshot::RECORD_KIND.into(),
            code_action_alpha_schema_version: CODE_ACTION_ALPHA_SCHEMA_VERSION,
            snapshot_id: request.snapshot_id,
            workspace_id: request.workspace_id,
            collection_id: request.collection_id,
            actions,
            aggregate_counts,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            export_safe_summary: format!(
                "Code-action alpha snapshot contains {total_count} actions."
            ),
        }
    }
}

/// Aggregate counts used by compact action surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CodeActionAlphaAggregateCounts {
    /// Total action records.
    pub total_count: usize,
    /// Quick-fix action records.
    pub quick_fix_count: usize,
    /// Actions requiring preview.
    pub preview_required_count: usize,
    /// Actions allowed to apply inline.
    pub silent_apply_allowed_count: usize,
    /// Actions blocked by user, policy, or trust review.
    pub blocked_count: usize,
    /// Multi-file or configuration actions refusing silent apply.
    pub multi_file_or_configuration_refused_count: usize,
    /// Actions touching generated or protected paths.
    pub generated_or_protected_count: usize,
    /// Actions requiring content-integrity preview.
    pub content_integrity_preview_count: usize,
    /// Mutating actions with named undo groups.
    pub named_undo_group_count: usize,
    /// Actions that disclose a side-effect class.
    pub side_effect_disclosure_count: usize,
}

impl CodeActionAlphaAggregateCounts {
    /// Builds aggregate counts from code-action records.
    pub fn from_actions(actions: &[CodeActionRecord]) -> Self {
        let mut counts = Self {
            total_count: actions.len(),
            ..Self::default()
        };

        for action in actions {
            if action.action_class.is_quick_fix() {
                counts.quick_fix_count += 1;
            }
            if action.preview_required() {
                counts.preview_required_count += 1;
            }
            if action.silent_apply_allowed() {
                counts.silent_apply_allowed_count += 1;
            }
            if action.apply_posture_class.is_blocked() {
                counts.blocked_count += 1;
            }
            if action.refuses_silent_apply_for_broad_change() {
                counts.multi_file_or_configuration_refused_count += 1;
            }
            if action.has_generated_or_protected_impact() {
                counts.generated_or_protected_count += 1;
            }
            if action.content_integrity_review.requires_preview() {
                counts.content_integrity_preview_count += 1;
            }
            if action.is_mutation_bearing() && action.has_named_undo_group() {
                counts.named_undo_group_count += 1;
            }
            counts.side_effect_disclosure_count += 1;
        }

        counts
    }
}

/// Snapshot of all code actions in one collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionAlphaSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub code_action_alpha_schema_version: CodeActionAlphaSchemaVersion,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Action collection id.
    pub collection_id: String,
    /// Action records in deterministic id order.
    pub actions: Vec<CodeActionRecord>,
    /// Aggregate counts for compact surfaces.
    pub aggregate_counts: CodeActionAlphaAggregateCounts,
    /// Redaction posture for the snapshot.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CodeActionAlphaSnapshot {
    /// Stable record-kind tag for code-action snapshots.
    pub const RECORD_KIND: &'static str = "code_action_alpha_snapshot_record";

    /// Returns true when any action needs preview, degraded, or side-effect disclosure.
    pub fn disclosure_required(&self) -> bool {
        self.actions.iter().any(|action| {
            action.preview_required()
                || !action.silent_apply_allowed()
                || action.acting_provider.requires_preview_or_block()
        })
    }

    /// Builds a surface-specific projection from this snapshot.
    pub fn surface_projection(
        &self,
        surface_class: CodeActionSurfaceClass,
        captured_at: impl Into<String>,
    ) -> CodeActionSurfaceProjection {
        let included_action_ids = self
            .actions
            .iter()
            .map(|action| action.code_action_id.clone())
            .collect::<Vec<_>>();
        let inline_apply_action_ids = self
            .actions
            .iter()
            .filter(|action| action.silent_apply_allowed())
            .map(|action| action.code_action_id.clone())
            .collect::<Vec<_>>();
        let preview_required_action_ids = self
            .actions
            .iter()
            .filter(|action| action.preview_required())
            .map(|action| action.code_action_id.clone())
            .collect::<Vec<_>>();
        let blocked_action_ids = self
            .actions
            .iter()
            .filter(|action| action.apply_posture_class.is_blocked())
            .map(|action| action.code_action_id.clone())
            .collect::<Vec<_>>();
        let undo_group_refs = self
            .actions
            .iter()
            .filter_map(|action| action.undo_group.as_ref())
            .map(|group| group.undo_group_id.clone())
            .collect::<Vec<_>>();

        CodeActionSurfaceProjection {
            record_kind: CodeActionSurfaceProjection::RECORD_KIND.into(),
            code_action_alpha_schema_version: CODE_ACTION_ALPHA_SCHEMA_VERSION,
            projection_id: format!(
                "code_action_projection:{}:{}",
                surface_class.as_str(),
                sanitize_id(&self.snapshot_id)
            ),
            snapshot_id: self.snapshot_id.clone(),
            surface_class,
            included_action_ids,
            inline_apply_action_ids,
            preview_required_action_ids,
            blocked_action_ids,
            undo_group_refs,
            disclosure_required: self.disclosure_required(),
            visible_count: self.actions.len(),
            captured_at: captured_at.into(),
            export_safe_summary: format!(
                "{} actions projected for {}.",
                self.actions.len(),
                surface_class.as_str()
            ),
        }
    }
}

/// Surface-specific projection over a code-action snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub code_action_alpha_schema_version: CodeActionAlphaSchemaVersion,
    /// Stable projection id.
    pub projection_id: String,
    /// Source snapshot id.
    pub snapshot_id: String,
    /// Surface consuming the projection.
    pub surface_class: CodeActionSurfaceClass,
    /// Action ids included in the projection.
    pub included_action_ids: Vec<String>,
    /// Action ids allowed to apply inline.
    pub inline_apply_action_ids: Vec<String>,
    /// Action ids requiring preview or review.
    pub preview_required_action_ids: Vec<String>,
    /// Action ids blocked pending user, policy, or trust review.
    pub blocked_action_ids: Vec<String>,
    /// Undo group refs attached to mutating actions.
    pub undo_group_refs: Vec<String>,
    /// Whether the surface must show side-effect or preview disclosure.
    pub disclosure_required: bool,
    /// Count visible in the surface's primary row set.
    pub visible_count: usize,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CodeActionSurfaceProjection {
    /// Stable record-kind tag for surface projections.
    pub const RECORD_KIND: &'static str = "code_action_surface_projection_record";
}

fn candidate_inside_workset(
    workset_scope: &WorksetScopeDescriptor,
    candidate: &RefactorScopeCandidate,
) -> bool {
    let scope_ref = candidate.scope_ref.trim();
    if scope_ref.is_empty() {
        return false;
    }
    if scope_ref == workset_scope.scope_id {
        return true;
    }
    if workset_scope
        .included_roots_or_repos
        .iter()
        .any(|included| included == scope_ref)
    {
        return true;
    }
    matches!(workset_scope.scope_mode, WorksetScopeMode::Full)
        && workset_scope.included_roots_or_repos.is_empty()
        && workset_scope.hidden_result_count == 0
        && workset_scope.index_coverage.not_loaded_count == 0
}

fn scope_widening_review(
    workset_scope: &WorksetScopeDescriptor,
    target_scope_rows: &[RefactorScopeTargetRow],
    refused_target_refs: &[String],
    captured_at: &str,
) -> CodeActionScopeWideningReview {
    let requested_scope_refs = target_scope_rows
        .iter()
        .filter(|row| !row.inside_named_workset)
        .map(|row| row.scope_ref.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let suffix = refused_target_refs
        .first()
        .map(|target_ref| sanitize_id(target_ref))
        .unwrap_or_else(|| "no_target".to_owned());

    CodeActionScopeWideningReview {
        record_kind: CodeActionScopeWideningReview::RECORD_KIND.into(),
        review_request_id: format!(
            "scope_widening_review:{}:{}",
            sanitize_id(&workset_scope.scope_id),
            suffix
        ),
        trigger_class: ScopeWideningReviewTriggerClass::RefactorWiden,
        active_scope_id: workset_scope.scope_id.clone(),
        active_scope_class: workset_scope.scope_class.clone(),
        active_scope_mode: workset_scope.scope_mode,
        requested_scope_refs,
        blocked_target_refs: refused_target_refs.to_vec(),
        hidden_result_count: workset_scope.hidden_result_count,
        typed_confirmation_required: true,
        captured_at: captured_at.to_owned(),
        export_safe_summary: format!(
            "Refactor scope widening from {} requires explicit review.",
            workset_scope.scope_id
        ),
    }
}

fn support_from_diagnostic_source(source: &DiagnosticSourceDescriptor) -> CodeActionSupportClass {
    match source.source_family {
        DiagnosticSourceFamily::LinterFormatterStyle
        | DiagnosticSourceFamily::CompilerOrBuild
        | DiagnosticSourceFamily::RuntimeTestOrDebug => CodeActionSupportClass::ExternalTooling,
        DiagnosticSourceFamily::PolicyTrustOrSecurity => CodeActionSupportClass::PolicyGoverned,
        _ if source.support_class == RouterSupportClass::Authoritative => {
            CodeActionSupportClass::FirstPartySupported
        }
        _ => CodeActionSupportClass::FirstPartyBestEffort,
    }
}

fn support_from_router_provider(
    provider_kind: ProviderKind,
    support_class: RouterSupportClass,
) -> CodeActionSupportClass {
    match provider_kind {
        ProviderKind::FormatterAdapter
        | ProviderKind::LinterAdapter
        | ProviderKind::BuildAdapter
        | ProviderKind::DebugAdapter
        | ProviderKind::TestAdapter => CodeActionSupportClass::ExternalTooling,
        _ if support_class == RouterSupportClass::Authoritative => {
            CodeActionSupportClass::FirstPartySupported
        }
        _ => CodeActionSupportClass::FirstPartyBestEffort,
    }
}

fn provider_family_from_source(source_family: DiagnosticSourceFamily) -> Option<ProviderFamily> {
    match source_family {
        DiagnosticSourceFamily::LanguageServer => Some(ProviderFamily::LanguageServer),
        DiagnosticSourceFamily::FrameworkOrSchemaAnalyzer => Some(ProviderFamily::FrameworkPack),
        _ => None,
    }
}

fn provider_family_from_kind(provider_kind: ProviderKind) -> Option<ProviderFamily> {
    match provider_kind {
        ProviderKind::LanguageServer => Some(ProviderFamily::LanguageServer),
        ProviderKind::FrameworkPack | ProviderKind::NativeAnalyzer => {
            Some(ProviderFamily::FrameworkPack)
        }
        ProviderKind::GeneratedSourceBridge => Some(ProviderFamily::GeneratedSourceBridge),
        ProviderKind::ProjectGraph => Some(ProviderFamily::ProjectGraph),
        ProviderKind::AiAssist => Some(ProviderFamily::AiAssist),
        _ => None,
    }
}

fn freshness_from_router(
    freshness_class: crate::lsp_router::FreshnessClass,
) -> CodeActionFreshnessClass {
    match freshness_class {
        crate::lsp_router::FreshnessClass::AuthoritativeLive => CodeActionFreshnessClass::Current,
        crate::lsp_router::FreshnessClass::WarmCached => CodeActionFreshnessClass::Recent,
        crate::lsp_router::FreshnessClass::DegradedCached
        | crate::lsp_router::FreshnessClass::Stale
        | crate::lsp_router::FreshnessClass::Unverified => CodeActionFreshnessClass::Stale,
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
