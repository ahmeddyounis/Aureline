use super::records::{
    CapabilityClass, CoordinateTranslationRequirementClass, DecisionOutcome, DegradedStateClass,
    FallbackClass, FaultDomainId, FreshnessClass, HealthState, LaneClass, LanguageServerHostStatus,
    LocalityClass, PlacementPreferenceClass, PrecedenceBand, ProviderKind, ProviderStackRow,
    RedactionClass, RequestedAuthorityFloorClass, ResolutionMode, RouterDecisionRecord,
    RouterRequestContext, RouterTrustState, RoutingContext, ScopeClaimClass, SupportClass,
    SurfaceClass, SurfaceReport, ROUTER_DECISION_SCHEMA_VERSION,
};

/// Request admitted by the baseline LSP router.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterRequest {
    /// Requested language id.
    pub language_id: String,
    /// Request context emitted into the decision record.
    pub request_context: RouterRequestContext,
    /// Root, workset, and toolchain context emitted into the decision record.
    pub routing_context: RoutingContext,
    /// Capture timestamp for deterministic fixtures.
    pub captured_at: String,
}

/// Parameters for a workspace-local router request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceLocalRouterRequest {
    /// Requested language id.
    pub language_id: String,
    /// Protected surface requesting language truth.
    pub surface_class: SurfaceClass,
    /// Capability being routed.
    pub capability_class: CapabilityClass,
    /// Opaque subject reference.
    pub requested_subject_ref: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Root reference.
    pub root_ref: String,
    /// Execution context id anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Capture timestamp.
    pub captured_at: String,
}

impl RouterRequest {
    /// Builds a launch-language request for a workspace-local subject.
    pub fn workspace_local(request: WorkspaceLocalRouterRequest) -> Self {
        let workspace_id = request.workspace_id;
        let root_ref = request.root_ref;
        let execution_context_id = request.execution_context_id;
        Self {
            language_id: request.language_id,
            request_context: RouterRequestContext {
                requested_surface_class: request.surface_class,
                requested_capability_class: request.capability_class,
                requested_authority_floor_class:
                    RequestedAuthorityFloorClass::AuthoritativePreferred,
                requested_scope_claim_class: ScopeClaimClass::ActiveWorkset,
                requested_subject_ref: request.requested_subject_ref,
                placement_preference_class: PlacementPreferenceClass::MatchSubjectLocation,
                coordinate_translation_requirement_class:
                    CoordinateTranslationRequirementClass::RequiredBeforeResult,
                policy_epoch: "policy:epoch:local:trusted".into(),
                trust_state: RouterTrustState::Trusted,
                execution_context_id: execution_context_id.clone(),
            },
            routing_context: RoutingContext {
                workspace_id: workspace_id.clone(),
                workset_id: format!("workset:active:{workspace_id}"),
                workspace_root_ref: root_ref.clone(),
                subject_root_ref: root_ref.clone(),
                package_root_ref: Some(format!("package-root:{root_ref}")),
                config_root_ref: Some(format!("config-root:{root_ref}:language")),
                lane_class: LaneClass::LocalOnly,
                target_summary: "Local desktop workspace target for the active workset.".into(),
                toolchain_summary: format!(
                    "Language tooling is anchored by {execution_context_id}."
                ),
            },
            captured_at: request.captured_at,
        }
    }
}

/// Baseline LSP router for launch-language language-server hosts.
#[derive(Debug, Clone, Default)]
pub struct LspRouter;

impl LspRouter {
    /// Builds the baseline LSP router.
    pub fn new() -> Self {
        Self
    }

    /// Routes one request through supervised LSP hosts and syntax fallback.
    pub fn route(
        &self,
        request: RouterRequest,
        host_statuses: &[LanguageServerHostStatus],
    ) -> RouterDecisionRecord {
        let matching_hosts = host_statuses
            .iter()
            .filter(|status| {
                status.identity.language_id == request.language_id
                    && status.identity.workspace_id == request.routing_context.workspace_id
                    && status.identity.root_ref == request.routing_context.subject_root_ref
            })
            .collect::<Vec<_>>();

        let requested_capability = request.request_context.requested_capability_class;
        let mut provider_stack_rows = matching_hosts
            .iter()
            .map(|status| status.provider_stack_row(requested_capability))
            .collect::<Vec<_>>();

        if provider_stack_rows.is_empty() {
            provider_stack_rows.push(missing_lsp_provider_row(&request));
        }

        if requested_capability.syntax_fallback_allowed() {
            provider_stack_rows.push(syntax_fallback_row(&request));
        }

        let selected_lsp = provider_stack_rows.iter().find(|row| {
            row.provider_kind == ProviderKind::LanguageServer
                && row.support_class == SupportClass::Authoritative
                && row.health_state.is_selectable_primary()
                && row.freshness_class == FreshnessClass::AuthoritativeLive
        });

        let (decision_outcome, surface_report, export_safe_summary) =
            if let Some(selected) = selected_lsp {
                ready_lsp_outcome(&request, selected)
            } else {
                fallback_outcome(&request, &provider_stack_rows)
            };

        RouterDecisionRecord {
            record_kind: RouterDecisionRecord::RECORD_KIND.into(),
            router_decision_schema_version: ROUTER_DECISION_SCHEMA_VERSION,
            router_decision_id: format!(
                "router:decision:{}:{}:{}",
                request.request_context.requested_surface_class.as_str(),
                sanitize_id(&request.language_id),
                sanitize_id(&request.routing_context.subject_root_ref)
            ),
            request_context: request.request_context,
            routing_context: request.routing_context,
            provider_stack_rows,
            decision_outcome,
            surface_report,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            export_safe_summary,
        }
    }
}

fn ready_lsp_outcome(
    request: &RouterRequest,
    selected: &ProviderStackRow,
) -> (DecisionOutcome, SurfaceReport, String) {
    let routing_reason = format!(
        "{} is healthy and authoritative for {} in the requested scope.",
        selected.provider_display_label,
        request.request_context.requested_capability_class.as_str()
    );
    let fallback_summary =
        "No fallback was used; syntax fallback remains visible as a lower-authority lane.".into();
    (
        DecisionOutcome {
            resolution_mode: ResolutionMode::SingleWinner,
            degraded_state_class: DegradedStateClass::None,
            fallback_class: FallbackClass::NoFallback,
            selected_provider_id: selected.provider_id.clone(),
            routing_reason: routing_reason.clone(),
            fallback_summary,
        },
        SurfaceReport {
            origin_label: selected.provider_display_label.clone(),
            degraded_state_class: DegradedStateClass::None,
            user_visible_summary: format!(
                "{} is served by {}.",
                surface_label(request),
                selected.provider_display_label
            ),
            export_safe_explanation: routing_reason.clone(),
        },
        format!(
            "{} route selected {} with live host identity {}.",
            surface_label(request),
            selected.provider_display_label,
            selected.provider_id
        ),
    )
}

fn fallback_outcome(
    request: &RouterRequest,
    provider_stack_rows: &[ProviderStackRow],
) -> (DecisionOutcome, SurfaceReport, String) {
    let requested_capability = request.request_context.requested_capability_class;
    let preferred_lsp = provider_stack_rows
        .iter()
        .find(|row| row.provider_kind == ProviderKind::LanguageServer);
    let fallback = provider_stack_rows
        .iter()
        .find(|row| row.provider_kind == ProviderKind::SyntaxParser);

    let degraded_state = preferred_lsp
        .map(|row| degraded_state_for(row.health_state, request.routing_context.lane_class))
        .unwrap_or(DegradedStateClass::DegradedProviderUnavailable);

    if let Some(fallback) = fallback.filter(|_| {
        requested_capability.syntax_fallback_allowed()
            && request
                .request_context
                .requested_authority_floor_class
                .allows_fallback()
    }) {
        let fallback_class = preferred_lsp
            .map(|row| row.fallback_class)
            .unwrap_or(FallbackClass::ProtocolToText);
        let routing_reason = fallback_routing_reason(request, preferred_lsp, fallback);
        let fallback_summary = fallback_summary(request, degraded_state);
        (
            DecisionOutcome {
                resolution_mode: ResolutionMode::OrderedFallback,
                degraded_state_class: degraded_state,
                fallback_class,
                selected_provider_id: fallback.provider_id.clone(),
                routing_reason: routing_reason.clone(),
                fallback_summary: fallback_summary.clone(),
            },
            SurfaceReport {
                origin_label: fallback.provider_display_label.clone(),
                degraded_state_class: degraded_state,
                user_visible_summary: format!(
                    "{} fell back to {}.",
                    surface_label(request),
                    fallback.provider_display_label
                ),
                export_safe_explanation: routing_reason.clone(),
            },
            format!(
                "{} route used fallback because the language-server lane was not eligible: {}",
                surface_label(request),
                fallback_summary
            ),
        )
    } else {
        let routing_reason =
            "No provider met the requested authority floor and no honest fallback was available."
                .to_owned();
        (
            DecisionOutcome {
                resolution_mode: ResolutionMode::Unsupported,
                degraded_state_class: degraded_state,
                fallback_class: FallbackClass::UnsupportedNoFallback,
                selected_provider_id: "provider:unsupported:none".into(),
                routing_reason: routing_reason.clone(),
                fallback_summary: "No fallback may satisfy this route.".into(),
            },
            SurfaceReport {
                origin_label: "No language provider selected".into(),
                degraded_state_class: degraded_state,
                user_visible_summary: format!("{} is unavailable.", surface_label(request)),
                export_safe_explanation: routing_reason.clone(),
            },
            format!("{} route is unsupported.", surface_label(request)),
        )
    }
}

fn fallback_routing_reason(
    request: &RouterRequest,
    preferred_lsp: Option<&ProviderStackRow>,
    fallback: &ProviderStackRow,
) -> String {
    match preferred_lsp {
        Some(row) => format!(
            "{} was {}, so the router selected {} with an explicit downgrade.",
            row.provider_display_label,
            health_phrase(row.health_state),
            fallback.provider_display_label
        ),
        None => format!(
            "No language-server host was registered for {}, so the router selected {} with an explicit downgrade.",
            request.language_id, fallback.provider_display_label
        ),
    }
}

fn fallback_summary(request: &RouterRequest, degraded_state: DegradedStateClass) -> String {
    match degraded_state {
        DegradedStateClass::DegradedCrashLoopQuarantine => format!(
            "{} cannot use the quarantined language server; only syntax or text fallback may answer.",
            surface_label(request)
        ),
        DegradedStateClass::DegradedRemoteUnreachable => format!(
            "{} cannot use the remote language lane; local fallback is explicitly lower authority.",
            surface_label(request)
        ),
        DegradedStateClass::DegradedCachedFallback => format!(
            "{} cannot claim live LSP semantics; cached or syntax fallback must be labeled.",
            surface_label(request)
        ),
        DegradedStateClass::DegradedHeuristicFallback => format!(
            "{} uses a heuristic fallback with no cross-file semantic guarantee.",
            surface_label(request)
        ),
        DegradedStateClass::DegradedPolicyNarrowed => format!(
            "{} is narrowed by policy and may only use admitted fallback lanes.",
            surface_label(request)
        ),
        _ => format!(
            "{} cannot claim live LSP semantics; syntax fallback is limited to the current file or workset disclosure.",
            surface_label(request)
        ),
    }
}

fn degraded_state_for(health_state: HealthState, lane_class: LaneClass) -> DegradedStateClass {
    match health_state {
        HealthState::Ready => DegradedStateClass::None,
        HealthState::Warming | HealthState::Degraded | HealthState::Unavailable => {
            if lane_class == LaneClass::RemoteOnly {
                DegradedStateClass::DegradedRemoteUnreachable
            } else {
                DegradedStateClass::DegradedProviderUnavailable
            }
        }
        HealthState::CachedOnly => DegradedStateClass::DegradedCachedFallback,
        HealthState::PolicyBlocked => DegradedStateClass::DegradedPolicyNarrowed,
        HealthState::CapabilityMissing => DegradedStateClass::DegradedProviderUnavailable,
        HealthState::CrashLoopQuarantined => DegradedStateClass::DegradedCrashLoopQuarantine,
    }
}

fn missing_lsp_provider_row(request: &RouterRequest) -> ProviderStackRow {
    ProviderStackRow {
        provider_id: format!("provider:lsp:missing:{}", sanitize_id(&request.language_id)),
        provider_display_label: "Language service (LSP)".into(),
        provider_kind: ProviderKind::LanguageServer,
        capability_class: request.request_context.requested_capability_class,
        support_class: SupportClass::Authoritative,
        precedence_band: PrecedenceBand::ProtocolCompatibility,
        locality_class: LocalityClass::LocalSidecar,
        health_state: HealthState::Unavailable,
        freshness_class: FreshnessClass::Unverified,
        fault_domain_id: FaultDomainId::SessionScopedExecutionHosts,
        restart_strike_count: 0,
        restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:01".into(),
        quarantine_ref: None,
        fallback_class: FallbackClass::ProtocolToText,
        summary: format!(
            "No language-server host is registered for {} in {}.",
            request.language_id, request.routing_context.subject_root_ref
        ),
    }
}

fn syntax_fallback_row(request: &RouterRequest) -> ProviderStackRow {
    let capability = request.request_context.requested_capability_class;
    ProviderStackRow {
        provider_id: format!(
            "provider:syntax:tree_sitter:{}",
            sanitize_id(&request.routing_context.subject_root_ref)
        ),
        provider_display_label: "Syntax / structure engine (fallback)".into(),
        provider_kind: ProviderKind::SyntaxParser,
        capability_class: capability,
        support_class: SupportClass::FallbackOnly,
        precedence_band: PrecedenceBand::HeuristicFallback,
        locality_class: LocalityClass::LocalInProcess,
        health_state: HealthState::Ready,
        freshness_class: FreshnessClass::AuthoritativeLive,
        fault_domain_id: FaultDomainId::ShellInteractionCore,
        restart_strike_count: 0,
        restart_budget_ref: "restart_budget:shell_interaction_core:parser:01".into(),
        quarantine_ref: None,
        fallback_class: FallbackClass::NoFallback,
        summary: format!(
            "Syntax fallback can provide explicit lower-authority {} output without claiming live LSP semantics.",
            capability.as_str()
        ),
    }
}

fn surface_label(request: &RouterRequest) -> &'static str {
    match request.request_context.requested_surface_class {
        SurfaceClass::Definition => "Definition",
        SurfaceClass::Reference => "References",
        SurfaceClass::Hover => "Hover",
        SurfaceClass::Rename => "Rename",
        SurfaceClass::Completion => "Completion",
        SurfaceClass::Formatting => "Formatting",
        SurfaceClass::CodeAction => "Code action",
        SurfaceClass::Diagnostic => "Diagnostics",
        SurfaceClass::SignatureHelp => "Signature help",
        SurfaceClass::InlineHint => "Inline hints",
        SurfaceClass::TestDiscovery => "Test discovery",
        SurfaceClass::TestRun => "Test run",
        SurfaceClass::DebugLaunch => "Debug launch",
        SurfaceClass::DebugAttach => "Debug attach",
        SurfaceClass::DebugSessionControl => "Debug session control",
        SurfaceClass::BuildTargetDiscovery => "Build target discovery",
        SurfaceClass::BuildDiagnostics => "Build diagnostics",
        SurfaceClass::FrameworkNavigation => "Framework navigation",
        SurfaceClass::FrameworkRunScaffold => "Framework run scaffold",
        SurfaceClass::NotebookContext => "Notebook context",
        SurfaceClass::AiAssistContext => "AI assist context",
    }
}

fn health_phrase(health_state: HealthState) -> &'static str {
    match health_state {
        HealthState::Ready => "ready",
        HealthState::Warming => "restarting or reconnecting",
        HealthState::Degraded => "degraded",
        HealthState::CachedOnly => "cached-only",
        HealthState::PolicyBlocked => "blocked by policy",
        HealthState::CapabilityMissing => "missing the requested capability",
        HealthState::CrashLoopQuarantined => "quarantined after a crash loop",
        HealthState::Unavailable => "unavailable",
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}
