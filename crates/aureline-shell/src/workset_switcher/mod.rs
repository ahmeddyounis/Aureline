//! Workset switcher, scope banner, and scope-diff review projections.
//!
//! This module is the first shell consumer of the portable
//! [`aureline_workspace::WorksetArtifactRecord`] contract. It turns saved
//! worksets and sparse slices into the rows, banners, and review sheets that
//! keep scope boundaries visible before search, graph, refactor, AI, support,
//! or export surfaces widen beyond the active scope.

use serde::{Deserialize, Serialize};

pub mod beta;

pub use beta::{
    render_activation_preview_lines, render_reopen_parity_lines, render_support_export_bundle_lines,
    render_switcher_beta_lines, render_switcher_row_lines,
};

use aureline_workspace::{
    ChipSurfaceClass, HiddenResultCountClass, HiddenResultSummary, MemberRefKind, NarrowingCause,
    PolicyLimitation, ReadinessState, ScopeClass, SourceClass, WorksetArtifactRecord,
};

/// Schema version for the workset-switcher payload family.
pub const WORKSET_SWITCHER_SCHEMA_VERSION: u32 = 1;

/// Schema version for the scope-diff review payload family.
pub const SCOPE_DIFF_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Identifies the concrete workset-switcher payload shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetSwitcherRecordKind {
    /// Parent switcher list containing row records.
    WorksetSwitcherRecord,
    /// One switcher row projected from a workset artifact.
    WorksetSwitcherRowRecord,
    /// Persistent active-scope banner record.
    ScopeBannerRecord,
}

/// Row classes rendered by the workset switcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitcherRowClass {
    /// User-named saved workset row.
    NamedWorksetRow,
    /// Current repository fallback row.
    CurrentRepoFallbackRow,
    /// Full workspace row.
    FullWorkspaceRow,
    /// Sparse slice row.
    SparseSliceRow,
    /// Policy overlay layered over an underlying workset.
    PolicyLimitedOverlayRow,
    /// Managed-provider owned workset row.
    ManagedWorksetRow,
    /// Imported portable workset row.
    ImportedPortableWorksetRow,
    /// Session-only workset row.
    EphemeralSessionRow,
}

/// Presentation state for the persistent scope banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BannerState {
    /// Active scope is narrow and fully ready.
    ActiveNarrowSafe,
    /// Active scope is intentionally narrowed or partially indexed.
    ActivePartial,
    /// Active scope is narrowed by a policy overlay.
    ActivePolicyLimited,
    /// Active scope was widened beyond the named workset.
    ActiveWidened,
    /// Active scope is cold or warming.
    ActiveWarming,
    /// Active banner is disclosing an outside-current-scope result.
    ActiveOutsideCurrentScopeDisclosed,
}

/// Actions a workset switcher row may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitcherAction {
    /// Open the workset as the active scope.
    OpenWorkset,
    /// Open the workset management surface.
    ManageWorksets,
    /// Create a new workset.
    CreateNewWorkset,
    /// Rename the workset artifact.
    RenameWorkset,
    /// Duplicate the workset artifact.
    DuplicateWorkset,
    /// Delete or remove the workset artifact.
    DeleteWorkset,
    /// Export the workset artifact.
    ExportWorksetArtifact,
    /// Copy the stable workset id.
    CopyWorksetId,
    /// Build indexes missing for this scope.
    BuildMissingIndexes,
    /// Open the scope-diff review.
    OpenScopeDiff,
    /// Open the policy overlay on an admin-only surface.
    ViewPolicyOverlayAdminOnly,
    /// Open the scope in a new pane.
    OpenInNewPane,
}

/// Actions the active scope banner may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BannerAction {
    /// Widen to full workspace through the scope-diff path.
    WidenToFullWorkspace,
    /// Widen after an explicit review.
    WidenWithReview,
    /// Narrow to the current repository.
    NarrowToCurrentRepo,
    /// Open the active scope-diff review.
    OpenScopeDiff,
    /// Build indexes missing for this scope.
    BuildMissingIndexes,
    /// Keep the current scope.
    KeepCurrentScope,
    /// Reset to the default saved workset.
    ResetToDefaultWorkset,
    /// Open the workset switcher.
    OpenWorksetSwitcher,
    /// Reveal hidden results on a policy-admin surface.
    RevealHiddenResultsPolicyAdminOnly,
}

/// Trust posture note attached to a switcher row or banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClass {
    /// Scope is fully trusted.
    FullyTrusted,
    /// Scope is trusted with caveats.
    TrustedWithCaveats,
    /// Scope is restricted by a managed provider.
    RestrictedManaged,
    /// Scope is restricted by admin policy.
    RestrictedAdmin,
    /// Scope is untrusted and inspect-only.
    UntrustedInspectOnly,
    /// Trust posture is not yet known.
    TrustUnknown,
}

/// Policy overlay rendered on policy-limited rows and banners.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyOverlayBlock {
    /// Why the policy narrowed the scope.
    pub narrowing_cause: NarrowingCause,
    /// Count of visible members after narrowing.
    pub visible_member_count: u32,
    /// Count of hidden members after narrowing.
    pub hidden_member_count: u32,
    /// Whether the hidden list may be rendered outside policy-admin surfaces.
    pub hidden_member_list_visible: bool,
    /// Opaque policy reference.
    pub policy_ref: String,
}

impl From<&PolicyLimitation> for PolicyOverlayBlock {
    fn from(limit: &PolicyLimitation) -> Self {
        Self {
            narrowing_cause: limit.narrowing_cause,
            visible_member_count: limit.visible_member_count,
            hidden_member_count: limit.hidden_member_count,
            hidden_member_list_visible: limit.hidden_member_list_visible,
            policy_ref: limit.policy_ref.clone(),
        }
    }
}

/// Short trust or policy note rendered below a row or banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustPolicyNote {
    /// Trust class for the scope.
    pub trust_class: TrustClass,
    /// Redaction-aware label.
    pub label: String,
}

/// One workset switcher row projected from a durable artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetSwitcherRowRecord {
    /// Serialized record discriminator.
    pub record_kind: WorksetSwitcherRecordKind,
    /// Schema version for this row.
    pub workset_switcher_schema_version: u32,
    /// Stable row identity.
    pub row_id: String,
    /// Parent switcher id.
    pub switcher_id_ref: String,
    /// Row class.
    pub switcher_row_class: SwitcherRowClass,
    /// Underlying workset artifact id.
    pub workset_ref: String,
    /// Human-readable workset name.
    pub workset_name: String,
    /// Optional subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
    /// Active scope class.
    pub scope_class: ScopeClass,
    /// Distinct root count.
    pub repo_count: u32,
    /// Folder/module member count.
    pub folder_count: u32,
    /// Artifact source class.
    pub source_class: SourceClass,
    /// Artifact readiness state.
    pub readiness_state: ReadinessState,
    /// Whether this row is active.
    pub is_active: bool,
    /// Hidden-result summary from the artifact chip projection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_result_summary: Option<HiddenResultSummary>,
    /// Policy overlay when this row is policy-limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_overlay: Option<PolicyOverlayBlock>,
    /// Trust or policy note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_policy_note: Option<TrustPolicyNote>,
    /// Underlying workset before a policy overlay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlying_workset_ref: Option<String>,
    /// Actions offered by this row.
    pub offered_actions: Vec<SwitcherAction>,
    /// Optional partial index note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_index_note: Option<String>,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
    /// Optional redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Parent workset switcher record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetSwitcherRecord {
    /// Serialized record discriminator.
    pub record_kind: WorksetSwitcherRecordKind,
    /// Schema version for the switcher.
    pub workset_switcher_schema_version: u32,
    /// Stable switcher identity.
    pub switcher_id: String,
    /// Active workspace reference.
    pub workspace_ref: String,
    /// Active workset artifact id.
    pub active_workset_ref: String,
    /// Ordered row list.
    pub rows: Vec<WorksetSwitcherRowRecord>,
    /// Whether create is available.
    #[serde(default)]
    pub supports_create_action: bool,
    /// Whether manage is available.
    #[serde(default)]
    pub supports_manage_action: bool,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
    /// Optional redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Persistent active-scope banner projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeBannerRecord {
    /// Serialized record discriminator.
    pub record_kind: WorksetSwitcherRecordKind,
    /// Schema version for the banner.
    pub workset_switcher_schema_version: u32,
    /// Stable banner identity.
    pub banner_id: String,
    /// Active workspace reference.
    pub workspace_ref: String,
    /// Active workset artifact id.
    pub active_workset_ref: String,
    /// Active scope class.
    pub scope_class: ScopeClass,
    /// Banner presentation state.
    pub banner_state: BannerState,
    /// Resolved banner label.
    pub banner_label: String,
    /// Distinct root count.
    pub repo_count: u32,
    /// Folder/module member count.
    pub folder_count: u32,
    /// Artifact readiness state.
    pub readiness_state: ReadinessState,
    /// Hidden-result summary from the artifact chip projection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_result_summary: Option<HiddenResultSummary>,
    /// Optional partial index note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_index_note: Option<String>,
    /// Policy overlay when the banner is policy-limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_overlay: Option<PolicyOverlayBlock>,
    /// Trust or policy note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_policy_note: Option<TrustPolicyNote>,
    /// Scope widen diff referenced by an active-widened banner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widen_diff_ref: Option<String>,
    /// Actions offered by this banner.
    pub offered_actions: Vec<BannerAction>,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
    /// Optional redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Errors detected while validating switcher and banner projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetSwitcherError {
    /// The schema version is not supported.
    SchemaVersionMismatch(u32),
    /// A required id field is empty.
    EmptyId(&'static str),
    /// The switcher must contain at least one row.
    EmptyRows,
    /// Exactly one active row is required.
    ActiveRowCount { count: usize },
    /// Active row does not match the parent active workset ref.
    ActiveWorksetMismatch,
    /// A row points at a different parent switcher id.
    RowSwitcherMismatch(String),
    /// A row has a missing or forbidden policy overlay.
    PolicyOverlayMismatch(String),
    /// A managed or session row offered artifact export.
    ExportForbidden(String),
    /// Widened banner must carry a diff ref and open-scope-diff action.
    WidenedBannerMissingDiff,
    /// Hidden outside-scope rows require a widen-with-review action.
    MissingWidenWithReview,
}

impl std::fmt::Display for WorksetSwitcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(version) => write!(
                f,
                "unsupported workset_switcher_schema_version {version}; this seed accepts version 1"
            ),
            Self::EmptyId(field) => write!(f, "{field} must not be empty"),
            Self::EmptyRows => write!(f, "workset switcher must contain at least one row"),
            Self::ActiveRowCount { count } => {
                write!(
                    f,
                    "workset switcher must contain exactly one active row, got {count}"
                )
            }
            Self::ActiveWorksetMismatch => {
                write!(f, "active row workset_ref must match active_workset_ref")
            }
            Self::RowSwitcherMismatch(row_id) => {
                write!(f, "row {row_id} points at a different switcher_id_ref")
            }
            Self::PolicyOverlayMismatch(row_id) => {
                write!(f, "row {row_id} policy overlay does not match row class")
            }
            Self::ExportForbidden(row_id) => {
                write!(f, "row {row_id} must not offer export_workset_artifact")
            }
            Self::WidenedBannerMissingDiff => write!(
                f,
                "active_widened banner must carry widen_diff_ref and open_scope_diff"
            ),
            Self::MissingWidenWithReview => write!(
                f,
                "outside-scope hidden result summaries require widen_with_review"
            ),
        }
    }
}

impl std::error::Error for WorksetSwitcherError {}

impl WorksetSwitcherRecord {
    /// Validates parent and row invariants for the switcher projection.
    pub fn validate(&self) -> Result<(), WorksetSwitcherError> {
        if self.workset_switcher_schema_version != WORKSET_SWITCHER_SCHEMA_VERSION {
            return Err(WorksetSwitcherError::SchemaVersionMismatch(
                self.workset_switcher_schema_version,
            ));
        }
        require_non_empty("switcher_id", &self.switcher_id)?;
        require_non_empty("workspace_ref", &self.workspace_ref)?;
        require_non_empty("active_workset_ref", &self.active_workset_ref)?;
        if self.rows.is_empty() {
            return Err(WorksetSwitcherError::EmptyRows);
        }
        let active_count = self.rows.iter().filter(|row| row.is_active).count();
        if active_count != 1 {
            return Err(WorksetSwitcherError::ActiveRowCount {
                count: active_count,
            });
        }
        for row in &self.rows {
            row.validate()?;
            if row.switcher_id_ref != self.switcher_id {
                return Err(WorksetSwitcherError::RowSwitcherMismatch(
                    row.row_id.clone(),
                ));
            }
            if row.is_active && row.workset_ref != self.active_workset_ref {
                return Err(WorksetSwitcherError::ActiveWorksetMismatch);
            }
        }
        Ok(())
    }
}

impl WorksetSwitcherRowRecord {
    /// Validates row-local switcher invariants.
    pub fn validate(&self) -> Result<(), WorksetSwitcherError> {
        if self.workset_switcher_schema_version != WORKSET_SWITCHER_SCHEMA_VERSION {
            return Err(WorksetSwitcherError::SchemaVersionMismatch(
                self.workset_switcher_schema_version,
            ));
        }
        require_non_empty("row_id", &self.row_id)?;
        require_non_empty("switcher_id_ref", &self.switcher_id_ref)?;
        require_non_empty("workset_ref", &self.workset_ref)?;
        require_non_empty("workset_name", &self.workset_name)?;
        let is_policy_row = self.switcher_row_class == SwitcherRowClass::PolicyLimitedOverlayRow;
        if is_policy_row != self.policy_overlay.is_some() {
            return Err(WorksetSwitcherError::PolicyOverlayMismatch(
                self.row_id.clone(),
            ));
        }
        if matches!(
            self.switcher_row_class,
            SwitcherRowClass::ManagedWorksetRow | SwitcherRowClass::EphemeralSessionRow
        ) && self
            .offered_actions
            .contains(&SwitcherAction::ExportWorksetArtifact)
        {
            return Err(WorksetSwitcherError::ExportForbidden(self.row_id.clone()));
        }
        Ok(())
    }
}

impl ScopeBannerRecord {
    /// Validates scope banner invariants.
    pub fn validate(&self) -> Result<(), WorksetSwitcherError> {
        if self.workset_switcher_schema_version != WORKSET_SWITCHER_SCHEMA_VERSION {
            return Err(WorksetSwitcherError::SchemaVersionMismatch(
                self.workset_switcher_schema_version,
            ));
        }
        require_non_empty("banner_id", &self.banner_id)?;
        require_non_empty("workspace_ref", &self.workspace_ref)?;
        require_non_empty("active_workset_ref", &self.active_workset_ref)?;
        if self.banner_state == BannerState::ActiveWidened
            && (self.widen_diff_ref.is_none()
                || !self.offered_actions.contains(&BannerAction::OpenScopeDiff))
        {
            return Err(WorksetSwitcherError::WidenedBannerMissingDiff);
        }
        if self.scope_class == ScopeClass::PolicyLimitedView && self.policy_overlay.is_none() {
            return Err(WorksetSwitcherError::PolicyOverlayMismatch(
                self.banner_id.clone(),
            ));
        }
        if self
            .hidden_result_summary
            .as_ref()
            .is_some_and(|summary| summary.count_class == HiddenResultCountClass::OutsideScopeRoots)
            && !self
                .offered_actions
                .contains(&BannerAction::WidenWithReview)
        {
            return Err(WorksetSwitcherError::MissingWidenWithReview);
        }
        Ok(())
    }
}

/// Projects a workset switcher record from durable workset artifacts.
pub fn project_workset_switcher(
    switcher_id: impl Into<String>,
    workspace_ref: impl Into<String>,
    active_workset_ref: impl AsRef<str>,
    artifacts: &[WorksetArtifactRecord],
    emitted_at: impl Into<String>,
) -> WorksetSwitcherRecord {
    let switcher_id = switcher_id.into();
    let workspace_ref = workspace_ref.into();
    let active_workset_ref = active_workset_ref.as_ref().to_string();
    let emitted_at = emitted_at.into();
    let rows = artifacts
        .iter()
        .map(|artifact| {
            project_switcher_row(
                &switcher_id,
                &active_workset_ref,
                artifact,
                emitted_at.clone(),
            )
        })
        .collect();

    WorksetSwitcherRecord {
        record_kind: WorksetSwitcherRecordKind::WorksetSwitcherRecord,
        workset_switcher_schema_version: WORKSET_SWITCHER_SCHEMA_VERSION,
        switcher_id,
        workspace_ref,
        active_workset_ref,
        rows,
        supports_create_action: artifacts
            .iter()
            .any(|artifact| artifact.portability.source_class != SourceClass::Managed),
        supports_manage_action: true,
        emitted_at,
        notes: None,
    }
}

/// Projects one switcher row from a durable workset artifact.
pub fn project_switcher_row(
    switcher_id: &str,
    active_workset_ref: &str,
    artifact: &WorksetArtifactRecord,
    emitted_at: impl Into<String>,
) -> WorksetSwitcherRowRecord {
    let emitted_at = emitted_at.into();
    let row_id = format!(
        "workset_switcher_row:{}",
        stable_id_fragment(&artifact.workset_id)
    );
    let chip = artifact.project_chip(
        format!(
            "workset_switcher_chip:{}",
            stable_id_fragment(&artifact.workset_id)
        ),
        ChipSurfaceClass::WorksetSwitcher,
        emitted_at.clone(),
    );
    WorksetSwitcherRowRecord {
        record_kind: WorksetSwitcherRecordKind::WorksetSwitcherRowRecord,
        workset_switcher_schema_version: WORKSET_SWITCHER_SCHEMA_VERSION,
        row_id,
        switcher_id_ref: switcher_id.to_string(),
        switcher_row_class: row_class_for(artifact),
        workset_ref: artifact.workset_id.clone(),
        workset_name: artifact.workset_name.clone(),
        presentation_subtitle: artifact.presentation_subtitle.clone(),
        scope_class: artifact.scope_class,
        repo_count: artifact.root_refs.len() as u32,
        folder_count: folder_count(artifact),
        source_class: artifact.portability.source_class,
        readiness_state: artifact.readiness.readiness_state,
        is_active: artifact.workset_id == active_workset_ref,
        hidden_result_summary: chip.hidden_result_summary,
        policy_overlay: artifact
            .policy_limitation
            .as_ref()
            .map(PolicyOverlayBlock::from),
        trust_policy_note: None,
        underlying_workset_ref: artifact
            .policy_limitation
            .as_ref()
            .map(|limit| limit.underlying_workset_ref.clone()),
        offered_actions: switcher_actions_for(artifact),
        partial_index_note: artifact.readiness.partial_index_note.clone(),
        emitted_at,
        notes: None,
    }
}

/// Projects the persistent scope banner from the active artifact.
pub fn project_scope_banner(
    banner_id: impl Into<String>,
    workspace_ref: impl Into<String>,
    artifact: &WorksetArtifactRecord,
    trust_policy_note: Option<TrustPolicyNote>,
    widen_diff_ref: Option<String>,
    emitted_at: impl Into<String>,
) -> ScopeBannerRecord {
    let emitted_at = emitted_at.into();
    let chip = artifact.project_chip(
        format!(
            "scope_banner_chip:{}",
            stable_id_fragment(&artifact.workset_id)
        ),
        ChipSurfaceClass::ScopeBanner,
        emitted_at.clone(),
    );
    let hidden = chip.hidden_result_summary;
    let banner_state = banner_state_for(artifact, hidden.as_ref(), widen_diff_ref.as_deref());
    let policy_overlay = artifact
        .policy_limitation
        .as_ref()
        .map(PolicyOverlayBlock::from);
    ScopeBannerRecord {
        record_kind: WorksetSwitcherRecordKind::ScopeBannerRecord,
        workset_switcher_schema_version: WORKSET_SWITCHER_SCHEMA_VERSION,
        banner_id: banner_id.into(),
        workspace_ref: workspace_ref.into(),
        active_workset_ref: artifact.workset_id.clone(),
        scope_class: artifact.scope_class,
        banner_state,
        banner_label: banner_label_for(artifact),
        repo_count: artifact.root_refs.len() as u32,
        folder_count: folder_count(artifact),
        readiness_state: artifact.readiness.readiness_state,
        hidden_result_summary: hidden.clone(),
        partial_index_note: artifact.readiness.partial_index_note.clone(),
        policy_overlay,
        trust_policy_note,
        widen_diff_ref: widen_diff_ref.clone(),
        offered_actions: banner_actions_for(
            artifact,
            hidden.as_ref(),
            banner_state,
            widen_diff_ref,
        ),
        emitted_at,
        notes: None,
    }
}

fn require_non_empty(field: &'static str, value: &str) -> Result<(), WorksetSwitcherError> {
    if value.is_empty() {
        Err(WorksetSwitcherError::EmptyId(field))
    } else {
        Ok(())
    }
}

fn folder_count(artifact: &WorksetArtifactRecord) -> u32 {
    artifact
        .member_refs
        .iter()
        .filter(|member| member.ref_kind != MemberRefKind::Root)
        .count() as u32
}

fn row_class_for(artifact: &WorksetArtifactRecord) -> SwitcherRowClass {
    if artifact.scope_class == ScopeClass::PolicyLimitedView {
        return SwitcherRowClass::PolicyLimitedOverlayRow;
    }
    match artifact.portability.source_class {
        SourceClass::Managed => SwitcherRowClass::ManagedWorksetRow,
        SourceClass::ProfileImported => SwitcherRowClass::ImportedPortableWorksetRow,
        SourceClass::EphemeralSession => SwitcherRowClass::EphemeralSessionRow,
        SourceClass::LocalOnly | SourceClass::WorkspaceShared => match artifact.scope_class {
            ScopeClass::CurrentRepo => SwitcherRowClass::CurrentRepoFallbackRow,
            ScopeClass::FullWorkspace => SwitcherRowClass::FullWorkspaceRow,
            ScopeClass::SparseSlice => SwitcherRowClass::SparseSliceRow,
            ScopeClass::SelectedWorkset => SwitcherRowClass::NamedWorksetRow,
            ScopeClass::PolicyLimitedView => SwitcherRowClass::PolicyLimitedOverlayRow,
        },
    }
}

fn switcher_actions_for(artifact: &WorksetArtifactRecord) -> Vec<SwitcherAction> {
    let mut actions = vec![SwitcherAction::OpenWorkset, SwitcherAction::CopyWorksetId];
    if matches!(
        artifact.readiness.readiness_state,
        ReadinessState::Cold | ReadinessState::Warming | ReadinessState::Partial
    ) {
        actions.push(SwitcherAction::BuildMissingIndexes);
    }
    if matches!(
        artifact.scope_class,
        ScopeClass::SelectedWorkset | ScopeClass::SparseSlice | ScopeClass::PolicyLimitedView
    ) {
        actions.push(SwitcherAction::OpenScopeDiff);
    }
    match artifact.portability.source_class {
        SourceClass::Managed => {}
        SourceClass::EphemeralSession => actions.push(SwitcherAction::DeleteWorkset),
        SourceClass::LocalOnly | SourceClass::WorkspaceShared | SourceClass::ProfileImported => {
            if !matches!(
                artifact.scope_class,
                ScopeClass::CurrentRepo | ScopeClass::FullWorkspace
            ) {
                actions.push(SwitcherAction::RenameWorkset);
                actions.push(SwitcherAction::DuplicateWorkset);
                actions.push(SwitcherAction::DeleteWorkset);
            }
            actions.push(SwitcherAction::ExportWorksetArtifact);
        }
    }
    if artifact
        .policy_limitation
        .as_ref()
        .is_some_and(|limit| limit.hidden_member_list_visible)
    {
        actions.push(SwitcherAction::ViewPolicyOverlayAdminOnly);
    }
    actions.push(SwitcherAction::OpenInNewPane);
    actions
}

fn banner_state_for(
    artifact: &WorksetArtifactRecord,
    hidden: Option<&HiddenResultSummary>,
    widen_diff_ref: Option<&str>,
) -> BannerState {
    if widen_diff_ref.is_some() {
        return BannerState::ActiveWidened;
    }
    if artifact.scope_class == ScopeClass::PolicyLimitedView {
        return BannerState::ActivePolicyLimited;
    }
    if matches!(
        artifact.readiness.readiness_state,
        ReadinessState::Cold | ReadinessState::Warming
    ) {
        return BannerState::ActiveWarming;
    }
    if matches!(artifact.readiness.readiness_state, ReadinessState::Partial)
        || artifact.has_partial_member_truth()
        || hidden.is_some_and(|summary| summary.count.unwrap_or(0) > 0)
    {
        return BannerState::ActivePartial;
    }
    BannerState::ActiveNarrowSafe
}

fn banner_label_for(artifact: &WorksetArtifactRecord) -> String {
    match artifact.scope_class {
        ScopeClass::CurrentRepo => "Current repo".to_string(),
        ScopeClass::FullWorkspace => "Full workspace".to_string(),
        ScopeClass::SelectedWorkset => format!("Selected workset: {}", artifact.workset_name),
        ScopeClass::SparseSlice => format!("Sparse slice: {}", artifact.workset_name),
        ScopeClass::PolicyLimitedView => "Policy-limited view".to_string(),
    }
}

fn banner_actions_for(
    artifact: &WorksetArtifactRecord,
    hidden: Option<&HiddenResultSummary>,
    banner_state: BannerState,
    widen_diff_ref: Option<String>,
) -> Vec<BannerAction> {
    let mut actions = vec![
        BannerAction::KeepCurrentScope,
        BannerAction::OpenWorksetSwitcher,
    ];
    if matches!(
        artifact.readiness.readiness_state,
        ReadinessState::Cold | ReadinessState::Warming | ReadinessState::Partial
    ) {
        actions.push(BannerAction::BuildMissingIndexes);
    }
    match artifact.scope_class {
        ScopeClass::CurrentRepo | ScopeClass::SelectedWorkset | ScopeClass::SparseSlice => {
            actions.push(BannerAction::WidenWithReview);
            actions.push(BannerAction::WidenToFullWorkspace);
        }
        ScopeClass::FullWorkspace => {
            actions.push(BannerAction::NarrowToCurrentRepo);
        }
        ScopeClass::PolicyLimitedView => {
            actions.push(BannerAction::WidenWithReview);
        }
    }
    if banner_state == BannerState::ActiveWidened || widen_diff_ref.is_some() {
        actions.push(BannerAction::OpenScopeDiff);
    }
    if hidden
        .is_some_and(|summary| summary.count_class == HiddenResultCountClass::OutsideScopeRoots)
        && !actions.contains(&BannerAction::WidenWithReview)
    {
        actions.push(BannerAction::WidenWithReview);
    }
    if artifact
        .policy_limitation
        .as_ref()
        .is_some_and(|limit| limit.hidden_member_list_visible)
    {
        actions.push(BannerAction::RevealHiddenResultsPolicyAdminOnly);
    }
    actions.push(BannerAction::ResetToDefaultWorkset);
    actions
}

fn stable_id_fragment(value: &str) -> String {
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

/// Identifies a scope-diff review record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDiffReviewRecordKind {
    /// Scope-diff review sheet payload.
    ScopeDiffReviewRecord,
}

/// Lifecycle state of a scope-diff review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    /// The review is waiting for user input.
    PendingUserReview,
    /// The user is actively reviewing the diff.
    InReview,
    /// The widen or narrow was confirmed but not yet applied.
    Confirmed,
    /// The user cancelled the widen or narrow.
    Cancelled,
    /// Trust posture blocks the review.
    BlockedByTrust,
    /// Policy posture blocks the review.
    BlockedByPolicy,
    /// Remote availability blocks the review.
    BlockedByUnavailableRemote,
    /// Candidate scope is active.
    Applied,
}

impl ReviewState {
    /// Returns true when confirm actions are forbidden.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedByTrust | Self::BlockedByPolicy | Self::BlockedByUnavailableRemote
        )
    }
}

/// Surface family that raised a scope-diff review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDiffTriggerClass {
    /// Banner widen action.
    BannerWiden,
    /// Switcher open action.
    SwitcherOpen,
    /// Search widen action.
    SearchWidenWithReview,
    /// Cross-repo jump widen action.
    CrossRepoJumpWiden,
    /// Symbol jump widen action.
    SymbolJumpWiden,
    /// Topology map widen action.
    TopologyWiden,
    /// Impact explorer widen action.
    ImpactExplorerWiden,
    /// Cited explainer widen action.
    CitedExplainerWiden,
    /// AI context widen action.
    AiContextWiden,
    /// Refactor widen action.
    RefactorWiden,
    /// Review semantic hint widen action.
    ReviewSemanticHintWiden,
    /// Support export widen action.
    SupportExportWiden,
    /// Navigation deep-link widen action.
    NavigationDeepLinkWiden,
}

/// Consumer surfaces that can render a scope-diff review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDiffConsumerSurfaceClass {
    /// Scope banner surface.
    ScopeBanner,
    /// Workset switcher surface.
    WorksetSwitcher,
    /// Full search surface.
    FullSearch,
    /// Quick open surface.
    QuickOpen,
    /// Symbol jump surface.
    SymbolJump,
    /// Topology map surface.
    TopologyMap,
    /// Impact explorer surface.
    ImpactExplorer,
    /// Cited explainer surface.
    CitedExplainer,
    /// AI context inspector surface.
    AiContextInspector,
    /// Review semantic hint surface.
    ReviewSemanticHint,
    /// Refactor preview surface.
    RefactorPreview,
    /// Support export surface.
    SupportExport,
    /// Navigation deep-link surface.
    NavigationDeepLink,
}

/// Root-level change class in a scope-diff review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootChangeClass {
    /// Root is added by the candidate scope.
    RootAdded,
    /// Root is removed by the candidate scope.
    RootRemoved,
}

/// Module kind changed by a scope-diff review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleKind {
    /// Folder member.
    Folder,
    /// Module member.
    Module,
    /// Manifest entry member.
    ManifestEntry,
    /// Graph seed member.
    GraphSeed,
}

/// Module-level change class in a scope-diff review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleChangeClass {
    /// Module is added by the candidate scope.
    ModuleAdded,
    /// Module is removed by the candidate scope.
    ModuleRemoved,
    /// Pattern admits more content in the candidate.
    PatternBroadened,
    /// Pattern admits less content in the candidate.
    PatternNarrowed,
}

/// Expected runtime cost class for a scope diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedRuntimeCostClass {
    /// No runtime cost is expected.
    None,
    /// Runtime cost is cheap.
    Cheap,
    /// Runtime cost is moderate.
    Moderate,
    /// Runtime cost is expensive.
    Expensive,
    /// Runtime cost is very expensive and remote-only.
    VeryExpensiveRemoteOnly,
}

/// Remote or cache source class disclosed by a review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteOrCacheSourceClass {
    /// Workspace-local cache source.
    WorkspaceLocalCache,
    /// Profile-local cache source.
    ProfileLocalCache,
    /// Machine-local cache source.
    MachineLocalCache,
    /// Remote authority is required.
    RemoteAuthoritativeRequired,
    /// Remote authority is optional.
    RemoteAuthoritativeOptional,
    /// Imported remote index source.
    ImportedRemoteIndex,
    /// Imported provider index source.
    ImportedProviderIndex,
    /// External provider index source.
    ExternalProviderIndex,
    /// Accelerator index source.
    AcceleratorIndex,
    /// No external source is required.
    NoExternalSourceRequired,
}

impl RemoteOrCacheSourceClass {
    /// Returns true when the class requires remote round-trip disclosure.
    pub const fn requires_round_trip_disclosure(self) -> bool {
        matches!(
            self,
            Self::RemoteAuthoritativeRequired
                | Self::ImportedRemoteIndex
                | Self::ImportedProviderIndex
                | Self::ExternalProviderIndex
        )
    }
}

/// Trust-stage impact of a candidate scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStageImpactClass {
    /// Trust posture is unchanged.
    NoChange,
    /// Candidate requires a trust review.
    RequiresTrustReview,
    /// Candidate requires a trust uplift.
    RequiresTrustUplift,
    /// Active trust stage blocks the candidate.
    BlockedByActiveTrustStage,
}

/// Policy impact of a candidate scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyImpactClass {
    /// Policy posture is unchanged.
    NoChange,
    /// A policy overlay is added.
    LayeredPolicyOverlayAdded,
    /// A policy overlay is removed.
    PolicyOverlayRemoved,
    /// Admin policy blocks the candidate.
    BlockedByAdminPolicy,
    /// License or export-control policy blocks the candidate.
    BlockedByLicenseOrExportControl,
}

/// Actions a scope-diff review may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAction {
    /// Confirm widening.
    ConfirmWiden,
    /// Confirm narrowing.
    ConfirmNarrow,
    /// Cancel widening and keep the current scope.
    CancelWidenKeepCurrentScope,
    /// Request a trust review.
    RequestTrustReview,
    /// Request policy-admin review.
    RequestPolicyReviewAdminOnly,
    /// Snapshot the current scope.
    SnapshotCurrentScope,
    /// Remember this scope pair decision for the same future trigger.
    RememberScopeChoice,
    /// Copy the diff id.
    CopyDiffId,
    /// Open the underlying artifact.
    OpenUnderlyingArtifact,
    /// Narrow to the current repository.
    NarrowToCurrentRepo,
    /// Build missing indexes.
    BuildMissingIndexes,
    /// Fetch missing objects.
    FetchMissingObjects,
}

/// One root added or removed by the review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDiffRootEntry {
    /// Root identity.
    pub root_ref: String,
    /// Redaction-aware root label.
    pub root_label: String,
    /// Root change class.
    pub change_class: RootChangeClass,
    /// Whether an imported root is disclosed.
    #[serde(default)]
    pub imported_root_disclosed: bool,
    /// Whether a managed provider root is disclosed.
    #[serde(default)]
    pub managed_provider_disclosed: bool,
    /// Optional subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
}

/// One module, folder, manifest entry, or graph seed added or removed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDiffModuleEntry {
    /// Module identity.
    pub module_ref: String,
    /// Redaction-aware module label.
    pub module_label: String,
    /// Module kind.
    pub module_kind: ModuleKind,
    /// Module change class.
    pub change_class: ModuleChangeClass,
    /// Containing root reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_root_ref: Option<String>,
    /// Optional subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
}

/// Aggregate pattern-change counts for a review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternChangeSummary {
    /// Count of broadened pattern rows.
    pub broadened_count: u32,
    /// Count of narrowed pattern rows.
    pub narrowed_count: u32,
    /// Count of conflicting pattern rows.
    #[serde(default)]
    pub conflicting_pattern_count: u32,
}

/// One remote or cache source note rendered on the review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteOrCacheSourceNote {
    /// Source class.
    pub source_class: RemoteOrCacheSourceClass,
    /// Optional source label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_label: Option<String>,
    /// Whether remote round-trip cost is disclosed.
    #[serde(default)]
    pub remote_round_trip_disclosed: bool,
}

/// Portability and availability impact disclosed on a review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportAvailabilityImpactBlock {
    /// Whether portability changes.
    pub changes_portability: bool,
    /// Whether readiness changes.
    pub changes_readiness: bool,
    /// Whether managed provider refs are introduced.
    pub includes_managed_provider_refs_after_widen: bool,
    /// Whether machine-local refs are introduced.
    pub includes_machine_local_refs_after_widen: bool,
    /// Whether support export completeness is reduced.
    pub reduces_support_export_completeness: bool,
    /// Whether offline capability is reduced.
    pub reduces_offline_capability: bool,
    /// Whether the impact block was disclosed.
    pub support_availability_impact_disclosed: bool,
}

impl SupportAvailabilityImpactBlock {
    /// Returns true when any impact flag must be disclosed.
    pub const fn has_material_impact(&self) -> bool {
        self.changes_portability
            || self.changes_readiness
            || self.includes_managed_provider_refs_after_widen
            || self.includes_machine_local_refs_after_widen
            || self.reduces_support_export_completeness
            || self.reduces_offline_capability
    }
}

/// User-visible review-sheet projection for a scope diff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDiffReviewRecord {
    /// Serialized record discriminator.
    pub record_kind: ScopeDiffReviewRecordKind,
    /// Schema version for this review.
    pub scope_diff_review_schema_version: u32,
    /// Stable review identity.
    pub review_id: String,
    /// Underlying scope widen diff reference.
    pub diff_ref: String,
    /// Active workset before the review.
    pub base_workset_ref: String,
    /// Candidate workset after the review.
    pub candidate_workset_ref: String,
    /// Base scope class.
    pub base_scope_class: ScopeClass,
    /// Candidate scope class.
    pub candidate_scope_class: ScopeClass,
    /// Surface that raised the review.
    pub trigger_class: ScopeDiffTriggerClass,
    /// Surfaces that render the review.
    pub consumer_surface_class: Vec<ScopeDiffConsumerSurfaceClass>,
    /// Review lifecycle state.
    pub review_state: ReviewState,
    /// Roots added by the candidate.
    pub added_roots: Vec<ScopeDiffRootEntry>,
    /// Roots removed by the candidate.
    pub removed_roots: Vec<ScopeDiffRootEntry>,
    /// Modules added by the candidate.
    pub added_modules: Vec<ScopeDiffModuleEntry>,
    /// Modules removed by the candidate.
    pub removed_modules: Vec<ScopeDiffModuleEntry>,
    /// Aggregate pattern-change summary.
    pub pattern_change_summary: PatternChangeSummary,
    /// Expected index cost class.
    pub expected_index_cost_class: aureline_workspace::ExpectedIndexCostClass,
    /// Expected runtime cost class.
    pub expected_runtime_cost_class: ExpectedRuntimeCostClass,
    /// Optional runtime cost note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_runtime_cost_note: Option<String>,
    /// Whether remote fetch is required.
    pub remote_fetch_required: bool,
    /// Remote or cache sources affected by the candidate.
    pub remote_or_cache_source_notes: Vec<RemoteOrCacheSourceNote>,
    /// Support, availability, and portability impact.
    pub support_availability_impact: SupportAvailabilityImpactBlock,
    /// Trust-stage impact class.
    pub trust_stage_impact_class: TrustStageImpactClass,
    /// Policy impact class.
    pub policy_impact_class: PolicyImpactClass,
    /// Actions offered by the review.
    pub offered_actions: Vec<ReviewAction>,
    /// Optional subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
    /// Producer-local monotonic timestamp.
    pub emitted_at: String,
    /// Optional redaction-aware notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Errors detected while validating a [`ScopeDiffReviewRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeDiffReviewError {
    /// The schema version is not supported.
    SchemaVersionMismatch(u32),
    /// A required id field is empty.
    EmptyId(&'static str),
    /// Base and candidate must be different workset identities.
    CandidateReusesBaseIdentity,
    /// A review must contain at least one consumer surface.
    EmptyConsumerSurface,
    /// A review must contain at least one source note.
    EmptySourceNotes,
    /// A review must contain at least one diff signal.
    EmptyDiffSignal,
    /// A blocked review offered a confirm action.
    BlockedReviewOffersConfirm,
    /// A trust block must set blocked-by-trust state.
    TrustBlockStateMismatch,
    /// A policy block must set blocked-by-policy state.
    PolicyBlockStateMismatch,
    /// Trust review action is missing.
    MissingTrustReviewAction,
    /// Cancel route is missing.
    MissingCancelRoute,
    /// Remote fetch disclosure is missing or inconsistent.
    RemoteFetchDisclosureMismatch,
    /// Support availability impact was not disclosed.
    SupportImpactNotDisclosed,
}

impl std::fmt::Display for ScopeDiffReviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(version) => write!(
                f,
                "unsupported scope_diff_review_schema_version {version}; this seed accepts version 1"
            ),
            Self::EmptyId(field) => write!(f, "{field} must not be empty"),
            Self::CandidateReusesBaseIdentity => {
                write!(f, "candidate_workset_ref must not equal base_workset_ref")
            }
            Self::EmptyConsumerSurface => {
                write!(f, "consumer_surface_class must contain at least one surface")
            }
            Self::EmptySourceNotes => {
                write!(f, "remote_or_cache_source_notes must contain at least one row")
            }
            Self::EmptyDiffSignal => write!(f, "scope-diff review must contain a diff signal"),
            Self::BlockedReviewOffersConfirm => {
                write!(f, "blocked reviews must not offer confirm actions")
            }
            Self::TrustBlockStateMismatch => write!(
                f,
                "blocked_by_active_trust_stage requires review_state blocked_by_trust"
            ),
            Self::PolicyBlockStateMismatch => write!(
                f,
                "blocked policy impact requires review_state blocked_by_policy"
            ),
            Self::MissingTrustReviewAction => write!(
                f,
                "trust review or uplift impact requires request_trust_review"
            ),
            Self::MissingCancelRoute => {
                write!(f, "review must offer cancel_widen_keep_current_scope or narrow_to_current_repo")
            }
            Self::RemoteFetchDisclosureMismatch => {
                write!(f, "remote fetch requirement or source disclosure is inconsistent")
            }
            Self::SupportImpactNotDisclosed => {
                write!(f, "material support/availability impact must be disclosed")
            }
        }
    }
}

impl std::error::Error for ScopeDiffReviewError {}

impl ScopeDiffReviewRecord {
    /// Validates review-sheet invariants that prevent silent widening.
    pub fn validate(&self) -> Result<(), ScopeDiffReviewError> {
        if self.scope_diff_review_schema_version != SCOPE_DIFF_REVIEW_SCHEMA_VERSION {
            return Err(ScopeDiffReviewError::SchemaVersionMismatch(
                self.scope_diff_review_schema_version,
            ));
        }
        require_review_non_empty("review_id", &self.review_id)?;
        require_review_non_empty("diff_ref", &self.diff_ref)?;
        require_review_non_empty("base_workset_ref", &self.base_workset_ref)?;
        require_review_non_empty("candidate_workset_ref", &self.candidate_workset_ref)?;
        if self.base_workset_ref == self.candidate_workset_ref {
            return Err(ScopeDiffReviewError::CandidateReusesBaseIdentity);
        }
        if self.consumer_surface_class.is_empty() {
            return Err(ScopeDiffReviewError::EmptyConsumerSurface);
        }
        if self.remote_or_cache_source_notes.is_empty() {
            return Err(ScopeDiffReviewError::EmptySourceNotes);
        }
        let pattern_signal = self.pattern_change_summary.broadened_count > 0
            || self.pattern_change_summary.narrowed_count > 0
            || self.pattern_change_summary.conflicting_pattern_count > 0;
        if self.added_roots.is_empty()
            && self.removed_roots.is_empty()
            && self.added_modules.is_empty()
            && self.removed_modules.is_empty()
            && !pattern_signal
        {
            return Err(ScopeDiffReviewError::EmptyDiffSignal);
        }
        let offers_confirm = self.offered_actions.contains(&ReviewAction::ConfirmWiden)
            || self.offered_actions.contains(&ReviewAction::ConfirmNarrow);
        if self.review_state.is_blocked() && offers_confirm {
            return Err(ScopeDiffReviewError::BlockedReviewOffersConfirm);
        }
        if self.trust_stage_impact_class == TrustStageImpactClass::BlockedByActiveTrustStage
            && self.review_state != ReviewState::BlockedByTrust
            && self.policy_impact_class == PolicyImpactClass::NoChange
        {
            return Err(ScopeDiffReviewError::TrustBlockStateMismatch);
        }
        if matches!(
            self.policy_impact_class,
            PolicyImpactClass::BlockedByAdminPolicy
                | PolicyImpactClass::BlockedByLicenseOrExportControl
        ) && self.review_state != ReviewState::BlockedByPolicy
        {
            return Err(ScopeDiffReviewError::PolicyBlockStateMismatch);
        }
        if matches!(
            self.trust_stage_impact_class,
            TrustStageImpactClass::RequiresTrustReview
                | TrustStageImpactClass::RequiresTrustUplift
                | TrustStageImpactClass::BlockedByActiveTrustStage
        ) && !self
            .offered_actions
            .contains(&ReviewAction::RequestTrustReview)
        {
            return Err(ScopeDiffReviewError::MissingTrustReviewAction);
        }
        if !self.offered_actions.iter().any(|action| {
            matches!(
                action,
                ReviewAction::CancelWidenKeepCurrentScope | ReviewAction::NarrowToCurrentRepo
            )
        }) {
            return Err(ScopeDiffReviewError::MissingCancelRoute);
        }
        if self.expected_runtime_cost_class == ExpectedRuntimeCostClass::VeryExpensiveRemoteOnly
            && !self.remote_fetch_required
        {
            return Err(ScopeDiffReviewError::RemoteFetchDisclosureMismatch);
        }
        if self.expected_index_cost_class
            == aureline_workspace::ExpectedIndexCostClass::RemoteFetchRequired
            && !self.remote_fetch_required
        {
            return Err(ScopeDiffReviewError::RemoteFetchDisclosureMismatch);
        }
        let any_remote_disclosure = self
            .remote_or_cache_source_notes
            .iter()
            .any(|note| note.remote_round_trip_disclosed);
        if self.remote_fetch_required && !any_remote_disclosure {
            return Err(ScopeDiffReviewError::RemoteFetchDisclosureMismatch);
        }
        if self.remote_or_cache_source_notes.iter().any(|note| {
            note.source_class.requires_round_trip_disclosure() && !note.remote_round_trip_disclosed
        }) {
            return Err(ScopeDiffReviewError::RemoteFetchDisclosureMismatch);
        }
        if self.support_availability_impact.has_material_impact()
            && !self
                .support_availability_impact
                .support_availability_impact_disclosed
        {
            return Err(ScopeDiffReviewError::SupportImpactNotDisclosed);
        }
        Ok(())
    }

    /// Returns true when the review exposes a remember-choice posture.
    pub fn has_remember_choice_option(&self) -> bool {
        self.offered_actions
            .contains(&ReviewAction::RememberScopeChoice)
    }
}

fn require_review_non_empty(field: &'static str, value: &str) -> Result<(), ScopeDiffReviewError> {
    if value.is_empty() {
        Err(ScopeDiffReviewError::EmptyId(field))
    } else {
        Ok(())
    }
}
