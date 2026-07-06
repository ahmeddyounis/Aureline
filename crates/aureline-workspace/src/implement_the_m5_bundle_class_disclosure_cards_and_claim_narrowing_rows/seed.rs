// Canonical seed for the M5 bundle class-disclosure primitive. Included from `mod.rs` so the
// seeded builder, its worked cases, the fixture generator, and the on-disk support export all stay
// byte-aligned.

/// A dependency disclosure naming a policy owner, mirror source, and entitlement dependency for a
/// fully managed, org-bound, mirror-served bundle.
fn managed_dependencies() -> M5BundleDependencyDisclosure {
    M5BundleDependencyDisclosure {
        depends_on_managed_registry: true,
        depends_on_org_identity: true,
        depends_on_mirror_freshness: true,
        depends_on_policy_availability: true,
        policy_owner: Some("Acme Platform Governance".to_owned()),
        mirror_source: Some("Acme internal mirror channel".to_owned()),
        entitlement_dependency: Some("Acme managed-stack entitlement".to_owned()),
    }
}

/// A dependency disclosure for an offline-served bundle whose freshness is mirror-bounded.
fn mirror_only_dependencies() -> M5BundleDependencyDisclosure {
    M5BundleDependencyDisclosure {
        depends_on_managed_registry: false,
        depends_on_org_identity: false,
        depends_on_mirror_freshness: true,
        depends_on_policy_availability: false,
        policy_owner: None,
        mirror_source: Some("Offline mirror snapshot".to_owned()),
        entitlement_dependency: None,
    }
}

/// A native, first-party, certified bundle offered on the start center: full native parity, no
/// dependencies, no narrowing.
fn start_center_native_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:rust-service:0001".to_owned(),
        surface_label: "Start-center class card for a native first-party Rust service bundle"
            .to_owned(),
        bundle_id_ref: "bundle:rust-service:0001".to_owned(),
        bundle_name: "Rust Service Starter".to_owned(),
        disclosure_class: M5BundleDisclosureClass::NativeFirstParty,
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        imported_confidence: ImportedVsNativeConfidence::Native,
        capability_confidence: M5CapabilityConfidence::Native,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        dependencies: M5BundleDependencyDisclosure::none(),
        reason_for_recommendation:
            "Recommended because it is a first-party native bundle with fresh certification and an exact platform match"
                .to_owned(),
        claims_full_native_parity: true,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// A design-partner certified bundle on the detail page: certified, but mapped exactly rather than
/// natively, so it discloses an exact-mapping compatibility claim instead of native parity.
fn detail_design_partner_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:web-app:0002".to_owned(),
        surface_label: "Bundle detail class panel for a design-partner certified web app bundle"
            .to_owned(),
        bundle_id_ref: "bundle:web-app:0002".to_owned(),
        bundle_name: "Design-Partner Web App".to_owned(),
        disclosure_class: M5BundleDisclosureClass::DesignPartnerCertified,
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        imported_confidence: ImportedVsNativeConfidence::Native,
        capability_confidence: M5CapabilityConfidence::Exact,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        dependencies: M5BundleDependencyDisclosure::none(),
        reason_for_recommendation:
            "Recommended because a design partner certified it against a fresh proof; its capabilities map exactly onto native ones"
                .to_owned(),
        claims_full_native_parity: false,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// An imported-user handoff bundle reviewed during migration: bridged, with stale certification, so
/// the row narrows the claim to capability-mapped and never inherits native parity.
fn migration_imported_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:imported-monorepo:0003".to_owned(),
        surface_label: "Migration class-disclosure row for an imported user-handoff bundle"
            .to_owned(),
        bundle_id_ref: "bundle:monorepo:0003".to_owned(),
        bundle_name: "Imported Monorepo Handoff".to_owned(),
        disclosure_class: M5BundleDisclosureClass::ImportedUserHandoff,
        bundle_class: BundleClass::ImportedHandoffBundle,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Labs,
        source_class: CertificationTarget::ImportedPendingReview,
        scorecard_class: BundleScorecardClass::Imported,
        certification_freshness: EvidenceFreshness::Stale,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        capability_confidence: M5CapabilityConfidence::CapabilityMapped,
        compatible_aureline_range: ">=2026.2, <2026.7".to_owned(),
        truth_mode: M5BundleTruthMode::Imported,
        dependencies: M5BundleDependencyDisclosure::none(),
        reason_for_recommendation:
            "Surfaced because it was imported from another setup; it is bridged, not native, and pending review"
                .to_owned(),
        claims_full_native_parity: false,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
            degraded_label:
                "This bundle was imported from another setup and mapped through a compatibility bridge, so the row narrows the claim to capability-mapped and flags the pending re-certification"
                    .to_owned(),
        }),
    }
}

/// A community bundle explained in docs / help: community-reviewed, bridged, aging certification, so
/// the docs block discloses a capability-mapped claim.
fn docs_community_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:framework-pack:0004".to_owned(),
        surface_label: "Docs / help class block for a community framework pack".to_owned(),
        bundle_id_ref: "bundle:framework-pack:0004".to_owned(),
        bundle_name: "Community Framework Pack".to_owned(),
        disclosure_class: M5BundleDisclosureClass::Community,
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        scorecard_class: BundleScorecardClass::Community,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        capability_confidence: M5CapabilityConfidence::CapabilityMapped,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        dependencies: M5BundleDependencyDisclosure::none(),
        reason_for_recommendation:
            "Recommended by the community for this framework; it is capability-mapped rather than native and community-reviewed"
                .to_owned(),
        claims_full_native_parity: false,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// A managed, org-approved bundle in a diagnostics report: native capability, but policy-bound and
/// mirror-served, so it discloses its managed dependencies and never inherits native parity.
fn diagnostics_managed_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:managed-web:0005".to_owned(),
        surface_label: "Diagnostics class report for a managed, org-approved web app bundle"
            .to_owned(),
        bundle_id_ref: "bundle:managed-web:0005".to_owned(),
        bundle_name: "Managed Web App".to_owned(),
        disclosure_class: M5BundleDisclosureClass::ManagedApproved,
        bundle_class: BundleClass::OrgManagedBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::PolicyGated,
        source_class: CertificationTarget::ManagedApproved,
        scorecard_class: BundleScorecardClass::Probable,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Native,
        capability_confidence: M5CapabilityConfidence::Native,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Mirrored,
        dependencies: managed_dependencies(),
        reason_for_recommendation:
            "Recommended by org policy for this team; it maps natively but its availability is policy-bound and mirror-served"
                .to_owned(),
        claims_full_native_parity: false,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
            degraded_label:
                "This managed bundle depends on org identity and a policy-controlled mirror whose evidence is aging, so the report keeps the policy owner and entitlement dependency explicit rather than implying standalone completeness"
                    .to_owned(),
        }),
    }
}

/// A local draft in a diagnostics report: no external claim, missing certification, so the report
/// narrows to a local-draft class strength even though the draft runs natively.
fn diagnostics_local_draft_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:local-draft:0006".to_owned(),
        surface_label: "Diagnostics class report for a local draft bundle".to_owned(),
        bundle_id_ref: "bundle:local-draft:0006".to_owned(),
        bundle_name: "Local Draft Stack".to_owned(),
        disclosure_class: M5BundleDisclosureClass::LocalDraft,
        bundle_class: BundleClass::TemplateBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Labs,
        source_class: CertificationTarget::LocalDraft,
        scorecard_class: BundleScorecardClass::LocalDraft,
        certification_freshness: EvidenceFreshness::Missing,
        imported_confidence: ImportedVsNativeConfidence::Native,
        capability_confidence: M5CapabilityConfidence::Native,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        dependencies: M5BundleDependencyDisclosure::none(),
        reason_for_recommendation:
            "Shown because it is your own local draft; it runs natively but carries no external certification claim yet"
                .to_owned(),
        claims_full_native_parity: false,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// An imported bundle reconstructed offline for support replay: approximate mapping, stale, mirror-
/// bounded, so the replay narrows the claim and names its offline provenance.
fn support_replay_input() -> M5BundleClassDisclosureInput {
    M5BundleClassDisclosureInput {
        disclosure_id: "disclosure:offline-replay:0007".to_owned(),
        surface_label: "Support / export replay reconstructing class truth from an offline cache"
            .to_owned(),
        bundle_id_ref: "bundle:imported-legacy:0007".to_owned(),
        bundle_name: "Imported Legacy Stack (offline)".to_owned(),
        disclosure_class: M5BundleDisclosureClass::ImportedUserHandoff,
        bundle_class: BundleClass::ImportedHandoffBundle,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::MirrorOnly,
        source_class: CertificationTarget::ImportedPendingReview,
        scorecard_class: BundleScorecardClass::Imported,
        certification_freshness: EvidenceFreshness::Stale,
        imported_confidence: ImportedVsNativeConfidence::Approximated,
        capability_confidence: M5CapabilityConfidence::Approximate,
        compatible_aureline_range: ">=2026.1, <2026.6".to_owned(),
        truth_mode: M5BundleTruthMode::CachedOffline,
        dependencies: mirror_only_dependencies(),
        reason_for_recommendation:
            "Reconstructed for support from an offline cache; it is an approximate import, mirror-bounded, and pending review"
                .to_owned(),
        claims_full_native_parity: false,
        implies_standalone_local_completeness: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "This imported bundle's class is reconstructed from an offline cache with stale certification, so the replay narrows the claim to approximate and names the mirror-bounded, offline-cache provenance"
                    .to_owned(),
        }),
    }
}

fn case(input: M5BundleClassDisclosureInput) -> M5BundleClassDisclosureCase {
    M5BundleClassDisclosureCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5BundleDisclosureSurfaceRow> {
    let base_source_refs = vec![
        M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_REF.to_owned(),
        M5_BUNDLE_CLASS_DISCLOSURE_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5BundleDisclosureExportField::ALL.to_vec();

    vec![
        M5BundleDisclosureSurfaceRow {
            surface_family: M5BundleDisclosureSurfaceFamily::StartCenterClassCard,
            owner_role: "Start-center guild".to_owned(),
            scope_summary: "Start-center class card disclosing a native first-party bundle's class, native compatibility, and fresh certification before launch"
                .to_owned(),
            disclosure_classes: vec![M5BundleDisclosureClass::NativeFirstParty],
            capability_confidences: vec![M5CapabilityConfidence::Native],
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::IncompatibleAureline,
            ],
            consumer_surfaces: vec!["start_center".to_owned(), "workspace_shell".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_disclosures: vec![case(start_center_native_input())],
            overclaims_native_parity: false,
            implies_standalone_completeness: false,
            collapses_class_to_generic: false,
        },
        M5BundleDisclosureSurfaceRow {
            surface_family: M5BundleDisclosureSurfaceFamily::BundleDetailClassPanel,
            owner_role: "Bundle detail guild".to_owned(),
            scope_summary: "Bundle detail class panel disclosing a design-partner certified bundle whose capabilities map exactly rather than natively"
                .to_owned(),
            disclosure_classes: vec![M5BundleDisclosureClass::DesignPartnerCertified],
            capability_confidences: vec![M5CapabilityConfidence::Exact],
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
            ],
            consumer_surfaces: vec!["bundle_detail".to_owned(), "docs_bundles".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_disclosures: vec![case(detail_design_partner_input())],
            overclaims_native_parity: false,
            implies_standalone_completeness: false,
            collapses_class_to_generic: false,
        },
        M5BundleDisclosureSurfaceRow {
            surface_family: M5BundleDisclosureSurfaceFamily::MigrationClassDisclosureRow,
            owner_role: "Migration guild".to_owned(),
            scope_summary: "Migration class-disclosure row disclosing an imported user-handoff bundle as bridged, pending review, and narrowed from native parity"
                .to_owned(),
            disclosure_classes: vec![M5BundleDisclosureClass::ImportedUserHandoff],
            capability_confidences: vec![M5CapabilityConfidence::CapabilityMapped],
            source_classes: vec![CertificationTarget::ImportedPendingReview],
            truth_modes: vec![M5BundleTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["migration_review".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_disclosures: vec![case(migration_imported_input())],
            overclaims_native_parity: false,
            implies_standalone_completeness: false,
            collapses_class_to_generic: false,
        },
        M5BundleDisclosureSurfaceRow {
            surface_family: M5BundleDisclosureSurfaceFamily::DocsHelpClassBlock,
            owner_role: "Docs / help guild".to_owned(),
            scope_summary: "Docs / help class block explaining a community bundle as capability-mapped and community-reviewed using the shared class vocabulary"
                .to_owned(),
            disclosure_classes: vec![M5BundleDisclosureClass::Community],
            capability_confidences: vec![M5CapabilityConfidence::CapabilityMapped],
            source_classes: vec![CertificationTarget::CommunityReviewed],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::UnverifiedSigner,
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
            ],
            consumer_surfaces: vec!["docs_help".to_owned(), "docs_bundles".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_disclosures: vec![case(docs_community_input())],
            overclaims_native_parity: false,
            implies_standalone_completeness: false,
            collapses_class_to_generic: false,
        },
        M5BundleDisclosureSurfaceRow {
            surface_family: M5BundleDisclosureSurfaceFamily::DiagnosticsClassReport,
            owner_role: "Diagnostics guild".to_owned(),
            scope_summary: "Diagnostics class report covering a managed org-approved bundle with disclosed policy / mirror dependencies and a local draft with no external claim"
                .to_owned(),
            disclosure_classes: vec![
                M5BundleDisclosureClass::ManagedApproved,
                M5BundleDisclosureClass::LocalDraft,
            ],
            capability_confidences: vec![M5CapabilityConfidence::Native],
            source_classes: vec![
                CertificationTarget::ManagedApproved,
                CertificationTarget::LocalDraft,
            ],
            truth_modes: vec![M5BundleTruthMode::Mirrored, M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::MirrorStale,
                M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_disclosures: vec![
                case(diagnostics_managed_input()),
                case(diagnostics_local_draft_input()),
            ],
            overclaims_native_parity: false,
            implies_standalone_completeness: false,
            collapses_class_to_generic: false,
        },
        M5BundleDisclosureSurfaceRow {
            surface_family: M5BundleDisclosureSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing an imported bundle's class as approximate, mirror-bounded, and stale, keeping the class stable offline"
                .to_owned(),
            disclosure_classes: vec![M5BundleDisclosureClass::ImportedUserHandoff],
            capability_confidences: vec![M5CapabilityConfidence::Approximate],
            source_classes: vec![CertificationTarget::ImportedPendingReview],
            truth_modes: vec![M5BundleTruthMode::CachedOffline],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_disclosures: vec![case(support_replay_input())],
            overclaims_native_parity: false,
            implies_standalone_completeness: false,
            collapses_class_to_generic: false,
        },
    ]
}

fn seeded_governance_review() -> M5BundleDisclosureGovernanceReview {
    M5BundleDisclosureGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        disclosure_identity_preserved_across_surfaces: true,
        class_disclosed_with_shared_vocabulary: true,
        native_parity_never_inherited_when_mapped: true,
        dependency_posture_disclosed: true,
        recommendation_and_strength_disclosed: true,
        support_export_reconstructs_disclosure: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5BundleDisclosureConsumerProjection {
    M5BundleDisclosureConsumerProjection {
        disclosure_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        narrowing_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5BundleDisclosureReleasePosture {
    M5BundleDisclosureReleasePosture {
        release_packet_ref: M5_BUNDLE_CLASS_DISCLOSURE_ARTIFACT_REF.to_owned(),
        disclosure_audit_ref: M5_BUNDLE_CLASS_DISCLOSURE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 bundle class-disclosure primitive packet. This is the one
/// source of truth shared by the tests, the fixture generator, and the on-disk support export so
/// all three stay byte-aligned.
pub fn seeded_m5_bundle_class_disclosure_packet() -> M5BundleClassDisclosurePacket {
    M5BundleClassDisclosurePacket::new(M5BundleClassDisclosurePacketInput {
        packet_id: "m5-bundle-class-disclosure-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Bundle Class-Disclosure Primitive: Class-Disclosure Card and Claim-Narrowing Row"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5BundleDisclosureVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_BUNDLE_CLASS_DISCLOSURE_SCHEMA_REF.to_owned(),
            M5_BUNDLE_CLASS_DISCLOSURE_DOC_REF.to_owned(),
            M5_BUNDLE_CLASS_DISCLOSURE_COMPONENT_MATRIX_REF.to_owned(),
            M5_BUNDLE_CLASS_DISCLOSURE_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "workflow_bundle_component_boundary_v1".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    })
}
