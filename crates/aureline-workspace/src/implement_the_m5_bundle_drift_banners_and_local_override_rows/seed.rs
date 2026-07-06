// Canonical seed for the M5 bundle drift / override primitive. Included from `mod.rs` so the
// seeded builder, its worked cases, the fixture generator, and the on-disk support export all
// stay byte-aligned.

/// Builds one local-override row at a given granularity, deriving its significance from its
/// drift kind so the row is always honestly attributed.
fn override_row(
    granularity: M5DriftGranularity,
    target_ref: &str,
    label: &str,
    drift_kind: M5DriftKind,
    diff_action: DiffAction,
    ownership: AssetOwnership,
    resolution: ResolutionChoice,
) -> M5BundleLocalOverride {
    M5BundleLocalOverride {
        granularity,
        target_ref: target_ref.to_owned(),
        label: label.to_owned(),
        drift_kind,
        diff_action,
        ownership,
        resolution,
        significance: M5DriftSignificance::for_kind(drift_kind),
    }
}

/// Builds a bundle-declared artifact absent from the current state.
fn missing_artifact(
    component_kind: BundleComponentKind,
    artifact_ref: &str,
    label: &str,
) -> M5MissingArtifact {
    M5MissingArtifact {
        component_kind,
        artifact_ref: artifact_ref.to_owned(),
        label: label.to_owned(),
    }
}

/// Builds a one-step, pre-mutation rollback checkpoint.
fn checkpoint(checkpoint_ref: &str, captured_component_count: usize) -> RollbackCheckpoint {
    RollbackCheckpoint {
        checkpoint_ref: checkpoint_ref.to_owned(),
        one_step: true,
        reversible: true,
        captured_before_mutation: true,
        captured_component_count,
    }
}

/// A read-only drift review with both a harmless field override and a support-significant
/// package drift plus a missing artifact: enumerated drift, both significances on one banner.
fn harmless_and_significant_drift_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:rust-service:0001".to_owned(),
        surface_label: "Workspace drift banner for a certified Rust service stack".to_owned(),
        bundle_id_ref: "bundle:rust-service:0001".to_owned(),
        bundle_name: "Rust Service Starter".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        drift_state: DriftState::Diverged,
        operation: BundleReviewOperation::DriftReview,
        local_overrides: vec![
            override_row(
                M5DriftGranularity::Field,
                "field:rustfmt.max_width",
                "rustfmt max_width overridden locally",
                M5DriftKind::LocalOnlyEdit,
                DiffAction::Modified,
                AssetOwnership::LocallyOverridden,
                ResolutionChoice::KeepLocal,
            ),
            override_row(
                M5DriftGranularity::Package,
                "package:ext.rust-analyzer",
                "Rust Analyzer extension is behind the bundle version",
                M5DriftKind::BundleVersionDrift,
                DiffAction::Modified,
                AssetOwnership::BundleOwned,
                ResolutionChoice::Rebase,
            ),
        ],
        missing_artifacts: vec![missing_artifact(
            BundleComponentKind::TaskRecipe,
            "artifact:task.cargo-check",
            "cargo check task recipe is missing locally",
        )],
        recommended_choices: vec![
            ResolutionChoice::Rebase,
            ResolutionChoice::KeepLocal,
            ResolutionChoice::Compare,
        ],
        side_effects: vec![],
        rollback_checkpoint: None,
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            degraded_label:
                "Local state has diverged from the certified bundle: one harmless settings-field override, one package behind the bundle version, and one missing task recipe; each is shown at its own granularity"
                    .to_owned(),
        }),
    }
}

/// A support-significant package drift plus a missing artifact rebased in an update fix: a
/// mutating operation with a rollback checkpoint.
fn version_drift_rebase_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:web-app-rebase:0002".to_owned(),
        surface_label: "Bundle detail drift panel offering a rebase to the bundle".to_owned(),
        bundle_id_ref: "bundle:web-app:0002".to_owned(),
        bundle_name: "Managed Web App".to_owned(),
        bundle_class: BundleClass::OrgManagedBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::ManagedApproved,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        drift_state: DriftState::BundleAhead,
        operation: BundleReviewOperation::Update,
        local_overrides: vec![override_row(
            M5DriftGranularity::Package,
            "package:ext.web-tools",
            "Web tooling extension is behind the bundle version",
            M5DriftKind::BundleVersionDrift,
            DiffAction::Modified,
            AssetOwnership::BundleOwned,
            ResolutionChoice::Rebase,
        )],
        missing_artifacts: vec![missing_artifact(
            BundleComponentKind::LaunchRecipe,
            "artifact:launch.dev-server",
            "Dev server launch recipe is missing locally",
        )],
        recommended_choices: vec![ResolutionChoice::Rebase, ResolutionChoice::Compare],
        side_effects: vec![
            M5BundleSideEffectClass::ExtensionInstall,
            M5BundleSideEffectClass::TaskRecipeRegistration,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:rebase:0002", 2)),
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: None,
    }
}

/// A read-only drift review whose only signal is a missing artifact: support-significant, no
/// local overrides, offered a compare / adopt choice.
fn missing_artifact_only_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:missing-artifact:0003".to_owned(),
        surface_label: "Extension drift row reporting a missing bundle artifact".to_owned(),
        bundle_id_ref: "bundle:framework-pack:0003".to_owned(),
        bundle_name: "Community Framework Pack".to_owned(),
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        scorecard_class: BundleScorecardClass::Community,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        drift_state: DriftState::LocalAhead,
        operation: BundleReviewOperation::DriftReview,
        local_overrides: vec![],
        missing_artifacts: vec![missing_artifact(
            BundleComponentKind::DocsPack,
            "artifact:docs.framework-guide",
            "Framework docs pack is missing locally",
        )],
        recommended_choices: vec![ResolutionChoice::AdoptBundle, ResolutionChoice::Compare],
        side_effects: vec![],
        rollback_checkpoint: None,
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: None,
    }
}

/// An imported bundle whose local state has an imported gap: imported-not-native provenance
/// preserved, read-only comparison from an imported snapshot.
fn imported_gap_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:imported-gap:0004".to_owned(),
        surface_label: "Migration drift view reconstructing an imported bundle's drift".to_owned(),
        bundle_id_ref: "bundle:monorepo:0004".to_owned(),
        bundle_name: "Imported Monorepo Migration".to_owned(),
        bundle_class: BundleClass::ImportedHandoffBundle,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Labs,
        source_class: CertificationTarget::ImportedPendingReview,
        scorecard_class: BundleScorecardClass::Imported,
        certification_freshness: EvidenceFreshness::Stale,
        imported_confidence: ImportedVsNativeConfidence::Approximated,
        compatible_aureline_range: ">=2026.2, <2026.7".to_owned(),
        truth_mode: M5BundleTruthMode::Imported,
        drift_state: DriftState::Diverged,
        operation: BundleReviewOperation::DriftReview,
        local_overrides: vec![
            override_row(
                M5DriftGranularity::Task,
                "task:migration.settings-map",
                "Imported settings migration mapping has no local counterpart",
                M5DriftKind::ImportedGap,
                DiffAction::Removed,
                AssetOwnership::Adopted,
                ResolutionChoice::KeepLocal,
            ),
            override_row(
                M5DriftGranularity::Field,
                "field:editor.tab_size",
                "Imported editor tab_size override kept locally",
                M5DriftKind::LocalOnlyEdit,
                DiffAction::Modified,
                AssetOwnership::LocallyOverridden,
                ResolutionChoice::KeepLocal,
            ),
        ],
        missing_artifacts: vec![],
        recommended_choices: vec![ResolutionChoice::KeepLocal, ResolutionChoice::Compare],
        side_effects: vec![],
        rollback_checkpoint: None,
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
            degraded_label:
                "This bundle was imported from another setup and its certification is stale; the drift view keeps the imported-not-native provenance and preserves the local overrides"
                    .to_owned(),
        }),
    }
}

/// A managed bundle served from a stale mirror with a policy / entitlement narrowing: still
/// intelligible, blocked asset kept at compare-only.
fn mirror_policy_narrowing_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:mirror-policy:0005".to_owned(),
        surface_label: "Diagnostics drift report served from a stale mirror".to_owned(),
        bundle_id_ref: "bundle:web-app:0005".to_owned(),
        bundle_name: "Managed Web App (mirror)".to_owned(),
        bundle_class: BundleClass::OrgManagedBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::PolicyGated,
        source_class: CertificationTarget::ManagedApproved,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Mirrored,
        drift_state: DriftState::Diverged,
        operation: BundleReviewOperation::DriftReview,
        local_overrides: vec![override_row(
            M5DriftGranularity::Package,
            "package:ext.web-secure",
            "Secure web extension is blocked by org policy",
            M5DriftKind::PolicyEntitlementNarrowing,
            DiffAction::Modified,
            AssetOwnership::BlockedByPolicy,
            ResolutionChoice::Compare,
        )],
        missing_artifacts: vec![],
        recommended_choices: vec![ResolutionChoice::Compare, ResolutionChoice::KeepLocal],
        side_effects: vec![],
        rollback_checkpoint: None,
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
            degraded_label:
                "This managed bundle is served from a mirror whose evidence is aging and one asset is blocked by org policy; the drift report stays intelligible and keeps the blocked asset's compare-only resolution"
                    .to_owned(),
        }),
    }
}

/// A mutating remove that rolls back a bundle-owned asset: the rollback / remove card carries a
/// one-step checkpoint and removal side effects.
fn remove_rollback_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:remove-rollback:0006".to_owned(),
        surface_label: "Diagnostics drift report offering a bundle removal".to_owned(),
        bundle_id_ref: "bundle:framework-pack:0006".to_owned(),
        bundle_name: "Community Framework Pack".to_owned(),
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        scorecard_class: BundleScorecardClass::Community,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        drift_state: DriftState::Diverged,
        operation: BundleReviewOperation::Remove,
        local_overrides: vec![
            override_row(
                M5DriftGranularity::Package,
                "package:ext.framework-lint",
                "Framework lint extension is bundle-owned and removable",
                M5DriftKind::BundleVersionDrift,
                DiffAction::Removed,
                AssetOwnership::Removable,
                ResolutionChoice::RemoveBundleOwned,
            ),
            override_row(
                M5DriftGranularity::Field,
                "field:framework.lint_level",
                "Framework lint level overridden locally and kept",
                M5DriftKind::LocalOnlyEdit,
                DiffAction::Modified,
                AssetOwnership::LocallyOverridden,
                ResolutionChoice::KeepLocal,
            ),
        ],
        missing_artifacts: vec![],
        recommended_choices: vec![
            ResolutionChoice::RemoveBundleOwned,
            ResolutionChoice::KeepLocal,
            ResolutionChoice::Compare,
        ],
        side_effects: vec![
            M5BundleSideEffectClass::ExtensionInstall,
            M5BundleSideEffectClass::TaskRecipeRegistration,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:remove:0006", 2)),
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
            degraded_label:
                "Removing this community bundle rolls back its owned assets in one step and keeps the locally overridden lint level; the local override is preserved, not reset"
                    .to_owned(),
        }),
    }
}

/// A community bundle reconstructed from an offline cache with a stale-certification narrowing:
/// still intelligible under offline, replayed for support.
fn offline_replay_input() -> M5BundleDriftInput {
    M5BundleDriftInput {
        drift_id: "drift:offline-replay:0007".to_owned(),
        surface_label: "Support / export replay reconstructing drift from an offline cache"
            .to_owned(),
        bundle_id_ref: "bundle:framework-pack:0007".to_owned(),
        bundle_name: "Community Framework Pack (offline)".to_owned(),
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        scorecard_class: BundleScorecardClass::Community,
        certification_freshness: EvidenceFreshness::Stale,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::CachedOffline,
        drift_state: DriftState::Diverged,
        operation: BundleReviewOperation::DriftReview,
        local_overrides: vec![override_row(
            M5DriftGranularity::Task,
            "task:framework.build",
            "Framework build task overridden locally and kept",
            M5DriftKind::StaleCertification,
            DiffAction::Modified,
            AssetOwnership::Adopted,
            ResolutionChoice::KeepLocal,
        )],
        missing_artifacts: vec![missing_artifact(
            BundleComponentKind::TourPack,
            "artifact:tour.framework-intro",
            "Framework intro tour pack is missing locally",
        )],
        recommended_choices: vec![ResolutionChoice::KeepLocal, ResolutionChoice::Compare],
        side_effects: vec![],
        rollback_checkpoint: None,
        reads_like_generic_update: false,
        claims_harmless_despite_significant: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "This community bundle's drift is reconstructed from an offline cache with stale certification; the replay stays intelligible and names the offline-cache provenance"
                    .to_owned(),
        }),
    }
}

fn case(input: M5BundleDriftInput) -> M5BundleDriftCase {
    M5BundleDriftCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5BundleDriftSurfaceRow> {
    let base_source_refs = vec![
        M5_BUNDLE_DRIFT_SCHEMA_REF.to_owned(),
        M5_BUNDLE_DRIFT_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5BundleDriftExportField::ALL.to_vec();

    vec![
        M5BundleDriftSurfaceRow {
            surface_family: M5BundleDriftSurfaceFamily::WorkspaceDriftBanner,
            owner_role: "Workspace drift guild".to_owned(),
            scope_summary: "Workspace drift banner enumerating local-only edits, version drift, and missing artifacts with rebase / keep-local / compare choices"
                .to_owned(),
            operations: vec![BundleReviewOperation::DriftReview],
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["workspace_shell".to_owned(), "start_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_drifts: vec![case(harmless_and_significant_drift_input())],
            reads_like_generic_update: false,
            collapses_to_opaque_customized: false,
            forces_reset_to_export: false,
        },
        M5BundleDriftSurfaceRow {
            surface_family: M5BundleDriftSurfaceFamily::BundleDetailDriftPanel,
            owner_role: "Bundle detail guild".to_owned(),
            scope_summary: "Bundle detail drift panel offering a rebase to the bundle with a one-step rollback checkpoint before mutation"
                .to_owned(),
            operations: vec![BundleReviewOperation::Update],
            source_classes: vec![CertificationTarget::ManagedApproved],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
                M5BundleComponentDowngradeTrigger::IncompatibleAureline,
            ],
            consumer_surfaces: vec!["bundle_detail".to_owned(), "docs_bundles".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_drifts: vec![case(version_drift_rebase_input())],
            reads_like_generic_update: false,
            collapses_to_opaque_customized: false,
            forces_reset_to_export: false,
        },
        M5BundleDriftSurfaceRow {
            surface_family: M5BundleDriftSurfaceFamily::ExtensionDriftRow,
            owner_role: "Extension drift guild".to_owned(),
            scope_summary: "Extension drift row reporting a missing bundle artifact as support-significant drift, not a generic update"
                .to_owned(),
            operations: vec![BundleReviewOperation::DriftReview],
            source_classes: vec![CertificationTarget::CommunityReviewed],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::UnverifiedSigner,
            ],
            consumer_surfaces: vec!["extension_list".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_drifts: vec![case(missing_artifact_only_input())],
            reads_like_generic_update: false,
            collapses_to_opaque_customized: false,
            forces_reset_to_export: false,
        },
        M5BundleDriftSurfaceRow {
            surface_family: M5BundleDriftSurfaceFamily::MigrationDriftView,
            owner_role: "Migration drift guild".to_owned(),
            scope_summary: "Migration drift view reconstructing an imported bundle's imported-gap and local-only edits, preserving imported-not-native provenance"
                .to_owned(),
            operations: vec![BundleReviewOperation::DriftReview],
            source_classes: vec![CertificationTarget::ImportedPendingReview],
            truth_modes: vec![M5BundleTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["migration_review".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_drifts: vec![case(imported_gap_input())],
            reads_like_generic_update: false,
            collapses_to_opaque_customized: false,
            forces_reset_to_export: false,
        },
        M5BundleDriftSurfaceRow {
            surface_family: M5BundleDriftSurfaceFamily::DiagnosticsDriftReport,
            owner_role: "Diagnostics drift guild".to_owned(),
            scope_summary: "Diagnostics drift report covering a mirror-served policy narrowing and a one-step bundle removal preserving local overrides"
                .to_owned(),
            operations: vec![BundleReviewOperation::DriftReview, BundleReviewOperation::Remove],
            source_classes: vec![
                CertificationTarget::ManagedApproved,
                CertificationTarget::CommunityReviewed,
            ],
            truth_modes: vec![M5BundleTruthMode::Mirrored, M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::MirrorStale,
                M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_drifts: vec![
                case(mirror_policy_narrowing_input()),
                case(remove_rollback_input()),
            ],
            reads_like_generic_update: false,
            collapses_to_opaque_customized: false,
            forces_reset_to_export: false,
        },
        M5BundleDriftSurfaceRow {
            surface_family: M5BundleDriftSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing drift truth from an offline cache with a stale-certification narrowing"
                .to_owned(),
            operations: vec![BundleReviewOperation::DriftReview],
            source_classes: vec![CertificationTarget::CommunityReviewed],
            truth_modes: vec![M5BundleTruthMode::CachedOffline],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_drifts: vec![case(offline_replay_input())],
            reads_like_generic_update: false,
            collapses_to_opaque_customized: false,
            forces_reset_to_export: false,
        },
    ]
}

fn seeded_governance_review() -> M5BundleDriftGovernanceReview {
    M5BundleDriftGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        drift_identity_preserved_across_surfaces: true,
        drift_reviewable_at_detail: true,
        significance_distinguished: true,
        overrides_attributable_without_reset: true,
        rollback_checkpoint_created_before_mutation: true,
        support_export_reconstructs_drift: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5BundleDriftConsumerProjection {
    M5BundleDriftConsumerProjection {
        drift_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        override_list_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5BundleDriftReleasePosture {
    M5BundleDriftReleasePosture {
        release_packet_ref: M5_BUNDLE_DRIFT_ARTIFACT_REF.to_owned(),
        drift_audit_ref: M5_BUNDLE_DRIFT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 bundle drift / override primitive packet. This is the one
/// source of truth shared by the tests, the fixture generator, and the on-disk support export so
/// all three stay byte-aligned.
pub fn seeded_m5_bundle_drift_override_packet() -> M5BundleDriftOverridePacket {
    M5BundleDriftOverridePacket::new(M5BundleDriftOverridePacketInput {
        packet_id: "m5-bundle-drift-override-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Bundle Drift / Override Primitive: Drift Banner, Local-Override Rows, and Rollback / Remove Card"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5BundleDriftVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_BUNDLE_DRIFT_SCHEMA_REF.to_owned(),
            M5_BUNDLE_DRIFT_DOC_REF.to_owned(),
            M5_BUNDLE_DRIFT_COMPONENT_MATRIX_REF.to_owned(),
            M5_BUNDLE_DRIFT_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "workflow_bundle_component_boundary_v1".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    })
}
