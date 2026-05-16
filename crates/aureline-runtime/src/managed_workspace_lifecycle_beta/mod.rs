//! Beta managed-workspace lifecycle truth with lineage across the
//! start → ready → suspend → resume → reconnect → retire phases.
//!
//! The alpha [`crate::managed_alpha`] module owns the suspend/resume/reattach
//! truth for one helper-backed preview/runtime row. This beta layer promotes
//! that primitive into a controlled lifecycle vocabulary every managed-workspace
//! surface (title bar, remote strip, activity center, docs/help, support
//! packet) reads so users can answer "is this workspace starting, live,
//! suspended, resuming, degraded, retiring, or retired?" without forking truth
//! per surface.
//!
//! Every beta record carries:
//!
//! - a closed [`ManagedLifecyclePhaseClass`] (start, ready, suspend, resume,
//!   reconnect, retire);
//! - a derived [`ManagedLifecycleStateClass`] surfaces render verbatim;
//! - a closed [`ManagedLocalEditingContinuityClass`] that names whether local
//!   editing remains usable while remote authority is paused, narrowed, or
//!   gone;
//! - an ordered lineage of [`ManagedLifecycleLineageEntry`] rows so support
//!   exports can reconstruct the path the workspace took, instead of seeing
//!   only the current state;
//! - the same `(row_id, workspace_ref)` pair the support export, surface
//!   projections, and docs/help references consume.
//!
//! The machine-readable boundary lives at
//! [`/schemas/providers/managed_workspace_lifecycle.schema.json`](../../../../schemas/providers/managed_workspace_lifecycle.schema.json).
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/managed_workspace_lifecycle_beta.md`](../../../../docs/runtime/m3/managed_workspace_lifecycle_beta.md).

use serde::{Deserialize, Serialize};

use crate::managed_alpha::{
    ManagedReachabilityClass, ManagedWorkspaceAlphaRecord, ManagedWorkspaceLifecycleState,
    ManagedWorkspaceTransitionReason,
};

/// Schema version for the managed-workspace lifecycle beta records.
pub const MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one beta lifecycle record.
pub const MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND: &str =
    "managed_workspace_lifecycle_beta_record";

/// Stable record-kind tag for one beta surface projection.
pub const MANAGED_WORKSPACE_LIFECYCLE_BETA_SURFACE_PROJECTION_RECORD_KIND: &str =
    "managed_workspace_lifecycle_beta_surface_projection_record";

/// Stable record-kind tag for the beta support-export bundle.
pub const MANAGED_WORKSPACE_LIFECYCLE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "managed_workspace_lifecycle_beta_support_export_record";

/// Closed lifecycle phase vocabulary.
///
/// A managed workspace moves through these named phases: it is `start`-ing
/// (provisioning, allocating, booting, attaching), reaches `ready`, may
/// `suspend` (with or without snapshot), may `resume`, may need `reconnect`
/// after a session loss or auth refresh, and finally `retire`s when its
/// reopen path is gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedLifecyclePhaseClass {
    /// Workspace is being provisioned, allocated, booted, or attached for the
    /// first time in this session. No live mutation yet.
    Start,
    /// Workspace is live and ready for normal work.
    Ready,
    /// Compute is paused; the workspace is intentionally not accepting live
    /// traffic.
    Suspend,
    /// Resume is in progress or has completed against the same target witness.
    Resume,
    /// A reconnect, reattach, or reauth path must complete before mutation or
    /// rerun. The workspace has not been retired.
    Reconnect,
    /// The workspace is retiring or has retired; no live reopen path remains.
    Retire,
}

impl ManagedLifecyclePhaseClass {
    /// All beta lifecycle phases.
    pub const ALL: [Self; 6] = [
        Self::Start,
        Self::Ready,
        Self::Suspend,
        Self::Resume,
        Self::Reconnect,
        Self::Retire,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Ready => "ready",
            Self::Suspend => "suspend",
            Self::Resume => "resume",
            Self::Reconnect => "reconnect",
            Self::Retire => "retire",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Start => "Starting",
            Self::Ready => "Ready",
            Self::Suspend => "Suspended",
            Self::Resume => "Resuming",
            Self::Reconnect => "Reconnect required",
            Self::Retire => "Retiring",
        }
    }
}

/// Closed lifecycle state vocabulary surfaces render verbatim.
///
/// This is what title bars, remote strips, activity center cards, docs/help,
/// and support packets *display*; the [`ManagedLifecyclePhaseClass`] is the
/// stable phase the surface is consuming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedLifecycleStateClass {
    /// The workspace is starting; surfaces show a starting/booting state.
    Starting,
    /// The workspace is live; surfaces show ready / connected.
    Live,
    /// The workspace is suspended; surfaces show suspended-no-traffic.
    Suspended,
    /// The workspace is resuming; surfaces show resuming-pending-ready.
    Resuming,
    /// The workspace is degraded but reachable; mutation may be narrowed.
    Degraded,
    /// The workspace requires reconnect/reauth/reattach before mutation.
    ReconnectRequired,
    /// The workspace is retiring; reopen path is closing but not yet closed.
    Retiring,
    /// The workspace is retired; no live reopen path remains.
    Retired,
}

impl ManagedLifecycleStateClass {
    /// All beta lifecycle states.
    pub const ALL: [Self; 8] = [
        Self::Starting,
        Self::Live,
        Self::Suspended,
        Self::Resuming,
        Self::Degraded,
        Self::ReconnectRequired,
        Self::Retiring,
        Self::Retired,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Live => "live",
            Self::Suspended => "suspended",
            Self::Resuming => "resuming",
            Self::Degraded => "degraded",
            Self::ReconnectRequired => "reconnect_required",
            Self::Retiring => "retiring",
            Self::Retired => "retired",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Starting => "Starting",
            Self::Live => "Live",
            Self::Suspended => "Suspended",
            Self::Resuming => "Resuming",
            Self::Degraded => "Degraded",
            Self::ReconnectRequired => "Reconnect required",
            Self::Retiring => "Retiring",
            Self::Retired => "Retired",
        }
    }

    /// True when no live reopen path remains.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// True when remote mutation is admitted in this state.
    pub const fn admits_remote_mutation(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Closed continuity vocabulary describing what local editing the user may do
/// while the managed runtime is paused, narrowed, or gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedLocalEditingContinuityClass {
    /// Local editing continues normally; saves can flow through to remote when
    /// the workspace returns to `Live`.
    PreservedFullLocalEditing,
    /// Local editing remains, but writes stay local-only until the workspace
    /// is reconnected, resumed, or rebuilt.
    PreservedLocalOnlyWrites,
    /// Local editing is inspect-only until the workspace recovers. Saves to
    /// the workspace are blocked.
    InspectOnlyUntilRecovery,
    /// Local editing continuity is not applicable; for example, a row that
    /// only describes a workspace that never opened, or that retired before
    /// local editing began.
    NotApplicable,
}

impl ManagedLocalEditingContinuityClass {
    /// All beta continuity classes.
    pub const ALL: [Self; 4] = [
        Self::PreservedFullLocalEditing,
        Self::PreservedLocalOnlyWrites,
        Self::InspectOnlyUntilRecovery,
        Self::NotApplicable,
    ];

    /// Stable token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedFullLocalEditing => "preserved_full_local_editing",
            Self::PreservedLocalOnlyWrites => "preserved_local_only_writes",
            Self::InspectOnlyUntilRecovery => "inspect_only_until_recovery",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PreservedFullLocalEditing => "Local editing preserved",
            Self::PreservedLocalOnlyWrites => "Local editing preserved (local-only writes)",
            Self::InspectOnlyUntilRecovery => "Inspect only until recovery",
            Self::NotApplicable => "Not applicable",
        }
    }

    /// True when the user may still type in the editor.
    pub const fn preserves_local_editing(self) -> bool {
        matches!(
            self,
            Self::PreservedFullLocalEditing | Self::PreservedLocalOnlyWrites
        )
    }
}

/// Surfaces that consume the beta record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedSurfaceClass {
    /// Title bar / window chrome.
    TitleBar,
    /// Remote strip / activity bar segment that shows lifecycle truth.
    RemoteStrip,
    /// Activity center entry (notification, lifecycle row).
    ActivityCenter,
    /// In-product docs/help reader.
    DocsHelp,
    /// Support / export packet projection.
    SupportPacket,
}

impl ManagedSurfaceClass {
    /// All beta surface classes.
    pub const ALL: [Self; 5] = [
        Self::TitleBar,
        Self::RemoteStrip,
        Self::ActivityCenter,
        Self::DocsHelp,
        Self::SupportPacket,
    ];

    /// Stable token recorded in projections, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleBar => "title_bar",
            Self::RemoteStrip => "remote_strip",
            Self::ActivityCenter => "activity_center",
            Self::DocsHelp => "docs_help",
            Self::SupportPacket => "support_packet",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TitleBar => "Title bar",
            Self::RemoteStrip => "Remote strip",
            Self::ActivityCenter => "Activity center",
            Self::DocsHelp => "Docs / help",
            Self::SupportPacket => "Support packet",
        }
    }
}

/// One ordered entry in the lifecycle lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedLifecycleLineageEntry {
    /// Lifecycle phase recorded by this entry.
    pub phase: ManagedLifecyclePhaseClass,
    /// Stable phase token.
    pub phase_token: String,
    /// State the workspace was in *after* this entry.
    pub state: ManagedLifecycleStateClass,
    /// Stable state token.
    pub state_token: String,
    /// Reason the workspace entered this phase.
    pub reason: ManagedWorkspaceTransitionReason,
    /// Stable reason token (mirrored from the alpha vocabulary).
    pub reason_token: String,
    /// Caller-supplied timestamp.
    pub observed_at: String,
    /// Export-safe evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Review-safe one-line summary for this lineage entry.
    pub summary: String,
}

impl ManagedLifecycleLineageEntry {
    /// Builds a lineage entry from the typed components.
    pub fn new(
        phase: ManagedLifecyclePhaseClass,
        state: ManagedLifecycleStateClass,
        reason: ManagedWorkspaceTransitionReason,
        observed_at: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            phase_token: phase.as_str().to_owned(),
            state,
            state_token: state.as_str().to_owned(),
            reason,
            reason_token: reason.as_str().to_owned(),
            observed_at: observed_at.into(),
            evidence_refs: Vec::new(),
            summary: summary.into(),
        }
    }
}

/// One beta lifecycle record.
///
/// The same `(row_id, workspace_ref)` pair appears in surface projections and
/// support exports so reviewers, support, and product surfaces read one truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleBetaRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the beta record family.
    pub schema_version: u32,
    /// Stable row id shared with the surface projections and support export.
    pub row_id: String,
    /// Opaque managed-workspace ref.
    pub workspace_ref: String,
    /// Opaque workspace instance ref when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_instance_ref: Option<String>,
    /// Caller-supplied generation timestamp.
    pub generated_at: String,
    /// Current lifecycle phase.
    pub current_phase: ManagedLifecyclePhaseClass,
    /// Stable phase token.
    pub current_phase_token: String,
    /// Reviewer-facing phase label.
    pub current_phase_label: String,
    /// Current lifecycle state surfaces render.
    pub current_state: ManagedLifecycleStateClass,
    /// Stable state token.
    pub current_state_token: String,
    /// Reviewer-facing state label.
    pub current_state_label: String,
    /// Local-editing continuity class.
    pub local_editing_continuity: ManagedLocalEditingContinuityClass,
    /// Stable continuity token.
    pub local_editing_continuity_token: String,
    /// Reviewer-facing continuity label.
    pub local_editing_continuity_label: String,
    /// True when remote mutation is currently admitted.
    pub mutation_allowed: bool,
    /// True when reconnect/reauth/reattach must complete before mutation.
    pub reconnect_required: bool,
    /// Ordered lineage; the last entry's phase and state MUST equal the
    /// current_phase / current_state fields.
    pub lineage: Vec<ManagedLifecycleLineageEntry>,
    /// Review-safe one-line visible summary.
    pub visible_summary: String,
    /// Review-safe one-line safe-continuation summary.
    pub safe_continuation: String,
    /// Compatibility, schema, fixture, doc, and support refs.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// Redaction-safe support packet refs the record can be embedded under.
    #[serde(default)]
    pub support_packet_refs: Vec<String>,
    /// Surface refs each projection attaches to.
    #[serde(default)]
    pub surface_refs: Vec<String>,
    /// True because raw tokens, endpoints, paths, and secrets are excluded.
    pub redaction_safe: bool,
}

impl ManagedWorkspaceLifecycleBetaRecord {
    /// Builds a beta record from explicit lineage. The caller is responsible
    /// for ensuring lineage is non-empty and ordered; [`Self::validate`]
    /// reports any inconsistencies between the lineage tail and the declared
    /// current phase/state.
    #[allow(clippy::too_many_arguments)]
    pub fn from_lineage(
        row_id: impl Into<String>,
        workspace_ref: impl Into<String>,
        workspace_instance_ref: Option<String>,
        generated_at: impl Into<String>,
        local_editing_continuity: ManagedLocalEditingContinuityClass,
        lineage: Vec<ManagedLifecycleLineageEntry>,
        visible_summary: impl Into<String>,
        safe_continuation: impl Into<String>,
        source_refs: Vec<String>,
        support_packet_refs: Vec<String>,
        surface_refs: Vec<String>,
    ) -> Self {
        let generated_at = generated_at.into();
        let last = lineage
            .last()
            .cloned()
            .unwrap_or_else(|| zero_lineage_entry(&generated_at));
        let current_phase = last.phase;
        let current_state = last.state;
        let mutation_allowed = current_state.admits_remote_mutation();
        let reconnect_required = matches!(current_state, ManagedLifecycleStateClass::ReconnectRequired);
        Self {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION,
            row_id: row_id.into(),
            workspace_ref: workspace_ref.into(),
            workspace_instance_ref,
            generated_at,
            current_phase,
            current_phase_token: current_phase.as_str().to_owned(),
            current_phase_label: current_phase.label().to_owned(),
            current_state,
            current_state_token: current_state.as_str().to_owned(),
            current_state_label: current_state.label().to_owned(),
            local_editing_continuity,
            local_editing_continuity_token: local_editing_continuity.as_str().to_owned(),
            local_editing_continuity_label: local_editing_continuity.label().to_owned(),
            mutation_allowed,
            reconnect_required,
            lineage,
            visible_summary: visible_summary.into(),
            safe_continuation: safe_continuation.into(),
            source_refs,
            support_packet_refs,
            surface_refs,
            redaction_safe: true,
        }
    }

    /// Builds a beta record by projecting an alpha managed-workspace row plus
    /// the lineage of prior phases the caller has on hand. The alpha row
    /// supplies the *current* row; the prior_lineage supplies whatever phases
    /// the caller can prove happened before it (start, ready, ...).
    #[allow(clippy::too_many_arguments)]
    pub fn from_alpha_record(
        row_id: impl Into<String>,
        alpha: &ManagedWorkspaceAlphaRecord,
        prior_lineage: Vec<ManagedLifecycleLineageEntry>,
        local_editing_continuity: ManagedLocalEditingContinuityClass,
        source_refs: Vec<String>,
        support_packet_refs: Vec<String>,
        surface_refs: Vec<String>,
    ) -> Self {
        let phase = ManagedLifecyclePhaseClass::derive_from_alpha(
            alpha.lifecycle_state,
            alpha.boundary.reachability_class,
        );
        let state = ManagedLifecycleStateClass::derive_from_alpha(
            alpha.lifecycle_state,
            alpha.boundary.reachability_class,
        );
        let tail_summary = format!(
            "alpha-row {} resolved to phase={} state={}",
            alpha.managed_workspace_ref,
            phase.as_str(),
            state.as_str()
        );
        let mut lineage = prior_lineage;
        lineage.push(ManagedLifecycleLineageEntry {
            phase,
            phase_token: phase.as_str().to_owned(),
            state,
            state_token: state.as_str().to_owned(),
            reason: alpha.transition.reason,
            reason_token: alpha.transition.reason.as_str().to_owned(),
            observed_at: alpha.transition.observed_at.clone(),
            evidence_refs: alpha.transition.evidence_refs.clone(),
            summary: tail_summary,
        });

        Self::from_lineage(
            row_id,
            alpha.managed_workspace_ref.clone(),
            None,
            alpha.updated_at.clone(),
            local_editing_continuity,
            lineage,
            alpha.summary.clone(),
            alpha.summary.clone(),
            source_refs,
            support_packet_refs,
            surface_refs,
        )
    }

    /// True when this record fails closed for mutating remote work.
    pub fn fails_closed_for_mutation(&self) -> bool {
        !self.mutation_allowed
    }

    /// Returns one deterministic plaintext summary line for status surfaces.
    pub fn summary_line(&self) -> String {
        format!(
            "row={}; workspace={}; phase={}; state={}; continuity={}; mutation_allowed={}; reconnect_required={}",
            self.row_id,
            self.workspace_ref,
            self.current_phase_token,
            self.current_state_token,
            self.local_editing_continuity_token,
            self.mutation_allowed,
            self.reconnect_required,
        )
    }

    /// Projects a surface-specific record for the requested surface.
    pub fn projection(
        &self,
        surface: ManagedSurfaceClass,
    ) -> ManagedWorkspaceLifecycleBetaSurfaceProjection {
        let lineage_tokens = self
            .lineage
            .iter()
            .map(|entry| format!("{}->{}", entry.phase_token, entry.state_token))
            .collect::<Vec<_>>();
        let lineage_path = lineage_tokens.join("|");
        let header = match surface {
            ManagedSurfaceClass::TitleBar => format!(
                "{} • {}",
                self.current_state.label(),
                self.local_editing_continuity.label()
            ),
            ManagedSurfaceClass::RemoteStrip => {
                format!("Remote workspace: {}", self.current_state.label())
            }
            ManagedSurfaceClass::ActivityCenter => format!(
                "Managed workspace {} — {}",
                self.current_state_token, self.visible_summary
            ),
            ManagedSurfaceClass::DocsHelp => format!(
                "{} (lineage {} steps)",
                self.current_state.label(),
                self.lineage.len()
            ),
            ManagedSurfaceClass::SupportPacket => format!(
                "{} / {}: {}",
                self.current_phase_token, self.current_state_token, self.visible_summary
            ),
        };
        ManagedWorkspaceLifecycleBetaSurfaceProjection {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_BETA_SURFACE_PROJECTION_RECORD_KIND
                .to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION,
            row_id: self.row_id.clone(),
            workspace_ref: self.workspace_ref.clone(),
            surface,
            surface_token: surface.as_str().to_owned(),
            current_phase_token: self.current_phase_token.clone(),
            current_state_token: self.current_state_token.clone(),
            local_editing_continuity_token: self.local_editing_continuity_token.clone(),
            mutation_allowed: self.mutation_allowed,
            reconnect_required: self.reconnect_required,
            lineage_tokens,
            lineage_path,
            header,
            visible_summary: self.visible_summary.clone(),
            safe_continuation: self.safe_continuation.clone(),
            redaction_safe: true,
        }
    }

    /// Returns validation issues that would make this row overclaim truth.
    pub fn validate(&self) -> Vec<ManagedWorkspaceLifecycleBetaViolation> {
        let mut issues = Vec::new();
        if self.record_kind != MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND {
            issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                "unexpected_record_kind",
                "record_kind",
                "managed-workspace lifecycle beta record kind must stay canonical",
            ));
        }
        if self.schema_version != MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION {
            issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                "unexpected_schema_version",
                "schema_version",
                "managed-workspace lifecycle beta schema version must match this crate",
            ));
        }
        if self.lineage.is_empty() {
            issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                "empty_lineage",
                "lineage",
                "lifecycle beta records must carry a non-empty lineage",
            ));
        } else {
            let tail = &self.lineage[self.lineage.len() - 1];
            if tail.phase != self.current_phase {
                issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                    "lineage_tail_phase_mismatch",
                    "lineage[last].phase",
                    "lineage tail phase must equal the current phase",
                ));
            }
            if tail.state != self.current_state {
                issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                    "lineage_tail_state_mismatch",
                    "lineage[last].state",
                    "lineage tail state must equal the current state",
                ));
            }
        }
        if self.mutation_allowed && !self.current_state.admits_remote_mutation() {
            issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                "mutation_allowed_in_non_live_state",
                "mutation_allowed",
                "mutation must be allowed only when current state is live",
            ));
        }
        if self.reconnect_required
            != matches!(self.current_state, ManagedLifecycleStateClass::ReconnectRequired)
        {
            issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                "reconnect_required_state_mismatch",
                "reconnect_required",
                "reconnect_required must be set iff state is reconnect_required",
            ));
        }
        if self.local_editing_continuity == ManagedLocalEditingContinuityClass::NotApplicable
            && self.current_state != ManagedLifecycleStateClass::Retired
            && !matches!(self.current_state, ManagedLifecycleStateClass::Starting)
        {
            issues.push(ManagedWorkspaceLifecycleBetaViolation::new(
                "continuity_not_applicable_outside_terminal",
                "local_editing_continuity",
                "not_applicable continuity is reserved for retired or starting rows",
            ));
        }
        issues
    }
}

fn zero_lineage_entry(generated_at: &impl ToString) -> ManagedLifecycleLineageEntry {
    ManagedLifecycleLineageEntry::new(
        ManagedLifecyclePhaseClass::Start,
        ManagedLifecycleStateClass::Starting,
        ManagedWorkspaceTransitionReason::LocalRuntimeInspection,
        generated_at.to_string(),
        "synthetic placeholder lineage entry",
    )
}

impl ManagedLifecyclePhaseClass {
    /// Derives the phase class from an alpha lifecycle row.
    pub fn derive_from_alpha(
        state: ManagedWorkspaceLifecycleState,
        reachability: ManagedReachabilityClass,
    ) -> Self {
        match state {
            ManagedWorkspaceLifecycleState::Ready => Self::Ready,
            ManagedWorkspaceLifecycleState::Suspended => Self::Suspend,
            ManagedWorkspaceLifecycleState::Resuming => Self::Resume,
            ManagedWorkspaceLifecycleState::Resumed => Self::Resume,
            ManagedWorkspaceLifecycleState::Reattaching => Self::Reconnect,
            ManagedWorkspaceLifecycleState::Reattached => Self::Resume,
            ManagedWorkspaceLifecycleState::ReconnectRequired => Self::Reconnect,
            ManagedWorkspaceLifecycleState::Stale => {
                if matches!(reachability, ManagedReachabilityClass::Unreachable) {
                    Self::Retire
                } else {
                    Self::Reconnect
                }
            }
            ManagedWorkspaceLifecycleState::InspectOnly => Self::Reconnect,
            ManagedWorkspaceLifecycleState::RebuildRequired => Self::Reconnect,
            ManagedWorkspaceLifecycleState::Closed => Self::Retire,
        }
    }
}

impl ManagedLifecycleStateClass {
    /// Derives the state class from an alpha lifecycle row.
    pub fn derive_from_alpha(
        state: ManagedWorkspaceLifecycleState,
        reachability: ManagedReachabilityClass,
    ) -> Self {
        match state {
            ManagedWorkspaceLifecycleState::Ready => Self::Live,
            ManagedWorkspaceLifecycleState::Suspended => Self::Suspended,
            ManagedWorkspaceLifecycleState::Resuming => Self::Resuming,
            ManagedWorkspaceLifecycleState::Resumed => Self::Live,
            ManagedWorkspaceLifecycleState::Reattaching => Self::Resuming,
            ManagedWorkspaceLifecycleState::Reattached => Self::Live,
            ManagedWorkspaceLifecycleState::ReconnectRequired => Self::ReconnectRequired,
            ManagedWorkspaceLifecycleState::Stale => match reachability {
                ManagedReachabilityClass::Unreachable => Self::Retired,
                ManagedReachabilityClass::SuspendedNoTraffic => Self::Suspended,
                _ => Self::Degraded,
            },
            ManagedWorkspaceLifecycleState::InspectOnly => Self::Degraded,
            ManagedWorkspaceLifecycleState::RebuildRequired => Self::Degraded,
            ManagedWorkspaceLifecycleState::Closed => Self::Retired,
        }
    }
}

/// Surface projection consumed by title bars, remote strips, activity center
/// cards, docs/help, and support packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleBetaSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id shared with the source record.
    pub row_id: String,
    /// Opaque managed-workspace ref shared with the source record.
    pub workspace_ref: String,
    /// Surface this projection targets.
    pub surface: ManagedSurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Stable current phase token.
    pub current_phase_token: String,
    /// Stable current state token.
    pub current_state_token: String,
    /// Stable continuity token.
    pub local_editing_continuity_token: String,
    /// Mutation flag.
    pub mutation_allowed: bool,
    /// Reconnect-required flag.
    pub reconnect_required: bool,
    /// Lineage step tokens (e.g. "ready->live").
    pub lineage_tokens: Vec<String>,
    /// Lineage joined into one path string for compact rendering.
    pub lineage_path: String,
    /// Surface-specific header copy derived from the typed state.
    pub header: String,
    /// Review-safe one-line visible summary.
    pub visible_summary: String,
    /// Review-safe safe-continuation summary.
    pub safe_continuation: String,
    /// True because raw tokens and endpoints are excluded.
    pub redaction_safe: bool,
}

/// Support-export bundle. The bundle holds the beta records, the projections
/// for the support-packet surface, and a flag describing whether any record
/// in the export fails closed for mutating remote work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Beta records included in this bundle.
    pub records: Vec<ManagedWorkspaceLifecycleBetaRecord>,
    /// Support-packet projections derived from the same records.
    pub support_projections: Vec<ManagedWorkspaceLifecycleBetaSurfaceProjection>,
    /// True when at least one record fails closed for mutating work.
    pub any_record_fails_closed_for_mutation: bool,
    /// True because raw payloads, endpoints, paths, and secrets are excluded.
    pub redaction_safe: bool,
}

impl ManagedWorkspaceLifecycleBetaSupportExport {
    /// Builds the support-export bundle from a sequence of beta records.
    pub fn from_records<'a>(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: impl IntoIterator<Item = &'a ManagedWorkspaceLifecycleBetaRecord>,
    ) -> Self {
        let records: Vec<ManagedWorkspaceLifecycleBetaRecord> =
            records.into_iter().cloned().collect();
        let support_projections = records
            .iter()
            .map(|record| record.projection(ManagedSurfaceClass::SupportPacket))
            .collect::<Vec<_>>();
        let any_record_fails_closed_for_mutation = records
            .iter()
            .any(ManagedWorkspaceLifecycleBetaRecord::fails_closed_for_mutation);
        Self {
            record_kind: MANAGED_WORKSPACE_LIFECYCLE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            records,
            support_projections,
            any_record_fails_closed_for_mutation,
            redaction_safe: true,
        }
    }

    /// Renders stable plaintext lines for support exports.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Managed-workspace lifecycle beta export: {}\n",
            self.support_export_id
        );
        for record in &self.records {
            out.push_str(&record.summary_line());
            out.push('\n');
        }
        out
    }
}

/// Validation issue raised when a beta record overclaims truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedWorkspaceLifecycleBetaViolation {
    /// Stable violation code.
    pub code: String,
    /// Dotted field path responsible for the issue.
    pub field_path: String,
    /// Review-safe issue summary.
    pub summary: String,
}

impl ManagedWorkspaceLifecycleBetaViolation {
    /// Creates a new violation entry.
    pub fn new(
        code: impl Into<String>,
        field_path: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            field_path: field_path.into(),
            summary: summary.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lineage_entry(
        phase: ManagedLifecyclePhaseClass,
        state: ManagedLifecycleStateClass,
        reason: ManagedWorkspaceTransitionReason,
        observed_at: &str,
        summary: &str,
    ) -> ManagedLifecycleLineageEntry {
        ManagedLifecycleLineageEntry::new(phase, state, reason, observed_at, summary)
    }

    #[test]
    fn ready_record_admits_mutation() {
        let lineage = vec![
            lineage_entry(
                ManagedLifecyclePhaseClass::Start,
                ManagedLifecycleStateClass::Starting,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:00:00Z",
                "starting",
            ),
            lineage_entry(
                ManagedLifecyclePhaseClass::Ready,
                ManagedLifecycleStateClass::Live,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:01:00Z",
                "live",
            ),
        ];
        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.ready",
            "managed_workspace:test",
            None,
            "2026-05-15T10:01:00Z",
            ManagedLocalEditingContinuityClass::PreservedFullLocalEditing,
            lineage,
            "Ready",
            "Continue working",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        assert!(record.mutation_allowed);
        assert!(!record.reconnect_required);
        assert!(!record.fails_closed_for_mutation());
        assert!(record.validate().is_empty());
    }

    #[test]
    fn suspended_record_preserves_local_editing() {
        let lineage = vec![
            lineage_entry(
                ManagedLifecyclePhaseClass::Ready,
                ManagedLifecycleStateClass::Live,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:00:00Z",
                "live",
            ),
            lineage_entry(
                ManagedLifecyclePhaseClass::Suspend,
                ManagedLifecycleStateClass::Suspended,
                ManagedWorkspaceTransitionReason::UserRequestedSuspend,
                "2026-05-15T11:00:00Z",
                "suspend",
            ),
        ];
        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.suspend",
            "managed_workspace:test",
            None,
            "2026-05-15T11:00:00Z",
            ManagedLocalEditingContinuityClass::PreservedLocalOnlyWrites,
            lineage,
            "Suspended",
            "Continue locally",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        assert!(!record.mutation_allowed);
        assert!(!record.reconnect_required);
        assert!(record
            .local_editing_continuity
            .preserves_local_editing());
        assert!(record.fails_closed_for_mutation());
        assert!(record.validate().is_empty());
    }

    #[test]
    fn reconnect_required_record_blocks_mutation() {
        let lineage = vec![
            lineage_entry(
                ManagedLifecyclePhaseClass::Ready,
                ManagedLifecycleStateClass::Live,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:00:00Z",
                "live",
            ),
            lineage_entry(
                ManagedLifecyclePhaseClass::Reconnect,
                ManagedLifecycleStateClass::ReconnectRequired,
                ManagedWorkspaceTransitionReason::TargetWitnessStale,
                "2026-05-15T11:00:00Z",
                "reconnect required",
            ),
        ];
        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.reconnect",
            "managed_workspace:test",
            None,
            "2026-05-15T11:00:00Z",
            ManagedLocalEditingContinuityClass::InspectOnlyUntilRecovery,
            lineage,
            "Reconnect required",
            "Inspect locally; reconnect to resume work",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        assert!(!record.mutation_allowed);
        assert!(record.reconnect_required);
        assert!(record.fails_closed_for_mutation());
    }

    #[test]
    fn retired_record_is_terminal() {
        let lineage = vec![
            lineage_entry(
                ManagedLifecyclePhaseClass::Ready,
                ManagedLifecycleStateClass::Live,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-10T10:00:00Z",
                "live",
            ),
            lineage_entry(
                ManagedLifecyclePhaseClass::Retire,
                ManagedLifecycleStateClass::Retired,
                ManagedWorkspaceTransitionReason::ReconnectWindowElapsed,
                "2026-05-12T10:00:00Z",
                "retired",
            ),
        ];
        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.retire",
            "managed_workspace:test",
            None,
            "2026-05-12T10:00:00Z",
            ManagedLocalEditingContinuityClass::NotApplicable,
            lineage,
            "Retired",
            "Local artifacts only; no remote reopen path",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        assert!(!record.mutation_allowed);
        assert!(record.current_state.is_terminal());
        assert!(record.validate().is_empty());
    }

    #[test]
    fn lineage_tail_mismatch_is_flagged() {
        let lineage = vec![lineage_entry(
            ManagedLifecyclePhaseClass::Ready,
            ManagedLifecycleStateClass::Live,
            ManagedWorkspaceTransitionReason::UserRequestedResume,
            "2026-05-15T10:00:00Z",
            "live",
        )];
        let mut record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.mismatch",
            "managed_workspace:test",
            None,
            "2026-05-15T10:00:00Z",
            ManagedLocalEditingContinuityClass::PreservedFullLocalEditing,
            lineage,
            "Ready",
            "Continue working",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        record.current_state = ManagedLifecycleStateClass::Suspended;
        record.current_state_token = ManagedLifecycleStateClass::Suspended.as_str().to_owned();
        let issues = record.validate();
        assert!(issues
            .iter()
            .any(|issue| issue.code == "lineage_tail_state_mismatch"));
    }

    #[test]
    fn projections_render_one_truth_across_surfaces() {
        let lineage = vec![
            lineage_entry(
                ManagedLifecyclePhaseClass::Start,
                ManagedLifecycleStateClass::Starting,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:00:00Z",
                "starting",
            ),
            lineage_entry(
                ManagedLifecyclePhaseClass::Ready,
                ManagedLifecycleStateClass::Live,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:01:00Z",
                "live",
            ),
        ];
        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.projections",
            "managed_workspace:projection_test",
            None,
            "2026-05-15T10:01:00Z",
            ManagedLocalEditingContinuityClass::PreservedFullLocalEditing,
            lineage,
            "Live",
            "Continue working",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        for surface in ManagedSurfaceClass::ALL {
            let projection = record.projection(surface);
            assert_eq!(projection.row_id, record.row_id);
            assert_eq!(projection.workspace_ref, record.workspace_ref);
            assert_eq!(projection.current_state_token, record.current_state_token);
            assert_eq!(projection.current_phase_token, record.current_phase_token);
            assert_eq!(projection.mutation_allowed, record.mutation_allowed);
            assert_eq!(projection.lineage_tokens.len(), record.lineage.len());
        }
    }

    #[test]
    fn support_export_bundles_share_row_ids() {
        let lineage = vec![
            lineage_entry(
                ManagedLifecyclePhaseClass::Ready,
                ManagedLifecycleStateClass::Live,
                ManagedWorkspaceTransitionReason::UserRequestedResume,
                "2026-05-15T10:00:00Z",
                "live",
            ),
            lineage_entry(
                ManagedLifecyclePhaseClass::Suspend,
                ManagedLifecycleStateClass::Suspended,
                ManagedWorkspaceTransitionReason::UserRequestedSuspend,
                "2026-05-15T11:00:00Z",
                "suspend",
            ),
        ];
        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            "managed-workspace-lifecycle-beta-row:test.export",
            "managed_workspace:test",
            None,
            "2026-05-15T11:00:00Z",
            ManagedLocalEditingContinuityClass::PreservedLocalOnlyWrites,
            lineage,
            "Suspended",
            "Continue locally",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let export = ManagedWorkspaceLifecycleBetaSupportExport::from_records(
            "support_export:managed_workspace_lifecycle_beta.test",
            "2026-05-15T11:01:00Z",
            std::iter::once(&record),
        );
        assert_eq!(export.records.len(), 1);
        assert_eq!(export.support_projections.len(), 1);
        assert_eq!(export.support_projections[0].row_id, record.row_id);
        assert!(export.any_record_fails_closed_for_mutation);
        let plaintext = export.render_plaintext();
        assert!(plaintext.contains(&record.row_id));
    }
}
