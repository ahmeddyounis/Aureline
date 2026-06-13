use super::*;

const PACKET_ID: &str = "m5-memory-fence-fallback:stable:0001";

fn guarded_cache_spill(lifetime: &str) -> RetentionSpillGuard {
    RetentionSpillGuard {
        retention_class: RetentionClass::ContentKeyedBounded,
        content_key_bound: true,
        max_lifetime_label: Some(lifetime.to_owned()),
        telemetry_export_allowed: false,
        spill_state: SpillState::Guarded,
    }
}

fn durable_spill() -> RetentionSpillGuard {
    RetentionSpillGuard {
        retention_class: RetentionClass::DurableConsented,
        content_key_bound: false,
        max_lifetime_label: None,
        telemetry_export_allowed: false,
        spill_state: SpillState::Guarded,
    }
}

fn closed_fence(scope: RecallScope) -> MemoryFence {
    MemoryFence {
        recall_scope: scope,
        cross_workspace_state: FenceState::Fenced,
        cross_tenant_state: FenceState::Fenced,
        fence_visible: true,
        consent_ref: None,
    }
}

fn unfiltered() -> CachePolicyFilter {
    CachePolicyFilter {
        filter_state: FilterState::Unfiltered,
        narrowed_reason: None,
        narrowed_disclosure: None,
    }
}

fn rows() -> Vec<MemoryFenceFallbackRow> {
    vec![
        // 0: local prompt-result cache, guarded, primary fallback.
        MemoryFenceFallbackRow {
            row_id: "local-prompt-result-cache-guarded".to_owned(),
            profile: M5Profile::LocalOnly,
            artifact_class: MemoryArtifactClass::PromptResultCache,
            label_summary:
                "On-device prompt-result cache keyed by content hash with a bounded lifetime"
                    .to_owned(),
            spill_guard: guarded_cache_spill("24h"),
            fence: closed_fence(RecallScope::Thread),
            policy_filter: unfiltered(),
            fallback: RetrievalFallback {
                fallback_state: FallbackState::PrimaryAvailable,
                fallback_chain: vec![
                    FallbackHopKind::OfflineLocalPack,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: true,
                precise_label: None,
            },
            delete_posture: DeleteExportPosture::UserScoped,
            export_posture: DeleteExportPosture::UserScoped,
            consumer_surfaces: vec![
                FenceFallbackConsumerSurface::ComposerAssist,
                FenceFallbackConsumerSurface::CodeUnderstanding,
            ],
            degraded_label: None,
            evidence_refs: vec!["evidence:prompt-result-cache:m5".to_owned()],
            source_contract_refs: vec![
                MEMORY_FENCE_FALLBACK_MEMORY_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
        // 1: BYOK prompt-result cache, telemetry export refused (shadow store blocked).
        MemoryFenceFallbackRow {
            row_id: "byok-prompt-result-cache-telemetry-blocked".to_owned(),
            profile: M5Profile::ByokDirect,
            artifact_class: MemoryArtifactClass::PromptResultCache,
            label_summary: "BYOK prompt-result cache whose telemetry export attempt is refused"
                .to_owned(),
            spill_guard: RetentionSpillGuard {
                spill_state: SpillState::ShadowStoreBlocked,
                ..guarded_cache_spill("1h")
            },
            fence: closed_fence(RecallScope::Workspace),
            policy_filter: unfiltered(),
            fallback: RetrievalFallback {
                fallback_state: FallbackState::PrimaryAvailable,
                fallback_chain: vec![
                    FallbackHopKind::ByokDirect,
                    FallbackHopKind::CachedPriorResult,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: false,
                precise_label: None,
            },
            delete_posture: DeleteExportPosture::UserScoped,
            export_posture: DeleteExportPosture::UserScoped,
            consumer_surfaces: vec![FenceFallbackConsumerSurface::ComposerAssist],
            degraded_label: Some(
                "Prompt-result cache telemetry export refused: cache stays content-keyed and lifetime-bounded and is blocked from becoming a shadow-telemetry store"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:prompt-result-cache:m5".to_owned()],
            source_contract_refs: vec![
                MEMORY_FENCE_FALLBACK_DELETE_EXPORT_CONTRACT_REF.to_owned(),
            ],
        },
        // 2: managed reusable semantic memory, tenant-narrowed filter.
        MemoryFenceFallbackRow {
            row_id: "managed-semantic-memory-tenant-narrowed".to_owned(),
            profile: M5Profile::ManagedHosted,
            artifact_class: MemoryArtifactClass::ReusableSemanticMemory,
            label_summary:
                "Managed reusable semantic memory narrowed to the tenant by tenant isolation"
                    .to_owned(),
            spill_guard: durable_spill(),
            fence: closed_fence(RecallScope::Tenant),
            policy_filter: CachePolicyFilter {
                filter_state: FilterState::Narrowed,
                narrowed_reason: Some(NarrowReason::TenantIsolation),
                narrowed_disclosure: Some(
                    "Cross-tenant rows withheld: tenant isolation narrows recall to this tenant's reusable semantic memory"
                        .to_owned(),
                ),
            },
            fallback: RetrievalFallback {
                fallback_state: FallbackState::PrimaryAvailable,
                fallback_chain: vec![
                    FallbackHopKind::ManagedPrimary,
                    FallbackHopKind::WorkspaceMirror,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: false,
                precise_label: None,
            },
            delete_posture: DeleteExportPosture::TenantScoped,
            export_posture: DeleteExportPosture::TenantScoped,
            consumer_surfaces: vec![
                FenceFallbackConsumerSurface::DocsBrowserRecall,
                FenceFallbackConsumerSurface::SemanticSearch,
            ],
            degraded_label: Some(
                "Managed reusable semantic memory narrowed to this tenant; cross-tenant rows withheld by tenant isolation"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:reusable-semantic-memory:m5".to_owned()],
            source_contract_refs: vec![
                MEMORY_FENCE_FALLBACK_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
            ],
        },
        // 3: managed durable saved memory, region-blocked filter + policy-blocked fallback.
        MemoryFenceFallbackRow {
            row_id: "managed-saved-memory-region-blocked".to_owned(),
            profile: M5Profile::ManagedHosted,
            artifact_class: MemoryArtifactClass::DurableSavedMemory,
            label_summary: "Managed durable saved memory fully blocked by a region policy gate"
                .to_owned(),
            spill_guard: durable_spill(),
            fence: closed_fence(RecallScope::Org),
            policy_filter: CachePolicyFilter {
                filter_state: FilterState::FullyBlocked,
                narrowed_reason: Some(NarrowReason::RegionGate),
                narrowed_disclosure: Some(
                    "Recall fully blocked: a region policy gate forbids managed recall of saved memory in this region"
                        .to_owned(),
                ),
            },
            fallback: RetrievalFallback {
                fallback_state: FallbackState::PolicyBlockedDegraded,
                fallback_chain: vec![
                    FallbackHopKind::ManagedPrimary,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: false,
                precise_label: Some(
                    "Managed saved-memory recall blocked by region gate; degraded to policy-blocked with no managed lane available in this region"
                        .to_owned(),
                ),
            },
            delete_posture: DeleteExportPosture::OrgScoped,
            export_posture: DeleteExportPosture::OrgScoped,
            consumer_surfaces: vec![
                FenceFallbackConsumerSurface::SupportExport,
                FenceFallbackConsumerSurface::ManagedOfflineReport,
            ],
            degraded_label: Some(
                "Managed durable saved memory withheld: region policy gate blocks managed recall in this region; delete and export remain org-scoped"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:durable-saved-memory:m5".to_owned()],
            source_contract_refs: vec![
                MEMORY_FENCE_FALLBACK_DELETE_EXPORT_CONTRACT_REF.to_owned(),
            ],
        },
        // 4: offline-mirror evictable derived cache, mirror-served fallback.
        MemoryFenceFallbackRow {
            row_id: "offline-mirror-derived-cache-mirror-served".to_owned(),
            profile: M5Profile::OfflineMirror,
            artifact_class: MemoryArtifactClass::EvictableDerivedCache,
            label_summary: "Offline-mirror evictable derived cache served from the workspace mirror"
                .to_owned(),
            spill_guard: guarded_cache_spill("session"),
            fence: closed_fence(RecallScope::Workspace),
            policy_filter: unfiltered(),
            fallback: RetrievalFallback {
                fallback_state: FallbackState::MirrorServed,
                fallback_chain: vec![
                    FallbackHopKind::ManagedPrimary,
                    FallbackHopKind::WorkspaceMirror,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: true,
                precise_label: Some(
                    "Managed route unreachable offline; served from the workspace mirror and labeled mirror-served, not managed-current"
                        .to_owned(),
                ),
            },
            delete_posture: DeleteExportPosture::WorkspaceScoped,
            export_posture: DeleteExportPosture::WorkspaceScoped,
            consumer_surfaces: vec![
                FenceFallbackConsumerSurface::SemanticSearch,
                FenceFallbackConsumerSurface::ComposerAssist,
            ],
            degraded_label: Some(
                "Managed route unreachable offline; recall served from the workspace mirror and labeled mirror-served rather than current managed truth"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:evictable-derived-cache:m5".to_owned()],
            source_contract_refs: vec![MEMORY_FENCE_FALLBACK_RETRIEVAL_CONTRACT_REF.to_owned()],
        },
        // 5: hybrid reusable semantic memory, blocked cross-tenant + offline-local served.
        MemoryFenceFallbackRow {
            row_id: "hybrid-semantic-memory-offline-local-served".to_owned(),
            profile: M5Profile::HybridManaged,
            artifact_class: MemoryArtifactClass::ReusableSemanticMemory,
            label_summary:
                "Hybrid reusable semantic memory served from the on-device pack after a blocked cross-tenant attempt"
                    .to_owned(),
            spill_guard: durable_spill(),
            fence: MemoryFence {
                recall_scope: RecallScope::Tenant,
                cross_workspace_state: FenceState::Fenced,
                cross_tenant_state: FenceState::BreachBlocked,
                fence_visible: true,
                consent_ref: None,
            },
            policy_filter: unfiltered(),
            fallback: RetrievalFallback {
                fallback_state: FallbackState::OfflineLocalServed,
                fallback_chain: vec![
                    FallbackHopKind::ManagedPrimary,
                    FallbackHopKind::ByokDirect,
                    FallbackHopKind::OfflineLocalPack,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: true,
                precise_label: Some(
                    "Managed and BYOK legs unreachable; served from the on-device offline pack and labeled offline-local-served"
                        .to_owned(),
                ),
            },
            delete_posture: DeleteExportPosture::TenantScoped,
            export_posture: DeleteExportPosture::TenantScoped,
            consumer_surfaces: vec![
                FenceFallbackConsumerSurface::CodeUnderstanding,
                FenceFallbackConsumerSurface::ManagedOfflineReport,
            ],
            degraded_label: Some(
                "Cross-tenant recall attempt blocked at the tenant fence; managed and BYOK legs unreachable so recall served from the on-device offline pack and labeled offline-local-served"
                    .to_owned(),
            ),
            evidence_refs: vec!["evidence:reusable-semantic-memory:m5".to_owned()],
            source_contract_refs: vec![MEMORY_FENCE_FALLBACK_ROUTING_CONTRACT_REF.to_owned()],
        },
        // 6: local ephemeral turn state, guarded.
        MemoryFenceFallbackRow {
            row_id: "local-ephemeral-turn-state-guarded".to_owned(),
            profile: M5Profile::LocalOnly,
            artifact_class: MemoryArtifactClass::EphemeralTurnState,
            label_summary: "On-device ephemeral turn state dropped when the session ends".to_owned(),
            spill_guard: RetentionSpillGuard {
                retention_class: RetentionClass::SessionScoped,
                content_key_bound: false,
                max_lifetime_label: None,
                telemetry_export_allowed: false,
                spill_state: SpillState::Guarded,
            },
            fence: closed_fence(RecallScope::Turn),
            policy_filter: unfiltered(),
            fallback: RetrievalFallback {
                fallback_state: FallbackState::PrimaryAvailable,
                fallback_chain: vec![
                    FallbackHopKind::OfflineLocalPack,
                    FallbackHopKind::NonAiTerminal,
                ],
                offline_safe: true,
                precise_label: None,
            },
            delete_posture: DeleteExportPosture::NotApplicable,
            export_posture: DeleteExportPosture::NotApplicable,
            consumer_surfaces: vec![FenceFallbackConsumerSurface::ComposerAssist],
            degraded_label: None,
            evidence_refs: vec!["evidence:ephemeral-turn-state:m5".to_owned()],
            source_contract_refs: vec![
                MEMORY_FENCE_FALLBACK_MEMORY_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn guardrails() -> FenceFallbackGuardrails {
    FenceFallbackGuardrails {
        no_cross_workspace_recall_by_default: true,
        no_cross_tenant_recall_by_default: true,
        prompt_result_caches_are_not_shadow_telemetry: true,
        caches_content_keyed_and_lifetime_bounded: true,
        policy_filtered_paths_disclose_narrowing: true,
        fallback_truth_visible_in_export: true,
        spend_route_failures_keep_precise_fallback: true,
        every_durable_artifact_declares_delete_export: true,
    }
}

fn consumer_projection() -> FenceFallbackConsumerProjection {
    FenceFallbackConsumerProjection {
        composer_shows_fence_and_fallback: true,
        docs_browser_shows_policy_narrowing: true,
        search_shows_retrieval_fallback_state: true,
        support_export_shows_retention_and_fence: true,
        managed_offline_shows_fallback_truth: true,
        blocked_or_degraded_lanes_labeled_below_current: true,
    }
}

fn proof_freshness() -> FenceFallbackProofFreshness {
    FenceFallbackProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-13T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MEMORY_FENCE_FALLBACK_SCHEMA_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_DOC_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_RECALL_MATRIX_CONTRACT_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_MEMORY_CLASS_CONTRACT_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_DELETE_EXPORT_CONTRACT_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_CONTEXT_ASSEMBLY_CONTRACT_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_RETRIEVAL_CONTRACT_REF.to_owned(),
        MEMORY_FENCE_FALLBACK_ROUTING_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> MemoryFenceFallbackPacket {
    MemoryFenceFallbackPacket::new(MemoryFenceFallbackPacketInput {
        packet_id: PACKET_ID.to_owned(),
        records_label: "Memory Fences, Spill Guards, Cache-Policy Filters, and Fallback Truth"
            .to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_profiles_and_required_classes_present() {
    let packet = packet();
    for profile in M5Profile::ALL {
        assert!(packet.represented_profiles().contains(&profile));
    }
    for class in [
        MemoryArtifactClass::PromptResultCache,
        MemoryArtifactClass::ReusableSemanticMemory,
        MemoryArtifactClass::DurableSavedMemory,
    ] {
        assert!(packet.represented_classes().contains(&class));
    }
}

#[test]
fn missing_profile_coverage_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.profile != M5Profile::OfflineMirror);
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::RequiredProfileCoverageMissing));
}

#[test]
fn missing_class_coverage_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.artifact_class != MemoryArtifactClass::DurableSavedMemory);
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::RequiredClassCoverageMissing));
}

#[test]
fn no_spill_blocked_case_fails() {
    let mut packet = packet();
    packet.rows[1].spill_guard.spill_state = SpillState::Guarded;
    packet.rows[1].degraded_label = None;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::SpillBlockedCaseMissing));
}

#[test]
fn no_narrowed_filter_case_fails() {
    let mut packet = packet();
    // Drop the only narrowing/blocking filters.
    packet.rows[2].policy_filter = unfiltered();
    packet.rows[2].degraded_label = None;
    packet.rows[3].policy_filter = unfiltered();
    // Keep row 3's fallback policy-blocked degraded so it still needs a label.
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::NarrowedFilterCaseMissing));
}

#[test]
fn no_offline_fallback_case_fails() {
    let mut packet = packet();
    packet.rows.retain(|row| {
        !matches!(
            row.fallback.fallback_state,
            FallbackState::MirrorServed | FallbackState::OfflineLocalServed
        )
    });
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::OfflineFallbackCaseMissing));
}

#[test]
fn no_policy_blocked_fallback_case_fails() {
    let mut packet = packet();
    packet.rows[3].fallback.fallback_state = FallbackState::PrimaryAvailable;
    packet.rows[3].fallback.precise_label = None;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::PolicyBlockedFallbackCaseMissing));
}

#[test]
fn unbounded_cache_fails() {
    let mut packet = packet();
    // A prompt-result cache that drops its content key is no longer guarded.
    packet.rows[0].spill_guard.content_key_bound = false;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::CacheUnboundedOrTelemetry));
}

#[test]
fn telemetry_exported_cache_fails() {
    let mut packet = packet();
    packet.rows[0].spill_guard.telemetry_export_allowed = true;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::CacheUnboundedOrTelemetry));
}

#[test]
fn cross_scope_fence_open_fails() {
    let mut packet = packet();
    // Cross-tenant crossing without a recorded consent is a default crossing.
    packet.rows[2].fence.cross_tenant_state = FenceState::ExplicitlyConsented;
    packet.rows[2].fence.consent_ref = None;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::CrossScopeFenceOpen));
}

#[test]
fn fence_invisible_fails() {
    let mut packet = packet();
    packet.rows[0].fence.fence_visible = false;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::CrossScopeFenceOpen));
}

#[test]
fn filter_without_disclosure_fails() {
    let mut packet = packet();
    packet.rows[2].policy_filter.narrowed_reason = None;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::FilterDisclosureMissing));
}

#[test]
fn fallback_chain_without_terminal_fails() {
    let mut packet = packet();
    packet.rows[0].fallback.fallback_chain = vec![FallbackHopKind::OfflineLocalPack];
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::FallbackChainNoTerminal));
}

#[test]
fn non_primary_fallback_without_label_fails() {
    let mut packet = packet();
    packet.rows[4].fallback.precise_label = None;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::FallbackLabelMissing));
}

#[test]
fn generic_fallback_label_fails() {
    let mut packet = packet();
    packet.rows[4].fallback.precise_label = Some("provider error".to_owned());
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::FallbackLabelMissing));
}

#[test]
fn offline_profile_not_offline_safe_fails() {
    let mut packet = packet();
    packet.rows[6].fallback.offline_safe = false;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::OfflineLaneNotOfflineSafe));
}

#[test]
fn durable_without_delete_export_fails() {
    let mut packet = packet();
    packet.rows[3].delete_posture = DeleteExportPosture::NotApplicable;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::DurableArtifactMissingDeleteExport));
}

#[test]
fn degraded_row_without_label_fails() {
    let mut packet = packet();
    // Row 5 has a tenant breach, so it needs a precise label.
    packet.rows[5].degraded_label = None;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::DegradedLabelMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::MissingSourceContracts));
}

#[test]
fn guardrails_incomplete_fails() {
    let mut packet = packet();
    packet
        .guardrails
        .prompt_result_caches_are_not_shadow_telemetry = false;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .blocked_or_degraded_lanes_labeled_below_current = false;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&MemoryFenceFallbackViolation::ProofFreshnessIncomplete));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: MemoryFenceFallbackPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(
        summary.contains("Memory Fences, Spill Guards, Cache-Policy Filters, and Fallback Truth")
    );
    assert!(summary.contains("shadow_store_blocked"));
    assert!(summary.contains("mirror_served"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_memory_fence_fallback_export()
        .expect("checked memory fence fallback export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}
