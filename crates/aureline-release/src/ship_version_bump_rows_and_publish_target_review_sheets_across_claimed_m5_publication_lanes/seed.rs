//! Canonical seed builders for the M5 version-bump-row / publish-target-review-sheet
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code matrix, the artifact, the worked resolutions, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical publication-review-primitive packet.
pub const M5_PUBLICATION_REVIEW_PRIMITIVE_PACKET_ID: &str =
    "m5-publish-target-review-sheet-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full publication state.
#[allow(clippy::too_many_arguments)]
fn case(
    proposal_label: &str,
    prior_version_repr: &str,
    next_version_repr: &str,
    version_bump_class: M5VersionBumpClass,
    compatibility_impact: M5CompatibilityImpact,
    changed_artifact_set: &[&str],
    target_class: M5PublishTargetClass,
    visibility: M5PublishTargetVisibility,
    mutability: M5TargetMutability,
    auth_source: M5TargetAuthSource,
    auth_disclosure_state: M5AuthDisclosureState,
    dry_run: M5DryRunAvailability,
    rollout_ring: M5RolloutRing,
    surface_impact_analysis: M5SurfaceImpactAnalysis,
) -> M5PublicationReviewResolutionCase {
    M5PublicationReviewResolutionCase::resolved(M5PublicationReviewInput {
        proposal_label: proposal_label.to_owned(),
        prior_version_repr: prior_version_repr.to_owned(),
        next_version_repr: next_version_repr.to_owned(),
        version_bump_class,
        compatibility_impact,
        changed_artifact_set: changed_artifact_set
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        target_class,
        visibility,
        mutability,
        auth_source,
        auth_disclosure_state,
        dry_run,
        rollout_ring,
        surface_impact_analysis,
    })
}

/// A base row with the shared fields filled in and the full anatomy, version-bump,
/// compatibility, public-surface-impact, target-class, visibility, mutability,
/// auth-source, auth-disclosure, dry-run, rollout-ring, surface-analysis,
/// reversibility, readiness, block-reason, next-action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5PublicationReviewConsumerSurface,
    qualification: M5ReleaseCenterQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_resolutions: Vec<M5PublicationReviewResolutionCase>,
) -> M5PublicationReviewRow {
    M5PublicationReviewRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PublicationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5PublicationReviewAnatomyPart::ALL.to_vec(),
        version_bump_classes: M5VersionBumpClass::ALL.to_vec(),
        compatibility_impacts: M5CompatibilityImpact::ALL.to_vec(),
        public_surface_impacts: M5PublicSurfaceImpact::ALL.to_vec(),
        target_classes: M5PublishTargetClass::ALL.to_vec(),
        target_visibilities: M5PublishTargetVisibility::ALL.to_vec(),
        target_mutabilities: M5TargetMutability::ALL.to_vec(),
        target_auth_sources: M5TargetAuthSource::ALL.to_vec(),
        auth_disclosure_states: M5AuthDisclosureState::ALL.to_vec(),
        dry_run_availabilities: M5DryRunAvailability::ALL.to_vec(),
        rollout_rings: M5RolloutRing::ALL.to_vec(),
        surface_impact_analyses: M5SurfaceImpactAnalysis::ALL.to_vec(),
        destination_reversibilities: M5DestinationReversibility::ALL.to_vec(),
        readiness_postures: M5PublicationReadiness::ALL.to_vec(),
        block_reasons: M5PublicationBlockReason::ALL.to_vec(),
        next_actions: M5PublicationNextAction::ALL.to_vec(),
        export_fields: M5PublicationExportField::ALL.to_vec(),
        accessibility_routes: M5ReleaseCenterAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
            M5ReleaseCenterConsumerSurface::HelpAbout,
            M5ReleaseCenterConsumerSurface::AdminConsole,
            M5ReleaseCenterConsumerSurface::EvaluationPack,
            M5ReleaseCenterConsumerSurface::MirrorConsole,
            M5ReleaseCenterConsumerSurface::SupportExport,
            M5ReleaseCenterConsumerSurface::CliInspect,
            M5ReleaseCenterConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5ReleaseCenterDowngradeTrigger::VersionBumpImpactUnstated,
            M5ReleaseCenterDowngradeTrigger::TargetAuthSourceMasked,
            M5ReleaseCenterDowngradeTrigger::TargetMutabilityHidden,
            M5ReleaseCenterDowngradeTrigger::DryRunAvailabilityUnstated,
            M5ReleaseCenterDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PUBLICATION_REVIEW_SCHEMA_REF,
            M5_PUBLICATION_REVIEW_OBJECT_MODEL_REF,
            M5_PUBLICATION_REVIEW_VERIFICATION_CONTRACT_REF,
        ]),
        example_resolutions,
        collapses_impact_into_semver_string: false,
        masks_target_auth_source_or_destination_class: false,
        confuses_mutable_with_immutable_publication: false,
        inherits_ambient_credentials_silently: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn publication_rows() -> Vec<M5PublicationReviewRow> {
    use M5AuthDisclosureState as Auth;
    use M5CompatibilityImpact as Compat;
    use M5DryRunAvailability as Dry;
    use M5PublishTargetClass as Target;
    use M5PublishTargetVisibility as Vis;
    use M5RolloutRing as Ring;
    use M5SurfaceImpactAnalysis as Surface;
    use M5TargetAuthSource as AuthSrc;
    use M5TargetMutability as Mut;
    use M5VersionBumpClass as Bump;

    let mut rows = Vec::new();

    // 1. Release-center publish sheet — a cleanly publishable minor bump to a
    //    registry, and a major breaking bump to a managed control plane that is
    //    blocked because the publish would inherit ambient credentials (the
    //    publishable / blocked and ambient-credential coverage proof).
    rows.push(base_row(
        M5PublicationReviewConsumerSurface::ReleaseCenterPublishSheet,
        M5ReleaseCenterQualificationClass::Stable,
        "Release-center publish-sheet owner",
        "The release-center publish sheet renders the shared version-bump / publish-target primitive so a minor additive bump to a public registry with a scoped CI identity and a supported dry-run reads as publishable, while a major breaking bump to a managed control plane that would inherit ambient credentials reads as blocked with a self-contained banner naming the reason and the disclose-auth-source next action",
        "evidence:m5-publish-target-center:001",
        vec![
            case(
                "aureline release 5.2.0",
                "5.1.4",
                "5.2.0",
                Bump::Minor,
                Compat::BackwardCompatible,
                &["artifact:core-runtime", "artifact:cli"],
                Target::RegistryTarget,
                Vis::PublicListed,
                Mut::AppendOnly,
                AuthSrc::CiFederatedIdentity,
                Auth::AuthScopedDisclosed,
                Dry::DryRunSupported,
                Ring::BroadRing,
                Surface::SurfaceImpactFresh,
            ),
            case(
                "aureline release 6.0.0",
                "5.1.4",
                "6.0.0",
                Bump::Major,
                Compat::BreakingChange,
                &["artifact:core-runtime", "artifact:graph", "artifact:shell"],
                Target::ManagedControlPlaneTarget,
                Vis::PrivateTenant,
                Mut::OverwriteAllowed,
                AuthSrc::OrgManagedIdentity,
                Auth::AmbientCredentialInherited,
                Dry::DryRunUnavailable,
                Ring::HeldNotPromoted,
                Surface::SurfaceImpactFresh,
            ),
        ],
    ));

    // 2. Update-center publish row — a patch runtime-behaviour bump to a channel
    //    pointer blocked on stale surface analysis, and a prerelease forward-
    //    incompatible bump to a mirror blocked on missing surface analysis
    //    (the stale-versus-missing surface-analysis proof).
    rows.push(base_row(
        M5PublicationReviewConsumerSurface::UpdateCenterPublishRow,
        M5ReleaseCenterQualificationClass::Stable,
        "Update-center publish-row owner",
        "The update-center publish row renders the shared primitive so a patch runtime-behaviour bump to a mutable channel pointer whose public-surface analysis has gone stale reads as blocked-surface-impact-stale with a refresh next action, while a prerelease forward-incompatible bump to a mirror missing its surface analysis reads as blocked-surface-impact-missing with a provide next action",
        "evidence:m5-publish-target-update:001",
        vec![
            case(
                "aureline update 5.1.5",
                "5.1.4",
                "5.1.5",
                Bump::Patch,
                Compat::RuntimeBehaviorOnly,
                &["artifact:update-agent"],
                Target::ChannelPointerTarget,
                Vis::MirrorReplicated,
                Mut::MutableTagRepointable,
                AuthSrc::MaintainerKey,
                Auth::AuthBroadDisclosed,
                Dry::DryRunPartial,
                Ring::EarlyAccessRing,
                Surface::SurfaceImpactStale,
            ),
            case(
                "aureline update 5.2.0-rc.1",
                "5.1.4",
                "5.2.0-rc.1",
                Bump::Prerelease,
                Compat::ForwardIncompatible,
                &["artifact:update-agent", "artifact:mirror"],
                Target::MirrorTarget,
                Vis::PublicUnlisted,
                Mut::RetractionAllowed,
                AuthSrc::DelegatedBotIdentity,
                Auth::AuthScopedDisclosed,
                Dry::DryRunSupported,
                Ring::PilotRing,
                Surface::SurfaceImpactMissing,
            ),
        ],
    ));

    // 3. CLI publish inspect — a build-metadata-only republish to a local immutable
    //    store whose review state is unknown, and a minor additive bump to a registry
    //    narrowed pending surface review.
    rows.push(base_row(
        M5PublicationReviewConsumerSurface::CliPublishInspect,
        M5ReleaseCenterQualificationClass::Stable,
        "CLI publish-inspect owner",
        "The CLI publish-inspect surface renders the shared primitive so a build-metadata-only publish to an immutable local store whose review state has not been evaluated reads as blocked-unknown-state with a run-review next action and an immutable-by-design reversibility, while a minor additive bump to a registry whose surface review is pending sign-off reads as narrowed-surface-review-pending with a complete-review next action",
        "evidence:m5-publish-target-cli:001",
        vec![
            case(
                "aureline preview 5.3.0",
                "5.3.0",
                "5.3.0+build.7",
                Bump::BuildMetadataOnly,
                Compat::BackwardCompatible,
                &["artifact:experimental-core"],
                Target::LocalArtifactStoreTarget,
                Vis::InternalOnly,
                Mut::ImmutableOncePublished,
                AuthSrc::HardwareTokenSigner,
                Auth::AuthDisclosureUnknown,
                Dry::DryRunUnavailable,
                Ring::HeldNotPromoted,
                Surface::SurfaceImpactUnknown,
            ),
            case(
                "aureline backport 5.0.9",
                "5.0.8",
                "5.0.9",
                Bump::Minor,
                Compat::BackwardCompatible,
                &["artifact:lts-core"],
                Target::RegistryTarget,
                Vis::PublicListed,
                Mut::AppendOnly,
                AuthSrc::CiFederatedIdentity,
                Auth::AuthDisclosurePendingReview,
                Dry::DryRunSupported,
                Ring::CanaryRing,
                Surface::SurfaceImpactFresh,
            ),
        ],
    ));

    // 4. Admin publish report — a patch additive bump to a mutable channel pointer
    //    narrowed because reversibility is unproven, and a major migration bump to a
    //    managed control plane publishable only after a required dry-run under waiver.
    rows.push(base_row(
        M5PublicationReviewConsumerSurface::AdminPublishReport,
        M5ReleaseCenterQualificationClass::Stable,
        "Admin publish-report owner",
        "The admin publish report renders the shared primitive so a patch additive bump to a mutable channel pointer with no dry-run reads as narrowed-reversibility-unproven with an enable-dry-run next action rather than reading like an immutable step, while a major schema-migration bump to a managed control plane held under a disclosed waiver with a required dry-run reads as publishable-dry-run-first",
        "evidence:m5-publish-target-admin:001",
        vec![
            case(
                "aureline lts 4.9.7",
                "4.9.6",
                "4.9.7",
                Bump::Patch,
                Compat::BackwardCompatible,
                &["artifact:lts-core", "artifact:lts-cli"],
                Target::ChannelPointerTarget,
                Vis::PrivateTenant,
                Mut::MutableTagRepointable,
                AuthSrc::OrgManagedIdentity,
                Auth::AuthScopedDisclosed,
                Dry::DryRunUnavailable,
                Ring::BroadRing,
                Surface::SurfaceImpactFresh,
            ),
            case(
                "aureline release 6.0.0-migration",
                "5.9.9",
                "6.0.0",
                Bump::Major,
                Compat::SchemaMigrationRequired,
                &["artifact:core-runtime"],
                Target::ManagedControlPlaneTarget,
                Vis::MirrorReplicated,
                Mut::OverwriteAllowed,
                AuthSrc::UnauthenticatedMirror,
                Auth::AuthDisclosedUnderWaiver,
                Dry::DryRunRequiredBeforePublish,
                Ring::GeneralAvailability,
                Surface::SurfaceImpactFresh,
            ),
        ],
    ));

    // 5. Support / evaluation export — a minor additive bump publishable with review
    //    on aging surface analysis, and a republish-no-version-change to a mirror
    //    publishable with review on a broadly-scoped auth identity (both stay
    //    publishable while the review reservation is disclosed).
    rows.push(base_row(
        M5PublicationReviewConsumerSurface::SupportEvaluationExport,
        M5ReleaseCenterQualificationClass::Stable,
        "Support / evaluation export owner",
        "The support / evaluation export renders the shared primitive so a minor additive bump whose surface analysis is aging reads as publishable-with-review rather than clean or blocked, and a republish with no version change to a mirror published by a broadly-scoped delegated bot reads as publishable-with-review — the same version-bump / publish-target vocabulary a support or evaluation reviewer reads elsewhere",
        "evidence:m5-publish-target-support:001",
        vec![
            case(
                "aureline release 5.2.0-rc.3",
                "5.1.4",
                "5.2.0-rc.3",
                Bump::Minor,
                Compat::BackwardCompatible,
                &["artifact:core-runtime", "artifact:graph"],
                Target::RegistryTarget,
                Vis::PublicListed,
                Mut::AppendOnly,
                AuthSrc::CiFederatedIdentity,
                Auth::AuthScopedDisclosed,
                Dry::DryRunSupported,
                Ring::BroadRing,
                Surface::SurfaceImpactAging,
            ),
            case(
                "aureline mirror republish 5.1.4",
                "5.1.4",
                "5.1.4",
                Bump::RepublishNoVersionChange,
                Compat::BackwardCompatible,
                &["artifact:mirror"],
                Target::MirrorTarget,
                Vis::MirrorReplicated,
                Mut::AppendOnly,
                AuthSrc::DelegatedBotIdentity,
                Auth::AuthBroadDisclosed,
                Dry::DryRunSupported,
                Ring::EarlyAccessRing,
                Surface::SurfaceImpactFresh,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5PublicationReviewGovernanceReview {
    M5PublicationReviewGovernanceReview {
        one_primitive_carries_publication_truth: true,
        version_identity_and_impact_always_shown: true,
        impact_never_collapsed_into_semver: true,
        auth_source_and_destination_shown_before_mutation: true,
        mutability_and_dry_run_never_confused_with_immutable: true,
        ambient_credentials_never_inherited_silently: true,
        blocked_state_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_publication_truth: true,
        no_surface_invents_second_publication_grammar: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5PublicationReviewConsumerProjection {
    M5PublicationReviewConsumerProjection {
        publication_surfaces_consume_shared_primitive: true,
        readiness_resolver_reads_single_source: true,
        public_surface_impact_reads_single_source: true,
        auth_source_disclosure_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5PublicationReviewProofFreshness {
    M5PublicationReviewProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PublicationReviewReleasePosture {
    M5PublicationReviewReleasePosture {
        release_packet_ref: M5_PUBLICATION_REVIEW_ARTIFACT_REF.to_owned(),
        publication_audit_ref: M5_PUBLICATION_REVIEW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PUBLICATION_REVIEW_SCHEMA_REF,
        M5_PUBLICATION_REVIEW_DOC_REF,
        M5_PUBLICATION_REVIEW_COMPONENT_MATRIX_REF,
        M5_PUBLICATION_REVIEW_OBJECT_MODEL_REF,
        M5_PUBLICATION_REVIEW_VERIFICATION_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 publication-review-primitive packet.
pub fn seeded_m5_publication_review_primitive_packet() -> M5PublicationReviewPrimitivePacket {
    M5PublicationReviewPrimitivePacket::new(M5PublicationReviewPrimitivePacketInput {
        packet_id: M5_PUBLICATION_REVIEW_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 version-bump row and publish-target review-sheet primitive: prior/next version, delta kind, public-surface impact, publish-target class, visibility, mutability, auth source, dry-run availability, and rollout ring"
                .to_owned(),
        publication_rows: publication_rows(),
        vocabulary_set: M5PublicationReviewVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the update-center publish row is held at Beta because a slice of
/// update-center publishes do not yet render the dry-run availability cue on every
/// profile; every consumer stays visible.
pub fn seeded_m5_publication_review_primitive_update_center_publish_row_beta_narrowed(
) -> M5PublicationReviewPrimitivePacket {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.packet_id =
        "m5-publish-target-review-sheet-primitive:update-center-beta:0001".to_owned();
    let row = packet
        .publication_rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5PublicationReviewConsumerSurface::UpdateCenterPublishRow
        })
        .expect("update-center publish row present");
    row.qualification = M5ReleaseCenterQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI publish-inspect surface is narrowed to Preview pending
/// self-contained-banner parity proof across every headless export path; every
/// consumer stays visible.
pub fn seeded_m5_publication_review_primitive_cli_publish_inspect_preview_narrowed(
) -> M5PublicationReviewPrimitivePacket {
    let mut packet = seeded_m5_publication_review_primitive_packet();
    packet.packet_id =
        "m5-publish-target-review-sheet-primitive:cli-publish-inspect-preview:0001".to_owned();
    let row = packet
        .publication_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PublicationReviewConsumerSurface::CliPublishInspect)
        .expect("cli publish-inspect row present");
    row.qualification = M5ReleaseCenterQualificationClass::Preview;
    packet
}
