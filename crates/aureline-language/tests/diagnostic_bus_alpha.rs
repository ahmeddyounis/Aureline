use std::path::Path;

use aureline_language::{
    DiagnosticAnchor, DiagnosticAnchorRemapStateClass, DiagnosticBus, DiagnosticBusAggregateCounts,
    DiagnosticBusSnapshot, DiagnosticBusSnapshotRequest, DiagnosticEnvelope,
    DiagnosticEvidencePlaneClass, DiagnosticEvidenceRef, DiagnosticEvidenceRoleClass,
    DiagnosticFreshness, DiagnosticFreshnessClass, DiagnosticOriginClass,
    DiagnosticProviderAvailabilityRow, DiagnosticScope, DiagnosticSeverityClass,
    DiagnosticSourceDescriptor, DiagnosticSourceFamily, DiagnosticSurfaceClass,
    LanguageServerHostIdentity, LanguageServerHostStatus, LspRouter, RouterCapabilityClass,
    RouterCompletenessClass, RouterFallbackClass, RouterFaultDomainId, RouterFreshnessClass,
    RouterHealthState, RouterLocalityClass, RouterProviderKind, RouterRequest,
    RouterScopeClaimClass, RouterSupportClass, RouterSurfaceClass, ScopeLimitClass,
    WorkspaceLocalRouterRequest, DIAGNOSTIC_BUS_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    workspace_id: String,
    collection_id: String,
    snapshot_id: String,
    root_ref: String,
    execution_context_id: String,
    captured_at: String,
    router_case: RouterCase,
    linter_provider: ProviderFixture,
    cases: Vec<DiagnosticCase>,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct RouterCase {
    language_id: String,
    server_label: String,
    provider_id: String,
    host_health: RouterHealthState,
    expected_degraded_provider_id: String,
}

#[derive(Debug, Deserialize)]
struct ProviderFixture {
    provider_id: String,
    provider_display_label: String,
    provider_kind: RouterProviderKind,
    support_class: RouterSupportClass,
    health_state: RouterHealthState,
    freshness_class: RouterFreshnessClass,
    scope_claim_class: RouterScopeClaimClass,
    completeness_class: RouterCompletenessClass,
    scope_limit_classes: Vec<ScopeLimitClass>,
    locality_class: RouterLocalityClass,
    fault_domain_id: RouterFaultDomainId,
    restart_strike_count: u32,
    quarantine_ref: Option<String>,
    router_decision_ref: Option<String>,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct DiagnosticCase {
    case_id: String,
    diagnostic_id: String,
    source_family: DiagnosticSourceFamily,
    evidence_plane_class: DiagnosticEvidencePlaneClass,
    origin_class: DiagnosticOriginClass,
    producer_ref: String,
    producer_version_ref: Option<String>,
    provider_id: Option<String>,
    router_host_ref: Option<String>,
    support_class: RouterSupportClass,
    locality_class: RouterLocalityClass,
    severity_class: DiagnosticSeverityClass,
    rule_id_ref: Option<String>,
    category_ref: Option<String>,
    freshness_class: DiagnosticFreshnessClass,
    epoch_ref: Option<String>,
    invalidation_ref: Option<String>,
    scope_claim_class: RouterScopeClaimClass,
    completeness_class: RouterCompletenessClass,
    scope_limit_classes: Vec<ScopeLimitClass>,
    target_ref: String,
    anchor_family_id: String,
    current_anchor_ref: Option<String>,
    path_ref: Option<String>,
    remap_state_class: DiagnosticAnchorRemapStateClass,
    expected_imported: bool,
    expected_cached: bool,
    expected_partial: bool,
    expected_inline: bool,
}

#[derive(Debug, Deserialize)]
struct Expected {
    total_count: usize,
    error_count: usize,
    warning_count: usize,
    notice_count: usize,
    hint_count: usize,
    local_count: usize,
    imported_count: usize,
    cached_count: usize,
    partial_count: usize,
    stale_or_unverified_count: usize,
    unavailable_provider_count: usize,
    editor_inline_count: usize,
    search_visible_count: usize,
    support_export_visible_count: usize,
    requires_degraded_disclosure: bool,
}

#[test]
fn diagnostic_bus_preserves_source_freshness_and_partiality_labels() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "diagnostic_bus_alpha_cases");
    assert_eq!(fixture.schema_version, DIAGNOSTIC_BUS_SCHEMA_VERSION);

    let mut bus = DiagnosticBus::new();
    let router_decision = route_degraded_language_server(&fixture);
    assert!(router_decision.requires_degraded_disclosure());
    assert!(router_decision
        .provider_stack_rows
        .iter()
        .any(
            |row| row.provider_id == fixture.router_case.expected_degraded_provider_id
                && row.health_state == RouterHealthState::CrashLoopQuarantined
        ));
    bus.ingest_router_decision(&router_decision);
    bus.ingest_provider_availability(provider_row(&fixture.linter_provider));

    for case in &fixture.cases {
        let diagnostic =
            diagnostic_from_case(&fixture, case, Some(&router_decision.router_decision_id));
        assert_eq!(
            diagnostic.is_imported(),
            case.expected_imported,
            "imported label mismatch for {}",
            case.case_id
        );
        assert_eq!(
            diagnostic.is_cached(),
            case.expected_cached,
            "cached label mismatch for {}",
            case.case_id
        );
        assert_eq!(
            diagnostic.is_partial(),
            case.expected_partial,
            "partial label mismatch for {}",
            case.case_id
        );
        assert_eq!(
            diagnostic.allows_editor_inline_projection(),
            case.expected_inline,
            "inline projection mismatch for {}",
            case.case_id
        );
        bus.publish(diagnostic);
    }

    let snapshot = bus.snapshot(DiagnosticBusSnapshotRequest {
        snapshot_id: fixture.snapshot_id.clone(),
        workspace_id: fixture.workspace_id.clone(),
        collection_id: fixture.collection_id.clone(),
        captured_at: fixture.captured_at.clone(),
    });

    assert_eq!(snapshot.record_kind, DiagnosticBusSnapshot::RECORD_KIND);
    assert_eq!(
        snapshot.aggregate_counts,
        DiagnosticBusAggregateCounts {
            total_count: fixture.expected.total_count,
            error_count: fixture.expected.error_count,
            warning_count: fixture.expected.warning_count,
            notice_count: fixture.expected.notice_count,
            hint_count: fixture.expected.hint_count,
            local_count: fixture.expected.local_count,
            imported_count: fixture.expected.imported_count,
            cached_count: fixture.expected.cached_count,
            partial_count: fixture.expected.partial_count,
            stale_or_unverified_count: fixture.expected.stale_or_unverified_count,
            unavailable_provider_count: fixture.expected.unavailable_provider_count,
        }
    );
    assert_eq!(
        snapshot.requires_degraded_disclosure(),
        fixture.expected.requires_degraded_disclosure
    );
    assert!(snapshot
        .provider_availability_rows
        .iter()
        .any(
            |row| row.source_family == DiagnosticSourceFamily::LanguageServer
                && row.health_state == RouterHealthState::CrashLoopQuarantined
        ));
    assert!(snapshot
        .provider_availability_rows
        .iter()
        .any(
            |row| row.source_family == DiagnosticSourceFamily::LinterFormatterStyle
                && row.health_state == RouterHealthState::Unavailable
        ));

    let editor_projection =
        snapshot.surface_projection(DiagnosticSurfaceClass::EditorInline, &fixture.captured_at);
    assert_eq!(
        editor_projection.visible_count,
        fixture.expected.editor_inline_count
    );
    assert_eq!(
        editor_projection.included_diagnostic_ids.len(),
        fixture.expected.total_count
    );
    assert_eq!(editor_projection.withheld_diagnostic_ids.len(), 1);
    assert!(editor_projection.disclosure_required);

    let search_projection =
        snapshot.surface_projection(DiagnosticSurfaceClass::SearchIndex, &fixture.captured_at);
    assert_eq!(
        search_projection.visible_count,
        fixture.expected.search_visible_count
    );
    assert_eq!(
        search_projection.included_diagnostic_ids,
        editor_projection.included_diagnostic_ids
    );

    let support_projection =
        snapshot.surface_projection(DiagnosticSurfaceClass::SupportExport, &fixture.captured_at);
    assert_eq!(
        support_projection.visible_count,
        fixture.expected.support_export_visible_count
    );
    assert_eq!(
        support_projection.provider_availability_refs.len(),
        snapshot.provider_availability_rows.len()
    );

    let serialized = serde_json::to_string(&snapshot).expect("snapshot serializes");
    let round_trip: DiagnosticBusSnapshot =
        serde_json::from_str(&serialized).expect("snapshot deserializes");
    assert_eq!(round_trip, snapshot);

    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/diagnostics/diagnostic_bus.schema.json");
    let schema_payload = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("read {schema_path:?}: {err}"));
    let schema: serde_json::Value = serde_json::from_str(&schema_payload)
        .unwrap_or_else(|err| panic!("parse {schema_path:?}: {err}"));
    assert_eq!(
        schema["$id"],
        "https://aureline.dev/schemas/diagnostics/diagnostic_bus.schema.json"
    );
}

fn route_degraded_language_server(fixture: &Fixture) -> aureline_language::RouterDecisionRecord {
    let router = LspRouter::new();
    let request = RouterRequest::workspace_local(WorkspaceLocalRouterRequest {
        language_id: fixture.router_case.language_id.clone(),
        surface_class: RouterSurfaceClass::Diagnostic,
        capability_class: RouterCapabilityClass::Diagnostics,
        requested_subject_ref: "subject:diagnostic-bus-alpha".into(),
        workspace_id: fixture.workspace_id.clone(),
        root_ref: fixture.root_ref.clone(),
        execution_context_id: fixture.execution_context_id.clone(),
        captured_at: fixture.captured_at.clone(),
    });
    let status = LanguageServerHostStatus {
        identity: LanguageServerHostIdentity {
            host_instance_id: "host:lsp:webapp:typescript".into(),
            provider_id: fixture.router_case.provider_id.clone(),
            workspace_id: fixture.workspace_id.clone(),
            root_ref: fixture.root_ref.clone(),
            language_id: fixture.router_case.language_id.clone(),
            server_label: fixture.router_case.server_label.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            locality_class: RouterLocalityClass::LocalSidecar,
            fault_domain_id: RouterFaultDomainId::SessionScopedExecutionHosts,
            restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:ts".into(),
        },
        health_state: fixture.router_case.host_health,
        freshness_class: RouterFreshnessClass::Unverified,
        scope_claim_class: RouterScopeClaimClass::ActiveWorkset,
        completeness_class: RouterCompletenessClass::UnavailableForClaimedScope,
        scope_limit_classes: Vec::new(),
        supported_capability_classes: vec![RouterCapabilityClass::Diagnostics],
        restart_strike_count: 3,
        quarantine_ref: Some("quarantine:lsp:typescript:diagnostic_bus".into()),
        fallback_class: RouterFallbackClass::LiveToCached,
        health_summary: "TypeScript language service is quarantined after repeated crashes.".into(),
    };
    router.route(request, &[status])
}

fn provider_row(provider: &ProviderFixture) -> DiagnosticProviderAvailabilityRow {
    DiagnosticProviderAvailabilityRow {
        provider_id: provider.provider_id.clone(),
        provider_display_label: provider.provider_display_label.clone(),
        source_family: DiagnosticSourceFamily::from_provider_kind(provider.provider_kind),
        provider_kind: provider.provider_kind,
        support_class: provider.support_class,
        health_state: provider.health_state,
        freshness_class: provider.freshness_class,
        scope_claim_class: provider.scope_claim_class,
        completeness_class: provider.completeness_class,
        scope_limit_classes: provider.scope_limit_classes.clone(),
        locality_class: provider.locality_class,
        fault_domain_id: provider.fault_domain_id,
        restart_strike_count: provider.restart_strike_count,
        quarantine_ref: provider.quarantine_ref.clone(),
        router_decision_ref: provider.router_decision_ref.clone(),
        summary: provider.summary.clone(),
    }
}

fn diagnostic_from_case(
    fixture: &Fixture,
    case: &DiagnosticCase,
    router_decision_ref: Option<&str>,
) -> DiagnosticEnvelope {
    let source = DiagnosticSourceDescriptor {
        source_descriptor_id: format!("source:{}:{}", case.source_family_token(), case.case_id),
        source_family: case.source_family,
        evidence_plane_class: case.evidence_plane_class,
        origin_class: case.origin_class,
        producer_ref: case.producer_ref.clone(),
        producer_version_ref: case.producer_version_ref.clone(),
        provider_id: case.provider_id.clone(),
        router_host_ref: case.router_host_ref.clone(),
        locality_class: case.locality_class,
        support_class: case.support_class,
        summary: format!(
            "{} source descriptor for diagnostic bus alpha.",
            case.case_id
        ),
    };
    DiagnosticEnvelope {
        record_kind: DiagnosticEnvelope::RECORD_KIND.into(),
        diagnostic_bus_schema_version: DIAGNOSTIC_BUS_SCHEMA_VERSION,
        diagnostic_id: case.diagnostic_id.clone(),
        collection_id: fixture.collection_id.clone(),
        workspace_id: fixture.workspace_id.clone(),
        source,
        severity_class: case.severity_class,
        rule_id_ref: case.rule_id_ref.clone(),
        category_ref: case.category_ref.clone(),
        freshness: DiagnosticFreshness {
            freshness_class: case.freshness_class,
            observed_at: fixture.captured_at.clone(),
            epoch_ref: case.epoch_ref.clone(),
            invalidation_ref: case.invalidation_ref.clone(),
            summary: format!("{} freshness is {:?}.", case.case_id, case.freshness_class),
        },
        scope: DiagnosticScope {
            scope_claim_class: case.scope_claim_class,
            completeness_class: case.completeness_class,
            scope_limit_classes: case.scope_limit_classes.clone(),
            target_ref: case.target_ref.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            summary: format!("{} scope is {:?}.", case.case_id, case.completeness_class),
        },
        anchor: DiagnosticAnchor {
            anchor_family_id: case.anchor_family_id.clone(),
            current_anchor_ref: case.current_anchor_ref.clone(),
            path_ref: case.path_ref.clone(),
            remap_state_class: case.remap_state_class,
            summary: format!(
                "{} anchor remap is {:?}.",
                case.case_id, case.remap_state_class
            ),
        },
        evidence_refs: vec![DiagnosticEvidenceRef {
            evidence_ref: format!("evidence:{}", case.case_id),
            evidence_role_class: DiagnosticEvidenceRoleClass::PrimarySource,
            summary: format!(
                "{} carries one export-safe source evidence ref.",
                case.case_id
            ),
        }],
        provider_status_refs: case.provider_id.iter().cloned().collect(),
        router_decision_ref: router_decision_ref.map(str::to_owned),
        redaction_class: aureline_language::RedactionClass::MetadataSafeDefault,
        captured_at: fixture.captured_at.clone(),
        export_safe_summary: format!(
            "{} diagnostic envelope preserves source labels.",
            case.case_id
        ),
    }
}

impl DiagnosticCase {
    fn source_family_token(&self) -> &'static str {
        match self.source_family {
            DiagnosticSourceFamily::EditorStructural => "editor_structural",
            DiagnosticSourceFamily::CompilerOrBuild => "compiler_or_build",
            DiagnosticSourceFamily::LanguageServer => "language_server",
            DiagnosticSourceFamily::LinterFormatterStyle => "linter_formatter_style",
            DiagnosticSourceFamily::FrameworkOrSchemaAnalyzer => "framework_or_schema_analyzer",
            DiagnosticSourceFamily::RuntimeTestOrDebug => "runtime_test_or_debug",
            DiagnosticSourceFamily::ScannerImport => "scanner_import",
            DiagnosticSourceFamily::PolicyTrustOrSecurity => "policy_trust_or_security",
            DiagnosticSourceFamily::ProjectGraph => "project_graph",
            DiagnosticSourceFamily::Heuristic => "heuristic",
        }
    }
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/diagnostic_bus_alpha/bus_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
