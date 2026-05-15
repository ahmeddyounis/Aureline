//! Unit and fixture coverage for the alpha extension review lane.

use super::*;
use crate::manifest_baseline::{
    DeclaredVsEffectiveDiffEntry, EffectivePermissionBaselineRecord, EffectivePermissionDiffClass,
    InstallDecisionClass, InstallDecisionReasonClass, ManifestInstallDecisionRecord,
    PermissionScopeClass, PermissionScopeEntry,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReviewFixture {
    input: ExtensionReviewAlphaInput,
    manifest_decision: ManifestInstallDecisionRecord,
    effective_permission: EffectivePermissionBaselineRecord,
    continuity: PublisherContinuityAlphaRecord,
    revocation: RevocationAlphaRecord,
    policy_applications: Vec<PolicyPackAlphaApplication>,
    expected_decision_class: ReviewDecisionClass,
    expected_reason_class: ReviewDecisionReasonClass,
}

fn load_fixture(name: &str) -> ReviewFixture {
    let raw = match name {
        "publisher_transfer_update" => include_str!(
            "../../../../fixtures/extensions/review_revocation_alpha/publisher_transfer_update.json"
        ),
        "revoked_mirror_install" => include_str!(
            "../../../../fixtures/extensions/review_revocation_alpha/revoked_mirror_install.json"
        ),
        "emergency_revoke_review" => include_str!(
            "../../../../fixtures/extensions/review_revocation_alpha/emergency_revoke_review.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).expect("fixture must deserialize")
}

fn manifest_decision(
    class: InstallDecisionClass,
    reason: InstallDecisionReasonClass,
) -> ManifestInstallDecisionRecord {
    ManifestInstallDecisionRecord {
        record_kind: crate::manifest_baseline::MANIFEST_INSTALL_DECISION_RECORD_KIND.to_string(),
        extension_manifest_baseline_schema_version:
            crate::manifest_baseline::EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: "manifest_baseline:acme-labs/prose-helper:1.4.4".to_string(),
        install_decision_class: class,
        install_decision_reason_class: reason,
        decision_summary: "test decision".to_string(),
        decided_at: "2026-05-14T08:00:00Z".to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn effective_permission(widening_count: u32) -> EffectivePermissionBaselineRecord {
    let mut declared_vs_effective_diff = vec![DeclaredVsEffectiveDiffEntry {
        scope_class: PermissionScopeClass::FilesystemRead,
        scope_target: "workspace:/docs/**".to_string(),
        diff_class: EffectivePermissionDiffClass::Unchanged,
        narrowing_reason_label: "unchanged".to_string(),
    }];
    if widening_count > 0 {
        declared_vs_effective_diff.push(DeclaredVsEffectiveDiffEntry {
            scope_class: PermissionScopeClass::FilesystemWrite,
            scope_target: "workspace:/secrets/**".to_string(),
            diff_class: EffectivePermissionDiffClass::WideningAttemptedBlocked,
            narrowing_reason_label: "declared scope did not include this scope; widening blocked"
                .to_string(),
        });
    }

    EffectivePermissionBaselineRecord {
        record_kind: crate::manifest_baseline::EFFECTIVE_PERMISSION_BASELINE_RECORD_KIND
            .to_string(),
        extension_manifest_baseline_schema_version:
            crate::manifest_baseline::EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_ref: "manifest_baseline:acme-labs/prose-helper:1.4.4".to_string(),
        extension_identity_ref: "acme-labs/prose-helper".to_string(),
        extension_version: "1.4.4".to_string(),
        effective_permissions: vec![PermissionScopeEntry {
            scope_class: PermissionScopeClass::FilesystemRead,
            scope_target: "workspace:/docs/**".to_string(),
            scope_constraint: Some("read-only under declared workspace prefix".to_string()),
            rationale_label: "Read prose documents for grammar suggestions.".to_string(),
        }],
        declared_vs_effective_diff,
        widening_attempted_blocked_count: widening_count,
        applied_policy_pack_refs: vec![],
        summary_freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        computed_at: "2026-05-14T08:00:00Z".to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn continuity(state: PublisherContinuityStateClass) -> PublisherContinuityAlphaRecord {
    PublisherContinuityAlphaRecord {
        record_kind: PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND.to_string(),
        review_alpha_schema_version: REVIEW_ALPHA_SCHEMA_VERSION,
        continuity_id: "publisher_continuity:acme-labs:2026-q2".to_string(),
        publisher_identity_ref: "publisher:acme-labs".to_string(),
        publisher_display_label: "Acme Labs".to_string(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        continuity_state_class: state,
        active_signing_key_refs: vec!["key:acme-labs:ed25519:2026-q2".to_string()],
        predecessor_publisher_ref: None,
        successor_publisher_ref: Some("publisher:acme-labs-next".to_string()),
        lineage_event_refs: vec!["publisher_transfer_completed".to_string()],
        revocation_event_refs: vec![],
        mirror_promotion_refs: vec!["mirror:public-registry-live".to_string()],
        private_registry_parity_assertion_refs: vec!["registry_parity:public:acme-labs".to_string()],
        freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn revocation(state: RevocationStateClass) -> RevocationAlphaRecord {
    RevocationAlphaRecord {
        record_kind: REVOCATION_ALPHA_RECORD_KIND.to_string(),
        review_alpha_schema_version: REVIEW_ALPHA_SCHEMA_VERSION,
        revocation_id: "revocation:none:acme-labs/prose-helper".to_string(),
        subject_class: RevocationSubjectClass::ExtensionArtifact,
        subject_ref: "acme-labs/prose-helper@1.4.4".to_string(),
        revocation_state_class: state,
        revocation_source_class: RevocationSourceClass::PublicRegistry,
        source_ref: "registry:public".to_string(),
        effective_at: "2026-05-14T08:00:00Z".to_string(),
        blast_radius_refs: vec!["installed:workspace:docs".to_string()],
        blocks_new_installs: !matches!(state, RevocationStateClass::NoKnownRevocation),
        blocks_updates: !matches!(state, RevocationStateClass::NoKnownRevocation),
        blocks_activation: !matches!(state, RevocationStateClass::NoKnownRevocation),
        rollback_or_repair_refs: vec!["repair:open-review".to_string()],
        audit_event_refs: if matches!(state, RevocationStateClass::NoKnownRevocation) {
            vec![]
        } else {
            vec!["audit:revocation-state".to_string()]
        },
        freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn input(action_class: ReviewActionClass) -> ExtensionReviewAlphaInput {
    ExtensionReviewAlphaInput {
        review_id: "review_alpha:acme-labs/prose-helper:1.4.4".to_string(),
        action_class,
        subject_ref: "acme-labs/prose-helper".to_string(),
        requested_version: Some("1.4.4".to_string()),
        current_version: Some("1.4.3".to_string()),
        declared_permissions_digest: "sha256:declared-prose-helper".to_string(),
        rendered_disclosures: required_disclosures_for(action_class),
        capability_lifecycle_row_refs: vec![
            "capability_lifecycle:alpha.extension_review".to_string()
        ],
        claimed_capability_lifecycle_state: None,
        boundary_manifest_row_refs: vec!["extension_registry_mirror".to_string()],
        review_event_refs: vec!["extension_install_review_opened".to_string()],
    }
}

#[test]
fn missing_required_disclosure_denies_before_any_mutation() {
    let mut input = input(ReviewActionClass::Install);
    input
        .rendered_disclosures
        .retain(|d| !matches!(d, ReviewDisclosureClass::PublisherContinuity));

    let packet = evaluate_extension_review_alpha(
        input,
        &manifest_decision(
            InstallDecisionClass::Admit,
            InstallDecisionReasonClass::AdmittedNoViolation,
        ),
        &effective_permission(0),
        &continuity(PublisherContinuityStateClass::Active),
        &revocation(RevocationStateClass::NoKnownRevocation),
        &[],
        "2026-05-14T08:00:01Z",
    );

    assert_eq!(packet.decision_class, ReviewDecisionClass::Denied);
    assert_eq!(
        packet.decision_reason_class,
        ReviewDecisionReasonClass::ReviewDisclosureIncomplete
    );
    assert_eq!(packet.mutation_class, ReviewMutationClass::NoMutation);
    assert!(validate_extension_review_alpha_packet(&packet)
        .iter()
        .any(|finding| finding.check_id == "review_alpha.packet.required_disclosure_missing"));
}

#[test]
fn widening_attempt_blocks_install_even_when_manifest_decision_is_admit() {
    let packet = evaluate_extension_review_alpha(
        input(ReviewActionClass::Install),
        &manifest_decision(
            InstallDecisionClass::Admit,
            InstallDecisionReasonClass::AdmittedNoViolation,
        ),
        &effective_permission(1),
        &continuity(PublisherContinuityStateClass::Active),
        &revocation(RevocationStateClass::NoKnownRevocation),
        &[],
        "2026-05-14T08:00:01Z",
    );

    assert_eq!(packet.decision_class, ReviewDecisionClass::Denied);
    assert_eq!(
        packet.decision_reason_class,
        ReviewDecisionReasonClass::EffectivePermissionWideningAttempted
    );
    assert_eq!(packet.mutation_class, ReviewMutationClass::NoMutation);
}

#[test]
fn stable_claim_on_preview_lifecycle_is_denied() {
    let mut input = input(ReviewActionClass::Install);
    input.capability_lifecycle_row_refs =
        vec!["capability_lifecycle:alpha.ai.routing_cost".to_string()];
    input.claimed_capability_lifecycle_state = Some(LifecycleState::Stable);

    let packet = evaluate_extension_review_alpha(
        input,
        &manifest_decision(
            InstallDecisionClass::Admit,
            InstallDecisionReasonClass::AdmittedNoViolation,
        ),
        &effective_permission(0),
        &continuity(PublisherContinuityStateClass::Active),
        &revocation(RevocationStateClass::NoKnownRevocation),
        &[],
        "2026-05-14T08:00:01Z",
    );

    assert_eq!(packet.decision_class, ReviewDecisionClass::Denied);
    assert_eq!(
        packet.decision_reason_class,
        ReviewDecisionReasonClass::CapabilityLifecycleClaimRefused
    );
    assert_eq!(packet.mutation_class, ReviewMutationClass::NoMutation);
}

#[test]
fn disable_flow_is_reviewed_and_admitted_without_relabeling_as_install() {
    let packet = evaluate_extension_review_alpha(
        input(ReviewActionClass::Disable),
        &manifest_decision(
            InstallDecisionClass::Admit,
            InstallDecisionReasonClass::AdmittedNoViolation,
        ),
        &effective_permission(0),
        &continuity(PublisherContinuityStateClass::Active),
        &revocation(RevocationStateClass::NoKnownRevocation),
        &[],
        "2026-05-14T08:00:01Z",
    );

    assert_eq!(packet.decision_class, ReviewDecisionClass::AdmitAfterReview);
    assert_eq!(
        packet.decision_reason_class,
        ReviewDecisionReasonClass::DisableReviewRequired
    );
    assert_eq!(
        packet.mutation_class,
        ReviewMutationClass::DisableStateMutation
    );
}

#[test]
fn fixture_matrix_replays_expected_review_decisions() {
    for fixture_name in [
        "publisher_transfer_update",
        "revoked_mirror_install",
        "emergency_revoke_review",
    ] {
        let fixture = load_fixture(fixture_name);
        assert!(
            validate_publisher_continuity_alpha_record(&fixture.continuity).is_empty(),
            "{fixture_name} continuity record must validate"
        );
        assert!(
            validate_revocation_alpha_record(&fixture.revocation).is_empty(),
            "{fixture_name} revocation record must validate"
        );

        let packet = evaluate_extension_review_alpha(
            fixture.input,
            &fixture.manifest_decision,
            &fixture.effective_permission,
            &fixture.continuity,
            &fixture.revocation,
            &fixture.policy_applications,
            "2026-05-14T09:00:00Z",
        );

        assert_eq!(packet.decision_class, fixture.expected_decision_class);
        assert_eq!(packet.decision_reason_class, fixture.expected_reason_class);

        let projection = project_review_alpha_surface(
            &packet,
            &fixture.continuity,
            &fixture.revocation,
            &fixture.policy_applications,
            ReviewSurfaceClass::CliHeadless,
        );
        assert_eq!(
            projection.visible_publisher_identity_ref,
            fixture.continuity.publisher_identity_ref
        );
        assert_eq!(
            projection.visible_publisher_continuity_state,
            fixture.continuity.continuity_state_class
        );
        assert_eq!(
            projection.visible_revocation_state,
            fixture.revocation.revocation_state_class
        );
        assert_eq!(
            projection.blocked_mutation,
            matches!(packet.decision_class, ReviewDecisionClass::Denied)
        );
    }
}
