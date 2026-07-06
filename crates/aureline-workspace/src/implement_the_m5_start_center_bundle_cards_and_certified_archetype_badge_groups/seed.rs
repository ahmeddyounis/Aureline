// Canonical seed for the M5 start-center launch-wedge primitive. Included from
// `mod.rs` so the seeded builder, its worked cases, the fixture generator, and the
// on-disk support export all stay byte-aligned.

/// A certified first-party launch wedge, fresh evidence, confirmed archetype.
fn certified_launch_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:certified-rust-service:0001".to_owned(),
        surface_label: "Start-center bundle card for a certified Rust service stack".to_owned(),
        bundle_id_ref: "bundle:rust-service:0001".to_owned(),
        bundle_name: "Rust Service Starter".to_owned(),
        persona_stack_tag: "Rust service".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        review_action_ref: "action:review-bundle:0001".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:rust-service:0001".to_owned(),
        archetype_id: "rust.service.axum".to_owned(),
        archetype_confidence: ArchetypeConfidence::Confirmed,
        supported_platform_envelope_ref: "envelope:linux-macos-win/rust-1.80".to_owned(),
        badge_count: 3,
        imported_confidence: ImportedVsNativeConfidence::Native,
        degraded: None,
    }
}

/// A managed org-approved launch wedge, fresh evidence, confirmed archetype.
fn managed_approved_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:managed-web-app:0002".to_owned(),
        surface_label: "Workspace switcher row for a managed-approved web-app stack".to_owned(),
        bundle_id_ref: "bundle:web-app:0002".to_owned(),
        bundle_name: "Managed Web App".to_owned(),
        persona_stack_tag: "web app".to_owned(),
        bundle_class: BundleClass::OrgManagedBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::PolicyGated,
        source_class: CertificationTarget::ManagedApproved,
        certification_freshness: EvidenceFreshness::Fresh,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        review_action_ref: "action:review-bundle:0002".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:web-app:0002".to_owned(),
        archetype_id: "web.spa.react".to_owned(),
        archetype_confidence: ArchetypeConfidence::Confirmed,
        supported_platform_envelope_ref: "envelope:node-20/web".to_owned(),
        badge_count: 2,
        imported_confidence: ImportedVsNativeConfidence::Native,
        degraded: None,
    }
}

/// A community-reviewed launch wedge whose evidence is aging (badge narrows to
/// Limited; assurance is approximate).
fn community_aging_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:community-data-pipeline:0003".to_owned(),
        surface_label: "Bundle-picker list entry for a community data-pipeline stack".to_owned(),
        bundle_id_ref: "bundle:data-pipeline:0003".to_owned(),
        bundle_name: "Community Data Pipeline".to_owned(),
        persona_stack_tag: "data pipeline".to_owned(),
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        certification_freshness: EvidenceFreshness::Aging,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        review_action_ref: "action:review-bundle:0003".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:data-pipeline:0003".to_owned(),
        archetype_id: "data.pipeline.spark".to_owned(),
        archetype_confidence: ArchetypeConfidence::Probable,
        supported_platform_envelope_ref: "envelope:linux/jvm-17".to_owned(),
        badge_count: 2,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
            degraded_label:
                "This community stack's certification evidence is aging past its freshness window; the badge group narrows to Limited and names the retest window"
                    .to_owned(),
        }),
    }
}

/// An imported launch wedge with stale evidence (badge shows Retest pending;
/// assurance is approximate).
fn imported_stale_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:imported-monorepo:0004".to_owned(),
        surface_label: "Docs / help bundle entry for an imported monorepo stack".to_owned(),
        bundle_id_ref: "bundle:monorepo:0004".to_owned(),
        bundle_name: "Imported Monorepo".to_owned(),
        persona_stack_tag: "polyglot monorepo".to_owned(),
        bundle_class: BundleClass::ImportedHandoffBundle,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Labs,
        source_class: CertificationTarget::ImportedPendingReview,
        certification_freshness: EvidenceFreshness::Stale,
        compatible_aureline_range: ">=2026.2, <2026.7".to_owned(),
        truth_mode: M5BundleTruthMode::Imported,
        review_action_ref: "action:review-bundle:0004".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:monorepo:0004".to_owned(),
        archetype_id: "monorepo.mixed".to_owned(),
        archetype_confidence: ArchetypeConfidence::Mixed,
        supported_platform_envelope_ref: "envelope:linux-macos/mixed".to_owned(),
        badge_count: 4,
        imported_confidence: ImportedVsNativeConfidence::Approximated,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
            degraded_label:
                "This imported stack's certification is stale and its archetype is mixed; the badge group shows Retest pending and keeps the imported-not-native provenance"
                    .to_owned(),
        }),
    }
}

/// A local-draft launch wedge with no external evidence (badge shows Retest
/// pending; assurance is local-only).
fn local_draft_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:local-draft-cli:0005".to_owned(),
        surface_label: "Diagnostics bundle view for a local-draft CLI stack".to_owned(),
        bundle_id_ref: "bundle:cli-draft:0005".to_owned(),
        bundle_name: "Local CLI Draft".to_owned(),
        persona_stack_tag: "cli tool".to_owned(),
        bundle_class: BundleClass::TemplateBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Labs,
        source_class: CertificationTarget::LocalDraft,
        certification_freshness: EvidenceFreshness::Missing,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        review_action_ref: "action:review-bundle:0005".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:cli:0005".to_owned(),
        archetype_id: "cli.rust.clap".to_owned(),
        archetype_confidence: ArchetypeConfidence::Undetected,
        supported_platform_envelope_ref: "envelope:local-device/rust".to_owned(),
        badge_count: 1,
        imported_confidence: ImportedVsNativeConfidence::Unverified,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
            degraded_label:
                "This local draft has no external certification evidence; the badge group shows Retest pending and the card names it as a local-only entry"
                    .to_owned(),
        }),
    }
}

/// A certified wedge served from a stale mirror (badge narrows to Limited on aging
/// mirror evidence; assurance stays certified).
fn mirrored_certified_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:mirrored-certified:0006".to_owned(),
        surface_label: "Start-center bundle card served from a mirror".to_owned(),
        bundle_id_ref: "bundle:rust-service:0006".to_owned(),
        bundle_name: "Rust Service Starter (mirror)".to_owned(),
        persona_stack_tag: "Rust service".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::MirrorOnly,
        source_class: CertificationTarget::Certified,
        certification_freshness: EvidenceFreshness::Aging,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Mirrored,
        review_action_ref: "action:review-bundle:0006".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:rust-service:0006".to_owned(),
        archetype_id: "rust.service.axum".to_owned(),
        archetype_confidence: ArchetypeConfidence::Confirmed,
        supported_platform_envelope_ref: "envelope:linux-macos-win/rust-1.80".to_owned(),
        badge_count: 3,
        imported_confidence: ImportedVsNativeConfidence::Native,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
            degraded_label:
                "This certified stack is served from a mirror whose evidence is aging; the badge group narrows to Limited and names the mirror provenance"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructed from an imported certified snapshot
/// (fresh snapshot evidence; badge stays current).
fn support_replay_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:support-replay:0007".to_owned(),
        surface_label: "Support / export replay reconstructing a launch-wedge snapshot".to_owned(),
        bundle_id_ref: "bundle:snapshot:0007".to_owned(),
        bundle_name: "Snapshot Rust Service".to_owned(),
        persona_stack_tag: "Rust service".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Imported,
        review_action_ref: "action:review-bundle:0007".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:rust-service:0007".to_owned(),
        archetype_id: "rust.service.axum".to_owned(),
        archetype_confidence: ArchetypeConfidence::Confirmed,
        supported_platform_envelope_ref: "envelope:linux-macos-win/rust-1.80".to_owned(),
        badge_count: 3,
        imported_confidence: ImportedVsNativeConfidence::Native,
        degraded: None,
    }
}

/// A community wedge from an offline cache with stale evidence (badge shows Retest
/// pending; assurance is approximate).
fn offline_community_wedge_input() -> M5LaunchWedgeInput {
    M5LaunchWedgeInput {
        wedge_id: "wedge:offline-community:0008".to_owned(),
        surface_label: "Bundle-picker list entry served from an offline cache".to_owned(),
        bundle_id_ref: "bundle:data-pipeline:0008".to_owned(),
        bundle_name: "Community Data Pipeline (offline)".to_owned(),
        persona_stack_tag: "data pipeline".to_owned(),
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        certification_freshness: EvidenceFreshness::Stale,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::CachedOffline,
        review_action_ref: "action:review-bundle:0008".to_owned(),
        inherits_hidden_marketplace_assumption: false,
        claims_current_despite_stale: false,
        archetype_family_ref: "archetype:data-pipeline:0008".to_owned(),
        archetype_id: "data.pipeline.spark".to_owned(),
        archetype_confidence: ArchetypeConfidence::Probable,
        supported_platform_envelope_ref: "envelope:linux/jvm-17".to_owned(),
        badge_count: 2,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "This community stack is served from an offline cache and its certification is stale; the badge group shows Retest pending and names the offline-cache provenance"
                    .to_owned(),
        }),
    }
}

fn case(input: M5LaunchWedgeInput) -> M5LaunchWedgeCase {
    M5LaunchWedgeCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5LaunchWedgeSurfaceRow> {
    let base_source_refs = vec![
        M5_START_CENTER_WEDGE_SCHEMA_REF.to_owned(),
        M5_START_CENTER_WEDGE_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5LaunchWedgeExportField::ALL.to_vec();

    vec![
        M5LaunchWedgeSurfaceRow {
            surface_family: M5LaunchWedgeSurfaceFamily::StartCenterCard,
            owner_role: "Start-center guild".to_owned(),
            scope_summary: "Start-center bundle card naming name, persona tag, support class, certification, range, signer, and a Review action"
                .to_owned(),
            source_classes: vec![CertificationTarget::Certified, CertificationTarget::ManagedApproved],
            truth_modes: vec![M5BundleTruthMode::Live, M5BundleTruthMode::Mirrored],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::MirrorStale,
            ],
            consumer_surfaces: vec!["start_center".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_wedges: vec![
                case(certified_launch_wedge_input()),
                case(mirrored_certified_wedge_input()),
            ],
            hides_entry_assurance: false,
            hides_badge_downgrade: false,
            inherits_hidden_source_class: false,
        },
        M5LaunchWedgeSurfaceRow {
            surface_family: M5LaunchWedgeSurfaceFamily::WorkspaceSwitcher,
            owner_role: "Workspace-switcher guild".to_owned(),
            scope_summary: "Workspace switcher row naming the active stack's source class and support class"
                .to_owned(),
            source_classes: vec![CertificationTarget::ManagedApproved],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
            ],
            consumer_surfaces: vec!["workspace_switcher".to_owned(), "start_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_wedges: vec![case(managed_approved_wedge_input())],
            hides_entry_assurance: false,
            hides_badge_downgrade: false,
            inherits_hidden_source_class: false,
        },
        M5LaunchWedgeSurfaceRow {
            surface_family: M5LaunchWedgeSurfaceFamily::BundlePickerList,
            owner_role: "Bundle-picker guild".to_owned(),
            scope_summary: "Bundle-picker list keeping certified, approximate, and local-only entries visibly distinct"
                .to_owned(),
            source_classes: vec![
                CertificationTarget::CommunityReviewed,
                CertificationTarget::ImportedPendingReview,
            ],
            truth_modes: vec![M5BundleTruthMode::Live, M5BundleTruthMode::CachedOffline],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["bundle_picker".to_owned(), "start_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_wedges: vec![
                case(community_aging_wedge_input()),
                case(offline_community_wedge_input()),
            ],
            hides_entry_assurance: false,
            hides_badge_downgrade: false,
            inherits_hidden_source_class: false,
        },
        M5LaunchWedgeSurfaceRow {
            surface_family: M5LaunchWedgeSurfaceFamily::DocsHelpBundleEntry,
            owner_role: "Docs / help guild".to_owned(),
            scope_summary: "Docs / help bundle entry preserving imported-not-native provenance and the retest-pending state"
                .to_owned(),
            source_classes: vec![CertificationTarget::ImportedPendingReview],
            truth_modes: vec![M5BundleTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["docs_help".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_wedges: vec![case(imported_stale_wedge_input())],
            hides_entry_assurance: false,
            hides_badge_downgrade: false,
            inherits_hidden_source_class: false,
        },
        M5LaunchWedgeSurfaceRow {
            surface_family: M5LaunchWedgeSurfaceFamily::DiagnosticsBundleView,
            owner_role: "Diagnostics guild".to_owned(),
            scope_summary: "Diagnostics bundle view naming a local-only draft without inheriting an official tier"
                .to_owned(),
            source_classes: vec![CertificationTarget::LocalDraft],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::UnverifiedSigner,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_wedges: vec![case(local_draft_wedge_input())],
            hides_entry_assurance: false,
            hides_badge_downgrade: false,
            inherits_hidden_source_class: false,
        },
        M5LaunchWedgeSurfaceRow {
            surface_family: M5LaunchWedgeSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing launch-wedge truth from an imported certified snapshot"
                .to_owned(),
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Imported],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
                M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_wedges: vec![case(support_replay_wedge_input())],
            hides_entry_assurance: false,
            hides_badge_downgrade: false,
            inherits_hidden_source_class: false,
        },
    ]
}

fn seeded_governance_review() -> M5LaunchWedgeGovernanceReview {
    M5LaunchWedgeGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        wedge_identity_preserved_across_surfaces: true,
        entry_assurance_legible_before_install: true,
        archetype_badges_degrade_visibly: true,
        source_class_never_inherited: true,
        support_export_reconstructs_wedge: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5LaunchWedgeConsumerProjection {
    M5LaunchWedgeConsumerProjection {
        launch_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        badge_group_reads_single_certification_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5LaunchWedgeReleasePosture {
    M5LaunchWedgeReleasePosture {
        release_packet_ref: M5_START_CENTER_WEDGE_ARTIFACT_REF.to_owned(),
        launch_wedge_audit_ref: M5_START_CENTER_WEDGE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 start-center launch-wedge primitive packet.
/// This is the one source of truth shared by the tests, the fixture generator, and
/// the on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_start_center_launch_wedge_packet() -> M5StartCenterLaunchWedgePacket {
    M5StartCenterLaunchWedgePacket::new(M5StartCenterLaunchWedgePacketInput {
        packet_id: "m5-start-center-launch-wedge-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Start-Center Launch-Wedge Primitive: Start-Center Bundle Card and Certified-Archetype Badge Group"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5LaunchWedgeVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_START_CENTER_WEDGE_SCHEMA_REF.to_owned(),
            M5_START_CENTER_WEDGE_DOC_REF.to_owned(),
            M5_START_CENTER_WEDGE_COMPONENT_MATRIX_REF.to_owned(),
            M5_START_CENTER_WEDGE_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "workflow_bundle_component_boundary_v1".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    })
}
