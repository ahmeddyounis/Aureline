use super::*;

fn all_components() -> Vec<CacheKeyComponentClass> {
    vec![
        CacheKeyComponentClass::WorkspaceIdentity,
        CacheKeyComponentClass::RepoIdentity,
        CacheKeyComponentClass::FeatureClass,
        CacheKeyComponentClass::ProviderModelVersion,
        CacheKeyComponentClass::PromptPackVersion,
        CacheKeyComponentClass::ToolSchemaVersion,
        CacheKeyComponentClass::PolicyEpoch,
        CacheKeyComponentClass::GraphDocsEpoch,
        CacheKeyComponentClass::RetentionPosture,
    ]
}

fn all_reasons() -> Vec<InvalidationReasonCode> {
    vec![
        InvalidationReasonCode::WorkspaceIdentityChanged,
        InvalidationReasonCode::RepoIdentityChanged,
        InvalidationReasonCode::OrgTenantProfileChanged,
        InvalidationReasonCode::WorkspaceTrustChanged,
        InvalidationReasonCode::ProviderModelVersionChanged,
        InvalidationReasonCode::PromptPackChanged,
        InvalidationReasonCode::ToolSchemaChanged,
        InvalidationReasonCode::PolicyEpochRolled,
        InvalidationReasonCode::GraphDocsEpochChanged,
        InvalidationReasonCode::RetentionPostureChanged,
        InvalidationReasonCode::DeleteRequestReceived,
    ]
}

fn class_rows() -> Vec<MemoryStateClassRow> {
    vec![
        MemoryStateClassRow {
            state_class: AiStateClass::TurnState,
            label: "Turn state".to_owned(),
            scope_class: MemoryScopeClass::SessionWorkspace,
            durability_class: DurabilityClass::Ephemeral,
            retention_mode: RetentionModeClass::EphemeralUntilCompletion,
            sensitivity_tier: SensitivityTierClass::T2CodeBearingBounded,
            owner_policy: OwnerPolicyClass::ActiveUserSession,
            delete_posture: DeletePostureClass::NoDurableState,
            export_posture: ExportPostureClass::NotExportableEphemeral,
            invalidation_key_components: vec![],
            invalidation_reason_codes: vec![InvalidationReasonCode::DeleteRequestReceived],
            broader_reuse_requires_policy_label: true,
            raw_prompt_response_bodies_excluded_by_default: true,
            t3_reusable_memory_forbidden: true,
            disclosure_label: "Current turn only; no reusable memory.".to_owned(),
        },
        MemoryStateClassRow {
            state_class: AiStateClass::ConversationThread,
            label: "Conversation thread".to_owned(),
            scope_class: MemoryScopeClass::UserWorkspaceRepo,
            durability_class: DurabilityClass::LocalDurable,
            retention_mode: RetentionModeClass::LocalThreadUntilDelete,
            sensitivity_tier: SensitivityTierClass::T2CodeBearingBounded,
            owner_policy: OwnerPolicyClass::UserWorkspacePolicy,
            delete_posture: DeletePostureClass::DeleteThreadWithReceipt,
            export_posture: ExportPostureClass::UserVisibleConversation,
            invalidation_key_components: vec![CacheKeyComponentClass::PolicyEpoch],
            invalidation_reason_codes: vec![
                InvalidationReasonCode::PolicyEpochRolled,
                InvalidationReasonCode::DeleteRequestReceived,
            ],
            broader_reuse_requires_policy_label: true,
            raw_prompt_response_bodies_excluded_by_default: true,
            t3_reusable_memory_forbidden: true,
            disclosure_label: "User-visible local thread, separable from caches.".to_owned(),
        },
        MemoryStateClassRow {
            state_class: AiStateClass::PromptResultCache,
            label: "Prompt/result cache".to_owned(),
            scope_class: MemoryScopeClass::WorkspaceFeatureProviderModel,
            durability_class: DurabilityClass::TtlBoundedCache,
            retention_mode: RetentionModeClass::TtlBoundedUntilInvalidated,
            sensitivity_tier: SensitivityTierClass::T2CodeBearingBounded,
            owner_policy: OwnerPolicyClass::WorkspaceCachePolicy,
            delete_posture: DeletePostureClass::InvalidateCacheByKeyClass,
            export_posture: ExportPostureClass::InventoryAndHashesOnly,
            invalidation_key_components: all_components(),
            invalidation_reason_codes: all_reasons(),
            broader_reuse_requires_policy_label: true,
            raw_prompt_response_bodies_excluded_by_default: true,
            t3_reusable_memory_forbidden: true,
            disclosure_label: "TTL-bounded cache keyed by workspace and model identities."
                .to_owned(),
        },
        MemoryStateClassRow {
            state_class: AiStateClass::ReusableRepoFactsSummaries,
            label: "Reusable repo facts and summaries".to_owned(),
            scope_class: MemoryScopeClass::WorkspaceOrTenantRepoPolicy,
            durability_class: DurabilityClass::DerivedRegeneratable,
            retention_mode: RetentionModeClass::DerivedUntilInvalidated,
            sensitivity_tier: SensitivityTierClass::T1LowRiskDerived,
            owner_policy: OwnerPolicyClass::WorkspaceFactPolicy,
            delete_posture: DeletePostureClass::InvalidateCacheByKeyClass,
            export_posture: ExportPostureClass::ProvenanceLabeledSummary,
            invalidation_key_components: vec![
                CacheKeyComponentClass::WorkspaceIdentity,
                CacheKeyComponentClass::RepoIdentity,
                CacheKeyComponentClass::PolicyEpoch,
                CacheKeyComponentClass::GraphDocsEpoch,
            ],
            invalidation_reason_codes: vec![
                InvalidationReasonCode::WorkspaceIdentityChanged,
                InvalidationReasonCode::RepoIdentityChanged,
                InvalidationReasonCode::WorkspaceTrustChanged,
                InvalidationReasonCode::PolicyEpochRolled,
                InvalidationReasonCode::GraphDocsEpochChanged,
                InvalidationReasonCode::DeleteRequestReceived,
            ],
            broader_reuse_requires_policy_label: true,
            raw_prompt_response_bodies_excluded_by_default: true,
            t3_reusable_memory_forbidden: true,
            disclosure_label: "Derived from graph and docs epochs with provenance.".to_owned(),
        },
        MemoryStateClassRow {
            state_class: AiStateClass::RetainedEvidenceCopy,
            label: "Retained evidence copy".to_owned(),
            scope_class: MemoryScopeClass::ActionScoped,
            durability_class: DurabilityClass::EvidencePolicyRetained,
            retention_mode: RetentionModeClass::EvidencePolicyExpiryOrCaseClose,
            sensitivity_tier: SensitivityTierClass::T2CodeBearingBounded,
            owner_policy: OwnerPolicyClass::EvidenceRetentionPolicy,
            delete_posture: DeletePostureClass::EvidenceRetentionRules,
            export_posture: ExportPostureClass::EvidencePacket,
            invalidation_key_components: vec![CacheKeyComponentClass::PolicyEpoch],
            invalidation_reason_codes: vec![
                InvalidationReasonCode::PolicyEpochRolled,
                InvalidationReasonCode::DeleteRequestReceived,
            ],
            broader_reuse_requires_policy_label: true,
            raw_prompt_response_bodies_excluded_by_default: true,
            t3_reusable_memory_forbidden: true,
            disclosure_label: "Retained only as a reviewed evidence packet copy.".to_owned(),
        },
        MemoryStateClassRow {
            state_class: AiStateClass::ExplicitSavedMemory,
            label: "Explicit saved memory".to_owned(),
            scope_class: MemoryScopeClass::UserRepoOrgExplicit,
            durability_class: DurabilityClass::ExplicitlySaved,
            retention_mode: RetentionModeClass::UntilUserOrAdminRemoves,
            sensitivity_tier: SensitivityTierClass::T1LowRiskDerived,
            owner_policy: OwnerPolicyClass::ExplicitOwnerPolicy,
            delete_posture: DeletePostureClass::DeleteSavedMemoryWithReceipt,
            export_posture: ExportPostureClass::ExplicitSavedMemoryObject,
            invalidation_key_components: vec![
                CacheKeyComponentClass::WorkspaceIdentity,
                CacheKeyComponentClass::RepoIdentity,
                CacheKeyComponentClass::PolicyEpoch,
            ],
            invalidation_reason_codes: vec![
                InvalidationReasonCode::PolicyEpochRolled,
                InvalidationReasonCode::DeleteRequestReceived,
            ],
            broader_reuse_requires_policy_label: true,
            raw_prompt_response_bodies_excluded_by_default: true,
            t3_reusable_memory_forbidden: true,
            disclosure_label: "Saved only with explicit owner and policy labels.".to_owned(),
        },
    ]
}

fn surface_rows() -> Vec<MemorySurfaceProjectionRow> {
    MemorySurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| MemorySurfaceProjectionRow {
            surface_class,
            projection_ref: format!("surface:ai-memory:{}", surface_class.as_str()),
            uses_memory_class_vocabulary: true,
            shows_scope_chip: true,
            shows_provider_model: true,
            shows_retention_mode: true,
            shows_saved_memory_owner_policy: true,
            discloses_retained_evidence_copy: true,
            delete_export_reachable: true,
        })
        .collect()
}

fn fences() -> Vec<ReusableMemoryFence> {
    [
        AiStateClass::PromptResultCache,
        AiStateClass::ReusableRepoFactsSummaries,
        AiStateClass::ExplicitSavedMemory,
    ]
    .into_iter()
    .map(|state_class| ReusableMemoryFence {
        fence_id: format!("fence:ai-memory:{}", state_class.as_str()),
        state_class,
        denies_t3_secret_adjacent: true,
        denies_raw_terminal_transcripts: true,
        denies_credentials: true,
        denies_disallowed_path_contents: true,
        retained_copy_requires_reviewed_redaction_packet: true,
        policy_ref: "policy:ai-memory:stable".to_owned(),
    })
    .collect()
}

fn drills() -> Vec<DeleteExportDrillRow> {
    vec![
        DeleteExportDrillRow {
            drill_id: "drill:delete-thread".to_owned(),
            selected_classes: vec![
                AiStateClass::ConversationThread,
                AiStateClass::PromptResultCache,
            ],
            excluded_by_policy: vec![AiStateClass::RetainedEvidenceCopy],
            invalidates_matching_durable_caches: true,
            retained_copies_labeled: true,
            export_before_delete_available: true,
            receipt_or_omission_reason_emitted: true,
        },
        DeleteExportDrillRow {
            drill_id: "drill:delete-workspace-ai-state".to_owned(),
            selected_classes: vec![
                AiStateClass::ConversationThread,
                AiStateClass::PromptResultCache,
                AiStateClass::ReusableRepoFactsSummaries,
                AiStateClass::ExplicitSavedMemory,
            ],
            excluded_by_policy: vec![AiStateClass::RetainedEvidenceCopy],
            invalidates_matching_durable_caches: true,
            retained_copies_labeled: true,
            export_before_delete_available: true,
            receipt_or_omission_reason_emitted: true,
        },
    ]
}

fn packet() -> AiMemoryStatePacket {
    AiMemoryStatePacket::new(AiMemoryStatePacketInput {
        packet_id: "ai-memory-state:stable:0001".to_owned(),
        display_label: "AI memory state stable truth".to_owned(),
        workspace_identity_ref: "workspace:stable-memory:0001".to_owned(),
        repo_identity_ref: "repo:stable-memory:0001".to_owned(),
        profile_identity_ref: "profile:local-default".to_owned(),
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        provider_model_ref: "provider-model:local-review:2026.06".to_owned(),
        memory_classes: class_rows(),
        surface_projections: surface_rows(),
        durable_cache_key_contracts: vec![DurableCacheKeyContract {
            cache_contract_id: "cache-contract:prompt-result:stable:0001".to_owned(),
            state_class: AiStateClass::PromptResultCache,
            required_components: all_components(),
            invalidation_reason_codes: all_reasons(),
            workspace_repo_profile_scoped: true,
            delete_request_invalidates_matching_entries: true,
            support_export_hashes_only: true,
        }],
        reusable_memory_fences: fences(),
        delete_export_drills: drills(),
        support_manifest: SupportSafeMemoryManifest {
            manifest_ref: "support-export:ai-memory:stable:0001".to_owned(),
            conversation_history_inventory_ref: "inventory:conversation-history:hashes".to_owned(),
            reusable_facts_inventory_ref: "inventory:reusable-facts:hashes".to_owned(),
            retained_evidence_inventory_ref: "inventory:retained-evidence:refs".to_owned(),
            cache_inventory_hash_ref: "inventory:prompt-result-cache:hashes".to_owned(),
            raw_prompt_bodies_excluded: true,
            raw_response_bodies_excluded: true,
            raw_terminal_transcripts_excluded: true,
            credentials_excluded: true,
        },
        source_contract_refs: vec![
            AI_MEMORY_STATE_AI_DOC_REF.to_owned(),
            AI_MEMORY_STATE_MATRIX_REF.to_owned(),
            AI_MEMORY_STATE_SCHEMA_REF.to_owned(),
            AI_MEMORY_OBJECT_SCHEMA_REF.to_owned(),
            AI_MEMORY_RECONCILIATION_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-06T20:30:00Z".to_owned(),
    })
}

#[test]
fn stable_memory_state_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_memory_class_is_rejected() {
    let mut packet = packet();
    packet
        .memory_classes
        .retain(|row| row.state_class != AiStateClass::ExplicitSavedMemory);

    assert!(packet
        .validate()
        .contains(&AiMemoryStateViolation::MemoryClassCoverageMissing));
}

#[test]
fn durable_prompt_cache_requires_full_key_and_reason_codes() {
    let mut packet = packet();
    packet.durable_cache_key_contracts[0]
        .required_components
        .retain(|component| *component != CacheKeyComponentClass::PolicyEpoch);

    assert!(packet
        .validate()
        .contains(&AiMemoryStateViolation::DurableCacheKeyIncomplete));
}

#[test]
fn t3_material_cannot_be_reusable_memory() {
    let mut packet = packet();
    let row = packet
        .memory_classes
        .iter_mut()
        .find(|row| row.state_class == AiStateClass::PromptResultCache)
        .expect("prompt cache row");
    row.sensitivity_tier = SensitivityTierClass::T3SensitiveSecretAdjacent;

    assert!(packet
        .validate()
        .contains(&AiMemoryStateViolation::T3ReusableMemoryAllowed));
}

#[test]
fn hidden_cross_workspace_reuse_is_rejected() {
    let mut packet = packet();
    let row = packet
        .memory_classes
        .iter_mut()
        .find(|row| row.state_class == AiStateClass::ReusableRepoFactsSummaries)
        .expect("repo fact row");
    row.broader_reuse_requires_policy_label = false;

    assert!(packet
        .validate()
        .contains(&AiMemoryStateViolation::HiddenCrossWorkspaceReuse));
}

#[test]
fn support_manifest_must_exclude_raw_bodies() {
    let mut packet = packet();
    packet.support_manifest.raw_response_bodies_excluded = false;

    assert!(packet
        .validate()
        .contains(&AiMemoryStateViolation::SupportManifestUnsafe));
}

#[test]
fn checked_artifact_loads_and_validates() {
    let packet = current_stable_ai_memory_state_export().expect("checked export should validate");
    assert_eq!(packet.packet_id, "ai-memory-state:stable:0001");
    assert_eq!(packet.memory_classes.len(), 6);
}
