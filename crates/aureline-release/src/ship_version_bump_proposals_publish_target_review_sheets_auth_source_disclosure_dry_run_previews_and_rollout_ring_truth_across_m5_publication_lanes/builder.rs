//! Deterministic builder for the publication review-sheet register.
//!
//! [`build_publication_review_register`] constructs the same register that the
//! checked-in JSON embeds, so the headless emitter can regenerate the artifact
//! and a test can prove the embedded JSON never drifts from the code.

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::release_center_model::{
    AuthSourceClass, CompatibilityImpactClass, CompatibilityNote, ContinuityClass, ContinuityNote,
    DryRunAvailabilityClass, DryRunDisclosure, EvidenceFreshnessClass, EvidenceRef,
    PublishTargetClass, PublishTargetDescriptor, RolloutRing, SemanticChangeClass,
    TargetMutabilityClass, TargetVisibilityClass, VersionBumpProposal,
};
use crate::stable_claim_manifest::{FreshnessSlo, FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, StableClaimLevel,
};

use super::{
    AuthDisclosure, AuthDisclosureState, MigrationFlag, NarrowingReason, ParityState,
    PublicSurfaceImpact, PublicationReviewRegister, PublicationReviewSheet,
    PublicationReviewStopRule, PublicationReviewSummary, PublishTargetReview, ReviewParity,
    ReviewSheetState, StopAction, VersionBumpReview, PUBLICATION_REVIEW_RECORD_KIND,
    PUBLICATION_REVIEW_SCHEMA_VERSION,
};

const AS_OF: &str = "2026-06-15";
const SLO_REGISTER_REF: &str = "release/freshness_slo_register";
const TARGET_MAX_AGE_DAYS: u32 = 90;
const WARN_WITHIN_DAYS: u32 = 14;

/// Builds the canonical publication review-sheet register in code.
pub fn build_publication_review_register() -> PublicationReviewRegister {
    let rows = vec![
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::NotebookPack,
            slug: "notebook-pack",
            title: "Notebook pack publication review sheet",
            lane_summary: "Publishes notebook packs and notebook-derived outputs.",
            claim_label: StableClaimLevel::Stable,
            owner: "notebook-release",
            target_class: PublishTargetClass::Stable,
            visibility_class: TargetVisibilityClass::PublicStable,
            mutability_class: TargetMutabilityClass::ImmutableVersionMutablePointer,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            semantic_change_class: SemanticChangeClass::Minor,
            public_surface_impact: PublicSurfaceImpact::BackwardCompatible,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "channel/stable/notebook-pack",
            slo_state: FreshnessSloState::Current,
        }),
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::RequestDataAsset,
            slug: "request-data-asset",
            title: "Request/data asset publication review sheet",
            lane_summary: "Publishes saved requests, datasets, and request/data fixtures.",
            claim_label: StableClaimLevel::Stable,
            owner: "data-release",
            target_class: PublishTargetClass::Stable,
            visibility_class: TargetVisibilityClass::PublicStable,
            mutability_class: TargetMutabilityClass::ImmutableVersionMutablePointer,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            semantic_change_class: SemanticChangeClass::Patch,
            public_surface_impact: PublicSurfaceImpact::BackwardCompatible,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "channel/stable/request-data-asset",
            slo_state: FreshnessSloState::DueForRefresh,
        }),
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::ProfilerReplayArtifact,
            slug: "profiler-replay",
            title: "Profiler/replay artifact publication review sheet",
            lane_summary: "Publishes profiler traces and replay recordings to the mirror feed.",
            claim_label: StableClaimLevel::Stable,
            owner: "profiler-release",
            target_class: PublishTargetClass::MirrorFeed,
            visibility_class: TargetVisibilityClass::MirrorOnly,
            mutability_class: TargetMutabilityClass::MirrorSnapshotSupersedable,
            auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
            rollout_ring: RolloutRing::MirrorOnly,
            semantic_change_class: SemanticChangeClass::Patch,
            public_surface_impact: PublicSurfaceImpact::NoPublicChange,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "mirror/profiler-replay",
            slo_state: FreshnessSloState::Current,
        }),
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::FrameworkTemplatePack,
            slug: "framework-template-pack",
            title: "Framework/template pack publication review sheet",
            lane_summary: "Publishes framework and template packs to the registry.",
            claim_label: StableClaimLevel::Stable,
            owner: "framework-release",
            target_class: PublishTargetClass::RegistryMarketplace,
            visibility_class: TargetVisibilityClass::RegistryPublic,
            mutability_class: TargetMutabilityClass::RegistryVersionImmutableMetadataMutable,
            auth_source_class: AuthSourceClass::RegistryPublisherIdentity,
            rollout_ring: RolloutRing::Stable,
            semantic_change_class: SemanticChangeClass::Minor,
            public_surface_impact: PublicSurfaceImpact::MigrationRequired,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "registry/framework-template-pack",
            slo_state: FreshnessSloState::Current,
        }),
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::DocsPack,
            slug: "docs-pack",
            title: "Docs pack publication review sheet",
            lane_summary: "Publishes user-facing and embedded documentation packs.",
            claim_label: StableClaimLevel::Lts,
            owner: "docs-release",
            target_class: PublishTargetClass::Lts,
            visibility_class: TargetVisibilityClass::PublicLts,
            mutability_class: TargetMutabilityClass::ImmutableVersionMutablePointer,
            auth_source_class: AuthSourceClass::ReleaseVaultToken,
            rollout_ring: RolloutRing::Lts,
            semantic_change_class: SemanticChangeClass::Minor,
            public_surface_impact: PublicSurfaceImpact::BackwardCompatible,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "channel/lts/docs-pack",
            slo_state: FreshnessSloState::Current,
        }),
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::ModelPack,
            slug: "model-pack",
            title: "Model pack publication review sheet",
            lane_summary: "Publishes local model bundles and model metadata to the mirror feed.",
            claim_label: StableClaimLevel::Stable,
            owner: "model-release",
            target_class: PublishTargetClass::MirrorFeed,
            visibility_class: TargetVisibilityClass::MirrorOnly,
            mutability_class: TargetMutabilityClass::MirrorSnapshotSupersedable,
            auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
            rollout_ring: RolloutRing::MirrorOnly,
            semantic_change_class: SemanticChangeClass::Minor,
            public_surface_impact: PublicSurfaceImpact::BackwardCompatible,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "mirror/model-pack",
            slo_state: FreshnessSloState::DueForRefresh,
        }),
        cleared_sheet(ClearedSpec {
            family_kind: M5ArtifactFamilyKind::CompanionOffboardingPacket,
            slug: "companion-offboarding",
            title: "Companion/offboarding packet publication review sheet",
            lane_summary: "Publishes companion and offboarding packets.",
            claim_label: StableClaimLevel::Stable,
            owner: "companion-release",
            target_class: PublishTargetClass::Stable,
            visibility_class: TargetVisibilityClass::PublicStable,
            mutability_class: TargetMutabilityClass::ImmutableVersionMutablePointer,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            semantic_change_class: SemanticChangeClass::Patch,
            public_surface_impact: PublicSurfaceImpact::BackwardCompatible,
            prior_version: "0.9.0",
            target_version: "1.0.0",
            destination: "channel/stable/companion-offboarding",
            slo_state: FreshnessSloState::Current,
        }),
        managed_output_sheet(),
    ];

    let release_blocking_lane_refs = rows
        .iter()
        .filter(|r| r.release_blocking)
        .map(|r| r.lane_ref.clone())
        .collect();

    let mut register = PublicationReviewRegister {
        schema_version: PUBLICATION_REVIEW_SCHEMA_VERSION,
        record_kind: PUBLICATION_REVIEW_RECORD_KIND.to_owned(),
        manifest_id: "m5-publication-review-sheets".to_owned(),
        status: "frozen".to_owned(),
        overview_page:
            "docs/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.md"
                .to_owned(),
        as_of: AS_OF.to_owned(),
        claim_manifest_ref: "release/stable_claim_manifest".to_owned(),
        publication_matrix_ref:
            "release/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix"
                .to_owned(),
        release_center_model_ref: "release/release_center_object_model".to_owned(),
        lifecycle_labels: StableClaimLevel::ALL.to_vec(),
        lane_kinds: M5ArtifactFamilyKind::ALL.to_vec(),
        sheet_states: ReviewSheetState::ALL.to_vec(),
        public_surface_impacts: PublicSurfaceImpact::ALL.to_vec(),
        auth_disclosure_states: AuthDisclosureState::ALL.to_vec(),
        parity_states: ParityState::ALL.to_vec(),
        narrowing_reasons: NarrowingReason::ALL.to_vec(),
        stop_rule_actions: StopAction::ALL.to_vec(),
        launch_cutline: launch_cutline(),
        release_blocking_lane_refs,
        stop_rules: stop_rules(),
        rows,
        publication: PromotionDecisionRecord {
            promotion_gate: "m5_publication_review_sheets".to_owned(),
            decision: PromotionDecision::Proceed,
            blocking_rule_ids: Vec::new(),
            blocking_claim_ids: Vec::new(),
            rationale:
                "Every release-blocking lane at or above the cutline shares one descriptor and diff payload across human review and headless plan, discloses an explicit non-ambient auth source and target scope before mutation, rides a current dry-run preview, discloses its rollout ring, and records a rollback target; the managed-output lane already inherits a below-cutline claim, so it narrows without blocking the train."
                    .to_owned(),
        },
        summary: placeholder_summary(),
    };

    register.publication.decision = register.computed_publication_decision();
    register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
    register.publication.blocking_claim_ids = register.computed_blocking_entry_ids();
    register.summary = register.computed_summary();
    register
}

fn launch_cutline() -> LaunchCutline {
    LaunchCutline {
        cutline_level: StableClaimLevel::Stable,
        above_cutline_levels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        below_cutline_levels: StableClaimLevel::BELOW_CUTLINE.to_vec(),
        description:
            "A lane publishes at or above the cutline only when its version-bump impact is disclosed, its publish-target descriptor and diff payload are shared verbatim across human review and headless plan, its auth source and target scope are disclosed and never ambient before mutation, its dry-run preview is current, its rollout ring is disclosed, a rollback target is recorded, its proof packet is within SLO, and it is owner-signed; otherwise it narrows below stable."
                .to_owned(),
    }
}

fn stop_rules() -> Vec<PublicationReviewStopRule> {
    let rule = |id: &str,
                title: &str,
                trigger_reason: NarrowingReason,
                default_action: StopAction,
                rationale: &str| PublicationReviewStopRule {
        rule_id: id.to_owned(),
        title: title.to_owned(),
        trigger_reason,
        applies_to_labels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        default_action,
        blocks_publication: true,
        rationale: rationale.to_owned(),
    };
    vec![
        rule(
            "stop-version-impact-undisclosed",
            "Version impact undisclosed",
            NarrowingReason::VersionImpactUndisclosed,
            StopAction::DiscloseVersionImpact,
            "A version bump that hides migration or compatibility impact behind a version number narrows the lane.",
        ),
        rule(
            "stop-auth-source-undisclosed",
            "Auth source undisclosed",
            NarrowingReason::AuthSourceUndisclosed,
            StopAction::DiscloseAuthSource,
            "A publication that does not disclose its auth source and target scope before mutation narrows the lane.",
        ),
        rule(
            "stop-ambient-credential-inheritance",
            "Ambient credential inheritance",
            NarrowingReason::AmbientCredentialInheritance,
            StopAction::RebindNonAmbientAuth,
            "A publish flow that would inherit ambient credentials invisibly narrows the lane.",
        ),
        rule(
            "stop-dry-run-unavailable",
            "Dry-run preview unavailable",
            NarrowingReason::DryRunUnavailable,
            StopAction::RefreshDryRun,
            "A lane whose dry-run preview is missing, stale, or failed narrows below the cutline.",
        ),
        rule(
            "stop-descriptor-parity-broken",
            "Descriptor parity broken",
            NarrowingReason::DescriptorParityBroken,
            StopAction::ReconcileDescriptorParity,
            "Human review and headless publication must share the same publish-target descriptor.",
        ),
        rule(
            "stop-diff-payload-parity-broken",
            "Diff payload parity broken",
            NarrowingReason::DiffPayloadParityBroken,
            StopAction::ReconcileDiffPayload,
            "Human review and headless publication must share the same diff payload.",
        ),
        rule(
            "stop-rollout-ring-undisclosed",
            "Rollout ring undisclosed",
            NarrowingReason::RolloutRingUndisclosed,
            StopAction::DiscloseRolloutRing,
            "A lane that does not disclose its rollout ring narrows below the cutline.",
        ),
        rule(
            "stop-rollback-target-missing",
            "Rollback target missing",
            NarrowingReason::RollbackTargetMissing,
            StopAction::RecordRollbackTarget,
            "A lane without a recorded rollback target cannot be published as one reversible system.",
        ),
        rule(
            "stop-proof-packet-stale",
            "Proof packet stale",
            NarrowingReason::ProofPacketStale,
            StopAction::RefreshProofPacket,
            "A proof packet outside its freshness SLO narrows the lane.",
        ),
        rule(
            "stop-proof-packet-missing",
            "Proof packet missing",
            NarrowingReason::ProofPacketMissing,
            StopAction::RefreshProofPacket,
            "A lane without a captured proof packet narrows below the cutline.",
        ),
        rule(
            "stop-owner-signoff-missing",
            "Owner sign-off missing",
            NarrowingReason::OwnerManifestUnsigned,
            StopAction::RequestOwnerSignoff,
            "A lane without owner sign-off cannot hold its claimed label.",
        ),
        rule(
            "stop-waiver-expired",
            "Waiver expired",
            NarrowingReason::WaiverExpired,
            StopAction::RenewWaiver,
            "A lane relying on an expired waiver narrows below the cutline.",
        ),
    ]
}

struct ClearedSpec {
    family_kind: M5ArtifactFamilyKind,
    slug: &'static str,
    title: &'static str,
    lane_summary: &'static str,
    claim_label: StableClaimLevel,
    owner: &'static str,
    target_class: PublishTargetClass,
    visibility_class: TargetVisibilityClass,
    mutability_class: TargetMutabilityClass,
    auth_source_class: AuthSourceClass,
    rollout_ring: RolloutRing,
    semantic_change_class: SemanticChangeClass,
    public_surface_impact: PublicSurfaceImpact,
    prior_version: &'static str,
    target_version: &'static str,
    destination: &'static str,
    slo_state: FreshnessSloState,
}

fn cleared_sheet(spec: ClearedSpec) -> PublicationReviewSheet {
    let slug = spec.slug;
    let descriptor_digest = format!("sha256/descriptor/m5-{slug}");
    let diff_digest = format!("sha256/diff/m5-{slug}");
    let migration_flags = if spec.public_surface_impact.requires_migration_flags() {
        vec![MigrationFlag {
            flag_id: format!("migration-{slug}-state-epoch"),
            summary: "Run the one-shot state migration before consuming the new pack.".to_owned(),
            blocking: true,
            migration_ref: format!("migration/m5-{slug}/state-epoch"),
        }]
    } else {
        Vec::new()
    };
    PublicationReviewSheet {
        entry_id: format!("sheet-{slug}"),
        title: spec.title.to_owned(),
        lane_kind: spec.family_kind,
        lane_ref: format!("publication_lane/m5-{slug}"),
        lane_summary: spec.lane_summary.to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: spec.claim_label,
        sheet_state: ReviewSheetState::Cleared,
        version_bump: VersionBumpReview {
            proposal: version_bump_proposal(&spec),
            public_surface_impact: spec.public_surface_impact,
            impact_disclosed: true,
            impact_summary: format!(
                "{} → {}: {} public-surface impact, disclosed with compatibility notes.",
                spec.prior_version,
                spec.target_version,
                spec.public_surface_impact.as_str()
            ),
            migration_flags,
        },
        publish_target: PublishTargetReview {
            descriptor: cleared_descriptor(&spec),
            auth_disclosure: AuthDisclosure {
                state: AuthDisclosureState::ExplicitDisclosed,
                auth_source_ref: format!("auth_source/m5-{slug}"),
                disclosed_before_mutation: true,
                target_scope_disclosed: true,
                summary: "Auth source and target scope disclosed before any channel/mirror/registry mutation.".to_owned(),
            },
            rollout_ring_disclosed: true,
            mirror_destination_ref: spec.destination.to_owned(),
        },
        review_parity: ReviewParity {
            human_review_ref: format!("review/human/m5-{slug}"),
            headless_plan_ref: format!("plan/headless/m5-{slug}"),
            human_descriptor_digest: descriptor_digest.clone(),
            headless_descriptor_digest: descriptor_digest,
            diff_payload_ref: format!("diff/m5-{slug}"),
            human_diff_payload_digest: diff_digest.clone(),
            headless_diff_payload_digest: diff_digest,
            parity_state: ParityState::Matched,
        },
        proof_packet: proof_packet(slug, spec.slo_state),
        waiver: None,
        owner_signoff: signed(spec.owner),
        active_narrowing_reasons: Vec::new(),
        published_label: spec.claim_label,
        rationale:
            "Disclosed version-bump impact, shared descriptor and diff payload across review and plan, explicit non-ambient auth disclosed before mutation, current dry run, disclosed rollout ring, recorded rollback target, fresh proof packet, and owner sign-off; the lane holds its claimed label."
                .to_owned(),
        publication_destinations: vec![
            "release_center".to_owned(),
            "support_export".to_owned(),
            "diagnostics".to_owned(),
        ],
    }
}

fn managed_output_sheet() -> PublicationReviewSheet {
    let slug = "managed-output";
    let descriptor_digest_human = format!("sha256/descriptor/m5-{slug}/human");
    let descriptor_digest_headless = format!("sha256/descriptor/m5-{slug}/headless");
    let diff_digest = format!("sha256/diff/m5-{slug}");
    let spec = ClearedSpec {
        family_kind: M5ArtifactFamilyKind::ManagedOutput,
        slug,
        title: "Managed output publication review sheet",
        lane_summary: "Publishes managed outputs produced by managed/tenant-scoped lanes.",
        claim_label: StableClaimLevel::Beta,
        owner: "managed-release",
        target_class: PublishTargetClass::MirrorFeed,
        visibility_class: TargetVisibilityClass::MirrorOnly,
        mutability_class: TargetMutabilityClass::MirrorSnapshotSupersedable,
        auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
        rollout_ring: RolloutRing::MirrorOnly,
        semantic_change_class: SemanticChangeClass::Major,
        public_surface_impact: PublicSurfaceImpact::Breaking,
        prior_version: "0.9.0",
        target_version: "1.0.0",
        destination: "mirror/managed-output",
        slo_state: FreshnessSloState::Breached,
    };
    PublicationReviewSheet {
        entry_id: format!("sheet-{slug}"),
        title: spec.title.to_owned(),
        lane_kind: spec.family_kind,
        lane_ref: format!("publication_lane/m5-{slug}"),
        lane_summary: spec.lane_summary.to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: spec.claim_label,
        sheet_state: ReviewSheetState::ReviewGap,
        version_bump: VersionBumpReview {
            proposal: version_bump_proposal(&spec),
            public_surface_impact: spec.public_surface_impact,
            impact_disclosed: true,
            impact_summary:
                "0.9.0 → 1.0.0: breaking public-surface impact, disclosed with a blocking migration flag."
                    .to_owned(),
            migration_flags: vec![MigrationFlag {
                flag_id: format!("migration-{slug}-result-grid"),
                summary: "Re-export managed result grids after the breaking schema change.".to_owned(),
                blocking: true,
                migration_ref: format!("migration/m5-{slug}/result-grid"),
            }],
        },
        publish_target: PublishTargetReview {
            descriptor: PublishTargetDescriptor {
                rollback_target_ref: String::new(),
                dry_run: DryRunDisclosure {
                    availability_class: DryRunAvailabilityClass::Failed,
                    dry_run_ref: Some(format!("dry_run/m5-{slug}")),
                    scope_preview_ref: None,
                    generated_at: Some(AS_OF.to_owned()),
                    expires_at: None,
                    blocking_findings: vec![format!("finding/m5-{slug}/dry-run-failed")],
                },
                ..cleared_descriptor(&spec)
            },
            auth_disclosure: AuthDisclosure {
                state: AuthDisclosureState::AmbientInherited,
                auth_source_ref: format!("auth_source/m5-{slug}/ambient"),
                disclosed_before_mutation: true,
                target_scope_disclosed: true,
                summary:
                    "Publish flow would inherit the ambient mirror-operator session instead of an explicit receipt."
                        .to_owned(),
            },
            rollout_ring_disclosed: true,
            mirror_destination_ref: spec.destination.to_owned(),
        },
        review_parity: ReviewParity {
            human_review_ref: format!("review/human/m5-{slug}"),
            headless_plan_ref: format!("plan/headless/m5-{slug}"),
            human_descriptor_digest: descriptor_digest_human,
            headless_descriptor_digest: descriptor_digest_headless,
            diff_payload_ref: format!("diff/m5-{slug}"),
            human_diff_payload_digest: diff_digest.clone(),
            headless_diff_payload_digest: diff_digest,
            parity_state: ParityState::Divergent,
        },
        proof_packet: proof_packet(slug, spec.slo_state),
        waiver: None,
        owner_signoff: signed("managed-release"),
        active_narrowing_reasons: vec![
            NarrowingReason::AmbientCredentialInheritance,
            NarrowingReason::DryRunUnavailable,
            NarrowingReason::DescriptorParityBroken,
            NarrowingReason::RollbackTargetMissing,
            NarrowingReason::ProofPacketStale,
        ],
        published_label: StableClaimLevel::Preview,
        rationale:
            "The managed-output lane would inherit ambient mirror-operator credentials, its dry-run preview failed, its publish-target descriptor diverges between human review and headless plan, it records no rollback target, and its proof packet breached its SLO; it inherits its below-cutline beta claim and narrows to preview, naming every gap."
                .to_owned(),
        publication_destinations: vec![
            "release_center".to_owned(),
            "support_export".to_owned(),
            "diagnostics".to_owned(),
        ],
    }
}

fn version_bump_proposal(spec: &ClearedSpec) -> VersionBumpProposal {
    let slug = spec.slug;
    let public_surface = spec.public_surface_impact != PublicSurfaceImpact::NoPublicChange;
    VersionBumpProposal {
        proposal_id: format!("version-bump/m5-{slug}"),
        prior_version: spec.prior_version.to_owned(),
        target_version: spec.target_version.to_owned(),
        semantic_change_class: spec.semantic_change_class,
        affected_artifact_refs: vec![format!("artifact/m5/{slug}")],
        manifest_schema_change_refs: vec![format!("schema/m5-{slug}")],
        sdk_abi_range_refs: Vec::new(),
        extension_compatibility_refs: Vec::new(),
        docs_pack_change_refs: vec![format!("docs/m5-{slug}")],
        mirror_import_implication_refs: vec![format!("mirror/m5-{slug}/import")],
        evidence_refs: vec![EvidenceRef {
            evidence_ref: format!("evidence/version-bump/m5-{slug}"),
            evidence_kind: "compatibility_report".to_owned(),
            freshness_class: EvidenceFreshnessClass::Current,
            generated_at: Some(AS_OF.to_owned()),
            required_for_promotion: true,
            summary: "Compatibility report backs the proposed public-surface impact.".to_owned(),
        }],
        compatibility_notes: vec![CompatibilityNote {
            note_id: format!("compat/m5-{slug}"),
            impact_class: compatibility_impact(spec.public_surface_impact),
            affected_surface: format!("artifact/m5/{slug}"),
            public_surface,
            summary: format!(
                "Public-surface impact for the {} → {} bump.",
                spec.prior_version, spec.target_version
            ),
            source_refs: vec![format!("diff/m5-{slug}")],
        }],
        approval_refs: vec![format!("approval/m5-{slug}/public-surface")],
    }
}

fn compatibility_impact(impact: PublicSurfaceImpact) -> CompatibilityImpactClass {
    match impact {
        PublicSurfaceImpact::NoPublicChange => CompatibilityImpactClass::None,
        PublicSurfaceImpact::BackwardCompatible => CompatibilityImpactClass::DocsPackChange,
        PublicSurfaceImpact::MigrationRequired => CompatibilityImpactClass::StateMigrationChange,
        PublicSurfaceImpact::Breaking => CompatibilityImpactClass::SchemaChange,
    }
}

fn cleared_descriptor(spec: &ClearedSpec) -> PublishTargetDescriptor {
    let slug = spec.slug;
    let public_surface = spec.public_surface_impact != PublicSurfaceImpact::NoPublicChange;
    PublishTargetDescriptor {
        publish_target_id: format!("publish_target/m5-{slug}"),
        target_class: spec.target_class,
        destination_class: spec.destination.to_owned(),
        visibility_class: spec.visibility_class,
        mutability_class: spec.mutability_class,
        auth_source_class: spec.auth_source_class,
        actor_class: "release_engineering".to_owned(),
        rollout_ring: spec.rollout_ring,
        dry_run: DryRunDisclosure {
            availability_class: DryRunAvailabilityClass::SupportedCurrent,
            dry_run_ref: Some(format!("dry_run/m5-{slug}")),
            scope_preview_ref: Some(format!("scope_preview/m5-{slug}")),
            generated_at: Some(AS_OF.to_owned()),
            expires_at: Some("2026-09-13".to_owned()),
            blocking_findings: Vec::new(),
        },
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        evidence_refs: vec![EvidenceRef {
            evidence_ref: format!("evidence/publish-target/m5-{slug}"),
            evidence_kind: "clean_room_rebuild".to_owned(),
            freshness_class: EvidenceFreshnessClass::Current,
            generated_at: Some(AS_OF.to_owned()),
            required_for_promotion: true,
            summary: "Clean-room rebuild matches the published exact build.".to_owned(),
        }],
        exact_build_identity_refs: vec![format!("exact_build/m5-{slug}")],
        surface_parity_refs: vec![format!("surface_parity/m5-{slug}")],
        compatibility_notes: vec![CompatibilityNote {
            note_id: format!("compat/publish-target/m5-{slug}"),
            impact_class: compatibility_impact(spec.public_surface_impact),
            affected_surface: format!("channel/m5-{slug}"),
            public_surface,
            summary: "Publish-target compatibility note shared with the review sheet.".to_owned(),
            source_refs: vec![format!("diff/m5-{slug}")],
        }],
        continuity_notes: vec![ContinuityNote {
            note_id: format!("continuity/m5-{slug}"),
            continuity_class: ContinuityClass::RollbackCoordinated,
            summary: "Rollback target and mirror continuity are coordinated for the lane."
                .to_owned(),
            known_issue_refs: Vec::new(),
            support_refs: vec![format!("support/m5-{slug}")],
        }],
    }
}

fn proof_packet(slug: &str, slo_state: FreshnessSloState) -> ProofPacket {
    let captured_at = if slo_state == FreshnessSloState::Missing {
        None
    } else {
        Some(AS_OF.to_owned())
    };
    let evidence_refs = if slo_state == FreshnessSloState::Missing {
        Vec::new()
    } else {
        vec![format!("evidence/proof/m5-{slug}")]
    };
    ProofPacket {
        packet_id: format!("packet-m5-{slug}"),
        packet_ref: format!("proof/m5-{slug}"),
        proof_index_ref: format!("proof_index/m5-{slug}"),
        captured_at,
        freshness_slo: FreshnessSlo {
            target_max_age_days: TARGET_MAX_AGE_DAYS,
            warn_within_days: WARN_WITHIN_DAYS,
            slo_register_ref: SLO_REGISTER_REF.to_owned(),
        },
        slo_state,
        evidence_refs,
    }
}

fn signed(owner: &str) -> OwnerSignoff {
    OwnerSignoff {
        owner_ref: owner.to_owned(),
        signed_off: true,
        signed_at: Some(AS_OF.to_owned()),
    }
}

fn placeholder_summary() -> PublicationReviewSummary {
    PublicationReviewSummary {
        total_entries: 0,
        total_claims: 0,
        entries_cleared: 0,
        entries_narrowed: 0,
        entries_on_active_waiver: 0,
        entries_with_impact_gap: 0,
        entries_with_auth_gap: 0,
        entries_with_dry_run_gap: 0,
        entries_with_parity_gap: 0,
        entries_with_rollback_gap: 0,
        release_blocking_total: 0,
        release_blocking_cleared: 0,
        release_blocking_narrowed: 0,
        notebook_pack_entries: 0,
        request_data_asset_entries: 0,
        profiler_replay_entries: 0,
        framework_template_entries: 0,
        docs_pack_entries: 0,
        model_pack_entries: 0,
        companion_offboarding_entries: 0,
        managed_output_entries: 0,
        parity_matched: 0,
        parity_divergent: 0,
        parity_missing: 0,
        auth_explicit_disclosed: 0,
        auth_undisclosed: 0,
        auth_ambient_inherited: 0,
        packets_current: 0,
        packets_due_for_refresh: 0,
        packets_breached: 0,
        packets_missing: 0,
        total_active_narrowing_reasons: 0,
        rules_firing: 0,
    }
}
