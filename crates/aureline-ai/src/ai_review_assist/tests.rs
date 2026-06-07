use super::*;

const PACKET_ID: &str = "ai-review-assist-truth:stable:0001";
const REVIEW_PACK_DIGEST: &str =
    "sha256:2222222222222222222222222222222222222222222222222222222222222222";

fn lineage(object_id: &str) -> AiReviewObjectLineage {
    AiReviewObjectLineage {
        object_id: object_id.to_owned(),
        evidence_packet_refs: vec![format!("evidence-packet:{object_id}")],
        export_lineage_refs: vec![format!("export:{object_id}")],
        export_safe: true,
    }
}

fn scope_selectors() -> Vec<ReviewScopeSelector> {
    vec![
        ReviewScopeSelector {
            selector_id: "scope:selected-diff:0001".to_owned(),
            scope_class: ReviewScopeClass::SelectedDiff,
            base_identity_ref: "git-base:main:abc123".to_owned(),
            head_identity_ref: "git-head:working-selected:def456".to_owned(),
            hosted_review_object_ref: None,
            review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
            freshness_class: ReviewScopeFreshnessClass::Current,
            rerun_action: ReviewScopeRerunActionClass::NotNeeded,
            material_diff_change_ref: None,
            lineage: lineage("scope:selected-diff:0001"),
        },
        ReviewScopeSelector {
            selector_id: "scope:uncommitted:0001".to_owned(),
            scope_class: ReviewScopeClass::UncommittedChanges,
            base_identity_ref: "git-base:main:abc123".to_owned(),
            head_identity_ref: "worktree-fingerprint:dirty:789".to_owned(),
            hosted_review_object_ref: None,
            review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
            freshness_class: ReviewScopeFreshnessClass::Current,
            rerun_action: ReviewScopeRerunActionClass::NotNeeded,
            material_diff_change_ref: None,
            lineage: lineage("scope:uncommitted:0001"),
        },
        ReviewScopeSelector {
            selector_id: "scope:hosted-review:0001".to_owned(),
            scope_class: ReviewScopeClass::HostedReviewObject,
            base_identity_ref: "provider-base:pull-request:1842:base".to_owned(),
            head_identity_ref: "provider-head:pull-request:1842:head".to_owned(),
            hosted_review_object_ref: Some("provider-review:github-enterprise:1842".to_owned()),
            review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
            freshness_class: ReviewScopeFreshnessClass::OutdatedDiffChanged,
            rerun_action: ReviewScopeRerunActionClass::RerunSameReviewPack,
            material_diff_change_ref: Some("diff-change:pull-request:1842:material".to_owned()),
            lineage: lineage("scope:hosted-review:0001"),
        },
    ]
}

fn hunk(id: &str) -> AffectedReviewHunk {
    AffectedReviewHunk {
        file_ref: format!("file-ref:{id}"),
        hunk_ref: format!("hunk-ref:{id}"),
        diff_fingerprint_ref: format!("diff-fingerprint:{id}"),
    }
}

fn findings() -> Vec<AiReviewFindingRow> {
    vec![
        AiReviewFindingRow {
            finding_id: "finding:selected-diff:0001".to_owned(),
            title: "Potential fallback serializer nil path".to_owned(),
            finding_class: AiReviewFindingClass::RiskOrBugConcern,
            severity_class: AiReviewSeverityClass::MediumAdvisory,
            confidence_class: AiReviewConfidenceClass::EvidenceBacked,
            scope_selector_id: "scope:selected-diff:0001".to_owned(),
            review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
            instruction_check_source: RepoInstructionCheckSourceClass::ReviewPackRequiredCheck,
            instruction_check_refs: vec!["review-pack-check:serde-fallback".to_owned()],
            affected_hunks: vec![hunk("selected-diff:0001")],
            lineage: lineage("finding:selected-diff:0001"),
            scope_freshness_class: ReviewScopeFreshnessClass::Current,
            resolution_state: AiReviewResolutionState::Published,
            publish_sheet_id: Some("publish-sheet:selected-diff:0001".to_owned()),
        },
        AiReviewFindingRow {
            finding_id: "finding:uncommitted:0001".to_owned(),
            title: "Missing retry branch coverage".to_owned(),
            finding_class: AiReviewFindingClass::MissingTestCoverage,
            severity_class: AiReviewSeverityClass::LowAdvisory,
            confidence_class: AiReviewConfidenceClass::Inferred,
            scope_selector_id: "scope:uncommitted:0001".to_owned(),
            review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
            instruction_check_source: RepoInstructionCheckSourceClass::RepoInstructionBundle,
            instruction_check_refs: vec!["repo-instruction:review-tests-required".to_owned()],
            affected_hunks: vec![hunk("uncommitted:0001")],
            lineage: lineage("finding:uncommitted:0001"),
            scope_freshness_class: ReviewScopeFreshnessClass::Current,
            resolution_state: AiReviewResolutionState::Open,
            publish_sheet_id: Some("publish-sheet:uncommitted:0001".to_owned()),
        },
        AiReviewFindingRow {
            finding_id: "finding:hosted-review:0001".to_owned(),
            title: "Hosted review finding is outdated after diff drift".to_owned(),
            finding_class: AiReviewFindingClass::RepoInstructionCheckFired,
            severity_class: AiReviewSeverityClass::HighAdvisory,
            confidence_class: AiReviewConfidenceClass::EvidenceBacked,
            scope_selector_id: "scope:hosted-review:0001".to_owned(),
            review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
            instruction_check_source: RepoInstructionCheckSourceClass::ProviderCheckMirror,
            instruction_check_refs: vec!["provider-check:required-review-pack".to_owned()],
            affected_hunks: vec![hunk("hosted-review:0001")],
            lineage: lineage("finding:hosted-review:0001"),
            scope_freshness_class: ReviewScopeFreshnessClass::OutdatedDiffChanged,
            resolution_state: AiReviewResolutionState::Outdated,
            publish_sheet_id: None,
        },
    ]
}

fn publish_sheets() -> Vec<PublishToReviewSheet> {
    vec![
        PublishToReviewSheet {
            publish_sheet_id: "publish-sheet:selected-diff:0001".to_owned(),
            finding_id: "finding:selected-diff:0001".to_owned(),
            provider_ref: "provider:github-enterprise".to_owned(),
            destination_class: PublishDestinationClass::ProviderThreadComment,
            destination_ref: "provider-thread:pull-request:1842:serializer".to_owned(),
            provider_write_access: ProviderWriteAccessClass::ProviderWriteAvailable,
            outbound_text_preview:
                "Potential nil path in the fallback serializer; verify the missing branch before merge."
                    .to_owned(),
            attribution_state: AttributionStateClass::PostedAsUserWithAiAssistDisclosed,
            redaction_note: RedactionNoteClass::NoRedactionRequired,
            action_class: PublishActionClass::PublishToDestination,
            fallback_actions: vec![
                PublishActionClass::CopyOnly,
                PublishActionClass::ExportLocalPacket,
                PublishActionClass::KeepLocal,
            ],
            lineage: lineage("publish-sheet:selected-diff:0001"),
        },
        PublishToReviewSheet {
            publish_sheet_id: "publish-sheet:uncommitted:0001".to_owned(),
            finding_id: "finding:uncommitted:0001".to_owned(),
            provider_ref: "provider:github-enterprise".to_owned(),
            destination_class: PublishDestinationClass::LocalOnly,
            destination_ref: "local-review-workspace-anchor:retry-coverage".to_owned(),
            provider_write_access: ProviderWriteAccessClass::MissingProviderWriteAccess,
            outbound_text_preview:
                "Missing retry coverage remains local because provider write access is unavailable."
                    .to_owned(),
            attribution_state: AttributionStateClass::KeptLocalNoAttribution,
            redaction_note: RedactionNoteClass::InternalIdentifierRedacted,
            action_class: PublishActionClass::BlockedProviderWriteMissing,
            fallback_actions: vec![
                PublishActionClass::CopyOnly,
                PublishActionClass::ExportLocalPacket,
                PublishActionClass::KeepLocal,
            ],
            lineage: lineage("publish-sheet:uncommitted:0001"),
        },
    ]
}

fn resolution_memory() -> Vec<ResolutionMemoryRow> {
    vec![
        ResolutionMemoryRow {
            resolution_id: "resolution:selected-diff:0001".to_owned(),
            finding_id: "finding:selected-diff:0001".to_owned(),
            state: AiReviewResolutionState::Published,
            actor_ref: "actor:user:reviewer".to_owned(),
            source_ref: "publish-sheet:selected-diff:0001".to_owned(),
            timestamp: "2026-06-07T04:00:00Z".to_owned(),
            reopen_lineage_refs: vec!["reopen:published:requires-user-action".to_owned()],
            predecessor_resolution_id: None,
            publish_sheet_id: Some("publish-sheet:selected-diff:0001".to_owned()),
            lineage: lineage("resolution:selected-diff:0001"),
        },
        ResolutionMemoryRow {
            resolution_id: "resolution:uncommitted:0001".to_owned(),
            finding_id: "finding:uncommitted:0001".to_owned(),
            state: AiReviewResolutionState::Open,
            actor_ref: "actor:user:reviewer".to_owned(),
            source_ref: "review-pack-check:tests".to_owned(),
            timestamp: "2026-06-07T04:00:00Z".to_owned(),
            reopen_lineage_refs: vec!["reopen:open:not-needed".to_owned()],
            predecessor_resolution_id: None,
            publish_sheet_id: None,
            lineage: lineage("resolution:uncommitted:0001"),
        },
        ResolutionMemoryRow {
            resolution_id: "resolution:hosted-review:0001".to_owned(),
            finding_id: "finding:hosted-review:0001".to_owned(),
            state: AiReviewResolutionState::Outdated,
            actor_ref: "actor:scope-freshness-watcher".to_owned(),
            source_ref: "diff-change:pull-request:1842:material".to_owned(),
            timestamp: "2026-06-07T04:00:00Z".to_owned(),
            reopen_lineage_refs: vec!["rerun:hosted-review:required".to_owned()],
            predecessor_resolution_id: Some("resolution:hosted-review:previous".to_owned()),
            publish_sheet_id: None,
            lineage: lineage("resolution:hosted-review:0001"),
        },
    ]
}

fn consumer_projections() -> Vec<AiReviewConsumerProjection> {
    AiReviewConsumerSurface::required_surfaces()
        .into_iter()
        .map(|surface| AiReviewConsumerProjection {
            surface,
            preserves_scope_identity: true,
            preserves_instruction_check_source: true,
            preserves_publish_destination_truth: true,
            preserves_resolution_memory: true,
            preserves_local_copy_export_fallback: true,
            export_ref: format!("projection:ai-review-assist:{}", surface.as_str()),
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_REVIEW_ASSIST_TRUTH_AI_DOC_REF.to_owned(),
        AI_REVIEW_ASSIST_REVIEW_PACK_CONTRACT_REF.to_owned(),
        AI_REVIEW_ASSIST_TRUTH_SCHEMA_REF.to_owned(),
    ]
}

fn packet() -> AiReviewAssistTruthPacket {
    AiReviewAssistTruthPacket::new(AiReviewAssistTruthPacketInput {
        packet_id: PACKET_ID.to_owned(),
        review_workspace_ref: "review-workspace:serializer-pr:1842".to_owned(),
        review_pack_digest_ref: REVIEW_PACK_DIGEST.to_owned(),
        display_label: "AI review assist and publish truth".to_owned(),
        scope_selectors: scope_selectors(),
        findings: findings(),
        publish_sheets: publish_sheets(),
        resolution_memory: resolution_memory(),
        consumer_projections: consumer_projections(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T04:00:00Z".to_owned(),
    })
}

#[test]
fn ai_review_assist_truth_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_required_scope_classes_must_be_present() {
    let mut packet = packet();
    packet
        .scope_selectors
        .retain(|scope| scope.scope_class != ReviewScopeClass::HostedReviewObject);

    assert!(packet
        .validate()
        .contains(&AiReviewAssistTruthViolation::RequiredScopeClassMissing));
}

#[test]
fn material_diff_change_downgrades_finding_to_outdated() {
    let mut packet = packet();
    packet.findings[2].resolution_state = AiReviewResolutionState::Open;
    packet.resolution_memory[2].state = AiReviewResolutionState::Open;

    assert!(packet
        .validate()
        .contains(&AiReviewAssistTruthViolation::MaterialDiffChangeNotDowngraded));
}

#[test]
fn missing_provider_write_access_forces_local_copy_export_fallback() {
    let mut packet = packet();
    packet.publish_sheets[1].destination_class = PublishDestinationClass::ProviderThreadComment;
    packet.publish_sheets[1].action_class = PublishActionClass::PublishToDestination;
    packet.publish_sheets[1].fallback_actions.clear();

    assert!(packet
        .validate()
        .contains(&AiReviewAssistTruthViolation::ProviderWriteMissingNotDowngraded));
}

#[test]
fn low_confidence_findings_cannot_publish_to_provider() {
    let mut packet = packet();
    packet.findings[0].confidence_class = AiReviewConfidenceClass::LowConfidence;

    assert!(packet
        .validate()
        .contains(&AiReviewAssistTruthViolation::UnsafeFindingPublishAllowed));
}

#[test]
fn consumer_projection_must_preserve_truth_across_surfaces() {
    let mut packet = packet();
    packet.consumer_projections[0].preserves_resolution_memory = false;

    assert!(packet
        .validate()
        .contains(&AiReviewAssistTruthViolation::ConsumerProjectionIncomplete));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_ai_review_assist_truth_export()
        .expect("checked AI review-assist export validates");

    assert_eq!(packet.packet_id, PACKET_ID);
}
