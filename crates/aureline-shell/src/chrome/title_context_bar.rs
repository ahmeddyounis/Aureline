//! Title/context bar identity tuple and projection record.
//!
//! This module materializes the canonical `title_context_bar_state_record`
//! described by `docs/ux/title_context_bar_contract.md`. The live shell, native
//! window title, workspace status surfaces, and support/export packets must
//! project from the same record rather than recomputing identity fields per
//! surface.

use std::path::{Path, PathBuf};

use aureline_commands::invocation::now_rfc3339;
use aureline_vfs::WatcherHealth;
use aureline_workspace::{
    TrustState as WorkspaceTrustState, WorkspaceLifecycleMachine,
    WorkspaceLifecycleState as RuntimeWorkspaceLifecycleState,
};
use serde::{Deserialize, Serialize};

/// Runtime inputs used to build the current title/context bar identity record.
#[derive(Debug, Clone, Copy)]
pub struct TitleContextBarRuntimeInputs<'a> {
    /// Workspace label rendered in chrome and native window titles.
    pub workspace_label: Option<&'a str>,
    /// Workspace root path when a local workspace is active.
    pub workspace_root: Option<&'a Path>,
    /// Workspace lifecycle machine snapshot when an active workspace exists.
    pub workspace_lifecycle: Option<&'a WorkspaceLifecycleMachine>,
    /// Current trust posture token (`trusted`, `restricted`, ...).
    pub workspace_trust_state_token: &'a str,
}

/// Mutable runtime state that keeps the canonical identity tuple stable across frames.
#[derive(Debug, Clone)]
pub struct TitleContextBarRuntimeState {
    record: TitleContextBarStateRecord,
    last_logged: Option<TitleContextBarLogKey>,
    state_path: PathBuf,
    last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TitleContextBarLogKey {
    workspace_label: String,
    workspace_kind: WorkspaceKind,
    lifecycle_state: WorkspaceLifecycleState,
    repo_state_class: RepoStateClass,
    repo_label: Option<String>,
    branch_label: Option<String>,
    trust_state: TrustState,
    trust_source: TrustSourceClass,
    host_class: HostClass,
    host_state: HostStateClass,
    host_label: Option<String>,
    route_kind: RouteKind,
    route_label: String,
    route_freshness: RouteFreshnessClass,
    degraded_tokens: Vec<DegradedStateToken>,
    recovery_mode: RecoveryModeClass,
    last_failure_summary: Option<String>,
}

impl TitleContextBarRuntimeState {
    /// Creates a new runtime state rooted at the empty-shell identity tuple.
    pub fn new() -> Self {
        let base = PathBuf::from(".logs").join("ux");
        Self {
            record: materialize_identity_tuple(TitleContextBarRuntimeInputs {
                workspace_label: None,
                workspace_root: None,
                workspace_lifecycle: None,
                workspace_trust_state_token: "trusted",
            }),
            last_logged: None,
            state_path: base.join("title_context_bar_state.json"),
            last_error: None,
        }
    }

    /// Returns the latest canonical record.
    pub fn record(&self) -> &TitleContextBarStateRecord {
        &self.record
    }

    /// Returns the last serialization error, if any.
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// Refreshes the identity tuple from `inputs`, returning true when it changes.
    pub fn update(&mut self, inputs: TitleContextBarRuntimeInputs<'_>) -> bool {
        let candidate = materialize_identity_tuple(inputs);
        let key = TitleContextBarLogKey {
            workspace_label: candidate.workspace_identity.display_label.clone(),
            workspace_kind: candidate.workspace_identity.workspace_kind,
            lifecycle_state: candidate.workspace_identity.lifecycle_state,
            repo_state_class: candidate.repo_identity.repo_state_class,
            repo_label: candidate.repo_identity.repo_label.clone(),
            branch_label: candidate.repo_identity.branch_label.clone(),
            trust_state: candidate.trust_identity.trust_state,
            trust_source: candidate.trust_identity.trust_source,
            host_class: candidate.host_identity.host_class,
            host_state: candidate.host_identity.host_state,
            host_label: candidate.host_identity.target_label.clone(),
            route_kind: candidate.route_state.route_kind,
            route_label: candidate.route_state.route_label.clone(),
            route_freshness: candidate.route_state.route_freshness,
            degraded_tokens: candidate.degraded_or_recovery_state.degraded_tokens.clone(),
            recovery_mode: candidate.degraded_or_recovery_state.recovery_mode,
            last_failure_summary: candidate
                .degraded_or_recovery_state
                .last_failure_summary
                .clone(),
        };

        if self.last_logged.as_ref() == Some(&key) {
            return false;
        }

        self.record = TitleContextBarStateRecord {
            updated_at: now_rfc3339(),
            ..candidate
        };
        self.last_logged = Some(key);
        self.flush_state_log();
        true
    }

    fn flush_state_log(&mut self) {
        if let Some(parent) = self.state_path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                self.last_error = Some(format!("title/context bar log dir create failed: {err}"));
                return;
            }
        }
        match serde_json::to_string_pretty(&self.record) {
            Ok(payload) => {
                if let Err(err) = std::fs::write(&self.state_path, payload) {
                    self.last_error = Some(format!("title/context bar record write failed: {err}"));
                }
            }
            Err(err) => {
                self.last_error = Some(format!("title/context bar record serialize failed: {err}"));
            }
        }
    }
}

fn materialize_identity_tuple(
    inputs: TitleContextBarRuntimeInputs<'_>,
) -> TitleContextBarStateRecord {
    let now = now_rfc3339();

    let (
        workspace_ref,
        workspace_label,
        workspace_kind,
        lifecycle_state,
        root_readiness,
        ready_root_count,
    ) = match inputs.workspace_lifecycle {
        Some(machine) => {
            let workspace_ref =
                sanitize_or_fallback_opaque_id(machine.workspace_id(), "workspace.unknown");
            let label = inputs
                .workspace_label
                .unwrap_or("workspace")
                .trim()
                .to_string();
            let lifecycle_state = workspace_lifecycle_state_token(machine.state());

            let (workspace_kind, root_readiness, ready_root_count) =
                classify_workspace_kind(inputs.workspace_root);
            (
                workspace_ref,
                label,
                workspace_kind,
                lifecycle_state,
                root_readiness,
                ready_root_count,
            )
        }
        None => (
            "workspace.empty_shell".to_string(),
            inputs
                .workspace_label
                .unwrap_or("Start Center")
                .trim()
                .to_string(),
            WorkspaceKind::EmptyShell,
            WorkspaceLifecycleState::WorkspaceClosed,
            RootReadinessClass::NotApplicable,
            0,
        ),
    };

    let (repo_identity, host_identity) = match inputs.workspace_root {
        Some(root) => {
            let git = discover_git_identity(root);
            (
                repo_identity_from_git(&workspace_ref, git.as_ref()),
                HostIdentity {
                    host_class: HostClass::Local,
                    host_state: HostStateClass::Ready,
                    target_ref: Some("target.local.desktop".to_string()),
                    target_label: Some("Local".to_string()),
                    boundary_note: "Local workspace identity is derived from the active root."
                        .to_string(),
                },
            )
        }
        None => (
            RepoIdentity::not_applicable(),
            HostIdentity {
                host_class: HostClass::Local,
                host_state: HostStateClass::Ready,
                target_ref: Some("target.local.desktop".to_string()),
                target_label: Some("Local".to_string()),
                boundary_note: "No active workspace root is attached to this shell window."
                    .to_string(),
            },
        ),
    };

    let (trust_state, trust_source) = trust_tuple(
        inputs.workspace_lifecycle,
        inputs.workspace_trust_state_token,
    );

    let degraded_tokens = degraded_tokens_for(inputs.workspace_lifecycle);
    let recovery_mode = recovery_mode_for(trust_state, lifecycle_state, &degraded_tokens);
    let last_failure_summary = last_failure_summary_for(
        inputs.workspace_lifecycle,
        inputs.workspace_root,
        &degraded_tokens,
    );

    let route_kind = route_kind_for(lifecycle_state);
    let route_freshness = route_freshness_for(lifecycle_state, &degraded_tokens);
    let route_label = route_label_for(route_kind, lifecycle_state);

    let profile_identity = ProfileIdentity {
        profile_ref: "profile.default".to_string(),
        profile_label: "Default".to_string(),
        profile_mode: ProfileModeClass::Standard,
        deployment_profile: DeploymentProfileClass::IndividualLocal,
        identity_mode: IdentityMode::AccountFreeLocal,
    };

    let mut field_visibility = Vec::new();
    field_visibility.push(FieldVisibilityRule {
        field_path: FieldPath::WorkspaceIdentityDisplayLabel,
        visibility: VisibilityClass::PrimaryVisible,
        visible_in_surfaces: vec![
            SurfaceKind::TitleContextBar,
            SurfaceKind::NativeWindowTitle,
            SurfaceKind::WorkspaceStatusItem,
            SurfaceKind::SupportExport,
        ],
        overflow_target_surface: None,
        reason: "Workspace label anchors the shell identity tuple.".to_string(),
    });
    if lifecycle_state != WorkspaceLifecycleState::WorkspaceReady {
        field_visibility.push(FieldVisibilityRule {
            field_path: FieldPath::WorkspaceIdentityLifecycleState,
            visibility: VisibilityClass::PrimaryVisible,
            visible_in_surfaces: vec![
                SurfaceKind::TitleContextBar,
                SurfaceKind::NativeWindowTitle,
                SurfaceKind::WorkspaceStatusItem,
                SurfaceKind::SupportExport,
            ],
            overflow_target_surface: Some(SurfaceKind::WorkspaceStatusItem),
            reason: "Non-ready lifecycle state must remain visible.".to_string(),
        });
    }
    if repo_identity.branch_label.is_some() {
        field_visibility.push(FieldVisibilityRule {
            field_path: FieldPath::RepoIdentityBranchLabel,
            visibility: VisibilityClass::CondensedVisible,
            visible_in_surfaces: vec![
                SurfaceKind::TitleContextBar,
                SurfaceKind::WorkspaceStatusItem,
                SurfaceKind::SupportExport,
            ],
            overflow_target_surface: Some(SurfaceKind::WorkspaceStatusItem),
            reason: "Branch identity is visible when repository metadata is authoritative."
                .to_string(),
        });
    }
    if trust_state != TrustState::Trusted {
        field_visibility.push(FieldVisibilityRule {
            field_path: FieldPath::TrustIdentityTrustState,
            visibility: VisibilityClass::CondensedVisible,
            visible_in_surfaces: vec![
                SurfaceKind::TitleContextBar,
                SurfaceKind::WorkspaceStatusItem,
                SurfaceKind::SupportExport,
            ],
            overflow_target_surface: Some(SurfaceKind::WorkspaceStatusItem),
            reason: "Non-trusted postures require a visible trust cue.".to_string(),
        });
    }
    if !degraded_tokens.is_empty() {
        field_visibility.push(FieldVisibilityRule {
            field_path: FieldPath::DegradedOrRecoveryStateDegradedTokens,
            visibility: VisibilityClass::PrimaryVisible,
            visible_in_surfaces: vec![
                SurfaceKind::TitleContextBar,
                SurfaceKind::NativeWindowTitle,
                SurfaceKind::WorkspaceStatusItem,
                SurfaceKind::SupportExport,
            ],
            overflow_target_surface: Some(SurfaceKind::WorkspaceStatusItem),
            reason: "Degraded tokens convey readiness and recovery posture.".to_string(),
        });
    }

    let title_label = title_context_bar_render_label(
        &workspace_label,
        lifecycle_state,
        &repo_identity,
        trust_state,
        &degraded_tokens,
    );
    let native_title_label = native_window_title_render_label(
        &workspace_label,
        trust_state,
        lifecycle_state,
        &degraded_tokens,
    );
    let status_item_label =
        status_item_render_label(lifecycle_state, trust_state, &degraded_tokens);
    let support_export_label = format!("Support summary for {workspace_label}");

    let title_projection_fields =
        title_projection_fields(&repo_identity, trust_state, &degraded_tokens);
    let status_projection_fields = status_projection_fields(
        &repo_identity,
        trust_state,
        &degraded_tokens,
        last_failure_summary.as_deref().is_some(),
    );
    let native_projection_fields =
        native_projection_fields(trust_state, lifecycle_state, &degraded_tokens);

    let surface_projections = vec![
        SurfaceProjection {
            surface: SurfaceKind::TitleContextBar,
            projection_ref: format!("projection.title_context.{workspace_ref}"),
            field_paths: title_projection_fields,
            render_label: title_label,
            opens_detail_surface_ref: Some(format!(
                "surface.workspace_status.drawer.{workspace_ref}"
            )),
            redaction_class: RedactionClass::MetadataSafeDefault,
            forbids_surface_local_fields: true,
        },
        SurfaceProjection {
            surface: SurfaceKind::NativeWindowTitle,
            projection_ref: format!("projection.native_title.{workspace_ref}"),
            field_paths: native_projection_fields,
            render_label: native_title_label,
            opens_detail_surface_ref: None,
            redaction_class: RedactionClass::MetadataSafeDefault,
            forbids_surface_local_fields: true,
        },
        SurfaceProjection {
            surface: SurfaceKind::WorkspaceStatusItem,
            projection_ref: format!("projection.status_item.{workspace_ref}"),
            field_paths: status_projection_fields.clone(),
            render_label: status_item_label,
            opens_detail_surface_ref: Some(format!(
                "surface.workspace_status.drawer.{workspace_ref}"
            )),
            redaction_class: RedactionClass::MetadataSafeDefault,
            forbids_surface_local_fields: true,
        },
        SurfaceProjection {
            surface: SurfaceKind::SupportExport,
            projection_ref: format!("projection.support_export.{workspace_ref}"),
            field_paths: status_projection_fields,
            render_label: support_export_label,
            opens_detail_surface_ref: None,
            redaction_class: RedactionClass::MetadataSafeDefault,
            forbids_surface_local_fields: true,
        },
    ];

    TitleContextBarStateRecord {
        schema: None,
        fixture: None,
        record_kind: TitleContextBarStateRecordKind::TitleContextBarStateRecord,
        title_context_bar_state_schema_version: 1,
        state_id: format!("title_context_state.{workspace_ref}"),
        workspace_identity: WorkspaceIdentity {
            workspace_ref: workspace_ref.clone(),
            display_label: workspace_label,
            workspace_kind,
            lifecycle_state,
            root_summary: RootSummary {
                root_count: if workspace_kind == WorkspaceKind::EmptyShell {
                    0
                } else {
                    1
                },
                ready_root_count,
                primary_root_label: if workspace_kind == WorkspaceKind::EmptyShell {
                    None
                } else {
                    Some("root".to_string())
                },
                root_readiness,
                root_detail_refs: Vec::new(),
            },
        },
        repo_identity,
        trust_identity: TrustIdentity {
            trust_state,
            trust_source,
            trust_review_action_ref: Some("action.workspace.trust_details".to_string()),
            last_changed_at: now.clone(),
        },
        host_identity,
        profile_identity,
        route_state: RouteState {
            route_kind,
            route_ref: None,
            route_label,
            authority_delta: AuthorityDeltaClass::None,
            route_freshness,
            active_surface_ref: "surface.shell.primary".to_string(),
        },
        degraded_or_recovery_state: DegradedOrRecoveryState {
            degraded_tokens,
            recovery_mode,
            last_failure_ref: None,
            last_failure_summary,
            primary_recovery_action_ref: None,
            support_status_ref: format!("support.workspace_status.{workspace_ref}"),
        },
        field_visibility: if field_visibility.is_empty() {
            vec![FieldVisibilityRule {
                field_path: FieldPath::WorkspaceIdentityDisplayLabel,
                visibility: VisibilityClass::PrimaryVisible,
                visible_in_surfaces: vec![
                    SurfaceKind::TitleContextBar,
                    SurfaceKind::NativeWindowTitle,
                    SurfaceKind::WorkspaceStatusItem,
                    SurfaceKind::SupportExport,
                ],
                overflow_target_surface: None,
                reason: "Workspace label anchors the shell identity tuple.".to_string(),
            }]
        } else {
            field_visibility
        },
        surface_projections,
        updated_at: now,
        narrative_refs: vec![
            "docs/ux/title_context_bar_contract.md".to_string(),
            "schemas/ux/title_context_bar_state.schema.json".to_string(),
        ],
    }
}

fn sanitize_or_fallback_opaque_id(value: &str, fallback: &str) -> String {
    let sanitized = sanitize_opaque_id(value);
    if sanitized.is_empty() {
        fallback.to_string()
    } else {
        sanitized
    }
}

fn sanitize_opaque_id(value: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_sep = false;
            continue;
        }
        if last_sep {
            continue;
        }
        out.push('-');
        last_sep = true;
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

fn workspace_lifecycle_state_token(
    state: RuntimeWorkspaceLifecycleState,
) -> WorkspaceLifecycleState {
    match state {
        RuntimeWorkspaceLifecycleState::Discovered => WorkspaceLifecycleState::WorkspaceDiscovered,
        RuntimeWorkspaceLifecycleState::TrustEvaluating => {
            WorkspaceLifecycleState::WorkspaceTrustEvaluating
        }
        RuntimeWorkspaceLifecycleState::Opening => WorkspaceLifecycleState::WorkspaceOpening,
        RuntimeWorkspaceLifecycleState::PartiallyReady => {
            WorkspaceLifecycleState::WorkspacePartiallyReady
        }
        RuntimeWorkspaceLifecycleState::Ready => WorkspaceLifecycleState::WorkspaceReady,
        RuntimeWorkspaceLifecycleState::Degraded => WorkspaceLifecycleState::WorkspaceDegraded,
        RuntimeWorkspaceLifecycleState::Closing => WorkspaceLifecycleState::WorkspaceClosing,
        RuntimeWorkspaceLifecycleState::Closed => WorkspaceLifecycleState::WorkspaceClosed,
    }
}

fn trust_tuple(
    lifecycle: Option<&WorkspaceLifecycleMachine>,
    trust_token: &str,
) -> (TrustState, TrustSourceClass) {
    let trust_state = lifecycle
        .map(|machine| machine.trust_state())
        .unwrap_or_else(|| match trust_token {
            "trusted" => WorkspaceTrustState::Trusted,
            "restricted" => WorkspaceTrustState::Restricted,
            _ => WorkspaceTrustState::PendingEvaluation,
        });

    match trust_state {
        WorkspaceTrustState::Trusted => {
            (TrustState::Trusted, TrustSourceClass::RememberedUserGrant)
        }
        WorkspaceTrustState::Restricted => (TrustState::Restricted, TrustSourceClass::UserGrant),
        WorkspaceTrustState::PendingEvaluation => (
            TrustState::UntrustedUnknown,
            TrustSourceClass::FirstOpenDefault,
        ),
    }
}

fn degraded_tokens_for(lifecycle: Option<&WorkspaceLifecycleMachine>) -> Vec<DegradedStateToken> {
    let Some(machine) = lifecycle else {
        return Vec::new();
    };

    let mut tokens: Vec<DegradedStateToken> = Vec::new();
    match machine.state() {
        RuntimeWorkspaceLifecycleState::Opening => {
            tokens.push(DegradedStateToken::Warming);
        }
        RuntimeWorkspaceLifecycleState::PartiallyReady => {
            tokens.push(DegradedStateToken::Partial);
        }
        RuntimeWorkspaceLifecycleState::Degraded => {
            tokens.push(DegradedStateToken::Limited);
        }
        _ => {}
    }

    if let Some(health) = machine.watcher_health() {
        match health {
            WatcherHealth::Warming => push_unique(&mut tokens, DegradedStateToken::Warming),
            WatcherHealth::Healthy => {}
            WatcherHealth::Degraded => push_unique(&mut tokens, DegradedStateToken::Stale),
            WatcherHealth::FallbackPolling => push_unique(&mut tokens, DegradedStateToken::Cached),
            WatcherHealth::Unavailable => push_unique(&mut tokens, DegradedStateToken::Offline),
        }
    }

    tokens
}

fn recovery_mode_for(
    trust_state: TrustState,
    lifecycle_state: WorkspaceLifecycleState,
    degraded_tokens: &[DegradedStateToken],
) -> RecoveryModeClass {
    if trust_state == TrustState::Restricted {
        return RecoveryModeClass::RestrictedMode;
    }
    if matches!(
        lifecycle_state,
        WorkspaceLifecycleState::WorkspaceOpening
            | WorkspaceLifecycleState::WorkspacePartiallyReady
    ) {
        return RecoveryModeClass::PartialOpen;
    }
    if degraded_tokens.contains(&DegradedStateToken::Offline) {
        return RecoveryModeClass::Reconnecting;
    }
    RecoveryModeClass::None
}

fn last_failure_summary_for(
    lifecycle: Option<&WorkspaceLifecycleMachine>,
    workspace_root: Option<&Path>,
    degraded_tokens: &[DegradedStateToken],
) -> Option<String> {
    let Some(machine) = lifecycle else {
        return None;
    };
    let state = machine.state();
    if state == RuntimeWorkspaceLifecycleState::PartiallyReady {
        return Some(
            "Workspace is interactive while background readiness gates complete.".to_string(),
        );
    }
    if degraded_tokens.contains(&DegradedStateToken::Offline) {
        return Some("Workspace watcher is unavailable; freshness and external-change signals may be degraded.".to_string());
    }
    if state == RuntimeWorkspaceLifecycleState::Degraded {
        return Some(
            "Workspace is degraded; some capabilities may be limited until recovery.".to_string(),
        );
    }
    if workspace_root.is_some_and(|root| !root.exists()) {
        return Some(
            "Workspace root is missing; recovery action is required before edits can apply safely."
                .to_string(),
        );
    }
    None
}

fn route_kind_for(lifecycle_state: WorkspaceLifecycleState) -> RouteKind {
    match lifecycle_state {
        WorkspaceLifecycleState::WorkspaceOpening
        | WorkspaceLifecycleState::WorkspacePartiallyReady => RouteKind::EntryRoute,
        WorkspaceLifecycleState::WorkspaceClosed => RouteKind::EntryRoute,
        _ => RouteKind::OrdinaryEditing,
    }
}

fn route_freshness_for(
    lifecycle_state: WorkspaceLifecycleState,
    degraded_tokens: &[DegradedStateToken],
) -> RouteFreshnessClass {
    if degraded_tokens.contains(&DegradedStateToken::Offline) {
        return RouteFreshnessClass::Offline;
    }
    match lifecycle_state {
        WorkspaceLifecycleState::WorkspaceOpening => RouteFreshnessClass::Warming,
        WorkspaceLifecycleState::WorkspacePartiallyReady => RouteFreshnessClass::Partial,
        WorkspaceLifecycleState::WorkspaceReady => RouteFreshnessClass::Current,
        _ => RouteFreshnessClass::Current,
    }
}

fn route_label_for(route_kind: RouteKind, lifecycle_state: WorkspaceLifecycleState) -> String {
    match route_kind {
        RouteKind::EntryRoute => match lifecycle_state {
            WorkspaceLifecycleState::WorkspaceClosing
            | WorkspaceLifecycleState::WorkspaceClosed => "Start Center".to_string(),
            WorkspaceLifecycleState::WorkspaceOpening => "Opening workspace".to_string(),
            WorkspaceLifecycleState::WorkspacePartiallyReady => "Opening workspace".to_string(),
            _ => "Entry".to_string(),
        },
        _ => "Editing".to_string(),
    }
}

fn title_projection_fields(
    repo: &RepoIdentity,
    trust_state: TrustState,
    degraded_tokens: &[DegradedStateToken],
) -> Vec<FieldPath> {
    let mut fields = vec![FieldPath::WorkspaceIdentityDisplayLabel];
    fields.push(FieldPath::WorkspaceIdentityLifecycleState);
    if repo.branch_label.is_some() {
        fields.push(FieldPath::RepoIdentityBranchLabel);
    }
    if trust_state != TrustState::Trusted {
        fields.push(FieldPath::TrustIdentityTrustState);
    }
    if !degraded_tokens.is_empty() {
        fields.push(FieldPath::DegradedOrRecoveryStateDegradedTokens);
    }
    fields
}

fn native_projection_fields(
    trust_state: TrustState,
    lifecycle_state: WorkspaceLifecycleState,
    degraded_tokens: &[DegradedStateToken],
) -> Vec<FieldPath> {
    let mut fields = vec![FieldPath::WorkspaceIdentityDisplayLabel];
    if lifecycle_state != WorkspaceLifecycleState::WorkspaceReady {
        fields.push(FieldPath::WorkspaceIdentityLifecycleState);
    }
    if trust_state != TrustState::Trusted {
        fields.push(FieldPath::TrustIdentityTrustState);
    }
    if !degraded_tokens.is_empty() {
        fields.push(FieldPath::DegradedOrRecoveryStateDegradedTokens);
    }
    fields
}

fn status_projection_fields(
    repo: &RepoIdentity,
    trust_state: TrustState,
    degraded_tokens: &[DegradedStateToken],
    include_failure_summary: bool,
) -> Vec<FieldPath> {
    let mut fields = vec![
        FieldPath::WorkspaceIdentityDisplayLabel,
        FieldPath::WorkspaceIdentityLifecycleState,
        FieldPath::RepoIdentityRepoStateClass,
        FieldPath::HostIdentityHostClass,
        FieldPath::RouteStateRouteLabel,
    ];
    if repo.branch_label.is_some() {
        fields.push(FieldPath::RepoIdentityBranchLabel);
    }
    if trust_state != TrustState::Trusted {
        fields.push(FieldPath::TrustIdentityTrustState);
        fields.push(FieldPath::TrustIdentityTrustSource);
    }
    if !degraded_tokens.is_empty() {
        fields.push(FieldPath::DegradedOrRecoveryStateDegradedTokens);
        fields.push(FieldPath::DegradedOrRecoveryStateRecoveryMode);
    }
    if include_failure_summary {
        fields.push(FieldPath::DegradedOrRecoveryStateLastFailureSummary);
    }
    fields
}

fn title_context_bar_render_label(
    workspace_label: &str,
    lifecycle_state: WorkspaceLifecycleState,
    repo_identity: &RepoIdentity,
    trust_state: TrustState,
    degraded_tokens: &[DegradedStateToken],
) -> String {
    let mut parts: Vec<String> = vec![workspace_label.to_string()];
    if lifecycle_state != WorkspaceLifecycleState::WorkspaceReady {
        parts.push(lifecycle_state.short_label().to_string());
    }
    if let Some(branch) = repo_identity.branch_label.as_deref() {
        parts.push(branch.to_string());
    }
    if trust_state != TrustState::Trusted {
        parts.push(trust_state.short_label().to_string());
    }
    if !degraded_tokens.is_empty() {
        parts.push(
            degraded_tokens
                .iter()
                .map(|t| t.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    parts.join(" · ")
}

fn native_window_title_render_label(
    workspace_label: &str,
    trust_state: TrustState,
    lifecycle_state: WorkspaceLifecycleState,
    degraded_tokens: &[DegradedStateToken],
) -> String {
    let mut parts: Vec<String> = vec![workspace_label.to_string()];
    if lifecycle_state != WorkspaceLifecycleState::WorkspaceReady {
        parts.push(lifecycle_state.short_label().to_string());
    }
    if trust_state != TrustState::Trusted {
        parts.push(trust_state.short_label().to_string());
    }
    if !degraded_tokens.is_empty() {
        parts.push(
            degraded_tokens
                .iter()
                .map(|t| t.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    parts.join(" · ")
}

fn status_item_render_label(
    lifecycle_state: WorkspaceLifecycleState,
    trust_state: TrustState,
    degraded_tokens: &[DegradedStateToken],
) -> String {
    if lifecycle_state == WorkspaceLifecycleState::WorkspaceReady
        && trust_state == TrustState::Trusted
        && degraded_tokens.is_empty()
    {
        return "Workspace ready".to_string();
    }
    let mut parts = vec![lifecycle_state.short_label().to_string()];
    if trust_state != TrustState::Trusted {
        parts.push(trust_state.short_label().to_string());
    }
    if !degraded_tokens.is_empty() {
        parts.push(
            degraded_tokens
                .iter()
                .map(|t| t.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    format!("Workspace {}", parts.join(" · "))
}

fn push_unique(tokens: &mut Vec<DegradedStateToken>, token: DegradedStateToken) {
    if tokens.contains(&token) {
        return;
    }
    tokens.push(token);
}

fn classify_workspace_kind(root: Option<&Path>) -> (WorkspaceKind, RootReadinessClass, i64) {
    let Some(root) = root else {
        return (
            WorkspaceKind::EmptyShell,
            RootReadinessClass::NotApplicable,
            0,
        );
    };

    if !root.exists() {
        return (
            WorkspaceKind::LocalFolder,
            RootReadinessClass::RootMissing,
            0,
        );
    }

    let git_root = find_git_root(root);
    if git_root.is_some_and(|git_root| git_root == root) {
        return (
            WorkspaceKind::LocalRepoRoot,
            RootReadinessClass::SingleRootReady,
            1,
        );
    }
    (
        WorkspaceKind::LocalFolder,
        RootReadinessClass::SingleRootReady,
        1,
    )
}

#[derive(Debug, Clone)]
struct GitIdentity {
    repo_label: String,
    branch: Option<String>,
    head_short_sha: Option<String>,
    detached: bool,
}

fn discover_git_identity(root: &Path) -> Option<GitIdentity> {
    let git_root = find_git_root(root)?;
    let git_dir = resolve_git_dir(&git_root)?;
    let head_path = git_dir.join("HEAD");
    let head = std::fs::read_to_string(head_path).ok()?;
    let head = head.trim();

    let repo_label = git_root
        .file_name()
        .map(|os| os.to_string_lossy().into_owned())
        .unwrap_or_else(|| "repository".to_string());

    if let Some(ref_path) = head.strip_prefix("ref:") {
        let ref_path = ref_path.trim();
        let branch = ref_path.rsplit('/').next().map(|s| s.to_string());
        let sha =
            read_ref_or_packed(&git_dir, ref_path).or_else(|| read_packed_ref(&git_dir, ref_path));
        let head_short_sha = sha.as_deref().map(|s| short_sha(s));
        return Some(GitIdentity {
            repo_label,
            branch,
            head_short_sha,
            detached: false,
        });
    }

    let head_short_sha = (!head.is_empty()).then_some(short_sha(head));
    Some(GitIdentity {
        repo_label,
        branch: None,
        head_short_sha,
        detached: true,
    })
}

fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut cursor = start;
    loop {
        if cursor.join(".git").exists() {
            return Some(cursor.to_path_buf());
        }
        cursor = cursor.parent()?;
    }
}

fn resolve_git_dir(root: &Path) -> Option<PathBuf> {
    let dot_git = root.join(".git");
    if dot_git.is_dir() {
        return Some(dot_git);
    }
    let payload = std::fs::read_to_string(&dot_git).ok()?;
    let payload = payload.trim();
    let rest = payload.strip_prefix("gitdir:")?.trim();
    let path = PathBuf::from(rest);
    Some(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

fn read_ref_or_packed(git_dir: &Path, ref_path: &str) -> Option<String> {
    let path = git_dir.join(ref_path);
    let payload = std::fs::read_to_string(path).ok()?;
    Some(payload.trim().to_string())
}

fn read_packed_ref(git_dir: &Path, ref_path: &str) -> Option<String> {
    let packed = git_dir.join("packed-refs");
    let payload = std::fs::read_to_string(packed).ok()?;
    for line in payload.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('^') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let sha = parts.next()?;
        let name = parts.next()?;
        if name == ref_path {
            return Some(sha.to_string());
        }
    }
    None
}

fn short_sha(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_hexdigit())
        .take(7)
        .collect()
}

fn repo_identity_from_git(workspace_ref: &str, git: Option<&GitIdentity>) -> RepoIdentity {
    let Some(git) = git else {
        return RepoIdentity::not_applicable();
    };

    if git.detached {
        return RepoIdentity {
            repo_state_class: RepoStateClass::DetachedHead,
            repo_ref: Some(format!("repo.local.{workspace_ref}")),
            repo_label: Some(git.repo_label.clone()),
            branch_label: None,
            branch_ref: None,
            revision_ref: git
                .head_short_sha
                .as_deref()
                .map(|sha| format!("git.rev.{sha}")),
            repo_detail_refs: Vec::new(),
        };
    }

    let branch_label = git.branch.clone();
    let branch_ref = branch_label
        .as_deref()
        .map(|branch| format!("git.branch.{}", sanitize_opaque_id(branch)));
    RepoIdentity {
        repo_state_class: RepoStateClass::AttachedSingleRepo,
        repo_ref: Some(format!("repo.local.{workspace_ref}")),
        repo_label: Some(git.repo_label.clone()),
        branch_label,
        branch_ref,
        revision_ref: git
            .head_short_sha
            .as_deref()
            .map(|sha| format!("git.rev.{sha}")),
        repo_detail_refs: Vec::new(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Identifies the serialized record kind for [`TitleContextBarStateRecord`].
pub enum TitleContextBarStateRecordKind {
    /// `title_context_bar_state_record`
    TitleContextBarStateRecord,
}

/// Canonical identity tuple for title/context bar and downstream projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TitleContextBarStateRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(
        rename = "__fixture__",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture: Option<serde_json::Value>,
    pub record_kind: TitleContextBarStateRecordKind,
    pub title_context_bar_state_schema_version: u32,
    pub state_id: String,
    pub workspace_identity: WorkspaceIdentity,
    pub repo_identity: RepoIdentity,
    pub trust_identity: TrustIdentity,
    pub host_identity: HostIdentity,
    pub profile_identity: ProfileIdentity,
    pub route_state: RouteState,
    pub degraded_or_recovery_state: DegradedOrRecoveryState,
    pub field_visibility: Vec<FieldVisibilityRule>,
    pub surface_projections: Vec<SurfaceProjection>,
    pub updated_at: String,
    pub narrative_refs: Vec<String>,
}

impl TitleContextBarStateRecord {
    /// Returns the projection label for the requested surface.
    pub fn projection_label(&self, surface: SurfaceKind) -> Option<&str> {
        self.surface_projections
            .iter()
            .find(|proj| proj.surface == surface)
            .map(|proj| proj.render_label.as_str())
    }

    /// Returns the render label for the native window title projection.
    pub fn native_window_title_label(&self) -> Option<&str> {
        self.projection_label(SurfaceKind::NativeWindowTitle)
    }
}

/// Workspace identity group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceIdentity {
    pub workspace_ref: String,
    pub display_label: String,
    pub workspace_kind: WorkspaceKind,
    pub lifecycle_state: WorkspaceLifecycleState,
    pub root_summary: RootSummary,
}

/// Root readiness summary for the active workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootSummary {
    pub root_count: i64,
    pub ready_root_count: i64,
    pub primary_root_label: Option<String>,
    pub root_readiness: RootReadinessClass,
    pub root_detail_refs: Vec<String>,
}

/// Repository identity group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoIdentity {
    pub repo_state_class: RepoStateClass,
    pub repo_ref: Option<String>,
    pub repo_label: Option<String>,
    pub branch_label: Option<String>,
    pub branch_ref: Option<String>,
    pub revision_ref: Option<String>,
    pub repo_detail_refs: Vec<String>,
}

impl RepoIdentity {
    fn not_applicable() -> Self {
        Self {
            repo_state_class: RepoStateClass::NotApplicable,
            repo_ref: None,
            repo_label: None,
            branch_label: None,
            branch_ref: None,
            revision_ref: None,
            repo_detail_refs: Vec::new(),
        }
    }
}

/// Trust identity group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustIdentity {
    pub trust_state: TrustState,
    pub trust_source: TrustSourceClass,
    pub trust_review_action_ref: Option<String>,
    pub last_changed_at: String,
}

/// Host identity group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostIdentity {
    pub host_class: HostClass,
    pub host_state: HostStateClass,
    pub target_ref: Option<String>,
    pub target_label: Option<String>,
    pub boundary_note: String,
}

/// Profile identity group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileIdentity {
    pub profile_ref: String,
    pub profile_label: String,
    pub profile_mode: ProfileModeClass,
    pub deployment_profile: DeploymentProfileClass,
    pub identity_mode: IdentityMode,
}

/// Route identity group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteState {
    pub route_kind: RouteKind,
    pub route_ref: Option<String>,
    pub route_label: String,
    pub authority_delta: AuthorityDeltaClass,
    pub route_freshness: RouteFreshnessClass,
    pub active_surface_ref: String,
}

/// Degraded or recovery state group for the canonical tuple.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedOrRecoveryState {
    pub degraded_tokens: Vec<DegradedStateToken>,
    pub recovery_mode: RecoveryModeClass,
    pub last_failure_ref: Option<String>,
    pub last_failure_summary: Option<String>,
    pub primary_recovery_action_ref: Option<String>,
    pub support_status_ref: String,
}

/// Field visibility rule for one canonical field path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldVisibilityRule {
    pub field_path: FieldPath,
    pub visibility: VisibilityClass,
    pub visible_in_surfaces: Vec<SurfaceKind>,
    pub overflow_target_surface: Option<SurfaceKind>,
    pub reason: String,
}

/// Surface projection binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProjection {
    pub surface: SurfaceKind,
    pub projection_ref: String,
    pub field_paths: Vec<FieldPath>,
    pub render_label: String,
    pub opens_detail_surface_ref: Option<String>,
    pub redaction_class: RedactionClass,
    pub forbids_surface_local_fields: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes what sort of workspace boundary the shell is currently representing.
pub enum WorkspaceKind {
    /// No workspace is active; the shell is showing entry surfaces.
    EmptyShell,
    /// A local folder root is open without asserting repository metadata.
    LocalFolder,
    /// A local repository root is open with authoritative VCS metadata.
    LocalRepoRoot,
    /// A workspace with one or more attached roots (multi-root or layered).
    WorkspaceWithRoots,
    /// A curated slice of a larger workspace (workset view).
    WorksetSlice,
    /// A workspace executing against a remote host boundary.
    RemoteWorkspace,
    /// A managed cloud workspace boundary where hosting is provider-owned.
    ManagedCloudWorkspace,
    /// A restored checkpoint or snapshot boundary.
    RestoredCheckpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Names the lifecycle state of the active workspace for identity and status projections.
pub enum WorkspaceLifecycleState {
    #[serde(rename = "workspace.discovered")]
    /// Workspace discovery has been recorded but opening has not started.
    WorkspaceDiscovered,
    #[serde(rename = "workspace.trust_evaluating")]
    /// Trust evaluation is running and may gate later readiness.
    WorkspaceTrustEvaluating,
    #[serde(rename = "workspace.opening")]
    /// Workspace open has started and the shell should treat it as warming.
    WorkspaceOpening,
    #[serde(rename = "workspace.partially_ready")]
    /// Some readiness gates are satisfied, but the workspace is not fully ready.
    WorkspacePartiallyReady,
    #[serde(rename = "workspace.ready")]
    /// Workspace readiness gates are satisfied and interactive editing is available.
    WorkspaceReady,
    #[serde(rename = "workspace.degraded")]
    /// Workspace is interactive but degraded (missing signals, errors, or partial data).
    WorkspaceDegraded,
    #[serde(rename = "workspace.read_only_degraded")]
    /// Workspace is degraded and effectively read-only.
    WorkspaceReadOnlyDegraded,
    #[serde(rename = "workspace.recovering")]
    /// Workspace is in recovery flow and may be unstable or partially restored.
    WorkspaceRecovering,
    #[serde(rename = "workspace.closing")]
    /// Workspace close has started.
    WorkspaceClosing,
    #[serde(rename = "workspace.closed")]
    /// Workspace is closed and no longer active in this shell window.
    WorkspaceClosed,
}

impl WorkspaceLifecycleState {
    fn short_label(self) -> &'static str {
        match self {
            Self::WorkspaceDiscovered => "Discovered",
            Self::WorkspaceTrustEvaluating => "Trust evaluating",
            Self::WorkspaceOpening => "Opening",
            Self::WorkspacePartiallyReady => "Partially ready",
            Self::WorkspaceReady => "Ready",
            Self::WorkspaceDegraded => "Degraded",
            Self::WorkspaceReadOnlyDegraded => "Read-only degraded",
            Self::WorkspaceRecovering => "Recovering",
            Self::WorkspaceClosing => "Closing",
            Self::WorkspaceClosed => "Closed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes readiness of the active root set for status and degraded-state projection.
pub enum RootReadinessClass {
    /// Exactly one root is attached and ready.
    SingleRootReady,
    /// All roots are attached and ready.
    MultiRootAllReady,
    /// Some roots are attached, others are still warming or missing.
    MultiRootPartial,
    /// Roots exist but trust state differs across them.
    MultiRootMixedTrust,
    /// The expected primary root is missing on disk or cannot be resolved.
    RootMissing,
    /// Root is present but hidden or gated by policy.
    RootPolicyHidden,
    /// Root readiness does not apply (for example, empty shell).
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes how repository metadata is attached to the current workspace boundary.
pub enum RepoStateClass {
    /// Repository identity does not apply to the current workspace.
    NotApplicable,
    /// Exactly one authoritative repository is attached.
    AttachedSingleRepo,
    /// Repository metadata exists but the current HEAD is detached.
    DetachedHead,
    /// Repository metadata is expected but currently missing/unavailable.
    MetadataMissing,
    /// Detached metadata is present but not authoritative enough to label a branch.
    DetachedMetadata,
    /// Multiple repositories are attached and cannot be reduced to one label.
    MixedMultiRepo,
    /// Provider overlay or index is stale relative to local filesystem truth.
    ProviderOverlayStale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Names the current workspace trust posture for identity and safety cues.
pub enum TrustState {
    /// Trust is unknown or untrusted, but the reason is not yet classified.
    UntrustedUnknown,
    /// Workspace is restricted and must avoid executing or mutating unsafe operations.
    Restricted,
    /// Workspace is trusted for ordinary editing and execution.
    Trusted,
    /// Workspace trust is trusted but time-bounded and may expire.
    TrustedTimeBounded,
    /// Workspace trust is trusted but degraded by policy or missing context.
    TrustedPolicyDegraded,
    /// Workspace is restricted as a recovery fallback posture.
    RestrictedRecoveryFallback,
    /// Workspace is restricted because extensions are quarantined.
    RestrictedExtensionQuarantine,
    /// Trust was revoked after previously being granted.
    TrustRevoked,
    /// Trust cannot be resolved because identity is gated or missing.
    TrustUnavailableIdentityGate,
}

impl TrustState {
    fn short_label(self) -> &'static str {
        match self {
            Self::Restricted => "Restricted",
            Self::Trusted => "Trusted",
            Self::UntrustedUnknown => "Untrusted",
            Self::TrustedTimeBounded => "Trusted (time bounded)",
            Self::TrustedPolicyDegraded => "Trusted (policy degraded)",
            Self::RestrictedRecoveryFallback => "Restricted (recovery)",
            Self::RestrictedExtensionQuarantine => "Restricted (quarantine)",
            Self::TrustRevoked => "Trust revoked",
            Self::TrustUnavailableIdentityGate => "Trust gated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Classifies why the current [`TrustState`] was selected.
pub enum TrustSourceClass {
    /// First open uses the default trust posture for the target kind.
    FirstOpenDefault,
    /// User granted trust in the current session.
    UserGrant,
    /// User grant was remembered and replayed.
    RememberedUserGrant,
    /// Admin policy pre-bound trust to the workspace target.
    AdminPolicyPrebinding,
    /// Policy narrowed the previously granted trust posture.
    PolicyNarrowed,
    /// Recovery ladder forced a trust posture to unblock access.
    RecoveryLadder,
    /// Extension quarantine forced a restricted posture.
    ExtensionQuarantine,
    /// Trust resolution was gated by missing identity.
    IdentityGate,
    /// User or admin revoked trust.
    UserOrAdminRevoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes where the workspace is executing (local, remote host, container, etc).
pub enum HostClass {
    /// Local host execution boundary.
    Local,
    /// Remote host execution boundary (SSH, tunnel, etc).
    RemoteHost,
    /// Container or devcontainer boundary.
    ContainerDevcontainer,
    /// Managed workspace boundary with hosted execution.
    ManagedWorkspace,
    /// Browser runtime bridge boundary (browser surface hosting).
    BrowserRuntimeBridge,
    /// Service-plane execution boundary (remote services).
    ServicePlane,
    /// Mixed local editing plus remote execution boundary.
    MixedLocalPlusRemote,
    /// Host details are missing or unknown.
    UnknownMissingDetails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Captures the health and readiness state of the active host boundary.
pub enum HostStateClass {
    /// Host boundary is ready.
    Ready,
    /// Host details are probing or still being discovered.
    Probing,
    /// Host is warming or syncing state before becoming fully usable.
    SyncWarming,
    /// Host is reconnecting after a disconnect.
    Reconnecting,
    /// Host is degraded into read-only mode.
    ReadOnlyDegraded,
    /// Host is offline.
    Offline,
    /// Host boundary is blocked by policy.
    PolicyBlocked,
    /// Host details are missing.
    MissingDetails,
    /// Host state is mixed across multiple targets.
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes the deployment envelope that can influence policy and defaults.
pub enum DeploymentProfileClass {
    /// Individual local development without managed services.
    IndividualLocal,
    /// Self-hosted deployment.
    SelfHosted,
    /// Enterprise online deployment.
    EnterpriseOnline,
    /// Air-gapped deployment.
    AirGapped,
    /// Managed cloud deployment.
    ManagedCloud,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes what identity authority is available for the current session.
pub enum IdentityMode {
    /// No account authority; local-only identity.
    AccountFreeLocal,
    /// Self-hosted organization identity.
    SelfHostedOrg,
    /// Managed workspace identity tied to a hosting provider.
    ManagedWorkspace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes which profile mode is currently affecting shell behavior.
pub enum ProfileModeClass {
    /// Standard profile behavior.
    Standard,
    /// Temporary session profile with non-persistent overrides.
    TemporarySession,
    /// Safe-mode profile with reduced functionality.
    SafeMode,
    /// Imported profile loaded from an external source.
    ImportedProfile,
    /// Profile managed by policy.
    ManagedPolicyProfile,
    /// Support or recovery profile used to stabilize the session.
    SupportRecovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Names the current shell route so identity surfaces can remain truthful.
pub enum RouteKind {
    /// Ordinary editing route.
    OrdinaryEditing,
    /// Entry route (start center, open/clone/import flows).
    EntryRoute,
    /// Restore route (rehydrating a previous session).
    RestoreRoute,
    /// Review route (diff/review surfaces).
    ReviewRoute,
    /// Run/debug route.
    RunDebugRoute,
    /// Remote attach route.
    RemoteAttachRoute,
    /// Deep-link review route.
    DeepLinkReview,
    /// Support/export route.
    SupportExport,
    /// Recovery route.
    RecoveryRoute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Classifies the authority delta implied by the current route/target binding.
pub enum AuthorityDeltaClass {
    /// No authority delta is currently active.
    None,
    /// Authority changed due to a trust-boundary crossing.
    TrustBoundaryCrossing,
    /// Authority changed due to policy binding.
    PolicyBoundaryCrossing,
    /// Authority rebound to a remote execution target.
    RemoteAuthorityRebind,
    /// Authority widened due to authentication scope changes.
    AuthScopeWidening,
    /// Authority change requires review before mutation or execution.
    MutationOrExecutionPendingReview,
    /// Authority delta cannot be classified and must be reviewed.
    UnknownRequiresReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes whether route metadata is current, warming, partial, stale, or offline.
pub enum RouteFreshnessClass {
    /// Route and attached metadata are current.
    Current,
    /// Route metadata is warming.
    Warming,
    /// Route metadata is partial.
    Partial,
    /// Route metadata is stale.
    Stale,
    /// Route metadata is offline.
    Offline,
    /// Freshness cannot be classified and must be reviewed.
    UnknownRequiresReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Degraded-state tokens projected into chrome and exported evidence.
pub enum DegradedStateToken {
    /// The relevant subsystem is warming.
    Warming,
    /// The identity or state is based on cached evidence.
    Cached,
    /// Only partial evidence is available.
    Partial,
    /// Evidence is stale relative to live truth.
    Stale,
    /// Subsystem is offline.
    Offline,
    /// Subsystem is blocked by policy.
    PolicyBlocked,
    /// Functionality is limited relative to normal operation.
    Limited,
    /// Feature or subsystem is unsupported in the current environment.
    Unsupported,
    /// Feature or subsystem is experimental.
    Experimental,
    /// A retest is pending before the state can be considered current.
    RetestPending,
}

impl DegradedStateToken {
    fn as_str(self) -> &'static str {
        match self {
            Self::Warming => "Warming",
            Self::Cached => "Cached",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Offline => "Offline",
            Self::PolicyBlocked => "PolicyBlocked",
            Self::Limited => "Limited",
            Self::Unsupported => "Unsupported",
            Self::Experimental => "Experimental",
            Self::RetestPending => "RetestPending",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Describes the recovery posture implied by degraded tokens and lifecycle state.
pub enum RecoveryModeClass {
    /// No recovery mode is active.
    None,
    /// Restricted mode recovery posture.
    RestrictedMode,
    /// Safe mode recovery posture.
    SafeMode,
    /// Open without restore recovery posture.
    OpenWithoutRestore,
    /// Partial open recovery posture.
    PartialOpen,
    /// Reconnecting recovery posture.
    Reconnecting,
    /// Read-only degraded recovery posture.
    ReadOnlyDegraded,
    /// Policy blocked recovery posture.
    PolicyBlocked,
    /// Missing metadata recovery posture.
    MissingMetadata,
    /// Mixed state review recovery posture.
    MixedStateReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Canonical field selectors used by `field_visibility` and `surface_projections`.
pub enum FieldPath {
    #[serde(rename = "workspace_identity.display_label")]
    /// Selects `workspace_identity.display_label`.
    WorkspaceIdentityDisplayLabel,
    #[serde(rename = "workspace_identity.workspace_kind")]
    /// Selects `workspace_identity.workspace_kind`.
    WorkspaceIdentityWorkspaceKind,
    #[serde(rename = "workspace_identity.lifecycle_state")]
    /// Selects `workspace_identity.lifecycle_state`.
    WorkspaceIdentityLifecycleState,
    #[serde(rename = "workspace_identity.root_summary")]
    /// Selects `workspace_identity.root_summary`.
    WorkspaceIdentityRootSummary,
    #[serde(rename = "repo_identity.repo_state_class")]
    /// Selects `repo_identity.repo_state_class`.
    RepoIdentityRepoStateClass,
    #[serde(rename = "repo_identity.repo_label")]
    /// Selects `repo_identity.repo_label`.
    RepoIdentityRepoLabel,
    #[serde(rename = "repo_identity.branch_label")]
    /// Selects `repo_identity.branch_label`.
    RepoIdentityBranchLabel,
    #[serde(rename = "repo_identity.revision_ref")]
    /// Selects `repo_identity.revision_ref`.
    RepoIdentityRevisionRef,
    #[serde(rename = "trust_identity.trust_state")]
    /// Selects `trust_identity.trust_state`.
    TrustIdentityTrustState,
    #[serde(rename = "trust_identity.trust_source")]
    /// Selects `trust_identity.trust_source`.
    TrustIdentityTrustSource,
    #[serde(rename = "host_identity.host_class")]
    /// Selects `host_identity.host_class`.
    HostIdentityHostClass,
    #[serde(rename = "host_identity.host_state")]
    /// Selects `host_identity.host_state`.
    HostIdentityHostState,
    #[serde(rename = "host_identity.target_label")]
    /// Selects `host_identity.target_label`.
    HostIdentityTargetLabel,
    #[serde(rename = "profile_identity.profile_label")]
    /// Selects `profile_identity.profile_label`.
    ProfileIdentityProfileLabel,
    #[serde(rename = "profile_identity.profile_mode")]
    /// Selects `profile_identity.profile_mode`.
    ProfileIdentityProfileMode,
    #[serde(rename = "profile_identity.deployment_profile")]
    /// Selects `profile_identity.deployment_profile`.
    ProfileIdentityDeploymentProfile,
    #[serde(rename = "profile_identity.identity_mode")]
    /// Selects `profile_identity.identity_mode`.
    ProfileIdentityIdentityMode,
    #[serde(rename = "route_state.route_kind")]
    /// Selects `route_state.route_kind`.
    RouteStateRouteKind,
    #[serde(rename = "route_state.route_label")]
    /// Selects `route_state.route_label`.
    RouteStateRouteLabel,
    #[serde(rename = "route_state.authority_delta")]
    /// Selects `route_state.authority_delta`.
    RouteStateAuthorityDelta,
    #[serde(rename = "route_state.route_freshness")]
    /// Selects `route_state.route_freshness`.
    RouteStateRouteFreshness,
    #[serde(rename = "degraded_or_recovery_state.degraded_tokens")]
    /// Selects `degraded_or_recovery_state.degraded_tokens`.
    DegradedOrRecoveryStateDegradedTokens,
    #[serde(rename = "degraded_or_recovery_state.recovery_mode")]
    /// Selects `degraded_or_recovery_state.recovery_mode`.
    DegradedOrRecoveryStateRecoveryMode,
    #[serde(rename = "degraded_or_recovery_state.last_failure_summary")]
    /// Selects `degraded_or_recovery_state.last_failure_summary`.
    DegradedOrRecoveryStateLastFailureSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Controls where a field is visible (primary chrome, condensed, inspector-only, etc).
pub enum VisibilityClass {
    /// Visible as a primary identity field.
    PrimaryVisible,
    /// Visible only in condensed mode or when space allows.
    CondensedVisible,
    /// Visible only in an inspector or drill-down surface.
    InspectorOnly,
    /// Visible only in the native window title.
    NativeTitleOnly,
    /// Visible only in support/export payloads.
    SupportExportOnly,
    /// Hidden because it is redacted by policy.
    HiddenRedacted,
    /// Omitted because the field does not apply.
    OmittedNotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Identifies one consuming surface that projects from the canonical record.
pub enum SurfaceKind {
    /// Title/context bar chrome surface.
    TitleContextBar,
    /// Native platform window title.
    NativeWindowTitle,
    /// Workspace status item surface (status bar / drawer entrypoint).
    WorkspaceStatusItem,
    /// Support/export packet projection.
    SupportExport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Declares the redaction posture for a surface projection.
pub enum RedactionClass {
    /// Default metadata-safe posture.
    MetadataSafeDefault,
    /// Operator-only restricted posture.
    OperatorOnlyRestricted,
    /// Internal support restricted posture.
    InternalSupportRestricted,
    /// Signing evidence-only posture.
    SigningEvidenceOnly,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn title_context_bar_example_fixtures_parse_and_project_required_surfaces() {
        let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/ux/title_context_bar_examples");
        let fixtures = [
            "restricted_mode_local_workspace.json",
            "partial_open_workspace.json",
            "detached_repo_metadata.json",
            "multi_root_mixed_state.json",
            "missing_host_details.json",
            "mixed_local_plus_remote_session.json",
        ];

        for fixture_name in fixtures {
            let payload = std::fs::read_to_string(fixtures_dir.join(fixture_name))
                .unwrap_or_else(|err| panic!("fixture read failed ({fixture_name}) — {err}"));
            let record: TitleContextBarStateRecord = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture parse failed ({fixture_name}) — {err}"));

            assert_eq!(
                record.record_kind,
                TitleContextBarStateRecordKind::TitleContextBarStateRecord
            );
            assert_eq!(record.title_context_bar_state_schema_version, 1);

            for required_surface in [
                SurfaceKind::TitleContextBar,
                SurfaceKind::NativeWindowTitle,
                SurfaceKind::WorkspaceStatusItem,
                SurfaceKind::SupportExport,
            ] {
                assert!(
                    record
                        .surface_projections
                        .iter()
                        .any(|proj| proj.surface == required_surface),
                    "fixture missing required projection ({fixture_name}) — {required_surface:?}",
                );
            }
        }
    }
}
