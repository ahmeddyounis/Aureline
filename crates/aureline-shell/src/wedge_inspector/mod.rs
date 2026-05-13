//! Labs-only inspector overlay for bounded wedge projections.
//!
//! The inspector is a read-only shell projection over the certified wedge
//! records that already expose deterministic `render_plaintext` output. It
//! does not productize the wedges or widen their contracts; it lists them,
//! quotes their prototype chip, quotes their claim-limit set, and surfaces the
//! underlying plaintext record for inspection.

use std::time::{SystemTime, UNIX_EPOCH};

use aureline_ai::{
    AttachmentKind, AttachmentStatusClass, ComposerAttachment, ComposerDraft, ComposerMention,
    MentionKind, MentionResolutionState, SelectionReasonClass, SourceClass as AiSourceClass,
    TrustPosture,
};
use aureline_content_safety::{detect_suspicious_content, TrustClass};
use aureline_extensions::manifest_baseline::{
    DeclaredVsEffectiveDiffEntry, EffectivePermissionBaselineRecord, EffectivePermissionDiffClass,
    ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord, HostContractFamilyClass,
    InstallDecisionClass, InstallDecisionReasonClass, ManifestInstallDecisionRecord,
    ManifestOriginSourceClass, ManifestScopeCompletenessClass, PermissionScopeClass,
    PermissionScopeEntry, PublisherLifecycleStateClass, PublisherTrustTierClass, RedactionClass,
    SummaryFreshnessClass, EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND,
    EXTENSION_MANIFEST_BASELINE_RECORD_KIND, EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
    MANIFEST_INSTALL_DECISION_RECORD_KIND,
};
use aureline_graph_proto::{
    ConfidenceLevel, NodeClass, ProvenanceClass, QueryFamilyTag, ShardAffinityTag,
    SourceClass as GraphSourceClass, Visibility, WorksetScopeClass,
};
use aureline_history::{HistoryStorageRoot, LocalHistoryStore, MutationJournalStore};
use aureline_preview::{build_risky_text_preview, RiskyTextInput};
use aureline_reactive_state::{
    open_workspace_readiness, LiveReactiveStore, WatcherHealthPhase, WorkspaceLifecyclePhase,
    WorkspaceReadinessSnapshot,
};
use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest, ExecutionContextResolver,
    ExecutionContextResolverConfig, IdentityMode, ScopeClass, TargetClass,
    TrustState as RuntimeTrustState,
};
use aureline_terminal::{HostClass, OpenSessionRequest, PtyHost, TerminalTrustState};

use crate::ai_context_inspector::AiContextInspectorSnapshot;
use crate::ai_truth_strip::{AiRouteSpendPosture, AiTruthStripInputs, AiTruthStripSnapshot};
use crate::graph_state_card::{materialize_graph_state_card, GraphStateCardSubject};
use crate::host_boundary_cues::{HostBoundaryCueCardRecord, HostBoundaryCueWedge};
use crate::install_review_fact_grid::{
    ActivationBudgetClass, InstallReviewFactGridRecord, InstallReviewFactGridWedge,
    RollbackPostureClass,
};
use crate::managed_workspace_labels::{ManagedAuthorityLineage, ManagedWorkspaceLifecycleWedge};
use crate::notebook_trust_badges::{
    CellContentClass, EscapeHatch, KernelAvailability, NotebookTrustBadgeRowBuilder,
    NotebookTrustBadgeWedge, NotebookTrustRung, OutputTrustState, RepresentationState,
    WidgetTrustState, WorkspaceTrustState,
};
use crate::permission_prompts::{
    AuthorityIssuerClass, DegradedCapabilityClass, GrantScopeClass, PermissionPromptAuthorityOwner,
    PermissionPromptDenialBranch, PermissionPromptQuestions, PermissionPromptRequester,
    PermissionPromptWedge, RequesterClass, ScopeFilterClass,
};
use crate::review_preview::DestructiveCoreEngine;
use crate::safe_preview_card::{SafePreviewCardSnapshot, SafePreviewSectionId};

/// Canonical command id that opens the Labs wedge inspector.
pub const WEDGE_INSPECTOR_COMMAND_ID: &str = "cmd:labs.open_wedge_inspector";

/// Canonical settings id that admits the Labs wedge inspector locally.
pub const WEDGE_INSPECTOR_SETTING_ID: &str = "shell.labs.wedge_inspector_enabled";

/// Runtime bindings supplied by the shell when opening the inspector.
#[derive(Debug, Clone)]
pub struct WedgeInspectorInputs {
    /// Current host-boundary card from the active terminal session, when any.
    pub host_boundary_card: Option<HostBoundaryCueCardRecord>,
    /// Current workspace lifecycle token observed by the shell.
    pub workspace_lifecycle_state_token: Option<String>,
    /// Current install-review fact-grid card, when an install review is active.
    pub install_review_card: Option<InstallReviewFactGridRecord>,
    /// Monotonic or wall-clock stamp quoted by fixture-backed rows.
    pub observed_at: String,
    /// Active workspace id used by rows that need a workspace binding.
    pub workspace_id: String,
}

impl Default for WedgeInspectorInputs {
    fn default() -> Self {
        Self {
            host_boundary_card: None,
            workspace_lifecycle_state_token: None,
            install_review_card: None,
            observed_at: "mono:wedge_inspector:0".to_string(),
            workspace_id: "workspace:wedge_inspector".to_string(),
        }
    }
}

/// One claim-limit row quoted by an inspector panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WedgeInspectorClaimLimit {
    /// Stable token quoted from the wedge record or wrapper.
    pub token: String,
    /// Human-readable claim text quoted next to [`Self::token`].
    pub label: String,
}

impl WedgeInspectorClaimLimit {
    fn new(token: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            label: label.into(),
        }
    }
}

/// One row in the wedge inspector overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WedgeInspectorRow {
    /// Stable row id used by tests and renderer selection.
    pub wedge_id: String,
    /// Human name shown in the inspector list.
    pub display_name: String,
    /// Prototype-label token quoted from the wedge record or wrapper.
    pub prototype_label_token: String,
    /// Prototype-label display text quoted from the wedge record or wrapper.
    pub prototype_label_display: String,
    /// Source binding used to produce this panel.
    pub source_binding: String,
    /// Claim-limit rows quoted before the panel body.
    pub claim_limits: Vec<WedgeInspectorClaimLimit>,
    /// Read-only panel text rendered when this row is selected.
    pub panel_plaintext: String,
}

impl WedgeInspectorRow {
    fn new(
        wedge_id: &str,
        display_name: &str,
        prototype_label_token: impl Into<String>,
        prototype_label_display: impl Into<String>,
        source_binding: impl Into<String>,
        claim_limits: Vec<WedgeInspectorClaimLimit>,
        render_plaintext: impl Into<String>,
    ) -> Self {
        let prototype_label_token = prototype_label_token.into();
        let prototype_label_display = prototype_label_display.into();
        let source_binding = source_binding.into();
        let render_plaintext = render_plaintext.into();
        let panel_plaintext = wrap_panel_plaintext(
            &source_binding,
            &prototype_label_token,
            &prototype_label_display,
            &claim_limits,
            &render_plaintext,
        );
        Self {
            wedge_id: wedge_id.to_string(),
            display_name: display_name.to_string(),
            prototype_label_token,
            prototype_label_display,
            source_binding,
            claim_limits,
            panel_plaintext,
        }
    }
}

/// Read-only inspector model consumed by the native overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WedgeInspectorOverlay {
    rows: Vec<WedgeInspectorRow>,
    selection: usize,
}

impl WedgeInspectorOverlay {
    /// Builds an inspector model from current shell bindings plus safe seeds.
    pub fn new(inputs: WedgeInspectorInputs) -> Self {
        Self {
            rows: build_rows(inputs),
            selection: 0,
        }
    }

    /// Returns all inspector rows in render order.
    pub fn rows(&self) -> &[WedgeInspectorRow] {
        &self.rows
    }

    /// Returns the currently selected row, if any rows exist.
    pub fn selected_row(&self) -> Option<&WedgeInspectorRow> {
        self.rows.get(self.selection)
    }

    /// Returns the selected row index.
    pub const fn selection(&self) -> usize {
        self.selection
    }

    /// Selects the next row, wrapping at the end.
    pub fn select_next(&mut self) {
        if self.rows.is_empty() {
            self.selection = 0;
        } else {
            self.selection = (self.selection + 1) % self.rows.len();
        }
    }

    /// Selects the previous row, wrapping at the start.
    pub fn select_prev(&mut self) {
        if self.rows.is_empty() {
            self.selection = 0;
        } else {
            self.selection = (self.selection + self.rows.len() - 1) % self.rows.len();
        }
    }

    /// Renders fixed-width lines for the native overlay.
    pub fn render_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("Wedge Inspector - Labs".to_string());
        lines.push(String::new());
        for (idx, row) in self.rows.iter().enumerate() {
            let marker = if idx == self.selection { ">" } else { " " };
            lines.push(format!(
                "{marker} {} [{}]",
                row.display_name, row.prototype_label_token
            ));
        }
        lines.push(String::new());
        if let Some(row) = self.selected_row() {
            lines.push(format!("Panel: {}", row.display_name));
            lines.extend(row.panel_plaintext.lines().map(str::to_owned));
        } else {
            lines.push("No wedge rows available.".to_string());
        }
        lines
    }
}

fn build_rows(inputs: WedgeInspectorInputs) -> Vec<WedgeInspectorRow> {
    let install_review_card = inputs
        .install_review_card
        .clone()
        .unwrap_or_else(|| install_review_wedge().card());
    vec![
        ai_context_row(&inputs),
        ai_truth_strip_row(&inputs),
        host_boundary_row(&inputs),
        managed_workspace_row(&inputs),
        notebook_trust_row(&inputs),
        install_review_row(&install_review_card),
        permission_prompt_row(&install_review_card),
        review_preview_row(&inputs),
        safe_preview_row(),
        graph_state_row(&inputs),
    ]
}

fn wrap_panel_plaintext(
    source_binding: &str,
    prototype_label_token: &str,
    prototype_label_display: &str,
    claim_limits: &[WedgeInspectorClaimLimit],
    render_plaintext: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("source_binding: {source_binding}\n"));
    out.push_str("prototype_label:\n");
    out.push_str(&format!("  token: {prototype_label_token}\n"));
    out.push_str(&format!("  label: {prototype_label_display}\n"));
    out.push_str("claim_limits:\n");
    for row in claim_limits {
        out.push_str(&format!("  - {}: {}\n", row.token, row.label));
    }
    out.push_str("render_plaintext:\n");
    out.push_str(render_plaintext);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

fn ai_context_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let snapshot = AiContextInspectorSnapshot::project(&composer_draft(&inputs.workspace_id));
    WedgeInspectorRow::new(
        "ai_context_inspector",
        "AI Context Inspector",
        snapshot.prototype_label_token.clone(),
        snapshot.prototype_label_text.clone(),
        "composer_draft:active_seed",
        vec![
            WedgeInspectorClaimLimit::new("single_bounded_wedge_only", "One composer draft only."),
            WedgeInspectorClaimLimit::new("read_only_no_mutation", "Read-only projection."),
            WedgeInspectorClaimLimit::new("no_model_dispatch", "No model dispatch."),
            WedgeInspectorClaimLimit::new("composer_draft_only", "Composer draft fields only."),
        ],
        snapshot.render_plaintext(),
    )
}

fn ai_truth_strip_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let snapshot = AiTruthStripSnapshot::project(
        &composer_draft(&inputs.workspace_id),
        &AiRouteSpendPosture::m1_seed_default(),
        AiTruthStripInputs {
            evidence_packet_id: "evidence_packet:wedge_inspector:ai_truth_strip",
            exact_build_identity_ref: "build:local:wedge_inspector",
            minted_at: &inputs.observed_at,
        },
    );
    let claim_limits = snapshot
        .evidence_packet
        .claim_limits
        .iter()
        .map(|limit| WedgeInspectorClaimLimit::new(limit.as_str(), limit.as_str()))
        .collect();
    WedgeInspectorRow::new(
        "ai_truth_strip",
        "AI Truth Strip",
        snapshot.evidence_packet.prototype_label_token.clone(),
        snapshot.evidence_packet.prototype_label_text.clone(),
        "composer_draft:active_seed + route_spend_posture:local_no_dispatch",
        claim_limits,
        snapshot.render_plaintext(),
    )
}

fn host_boundary_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let card = inputs
        .host_boundary_card
        .clone()
        .unwrap_or_else(|| fallback_host_boundary_card(&inputs.workspace_id, &inputs.observed_at));
    let claim_limits = card
        .claim_limits
        .iter()
        .map(|row| WedgeInspectorClaimLimit::new(row.token.clone(), row.label.clone()))
        .collect();
    WedgeInspectorRow::new(
        "host_boundary_cues",
        "Host Boundary Cues",
        card.prototype_label_token.clone(),
        card.prototype_label_display.clone(),
        "terminal_pane:active_session_header",
        claim_limits,
        card.render_plaintext(),
    )
}

fn managed_workspace_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let mut wedge = ManagedWorkspaceLifecycleWedge::new(inputs.workspace_id.clone());
    let lineage = managed_lineage(&inputs.workspace_id);
    let _ = wedge.open_authenticating(lineage.clone(), &inputs.observed_at);
    let _ = wedge.record_connecting(lineage.clone(), &inputs.observed_at);
    let _ = wedge.record_warming(lineage.clone(), &inputs.observed_at);
    match inputs.workspace_lifecycle_state_token.as_deref() {
        Some("ready") => {
            let _ = wedge.record_ready(lineage.clone(), &inputs.observed_at);
        }
        Some("degraded") => {
            let _ = wedge.record_ready(lineage.clone(), &inputs.observed_at);
            let _ = wedge.record_read_only_degraded(
                lineage.clone(),
                &inputs.observed_at,
                "workspace_lifecycle_degraded",
            );
        }
        Some("closed") | Some("closing") => {
            let _ = wedge.record_ready(lineage.clone(), &inputs.observed_at);
            let _ = wedge.record_closed(
                lineage.clone(),
                &inputs.observed_at,
                Some("workspace_lifecycle_closed"),
            );
        }
        _ => {}
    }
    let card = wedge.card();
    let claim_limits = card
        .claim_limits
        .iter()
        .map(|row| WedgeInspectorClaimLimit::new(row.token.clone(), row.label.clone()))
        .collect();
    let source = format!(
        "workspace_lifecycle:current_state={}",
        inputs
            .workspace_lifecycle_state_token
            .as_deref()
            .unwrap_or("unobserved")
    );
    WedgeInspectorRow::new(
        "managed_workspace_labels",
        "Managed Workspace Labels",
        card.prototype_label_token.clone(),
        card.prototype_label_display.clone(),
        source,
        claim_limits,
        card.render_plaintext(),
    )
}

fn notebook_trust_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let mut wedge = NotebookTrustBadgeWedge::new(
        inputs.workspace_id.clone(),
        "notebook:wedge_inspector:demo.ipynb",
    )
    .with_workspace_trust(WorkspaceTrustState::TrustedWorkspace)
    .with_notebook_trust_rung(NotebookTrustRung::StructuralOnlyTrusted)
    .with_kernel_availability(KernelAvailability::LocalManagedAvailable)
    .with_output_trust(OutputTrustState::CapturedFromPriorSession)
    .with_widget_trust(WidgetTrustState::WidgetDeniedByDefault);
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.markdown.summary",
            "cell:1",
            CellContentClass::MarkdownCell,
            RepresentationState::Sanitized,
        )
        .with_escape_hatches([EscapeHatch::SafePreview]),
    );
    wedge.add_row(
        NotebookTrustBadgeRowBuilder::new(
            "row.code.seed",
            "cell:2",
            CellContentClass::CodeCell,
            RepresentationState::Escaped,
        )
        .with_honesty_marker(true)
        .with_escape_hatches([EscapeHatch::SafePreview, EscapeHatch::ExportRawSource]),
    );
    let card = wedge.card();
    let claim_limits = card
        .claim_limits
        .iter()
        .map(|row| WedgeInspectorClaimLimit::new(row.token.clone(), row.label.clone()))
        .collect();
    WedgeInspectorRow::new(
        "notebook_trust_badges",
        "Notebook Trust Badges",
        card.prototype_label_token.clone(),
        card.prototype_label_display.clone(),
        "notebook_preview:seeded_trust_axes",
        claim_limits,
        card.render_plaintext(),
    )
}

fn install_review_row(card: &InstallReviewFactGridRecord) -> WedgeInspectorRow {
    let claim_limits = card
        .claim_limits
        .iter()
        .map(|row| WedgeInspectorClaimLimit::new(row.token.clone(), row.label.clone()))
        .collect();
    WedgeInspectorRow::new(
        "install_review_fact_grid",
        "Install Review Fact Grid",
        card.prototype_label_token.clone(),
        card.prototype_label_display.clone(),
        "install_review:active_fact_grid_or_seed",
        claim_limits,
        card.render_plaintext(),
    )
}

fn permission_prompt_row(install_review: &InstallReviewFactGridRecord) -> WedgeInspectorRow {
    let prompt = PermissionPromptWedge::new(
        install_review.clone(),
        "permission_prompt:wedge_inspector:install_review",
        PermissionPromptRequester {
            requester_class: RequesterClass::Extension,
            requester_class_token: RequesterClass::Extension.as_str().to_string(),
            requester_ref: "extension:acme-labs/prose-helper".to_string(),
            requester_display_label: "Acme Labs Prose Helper".to_string(),
            request_origin_label: "Install-review fact grid selected in the inspector."
                .to_string(),
        },
        PermissionPromptAuthorityOwner {
            issuer_class: AuthorityIssuerClass::Shell,
            issuer_class_token: AuthorityIssuerClass::Shell.as_str().to_string(),
            issuer_source_ref: "source.shell.user_approval".to_string(),
            issuer_source_label: "Shell approval lane".to_string(),
        },
        ScopeFilterClass::CurrentRoot,
        "Current workspace root for declared read-only manifest scope.",
        GrantScopeClass::Workspace,
        "Workspace grant remembered until revoked from settings.",
        PermissionPromptDenialBranch {
            degraded_capability_class: DegradedCapabilityClass::ReadOnlyInspectionContinues,
            degraded_capability_token: DegradedCapabilityClass::ReadOnlyInspectionContinues
                .as_str()
                .to_string(),
            deny_path_label:
                "Local editing and read-only manifest inspection continue; extension is not enabled."
                    .to_string(),
            preserved_work_refs: vec![
                "workspace.local_editing.current_root".to_string(),
                "extension.manifest.metadata_only_view".to_string(),
            ],
        },
        PermissionPromptQuestions {
            who_is_asking: "The Acme Labs Prose Helper extension manifest is asking.".to_string(),
            what_boundary: "Workspace read access and connected AI-provider use.".to_string(),
            why_needed: "The extension reviews prose documents and suggests edits.".to_string(),
            what_changes_if_allowed:
                "The extension can activate under the install-review decision.".to_string(),
            what_works_if_denied:
                "Local editing and read-only extension metadata remain available.".to_string(),
            grant_persistence_statement:
                "Workspace grant only; it can be revoked from settings.".to_string(),
        },
    );
    let card = prompt.card();
    let claim_limits = card
        .claim_limits
        .iter()
        .map(|row| WedgeInspectorClaimLimit::new(row.token.clone(), row.label.clone()))
        .collect();
    WedgeInspectorRow::new(
        "permission_prompts",
        "Permission Prompts",
        card.prototype_label_token.clone(),
        card.prototype_label_display.clone(),
        format!("install_review_fact_grid:{}", card.install_review_card_ref),
        claim_limits,
        card.render_plaintext(),
    )
}

fn review_preview_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let packet_text = review_preview_plaintext(&inputs.workspace_id)
        .unwrap_or_else(|err| format!("Preview / apply / revert unavailable: {err}\n"));
    WedgeInspectorRow::new(
        "review_preview",
        "Review Preview",
        "prototype_preview_apply_revert_destructive_core",
        "Prototype - preview/apply/revert destructive core",
        "destructive_core:seeded_preview_packet",
        vec![
            WedgeInspectorClaimLimit::new(
                "single_destructive_core_path_only",
                "One destructive core path only.",
            ),
            WedgeInspectorClaimLimit::new(
                "preview_apply_revert_prototype_only",
                "Preview/apply/revert prototype only.",
            ),
            WedgeInspectorClaimLimit::new(
                "local_history_checkpoint_only",
                "Local-history checkpoint path only.",
            ),
            WedgeInspectorClaimLimit::new(
                "no_scope_widen_after_preview",
                "Scope cannot widen after preview.",
            ),
        ],
        packet_text,
    )
}

fn safe_preview_row() -> WedgeInspectorRow {
    let record = build_risky_text_preview(RiskyTextInput {
        preview_id: "preview:risky_text:wedge_inspector".to_string(),
        source_subject_ref: "buffer:file:src/lib.rs#identifier:config_loader".to_string(),
        source_surface_family: "editor".to_string(),
        trust_class: TrustClass::RawText,
        detection: detect_suspicious_content("let \u{202E}admin\u{200D}user = 1;"),
    });
    let snapshot = SafePreviewCardSnapshot::project(&record);
    let claim_limits = snapshot
        .section(SafePreviewSectionId::ClaimLimits)
        .map(|section| {
            section
                .rows
                .iter()
                .map(|row| {
                    WedgeInspectorClaimLimit::new(
                        row.value_token.clone().unwrap_or_else(|| row.label.clone()),
                        row.value.clone(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    WedgeInspectorRow::new(
        "safe_preview_card",
        "Safe Preview Card",
        snapshot.prototype_label_token.clone(),
        snapshot.prototype_label_text.clone(),
        "safe_preview_record:risky_text_seed",
        claim_limits,
        snapshot.render_plaintext(),
    )
}

fn graph_state_row(inputs: &WedgeInspectorInputs) -> WedgeInspectorRow {
    let store = LiveReactiveStore::new();
    let snapshot = WorkspaceReadinessSnapshot {
        workspace_id: inputs.workspace_id.clone(),
        lifecycle_phase: match inputs.workspace_lifecycle_state_token.as_deref() {
            Some("ready") => WorkspaceLifecyclePhase::Ready,
            Some("degraded") => WorkspaceLifecyclePhase::Degraded,
            Some("closed") => WorkspaceLifecyclePhase::Closed,
            _ => WorkspaceLifecyclePhase::PartiallyReady,
        },
        watcher_health: Some(match inputs.workspace_lifecycle_state_token.as_deref() {
            Some("ready") => WatcherHealthPhase::Healthy,
            Some("degraded") => WatcherHealthPhase::Degraded,
            Some("closed") => WatcherHealthPhase::Unavailable,
            _ => WatcherHealthPhase::Warming,
        }),
        hot_index_ready: matches!(
            inputs.workspace_lifecycle_state_token.as_deref(),
            Some("ready")
        ),
        command_graph_ready: matches!(
            inputs.workspace_lifecycle_state_token.as_deref(),
            Some("ready")
        ),
        observed_at: inputs.observed_at.clone(),
    };
    let (_, projection) =
        open_workspace_readiness(&store, &snapshot).expect("readiness seed publishes");
    let subject = GraphStateCardSubject {
        target_id: "topology_walk:workset:wedge_inspector".to_string(),
        node_class: NodeClass::WorksetScopeNode,
        query_family: QueryFamilyTag::TopologyWalk,
        shard_affinity: ShardAffinityTag::GraphOverlayShard,
        source_class: GraphSourceClass::SymbolResolver,
        provenance_class: ProvenanceClass::AuthoritativeProducer,
        scope_class: WorksetScopeClass::NamedWorkset,
        scope_visibility: Visibility::FullyVisible,
        rolled_up_confidence: Some(ConfidenceLevel::High),
        partial_note: Some("Inspector reflects the active workspace readiness token.".to_string()),
    };
    let card = materialize_graph_state_card(&projection, &subject);
    let claim_limits = card
        .claim_limits
        .iter()
        .map(|row| WedgeInspectorClaimLimit::new(row.token.clone(), row.label.clone()))
        .collect();
    WedgeInspectorRow::new(
        "graph_state_card",
        "Graph State Card",
        card.prototype_label_token.clone(),
        card.prototype_label_display.clone(),
        "workspace_readiness_projection:active_workspace",
        claim_limits,
        card.render_plaintext(),
    )
}

fn composer_draft(workspace_id: &str) -> ComposerDraft {
    let mut draft = ComposerDraft::new(
        "draft:wedge_inspector",
        "session:wedge_inspector",
        workspace_id,
        "Explain the current command and workspace context.",
    );
    draft.add_mention(ComposerMention {
        mention_id: "mention:command_palette".to_string(),
        kind: MentionKind::SymbolMention,
        target_stable_id: Some("cmd:command_palette.open".to_string()),
        display_label: "@command_palette.open".to_string(),
        resolution_state: MentionResolutionState::Resolved,
    });
    draft.add_attachment(ComposerAttachment {
        attachment_id: "attachment:workspace_slice".to_string(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: AiSourceClass::WorkspaceFileSlice,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::Live,
        estimated_byte_size: 1024,
        display_label: "src/lib.rs slice".to_string(),
        scope_truth: None,
        placed_under_fenced_role: false,
    });
    draft
}

fn fallback_host_boundary_card(workspace_id: &str, observed_at: &str) -> HostBoundaryCueCardRecord {
    let mut resolver = ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: workspace_id.to_string(),
        profile_id: Some("profile:local".to_string()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some("/workspace".to_string()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: format!(
            "localhost:{}-{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: format!("caps:{workspace_id}:wedge_inspector"),
            capsule_hash: "sha256:wedge-inspector".to_string(),
            resolved_schema_version: "1".to_string(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "wedge-inspector".to_string(),
    });
    let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "terminal.wedge_inspector",
        RuntimeTrustState::Trusted,
        observed_at,
    ));
    let mut host = PtyHost::new();
    let id = host.open_session(OpenSessionRequest {
        workspace_id,
        host_class: HostClass::HostDesktop,
        display_title: "zsh",
        cwd_hint: Some("/workspace"),
        execution_context_ref: context.execution_context_id(),
        trust_state: TerminalTrustState::Trusted,
        observed_at,
    });
    let _ = host.mark_starting(&id, observed_at);
    let _ = host.mark_active(&id, observed_at);
    let session = host.session(&id).expect("session opened").clone();
    let mut wedge = HostBoundaryCueWedge::new(workspace_id);
    let _ = wedge.open_initial(&context, &session, observed_at);
    wedge.card()
}

fn managed_lineage(workspace_id: &str) -> ManagedAuthorityLineage {
    ManagedAuthorityLineage::new(
        workspace_id,
        "tenant:local-managed-demo",
        "account_free_local",
        "local_first",
    )
}

fn install_review_wedge() -> InstallReviewFactGridWedge {
    InstallReviewFactGridWedge::new(
        verified_admit_manifest(),
        verified_admit_effective(),
        verified_admit_decision(),
        ActivationBudgetClass::LazyOnDemandOnly,
        RollbackPostureClass::CleanUninstallAndStatePurge,
    )
}

fn verified_admit_manifest() -> ExtensionManifestBaselineRecord {
    ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_string(),
        extension_identity: "acme-labs/prose-helper".to_string(),
        extension_version: "1.4.2".to_string(),
        extension_lifecycle_state_class: ExtensionLifecycleStateClass::Published,
        host_contract_family_class: HostContractFamilyClass::WasmComponentModel,
        manifest_origin_source_class: ManifestOriginSourceClass::PublicRegistry,
        origin_source_label: "public registry: registry.aureline.dev".to_string(),
        publisher_identity_ref: "publisher:acme-labs".to_string(),
        publisher_display_label: "Acme Labs".to_string(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        publisher_lifecycle_state_class: PublisherLifecycleStateClass::Active,
        publisher_signing_key_ref: "key:acme-labs:ed25519:2026-q2".to_string(),
        declared_permissions: vec![
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::FilesystemRead,
                scope_target: "workspace:/docs/**".to_string(),
                scope_constraint: Some("read-only under declared workspace prefix".to_string()),
                rationale_label: "Read prose documents for grammar suggestions.".to_string(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::AiProviderAccess,
                scope_target: "connected-provider:ai:acme-default".to_string(),
                scope_constraint: Some("requires user-configured provider link".to_string()),
                rationale_label: "Use the user's AI provider to refine suggestions.".to_string(),
            },
        ],
        manifest_scope_completeness_class: ManifestScopeCompletenessClass::Complete,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_effective() -> EffectivePermissionBaselineRecord {
    let manifest = verified_admit_manifest();
    EffectivePermissionBaselineRecord {
        record_kind: EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        extension_identity_ref: manifest.extension_identity.clone(),
        extension_version: manifest.extension_version.clone(),
        effective_permissions: manifest.declared_permissions.clone(),
        declared_vs_effective_diff: manifest
            .declared_permissions
            .iter()
            .map(|permission| DeclaredVsEffectiveDiffEntry {
                scope_class: permission.scope_class,
                scope_target: permission.scope_target.clone(),
                diff_class: EffectivePermissionDiffClass::Unchanged,
                narrowing_reason_label: "unchanged".to_string(),
            })
            .collect(),
        widening_attempted_blocked_count: 0,
        applied_policy_pack_refs: Vec::new(),
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-11T08:00:00Z".to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn verified_admit_decision() -> ManifestInstallDecisionRecord {
    ManifestInstallDecisionRecord {
        record_kind: MANIFEST_INSTALL_DECISION_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: "manifest_baseline:acme-labs/prose-helper:1.4.2".to_string(),
        install_decision_class: InstallDecisionClass::Admit,
        install_decision_reason_class: InstallDecisionReasonClass::AdmittedNoViolation,
        decision_summary: "Admitted: complete manifest, attributed publisher, no widening attempt."
            .to_string(),
        decided_at: "2026-05-11T08:00:01Z".to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn review_preview_plaintext(workspace_id: &str) -> Result<String, String> {
    let root = std::env::temp_dir().join(format!(
        "aureline_wedge_inspector_review_preview_{}",
        unique_stamp()
    ));
    let storage = HistoryStorageRoot::new(&root);
    let journal = MutationJournalStore::new(storage.clone());
    let history = LocalHistoryStore::new(storage);
    let mut engine = DestructiveCoreEngine::new(workspace_id, "local user", journal, history)
        .with_pinned_clock(vec![
            "2026-05-11T13:30:00Z".to_string(),
            "2026-05-11T13:30:01Z".to_string(),
        ]);
    engine.seed_target("docs/alpha.md", b"alpha alpha".to_vec());
    engine.seed_target("docs/beta.md", b"beta alpha".to_vec());
    let mut packet = engine
        .propose(&["docs/alpha.md", "docs/beta.md"], "alpha", "delta")
        .map_err(|err| err.to_string())?;
    engine.preview(&mut packet).map_err(|err| err.to_string())?;
    Ok(packet.render_plaintext())
}

fn unique_stamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
