//! Workset-scope beta truth.
//!
//! The beta layer hardens the [`WorksetArtifactRecord`] foundation so search,
//! graph, refactor, AI, and export surfaces consume one closed vocabulary that:
//!
//! 1. Names every included root and every root that is *not* in scope, with a
//!    typed reason instead of a silent omission.
//! 2. Narrows or blocks unsafe broad actions (`refactor_apply`, `ai_apply`,
//!    `export_artifact`, `support_archive`) whenever the active scope is
//!    partial, policy-limited, managed-provider locked, or has unresolved
//!    membership.
//! 3. Carries the scope lineage chain a support / export consumer needs in
//!    order to replay the same scope instead of flattening it into a
//!    workspace-wide answer.
//!
//! The runtime projection in
//! `schemas/runtime/execution_context.schema.json` still owns the live
//! activation contract; this module owns the durable, replayable, exportable
//! scope-truth a beta search, graph, refactor, AI, or export surface reads
//! when it decides whether to allow, narrow, or block a broad action.

use serde::{Deserialize, Serialize};

use super::{
    IncludedRootRef, NarrowingCause, PartialTruthLabel, PatternEntry, PatternKind,
    PortabilityClass, ReadinessState, ScopeClass, ScopeMode, WorksetArtifactRecord,
};

/// Schema version for the beta workset-scope truth payload shape.
pub const WORKSET_SCOPE_BETA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator carried in the serialized beta payload.
pub const WORKSET_SCOPE_BETA_TRUTH_RECORD_KIND: &str = "workset_scope_beta_truth";
/// Record-kind discriminator carried in the serialized support-export payload.
pub const WORKSET_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str = "workset_scope_beta_support_export";

/// Consumer surface that activates a [`WorksetScopeBetaTruth`].
///
/// Every beta consumer that reads scope reads one of these tokens; a surface
/// that quotes its own private string is non-conforming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaConsumerSurface {
    /// Search lane (lexical/semantic/structural).
    Search,
    /// Graph lane (call graph, dependency graph, reference graph).
    Graph,
    /// Refactor lane (rename, move, extract, etc.).
    Refactor,
    /// AI apply lane (AI-authored edit, AI-context excerpt selection).
    Ai,
    /// Export lane (export-bundle writer, archive-on-disk).
    Export,
    /// Support-export lane (support-packet header / triage replay).
    SupportPacket,
}

impl BetaConsumerSurface {
    /// Stable string vocabulary used in records, fixtures, and shell logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Refactor => "refactor",
            Self::Ai => "ai",
            Self::Export => "export",
            Self::SupportPacket => "support_packet",
        }
    }

    /// Returns true when this surface is a "broad mutation/export" lane that
    /// MUST narrow or block on partial / policy-limited scope.
    pub const fn is_broad_mutation(self) -> bool {
        matches!(self, Self::Refactor | Self::Ai | Self::Export)
    }
}

/// Reason a candidate root is *not* part of the active beta scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExcludedRootReason {
    /// A user-authored exclude pattern removed this root.
    ExcludedByPattern,
    /// A workspace root that the active workset did not include.
    NotInWorksetRootList,
    /// A policy overlay hides this root from the active view.
    PolicyHidden,
    /// The root materialization is unavailable (remote down, managed
    /// provider unreachable, container detached).
    Unavailable,
}

impl ExcludedRootReason {
    /// Stable string vocabulary for the exclusion reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExcludedByPattern => "excluded_by_pattern",
            Self::NotInWorksetRootList => "not_in_workset_root_list",
            Self::PolicyHidden => "policy_hidden",
            Self::Unavailable => "unavailable",
        }
    }
}

/// One excluded root with the reason for the exclusion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludedRootEntry {
    pub root_ref: String,
    pub reason: ExcludedRootReason,
    /// Optional pointer back to the pattern that excluded this root.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Optional presentation label so support reviewers can re-establish
    /// the human meaning of the excluded root.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Closed vocabulary of beta broad-action classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadActionClass {
    /// Lexical/structural search query — never destructive, but may
    /// declare an outside-scope row.
    SearchQuery,
    /// Graph traversal across roots — never destructive, but may declare
    /// an outside-scope edge.
    GraphTraversal,
    /// Refactor application across multiple members.
    RefactorApply,
    /// AI-authored edit application or AI-context excerpt selection.
    AiApply,
    /// Export archive writer.
    ExportArtifact,
    /// Support archive writer.
    SupportArchive,
}

impl BroadActionClass {
    /// Stable string vocabulary for the broad-action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchQuery => "search_query",
            Self::GraphTraversal => "graph_traversal",
            Self::RefactorApply => "refactor_apply",
            Self::AiApply => "ai_apply",
            Self::ExportArtifact => "export_artifact",
            Self::SupportArchive => "support_archive",
        }
    }

    /// Returns the full ordered vocabulary used by [`WorksetScopeBetaTruth`].
    pub const fn all() -> [BroadActionClass; 6] {
        [
            Self::SearchQuery,
            Self::GraphTraversal,
            Self::RefactorApply,
            Self::AiApply,
            Self::ExportArtifact,
            Self::SupportArchive,
        ]
    }

    /// Returns true when this action class is unsafe under partial scope.
    pub const fn is_destructive_or_exfiltrating(self) -> bool {
        matches!(
            self,
            Self::RefactorApply | Self::AiApply | Self::ExportArtifact | Self::SupportArchive
        )
    }
}

/// Decision the beta gate emitted for a broad-action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadActionDecision {
    /// The action is allowed against the active scope without narrowing.
    Allowed,
    /// The action is allowed only after narrowing to the current scope's
    /// declared roots / patterns.
    NarrowedToScope,
    /// The action is blocked by a policy overlay (admin, license, trust).
    BlockedByPolicy,
    /// The action is blocked by the artifact's portability class (managed
    /// provider locked, ephemeral session).
    BlockedByPortability,
    /// The action is blocked because the scope is partial / not yet ready
    /// and the action cannot be replayed truthfully.
    BlockedBySparsePartial,
    /// The action is blocked because the artifact references an outside
    /// root that is not part of the active scope.
    BlockedByOutsideScope,
}

impl BroadActionDecision {
    /// Stable string vocabulary for the broad-action decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::NarrowedToScope => "narrowed_to_scope",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByPortability => "blocked_by_portability",
            Self::BlockedBySparsePartial => "blocked_by_sparse_partial",
            Self::BlockedByOutsideScope => "blocked_by_outside_scope",
        }
    }

    /// Returns true when the decision is a block (action denied entirely).
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedByPolicy
                | Self::BlockedByPortability
                | Self::BlockedBySparsePartial
                | Self::BlockedByOutsideScope
        )
    }

    /// Returns true when the decision restricts the action without
    /// blocking it (still requires a reason).
    pub const fn is_narrowed(self) -> bool {
        matches!(self, Self::NarrowedToScope)
    }
}

/// Typed reason explaining a [`BroadActionDecision`] other than `Allowed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadActionReason {
    /// Action narrowed to the active workset's roots / patterns.
    HonorActiveWorksetRoots,
    /// Action narrowed because the artifact is in sparse mode.
    SparseSliceNarrowing,
    /// Action narrowed because readiness is below `Ready`.
    ReadinessBelowReady,
    /// Action narrowed because the index is still warming.
    IndexWarming,
    /// Blocked by admin-policy overlay.
    AdminPolicyBlocked,
    /// Blocked by trust-policy overlay.
    TrustPolicyBlocked,
    /// Blocked by license / export-control overlay.
    LicenseOrExportControlBlocked,
    /// Blocked because the managed provider locks the artifact.
    ManagedProviderLocked,
    /// Blocked because the session is ephemeral and may not persist
    /// outside this run.
    EphemeralSession,
    /// Blocked because the artifact references a missing or hidden member.
    PartialMembershipMissing,
    /// Blocked because the action would have escaped the active workset's
    /// declared roots.
    OutsideWorksetRoots,
}

impl BroadActionReason {
    /// Stable string vocabulary for the broad-action reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HonorActiveWorksetRoots => "honor_active_workset_roots",
            Self::SparseSliceNarrowing => "sparse_slice_narrowing",
            Self::ReadinessBelowReady => "readiness_below_ready",
            Self::IndexWarming => "index_warming",
            Self::AdminPolicyBlocked => "admin_policy_blocked",
            Self::TrustPolicyBlocked => "trust_policy_blocked",
            Self::LicenseOrExportControlBlocked => "license_or_export_control_blocked",
            Self::ManagedProviderLocked => "managed_provider_locked",
            Self::EphemeralSession => "ephemeral_session",
            Self::PartialMembershipMissing => "partial_membership_missing",
            Self::OutsideWorksetRoots => "outside_workset_roots",
        }
    }
}

/// Admission row for one broad-action class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BroadActionAdmission {
    pub action_class: BroadActionClass,
    pub decision: BroadActionDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<BroadActionReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain_note: Option<String>,
}

/// One ancestor entry in the scope lineage chain.
///
/// The lineage starts with the artifact the truth describes and walks
/// upward through `parent_workset_ref` / `policy_limitation
/// .underlying_workset_ref` so an export reviewer can re-establish the
/// pre-policy and pre-derivation scope without re-deriving it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeLineageEntry {
    pub workset_ref: String,
    pub scope_class: ScopeClass,
    pub scope_mode: ScopeMode,
    pub readiness_state: ReadinessState,
    pub portability_class: PortabilityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_cause: Option<NarrowingCause>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Inputs that callers (search, graph, refactor, AI, export, support)
/// hand to the projection so the beta truth reflects the actual scope
/// observation, not just the saved artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeObservationInputs<'a> {
    /// The workspace's declared roots, used to compute the `excluded_roots`
    /// list for scope classes that narrow below the workspace.
    pub workspace_root_refs: &'a [String],
    /// Optional workspace-side presentation labels keyed by root_ref.
    pub workspace_root_labels: &'a [(String, String)],
    /// Optional parent artifact for `policy_limited_view` so the lineage
    /// chain can quote the underlying scope class / portability without
    /// re-deriving it from a side channel.
    pub parent_artifact: Option<&'a WorksetArtifactRecord>,
}

impl ScopeObservationInputs<'_> {
    /// Empty observation (single-artifact lineage, workspace roots unknown).
    pub const fn empty() -> Self {
        Self {
            workspace_root_refs: &[],
            workspace_root_labels: &[],
            parent_artifact: None,
        }
    }
}

/// Errors detected while validating a [`WorksetScopeBetaTruth`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetScopeBetaError {
    SchemaVersionMismatch(u32),
    EmptyStableScopeId,
    EmptyWorksetRef,
    EmptyWorksetName,
    EmptyIncludedRoots,
    EmptyLineage,
    LineageRootMismatch,
    PolicyLimitedLineageMissingUnderlying,
    BroadActionVocabularyIncomplete(BroadActionClass),
    BroadActionVocabularyDuplicate(BroadActionClass),
    BlockedActionMissingReason(BroadActionClass),
    NarrowedActionMissingReason(BroadActionClass),
    AllowedActionCarriesReason(BroadActionClass),
    BroadActionMustNarrowOrBlockOnPartial(BroadActionClass),
    BroadActionMustBlockOnPolicyLimited(BroadActionClass),
    ManagedProviderLockedMustBlockExport,
    EphemeralMustBlockSupportArchive,
    DuplicateExcludedRoot(String),
    ExcludedRootInIncludedRoots(String),
    OutsideMarkerMissingNote,
}

impl std::fmt::Display for WorksetScopeBetaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(v) => write!(
                f,
                "unsupported workset_scope_beta_schema_version {v}; this layer accepts 1"
            ),
            Self::EmptyStableScopeId => write!(f, "stable_scope_id must not be empty"),
            Self::EmptyWorksetRef => write!(f, "workset_ref must not be empty"),
            Self::EmptyWorksetName => write!(f, "workset_name must not be empty"),
            Self::EmptyIncludedRoots => write!(f, "included_roots must contain at least one root"),
            Self::EmptyLineage => write!(f, "lineage must include the active artifact"),
            Self::LineageRootMismatch => write!(
                f,
                "lineage[0] must reference the active artifact's workset_ref"
            ),
            Self::PolicyLimitedLineageMissingUnderlying => write!(
                f,
                "policy_limited_view truth must include the underlying workset as a lineage ancestor"
            ),
            Self::BroadActionVocabularyIncomplete(action) => write!(
                f,
                "admissions must include broad action {}",
                action.as_str()
            ),
            Self::BroadActionVocabularyDuplicate(action) => write!(
                f,
                "admissions must include broad action {} exactly once",
                action.as_str()
            ),
            Self::BlockedActionMissingReason(action) => write!(
                f,
                "blocked admission for {} must carry a typed reason",
                action.as_str()
            ),
            Self::NarrowedActionMissingReason(action) => write!(
                f,
                "narrowed admission for {} must carry a typed reason",
                action.as_str()
            ),
            Self::AllowedActionCarriesReason(action) => write!(
                f,
                "allowed admission for {} must not carry a typed reason",
                action.as_str()
            ),
            Self::BroadActionMustNarrowOrBlockOnPartial(action) => write!(
                f,
                "broad action {} must narrow or block when the scope is partial",
                action.as_str()
            ),
            Self::BroadActionMustBlockOnPolicyLimited(action) => write!(
                f,
                "broad action {} must block when the scope is policy-limited",
                action.as_str()
            ),
            Self::ManagedProviderLockedMustBlockExport => write!(
                f,
                "managed_provider_locked artifacts must block export_artifact admissions"
            ),
            Self::EphemeralMustBlockSupportArchive => write!(
                f,
                "ephemeral_session artifacts must block support_archive admissions"
            ),
            Self::DuplicateExcludedRoot(root) => {
                write!(f, "duplicate excluded_root: {root}")
            }
            Self::ExcludedRootInIncludedRoots(root) => write!(
                f,
                "root {root} cannot appear in both included_roots and excluded_roots"
            ),
            Self::OutsideMarkerMissingNote => write!(
                f,
                "outside_current_scope_marker_visible truths must carry an explain note"
            ),
        }
    }
}

impl std::error::Error for WorksetScopeBetaError {}

/// Beta scope-truth record consumed by search, graph, refactor, AI, export,
/// and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeBetaTruth {
    pub record_kind: String,
    pub schema_version: u32,
    pub stable_scope_id: String,
    pub workset_ref: String,
    pub workset_name: String,
    pub scope_class: ScopeClass,
    pub scope_mode: ScopeMode,
    pub consumer_surface: BetaConsumerSurface,
    pub included_roots: Vec<IncludedRootRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_roots: Vec<ExcludedRootEntry>,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub admissions: Vec<BroadActionAdmission>,
    pub lineage: Vec<ScopeLineageEntry>,
    pub portability_lineage_preserved: bool,
    pub outside_current_scope_marker_visible: bool,
    pub emitted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain_note: Option<String>,
}

impl WorksetScopeBetaTruth {
    /// Validates the beta truth against the closed invariants.
    pub fn validate(&self) -> Result<(), WorksetScopeBetaError> {
        if self.schema_version != WORKSET_SCOPE_BETA_SCHEMA_VERSION {
            return Err(WorksetScopeBetaError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        if self.stable_scope_id.is_empty() {
            return Err(WorksetScopeBetaError::EmptyStableScopeId);
        }
        if self.workset_ref.is_empty() {
            return Err(WorksetScopeBetaError::EmptyWorksetRef);
        }
        if self.workset_name.is_empty() {
            return Err(WorksetScopeBetaError::EmptyWorksetName);
        }
        if self.included_roots.is_empty() {
            return Err(WorksetScopeBetaError::EmptyIncludedRoots);
        }
        let mut seen_excluded: Vec<&str> = Vec::with_capacity(self.excluded_roots.len());
        for excl in &self.excluded_roots {
            if seen_excluded.iter().any(|r| *r == excl.root_ref.as_str()) {
                return Err(WorksetScopeBetaError::DuplicateExcludedRoot(
                    excl.root_ref.clone(),
                ));
            }
            seen_excluded.push(excl.root_ref.as_str());
            if self
                .included_roots
                .iter()
                .any(|incl| incl.root_ref == excl.root_ref)
            {
                return Err(WorksetScopeBetaError::ExcludedRootInIncludedRoots(
                    excl.root_ref.clone(),
                ));
            }
        }
        if self.lineage.is_empty() {
            return Err(WorksetScopeBetaError::EmptyLineage);
        }
        if self.lineage[0].workset_ref != self.workset_ref {
            return Err(WorksetScopeBetaError::LineageRootMismatch);
        }
        if self.scope_class == ScopeClass::PolicyLimitedView && self.lineage.len() < 2 {
            return Err(WorksetScopeBetaError::PolicyLimitedLineageMissingUnderlying);
        }
        for required in BroadActionClass::all() {
            let count = self
                .admissions
                .iter()
                .filter(|a| a.action_class == required)
                .count();
            if count == 0 {
                return Err(WorksetScopeBetaError::BroadActionVocabularyIncomplete(
                    required,
                ));
            }
            if count > 1 {
                return Err(WorksetScopeBetaError::BroadActionVocabularyDuplicate(
                    required,
                ));
            }
        }
        for admission in &self.admissions {
            match (admission.decision, &admission.reason) {
                (BroadActionDecision::Allowed, Some(_)) => {
                    return Err(WorksetScopeBetaError::AllowedActionCarriesReason(
                        admission.action_class,
                    ));
                }
                (decision, None) if decision.is_blocked() => {
                    return Err(WorksetScopeBetaError::BlockedActionMissingReason(
                        admission.action_class,
                    ));
                }
                (decision, None) if decision.is_narrowed() => {
                    return Err(WorksetScopeBetaError::NarrowedActionMissingReason(
                        admission.action_class,
                    ));
                }
                _ => {}
            }
        }

        let partial = self.is_partial_scope();
        let active_lineage = &self.lineage[0];
        let policy_limited = active_lineage.scope_class == ScopeClass::PolicyLimitedView;
        for admission in &self.admissions {
            if !admission.action_class.is_destructive_or_exfiltrating() {
                continue;
            }
            if policy_limited && admission.decision == BroadActionDecision::Allowed {
                return Err(WorksetScopeBetaError::BroadActionMustBlockOnPolicyLimited(
                    admission.action_class,
                ));
            }
            if partial && admission.decision == BroadActionDecision::Allowed {
                return Err(
                    WorksetScopeBetaError::BroadActionMustNarrowOrBlockOnPartial(
                        admission.action_class,
                    ),
                );
            }
            if admission.action_class == BroadActionClass::ExportArtifact
                && active_lineage.portability_class == PortabilityClass::ManagedProviderLocked
                && admission.decision != BroadActionDecision::BlockedByPortability
            {
                return Err(WorksetScopeBetaError::ManagedProviderLockedMustBlockExport);
            }
            if admission.action_class == BroadActionClass::SupportArchive {
                // Lineage roots can quote any source class on the active
                // artifact; we look at the first lineage entry for its
                // portability label and consider an ephemeral session via
                // its narrowing cause + portability_class combination.
                if active_lineage.portability_class == PortabilityClass::MachineLocalOnly
                    && admission.decision == BroadActionDecision::Allowed
                    && active_lineage.readiness_state != ReadinessState::Ready
                {
                    return Err(WorksetScopeBetaError::EphemeralMustBlockSupportArchive);
                }
            }
        }

        if self.outside_current_scope_marker_visible && self.explain_note.is_none() {
            return Err(WorksetScopeBetaError::OutsideMarkerMissingNote);
        }
        Ok(())
    }

    /// Returns true when the scope is narrower than the workspace OR the
    /// artifact's readiness is below `Ready` OR any root carries a
    /// non-loaded partial-truth label.
    pub fn is_partial_scope(&self) -> bool {
        let active = &self.lineage[0];
        let class_is_narrowed = active.scope_class.is_narrowed();
        let readiness_partial = !matches!(active.readiness_state, ReadinessState::Ready);
        let any_member_partial = self
            .included_roots
            .iter()
            .any(|root| root.partial_truth != PartialTruthLabel::Loaded);
        class_is_narrowed || readiness_partial || any_member_partial
    }

    /// Returns the admission for a given broad-action class, if any.
    pub fn admission_for(&self, action: BroadActionClass) -> Option<&BroadActionAdmission> {
        self.admissions.iter().find(|a| a.action_class == action)
    }
}

impl WorksetArtifactRecord {
    /// Projects a [`WorksetScopeBetaTruth`] for one consumer surface.
    ///
    /// The projection derives every field from the artifact + observation
    /// inputs; callers do not author chip labels, admission decisions, or
    /// lineage chains.
    pub fn project_beta_truth(
        &self,
        consumer_surface: BetaConsumerSurface,
        observation: ScopeObservationInputs<'_>,
        emitted_at: impl Into<String>,
    ) -> WorksetScopeBetaTruth {
        let emitted_at = emitted_at.into();
        let included_roots = self.included_roots.clone();
        let include_patterns: Vec<String> = self
            .patterns
            .iter()
            .filter(|p| p.pattern_kind == PatternKind::Include)
            .map(pattern_token)
            .collect();
        let exclude_patterns: Vec<String> = self
            .patterns
            .iter()
            .filter(|p| p.pattern_kind == PatternKind::Exclude)
            .map(pattern_token)
            .collect();
        let excluded_roots = derive_excluded_roots(
            self,
            observation.workspace_root_refs,
            observation.workspace_root_labels,
        );
        let lineage = derive_lineage(self, observation.parent_artifact);
        let admissions = derive_admissions(self);
        let portability_lineage_preserved = lineage.iter().all(|entry| {
            entry.portability_class != PortabilityClass::FullyPortable
                || entry.scope_mode != ScopeMode::Sparse
                || entry.readiness_state == ReadinessState::Ready
        }) || lineage.first().is_some_and(|entry| {
            matches!(
                entry.portability_class,
                PortabilityClass::ManagedProviderLocked | PortabilityClass::PortableWithRebinding
            )
        });
        WorksetScopeBetaTruth {
            record_kind: WORKSET_SCOPE_BETA_TRUTH_RECORD_KIND.to_string(),
            schema_version: WORKSET_SCOPE_BETA_SCHEMA_VERSION,
            stable_scope_id: self.stable_scope_id().to_string(),
            workset_ref: self.workset_id.clone(),
            workset_name: self.workset_name.clone(),
            scope_class: self.scope_class,
            scope_mode: self.scope_mode,
            consumer_surface,
            included_roots,
            excluded_roots,
            include_patterns,
            exclude_patterns,
            admissions,
            lineage,
            portability_lineage_preserved,
            outside_current_scope_marker_visible: false,
            emitted_at,
            explain_note: None,
        }
    }

    /// Projects an outside-current-scope truth for a row whose owning root is
    /// not part of the active scope. The truth carries an explain note so
    /// the chrome and support consumers preserve the cue.
    pub fn project_beta_truth_outside_scope(
        &self,
        consumer_surface: BetaConsumerSurface,
        observation: ScopeObservationInputs<'_>,
        outside_root_ref: impl Into<String>,
        explain_note: impl Into<String>,
        emitted_at: impl Into<String>,
    ) -> WorksetScopeBetaTruth {
        let mut truth = self.project_beta_truth(consumer_surface, observation, emitted_at);
        truth.outside_current_scope_marker_visible = true;
        let note = explain_note.into();
        let outside_root = outside_root_ref.into();
        truth.explain_note = Some(format!(
            "Row outside current scope (root {outside_root}): {note}"
        ));
        for admission in truth.admissions.iter_mut() {
            if admission.action_class.is_destructive_or_exfiltrating() {
                admission.decision = BroadActionDecision::BlockedByOutsideScope;
                admission.reason = Some(BroadActionReason::OutsideWorksetRoots);
                admission.explain_note = Some(format!(
                    "Cannot widen {action} across root {outside_root} without an explicit scope-widen review.",
                    action = admission.action_class.as_str()
                ));
            }
        }
        truth
    }
}

fn pattern_token(pattern: &PatternEntry) -> String {
    match pattern.applies_to_root_ref.as_deref() {
        Some(root) => format!("{root}::{}", pattern.pattern),
        None => pattern.pattern.clone(),
    }
}

fn derive_lineage(
    artifact: &WorksetArtifactRecord,
    parent: Option<&WorksetArtifactRecord>,
) -> Vec<ScopeLineageEntry> {
    let mut lineage: Vec<ScopeLineageEntry> = Vec::new();
    lineage.push(ScopeLineageEntry {
        workset_ref: artifact.workset_id.clone(),
        scope_class: artifact.scope_class,
        scope_mode: artifact.scope_mode,
        readiness_state: artifact.readiness.readiness_state,
        portability_class: artifact.portability.portability_class,
        narrowing_cause: artifact
            .policy_limitation
            .as_ref()
            .map(|p| p.narrowing_cause),
        presentation_label: Some(artifact.workset_name.clone()),
    });
    if let Some(parent) = parent {
        lineage.push(ScopeLineageEntry {
            workset_ref: parent.workset_id.clone(),
            scope_class: parent.scope_class,
            scope_mode: parent.scope_mode,
            readiness_state: parent.readiness.readiness_state,
            portability_class: parent.portability.portability_class,
            narrowing_cause: parent.policy_limitation.as_ref().map(|p| p.narrowing_cause),
            presentation_label: Some(parent.workset_name.clone()),
        });
    } else if let Some(policy) = artifact.policy_limitation.as_ref() {
        // Policy-limited views without a separately fetched parent artifact
        // still preserve the underlying workset ref as a lineage stub so
        // support consumers can quote the pre-narrowing identity.
        lineage.push(ScopeLineageEntry {
            workset_ref: policy.underlying_workset_ref.clone(),
            scope_class: ScopeClass::SelectedWorkset,
            scope_mode: ScopeMode::Sparse,
            readiness_state: artifact.readiness.readiness_state,
            portability_class: artifact.portability.portability_class,
            narrowing_cause: Some(policy.narrowing_cause),
            presentation_label: None,
        });
    } else if let Some(parent_ref) = artifact.parent_workset_ref.as_ref() {
        lineage.push(ScopeLineageEntry {
            workset_ref: parent_ref.clone(),
            scope_class: ScopeClass::SelectedWorkset,
            scope_mode: artifact.scope_mode,
            readiness_state: artifact.readiness.readiness_state,
            portability_class: artifact.portability.portability_class,
            narrowing_cause: None,
            presentation_label: None,
        });
    }
    lineage
}

fn derive_excluded_roots(
    artifact: &WorksetArtifactRecord,
    workspace_root_refs: &[String],
    workspace_root_labels: &[(String, String)],
) -> Vec<ExcludedRootEntry> {
    let mut entries: Vec<ExcludedRootEntry> = Vec::new();
    let lookup_label = |root_ref: &str| -> Option<String> {
        workspace_root_labels
            .iter()
            .find(|(r, _)| r == root_ref)
            .map(|(_, label)| label.clone())
    };
    for ws_root in workspace_root_refs {
        if artifact.contains_root_ref(ws_root) {
            continue;
        }
        entries.push(ExcludedRootEntry {
            root_ref: ws_root.clone(),
            reason: ExcludedRootReason::NotInWorksetRootList,
            pattern: None,
            presentation_label: lookup_label(ws_root),
        });
    }
    for pattern in &artifact.patterns {
        if pattern.pattern_kind != PatternKind::Exclude {
            continue;
        }
        let Some(scoped_root) = pattern.applies_to_root_ref.as_deref() else {
            continue;
        };
        if artifact.contains_root_ref(scoped_root)
            && !entries.iter().any(|e| e.root_ref == scoped_root)
        {
            entries.push(ExcludedRootEntry {
                root_ref: scoped_root.to_string(),
                reason: ExcludedRootReason::ExcludedByPattern,
                pattern: Some(pattern.pattern.clone()),
                presentation_label: artifact
                    .included_roots
                    .iter()
                    .find(|r| r.root_ref == scoped_root)
                    .and_then(|r| r.presentation_label.clone()),
            });
        }
    }
    if let Some(policy) = artifact.policy_limitation.as_ref() {
        if policy.hidden_member_count > 0
            && !entries
                .iter()
                .any(|e| e.reason == ExcludedRootReason::PolicyHidden)
        {
            entries.push(ExcludedRootEntry {
                root_ref: format!(
                    "policy:{policy_ref}:hidden_members",
                    policy_ref = policy.policy_ref
                ),
                reason: ExcludedRootReason::PolicyHidden,
                pattern: None,
                presentation_label: Some(format!(
                    "{} member(s) hidden by policy",
                    policy.hidden_member_count
                )),
            });
        }
    }
    for included in &artifact.included_roots {
        if included.partial_truth == PartialTruthLabel::Unavailable
            && !entries.iter().any(|e| e.root_ref == included.root_ref)
        {
            entries.push(ExcludedRootEntry {
                root_ref: included.root_ref.clone(),
                reason: ExcludedRootReason::Unavailable,
                pattern: None,
                presentation_label: included.presentation_label.clone(),
            });
        }
    }
    entries
}

fn derive_admissions(artifact: &WorksetArtifactRecord) -> Vec<BroadActionAdmission> {
    BroadActionClass::all()
        .into_iter()
        .map(|action| derive_admission(artifact, action))
        .collect()
}

fn derive_admission(
    artifact: &WorksetArtifactRecord,
    action: BroadActionClass,
) -> BroadActionAdmission {
    let portability = artifact.portability.portability_class;
    let policy = artifact.policy_limitation.as_ref();
    let readiness = artifact.readiness.readiness_state;
    let is_partial = artifact.is_narrowed_scope()
        || !matches!(readiness, ReadinessState::Ready)
        || artifact.has_partial_member_truth();

    let policy_block: Option<BroadActionReason> = policy.map(|p| match p.narrowing_cause {
        NarrowingCause::AdminPolicy => BroadActionReason::AdminPolicyBlocked,
        NarrowingCause::TrustPolicy => BroadActionReason::TrustPolicyBlocked,
        NarrowingCause::LicenseOrExportControl => BroadActionReason::LicenseOrExportControlBlocked,
        _ => BroadActionReason::AdminPolicyBlocked,
    });

    match action {
        BroadActionClass::SearchQuery | BroadActionClass::GraphTraversal => {
            // Read-only lanes are allowed; the chip already discloses
            // partial scope and outside-row markers as a separate cue.
            BroadActionAdmission {
                action_class: action,
                decision: BroadActionDecision::Allowed,
                reason: None,
                explain_note: None,
            }
        }
        BroadActionClass::RefactorApply | BroadActionClass::AiApply => {
            if let Some(reason) = policy_block {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::BlockedByPolicy,
                    reason: Some(reason),
                    explain_note: Some(format!(
                        "{action} blocked under policy_limited_view; reopen the underlying workset.",
                        action = action.as_str()
                    )),
                };
            }
            if portability == PortabilityClass::MachineLocalOnly
                && !matches!(readiness, ReadinessState::Ready | ReadinessState::Warm)
            {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::BlockedBySparsePartial,
                    reason: Some(BroadActionReason::EphemeralSession),
                    explain_note: Some(format!(
                        "{action} blocked: ephemeral session has not warmed scope.",
                        action = action.as_str()
                    )),
                };
            }
            if is_partial {
                let reason = if !matches!(readiness, ReadinessState::Ready) {
                    BroadActionReason::ReadinessBelowReady
                } else if artifact.scope_class == ScopeClass::SparseSlice {
                    BroadActionReason::SparseSliceNarrowing
                } else if artifact.has_partial_member_truth() {
                    BroadActionReason::PartialMembershipMissing
                } else {
                    BroadActionReason::HonorActiveWorksetRoots
                };
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::NarrowedToScope,
                    reason: Some(reason),
                    explain_note: Some(format!(
                        "{action} narrowed to the active scope's roots and patterns.",
                        action = action.as_str()
                    )),
                };
            }
            BroadActionAdmission {
                action_class: action,
                decision: BroadActionDecision::Allowed,
                reason: None,
                explain_note: None,
            }
        }
        BroadActionClass::ExportArtifact => {
            if portability == PortabilityClass::ManagedProviderLocked {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::BlockedByPortability,
                    reason: Some(BroadActionReason::ManagedProviderLocked),
                    explain_note: Some(
                        "Export blocked: managed_provider_locked artifacts cannot leave their provider."
                            .to_string(),
                    ),
                };
            }
            if let Some(reason) = policy_block {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::BlockedByPolicy,
                    reason: Some(reason),
                    explain_note: Some(
                        "Export blocked under policy_limited_view; reopen the underlying workset before exporting."
                            .to_string(),
                    ),
                };
            }
            if is_partial {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::NarrowedToScope,
                    reason: Some(BroadActionReason::HonorActiveWorksetRoots),
                    explain_note: Some(
                        "Export narrowed to the active scope's declared roots and patterns; lineage chain preserved."
                            .to_string(),
                    ),
                };
            }
            BroadActionAdmission {
                action_class: action,
                decision: BroadActionDecision::Allowed,
                reason: None,
                explain_note: None,
            }
        }
        BroadActionClass::SupportArchive => {
            if portability == PortabilityClass::MachineLocalOnly
                && matches!(readiness, ReadinessState::Cold | ReadinessState::Warming)
            {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::BlockedBySparsePartial,
                    reason: Some(BroadActionReason::EphemeralSession),
                    explain_note: Some(
                        "Support archive blocked: ephemeral_session readiness is below warm."
                            .to_string(),
                    ),
                };
            }
            if let Some(reason) = policy_block {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::NarrowedToScope,
                    reason: Some(reason),
                    explain_note: Some(
                        "Support archive narrowed: policy_limited_view scope preserved; hidden members are not exported."
                            .to_string(),
                    ),
                };
            }
            if is_partial {
                return BroadActionAdmission {
                    action_class: action,
                    decision: BroadActionDecision::NarrowedToScope,
                    reason: Some(BroadActionReason::HonorActiveWorksetRoots),
                    explain_note: Some(
                        "Support archive narrowed to the active workset's declared roots and patterns; lineage preserved."
                            .to_string(),
                    ),
                };
            }
            BroadActionAdmission {
                action_class: action,
                decision: BroadActionDecision::Allowed,
                reason: None,
                explain_note: None,
            }
        }
    }
}

/// Support-export packet wrapping one or more [`WorksetScopeBetaTruth`]
/// records that share an artifact identity. A support reviewer reopens
/// every consumer surface against the same artifact without re-deriving
/// scope from a side channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub artifact_workset_ref: String,
    pub artifact_stable_scope_id: String,
    pub artifact_workset_name: String,
    pub artifact_scope_class: ScopeClass,
    pub artifact_scope_mode: ScopeMode,
    pub lineage: Vec<ScopeLineageEntry>,
    pub truths: Vec<WorksetScopeBetaTruth>,
    pub emitted_at: String,
}

impl WorksetScopeBetaSupportExport {
    /// Returns the canonical record-kind tag for the support-export packet.
    pub const RECORD_KIND: &'static str = WORKSET_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND;
    /// Returns the schema version for the support-export packet.
    pub const SCHEMA_VERSION: u32 = WORKSET_SCOPE_BETA_SCHEMA_VERSION;

    /// Bundles the given truths into a support-export packet. Every truth
    /// must reference the same workset / stable scope id and the lineage
    /// chain is taken from the first truth.
    pub fn from_truths(
        truths: Vec<WorksetScopeBetaTruth>,
        emitted_at: impl Into<String>,
    ) -> Result<Self, WorksetScopeBetaError> {
        if truths.is_empty() {
            return Err(WorksetScopeBetaError::EmptyLineage);
        }
        let head = &truths[0];
        for truth in &truths {
            truth.validate()?;
            if truth.workset_ref != head.workset_ref
                || truth.stable_scope_id != head.stable_scope_id
            {
                return Err(WorksetScopeBetaError::LineageRootMismatch);
            }
        }
        Ok(Self {
            record_kind: Self::RECORD_KIND.to_string(),
            schema_version: Self::SCHEMA_VERSION,
            artifact_workset_ref: head.workset_ref.clone(),
            artifact_stable_scope_id: head.stable_scope_id.clone(),
            artifact_workset_name: head.workset_name.clone(),
            artifact_scope_class: head.scope_class,
            artifact_scope_mode: head.scope_mode,
            lineage: head.lineage.clone(),
            truths,
            emitted_at: emitted_at.into(),
        })
    }

    /// Returns the truth for a given consumer surface, if any.
    pub fn truth_for(&self, surface: BetaConsumerSurface) -> Option<&WorksetScopeBetaTruth> {
        self.truths.iter().find(|t| t.consumer_surface == surface)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roots::WorkspaceRootKind;
    use crate::worksets::{
        IncludedRootRef, MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause,
        PartialTruthLabel, PatternEntry, PatternKind, PolicyLimitation, PortabilityMetadata,
        ReadinessMetadata, SourceClass, WorksetArtifactRecordKind,
    };

    fn full_workspace_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:beta:full".to_string(),
            scope_id: Some("scope:beta:full".to_string()),
            workset_name: "Full payments workspace".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::FullWorkspace,
            scope_mode: ScopeMode::Full,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string(), "fs-r-1".to_string()],
            included_roots: vec![
                IncludedRootRef {
                    root_ref: "fs-r-0".to_string(),
                    root_kind: WorkspaceRootKind::LocalRepoRoot,
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-a".to_string()),
                },
                IncludedRootRef {
                    root_ref: "fs-r-1".to_string(),
                    root_kind: WorkspaceRootKind::LocalRepoRoot,
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-b".to_string()),
                },
            ],
            patterns: vec![],
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-0".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-a".to_string()),
                },
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-1".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: Some("repo-b".to_string()),
                },
            ],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::WorkspaceShared,
                portability_class: PortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Ready,
                hidden_result_count_known: true,
                hidden_result_count: Some(0),
                partial_index_note: None,
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:0".to_string(),
            notes: None,
        }
    }

    fn sparse_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:beta:sparse".to_string(),
            scope_id: Some("scope:beta:sparse".to_string()),
            workset_name: "Frontend sparse slice".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::SparseSlice,
            scope_mode: ScopeMode::Sparse,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            included_roots: vec![IncludedRootRef {
                root_ref: "fs-r-0".to_string(),
                root_kind: WorkspaceRootKind::LocalRepoRoot,
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("repo-a".to_string()),
            }],
            patterns: vec![
                PatternEntry {
                    pattern_kind: PatternKind::Include,
                    pattern: "apps/web/**".to_string(),
                    applies_to_root_ref: None,
                },
                PatternEntry {
                    pattern_kind: PatternKind::Exclude,
                    pattern: "apps/web/public/vendor/**".to_string(),
                    applies_to_root_ref: None,
                },
            ],
            membership_policy: MembershipPolicy::GlobPattern,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::LocalOnly,
                portability_class: PortabilityClass::PortableWithRebinding,
                includes_machine_local_refs: true,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Partial,
                hidden_result_count_known: true,
                hidden_result_count: Some(17_412),
                partial_index_note: Some("Backend folders excluded.".to_string()),
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    fn policy_limited_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:beta:policy".to_string(),
            scope_id: Some("scope:beta:policy".to_string()),
            workset_name: "Restricted view".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::PolicyLimitedView,
            scope_mode: ScopeMode::Sparse,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            included_roots: vec![IncludedRootRef {
                root_ref: "fs-r-0".to_string(),
                root_kind: WorkspaceRootKind::ManagedCloudRoot,
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-a".to_string()),
            }],
            patterns: vec![],
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-a".to_string()),
            }],
            policy_limitation: Some(PolicyLimitation {
                underlying_workset_ref: "wks:beta:policy:underlying".to_string(),
                policy_ref: "policy:test:admin".to_string(),
                narrowing_cause: NarrowingCause::AdminPolicy,
                visible_member_count: 1,
                hidden_member_count: 2,
                hidden_member_list_visible: false,
            }),
            portability: PortabilityMetadata {
                source_class: SourceClass::Managed,
                portability_class: PortabilityClass::ManagedProviderLocked,
                includes_machine_local_refs: false,
                includes_managed_provider_refs: true,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Ready,
                hidden_result_count_known: true,
                hidden_result_count: Some(2),
                partial_index_note: None,
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    #[test]
    fn full_workspace_allows_every_action() {
        let artifact = full_workspace_artifact();
        let workspace_roots = vec!["fs-r-0".to_string(), "fs-r-1".to_string()];
        let truth = artifact.project_beta_truth(
            BetaConsumerSurface::Refactor,
            ScopeObservationInputs {
                workspace_root_refs: &workspace_roots,
                workspace_root_labels: &[],
                parent_artifact: None,
            },
            "mono:1",
        );
        truth.validate().expect("truth must validate");
        for action in BroadActionClass::all() {
            let admission = truth.admission_for(action).expect("admission required");
            assert_eq!(admission.decision, BroadActionDecision::Allowed);
            assert!(admission.reason.is_none());
        }
        assert!(truth.excluded_roots.is_empty());
        assert_eq!(truth.lineage.len(), 1);
    }

    #[test]
    fn sparse_slice_narrows_refactor_and_ai() {
        let artifact = sparse_artifact();
        let workspace_roots = vec!["fs-r-0".to_string(), "fs-r-other".to_string()];
        let truth = artifact.project_beta_truth(
            BetaConsumerSurface::Refactor,
            ScopeObservationInputs {
                workspace_root_refs: &workspace_roots,
                workspace_root_labels: &[("fs-r-other".to_string(), "repo-b".to_string())],
                parent_artifact: None,
            },
            "mono:1",
        );
        truth.validate().expect("truth must validate");
        let refactor = truth
            .admission_for(BroadActionClass::RefactorApply)
            .expect("refactor admission required");
        assert_eq!(refactor.decision, BroadActionDecision::NarrowedToScope);
        let ai = truth
            .admission_for(BroadActionClass::AiApply)
            .expect("ai admission required");
        assert_eq!(ai.decision, BroadActionDecision::NarrowedToScope);
        let export = truth
            .admission_for(BroadActionClass::ExportArtifact)
            .expect("export admission required");
        assert_eq!(export.decision, BroadActionDecision::NarrowedToScope);
        assert!(truth
            .excluded_roots
            .iter()
            .any(|e| e.root_ref == "fs-r-other"
                && e.reason == ExcludedRootReason::NotInWorksetRootList));
    }

    #[test]
    fn policy_limited_blocks_refactor_ai_and_export() {
        let artifact = policy_limited_artifact();
        let workspace_roots = vec!["fs-r-0".to_string()];
        let truth = artifact.project_beta_truth(
            BetaConsumerSurface::Export,
            ScopeObservationInputs {
                workspace_root_refs: &workspace_roots,
                workspace_root_labels: &[],
                parent_artifact: None,
            },
            "mono:1",
        );
        truth
            .validate()
            .expect("policy-limited truth must validate");
        let refactor = truth
            .admission_for(BroadActionClass::RefactorApply)
            .expect("refactor admission required");
        assert_eq!(refactor.decision, BroadActionDecision::BlockedByPolicy);
        assert_eq!(refactor.reason, Some(BroadActionReason::AdminPolicyBlocked));
        let export = truth
            .admission_for(BroadActionClass::ExportArtifact)
            .expect("export admission required");
        assert_eq!(export.decision, BroadActionDecision::BlockedByPortability);
        assert_eq!(
            export.reason,
            Some(BroadActionReason::ManagedProviderLocked)
        );
        let support = truth
            .admission_for(BroadActionClass::SupportArchive)
            .expect("support admission required");
        assert_eq!(support.decision, BroadActionDecision::NarrowedToScope);
        assert!(truth
            .excluded_roots
            .iter()
            .any(|e| e.reason == ExcludedRootReason::PolicyHidden));
        assert!(truth.lineage.len() >= 2);
    }

    #[test]
    fn outside_scope_truth_blocks_broad_actions_and_carries_note() {
        let artifact = sparse_artifact();
        let workspace_roots = vec!["fs-r-0".to_string(), "fs-r-other".to_string()];
        let truth = artifact.project_beta_truth_outside_scope(
            BetaConsumerSurface::Search,
            ScopeObservationInputs {
                workspace_root_refs: &workspace_roots,
                workspace_root_labels: &[],
                parent_artifact: None,
            },
            "fs-r-other",
            "Quick-open jumped into a sibling repo without a widen review.",
            "mono:1",
        );
        truth.validate().expect("outside-scope truth must validate");
        assert!(truth.outside_current_scope_marker_visible);
        assert!(truth.explain_note.is_some());
        let refactor = truth
            .admission_for(BroadActionClass::RefactorApply)
            .unwrap();
        assert_eq!(
            refactor.decision,
            BroadActionDecision::BlockedByOutsideScope
        );
    }

    #[test]
    fn support_export_packet_bundles_truths_per_surface() {
        let artifact = sparse_artifact();
        let workspace_roots = vec!["fs-r-0".to_string()];
        let inputs = || ScopeObservationInputs {
            workspace_root_refs: &workspace_roots,
            workspace_root_labels: &[],
            parent_artifact: None,
        };
        let truths = vec![
            artifact.project_beta_truth(BetaConsumerSurface::Search, inputs(), "mono:1"),
            artifact.project_beta_truth(BetaConsumerSurface::Refactor, inputs(), "mono:2"),
            artifact.project_beta_truth(BetaConsumerSurface::Export, inputs(), "mono:3"),
            artifact.project_beta_truth(BetaConsumerSurface::SupportPacket, inputs(), "mono:4"),
        ];
        let packet = WorksetScopeBetaSupportExport::from_truths(truths, "mono:5")
            .expect("support export must validate");
        assert_eq!(packet.artifact_workset_ref, artifact.workset_id);
        assert_eq!(packet.truths.len(), 4);
        assert!(packet.truth_for(BetaConsumerSurface::Refactor).is_some());
        // Round-trip through serde to make sure the support packet is exportable.
        let payload = serde_json::to_string(&packet).expect("packet must serialize");
        let parsed: WorksetScopeBetaSupportExport =
            serde_json::from_str(&payload).expect("packet must round-trip");
        assert_eq!(parsed, packet);
    }
}
