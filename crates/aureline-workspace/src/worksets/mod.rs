//! Workset / sparse-slice artifact seed.
//!
//! A workset artifact is the durable, portable, nameable scope object every
//! search, graph, refactor, support, and AI surface reads when it has to know
//! what "in scope" means. The artifact's stable identity is the
//! [`WorksetArtifactRecord::workset_id`]; downstream surfaces never re-mint a
//! parallel id for the same scope.
//!
//! This module mirrors `schemas/workspace/workset_artifact.schema.json` for
//! the M1 seed (the `workset_artifact_record` and `scope_truth_chip_record`
//! shapes). Patterns and policy details flow verbatim; raw absolute paths,
//! credential material, and the exact membership of an admin-narrowed hidden
//! set never leak across this surface.

use serde::{Deserialize, Serialize};

use crate::roots::WorkspaceRootKind;

/// Schema version for the seeded workset-artifact payload shape.
pub type WorksetArtifactSchemaVersion = u32;

/// Identifies the `workset_artifact_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetArtifactRecordKind {
    /// `workset_artifact_record`
    WorksetArtifactRecord,
}

/// Identifies the `scope_truth_chip_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeTruthChipRecordKind {
    /// `scope_truth_chip_record`
    ScopeTruthChipRecord,
}

/// Identifies the `scope_widen_diff_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeWidenDiffRecordKind {
    /// `scope_widen_diff_record`.
    ScopeWidenDiffRecord,
}

/// Frozen scope-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    CurrentRepo,
    SelectedWorkset,
    SparseSlice,
    FullWorkspace,
    PolicyLimitedView,
}

impl ScopeClass {
    /// Returns the stable string vocabulary for this scope class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepo => "current_repo",
            Self::SelectedWorkset => "selected_workset",
            Self::SparseSlice => "sparse_slice",
            Self::FullWorkspace => "full_workspace",
            Self::PolicyLimitedView => "policy_limited_view",
        }
    }

    /// Returns true when the scope covers every member of the workspace.
    pub const fn is_full_workspace(self) -> bool {
        matches!(self, Self::FullWorkspace)
    }

    /// Returns true when the scope is narrowed below the workspace.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::CurrentRepo | Self::SelectedWorkset | Self::SparseSlice | Self::PolicyLimitedView
        )
    }

    /// Resolves the closed scope-chip label family.
    pub const fn chip_label_family(self) -> &'static str {
        match self {
            Self::CurrentRepo => "Current repo",
            Self::SelectedWorkset => "Selected workset",
            Self::SparseSlice => "Sparse slice",
            Self::FullWorkspace => "Full workspace",
            Self::PolicyLimitedView => "Policy-limited view",
        }
    }
}

/// Whether the scope materializes complete roots or a sparse subset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeMode {
    Full,
    Sparse,
}

impl ScopeMode {
    /// Returns the stable string vocabulary for this scope mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Sparse => "sparse",
        }
    }
}

impl Default for ScopeMode {
    fn default() -> Self {
        Self::Full
    }
}

/// How the artifact enumerates its members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipPolicy {
    ExplicitRootList,
    GlobPattern,
    DependencyGraphReachable,
    ManifestDriven,
}

/// Where the artifact lives on disk / in a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    LocalOnly,
    WorkspaceShared,
    ProfileImported,
    Managed,
    EphemeralSession,
}

/// How the artifact survives export/import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityClass {
    FullyPortable,
    PortableWithRebinding,
    MachineLocalOnly,
    ManagedProviderLocked,
}

/// Index/materialization readiness vocabulary used on switcher rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    Cold,
    Warming,
    Warm,
    Partial,
    Ready,
}

impl ReadinessState {
    /// Returns the stable string vocabulary for this readiness state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cold => "cold",
            Self::Warming => "warming",
            Self::Warm => "warm",
            Self::Partial => "partial",
            Self::Ready => "ready",
        }
    }
}

/// TAD 12.6 partial-truth labels carried on member refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialTruthLabel {
    Loaded,
    ManifestKnown,
    Cached,
    Unavailable,
}

impl PartialTruthLabel {
    /// Returns the stable string vocabulary for this label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Loaded => "loaded",
            Self::ManifestKnown => "manifest_known",
            Self::Cached => "cached",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Why a policy hides members from a view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingCause {
    AdminPolicy,
    TrustPolicy,
    LicenseOrExportControl,
    RemoteUnavailable,
    IndexNotBuilt,
    UserMuted,
}

impl NarrowingCause {
    /// Returns true when the hidden member list MUST never project outside a
    /// policy-admin surface.
    pub const fn forbids_hidden_list(self) -> bool {
        matches!(self, Self::AdminPolicy | Self::LicenseOrExportControl)
    }
}

/// Include / exclude pattern row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternKind {
    Include,
    Exclude,
}

/// One include/exclude pattern entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternEntry {
    pub pattern_kind: PatternKind,
    pub pattern: String,
    #[serde(default)]
    pub applies_to_root_ref: Option<String>,
}

/// Kind of member reference inside a workset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRefKind {
    Root,
    Folder,
    Module,
    ManifestEntry,
    GraphSeed,
}

/// One member reference resolved from the workset's policy and patterns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberRef {
    pub ref_kind: MemberRefKind,
    #[serde(rename = "ref")]
    pub ref_id: String,
    pub partial_truth: PartialTruthLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Root entry included by a workset or slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncludedRootRef {
    pub root_ref: String,
    pub root_kind: WorkspaceRootKind,
    pub partial_truth: PartialTruthLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
}

/// Portability metadata attached to the workset artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortabilityMetadata {
    pub source_class: SourceClass,
    pub portability_class: PortabilityClass,
    pub includes_machine_local_refs: bool,
    pub includes_managed_provider_refs: bool,
    pub requires_rebinding_on_import: bool,
    #[serde(default)]
    pub profile_sync_group_ref: Option<String>,
}

/// Readiness and hidden-result accounting attached to the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessMetadata {
    pub readiness_state: ReadinessState,
    pub hidden_result_count_known: bool,
    #[serde(default)]
    pub hidden_result_count: Option<u64>,
    #[serde(default)]
    pub partial_index_note: Option<String>,
}

/// Policy-limited narrowing block. Attached only when scope_class is
/// [`ScopeClass::PolicyLimitedView`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLimitation {
    pub underlying_workset_ref: String,
    pub policy_ref: String,
    pub narrowing_cause: NarrowingCause,
    pub visible_member_count: u32,
    pub hidden_member_count: u32,
    pub hidden_member_list_visible: bool,
}

/// Durable, portable, nameable workset artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetArtifactRecord {
    pub record_kind: WorksetArtifactRecordKind,
    pub workset_artifact_schema_version: WorksetArtifactSchemaVersion,
    pub workset_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    pub workset_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
    pub scope_class: ScopeClass,
    #[serde(default)]
    pub scope_mode: ScopeMode,
    #[serde(default)]
    pub workspace_ref: Option<String>,
    pub root_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub included_roots: Vec<IncludedRootRef>,
    pub patterns: Vec<PatternEntry>,
    pub membership_policy: MembershipPolicy,
    pub member_refs: Vec<MemberRef>,
    #[serde(default)]
    pub policy_limitation: Option<PolicyLimitation>,
    pub portability: PortabilityMetadata,
    pub readiness: ReadinessMetadata,
    #[serde(default)]
    pub parent_workset_ref: Option<String>,
    #[serde(default)]
    pub manifest_source_ref: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Errors detected while validating a workset artifact against the seed
/// invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetArtifactError {
    EmptyWorksetId,
    EmptyScopeId,
    EmptyWorksetName,
    EmptyRootRefs,
    DuplicateRootRef(String),
    MissingIncludedRootRef(String),
    IncludedRootNotInRootRefs(String),
    DuplicateIncludedRootRef(String),
    SparseSliceRequiresPattern,
    ScopeModeMismatch {
        scope_class: ScopeClass,
        scope_mode: ScopeMode,
    },
    ReadyStateRequiresKnownHiddenCount,
    PolicyLimitationRequired,
    PolicyLimitationForbidden,
    PolicyExposesAdminHiddenList(NarrowingCause),
    PatternRootRefNotInRootRefs(String),
    SchemaVersionMismatch(WorksetArtifactSchemaVersion),
    EmptyMemberRefsForResolvedPolicy,
}

impl std::fmt::Display for WorksetArtifactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyWorksetId => write!(f, "workset_id must not be empty"),
            Self::EmptyScopeId => write!(f, "scope_id must not be empty when present"),
            Self::EmptyWorksetName => write!(f, "workset_name must not be empty"),
            Self::EmptyRootRefs => write!(f, "root_refs must contain at least one root"),
            Self::DuplicateRootRef(r) => write!(f, "duplicate root_ref: {r}"),
            Self::MissingIncludedRootRef(r) => write!(
                f,
                "included_roots must carry root kind and result-state truth for root_ref {r}"
            ),
            Self::IncludedRootNotInRootRefs(r) => {
                write!(f, "included root {r} is not declared in root_refs")
            }
            Self::DuplicateIncludedRootRef(r) => write!(f, "duplicate included root: {r}"),
            Self::SparseSliceRequiresPattern => {
                write!(
                    f,
                    "sparse_slice scope requires at least one include/exclude pattern"
                )
            }
            Self::ScopeModeMismatch {
                scope_class,
                scope_mode,
            } => write!(
                f,
                "scope_class {scope_class:?} cannot use scope_mode {scope_mode:?}"
            ),
            Self::ReadyStateRequiresKnownHiddenCount => write!(
                f,
                "ready workset artifacts must state whether hidden result counts are known"
            ),
            Self::PolicyLimitationRequired => {
                write!(
                    f,
                    "policy_limitation is required for policy_limited_view scope"
                )
            }
            Self::PolicyLimitationForbidden => {
                write!(
                    f,
                    "policy_limitation is forbidden outside policy_limited_view scope"
                )
            }
            Self::PolicyExposesAdminHiddenList(cause) => write!(
                f,
                "narrowing cause {cause:?} forbids exposing the hidden member list"
            ),
            Self::PatternRootRefNotInRootRefs(r) => {
                write!(
                    f,
                    "pattern applies_to_root_ref {r} is not declared in root_refs"
                )
            }
            Self::SchemaVersionMismatch(v) => write!(
                f,
                "unsupported workset_artifact_schema_version {v}; this seed accepts version 1"
            ),
            Self::EmptyMemberRefsForResolvedPolicy => write!(
                f,
                "member_refs must be non-empty for explicit_root_list and glob_pattern policies"
            ),
        }
    }
}

impl std::error::Error for WorksetArtifactError {}

/// Membership decision for a candidate root reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MembershipDecision {
    /// The candidate root is a declared member of this workset.
    InScope { partial_truth: PartialTruthLabel },
    /// The candidate root is in `root_refs` but the policy hides it.
    PolicyHidden { cause: NarrowingCause },
    /// The candidate root is outside the workset's declared root list.
    OutsideCurrentScope,
}

impl WorksetArtifactRecord {
    /// Validates the artifact against the M1 seed invariants. Returns
    /// `Ok(())` only when every invariant holds.
    pub fn validate(&self) -> Result<(), WorksetArtifactError> {
        if self.workset_artifact_schema_version != 1 {
            return Err(WorksetArtifactError::SchemaVersionMismatch(
                self.workset_artifact_schema_version,
            ));
        }
        if self.workset_id.is_empty() {
            return Err(WorksetArtifactError::EmptyWorksetId);
        }
        if self.scope_id.as_deref().is_some_and(str::is_empty) {
            return Err(WorksetArtifactError::EmptyScopeId);
        }
        if self.workset_name.is_empty() {
            return Err(WorksetArtifactError::EmptyWorksetName);
        }
        if self.root_refs.is_empty() {
            return Err(WorksetArtifactError::EmptyRootRefs);
        }
        let mut seen: Vec<&str> = Vec::with_capacity(self.root_refs.len());
        for r in &self.root_refs {
            if seen.iter().any(|s| *s == r.as_str()) {
                return Err(WorksetArtifactError::DuplicateRootRef(r.clone()));
            }
            seen.push(r);
        }
        let mut seen_included_roots: Vec<&str> = Vec::with_capacity(self.included_roots.len());
        for root in &self.included_roots {
            if !self.root_refs.iter().any(|r| r == &root.root_ref) {
                return Err(WorksetArtifactError::IncludedRootNotInRootRefs(
                    root.root_ref.clone(),
                ));
            }
            if seen_included_roots
                .iter()
                .any(|r| *r == root.root_ref.as_str())
            {
                return Err(WorksetArtifactError::DuplicateIncludedRootRef(
                    root.root_ref.clone(),
                ));
            }
            seen_included_roots.push(root.root_ref.as_str());
        }
        for root_ref in &self.root_refs {
            if !self
                .included_roots
                .iter()
                .any(|root| root.root_ref == *root_ref)
            {
                return Err(WorksetArtifactError::MissingIncludedRootRef(
                    root_ref.clone(),
                ));
            }
        }
        match (self.scope_class, self.scope_mode) {
            (ScopeClass::FullWorkspace | ScopeClass::CurrentRepo, ScopeMode::Sparse) => {
                return Err(WorksetArtifactError::ScopeModeMismatch {
                    scope_class: self.scope_class,
                    scope_mode: self.scope_mode,
                });
            }
            (ScopeClass::SparseSlice, ScopeMode::Full) => {
                return Err(WorksetArtifactError::ScopeModeMismatch {
                    scope_class: self.scope_class,
                    scope_mode: self.scope_mode,
                });
            }
            _ => {}
        }
        if self.scope_class == ScopeClass::SparseSlice && self.patterns.is_empty() {
            return Err(WorksetArtifactError::SparseSliceRequiresPattern);
        }
        if self.readiness.readiness_state == ReadinessState::Ready
            && !self.readiness.hidden_result_count_known
        {
            return Err(WorksetArtifactError::ReadyStateRequiresKnownHiddenCount);
        }
        for pattern in &self.patterns {
            if let Some(scoped) = pattern.applies_to_root_ref.as_deref() {
                if !self.root_refs.iter().any(|r| r == scoped) {
                    return Err(WorksetArtifactError::PatternRootRefNotInRootRefs(
                        scoped.to_string(),
                    ));
                }
            }
        }
        match (self.scope_class, self.policy_limitation.as_ref()) {
            (ScopeClass::PolicyLimitedView, None) => {
                return Err(WorksetArtifactError::PolicyLimitationRequired);
            }
            (cls, Some(_)) if cls != ScopeClass::PolicyLimitedView => {
                return Err(WorksetArtifactError::PolicyLimitationForbidden);
            }
            (ScopeClass::PolicyLimitedView, Some(limit))
                if limit.narrowing_cause.forbids_hidden_list()
                    && limit.hidden_member_list_visible =>
            {
                return Err(WorksetArtifactError::PolicyExposesAdminHiddenList(
                    limit.narrowing_cause,
                ));
            }
            _ => {}
        }
        let resolved_membership = matches!(
            self.membership_policy,
            MembershipPolicy::ExplicitRootList | MembershipPolicy::GlobPattern
        );
        if resolved_membership && self.member_refs.is_empty() {
            return Err(WorksetArtifactError::EmptyMemberRefsForResolvedPolicy);
        }
        Ok(())
    }

    /// Returns the count of declared filesystem roots in scope.
    pub fn root_count(&self) -> usize {
        self.root_refs.len()
    }

    /// Returns true when the artifact spans more than one filesystem root.
    pub fn is_multi_root(&self) -> bool {
        self.root_refs.len() >= 2
    }

    /// Returns true when the artifact carries the global-workspace scope.
    pub fn is_full_workspace(&self) -> bool {
        self.scope_class.is_full_workspace()
    }

    /// Returns true when the active scope is narrower than the workspace.
    pub fn is_narrowed_scope(&self) -> bool {
        !self.is_full_workspace()
    }

    /// Returns the stable scope identity every consumer must preserve.
    pub fn stable_scope_id(&self) -> &str {
        self.scope_id.as_deref().unwrap_or(&self.workset_id)
    }

    /// Returns whether this scope is materialized as full or sparse.
    pub const fn scope_mode(&self) -> ScopeMode {
        self.scope_mode
    }

    /// Returns the included roots with root-kind and result-state truth.
    pub fn included_roots(&self) -> &[IncludedRootRef] {
        &self.included_roots
    }

    /// Returns true when `root_ref` is a declared member of this workset.
    pub fn contains_root_ref(&self, root_ref: &str) -> bool {
        self.root_refs.iter().any(|r| r == root_ref)
    }

    /// Returns the partial-truth label for a declared root member, if any.
    pub fn root_member_partial_truth(&self, root_ref: &str) -> Option<PartialTruthLabel> {
        self.member_refs
            .iter()
            .find(|m| m.ref_kind == MemberRefKind::Root && m.ref_id == root_ref)
            .map(|m| m.partial_truth)
    }

    /// Returns a typed membership decision for a candidate root reference.
    pub fn root_membership_decision(&self, root_ref: &str) -> MembershipDecision {
        if !self.contains_root_ref(root_ref) {
            return MembershipDecision::OutsideCurrentScope;
        }
        if let Some(limit) = self.policy_limitation.as_ref() {
            if self.root_member_partial_truth(root_ref).is_none() {
                return MembershipDecision::PolicyHidden {
                    cause: limit.narrowing_cause,
                };
            }
        }
        let partial_truth = self
            .root_member_partial_truth(root_ref)
            .unwrap_or(PartialTruthLabel::ManifestKnown);
        MembershipDecision::InScope { partial_truth }
    }

    /// Returns true when at least one root_ref carries a non-loaded
    /// partial-truth label.
    pub fn has_partial_member_truth(&self) -> bool {
        self.member_refs
            .iter()
            .filter(|m| m.ref_kind == MemberRefKind::Root)
            .any(|m| m.partial_truth != PartialTruthLabel::Loaded)
    }

    /// Projects a [`ScopeTruthChipRecord`] for the given surface.
    pub fn project_chip(
        &self,
        chip_id: impl Into<String>,
        surface_class: ChipSurfaceClass,
        emitted_at: impl Into<String>,
    ) -> ScopeTruthChipRecord {
        let presentation_state = self.derive_presentation_state();
        let chip_label = match self.scope_class {
            ScopeClass::CurrentRepo | ScopeClass::FullWorkspace => {
                self.scope_class.chip_label_family().to_string()
            }
            ScopeClass::SelectedWorkset
            | ScopeClass::SparseSlice
            | ScopeClass::PolicyLimitedView => {
                format!(
                    "{} · {}",
                    self.scope_class.chip_label_family(),
                    self.workset_name
                )
            }
        };
        let hidden_summary = self.derive_hidden_result_summary();
        let mut offered_actions: Vec<ChipAction> = Vec::new();
        match self.scope_class {
            ScopeClass::CurrentRepo => {
                offered_actions.push(ChipAction::WidenToFullWorkspace);
            }
            ScopeClass::SelectedWorkset | ScopeClass::SparseSlice => {
                offered_actions.push(ChipAction::WidenWithReview);
                offered_actions.push(ChipAction::WidenToFullWorkspace);
                offered_actions.push(ChipAction::OpenScopeDiff);
            }
            ScopeClass::FullWorkspace => {
                offered_actions.push(ChipAction::NarrowToCurrentRepo);
            }
            ScopeClass::PolicyLimitedView => {
                offered_actions.push(ChipAction::KeepCurrentScope);
                if let Some(limit) = self.policy_limitation.as_ref() {
                    if limit.hidden_member_list_visible {
                        offered_actions.push(ChipAction::RevealHiddenResultsPolicyAdminOnly);
                    }
                }
            }
        }
        offered_actions.push(ChipAction::CopyWorksetId);
        if !matches!(
            self.portability.portability_class,
            PortabilityClass::ManagedProviderLocked
        ) {
            offered_actions.push(ChipAction::ExportWorksetArtifact);
        }
        ScopeTruthChipRecord {
            record_kind: ScopeTruthChipRecordKind::ScopeTruthChipRecord,
            workset_artifact_schema_version: 1,
            chip_id: chip_id.into(),
            surface_class,
            stable_scope_id: self.stable_scope_id().to_string(),
            workset_ref: self.workset_id.clone(),
            scope_class: self.scope_class,
            scope_mode: self.scope_mode,
            chip_presentation_state: presentation_state,
            chip_label,
            included_roots: self.included_roots.clone(),
            member_count: Some(self.member_refs.len() as u32),
            root_count: Some(self.root_refs.len() as u32),
            hidden_result_summary: hidden_summary,
            partial_index_note: self.readiness.partial_index_note.clone(),
            outside_current_scope_marker_visible: false,
            offered_actions,
            emitted_at: emitted_at.into(),
            notes: None,
        }
    }

    /// Projects an outside-current-scope chip for a search/result row whose
    /// owning root is not in this artifact's `root_refs`.
    pub fn project_outside_scope_chip(
        &self,
        chip_id: impl Into<String>,
        surface_class: ChipSurfaceClass,
        emitted_at: impl Into<String>,
    ) -> ScopeTruthChipRecord {
        ScopeTruthChipRecord {
            record_kind: ScopeTruthChipRecordKind::ScopeTruthChipRecord,
            workset_artifact_schema_version: 1,
            chip_id: chip_id.into(),
            surface_class,
            stable_scope_id: self.stable_scope_id().to_string(),
            workset_ref: self.workset_id.clone(),
            scope_class: self.scope_class,
            scope_mode: self.scope_mode,
            chip_presentation_state: ChipPresentationState::OutsideCurrentScope,
            chip_label: "Outside current scope".to_string(),
            included_roots: self.included_roots.clone(),
            member_count: None,
            root_count: None,
            hidden_result_summary: None,
            partial_index_note: None,
            outside_current_scope_marker_visible: true,
            offered_actions: vec![ChipAction::WidenWithReview, ChipAction::OpenInNewPane],
            emitted_at: emitted_at.into(),
            notes: None,
        }
    }

    fn derive_presentation_state(&self) -> ChipPresentationState {
        if self.scope_class == ScopeClass::PolicyLimitedView {
            return ChipPresentationState::ActivePolicyLimited;
        }
        match self.readiness.readiness_state {
            ReadinessState::Cold | ReadinessState::Warming | ReadinessState::Partial => {
                ChipPresentationState::ActivePartial
            }
            ReadinessState::Warm | ReadinessState::Ready => {
                if self.has_partial_member_truth() {
                    ChipPresentationState::ActivePartial
                } else {
                    ChipPresentationState::ActiveNarrowSafe
                }
            }
        }
    }

    fn derive_hidden_result_summary(&self) -> Option<HiddenResultSummary> {
        if !self.readiness.hidden_result_count_known
            && self.readiness.hidden_result_count.is_none()
            && self.policy_limitation.is_none()
        {
            return None;
        }
        let count_class = match self.scope_class {
            ScopeClass::PolicyLimitedView => HiddenResultCountClass::PolicyHidden,
            ScopeClass::SparseSlice => HiddenResultCountClass::PartialIndex,
            ScopeClass::SelectedWorkset => HiddenResultCountClass::OutsideScopeRoots,
            ScopeClass::FullWorkspace | ScopeClass::CurrentRepo => {
                if matches!(
                    self.readiness.readiness_state,
                    ReadinessState::Warming | ReadinessState::Partial
                ) {
                    HiddenResultCountClass::WarmingIndex
                } else {
                    HiddenResultCountClass::NoneKnown
                }
            }
        };
        Some(HiddenResultSummary {
            known: self.readiness.hidden_result_count_known,
            count: self.readiness.hidden_result_count,
            count_class,
        })
    }
}

/// Surface a [`ScopeTruthChipRecord`] renders on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChipSurfaceClass {
    WorksetSwitcher,
    ScopeBanner,
    SearchResultGroupHeader,
    SearchResultRowMarker,
    CrossRepoResultGroup,
    OpenFlowTrustCard,
    SupportPacketHeader,
    AiContextInspector,
    RefactorScopeFooter,
    ExportScopeFooter,
}

/// Chip presentation states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChipPresentationState {
    ActiveNarrowSafe,
    ActivePartial,
    ActivePolicyLimited,
    ActiveWidened,
    OutsideCurrentScope,
}

/// Typed chip actions a chip may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChipAction {
    WidenToFullWorkspace,
    WidenWithReview,
    NarrowToCurrentRepo,
    OpenScopeDiff,
    BuildMissingIndexes,
    KeepCurrentScope,
    RevealHiddenResultsPolicyAdminOnly,
    OpenInNewPane,
    CopyWorksetId,
    ExportWorksetArtifact,
}

/// Hidden-result count summary attached to a chip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenResultSummary {
    pub known: bool,
    #[serde(default)]
    pub count: Option<u64>,
    pub count_class: HiddenResultCountClass,
}

/// Typed class of hidden-result summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenResultCountClass {
    NoneKnown,
    PartialIndex,
    OutsideScopeRoots,
    PolicyHidden,
    WarmingIndex,
    RemoteUnreachable,
}

/// One scope-truth chip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeTruthChipRecord {
    pub record_kind: ScopeTruthChipRecordKind,
    pub workset_artifact_schema_version: WorksetArtifactSchemaVersion,
    pub chip_id: String,
    pub surface_class: ChipSurfaceClass,
    pub stable_scope_id: String,
    pub workset_ref: String,
    pub scope_class: ScopeClass,
    pub scope_mode: ScopeMode,
    pub chip_presentation_state: ChipPresentationState,
    pub chip_label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub included_roots: Vec<IncludedRootRef>,
    #[serde(default)]
    pub member_count: Option<u32>,
    #[serde(default)]
    pub root_count: Option<u32>,
    #[serde(default)]
    pub hidden_result_summary: Option<HiddenResultSummary>,
    #[serde(default)]
    pub partial_index_note: Option<String>,
    pub outside_current_scope_marker_visible: bool,
    pub offered_actions: Vec<ChipAction>,
    pub emitted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Typed class for one workset/slice diff entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDiffClass {
    /// Stable identity changed, which is invalid for widening diff records.
    IdentityChange,
    /// A root, folder, module, manifest entry, or graph seed was added.
    MemberAdded,
    /// A root, folder, module, manifest entry, or graph seed was removed.
    MemberRemoved,
    /// An include/exclude pattern admits more content in the candidate.
    PatternBroadened,
    /// An include/exclude pattern admits less content in the candidate.
    PatternNarrowed,
    /// A policy overlay hides more content in the candidate.
    PolicyNarrowed,
    /// A policy overlay exposes more content in the candidate.
    PolicyWidened,
    /// Readiness changed between the base and candidate artifacts.
    ReadinessChanged,
    /// Portability changed between the base and candidate artifacts.
    PortabilityChanged,
    /// Only presentation text changed.
    PresentationOnly,
}

impl ScopeDiffClass {
    /// Returns true when the entry contributes to a widening signal.
    pub const fn widens_scope(self) -> bool {
        matches!(
            self,
            Self::MemberAdded | Self::PatternBroadened | Self::PolicyWidened
        )
    }

    /// Returns true when the entry contributes to a narrowing signal.
    pub const fn narrows_scope(self) -> bool {
        matches!(
            self,
            Self::MemberRemoved | Self::PatternNarrowed | Self::PolicyNarrowed
        )
    }
}

/// Rough index or fetch cost for materializing a scope candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedIndexCostClass {
    /// No index or fetch work is expected.
    None,
    /// Existing warm caches can satisfy the candidate.
    CacheWarm,
    /// A bounded index pass is needed for newly visible members.
    TargetedIndex,
    /// A full index rebuild is needed.
    FullReindex,
    /// Remote data must be fetched before the candidate can be authoritative.
    RemoteFetchRequired,
}

/// One typed entry in a scope widen or narrow diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDiffEntry {
    /// Class of change represented by this entry.
    pub diff_class: ScopeDiffClass,
    /// Member affected by the change, when the diff is member-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_member_ref: Option<MemberRef>,
    /// Pattern affected by the change, when the diff is pattern-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_pattern: Option<PatternEntry>,
    /// Redaction-aware explanation rendered by review and support surfaces.
    pub note: String,
}

/// Typed diff between two workset artifacts or scope candidates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeWidenDiffRecord {
    /// Discriminator for the serialized record shape.
    pub record_kind: ScopeWidenDiffRecordKind,
    /// Schema version for the serialized diff shape.
    pub workset_artifact_schema_version: WorksetArtifactSchemaVersion,
    /// Stable diff identity.
    pub diff_id: String,
    /// Active workset before the diff is applied.
    pub base_workset_ref: String,
    /// Candidate workset after widening or narrowing.
    pub candidate_workset_ref: String,
    /// Typed entry list; never empty.
    pub entries: Vec<ScopeDiffEntry>,
    /// True when the candidate admits more scope than the base.
    pub widens_scope: bool,
    /// True when the candidate admits less scope than the base.
    pub narrows_scope: bool,
    /// True when the candidate changes portability posture.
    pub changes_portability: bool,
    /// True when the candidate changes readiness posture.
    pub changes_readiness: bool,
    /// True only when every entry is presentation-only.
    pub presentation_only: bool,
    /// Rough index/fetch cost for materializing the candidate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_index_cost_class: Option<ExpectedIndexCostClass>,
    /// Whether a remote fetch is required before the candidate can activate.
    #[serde(default)]
    pub remote_fetch_required: bool,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
    /// Optional redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Errors detected while validating a [`ScopeWidenDiffRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeWidenDiffError {
    /// The diff schema version is not supported by this crate.
    SchemaVersionMismatch(WorksetArtifactSchemaVersion),
    /// The diff id is missing.
    EmptyDiffId,
    /// The base workset ref is missing.
    EmptyBaseWorksetRef,
    /// The candidate workset ref is missing.
    EmptyCandidateWorksetRef,
    /// Base and candidate refs must be different stable identities.
    CandidateReusesBaseIdentity,
    /// Diff entries are required.
    EmptyEntries,
    /// A diff entry used the forbidden identity-change class.
    IdentityChangeForbidden,
    /// Presentation-only diffs cannot also widen, narrow, or change readiness/portability.
    PresentationOnlyHasBehavioralChange,
    /// A behavioral diff was marked presentation-only.
    BehavioralDiffMarkedPresentationOnly,
    /// The widens_scope flag does not match the entries.
    WidenFlagMismatch,
    /// The narrows_scope flag does not match the entries.
    NarrowFlagMismatch,
    /// Remote fetch cost must disclose a remote fetch requirement.
    RemoteFetchCostWithoutRemoteRequirement,
}

impl std::fmt::Display for ScopeWidenDiffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(version) => write!(
                f,
                "unsupported workset_artifact_schema_version {version}; this seed accepts version 1"
            ),
            Self::EmptyDiffId => write!(f, "diff_id must not be empty"),
            Self::EmptyBaseWorksetRef => write!(f, "base_workset_ref must not be empty"),
            Self::EmptyCandidateWorksetRef => write!(f, "candidate_workset_ref must not be empty"),
            Self::CandidateReusesBaseIdentity => {
                write!(f, "candidate_workset_ref must not equal base_workset_ref")
            }
            Self::EmptyEntries => write!(f, "entries must contain at least one diff entry"),
            Self::IdentityChangeForbidden => {
                write!(f, "identity_change is forbidden on scope widen diff records")
            }
            Self::PresentationOnlyHasBehavioralChange => write!(
                f,
                "presentation_only diffs must not widen, narrow, or change readiness/portability"
            ),
            Self::BehavioralDiffMarkedPresentationOnly => {
                write!(f, "behavioral diff entries cannot be marked presentation_only")
            }
            Self::WidenFlagMismatch => write!(f, "widens_scope does not match diff entries"),
            Self::NarrowFlagMismatch => write!(f, "narrows_scope does not match diff entries"),
            Self::RemoteFetchCostWithoutRemoteRequirement => write!(
                f,
                "expected_index_cost_class remote_fetch_required requires remote_fetch_required = true"
            ),
        }
    }
}

impl std::error::Error for ScopeWidenDiffError {}

impl ScopeWidenDiffRecord {
    /// Validates the diff record against the portable workset alpha invariants.
    pub fn validate(&self) -> Result<(), ScopeWidenDiffError> {
        if self.workset_artifact_schema_version != 1 {
            return Err(ScopeWidenDiffError::SchemaVersionMismatch(
                self.workset_artifact_schema_version,
            ));
        }
        if self.diff_id.is_empty() {
            return Err(ScopeWidenDiffError::EmptyDiffId);
        }
        if self.base_workset_ref.is_empty() {
            return Err(ScopeWidenDiffError::EmptyBaseWorksetRef);
        }
        if self.candidate_workset_ref.is_empty() {
            return Err(ScopeWidenDiffError::EmptyCandidateWorksetRef);
        }
        if self.base_workset_ref == self.candidate_workset_ref {
            return Err(ScopeWidenDiffError::CandidateReusesBaseIdentity);
        }
        if self.entries.is_empty() {
            return Err(ScopeWidenDiffError::EmptyEntries);
        }
        if self
            .entries
            .iter()
            .any(|entry| entry.diff_class == ScopeDiffClass::IdentityChange)
        {
            return Err(ScopeWidenDiffError::IdentityChangeForbidden);
        }

        let all_presentation = self
            .entries
            .iter()
            .all(|entry| entry.diff_class == ScopeDiffClass::PresentationOnly);
        if self.presentation_only {
            if !all_presentation {
                return Err(ScopeWidenDiffError::BehavioralDiffMarkedPresentationOnly);
            }
            if self.widens_scope
                || self.narrows_scope
                || self.changes_portability
                || self.changes_readiness
            {
                return Err(ScopeWidenDiffError::PresentationOnlyHasBehavioralChange);
            }
        } else if all_presentation {
            return Err(ScopeWidenDiffError::BehavioralDiffMarkedPresentationOnly);
        }

        let entry_widens = self
            .entries
            .iter()
            .any(|entry| entry.diff_class.widens_scope());
        if self.widens_scope != entry_widens {
            return Err(ScopeWidenDiffError::WidenFlagMismatch);
        }
        let entry_narrows = self
            .entries
            .iter()
            .any(|entry| entry.diff_class.narrows_scope());
        if self.narrows_scope != entry_narrows {
            return Err(ScopeWidenDiffError::NarrowFlagMismatch);
        }
        if self.expected_index_cost_class == Some(ExpectedIndexCostClass::RemoteFetchRequired)
            && !self.remote_fetch_required
        {
            return Err(ScopeWidenDiffError::RemoteFetchCostWithoutRemoteRequirement);
        }
        Ok(())
    }
}

/// Consumer class that projects a workset/scope binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetScopeConsumerClass {
    LocalUi,
    RemoteUi,
    Headless,
    SupportExport,
    Navigation,
    RefactorScope,
}

impl WorksetScopeConsumerClass {
    /// Stable token used in UI, headless, and support-export records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUi => "local_ui",
            Self::RemoteUi => "remote_ui",
            Self::Headless => "headless",
            Self::SupportExport => "support_export",
            Self::Navigation => "navigation",
            Self::RefactorScope => "refactor_scope",
        }
    }
}

/// Whether a saved workset/slice reopened exactly for a consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReopenState {
    Exact,
    Degraded,
}

impl ScopeReopenState {
    /// Stable token used in exported scope bindings.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Degraded => "degraded",
        }
    }
}

/// Explicit reason a saved scope could not reopen exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDegradedReason {
    MissingRoot,
    RootKindUnsupported,
    RebindingRequired,
    RemoteUnavailable,
    ManagedProviderUnavailable,
    ManifestUnavailable,
    PolicyLimited,
}

impl ScopeDegradedReason {
    /// Stable token used in exported scope bindings.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRoot => "missing_root",
            Self::RootKindUnsupported => "root_kind_unsupported",
            Self::RebindingRequired => "rebinding_required",
            Self::RemoteUnavailable => "remote_unavailable",
            Self::ManagedProviderUnavailable => "managed_provider_unavailable",
            Self::ManifestUnavailable => "manifest_unavailable",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// Reopen posture requested by a scope consumer projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeReopenPosture {
    Exact,
    Degraded(ScopeDegradedReason),
}

/// Exportable scope binding consumed by UI, remote, headless, support, navigation, and refactor surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetScopeConsumerBinding {
    pub record_kind: String,
    pub schema_version: u32,
    pub stable_scope_id: String,
    pub workset_ref: String,
    pub workset_name: String,
    pub scope_class: ScopeClass,
    pub scope_mode: ScopeMode,
    pub consumer_class: WorksetScopeConsumerClass,
    pub reopen_state: ScopeReopenState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_reason: Option<ScopeDegradedReason>,
    pub included_roots: Vec<IncludedRootRef>,
    pub patterns: Vec<PatternEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_artifact_ref: Option<String>,
    pub emitted_at: String,
}

impl WorksetScopeConsumerBinding {
    /// Stable record-kind tag carried in serialized consumer bindings.
    pub const RECORD_KIND: &'static str = "workset_scope_consumer_binding";
    /// Schema version for serialized consumer bindings.
    pub const SCHEMA_VERSION: u32 = 1;
}

impl WorksetArtifactRecord {
    /// Projects an exportable binding for a concrete consumer.
    pub fn project_consumer_binding(
        &self,
        consumer_class: WorksetScopeConsumerClass,
        reopen_posture: ScopeReopenPosture,
        emitted_at: impl Into<String>,
    ) -> WorksetScopeConsumerBinding {
        let (reopen_state, degraded_reason) = match reopen_posture {
            ScopeReopenPosture::Exact => (ScopeReopenState::Exact, None),
            ScopeReopenPosture::Degraded(reason) => (ScopeReopenState::Degraded, Some(reason)),
        };
        WorksetScopeConsumerBinding {
            record_kind: WorksetScopeConsumerBinding::RECORD_KIND.to_string(),
            schema_version: WorksetScopeConsumerBinding::SCHEMA_VERSION,
            stable_scope_id: self.stable_scope_id().to_string(),
            workset_ref: self.workset_id.clone(),
            workset_name: self.workset_name.clone(),
            scope_class: self.scope_class,
            scope_mode: self.scope_mode,
            consumer_class,
            reopen_state,
            degraded_reason,
            included_roots: self.included_roots.clone(),
            patterns: self.patterns.clone(),
            reviewable_artifact_ref: self.manifest_source_ref.clone(),
            emitted_at: emitted_at.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:test:hot:0".to_string(),
            scope_id: Some("scope:test:hot:0".to_string()),
            workset_name: "Hot path".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::SelectedWorkset,
            scope_mode: ScopeMode::Sparse,
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
                    root_kind: WorkspaceRootKind::RemoteRepository,
                    partial_truth: PartialTruthLabel::Cached,
                    presentation_label: Some("repo-b".to_string()),
                },
            ],
            patterns: vec![PatternEntry {
                pattern_kind: PatternKind::Include,
                pattern: "src/**".to_string(),
                applies_to_root_ref: Some("fs-r-0".to_string()),
            }],
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
                    partial_truth: PartialTruthLabel::Cached,
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
                readiness_state: ReadinessState::Warm,
                hidden_result_count_known: true,
                hidden_result_count: Some(2),
                partial_index_note: Some("Cached root warms on first search.".to_string()),
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:1".to_string(),
            notes: None,
        }
    }

    #[test]
    fn validate_accepts_a_well_formed_artifact() {
        let artifact = fixture_artifact();
        artifact.validate().expect("must validate");
        assert!(artifact.is_multi_root());
        assert!(!artifact.is_full_workspace());
        assert!(artifact.is_narrowed_scope());
        assert_eq!(artifact.stable_scope_id(), "scope:test:hot:0");
        assert_eq!(artifact.scope_mode(), ScopeMode::Sparse);
    }

    #[test]
    fn validate_rejects_pattern_for_undeclared_root() {
        let mut artifact = fixture_artifact();
        artifact.patterns[0].applies_to_root_ref = Some("fs-r-99".to_string());
        let err = artifact.validate().expect_err("must reject");
        assert_eq!(
            err,
            WorksetArtifactError::PatternRootRefNotInRootRefs("fs-r-99".to_string())
        );
    }

    #[test]
    fn validate_requires_policy_limitation_for_policy_limited_view() {
        let mut artifact = fixture_artifact();
        artifact.scope_class = ScopeClass::PolicyLimitedView;
        let err = artifact.validate().expect_err("must reject");
        assert_eq!(err, WorksetArtifactError::PolicyLimitationRequired);
    }

    #[test]
    fn validate_forbids_admin_policy_hidden_list_exposure() {
        let mut artifact = fixture_artifact();
        artifact.scope_class = ScopeClass::PolicyLimitedView;
        artifact.policy_limitation = Some(PolicyLimitation {
            underlying_workset_ref: "wks:test:hot:0".to_string(),
            policy_ref: "policy:test:admin".to_string(),
            narrowing_cause: NarrowingCause::AdminPolicy,
            visible_member_count: 1,
            hidden_member_count: 1,
            hidden_member_list_visible: true,
        });
        let err = artifact.validate().expect_err("must reject");
        assert_eq!(
            err,
            WorksetArtifactError::PolicyExposesAdminHiddenList(NarrowingCause::AdminPolicy)
        );
    }

    #[test]
    fn membership_decisions_distinguish_outside_and_in_scope() {
        let artifact = fixture_artifact();
        assert_eq!(
            artifact.root_membership_decision("fs-r-0"),
            MembershipDecision::InScope {
                partial_truth: PartialTruthLabel::Loaded
            }
        );
        assert_eq!(
            artifact.root_membership_decision("fs-r-1"),
            MembershipDecision::InScope {
                partial_truth: PartialTruthLabel::Cached
            }
        );
        assert_eq!(
            artifact.root_membership_decision("fs-r-99"),
            MembershipDecision::OutsideCurrentScope
        );
    }

    #[test]
    fn project_chip_marks_partial_when_member_truth_is_not_loaded() {
        let artifact = fixture_artifact();
        let chip = artifact.project_chip("chip:0", ChipSurfaceClass::WorksetSwitcher, "mono:1");
        assert_eq!(
            chip.chip_presentation_state,
            ChipPresentationState::ActivePartial
        );
        assert_eq!(chip.root_count, Some(2));
        assert_eq!(chip.stable_scope_id, "scope:test:hot:0");
        assert_eq!(chip.scope_mode, ScopeMode::Sparse);
        assert_eq!(chip.included_roots.len(), 2);
        assert_eq!(chip.scope_class, ScopeClass::SelectedWorkset);
        assert!(chip.chip_label.starts_with("Selected workset"));
        assert!(chip.offered_actions.contains(&ChipAction::WidenWithReview));
        assert!(chip.offered_actions.contains(&ChipAction::OpenScopeDiff));
        assert!(chip.offered_actions.contains(&ChipAction::CopyWorksetId));
    }

    #[test]
    fn outside_scope_chip_marks_outside_current_scope() {
        let artifact = fixture_artifact();
        let chip = artifact.project_outside_scope_chip(
            "chip:1",
            ChipSurfaceClass::SearchResultRowMarker,
            "mono:2",
        );
        assert!(chip.outside_current_scope_marker_visible);
        assert_eq!(
            chip.chip_presentation_state,
            ChipPresentationState::OutsideCurrentScope
        );
        assert_eq!(chip.chip_label, "Outside current scope");
    }

    #[test]
    fn consumer_bindings_preserve_identity_and_degrade_reason() {
        let artifact = fixture_artifact();
        let local = artifact.project_consumer_binding(
            WorksetScopeConsumerClass::LocalUi,
            ScopeReopenPosture::Exact,
            "mono:2",
        );
        let support = artifact.project_consumer_binding(
            WorksetScopeConsumerClass::SupportExport,
            ScopeReopenPosture::Degraded(ScopeDegradedReason::RebindingRequired),
            "mono:3",
        );

        assert_eq!(local.stable_scope_id, support.stable_scope_id);
        assert_eq!(local.workset_ref, support.workset_ref);
        assert_eq!(local.included_roots, support.included_roots);
        assert_eq!(local.reopen_state, ScopeReopenState::Exact);
        assert_eq!(support.reopen_state, ScopeReopenState::Degraded);
        assert_eq!(
            support.degraded_reason,
            Some(ScopeDegradedReason::RebindingRequired)
        );
    }
}
