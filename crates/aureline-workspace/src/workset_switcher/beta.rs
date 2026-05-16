//! Workset switcher beta — typed switcher row, activation preview, and
//! reopen-parity packet.
//!
//! See the crate-level module doc in `workset_switcher/mod.rs` for the
//! contract overview. This file owns the record shapes, validation, and the
//! projection helpers consumed by the first shell consumer.

use serde::{Deserialize, Serialize};

use crate::roots::WorkspaceRootKind;
use crate::worksets::{
    HiddenResultSummary, IncludedRootRef, NarrowingCause, PatternEntry, PolicyLimitation,
    PortabilityClass, ReadinessState, ScopeClass, ScopeDegradedReason, ScopeDiffClass,
    ScopeDiffEntry, ScopeMode, ScopeReopenPosture, ScopeReopenState, ScopeWidenDiffError,
    ScopeWidenDiffRecord, ScopeWidenDiffRecordKind, SourceClass, WorksetArtifactRecord,
    WorksetScopeConsumerBinding, WorksetScopeConsumerClass,
};

/// Schema version for every beta workset-switcher payload in this module.
pub const WORKSET_SWITCHER_BETA_SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for [`WorksetSwitcherBetaRecord`].
pub const WORKSET_SWITCHER_BETA_RECORD_KIND: &str = "workset_switcher_beta_record";
/// Record-kind discriminator for [`WorksetSwitcherBetaRow`].
pub const WORKSET_SWITCHER_BETA_ROW_RECORD_KIND: &str = "workset_switcher_beta_row";
/// Record-kind discriminator for [`WorksetActivationPreview`].
pub const WORKSET_ACTIVATION_PREVIEW_RECORD_KIND: &str = "workset_activation_preview";
/// Record-kind discriminator for [`WorksetReopenParityPacket`].
pub const WORKSET_REOPEN_PARITY_PACKET_RECORD_KIND: &str = "workset_reopen_parity_packet";
/// Record-kind discriminator for [`WorksetSwitcherBetaSupportExport`].
pub const WORKSET_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "workset_switcher_beta_support_export";

/// Closed portability label rendered on the switcher row and quoted by
/// support / export consumers.
///
/// This is the user-visible portability truth derived from the artifact's
/// `portability_class`, `source_class`, and policy overlay. A row never
/// invents a parallel label — the reviewer always reads one of these tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetPortabilityLabel {
    /// Identity + member refs round-trip cleanly across export/import.
    Portable,
    /// Identity survives export/import; member refs rebind on the new host.
    PortableWithRebinding,
    /// Refs touch machine-local paths or session-only state — survives this
    /// host only.
    LocalOnly,
    /// Scope is layered with a policy overlay; portability is conditional.
    PolicyLimited,
    /// Owned by a managed provider; cannot leave the provider boundary.
    ManagedProviderLocked,
}

impl WorksetPortabilityLabel {
    /// Stable string vocabulary used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::PortableWithRebinding => "portable_with_rebinding",
            Self::LocalOnly => "local_only",
            Self::PolicyLimited => "policy_limited",
            Self::ManagedProviderLocked => "managed_provider_locked",
        }
    }

    /// Returns true when the label forbids export off the producing host.
    pub const fn forbids_export(self) -> bool {
        matches!(self, Self::ManagedProviderLocked)
    }
}

/// Derives the closed [`WorksetPortabilityLabel`] from the durable artifact.
pub fn derive_portability_label(artifact: &WorksetArtifactRecord) -> WorksetPortabilityLabel {
    if artifact.scope_class == ScopeClass::PolicyLimitedView || artifact.policy_limitation.is_some()
    {
        if artifact.portability.portability_class == PortabilityClass::ManagedProviderLocked {
            return WorksetPortabilityLabel::ManagedProviderLocked;
        }
        return WorksetPortabilityLabel::PolicyLimited;
    }
    match artifact.portability.portability_class {
        PortabilityClass::FullyPortable => WorksetPortabilityLabel::Portable,
        PortabilityClass::PortableWithRebinding => WorksetPortabilityLabel::PortableWithRebinding,
        PortabilityClass::MachineLocalOnly => WorksetPortabilityLabel::LocalOnly,
        PortabilityClass::ManagedProviderLocked => WorksetPortabilityLabel::ManagedProviderLocked,
    }
}

/// Closed action vocabulary the beta switcher row may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitcherRowAction {
    /// Activate this workset; the chrome opens an activation preview first
    /// when the candidate differs from the active artifact.
    OpenWorkset,
    /// Open the typed activation preview before applying.
    PreviewActivationDiff,
    /// Copy the stable workset id for support tickets / pasting.
    CopyWorksetId,
    /// Export the workset artifact when portability permits it.
    ExportWorksetArtifact,
    /// Open the underlying workset on a policy-admin surface.
    OpenUnderlyingWorkset,
    /// Trigger a remote / headless reopen-parity check.
    ReopenAcrossSurfaces,
    /// Open this workset in a new pane without changing the active scope.
    OpenInNewPane,
}

impl SwitcherRowAction {
    /// Stable string vocabulary for the offered action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenWorkset => "open_workset",
            Self::PreviewActivationDiff => "preview_activation_diff",
            Self::CopyWorksetId => "copy_workset_id",
            Self::ExportWorksetArtifact => "export_workset_artifact",
            Self::OpenUnderlyingWorkset => "open_underlying_workset",
            Self::ReopenAcrossSurfaces => "reopen_across_surfaces",
            Self::OpenInNewPane => "open_in_new_pane",
        }
    }
}

/// Closed scope-drift class returned by [`WorksetActivationPreview`].
///
/// The chrome reads this single token to colour the preview header
/// (additive / narrowing / mixed / portability / readiness / identity-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDriftClass {
    /// Candidate is the active artifact — same identity, nothing to apply.
    SameIdentity,
    /// Candidate adds roots / broadens patterns / widens policy.
    Widens,
    /// Candidate removes roots / narrows patterns / narrows policy.
    Narrows,
    /// Candidate both widens and narrows.
    Mixed,
    /// Candidate only changes portability or readiness posture.
    PortabilityOrReadinessOnly,
    /// Candidate only changes presentation (rename / subtitle).
    PresentationOnly,
}

impl ScopeDriftClass {
    /// Stable string vocabulary for the drift class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameIdentity => "same_identity",
            Self::Widens => "widens",
            Self::Narrows => "narrows",
            Self::Mixed => "mixed",
            Self::PortabilityOrReadinessOnly => "portability_or_readiness_only",
            Self::PresentationOnly => "presentation_only",
        }
    }
}

/// Policy-overlay block carried on a [`WorksetSwitcherBetaRow`].
///
/// Mirrors the durable [`PolicyLimitation`] but never carries the hidden
/// member list — admin-policy and license/export-control overlays MUST keep
/// `hidden_member_list_visible = false` per the alpha contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyOverlaySummary {
    pub narrowing_cause: NarrowingCause,
    pub visible_member_count: u32,
    pub hidden_member_count: u32,
    pub hidden_member_list_visible: bool,
    pub policy_ref: String,
    pub underlying_workset_ref: String,
}

impl From<&PolicyLimitation> for PolicyOverlaySummary {
    fn from(limit: &PolicyLimitation) -> Self {
        Self {
            narrowing_cause: limit.narrowing_cause,
            visible_member_count: limit.visible_member_count,
            hidden_member_count: limit.hidden_member_count,
            hidden_member_list_visible: limit.hidden_member_list_visible,
            policy_ref: limit.policy_ref.clone(),
            underlying_workset_ref: limit.underlying_workset_ref.clone(),
        }
    }
}

/// One beta switcher row projected from a durable [`WorksetArtifactRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetSwitcherBetaRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub workset_ref: String,
    pub stable_scope_id: String,
    pub workset_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
    pub scope_class: ScopeClass,
    pub scope_mode: ScopeMode,
    pub source_class: SourceClass,
    pub portability_class: PortabilityClass,
    pub portability_label: WorksetPortabilityLabel,
    pub readiness_state: ReadinessState,
    pub repo_count: u32,
    pub folder_count: u32,
    pub is_active: bool,
    pub root_taxonomy: Vec<IncludedRootRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include_patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_result_summary: Option<HiddenResultSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_overlay: Option<PolicyOverlaySummary>,
    pub includes_machine_local_refs: bool,
    pub includes_managed_provider_refs: bool,
    pub requires_rebinding_on_import: bool,
    pub offered_actions: Vec<SwitcherRowAction>,
    pub emitted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_index_note: Option<String>,
}

/// Parent beta switcher record containing every projected row plus the
/// active-workset highlight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetSwitcherBetaRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub switcher_id: String,
    pub workspace_ref: String,
    pub active_workset_ref: String,
    pub rows: Vec<WorksetSwitcherBetaRow>,
    pub emitted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Errors detected while validating beta switcher records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetSwitcherBetaError {
    SchemaVersionMismatch(u32),
    EmptyId(&'static str),
    EmptyRows,
    ActiveRowMismatch,
    ActiveRowCount(usize),
    DuplicateRow(String),
    EmptyRootTaxonomy(String),
    PolicyLabelMissingOverlay(String),
    OverlayWithoutPolicyLabel(String),
    PolicyExposesAdminHiddenList(String),
    ManagedRowOffersExport(String),
    LocalRowOffersExport(String),
    PolicyRowMissingUnderlying(String),
    RowPortabilityLabelMismatch {
        row_id: String,
        expected: WorksetPortabilityLabel,
        actual: WorksetPortabilityLabel,
    },
    RowMissingPreviewWhenInactive(String),
    UnknownConsumerSurface(String),
}

impl std::fmt::Display for WorksetSwitcherBetaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(v) => write!(
                f,
                "unsupported workset_switcher_beta schema_version {v}; this layer accepts 1"
            ),
            Self::EmptyId(field) => write!(f, "{field} must not be empty"),
            Self::EmptyRows => write!(f, "switcher record must contain at least one row"),
            Self::ActiveRowMismatch => write!(
                f,
                "exactly one row must reference active_workset_ref and have is_active = true"
            ),
            Self::ActiveRowCount(count) => {
                write!(f, "exactly one row must be active; got {count}")
            }
            Self::DuplicateRow(id) => write!(f, "duplicate row id: {id}"),
            Self::EmptyRootTaxonomy(id) => {
                write!(f, "row {id} must carry root_taxonomy for every declared root")
            }
            Self::PolicyLabelMissingOverlay(id) => write!(
                f,
                "row {id} has portability_label = policy_limited but no policy_overlay"
            ),
            Self::OverlayWithoutPolicyLabel(id) => write!(
                f,
                "row {id} has a policy_overlay but portability_label is not policy_limited or managed_provider_locked"
            ),
            Self::PolicyExposesAdminHiddenList(id) => write!(
                f,
                "row {id} policy_overlay narrowing_cause forbids exposing the hidden list"
            ),
            Self::ManagedRowOffersExport(id) => write!(
                f,
                "row {id} is managed_provider_locked but offers export_workset_artifact"
            ),
            Self::LocalRowOffersExport(id) => write!(
                f,
                "row {id} is local_only / ephemeral_session but offers export_workset_artifact"
            ),
            Self::PolicyRowMissingUnderlying(id) => write!(
                f,
                "row {id} policy_overlay must reference the underlying workset"
            ),
            Self::RowPortabilityLabelMismatch {
                row_id,
                expected,
                actual,
            } => write!(
                f,
                "row {row_id} portability_label {actual:?} does not match derived {expected:?}"
            ),
            Self::RowMissingPreviewWhenInactive(id) => write!(
                f,
                "row {id} is inactive and must offer preview_activation_diff"
            ),
            Self::UnknownConsumerSurface(surface) => {
                write!(f, "unknown consumer surface {surface}")
            }
        }
    }
}

impl std::error::Error for WorksetSwitcherBetaError {}

impl WorksetSwitcherBetaRow {
    /// Returns the record-kind tag for serialized rows.
    pub const RECORD_KIND: &'static str = WORKSET_SWITCHER_BETA_ROW_RECORD_KIND;
    /// Returns the schema version for serialized rows.
    pub const SCHEMA_VERSION: u32 = WORKSET_SWITCHER_BETA_SCHEMA_VERSION;

    /// Validates the beta row against the closed invariants.
    pub fn validate(&self) -> Result<(), WorksetSwitcherBetaError> {
        if self.schema_version != WORKSET_SWITCHER_BETA_SCHEMA_VERSION {
            return Err(WorksetSwitcherBetaError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        if self.row_id.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("row_id"));
        }
        if self.workset_ref.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("workset_ref"));
        }
        if self.stable_scope_id.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("stable_scope_id"));
        }
        if self.workset_name.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("workset_name"));
        }
        if self.root_taxonomy.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyRootTaxonomy(
                self.row_id.clone(),
            ));
        }
        match (self.portability_label, self.policy_overlay.as_ref()) {
            (WorksetPortabilityLabel::PolicyLimited, None) => {
                return Err(WorksetSwitcherBetaError::PolicyLabelMissingOverlay(
                    self.row_id.clone(),
                ));
            }
            (label, Some(overlay))
                if label != WorksetPortabilityLabel::PolicyLimited
                    && label != WorksetPortabilityLabel::ManagedProviderLocked =>
            {
                let _ = overlay;
                return Err(WorksetSwitcherBetaError::OverlayWithoutPolicyLabel(
                    self.row_id.clone(),
                ));
            }
            (_, Some(overlay)) => {
                if overlay.underlying_workset_ref.is_empty() {
                    return Err(WorksetSwitcherBetaError::PolicyRowMissingUnderlying(
                        self.row_id.clone(),
                    ));
                }
                if overlay.narrowing_cause.forbids_hidden_list()
                    && overlay.hidden_member_list_visible
                {
                    return Err(WorksetSwitcherBetaError::PolicyExposesAdminHiddenList(
                        self.row_id.clone(),
                    ));
                }
            }
            _ => {}
        }
        if self.portability_label.forbids_export()
            && self
                .offered_actions
                .contains(&SwitcherRowAction::ExportWorksetArtifact)
        {
            return Err(WorksetSwitcherBetaError::ManagedRowOffersExport(
                self.row_id.clone(),
            ));
        }
        if self.portability_label == WorksetPortabilityLabel::LocalOnly
            && self.source_class == SourceClass::EphemeralSession
            && self
                .offered_actions
                .contains(&SwitcherRowAction::ExportWorksetArtifact)
        {
            return Err(WorksetSwitcherBetaError::LocalRowOffersExport(
                self.row_id.clone(),
            ));
        }
        if !self.is_active
            && !self
                .offered_actions
                .contains(&SwitcherRowAction::PreviewActivationDiff)
        {
            return Err(WorksetSwitcherBetaError::RowMissingPreviewWhenInactive(
                self.row_id.clone(),
            ));
        }
        Ok(())
    }
}

impl WorksetSwitcherBetaRecord {
    /// Record-kind tag for serialized switcher records.
    pub const RECORD_KIND: &'static str = WORKSET_SWITCHER_BETA_RECORD_KIND;
    /// Schema version for serialized switcher records.
    pub const SCHEMA_VERSION: u32 = WORKSET_SWITCHER_BETA_SCHEMA_VERSION;

    /// Validates the parent switcher record and every row inside it.
    pub fn validate(&self) -> Result<(), WorksetSwitcherBetaError> {
        if self.schema_version != WORKSET_SWITCHER_BETA_SCHEMA_VERSION {
            return Err(WorksetSwitcherBetaError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        if self.switcher_id.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("switcher_id"));
        }
        if self.workspace_ref.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("workspace_ref"));
        }
        if self.active_workset_ref.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyId("active_workset_ref"));
        }
        if self.rows.is_empty() {
            return Err(WorksetSwitcherBetaError::EmptyRows);
        }
        let mut seen: Vec<&str> = Vec::with_capacity(self.rows.len());
        let mut active_count = 0usize;
        for row in &self.rows {
            row.validate()?;
            if seen.iter().any(|id| *id == row.row_id.as_str()) {
                return Err(WorksetSwitcherBetaError::DuplicateRow(row.row_id.clone()));
            }
            seen.push(row.row_id.as_str());
            if row.is_active {
                active_count += 1;
                if row.workset_ref != self.active_workset_ref {
                    return Err(WorksetSwitcherBetaError::ActiveRowMismatch);
                }
            }
        }
        if active_count != 1 {
            return Err(WorksetSwitcherBetaError::ActiveRowCount(active_count));
        }
        Ok(())
    }

    /// Returns the row marked active.
    pub fn active_row(&self) -> Option<&WorksetSwitcherBetaRow> {
        self.rows.iter().find(|row| row.is_active)
    }

    /// Returns the row with the given workset_ref, if any.
    pub fn row_for(&self, workset_ref: &str) -> Option<&WorksetSwitcherBetaRow> {
        self.rows.iter().find(|row| row.workset_ref == workset_ref)
    }
}

/// Projects a [`WorksetSwitcherBetaRow`] from one durable artifact.
pub fn project_switcher_row(
    artifact: &WorksetArtifactRecord,
    row_id: impl Into<String>,
    is_active: bool,
    emitted_at: impl Into<String>,
) -> WorksetSwitcherBetaRow {
    let portability_label = derive_portability_label(artifact);
    let folder_count = artifact
        .member_refs
        .iter()
        .filter(|m| {
            !matches!(
                m.ref_kind,
                crate::worksets::MemberRefKind::Root | crate::worksets::MemberRefKind::GraphSeed
            )
        })
        .count() as u32;
    let include_patterns: Vec<String> = artifact
        .patterns
        .iter()
        .filter(|p| p.pattern_kind == crate::worksets::PatternKind::Include)
        .map(pattern_token)
        .collect();
    let exclude_patterns: Vec<String> = artifact
        .patterns
        .iter()
        .filter(|p| p.pattern_kind == crate::worksets::PatternKind::Exclude)
        .map(pattern_token)
        .collect();
    let policy_overlay = artifact
        .policy_limitation
        .as_ref()
        .map(PolicyOverlaySummary::from);
    let hidden_result_summary = derive_hidden_summary(artifact);
    let mut offered_actions = derive_offered_actions(artifact, is_active, portability_label);
    if !is_active && !offered_actions.contains(&SwitcherRowAction::PreviewActivationDiff) {
        offered_actions.push(SwitcherRowAction::PreviewActivationDiff);
    }
    WorksetSwitcherBetaRow {
        record_kind: WorksetSwitcherBetaRow::RECORD_KIND.to_string(),
        schema_version: WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
        row_id: row_id.into(),
        workset_ref: artifact.workset_id.clone(),
        stable_scope_id: artifact.stable_scope_id().to_string(),
        workset_name: artifact.workset_name.clone(),
        presentation_subtitle: artifact.presentation_subtitle.clone(),
        scope_class: artifact.scope_class,
        scope_mode: artifact.scope_mode,
        source_class: artifact.portability.source_class,
        portability_class: artifact.portability.portability_class,
        portability_label,
        readiness_state: artifact.readiness.readiness_state,
        repo_count: artifact.root_refs.len() as u32,
        folder_count,
        is_active,
        root_taxonomy: artifact.included_roots.clone(),
        include_patterns,
        exclude_patterns,
        hidden_result_summary,
        policy_overlay,
        includes_machine_local_refs: artifact.portability.includes_machine_local_refs,
        includes_managed_provider_refs: artifact.portability.includes_managed_provider_refs,
        requires_rebinding_on_import: artifact.portability.requires_rebinding_on_import,
        offered_actions,
        emitted_at: emitted_at.into(),
        partial_index_note: artifact.readiness.partial_index_note.clone(),
    }
}

fn pattern_token(pattern: &PatternEntry) -> String {
    match pattern.applies_to_root_ref.as_deref() {
        Some(root) => format!("{root}::{}", pattern.pattern),
        None => pattern.pattern.clone(),
    }
}

fn derive_hidden_summary(artifact: &WorksetArtifactRecord) -> Option<HiddenResultSummary> {
    use crate::worksets::HiddenResultCountClass;
    if !artifact.readiness.hidden_result_count_known
        && artifact.readiness.hidden_result_count.is_none()
        && artifact.policy_limitation.is_none()
    {
        return None;
    }
    let count_class = if artifact.policy_limitation.is_some() {
        HiddenResultCountClass::PolicyHidden
    } else if artifact.scope_class == ScopeClass::SparseSlice {
        HiddenResultCountClass::PartialIndex
    } else if matches!(
        artifact.readiness.readiness_state,
        ReadinessState::Warming | ReadinessState::Partial
    ) {
        HiddenResultCountClass::WarmingIndex
    } else if artifact.scope_class == ScopeClass::SelectedWorkset
        || artifact.scope_class == ScopeClass::CurrentRepo
    {
        HiddenResultCountClass::OutsideScopeRoots
    } else {
        HiddenResultCountClass::NoneKnown
    };
    Some(HiddenResultSummary {
        known: artifact.readiness.hidden_result_count_known,
        count: artifact.readiness.hidden_result_count,
        count_class,
    })
}

fn derive_offered_actions(
    artifact: &WorksetArtifactRecord,
    is_active: bool,
    portability_label: WorksetPortabilityLabel,
) -> Vec<SwitcherRowAction> {
    let mut actions: Vec<SwitcherRowAction> = Vec::new();
    if !is_active {
        actions.push(SwitcherRowAction::OpenWorkset);
        actions.push(SwitcherRowAction::PreviewActivationDiff);
    }
    actions.push(SwitcherRowAction::CopyWorksetId);
    actions.push(SwitcherRowAction::OpenInNewPane);
    actions.push(SwitcherRowAction::ReopenAcrossSurfaces);
    if !portability_label.forbids_export()
        && artifact.portability.source_class != SourceClass::EphemeralSession
        && portability_label != WorksetPortabilityLabel::LocalOnly
    {
        actions.push(SwitcherRowAction::ExportWorksetArtifact);
    }
    if portability_label == WorksetPortabilityLabel::PolicyLimited {
        actions.push(SwitcherRowAction::OpenUnderlyingWorkset);
    }
    actions
}

/// Projects a [`WorksetSwitcherBetaRecord`] from the active artifact and a
/// candidate list of artifacts.
pub fn project_switcher_record(
    switcher_id: impl Into<String>,
    workspace_ref: impl Into<String>,
    active_workset_ref: &str,
    artifacts: &[WorksetArtifactRecord],
    emitted_at: impl Into<String>,
) -> WorksetSwitcherBetaRecord {
    let emitted_at = emitted_at.into();
    let rows: Vec<WorksetSwitcherBetaRow> = artifacts
        .iter()
        .enumerate()
        .map(|(index, artifact)| {
            let row_id = format!("switcher_row:{}:{}", artifact.workset_id, index);
            let is_active = artifact.workset_id == active_workset_ref;
            project_switcher_row(artifact, row_id, is_active, emitted_at.clone())
        })
        .collect();
    WorksetSwitcherBetaRecord {
        record_kind: WorksetSwitcherBetaRecord::RECORD_KIND.to_string(),
        schema_version: WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
        switcher_id: switcher_id.into(),
        workspace_ref: workspace_ref.into(),
        active_workset_ref: active_workset_ref.to_string(),
        rows,
        emitted_at,
        notes: None,
    }
}

/// Typed activation preview produced when a candidate row is selected
/// before opening the workset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetActivationPreview {
    pub record_kind: String,
    pub schema_version: u32,
    pub preview_id: String,
    pub base_workset_ref: String,
    pub base_stable_scope_id: String,
    pub candidate_workset_ref: String,
    pub candidate_stable_scope_id: String,
    pub same_identity: bool,
    pub scope_drift: ScopeDriftClass,
    pub root_additions: Vec<IncludedRootRef>,
    pub root_removals: Vec<IncludedRootRef>,
    pub changes_portability: bool,
    pub changes_readiness: bool,
    pub base_portability_label: WorksetPortabilityLabel,
    pub candidate_portability_label: WorksetPortabilityLabel,
    pub base_readiness: ReadinessState,
    pub candidate_readiness: ReadinessState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<ScopeWidenDiffRecord>,
    pub emitted_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain_note: Option<String>,
}

/// Errors detected while validating a [`WorksetActivationPreview`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetActivationPreviewError {
    SchemaVersionMismatch(u32),
    EmptyId(&'static str),
    DiffIdentityMismatch,
    DiffWidenError(ScopeWidenDiffError),
    SameIdentityHasBehavioralDrift,
    DriftClassMismatch(ScopeDriftClass),
    AdditionsRequireWideningDrift,
    RemovalsRequireNarrowingDrift,
}

impl std::fmt::Display for WorksetActivationPreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(v) => write!(
                f,
                "unsupported workset_activation_preview schema_version {v}; this layer accepts 1"
            ),
            Self::EmptyId(field) => write!(f, "{field} must not be empty"),
            Self::DiffIdentityMismatch => {
                write!(
                    f,
                    "diff record must reference the preview base/candidate refs"
                )
            }
            Self::DiffWidenError(err) => write!(f, "diff record invalid: {err}"),
            Self::SameIdentityHasBehavioralDrift => {
                write!(f, "same_identity previews cannot carry behavioural drift")
            }
            Self::DriftClassMismatch(class) => {
                write!(
                    f,
                    "scope_drift {} does not match the recorded changes",
                    class.as_str()
                )
            }
            Self::AdditionsRequireWideningDrift => {
                write!(f, "root_additions require a widening or mixed scope_drift")
            }
            Self::RemovalsRequireNarrowingDrift => {
                write!(f, "root_removals require a narrowing or mixed scope_drift")
            }
        }
    }
}

impl std::error::Error for WorksetActivationPreviewError {}

impl WorksetActivationPreview {
    /// Record-kind tag for serialized previews.
    pub const RECORD_KIND: &'static str = WORKSET_ACTIVATION_PREVIEW_RECORD_KIND;
    /// Schema version for serialized previews.
    pub const SCHEMA_VERSION: u32 = WORKSET_SWITCHER_BETA_SCHEMA_VERSION;

    /// Validates the preview against the closed invariants.
    pub fn validate(&self) -> Result<(), WorksetActivationPreviewError> {
        if self.schema_version != WORKSET_SWITCHER_BETA_SCHEMA_VERSION {
            return Err(WorksetActivationPreviewError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        if self.preview_id.is_empty() {
            return Err(WorksetActivationPreviewError::EmptyId("preview_id"));
        }
        if self.base_workset_ref.is_empty() {
            return Err(WorksetActivationPreviewError::EmptyId("base_workset_ref"));
        }
        if self.candidate_workset_ref.is_empty() {
            return Err(WorksetActivationPreviewError::EmptyId(
                "candidate_workset_ref",
            ));
        }
        if let Some(diff) = self.diff.as_ref() {
            diff.validate()
                .map_err(WorksetActivationPreviewError::DiffWidenError)?;
            if diff.base_workset_ref != self.base_workset_ref
                || diff.candidate_workset_ref != self.candidate_workset_ref
            {
                return Err(WorksetActivationPreviewError::DiffIdentityMismatch);
            }
        }
        if self.same_identity {
            let behavioural = !self.root_additions.is_empty()
                || !self.root_removals.is_empty()
                || self.changes_portability
                || self.changes_readiness;
            if behavioural {
                return Err(WorksetActivationPreviewError::SameIdentityHasBehavioralDrift);
            }
            if !matches!(
                self.scope_drift,
                ScopeDriftClass::SameIdentity | ScopeDriftClass::PresentationOnly
            ) {
                return Err(WorksetActivationPreviewError::DriftClassMismatch(
                    self.scope_drift,
                ));
            }
        } else {
            let widens = !self.root_additions.is_empty();
            let narrows = !self.root_removals.is_empty();
            let posture_only =
                !widens && !narrows && (self.changes_portability || self.changes_readiness);
            match self.scope_drift {
                ScopeDriftClass::SameIdentity => {
                    return Err(WorksetActivationPreviewError::DriftClassMismatch(
                        self.scope_drift,
                    ));
                }
                ScopeDriftClass::Widens if !widens => {
                    return Err(WorksetActivationPreviewError::AdditionsRequireWideningDrift);
                }
                ScopeDriftClass::Narrows if !narrows => {
                    return Err(WorksetActivationPreviewError::RemovalsRequireNarrowingDrift);
                }
                ScopeDriftClass::Mixed if !(widens && narrows) => {
                    return Err(WorksetActivationPreviewError::DriftClassMismatch(
                        self.scope_drift,
                    ));
                }
                ScopeDriftClass::PortabilityOrReadinessOnly if !posture_only => {
                    return Err(WorksetActivationPreviewError::DriftClassMismatch(
                        self.scope_drift,
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl WorksetArtifactRecord {
    /// Projects a [`WorksetActivationPreview`] between the active artifact
    /// (`self`) and a candidate artifact.
    pub fn project_activation_preview(
        &self,
        candidate: &WorksetArtifactRecord,
        preview_id: impl Into<String>,
        diff_id: impl Into<String>,
        emitted_at: impl Into<String>,
    ) -> WorksetActivationPreview {
        let emitted_at = emitted_at.into();
        let same_identity = self.workset_id == candidate.workset_id;
        let root_additions: Vec<IncludedRootRef> = candidate
            .included_roots
            .iter()
            .filter(|root| {
                !self
                    .included_roots
                    .iter()
                    .any(|existing| existing.root_ref == root.root_ref)
            })
            .cloned()
            .collect();
        let root_removals: Vec<IncludedRootRef> = self
            .included_roots
            .iter()
            .filter(|root| {
                !candidate
                    .included_roots
                    .iter()
                    .any(|existing| existing.root_ref == root.root_ref)
            })
            .cloned()
            .collect();
        let changes_portability = self.portability.portability_class
            != candidate.portability.portability_class
            || self.portability.source_class != candidate.portability.source_class;
        let changes_readiness =
            self.readiness.readiness_state != candidate.readiness.readiness_state;
        let presentation_only = same_identity
            && self.workset_name != candidate.workset_name
            && root_additions.is_empty()
            && root_removals.is_empty()
            && !changes_portability
            && !changes_readiness;
        let scope_drift = if same_identity && !presentation_only {
            ScopeDriftClass::SameIdentity
        } else if presentation_only {
            ScopeDriftClass::PresentationOnly
        } else if !root_additions.is_empty() && !root_removals.is_empty() {
            ScopeDriftClass::Mixed
        } else if !root_additions.is_empty() {
            ScopeDriftClass::Widens
        } else if !root_removals.is_empty() {
            ScopeDriftClass::Narrows
        } else if changes_portability || changes_readiness {
            ScopeDriftClass::PortabilityOrReadinessOnly
        } else {
            ScopeDriftClass::SameIdentity
        };
        let diff = if same_identity {
            None
        } else {
            Some(derive_widen_diff(
                self,
                candidate,
                diff_id,
                emitted_at.clone(),
            ))
        };
        let explain_note = if same_identity {
            Some(format!(
                "Activating {} keeps the active scope identity; no diff to apply.",
                candidate.workset_name
            ))
        } else if root_additions.is_empty() && root_removals.is_empty() {
            Some(format!(
                "Activating {} only changes posture (portability or readiness); root taxonomy is unchanged.",
                candidate.workset_name
            ))
        } else {
            Some(format!(
                "Activating {} alters scope: {} root(s) added, {} root(s) removed.",
                candidate.workset_name,
                root_additions.len(),
                root_removals.len()
            ))
        };
        WorksetActivationPreview {
            record_kind: WorksetActivationPreview::RECORD_KIND.to_string(),
            schema_version: WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
            preview_id: preview_id.into(),
            base_workset_ref: self.workset_id.clone(),
            base_stable_scope_id: self.stable_scope_id().to_string(),
            candidate_workset_ref: candidate.workset_id.clone(),
            candidate_stable_scope_id: candidate.stable_scope_id().to_string(),
            same_identity,
            scope_drift,
            root_additions,
            root_removals,
            changes_portability,
            changes_readiness,
            base_portability_label: derive_portability_label(self),
            candidate_portability_label: derive_portability_label(candidate),
            base_readiness: self.readiness.readiness_state,
            candidate_readiness: candidate.readiness.readiness_state,
            diff,
            emitted_at,
            explain_note,
        }
    }
}

fn derive_widen_diff(
    base: &WorksetArtifactRecord,
    candidate: &WorksetArtifactRecord,
    diff_id: impl Into<String>,
    emitted_at: String,
) -> ScopeWidenDiffRecord {
    let mut entries: Vec<ScopeDiffEntry> = Vec::new();
    for added in candidate.included_roots.iter().filter(|root| {
        !base
            .included_roots
            .iter()
            .any(|existing| existing.root_ref == root.root_ref)
    }) {
        entries.push(ScopeDiffEntry {
            diff_class: ScopeDiffClass::MemberAdded,
            affected_member_ref: Some(crate::worksets::MemberRef {
                ref_kind: crate::worksets::MemberRefKind::Root,
                ref_id: added.root_ref.clone(),
                partial_truth: added.partial_truth,
                presentation_label: added.presentation_label.clone(),
            }),
            affected_pattern: None,
            note: format!(
                "Adds root {} ({})",
                added
                    .presentation_label
                    .as_deref()
                    .unwrap_or(added.root_ref.as_str()),
                added.root_kind.as_str()
            ),
        });
    }
    for removed in base.included_roots.iter().filter(|root| {
        !candidate
            .included_roots
            .iter()
            .any(|existing| existing.root_ref == root.root_ref)
    }) {
        entries.push(ScopeDiffEntry {
            diff_class: ScopeDiffClass::MemberRemoved,
            affected_member_ref: Some(crate::worksets::MemberRef {
                ref_kind: crate::worksets::MemberRefKind::Root,
                ref_id: removed.root_ref.clone(),
                partial_truth: removed.partial_truth,
                presentation_label: removed.presentation_label.clone(),
            }),
            affected_pattern: None,
            note: format!(
                "Removes root {} ({})",
                removed
                    .presentation_label
                    .as_deref()
                    .unwrap_or(removed.root_ref.as_str()),
                removed.root_kind.as_str()
            ),
        });
    }
    if base.portability.portability_class != candidate.portability.portability_class
        || base.portability.source_class != candidate.portability.source_class
    {
        entries.push(ScopeDiffEntry {
            diff_class: ScopeDiffClass::PortabilityChanged,
            affected_member_ref: None,
            affected_pattern: None,
            note: format!(
                "Portability changes from {:?}/{:?} to {:?}/{:?}",
                base.portability.source_class,
                base.portability.portability_class,
                candidate.portability.source_class,
                candidate.portability.portability_class
            ),
        });
    }
    if base.readiness.readiness_state != candidate.readiness.readiness_state {
        entries.push(ScopeDiffEntry {
            diff_class: ScopeDiffClass::ReadinessChanged,
            affected_member_ref: None,
            affected_pattern: None,
            note: format!(
                "Readiness changes from {} to {}",
                base.readiness.readiness_state.as_str(),
                candidate.readiness.readiness_state.as_str()
            ),
        });
    }
    if entries.is_empty() {
        entries.push(ScopeDiffEntry {
            diff_class: ScopeDiffClass::PresentationOnly,
            affected_member_ref: None,
            affected_pattern: None,
            note: "Only presentation (rename / subtitle) changed.".to_string(),
        });
    }
    let widens = entries.iter().any(|entry| entry.diff_class.widens_scope());
    let narrows = entries.iter().any(|entry| entry.diff_class.narrows_scope());
    let presentation_only = entries
        .iter()
        .all(|entry| entry.diff_class == ScopeDiffClass::PresentationOnly);
    let changes_portability = entries
        .iter()
        .any(|entry| entry.diff_class == ScopeDiffClass::PortabilityChanged);
    let changes_readiness = entries
        .iter()
        .any(|entry| entry.diff_class == ScopeDiffClass::ReadinessChanged);
    ScopeWidenDiffRecord {
        record_kind: ScopeWidenDiffRecordKind::ScopeWidenDiffRecord,
        workset_artifact_schema_version: 1,
        diff_id: diff_id.into(),
        base_workset_ref: base.workset_id.clone(),
        candidate_workset_ref: candidate.workset_id.clone(),
        entries,
        widens_scope: widens,
        narrows_scope: narrows,
        changes_portability,
        changes_readiness,
        presentation_only,
        expected_index_cost_class: None,
        remote_fetch_required: false,
        emitted_at,
        notes: None,
    }
}

/// Single per-consumer downgrade row inside a [`WorksetReopenParityPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenParityDowngrade {
    pub consumer_class: WorksetScopeConsumerClass,
    pub reason: ScopeDegradedReason,
    pub note: String,
}

/// Reopen-parity packet bundling consumer bindings for one workset across
/// local UI, remote UI, and headless surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetReopenParityPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub packet_id: String,
    pub workset_ref: String,
    pub stable_scope_id: String,
    pub workset_name: String,
    pub portability_label: WorksetPortabilityLabel,
    pub bindings: Vec<WorksetScopeConsumerBinding>,
    pub identity_preserved_across_consumers: bool,
    pub exact_consumer_classes: Vec<WorksetScopeConsumerClass>,
    pub degraded: Vec<ReopenParityDowngrade>,
    pub emitted_at: String,
}

/// Errors detected while validating a [`WorksetReopenParityPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorksetReopenParityError {
    SchemaVersionMismatch(u32),
    EmptyId(&'static str),
    EmptyBindings,
    DuplicateConsumer(WorksetScopeConsumerClass),
    IdentityDriftAcrossConsumers,
    DegradedBindingMissingReason(WorksetScopeConsumerClass),
    ExactBindingCarriesReason(WorksetScopeConsumerClass),
    DegradedReasonClassMismatch,
    PortabilityLabelInvalidForExport(WorksetPortabilityLabel),
    MissingDefaultConsumer(WorksetScopeConsumerClass),
}

impl std::fmt::Display for WorksetReopenParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(v) => write!(
                f,
                "unsupported workset_reopen_parity schema_version {v}; this layer accepts 1"
            ),
            Self::EmptyId(field) => write!(f, "{field} must not be empty"),
            Self::EmptyBindings => write!(
                f,
                "reopen-parity packet must include at least one consumer binding"
            ),
            Self::DuplicateConsumer(class) => {
                write!(
                    f,
                    "duplicate consumer class in bindings: {}",
                    class.as_str()
                )
            }
            Self::IdentityDriftAcrossConsumers => write!(
                f,
                "every binding must reference the same workset_ref and stable_scope_id"
            ),
            Self::DegradedBindingMissingReason(class) => write!(
                f,
                "degraded binding for {} must carry a typed degraded_reason",
                class.as_str()
            ),
            Self::ExactBindingCarriesReason(class) => write!(
                f,
                "exact binding for {} must not carry a degraded_reason",
                class.as_str()
            ),
            Self::DegradedReasonClassMismatch => write!(
                f,
                "degraded list rows must match the binding's degraded_reason"
            ),
            Self::PortabilityLabelInvalidForExport(label) => write!(
                f,
                "portability_label {} forbids a support-export consumer binding",
                label.as_str()
            ),
            Self::MissingDefaultConsumer(class) => write!(
                f,
                "reopen-parity packet must include the default consumer class {}",
                class.as_str()
            ),
        }
    }
}

impl std::error::Error for WorksetReopenParityError {}

impl WorksetReopenParityPacket {
    /// Record-kind tag for serialized parity packets.
    pub const RECORD_KIND: &'static str = WORKSET_REOPEN_PARITY_PACKET_RECORD_KIND;
    /// Schema version for serialized parity packets.
    pub const SCHEMA_VERSION: u32 = WORKSET_SWITCHER_BETA_SCHEMA_VERSION;

    /// Default consumer classes a beta parity packet MUST cover.
    pub const DEFAULT_CONSUMER_CLASSES: [WorksetScopeConsumerClass; 3] = [
        WorksetScopeConsumerClass::LocalUi,
        WorksetScopeConsumerClass::RemoteUi,
        WorksetScopeConsumerClass::Headless,
    ];

    /// Validates the parity packet against the closed invariants.
    pub fn validate(&self) -> Result<(), WorksetReopenParityError> {
        if self.schema_version != WORKSET_SWITCHER_BETA_SCHEMA_VERSION {
            return Err(WorksetReopenParityError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        if self.packet_id.is_empty() {
            return Err(WorksetReopenParityError::EmptyId("packet_id"));
        }
        if self.workset_ref.is_empty() {
            return Err(WorksetReopenParityError::EmptyId("workset_ref"));
        }
        if self.stable_scope_id.is_empty() {
            return Err(WorksetReopenParityError::EmptyId("stable_scope_id"));
        }
        if self.bindings.is_empty() {
            return Err(WorksetReopenParityError::EmptyBindings);
        }
        if self.portability_label == WorksetPortabilityLabel::ManagedProviderLocked
            && self
                .bindings
                .iter()
                .any(|b| b.consumer_class == WorksetScopeConsumerClass::SupportExport)
        {
            return Err(WorksetReopenParityError::PortabilityLabelInvalidForExport(
                self.portability_label,
            ));
        }
        let mut seen: Vec<WorksetScopeConsumerClass> = Vec::with_capacity(self.bindings.len());
        for binding in &self.bindings {
            if seen.iter().any(|c| *c == binding.consumer_class) {
                return Err(WorksetReopenParityError::DuplicateConsumer(
                    binding.consumer_class,
                ));
            }
            seen.push(binding.consumer_class);
            if binding.workset_ref != self.workset_ref
                || binding.stable_scope_id != self.stable_scope_id
            {
                return Err(WorksetReopenParityError::IdentityDriftAcrossConsumers);
            }
            match (binding.reopen_state, binding.degraded_reason) {
                (ScopeReopenState::Exact, Some(_)) => {
                    return Err(WorksetReopenParityError::ExactBindingCarriesReason(
                        binding.consumer_class,
                    ));
                }
                (ScopeReopenState::Degraded, None) => {
                    return Err(WorksetReopenParityError::DegradedBindingMissingReason(
                        binding.consumer_class,
                    ));
                }
                (ScopeReopenState::Degraded, Some(reason)) => {
                    let listed = self
                        .degraded
                        .iter()
                        .find(|d| d.consumer_class == binding.consumer_class);
                    if let Some(listed) = listed {
                        if listed.reason != reason {
                            return Err(WorksetReopenParityError::DegradedReasonClassMismatch);
                        }
                    } else {
                        return Err(WorksetReopenParityError::DegradedReasonClassMismatch);
                    }
                }
                _ => {}
            }
        }
        for default_class in Self::DEFAULT_CONSUMER_CLASSES {
            if !seen.contains(&default_class) {
                return Err(WorksetReopenParityError::MissingDefaultConsumer(
                    default_class,
                ));
            }
        }
        for listed in &self.degraded {
            let binding = self
                .bindings
                .iter()
                .find(|b| b.consumer_class == listed.consumer_class);
            if binding.is_none() {
                return Err(WorksetReopenParityError::DegradedReasonClassMismatch);
            }
        }
        let identity_preserved = self.bindings.iter().all(|b| {
            b.workset_ref == self.workset_ref && b.stable_scope_id == self.stable_scope_id
        });
        if identity_preserved != self.identity_preserved_across_consumers {
            return Err(WorksetReopenParityError::IdentityDriftAcrossConsumers);
        }
        let exact: Vec<WorksetScopeConsumerClass> = self
            .bindings
            .iter()
            .filter(|b| b.reopen_state == ScopeReopenState::Exact)
            .map(|b| b.consumer_class)
            .collect();
        if exact != self.exact_consumer_classes {
            return Err(WorksetReopenParityError::DegradedReasonClassMismatch);
        }
        Ok(())
    }
}

impl WorksetArtifactRecord {
    /// Projects a [`WorksetReopenParityPacket`] bundling consumer bindings
    /// for every default consumer class. Callers supply the per-class reopen
    /// posture so the packet quotes the real local / remote / headless state
    /// at packet emit time.
    pub fn project_reopen_parity_packet(
        &self,
        packet_id: impl Into<String>,
        local_posture: ScopeReopenPosture,
        remote_posture: ScopeReopenPosture,
        headless_posture: ScopeReopenPosture,
        emitted_at: impl Into<String>,
    ) -> WorksetReopenParityPacket {
        let emitted_at = emitted_at.into();
        let bindings = vec![
            self.project_consumer_binding(
                WorksetScopeConsumerClass::LocalUi,
                local_posture,
                emitted_at.clone(),
            ),
            self.project_consumer_binding(
                WorksetScopeConsumerClass::RemoteUi,
                remote_posture,
                emitted_at.clone(),
            ),
            self.project_consumer_binding(
                WorksetScopeConsumerClass::Headless,
                headless_posture,
                emitted_at.clone(),
            ),
        ];
        let mut degraded: Vec<ReopenParityDowngrade> = Vec::new();
        for binding in &bindings {
            if let Some(reason) = binding.degraded_reason {
                degraded.push(ReopenParityDowngrade {
                    consumer_class: binding.consumer_class,
                    reason,
                    note: degraded_note(binding.consumer_class, reason),
                });
            }
        }
        let exact_consumer_classes: Vec<WorksetScopeConsumerClass> = bindings
            .iter()
            .filter(|b| b.reopen_state == ScopeReopenState::Exact)
            .map(|b| b.consumer_class)
            .collect();
        let identity_preserved = bindings.iter().all(|b| {
            b.workset_ref == self.workset_id && b.stable_scope_id == self.stable_scope_id()
        });
        WorksetReopenParityPacket {
            record_kind: WorksetReopenParityPacket::RECORD_KIND.to_string(),
            schema_version: WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            workset_ref: self.workset_id.clone(),
            stable_scope_id: self.stable_scope_id().to_string(),
            workset_name: self.workset_name.clone(),
            portability_label: derive_portability_label(self),
            bindings,
            identity_preserved_across_consumers: identity_preserved,
            exact_consumer_classes,
            degraded,
            emitted_at,
        }
    }
}

fn degraded_note(consumer: WorksetScopeConsumerClass, reason: ScopeDegradedReason) -> String {
    format!(
        "{consumer} reopen degraded ({reason}); identity preserved.",
        consumer = consumer.as_str(),
        reason = reason.as_str()
    )
}

/// Support-export packet bundling the switcher record, every activation
/// preview, and the reopen-parity packet for one workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorksetSwitcherBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub switcher: WorksetSwitcherBetaRecord,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_previews: Vec<WorksetActivationPreview>,
    pub reopen_parity_packets: Vec<WorksetReopenParityPacket>,
    pub emitted_at: String,
}

impl WorksetSwitcherBetaSupportExport {
    /// Record-kind tag for serialized support-export packets.
    pub const RECORD_KIND: &'static str = WORKSET_SWITCHER_BETA_SUPPORT_EXPORT_RECORD_KIND;
    /// Schema version for serialized support-export packets.
    pub const SCHEMA_VERSION: u32 = WORKSET_SWITCHER_BETA_SCHEMA_VERSION;

    /// Validates the full support-export bundle.
    pub fn validate(&self) -> Result<(), WorksetSwitcherBetaError> {
        if self.schema_version != WORKSET_SWITCHER_BETA_SCHEMA_VERSION {
            return Err(WorksetSwitcherBetaError::SchemaVersionMismatch(
                self.schema_version,
            ));
        }
        self.switcher.validate()?;
        for preview in &self.activation_previews {
            preview
                .validate()
                .map_err(|err| WorksetSwitcherBetaError::EmptyId(map_preview_field(&err)))?;
            if !self
                .switcher
                .rows
                .iter()
                .any(|row| row.workset_ref == preview.candidate_workset_ref)
            {
                return Err(WorksetSwitcherBetaError::UnknownConsumerSurface(
                    preview.candidate_workset_ref.clone(),
                ));
            }
        }
        for parity in &self.reopen_parity_packets {
            parity
                .validate()
                .map_err(|_| WorksetSwitcherBetaError::EmptyId("reopen_parity"))?;
            if !self
                .switcher
                .rows
                .iter()
                .any(|row| row.workset_ref == parity.workset_ref)
            {
                return Err(WorksetSwitcherBetaError::UnknownConsumerSurface(
                    parity.workset_ref.clone(),
                ));
            }
        }
        Ok(())
    }
}

fn map_preview_field(err: &WorksetActivationPreviewError) -> &'static str {
    match err {
        WorksetActivationPreviewError::EmptyId(field) => field,
        _ => "activation_preview",
    }
}

/// Returns the chrome badge string for a root taxonomy entry.
pub fn root_taxonomy_badge(root: &IncludedRootRef) -> &'static str {
    WorkspaceRootKind::root_badge(root.root_kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worksets::{
        MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause, PartialTruthLabel,
        PatternEntry, PatternKind, PolicyLimitation, PortabilityMetadata, ReadinessMetadata,
        SourceClass, WorksetArtifactRecordKind,
    };

    fn portable_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:beta:portable".to_string(),
            scope_id: Some("scope:beta:portable".to_string()),
            workset_name: "Portable named workset".to_string(),
            presentation_subtitle: Some("Two repos".to_string()),
            scope_class: ScopeClass::SelectedWorkset,
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
                    presentation_label: None,
                },
                MemberRef {
                    ref_kind: MemberRefKind::Root,
                    ref_id: "fs-r-1".to_string(),
                    partial_truth: PartialTruthLabel::Loaded,
                    presentation_label: None,
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
            workset_name: "Frontend slice".to_string(),
            presentation_subtitle: Some("One repo".to_string()),
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
            patterns: vec![PatternEntry {
                pattern_kind: PatternKind::Include,
                pattern: "apps/web/**".to_string(),
                applies_to_root_ref: None,
            }],
            membership_policy: MembershipPolicy::GlobPattern,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::ManifestKnown,
                presentation_label: None,
            }],
            policy_limitation: None,
            portability: PortabilityMetadata {
                source_class: SourceClass::LocalOnly,
                portability_class: PortabilityClass::MachineLocalOnly,
                includes_machine_local_refs: true,
                includes_managed_provider_refs: false,
                requires_rebinding_on_import: true,
                profile_sync_group_ref: None,
            },
            readiness: ReadinessMetadata {
                readiness_state: ReadinessState::Partial,
                hidden_result_count_known: true,
                hidden_result_count: Some(99),
                partial_index_note: Some("Backend folders excluded.".to_string()),
            },
            parent_workset_ref: None,
            manifest_source_ref: None,
            created_at: "mono:0".to_string(),
            updated_at: "mono:0".to_string(),
            notes: None,
        }
    }

    fn policy_limited_artifact() -> WorksetArtifactRecord {
        WorksetArtifactRecord {
            record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
            workset_artifact_schema_version: 1,
            workset_id: "wks:beta:policy".to_string(),
            scope_id: Some("scope:beta:policy".to_string()),
            workset_name: "Policy-limited view".to_string(),
            presentation_subtitle: None,
            scope_class: ScopeClass::PolicyLimitedView,
            scope_mode: ScopeMode::Sparse,
            workspace_ref: Some("wksp:test".to_string()),
            root_refs: vec!["fs-r-0".to_string()],
            included_roots: vec![IncludedRootRef {
                root_ref: "fs-r-0".to_string(),
                root_kind: WorkspaceRootKind::ManagedCloudRoot,
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some("repo-managed".to_string()),
            }],
            patterns: vec![],
            membership_policy: MembershipPolicy::ExplicitRootList,
            member_refs: vec![MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: "fs-r-0".to_string(),
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: None,
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
            updated_at: "mono:0".to_string(),
            notes: None,
        }
    }

    #[test]
    fn derive_portability_label_classifies_each_class() {
        assert_eq!(
            derive_portability_label(&portable_artifact()),
            WorksetPortabilityLabel::PortableWithRebinding,
        );
        assert_eq!(
            derive_portability_label(&sparse_artifact()),
            WorksetPortabilityLabel::LocalOnly,
        );
        assert_eq!(
            derive_portability_label(&policy_limited_artifact()),
            WorksetPortabilityLabel::ManagedProviderLocked,
        );
    }

    #[test]
    fn projected_row_validates_and_names_each_root() {
        let artifact = portable_artifact();
        let row = project_switcher_row(&artifact, "row:0", true, "mono:1");
        row.validate().expect("row must validate");
        assert_eq!(row.root_taxonomy.len(), 2);
        assert_eq!(row.repo_count, 2);
        assert_eq!(
            row.portability_label,
            WorksetPortabilityLabel::PortableWithRebinding
        );
        assert!(row.is_active);
        // Active row should NOT advertise preview action since it's already active.
        assert!(!row
            .offered_actions
            .contains(&SwitcherRowAction::PreviewActivationDiff));
        assert!(row
            .offered_actions
            .contains(&SwitcherRowAction::ExportWorksetArtifact));
    }

    #[test]
    fn local_only_row_does_not_offer_export() {
        let artifact = sparse_artifact();
        let row = project_switcher_row(&artifact, "row:1", false, "mono:1");
        row.validate().expect("row must validate");
        assert_eq!(row.portability_label, WorksetPortabilityLabel::LocalOnly);
        assert!(!row
            .offered_actions
            .contains(&SwitcherRowAction::ExportWorksetArtifact));
        assert!(row
            .offered_actions
            .contains(&SwitcherRowAction::PreviewActivationDiff));
    }

    #[test]
    fn policy_limited_row_carries_overlay_and_does_not_export() {
        let artifact = policy_limited_artifact();
        let row = project_switcher_row(&artifact, "row:2", false, "mono:1");
        row.validate().expect("row must validate");
        assert_eq!(
            row.portability_label,
            WorksetPortabilityLabel::ManagedProviderLocked,
        );
        let overlay = row.policy_overlay.expect("overlay required");
        assert_eq!(overlay.narrowing_cause, NarrowingCause::AdminPolicy);
        assert!(!overlay.hidden_member_list_visible);
        assert!(!row
            .offered_actions
            .contains(&SwitcherRowAction::ExportWorksetArtifact));
    }

    #[test]
    fn switcher_record_validates_active_row() {
        let artifacts = vec![portable_artifact(), sparse_artifact()];
        let record = project_switcher_record(
            "switcher:test",
            "wksp:test",
            &artifacts[0].workset_id,
            &artifacts,
            "mono:test",
        );
        record.validate().expect("switcher must validate");
        assert_eq!(record.rows.len(), 2);
        assert_eq!(
            record.active_row().unwrap().workset_ref,
            artifacts[0].workset_id
        );
    }

    #[test]
    fn activation_preview_records_root_additions_and_removals() {
        let base = sparse_artifact();
        let candidate = portable_artifact();
        let preview =
            base.project_activation_preview(&candidate, "preview:0", "diff:0", "mono:preview");
        preview.validate().expect("preview must validate");
        assert!(!preview.same_identity);
        // sparse covers fs-r-0; portable covers fs-r-0 + fs-r-1 → purely widening.
        assert_eq!(preview.scope_drift, ScopeDriftClass::Widens);
        assert_eq!(preview.root_additions.len(), 1);
        assert_eq!(preview.root_removals.len(), 0);
        let diff = preview.diff.as_ref().expect("diff present");
        assert!(diff.widens_scope);
        assert!(!diff.narrows_scope);
    }

    #[test]
    fn activation_preview_classifies_mixed_drift_when_both_directions() {
        let mut base = portable_artifact();
        base.workset_id = "wks:beta:base".to_string();
        // candidate keeps fs-r-0 but swaps fs-r-1 → fs-r-2; produces add + remove.
        let mut candidate = portable_artifact();
        candidate.workset_id = "wks:beta:candidate".to_string();
        candidate.root_refs = vec!["fs-r-0".to_string(), "fs-r-2".to_string()];
        candidate.included_roots[1].root_ref = "fs-r-2".to_string();
        candidate.included_roots[1].presentation_label = Some("repo-c".to_string());
        candidate.member_refs[1].ref_id = "fs-r-2".to_string();
        let preview = base.project_activation_preview(
            &candidate,
            "preview:mixed",
            "diff:mixed",
            "mono:preview",
        );
        preview.validate().expect("preview must validate");
        assert_eq!(preview.scope_drift, ScopeDriftClass::Mixed);
        assert_eq!(preview.root_additions.len(), 1);
        assert_eq!(preview.root_removals.len(), 1);
    }

    #[test]
    fn activation_preview_for_same_identity_marks_no_drift() {
        let artifact = portable_artifact();
        let preview =
            artifact.project_activation_preview(&artifact, "preview:1", "diff:1", "mono:preview");
        preview.validate().expect("same-identity preview validates");
        assert!(preview.same_identity);
        assert_eq!(preview.scope_drift, ScopeDriftClass::SameIdentity);
        assert!(preview.diff.is_none());
    }

    #[test]
    fn reopen_parity_packet_preserves_identity_and_lists_degrades() {
        let artifact = portable_artifact();
        let packet = artifact.project_reopen_parity_packet(
            "parity:0",
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Degraded(ScopeDegradedReason::RebindingRequired),
            ScopeReopenPosture::Exact,
            "mono:parity",
        );
        packet.validate().expect("parity must validate");
        assert!(packet.identity_preserved_across_consumers);
        assert_eq!(packet.bindings.len(), 3);
        assert_eq!(packet.exact_consumer_classes.len(), 2);
        assert_eq!(packet.degraded.len(), 1);
        assert_eq!(
            packet.degraded[0].consumer_class,
            WorksetScopeConsumerClass::RemoteUi
        );
        assert_eq!(
            packet.degraded[0].reason,
            ScopeDegradedReason::RebindingRequired
        );
    }

    #[test]
    fn reopen_parity_rejects_managed_support_export() {
        let artifact = policy_limited_artifact();
        let mut packet = artifact.project_reopen_parity_packet(
            "parity:managed",
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Exact,
            "mono:parity",
        );
        // Inject an illegal support_export binding.
        packet.bindings.push(artifact.project_consumer_binding(
            WorksetScopeConsumerClass::SupportExport,
            ScopeReopenPosture::Exact,
            "mono:parity",
        ));
        let err = packet
            .validate()
            .expect_err("managed-locked support export forbidden");
        assert!(matches!(
            err,
            WorksetReopenParityError::PortabilityLabelInvalidForExport(
                WorksetPortabilityLabel::ManagedProviderLocked
            )
        ));
    }

    #[test]
    fn support_export_bundles_switcher_previews_and_parity_packets() {
        let artifacts = vec![
            portable_artifact(),
            sparse_artifact(),
            policy_limited_artifact(),
        ];
        let switcher = project_switcher_record(
            "switcher:bundle",
            "wksp:test",
            &artifacts[0].workset_id,
            &artifacts,
            "mono:test",
        );
        let preview = artifacts[0].project_activation_preview(
            &artifacts[1],
            "preview:bundle",
            "diff:bundle",
            "mono:test",
        );
        let parity = artifacts[0].project_reopen_parity_packet(
            "parity:bundle",
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Exact,
            ScopeReopenPosture::Exact,
            "mono:test",
        );
        let bundle = WorksetSwitcherBetaSupportExport {
            record_kind: WorksetSwitcherBetaSupportExport::RECORD_KIND.to_string(),
            schema_version: WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
            switcher,
            activation_previews: vec![preview],
            reopen_parity_packets: vec![parity],
            emitted_at: "mono:test".to_string(),
        };
        bundle.validate().expect("bundle must validate");
        let payload = serde_json::to_string(&bundle).expect("bundle serializes");
        let parsed: WorksetSwitcherBetaSupportExport =
            serde_json::from_str(&payload).expect("bundle round-trips");
        assert_eq!(parsed, bundle);
    }
}
