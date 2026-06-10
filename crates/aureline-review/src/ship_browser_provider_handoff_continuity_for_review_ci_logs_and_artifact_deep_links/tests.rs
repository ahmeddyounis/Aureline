use super::*;

const PACKET_ID: &str = "handoff-continuity:stable:0001";

fn target_rows() -> Vec<HandoffTargetRow> {
    vec![
        HandoffTargetRow {
            target_id: "target:review-thread".to_owned(),
            target_class: HandoffTargetClass::ReviewThread,
            target_label: "Review thread".to_owned(),
            supports_deep_link: true,
            supports_safe_preview: true,
            supports_provider_handoff: true,
            coverage_label: "Anchored deep link, safe preview, and provider handoff".to_owned(),
            disclosure_label: "Review thread with disclosed capabilities".to_owned(),
        },
        HandoffTargetRow {
            target_id: "target:ci-run".to_owned(),
            target_class: HandoffTargetClass::CiRun,
            target_label: "CI run".to_owned(),
            supports_deep_link: true,
            supports_safe_preview: false,
            supports_provider_handoff: true,
            coverage_label: "Anchored deep link and provider handoff; no safe preview".to_owned(),
            disclosure_label: "CI run with disclosed capabilities".to_owned(),
        },
        HandoffTargetRow {
            target_id: "target:artifact-deep-link".to_owned(),
            target_class: HandoffTargetClass::ArtifactDeepLink,
            target_label: "Artifact deep link".to_owned(),
            supports_deep_link: true,
            supports_safe_preview: true,
            supports_provider_handoff: false,
            coverage_label: "Anchored deep link and safe preview; no provider handoff".to_owned(),
            disclosure_label: "Artifact deep link with disclosed capabilities".to_owned(),
        },
        HandoffTargetRow {
            target_id: "target:provider-owned".to_owned(),
            target_class: HandoffTargetClass::UnknownTargetProviderOwned,
            target_label: "Provider-owned target".to_owned(),
            supports_deep_link: false,
            supports_safe_preview: false,
            supports_provider_handoff: false,
            coverage_label: "Provider-owned target with no disclosed capabilities".to_owned(),
            disclosure_label: "Unknown provider-owned target; capabilities not assumed".to_owned(),
        },
    ]
}

fn handoff_rows() -> Vec<HandoffRow> {
    vec![
        HandoffRow {
            handoff_id: "handoff:review-open-local".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            target_id: "target:review-thread".to_owned(),
            subject_label: "Review thread for feature/login dashboard".to_owned(),
            target_identity: HandoffTargetIdentity {
                destination_class: HandoffDestinationClass::InProductSurface,
                trust_class: TargetTrustClass::FirstPartyTrusted,
                host_label: "In-product review host".to_owned(),
                provider_label: "First-party review provider".to_owned(),
                identity_disclosed: true,
            },
            deep_link: DeepLinkDisclosure {
                link_class: DeepLinkClass::AnchoredDeepLink,
                freshness_class: HandoffFreshnessClass::FreshCurrentTruth,
                link_disclosed: true,
                link_label: "Anchored deep link to the current review thread".to_owned(),
            },
            safe_preview: SafePreviewDisclosure {
                preview_class: SafePreviewClass::SafePreviewReadOnly,
                preview_disclosed: true,
                preview_label: "Read-only review preview".to_owned(),
            },
            handoff_action: HandoffAction {
                action_kind: HandoffActionKind::OpenInProduct,
                action_disclosed: true,
                read_only: true,
                action_label: "Open review thread".to_owned(),
                handoff_ref: None,
            },
            blocked_class: HandoffBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:handoff-review-open-local:0001".to_owned(),
            attention_reasons: Vec::new(),
            review_summary: "In-product review open with an anchored, fresh deep link".to_owned(),
            source_contract_refs: vec![
                HANDOFF_CONTINUITY_REMOTE_PREVIEW_CONTRACT_REF.to_owned(),
                HANDOFF_CONTINUITY_TRUST_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
        HandoffRow {
            handoff_id: "handoff:ci-run-provider".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            target_id: "target:ci-run".to_owned(),
            subject_label: "CI run for feature/login dashboard".to_owned(),
            target_identity: HandoffTargetIdentity {
                destination_class: HandoffDestinationClass::ProviderWebSurface,
                trust_class: TargetTrustClass::ProviderVerified,
                host_label: "Provider CI host".to_owned(),
                provider_label: "Verified CI provider".to_owned(),
                identity_disclosed: true,
            },
            deep_link: DeepLinkDisclosure {
                link_class: DeepLinkClass::PathScopedLink,
                freshness_class: HandoffFreshnessClass::StalePriorTruth,
                link_disclosed: true,
                link_label: "Path-scoped link to the CI run from a prior base".to_owned(),
            },
            safe_preview: SafePreviewDisclosure {
                preview_class: SafePreviewClass::PreviewUnsupported,
                preview_disclosed: true,
                preview_label: "CI run target does not support an in-product preview".to_owned(),
            },
            handoff_action: HandoffAction {
                action_kind: HandoffActionKind::OpenInProviderHandoff,
                action_disclosed: true,
                read_only: false,
                action_label: "Open CI run in provider".to_owned(),
                handoff_ref: Some("handoff:ci-run-provider-route".to_owned()),
            },
            blocked_class: HandoffBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:handoff-ci-run-provider:0002".to_owned(),
            attention_reasons: vec![
                "CI run truth is from a prior base and may be stale".to_owned(),
                "CI run target does not support an in-product safe preview".to_owned(),
            ],
            review_summary: "Attributed provider handoff to a CI run with a disclosed stale base"
                .to_owned(),
            source_contract_refs: vec![
                HANDOFF_CONTINUITY_MERGE_QUEUE_CONTRACT_REF.to_owned(),
                HANDOFF_CONTINUITY_PIPELINE_CONTRACT_REF.to_owned(),
            ],
        },
        HandoffRow {
            handoff_id: "handoff:provider-artifact-blocked".to_owned(),
            durable_anchor_id: "anchor:review:0003".to_owned(),
            target_id: "target:provider-owned".to_owned(),
            subject_label: "Provider-owned artifact with no resolvable origin".to_owned(),
            target_identity: HandoffTargetIdentity {
                destination_class: HandoffDestinationClass::UnknownDestinationProviderOwned,
                trust_class: TargetTrustClass::UnknownTrustProviderOwned,
                host_label: "Provider-owned host".to_owned(),
                provider_label: "Unknown provider".to_owned(),
                identity_disclosed: true,
            },
            deep_link: DeepLinkDisclosure {
                link_class: DeepLinkClass::UnanchoredLink,
                freshness_class: HandoffFreshnessClass::UnknownFreshnessProviderOwned,
                link_disclosed: true,
                link_label: "Unanchored, provider-owned link with no durable target".to_owned(),
            },
            safe_preview: SafePreviewDisclosure {
                preview_class: SafePreviewClass::UnsafePreviewBlocked,
                preview_disclosed: true,
                preview_label: "Unsafe preview blocked from rendering".to_owned(),
            },
            handoff_action: HandoffAction {
                action_kind: HandoffActionKind::UnsupportedNoContinuity,
                action_disclosed: true,
                read_only: true,
                action_label: "Handoff unavailable; no durable continuity".to_owned(),
                handoff_ref: None,
            },
            blocked_class: HandoffBlockedClass::BlockedNoDurableAnchor,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:handoff-provider-artifact-blocked:0003".to_owned(),
            attention_reasons: vec![
                "Provider-owned destination and trust are not assumed".to_owned(),
                "Link is unanchored, so no durable continuity is available".to_owned(),
                "Preview is unsafe and blocked from rendering".to_owned(),
            ],
            review_summary: "Blocked handoff to a provider-owned target with no durable continuity"
                .to_owned(),
            source_contract_refs: vec![
                HANDOFF_CONTINUITY_PIPELINE_CONTRACT_REF.to_owned(),
                HANDOFF_CONTINUITY_TRUST_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn trust_review() -> HandoffContinuityTrustReview {
    HandoffContinuityTrustReview {
        target_identity_disclosed: true,
        target_trust_explicit: true,
        deep_link_disclosed: true,
        truth_freshness_disclosed: true,
        safe_preview_disclosed: true,
        handoff_action_disclosed: true,
        handoff_read_only_unless_attributed: true,
        every_handoff_anchored: true,
        every_action_attributable: true,
        no_hidden_write_scope: true,
        stale_truth_narrows_action: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> HandoffContinuityConsumerProjection {
    HandoffContinuityConsumerProjection {
        review_workspace_header_shows_anchor: true,
        merge_queue_panel_shows_freshness: true,
        pipeline_run_viewer_shows_target_identity: true,
        log_viewer_shows_safe_preview: true,
        artifact_browser_shows_deep_link: true,
        handoff_action_shows_trust: true,
        command_palette_shows_handoff_state: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        label_for_unqualified: true,
    }
}

fn proof_freshness() -> HandoffContinuityProofFreshness {
    HandoffContinuityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<HandoffContinuityDowngradeTrigger> {
    vec![
        HandoffContinuityDowngradeTrigger::ProofStale,
        HandoffContinuityDowngradeTrigger::HandoffAttributionMissing,
        HandoffContinuityDowngradeTrigger::TruthStale,
        HandoffContinuityDowngradeTrigger::DeepLinkUnanchored,
        HandoffContinuityDowngradeTrigger::TargetTrustUnknown,
        HandoffContinuityDowngradeTrigger::SafePreviewUnsupported,
        HandoffContinuityDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<HandoffContinuityConsumerSurface> {
    vec![
        HandoffContinuityConsumerSurface::ReviewWorkspaceHeader,
        HandoffContinuityConsumerSurface::MergeQueuePanel,
        HandoffContinuityConsumerSurface::PipelineRunViewer,
        HandoffContinuityConsumerSurface::LogViewer,
        HandoffContinuityConsumerSurface::ArtifactBrowser,
        HandoffContinuityConsumerSurface::HandoffAction,
        HandoffContinuityConsumerSurface::CliHeadless,
        HandoffContinuityConsumerSurface::SupportExport,
        HandoffContinuityConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        HANDOFF_CONTINUITY_SCHEMA_REF.to_owned(),
        HANDOFF_CONTINUITY_DOC_REF.to_owned(),
        HANDOFF_CONTINUITY_REMOTE_PREVIEW_CONTRACT_REF.to_owned(),
        HANDOFF_CONTINUITY_MERGE_QUEUE_CONTRACT_REF.to_owned(),
        HANDOFF_CONTINUITY_PIPELINE_CONTRACT_REF.to_owned(),
        HANDOFF_CONTINUITY_TRUST_CLASS_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> HandoffContinuityPacket {
    HandoffContinuityPacket::new(HandoffContinuityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Browser/provider handoff continuity for review, CI, logs, and artifacts"
            .to_owned(),
        target_rows: target_rows(),
        handoff_rows: handoff_rows(),
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
fn handoff_continuity_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_target_rows_fails() {
    let mut packet = packet();
    packet.target_rows.clear();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::TargetRowsMissing));
}

#[test]
fn incomplete_target_row_fails() {
    let mut packet = packet();
    packet.target_rows[0].target_label = String::new();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::TargetRowIncomplete));
}

#[test]
fn missing_handoff_rows_fails() {
    let mut packet = packet();
    packet.handoff_rows.clear();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::HandoffRowsMissing));
}

#[test]
fn incomplete_handoff_row_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].subject_label = String::new();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::HandoffRowIncomplete));
}

#[test]
fn orphan_target_reference_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].target_id = "target:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::OrphanTargetReference));
}

#[test]
fn undisclosed_target_identity_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].target_identity.identity_disclosed = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::TargetIdentityUndisclosed));
}

#[test]
fn empty_host_label_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].target_identity.host_label = String::new();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::TargetIdentityUndisclosed));
}

#[test]
fn undisclosed_deep_link_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].deep_link.link_disclosed = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::DeepLinkUndisclosed));
}

#[test]
fn undisclosed_safe_preview_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].safe_preview.preview_disclosed = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::SafePreviewUndisclosed));
}

#[test]
fn undisclosed_handoff_action_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].handoff_action.action_disclosed = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::HandoffActionUndisclosed));
}

#[test]
fn handoff_action_read_only_mismatch_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].handoff_action.read_only = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::HandoffActionReadOnlyMismatch));
}

#[test]
fn handoff_action_without_ref_fails() {
    let mut packet = packet();
    packet.handoff_rows[1].handoff_action.handoff_ref = None;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::HandoffRefMissing));
}

#[test]
fn unsupported_action_without_block_fails() {
    let mut packet = packet();
    packet.handoff_rows[2].blocked_class = HandoffBlockedClass::NotBlocked;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::UnsupportedHandoffNotBlocked));
}

#[test]
fn missing_attribution_fails() {
    let mut packet = packet();
    packet.handoff_rows[0].actor_attribution_label = String::new();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::AttributionMissing));
}

#[test]
fn missing_audit_row_ref_fails() {
    let mut packet = packet();
    packet.handoff_rows[1].audit_row_ref = String::new();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::AttributionMissing));
}

#[test]
fn attention_required_without_reason_fails() {
    let mut packet = packet();
    packet.handoff_rows[1].attention_reasons.clear();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::AttentionReasonMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.handoff_action_shows_trust = false;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&HandoffContinuityViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Handoff targets"));
    assert!(summary.contains("## Handoffs"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_handoff_continuity_export().expect("checked handoff continuity export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/stale_truth_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/untrusted_target_blocked.json"
        )),
    ] {
        let packet: HandoffContinuityPacket =
            serde_json::from_str(raw).expect("fixture parses as handoff continuity packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
