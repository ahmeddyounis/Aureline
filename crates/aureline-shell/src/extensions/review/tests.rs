//! Fixture coverage for the shell extension mutation review surface.

use serde::Deserialize;

use aureline_extensions::{evaluate_install_review_alpha, PermissionScopeClass};
use aureline_extensions::{
    ActivationBudgetDisclosure, CapabilityClassClass, CapabilityClassDeltaClass,
    CapabilityClassDeltaEntry, CompatibilityLabel, CompatibilityLabelBlock,
    EffectivePermissionBaselineRecord, ExtensionReviewAlphaPacketRecord, InstallReviewActionClass,
    InstallReviewAlphaEvaluation, InstallReviewAlphaInput, InstallReviewAlphaPacketRecord,
    InstallReviewBoundaryTruth, InstallReviewDecisionClass, PermissionDeltaClass,
    PermissionDeltaEntry, PermissionManifestDeltaRecord, PublisherContinuityAlphaRecord,
    PublisherContinuityStateClass, PublisherTrustTierClass, ReConsentDecisionClass,
    ReConsentReasonClass, RedactionClass, ReviewActionClass, ReviewDecisionClass,
    ReviewDecisionReasonClass, ReviewDisclosureClass, ReviewMutationClass, SummaryFreshnessClass,
    PERMISSION_MANIFEST_DELTA_RECORD_KIND, PERMISSION_MANIFEST_SCHEMA_VERSION,
    PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND, REVIEW_ALPHA_SCHEMA_VERSION,
};
use aureline_install::InstallTopologyAlphaPacket;

use super::*;

#[derive(Debug, Deserialize)]
struct MutationReviewFixture {
    review_id: String,
    action_class: ExtensionMutationReviewActionClass,
    surface_class: ExtensionMutationReviewSurfaceClass,
    subject_ref: String,
    source_posture: ExtensionMutationReviewSourcePosture,
    state_plan: ExtensionMutationStatePlan,
    rendered_disclosures: Vec<ExtensionMutationReviewDisclosureClass>,
    review_event_refs: Vec<String>,
    expected_decision_class: ExtensionMutationReviewDecisionClass,
    expected_reason_class: ExtensionMutationReviewReasonClass,
}

#[derive(Debug, Deserialize)]
struct InstallReviewFixture {
    input: InstallReviewAlphaInput,
    extension_review: ExtensionReviewAlphaPacketRecord,
    effective_permission: EffectivePermissionBaselineRecord,
    boundary_truth: InstallReviewBoundaryTruth,
    compatibility: CompatibilityLabelBlock,
    activation_budget: ActivationBudgetDisclosure,
    install_topology_row_id: String,
    expected_decision_class: InstallReviewDecisionClass,
}

fn load_fixture(name: &str) -> MutationReviewFixture {
    let raw = match name {
        "install_primary_registry_ready" => include_str!(
            "../../../../../fixtures/extensions/m3/install_update_review/install_primary_registry_ready.json"
        ),
        "update_mirror_permission_widening_requires_review" => include_str!(
            "../../../../../fixtures/extensions/m3/install_update_review/update_mirror_permission_widening_requires_review.json"
        ),
        "disable_manual_import_preserves_state" => include_str!(
            "../../../../../fixtures/extensions/m3/install_update_review/disable_manual_import_preserves_state.json"
        ),
        "rollback_offline_bundle_ready" => include_str!(
            "../../../../../fixtures/extensions/m3/install_update_review/rollback_offline_bundle_ready.json"
        ),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).expect("fixture must deserialize")
}

fn load_install_review_fixture() -> InstallReviewFixture {
    serde_json::from_str(include_str!(
        "../../../../../fixtures/extensions/install_review_alpha/native_marketplace_package_lane.json"
    ))
    .expect("install review fixture must deserialize")
}

fn topology_packet() -> InstallTopologyAlphaPacket {
    serde_json::from_str(include_str!(
        "../../../../../fixtures/install/topology_alpha/install_topology_alpha_packet.json"
    ))
    .expect("install topology fixture must deserialize")
}

fn evaluated_install_packet(
    action_class: ExtensionMutationReviewActionClass,
    source_posture: &ExtensionMutationReviewSourcePosture,
) -> Option<InstallReviewAlphaPacketRecord> {
    let install_action_class = match action_class {
        ExtensionMutationReviewActionClass::Install => InstallReviewActionClass::Install,
        ExtensionMutationReviewActionClass::Update => InstallReviewActionClass::Update,
        ExtensionMutationReviewActionClass::Disable
        | ExtensionMutationReviewActionClass::Rollback => {
            return None;
        }
    };
    let review_action_class = match action_class {
        ExtensionMutationReviewActionClass::Install => ReviewActionClass::Install,
        ExtensionMutationReviewActionClass::Update => ReviewActionClass::Update,
        ExtensionMutationReviewActionClass::Disable => ReviewActionClass::Disable,
        ExtensionMutationReviewActionClass::Rollback => ReviewActionClass::Update,
    };

    let mut fixture = load_install_review_fixture();
    fixture.input.action_class = install_action_class;
    fixture.extension_review.action_class = review_action_class;
    fixture.boundary_truth.content_source_class = source_posture.content_source_class;
    fixture.boundary_truth.manifest_origin_source_class =
        source_posture.manifest_origin_source_class;
    fixture.boundary_truth.owner_origin_summary = source_posture.source_summary.clone();

    let topology = topology_packet();
    let row = topology
        .row_by_id(&fixture.install_topology_row_id)
        .expect("fixture must cite an install-topology row");
    let packet = evaluate_install_review_alpha(InstallReviewAlphaEvaluation {
        input: fixture.input,
        extension_review: &fixture.extension_review,
        effective_permission: &fixture.effective_permission,
        boundary_truth: fixture.boundary_truth,
        compatibility: fixture.compatibility,
        activation_budget: fixture.activation_budget,
        install_topology_row: row,
        decided_at: "2026-05-16T12:00:00Z",
    });

    assert_eq!(
        packet.decision_class, fixture.expected_decision_class,
        "shared install-review fixture must remain admitted before shell-level gating"
    );
    Some(packet)
}

fn extension_review_packet(
    action_class: ExtensionMutationReviewActionClass,
) -> ExtensionReviewAlphaPacketRecord {
    let (review_action, mutation_class, reason_class, decision_summary) = match action_class {
        ExtensionMutationReviewActionClass::Install => (
            ReviewActionClass::Install,
            ReviewMutationClass::InstallStateMutation,
            ReviewDecisionReasonClass::AdmittedAfterReview,
            "Admitted after extension review.",
        ),
        ExtensionMutationReviewActionClass::Update => (
            ReviewActionClass::Update,
            ReviewMutationClass::UpdateStateMutation,
            ReviewDecisionReasonClass::AdmittedAfterReview,
            "Admitted after extension update review.",
        ),
        ExtensionMutationReviewActionClass::Disable => (
            ReviewActionClass::Disable,
            ReviewMutationClass::DisableStateMutation,
            ReviewDecisionReasonClass::DisableReviewRequired,
            "Admitted after disable review.",
        ),
        ExtensionMutationReviewActionClass::Rollback => (
            ReviewActionClass::Update,
            ReviewMutationClass::UpdateStateMutation,
            ReviewDecisionReasonClass::AdmittedAfterReview,
            "Admitted after rollback candidate review.",
        ),
    };

    ExtensionReviewAlphaPacketRecord {
        record_kind: "extension_review_alpha_packet_record".to_string(),
        review_alpha_schema_version: REVIEW_ALPHA_SCHEMA_VERSION,
        review_id: "review_alpha:acme-labs/prose-helper:reviewed".to_string(),
        action_class: review_action,
        subject_ref: "acme-labs/prose-helper".to_string(),
        requested_version: Some("1.4.4".to_string()),
        current_version: Some("1.4.3".to_string()),
        manifest_baseline_ref: "manifest_baseline:acme-labs/prose-helper:1.4.4".to_string(),
        manifest_install_decision_ref:
            "manifest_install_decision:manifest_baseline:acme-labs/prose-helper:1.4.4".to_string(),
        effective_permission_summary_ref: "manifest_baseline:acme-labs/prose-helper:1.4.4"
            .to_string(),
        declared_permissions_digest: "sha256:declared-prose-helper".to_string(),
        publisher_continuity_ref: "publisher_continuity:acme-labs:2026-q2".to_string(),
        revocation_ref: "revocation:none:acme-labs/prose-helper".to_string(),
        policy_pack_application_refs: Vec::new(),
        capability_lifecycle_row_refs: vec![
            "capability_lifecycle:alpha.extension_review".to_string()
        ],
        claimed_capability_lifecycle_state: None,
        boundary_manifest_row_refs: vec!["extension_registry_mirror".to_string()],
        required_disclosures: vec![
            ReviewDisclosureClass::PublisherIdentity,
            ReviewDisclosureClass::PublisherContinuity,
            ReviewDisclosureClass::RevocationState,
            ReviewDisclosureClass::PolicyPackNarrowing,
            ReviewDisclosureClass::DeclaredEffectivePermissionDiff,
            ReviewDisclosureClass::RollbackCheckpoint,
            ReviewDisclosureClass::CapabilityLifecycle,
            ReviewDisclosureClass::BoundaryManifest,
        ],
        rendered_disclosures: vec![
            ReviewDisclosureClass::PublisherIdentity,
            ReviewDisclosureClass::PublisherContinuity,
            ReviewDisclosureClass::RevocationState,
            ReviewDisclosureClass::PolicyPackNarrowing,
            ReviewDisclosureClass::DeclaredEffectivePermissionDiff,
            ReviewDisclosureClass::RollbackCheckpoint,
            ReviewDisclosureClass::CapabilityLifecycle,
            ReviewDisclosureClass::BoundaryManifest,
        ],
        mutation_class,
        decision_class: ReviewDecisionClass::AdmitAfterReview,
        decision_reason_class: reason_class,
        decision_summary: decision_summary.to_string(),
        review_event_refs: vec!["extension_review.completed".to_string()],
        decided_at: "2026-05-16T12:00:00Z".to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn publisher_continuity() -> PublisherContinuityAlphaRecord {
    PublisherContinuityAlphaRecord {
        record_kind: PUBLISHER_CONTINUITY_ALPHA_RECORD_KIND.to_string(),
        review_alpha_schema_version: REVIEW_ALPHA_SCHEMA_VERSION,
        continuity_id: "publisher_continuity:acme-labs:2026-q2".to_string(),
        publisher_identity_ref: "publisher:acme-labs".to_string(),
        publisher_display_label: "Acme Labs".to_string(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        continuity_state_class: PublisherContinuityStateClass::Active,
        active_signing_key_refs: vec!["key:acme-labs:ed25519:2026-q2".to_string()],
        predecessor_publisher_ref: None,
        successor_publisher_ref: None,
        lineage_event_refs: vec!["publisher_active".to_string()],
        revocation_event_refs: Vec::new(),
        mirror_promotion_refs: vec!["mirror-continuity-public-registry-live".to_string()],
        private_registry_parity_assertion_refs: vec![
            "registry_parity:public:acme-labs/prose-helper".to_string(),
        ],
        freshness_class: SummaryFreshnessClass::AuthoritativeLive,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn widening_permission_delta() -> PermissionManifestDeltaRecord {
    PermissionManifestDeltaRecord {
        record_kind: PERMISSION_MANIFEST_DELTA_RECORD_KIND.to_string(),
        permission_manifest_schema_version: PERMISSION_MANIFEST_SCHEMA_VERSION,
        delta_id: "permission_manifest_delta:acme-labs/prose-helper:1.4.4-to-1.4.5".to_string(),
        extension_identity_ref: "acme-labs/prose-helper".to_string(),
        prior_manifest_ref: "permission_manifest:acme-labs/prose-helper:1.4.4".to_string(),
        prior_extension_version: "1.4.4".to_string(),
        next_manifest_ref: "permission_manifest:acme-labs/prose-helper:1.4.5".to_string(),
        next_extension_version: "1.4.5".to_string(),
        delta_entries: vec![PermissionDeltaEntry {
            capability_class_class: CapabilityClassClass::Network,
            scope_class: PermissionScopeClass::NetworkEgress,
            scope_target: "endpoint_class:public_docs_metadata".to_string(),
            delta_class: PermissionDeltaClass::ScopeAdded,
            prior_constraint: None,
            next_constraint: Some("public docs metadata only".to_string()),
            prior_rationale_label: None,
            next_rationale_label: Some(
                "Fetch signed docs metadata from public endpoints.".to_string(),
            ),
            delta_reason_label: "network egress scope added by the next manifest".to_string(),
        }],
        capability_class_deltas: vec![CapabilityClassDeltaEntry {
            capability_class_class: CapabilityClassClass::Network,
            prior_entry_count: 0,
            next_entry_count: 1,
            delta_class: CapabilityClassDeltaClass::CapabilityClassAdded,
            entries_added_count: 1,
            entries_removed_count: 0,
        }],
        widening_count: 1,
        narrowing_count: 0,
        rationale_only_changed_count: 0,
        re_consent_decision_class: ReConsentDecisionClass::ReConsentRequiredWidening,
        re_consent_reason_class: ReConsentReasonClass::WideningAddedNewScope,
        delta_summary: "Network egress scope added; re-consent required.".to_string(),
        computed_at: "2026-05-16T12:00:00Z".to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn permission_delta_for(
    action_class: ExtensionMutationReviewActionClass,
) -> Option<PermissionManifestDeltaRecord> {
    if matches!(action_class, ExtensionMutationReviewActionClass::Update) {
        Some(widening_permission_delta())
    } else {
        None
    }
}

fn project_fixture(name: &str) -> ExtensionMutationReviewSurface {
    let fixture = load_fixture(name);
    let install_packet = evaluated_install_packet(fixture.action_class, &fixture.source_posture);
    let extension_review = extension_review_packet(fixture.action_class);
    let continuity = publisher_continuity();
    let permission_delta = permission_delta_for(fixture.action_class);

    let surface = project_extension_mutation_review_surface(ExtensionMutationReviewInput {
        review_id: &fixture.review_id,
        action_class: fixture.action_class,
        surface_class: fixture.surface_class,
        subject_ref: &fixture.subject_ref,
        source_posture: fixture.source_posture,
        install_review_packet: install_packet.as_ref(),
        extension_review_packet: &extension_review,
        publisher_continuity: &continuity,
        permission_manifest_delta: permission_delta.as_ref(),
        state_plan: fixture.state_plan,
        rendered_disclosures: fixture.rendered_disclosures,
        review_event_refs: fixture.review_event_refs,
        decided_at: "2026-05-16T12:01:00Z",
    });

    assert_eq!(surface.decision_class, fixture.expected_decision_class);
    assert_eq!(surface.decision_reason_class, fixture.expected_reason_class);
    surface
}

#[test]
fn install_primary_registry_renders_permission_compatibility_budget_publisher_and_rollback_truth() {
    let surface = project_fixture("install_primary_registry_ready");

    assert!(surface.mutation_allowed);
    assert!(validate_extension_mutation_review_surface(&surface).is_empty());
    assert_eq!(
        surface.source_posture.manifest_origin_source_class,
        ManifestOriginSourceClass::PublicRegistry
    );
    assert_eq!(surface.publisher_identity_ref, "publisher:acme-labs");
    assert_eq!(surface.compatibility_label, Some(CompatibilityLabel::Exact));
    assert_eq!(
        surface.compatibility_range.as_deref(),
        Some(">=0.0.0-alpha <0.1.0")
    );
    assert_eq!(
        surface
            .activation_budget
            .as_ref()
            .map(|budget| budget.memory.as_str()),
        Some("rss<=64MiB")
    );
    assert!(!surface.effective_permission_delta_entries.is_empty());
    assert_eq!(
        surface.state_plan.rollback_implication_class,
        RollbackImplicationClass::NotRequiredFreshInstall
    );
    assert!(surface
        .offered_actions
        .contains(&ExtensionMutationReviewActionOfferClass::ApproveInstall));
}

#[test]
fn update_mirror_uses_same_vocabulary_and_blocks_widening_until_reconsent() {
    let surface = project_fixture("update_mirror_permission_widening_requires_review");

    assert!(!surface.mutation_allowed);
    assert!(validate_extension_mutation_review_surface(&surface).is_empty());
    assert_eq!(
        surface.source_posture.manifest_origin_source_class,
        ManifestOriginSourceClass::Mirror
    );
    assert_eq!(
        surface.required_disclosures,
        required_disclosures_for(ExtensionMutationReviewActionClass::Update)
    );
    assert!(surface.requires_re_consent);
    assert_eq!(surface.permission_widening_count, 1);
    assert_eq!(
        surface.re_consent_reason_class,
        Some(ReConsentReasonClass::WideningAddedNewScope)
    );
    assert!(surface.permission_delta_ref.is_some());
    assert!(!surface
        .offered_actions
        .contains(&ExtensionMutationReviewActionOfferClass::ApproveUpdate));
}

#[test]
fn disable_manual_import_preserves_user_state_and_explains_remaining_state() {
    let surface = project_fixture("disable_manual_import_preserves_state");

    assert!(surface.mutation_allowed);
    assert!(validate_extension_mutation_review_surface(&surface).is_empty());
    assert_eq!(
        surface.source_posture.manifest_origin_source_class,
        ManifestOriginSourceClass::VendoredLocal
    );
    assert_eq!(
        surface.state_plan.user_state_retention_class,
        UserStateRetentionClass::UserOwnedStatePreserved
    );
    assert_eq!(
        surface.state_plan.installed_artifact_disposition_class,
        InstalledArtifactDispositionClass::InstalledDisabled
    );
    assert_eq!(
        surface.state_plan.cache_disposition_class,
        CacheDispositionClass::CachePreserved
    );
    assert_eq!(
        surface.state_plan.revocation_disposition_class,
        RevocationDispositionClass::ActivationRevoked
    );
    assert!(!surface.state_plan.preserved_user_state_refs.is_empty());
    assert!(surface
        .offered_actions
        .contains(&ExtensionMutationReviewActionOfferClass::DisableExtension));
}

#[test]
fn rollback_offline_bundle_restores_last_known_good_and_keeps_audit_state() {
    let surface = project_fixture("rollback_offline_bundle_ready");

    assert!(surface.mutation_allowed);
    assert!(validate_extension_mutation_review_surface(&surface).is_empty());
    assert_eq!(
        surface.source_posture.manifest_origin_source_class,
        ManifestOriginSourceClass::OfflineBundle
    );
    assert_eq!(
        surface.state_plan.last_known_good_version.as_deref(),
        Some("1.4.4")
    );
    assert!(surface.state_plan.rollback_checkpoint_ref.is_some());
    assert_eq!(
        surface.state_plan.installed_artifact_disposition_class,
        InstalledArtifactDispositionClass::RestoredToLastKnownGood
    );
    assert_eq!(
        surface.state_plan.cache_disposition_class,
        CacheDispositionClass::CacheRetainedForRollback
    );
    assert!(!surface.state_plan.revoked_artifact_refs.is_empty());
    assert!(surface
        .offered_actions
        .contains(&ExtensionMutationReviewActionOfferClass::ApplyRollback));
}

#[test]
fn rollback_missing_checkpoint_is_denied_before_mutation() {
    let mut fixture = load_fixture("rollback_offline_bundle_ready");
    fixture.state_plan.rollback_checkpoint_ref = None;
    let extension_review = extension_review_packet(fixture.action_class);
    let continuity = publisher_continuity();

    let surface = project_extension_mutation_review_surface(ExtensionMutationReviewInput {
        review_id: &fixture.review_id,
        action_class: fixture.action_class,
        surface_class: fixture.surface_class,
        subject_ref: &fixture.subject_ref,
        source_posture: fixture.source_posture,
        install_review_packet: None,
        extension_review_packet: &extension_review,
        publisher_continuity: &continuity,
        permission_manifest_delta: None,
        state_plan: fixture.state_plan,
        rendered_disclosures: fixture.rendered_disclosures,
        review_event_refs: fixture.review_event_refs,
        decided_at: "2026-05-16T12:02:00Z",
    });

    assert!(!surface.mutation_allowed);
    assert_eq!(
        surface.decision_reason_class,
        ExtensionMutationReviewReasonClass::MissingRollbackTruth
    );
    assert!(validate_extension_mutation_review_surface(&surface)
        .iter()
        .any(|finding| finding.check_id == "extension_mutation_review.rollback_truth_missing"));
}

#[test]
fn support_export_quotes_source_decision_and_state_vocabulary() {
    let surface = project_fixture("disable_manual_import_preserves_state");
    let export = project_extension_mutation_review_support_export(
        &surface,
        "extension_mutation_review_support_export:legacy-tools/native-helper:disable",
    );

    assert!(validate_extension_mutation_review_support_export(&export, &surface).is_empty());
    assert_eq!(export.review_ref, surface.review_id);
    assert_eq!(
        export.action_class,
        ExtensionMutationReviewActionClass::Disable
    );
    assert_eq!(
        export.manifest_origin_source_class,
        ManifestOriginSourceClass::VendoredLocal
    );
    assert_eq!(
        export.user_state_retention_class,
        UserStateRetentionClass::UserOwnedStatePreserved
    );
    assert_eq!(
        export.installed_artifact_disposition_class,
        InstalledArtifactDispositionClass::InstalledDisabled
    );
}
