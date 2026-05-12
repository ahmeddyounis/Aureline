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
    pub workset_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
    pub scope_class: ScopeClass,
    #[serde(default)]
    pub workspace_ref: Option<String>,
    pub root_refs: Vec<String>,
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
    EmptyWorksetName,
    EmptyRootRefs,
    DuplicateRootRef(String),
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
            Self::EmptyWorksetName => write!(f, "workset_name must not be empty"),
            Self::EmptyRootRefs => write!(f, "root_refs must contain at least one root"),
            Self::DuplicateRootRef(r) => write!(f, "duplicate root_ref: {r}"),
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
            workset_ref: self.workset_id.clone(),
            scope_class: self.scope_class,
            chip_presentation_state: presentation_state,
            chip_label,
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
            workset_ref: self.workset_id.clone(),
            scope_class: self.scope_class,
            chip_presentation_state: ChipPresentationState::OutsideCurrentScope,
            chip_label: "Outside current scope".to_string(),
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
    pub workset_ref: String,
    pub scope_class: ScopeClass,
    pub chip_presentation_state: ChipPresentationState,
    pub chip_label: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:test:hot:0".to_string(),
            workset_name: "Hot path".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::SelectedWorkset,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string(), "fs-r-1".to_string()],
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
}
