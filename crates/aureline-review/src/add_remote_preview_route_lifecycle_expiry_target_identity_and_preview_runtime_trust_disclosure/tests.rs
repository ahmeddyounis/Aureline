use super::*;

const PACKET_ID: &str = "remote-preview-route:stable:0001";

fn route_rows() -> Vec<RemotePreviewRouteRow> {
    vec![
        RemotePreviewRouteRow {
            route_id: "route:feature-login".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            target_identity_label: "Preview of feature/login vs base main".to_owned(),
            target_run_id: "run:feature-login".to_owned(),
            target_commit_label: "feature/login @ commit 4f2a1c".to_owned(),
            lifecycle_phase: RouteLifecyclePhase::Live,
            expiry: RouteExpiryDisclosure {
                expiry_state: RouteExpiryState::ActiveTimeBounded,
                ttl_seconds: 7200,
                expires_at_label: "Expires in 2 hours".to_owned(),
                auto_revoke_on_expiry: true,
            },
            host_identity: RouteHostIdentity {
                host_class: RouteHostClass::AurelineManagedHost,
                host_label: "Aureline-managed preview host".to_owned(),
                origin_disclosed: true,
            },
            preview_trust: PreviewRuntimeTrustDisclosure {
                trust_class: PreviewRuntimeTrustClass::SandboxedIsolated,
                network_egress: NetworkEgressClass::NoEgress,
                executes_untrusted_code: false,
                runtime_writes_disclosed: true,
                trust_disclosure_label: "Sandboxed runtime with no network egress".to_owned(),
            },
            mutation_mode: RouteMutationMode::PublishNow,
            blocked_class: RouteBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:route-feature-login:0001".to_owned(),
            attention_reasons: Vec::new(),
            review_summary: "Live time-bounded preview of the feature branch".to_owned(),
            approval_ticket_ref: Some("approval:route-feature-login".to_owned()),
            browser_handoff_ref: None,
            deferred_queue_ref: None,
            source_contract_refs: vec![
                REMOTE_PREVIEW_ROUTE_PREVIEW_ROUTE_CONTRACT_REF.to_owned(),
                REMOTE_PREVIEW_ROUTE_PIPELINE_RUN_CONTRACT_REF.to_owned(),
            ],
        },
        RemotePreviewRouteRow {
            route_id: "route:docs-site".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            target_identity_label: "Preview of docs/site vs base main".to_owned(),
            target_run_id: "run:docs-site".to_owned(),
            target_commit_label: "docs/site @ commit 9b7e02".to_owned(),
            lifecycle_phase: RouteLifecyclePhase::ExpiringSoon,
            expiry: RouteExpiryDisclosure {
                expiry_state: RouteExpiryState::ExpiringSoon,
                ttl_seconds: 600,
                expires_at_label: "Expires in 10 minutes".to_owned(),
                auto_revoke_on_expiry: true,
            },
            host_identity: RouteHostIdentity {
                host_class: RouteHostClass::ProviderHosted,
                host_label: "Provider-hosted preview environment".to_owned(),
                origin_disclosed: true,
            },
            preview_trust: PreviewRuntimeTrustDisclosure {
                trust_class: PreviewRuntimeTrustClass::SandboxedNetworkLimited,
                network_egress: NetworkEgressClass::EgressToNamedTargets,
                executes_untrusted_code: false,
                runtime_writes_disclosed: true,
                trust_disclosure_label: "Sandboxed runtime with named-target egress only"
                    .to_owned(),
            },
            mutation_mode: RouteMutationMode::OpenInProvider,
            blocked_class: RouteBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:route-docs-site:0002".to_owned(),
            attention_reasons: Vec::new(),
            review_summary: "Provider-hosted preview within its expiry warning window".to_owned(),
            approval_ticket_ref: None,
            browser_handoff_ref: Some("handoff:route-docs-site".to_owned()),
            deferred_queue_ref: None,
            source_contract_refs: vec![
                REMOTE_PREVIEW_ROUTE_PREVIEW_ROUTE_CONTRACT_REF.to_owned(),
                REMOTE_PREVIEW_ROUTE_BROWSER_RUNTIME_CONTRACT_REF.to_owned(),
            ],
        },
        RemotePreviewRouteRow {
            route_id: "route:third-party-embed".to_owned(),
            durable_anchor_id: "anchor:review:0003".to_owned(),
            target_identity_label: "Preview of embed/third-party vs base main".to_owned(),
            target_run_id: "run:third-party-embed".to_owned(),
            target_commit_label: "embed/third-party @ commit 1c44de".to_owned(),
            lifecycle_phase: RouteLifecyclePhase::Live,
            expiry: RouteExpiryDisclosure {
                expiry_state: RouteExpiryState::ActiveTimeBounded,
                ttl_seconds: 1800,
                expires_at_label: "Expires in 30 minutes".to_owned(),
                auto_revoke_on_expiry: true,
            },
            host_identity: RouteHostIdentity {
                host_class: RouteHostClass::SelfHostedTunnel,
                host_label: "Self-hosted tunnel to the developer machine".to_owned(),
                origin_disclosed: true,
            },
            preview_trust: PreviewRuntimeTrustDisclosure {
                trust_class: PreviewRuntimeTrustClass::UntrustedRemoteContent,
                network_egress: NetworkEgressClass::UnrestrictedEgress,
                executes_untrusted_code: true,
                runtime_writes_disclosed: true,
                trust_disclosure_label:
                    "Untrusted remote content with unrestricted egress; review before opening"
                        .to_owned(),
            },
            mutation_mode: RouteMutationMode::DeferredPublish,
            blocked_class: RouteBlockedClass::BlockedUntrustedRuntimeReviewRequired,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:route-third-party-embed:0003".to_owned(),
            attention_reasons: vec![
                "Preview serves untrusted remote content and executes untrusted code".to_owned(),
                "Runtime has unrestricted network egress and is blocked pending review".to_owned(),
            ],
            review_summary: "Route is blocked until the untrusted runtime is reviewed".to_owned(),
            approval_ticket_ref: None,
            browser_handoff_ref: None,
            deferred_queue_ref: Some("queue:route-third-party-embed".to_owned()),
            source_contract_refs: vec![
                REMOTE_PREVIEW_ROUTE_PREVIEW_ROUTE_CONTRACT_REF.to_owned(),
                REMOTE_PREVIEW_ROUTE_TRUST_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn lifecycle_event_rows() -> Vec<RouteLifecycleEventRow> {
    vec![
        RouteLifecycleEventRow {
            route_id: "route:feature-login".to_owned(),
            event_id: "event:feature-login:provisioned".to_owned(),
            event_label: "Provisioned the feature/login preview route".to_owned(),
            from_phase: RouteLifecyclePhase::Provisioning,
            to_phase: RouteLifecyclePhase::Live,
            event_kind: RouteLifecycleEventKind::WentLive,
            disclosure_label: "Route went live and is time-bounded".to_owned(),
        },
        RouteLifecycleEventRow {
            route_id: "route:docs-site".to_owned(),
            event_id: "event:docs-site:warning".to_owned(),
            event_label: "Docs preview route entered its expiry warning window".to_owned(),
            from_phase: RouteLifecyclePhase::Live,
            to_phase: RouteLifecyclePhase::ExpiringSoon,
            event_kind: RouteLifecycleEventKind::ExpiryWarning,
            disclosure_label: "Route will auto-revoke when it expires".to_owned(),
        },
        RouteLifecycleEventRow {
            route_id: "route:third-party-embed".to_owned(),
            event_id: "event:third-party-embed:provisioned".to_owned(),
            event_label: "Provisioned the third-party embed preview route".to_owned(),
            from_phase: RouteLifecyclePhase::Requested,
            to_phase: RouteLifecyclePhase::Live,
            event_kind: RouteLifecycleEventKind::Provisioned,
            disclosure_label: "Route is live but blocked pending untrusted-runtime review"
                .to_owned(),
        },
    ]
}

fn trust_review() -> RemotePreviewRouteTrustReview {
    RemotePreviewRouteTrustReview {
        route_lifecycle_phase_explicit: true,
        every_route_time_bounded: true,
        expiry_auto_revoke_enforced: true,
        target_identity_explicit: true,
        host_identity_disclosed: true,
        preview_runtime_trust_disclosed: true,
        network_egress_disclosed: true,
        every_mutating_route_attributable: true,
        audit_row_recorded_for_every_route: true,
        mutation_mode_cites_required_grant: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> RemotePreviewRouteConsumerProjection {
    RemotePreviewRouteConsumerProjection {
        preview_panel_shows_lifecycle_phase: true,
        route_card_shows_expiry: true,
        route_card_shows_target_identity: true,
        route_card_shows_host_identity: true,
        route_lifecycle_sheet_shows_trust_disclosure: true,
        review_workspace_header_shows_attribution: true,
        command_palette_shows_route_state: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> RemotePreviewRouteProofFreshness {
    RemotePreviewRouteProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<RemotePreviewRouteDowngradeTrigger> {
    vec![
        RemotePreviewRouteDowngradeTrigger::ProofStale,
        RemotePreviewRouteDowngradeTrigger::RouteAttributionMissing,
        RemotePreviewRouteDowngradeTrigger::RouteExpiryUnbounded,
        RemotePreviewRouteDowngradeTrigger::RuntimeTrustUndisclosed,
        RemotePreviewRouteDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<RemotePreviewRouteConsumerSurface> {
    vec![
        RemotePreviewRouteConsumerSurface::PreviewPanel,
        RemotePreviewRouteConsumerSurface::RemotePreviewRouteCard,
        RemotePreviewRouteConsumerSurface::RouteLifecycleSheet,
        RemotePreviewRouteConsumerSurface::CliHeadless,
        RemotePreviewRouteConsumerSurface::SupportExport,
        RemotePreviewRouteConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REMOTE_PREVIEW_ROUTE_SCHEMA_REF.to_owned(),
        REMOTE_PREVIEW_ROUTE_DOC_REF.to_owned(),
        REMOTE_PREVIEW_ROUTE_PREVIEW_ROUTE_CONTRACT_REF.to_owned(),
        REMOTE_PREVIEW_ROUTE_BROWSER_RUNTIME_CONTRACT_REF.to_owned(),
        REMOTE_PREVIEW_ROUTE_PIPELINE_RUN_CONTRACT_REF.to_owned(),
        REMOTE_PREVIEW_ROUTE_TRUST_CLASS_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> RemotePreviewRoutePacket {
    RemotePreviewRoutePacket::new(RemotePreviewRoutePacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Remote preview routes with lifecycle, expiry, and trust disclosure"
            .to_owned(),
        route_rows: route_rows(),
        lifecycle_event_rows: lifecycle_event_rows(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn remote_preview_route_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_route_rows_fails() {
    let mut packet = packet();
    packet.route_rows.clear();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::RouteRowsMissing));
}

#[test]
fn missing_target_identity_fails() {
    let mut packet = packet();
    packet.route_rows[0].target_commit_label = String::new();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::TargetIdentityMissing));
}

#[test]
fn unbounded_expiry_without_block_fails() {
    let mut packet = packet();
    packet.route_rows[0].expiry.expiry_state = RouteExpiryState::NoExpiryUnbounded;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::ExpiryNotTimeBounded));
}

#[test]
fn time_bounded_without_auto_revoke_fails() {
    let mut packet = packet();
    packet.route_rows[0].expiry.auto_revoke_on_expiry = false;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::ExpiryDisclosureIncomplete));
}

#[test]
fn time_bounded_zero_ttl_fails() {
    let mut packet = packet();
    packet.route_rows[0].expiry.ttl_seconds = 0;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::ExpiryDisclosureIncomplete));
}

#[test]
fn undisclosed_host_origin_fails() {
    let mut packet = packet();
    packet.route_rows[0].host_identity.origin_disclosed = false;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::HostIdentityUndisclosed));
}

#[test]
fn undisclosed_runtime_writes_fails() {
    let mut packet = packet();
    packet.route_rows[0].preview_trust.runtime_writes_disclosed = false;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::RuntimeTrustUndisclosed));
}

#[test]
fn missing_attribution_fails() {
    let mut packet = packet();
    packet.route_rows[0].actor_attribution_label = String::new();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::AttributionMissing));
}

#[test]
fn missing_audit_row_ref_fails() {
    let mut packet = packet();
    packet.route_rows[1].audit_row_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::AttributionMissing));
}

#[test]
fn publish_now_without_approval_ref_fails() {
    let mut packet = packet();
    packet.route_rows[0].approval_ticket_ref = None;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::MutationGrantRefMissing));
}

#[test]
fn open_in_provider_without_handoff_ref_fails() {
    let mut packet = packet();
    packet.route_rows[1].browser_handoff_ref = None;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::MutationGrantRefMissing));
}

#[test]
fn deferred_publish_without_queue_ref_fails() {
    let mut packet = packet();
    packet.route_rows[2].deferred_queue_ref = None;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::MutationGrantRefMissing));
}

#[test]
fn blocked_route_without_attention_reason_fails() {
    let mut packet = packet();
    packet.route_rows[2].attention_reasons.clear();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::AttentionReasonMissing));
}

#[test]
fn route_without_lifecycle_event_fails() {
    let mut packet = packet();
    packet
        .lifecycle_event_rows
        .retain(|row| row.route_id != "route:feature-login");
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::RouteMissingLifecycleEvent));
}

#[test]
fn orphan_lifecycle_event_reference_fails() {
    let mut packet = packet();
    packet.lifecycle_event_rows[0].route_id = "route:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::OrphanEventReference));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.route_card_shows_expiry = false;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RemotePreviewRouteViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Routes"));
    assert!(summary.contains("## Lifecycle events"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_remote_preview_route_export()
        .expect("checked remote preview route export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/expired_route_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/unbounded_route_blocked.json"
        )),
    ] {
        let packet: RemotePreviewRoutePacket =
            serde_json::from_str(raw).expect("fixture parses as remote preview route packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
